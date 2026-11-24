//! Hardware Abstraction Layer (HAL) and Pinball 2000 Power Driver Board (PDB)
//! parallel-port communication stack.

pub mod lpt;
pub mod pdb_driver;
pub mod serial;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use lpt::{
    LptPort, PlayfieldSignature, TracedLptPort, VirtualPdbPort, PDB_DISCONNECTED_STATUS,
    PDB_PRESENT_STATUS, SWE1_OPTO_NC_MASK,
};
#[cfg(target_os = "linux")]
pub use lpt::{DevPortLpt, LinuxPpdevPort};
pub use pdb_driver::{PdbDriver, MAX_COIL_PULSE_MS, PDB_DRIVE_BANK_REGS};
pub use serial::{NativeSerialPort, SerialPdbPort, VirtualSerialBridge};

/// Unique identifier for an extracted graphical sprite or animation asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetId(pub u32);

/// Unique identifier for a DCS-2 audio command (`0x0001..0xFFFF`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SoundId(pub u16);

/// Controlled lamp illumination state across Dual 8×8 Matrices A & B (`0..128`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LampState {
    Off,
    On,
    /// Fixed-rate blink with period in milliseconds.
    Blinking { period_ms: u16 },
    /// PWM brightness level (`0` = off, `255` = full brightness).
    Pwm(u8),
}

/// Errors that may occur in hardware I/O backends.
#[derive(Debug, Error)]
pub enum HalError {
    #[error("invalid coil/drive id: {0}")]
    InvalidCoil(u8),
    #[error("invalid lamp id: {0}")]
    InvalidLamp(u8),
    #[error("hardware I/O error: {0}")]
    BackendIo(String),
}

/// Playfield I/O abstraction replacing XINA's `PinIO` / `Drive` / `Lamp` interface.
///
/// Implemented by `PdbDriver<P>` (talking the real Pinball 2000 parallel-port
/// wire protocol to `/dev/parportN`, `/dev/port`, or `VirtualPdbPort`) as well
/// as lightweight unit-test mocks (`MockHardware`).
pub trait PinballHardware {
    /// Returns the 8×8 playfield switch matrix (`columns 0..8`).
    fn read_switch_matrix(&mut self) -> [u8; 8];

    /// Returns XINA's full 12-column logical switch table (`96` switches:
    /// `0..8` playfield matrix, `8` coins, `9` service/EOS, `10` flippers/tilts/door, `11` status).
    fn read_logical_switches(&mut self) -> [u8; 12] {
        let m = self.read_switch_matrix();
        let mut out = [0u8; 12];
        out[..8].copy_from_slice(&m);
        out[10] = 0x02; // default coin door closed
        out
    }

    /// Fires a solenoid/flasher (`0..48`) for `pulse_ms` milliseconds.
    fn fire_coil(&mut self, coil_id: u8, pulse_ms: u8);

    /// Controls continuous hold state for a drive (`0..48`, e.g. Flipper Hold, Shield Hold, Magnet, Neon).
    fn set_drive_hold(&mut self, _drive_id: u8, _active: bool) {}

    /// Updates a playfield lamp's illumination state (`0..128` across Matrix A & Matrix B).
    fn set_lamp(&mut self, lamp_id: u8, state: LampState);

    /// Advances hardware timers and flushes I/O bus transactions by `dt_ms` milliseconds.
    fn sync(&mut self, _dt_ms: u32) {}
}

/// 640×480 (640×240×2) 2D compositor abstraction replacing the MediaGX blitter.
pub trait VideoBackend {
    /// Queues a sprite/animation frame for drawing at `(x, y)` with Z-order `z`.
    fn draw_sprite(&mut self, id: AssetId, frame: usize, x: i32, y: i32, z: u8);

    /// Draws HUD text onto the playfield reflection overlay.
    fn draw_text(&mut self, _text: &str, _x: i32, _y: i32, _color_rgb555: u16, _scale: u8) {}

    /// Presents the current frame to the display.
    fn present(&mut self);
}

/// Audio playback abstraction replacing the ADSP-2105 DCS-2 DSP.
pub trait AudioBackend {
    fn play_sound(&mut self, id: SoundId, looping: bool);
    fn stop_sound(&mut self, id: SoundId);
    fn stop_all(&mut self) {}
    fn set_master_volume(&mut self, volume: f32);
}

/// Helper for inspecting bits in an 8×8 switch matrix.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SwitchMatrix(pub [u8; 8]);

impl SwitchMatrix {
    pub fn is_closed(&self, col: u8, row: u8) -> bool {
        if col < 8 && row < 8 {
            (self.0[col as usize] & (1 << row)) != 0
        } else {
            false
        }
    }

    pub fn set(&mut self, col: u8, row: u8, closed: bool) {
        if col < 8 && row < 8 {
            if closed {
                self.0[col as usize] |= 1 << row;
            } else {
                self.0[col as usize] &= !(1 << row);
            }
        }
    }
}

/// In-memory mock hardware implementation for fast unit tests.
#[derive(Debug)]
pub struct MockHardware {
    pub switches: SwitchMatrix,
    pub direct_cols: [u8; 4],
    pub fired_coils: Vec<(u8, u8)>,
    pub held_drives: u64,
    pub lamps: [LampState; 128],
}

impl Default for MockHardware {
    fn default() -> Self {
        Self::new()
    }
}

impl MockHardware {
    pub fn new() -> Self {
        let mut direct_cols = [0u8; 4];
        direct_cols[2] = 0x02; // Coin door closed (Logical col 10, bit 1)
        let mut switches = SwitchMatrix::default();
        switches.set(1, 3, true); // Sw[11] Always Closed
        for row in 1..=4 {
            switches.set(3, row, true); // Sw[25..=28] 4 balls in trough
        }
        Self {
            switches,
            direct_cols,
            fired_coils: Vec::new(),
            held_drives: 0,
            lamps: [LampState::Off; 128],
        }
    }

    pub fn set_logical_switch(&mut self, sw_id: u8, closed: bool) {
        if sw_id < 64 {
            self.switches.set(sw_id / 8, sw_id % 8, closed);
        } else if sw_id < 96 {
            let dcol = ((sw_id - 64) / 8) as usize;
            let bit = sw_id % 8;
            if closed {
                self.direct_cols[dcol] |= 1 << bit;
            } else {
                self.direct_cols[dcol] &= !(1 << bit);
            }
        }
    }
}

impl PinballHardware for MockHardware {
    fn read_switch_matrix(&mut self) -> [u8; 8] {
        self.switches.0
    }

    fn read_logical_switches(&mut self) -> [u8; 12] {
        let mut out = [0u8; 12];
        out[..8].copy_from_slice(&self.switches.0);
        out[8..12].copy_from_slice(&self.direct_cols);
        out
    }

    fn fire_coil(&mut self, coil_id: u8, pulse_ms: u8) {
        self.fired_coils.push((coil_id, pulse_ms));
    }

    fn set_drive_hold(&mut self, drive_id: u8, active: bool) {
        if drive_id < 48 {
            if active {
                self.held_drives |= 1u64 << drive_id;
            } else {
                self.held_drives &= !(1u64 << drive_id);
            }
        }
    }

    fn set_lamp(&mut self, lamp_id: u8, state: LampState) {
        if let Some(slot) = self.lamps.get_mut(lamp_id as usize) {
            *slot = state;
        }
    }
}

/// Headless video backend that records draw calls for testing.
#[derive(Debug, Default)]
pub struct HeadlessVideo {
    pub draw_calls: Vec<(AssetId, usize, i32, i32, u8)>,
    pub text_calls: Vec<(String, i32, i32)>,
    pub frames_presented: u64,
}

impl VideoBackend for HeadlessVideo {
    fn draw_sprite(&mut self, id: AssetId, frame: usize, x: i32, y: i32, z: u8) {
        self.draw_calls.push((id, frame, x, y, z));
    }

    fn draw_text(&mut self, text: &str, x: i32, y: i32, _color_rgb555: u16, _scale: u8) {
        self.text_calls.push((text.to_string(), x, y));
    }

    fn present(&mut self) {
        self.frames_presented += 1;
    }
}

/// Headless audio backend that logs triggered DCS sound IDs for verification.
#[derive(Debug, Default)]
pub struct HeadlessAudio {
    pub played: Vec<(SoundId, bool)>,
    pub stopped: Vec<SoundId>,
    pub volume: f32,
}

impl AudioBackend for HeadlessAudio {
    fn play_sound(&mut self, id: SoundId, looping: bool) {
        self.played.push((id, looping));
    }

    fn stop_sound(&mut self, id: SoundId) {
        self.stopped.push(id);
    }

    fn stop_all(&mut self) {
        self.played.clear();
    }

    fn set_master_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }
}
