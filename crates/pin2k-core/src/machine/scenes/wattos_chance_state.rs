//! State & ROM coordinates for `WattosChanceScene` / `DeffWattosChanceSceneBackground` (`0x00181520..0x00183600`).

use serde::{Deserialize, Serialize};

/// Exact ROM cube positions (`DeffWattosChanceSceneBackground::Run`, `0x001820fc`).
pub const CUBE_X: [i32; 3] = [45, 222, 399];
pub const CUBE_Y: [i32; 3] = [171, 175, 171];

/// Watto 7-segment animation timeline (`anim_watto_scene_*`, `duration_ms`, `x`, `y`, `loops`, `active_lane`).
pub const WATTO_TIMELINE: [(&str, u32, i32, i32, bool, usize); 7] = [
    ("anim_watto_scene_left_loop", 1_000, 30, 46, true, 0),
    ("anim_watto_scene_left_to_middle", 167, 100, 40, false, 1),
    ("anim_watto_scene_hand_to_chin_middle", 250, 233, 40, false, 1),
    ("anim_watto_scene_chin_loop_middle", 1_000, 233, 44, true, 1),
    ("anim_watto_scene_hand_down_middle", 233, 233, 40, false, 1),
    ("anim_watto_scene_middle_to_right", 233, 160, 44, false, 2),
    ("anim_watto_scene_right_loop", 1_000, 400, 46, true, 2),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WattosChanceState {
    pub hits_remaining: u8,
    pub timeline_ms: u32,
    pub active_lane: usize,
    pub explode_ms: u32,
    pub explode_lane: usize,
    pub miss_ms: u32,
    pub miss_lane: usize,
}

impl WattosChanceState {
    pub fn new() -> Self {
        Self {
            hits_remaining: 3,
            timeline_ms: 0,
            active_lane: 0,
            explode_ms: 0,
            explode_lane: 0,
            miss_ms: 0,
            miss_lane: 0,
        }
    }

    pub fn current_segment(&self) -> (&'static str, u32, i32, i32, bool, usize) {
        let total: u32 = WATTO_TIMELINE.iter().map(|s| s.1).sum();
        let mut t = self.timeline_ms % total;
        for seg in WATTO_TIMELINE {
            if t < seg.1 {
                return seg;
            }
            t -= seg.1;
        }
        WATTO_TIMELINE[0]
    }
}

impl Default for WattosChanceState {
    fn default() -> Self {
        Self::new()
    }
}
