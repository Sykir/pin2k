//! Pinball 2000 Operator Administration Menu (`Diagnostics`, `Audits`, `Adjustments`, `Utilities`)
//! and persistent BAR2 NVRAM settings.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub const fn label(self) -> &'static str {
        match self {
            Difficulty::Easy => "EASY",
            Difficulty::Medium => "MEDIUM",
            Difficulty::Hard => "HARD",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }
}

/// Persistent operator adjustments (`Adjustments` menu).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameAdjustments {
    pub free_play: bool,
    pub balls_per_game: u8,
    pub ball_save_secs: u8,
    pub tilt_warnings_max: u8,
    pub master_volume_pct: u8,
    pub difficulty: Difficulty,
}

impl Default for GameAdjustments {
    fn default() -> Self {
        Self {
            free_play: true,
            balls_per_game: 3,
            ball_save_secs: 15,
            tilt_warnings_max: 3,
            master_volume_pct: 85,
            difficulty: Difficulty::Medium,
        }
    }
}

/// Live machine audit counters (`Audits` menu).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameAudits {
    pub total_games_played: u32,
    pub total_balls_played: u32,
    pub total_credits: u32,
    pub scenes_started: u32,
    pub scenes_completed: u32,
    pub multiballs_started: u32,
    pub maul_duels_won: u32,
    pub highest_score: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdminScreen {
    Root { cursor: u8 },
    DiagnosticsMenu { cursor: u8 },
    SwitchTest,
    CoilTest { selected_drive: u8 },
    LampTest { selected_lamp: u8 },
    SoundTest { selected_idx: u8 },
    Audits { cursor: u8 },
    Adjustments { cursor: u8 },
    Utilities { cursor: u8 },
}

impl Default for AdminScreen {
    fn default() -> Self {
        Self::Root { cursor: 0 }
    }
}

pub const ADMIN_SOUND_PRESETS: &[(u16, &str)] = &[
    (0x003A, "BOOT BONG"),
    (0x0001, "MAIN THEME"),
    (0x0002, "BATTLE MUSIC"),
    (0x0006, "PODRACE LOOP"),
    (0x0007, "DUEL OF THE FATES"),
    (0x0009, "HANGAR MULTIBALL"),
    (0x000D, "LASER FIRE"),
    (0x0014, "SABER CLASH"),
    (0x0015, "EXPLOSION"),
];
