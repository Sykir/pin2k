//! State & ROM constants for `ProbeDroidScene` / `DeffProbeDroidSceneBackground` (`0x00164f4c..0x00167b00`).

use serde::{Deserialize, Serialize};

pub const PROBE_MISS_SCORE: u64 = 70_000;
pub const SHOT_COOLDOWN_MS: u32 = 480;
pub const TARGET_MIN_X: i32 = 28;
pub const TARGET_MAX_X: i32 = 440;
/// ~1.0s per sweep across 412px (`livre_regles_swe1_14_essentiel.md`).
pub const TARGET_SPEED_PX_S: i32 = 412;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeDroidState {
    pub attempts: u32,
    pub probes_remaining: u8,
    pub probes_missed: u32,
    pub shot_cooldown_ms: u32,
    pub target_x: i32,
    pub target_dir: i32,
    pub scroll_x: i32,
    pub fly_ms: u32,
    pub queen_shoot_ms: u32,
    pub explosion_ms: u32,
    pub miss_blast_ms: u32,
    pub hit_variant: u8,
}

impl ProbeDroidState {
    pub fn new(attempts: u32) -> Self {
        Self {
            attempts: attempts.max(1),
            probes_remaining: 3,
            probes_missed: 0,
            shot_cooldown_ms: 0,
            target_x: 258,
            target_dir: 1,
            scroll_x: 0,
            fly_ms: 0,
            queen_shoot_ms: 0,
            explosion_ms: 0,
            miss_blast_ms: 0,
            hit_variant: 0,
        }
    }
}

impl Default for ProbeDroidState {
    fn default() -> Self {
        Self::new(1)
    }
}
