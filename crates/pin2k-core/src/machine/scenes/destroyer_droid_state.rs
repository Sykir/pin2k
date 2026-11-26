//! State & ROM constants for `DestroyerDroidScene` (`0x00110450..0x00113d00`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DestroyerSize {
    Small,
    Medium,
    Large,
}

impl DestroyerSize {
    pub const fn anim_roll(self) -> &'static str {
        match self {
            Self::Small => "anim_destroyer_droid_s_roll",
            Self::Medium => "anim_destroyer_droid_m_roll",
            Self::Large => "anim_destroyer_droid_l_roll",
        }
    }

    pub const fn anim_idle(self) -> &'static str {
        match self {
            Self::Small => "anim_destroyer_droid_s_idle",
            Self::Medium => "anim_destroyer_droid_m_idle",
            Self::Large => "anim_destroyer_droid_l_idle",
        }
    }

    pub const fn anim_shooting(self) -> &'static str {
        match self {
            Self::Small => "anim_destroyer_droid_s_shooting",
            Self::Medium => "anim_destroyer_droid_m_shooting",
            Self::Large => "anim_destroyer_droid_l_shooting",
        }
    }

    pub const fn anim_explosion(self) -> &'static str {
        match self {
            Self::Small => "anim_destroyer_droid_s_explosion",
            Self::Medium => "anim_destroyer_droid_m_explosion",
            Self::Large => "anim_destroyer_droid_l_explosion",
        }
    }

    /// `(x, y, z)` position from `DeffDestroyerDroidSceneBackground::destroyer_droid_process`.
    pub const fn pos(self) -> (i32, i32, u8) {
        match self {
            Self::Small => (80, 92, 3),
            Self::Medium => (430, 82, 3),
            Self::Large => (20, 62, 5),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DestroyerDroidPhase {
    Rolling,
    Idle,
    Shooting,
    Exploding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestroyerSlot {
    pub size: DestroyerSize,
    pub phase: DestroyerDroidPhase,
    pub phase_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestroyerDroidState {
    pub destroyers_remaining: u8,
    pub destroyers_missed: u32,
    pub no_hits: bool,
    pub speech_counter: u8,
    /// Horizontal target cursor X (`anim_sith_probe_target`, 125x65 at y=100).
    pub target_x: i32,
    pub target_dir: i32,
    pub miss_blast_ms: u32,
    pub slots: [DestroyerSlot; 3],
}

impl DestroyerDroidState {
    pub fn new() -> Self {
        Self {
            destroyers_remaining: 6,
            destroyers_missed: 0,
            no_hits: true,
            speech_counter: 0,
            target_x: 258,
            target_dir: 1,
            miss_blast_ms: 0,
            slots: [
                DestroyerSlot {
                    size: DestroyerSize::Small,
                    phase: DestroyerDroidPhase::Rolling,
                    phase_ms: 0,
                },
                DestroyerSlot {
                    size: DestroyerSize::Medium,
                    phase: DestroyerDroidPhase::Idle,
                    phase_ms: 0,
                },
                DestroyerSlot {
                    size: DestroyerSize::Large,
                    phase: DestroyerDroidPhase::Idle,
                    phase_ms: 400,
                },
            ],
        }
    }
}

impl Default for DestroyerDroidState {
    fn default() -> Self {
        Self::new()
    }
}
