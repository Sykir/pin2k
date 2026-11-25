//! Cross-platform native OGG Vorbis audio player (pure-Rust `lewton` decoder)
//! + WebAudio bridge (`CompositeAudioBackend`).

mod platform;

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use lewton::inside_ogg::OggStreamReader;
use pin2k_hal::{AudioBackend, SoundId};
use pin2k_rom_tool::asset_extractor::SoundManifest;

use crate::web_ui::{SharedWebState, WebAudioBridge};

enum AudioCmd {
    Play {
        id: u16,
        file: PathBuf,
        volume: f32,
        looping: bool,
    },
    Stop(u16),
    StopAll,
}

#[derive(Clone)]
pub struct CompositeAudioBackend {
    web: WebAudioBridge,
    sound_files: Arc<HashMap<u16, String>>,
    override_paths: Arc<HashMap<u16, PathBuf>>,
    audio_dir: Option<PathBuf>,
    native_tx: Option<Sender<AudioCmd>>,
    volume: f32,
}

impl CompositeAudioBackend {
    pub fn new(
        shared: Arc<Mutex<SharedWebState>>,
        assets_dir: Option<&Path>,
        enable_native: bool,
    ) -> Self {
        let web = WebAudioBridge::new(shared, assets_dir);
        let mut map = HashMap::new();
        let audio_dir = assets_dir.map(|d| d.join("audio"));
        if let Some(ref adir) = audio_dir {
            let manifest_path = adir.join("sound_manifest.json");
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
        let native_tx = enable_native.then(spawn_native_audio_worker);
        Self {
            web,
            sound_files: Arc::new(map),
            override_paths: Arc::new(HashMap::new()),
            audio_dir,
            native_tx,
            volume: 0.85,
        }
    }

    /// Registers custom `.ogg` audio overrides from `<mod_dir>/audio/` for native and WebAudio.
    pub fn apply_mod_audio(&mut self, mod_dir: &Path, overrides: &HashMap<String, String>) {
        self.web.apply_mod_audio(mod_dir, overrides);
        let adir = mod_dir.join("audio");
        if !adir.is_dir() {
            return;
        }
        let mut next = (*self.override_paths).clone();
        for (hex_id, rel_file) in overrides {
            let trimmed = hex_id.trim_start_matches("0x").trim_start_matches("0X");
            if let Ok(id) = u16::from_str_radix(trimmed, 16).or_else(|_| hex_id.parse::<u16>()) {
                let p = adir.join(rel_file);
                if p.exists() {
                    next.insert(id, p);
                }
            }
        }
        self.override_paths = Arc::new(next);
    }
}

impl AudioBackend for CompositeAudioBackend {
    fn play_sound(&mut self, id: SoundId, looping: bool) {
        self.web.play_sound(id, looping);
        if let Some(tx) = &self.native_tx {
            if let Some(path) = self.override_paths.get(&id.0) {
                let _ = tx.send(AudioCmd::Play {
                    id: id.0,
                    file: path.clone(),
                    volume: self.volume,
                    looping,
                });
                return;
            }
            if let Some(ref dir) = self.audio_dir {
                let fname = self
                    .sound_files
                    .get(&id.0)
                    .cloned()
                    .unwrap_or_else(|| format!("S{:04X}.ogg", id.0));
                let path = dir.join(fname);
                if path.exists() {
                    let _ = tx.send(AudioCmd::Play {
                        id: id.0,
                        file: path,
                        volume: self.volume,
                        looping,
                    });
                }
            }
        }
    }

    fn stop_sound(&mut self, id: SoundId) {
        self.web.stop_sound(id);
        if let Some(ref tx) = self.native_tx {
            let _ = tx.send(AudioCmd::Stop(id.0));
        }
    }

    fn stop_all(&mut self) {
        self.web.stop_all();
        if let Some(ref tx) = self.native_tx {
            let _ = tx.send(AudioCmd::StopAll);
        }
    }

    fn set_master_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        self.web.set_master_volume(self.volume);
    }
}

fn spawn_native_audio_worker() -> Sender<AudioCmd> {
    let (tx, rx) = mpsc::channel::<AudioCmd>();
    thread::spawn(move || {
        let stop_gen = Arc::new(AtomicU64::new(0));
        let mut active: HashMap<u16, Arc<AtomicBool>> = HashMap::new();
        while let Ok(cmd) = rx.recv() {
            match cmd {
                AudioCmd::Play { id, file, volume, looping } => {
                    if looping {
                        if let Some(prev) = active.remove(&id) {
                            prev.store(true, Ordering::Relaxed);
                        }
                    }
                    let cancel = Arc::new(AtomicBool::new(false));
                    active.insert(id, Arc::clone(&cancel));
                    let gen = stop_gen.load(Ordering::Relaxed);
                    let gen_ref = Arc::clone(&stop_gen);
                    if let Ok((pcm, sample_rate, channels)) = decode_ogg_to_pcm(&file, volume) {
                        thread::spawn(move || loop {
                            if cancel.load(Ordering::Relaxed)
                                || gen_ref.load(Ordering::Relaxed) != gen
                            {
                                break;
                            }
                            platform::play_pcm_platform(&pcm, sample_rate, channels, &cancel, &gen_ref, gen);
                            if !looping {
                                break;
                            }
                        });
                    }
                }
                AudioCmd::Stop(id) => {
                    if let Some(flag) = active.remove(&id) {
                        flag.store(true, Ordering::Relaxed);
                    }
                }
                AudioCmd::StopAll => {
                    stop_gen.fetch_add(1, Ordering::Relaxed);
                    for (_, flag) in active.drain() {
                        flag.store(true, Ordering::Relaxed);
                    }
                }
            }
        }
    });
    tx
}

fn decode_ogg_to_pcm(path: &Path, volume: f32) -> anyhow::Result<(Vec<i16>, u32, u16)> {
    let f = File::open(path)?;
    let mut reader = OggStreamReader::new(f)?;
    let sr = reader.ident_hdr.audio_sample_rate;
    let ch = reader.ident_hdr.audio_channels as u16;
    let mut pcm = Vec::new();
    while let Some(pkt) = reader.read_dec_packet_itl()? {
        for s in pkt {
            let scaled = ((s as f32) * volume).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
            pcm.push(scaled);
        }
    }
    Ok((pcm, sr, ch))
}

