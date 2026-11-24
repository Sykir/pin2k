//! 12-column (96-switch) logical switch matrix debouncer and edge detector.
//!
//! Columns `0..7` scan the 8×8 playfield matrix (`Sw 0..63`).
//! Columns `8..11` scan direct cabinet inputs (`Sw 64..95`: coins, service/EOS,
//! flippers/action buttons/tilts/coin-door interlock, and PDB status).

use serde::{Deserialize, Serialize};

/// Identifies one of the 96 logical switches (`col: 0..12`, `row: 0..8`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SwitchId {
    pub col: u8,
    pub row: u8,
}

impl SwitchId {
    pub const fn new(col: u8, row: u8) -> Self {
        Self { col, row }
    }

    pub const fn from_linear(id: u8) -> Self {
        Self {
            col: id / 8,
            row: id % 8,
        }
    }

    pub const fn index(self) -> usize {
        (self.col as usize) * 8 + (self.row as usize)
    }

    pub const fn linear_id(self) -> u8 {
        self.col * 8 + self.row
    }
}

/// Edge transition emitted after a switch state stabilizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SwitchEvent {
    Closed(SwitchId),
    Opened(SwitchId),
}

/// Fixed-sample debouncer for XINA's 12-column (96-switch) logical switch table.
#[derive(Debug)]
pub struct SwitchDebouncer {
    stable: [u8; 12],
    counters: [u8; 96],
    threshold_ticks: u8,
}

impl SwitchDebouncer {
    /// Creates a debouncer requiring `threshold_ticks` consecutive matching samples
    /// for matrix switches (direct flipper/action buttons in col 10 bits 4..7 use 1-tick fast response).
    pub fn new(threshold_ticks: u8) -> Self {
        let mut stable = [0u8; 12];
        stable[1] = 1 << 3; // Sw[11] Always Closed
        stable[10] = 0x02; // Sw[81] Coin Door Closed default
        Self {
            stable,
            counters: [0; 96],
            threshold_ticks: threshold_ticks.max(1),
        }
    }

    /// Seeds initial resting switch states (e.g. balls in trough at power-on) without firing edge events.
    pub fn seed_state(&mut self, initial: [u8; 12]) {
        self.stable = initial;
        self.counters = [0; 96];
    }

    /// Compatibility helper for 8×8 matrix-only callers.
    pub fn update(&mut self, raw8: [u8; 8]) -> Vec<SwitchEvent> {
        let mut full = self.stable;
        full[..8].copy_from_slice(&raw8);
        self.update_logical(full)
    }

    /// Feeds the latest 12-column switch sample and returns debounced edge events.
    pub fn update_logical(&mut self, raw: [u8; 12]) -> Vec<SwitchEvent> {
        let mut events = Vec::new();
        for col in 0..12u8 {
            let raw_col = raw[col as usize];
            let stable_col = self.stable[col as usize];
            for row in 0..8u8 {
                let mask = 1u8 << row;
                let raw_bit = (raw_col & mask) != 0;
                let stable_bit = (stable_col & mask) != 0;
                let idx = (col as usize) * 8 + (row as usize);

                // Direct cabinet flipper & laser action buttons (col 10, rows 4..7) respond in 1 tick
                let needed = if col == 10 && row >= 4 {
                    1
                } else {
                    self.threshold_ticks
                };

                if raw_bit != stable_bit {
                    self.counters[idx] = self.counters[idx].saturating_add(1);
                    if self.counters[idx] >= needed {
                        self.counters[idx] = 0;
                        if raw_bit {
                            self.stable[col as usize] |= mask;
                            events.push(SwitchEvent::Closed(SwitchId::new(col, row)));
                        } else {
                            self.stable[col as usize] &= !mask;
                            events.push(SwitchEvent::Opened(SwitchId::new(col, row)));
                        }
                    }
                } else {
                    self.counters[idx] = 0;
                }
            }
        }
        events
    }

    pub fn is_closed(&self, linear_id: u8) -> bool {
        if linear_id < 96 {
            let col = (linear_id / 8) as usize;
            let row = linear_id % 8;
            (self.stable[col] & (1 << row)) != 0
        } else {
            false
        }
    }

    pub fn stable_logical(&self) -> [u8; 12] {
        self.stable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debouncer_filters_single_tick_glitch() {
        let mut debouncer = SwitchDebouncer::new(2);
        debouncer.seed_state([0u8; 12]);
        let mut sample = [0u8; 12];
        sample[1] = 1 << 3;

        assert!(debouncer.update_logical(sample).is_empty());
        assert!(debouncer.update_logical([0u8; 12]).is_empty());
        assert!(debouncer.update_logical(sample).is_empty());
        assert_eq!(
            debouncer.update_logical(sample),
            vec![SwitchEvent::Closed(SwitchId::new(1, 3))]
        );
    }
}
