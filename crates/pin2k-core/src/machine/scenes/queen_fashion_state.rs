//! State & ROM tables for `QueenFashionScene` / `DeffQueenFashionSceneBackground` (`0x00118d10..0x0011ae00`).

use serde::{Deserialize, Serialize};

pub const THRONE_POS: (i32, i32) = (5, 44);
pub const WHEEL_POS: (i32, i32) = (29, 88);
pub const QUEEN_POS: (i32, i32) = (178, 46);
pub const BASE_POS: (i32, i32) = (167, 210);
pub const SPARKLE_LEFT_POS: (i32, i32) = (28, 177);
pub const SPARKLE_RIGHT_POS: (i32, i32) = (382, 177);

#[derive(Debug, Clone, Copy)]
pub struct WheelPositionSpec {
    pub card_bit: u8,
    pub left_bit: u8,
    pub right_bit: u8,
    pub anim_name: &'static str,
    pub start_step: usize,
}

/// Exact `wheel_position_table` (`0x33adb8`) from `swe1_0200.elf`.
pub const WHEEL_POSITIONS: [WheelPositionSpec; 6] = [
    WheelPositionSpec { card_bit: 0x20, left_bit: 0x20, right_bit: 0x04, anim_name: "anim_fashion_icon_black", start_step: 49 },
    WheelPositionSpec { card_bit: 0x01, left_bit: 0x10, right_bit: 0x02, anim_name: "anim_fashion_icon_fanhead", start_step: 59 },
    WheelPositionSpec { card_bit: 0x02, left_bit: 0x08, right_bit: 0x01, anim_name: "anim_fashion_icon_hornhead", start_step: 9 },
    WheelPositionSpec { card_bit: 0x04, left_bit: 0x04, right_bit: 0x20, anim_name: "anim_fashion_icon_nun", start_step: 19 },
    WheelPositionSpec { card_bit: 0x08, left_bit: 0x02, right_bit: 0x10, anim_name: "anim_fashion_icon_spade", start_step: 29 },
    WheelPositionSpec { card_bit: 0x10, left_bit: 0x01, right_bit: 0x08, anim_name: "anim_fashion_icon_victorian", start_step: 39 },
];

/// Exact 60 `(x, y)` points of `card_position_table_default` (`0x2d9fc4`) relative to `WHEEL_POS`.
#[rustfmt::skip]
pub const CARD_POSITIONS: [(i32, i32); 60] = [
    (117, 14), (136, 12), (154, 11), (175, 10), (198, 10), (221, 10), (234, 11), (248, 12), (262, 14), (276, 17),
    (289, 21), (302, 26), (314, 32), (326, 39), (336, 47), (346, 55), (354, 65), (361, 75), (366, 86), (369, 97),
    (366, 107), (360, 114), (355, 121), (346, 127), (337, 134), (327, 141), (314, 147), (302, 153), (289, 159), (276, 164),
    (262, 169), (249, 173), (235, 176), (221, 179), (199, 180), (177, 178), (156, 176), (136, 173), (117, 169), (99, 164),
    (84, 159), (69, 153), (59, 147), (48, 141), (40, 134), (35, 127), (31, 121), (27, 114), (27, 107), (26, 96),
    (26, 85), (27, 75), (30, 65), (35, 55), (39, 47), (48, 39), (57, 32), (70, 26), (84, 21), (101, 17),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueenFashionState {
    pub wheel_pos: usize,
    /// Current wheel animation step `0..60` (reaches `wheel_pos * 10` when stopped).
    pub wheel_step: usize,
    pub outfits_remaining: u8,
    /// Bitmask of exposed cards (`0x3F` = all 6 cards exposed).
    pub cards_mask: u8,
    pub spinning: bool,
    pub spin_step_ms: u32,
    pub base_flash_steps: u8,
    pub rng_state: u32,
}

impl QueenFashionState {
    pub fn new() -> Self {
        Self {
            wheel_pos: 0,
            wheel_step: 0,
            outfits_remaining: 6,
            cards_mask: 0,
            spinning: false,
            spin_step_ms: 0,
            base_flash_steps: 0,
            rng_state: 0xFA54_1001,
        }
    }

    pub fn is_left_card_clear(&self) -> bool {
        (self.cards_mask & WHEEL_POSITIONS[self.wheel_pos % 6].left_bit) == 0
    }

    pub fn is_right_card_clear(&self) -> bool {
        (self.cards_mask & WHEEL_POSITIONS[self.wheel_pos % 6].right_bit) == 0
    }

    pub fn is_wheel_position_valid(&self, pos: usize) -> bool {
        let wp = WHEEL_POSITIONS[pos % 6];
        (self.cards_mask & wp.left_bit) == 0 || (self.cards_mask & wp.right_bit) == 0
    }

    pub fn start_spin(&mut self, seed: u32) -> bool {
        if self.spinning {
            return false;
        }
        self.rng_state = self.rng_state.wrapping_mul(1_664_525).wrapping_add(seed | 1);
        let start = (self.rng_state as usize) % 6;
        for off in 0..6 {
            let cand = (start + off) % 6;
            if cand != self.wheel_pos && self.is_wheel_position_valid(cand) {
                self.wheel_pos = cand;
                self.spinning = true;
                self.spin_step_ms = 0;
                self.base_flash_steps = 3;
                return true;
            }
        }
        if self.is_wheel_position_valid(self.wheel_pos) {
            self.spinning = true;
            self.spin_step_ms = 0;
            self.base_flash_steps = 3;
            return true;
        }
        false
    }
}

impl Default for QueenFashionState {
    fn default() -> Self {
        Self::new()
    }
}
