use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use pin2k_rom_tool::gfx_decoder::{PaletteFormat, PixelFormat};
use pin2k_rom_tool::interleave::{ChipPair, FlashOverlay, InterleaveMode};
use pin2k_rom_tool::{parse_hex_or_dec, parse_hex_or_dec_u16, parse_hex_or_dec_u32};

#[derive(Debug, Parser)]
#[command(
    name = "pin2k-rom-tool",
    version,
    about = "Pinball 2000 PRISM ROM reconstructor, BAR3 flash assembler, verifier, scanner, and sprite previewer"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
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

    /// Extract complete screen layouts, coordinates, Z-orders, fonts, colors, and Deff metadata from an ELF32 binary.
    ExtractLayout {
        /// Path to the symbolized ELF32 executable (e.g. `work/swe1_0200.elf`).
        #[arg(long)]
        elf: PathBuf,

        /// Output directory for `rom_layout_db.json`.
        #[arg(short, long)]
        output_dir: PathBuf,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliInterleaveMode {
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
pub(crate) enum CliPixelFormat {
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
pub(crate) enum CliPaletteFormat {
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

pub(crate) fn parse_pair_arg(spec: &str, idx: usize) -> Result<ChipPair> {
    let (low, high) = spec
        .split_once(':')
        .with_context(|| format!("expected LOW:HIGH pair, got '{spec}'"))?;
    Ok(ChipPair {
        label: format!("PAIR_{idx}"),
        low_path: PathBuf::from(low),
        high_path: PathBuf::from(high),
    })
}

pub(crate) fn parse_overlay_arg(spec: &str) -> Result<FlashOverlay> {
    let (path, off_str) = spec
        .rsplit_once('@')
        .with_context(|| format!("expected PATH@OFFSET overlay, got '{spec}'"))?;
    let offset = parse_hex_or_dec(off_str).map_err(|e| anyhow::anyhow!(e))?;
    Ok(FlashOverlay {
        path: PathBuf::from(path),
        offset,
    })
}

pub(crate) fn ensure_parent_dir(path: &PathBuf) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

