//! Authentic `QueenFashionScene` + `DeffQueenFashionSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - Exact ROM coordinates: throne `(5, 44)`, wheel `(29, 88)`, queen `(178, 46)`,
//!   base `(167, 210)`, sparkles `(28, 177)` & `(382, 177)`.
//! - `SHIELD_HIT` (`0x27`): spins the fashion wheel (`ard_fashion_rotate`).
//! - `LEFT_RAMP_ENTER` / `LEFT_SHIELD_TARGET`: collects Left card.
//! - `RIGHT_RAMP_ENTER` / `RIGHT_SHIELD_TARGET`: collects Right card.

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::queen_fashion_state::{QueenFashionState, FASHION_ICONS};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_queen_fashion_state(&mut self) {
        self.queen_fashion = Some(QueenFashionState::new());
    }

    pub(crate) fn tick_queen_fashion(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::QueenFashion && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut qf) = self.queen_fashion {
            qf.flash_ms = qf.flash_ms.saturating_sub(dt_ms);
            qf.spin_ms = qf.spin_ms.saturating_sub(dt_ms);
        }
    }

    /// Handles authentic `QueenFashionScene` switches (`0x1d`, `0x1e`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_queen_fashion_switch(&mut self, sw_id: u8) -> bool {
        let is_qf = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::QueenFashion);
        if !is_qf {
            return false;
        }
        if self.queen_fashion.is_none() {
            self.start_queen_fashion_state();
        }
        match sw_id {
            sw::SHIELD_HIT => {
                if let Some(ref mut qf) = self.queen_fashion {
                    qf.wheel_pos = (qf.wheel_pos + 1) % FASHION_ICONS.len();
                    qf.spin_ms = 400;
                    qf.left_collected = false;
                    qf.right_collected = false;
                }
                self.add_score(50_000);
                self.emit_ard("ard_fashion_rotate");
                true
            }
            sw::LEFT_RAMP_ENTER
            | sw::LEFT_SHIELD_TARGET
            | sw::RIGHT_RAMP_ENTER
            | sw::RIGHT_SHIELD_TARGET => {
                let is_right = matches!(sw_id, sw::RIGHT_RAMP_ENTER | sw::RIGHT_SHIELD_TARGET);
                let mut completed = false;
                if let Some(ref mut qf) = self.queen_fashion {
                    if is_right {
                        qf.right_collected = true;
                    } else {
                        qf.left_collected = true;
                    }
                    qf.flash_ms = 450;
                    qf.wheel_pos = (qf.wheel_pos + 1) % FASHION_ICONS.len();
                    qf.outfits_remaining = qf.outfits_remaining.saturating_sub(1);
                    completed = qf.outfits_remaining == 0;
                }
                self.add_score(scoring::QUEEN_FASHION_MATCH);
                self.emit_ard("ard_fashion_expose");
                self.trigger_leff(LeffKind::SceneHit);
                if let Some(ref mut sc) = self.active_scene {
                    sc.hits_remaining = self
                        .queen_fashion
                        .as_ref()
                        .map_or(0, |q| q.outfits_remaining);
                }
                if completed {
                    self.complete_active_scene(Swe1Scene::QueenFashion);
                    self.queen_fashion = None;
                }
                true
            }
            _ => false,
        }
    }

    pub(crate) fn render_queen_fashion(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let qf = self.queen_fashion.clone().unwrap_or_default();

        self.scene_sprite("anim_fashion_throne", f60, 5, 44, 2);
        self.scene_sprite("anim_fashion_wheel", f60, 29, 88, 3);
        self.scene_sprite("anim_fashion_queen", f60, 178, 46, 4);
        self.scene_sprite("anim_fashion_base", 0, 167, 210, 5);

        if !qf.left_collected {
            self.scene_sprite("anim_fashion_sparkle_left", f60, 28, 177, 4);
        }
        if !qf.right_collected {
            self.scene_sprite("anim_fashion_sparkle_right", f60, 382, 177, 4);
        }

        let left_icon = FASHION_ICONS[qf.wheel_pos % FASHION_ICONS.len()];
        let right_icon = FASHION_ICONS[(qf.wheel_pos + 1) % FASHION_ICONS.len()];
        self.scene_sprite(left_icon, f60, 24, 108, 6);
        self.scene_sprite(right_icon, f60, 488, 108, 6);

        if qf.flash_ms > 0 {
            let fr = ((450 - qf.flash_ms) * 60 / 1000) as usize;
            self.scene_sprite("anim_fashion_flash", fr, 225, 50, 7);
        }

        self.render_scene_footer(qf.outfits_remaining, secs, "OUTFITS");
    }
}
