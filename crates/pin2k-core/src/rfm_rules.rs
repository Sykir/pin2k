//! *Revenge From Mars* (`#50070`) Saucer Waves, Martian Attack,
//! Capture Multiball, and Attack Mars progression.

use serde::{Deserialize, Serialize};

/// Supported Williams Pinball 2000 titles and Homebrew mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameTitle {
    /// *Star Wars Episode I* (`#50069`)
    Swe1,
    /// *Revenge From Mars* (`#50070`)
    Rfm,
    /// Standalone community Homebrew game (`mod.json`)
    Homebrew,
}

impl GameTitle {
    pub const fn title(self) -> &'static str {
        match self {
            GameTitle::Swe1 => "STAR WARS EPISODE I",
            GameTitle::Rfm => "REVENGE FROM MARS",
            GameTitle::Homebrew => "PINBALL 2000 HOMEBREW",
        }
    }
}

/// RFM Saucer Attack Waves (`9` waves -> `Attack Mars`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RfmWave {
    MarsKneadsWomen,
    BigOBeam,
    ParisInPeril,
    TowerOfTerror,
    LeaningTower,
    SecretWeapon,
    DriveIn,
    MartianBowling,
    AttackMars,
}

impl RfmWave {
    pub const ALL: [RfmWave; 9] = [
        RfmWave::MarsKneadsWomen,
        RfmWave::BigOBeam,
        RfmWave::ParisInPeril,
        RfmWave::TowerOfTerror,
        RfmWave::LeaningTower,
        RfmWave::SecretWeapon,
        RfmWave::DriveIn,
        RfmWave::MartianBowling,
        RfmWave::AttackMars,
    ];

    pub const fn title(self) -> &'static str {
        match self {
            RfmWave::MarsKneadsWomen => "MARS KNEADS WOMEN",
            RfmWave::BigOBeam => "BIG-O-BEAM",
            RfmWave::ParisInPeril => "PARIS IN PERIL",
            RfmWave::TowerOfTerror => "TOWER OF TERROR",
            RfmWave::LeaningTower => "LEANING TOWER",
            RfmWave::SecretWeapon => "SECRET WEAPON",
            RfmWave::DriveIn => "ALIEN DRIVE-IN",
            RfmWave::MartianBowling => "MARTIAN BOWLING",
            RfmWave::AttackMars => "ATTACK MARS!",
        }
    }

    pub const fn required_hits(self) -> u8 {
        match self {
            RfmWave::AttackMars => 8,
            _ => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RfmMultiball {
    None,
    MartianAttack,
    CaptureMultiball { jackpots: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfmState {
    pub martian_targets_mask: u8, // bits 0..6 (7 Martian targets)
    pub saucer_locks: u8,
    pub waves_completed: u16,
    pub active_wave: Option<RfmWave>,
    pub wave_hits_left: u8,
    pub multiball: RfmMultiball,
}

impl Default for RfmState {
    fn default() -> Self {
        Self {
            martian_targets_mask: 0,
            saucer_locks: 0,
            waves_completed: 0,
            active_wave: None,
            wave_hits_left: 0,
            multiball: RfmMultiball::None,
        }
    }
}
