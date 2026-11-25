//! Zero-dependency pure-Rust HTTP live cabinet viewer, WebAudio DCS player,
//! and interactive switchboard + full Debug Console (`--web-port <PORT>`).

mod state;

pub use state::{percent_decode, DebugCommand, SharedWebState, WebAudioBridge};

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn spawn_web_server(
    bind_addr: &str,
    port: u16,
    shared: Arc<Mutex<SharedWebState>>,
    assets_dir: Option<std::path::PathBuf>,
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
    let mut buf = [0u8; 4096];
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
        if let Ok(sw_id) = percent_decode(path.trim_start_matches("/api/switch/")).parse::<u8>() {
            if let Ok(mut st) = shared.lock() {
                st.pending_switches.push((sw_id, true, 96));
            }
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/mode/") {
        let mode = percent_decode(path.trim_start_matches("/api/debug/mode/"));
        if let Ok(mut st) = shared.lock() {
            st.pending_debug.push(DebugCommand::LaunchMode(mode));
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/anim/") {
        let anim = percent_decode(path.trim_start_matches("/api/debug/anim/"));
        if let Ok(mut st) = shared.lock() {
            st.pending_debug.push(DebugCommand::PlayAnim(anim));
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/sound/") {
        let raw_id = percent_decode(path.trim_start_matches("/api/debug/sound/"));
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
        if let Ok(pts) = percent_decode(path.trim_start_matches("/api/debug/score/")).parse::<u64>() {
            if let Ok(mut st) = shared.lock() {
                st.pending_debug.push(DebugCommand::AddScore(pts));
            }
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/drive/") {
        let rest = percent_decode(path.trim_start_matches("/api/debug/drive/"));
        let mut sp = rest.split('/');
        if let (Some(id_s), Some(ms_s)) = (sp.next(), sp.next()) {
            if let (Ok(id), Ok(ms)) = (id_s.parse::<u8>(), ms_s.parse::<u8>()) {
                if let Ok(mut st) = shared.lock() {
                    st.pending_debug.push(DebugCommand::PulseDrive(id, ms));
                }
            }
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/lamp/") {
        let rest = percent_decode(path.trim_start_matches("/api/debug/lamp/"));
        let mut sp = rest.split('/');
        if let (Some(id_s), Some(st_s)) = (sp.next(), sp.next()) {
            if let (Ok(id), Ok(val)) = (id_s.parse::<u8>(), st_s.parse::<u8>()) {
                if let Ok(mut st) = shared.lock() {
                    st.pending_debug.push(DebugCommand::SetLamp(id, val));
                }
            }
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path.starts_with("/api/debug/player/") {
        let cmd = percent_decode(path.trim_start_matches("/api/debug/player/"));
        if let Ok(mut st) = shared.lock() {
            st.pending_debug.push(DebugCommand::PlayerCmd(cmd));
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path == "/api/debug/pause" {
        if let Ok(mut st) = shared.lock() {
            st.pending_debug.push(DebugCommand::TogglePause);
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if method == "POST" && path == "/api/debug/step" {
        if let Ok(mut st) = shared.lock() {
            st.pending_debug.push(DebugCommand::StepFrame);
        }
        return write_response(&mut stream, "200 OK", "application/json", b"{\"ok\":true}");
    }

    if path == "/api/assets" {
        let json = shared
            .lock()
            .map(|s| {
                if s.assets_json.is_empty() {
                    "{\"anims\":[],\"fonts\":[],\"sounds\":[]}".into()
                } else {
                    s.assets_json.clone()
                }
            })
            .unwrap_or_else(|_| "{}".into());
        return write_response(&mut stream, "200 OK", "application/json", json.as_bytes());
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
        if let Some(override_path) = shared.lock().ok().and_then(|s| s.audio_overrides.get(&clean).cloned()) {
            if let Ok(bytes) = fs::read(override_path) {
                return write_response(&mut stream, "200 OK", "audio/ogg", &bytes);
            }
        }
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

const INDEX_HTML: &str = include_str!("../web_ui.html");
