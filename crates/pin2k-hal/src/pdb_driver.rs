//! Host-side Pinball 2000 Power Driver Board (`PdbDriver`) protocol engine.
//!
//! Lifted from XINA's `PinIOInit`, `PinIOWrite`, `PinIORead`, `PinIOWork_16ms`,
//! `DriveWork_1ms`, `LampWork_0_5ms`, and `SwitchScan`:
//! - Drives any `LptPort` (`LinuxPpdevPort`, `DevPortLpt`, `VirtualPdbPort`, or `TracedLptPort`)
//!   using the exact 6-step parallel-port register handshake.
//! - Scans all 12 logical switch columns (96 switches: 8×8 matrix + coins + service + flippers/door).
//! - Multiplexes both 8×8 Lamp Matrices (`0..63` Matrix A via `0x06`, `64..127` Matrix B via `0x07`,
//!   strobed via `0x08`).
//! - Pulses and holds all 48 solenoid/flasher/flipper/neon drives across PDB registers
//!   `[0x0B, 0x0A, 0x09, 0x0D, 0x0C, 0x0E]`, enforcing coin-door high-voltage interlock safety.

use crate::lpt::{LptPort, PlayfieldSignature, SWE1_OPTO_NC_MASK};
use crate::{LampState, PinballHardware};

/// Exact PDB hardware register opcodes for the 6 × 8-bit drive banks (`HardwareDrives` @ `0x002e4fac`).
pub const PDB_DRIVE_BANK_REGS: [u8; 6] = [
    0x0B, // Drives  0.. 7 (Saucers, Drop Targets, Magnet, Shield Popper)
    0x0A, // Drives  8..15 (Trough Eject, Slingshots, Jets, Right Saucer)
    0x09, // Drives 16..23 (Playfield & Backpanel Flashers)
    0x0D, // Drives 24..31 (Backpanel Flashers)
    0x0C, // Drives 32..39 (Right/Left Flipper Power+Hold, Shield Power+Hold, Laser Flashers)
    0x0E, // Drives 40..47 (Lightsaber Neon, Ticket Dispenser)
];

/// Maximum single-shot coil pulse allowed in software (ms) to protect physical coils.
pub const MAX_COIL_PULSE_MS: u16 = 250;

/// Host-side Pinball 2000 Power Driver Board controller over an `LptPort`.
#[derive(Debug)]
pub struct PdbDriver<P: LptPort> {
    pub port: P,
    pub pdb_present: bool,
    pub status_byte: u8,
    pub high_voltage_enabled: bool,
    /// Column XOR mask converting active-low IR opto-interrupters (`D_NORM_CLOSED`) to active-high logical switches.
    pub opto_nc_mask: [u8; 8],

    // Cached 12-column logical switch state from most recent scan
    logical_switches: [u8; 12],

    // 48 hardware drives: remaining pulse milliseconds + continuous hold bitmask
    pulse_remaining_ms: [u16; 48],
    hold_mask: u64,
    last_drive_banks: [u8; 6],

    // 128 controlled lamps (Matrix A: 0..63, Matrix B: 64..127)
    lamps: [LampState; 128],
    elapsed_ms: u64,
    pwm_phase: u8,
}

impl<P: LptPort> PdbDriver<P> {
    /// Initializes the parallel port and clears all PDB lamp & solenoid outputs (`PinIOInit`).
    pub fn new(port: P) -> Self {
        let mut drv = Self {
            port,
            pdb_present: false,
            status_byte: 0xFF,
            high_voltage_enabled: true,
            opto_nc_mask: SWE1_OPTO_NC_MASK,
            logical_switches: [0; 12],
            pulse_remaining_ms: [0; 48],
            hold_mask: 0,
            last_drive_banks: [0xFF; 6], // force initial write
            lamps: [LampState::Off; 128],
            elapsed_ms: 0,
            pwm_phase: 0,
        };
        drv.init_bus();
        drv
    }

    /// Reproduces `PinIOInit(void)` (`0x001a828c`): sets `CONTROL=0x04` and zeroes
    /// lamp strobe (`0x08`) and all six drive banks (`0x09..0x0E`).
    pub fn init_bus(&mut self) {
        self.refresh_presence();
        self.port.write_control(0x04);
        for reg in [0x08u8, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E] {
            self.pin_io_write(reg, 0x00);
        }
        self.last_drive_banks = [0; 6];
    }

    /// Exact 6-step `PinIOWrite(reg, val)` (`0x001a8360`).
    #[inline]
    pub fn pin_io_write(&mut self, reg: u8, val: u8) {
        if !self.pdb_present {
            return;
        }
        self.port.write_data(reg);
        self.port.write_control(0x00);
        self.port.write_control(0x04); // bit 2 rising -> latch opcode `reg`
        self.port.write_data(val);
        self.port.write_control(0x05); // bit 0 high
        self.port.write_control(0x04); // bit 0 falling -> dispatch `(reg, val)`
    }

    /// Exact 6-step `PinIORead(reg)` (`0x001a8450`).
    #[inline]
    pub fn pin_io_read(&mut self, reg: u8) -> u8 {
        if !self.pdb_present {
            return 0x00;
        }
        self.port.write_data(reg);
        self.port.write_control(0x00);
        self.port.write_control(0x04); // bit 2 rising -> latch opcode `reg`
        self.port.write_control(0x2D); // bit 5 (input dir) + bit 3 + bit 2 + bit 0 -> gated read
        let val = self.port.read_data();
        self.port.write_control(0x04); // restore output idle
        val
    }

    /// Reproduces `PinIOWork_16ms` (`0x001a8540`): checks `(inb(STATUS) & 0xF8) != 0x78`.
    pub fn refresh_presence(&mut self) -> bool {
        let st = self.port.read_status();
        self.status_byte = st;
        self.pdb_present = (st & 0xF8) != 0x78;
        self.pdb_present
    }

    /// Reproduces `wms_pdb_fuse_status` (`0x001a8624`): reads fuse registers `0x12` and `0x13`.
    /// Returns `Ok((0, 0))` when all solenoid fuses are intact.
    pub fn read_fuse_status(&mut self) -> (u8, u8) {
        if !self.refresh_presence() {
            return (0xFF, 0x0F);
        }
        let f0 = self.pin_io_read(0x12);
        let f1 = self.pin_io_read(0x13) & 0x0F;
        (f0, f1)
    }

    /// Reproduces the hardware playfield diode-matrix probe (`p2k_lpt_probe_playfield`)
    /// to auto-identify whether a connected cabinet playfield is *Star Wars Episode I* or *Revenge From Mars*.
    pub fn probe_playfield_signature(&mut self) -> PlayfieldSignature {
        if !self.refresh_presence() {
            return PlayfieldSignature::Unknown;
        }
        let mut aux6 = [0u8; 8];
        let mut aux7 = [0u8; 8];

        self.pin_io_write(0x0D, 0x80);
        for i in 0..8 {
            let bit = 1u8 << i;
            self.pin_io_write(0x05, bit);
            self.pin_io_write(0x08, bit);
            self.pin_io_write(0x06, 0xFF);
            self.pin_io_write(0x05, 0x00);
            aux6[i] = self.pin_io_read(0x10);
            self.pin_io_write(0x08, 0x00);
            self.pin_io_write(0x06, 0x00);
        }
        for i in 0..8 {
            let bit = 1u8 << i;
            self.pin_io_write(0x05, bit);
            self.pin_io_write(0x08, bit);
            self.pin_io_write(0x07, 0xFF);
            self.pin_io_write(0x05, 0x00);
            aux7[i] = self.pin_io_read(0x11);
            self.pin_io_write(0x08, 0x00);
            self.pin_io_write(0x07, 0x00);
        }
        self.pin_io_write(0x0D, 0x00);

        let low_bits = |mask: u8, val: u8| -> u32 { ((!val) & mask).count_ones() };
        let primary = low_bits(0x04, aux6[1])
            + low_bits(0xC0, aux6[5])
            + low_bits(0x06, aux7[3])
            + low_bits(0xFF, aux7[5]);
        let secondary =
            low_bits(0x80, aux6[4]) + low_bits(0x80, aux7[2]) + low_bits(0x80, aux7[3]);

        let sig = if primary <= 1 && secondary > 1 {
            PlayfieldSignature::Swe1
        } else if primary > 11 && secondary <= 1 {
            PlayfieldSignature::Rfm
        } else {
            PlayfieldSignature::Unknown
        };
        if sig == PlayfieldSignature::Swe1 {
            self.opto_nc_mask = SWE1_OPTO_NC_MASK;
        } else if sig == PlayfieldSignature::Rfm {
            self.opto_nc_mask = [0; 8];
        }
        sig
    }

    /// Scans all 8 matrix columns + 3 direct cabinet switch groups over the parallel port,
    /// inverting `D_NORM_CLOSED` opto bits (`opto_nc_mask`) into active-high logical switches.
    pub fn scan_all_switches(&mut self) -> [u8; 12] {
        if !self.refresh_presence() {
            self.logical_switches = [0; 12];
            return self.logical_switches;
        }

        // Columns 0..7: standard 8×8 playfield matrix (`0x05` strobe + `0x04` read)
        for col in 0..8usize {
            self.pin_io_write(0x05, 1u8 << col);
            self.logical_switches[col] = self.pin_io_read(0x04) ^ self.opto_nc_mask[col];
        }
        self.pin_io_write(0x05, 0x00);

        // Column 8: Coin slots (`0x00`)
        self.logical_switches[8] = self.pin_io_read(0x00) & 0x0F;
        // Column 9: Service panel & flipper EOS (`0x03`)
        self.logical_switches[9] = self.pin_io_read(0x03);
        // Column 10: Slam tilt, Coin door closed (bit 1), Plumb tilt, Flippers, Action buttons (`0x01`)
        self.logical_switches[10] = self.pin_io_read(0x01);
        // Column 11: PDB board presence & ready/busy status (`0x02` / `0x0F`)
        self.logical_switches[11] = self.pin_io_read(0x02);

        // Enforce coin-door high-voltage interlock (`Sw[81]` = Col 10 bit 1)
        let door_closed = (self.logical_switches[10] & 0x02) != 0;
        self.high_voltage_enabled = door_closed;

        self.logical_switches
    }

    /// Advances coil pulse timers by `dt_ms`, flushes all modified solenoid/flasher/flipper
    /// drive banks (`0x09..0x0E`), and refreshes all 8 columns of Lamp Matrices A & B (`0x06..0x08`).
    pub fn sync_io(&mut self, dt_ms: u32) -> [u8; 12] {
        self.elapsed_ms = self.elapsed_ms.saturating_add(dt_ms as u64);
        self.pwm_phase = self.pwm_phase.wrapping_add(32);

        let switches = self.scan_all_switches();
        self.flush_drives(dt_ms);
        self.flush_lamps();
        switches
    }

    fn flush_drives(&mut self, dt_ms: u32) {
        let dt = dt_ms.min(u16::MAX as u32) as u16;
        let mut banks = [0u8; 6];

        for id in 0..48usize {
            let bank = id / 8;
            let bit = 1u8 << (id % 8);
            let was_energized = (self.last_drive_banks[bank] & bit) != 0;
            if was_energized && self.pulse_remaining_ms[id] > 0 {
                self.pulse_remaining_ms[id] = self.pulse_remaining_ms[id].saturating_sub(dt);
            }
            let pulsing = self.pulse_remaining_ms[id] > 0;
            let held = ((self.hold_mask >> id) & 1) != 0;
            // Ticket dispenser (47) and Neon (40) can run at low voltage; high-power coils require interlock closed
            let hv_ok = self.high_voltage_enabled || id >= 40;
            if (pulsing || held) && hv_ok {
                banks[bank] |= bit;
            } else if !hv_ok {
                self.pulse_remaining_ms[id] = 0;
            }
        }

        for (bank_idx, &reg) in PDB_DRIVE_BANK_REGS.iter().enumerate() {
            if banks[bank_idx] != self.last_drive_banks[bank_idx] {
                self.pin_io_write(reg, banks[bank_idx]);
                self.last_drive_banks[bank_idx] = banks[bank_idx];
            }
        }
    }

    fn eval_lamp(&self, state: LampState) -> bool {
        match state {
            LampState::Off => false,
            LampState::On => true,
            LampState::Blinking { period_ms } => {
                let p = (period_ms.max(60)) as u64;
                (self.elapsed_ms % p) < (p / 2)
            }
            LampState::Pwm(level) => level > self.pwm_phase,
        }
    }

    /// Multiplexes all 8 columns of Lamp Matrix A (`0..63`) and Lamp Matrix B (`64..127`)
    /// using XINA's exact blank-before-strobe sequence (`0x08=0 -> 0x06=rowA -> 0x07=rowB -> 0x08=1<<col`).
    fn flush_lamps(&mut self) {
        if !self.pdb_present {
            return;
        }
        for col in 0..8usize {
            let mut row_a = 0u8;
            let mut row_b = 0u8;
            for row in 0..8usize {
                if self.eval_lamp(self.lamps[col * 8 + row]) {
                    row_a |= 1u8 << row;
                }
                if self.eval_lamp(self.lamps[64 + col * 8 + row]) {
                    row_b |= 1u8 << row;
                }
            }
            self.pin_io_write(0x08, 0x00);
            self.pin_io_write(0x06, row_a);
            self.pin_io_write(0x07, row_b);
            self.pin_io_write(0x08, 1u8 << col);
        }
    }

    pub fn lamp_states(&self) -> &[LampState; 128] {
        &self.lamps
    }

    pub fn active_drive_mask(&self) -> u64 {
        let mut mask = 0u64;
        for (bank, &b) in self.last_drive_banks.iter().enumerate() {
            mask |= (b as u64) << (bank * 8);
        }
        mask
    }
}

impl<P: LptPort> PinballHardware for PdbDriver<P> {
    fn read_switch_matrix(&mut self) -> [u8; 8] {
        let all = self.scan_all_switches();
        let mut mat = [0u8; 8];
        mat.copy_from_slice(&all[..8]);
        mat
    }

    fn read_logical_switches(&mut self) -> [u8; 12] {
        self.logical_switches
    }

    fn fire_coil(&mut self, coil_id: u8, pulse_ms: u8) {
        if let Some(slot) = self.pulse_remaining_ms.get_mut(coil_id as usize) {
            let capped = (pulse_ms as u16).min(MAX_COIL_PULSE_MS);
            *slot = (*slot).max(capped);
        }
    }

    fn set_drive_hold(&mut self, drive_id: u8, active: bool) {
        if drive_id < 48 {
            if active {
                self.hold_mask |= 1u64 << drive_id;
            } else {
                self.hold_mask &= !(1u64 << drive_id);
            }
        }
    }

    fn set_lamp(&mut self, lamp_id: u8, state: LampState) {
        if let Some(slot) = self.lamps.get_mut(lamp_id as usize) {
            *slot = state;
        }
    }

    fn sync(&mut self, dt_ms: u32) {
        let _ = self.sync_io(dt_ms);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lpt::{PlayfieldSignature, VirtualPdbPort};

    #[test]
    fn pdb_driver_speaks_exact_wire_protocol_to_virtual_board() {
        let vboard = VirtualPdbPort::new(PlayfieldSignature::Swe1);
        let mut drv = PdbDriver::new(vboard);

        assert!(drv.pdb_present);
        assert_eq!(drv.probe_playfield_signature(), PlayfieldSignature::Swe1);
        assert_eq!(drv.read_fuse_status(), (0x00, 0x00));

        // Close Start button (Sw 2 = M13), Left Coin (Sw 64), and Left Flipper (Sw 85)
        drv.port.set_logical_switch(2, true);
        drv.port.set_logical_switch(64, true);
        drv.port.set_logical_switch(85, true);

        // Fire Trough Eject (Drive 8, 64ms), hold Left Flipper Hold (Drive 35),
        // and light JEDI 'J' (Lamp 29, Matrix A) + Shoot Again (Lamp 94, Matrix B)
        drv.fire_coil(8, 64);
        drv.set_drive_hold(35, true);
        drv.set_lamp(29, LampState::On);
        drv.set_lamp(94, LampState::On);

        let sw = drv.sync_io(16);
        assert_ne!(sw[0] & (1 << 2), 0, "Start button M13 closed");
        assert_ne!(sw[8] & 1, 0, "Left coin slot closed");
        assert_ne!(sw[10] & (1 << 5), 0, "Left flipper button closed");
        assert!(drv.port.is_drive_active(8), "Trough eject coil energized");
        assert!(drv.port.is_drive_active(35), "Left flipper hold energized");
        assert!(drv.port.is_lamp_lit(29), "JEDI 'J' lamp lit in Matrix A");
        assert!(drv.port.is_lamp_lit(94), "Shoot Again lamp lit in Matrix B");

        // After pulse expires (64ms total), Trough Eject turns off while Flipper Hold stays on
        drv.sync_io(64);
        assert!(!drv.port.is_drive_active(8), "Trough eject pulse finished");
        assert!(drv.port.is_drive_active(35), "Flipper hold remains active");

        // Opening the coin door interlock cuts high-voltage coils automatically
        drv.port.set_logical_switch(81, false);
        drv.sync_io(16);
        assert!(
            !drv.port.is_drive_active(35),
            "Coin door open cuts high voltage to flipper hold"
        );
    }

    #[test]
    fn pdb_driver_detects_rfm_signature_fuse_faults_and_open_bus() {
        let mut rfm_board = VirtualPdbPort::new(PlayfieldSignature::Rfm);
        rfm_board.fuse_byte0 = 0x04;
        rfm_board.fuse_byte1 = 0x02;
        let mut drv = PdbDriver::new(rfm_board);

        assert_eq!(drv.probe_playfield_signature(), PlayfieldSignature::Rfm);
        assert_eq!(drv.read_fuse_status(), (0x04, 0x02));

        // Simulate unplugged DB25 cable -> open-bus status 0x7F ((0x7F & 0xF8) == 0x78)
        drv.port.connected = false;
        assert!(!drv.refresh_presence());
        assert_eq!(drv.probe_playfield_signature(), PlayfieldSignature::Unknown);
        assert_eq!(drv.scan_all_switches(), [0u8; 12]);
    }
}
