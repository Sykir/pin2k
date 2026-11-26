//! State & ROM constants for `R2D2Scene` / `DeffR2D2SceneBackground` (`0x0016d9f8..0x0016fb00`).

use serde::{Deserialize, Serialize};

pub const R2_UNITS: [(&str, i32, i32); 6] = [
    ("anim_r2d2_scene_r2d2_unit", 225, 105),
    ("anim_r2d2_scene_blue_yellow_r2_unit", 25, 135),
    ("anim_r2d2_scene_red_r2_unit", 410, 130),
    ("anim_r2d2_scene_dark_blue_r2_unit", 400, 176),
    ("anim_r2d2_scene_yellow_r2_unit", 34, 172),
    ("anim_r2d2_scene_red_white_r2_unit", 180, 178),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2D2State {
    pub units_remaining: u8,
    pub center_unit: usize,
    pub hurry_up_value: u64,
    pub suck_ms: u32,
    pub speech_counter: u8,
}

impl R2D2State {
    pub fn new() -> Self {
        Self {
            units_remaining: 6,
            center_unit: 0,
            hurry_up_value: 500_000,
            suck_ms: 0,
            speech_counter: 0,
        }
    }
}

impl Default for R2D2State {
    fn default() -> Self {
        Self::new()
    }
}
