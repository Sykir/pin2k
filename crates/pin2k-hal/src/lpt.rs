//! Parallel-port (`LPT` / IEEE-1284 SPP) bus transports and wire-level
//! Williams Pinball 2000 Power Driver Board (`VirtualPdbPort`) state machine.
//!
//! Supports:
//! - Physical Linux `/dev/parportN` (`ppdev` ioctl passthrough) for real cabinets
//! - Physical Linux `/dev/port` raw x86 I/O ports (`0x378`, `0x278`, `0x3BC`)
//! - Wire-exact `VirtualPdbPort` emulating the real PDB edge-triggered latch protocol
//! - Optional Encore-compatible wire trace file (`TracedLptPort`)

use std::fs::{File, OpenOptions};
#[cfg(target_os = "linux")]
use std::io::{Read, Seek, SeekFrom};
use std::io::Write;
use std::path::Path;
#[cfg(target_os = "linux")]
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::HalError;

/// Default status byte returned by a powered Pinball 2000 Power Driver Board.
pub const PDB_PRESENT_STATUS: u8 = 0x87;
/// Open-bus SPP status byte seen when no driver board is attached (`(0x7F & 0xF8) == 0x78`).
pub const PDB_DISCONNECTED_STATUS: u8 = 0x7F;

/// Three-byte IEEE-1284 SPP register interface (`DATA +0`, `STATUS +1`, `CONTROL +2`).
pub trait LptPort {
    fn read_data(&mut self) -> u8;
    fn write_data(&mut self, val: u8);
    fn read_status(&mut self) -> u8;
    fn read_control(&mut self) -> u8;
    fn write_control(&mut self, val: u8);
}

// ============================================================================
// Linux `/dev/parportN` (`ppdev`) Hardware Backend
// ============================================================================

#[cfg(target_os = "linux")]
mod linux_ppdev {
    use super::*;
    use std::os::unix::io::AsRawFd;

    // Exact Linux <linux/ppdev.h> ioctl constants on x86_64/i386/arm64:
    // _IO('p', nr), _IOR('p', nr, u8/i32), _IOW('p', nr, u8/i32) where 'p' = 0x70
    const PPCLAIM: libc::c_ulong = 0x0000_708B;
    const PPRELEASE: libc::c_ulong = 0x0000_708C;
    const PPRSTATUS: libc::c_ulong = 0x8001_7081;
    const PPRCONTROL: libc::c_ulong = 0x8001_7083;
    const PPWCONTROL: libc::c_ulong = 0x4001_7084;
    const PPRDATA: libc::c_ulong = 0x8001_7085;
    const PPWDATA: libc::c_ulong = 0x4001_7086;
    const PPDATADIR: libc::c_ulong = 0x4004_7090;
    const PPNEGOT: libc::c_ulong = 0x4004_7091;
    const IEEE1284_MODE_COMPAT: libc::c_int = 0x0000;

    /// Real hardware parallel-port connection to a Pinball 2000 Power Driver Board via Linux `ppdev`.
    #[derive(Debug)]
    pub struct LinuxPpdevPort {
        file: File,
        path: PathBuf,
        claimed: bool,
    }

    impl LinuxPpdevPort {
        /// Opens and claims `/dev/parportN` in IEEE-1284 SPP compatibility mode.
        pub fn open(path: impl AsRef<Path>) -> Result<Self, HalError> {
            let path = path.as_ref().to_path_buf();
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&path)
                .map_err(|e| {
                    HalError::BackendIo(format!("cannot open '{}': {e}", path.display()))
                })?;

            let fd = file.as_raw_fd();
            // SAFETY: Standard ppdev ioctls operating on a valid open character device fd.
            unsafe {
                if libc::ioctl(fd, PPCLAIM) < 0 {
                    return Err(HalError::BackendIo(format!(
                        "PPCLAIM failed on '{}': {}",
                        path.display(),
                        std::io::Error::last_os_error()
                    )));
                }
                let mode: libc::c_int = IEEE1284_MODE_COMPAT;
                let _ = libc::ioctl(fd, PPNEGOT, &mode);
                let dir: libc::c_int = 0; // forward (output) default
                let _ = libc::ioctl(fd, PPDATADIR, &dir);
            }

            Ok(Self {
                file,
                path,
                claimed: true,
            })
        }

        /// Scans `/dev/parport0`..`/dev/parport31` for a responsive Pinball 2000 Power Driver Board.
        pub fn auto_discover() -> Option<Self> {
            for idx in 0..32 {
                let candidate = PathBuf::from(format!("/dev/parport{idx}"));
                if !candidate.exists() {
                    continue;
                }
                if let Ok(mut port) = Self::open(&candidate) {
                    let st = port.read_status();
                    if (st & 0xF8) != 0x78 {
                        return Some(port);
                    }
                }
            }
            None
        }

        pub fn device_path(&self) -> &Path {
            &self.path
        }
    }

    impl LptPort for LinuxPpdevPort {
        fn read_data(&mut self) -> u8 {
            let mut v: u8 = 0xFF;
            unsafe {
                if libc::ioctl(self.file.as_raw_fd(), PPRDATA, &mut v) < 0 {
                    return 0xFF;
                }
            }
            v
        }

        fn write_data(&mut self, val: u8) {
            let v: u8 = val;
            unsafe {
                let _ = libc::ioctl(self.file.as_raw_fd(), PPWDATA, &v);
            }
        }

        fn read_status(&mut self) -> u8 {
            let mut v: u8 = 0xFF;
            unsafe {
                if libc::ioctl(self.file.as_raw_fd(), PPRSTATUS, &mut v) < 0 {
                    return 0xFF;
                }
            }
            v
        }

        fn read_control(&mut self) -> u8 {
            let mut v: u8 = 0xFF;
            unsafe {
                if libc::ioctl(self.file.as_raw_fd(), PPRCONTROL, &mut v) < 0 {
                    return 0xFF;
                }
            }
            v
        }

        fn write_control(&mut self, val: u8) {
            let fd = self.file.as_raw_fd();
            // Mirror CONTROL bit 5 (0x20) explicitly via PPDATADIR so reverse-direction
            // switch scans sample external driver-board outputs reliably.
            let dir: libc::c_int = if (val & 0x20) != 0 { 1 } else { 0 };
            let v: u8 = val;
            unsafe {
                let _ = libc::ioctl(fd, PPDATADIR, &dir);
                let _ = libc::ioctl(fd, PPWCONTROL, &v);
            }
        }
    }

    impl Drop for LinuxPpdevPort {
        fn drop(&mut self) {
            if self.claimed {
                unsafe {
                    let _ = libc::ioctl(self.file.as_raw_fd(), PPRELEASE);
                }
                self.claimed = false;
            }
        }
    }
}

#[cfg(target_os = "linux")]
pub use linux_ppdev::LinuxPpdevPort;

// ============================================================================
// Direct `/dev/port` Raw x86 I/O Port Backend (`0x378`, `0x278`, `0x3BC`, Linux)
// ============================================================================

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct DevPortLpt {
    file: File,
    base_port: u16,
}

#[cfg(target_os = "linux")]
impl DevPortLpt {
    pub fn open(base_port: u16) -> Result<Self, HalError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/port")
            .map_err(|e| HalError::BackendIo(format!("cannot open /dev/port: {e}")))?;
        Ok(Self { file, base_port })
    }

    fn inb(&mut self, offset: u16) -> u8 {
        let mut b = [0xFFu8; 1];
        if self
            .file
            .seek(SeekFrom::Start((self.base_port + offset) as u64))
            .is_ok()
        {
            let _ = self.file.read_exact(&mut b);
        }
        b[0]
    }

    fn outb(&mut self, offset: u16, val: u8) {
        if self
            .file
            .seek(SeekFrom::Start((self.base_port + offset) as u64))
            .is_ok()
        {
            let _ = self.file.write_all(&[val]);
        }
    }
}

#[cfg(target_os = "linux")]
impl LptPort for DevPortLpt {
    fn read_data(&mut self) -> u8 {
        self.inb(0)
    }
    fn write_data(&mut self, val: u8) {
        self.outb(0, val);
    }
    fn read_status(&mut self) -> u8 {
        self.inb(1)
    }
    fn read_control(&mut self) -> u8 {
        self.inb(2)
    }
    fn write_control(&mut self, val: u8) {
        self.outb(2, val);
    }
}

// ============================================================================
// Wire-Exact Virtual Williams Pinball 2000 Power Driver Board (`VirtualPdbPort`)
// ============================================================================

/// Cabinet playfield identity reported during the hardware aux-strobe signature probe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayfieldSignature {
    Swe1,
    Rfm,
    Unknown,
}

/// Physical normally-closed opto-interrupter column masks for *Star Wars Episode I* (`D_NORM_CLOSED`
/// from `SwitchTable` @ `0x002df6b4`: `Sw 24..30` in Col 3 = `0x7F`, `Sw 32, 39` in Col 4 = `0x81`).
/// On a physical Pinball 2000 cabinet, unbroken IR optos read `1` on register `0x04` and `0` when blocked.
pub const SWE1_OPTO_NC_MASK: [u8; 8] = [0x00, 0x00, 0x00, 0x7F, 0x81, 0x00, 0x00, 0x00];

/// Wire-accurate software model of the Williams Pinball 2000 Power Driver Board.
///
/// Decodes the exact edge-triggered parallel-port bus transactions emitted by XINA's
/// `PinIOWrite` / `PinIORead` and `PdbDriver`:
/// - `CONTROL` bit 2 rising latches `DATA` as the active register opcode.
/// - `CONTROL` bit 0 falling dispatches `(opcode, DATA)` to the board's output latches.
/// - `CONTROL` bits `0` and `3` set (`0x09` / `0x2D`) gates the selected input register onto `DATA`.
#[derive(Debug)]
pub struct VirtualPdbPort {
    pub connected: bool,
    pub signature: PlayfieldSignature,
    /// Electrical 8×8 playfield switch matrix (`col 0..7`, `row 0..7` wire bits).
    /// Normally-open switches read `1` when closed; normally-closed optos (`opto_nc_mask`)
    /// read `1` when unobstructed and `0` when blocked by a ball.
    pub matrix_cols: [u8; 8],
    pub opto_nc_mask: [u8; 8],
    /// Direct coin switches (`opcode 0x00`, XINA logical column 8, bits 0..3).
    pub coin_switches: u8,
    /// Direct service & flipper EOS switches (`opcode 0x03`, XINA logical column 9).
    pub service_switches: u8,
    /// Direct cabinet buttons & tilts (`opcode 0x01`, XINA logical column 10:
    /// bit 0 = slam tilt, bit 1 = coin door closed, bit 2 = plumb bob tilt,
    /// bit 4 = right flipper, bit 5 = left flipper, bit 6 = right action, bit 7 = left action).
    pub cabinet_switches: u8,
    pub coin_door_closed: bool,
    /// Fuse fault bytes (`opcode 0x12`, `0x13`); `0x00` = all fuses healthy.
    pub fuse_byte0: u8,
    pub fuse_byte1: u8,

    /// Latched Lamp Matrix A columns (`opcode 0x06` + `0x08`).
    pub lamp_matrix_a: [u8; 8],
    /// Latched Lamp Matrix B columns (`opcode 0x07` + `0x08`).
    pub lamp_matrix_b: [u8; 8],
    /// Latched 6 × 8-bit solenoid/flasher/flipper/neon drive registers:
    /// `[0]=0x0B (drives 0..7), [1]=0x0A (8..15), [2]=0x09 (16..23),
    ///  [3]=0x0D (24..31), [4]=0x0C (32..39), [5]=0x0E (40..47)`.
    pub drive_banks: [u8; 6],

    // Internal parallel-port latch state
    data_latch: u8,
    ctrl_latch: u8,
    opcode_latch: u8,
    prev_ctrl_bit2: bool,
    prev_ctrl_bit0: bool,
    switch_col_select: u8,
    lamp_a_staging: u8,
    lamp_b_staging: u8,
    lamp_col_select: u8,
    ready_toggle: bool,
    flag1: u8,

    // Protocol telemetry counters
    pub data_writes: u64,
    pub ctrl_writes: u64,
    pub dispatches: u64,
}

impl Default for VirtualPdbPort {
    fn default() -> Self {
        Self::new(PlayfieldSignature::Swe1)
    }
}

impl VirtualPdbPort {
    pub fn new(signature: PlayfieldSignature) -> Self {
        let opto_nc_mask = match signature {
            PlayfieldSignature::Swe1 => SWE1_OPTO_NC_MASK,
            _ => [0; 8],
        };
        let mut s = Self {
            connected: true,
            signature,
            matrix_cols: opto_nc_mask,
            opto_nc_mask,
            coin_switches: 0,
            service_switches: 0,
            cabinet_switches: 0,
            coin_door_closed: true,
            fuse_byte0: 0,
            fuse_byte1: 0,
            lamp_matrix_a: [0; 8],
            lamp_matrix_b: [0; 8],
            drive_banks: [0; 6],
            data_latch: 0,
            ctrl_latch: 0x04,
            opcode_latch: 0,
            prev_ctrl_bit2: true,
            prev_ctrl_bit0: false,
            switch_col_select: 0,
            lamp_a_staging: 0,
            lamp_b_staging: 0,
            lamp_col_select: 0,
            ready_toggle: false,
            flag1: 0,
            data_writes: 0,
            ctrl_writes: 0,
            dispatches: 0,
        };
        // SWE1 Switch[11] (Col 1, Row 3 = M24) is "Always Closed" jumper
        s.set_logical_switch(11, true);
        // SWE1 4-ball trough starts with 4 balls blocking Trough Ball 1..4 optos (Sw 25..28)
        for sw in 25..=28 {
            s.set_logical_switch(sw, true);
        }
        s
    }

    /// Sets or clears one of XINA's 96 logical switches (`0..96`).
    /// - `0..64`: 8×8 playfield matrix (`col = sw / 8`, `row = sw % 8`), automatically encoding
    ///   active-low electrical levels (`0` = blocked) for IR opto switches in `opto_nc_mask`.
    /// - `64..72`: coin switches (`Physical[8]`).
    /// - `72..80`: service & flipper EOS switches (`Physical[9]`).
    /// - `80..88`: cabinet buttons, tilts, & coin-door interlock (`Physical[10]`).
    pub fn set_logical_switch(&mut self, sw_id: u8, closed: bool) {
        match sw_id {
            0..=63 => {
                let col = (sw_id / 8) as usize;
                let row = sw_id % 8;
                let bit = 1u8 << row;
                let is_opto = (self.opto_nc_mask[col] & bit) != 0;
                let wire_high = if is_opto { !closed } else { closed };
                if wire_high {
                    self.matrix_cols[col] |= bit;
                } else {
                    self.matrix_cols[col] &= !bit;
                }
            }
            64..=71 => {
                let bit = sw_id - 64;
                if closed {
                    self.coin_switches |= 1 << bit;
                } else {
                    self.coin_switches &= !(1 << bit);
                }
            }
            72..=79 => {
                let bit = sw_id - 72;
                if closed {
                    self.service_switches |= 1 << bit;
                } else {
                    self.service_switches &= !(1 << bit);
                }
            }
            80..=87 => {
                let bit = sw_id - 80;
                if bit == 1 {
                    self.coin_door_closed = closed;
                } else if closed {
                    self.cabinet_switches |= 1 << bit;
                } else {
                    self.cabinet_switches &= !(1 << bit);
                }
            }
            _ => {}
        }
    }

    /// Sets a standard Williams 2-digit playfield matrix switch (`11`..`88`).
    pub fn set_matrix_number(&mut self, matrix_num: u8, closed: bool) -> bool {
        let col = matrix_num / 10;
        let row = matrix_num % 10;
        if !(1..=8).contains(&col) || !(1..=8).contains(&row) {
            return false;
        }
        let sw_id = (col - 1) * 8 + (row - 1);
        self.set_logical_switch(sw_id, closed);
        true
    }

    /// Returns whether `drive_id` (`0..48`) is currently energized on the PDB output latches.
    pub fn is_drive_active(&self, drive_id: u8) -> bool {
        if drive_id >= 48 {
            return false;
        }
        let bank = (drive_id / 8) as usize;
        let bit = drive_id % 8;
        (self.drive_banks[bank] & (1 << bit)) != 0
    }

    /// Returns whether `lamp_id` (`0..128`) was lit during the most recent column scan.
    pub fn is_lamp_lit(&self, lamp_id: u8) -> bool {
        if lamp_id < 64 {
            let col = (lamp_id / 8) as usize;
            let row = lamp_id % 8;
            (self.lamp_matrix_a[col] & (1 << row)) != 0
        } else if lamp_id < 128 {
            let idx = lamp_id - 64;
            let col = (idx / 8) as usize;
            let row = idx % 8;
            (self.lamp_matrix_b[col] & (1 << row)) != 0
        } else {
            false
        }
    }

    fn one_hot_index(mask: u8) -> Option<usize> {
        if mask != 0 && mask.is_power_of_two() {
            Some(mask.trailing_zeros() as usize)
        } else {
            None
        }
    }

    fn read_gated_register(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x00 => self.coin_switches & 0x0F,
            0x01 => {
                let mut v = self.cabinet_switches & 0xF5;
                if self.coin_door_closed {
                    v |= 0x02;
                }
                v
            }
            0x02 => 0xF0,
            0x03 => self.service_switches,
            0x04 => {
                let col = Self::one_hot_index(self.switch_col_select).unwrap_or(0);
                self.matrix_cols[col]
            }
            0x0F => (self.flag1 << 6) | ((self.ready_toggle as u8) << 7),
            0x10 | 0x11 => {
                // During playfield signature probing (`lamp_col_select != 0` and staging `0xFF`),
                // return the real SWE1 / RFM resistor-diode signature bits; otherwise alternate
                // bit-6 ready/busy (`0xFF` / `0x00`).
                if self.lamp_col_select != 0
                    && (self.lamp_a_staging == 0xFF || self.lamp_b_staging == 0xFF)
                {
                    let col = Self::one_hot_index(self.lamp_col_select).unwrap_or(0);
                    match (self.signature, opcode, col) {
                        // SWE1 secondary signature: low bit 7 on aux6[4], aux7[2], aux7[3]
                        (PlayfieldSignature::Swe1, 0x10, 4) => return 0x7F,
                        (PlayfieldSignature::Swe1, 0x11, 2 | 3) => return 0x7F,
                        (PlayfieldSignature::Swe1, _, _) => return 0xFF,
                        // RFM primary signature: >11 low bits across aux6[1,5] & aux7[3,5]
                        (PlayfieldSignature::Rfm, 0x10, 1) => return !0x04,
                        (PlayfieldSignature::Rfm, 0x10, 5) => return !0xC0,
                        (PlayfieldSignature::Rfm, 0x11, 3) => return !0x06,
                        (PlayfieldSignature::Rfm, 0x11, 5) => return 0x00,
                        (PlayfieldSignature::Rfm, _, _) => return 0xFF,
                        (PlayfieldSignature::Unknown, _, _) => return 0xFF,
                    }
                }
                let v = if self.ready_toggle { 0x00 } else { 0xFF };
                self.ready_toggle = !self.ready_toggle;
                v
            }
            0x12 => self.fuse_byte0,
            0x13 => self.fuse_byte1 & 0x0F,
            _ => 0x00,
        }
    }

    fn dispatch_write(&mut self, opcode: u8, data: u8) {
        match opcode {
            0x05 => {
                self.switch_col_select = data;
                self.flag1 = 1;
            }
            0x06 => {
                self.lamp_a_staging = data;
            }
            0x07 => {
                self.lamp_b_staging = data;
            }
            0x08 => {
                self.lamp_col_select = data;
                if let Some(col) = Self::one_hot_index(data) {
                    self.lamp_matrix_a[col] = self.lamp_a_staging;
                    self.lamp_matrix_b[col] = self.lamp_b_staging;
                }
            }
            // 48-Drive Solenoid / Flasher / Flipper / Neon registers (matching XINA HardwareDrives)
            0x0B => self.drive_banks[0] = data, // Drives 0..7
            0x0A => self.drive_banks[1] = data, // Drives 8..15
            0x09 => self.drive_banks[2] = data, // Drives 16..23
            0x0D => self.drive_banks[3] = data, // Drives 24..31
            0x0C => self.drive_banks[4] = data, // Drives 32..39
            0x0E => self.drive_banks[5] = data, // Drives 40..47
            _ => {}
        }
    }
}

impl LptPort for VirtualPdbPort {
    fn read_data(&mut self) -> u8 {
        if !self.connected {
            return 0xFF;
        }
        let gated = (self.ctrl_latch & 0x01) != 0 && (self.ctrl_latch & 0x08) != 0;
        if gated {
            self.read_gated_register(self.opcode_latch)
        } else {
            self.data_latch
        }
    }

    fn write_data(&mut self, val: u8) {
        if !self.connected {
            return;
        }
        self.data_writes += 1;
        self.data_latch = val;
    }

    fn read_status(&mut self) -> u8 {
        if self.connected {
            PDB_PRESENT_STATUS
        } else {
            PDB_DISCONNECTED_STATUS
        }
    }

    fn read_control(&mut self) -> u8 {
        if self.connected {
            self.ctrl_latch
        } else {
            0xFF
        }
    }

    fn write_control(&mut self, val: u8) {
        if !self.connected {
            return;
        }
        self.ctrl_writes += 1;
        let bit2 = (val & 0x04) != 0;
        let bit0 = (val & 0x01) != 0;

        // Rising edge on CONTROL bit 2 captures DATA as opcode
        if !self.prev_ctrl_bit2 && bit2 {
            self.opcode_latch = self.data_latch;
        }
        self.prev_ctrl_bit2 = bit2;

        // Falling edge on CONTROL bit 0 dispatches (opcode, DATA)
        if self.prev_ctrl_bit0 && !bit0 {
            self.dispatches += 1;
            let op = self.opcode_latch;
            let d = self.data_latch;
            self.dispatch_write(op, d);
        }
        self.prev_ctrl_bit0 = bit0;
        self.ctrl_latch = val;
    }
}

// ============================================================================
// Wire-Level Trace Decorator (`--lpt-trace`)
// ============================================================================

pub struct TracedLptPort<P: LptPort> {
    pub inner: P,
    trace_file: File,
}

impl<P: LptPort> TracedLptPort<P> {
    pub fn new(inner: P, trace_path: impl AsRef<Path>) -> Result<Self, HalError> {
        let trace_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(trace_path.as_ref())
            .map_err(|e| HalError::BackendIo(format!("cannot open LPT trace file: {e}")))?;
        Ok(Self { inner, trace_file })
    }

    fn log_event(&mut self, dir: char, reg: u8, val: u8) {
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let _ = writeln!(
            self.trace_file,
            "{}.{:06} {} {:02x}={:02x}",
            dur.as_secs(),
            dur.subsec_micros(),
            dir,
            reg,
            val
        );
    }
}

impl<P: LptPort> LptPort for TracedLptPort<P> {
    fn read_data(&mut self) -> u8 {
        let v = self.inner.read_data();
        self.log_event('R', 0, v);
        v
    }
    fn write_data(&mut self, val: u8) {
        self.log_event('W', 0, val);
        self.inner.write_data(val);
    }
    fn read_status(&mut self) -> u8 {
        let v = self.inner.read_status();
        self.log_event('R', 1, v);
        v
    }
    fn read_control(&mut self) -> u8 {
        let v = self.inner.read_control();
        self.log_event('R', 2, v);
        v
    }
    fn write_control(&mut self, val: u8) {
        self.log_event('W', 2, val);
        self.inner.write_control(val);
    }
}
