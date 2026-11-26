//! State & ROM constants for `GunganScene` / `DeffSubEscapeSceneBackground` (`0x001271c4..0x00129b00`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GunganBattleState {
    /// Sub escape distance (`0..=10`, starts at `5` in `GunganScene::start`).
    pub distance: u8,
    pub speech_counter: u8,
    pub boost_flash_ms: u32,
    pub sando_ms: u32,
}

impl GunganBattleState {
    pub fn new() -> Self {
        Self {
            distance: 5,
            speech_counter: 0,
            boost_flash_ms: 0,
            sando_ms: 0,
        }
    }
}

impl Default for GunganBattleState {
    fn default() -> Self {
        Self::new()
    }
}
