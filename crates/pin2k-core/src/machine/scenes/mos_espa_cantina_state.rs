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

pub const CHAIR_X: [i32; 5] = [82, 166, 250, 354, 444];
pub const CHAIR_Y: i32 = 64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MosEspaCantinaState {
    pub seats_remaining: u8,
    pub music_stopped: bool,
    pub phase_ms: u32,
    /// Permutation of member indices across the 5 chairs.
    pub order: [usize; 5],
    pub jump_ms: u32,
}

impl MosEspaCantinaState {
    pub fn new() -> Self {
        Self {
            seats_remaining: 5,
            music_stopped: false,
            phase_ms: 0,
            order: [0, 1, 2, 3, 5],
            jump_ms: 0,
        }
    }

    pub fn shuffle(&mut self) {
        self.order.rotate_left(1);
    }
}

impl Default for MosEspaCantinaState {
    fn default() -> Self {
        Self::new()
    }
}
