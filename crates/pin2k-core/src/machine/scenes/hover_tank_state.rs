//! State & ROM constants for `HoverTankScene` (`0x0012fc50..0x00132500`).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TankLane {
    Left,
    Center,
    Right,
}

impl TankLane {
    pub const fn anim_enter(self) -> &'static str {
        match self {
            Self::Left => "anim_hover_tank_l_enter",
            Self::Center => "anim_hover_tank_c_enter",
            Self::Right => "anim_hover_tank_r_enter",
        }
    }

    pub const fn anim_float(self) -> &'static str {
        match self {
            Self::Left => "anim_hover_tank_l_float",
            Self::Center => "anim_hover_tank_c_float",
            Self::Right => "anim_hover_tank_r_float",
        }
    }

    pub const fn anim_shoot(self) -> &'static str {
        match self {
            Self::Left => "anim_hover_tank_l_shoot",
            Self::Center => "anim_hover_tank_c_shoot",
            Self::Right => "anim_hover_tank_r_shoot",
        }
    }

    pub const fn anim_explode(self) -> &'static str {
        match self {
            Self::Left => "anim_hover_tank_l_explode",
            Self::Center => "anim_hover_tank_c_explode",
            Self::Right => "anim_hover_tank_r_explode",
        }
    }

    pub const fn pos(self) -> (i32, i32, u8) {
        match self {
            Self::Left => (60, 72, 3),
            Self::Center => (233, 66, 4),
            Self::Right => (440, 72, 3),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TankPhase {
    Entering,
    Floating,
    Shooting,
    Exploding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverTankSlot {
    pub lane: TankLane,
    pub phase: TankPhase,
    pub phase_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoverTankState {
    pub tanks_remaining: u8,
    pub catapult_x: i32,
    pub catapult_dir: i32,
    pub catapult_shoot_ms: u32,
    pub slots: [HoverTankSlot; 3],
}

impl HoverTankState {
    pub fn new() -> Self {
        Self {
            tanks_remaining: 6,
            catapult_x: 240,
            catapult_dir: 1,
            catapult_shoot_ms: 0,
            slots: [
                HoverTankSlot {
                    lane: TankLane::Left,
                    phase: TankPhase::Entering,
                    phase_ms: 0,
                },
                HoverTankSlot {
                    lane: TankLane::Center,
                    phase: TankPhase::Floating,
                    phase_ms: 0,
                },
                HoverTankSlot {
                    lane: TankLane::Right,
                    phase: TankPhase::Entering,
                    phase_ms: 150,
                },
            ],
        }
    }
}

impl Default for HoverTankState {
    fn default() -> Self {
        Self::new()
    }
}
