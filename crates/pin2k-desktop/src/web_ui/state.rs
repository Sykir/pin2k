use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use pin2k_hal::{AudioBackend, SoundId};
use pin2k_rom_tool::asset_extractor::SoundManifest;

/// Decodes URL percent-encoded strings (`%3A` -> `:`, `%20` / `+` -> ` `).
pub fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let h1 = (bytes[i + 1] as char).to_digit(16);
            let h2 = (bytes[i + 2] as char).to_digit(16);
            if let (Some(a), Some(b)) = (h1, h2) {
                out.push(((a << 4) | b) as u8);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            out.push(b' ');
            i += 1;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

#[derive(Debug, Clone)]
pub enum DebugCommand {
    LaunchMode(String),
    PlayAnim(String),
    PlaySound(u16),
    AddScore(u64),
    PulseDrive(u8, u8),
    SetLamp(u8, u8),
    PlayerCmd(String),
    TogglePause,
    StepFrame,
}

#[derive(Default)]
pub struct SharedWebState {
    pub latest_png: Vec<u8>,
    pub status_json: String,
    pub events_json: String,
    pub db_json: String,
    pub assets_json: String,
    pub pending_switches: Vec<(u8, bool, u32)>,
    pub pending_debug: Vec<DebugCommand>,
    pub active_sound_file: Option<(u64, String, bool)>,
    pub audio_overrides: HashMap<String, PathBuf>,
    pub paused: bool,
}

#[derive(Clone)]
pub struct WebAudioBridge {
    pub shared: Arc<Mutex<SharedWebState>>,
    sound_files: Arc<HashMap<u16, String>>,
    override_files: Arc<HashMap<u16, String>>,
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
                            map.entry(id).or_insert(entry.file);
                        }
                    }
                }
            }
        }
        Self {
            shared,
            sound_files: Arc::new(map),
            override_files: Arc::new(HashMap::new()),
            seq: 0,
        }
    }

    /// Registers `<mod_dir>/audio/` overrides for WebAudio `/audio/{route_key}` serving.
    pub fn apply_mod_audio(&mut self, mod_dir: &Path, overrides: &HashMap<String, String>) {
        let adir = mod_dir.join("audio");
        if !adir.is_dir() {
            return;
        }
        let mut next = (*self.override_files).clone();
        let mut route_paths = HashMap::new();
        for (hex_id, rel_file) in overrides {
            let trimmed = hex_id.trim_start_matches("0x").trim_start_matches("0X");
            if let Ok(id) = u16::from_str_radix(trimmed, 16).or_else(|_| hex_id.parse::<u16>()) {
                let full = adir.join(rel_file);
                if full.exists() {
                    let route_name = format!("mod_{id:04X}.ogg");
                    next.insert(id, route_name.clone());
                    route_paths.insert(route_name, full);
                }
            }
        }
        self.override_files = Arc::new(next);
        if let Ok(mut st) = self.shared.lock() {
            st.audio_overrides.extend(route_paths);
        }
    }
}

impl AudioBackend for WebAudioBridge {
    fn play_sound(&mut self, id: SoundId, looping: bool) {
        self.seq = self.seq.wrapping_add(1);
        let file = self
            .override_files
            .get(&id.0)
            .cloned()
            .or_else(|| self.sound_files.get(&id.0).cloned())
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

#[cfg(test)]
mod tests {
    use super::percent_decode;

    #[test]
    fn percent_decode_handles_colons_and_spaces() {
        assert_eq!(percent_decode("scene%3A0"), "scene:0");
        assert_eq!(percent_decode("game%3Aswe1"), "game:swe1");
        assert_eq!(percent_decode("rfm_wave%3A8"), "rfm_wave:8");
        assert_eq!(percent_decode("hello+world%21"), "hello world!");
    }
}
