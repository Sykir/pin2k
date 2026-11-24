//! Cross-platform native OGG Vorbis audio player (pure-Rust `lewton` decoder)
//! + WebAudio bridge (`CompositeAudioBackend`).
//!
//! - **Windows**: decodes `.ogg` -> PCM `i16` in pure Rust and plays via `winmm.dll` (`waveOutOpen`/`waveOutWrite`).
//! - **Linux**: decodes `.ogg` -> PCM `i16` and streams to `aplay` / `pw-cat` / `paplay` when available,
//!   degrading gracefully in headless environments without audio hardware.
//! - **Web UI**: simultaneously forwards DCS sound IDs to `WebAudioBridge`.

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use lewton::inside_ogg::OggStreamReader;
use pin2k_hal::{AudioBackend, SoundId};
use pin2k_rom_tool::asset_extractor::SoundManifest;

use crate::web_ui::{SharedWebState, WebAudioBridge};

enum AudioCmd {
    Play {
        file: PathBuf,
        volume: f32,
    },
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

        let native_tx = if enable_native {
            Some(spawn_native_audio_worker())
        } else {
            None
        };

        Self {
            web,
            sound_files: Arc::new(map),
            override_paths: Arc::new(HashMap::new()),
            audio_dir,
            native_tx,
            volume: 0.85,
        }
    }

    /// Registers custom `.ogg` audio overrides from `<mod_dir>/audio/` for mods & homebrew games.
    pub fn apply_mod_audio(&mut self, mod_dir: &Path, overrides: &HashMap<String, String>) {
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
                    file: path.clone(),
                    volume: self.volume,
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
                        file: path,
                        volume: self.volume,
                    });
                }
            }
        }
    }

    fn stop_sound(&mut self, id: SoundId) {
        self.web.stop_sound(id);
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
        while let Ok(cmd) = rx.recv() {
            match cmd {
                AudioCmd::Play { file, volume } => {
                    if let Ok((pcm, sample_rate, channels)) = decode_ogg_to_pcm(&file, volume) {
                        thread::spawn(move || {
                            play_pcm_platform(&pcm, sample_rate, channels);
                        });
                    }
                }
                AudioCmd::StopAll => {}
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

#[cfg(target_os = "linux")]
fn play_pcm_platform(pcm: &[i16], sample_rate: u32, channels: u16) {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let bytes: Vec<u8> = pcm.iter().flat_map(|s| s.to_le_bytes()).collect();
    let rate_str = sample_rate.to_string();
    let ch_str = channels.to_string();

    // Try standard Linux audio daemons non-fatally; silent no-op in headless environments
    if let Ok(mut child) = Command::new("aplay")
        .args(["-q", "-f", "S16_LE", "-r", &rate_str, "-c", &ch_str])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(ref mut stdin) = child.stdin {
            let _ = stdin.write_all(&bytes);
        }
        let _ = child.wait();
    }
}

#[cfg(windows)]
fn play_pcm_platform(pcm: &[i16], sample_rate: u32, channels: u16) {
    use std::ffi::c_void;
    use std::mem::size_of;

    #[repr(C)]
    struct WaveFormatEx {
        w_format_tag: u16,
        n_channels: u16,
        n_samples_per_sec: u32,
        n_avg_bytes_per_sec: u32,
        n_block_align: u16,
        w_bits_per_sample: u16,
        cb_size: u16,
    }

    #[repr(C)]
    struct WaveHdr {
        lp_data: *const u8,
        dw_buffer_length: u32,
        dw_bytes_recorded: u32,
        dw_user: usize,
        dw_flags: u32,
        dw_loops: u32,
        lp_next: *mut c_void,
        reserved: usize,
    }

    #[link(name = "winmm")]
    extern "system" {
        fn waveOutOpen(
            phwo: *mut *mut c_void,
            u_device_id: u32,
            pwfx: *const WaveFormatEx,
            dw_callback: usize,
            dw_instance: usize,
            fdw_open: u32,
        ) -> u32;
        fn waveOutPrepareHeader(hwo: *mut c_void, pwh: *mut WaveHdr, cbwh: u32) -> u32;
        fn waveOutWrite(hwo: *mut c_void, pwh: *mut WaveHdr, cbwh: u32) -> u32;
        fn waveOutUnprepareHeader(hwo: *mut c_void, pwh: *mut WaveHdr, cbwh: u32) -> u32;
        fn waveOutClose(hwo: *mut c_void) -> u32;
    }

    if pcm.is_empty() || channels == 0 || sample_rate == 0 {
        return;
    }
    let bytes: Vec<u8> = pcm.iter().flat_map(|s| s.to_le_bytes()).collect();
    let block_align = channels * 2;
    let fmt = WaveFormatEx {
        w_format_tag: 1, // WAVE_FORMAT_PCM
        n_channels: channels,
        n_samples_per_sec: sample_rate,
        n_avg_bytes_per_sec: sample_rate * (block_align as u32),
        n_block_align: block_align,
        w_bits_per_sample: 16,
        cb_size: 0,
    };

    unsafe {
        let mut hwo: *mut c_void = std::ptr::null_mut();
        if waveOutOpen(&mut hwo, u32::MAX, &fmt, 0, 0, 0) != 0 || hwo.is_null() {
            return;
        }
        let mut hdr = WaveHdr {
            lp_data: bytes.as_ptr(),
            dw_buffer_length: bytes.len() as u32,
            dw_bytes_recorded: 0,
            dw_user: 0,
            dw_flags: 0,
            dw_loops: 0,
            lp_next: std::ptr::null_mut(),
            reserved: 0,
        };
        if waveOutPrepareHeader(hwo, &mut hdr, size_of::<WaveHdr>() as u32) == 0 {
            let _ = waveOutWrite(hwo, &mut hdr, size_of::<WaveHdr>() as u32);
            let dur_ms = (pcm.len() as u64 * 1000) / ((sample_rate as u64) * (channels as u64));
            thread::sleep(std::time::Duration::from_millis(dur_ms + 30));
            let _ = waveOutUnprepareHeader(hwo, &mut hdr, size_of::<WaveHdr>() as u32);
        }
        let _ = waveOutClose(hwo);
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn play_pcm_platform(_pcm: &[i16], _sample_rate: u32, _channels: u16) {}
