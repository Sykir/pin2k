use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "pin2k-desktop",
    version,
    about = "Star Wars Episode I (Pinball 2000) native Rust runtime with parallel-port Power Driver Board support"
)]
pub(crate) struct Cli {
    /// Parallel-port or serial driver board backend:
    /// `auto` (probes /dev/parport0..31 on Linux, falls back to virtual PDB),
    /// `emulated` (wire-exact VirtualPdbPort),
    /// `/dev/parportN` (Linux ppdev board),
    /// `ioport:0x378` (Linux /dev/port SPP base address), or
    /// `serial:/dev/ttyUSB0` / `serial:COM3` (cross-platform UART-to-PDB bridge).
    #[arg(long, default_value = "auto")]
    pub(crate) lpt_device: String,

    /// Dedicated serial port device for UART-to-PDB bridge (e.g. `/dev/ttyUSB0` or `COM3`).
    #[arg(long)]
    pub(crate) serial_device: Option<String>,

    /// Baud rate for `--serial-device` or `serial:...` (default: 115200).
    #[arg(long, default_value_t = 115_200)]
    pub(crate) serial_baud: u32,

    /// Optional file path to record every parallel-port byte read/write (`R/W reg=val`).
    #[arg(long)]
    pub(crate) lpt_trace: Option<PathBuf>,

    /// Optional JSONL file path to log every structured `GameEvent` emitted by `pin2k-core`.
    #[arg(long)]
    pub(crate) events_log: Option<PathBuf>,

    /// Disable native OS OGG/PCM audio playback (`--no-native-audio`).
    #[arg(long, default_value_t = false)]
    pub(crate) no_native_audio: bool,

    /// Show lower-half playfield lamp/coil debug telemetry instead of full-screen 640×480 native display.
    #[arg(long, default_value_t = false)]
    pub(crate) telemetry_overlay: bool,

    /// Path to extracted SWE1 assets directory (containing `gfx/gfx_manifest.json` and `audio/sound_manifest.json`).
    #[arg(long)]
    pub(crate) assets_dir: Option<PathBuf>,

    /// Path to extracted `game_script.json` (auto-detected at `<assets-dir>/scripts/game_script.json` when omitted).
    #[arg(long)]
    pub(crate) game_script: Option<PathBuf>,

    /// Path to local JSON database (`pin2k_db.json`) for high scores, audits, and adjustments.
    #[arg(long)]
    pub(crate) db_path: Option<PathBuf>,

    /// Debug: launch directly into a mode or scene (`attract`, `ingame`, `podrace`, `hangar_multiball`, `darth_maul_duel`, `scene:0`..`11`, `admin`).
    #[arg(long)]
    pub(crate) debug_mode: Option<String>,

    /// Enforce strict script-only rules when `game_script.json` is loaded (disables hardcoded rule fallbacks).
    #[arg(long, default_value_t = false)]
    pub(crate) strict_script: bool,

    /// Optional Pinball 2000 ROM update package (directory, `.exe`, or `.zip`) to validate & report active ROM version.
    #[arg(long)]
    pub(crate) rom_update: Option<PathBuf>,

    /// Game ruleset to run (`auto`, `swe1`, `rfm`, or `homebrew`). `auto` detects `#50069`/`#50070` or `mod.json`.
    #[arg(long, default_value = "auto")]
    pub(crate) game: String,

    /// One or more Mod / Homebrew directories containing `mod.json`, optional `gfx/`, and `audio/`.
    #[arg(long = "mod")]
    pub(crate) mods: Vec<PathBuf>,

    /// Optional HTTP port for live 640×480 Holoprojector + Playfield + WebAudio UI.
    #[arg(long)]
    pub(crate) web_port: Option<u16>,

    /// Bind address for `--web-port` (default: `0.0.0.0` for Docker/Kubernetes/LAN access).
    #[arg(long, default_value = "0.0.0.0")]
    pub(crate) web_bind: String,

    /// Built-in automated switch script to execute (`none` or `demo`).
    #[arg(long, default_value = "none")]
    pub(crate) script: String,

    /// Number of 60 Hz (`16 ms`) frames to simulate (`0` = run indefinitely until Ctrl-C).
    #[arg(long, default_value_t = 60)]
    pub(crate) frames: u64,

    /// Sleep 16 ms per frame for real-time 60 Hz pacing (automatically enabled when `--web-port` is set).
    #[arg(long, default_value_t = false)]
    pub(crate) realtime: bool,

    /// Optional PNG path to save the final rendered 640×480 composite frame.
    #[arg(long)]
    pub(crate) screenshot_out: Option<PathBuf>,

    /// Optional directory to export every rendered 640×480 frame (`frame_0000.png`..) for video encoding.
    #[arg(long)]
    pub(crate) frames_out_dir: Option<PathBuf>,
}
