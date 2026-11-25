//! 640×480 Pinball 2000 software compositor (`SoftwareCompositor`).
//!
//! Emulates XINA's `BaseViewManager` Z-ordered 640×240 MediaGX compositor with
//! vertical scan-doubling to `640×480` (or split playfield telemetry mode):
//! - Decodes all extracted SWE1 `.png` animations (`gfx_manifest.json`) lazily on demand.
//! - Loads all 19 Pinball 2000 bitmap fonts (`fonts_manifest.json`) with Left/Center/Right
//!   alignment and RGB555 foreground + shadow colors.
//! - Supports `flip_x`, X/Y scaling, and RGB555 monochrome palette tinting
//!   (`set_cloned_anim_palette_monochrome`).

use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};
use pin2k_hal::{AssetId, RectDrawCmd, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};
use pin2k_rom_tool::asset_extractor::{FontGlyphEntry, FontManifestEntry, FontsManifest, GfxManifest};

use crate::raster::{blit_rgba_clipped, blit_rgba_ex, draw_string, fill_rect, rgb555_to_rgba};

pub const SCREEN_W: u32 = 640;
pub const SCREEN_H: u32 = 480;

pub struct LoadedFont {
    pub meta: FontManifestEntry,
    pub atlas: RgbaImage,
    pub glyphs: HashMap<u8, FontGlyphEntry>,
}

pub struct LoadedAnim {
    pub name: String,
    pub fps: u32,
    pub height: u32,
    frame_paths: Vec<PathBuf>,
    decoded_frames: Vec<RgbaImage>,
}

impl LoadedAnim {
    fn ensure_decoded(&mut self) -> &[RgbaImage] {
        if self.decoded_frames.is_empty() && !self.frame_paths.is_empty() {
            for p in &self.frame_paths {
                if let Ok(img) = image::open(p) {
                    self.decoded_frames.push(img.to_rgba8());
                }
            }
        }
        &self.decoded_frames
    }
}

#[derive(Debug, Clone)]
enum LayerOp {
    Sprite(SpriteDrawCmd),
    Text(TextDrawCmd),
    Rect(RectDrawCmd),
}

impl LayerOp {
    fn z(&self) -> u8 {
        match self {
            LayerOp::Sprite(s) => s.z,
            LayerOp::Text(t) => t.z,
            LayerOp::Rect(r) => r.z,
        }
    }
}

pub struct SoftwareCompositor {
    pub framebuffer: RgbaImage,
    pub native_240: RgbaImage,
    pub native_fullscreen: bool,
    pub frames_presented: u64,
    anims: Vec<LoadedAnim>,
    anim_lookup: HashMap<String, usize>,
    fonts: HashMap<String, LoadedFont>,
    active_anim_idx: Option<usize>,
    pub(crate) anim_elapsed_ms: u64,
    anim_loop: bool,
    queued_texts: Vec<(String, i32, i32, [u8; 4], u8)>,
    queued_frame: usize,
    layer_ops: Vec<LayerOp>,
    last_layers: Vec<LayerOp>,
}

impl Default for SoftwareCompositor {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareCompositor {
    pub fn new() -> Self {
        Self {
            framebuffer: ImageBuffer::from_pixel(SCREEN_W, SCREEN_H, Rgba([6, 8, 18, 255])),
            native_240: ImageBuffer::from_pixel(SCREEN_W, 240, Rgba([5, 7, 15, 255])),
            native_fullscreen: true,
            frames_presented: 0,
            anims: Vec::new(),
            anim_lookup: HashMap::new(),
            fonts: HashMap::new(),
            active_anim_idx: None,
            anim_elapsed_ms: 0,
            anim_loop: true,
            queued_texts: Vec::new(),
            queued_frame: 0,
            layer_ops: Vec::new(),
            last_layers: Vec::new(),
        }
    }

    /// Indexes extracted SWE1 `.png` animations from `<assets_dir>/gfx/gfx_manifest.json`
    /// and decodes frames lazily on demand for instant (`< 5 ms`) startup.
    pub fn load_assets(&mut self, assets_dir: &Path) -> anyhow::Result<usize> {
        let gfx_dir = assets_dir.join("gfx");
        let manifest_path = gfx_dir.join("gfx_manifest.json");
        if !manifest_path.exists() {
            return Ok(0);
        }
        let raw = fs::read_to_string(&manifest_path)?;
        let manifest: GfxManifest = serde_json::from_str(&raw)?;

        for entry in manifest.animations {
            let frame_paths: Vec<PathBuf> = entry
                .exported_frames
                .iter()
                .map(|rel| gfx_dir.join(rel))
                .filter(|p| p.exists())
                .collect();
            if !frame_paths.is_empty() {
                let idx = self.anims.len();
                let clean = entry.symbol_name.trim_end_matches("_ptr").to_string();
                self.anim_lookup.insert(clean.clone(), idx);
                self.anims.push(LoadedAnim {
                    name: clean,
                    fps: entry.fps.max(1),
                    height: entry.height,
                    frame_paths,
                    decoded_frames: Vec::new(),
                });
            }
        }
        if !self.anims.is_empty() {
            self.active_anim_idx = Some(0);
        }

        // Load all 19 extracted Pinball 2000 bitmap fonts from `<assets_dir>/fonts/fonts_manifest.json`
        let fonts_dir = assets_dir.join("fonts");
        let fonts_manifest_path = fonts_dir.join("fonts_manifest.json");
        if fonts_manifest_path.exists() {
            if let Ok(fraw) = fs::read_to_string(&fonts_manifest_path) {
                if let Ok(fm) = serde_json::from_str::<FontsManifest>(&fraw) {
                    for fentry in fm.fonts {
                        let atlas_path = fonts_dir.join(&fentry.atlas_file);
                        if let Ok(img) = image::open(&atlas_path) {
                            let mut glyphs = HashMap::new();
                            for g in &fentry.glyphs {
                                glyphs.insert(g.code, g.clone());
                            }
                            self.fonts.insert(
                                fentry.name.clone(),
                                LoadedFont {
                                    meta: fentry,
                                    atlas: img.to_rgba8(),
                                    glyphs,
                                },
                            );
                        }
                    }
                }
            }
        }

        Ok(self.anims.len())
    }

    /// Loads custom `.png` animation folders from `<mod_dir>/gfx/<anim_name>/*.png`,
    /// overriding or adding animations for mods & standalone homebrew games.
    pub fn load_mod_assets(
        &mut self,
        mod_dir: &Path,
        anim_overrides: &HashMap<String, String>,
    ) -> anyhow::Result<usize> {
        let gfx_dir = mod_dir.join("gfx");
        if !gfx_dir.is_dir() {
            return Ok(0);
        }
        let mut loaded = 0usize;
        for entry in fs::read_dir(&gfx_dir)? {
            let entry = entry?;
            let p = entry.path();
            if !p.is_dir() {
                continue;
            }
            let Some(folder_name) = p.file_name().and_then(|s| s.to_str()).map(str::to_string) else {
                continue;
            };
            let mut frame_paths: Vec<PathBuf> = fs::read_dir(&p)?
                .filter_map(|f| f.ok().map(|x| x.path()))
                .filter(|fp| fp.extension().and_then(|s| s.to_str()) == Some("png"))
                .collect();
            frame_paths.sort();
            if frame_paths.is_empty() {
                continue;
            }
            let height = image::image_dimensions(&frame_paths[0]).map(|(_, h)| h).unwrap_or(120);
            let idx = self.anims.len();
            self.anim_lookup.insert(folder_name.clone(), idx);
            self.anims.push(LoadedAnim {
                name: folder_name.clone(),
                fps: 30,
                height,
                frame_paths,
                decoded_frames: Vec::new(),
            });
            for (target_hint, src_folder) in anim_overrides {
                if src_folder == &folder_name {
                    self.anim_lookup.insert(target_hint.clone(), idx);
                }
            }
            loaded += 1;
        }
        if self.active_anim_idx.is_none() && !self.anims.is_empty() {
            self.active_anim_idx = Some(0);
        }
        Ok(loaded)
    }

    /// Advances the compositor's animation clock by `dt_ms` (e.g. 16 ms per 60 Hz frame).
    pub fn tick_clock(&mut self, dt_ms: u64) {
        self.anim_elapsed_ms = self.anim_elapsed_ms.saturating_add(dt_ms);
    }

    fn is_oneshot_anim(name: &str) -> bool {
        let l = name.to_ascii_lowercase();
        l.contains("intro_movie")
            || l.contains("_single")
            || l.contains("_death")
            || l.contains("_clash")
            || l.contains("explosion")
            || l == "anim_p2k_logo_animated"
            || l == "anim_klogo_zoom_back"
            || l == "anim_attract_hasbro_ad"
    }

    /// Selects the best-matching loaded SWE1 animation for `hint` (exact match, then full-height substring match).
    /// Resets the animation clock when switching to a different animation, and enforces one-shot vs loop mode.
    pub fn select_anim_by_hint(&mut self, hint: &str) {
        if hint.is_empty() {
            self.active_anim_idx = None;
            self.anim_elapsed_ms = 0;
            return;
        }
        let target_idx = if let Some(&idx) = self.anim_lookup.get(hint) {
            Some(idx)
        } else {
            let lower = hint.to_ascii_lowercase();
            self.anims
                .iter()
                .enumerate()
                .find(|(_, a)| a.height >= 90 && a.name.to_ascii_lowercase().contains(&lower))
                .or_else(|| {
                    self.anims
                        .iter()
                        .enumerate()
                        .find(|(_, a)| a.name.to_ascii_lowercase().contains(&lower))
                })
                .map(|(i, _)| i)
        };
        if let Some(idx) = target_idx {
            if self.active_anim_idx != Some(idx) {
                self.active_anim_idx = Some(idx);
                self.anim_elapsed_ms = 0;
                self.anim_loop = !Self::is_oneshot_anim(&self.anims[idx].name);
            }
        }
    }

    pub fn encode_png(&self) -> anyhow::Result<Vec<u8>> {
        let mut out = Vec::new();
        self.framebuffer
            .write_to(&mut Cursor::new(&mut out), ImageFormat::Png)?;
        Ok(out)
    }

    /// Lists all loaded animations as `(name, frame_count, fps)`.
    pub fn anim_catalog(&self) -> Vec<(String, usize, u32)> {
        let mut out: Vec<(String, usize, u32)> = self
            .anims
            .iter()
            .map(|a| (a.name.clone(), a.frame_paths.len(), a.fps))
            .collect();
        out.sort_by(|a, b| a.0.cmp(&b.0));
        out
    }

    /// Lists all loaded Pinball 2000 bitmap font identifiers.
    pub fn font_names(&self) -> Vec<String> {
        let mut out: Vec<String> = self.fonts.keys().cloned().collect();
        out.sort();
        out
    }

    /// Returns a JSON-serializable summary of all Z-sorted layers rendered in the last frame.
    pub fn last_frame_layers_json(&self) -> serde_json::Value {
        let items: Vec<serde_json::Value> = self
            .last_layers
            .iter()
            .map(|op| match op {
                LayerOp::Sprite(s) => serde_json::json!({
                    "kind": "sprite",
                    "z": s.z,
                    "anim": s.anim_name,
                    "frame": s.frame,
                    "x": s.x,
                    "y": s.y,
                    "flip_x": s.flip_x,
                    "tint": s.monochrome_rgb555.map(|c| format!("0x{c:04X}")),
                }),
                LayerOp::Text(t) => serde_json::json!({
                    "kind": "text",
                    "z": t.z,
                    "font": t.font_name,
                    "text": t.text,
                    "x": t.x,
                    "y": t.y,
                    "color": format!("0x{:04X}", t.color_rgb555),
                    "align": format!("{:?}", t.align),
                }),
                LayerOp::Rect(r) => serde_json::json!({
                    "kind": "rect",
                    "z": r.z,
                    "x": r.x,
                    "y": r.y,
                    "w": r.w,
                    "h": r.h,
                    "color": format!("0x{:04X}", r.color_rgb555),
                    "alpha": r.alpha,
                }),
            })
            .collect();
        serde_json::Value::Array(items)
    }
}

impl VideoBackend for SoftwareCompositor {
    fn begin_frame(&mut self) {
        self.layer_ops.clear();
        self.queued_texts.clear();
    }

    fn draw_sprite(&mut self, _id: AssetId, frame: usize, _x: i32, _y: i32, _z: u8) {
        self.queued_frame = frame;
    }

    fn draw_named_sprite(&mut self, cmd: SpriteDrawCmd) {
        self.layer_ops.push(LayerOp::Sprite(cmd));
    }

    fn draw_font_text(&mut self, cmd: TextDrawCmd) {
        self.layer_ops.push(LayerOp::Text(cmd));
    }

    fn draw_rect(&mut self, cmd: RectDrawCmd) {
        self.layer_ops.push(LayerOp::Rect(cmd));
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32, color_rgb555: u16, scale: u8) {
        let [r, g, b, a] = rgb555_to_rgba(color_rgb555, 255);
        self.queued_texts
            .push((text.to_string(), x, y, [r, g, b, a], scale.max(1)));
    }

    fn present(&mut self) {
        // Render into native 640x240 MediaGX logical buffer first
        for px in self.native_240.pixels_mut() {
            *px = Rgba([0, 0, 0, 255]);
        }

        // Render fallback scene/animation layer when no explicit background layer (z <= 7) is queued
        let has_bg_layer = self.layer_ops.iter().any(|op| op.z() <= 7);

        if !has_bg_layer {
            for px in self.native_240.pixels_mut() {
                *px = Rgba([5, 7, 15, 255]);
            }
            for i in 0..48u32 {
                let sx = (i.wrapping_mul(137) + (self.frames_presented as u32) / 2) % SCREEN_W;
                let sy = i.wrapping_mul(53) % 235;
                self.native_240.put_pixel(sx, sy, Rgba([110, 130, 180, 255]));
            }

            if let Some(idx) = self.active_anim_idx {
                let elapsed = self.anim_elapsed_ms;
                let looping = self.anim_loop;
                if let Some(anim) = self.anims.get_mut(idx) {
                    let fps = anim.fps.max(1) as u64;
                    let frames = anim.ensure_decoded();
                    if !frames.is_empty() {
                        let raw_idx = ((elapsed * fps) / 1000) as usize;
                        let fi = if looping {
                            raw_idx % frames.len()
                        } else {
                            raw_idx.min(frames.len() - 1)
                        };
                        let f = &frames[fi];
                        let scale_x = if f.width() <= 240 { 2u32 } else { 1u32 };
                        let draw_w = f.width() * scale_x;
                        let draw_h = f.height();
                        let ox = ((SCREEN_W as i32) - (draw_w as i32)) / 2;
                        let oy = (185i32 - (draw_h as i32)).max(4);
                        blit_rgba_clipped(&mut self.native_240, f, ox, oy, scale_x, 240);
                    }
                }
            }
        }

        // Execute all Z-sorted XINA BaseViewManager layers
        let mut ops = std::mem::take(&mut self.layer_ops);
        ops.sort_by_key(|op| op.z());
        for op in &ops {
            match op {
                LayerOp::Sprite(s) => {
                    let _ = self.blit_named_ex(
                        &s.anim_name,
                        s.frame,
                        s.x,
                        s.y,
                        s.scale_x,
                        s.scale_y,
                        s.flip_x,
                        s.monochrome_rgb555,
                    );
                }
                LayerOp::Text(t) => {
                    let fg = rgb555_to_rgba(t.color_rgb555, 255);
                    let shadow = t.shadow_rgb555.map(|c| rgb555_to_rgba(c, 255));
                    if !self.draw_bitmap_text_ex(&t.font_name, &t.text, t.x, t.y, fg, shadow, t.align) {
                        let start_x = match t.align {
                            TextAlign::Left => t.x,
                            TextAlign::Center => t.x - (t.text.len() as i32 * 4),
                            TextAlign::Right => t.x - (t.text.len() as i32 * 8),
                        };
                        draw_string(&mut self.native_240, &t.text, start_x, t.y, fg, 1);
                    }
                }
                LayerOp::Rect(r) => {
                    let rgba = rgb555_to_rgba(r.color_rgb555, r.alpha);
                    fill_rect(&mut self.native_240, r.x, r.y, r.w, r.h, rgba);
                }
            }
        }
        self.last_layers = ops;

        // Draw legacy queued centered HUD texts if any remain (used by RFM/Boot/Admin fallback)
        let texts = std::mem::take(&mut self.queued_texts);
        for (txt, cx, y, color, mut scale) in texts {
            let font_name = Self::pick_bitmap_font(scale, &txt);
            if self.draw_bitmap_text_ex(font_name, &txt, cx, y, color, None, TextAlign::Center) {
                continue;
            }
            while scale > 1 && (txt.len() as i32) * 8 * (scale as i32) > 624 {
                scale -= 1;
            }
            let char_w = 8 * (scale as i32);
            let total_w = (txt.len() as i32) * char_w;
            let start_x = (cx - total_w / 2).max(8);
            draw_string(&mut self.native_240, &txt, start_x, y, color, scale);
        }

        if self.native_fullscreen {
            // Authentic MediaGX 640x240 -> 640x480 vertical scan-doubler
            let src_raw = self.native_240.as_raw();
            let dst_raw = self.framebuffer.as_mut();
            let row_bytes = (SCREEN_W as usize) * 4;
            for y in 0..240usize {
                let src_row = &src_raw[y * row_bytes..(y + 1) * row_bytes];
                dst_raw[(y * 2) * row_bytes..(y * 2 + 1) * row_bytes].copy_from_slice(src_row);
                dst_raw[(y * 2 + 1) * row_bytes..(y * 2 + 2) * row_bytes].copy_from_slice(src_row);
            }
        } else {
            // Split debug mode: 640x240 top + telemetry bottom
            let src_raw = self.native_240.as_raw();
            let dst_raw = self.framebuffer.as_mut();
            let row_bytes = (SCREEN_W as usize) * 4;
            dst_raw[..240 * row_bytes].copy_from_slice(src_raw);
            for px in dst_raw[240 * row_bytes..].chunks_exact_mut(4) {
                px.copy_from_slice(&[5, 7, 15, 255]);
            }
            fill_rect(&mut self.framebuffer, 0, 244, SCREEN_W as i32, 2, [70, 130, 220, 255]);
        }

        self.frames_presented += 1;
    }
}

impl SoftwareCompositor {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn blit_named_ex(
        &mut self,
        name: &str,
        frame_idx: usize,
        ox: i32,
        oy: i32,
        scale_x: u32,
        scale_y: u32,
        flip_x: bool,
        monochrome_rgb555: Option<u16>,
    ) -> bool {
        let Some(&idx) = self.anim_lookup.get(name) else {
            return false;
        };
        let Some(anim) = self.anims.get_mut(idx) else {
            return false;
        };
        let frames = anim.ensure_decoded();
        if frames.is_empty() {
            return false;
        }
        let f = frames[frame_idx % frames.len()].clone();
        blit_rgba_ex(
            &mut self.native_240,
            &f,
            ox,
            oy,
            scale_x,
            scale_y,
            240,
            flip_x,
            monochrome_rgb555,
        );
        true
    }

    fn pick_bitmap_font(scale: u8, text: &str) -> &'static str {
        let len = text.len();
        if scale >= 2 {
            if len <= 18 {
                "font_starwars_19_data"
            } else if len <= 26 {
                "font_shadow_15_narrow_data"
            } else {
                "font_shadow_11_narrow_data"
            }
        } else if len <= 28 {
            "font_shadow_9_data"
        } else {
            "font_system_7_data"
        }
    }

    /// All 19 extracted Pinball 2000 font identifiers available in `fonts_manifest.json`.
    #[allow(dead_code)]
    pub const ALL_EXTRACTED_FONTS: [&'static str; 19] = [
        "font_basic_19_data",
        "font_kahuna_shadow_9_data",
        "font_shadow_7_data",
        "font_shadow_7_narrow_data",
        "font_shadow_9_data",
        "font_shadow_9_narrow_data",
        "font_shadow_11_data",
        "font_shadow_11_narrow_data",
        "font_shadow_15_data",
        "font_shadow_15_narrow_data",
        "font_shadow_22_data",
        "font_starwars_19_data",
        "font_system_7_data",
        "font_system_9_data",
        "font_system_10_data",
        "font_system_11_data",
        "font_system_15_data",
        "font_system_20_data",
        "font_system_fxd6x7_data",
    ];

    fn measure_bitmap_text(&self, font: &LoadedFont, text: &str) -> i32 {
        let spacing = font.meta.default_spacing.max(1);
        let mut total_w = 0i32;
        for ch in text.bytes() {
            if let Some(g) = font.glyphs.get(&ch) {
                let adv = if g.advance_extra != 0 { g.advance_extra } else { spacing };
                total_w += g.width as i32 + adv;
            } else {
                total_w += 5 + spacing;
            }
        }
        total_w
    }

    /// Renders `text` with `align` (`Left`, `Center`, `Right`) using an extracted Pinball 2000 bitmap font atlas.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_bitmap_text_ex(
        &mut self,
        font_name: &str,
        text: &str,
        x: i32,
        y: i32,
        color: [u8; 4],
        shadow: Option<[u8; 4]>,
        align: TextAlign,
    ) -> bool {
        let Some(font) = self.fonts.get(font_name) else {
            return false;
        };
        let total_w = self.measure_bitmap_text(font, text);
        if align == TextAlign::Center && total_w > 632 && font_name != "font_system_7_data" {
            return self.draw_bitmap_text_ex("font_system_7_data", text, x, y, color, shadow, align);
        }

        let spacing = font.meta.default_spacing.max(1);
        let mut cur_x = match align {
            TextAlign::Left => x,
            TextAlign::Center => (x - total_w / 2).max(2),
            TextAlign::Right => x - total_w,
        };
        let ascent = font.meta.ascent as i32;
        let shadow_col = shadow.unwrap_or([
            color[0] / 3,
            color[1] / 3,
            color[2] / 3,
            255,
        ]);

        for ch in text.bytes() {
            let Some(g) = font.glyphs.get(&ch) else {
                cur_x += 5 + spacing;
                continue;
            };
            let gx = cur_x + g.x_offset;
            let gy = y + ascent + g.y_offset - g.height as i32;
            for py in 0..g.height {
                let dy = gy + py as i32;
                if !(0..240).contains(&dy) {
                    continue;
                }
                for px in 0..g.width {
                    let dx = gx + px as i32;
                    if !(0..SCREEN_W as i32).contains(&dx) {
                        continue;
                    }
                    let src = font.atlas.get_pixel(g.atlas_x + px, g.atlas_y + py);
                    if src[3] == 0 {
                        continue;
                    }
                    let out = if src[0] > 180 {
                        Rgba(color)
                    } else if src[0] < 90 {
                        Rgba(shadow_col)
                    } else {
                        Rgba([
                            ((src[0] as u16 * color[0] as u16) / 255) as u8,
                            ((src[1] as u16 * color[1] as u16) / 255) as u8,
                            ((src[2] as u16 * color[2] as u16) / 255) as u8,
                            255,
                        ])
                    };
                    self.native_240.put_pixel(dx as u32, dy as u32, out);
                }
            }
            let adv = if g.advance_extra != 0 { g.advance_extra } else { spacing };
            cur_x += g.width as i32 + adv;
        }
        true
    }
}
