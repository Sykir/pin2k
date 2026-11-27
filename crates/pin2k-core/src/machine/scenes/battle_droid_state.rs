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
/// Left unpacking (`0xffffffb4, 0x32` rel to MTT) = `(88, 140)`, standing (`0xffffffa1 - 1, 0x32 - 5`) = `(68, 135)`.
/// Right unpacking (`0x8b, 0x30` rel to MTT) = `(303, 138)`, standing (`0x109 - 8, 0x32 - 6`) = `(421, 134)`.
pub const LEFT_DROID_UNPACK_X: i32 = MTT_X - 76;
pub const LEFT_DROID_UNPACK_Y: i32 = MTT_Y + 50;
pub const RIGHT_DROID_UNPACK_X: i32 = MTT_X + 139;
pub const RIGHT_DROID_UNPACK_Y: i32 = MTT_Y + 48;
pub const LEFT_DROID_STAND_X: i32 = MTT_X - 95 - 1;
pub const LEFT_DROID_STAND_Y: i32 = MTT_Y + 50 - 5;
pub const RIGHT_DROID_STAND_X: i32 = MTT_X + 265 - 8;
pub const RIGHT_DROID_STAND_Y: i32 = MTT_Y + 50 - 6;
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
    pub stap_combo_window_ms: u32,
    pub stap_hits: u32,
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
            stap_combo_window_ms: 0,
            stap_hits: 0,
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

    /// Applies a STAP / ramp / laser hit and returns the authentic score award (`200k`, `400k`, or `1M` within 0.8s).
    pub fn record_stap_hit(&mut self, is_right: bool, is_laser: bool) -> u64 {
        let pts = if self.stap_combo_window_ms > 0 {
            1_000_000
        } else if self.stap_hits == 0 {
            200_000
        } else {
            400_000
        };
        self.stap_hits = self.stap_hits.saturating_add(1);
        self.stap_combo_window_ms = 800;
        self.staps_destroyed = self.staps_destroyed.saturating_add(1);
        self.stap_exploding_ms = 350;
        self.stap_side_right = is_right;
        self.last_hit_laser = is_laser;
        self.explode_side_droid(is_right);
        pts
    }

    pub fn explode_side_droid(&mut self, is_right: bool) {
        if is_right {
            self.right_droid = DroidSideState::Exploding;
            self.right_droid_ms = 0;
        } else {
            self.left_droid = DroidSideState::Exploding;
            self.left_droid_ms = 0;
        }
    }
}

impl Default for BattleDroidState {
    fn default() -> Self {
        Self::new()
    }
}
