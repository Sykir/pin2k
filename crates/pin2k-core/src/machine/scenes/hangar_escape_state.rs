//! State & ROM constants for `HangarEscapeScene` / `DeffHangarEscapeSceneBackground` (`0x0012a6e4..0x0012dc00`).

use serde::{Deserialize, Serialize};

pub const DROID_HIT_SCORE: u64 = 200_000;
pub const TORPEDO_MISS_SCORE: u64 = 20_000;

/// Active droid lane: `0` = Left, `1` = Center, `2` = Right.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HangarEscapeState {
    pub attempts: u32,
    pub droids_remaining: u8,
    pub droids_missed: u32,
    /// Number of droids currently visible in `[left, center, right]`.
    pub lanes: [u8; 3],
    pub hit_variant: u8,
    pub last_hit_lane: usize,
    pub explosion_ms: u32,
    pub zap_ms: u32,
    pub rng_state: u32,
}

impl HangarEscapeState {
    pub fn new(attempts: u32) -> Self {
        let clamped = attempts.max(1);
        let droids = ((clamped - 1) * 6 + 8).min(14) as u8;
        // First droid always enters at Center alone (`lanes = [0, 1, 0]`)
        Self {
            attempts: clamped,
            droids_remaining: droids,
            droids_missed: 0,
            lanes: [0, 1, 0],
            hit_variant: 0,
            last_hit_lane: 1,
            explosion_ms: 0,
            zap_ms: 0,
            rng_state: 0x4841_4E47 ^ clamped,
        }
    }

    pub fn door_open(&self) -> bool {
        self.droids_remaining == 0
    }

    pub fn respawn_droids(&mut self) {
        let active: u8 = self.lanes.iter().sum();
        let unspawned = self.droids_remaining.saturating_sub(active);
        let target_active = self.droids_remaining.min(3);
        let mut to_spawn = target_active.saturating_sub(active).min(unspawned);
        while to_spawn > 0 {
            self.rng_state = self.rng_state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let lane = (self.rng_state as usize) % 3;
            // Prefer empty lanes first
            let chosen = (0..3)
                .map(|off| (lane + off) % 3)
                .find(|&l| self.lanes[l] == 0)
                .unwrap_or(lane);
            self.lanes[chosen] = self.lanes[chosen].saturating_add(1);
            to_spawn -= 1;
        }
    }
}

impl Default for HangarEscapeState {
    fn default() -> Self {
        Self::new(1)
    }
}
