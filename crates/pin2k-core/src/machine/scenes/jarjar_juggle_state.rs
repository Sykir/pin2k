//! Data structures & weighted award picker for `JarjarJuggleScene` (`0x00137e64..0x00139cbc`).

use serde::{Deserialize, Serialize};

/// Exact ROM lane anchor X coordinates (`DeffJarjarJuggleScene::Run`, `0x13849e..0x1384c0`).
pub const LANE_X: [i32; 5] = [100, 175, 285, 400, 475];
/// Exact ROM lane anchor Y coordinates (`DeffJarjarJuggleScene::Run`, `0x1384c7..0x1384ef`).
pub const LANE_Y: [i32; 5] = [225, 240, 240, 240, 225];

/// Authentic 7-entry `ra_juggle_random_award_table` (`0x0013af98`) + `junk_table_data` (`0x0033dd88`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JuggleAward {
    Points500k,
    Points1m,
    Points1_5m,
    Points2m,
    FiveLasers,
    IncreaseBonusX,
    LightExtraBall,
}

impl JuggleAward {
    const TABLE: [(JuggleAward, u32); 7] = [
        (JuggleAward::Points500k, 30),
        (JuggleAward::Points1m, 30),
        (JuggleAward::Points1_5m, 20),
        (JuggleAward::Points2m, 20),
        (JuggleAward::FiveLasers, 20),
        (JuggleAward::IncreaseBonusX, 25),
        (JuggleAward::LightExtraBall, 30),
    ];

    pub const fn sprite_info(self) -> (&'static str, i32, i32) {
        match self {
            Self::Points500k => ("anim_jarjar_junk1", 96, 46),
            Self::Points1m => ("anim_jarjar_junk2", 126, 60),
            Self::Points1_5m => ("anim_jarjar_junk3", 112, 44),
            Self::Points2m => ("anim_jarjar_junk4", 88, 58),
            Self::FiveLasers => ("anim_jarjar_junk5", 103, 38),
            Self::IncreaseBonusX => ("anim_jarjar_junk6", 124, 63),
            Self::LightExtraBall => ("anim_jarjar_junk_eb", 116, 61),
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Points500k => "500,000",
            Self::Points1m => "1,000,000",
            Self::Points1_5m => "1,500,000",
            Self::Points2m => "2,000,000",
            Self::FiveLasers => "+5 LASERS",
            Self::IncreaseBonusX => "BONUS +1X",
            Self::LightExtraBall => "EXTRA BALL!",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JuggleAnimState {
    Juggle1,
    Juggle2,
    Fall1,
    Fall2,
    JumpUp,
    Switch,
    Stare,
}

impl JuggleAnimState {
    /// `(anim_name, width, height, frames, ms_per_frame)` from `gfx_manifest.json` & `sleep100()`.
    pub const fn spec(self) -> (&'static str, i32, i32, usize, u32) {
        match self {
            Self::Juggle1 => ("anim_jarjar_juggle", 407, 191, 19, 50),
            Self::Juggle2 => ("anim_jarjar_juggle_2", 466, 213, 32, 50),
            Self::Fall1 => ("anim_jarjar_fall", 470, 240, 20, 45),
            Self::Fall2 => ("anim_jarjar_fall_2", 318, 220, 18, 45),
            Self::JumpUp => ("anim_jarjar_jumpup", 400, 213, 40, 40),
            Self::Switch => ("anim_jarjar_switch", 189, 209, 4, 50),
            Self::Stare => ("anim_jarjar_stare", 189, 209, 11, 90),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JuggleLane {
    pub award: JuggleAward,
    pub collected: bool,
    pub flying_ms: u32,
    pub x_q8: i32,
    pub y_q8: i32,
    pub vx_q8: i32,
    pub vy_q8: i32,
    pub award_text_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JarjarJuggleState {
    pub lanes: [JuggleLane; 5],
    /// `false` while Jar Jar is juggling items in the air; `true` once dropped into lanes.
    pub dropped: bool,
    pub anim: JuggleAnimState,
    pub anim_ms: u32,
    pub juggle_cycles: u8,
    pub intro_textbox_ms: u32,
    pub remaining: u8,
}

impl JarjarJuggleState {
    /// Picks 5 weighted awards (`ra_juggle_random_award`, max 1 `LightExtraBall`).
    pub fn new(seed: u64) -> Self {
        let mut rng = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let mut eb_used = false;
        let mut awards = [JuggleAward::Points500k; 5];
        for slot in &mut awards {
            let total_w: u32 = JuggleAward::TABLE
                .iter()
                .filter(|(a, _)| !eb_used || *a != JuggleAward::LightExtraBall)
                .map(|(_, w)| *w)
                .sum();
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            let mut pick = ((rng >> 32) as u32) % total_w;
            let mut chosen = JuggleAward::Points500k;
            for &(aw, w) in &JuggleAward::TABLE {
                if eb_used && aw == JuggleAward::LightExtraBall {
                    continue;
                }
                if pick < w {
                    chosen = aw;
                    break;
                }
                pick -= w;
            }
            if chosen == JuggleAward::LightExtraBall {
                eb_used = true;
            }
            *slot = chosen;
        }
        let lanes = std::array::from_fn(|i| JuggleLane {
            award: awards[i],
            collected: false,
            flying_ms: 0,
            x_q8: LANE_X[i] << 8,
            y_q8: (LANE_Y[i] - 48) << 8,
            vx_q8: 0,
            vy_q8: 0,
            award_text_ms: 0,
        });
        Self {
            lanes,
            dropped: false,
            anim: JuggleAnimState::Juggle1,
            anim_ms: 0,
            juggle_cycles: 0,
            intro_textbox_ms: 3_500,
            remaining: 5,
        }
    }
}
