//! State & ROM constants for `R2D2Scene` / `DeffR2D2SceneBackground` (`0x0016d9f8..0x0016fb00`).

use serde::{Deserialize, Serialize};

pub const HURRY_UP_START: u64 = 500_000;
pub const HURRY_UP_FLOOR: u64 = 50_000;
pub const HURRY_UP_MAX: u64 = 2_000_000;
pub const HURRY_UP_RAMP_STEP: u64 = 250_000;
/// `DeffR2D2SceneBackground` / `R2D2Scene::hurry_up_timer_process`: `0x155E` (`5,470`) every `60ms`.
pub const HURRY_UP_DECAY_STEP: u64 = 5_470;
pub const HURRY_UP_FREEZE_MS: u32 = 3_000;

#[derive(Debug, Clone, Copy)]
pub struct R2UnitSpec {
    pub anim_name: &'static str,
    pub x: i32,
    pub y: i32,
    pub move_start: usize,
    pub move_end: usize,
    pub center_start: usize,
    pub center_end: usize,
    pub suck_start: usize,
    pub suck_end: usize,
}

/// Exact `r2_position_table_data` (`0x33f4f8`) from `swe1_0200.elf`.
/// Units embark in reverse order `5 -> 4 -> 3 -> 2 -> 1 -> 0` (R2-D2 is last!).
pub const R2_UNITS: [R2UnitSpec; 6] = [
    R2UnitSpec { anim_name: "anim_r2d2_scene_r2d2_unit", x: 134, y: 112, move_start: 10, move_end: 39, center_start: 40, center_end: 59, suck_start: 60, suck_end: 79 },
    R2UnitSpec { anim_name: "anim_r2d2_scene_blue_yellow_r2_unit", x: 231, y: 112, move_start: 10, move_end: 29, center_start: 30, center_end: 39, suck_start: 40, suck_end: 59 },
    R2UnitSpec { anim_name: "anim_r2d2_scene_dark_blue_r2_unit", x: 88, y: 109, move_start: 10, move_end: 29, center_start: 30, center_end: 39, suck_start: 40, suck_end: 59 },
    R2UnitSpec { anim_name: "anim_r2d2_scene_red_r2_unit", x: 223, y: 108, move_start: 10, move_end: 29, center_start: 30, center_end: 39, suck_start: 40, suck_end: 59 },
    R2UnitSpec { anim_name: "anim_r2d2_scene_yellow_r2_unit", x: 36, y: 99, move_start: 10, move_end: 29, center_start: 30, center_end: 39, suck_start: 40, suck_end: 59 },
    R2UnitSpec { anim_name: "anim_r2d2_scene_red_white_r2_unit", x: 219, y: 105, move_start: 10, move_end: 29, center_start: 30, center_end: 39, suck_start: 40, suck_end: 59 },
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2D2State {
    pub units_remaining: u8,
    pub hurry_up_value: u64,
    pub freeze_ms: u32,
    pub decay_accum_ms: u32,
    pub ramp_boosts: u32,
    pub suck_ms: u32,
    pub sucking_unit: Option<usize>,
    pub move_ms: u32,
    pub speech_counter: u8,
}

impl R2D2State {
    pub fn new() -> Self {
        Self {
            units_remaining: 6,
            hurry_up_value: HURRY_UP_START,
            freeze_ms: HURRY_UP_FREEZE_MS,
            decay_accum_ms: 0,
            ramp_boosts: 0,
            suck_ms: 0,
            sucking_unit: None,
            move_ms: 0,
            speech_counter: 0,
        }
    }
}

impl Default for R2D2State {
    fn default() -> Self {
        Self::new()
    }
}
