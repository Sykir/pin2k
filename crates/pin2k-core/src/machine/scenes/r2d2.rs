//! Authentic `R2D2Scene` + `DeffR2D2SceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 6 Astromech units (`units_remaining = 6`) with 500,000 -> 100,000 hurry-up decay.
//! - Handles `SHIELD_HIT` (`0x27`), `LEFT/RIGHT_SHIELD_TARGET` (`0x22`/`0x23`),
//!   `RAMP_MADE_LEFT/RIGHT` (`0x1b`/`0x1c`), `LEFT_LOOP_UPPER/ROLLOVER` (`0x1f`/`0x20`).

use pin2k_hal::{AudioBackend, PinballHardware, TextAlign, VideoBackend};

use super::r2d2_state::{R2D2State, R2_UNITS};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::Swe1Scene;
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_r2d2_state(&mut self) {
        self.r2d2 = Some(R2D2State::new());
    }

    pub(crate) fn tick_r2d2(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::R2D2 && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut r2) = self.r2d2 {
            r2.suck_ms = r2.suck_ms.saturating_sub(dt_ms);
            let decay = ((dt_ms as u64) * 20_000) / 1000;
            r2.hurry_up_value = r2.hurry_up_value.saturating_sub(decay).max(100_000);
        }
    }

    /// Handles authentic `R2D2Scene` switches (`0x1b`, `0x1c`, `0x1f`, `0x20`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_r2d2_switch(&mut self, sw_id: u8) -> bool {
        let is_r2 = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::R2D2);
        if !is_r2 {
            return false;
        }
        let valid = matches!(
            sw_id,
            sw::SHIELD_HIT
                | sw::LEFT_SHIELD_TARGET
                | sw::RIGHT_SHIELD_TARGET
                | sw::RAMP_MADE_LEFT
                | sw::RAMP_MADE_RIGHT
                | sw::LEFT_LOOP_UPPER
                | sw::LEFT_LOOP_ROLLOVER
        );
        if !valid {
            return false;
        }
        if self.r2d2.is_none() {
            self.start_r2d2_state();
        }
        let mut pts = 250_000u64;
        let mut completed = false;
        let mut speech = false;
        if let Some(ref mut r2) = self.r2d2 {
            pts = r2.hurry_up_value;
            r2.suck_ms = 400;
            r2.units_remaining = r2.units_remaining.saturating_sub(1);
            r2.center_unit = (r2.center_unit + 1) % R2_UNITS.len();
            r2.hurry_up_value = 500_000;
            r2.speech_counter = r2.speech_counter.wrapping_add(1);
            speech = (r2.speech_counter & 1) == 1;
            completed = r2.units_remaining == 0;
        }
        self.add_score(pts);
        self.emit_ard("ard_r2d2_scene_hit_1");
        if speech {
            self.emit_ard("ard_hit_r2d2_speech");
        }
        self.trigger_leff(LeffKind::SceneHit);
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = self.r2d2.as_ref().map_or(0, |r| r.units_remaining);
        }
        if completed {
            self.complete_active_scene(Swe1Scene::R2D2);
            self.r2d2 = None;
        }
        true
    }

    pub(crate) fn render_r2d2(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let r2 = self.r2d2.clone().unwrap_or_default();

        self.scene_sprite("anim_r2d2_scene_background_upper", 0, 0, 0, 1);
        self.scene_sprite("anim_r2d2_scene_background_lower", 0, 0, 108, 2);

        for idx in 0..r2.units_remaining as usize {
            let unit_idx = (r2.center_unit + idx) % R2_UNITS.len();
            let (name, ux, uy) = R2_UNITS[unit_idx];
            if idx == 0 {
                // Center unit
                self.scene_sprite(name, f60, 225, 105, 4);
            } else {
                self.scene_sprite(name, f60, ux, uy, 5);
            }
        }

        self.scene_text(
            "font_shadow_11_narrow_data",
            format!("REPAIR SHIELD!  {} LEFT  ({secs}S)", r2.units_remaining),
            320,
            214,
            7,
            0x7E00,
            Some(0x0000),
            TextAlign::Center,
        );
    }
}
