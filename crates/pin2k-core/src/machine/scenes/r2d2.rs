//! Authentic `R2D2Scene` + `DeffR2D2SceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 6 Astromech units embarking `5 -> 0` (R2-D2 last).
//! - Hurry-up starts at `500,000` (frozen `3s`), decays `5,470 / 60ms` down to `50,000`.
//! - `SHIELD_HIT` / `LEFT_SHIELD_TARGET` / `RIGHT_SHIELD_TARGET` embark the center unit.
//! - `RAMP_MADE_LEFT` / `RAMP_MADE_RIGHT` boost hurry-up (`500,000 + boosts * 250,000`, max `2M`) and freeze `3s`.

use pin2k_hal::{AudioBackend, PinballHardware, TextAlign, VideoBackend};

use super::r2d2_state::{
    R2D2State, HURRY_UP_DECAY_STEP, HURRY_UP_FLOOR, HURRY_UP_FREEZE_MS, HURRY_UP_MAX,
    HURRY_UP_RAMP_STEP, HURRY_UP_START, R2_UNITS,
};
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
        let Some(ref mut r2) = self.r2d2 else {
            return;
        };
        r2.suck_ms = r2.suck_ms.saturating_sub(dt_ms);
        if r2.suck_ms == 0 {
            r2.sucking_unit = None;
        }
        r2.move_ms = r2.move_ms.saturating_sub(dt_ms);
        if r2.freeze_ms > 0 {
            r2.freeze_ms = r2.freeze_ms.saturating_sub(dt_ms);
        } else {
            r2.decay_accum_ms = r2.decay_accum_ms.saturating_add(dt_ms);
            while r2.decay_accum_ms >= 60 {
                r2.decay_accum_ms -= 60;
                r2.hurry_up_value = r2
                    .hurry_up_value
                    .saturating_sub(HURRY_UP_DECAY_STEP)
                    .max(HURRY_UP_FLOOR);
            }
        }
    }

    /// Handles authentic `R2D2Scene` switches (`0x1b`, `0x1c`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_r2d2_switch(&mut self, sw_id: u8) -> bool {
        let is_r2 = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::R2D2);
        if !is_r2 {
            return false;
        }
        if self.r2d2.is_none() {
            self.start_r2d2_state();
        }
        match sw_id {
            sw::RAMP_MADE_LEFT | sw::RAMP_MADE_RIGHT => {
                let boosted = if let Some(ref mut r2) = self.r2d2 {
                    r2.ramp_boosts = r2.ramp_boosts.saturating_add(1);
                    r2.hurry_up_value = HURRY_UP_START
                        .saturating_add((r2.ramp_boosts as u64) * HURRY_UP_RAMP_STEP)
                        .min(HURRY_UP_MAX);
                    r2.freeze_ms = HURRY_UP_FREEZE_MS;
                    r2.decay_accum_ms = 0;
                    r2.hurry_up_value
                } else {
                    return true;
                };
                self.emit_ard("ard_r2d2_scene_move");
                self.set_banner(format!("DROID VALUE BOOSTED! {boosted}"), 1_600);
                true
            }
            sw::SHIELD_HIT | sw::LEFT_SHIELD_TARGET | sw::RIGHT_SHIELD_TARGET => {
                let (pts, completed, speech) = if let Some(ref mut r2) = self.r2d2 {
                    let aw = r2.hurry_up_value;
                    let idx = (r2.units_remaining as usize).saturating_sub(1);
                    r2.sucking_unit = Some(idx);
                    r2.suck_ms = 500;
                    r2.move_ms = 450;
                    r2.units_remaining = r2.units_remaining.saturating_sub(1);
                    r2.hurry_up_value = HURRY_UP_START;
                    r2.freeze_ms = HURRY_UP_FREEZE_MS;
                    r2.decay_accum_ms = 0;
                    r2.ramp_boosts = 0;
                    r2.speech_counter = r2.speech_counter.wrapping_add(1);
                    (aw, r2.units_remaining == 0, (r2.speech_counter & 1) == 1)
                } else {
                    return true;
                };
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
                } else {
                    self.set_banner(format!("ASTROMECH BOARDED! +{pts}"), 1_600);
                }
                true
            }
            _ => false,
        }
    }

    pub(crate) fn render_r2d2(&mut self, secs: u32) {
        let f10 = ((self.elapsed_ms * 10) / 1000) as usize;
        let r2 = self.r2d2.clone().unwrap_or_default();

        self.scene_sprite("anim_r2d2_scene_background_lower", 0, 0, 108, 3);
        self.scene_sprite("anim_r2d2_scene_background_upper", 0, 0, 0, 7);

        let rem = (r2.units_remaining as usize).min(R2_UNITS.len());
        for idx in 0..rem {
            let spec = R2_UNITS[idx];
            let frame = if idx + 1 == rem {
                if r2.move_ms > 0 {
                    let span = (spec.move_end - spec.move_start + 1).max(1);
                    spec.move_start + (((450 - r2.move_ms) * 20 / 1000) as usize) % span
                } else {
                    let span = (spec.center_end - spec.center_start + 1).max(1);
                    spec.center_start + (f10 % span)
                }
            } else {
                f10 % 10
            };
            self.scene_sprite(spec.anim_name, frame, spec.x, spec.y, 10);
        }

        if let Some(suck_idx) = r2.sucking_unit {
            if r2.suck_ms > 0 && suck_idx < R2_UNITS.len() {
                let spec = R2_UNITS[suck_idx];
                let span = (spec.suck_end - spec.suck_start + 1).max(1);
                let fr = spec.suck_start + (((500 - r2.suck_ms) * 30 / 1000) as usize).min(span - 1);
                self.scene_sprite(spec.anim_name, fr, spec.x, spec.y, 10);
            }
        }

        self.scene_text(
            "font_shadow_11_narrow_data",
            format!("{}", r2.hurry_up_value),
            280,
            210,
            12,
            0x07FF,
            Some(0x0000),
            TextAlign::Center,
        );
        self.render_scene_footer(r2.units_remaining, secs, "DROIDS");
    }
}
