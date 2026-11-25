//! Zero-dependency pure-Rust HTTP live cabinet viewer, WebAudio DCS player,
//! and interactive switchboard (`--web-port <PORT>`).
//!
//! Lets any browser connect to `http://127.0.0.1:<PORT>` to view the live
//! 640×480 *Star Wars Episode I* holoprojector + playfield telemetry, hear
//! extracted `.ogg` DCS audio tracks via WebAudio, and trigger real playfield
//! switches with keyboard or mouse clicks.

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use pin2k_hal::{AudioBackend, SoundId};
use pin2k_rom_tool::asset_extractor::SoundManifest;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum DebugCommand {
    LaunchMode(String),
    PlayAnim(String),
    PlaySound(u16),
    AddScore(u64),
    PulseDrive(u8, u8),
}

#[derive(Default)]
pub struct SharedWebState {
    pub latest_png: Vec<u8>,
    pub status_json: String,
    pub events_json: String,
    pub db_json: String,
    pub pending_switches: Vec<(u8, bool, u32)>, // (sw_id, closed, hold_ms)
    pub pending_debug: Vec<DebugCommand>,
    pub active_sound_file: Option<(u64, String, bool)>, // (seq, filename, looping)
}

#[derive(Clone)]
pub struct WebAudioBridge {
    pub shared: Arc<Mutex<SharedWebState>>,
    sound_files: Arc<HashMap<u16, String>>,
    seq: u64,
}

impl WebAudioBridge {
    pub fn new(shared: Arc<Mutex<SharedWebState>>, assets_dir: Option<&Path>) -> Self {
        let mut map = HashMap::new();
        if let Some(dir) = assets_dir {
            let manifest_path = dir.join("audio/sound_manifest.json");
            if let Ok(raw) = fs::read_to_string(&manifest_path) {
                if let Ok(m) = serde_json::from_str::<SoundManifest>(&raw) {
                    for entry in m.sounds {
                        if let Some(id) = entry.dcs_id {
                            // Prefer single-loop or intro or oneshot first seen
                            map.entry(id).or_insert(entry.file);
                        }
                    }
                }
            }
        }
        Self {
            shared,
            sound_files: Arc::new(map),
            seq: 0,
        }
    }
}

impl AudioBackend for WebAudioBridge {
    fn play_sound(&mut self, id: SoundId, looping: bool) {
        self.seq = self.seq.wrapping_add(1);
        let file = self
            .sound_files
            .get(&id.0)
            .cloned()
            .unwrap_or_else(|| format!("S{:04X}.ogg", id.0));
        if let Ok(mut st) = self.shared.lock() {
            st.active_sound_file = Some((self.seq, file, looping));
        }
    }

    fn stop_sound(&mut self, _id: SoundId) {}

    fn stop_all(&mut self) {
        self.seq = self.seq.wrapping_add(1);
        if let Ok(mut st) = self.shared.lock() {
            st.active_sound_file = None;
        }
    }

    fn set_master_volume(&mut self, _volume: f32) {}
}

pub fn spawn_web_server(
    bind_addr: &str,
    port: u16,
    shared: Arc<Mutex<SharedWebState>>,
    assets_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind((bind_addr, port))?;
    println!("Live Pinball 2000 Web Cabinet UI listening at http://{bind_addr}:{port}");

    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let shared_cl = Arc::clone(&shared);
            let assets_cl = assets_dir.clone();
            thread::spawn(move || {
                let _ = handle_client(stream, &shared_cl, assets_cl.as_deref());
            });
        }
    });

    Ok(())
}

fn handle_client(
    mut stream: TcpStream,
    shared: &Arc<Mutex<SharedWebState>>,
    assets_dir: Option<&Path>,
) -> std::io::Result<()> {
    let mut buf = [0u8; 2048];
    let n = stream.read(&mut buf)?;
    if n == 0 {
        return Ok(());
    }
    let req = String::from_utf8_lossy(&buf[..n]);
    let first_line = req.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let raw_path = parts.next().unwrap_or("/");
    let path = raw_path.split('?').next().unwrap_or(raw_path);

    if method == "POST" && path.starts_with("/api/switch/") {
        if let Ok(sw_id) = path.trim_start_matches("/api/switch/").parse::<u8>() {
            if let Ok(mut st) = shared.lock() {
                st.pending_switches.push((sw_id, true, 96));
            }
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/mode/") {
        let mode = path.trim_start_matches("/api/debug/mode/").to_string();
        if let Ok(mut st) = shared.lock() {
            st.pending_debug.push(DebugCommand::LaunchMode(mode));
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/anim/") {
        let anim = path.trim_start_matches("/api/debug/anim/").to_string();
        if let Ok(mut st) = shared.lock() {
            st.pending_debug.push(DebugCommand::PlayAnim(anim));
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/sound/") {
        let raw_id = path.trim_start_matches("/api/debug/sound/");
        let parsed = if let Some(hex) = raw_id.strip_prefix("0x").or_else(|| raw_id.strip_prefix("0X")) {
            u16::from_str_radix(hex, 16).ok()
        } else {
            raw_id.parse::<u16>().ok()
        };
        if let Some(sid) = parsed {
            if let Ok(mut st) = shared.lock() {
                st.pending_debug.push(DebugCommand::PlaySound(sid));
            }
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/score/") {
        if let Ok(pts) = path.trim_start_matches("/api/debug/score/").parse::<u64>() {
            if let Ok(mut st) = shared.lock() {
                st.pending_debug.push(DebugCommand::AddScore(pts));
            }
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if path == "/api/db" {
        let json = shared
            .lock()
            .map(|s| {
                if s.db_json.is_empty() {
                    "{}".into()
                } else {
                    s.db_json.clone()
                }
            })
            .unwrap_or_else(|_| "{}".into());
        return write_response(&mut stream, "200 OK", "application/json", json.as_bytes());
    }

    if path == "/frame.png" {
        let png = shared
            .lock()
            .map(|s| s.latest_png.clone())
            .unwrap_or_default();
        return write_response(&mut stream, "200 OK", "image/png", &png);
    }

    if path == "/api/state" {
        let json = shared
            .lock()
            .map(|s| s.status_json.clone())
            .unwrap_or_else(|_| "{}".into());
        return write_response(&mut stream, "200 OK", "application/json", json.as_bytes());
    }

    if path == "/api/events" {
        let json = shared
            .lock()
            .map(|s| {
                if s.events_json.is_empty() {
                    "[]".into()
                } else {
                    s.events_json.clone()
                }
            })
            .unwrap_or_else(|_| "[]".into());
        return write_response(&mut stream, "200 OK", "application/json", json.as_bytes());
    }

    if let Some(audio_file) = path.strip_prefix("/audio/") {
        let clean: String = audio_file
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_' || *c == '.')
            .collect();
        if let Some(dir) = assets_dir {
            let full = dir.join("audio").join(&clean);
            if let Ok(bytes) = fs::read(full) {
                return write_response(&mut stream, "200 OK", "audio/ogg", &bytes);
            }
        }
        return write_response(&mut stream, "404 Not Found", "text/plain", b"not found");
    }

    write_response(&mut stream, "200 OK", "text/html; charset=utf-8", INDEX_HTML.as_bytes())
}

fn write_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    stream.write_all(body)
}

const INDEX_HTML: &str = include_str!("web_ui.html");
