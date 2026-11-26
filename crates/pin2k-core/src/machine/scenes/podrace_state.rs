//! Data structures, probability tables, and animation state for `PodraceScene` (`0x0015fcb0..0x001644a0`).

use serde::{Deserialize, Serialize};

/// Exact ROM lane anchor X coordinates (`DeffPodRaceSceneBackground::podrace_icon_process`, `0x163068`).
pub const PODRACE_LANE_X: [i32; 3] = [100, 285, 470];
/// Exact ROM lane anchor Y coordinates (`DeffPodRaceSceneBackground::podrace_icon_process`, `0x163068`).
pub const PODRACE_LANE_Y: [i32; 3] = [230, 235, 230];

/// Exact ROM checkpoint names (`me_podrace_checkpoint_1..5`, `0x1524cc..0x15254c`).
pub const CHECKPOINT_NAMES: [&str; 5] = [
    "BEGGAR'S CANYON",
    "CRATER VALLEY",
    "DEVIL'S DOORKNOB",
    "DROOPY DROP",
    "CHECKPOINT 5",
];

/// Early icon probability table (`0x00163e28`, `checkpoints < 6`).
const ICON_TABLE_EARLY: [u8; 16] = [1, 2, 3, 4, 1, 2, 4, 4, 4, 2, 3, 4, 1, 2, 3, 4];
/// Mid icon probability table (`0x00163e68`, `6 <= checkpoints < 12`).
const ICON_TABLE_MID: [u8; 16] = [1, 2, 3, 4, 4, 4, 4, 4, 1, 2, 3, 4, 1, 2, 3, 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PodracePhase {
    /// `anim_podrace_starting` (320x240 scaled 2x, 60 frames @ 50ms = 3,000ms).
    StartingGate,
    /// `anim_podrace_closeup` (640x240, 31 frames @ 25ms = 775ms).
    Closeup,
    /// Active canyon racing (`anim_podrace_1` + horizon + ground + opponents + lane icons).
    Racing,
    /// Completion zoom (`anim_podrace_anakin_zoom`, 20 frames @ 50ms = 1,000ms).
    ZoomOut,
}

/// Icon types placed in the 3 lanes (`podrace_icon_process`, `0x163068`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PodraceIconKind {
    /// ROM ID `1`: `anim_podrace_icon2` (70x30), +50 MPH.
    Speed50A,
    /// ROM ID `2`: `anim_podrace_icon3` (70x30), +100 MPH.
    Speed100,
    /// ROM ID `3`: `anim_podrace_icon1` (70x30), +50 MPH.
    Speed50B,
    /// ROM ID `4`: `anim_podrace_icon4` (87x52), Checkpoint!
    Checkpoint,
}

impl PodraceIconKind {
    pub const fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(Self::Speed50A),
            2 => Some(Self::Speed100),
            3 => Some(Self::Speed50B),
            4 => Some(Self::Checkpoint),
            _ => None,
        }
    }

    pub const fn sprite_info(self) -> (&'static str, &'static str, i32, i32) {
        match self {
            Self::Speed50A => ("anim_podrace_icon2", "anim_podrace_icon2_blur", 70, 30),
            Self::Speed100 => ("anim_podrace_icon3", "anim_podrace_icon3_blur", 70, 30),
            Self::Speed50B => ("anim_podrace_icon1", "anim_podrace_icon1_blur", 70, 30),
            Self::Checkpoint => ("anim_podrace_icon4", "anim_podrace_icon4_blur", 87, 52),
        }
    }

    pub const fn speed_boost(self) -> u32 {
        match self {
            Self::Speed50A | Self::Speed50B => 50,
            Self::Speed100 => 100,
            Self::Checkpoint => 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodraceLane {
    pub icon: Option<PodraceIconKind>,
    pub anim_ms: u32,
    pub flying_ms: u32,
    pub flying_kind: Option<PodraceIconKind>,
    pub x_q8: i32,
    pub y_q8: i32,
    pub vx_q8: i32,
    pub vy_q8: i32,
}

impl PodraceLane {
    pub const fn empty() -> Self {
        Self {
            icon: None,
            anim_ms: 0,
            flying_ms: 0,
            flying_kind: None,
            x_q8: 0,
            y_q8: 0,
            vx_q8: 0,
            vy_q8: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodraceOpponent {
    /// ROM opponent kind `1..=4`.
    pub kind: u8,
    pub x_q8: i32,
    pub y_q8: i32,
    pub dir_right: bool,
    pub anim_ms: u32,
    pub exploding_ms: u32,
    pub vx_q8: i32,
    pub vy_q8: i32,
    pub ax_q8: i32,
    pub ay_q8: i32,
}

impl PodraceOpponent {
    pub fn sprite_info(&self) -> (&'static str, &'static str, i32, i32) {
        match self.kind {
            2 => ("anim_podrace_3", "anim_podrace_3_explosion", 60, 21),
            4 => ("anim_podrace_4", "anim_podrace_3_explosion", 65, 22),
            _ => ("anim_podrace_2", "anim_podrace_2_explosion", 147, 43),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodraceState {
    pub phase: PodracePhase,
    pub phase_ms: u32,
    pub checkpoints: u8,
    pub speed_mph: u32,
    pub speed_decay_ms: u32,
    pub first_icon_placed: bool,
    pub lanes: [PodraceLane; 3],
    pub opponent: Option<PodraceOpponent>,
    pub anakin_bank: i8, // -1 = left, 0 = center, 1 = right
    pub anakin_bank_ms: u32,
    pub horizon_x: i32,
    pub horizon_target: i32,
    pub checkpoint_banner_ms: u32,
    pub instruct_ms: u32,
    pub debounce_ms: u32,
    pub speech_toggle: bool,
    pub rng: u64,
}

impl PodraceState {
    pub fn new(seed: u64) -> Self {
        let rng = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        Self {
            phase: PodracePhase::StartingGate,
            phase_ms: 0,
            checkpoints: 0,
            speed_mph: 150,
            speed_decay_ms: 0,
            first_icon_placed: false,
            lanes: [PodraceLane::empty(), PodraceLane::empty(), PodraceLane::empty()],
            opponent: None,
            anakin_bank: 0,
            anakin_bank_ms: 0,
            horizon_x: 50,
            horizon_target: 80,
            checkpoint_banner_ms: 0,
            instruct_ms: 3_500,
            debounce_ms: 0,
            speech_toggle: false,
            rng,
        }
    }

    pub fn next_rand(&mut self) -> u32 {
        self.rng = self.rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.rng >> 32) as u32
    }

    /// Places an icon in a random empty lane using ROM probability tables (`place_icon`, `0x163efc`).
    pub fn try_place_icon(&mut self) {
        let start = (self.next_rand() % 3) as usize;
        let mut chosen_lane = None;
        for off in 0..3 {
            let idx = (start + off) % 3;
            if self.lanes[idx].icon.is_none() && self.lanes[idx].flying_ms == 0 {
                chosen_lane = Some(idx);
                break;
            }
        }
        let Some(idx) = chosen_lane else { return };
        let id = if !self.first_icon_placed {
            self.first_icon_placed = true;
            4
        } else if self.checkpoints < 6 {
            ICON_TABLE_EARLY[(self.next_rand() % 16) as usize]
        } else if self.checkpoints < 12 {
            ICON_TABLE_MID[(self.next_rand() % 16) as usize]
        } else {
            4
        };
        self.lanes[idx].icon = PodraceIconKind::from_id(id);
        self.lanes[idx].anim_ms = 0;
    }

    pub fn try_place_opponent(&mut self, initial: bool) {
        if self.opponent.is_some() {
            return;
        }
        let kind = if initial { 1 } else { ((self.next_rand() % 4) + 1) as u8 };
        let w = match kind {
            2 => 60,
            4 => 65,
            _ => 147,
        };
        let h = match kind {
            2 => 21,
            4 => 22,
            _ => 43,
        };
        let half_w = w / 2;
        let (x, y, dir_right) = if initial {
            (285 - half_w, (115 - h / 2) - 5, false)
        } else {
            let right = (self.next_rand() % 2) == 1;
            let start_x = if right { -147 } else { 640 };
            let start_y = 115 + (self.next_rand() % 10) as i32;
            (start_x, start_y, right)
        };
        self.opponent = Some(PodraceOpponent {
            kind,
            x_q8: x << 8,
            y_q8: y << 8,
            dir_right,
            anim_ms: 0,
            exploding_ms: 0,
            vx_q8: 0,
            vy_q8: 0,
            ax_q8: 0,
            ay_q8: 0,
        });
    }

    pub fn checkpoint_label(&self) -> String {
        if self.checkpoints >= 1 && (self.checkpoints as usize) <= CHECKPOINT_NAMES.len() {
            CHECKPOINT_NAMES[(self.checkpoints - 1) as usize].to_string()
        } else {
            format!("CHECKPOINT {}", self.checkpoints)
        }
    }
}
