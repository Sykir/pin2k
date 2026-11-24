//! 640×480 Pinball 2000 software compositor (`SoftwareCompositor`).
//!
//! Renders:
//! - Upper holoprojector region (`640×240`): real extracted SWE1 animations/movies
//!   from `gfx_manifest.json` + crisp 8×8 bitmap HUD text.
//! - Lower playfield diagnostics & Pepper's Ghost reflection overlay (`640×240`):
//!   live visualization of SWE1's 128 Dual-Matrix Lamps, 48 Solenoid/Flasher/Neon
//!   Drives, and PDB bus status at their authentic SWE1 playfield `(x, y)` coordinates.

use std::collections::HashMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::{ImageBuffer, ImageFormat, Rgba, RgbaImage};
use crate::raster::{blit_rgba_clipped, draw_string, fill_rect};
use pin2k_hal::{AssetId, VideoBackend};
use pin2k_rom_tool::asset_extractor::{FontGlyphEntry, FontManifestEntry, FontsManifest, GfxManifest};

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
    podrace_scene_ticks: usize,
    queued_texts: Vec<(String, i32, i32, [u8; 4], u8)>,
    queued_frame: usize,
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
            podrace_scene_ticks: 0,
            queued_texts: Vec::new(),
            queued_frame: 0,
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

    /// Draws the live SWE1 playfield lamp & solenoid telemetry panel onto the lower half (`y=248..476`).
    pub fn encode_png(&self) -> anyhow::Result<Vec<u8>> {
        let mut out = Vec::new();
        self.framebuffer
            .write_to(&mut Cursor::new(&mut out), ImageFormat::Png)?;
        Ok(out)
    }
}

impl VideoBackend for SoftwareCompositor {
    fn draw_sprite(&mut self, _id: AssetId, frame: usize, _x: i32, _y: i32, _z: u8) {
        self.queued_frame = frame;
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32, color_rgb555: u16, scale: u8) {
        let r = (((color_rgb555 >> 10) & 0x1F) as u8) << 3;
        let g = (((color_rgb555 >> 5) & 0x1F) as u8) << 3;
        let b = ((color_rgb555 & 0x1F) as u8) << 3;
        self.queued_texts
            .push((text.to_string(), x, y, [r, g, b, 255], scale.max(1)));
    }

    fn present(&mut self) {
        // Render into native 640x240 MediaGX logical buffer first
        for px in self.native_240.pixels_mut() {
            *px = Rgba([5, 7, 15, 255]);
        }

        // Draw starfield backdrop
        for i in 0..48u32 {
            let sx = (i.wrapping_mul(137) + (self.frames_presented as u32) / 2) % SCREEN_W;
            let sy = i.wrapping_mul(53) % 235;
            self.native_240.put_pixel(sx, sy, Rgba([110, 130, 180, 255]));
        }

        let active_name = self
            .active_anim_idx
            .and_then(|i| self.anims.get(i))
            .map(|a| a.name.clone())
            .unwrap_or_default();

        if !self.render_specialized_scene(&active_name) {
            self.podrace_scene_ticks = 0;
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

        // Draw queued centered HUD texts using extracted Pinball 2000 bitmap fonts (with 8x8 fallback)
        let texts = std::mem::take(&mut self.queued_texts);
        for (txt, cx, y, color, mut scale) in texts {
            let font_name = Self::pick_bitmap_font(scale, &txt);
            if self.draw_bitmap_text(font_name, &txt, cx, y, color) {
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
    pub(crate) fn blit_named(&mut self, name: &str, frame_idx: usize, ox: i32, oy: i32, scale_x: u32) -> bool {
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
        blit_rgba_clipped(&mut self.native_240, &f, ox, oy, scale_x, 240);
        true
    }

    /// Multi-layer *Star Wars Episode I* `DeffPodRaceSceneBackground` holoprojector compositor:
    /// - Intro (`t < 60`): Boonta Eve starting-grid movie (`anim_podrace_starting`, 60 frames) + badge
    /// - Race (`t >= 60`): Sky + scrolling Beggar's Canyon horizon + rushing desert floor +
    ///   rival podracer / explosion + 100-frame Anakin's Podracer + Checkpoint & Speedometer HUD
    /// - Victory (`victory == true`): Sky + Horizon + Ground + `anim_podrace_closeup` camera flyby
    pub(crate) fn render_podrace_scene(&mut self, victory: bool) {
        let t = self.podrace_scene_ticks;
        self.podrace_scene_ticks = self.podrace_scene_ticks.saturating_add(1);

        if !victory && t < 60 {
            if self.blit_named("anim_podrace_starting", t, 0, 0, 2) {
                let _ = self.blit_named("anim_podrace_intro_badge", 0, 187, 8, 1);
                return;
            }
        }

        let race_t = t.saturating_sub(60);
        // Layer 0: Tatooine desert sky (`640x99`)
        let _ = self.blit_named("anim_podrace_sky", 0, 0, 0, 1);
        // Layer 1: Scrolling Beggar's Canyon cliffs (`800x112`)
        let scroll_x = -(((race_t * 3) % 160) as i32);
        let _ = self.blit_named("anim_podrace_horizon", 0, scroll_x, 4, 1);
        // Layer 2: High-speed rushing canyon floor (`640x128`, 21 frames)
        let _ = self.blit_named("anim_podrace_ground", race_t, 0, 112, 1);

        if victory {
            let _ = self.blit_named("anim_podrace_closeup", race_t, 0, 0, 1);
            return;
        }

        // Layer 3: Rival Sebulba podracer ahead (`anim_podrace_2`) or fiery explosion (`anim_podrace_2_explosion`)
        let cycle = race_t % 55;
        if cycle >= 28 && cycle < 54 {
            let _ = self.blit_named("anim_podrace_2_explosion", cycle - 28, 145, 18, 1);
        } else {
            let rival_x = 245 + (((race_t as i32) * 5) % 90) - 45;
            let _ = self.blit_named("anim_podrace_2", race_t, rival_x, 90, 1);
        }

        // Layer 4: Anakin's Radon-Ulzer 620C Podracer (`anim_podrace_1`, 100 banking/boosting frames)
        let sway = match (race_t / 20) % 4 {
            0 => (race_t % 20) as i32 * 2 - 20,
            1 => 20 - (race_t % 20) as i32 * 2,
            2 => -((race_t % 20) as i32),
            _ => -20 + (race_t % 20) as i32,
        };
        let _ = self.blit_named("anim_podrace_1", race_t, 143 + sway, 76, 1);

        // Layer 5: Checkpoint badge flash & live Podracer Speedometer HUD
        if (race_t > 50 && race_t < 68) || (race_t > 100 && race_t < 118) {
            let _ = self.blit_named("anim_podrace_checkpoint", 0, 224, 10, 1);
        }
        let mph = 615 + ((race_t * 4) % 332);
        fill_rect(&mut self.native_240, 488, 8, 142, 16, [10, 18, 34, 255]);
        draw_string(
            &mut self.native_240,
            &format!("SPEED {mph} MPH"),
            496,
            12,
            [110, 250, 255, 255],
            1,
        );
    }

    /// Selects the authentic Pinball 2000 bitmap font name for a given scale and text length.
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

    /// Renders `text` centered at `cx` using an extracted Pinball 2000 bitmap font atlas.
    pub fn draw_bitmap_text(
        &mut self,
        font_name: &str,
        text: &str,
        cx: i32,
        y: i32,
        color: [u8; 4],
    ) -> bool {
        let Some(font) = self.fonts.get(font_name) else {
            return false;
        };
        let spacing = font.meta.default_spacing.max(1);
        let mut total_w = 0i32;
        for ch in text.bytes() {
            if let Some(g) = font.glyphs.get(&ch) {
                let adv = if g.advance_extra != 0 { g.advance_extra } else { spacing };
                total_w += g.width as i32 + adv;
            } else {
                total_w += 6 + spacing;
            }
        }
        let mut cur_x = (cx - total_w / 2).max(4);
        if total_w > 632 {
            // Fall back to a narrower font if available
            if font_name != "font_system_7_data" {
                return self.draw_bitmap_text("font_system_7_data", text, cx, y, color);
            }
        }
        let ascent = font.meta.ascent as i32;
        for ch in text.bytes() {
            let Some(g) = font.glyphs.get(&ch) else {
                cur_x += 6 + spacing;
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

    /// Renders the authentic SWE1 InGame Holoprojector Spinners (`anim_spinner_1` + `anim_spinner_2`
    /// with brackets and `anim_naboo_fighter_tiny_rotate` center ship).
    pub(crate) fn render_ingame_spinners(&mut self) {
        let f60 = ((self.anim_elapsed_ms * 60) / 1000) as usize;
        // Left Spinner + bracket
        let _ = self.blit_named("anim_spinner_bracket_left", 0, 38, 72, 1);
        let _ = self.blit_named("anim_spinner_1", f60, 38, 72, 1);
        // Right Spinner + bracket
        let _ = self.blit_named("anim_spinner_bracket_right", 0, 452, 72, 1);
        let _ = self.blit_named("anim_spinner_2", f60, 452, 72, 1);
        // Center rotating Naboo Starfighter
        let _ = self.blit_named("anim_naboo_fighter_tiny_rotate", f60, 270, 26, 1);
    }
}

