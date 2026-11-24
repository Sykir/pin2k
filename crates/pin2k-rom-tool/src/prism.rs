//! PRISM Option ROM, `BootData`, BAR3 Update Flash, and Masked Image Bank
//! parser and checksum verifier (Phase 1).
//!
//! Mirrors the exact validation checks performed at power-on by the Williams
//! Pinball 2000 PRISM PCI card (`146e:0001`).

use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use serde::Serialize;
use zip::ZipArchive;

use crate::symbols::SymbolTable;
use crate::RomToolError;

pub const PRISM_OPTION_ROM_SIZE: usize = 0x8000; // 32 KiB
pub const PRISM_BOOTDATA_SIZE: usize = 0x8000; // 32 KiB
pub const PRISM_BANK_SIZE: usize = 0x0100_0000; // 16 MiB per Mask ROM pair
pub const PRISM_MASKED_BANK0_OFFSET: usize = 0x0040_0000; // Masked art starts at +4 MiB in Bank 0
pub const BAR3_FLASH_SIZE: usize = 0x0040_0000; // 4 MiB Intel 28F320
pub const BOOTDATA_MAGIC: &[u8; 7] = b"13Dec96";
pub const WMS_PCI_VENDOR_ID: u16 = 0x146E;
pub const PRISM_PCI_DEVICE_ID: u16 = 0x0001;
pub const PRISM_PM_ENTRY_OFFSET: usize = 0x01D9;
pub const PRISM_PM_ENTRY_RAM_ADDR: u32 = 0x0008_01D9;

/// Parsed PCI Option ROM + PRISM protected-mode entry metadata from Bank 0 `[0x0000..0x8000)`.
#[derive(Debug, Clone, Serialize)]
pub struct OptionRomInfo {
    pub valid_55aa: bool,
    pub declared_size_bytes: usize,
    pub real_mode_jmp_opcode: u8,
    pub pcir_offset: usize,
    pub pci_vendor_id: u16,
    pub pci_device_id: u16,
    pub pm_far_jmp_target: Option<(u16, u32)>,
    pub pm_entry_ram_addr: u32,
    pub pm_entry_bytes_hex: String,
}

/// Checksum comparison for a single validated PRISM/BAR3 region.
#[derive(Debug, Clone, Serialize)]
pub struct ChecksumStatus {
    pub region: String,
    pub offset: usize,
    pub size_bytes: usize,
    pub expected_sum32: u32,
    pub actual_sum32: u32,
    pub ok: bool,
}

/// Parsed 32 KiB `BootData` block (located at `0x008000` in Bank 0 and `0x000000` in BAR3).
#[derive(Debug, Clone, Serialize)]
pub struct BootDataHeader {
    pub build_timestamp: String,
    pub format_version: u32,
    pub bootdata_dwords: u32,
    pub bootdata_checksum: u32,
    pub sys_image_dwords: u32,
    pub sys_image_checksum: u32,
    pub xina_version: String,
    pub game_number: u32,
    pub game_name: String,
    pub version_major: u32,
    pub version_minor: u32,
    pub game_load_addr: u32,
    pub game_dwords: u32,
    pub game_checksum: u32,
    pub symbols_dwords: u32,
    pub symbols_checksum: u32,
    pub masked_banks: [(u32, u32); 4],
    pub magic_ok: bool,
}

/// Full verification report for a reconstructed 64 MiB PRISM flat image (and optional BAR3 flash).
#[derive(Debug, Clone, Serialize)]
pub struct PrismVerifyReport {
    pub image_size: usize,
    pub option_rom: OptionRomInfo,
    pub base_bootdata: BootDataHeader,
    pub base_symbol_count: Option<usize>,
    pub update_bootdata: Option<BootDataHeader>,
    pub update_symbol_count: Option<usize>,
    pub checksums: Vec<ChecksumStatus>,
    pub all_valid: bool,
}

/// Computes the 32-bit little-endian wrapping `u32` word sum used by PRISM boot ROMs.
pub fn wrapping_sum_u32_le(data: &[u8]) -> u32 {
    let mut acc = 0u32;
    for chunk in data.chunks_exact(4) {
        let w = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        acc = acc.wrapping_add(w);
    }
    acc
}

fn read_u16_le(buf: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([buf[off], buf[off + 1]])
}

fn read_u32_le(buf: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([buf[off], buf[off + 1], buf[off + 2], buf[off + 3]])
}

/// Inspects the first 32 KiB of Bank 0 for the PCI Option ROM header and PM entry vector.
pub fn parse_option_rom(bank0: &[u8]) -> Result<OptionRomInfo, RomToolError> {
    if bank0.len() < PRISM_OPTION_ROM_SIZE {
        return Err(RomToolError::InvalidFormat(format!(
            "image too small for 32 KiB PRISM Option ROM ({} bytes)",
            bank0.len()
        )));
    }

    let valid_55aa = bank0[0] == 0x55 && bank0[1] == 0xAA;
    let declared_size_bytes = (bank0[2] as usize) * 512;
    let real_mode_jmp_opcode = bank0[3];
    let pcir_offset = read_u16_le(bank0, 0x18) as usize;

    let (pci_vendor_id, pci_device_id) =
        if pcir_offset + 8 <= PRISM_OPTION_ROM_SIZE && &bank0[pcir_offset..pcir_offset + 4] == b"PCIR"
        {
            (
                read_u16_le(bank0, pcir_offset + 4),
                read_u16_le(bank0, pcir_offset + 6),
            )
        } else {
            (0, 0)
        };

    // Check far jump `EA <off32> <seg16>` at 0x01D2 -> 0008:000801D9
    let pm_far_jmp_target = if bank0[0x01D2] == 0xEA {
        let eip = read_u32_le(bank0, 0x01D3);
        let cs = read_u16_le(bank0, 0x01D7);
        Some((cs, eip))
    } else {
        None
    };

    let prolog = &bank0[PRISM_PM_ENTRY_OFFSET..PRISM_PM_ENTRY_OFFSET + 16];
    let pm_entry_bytes_hex = prolog
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<Vec<_>>()
        .join(" ");

    Ok(OptionRomInfo {
        valid_55aa,
        declared_size_bytes,
        real_mode_jmp_opcode,
        pcir_offset,
        pci_vendor_id,
        pci_device_id,
        pm_far_jmp_target,
        pm_entry_ram_addr: PRISM_PM_ENTRY_RAM_ADDR,
        pm_entry_bytes_hex,
    })
}

/// Parses a 32 KiB `BootData` block.
pub fn parse_bootdata(block: &[u8]) -> Result<BootDataHeader, RomToolError> {
    if block.len() < 0x88 {
        return Err(RomToolError::InvalidFormat(
            "BootData block shorter than 0x88 bytes".into(),
        ));
    }

    let ts_end = block[..0x1C]
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(0x1C);
    let build_timestamp = String::from_utf8_lossy(&block[..ts_end]).into_owned();

    let format_version = read_u32_le(block, 0x20);
    let bootdata_dwords = read_u32_le(block, 0x24);
    let bootdata_checksum = read_u32_le(block, 0x28);
    let sys_image_dwords = read_u32_le(block, 0x2C);
    let sys_image_checksum = read_u32_le(block, 0x30);

    let xina_maj = String::from_utf8_lossy(&block[0x34..0x38])
        .trim_matches(|c: char| c == '\0' || c.is_whitespace())
        .to_string();
    let xina_min = String::from_utf8_lossy(&block[0x38..0x3C])
        .trim_matches(|c: char| c == '\0' || c.is_whitespace())
        .to_string();
    let xina_version = format!("{xina_maj}.{xina_min}");

    let game_number = read_u32_le(block, 0x3C);
    let game_name = match game_number {
        50069 => "Star Wars Episode I".to_string(),
        50070 => "Revenge From Mars".to_string(),
        other => format!("Unknown ({other})"),
    };

    let version_major = read_u32_le(block, 0x40);
    let version_minor = read_u32_le(block, 0x44);
    let game_load_addr = read_u32_le(block, 0x48);
    let game_dwords = read_u32_le(block, 0x4C);
    let game_checksum = read_u32_le(block, 0x50);
    let symbols_dwords = read_u32_le(block, 0x54);
    let symbols_checksum = read_u32_le(block, 0x58);

    let masked_banks = [
        (read_u32_le(block, 0x5C), read_u32_le(block, 0x60)),
        (read_u32_le(block, 0x64), read_u32_le(block, 0x68)),
        (read_u32_le(block, 0x6C), read_u32_le(block, 0x70)),
        (read_u32_le(block, 0x74), read_u32_le(block, 0x78)),
    ];

    let magic_ok = &block[0x7C..0x83] == BOOTDATA_MAGIC;

    Ok(BootDataHeader {
        build_timestamp,
        format_version,
        bootdata_dwords,
        bootdata_checksum,
        sys_image_dwords,
        sys_image_checksum,
        xina_version,
        game_number,
        game_name,
        version_major,
        version_minor,
        game_load_addr,
        game_dwords,
        game_checksum,
        symbols_dwords,
        symbols_checksum,
        masked_banks,
        magic_ok,
    })
}

fn verify_code_triplet(
    label_prefix: &str,
    image: &[u8],
    boot_off: usize,
    im_off: usize,
    bd: &BootDataHeader,
    checksums: &mut Vec<ChecksumStatus>,
) -> Option<usize> {
    // 1. BootData self-sum: wrapping sum of all `bootdata_dwords` dwords equals 1
    let bd_bytes = (bd.bootdata_dwords as usize) * 4;
    if boot_off + bd_bytes <= image.len() {
        let total = wrapping_sum_u32_le(&image[boot_off..boot_off + bd_bytes]);
        // Expected wrapping sum of whole block (including +0x28) is 0x0000_0001
        checksums.push(ChecksumStatus {
            region: format!("{label_prefix} BootData"),
            offset: boot_off,
            size_bytes: bd_bytes,
            expected_sum32: 1,
            actual_sum32: total,
            ok: total == 1,
        });
    }

    // 2. Sys image (`im_flsh0`)
    let im_bytes = (bd.sys_image_dwords as usize) * 4;
    let game_off = im_off.saturating_add(im_bytes);
    if game_off <= image.len() {
        let actual = wrapping_sum_u32_le(&image[im_off..game_off]);
        checksums.push(ChecksumStatus {
            region: format!("{label_prefix} SysImage (im_flsh0)"),
            offset: im_off,
            size_bytes: im_bytes,
            expected_sum32: bd.sys_image_checksum,
            actual_sum32: actual,
            ok: actual == bd.sys_image_checksum,
        });
    }

    // 3. Game code (`game.rom`)
    let game_bytes = (bd.game_dwords as usize) * 4;
    let sym_off = game_off.saturating_add(game_bytes);
    if sym_off <= image.len() {
        let actual = wrapping_sum_u32_le(&image[game_off..sym_off]);
        checksums.push(ChecksumStatus {
            region: format!("{label_prefix} GameCode"),
            offset: game_off,
            size_bytes: game_bytes,
            expected_sum32: bd.game_checksum,
            actual_sum32: actual,
            ok: actual == bd.game_checksum,
        });
    }

    // 4. Symbol table (`symbols.rom`)
    let sym_bytes = (bd.symbols_dwords as usize) * 4;
    let sym_end = sym_off.saturating_add(sym_bytes);
    let mut parsed_sym_count = None;
    if sym_end <= image.len() {
        let sym_slice = &image[sym_off..sym_end];
        let actual = wrapping_sum_u32_le(sym_slice);
        checksums.push(ChecksumStatus {
            region: format!("{label_prefix} Symbols"),
            offset: sym_off,
            size_bytes: sym_bytes,
            expected_sum32: bd.symbols_checksum,
            actual_sum32: actual,
            ok: actual == bd.symbols_checksum,
        });
        if let Ok(st) = SymbolTable::parse(sym_slice) {
            parsed_sym_count = Some(st.entries.len());
        }
    }

    parsed_sym_count
}

/// Verifies a reconstructed PRISM flat ROM image (16–64 MiB) and an optional
/// 4 MiB BAR3 update flash image.
pub fn verify_prism_images(
    prism_flat: &[u8],
    bar3_flash: Option<&[u8]>,
) -> Result<PrismVerifyReport, RomToolError> {
    if prism_flat.len() < 0x1_0000 {
        return Err(RomToolError::InvalidFormat(
            "PRISM flat image must be at least 64 KiB".into(),
        ));
    }

    let option_rom = parse_option_rom(prism_flat)?;
    let base_bootdata = parse_bootdata(&prism_flat[0x8000..0x1_0000])?;

    let mut checksums = Vec::new();
    let base_symbol_count = verify_code_triplet(
        "Bank0 Base",
        prism_flat,
        0x8000,
        0x1_0000,
        &base_bootdata,
        &mut checksums,
    );

    // Verify Masked Image Banks 0..3 against active bootdata (update if provided, else base)
    let (update_bootdata, update_symbol_count) = if let Some(flash) = bar3_flash {
        let ubd = parse_bootdata(flash)?;
        let sc = verify_code_triplet("BAR3 Update", flash, 0, 0x8000, &ubd, &mut checksums);
        (Some(ubd), sc)
    } else {
        (None, None)
    };

    let ref_bd = update_bootdata.as_ref().unwrap_or(&base_bootdata);
    let bank_starts = [
        PRISM_MASKED_BANK0_OFFSET,
        PRISM_BANK_SIZE,
        PRISM_BANK_SIZE * 2,
        PRISM_BANK_SIZE * 3,
    ];

    for (bank_idx, (&start, &(dwords, expected))) in
        bank_starts.iter().zip(ref_bd.masked_banks.iter()).enumerate()
    {
        if dwords == 0 {
            continue;
        }
        let len_bytes = (dwords as usize) * 4;
        if start + len_bytes <= prism_flat.len() {
            let actual = wrapping_sum_u32_le(&prism_flat[start..start + len_bytes]);
            checksums.push(ChecksumStatus {
                region: format!("Masked Image Bank {bank_idx}"),
                offset: start,
                size_bytes: len_bytes,
                expected_sum32: expected,
                actual_sum32: actual,
                ok: actual == expected,
            });
        }
    }

    let all_valid = option_rom.valid_55aa
        && option_rom.pci_vendor_id == WMS_PCI_VENDOR_ID
        && option_rom.pci_device_id == PRISM_PCI_DEVICE_ID
        && base_bootdata.magic_ok
        && update_bootdata.as_ref().is_none_or(|u| u.magic_ok)
        && checksums.iter().all(|c| c.ok);

    Ok(PrismVerifyReport {
        image_size: prism_flat.len(),
        option_rom,
        base_bootdata,
        base_symbol_count,
        update_bootdata,
        update_symbol_count,
        checksums,
        all_valid,
    })
}

/// Loaded Pinball 2000 ROM update bundle (`bootdata`, `im_flsh0`, `game`, `symbols`)
/// from either an extracted directory or an official `.exe`/`.zip` updater archive.
#[derive(Debug, Clone)]
pub struct UpdateBundle {
    pub bootdata: Vec<u8>,
    pub im_flsh0: Vec<u8>,
    pub game: Vec<u8>,
    pub symbols: Vec<u8>,
    pub flash: Vec<u8>,
    pub header: BootDataHeader,
}

fn build_flash_from_parts(
    bootdata: &[u8],
    im_flsh0: &[u8],
    game: &[u8],
    symbols: &[u8],
) -> Result<(Vec<u8>, BootDataHeader), RomToolError> {
    let total_needed = PRISM_BOOTDATA_SIZE
        .saturating_add(im_flsh0.len())
        .saturating_add(game.len())
        .saturating_add(symbols.len());

    if total_needed > BAR3_FLASH_SIZE {
        return Err(RomToolError::InvalidFormat(format!(
            "update components ({total_needed} bytes) exceed 4 MiB BAR3 flash size"
        )));
    }

    let mut flash = vec![0xFFu8; BAR3_FLASH_SIZE];
    let boot_copy = bootdata.len().min(PRISM_BOOTDATA_SIZE);
    flash[..boot_copy].copy_from_slice(&bootdata[..boot_copy]);

    let mut off = PRISM_BOOTDATA_SIZE;
    flash[off..off + im_flsh0.len()].copy_from_slice(im_flsh0);
    off += im_flsh0.len();
    flash[off..off + game.len()].copy_from_slice(game);
    off += game.len();
    flash[off..off + symbols.len()].copy_from_slice(symbols);

    let parsed = parse_bootdata(&flash[..PRISM_BOOTDATA_SIZE])?;
    Ok((flash, parsed))
}

fn find_file_in_dir_recursive(dir: &Path, suffix: &str) -> Option<Vec<u8>> {
    let entries = fs::read_dir(dir).ok()?;
    let mut subdirs = Vec::new();
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_file() {
            if let Some(name) = p.file_name().and_then(|n| n.to_str()) {
                let lower = name.to_ascii_lowercase();
                if lower.ends_with(suffix) {
                    return fs::read(&p).ok();
                }
            }
        } else if p.is_dir() {
            subdirs.push(p);
        }
    }
    for sub in subdirs {
        if let Some(bytes) = find_file_in_dir_recursive(&sub, suffix) {
            return Some(bytes);
        }
    }
    None
}

/// Loads a Pinball 2000 ROM update bundle from either a directory or an official `.exe`/`.zip` archive.
pub fn load_update_bundle(source: &Path) -> Result<UpdateBundle, RomToolError> {
    let (bootdata, im_flsh0, game, symbols) = if source.is_dir() {
        let bootdata = find_file_in_dir_recursive(source, "bootdata.rom").ok_or_else(|| {
            RomToolError::InvalidFormat(format!("missing *bootdata.rom in '{}'", source.display()))
        })?;
        let im_flsh0 = find_file_in_dir_recursive(source, "im_flsh0.rom").ok_or_else(|| {
            RomToolError::InvalidFormat(format!("missing *im_flsh0.rom in '{}'", source.display()))
        })?;
        let game = find_file_in_dir_recursive(source, "game.rom").ok_or_else(|| {
            RomToolError::InvalidFormat(format!("missing *game.rom in '{}'", source.display()))
        })?;
        let symbols = find_file_in_dir_recursive(source, "symbols.rom").ok_or_else(|| {
            RomToolError::InvalidFormat(format!("missing *symbols.rom in '{}'", source.display()))
        })?;
        (bootdata, im_flsh0, game, symbols)
    } else {
        let file = File::open(source)?;
        let mut archive = ZipArchive::new(file)
            .map_err(|e| RomToolError::InvalidFormat(format!("invalid zip/exe archive: {e}")))?;
        let read_entry = |arch: &mut ZipArchive<File>, suffix: &str| -> Result<Vec<u8>, RomToolError> {
            for idx in 0..arch.len() {
                let mut f = arch.by_index(idx).map_err(|e| {
                    RomToolError::InvalidFormat(format!("zip entry error: {e}"))
                })?;
                if f.is_dir() {
                    continue;
                }
                let lower = f.name().to_ascii_lowercase();
                if lower.ends_with(suffix) {
                    let mut buf = Vec::with_capacity(f.size() as usize);
                    f.read_to_end(&mut buf)?;
                    return Ok(buf);
                }
            }
            Err(RomToolError::InvalidFormat(format!(
                "missing '*{suffix}' inside archive '{}'",
                source.display()
            )))
        };
        let bootdata = read_entry(&mut archive, "bootdata.rom")?;
        let im_flsh0 = read_entry(&mut archive, "im_flsh0.rom")?;
        let game = read_entry(&mut archive, "game.rom")?;
        let symbols = read_entry(&mut archive, "symbols.rom")?;
        (bootdata, im_flsh0, game, symbols)
    };

    let (flash, header) = build_flash_from_parts(&bootdata, &im_flsh0, &game, &symbols)?;
    Ok(UpdateBundle {
        bootdata,
        im_flsh0,
        game,
        symbols,
        flash,
        header,
    })
}

/// Assembles a 4 MiB BAR3 Update Flash image from an update bundle directory or `.exe`/`.zip` archive.
pub fn assemble_bar3_update_dir(bundle_dir: &Path) -> Result<(Vec<u8>, BootDataHeader), RomToolError> {
    let b = load_update_bundle(bundle_dir)?;
    Ok((b.flash, b.header))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrapping_sum_u32_computes_le_dword_sum() {
        let bytes = [1, 0, 0, 0, 2, 0, 0, 0];
        assert_eq!(wrapping_sum_u32_le(&bytes), 3);
    }
}
