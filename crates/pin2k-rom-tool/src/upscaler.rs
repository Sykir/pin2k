//! Pure-Rust HD sprite upscaler (`pin2k-rom-tool upscale-assets`).
//!
//! Performs transparent-pixel RGB edge-padding before resizing (`Lanczos3`, `CatmullRom`,
//! or `Nearest`) so 1-bit transparent Pinball 2000 sprites (`0x7C1F`) upscale cleanly to
//! `gfx_hd/` without dark or magenta fringes.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use image::imageops::{resize, FilterType};
use image::{Rgba, RgbaImage};

use crate::asset_extractor::GfxManifest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpscaleFilter {
    Lanczos3,
    CatmullRom,
    Nearest,
}

impl UpscaleFilter {
    fn to_image_filter(self) -> FilterType {
        match self {
            UpscaleFilter::Lanczos3 => FilterType::Lanczos3,
            UpscaleFilter::CatmullRom => FilterType::CatmullRom,
            UpscaleFilter::Nearest => FilterType::Nearest,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpscaleOptions {
    pub scale: u32,
    pub filter: UpscaleFilter,
    pub anim_filter: Option<String>,
    pub crisp_alpha: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct UpscaleReport {
    pub animations_processed: usize,
    pub frames_upscaled: usize,
    pub scale: u32,
}

/// Dilates RGB channels from opaque pixels into neighboring transparent pixels (`a == 0`)
/// so multi-tap resize kernels (Lanczos3/CatmullRom) never bleed black/magenta borders.
pub fn edge_pad_transparent_rgb(src: &RgbaImage, passes: u32) -> RgbaImage {
    let (w, h) = src.dimensions();
    let mut cur = src.clone();
    let mut known: Vec<bool> = src.pixels().map(|p| p[3] > 0).collect();

    for _ in 0..passes {
        let mut next = cur.clone();
        let mut next_known = known.clone();
        for y in 0..h {
            for x in 0..w {
                let idx = (y * w + x) as usize;
                if known[idx] {
                    continue;
                }
                let (mut r_sum, mut g_sum, mut b_sum, mut count) = (0u32, 0u32, 0u32, 0u32);
                for dy in -1i32..=1 {
                    for dx in -1i32..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        if nx >= 0 && ny >= 0 && (nx as u32) < w && (ny as u32) < h {
                            let nidx = (ny as u32 * w + nx as u32) as usize;
                            if known[nidx] {
                                let p = cur.get_pixel(nx as u32, ny as u32);
                                r_sum += p[0] as u32;
                                g_sum += p[1] as u32;
                                b_sum += p[2] as u32;
                                count += 1;
                            }
                        }
                    }
                }
                if count > 0 {
                    next.put_pixel(
                        x,
                        y,
                        Rgba([
                            (r_sum / count) as u8,
                            (g_sum / count) as u8,
                            (b_sum / count) as u8,
                            0,
                        ]),
                    );
                    next_known[idx] = true;
                }
            }
        }
        cur = next;
        known = next_known;
    }
    cur
}

/// Upscales a single RGBA8888 sprite frame by `scale` (`2..=4`) with edge-padding.
pub fn upscale_rgba_image(
    src: &RgbaImage,
    scale: u32,
    filter: UpscaleFilter,
    crisp_alpha: bool,
) -> RgbaImage {
    let s = scale.clamp(1, 4);
    if s == 1 {
        return src.clone();
    }
    let padded = if filter == UpscaleFilter::Nearest {
        src.clone()
    } else {
        edge_pad_transparent_rgb(src, 2)
    };
    let dst_w = padded.width().saturating_mul(s).max(1);
    let dst_h = padded.height().saturating_mul(s).max(1);
    let mut out = resize(&padded, dst_w, dst_h, filter.to_image_filter());
    if crisp_alpha {
        for px in out.pixels_mut() {
            // Preserve edge-padded RGB on transparent pixels so downstream bilinear/GPU
            // downsampling never bleeds black halos into the sprite silhouette.
            px[3] = if px[3] >= 128 { 255 } else { 0 };
        }
    }
    out
}

/// Upscales extracted animations from `<assets_dir>/gfx/` into `<assets_dir>/gfx_hd/`.
pub fn upscale_assets_dir(assets_dir: &Path, opts: &UpscaleOptions) -> Result<UpscaleReport> {
    let scale = opts.scale.clamp(2, 4);
    let src_gfx = assets_dir.join("gfx");
    let manifest_path = src_gfx.join("gfx_manifest.json");
    let raw = fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;
    let mut manifest: GfxManifest = serde_json::from_str(&raw)?;

    let dst_gfx = assets_dir.join("gfx_hd");
    fs::create_dir_all(&dst_gfx)?;

    let filter_lc = opts.anim_filter.as_ref().map(|s| s.to_ascii_lowercase());
    let mut report = UpscaleReport {
        animations_processed: 0,
        frames_upscaled: 0,
        scale,
    };

    let mut out_anims = Vec::new();
    for mut entry in manifest.animations.drain(..) {
        if let Some(ref flt) = filter_lc {
            if !entry.symbol_name.to_ascii_lowercase().contains(flt) {
                continue;
            }
        }
        let mut upscaled_frames = Vec::with_capacity(entry.exported_frames.len());
        for rel in &entry.exported_frames {
            let src_path = src_gfx.join(rel);
            if !src_path.exists() {
                continue;
            }
            let img = image::open(&src_path)
                .with_context(|| format!("failed to open {}", src_path.display()))?
                .to_rgba8();
            let up = upscale_rgba_image(&img, scale, opts.filter, opts.crisp_alpha);
            let dst_path = dst_gfx.join(rel);
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)?;
            }
            up.save(&dst_path)?;
            upscaled_frames.push(rel.clone());
            report.frames_upscaled += 1;
        }
        if !upscaled_frames.is_empty() {
            entry.width = entry.width.saturating_mul(scale);
            entry.height = entry.height.saturating_mul(scale);
            entry.exported_frames = upscaled_frames;
            out_anims.push(entry);
            report.animations_processed += 1;
        }
    }

    manifest.animations_exported = out_anims.len();
    manifest.animations = out_anims;
    let out_manifest = dst_gfx.join("gfx_manifest.json");
    fs::write(&out_manifest, serde_json::to_string_pretty(&manifest)?)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_padding_prevents_black_halo_and_scales_dimensions() {
        let mut img = RgbaImage::from_pixel(4, 4, Rgba([0, 0, 0, 0]));
        img.put_pixel(1, 1, Rgba([255, 220, 40, 255]));
        img.put_pixel(2, 1, Rgba([255, 220, 40, 255]));
        img.put_pixel(1, 2, Rgba([255, 220, 40, 255]));
        img.put_pixel(2, 2, Rgba([255, 220, 40, 255]));

        let padded = edge_pad_transparent_rgb(&img, 1);
        // Neighboring transparent pixel inherits yellow RGB while keeping alpha == 0
        let edge_px = padded.get_pixel(0, 1);
        assert_eq!(edge_px[3], 0);
        assert_eq!(edge_px[0], 255);
        assert_eq!(edge_px[1], 220);

        let up = upscale_rgba_image(&img, 2, UpscaleFilter::Lanczos3, true);
        assert_eq!(up.dimensions(), (8, 8));
        assert_eq!(up.get_pixel(3, 3)[3], 255);
        assert_eq!(up.get_pixel(0, 0)[3], 0);
    }
}
