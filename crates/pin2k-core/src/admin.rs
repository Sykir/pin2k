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
    DebugLauncher { cursor: u8 },
}

pub const DEBUG_LAUNCH_PRESETS: &[(&str, &str)] = &[
    ("game:swe1", "GAME: STAR WARS EP I"),
    ("game:rfm", "GAME: REVENGE FROM MARS"),
    ("boot", "STATE: BOOT SEQUENCE"),
    ("attract", "STATE: ATTRACT MODE"),
    ("ingame", "STATE: IN-GAME (BALL 1)"),
    ("skill_shot", "MODE: SKILL SHOT"),
    ("hangar_multiball", "MODE: HANGAR MULTIBALL"),
    ("darth_maul_duel", "MODE: DARTH MAUL DUEL"),
    ("hstd_entry", "STATE: HIGH SCORE ENTRY"),
    ("scene:0", "SWE1 01: BATTLE DROID"),
    ("scene:1", "SWE1 02: DESTROYER DROID"),
    ("scene:2", "SWE1 03: FED FIGHTERS"),
    ("scene:3", "SWE1 04: GUNGAN BATTLE"),
    ("scene:4", "SWE1 05: HANGAR ESCAPE"),
    ("scene:5", "SWE1 06: HOVER TANK"),
    ("scene:6", "SWE1 07: PODRACE"),
    ("scene:7", "SWE1 08: PROBE DROID"),
    ("scene:8", "SWE1 09: QUEEN FASHION"),
    ("scene:9", "SWE1 10: JAR JAR JUGGLE"),
    ("scene:10", "SWE1 11: WATTO CHANCE"),
    ("scene:11", "SWE1 12: MOS ESPA CANTINA"),
    ("rfm_wave:0", "RFM 01: MARS KNEADS WOMEN"),
    ("rfm_wave:1", "RFM 02: BIG-O-BEAM"),
    ("rfm_wave:2", "RFM 03: PARIS IN PERIL"),
    ("rfm_wave:3", "RFM 04: TOWER OF TERROR"),
    ("rfm_wave:4", "RFM 05: LEANING TOWER"),
    ("rfm_wave:5", "RFM 06: SECRET WEAPON"),
    ("rfm_wave:6", "RFM 07: ALIEN DRIVE-IN"),
    ("rfm_wave:7", "RFM 08: MARTIAN BOWLING"),
    ("rfm_wave:8", "RFM 09: ATTACK MARS!"),
    ("capture_multiball", "RFM: CAPTURE MULTIBALL"),
    ("martian_attack", "RFM: MARTIAN ATTACK"),
];

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
