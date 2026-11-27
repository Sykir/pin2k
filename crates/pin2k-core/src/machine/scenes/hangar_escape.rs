//! Authentic `HangarEscapeScene` + `DeffHangarEscapeSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 8 droids on 1st attempt, 14 on subsequent attempts; first droid starts alone at Center.
//! - Left (`LEFT_RAMP_ENTER`, `LEFT_DROP_TARGET`, `LEFT_LASER`), Center (`SHIELD_HIT`,
//!   `LEFT/RIGHT_SHIELD_TARGET`), Right (`RIGHT_RAMP_ENTER`, `RIGHT_DROP_TARGET`, `RIGHT_LASER`).
//! - Hit = `200,000`, Empty-lane miss = `20,000` (+1 `droids_missed`).
//! - When `droids_remaining == 0`, shield raises and Center/Popper exits (`3M/6M/9M` perfect, `1M/2M/3M` normal).

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::hangar_escape_state::{HangarEscapeState, DROID_HIT_SCORE, TORPEDO_MISS_SCORE};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::Swe1Scene;
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_hangar_escape_state(&mut self) {
        let attempts = self.players[self.current_player].scenes_started.max(1);
        self.hangar_escape = Some(HangarEscapeState::new(attempts));
    }

    pub(crate) fn tick_hangar_escape(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::HangarEscape && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut he) = self.hangar_escape {
            he.explosion_ms = he.explosion_ms.saturating_sub(dt_ms);
            he.zap_ms = he.zap_ms.saturating_add(dt_ms) % 833;
        }
    }

    /// Handles authentic `HangarEscapeScene` switches.
    pub(crate) fn handle_hangar_escape_switch(&mut self, sw_id: u8) -> bool {
        let is_he = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::HangarEscape);
        if !is_he {
            return false;
        }
        if self.hangar_escape.is_none() {
            self.start_hangar_escape_state();
        }
        let door_open = self.hangar_escape.as_ref().is_some_and(|h| h.door_open());
        if door_open {
            if matches!(sw_id, sw::SHIELD_POPPER | sw::SHIELD_HIT) {
                self.finish_hangar_escape();
                return true;
            }
            return matches!(
                sw_id,
                sw::LEFT_RAMP_ENTER
                    | sw::RIGHT_RAMP_ENTER
                    | sw::LEFT_DROP_TARGET
                    | sw::RIGHT_DROP_TARGET
                    | sw::LEFT_SHIELD_TARGET
                    | sw::RIGHT_SHIELD_TARGET
            );
        }

        let lane = match sw_id {
            sw::LEFT_RAMP_ENTER | sw::LEFT_DROP_TARGET => 0usize,
            sw::RIGHT_RAMP_ENTER | sw::RIGHT_DROP_TARGET => 2,
            sw::SHIELD_HIT => 1,
            sw::LEFT_SHIELD_TARGET => {
                if self.hangar_escape.as_ref().is_some_and(|h| h.lanes[1] > 0) {
                    1
                } else {
                    0
                }
            }
            sw::RIGHT_SHIELD_TARGET => {
                if self.hangar_escape.as_ref().is_some_and(|h| h.lanes[1] > 0) {
                    1
                } else {
                    2
                }
            }
            sw::SHIELD_POPPER => return false,
            _ => return false,
        };
        self.fire_hangar_torpedo(lane);
        true
    }

    pub(crate) fn handle_hangar_escape_laser(&mut self, right: bool) -> bool {
        let is_he = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::HangarEscape);
        if !is_he {
            return false;
        }
        if self.hangar_escape.is_none() {
            self.start_hangar_escape_state();
        }
        if self.hangar_escape.as_ref().is_some_and(|h| h.door_open()) {
            return true;
        }
        self.fire_hangar_torpedo(if right { 2 } else { 0 });
        true
    }

    fn fire_hangar_torpedo(&mut self, lane: usize) {
        let (hit, door_now_open, remaining) = if let Some(ref mut he) = self.hangar_escape {
            if he.lanes[lane] > 0 {
                he.lanes[lane] -= 1;
                he.droids_remaining = he.droids_remaining.saturating_sub(1);
                he.explosion_ms = 500;
                he.last_hit_lane = lane;
                he.hit_variant = he.hit_variant.wrapping_add(1);
                he.respawn_droids();
                (true, he.droids_remaining == 0, he.droids_remaining)
            } else {
                he.droids_missed = he.droids_missed.saturating_add(1);
                (false, false, he.droids_remaining)
            }
        } else {
            return;
        };

        if hit {
            self.add_score(DROID_HIT_SCORE);
            self.emit_ard("ard_hangar_escape_scene_droid_boom");
            self.trigger_leff(LeffKind::SceneHit);
            if let Some(ref mut sc) = self.active_scene {
                sc.hits_remaining = remaining;
            }
            if door_now_open {
                self.set_shield_raised(true);
                self.emit_ard("ard_hangar_escape_scene_warning_message");
                self.set_banner("DOOR OPEN! SHOOT CENTER TO ESCAPE!", 3_000);
            } else {
                self.set_banner("DROID DESTROYED! +200,000", 1_400);
            }
        } else {
            self.add_score(TORPEDO_MISS_SCORE);
            let ard = match lane {
                0 => "ard_hangar_escape_scene_droid_miss_l",
                2 => "ard_hangar_escape_scene_droid_miss_r",
                _ => "ard_hangar_escape_scene_laser_c",
            };
            self.emit_ard(ard);
            self.set_banner("MISSED TORPEDO! +20,000", 1_200);
        }
    }

    fn finish_hangar_escape(&mut self) {
        let (attempts, perfect) = self
            .hangar_escape
            .as_ref()
            .map_or((1, false), |h| (h.attempts, h.droids_missed == 0));
        let mult = (attempts.min(3)) as u64;
        let total_award = if perfect { 3_000_000 * mult } else { 1_000_000 * mult };
        let extra = total_award.saturating_sub(Swe1Scene::HangarEscape.completion_score());
        if extra > 0 {
            self.add_score(extra);
        }
        if perfect {
            self.emit_ard("ard_hangar_escape_scene_perfect");
            self.set_banner(format!("PERFECT SHOOTING! +{total_award}"), 3_000);
        } else {
            self.emit_ard("ard_hangar_escape_scene_naboo_exit");
        }
        self.set_shield_raised(false);
        self.complete_active_scene(Swe1Scene::HangarEscape);
        self.hangar_escape = None;
    }

    pub(crate) fn render_hangar_escape(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let he = self.hangar_escape.clone().unwrap_or_default();

        self.scene_sprite("anim_federation_fighter_cockpit", 0, 0, 0, 1);
        const LANE_X: [i32; 3] = [120, 275, 430];
        for (lane, &count) in he.lanes.iter().enumerate() {
            if count > 0 {
                self.scene_sprite("anim_hangar_droid_run", f60, LANE_X[lane], 48, 3);
            }
        }
        self.scene_sprite("anim_hangar_escape_torpedo", f60, 220, 80, 4);
        if he.zap_ms < 233 {
            self.scene_sprite("anim_federation_fighter_zap", f60, 200, 120, 5);
        }
        self.scene_sprite("anim_naboo_xhair_blue", f60, 252, 84, 6);
        if he.explosion_ms > 0 {
            let fr = ((500 - he.explosion_ms) * 60 / 1000) as usize;
            let anim = match he.hit_variant % 4 {
                0 => "anim_hangar_droid_explosion",
                1 => "anim_hangar_droid_explosion_2",
                2 => "anim_hangar_droid_explosion_3",
                _ => "anim_hangar_droid_explosion_4",
            };
            let ex = match he.last_hit_lane {
                0 => 40,
                2 => 330,
                _ => 185,
            };
            self.scene_sprite(anim, fr, ex, 42, 7);
        }

        let label = if he.door_open() { "EXIT" } else { "DROIDS" };
        self.render_scene_footer(he.droids_remaining, secs, label);
    }
}
