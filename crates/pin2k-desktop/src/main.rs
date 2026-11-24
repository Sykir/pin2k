//! Native Rust Pinball 2000 (*Star Wars Episode I*) cabinet & desktop runtime.
//!
//! Connects the lifted SWE1 game engine (`pin2k-core`) to:
//! - Physical Pinball 2000 Power Driver Board via Linux `/dev/parportN` (`ppdev`),
//!   raw x86 I/O port `/dev/port` (`0x378`/`0x278`/`0x3BC`), or wire-exact `VirtualPdbPort`
//! - Optional Encore-compatible wire trace output (`--lpt-trace <FILE>`)
//! - Extracted SWE1 animations & DCS audio (`--assets-dir <DIR>`)
//! - Live HTTP/HTML5 Canvas + WebAudio cabinet view (`--web-port <PORT>`)
//! - Scripted deterministic SWE1 gameplay showcase (`--script demo`) & PNG frame export

mod audio;
mod backend;
mod demo_scripts;
mod compositor;
mod raster;
mod telemetry;
mod scenes_compositor;
mod web_ui;

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use anyhow::Result;
use audio::CompositeAudioBackend;
use clap::Parser;
use compositor::SoftwareCompositor;
use pin2k_core::{GameEvent, GameScript, GameTitle, LocalDb, ModManifest, Pin2kMachine};
use backend::{open_lpt_backend, MaybeTracedLpt};
use demo_scripts::{
    build_admin_script, build_all_sequences_script, build_demo_script, build_diag_script,
    build_podrace_script, build_rfm_demo_script,
};
use pin2k_hal::{PdbDriver, PlayfieldSignature, TracedLptPort};
use web_ui::{spawn_web_server, DebugCommand, SharedWebState};

#[derive(Debug, Parser)]
#[command(
    name = "pin2k-desktop",
    version,
    about = "Star Wars Episode I (Pinball 2000) native Rust runtime with parallel-port Power Driver Board support"
)]
struct Cli {
    /// Parallel-port or serial driver board backend:
    /// `auto` (probes /dev/parport0..31 on Linux, falls back to virtual PDB),
    /// `emulated` (wire-exact VirtualPdbPort),
    /// `/dev/parportN` (Linux ppdev board),
    /// `ioport:0x378` (Linux /dev/port SPP base address), or
    /// `serial:/dev/ttyUSB0` / `serial:COM3` (cross-platform UART-to-PDB bridge).
    #[arg(long, default_value = "auto")]
    lpt_device: String,

    /// Dedicated serial port device for UART-to-PDB bridge (e.g. `/dev/ttyUSB0` or `COM3`).
    #[arg(long)]
    serial_device: Option<String>,

    /// Baud rate for `--serial-device` or `serial:...` (default: 115200).
    #[arg(long, default_value_t = 115_200)]
    serial_baud: u32,

    /// Optional file path to record every parallel-port byte read/write (`R/W reg=val`).
    #[arg(long)]
    lpt_trace: Option<PathBuf>,

    /// Optional JSONL file path to log every structured `GameEvent` emitted by `pin2k-core`.
    #[arg(long)]
    events_log: Option<PathBuf>,

    /// Disable native OS OGG/PCM audio playback (`--no-native-audio`).
    #[arg(long, default_value_t = false)]
    no_native_audio: bool,

    /// Show lower-half playfield lamp/coil debug telemetry instead of full-screen 640×480 native display.
    #[arg(long, default_value_t = false)]
    telemetry_overlay: bool,

    /// Path to extracted SWE1 assets directory (containing `gfx/gfx_manifest.json` and `audio/sound_manifest.json`).
    #[arg(long)]
    assets_dir: Option<PathBuf>,

    /// Path to extracted `game_script.json` (auto-detected at `<assets-dir>/scripts/game_script.json` when omitted).
    #[arg(long)]
    game_script: Option<PathBuf>,

    /// Path to local JSON database (`pin2k_db.json`) for high scores, audits, and adjustments.
    #[arg(long)]
    db_path: Option<PathBuf>,

    /// Debug: launch directly into a mode or scene (`attract`, `ingame`, `podrace`, `hangar_multiball`, `darth_maul_duel`, `scene:0`..`11`, `admin`).
    #[arg(long)]
    debug_mode: Option<String>,

    /// Enforce strict script-only rules when `game_script.json` is loaded (disables hardcoded rule fallbacks).
    #[arg(long, default_value_t = false)]
    strict_script: bool,

    /// Optional Pinball 2000 ROM update package (directory, `.exe`, or `.zip`) to validate & report active ROM version.
    #[arg(long)]
    rom_update: Option<PathBuf>,

    /// Game ruleset to run (`auto`, `swe1`, `rfm`, or `homebrew`). `auto` detects `#50069`/`#50070` or `mod.json`.
    #[arg(long, default_value = "auto")]
    game: String,

    /// One or more Mod / Homebrew directories containing `mod.json`, optional `gfx/`, and `audio/`.
    #[arg(long = "mod")]
    mods: Vec<PathBuf>,

    /// Optional HTTP port for live 640×480 Holoprojector + Playfield + WebAudio UI.
    #[arg(long)]
    web_port: Option<u16>,

    /// Bind address for `--web-port` (default: `0.0.0.0` for Docker/Kubernetes/LAN access).
    #[arg(long, default_value = "0.0.0.0")]
    web_bind: String,

    /// Built-in automated switch script to execute (`none` or `demo`).
    #[arg(long, default_value = "none")]
    script: String,

    /// Number of 60 Hz (`16 ms`) frames to simulate (`0` = run indefinitely until Ctrl-C).
    #[arg(long, default_value_t = 60)]
    frames: u64,

    /// Sleep 16 ms per frame for real-time 60 Hz pacing (automatically enabled when `--web-port` is set).
    #[arg(long, default_value_t = false)]
    realtime: bool,

    /// Optional PNG path to save the final rendered 640×480 composite frame.
    #[arg(long)]
    screenshot_out: Option<PathBuf>,

    /// Optional directory to export every rendered 640×480 frame (`frame_0000.png`..) for video encoding.
    #[arg(long)]
    frames_out_dir: Option<PathBuf>,
}

/// Polymorphic `LptPort` wrapper allowing runtime selection between physical `ppdev`,
/// `/dev/port`, and `VirtualPdbPort` while still allowing software switch injection on `VirtualPdbPort`.
fn main() -> Result<()> {
    let cli = Cli::parse();

    let update_bundle = match cli.rom_update.as_ref() {
        Some(upd_path) => Some(pin2k_rom_tool::prism::load_update_bundle(upd_path)?),
        None => None,
    };

    let mut loaded_mods = Vec::new();
    for mod_dir in &cli.mods {
        let m = ModManifest::load_from_dir(mod_dir)
            .map_err(|e| anyhow::anyhow!("failed to load mod '{}': {e}", mod_dir.display()))?;
        loaded_mods.push((mod_dir.clone(), m));
    }

    let mod_base = loaded_mods
        .iter()
        .find_map(|(_, m)| m.base_game.as_deref())
        .map(str::to_ascii_lowercase);

    let requested_title = match cli.game.to_ascii_lowercase().as_str() {
        "rfm" => Some(GameTitle::Rfm),
        "swe1" => Some(GameTitle::Swe1),
        "homebrew" => Some(GameTitle::Homebrew),
        _ => match mod_base.as_deref() {
            Some("homebrew") => Some(GameTitle::Homebrew),
            Some("rfm") => Some(GameTitle::Rfm),
            Some("swe1") => Some(GameTitle::Swe1),
            _ => update_bundle.as_ref().map(|b| {
                if b.header.game_number == 50070 {
                    GameTitle::Rfm
                } else {
                    GameTitle::Swe1
                }
            }),
        },
    };
    let virtual_sig = match requested_title {
        Some(GameTitle::Rfm) => PlayfieldSignature::Rfm,
        _ => PlayfieldSignature::Swe1,
    };

    let (dyn_lpt, lpt_desc) = open_lpt_backend(
        &cli.lpt_device,
        cli.serial_device.as_deref(),
        cli.serial_baud,
        virtual_sig,
    )?;
    let lpt = match cli.lpt_trace.as_ref() {
        Some(trace_path) => {
            if let Some(p) = trace_path.parent() {
                if !p.as_os_str().is_empty() {
                    fs::create_dir_all(p)?;
                }
            }
            MaybeTracedLpt::Traced(TracedLptPort::new(dyn_lpt, trace_path)?)
        }
        None => MaybeTracedLpt::Plain(dyn_lpt),
    };

    let mut events_file = match cli.events_log.as_ref() {
        Some(path) => {
            if let Some(p) = path.parent() {
                if !p.as_os_str().is_empty() {
                    fs::create_dir_all(p)?;
                }
            }
            Some(OpenOptions::new().create(true).write(true).truncate(true).open(path)?)
        }
        None => None,
    };

    let mut pdb = PdbDriver::new(lpt);
    let detected_sig = pdb.probe_playfield_signature();
    let fuses = pdb.read_fuse_status();

    let mut compositor = SoftwareCompositor::new();
    compositor.native_fullscreen = !cli.telemetry_overlay;
    let mut loaded_anims = if let Some(ref dir) = cli.assets_dir {
        compositor.load_assets(dir)?
    } else {
        0
    };

    let shared_web = Arc::new(Mutex::new(SharedWebState::default()));
    let mut audio = CompositeAudioBackend::new(
        Arc::clone(&shared_web),
        cli.assets_dir.as_deref(),
        !cli.no_native_audio,
    );

    for (mod_dir, manifest) in &loaded_mods {
        loaded_anims += compositor.load_mod_assets(mod_dir, &manifest.anim_overrides)?;
        audio.apply_mod_audio(mod_dir, &manifest.sound_overrides);
    }

    if let Some(port) = cli.web_port {
        spawn_web_server(&cli.web_bind, port, Arc::clone(&shared_web), cli.assets_dir.clone())?;
    }

    let rom_version_str = if let Some(ref bundle) = update_bundle {
        format!(
            "{} (#{}) v{}.{:02} (XINA {})",
            bundle.header.game_name,
            bundle.header.game_number,
            bundle.header.version_major,
            bundle.header.version_minor,
            bundle.header.xina_version
        )
    } else {
        match requested_title {
            Some(GameTitle::Rfm) => "RFM (#50070) native Rust rules".to_string(),
            _ => "SWE1 (#50069) native Rust rules".to_string(),
        }
    };

    let active_game = requested_title.unwrap_or(match detected_sig {
        PlayfieldSignature::Rfm => GameTitle::Rfm,
        _ => GameTitle::Swe1,
    });

    println!(
        "pin2k-desktop initialized: GAME={:?}, ROM=[{}], LPT=[{}], playfield_probe={:?}, fuses=(0x{:02X},0x{:02X}), loaded_anims={}",
        active_game, rom_version_str, lpt_desc, detected_sig, fuses.0, fuses.1, loaded_anims
    );

    if let Some(ref out_dir) = cli.frames_out_dir {
        fs::create_dir_all(out_dir)?;
    }

    let mut machine = Pin2kMachine::with_game(pdb, compositor, audio, active_game);
    for (mod_dir, manifest) in &loaded_mods {
        println!(
            "Loaded mod '{}' v{} by {} ({})",
            manifest.name,
            manifest.version,
            manifest.author,
            mod_dir.display()
        );
        machine.apply_mod(manifest);
    }

    let script_path = cli.game_script.clone().or_else(|| {
        cli.assets_dir
            .as_ref()
            .map(|d| d.join("scripts").join("game_script.json"))
            .filter(|p| p.is_file())
    });
    if let Some(ref sp) = script_path {
        let script = GameScript::load_from_file(sp)?;
        println!(
            "Loaded gamecode script '{}' (#{} v{}, {} rules, {} modes)",
            script.meta.title,
            script.meta.game_number,
            script.meta.version,
            script.rules.len(),
            script.modes.len()
        );
        machine.load_game_script(script);
    }
    machine.strict_script_mode = cli.strict_script;

    let db_file_path = cli.db_path.clone().unwrap_or_else(|| {
        cli.assets_dir
            .as_ref()
            .map(|d| d.join("pin2k_db.json"))
            .unwrap_or_else(|| PathBuf::from("pin2k_db.json"))
    });
    let mut local_db = LocalDb::load_or_create(&db_file_path)?;
    machine.apply_db(&local_db);
    println!("Local DB ready at {}", db_file_path.display());

    if let Some(ref dbg) = cli.debug_mode {
        let ok = machine.debug_launch_mode(dbg);
        println!("Debug launch '{dbg}' -> {}", if ok { "OK" } else { "unknown mode" });
    }
    let demo_events = match cli.script.as_str() {
        "demo" => build_demo_script(),
        "rfm-demo" => build_rfm_demo_script(),
        "diagnostics" => build_diag_script(),
        "admin" => build_admin_script(),
        "podrace" => build_podrace_script(),
        "all-sequences" => build_all_sequences_script(),
        _ => Vec::new(),
    };

    let mut scheduled_releases: Vec<(u64, u8)> = Vec::new();
    let pace_realtime = cli.realtime || cli.web_port.is_some();
    let mut frame: u64 = 0;

    loop {
        if cli.frames > 0 && frame >= cli.frames {
            break;
        }

        // Queue Swe1Scene::Podrace (index 6) right before Center Scoop entry in `--script podrace`
        if cli.script == "podrace" && frame == 25 {
            machine.players[0].next_scene_idx = 6;
        }

        // In `--script all-sequences`, sequentially trigger all 12 scenes, Darth Maul Duel, and Hangar Multiball
        if cli.script == "all-sequences" {
            if frame >= 140 && frame < 500 && (frame - 140) % 30 == 0 {
                let scene_idx = (frame - 140) / 30;
                machine.debug_launch_mode(&format!("scene:{scene_idx}"));
            } else if frame == 540 {
                machine.debug_launch_mode("darth_maul_duel");
            } else if frame == 575 {
                machine.debug_launch_mode("hangar_multiball");
            }
        }

        // Apply scripted switch presses/releases
        for &(start_f, sw_id, hold_f) in &demo_events {
            if frame == start_f {
                machine.hw.port.inner_mut().set_virtual_switch(sw_id, true);
                scheduled_releases.push((frame + hold_f, sw_id));
            }
        }
        scheduled_releases.retain(|&(rel_f, sw_id)| {
            if frame >= rel_f {
                machine.hw.port.inner_mut().set_virtual_switch(sw_id, false);
                false
            } else {
                true
            }
        });

        // Apply web UI switch presses & debug commands
        let mut pending_dbg = Vec::new();
        if let Ok(mut st) = shared_web.lock() {
            for (sw_id, closed, hold_ms) in st.pending_switches.drain(..) {
                machine.hw.port.inner_mut().set_virtual_switch(sw_id, closed);
                let hold_frames = (hold_ms as u64 / 16).max(2);
                scheduled_releases.push((frame + hold_frames, sw_id));
            }
            pending_dbg.extend(st.pending_debug.drain(..));
        }
        for cmd in pending_dbg {
            match cmd {
                DebugCommand::LaunchMode(m) => {
                    machine.debug_launch_mode(&m);
                }
                DebugCommand::PlayAnim(a) => {
                    machine.debug_play_anim(&a);
                }
                DebugCommand::PlaySound(sid) => {
                    machine.debug_play_sound(sid);
                }
                DebugCommand::AddScore(pts) => {
                    machine.debug_add_score(pts);
                }
                DebugCommand::PulseDrive(did, ms) => {
                    machine.debug_pulse_drive(did, ms);
                }
            }
        }

        // Update active animation selection from SWE1 game state before ticking
        let hint = machine.active_anim_hint.clone();
        machine.video.select_anim_by_hint(&hint);
        machine.video.tick_clock(16);

        machine.tick(16);

        let emitted_events = machine.drain_events();
        let mut db_dirty = false;
        for ev in &emitted_events {
            if matches!(
                ev.event,
                GameEvent::GameOver { .. }
                    | GameEvent::CreditAdded { .. }
                    | GameEvent::AdjustmentChanged { .. }
                    | GameEvent::SceneCompleted { .. }
            ) {
                db_dirty = true;
            }
            if matches!(ev.event, GameEvent::GameOver { .. }) {
                local_db.record_game(machine.snapshot_game_history());
            }
        }
        if db_dirty || (frame > 0 && frame % 600 == 0) {
            machine.sync_to_db(&mut local_db);
            let _ = local_db.save_to_file(&db_file_path);
        }
        if let Some(ref mut ef) = events_file {
            for ev in &emitted_events {
                if let Ok(line) = serde_json::to_string(ev) {
                    let _ = writeln!(ef, "{line}");
                }
            }
        }
        if cli.web_port.is_some() && !emitted_events.is_empty() {
            if let Ok(mut st) = shared_web.lock() {
                if let Ok(ev_json) = serde_json::to_string(&emitted_events) {
                    st.events_json = ev_json;
                }
            }
        }

        // Overlay live playfield lamps & PDB coil telemetry onto lower screen half only when `--telemetry-overlay` is enabled
        let rank_str = machine.players[machine.current_player].jedi_rank.label();
        if cli.telemetry_overlay {
            let lamps_copy = *machine.hw.lamp_states();
            let drive_mask = machine.hw.active_drive_mask();
            let pdb_ok = machine.hw.pdb_present;
            let door_ok = machine.coin_door_closed;
            let lasers = machine.players[machine.current_player].laser_shots;
            machine.video.draw_playfield_telemetry(
                &lamps_copy,
                drive_mask,
                pdb_ok,
                door_ok,
                machine.credits,
                machine.current_ball,
                rank_str,
                lasers,
                machine.elapsed_ms,
            );
        }

        if let Some(ref out_dir) = cli.frames_out_dir {
            let p = out_dir.join(format!("frame_{frame:04}.png"));
            machine.video.framebuffer.save(&p)?;
        }

        // Publish frame & state to web UI if active
        if cli.web_port.is_some() && frame % 4 == 0 {
            if let Ok(png) = machine.video.encode_png() {
                if let Ok(mut st) = shared_web.lock() {
                    let (seq, sfile, sloop) = st
                        .active_sound_file
                        .clone()
                        .unwrap_or((0, String::new(), false));
                    st.latest_png = png;
                    st.status_json = serde_json::json!({
                        "state": format!("{:?}", machine.state),
                        "score": machine.score,
                        "ball": machine.current_ball,
                        "credits": machine.credits,
                        "rank": rank_str,
                        "banner": machine.status_banner,
                        "sound_seq": seq,
                        "sound_file": sfile,
                        "sound_loop": sloop,
                    })
                    .to_string();
                    let gnum = machine.active_game_number();
                    if let Some(rec) = local_db.games.get(&gnum) {
                        if let Ok(db_str) = serde_json::to_string(rec) {
                            st.db_json = db_str;
                        }
                    } else {
                        machine.sync_to_db(&mut local_db);
                        if let Some(rec) = local_db.games.get(&gnum) {
                            if let Ok(db_str) = serde_json::to_string(rec) {
                                st.db_json = db_str;
                            }
                        }
                    }
                }
            }
        }

        frame += 1;
        if pace_realtime {
            thread::sleep(Duration::from_millis(16));
        }
    }

    if let Some(ref shot_path) = cli.screenshot_out {
        if let Some(p) = shot_path.parent() {
            if !p.as_os_str().is_empty() {
                fs::create_dir_all(p)?;
            }
        }
        machine.video.framebuffer.save(shot_path)?;
        println!("Saved 640x480 composite screenshot -> {}", shot_path.display());
    }

    machine.sync_to_db(&mut local_db);
    local_db.save_to_file(&db_file_path)?;

    let p = &machine.players[machine.current_player];
    println!(
        "pin2k-desktop summary: frames={}, state={:?}, score={}, rank={}, scenes_done={}, banner=\"{}\"",
        machine.video.frames_presented,
        machine.state,
        machine.score,
        p.jedi_rank.label(),
        p.scenes_completed.count_ones(),
        machine.status_banner
    );
    if let Some((dw, cw, disp)) = machine.hw.port.inner_ref().virtual_stats() {
        println!(
            "PDB LPT bus telemetry: data_writes={}, ctrl_writes={}, register_dispatches={}",
            dw, cw, disp
        );
    }

    Ok(())
}
