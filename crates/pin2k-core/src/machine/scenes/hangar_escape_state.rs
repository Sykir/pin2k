//! State & ROM constants for `HangarEscapeScene` / `DeffHangarEscapeSceneBackground` (`0x0012a6e4..0x0012dc00`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HangarEscapeState {
    pub droids_remaining: u8,
    pub droids_missed: u32,
    pub hit_variant: u8,
    pub explosion_ms: u32,
    pub zap_ms: u32,
}

impl HangarEscapeState {
    pub fn new(attempts: u32) -> Self {
        let droids = (attempts.saturating_sub(1) * 6 + 8).min(14) as u8;
        Self {
            droids_remaining: droids,
            droids_missed: 0,
            hit_variant: 0,
            explosion_ms: 0,
            zap_ms: 0,
        }
    }
}

impl Default for HangarEscapeState {
    fn default() -> Self {
        Self::new(1)
    }
}
