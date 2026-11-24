//! Phase 2 Asset Extraction Pipeline:
//! - Extracts named `Animation` sprites and movies from `prism_flat.bin` + `bar3.flash`
//!   using symbol pointers from `game.rom` + `symbols.rom`, exporting RGBA8888 `.png`
//!   frames and `gfx_manifest.json`.
//! - Extracts DCS-2 / `pb2kslib` OGG Vorbis audio tracks from `{game}_sound.bin`
//!   (XOR `0x3A` container) and writes `sound_manifest.json` indexed by hex DCS ID.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::gfx_decoder::{
    decompress_p2k_frame_into_rgb555, rgb555_canvas_to_rgba, save_png, P2K_TRANSPARENT_KEY_RGB555,
};
use crate::symbols::SymbolTable;
use crate::RomToolError;

const PB2KSLIB_XOR_KEY: u8 = 0x3A;

/// Virtual memory view spanning `game.rom` (`0x0010_0000`), `BAR3` (`0x1200_0000`),
/// and `BAR5` (`0x1400_0000` + PLX `0x0880_0000` mirror).
pub struct PrismMemorySpace<'a> {
    pub game_rom: &'a [u8],
    pub game_load_addr: u32,
    pub prism_flat: &'a [u8],
    pub bar3_flash: Option<&'a [u8]>,
}

impl<'a> PrismMemorySpace<'a> {
    /// Slices bytes starting at guest physical/virtual address `addr` up to the end of its region.
    pub fn slice_from(&self, addr: u32) -> Option<&'a [u8]> {
        if addr >= self.game_load_addr {
            let off = (addr - self.game_load_addr) as usize;
            if off < self.game_rom.len() {
                return Some(&self.game_rom[off..]);
            }
        }
        if (0x1400_0000..0x1800_0000).contains(&addr) {
            let off = (addr - 0x1400_0000) as usize;
            if off < self.prism_flat.len() {
                return Some(&self.prism_flat[off..]);
            }
        }
        if let Some(flash) = self.bar3_flash {
            if (0x1200_0000..0x1240_0000).contains(&addr) {
                let off = (addr - 0x1200_0000) as usize;
                if off < flash.len() {
                    return Some(&flash[off..]);
                }
            }
        }
        if (0x0800_0000..0x0880_0000).contains(&addr) {
            let off = (addr - 0x0800_0000) as usize;
            if off < self.prism_flat.len() {
                return Some(&self.prism_flat[off..]);
            }
        }
        if (0x0880_0000..0x0B80_0000).contains(&addr) {
            let off = (addr - 0x0880_0000) as usize + 0x0100_0000;
            if off < self.prism_flat.len() {
                return Some(&self.prism_flat[off..]);
            }
        }
        None
    }

    pub fn read_u32(&self, addr: u32) -> Option<u32> {
        let s = self.slice_from(addr)?;
        if s.len() < 4 {
            return None;
        }
        Some(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
}

/// Metadata for a single extracted `Animation` sequence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationManifestEntry {
    pub symbol_name: String,
    pub anim_addr: u32,
    pub frame_count: u32,
    pub fps: u32,
    pub width: u32,
    pub height: u32,
    pub compression_codes: Vec<u32>,
    pub exported_frames: Vec<String>,
}

/// Manifest written to `gfx_manifest.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GfxManifest {
    pub total_animations_discovered: usize,
    pub animations_exported: usize,
    pub animations: Vec<AnimationManifestEntry>,
}

/// Discovers and extracts named `Animation` sequences referenced by `*_ptr` globals in `.data`.
pub fn extract_named_animations(
    mem: &PrismMemorySpace<'_>,
    symbols: &SymbolTable,
    out_dir: &Path,
    filter: Option<&str>,
    max_anims: Option<usize>,
    max_frames_per_anim: usize,
) -> Result<GfxManifest, RomToolError> {
    fs::create_dir_all(out_dir)?;
    let filter_lower = filter.map(|s| s.to_ascii_lowercase());

    let mut discovered = 0usize;
    let mut entries = Vec::new();

    for sym in &symbols.entries {
        if !sym.name.ends_with("_ptr") {
            continue;
        }
        if let Some(ref needle) = filter_lower {
            if !sym.name.to_ascii_lowercase().contains(needle) {
                continue;
            }
        }

        let Some(anim_addr) = mem.read_u32(sym.address) else {
            continue;
        };
        let Some(anim_hdr) = mem.slice_from(anim_addr) else {
            continue;
        };
        if anim_hdr.len() < 32 {
            continue;
        }

        let num_frames = u32::from_le_bytes(anim_hdr[0..4].try_into().unwrap());
        let fps = u32::from_le_bytes(anim_hdr[4..8].try_into().unwrap());
        let anim_w = u32::from_le_bytes(anim_hdr[8..12].try_into().unwrap());
        let anim_h = u32::from_le_bytes(anim_hdr[12..16].try_into().unwrap());
        let pal_entries = u32::from_le_bytes(anim_hdr[16..20].try_into().unwrap());
        let pal_ptr = u32::from_le_bytes(anim_hdr[20..24].try_into().unwrap());
        let frames_ptr = u32::from_le_bytes(anim_hdr[28..32].try_into().unwrap());

        if num_frames == 0 || num_frames > 4096 || anim_w == 0 || anim_w > 1024 || anim_h == 0 || anim_h > 1024 {
            continue;
        }

        let Some(frames_table) = mem.slice_from(frames_ptr) else {
            continue;
        };
        let table_bytes = (num_frames as usize) * 24;
        if frames_table.len() < table_bytes {
            continue;
        }

        // Validate first frame header
        let comp0 = u32::from_le_bytes(frames_table[0..4].try_into().unwrap());
        if !matches!(comp0, 0 | 1 | 20 | 21 | 22 | 23 | 30 | 42 | 43 | 44 | 45) {
            continue;
        }

        discovered += 1;
        if max_anims.is_some_and(|max| entries.len() >= max) {
            continue;
        }

        let clean_name = sym
            .name
            .trim_end_matches("_ptr")
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
            .collect::<String>();

        let anim_dir = out_dir.join(&clean_name);
        fs::create_dir_all(&anim_dir)?;

        let palette_slice = if pal_ptr != 0 {
            mem.slice_from(pal_ptr)
        } else {
            None
        };

        let mut canvas = vec![P2K_TRANSPARENT_KEY_RGB555; (anim_w as usize) * (anim_h as usize)];
        let frames_to_decode = (num_frames as usize).min(max_frames_per_anim.max(1));
        let mut exported_frames = Vec::new();
        let mut comp_codes = Vec::new();

        for fi in 0..frames_to_decode {
            let foff = fi * 24;
            let comp = u32::from_le_bytes(frames_table[foff..foff + 4].try_into().unwrap());
            let fw = u32::from_le_bytes(frames_table[foff + 4..foff + 8].try_into().unwrap());
            let fh = u32::from_le_bytes(frames_table[foff + 8..foff + 12].try_into().unwrap());
            let data_ptr = u32::from_le_bytes(frames_table[foff + 20..foff + 24].try_into().unwrap());

            if !comp_codes.contains(&comp) {
                comp_codes.push(comp);
            }

            let w = if fw > 0 { fw } else { anim_w };
            let h = if fh > 0 { fh } else { anim_h };
            if canvas.len() != (w as usize) * (h as usize) {
                canvas.resize((w as usize) * (h as usize), P2K_TRANSPARENT_KEY_RGB555);
            }

            let Some(frame_bytes) = mem.slice_from(data_ptr) else {
                break;
            };
            if decompress_p2k_frame_into_rgb555(
                comp,
                w,
                h,
                palette_slice,
                pal_entries,
                frame_bytes,
                &mut canvas,
            )
            .is_err()
            {
                break;
            }

            // Movies (code 30) are opaque video clips; sprites treat 0x7C1F as transparent
            let key = if comp == 30 {
                None
            } else {
                Some(P2K_TRANSPARENT_KEY_RGB555)
            };
            let rgba = rgb555_canvas_to_rgba(&canvas, w, h, key);
            let rel_png = format!("{clean_name}/frame_{fi:04}.png");
            let full_png = out_dir.join(&rel_png);
            save_png(&rgba, &full_png)
                .map_err(|e| RomToolError::InvalidFormat(format!("PNG save error: {e}")))?;
            exported_frames.push(rel_png);
        }

        entries.push(AnimationManifestEntry {
            symbol_name: sym.name.clone(),
            anim_addr,
            frame_count: num_frames,
            fps,
            width: anim_w,
            height: anim_h,
            compression_codes: comp_codes,
            exported_frames,
        });
    }

    let manifest = GfxManifest {
        total_animations_discovered: discovered,
        animations_exported: entries.len(),
        animations: entries,
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| RomToolError::InvalidFormat(format!("JSON serialize error: {e}")))?;
    fs::write(out_dir.join("gfx_manifest.json"), manifest_json)?;

    Ok(manifest)
}

/// Loop mode inferred from a `pb2kslib` entry name (`S0001-LP`, `S0001-LP1`, `S0001-LP2`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SoundLoopKind {
    OneShot,
    SingleLoop,
    LoopIntro,
    LoopBody,
}

/// Metadata for an extracted DCS-2 / `pb2kslib` audio asset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundManifestEntry {
    pub name: String,
    pub dcs_id: Option<u16>,
    pub format: String,
    pub loop_kind: SoundLoopKind,
    pub byte_size: u32,
    pub file: String,
}

/// Manifest written to `sound_manifest.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundManifest {
    pub version: u32,
    pub total_entries: usize,
    pub sounds: Vec<SoundManifestEntry>,
}

/// Extracts XOR-`0x3A` OGG Vorbis audio streams from `{game}_sound.bin` and writes `sound_manifest.json`.
pub fn extract_pb2kslib_sounds(
    sound_bin: &[u8],
    out_dir: &Path,
    limit: Option<usize>,
) -> Result<SoundManifest, RomToolError> {
    if sound_bin.len() < 16 {
        return Err(RomToolError::InvalidFormat(
            "sound library shorter than 16-byte header".into(),
        ));
    }

    let version = u32::from_le_bytes(sound_bin[0..4].try_into().unwrap());
    let hdr_size = u32::from_le_bytes(sound_bin[4..8].try_into().unwrap()) as usize;
    let count = u32::from_le_bytes(sound_bin[8..12].try_into().unwrap()) as usize;
    let entry_size = u32::from_le_bytes(sound_bin[12..16].try_into().unwrap()) as usize;

    if version != 1 || hdr_size != 16 || entry_size != 72 || count > 8192 {
        return Err(RomToolError::InvalidFormat(format!(
            "unsupported pb2kslib header: ver={version} hdr={hdr_size} count={count} entry_sz={entry_size}"
        )));
    }
    if hdr_size + count * entry_size > sound_bin.len() {
        return Err(RomToolError::InvalidFormat(
            "truncated pb2kslib index table".into(),
        ));
    }

    fs::create_dir_all(out_dir)?;
    let max_to_extract = limit.unwrap_or(count).min(count);
    let mut sounds = Vec::with_capacity(max_to_extract);

    for i in 0..max_to_extract {
        let eoff = hdr_size + i * entry_size;
        let mut dec = [0u8; 72];
        for (dst, &src) in dec.iter_mut().zip(&sound_bin[eoff..eoff + 72]) {
            *dst = src ^ PB2KSLIB_XOR_KEY;
        }

        let n1 = dec.iter().position(|&b| b == 0).unwrap_or(32);
        let raw_name = String::from_utf8_lossy(&dec[..n1]).into_owned();
        let safe_name: String = raw_name
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
            .collect();

        let fmt_start = (n1 + 1).min(64);
        let n2 = dec[fmt_start..64]
            .iter()
            .position(|&b| b == 0)
            .map(|p| fmt_start + p)
            .unwrap_or(fmt_start);
        let fmt = if n2 > fmt_start {
            String::from_utf8_lossy(&dec[fmt_start..n2]).into_owned()
        } else {
            "ogg".to_string()
        };

        let data_off = u32::from_le_bytes(dec[64..68].try_into().unwrap()) as usize;
        let data_sz = u32::from_le_bytes(dec[68..72].try_into().unwrap()) as usize;
        if data_off + data_sz > sound_bin.len() {
            continue;
        }

        let (dcs_id, loop_kind) = parse_sound_name(&safe_name);
        let file_name = format!("{safe_name}.{fmt}");
        let decoded_blob: Vec<u8> = sound_bin[data_off..data_off + data_sz]
            .iter()
            .map(|&b| b ^ PB2KSLIB_XOR_KEY)
            .collect();

        fs::write(out_dir.join(&file_name), &decoded_blob)?;
        sounds.push(SoundManifestEntry {
            name: safe_name,
            dcs_id,
            format: fmt,
            loop_kind,
            byte_size: data_sz as u32,
            file: file_name,
        });
    }

    let manifest = SoundManifest {
        version,
        total_entries: count,
        sounds,
    };
    let json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| RomToolError::InvalidFormat(format!("JSON error: {e}")))?;
    fs::write(out_dir.join("sound_manifest.json"), json)?;

    Ok(manifest)
}

fn parse_sound_name(name: &str) -> (Option<u16>, SoundLoopKind) {
    if name.eq_ignore_ascii_case("dcs-bong") {
        return (Some(0x003A), SoundLoopKind::OneShot);
    }
    let (stem, loop_kind) = if let Some(s) = name.strip_suffix("-LP1") {
        (s, SoundLoopKind::LoopIntro)
    } else if let Some(s) = name.strip_suffix("-LP2") {
        (s, SoundLoopKind::LoopBody)
    } else if let Some(s) = name.strip_suffix("-LP") {
        (s, SoundLoopKind::SingleLoop)
    } else {
        (name, SoundLoopKind::OneShot)
    };

    let id = stem
        .strip_prefix('S')
        .or_else(|| stem.strip_prefix('s'))
        .and_then(|hex| u16::from_str_radix(hex, 16).ok());
    (id, loop_kind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pb2kslib_sound_names() {
        assert_eq!(
            parse_sound_name("dcs-bong"),
            (Some(0x003A), SoundLoopKind::OneShot)
        );
        assert_eq!(
            parse_sound_name("S00E1-LP"),
            (Some(0x00E1), SoundLoopKind::SingleLoop)
        );
        assert_eq!(
            parse_sound_name("S0001-LP1"),
            (Some(0x0001), SoundLoopKind::LoopIntro)
        );
    }
}
