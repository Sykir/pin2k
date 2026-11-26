//! State & ROM constants for `QuestionMarkScene` (`0x00169060..0x00169870`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MysteryAward {
    Points1M,
    Lasers3,
    Bonus2X,
    ExtraBall,
}

impl MysteryAward {
    pub const fn from_seed(seed: u64) -> Self {
        match (seed / 16) % 4 {
            0 => Self::Points1M,
            1 => Self::Lasers3,
            2 => Self::Bonus2X,
            _ => Self::ExtraBall,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Points1M => "1,000,000 POINTS!",
            Self::Lasers3 => "+3 LASER SHOTS!",
            Self::Bonus2X => "BONUS +2X!",
            Self::ExtraBall => "EXTRA BALL!",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionMarkState {
    pub award: MysteryAward,
    pub elapsed_ms: u32,
}

impl QuestionMarkState {
    pub fn new(seed: u64) -> Self {
        Self {
            award: MysteryAward::from_seed(seed),
            elapsed_ms: 0,
        }
    }
}

impl Default for QuestionMarkState {
    fn default() -> Self {
        Self::new(0)
    }
}
