//! State & ROM constants for `FederationFighterScene` (`0x0011b074..0x0011e500`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FighterPhase {
    Approaching,
    RedEye,
    Flyby,
    Exploding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationFighterState {
    pub fighters_remaining: u8,
    pub fighters_missed: u32,
    pub phase: FighterPhase,
    pub phase_ms: u32,
    pub scroll_x: i32,
    pub zap_ms: u32,
    pub last_hit_laser: bool,
    pub hit_variant: u8,
}

impl FederationFighterState {
    pub fn new() -> Self {
        Self {
            fighters_remaining: 9,
            fighters_missed: 0,
            phase: FighterPhase::Approaching,
            phase_ms: 0,
            scroll_x: 0,
            zap_ms: 0,
            last_hit_laser: false,
            hit_variant: 0,
        }
    }
}

impl Default for FederationFighterState {
    fn default() -> Self {
        Self::new()
    }
}
