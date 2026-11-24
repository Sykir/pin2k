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

/// Exact scoring and rule constants extracted from `rfm_0195.elf` (RFM 0195 / v1.09).
pub mod scoring {
    pub const SKILL_SHOT_BASE: u64 = 500_000;
    pub const SKILL_SHOT_STEP: u64 = 100_000;
    pub const SKILL_SHOT_CAP: u64 = 1_500_000;

    pub const SUPER_SKILL_SHOT_BASE: u64 = 1_000_000;
    pub const SUPER_SKILL_SHOT_STEP: u64 = 200_000;
    pub const SUPER_SKILL_SHOT_CAP: u64 = 3_000_000;

    pub const MARTIAN_ATTACK_BASE: u64 = 1_250_000;
    pub const MARTIAN_ATTACK_STEP: u64 = 250_000;
    pub const MARTIAN_ATTACK_CAP: u64 = 2_500_000;

    pub const CAPTURE_LOCK_LIT: u64 = 25_000;
    pub const CAPTURE_LOCK_AWARD: u64 = 50_000;
    pub const CAPTURE_MB_START: u64 = 100_000;

    pub const HYPNO_BEAM_HIT: u64 = 250_000;
    pub const HYPNO_BEAM_JACKPOT_BASE: u64 = 10_000_000;
    pub const HYPNO_BEAM_JACKPOT_STEP: u64 = 5_000_000;
    pub const HYPNO_BEAM_JACKPOT_CAP: u64 = 50_000_000;

    /// `SkillShots::calc_skill_shot_value(n)` (`0x1a3658`).
    pub const fn skill_shot_value(n: u32) -> u64 {
        let idx = n.saturating_sub(1) as u64;
        let val = SKILL_SHOT_BASE + idx * SKILL_SHOT_STEP;
        if val > SKILL_SHOT_CAP { SKILL_SHOT_CAP } else { val }
    }

    /// `SkillShots::calc_super_skill_shot_value(n)` (`0x1a3968`).
    pub const fn super_skill_shot_value(n: u32) -> u64 {
        let idx = n.saturating_sub(1) as u64;
        let val = SUPER_SKILL_SHOT_BASE + idx * SUPER_SKILL_SHOT_STEP;
        if val > SUPER_SKILL_SHOT_CAP { SUPER_SKILL_SHOT_CAP } else { val }
    }

    /// `MartianAttack::setup_martian_value(n)` (`0x1129a0`).
    pub const fn martian_attack_value(n: u32) -> u64 {
        let idx = n.saturating_sub(1) as u64;
        let val = MARTIAN_ATTACK_BASE + idx * MARTIAN_ATTACK_STEP;
        if val > MARTIAN_ATTACK_CAP { MARTIAN_ATTACK_CAP } else { val }
    }

    /// `HypnoBeamMultiball::award_beam_hit` (`0x19edd0`).
    pub const fn hypno_beam_jackpot(jackpots: u32) -> u64 {
        let val = HYPNO_BEAM_JACKPOT_BASE + (jackpots as u64) * HYPNO_BEAM_JACKPOT_STEP;
        if val > HYPNO_BEAM_JACKPOT_CAP { HYPNO_BEAM_JACKPOT_CAP } else { val }
    }
}

/// Qualification flags for `AttackMars` (`AttackMars::ok_for_qualify` at `0x191458`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttackMarsQualifyFlags {
    pub all_modes: bool,
    pub mothership: bool,
    pub all_saucer_lights: bool,
}

impl AttackMarsQualifyFlags {
    pub const fn is_ready(&self) -> bool {
        self.all_modes && self.mothership && self.all_saucer_lights
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rfm_extracted_scoring_formulas() {
        assert_eq!(scoring::skill_shot_value(1), 500_000);
        assert_eq!(scoring::skill_shot_value(5), 900_000);
        assert_eq!(scoring::skill_shot_value(99), 1_500_000);

        assert_eq!(scoring::super_skill_shot_value(1), 1_000_000);
        assert_eq!(scoring::super_skill_shot_value(15), 3_000_000);

        assert_eq!(scoring::martian_attack_value(1), 1_250_000);
        assert_eq!(scoring::martian_attack_value(10), 2_500_000);

        assert_eq!(scoring::hypno_beam_jackpot(0), 10_000_000);
        assert_eq!(scoring::hypno_beam_jackpot(2), 20_000_000);
        assert_eq!(scoring::hypno_beam_jackpot(20), 50_000_000);

        let mut flags = AttackMarsQualifyFlags::default();
        assert!(!flags.is_ready());
        flags.all_modes = true;
        flags.mothership = true;
        assert!(!flags.is_ready());
        flags.all_saucer_lights = true;
        assert!(flags.is_ready());
    }
}
