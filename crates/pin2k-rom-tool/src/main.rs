//! CLI entrypoint for `pin2k-rom-tool`.

use std::fs;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use pin2k_rom_tool::asset_extractor::{
    extract_fonts, extract_named_animations, extract_pb2kslib_sounds, PrismMemorySpace,
};
use pin2k_rom_tool::elf_writer::{wrap_raw_x86_to_elf, ElfWrapOptions};
use pin2k_rom_tool::gamecode_extractor::extract_gamecode_script;
use pin2k_rom_tool::gfx_decoder::{
    decode_palette, decode_raw_image, render_palette_swatch, save_png, PaletteFormat, PixelFormat,
    PreviewOptions,
};
use pin2k_rom_tool::interleave::{
    crc32_ieee, discover_dcs_pair, discover_prism_pairs, interleave_buffers, reconstruct_from_zip,
    reconstruct_prism_image, ChipPair, FlashOverlay, InterleaveMode,
};
use pin2k_rom_tool::prism::{assemble_bar3_update_dir, load_update_bundle, verify_prism_images};
use pin2k_rom_tool::scanner::{scan_ascii_strings, scan_headers, ScanReport};
use pin2k_rom_tool::symbols::SymbolTable;
use pin2k_rom_tool::{parse_hex_or_dec, parse_hex_or_dec_u16, parse_hex_or_dec_u32};

#[derive(Debug, Parser)]
#[command(
    name = "pin2k-rom-tool",
    version,
    about = "Pinball 2000 PRISM ROM reconstructor, BAR3 flash assembler, verifier, scanner, and sprite previewer"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Interleave PRISM Mask ROM pairs (U100/U101..U106/U107) from a directory, MAME `.zip`, or pairs.
    Reconstruct {
        /// Directory containing PRISM ROM dumps (auto-discovers u100/u101..u106/u107).
        #[arg(long)]
        input_dir: Option<PathBuf>,

        /// MAME-style `.zip` archive containing PRISM ROM dumps.
        #[arg(long)]
        input_zip: Option<PathBuf>,

        /// Optional filename prefix filter when using `--input-dir` or `--input-zip` (e.g. `swe1` or `rfm`).
        #[arg(long)]
        prefix: Option<String>,

        /// Explicit chip pair in `LOW:HIGH` path format (may be repeated).
        #[arg(long = "pair", value_name = "LOW:HIGH")]
        pairs: Vec<String>,

        /// Optional Flash update overlay in `PATH@OFFSET` format (hex `0x...` or decimal).
        #[arg(long = "flash-overlay", value_name = "PATH@OFFSET")]
        flash_overlays: Vec<String>,

        /// Output path for the unified flat PRISM binary (e.g. `prism_flat.bin`).
        #[arg(short, long)]
        output: PathBuf,

        /// Optional output path for the reconstructed 8 MiB DCS-2 sound ROM (`u109`/`u110`).
        #[arg(long)]
        dcs_output: Option<PathBuf>,

        /// Interleave width between low and high chips.
        #[arg(long, value_enum, default_value_t = CliInterleaveMode::Word16)]
        mode: CliInterleaveMode,

        /// Swap bytes within each 16-bit chip word before interleaving.
        #[arg(long, default_value_t = false)]
        byte_swap: bool,

        /// Optional JSON report output path.
        #[arg(long)]
        report_json: Option<PathBuf>,
    },

    /// Assemble a 4 MiB BAR3 Update Flash image (`0x12000000`) from an update directory or `.exe`/`.zip` archive.
    AssembleUpdate {
        /// Directory or official `.exe`/`.zip` update archive containing `bootdata`, `im_flsh0`, `game`, and `symbols`.
        #[arg(short, long)]
        bundle_dir: PathBuf,

        /// Output path for the 4 MiB BAR3 flash binary (e.g. `swe1_bar3.flash`).
        #[arg(short, long)]
        output: PathBuf,
    },

    /// Scan a directory of Pinball 2000 ROM updates (`.exe`, `.zip`, or extracted folders) and show versions/checksums.
    ListUpdates {
        /// Directory containing update packages (`.exe`, `.zip`, or subdirectories).
        #[arg(short, long)]
        dir: PathBuf,
    },

    /// Verify PRISM Option ROM entry points, `BootData` headers, symbol tables, and all bank/flash checksums.
    Verify {
        /// Path to the reconstructed flat PRISM image (`prism_flat.bin`).
        #[arg(short, long)]
        input: PathBuf,

        /// Optional 4 MiB BAR3 update flash image (`swe1_bar3.flash`) to verify alongside Bank 0..3.
        #[arg(long)]
        bar3_flash: Option<PathBuf>,

        /// Emit JSON verification report.
        #[arg(long, default_value_t = false)]
        json: bool,
    },

    /// Scan a ROM image for printable ASCII strings and executable/asset headers.
    Scan {
        /// Path to the flat ROM binary (`prism_flat.bin`).
        #[arg(short, long)]
        input: PathBuf,

        /// Start offset within the file (hex `0x...` or decimal).
        #[arg(long, default_value = "0", value_parser = parse_hex_or_dec)]
        offset: usize,

        /// Maximum bytes to scan from `offset` (omit to scan to EOF).
        #[arg(long, value_parser = parse_hex_or_dec)]
        length: Option<usize>,

        /// Minimum printable ASCII string length.
        #[arg(long, default_value_t = 6)]
        min_len: usize,

        /// Case-insensitive substring filter for ASCII strings.
        #[arg(long)]
        filter: Option<String>,

        /// Maximum number of ASCII strings to print.
        #[arg(long)]
        limit: Option<usize>,

        /// Skip ASCII string scanning and only report binary headers.
        #[arg(long, default_value_t = false)]
        headers_only: bool,

        /// Emit machine-readable JSON output.
        #[arg(long, default_value_t = false)]
        json: bool,
    },

    /// Render a raw RGB555/RGB565/Indexed8 byte slice from a ROM image to PNG.
    Preview {
        /// Path to the ROM binary (`prism_flat.bin`).
        #[arg(short, long)]
        input: PathBuf,

        /// Output PNG path.
        #[arg(short, long)]
        output: PathBuf,

        /// Byte offset of the candidate graphic in the ROM (hex `0x...` or decimal).
        #[arg(long, default_value = "0", value_parser = parse_hex_or_dec)]
        offset: usize,

        /// Image width in pixels.
        #[arg(short, long)]
        width: u32,

        /// Image height in pixels (if omitted, computed from `--length` or remaining bytes).
        #[arg(short = 'H', long)]
        height: Option<u32>,

        /// Optional byte length to read (used to infer height when `--height` is omitted).
        #[arg(long, value_parser = parse_hex_or_dec)]
        length: Option<usize>,

        /// Pixel format of the candidate graphic (Pinball 2000 native framebuffer is `rgb555`).
        #[arg(long, value_enum, default_value_t = CliPixelFormat::Rgb555)]
        format: CliPixelFormat,

        /// Optional row stride in bytes (defaults to `width * bytes_per_pixel`).
        #[arg(long, value_parser = parse_hex_or_dec)]
        stride: Option<usize>,

        /// Interpret 16-bit pixels as big-endian instead of little-endian.
        #[arg(long, default_value_t = false)]
        big_endian: bool,

        /// Optional transparent color/index key (Pinball 2000 RGB555 key is `0x7C1F`).
        #[arg(long, value_parser = parse_hex_or_dec_u16)]
        transparent_key: Option<u16>,

        /// Nearest-neighbor upscale factor for visual inspection.
        #[arg(long, default_value_t = 1)]
        scale: u32,

        /// Optional ROM offset for a 256-color palette (used with `--format indexed8`).
        #[arg(long, value_parser = parse_hex_or_dec)]
        palette_offset: Option<usize>,

        /// Palette entry format when `--palette-offset` is supplied.
        #[arg(long, value_enum, default_value_t = CliPaletteFormat::Rgb555)]
        palette_format: CliPaletteFormat,

        /// Optional path to also save a 16×16 palette swatch PNG.
        #[arg(long)]
        palette_swatch: Option<PathBuf>,
    },

    /// Inspect, search, or resolve addresses in a Pinball 2000 `*_symbols.rom` file.
    Symbols {
        /// Path to `*_symbols.rom`.
        #[arg(short, long)]
        input: PathBuf,

        /// Case-insensitive substring filter on symbol names.
        #[arg(long)]
        filter: Option<String>,

        /// Resolve a virtual address (hex `0x...` or decimal) to the nearest symbol.
        #[arg(long, value_parser = parse_hex_or_dec_u32)]
        addr: Option<u32>,

        /// Maximum number of symbols to print.
        #[arg(long)]
        limit: Option<usize>,

        /// Emit JSON output.
        #[arg(long, default_value_t = false)]
        json: bool,
    },

    /// Wrap a raw Pinball 2000 `*_game.rom` (+ optional `*_symbols.rom`) into a 32-bit x86 ELF
    /// ready for Ghidra, IDA, objdump, or REA (`rea-agents`).
    WrapElf {
        /// Path to the raw 32-bit x86 `*_game.rom` binary.
        #[arg(short, long)]
        game: PathBuf,

        /// Optional path to the matching `*_symbols.rom` symbol table.
        #[arg(short, long)]
        symbols: Option<PathBuf>,

        /// Output ELF32 file path (e.g. `swe1_game.elf`).
        #[arg(short, long)]
        output: PathBuf,

        /// Virtual load address of byte 0 of `game.rom` (default: `0x00100000`).
        #[arg(long, default_value = "0x00100000", value_parser = parse_hex_or_dec_u32)]
        load_addr: u32,

        /// Entry point address (defaults to `--load-addr`).
        #[arg(long, value_parser = parse_hex_or_dec_u32)]
        entry: Option<u32>,

        /// Explicit `.text` end address (auto-detected from `etext` symbol when omitted).
        #[arg(long, value_parser = parse_hex_or_dec_u32)]
        etext: Option<u32>,
    },

    /// Extract Pinball 2000 animations/sprites (`.png` + `gfx_manifest.json`) and/or
    /// DCS audio tracks (`.ogg` + `sound_manifest.json`).
    ExtractAssets {
        /// Reconstructed 64 MiB PRISM flat ROM (`swe1_prism_flat.bin`).
        #[arg(long)]
        prism_flat: Option<PathBuf>,

        /// Optional Pinball 2000 ROM update package (directory, `.exe`, or `.zip`) supplying `game`, `symbols`, and `BAR3` flash automatically.
        #[arg(long)]
        update: Option<PathBuf>,

        /// Optional 4 MiB BAR3 update flash (`swe1_bar3_0200.flash`).
        #[arg(long)]
        bar3_flash: Option<PathBuf>,

        /// Path to `*_game.rom` (used with `--symbols` to locate named animations).
        #[arg(long)]
        game: Option<PathBuf>,

        /// Path to `*_symbols.rom`.
        #[arg(long)]
        symbols: Option<PathBuf>,

        /// Output directory for extracted assets.
        #[arg(short, long)]
        output_dir: PathBuf,

        /// Optional substring filter on animation symbol names (e.g. `jedi` or `podrace`).
        #[arg(long)]
        anim_filter: Option<String>,

        /// Optional cap on number of animations to export.
        #[arg(long)]
        max_anims: Option<usize>,

        /// Maximum frames to decode per animation (omit to extract ALL frames).
        #[arg(long)]
        max_frames: Option<usize>,

        /// Optional path to `{game}_sound.bin` (`pb2kslib` container) to extract `.ogg` audio.
        #[arg(long)]
        sound_bin: Option<PathBuf>,

        /// Optional cap on number of audio tracks to extract.
        #[arg(long)]
        max_sounds: Option<usize>,
    },

    /// Extract hardware tables, switch handlers, attract state machine, and game logic into `game_script.json`.
    ExtractGamecode {
        /// Optional Pinball 2000 ROM update package (directory, `.exe`, or `.zip`).
        #[arg(long)]
        update: Option<PathBuf>,

        /// Path to `*_game.rom` (when `--update` is not provided).
        #[arg(long)]
        game: Option<PathBuf>,

        /// Path to `*_symbols.rom` (when `--update` is not provided).
        #[arg(long)]
        symbols: Option<PathBuf>,

        /// Optional reconstructed 64 MiB PRISM flat ROM (`prism_flat.bin`).
        #[arg(long)]
        prism_flat: Option<PathBuf>,

        /// Output directory for `game_script.json`.
        #[arg(short, long)]
        output_dir: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliInterleaveMode {
    Word16,
    Byte8,
    Concat,
}

impl From<CliInterleaveMode> for InterleaveMode {
    fn from(value: CliInterleaveMode) -> Self {
        match value {
            CliInterleaveMode::Word16 => InterleaveMode::Word16,
            CliInterleaveMode::Byte8 => InterleaveMode::Byte8,
            CliInterleaveMode::Concat => InterleaveMode::Concat,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliPixelFormat {
    Rgb565,
    Rgb555,
    Bgr565,
    Bgr555,
    Indexed8,
}

impl From<CliPixelFormat> for PixelFormat {
    fn from(value: CliPixelFormat) -> Self {
        match value {
            CliPixelFormat::Rgb565 => PixelFormat::Rgb565,
            CliPixelFormat::Rgb555 => PixelFormat::Rgb555,
            CliPixelFormat::Bgr565 => PixelFormat::Bgr565,
            CliPixelFormat::Bgr555 => PixelFormat::Bgr555,
            CliPixelFormat::Indexed8 => PixelFormat::Indexed8,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliPaletteFormat {
    Rgb565,
    Rgb555,
    Rgb888,
}

impl From<CliPaletteFormat> for PaletteFormat {
    fn from(value: CliPaletteFormat) -> Self {
        match value {
            CliPaletteFormat::Rgb565 => PaletteFormat::Rgb565,
            CliPaletteFormat::Rgb555 => PaletteFormat::Rgb555,
            CliPaletteFormat::Rgb888 => PaletteFormat::Rgb888,
        }
    }
}

fn parse_pair_arg(spec: &str, idx: usize) -> Result<ChipPair> {
    let (low, high) = spec
        .split_once(':')
        .with_context(|| format!("expected LOW:HIGH pair, got '{spec}'"))?;
    Ok(ChipPair {
        label: format!("PAIR_{idx}"),
        low_path: PathBuf::from(low),
        high_path: PathBuf::from(high),
    })
}

fn parse_overlay_arg(spec: &str) -> Result<FlashOverlay> {
    let (path, off_str) = spec
        .rsplit_once('@')
        .with_context(|| format!("expected PATH@OFFSET overlay, got '{spec}'"))?;
    let offset = parse_hex_or_dec(off_str).map_err(|e| anyhow::anyhow!(e))?;
    Ok(FlashOverlay {
        path: PathBuf::from(path),
        offset,
    })
}

fn ensure_parent_dir(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

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
