//! Authentic `WattosChanceScene` + `DeffWattosChanceSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 3 Chance Cubes at exact ROM `(45, 171)`, `(222, 175)`, `(399, 171)`.
//! - Watto 7-segment timeline (`Left -> Middle -> Right`) highlights the active cube.
//! - `LEFT_RAMP_ENTER`/`LEFT_SHIELD_TARGET` -> lane 0, `SHIELD_HIT` -> lane 1,
//!   `RIGHT_RAMP_ENTER`/`RIGHT_SHIELD_TARGET` -> lane 2.

use pin2k_hal::{AudioBackend, PinballHardware, TextAlign, VideoBackend};

use super::wattos_chance_state::{WattosChanceState, CUBE_X, CUBE_Y, WATTO_TIMELINE};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_wattos_chance_state(&mut self) {
        self.wattos_chance = Some(WattosChanceState::new());
    }

    pub(crate) fn tick_wattos_chance(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::WattosChance && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut wc) = self.wattos_chance {
            let total: u32 = WATTO_TIMELINE.iter().map(|s| s.1).sum();
            wc.timeline_ms = wc.timeline_ms.saturating_add(dt_ms) % total;
            wc.active_lane = wc.current_segment().5;
            wc.explode_ms = wc.explode_ms.saturating_sub(dt_ms);
            wc.miss_ms = wc.miss_ms.saturating_sub(dt_ms);
        }
    }

    /// Handles authentic `WattosChanceScene` switches (`0x1d`, `0x1e`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_wattos_chance_switch(&mut self, sw_id: u8) -> bool {
        let is_wc = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::WattosChance);
        if !is_wc {
            return false;
        }
        let lane = match sw_id {
            sw::LEFT_RAMP_ENTER | sw::LEFT_SHIELD_TARGET => 0usize,
            sw::SHIELD_HIT => 1,
            sw::RIGHT_SHIELD_TARGET | sw::RIGHT_RAMP_ENTER => 2,
            _ => return false,
        };
        if self.wattos_chance.is_none() {
            self.start_wattos_chance_state();
        }
        let mut hit = false;
        let mut completed = false;
        if let Some(ref mut wc) = self.wattos_chance {
            if lane == wc.active_lane {
                hit = true;
                wc.explode_ms = 500;
                wc.explode_lane = lane;
                wc.hits_remaining = wc.hits_remaining.saturating_sub(1);
                completed = wc.hits_remaining == 0;
            } else {
                wc.miss_ms = 350;
                wc.miss_lane = lane;
            }
        }
        if hit {
            self.add_score(scoring::WATTOS_CHANCE_WIN);
            self.emit_ard("ard_wattos_chance_scene_cube_hit_c");
            self.trigger_leff(LeffKind::SceneHit);
            if let Some(ref mut sc) = self.active_scene {
                sc.hits_remaining = self
                    .wattos_chance
                    .as_ref()
                    .map_or(0, |w| w.hits_remaining);
            }
            if completed {
                self.complete_active_scene(Swe1Scene::WattosChance);
                self.wattos_chance = None;
            }
        } else {
            self.add_score(50_000);
            self.emit_ard("ard_wattos_chance_scene_cube_miss");
        }
        true
    }

    pub(crate) fn render_wattos_chance(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let wc = self.wattos_chance.clone().unwrap_or_default();

        self.scene_sprite("anim_watto_scene_background", 0, 0, 0, 1);

        let (watto_anim, _, wx, wy, loops, active_lane) = wc.current_segment();
        let w_fr = if loops { f60 } else { f60 % 14 };
        self.scene_sprite(watto_anim, w_fr, wx, wy, 3);

        for lane in 0..3 {
            let cx = CUBE_X[lane];
            let cy = CUBE_Y[lane];
            self.scene_sprite("anim_watto_scene_cube", f60 + lane * 7, cx, cy, 5);
            if lane == active_lane {
                self.scene_sprite("anim_watto_scene_cube_flash", f60, cx - 31, cy - 12, 6);
            }
        }

        if wc.explode_ms > 0 {
            let fr = ((500 - wc.explode_ms) * 60 / 1000) as usize;
            self.scene_sprite(
                "anim_watto_scene_cube_explode",
                fr,
                CUBE_X[wc.explode_lane] - 65,
                CUBE_Y[wc.explode_lane] - 27,
                7,
            );
        } else if wc.miss_ms > 0 {
            let fr = ((350 - wc.miss_ms) * 60 / 1000) as usize;
            self.scene_sprite(
                "anim_sith_probe_miss_explosion",
                fr,
                CUBE_X[wc.miss_lane],
                CUBE_Y[wc.miss_lane],
                7,
            );
        }

        self.scene_text(
            "font_shadow_7_data",
            format!("SHOOT CUBE!  HITS LEFT: {}   {secs}S", wc.hits_remaining),
            320,
            208,
            7,
            0x7FE0,
            Some(0x0000),
            TextAlign::Center,
        );
    }
}
