//! CLI entrypoint for `pin2k-rom-tool`.

mod cli;

use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;
use pin2k_rom_tool::asset_extractor::{
    extract_fonts, extract_named_animations, extract_pb2kslib_sounds, PrismMemorySpace,
};
use pin2k_rom_tool::elf_writer::{wrap_raw_x86_to_elf, ElfWrapOptions};
use pin2k_rom_tool::gamecode_extractor::extract_gamecode_script;
use pin2k_rom_tool::gfx_decoder::{
    decode_palette, decode_raw_image, render_palette_swatch, save_png, PixelFormat,
    PreviewOptions,
};
use pin2k_rom_tool::interleave::{
    crc32_ieee, discover_dcs_pair, discover_prism_pairs, interleave_buffers, reconstruct_from_zip,
    reconstruct_prism_image, InterleaveMode,
};
use pin2k_rom_tool::prism::{assemble_bar3_update_dir, load_update_bundle, verify_prism_images};
use pin2k_rom_tool::scanner::{scan_ascii_strings, scan_headers, ScanReport};
use pin2k_rom_tool::symbols::SymbolTable;

use cli::{ensure_parent_dir, parse_overlay_arg, parse_pair_arg, Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Reconstruct {
            input_dir,
            input_zip,
            prefix,
            pairs,
            flash_overlays,
            output,
            dcs_output,
            mode,
            byte_swap,
            report_json,
        } => {
            let overlays = flash_overlays
                .iter()
                .map(|s| parse_overlay_arg(s))
                .collect::<Result<Vec<_>>>()?;
            let il_mode: InterleaveMode = mode.into();

            let (flat, report, dcs_blob) = if let Some(zip_path) = input_zip {
                reconstruct_from_zip(&zip_path, prefix.as_deref(), &overlays, il_mode, byte_swap)?
            } else if !pairs.is_empty() {
                let chip_pairs = pairs
                    .iter()
                    .enumerate()
                    .map(|(i, s)| parse_pair_arg(s, i))
                    .collect::<Result<Vec<_>>>()?;
                let (f, r) = reconstruct_prism_image(&chip_pairs, &overlays, il_mode, byte_swap)?;
                (f, r, None)
            } else if let Some(ref dir) = input_dir {
                let chip_pairs = discover_prism_pairs(dir, prefix.as_deref())?;
                let (f, r) = reconstruct_prism_image(&chip_pairs, &overlays, il_mode, byte_swap)?;
                let dcs = if dcs_output.is_some() {
                    if let Some(dp) = discover_dcs_pair(dir, prefix.as_deref())? {
                        let low = fs::read(&dp.low_path)?;
                        let high = fs::read(&dp.high_path)?;
                        Some(interleave_buffers(
                            &low,
                            &high,
                            il_mode,
                            byte_swap,
                            &dp.low_path.display().to_string(),
                            &dp.high_path.display().to_string(),
                        )?)
                    } else {
                        None
                    }
                } else {
                    None
                };
                (f, r, dcs)
            } else {
                bail!("provide --input-dir <DIR>, --input-zip <ZIP>, or one or more --pair LOW:HIGH arguments");
            };

            ensure_parent_dir(&output)?;
            fs::write(&output, &flat)
                .with_context(|| format!("failed to write '{}'", output.display()))?;

            println!(
                "Reconstructed {} ({} bytes / {:.2} MiB, CRC32=0x{:08X})",
                output.display(),
                report.total_size,
                report.total_size as f64 / (1024.0 * 1024.0),
                report.crc32
            );
            for seg in &report.segments {
                println!(
                    "  [0x{:08X}..0x{:08X}] {} ({} bytes, CRC32=0x{:08X})",
                    seg.base_offset,
                    seg.base_offset + seg.size,
                    seg.label,
                    seg.size,
                    seg.crc32
                );
            }
            for (path, off, len) in &report.overlays_applied {
                println!("  [overlay @ 0x{off:08X}] {path} ({len} bytes)");
            }

            if let Some(dcs_path) = dcs_output {
                if let Some(dcs) = dcs_blob {
                    ensure_parent_dir(&dcs_path)?;
                    fs::write(&dcs_path, &dcs)?;
                    println!(
                        "Reconstructed DCS sound ROM {} ({} bytes / {:.2} MiB, CRC32=0x{:08X})",
                        dcs_path.display(),
                        dcs.len(),
                        dcs.len() as f64 / (1024.0 * 1024.0),
                        crc32_ieee(&dcs)
                    );
                } else {
                    println!("Note: DCS sound pair (U109/U110) not found; skipped --dcs-output");
                }
            }

            if let Some(json_path) = report_json {
                ensure_parent_dir(&json_path)?;
                let json = serde_json::to_string_pretty(&report)?;
                fs::write(&json_path, json)?;
            }
        }

        Commands::AssembleUpdate { bundle_dir, output } => {
            let (flash, bd) = assemble_bar3_update_dir(&bundle_dir)?;
            ensure_parent_dir(&output)?;
            fs::write(&output, &flash)?;
            println!(
                "Assembled 4 MiB BAR3 flash {} -> {} v{}.{:02} (XINA {}, built '{}', CRC32=0x{:08X})",
                output.display(),
                bd.game_name,
                bd.version_major,
                bd.version_minor,
                bd.xina_version,
                bd.build_timestamp,
                crc32_ieee(&flash)
            );
        }

        Commands::ListUpdates { dir } => {
            let mut found = Vec::new();
            let mut candidates: Vec<PathBuf> = fs::read_dir(&dir)?
                .filter_map(|e| e.ok().map(|x| x.path()))
                .collect();
            candidates.sort();
            for path in candidates {
                let is_pkg = path.is_dir()
                    || path
                        .extension()
                        .and_then(|s| s.to_str())
                        .is_some_and(|ext| {
                            ext.eq_ignore_ascii_case("exe") || ext.eq_ignore_ascii_case("zip")
                        });
                if !is_pkg {
                    continue;
                }
                if let Ok(bundle) = load_update_bundle(&path) {
                    let name = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("?")
                        .to_string();
                    found.push((name, bundle.header));
                }
            }
            println!(
                "{:<48} {:<6} {:<7} {:<10} {}",
                "PACKAGE", "GAME#", "VER", "XINA", "BUILD"
            );
            for (name, bd) in found {
                println!(
                    "{:<48} #{:<5} v{}.{:02}  {:<10} {}",
                    name,
                    bd.game_number,
                    bd.version_major,
                    bd.version_minor,
                    bd.xina_version,
                    bd.build_timestamp
                );
            }
        }

        Commands::Verify {
            input,
            bar3_flash,
            json,
        } => {
            let prism_bytes = fs::read(&input)
                .with_context(|| format!("failed to read '{}'", input.display()))?;
            let bar3_bytes = match bar3_flash.as_ref() {
                Some(p) => {
                    Some(fs::read(p).with_context(|| format!("failed to read '{}'", p.display()))?)
                }
                None => None,
            };

            let report = verify_prism_images(&prism_bytes, bar3_bytes.as_deref())?;

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("=== PRISM Option ROM (Bank 0 @ 0x0000..0x8000) ===");
                println!(
                    "  Signature 55 AA : {} (declared {} bytes)",
                    report.option_rom.valid_55aa, report.option_rom.declared_size_bytes
                );
                println!(
                    "  PCI ID (@0x{:04X}): {:04X}:{:04X}",
                    report.option_rom.pcir_offset,
                    report.option_rom.pci_vendor_id,
                    report.option_rom.pci_device_id
                );
                if let Some((cs, eip)) = report.option_rom.pm_far_jmp_target {
                    println!("  PM Entry Far Jmp: {cs:04X}:{eip:08X}");
                }
                println!(
                    "  PM Entry Prolog : {}",
                    report.option_rom.pm_entry_bytes_hex
                );

                let bd = &report.base_bootdata;
                println!("\n=== Bank 0 Base BootData (@ 0x008000) ===");
                println!(
                    "  Game            : {} (#{}), v{}.{:02}, XINA {}",
                    bd.game_name, bd.game_number, bd.version_major, bd.version_minor, bd.xina_version
                );
                println!("  Build Timestamp : {}", bd.build_timestamp);
                println!(
                    "  Game Load Addr  : 0x{:08X} ({} bytes), Symbols: {:?} entries",
                    bd.game_load_addr,
                    bd.game_dwords * 4,
                    report.base_symbol_count
                );

                if let Some(ref ubd) = report.update_bootdata {
                    println!("\n=== BAR3 Update Flash BootData (@ 0x12000000) ===");
                    println!(
                        "  Game            : {} (#{}), v{}.{:02}, XINA {}",
                        ubd.game_name,
                        ubd.game_number,
                        ubd.version_major,
                        ubd.version_minor,
                        ubd.xina_version
                    );
                    println!("  Build Timestamp : {}", ubd.build_timestamp);
                    println!(
                        "  Game Load Addr  : 0x{:08X} ({} bytes), Symbols: {:?} entries",
                        ubd.game_load_addr,
                        ubd.game_dwords * 4,
                        report.update_symbol_count
                    );
                }

                println!("\n=== Checksum Verification ({}) ===", report.checksums.len());
                for c in &report.checksums {
                    let status = if c.ok { "OK" } else { "MISMATCH" };
                    println!(
                        "  [{:<8}] {:<28} @ 0x{:08X} ({:>10} B) expected=0x{:08X} actual=0x{:08X}",
                        status, c.region, c.offset, c.size_bytes, c.expected_sum32, c.actual_sum32
                    );
                }

                println!(
                    "\nOverall Verdict: {}",
                    if report.all_valid {
                        "PASS (all headers, entry points, and checksums verified)"
                    } else {
                        "FAIL (one or more checks failed)"
                    }
                );
            }

            if !report.all_valid {
                bail!("PRISM image verification failed");
            }
        }

        Commands::Scan {
            input,
            offset,
            length,
            min_len,
            filter,
            limit,
            headers_only,
            json,
        } => {
            let rom = fs::read(&input)
                .with_context(|| format!("failed to read '{}'", input.display()))?;
            if offset >= rom.len() {
                bail!(
                    "offset 0x{offset:X} is beyond end of file ({} bytes)",
                    rom.len()
                );
            }
            let slice_end = match length {
                Some(len) => offset.saturating_add(len).min(rom.len()),
                None => rom.len(),
            };
            let slice = &rom[offset..slice_end];

            let headers = scan_headers(slice, offset);
            let strings = if headers_only {
                Vec::new()
            } else {
                scan_ascii_strings(slice, offset, min_len, filter.as_deref(), limit)
            };

            if json {
                let report = ScanReport {
                    rom_size: rom.len(),
                    base_offset: offset,
                    headers,
                    strings,
                };
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!(
                    "Scanned {} (range 0x{:08X}..0x{:08X}, {} bytes)",
                    input.display(),
                    offset,
                    slice_end,
                    slice.len()
                );
                println!("\n=== Headers & Signatures ({}) ===", headers.len());
                for h in &headers {
                    println!("  0x{:08X}  [{:<16}]  {}", h.offset, h.kind, h.detail);
                }

                if !headers_only {
                    println!("\n=== ASCII Strings ({}) ===", strings.len());
                    for s in &strings {
                        println!("  0x{:08X}  {}", s.offset, s.text);
                    }
                }
            }
        }

        Commands::Preview {
            input,
            output,
            offset,
            width,
            height,
            length,
            format,
            stride,
            big_endian,
            transparent_key,
            scale,
            palette_offset,
            palette_format,
            palette_swatch,
        } => {
            let rom = fs::read(&input)
                .with_context(|| format!("failed to read '{}'", input.display()))?;
            if offset >= rom.len() {
                bail!(
                    "offset 0x{offset:X} is beyond end of file ({} bytes)",
                    rom.len()
                );
            }

            let pixel_format: PixelFormat = format.into();
            let bpp = pixel_format.bytes_per_pixel();
            let row_stride = stride.unwrap_or((width as usize) * bpp);
            if row_stride == 0 {
                bail!("width/stride must be greater than zero");
            }

            let available = match length {
                Some(len) => len.min(rom.len() - offset),
                None => rom.len() - offset,
            };

            let resolved_height = match height {
                Some(h) => h,
                None => {
                    let h = (available / row_stride) as u32;
                    if h == 0 {
                        bail!("not enough bytes ({available}) for a single row of {row_stride} bytes");
                    }
                    h
                }
            };

            let palette = if let Some(pal_off) = palette_offset {
                if pal_off >= rom.len() {
                    bail!("palette offset 0x{pal_off:X} exceeds ROM size");
                }
                Some(decode_palette(
                    &rom[pal_off..],
                    palette_format.into(),
                    big_endian,
                ))
            } else {
                None
            };

            if let (Some(swatch_path), Some(pal)) = (palette_swatch, palette.as_ref()) {
                let swatch = render_palette_swatch(pal, 16);
                ensure_parent_dir(&swatch_path)?;
                save_png(&swatch, &swatch_path)?;
                println!("Saved palette swatch to {}", swatch_path.display());
            }

            let opts = PreviewOptions {
                width,
                height: resolved_height,
                format: pixel_format,
                stride,
                big_endian,
                transparent_key,
                scale,
            };

            let img = decode_raw_image(&rom[offset..offset + available], &opts, palette.as_ref())?;
            ensure_parent_dir(&output)?;
            save_png(&img, &output)?;
            println!(
                "Rendered {}x{} ({:?}) from 0x{:08X} -> {}",
                img.width(),
                img.height(),
                pixel_format,
                offset,
                output.display()
            );
        }

        Commands::Symbols {
            input,
            filter,
            addr,
            limit,
            json,
        } => {
            let raw = fs::read(&input)
                .with_context(|| format!("failed to read '{}'", input.display()))?;
            let table = SymbolTable::parse(&raw)?;

            if let Some(target) = addr {
                match table.resolve_address(target) {
                    Some((sym, 0)) => {
                        println!("0x{target:08X} -> 0x{:08X} {}", sym.address, sym.name)
                    }
                    Some((sym, delta)) => println!(
                        "0x{target:08X} -> 0x{:08X} {} + 0x{delta:X}",
                        sym.address, sym.name
                    ),
                    None => println!("0x{target:08X} -> <no symbol>"),
                }
                return Ok(());
            }

            let mut matched: Vec<_> = match filter.as_deref() {
                Some(needle) => table.filter_contains(needle),
                None => table.entries.iter().collect(),
            };
            if let Some(max) = limit {
                matched.truncate(max);
            }

            if json {
                println!("{}", serde_json::to_string_pretty(&matched)?);
            } else {
                println!(
                    "Symbol table {} ({} total entries, showing {})",
                    input.display(),
                    table.entries.len(),
                    matched.len()
                );
                for sym in matched {
                    println!("  0x{:08X}  {}", sym.address, sym.name);
                }
            }
        }

        Commands::WrapElf {
            game,
            symbols,
            output,
            load_addr,
            entry,
            etext,
        } => {
            let game_bytes = fs::read(&game)
                .with_context(|| format!("failed to read '{}'", game.display()))?;
            let symtab = match symbols.as_ref() {
                Some(sym_path) => {
                    let raw = fs::read(sym_path)
                        .with_context(|| format!("failed to read '{}'", sym_path.display()))?;
                    Some(SymbolTable::parse(&raw)?)
                }
                None => None,
            };

            let opts = ElfWrapOptions {
                load_addr,
                entry_addr: entry.unwrap_or(load_addr),
                etext_addr: etext,
                ..ElfWrapOptions::default()
            };

            let (elf, summary) = wrap_raw_x86_to_elf(&game_bytes, symtab.as_ref(), &opts)?;
            ensure_parent_dir(&output)?;
            fs::write(&output, &elf)
                .with_context(|| format!("failed to write '{}'", output.display()))?;

            println!(
                "Wrote ELF32 {} ({} bytes): load=0x{:08X}, entry=0x{:08X}, .text=0x{:X}, .data=0x{:X}, .bss=0x{:X}, symbols={}",
                output.display(),
                summary.total_elf_bytes,
                summary.load_addr,
                summary.entry_addr,
                summary.text_size,
                summary.data_size,
                summary.bss_size,
                summary.symbol_count
            );
        }

        Commands::ExtractAssets {
            prism_flat,
            update,
            bar3_flash,
            game,
            symbols,
            output_dir,
            anim_filter,
            max_anims,
            max_frames,
            sound_bin,
            max_sounds,
        } => {
            fs::create_dir_all(&output_dir)?;

            if let Some(prism_path) = prism_flat.as_ref() {
                let prism_bytes = fs::read(prism_path)?;
                let (game_bytes, sym_bytes, bar3_bytes) = if let Some(upd_path) = update.as_ref() {
                    let b = load_update_bundle(upd_path)?;
                    println!(
                        "Using ROM update {} v{}.{:02} (XINA {}, built '{}')",
                        b.header.game_name,
                        b.header.version_major,
                        b.header.version_minor,
                        b.header.xina_version,
                        b.header.build_timestamp
                    );
                    (b.game, b.symbols, Some(b.flash))
                } else if let (Some(game_path), Some(sym_path)) = (game.as_ref(), symbols.as_ref()) {
                    let b3 = match bar3_flash.as_ref() {
                        Some(p) => Some(fs::read(p)?),
                        None => None,
                    };
                    (fs::read(game_path)?, fs::read(sym_path)?, b3)
                } else {
                    bail!("provide either `--update <DIR|.exe|.zip>` or both `--game` and `--symbols`");
                };

                let symtab = SymbolTable::parse(&sym_bytes)?;
                let mem = PrismMemorySpace {
                    game_rom: &game_bytes,
                    game_load_addr: 0x0010_0000,
                    prism_flat: &prism_bytes,
                    bar3_flash: bar3_bytes.as_deref(),
                };
                let gfx_dir = output_dir.join("gfx");
                let manifest = extract_named_animations(
                    &mem,
                    &symtab,
                    &gfx_dir,
                    anim_filter.as_deref(),
                    max_anims,
                    max_frames,
                )?;
                println!(
                    "Extracted {} / {} discovered animations -> {}/gfx_manifest.json",
                    manifest.animations_exported,
                    manifest.total_animations_discovered,
                    gfx_dir.display()
                );

                let fonts_dir = output_dir.join("fonts");
                let fonts_manifest = extract_fonts(&mem, &symtab, &fonts_dir)?;
                println!(
                    "Extracted {} bitmap fonts -> {}/fonts_manifest.json",
                    fonts_manifest.fonts_exported,
                    fonts_dir.display()
                );
            }

            if let Some(snd_path) = sound_bin {
                let snd_bytes = fs::read(&snd_path)?;
                let snd_dir = output_dir.join("audio");
                let snd_manifest = extract_pb2kslib_sounds(&snd_bytes, &snd_dir, max_sounds)?;
                println!(
                    "Extracted {} / {} audio tracks -> {}/sound_manifest.json",
                    snd_manifest.sounds.len(),
                    snd_manifest.total_entries,
                    snd_dir.display()
                );
            }
        }

        Commands::ExtractGamecode {
            update,
            game,
            symbols,
            prism_flat,
            output_dir,
        } => {
            let (game_bytes, sym_bytes, header, bar3_bytes) = if let Some(upd_path) = update.as_ref()
            {
                let b = load_update_bundle(upd_path)?;
                (b.game, b.symbols, Some(b.header), Some(b.flash))
            } else if let (Some(game_path), Some(sym_path)) = (game.as_ref(), symbols.as_ref()) {
                (fs::read(game_path)?, fs::read(sym_path)?, None, None)
            } else {
                bail!("provide either `--update <DIR|.exe|.zip>` or both `--game` and `--symbols`");
            };

            let prism_bytes = match prism_flat.as_ref() {
                Some(p) => fs::read(p)?,
                None => Vec::new(),
            };
            let symtab = SymbolTable::parse(&sym_bytes)?;
            let mem = PrismMemorySpace {
                game_rom: &game_bytes,
                game_load_addr: 0x0010_0000,
                prism_flat: &prism_bytes,
                bar3_flash: bar3_bytes.as_deref(),
            };
            let script = extract_gamecode_script(&mem, &symtab, header.as_ref(), &output_dir)?;
            println!(
                "Extracted gamecode script for {} (#{} v{}) -> {}/game_script.json (switches={}, drives={}, lamps={}, attract={}, modes={}, rules={})",
                script.meta.title,
                script.meta.game_number,
                script.meta.version,
                output_dir.display(),
                script.hardware.switches.len(),
                script.hardware.drives.len(),
                script.hardware.lamps.len(),
                script.attract.len(),
                script.modes.len(),
                script.rules.len()
            );
        }
    }

    Ok(())
}
