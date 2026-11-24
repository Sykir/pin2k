//! PRISM Mask ROM & DCS Sound ROM interleaver (from directories, explicit pairs, or MAME `.zip`).
//!
//! Pinball 2000's PRISM daughterboard pairs 16-bit wide Mask ROMs (`U100/U101`,
//! `U102/U103`, `U104/U105`, `U106/U107`) onto a 32-bit little-endian memory bus,
//! and pairs `U109/U110` into an 8 MiB DCS-2 sound ROM bank.

use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

use serde::Serialize;
use zip::ZipArchive;

use crate::RomToolError;

/// Standard PRISM game Mask ROM pairs in ascending memory order (`BAR5` banks 0..3).
pub const STANDARD_PRISM_PAIRS: [(&str, &str); 4] = [
    ("u100", "u101"),
    ("u102", "u103"),
    ("u104", "u105"),
    ("u106", "u107"),
];

/// DCS-2 sound Mask ROM pair (`Expansion ROM` / `CS3`).
pub const DCS_ROM_PAIR: (&str, &str) = ("u109", "u110");

/// How two ROM chips in a pair are interleaved into the flat memory image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterleaveMode {
    /// 16-bit word interleaving: 2 bytes from low chip (bits 0..15) + 2 bytes from high chip (bits 16..31).
    Word16,
    /// 8-bit byte interleaving: 1 byte from low chip + 1 byte from high chip.
    Byte8,
    /// Direct concatenation (low chip followed by high chip).
    Concat,
}

/// A pair of low/high ROM chip files on disk.
#[derive(Debug, Clone)]
pub struct ChipPair {
    pub label: String,
    pub low_path: PathBuf,
    pub high_path: PathBuf,
}

/// Optional Flash ROM update overlay patched at a target offset.
#[derive(Debug, Clone)]
pub struct FlashOverlay {
    pub path: PathBuf,
    pub offset: usize,
}

/// Metadata recorded for each reconstructed ROM pair.
#[derive(Debug, Clone, Serialize)]
pub struct PairSegmentInfo {
    pub label: String,
    pub low_file: String,
    pub high_file: String,
    pub base_offset: usize,
    pub size: usize,
    pub crc32: u32,
}

/// Summary report emitted after reconstructing a flat PRISM ROM image.
#[derive(Debug, Clone, Serialize)]
pub struct ReconstructionReport {
    pub total_size: usize,
    pub crc32: u32,
    pub segments: Vec<PairSegmentInfo>,
    pub overlays_applied: Vec<(String, usize, usize)>,
}

/// Interleaves two equal-length chip buffers into a single contiguous output vector.
pub fn interleave_buffers(
    low: &[u8],
    high: &[u8],
    mode: InterleaveMode,
    swap_16bit_bytes: bool,
    low_name: &str,
    high_name: &str,
) -> Result<Vec<u8>, RomToolError> {
    if low.len() != high.len() {
        return Err(RomToolError::PairSizeMismatch {
            low_name: low_name.to_string(),
            low_len: low.len(),
            high_name: high_name.to_string(),
            high_len: high.len(),
        });
    }

    let total = low.len() + high.len();
    let mut out = Vec::with_capacity(total);

    match mode {
        InterleaveMode::Word16 => {
            if low.len() % 2 != 0 {
                return Err(RomToolError::UnalignedChipSize {
                    size: low.len(),
                    alignment: 2,
                });
            }
            for (l_word, h_word) in low.chunks_exact(2).zip(high.chunks_exact(2)) {
                if swap_16bit_bytes {
                    out.extend_from_slice(&[l_word[1], l_word[0], h_word[1], h_word[0]]);
                } else {
                    out.extend_from_slice(&[l_word[0], l_word[1], h_word[0], h_word[1]]);
                }
            }
        }
        InterleaveMode::Byte8 => {
            for (&l_byte, &h_byte) in low.iter().zip(high.iter()) {
                out.push(l_byte);
                out.push(h_byte);
            }
        }
        InterleaveMode::Concat => {
            if swap_16bit_bytes {
                if low.len() % 2 != 0 {
                    return Err(RomToolError::UnalignedChipSize {
                        size: low.len(),
                        alignment: 2,
                    });
                }
                for w in low.chunks_exact(2) {
                    out.extend_from_slice(&[w[1], w[0]]);
                }
                for w in high.chunks_exact(2) {
                    out.extend_from_slice(&[w[1], w[0]]);
                }
            } else {
                out.extend_from_slice(low);
                out.extend_from_slice(high);
            }
        }
    }

    Ok(out)
}

fn list_matching_files(dir: &Path, prefix: Option<&str>) -> Result<Vec<(String, PathBuf)>, RomToolError> {
    let prefix_lower = prefix.map(|p| p.to_ascii_lowercase());
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                let lower = name.to_ascii_lowercase();
                if let Some(ref p) = prefix_lower {
                    if !lower.contains(p) {
                        continue;
                    }
                }
                files.push((lower, path));
            }
        }
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}

/// Auto-discovers standard PRISM chip pairs (`u100`/`u101` .. `u106`/`u107`) in `dir`.
pub fn discover_prism_pairs(
    dir: &Path,
    prefix: Option<&str>,
) -> Result<Vec<ChipPair>, RomToolError> {
    let files = list_matching_files(dir, prefix)?;
    let mut pairs = Vec::new();
    for (low_tag, high_tag) in STANDARD_PRISM_PAIRS {
        let low = files.iter().find(|(name, _)| name.contains(low_tag));
        let high = files.iter().find(|(name, _)| name.contains(high_tag));

        if let (Some((_, low_path)), Some((_, high_path))) = (low, high) {
            pairs.push(ChipPair {
                label: format!("{}/{}", low_tag.to_uppercase(), high_tag.to_uppercase()),
                low_path: low_path.clone(),
                high_path: high_path.clone(),
            });
        }
    }

    if pairs.is_empty() {
        return Err(RomToolError::NoChipPairsFound(dir.display().to_string()));
    }

    Ok(pairs)
}

/// Auto-discovers the DCS-2 sound ROM pair (`u109`/`u110`) in `dir` if present.
pub fn discover_dcs_pair(dir: &Path, prefix: Option<&str>) -> Result<Option<ChipPair>, RomToolError> {
    let files = list_matching_files(dir, prefix)?;
    let (low_tag, high_tag) = DCS_ROM_PAIR;
    let low = files.iter().find(|(name, _)| name.contains(low_tag));
    let high = files.iter().find(|(name, _)| name.contains(high_tag));

    if let (Some((_, low_path)), Some((_, high_path))) = (low, high) {
        Ok(Some(ChipPair {
            label: "U109/U110 (DCS)".to_string(),
            low_path: low_path.clone(),
            high_path: high_path.clone(),
        }))
    } else {
        Ok(None)
    }
}

/// Reconstructs a unified flat binary from chip pairs on disk and optional Flash overlays.
pub fn reconstruct_prism_image(
    pairs: &[ChipPair],
    overlays: &[FlashOverlay],
    mode: InterleaveMode,
    swap_16bit_bytes: bool,
) -> Result<(Vec<u8>, ReconstructionReport), RomToolError> {
    let mut loaded_pairs = Vec::with_capacity(pairs.len());
    for pair in pairs {
        let low = fs::read(&pair.low_path)?;
        let high = fs::read(&pair.high_path)?;
        loaded_pairs.push((
            pair.label.clone(),
            pair.low_path.display().to_string(),
            low,
            pair.high_path.display().to_string(),
            high,
        ));
    }
    reconstruct_from_named_buffers(&loaded_pairs, overlays, mode, swap_16bit_bytes)
}

/// Reconstructs a unified flat PRISM binary (and optional DCS sound ROM) directly from a MAME `.zip`.
pub fn reconstruct_from_zip(
    zip_path: &Path,
    prefix: Option<&str>,
    overlays: &[FlashOverlay],
    mode: InterleaveMode,
    swap_16bit_bytes: bool,
) -> Result<(Vec<u8>, ReconstructionReport, Option<Vec<u8>>), RomToolError> {
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)
        .map_err(|e| RomToolError::InvalidFormat(format!("invalid zip '{}': {e}", zip_path.display())))?;

    let prefix_lower = prefix.map(|p| p.to_ascii_lowercase());
    let mut entries: Vec<(String, String, Vec<u8>)> = Vec::new();

    for i in 0..archive.len() {
        let mut zf = archive
            .by_index(i)
            .map_err(|e| RomToolError::InvalidFormat(format!("zip entry error: {e}")))?;
        if zf.is_dir() {
            continue;
        }
        let full_name = zf.name().to_string();
        let base_lower = Path::new(&full_name)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&full_name)
            .to_ascii_lowercase();

        if let Some(ref p) = prefix_lower {
            if !base_lower.contains(p) {
                continue;
            }
        }
        let mut buf = Vec::with_capacity(zf.size() as usize);
        zf.read_to_end(&mut buf)?;
        entries.push((base_lower, full_name, buf));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut pairs = Vec::new();
    for (low_tag, high_tag) in STANDARD_PRISM_PAIRS {
        let low = entries.iter().find(|(name, _, _)| name.contains(low_tag));
        let high = entries.iter().find(|(name, _, _)| name.contains(high_tag));
        if let (Some((_, low_name, low_buf)), Some((_, high_name, high_buf))) = (low, high) {
            pairs.push((
                format!("{}/{}", low_tag.to_uppercase(), high_tag.to_uppercase()),
                low_name.clone(),
                low_buf.clone(),
                high_name.clone(),
                high_buf.clone(),
            ));
        }
    }

    if pairs.is_empty() {
        return Err(RomToolError::NoChipPairsFound(zip_path.display().to_string()));
    }

    let (flat, report) = reconstruct_from_named_buffers(&pairs, overlays, mode, swap_16bit_bytes)?;

    // Also extract U109/U110 DCS sound ROM if present in the zip
    let dcs = {
        let (low_tag, high_tag) = DCS_ROM_PAIR;
        let low = entries.iter().find(|(name, _, _)| name.contains(low_tag));
        let high = entries.iter().find(|(name, _, _)| name.contains(high_tag));
        if let (Some((_, low_name, low_buf)), Some((_, high_name, high_buf))) = (low, high) {
            Some(interleave_buffers(
                low_buf,
                high_buf,
                mode,
                swap_16bit_bytes,
                low_name,
                high_name,
            )?)
        } else {
            None
        }
    };

    Ok((flat, report, dcs))
}

fn reconstruct_from_named_buffers(
    pairs: &[(String, String, Vec<u8>, String, Vec<u8>)],
    overlays: &[FlashOverlay],
    mode: InterleaveMode,
    swap_16bit_bytes: bool,
) -> Result<(Vec<u8>, ReconstructionReport), RomToolError> {
    let mut flat = Vec::new();
    let mut segments = Vec::with_capacity(pairs.len());

    for (label, low_name, low, high_name, high) in pairs {
        let base_offset = flat.len();
        let interleaved =
            interleave_buffers(low, high, mode, swap_16bit_bytes, low_name, high_name)?;
        let seg_crc = crc32_ieee(&interleaved);
        let seg_size = interleaved.len();

        flat.extend_from_slice(&interleaved);
        segments.push(PairSegmentInfo {
            label: label.clone(),
            low_file: low_name.clone(),
            high_file: high_name.clone(),
            base_offset,
            size: seg_size,
            crc32: seg_crc,
        });
    }

    let mut overlays_applied = Vec::new();
    for overlay in overlays {
        let data = fs::read(&overlay.path)?;
        let end = overlay.offset.saturating_add(data.len());
        if end > flat.len() {
            flat.resize(end, 0xFF);
        }
        flat[overlay.offset..end].copy_from_slice(&data);
        overlays_applied.push((overlay.path.display().to_string(), overlay.offset, data.len()));
    }

    let report = ReconstructionReport {
        total_size: flat.len(),
        crc32: crc32_ieee(&flat),
        segments,
        overlays_applied,
    };

    Ok((flat, report))
}

/// Standard IEEE 802.3 CRC32 checksum.
pub fn crc32_ieee(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};
    use zip::write::SimpleFileOptions;
    use zip::ZipWriter;

    #[test]
    fn word16_interleave_builds_32bit_le_words() {
        let low = [0x11, 0x22, 0x55, 0x66];
        let high = [0x33, 0x44, 0x77, 0x88];
        let out = interleave_buffers(&low, &high, InterleaveMode::Word16, false, "u100", "u101")
            .unwrap();
        assert_eq!(out, vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
    }

    #[test]
    fn word16_interleave_with_byte_swap() {
        let low = [0x22, 0x11];
        let high = [0x44, 0x33];
        let out = interleave_buffers(&low, &high, InterleaveMode::Word16, true, "u100", "u101")
            .unwrap();
        assert_eq!(out, vec![0x11, 0x22, 0x33, 0x44]);
    }

    #[test]
    fn mismatched_pair_sizes_error() {
        let err =
            interleave_buffers(&[0, 1], &[0, 1, 2, 3], InterleaveMode::Word16, false, "a", "b")
                .unwrap_err();
        assert!(matches!(err, RomToolError::PairSizeMismatch { .. }));
    }

    #[test]
    fn crc32_matches_known_vector() {
        assert_eq!(crc32_ieee(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn reconstructs_from_mame_style_zip() {
        let tmp_path = std::env::temp_dir().join("pin2k_test_rom.zip");
        {
            let mut buf = Cursor::new(Vec::new());
            {
                let mut zw = ZipWriter::new(&mut buf);
                let opts = SimpleFileOptions::default();
                zw.start_file("swe1_u100.rom", opts).unwrap();
                zw.write_all(&[0xAA, 0x55]).unwrap();
                zw.start_file("swe1_u101.rom", opts).unwrap();
                zw.write_all(&[0x12, 0x34]).unwrap();
                zw.finish().unwrap();
            }
            fs::write(&tmp_path, buf.into_inner()).unwrap();
        }
        let (flat, report, dcs) =
            reconstruct_from_zip(&tmp_path, Some("swe1"), &[], InterleaveMode::Word16, false)
                .unwrap();
        let _ = fs::remove_file(&tmp_path);
        assert_eq!(flat, vec![0xAA, 0x55, 0x12, 0x34]);
        assert_eq!(report.segments.len(), 1);
        assert!(dcs.is_none());
    }
}
