//! Authentic `HoverTankScene` + `DeffHoverTankSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 6 AAT Hover Tanks (`tanks_remaining = 6`) across Left/Center/Right lanes.
//! - Sweeping Gungan catapult (`anim_hover_tank_catapult_idle`/`shoot`) + boomer ball.

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::hover_tank_state::{HoverTankState, TankLane, TankPhase};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_hover_tank_state(&mut self) {
        self.hover_tank = Some(HoverTankState::new());
    }

    pub(crate) fn tick_hover_tank(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::HoverTank && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut ht) = self.hover_tank {
            ht.catapult_shoot_ms = ht.catapult_shoot_ms.saturating_sub(dt_ms);
            let step = ((dt_ms as i32) * 160) / 1000;
            ht.catapult_x += ht.catapult_dir * step.max(1);
            if ht.catapult_x >= 456 {
                ht.catapult_x = 456;
                ht.catapult_dir = -1;
            } else if ht.catapult_x <= 32 {
                ht.catapult_x = 32;
                ht.catapult_dir = 1;
            }
            for slot in &mut ht.slots {
                slot.phase_ms = slot.phase_ms.saturating_add(dt_ms);
                match slot.phase {
                    TankPhase::Entering if slot.phase_ms >= 417 => {
                        slot.phase = TankPhase::Floating;
                        slot.phase_ms = 0;
                    }
                    TankPhase::Floating if slot.phase_ms >= 1_800 => {
                        slot.phase = TankPhase::Shooting;
                        slot.phase_ms = 0;
                    }
                    TankPhase::Shooting if slot.phase_ms >= 167 => {
                        slot.phase = TankPhase::Floating;
                        slot.phase_ms = 0;
                    }
                    TankPhase::Exploding if slot.phase_ms >= 450 => {
                        slot.phase = TankPhase::Entering;
                        slot.phase_ms = 0;
                    }
                    _ => {}
                }
            }
        }
    }

    /// Handles authentic `HoverTankScene` switches (`0x1d`, `0x1e`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_hover_tank_switch(&mut self, sw_id: u8) -> bool {
        let is_ht = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::HoverTank);
        if !is_ht {
            return false;
        }
        let lane_idx = match sw_id {
            sw::LEFT_RAMP_ENTER | sw::LEFT_SHIELD_TARGET => 0usize,
            sw::SHIELD_HIT => 1,
            sw::RIGHT_SHIELD_TARGET | sw::RIGHT_RAMP_ENTER => 2,
            _ => return false,
        };
        if self.hover_tank.is_none() {
            self.start_hover_tank_state();
        }
        let mut completed = false;
        if let Some(ref mut ht) = self.hover_tank {
            ht.catapult_shoot_ms = 433;
            ht.slots[lane_idx].phase = TankPhase::Exploding;
            ht.slots[lane_idx].phase_ms = 0;
            ht.tanks_remaining = ht.tanks_remaining.saturating_sub(1);
            completed = ht.tanks_remaining == 0;
        }
        self.add_score(scoring::HOVER_TANK_DESTROY);
        self.emit_ard("ard_hover_tank_scene_tank_boom");
        self.trigger_leff(LeffKind::SceneHit);
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = self
                .hover_tank
                .as_ref()
                .map_or(0, |h| h.tanks_remaining);
        }
        if completed {
            self.complete_active_scene(Swe1Scene::HoverTank);
            self.hover_tank = None;
        }
        true
    }

    pub(crate) fn render_hover_tank(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let ht = self.hover_tank.clone().unwrap_or_default();

        self.scene_sprite("anim_hover_tank_background", 0, 0, 0, 1);
        for slot in &ht.slots {
            let (x, y, z) = slot.lane.pos();
            match slot.phase {
                TankPhase::Entering => {
                    let fr = ((slot.phase_ms * 60) / 1000) as usize;
                    let ey = if slot.lane == TankLane::Center { 50 } else { y };
                    self.scene_sprite(slot.lane.anim_enter(), fr, x, ey, z);
                }
                TankPhase::Floating => {
                    self.scene_sprite(slot.lane.anim_float(), f60, x, y, z);
                }
                TankPhase::Shooting => {
                    let fr = ((slot.phase_ms * 60) / 1000) as usize;
                    let sx = match slot.lane {
                        TankLane::Left => 60,
                        TankLane::Center => 256,
                        TankLane::Right => 300,
                    };
                    self.scene_sprite(slot.lane.anim_shoot(), fr, sx, y, z);
                }
                TankPhase::Exploding => {
                    let fr = ((slot.phase_ms * 60) / 1000) as usize;
                    self.scene_sprite(slot.lane.anim_explode(), fr, x.saturating_sub(40), 40, 7);
                }
            }
        }

        if ht.catapult_shoot_ms > 0 {
            let fr = ((433 - ht.catapult_shoot_ms) * 60 / 1000) as usize;
            self.scene_sprite("anim_hover_tank_catapult_shoot", fr, ht.catapult_x, 112, 5);
            self.scene_sprite("anim_hover_tank_boomer_ball", f60, ht.catapult_x + 20, 85, 6);
        } else {
            self.scene_sprite("anim_hover_tank_catapult_idle", f60, ht.catapult_x, 120, 5);
            self.scene_sprite("anim_hover_tank_boomer_ball", f60, 360, 115, 6);
        }

        self.render_scene_footer(ht.tanks_remaining, secs, "TANKS");
    }
}
