//! State & ROM coordinates for `QueenFashionScene` / `DeffQueenFashionSceneBackground` (`0x00118d10..0x0011ae00`).

use serde::{Deserialize, Serialize};

pub const FASHION_ICONS: [&str; 7] = [
    "anim_fashion_icon_black",
    "anim_fashion_icon_fanhead",
    "anim_fashion_icon_hornhead",
    "anim_fashion_icon_nun",
    "anim_fashion_icon_spade",
    "anim_fashion_icon_victorian",
    "anim_fashion_icon_extra_ball",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueenFashionState {
    pub wheel_pos: usize,
    pub outfits_remaining: u8,
    pub left_collected: bool,
    pub right_collected: bool,
    pub flash_ms: u32,
    pub spin_ms: u32,
}

impl QueenFashionState {
    pub fn new() -> Self {
        Self {
            wheel_pos: 0,
            outfits_remaining: 3,
            left_collected: false,
            right_collected: false,
            flash_ms: 0,
            spin_ms: 0,
        }
    }
}

impl Default for QueenFashionState {
    fn default() -> Self {
        Self::new()
    }
}
