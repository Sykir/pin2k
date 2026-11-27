//! State & ROM constants for `MusicalChairsScene` (`MosEspaCantina`, `0x00158b58..0x0015bd00`).

use serde::{Deserialize, Serialize};

pub const COUNCIL_ANIMS: [&str; 6] = [
    "anim_jedi_council_mace",
    "anim_jedi_council_neck",
    "anim_jedi_council_yoda",
    "anim_jedi_council_member_1",
    "anim_jedi_council_member_2",
    "anim_jedi_council_chic",
];

pub const BG_X: i32 = 6;
pub const BG_Y: i32 = 3;
pub const CHAIR_X: [i32; 5] = [82, 166, 250, 354, 444];
pub const CHAIR_Y: i32 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CouncilAward {
    Points(u64),
    AddJediLetter,
    AdvanceBonusX,
}

impl CouncilAward {
    pub fn label(self) -> String {
        match self {
            CouncilAward::Points(p) => format!("{p}"),
            CouncilAward::AddJediLetter => "ADD JEDI LETTER".to_string(),
            CouncilAward::AdvanceBonusX => "ADVANCE BONUS X".to_string(),
        }
    }
}

pub const DEFAULT_CHAIR_AWARDS: [CouncilAward; 5] = [
    CouncilAward::Points(200_000),
    CouncilAward::Points(500_000),
    CouncilAward::AddJediLetter,
    CouncilAward::AdvanceBonusX,
    CouncilAward::Points(1_000_000),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosEspaCantinaState {
    /// Number of departures remaining to win the scene (3 in authentic ROM & rulebook).
    pub seats_remaining: u8,
    pub music_stopped: bool,
    pub phase_ms: u32,
    pub music_duration_ms: u32,
    /// Permutation of member indices across the 5 chairs (`order[2]` is center seat).
    pub order: [usize; 5],
    /// Bitmask of departed seat slots (`0..5`).
    pub departed_mask: u8,
    pub awards: [CouncilAward; 5],
    pub jump_ms: u32,
}

impl MosEspaCantinaState {
    pub fn new() -> Self {
        Self {
            seats_remaining: 3,
            music_stopped: false,
            phase_ms: 0,
            music_duration_ms: 5_000,
            order: [0, 1, 2, 3, 5],
            departed_mask: 0,
            awards: DEFAULT_CHAIR_AWARDS,
            jump_ms: 0,
        }
    }

    pub fn shuffle(&mut self) {
        self.order.rotate_left(1);
    }

    pub fn is_seat_departed(&self, idx: usize) -> bool {
        (self.departed_mask & (1 << idx)) != 0
    }

    pub fn mark_center_departed(&mut self) {
        self.departed_mask |= 1 << 2;
        self.seats_remaining = self.seats_remaining.saturating_sub(1);
    }
}

impl Default for MosEspaCantinaState {
    fn default() -> Self {
        Self::new()
    }
}
