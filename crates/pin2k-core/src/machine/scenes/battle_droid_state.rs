//! State & ROM constants for `BattleDroidScene` (`0x00105878..0x00109250`).

use serde::{Deserialize, Serialize};

/// Exact ROM MTT frame position (`DeffBattleDroidSceneBackground::Run`, `0x00106bc0`):
/// `x = 0x11d - (242 >> 1) = 164`, `y = 0xe1 - 135 = 90`.
pub const MTT_X: i32 = 164;
pub const MTT_Y: i32 = 90;
/// MTT doors offset relative to `MTT_X, MTT_Y`: `(0x4b, -1)` = `(75, -1)`.
pub const MTT_DOORS_X: i32 = MTT_X + 75;
pub const MTT_DOORS_Y: i32 = MTT_Y - 1;
/// MTT explosion (`anim_bdroid_tank_explosion`, 580x220) at `(0, 20)` (`0x107b5e..0x107b60`).
pub const TANK_EXPLOSION_X: i32 = 0;
pub const TANK_EXPLOSION_Y: i32 = 20;
/// Exact ROM Battle Droid coordinates (`DeffBattleDroidSceneBackground::Run` + `bdroid_process`):
/// Left unpacking at `(MTT_X - 95, MTT_Y + 50) = (69, 140)`, standing at `(69, 135)`.
/// Right unpacking at `(MTT_X + 265, MTT_Y + 50) = (429, 140)`, standing at `(421, 134)`.
pub const LEFT_DROID_UNPACK_X: i32 = MTT_X - 95;
pub const RIGHT_DROID_UNPACK_X: i32 = MTT_X + 265;
pub const DROID_UNPACK_Y: i32 = MTT_Y + 50;
pub const LEFT_DROID_STAND_X: i32 = LEFT_DROID_UNPACK_X;
pub const LEFT_DROID_STAND_Y: i32 = DROID_UNPACK_Y - 5;
pub const RIGHT_DROID_STAND_X: i32 = RIGHT_DROID_UNPACK_X - 8;
pub const RIGHT_DROID_STAND_Y: i32 = DROID_UNPACK_Y - 6;
/// Exact ROM STAP start positions (`DeffBattleDroidSceneBackground::Run` `0x10727d`, `0x1072c9`):
/// Left STAP `(0, 84)`, Right STAP `(477, 80)` (`0x25c` adjusted for 163px sprite width).
pub const STAP_LEFT_X: i32 = 0;
pub const STAP_LEFT_Y: i32 = 84;
pub const STAP_RIGHT_X: i32 = 477;
pub const STAP_RIGHT_Y: i32 = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DroidSideState {
    Unpacking,
    Standing,
    Shooting,
    Exploding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleDroidState {
    /// Tank hits remaining (starts at 8 in `BattleDroidScene::start`).
    pub tank_hits_remaining: u8,
    /// Tank hits landed (`8 - tank_hits_remaining`).
    pub tank_hits_landed: u8,
    /// Number of MTT/STAP shots missed/hit counter (`mtts_missed`).
    pub staps_destroyed: u32,
    /// Speech counter (`speech_counter & 3 == 3` triggers `ard_hit_bdroids_speech`).
    pub speech_counter: u8,
    pub intro_elapsed_ms: u32,
    pub tank_hit_flash_ms: u32,
    pub tank_defeated_ms: u32,
    pub left_droid: DroidSideState,
    pub left_droid_ms: u32,
    pub right_droid: DroidSideState,
    pub right_droid_ms: u32,
    pub stap_zap_ms: u32,
    pub stap_exploding_ms: u32,
    pub stap_side_right: bool,
    pub last_hit_laser: bool,
}

impl BattleDroidState {
    pub fn new() -> Self {
        Self {
            tank_hits_remaining: 8,
            tank_hits_landed: 0,
            staps_destroyed: 0,
            speech_counter: 0,
            intro_elapsed_ms: 0,
            tank_hit_flash_ms: 0,
            tank_defeated_ms: 0,
            left_droid: DroidSideState::Unpacking,
            left_droid_ms: 0,
            right_droid: DroidSideState::Unpacking,
            right_droid_ms: 0,
            stap_zap_ms: 0,
            stap_exploding_ms: 0,
            stap_side_right: false,
            last_hit_laser: false,
        }
    }

    /// MTT doors animation based on tank hits landed (`anim_mtt_doors_*_anim`, 138x152).
    pub const fn mtt_doors_anim(&self) -> &'static str {
        match self.tank_hits_landed {
            0..=1 => "anim_mtt_doors_clean_anim",
            2..=3 => "anim_mtt_doors_damage1_anim",
            4..=5 => "anim_mtt_doors_damage2_anim",
            _ => "anim_mtt_doors_damage3_anim",
        }
    }
}

impl Default for BattleDroidState {
    fn default() -> Self {
        Self::new()
    }
}
