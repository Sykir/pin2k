//! Authentic `QueenFashionScene` + `DeffQueenFashionSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 6 outfits (`cards_mask == 0x3F`), Left/Right card reveal (`500,000` new, `15,520` duplicate).
//! - Center `SHIELD_HIT` or Lasers spin the wheel (`20,000`) to a valid position showing >= 1 unexposed card.
//! - All shots ignored while spinning (`sleep100(3)` = `30ms` per +2 step, <= `0.9s`).

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::queen_fashion_state::{
    QueenFashionState, BASE_POS, CARD_POSITIONS, QUEEN_POS, SPARKLE_LEFT_POS, SPARKLE_RIGHT_POS,
    THRONE_POS, WHEEL_POS, WHEEL_POSITIONS,
};
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
        let Some(ref mut qf) = self.queen_fashion else {
            return;
        };
        if !qf.spinning {
            return;
        }
        qf.spin_step_ms = qf.spin_step_ms.saturating_add(dt_ms);
        while qf.spinning && qf.spin_step_ms >= 30 {
            qf.spin_step_ms -= 30;
            qf.wheel_step = (qf.wheel_step + 2) % 60;
            let target = (qf.wheel_pos % 6) * 10;
            if qf.wheel_step == target.wrapping_sub(1) {
                qf.wheel_step = target;
            }
            if qf.base_flash_steps > 0 {
                qf.base_flash_steps -= 1;
            }
            if qf.wheel_step == target {
                qf.spinning = false;
            }
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
                self.spin_queen_fashion_wheel();
                true
            }
            sw::LEFT_RAMP_ENTER
            | sw::LEFT_SHIELD_TARGET
            | sw::RIGHT_RAMP_ENTER
            | sw::RIGHT_SHIELD_TARGET => {
                if self.queen_fashion.as_ref().is_some_and(|q| q.spinning) {
                    return true;
                }
                let is_right = matches!(sw_id, sw::RIGHT_RAMP_ENTER | sw::RIGHT_SHIELD_TARGET);
                let (newly_exposed, completed) = if let Some(ref mut qf) = self.queen_fashion {
                    let wp = WHEEL_POSITIONS[qf.wheel_pos % 6];
                    let card_bit = if is_right { wp.right_bit } else { wp.left_bit };
                    let is_new = (qf.cards_mask & card_bit) == 0;
                    if is_new {
                        qf.cards_mask |= card_bit;
                        qf.outfits_remaining = 6u8.saturating_sub((qf.cards_mask & 0x3F).count_ones() as u8);
                    }
                    (is_new, (qf.cards_mask & 0x3F) == 0x3F)
                } else {
                    return true;
                };
                if newly_exposed {
                    self.add_score(scoring::QUEEN_FASHION_MATCH);
                    self.emit_ard("ard_fashion_expose");
                    self.trigger_leff(LeffKind::SceneHit);
                    self.set_banner("OUTFIT REVEALED! +500,000", 1_800);
                } else {
                    self.add_score(15_520);
                    self.set_banner("ALREADY REVEALED! +15,520", 1_200);
                }
                if let Some(ref mut sc) = self.active_scene {
                    sc.hits_remaining = self.queen_fashion.as_ref().map_or(0, |q| q.outfits_remaining);
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

    pub(crate) fn handle_queen_fashion_laser(&mut self) -> bool {
        let is_qf = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::QueenFashion);
        if !is_qf {
            return false;
        }
        self.spin_queen_fashion_wheel();
        true
    }

    fn spin_queen_fashion_wheel(&mut self) {
        if self.queen_fashion.is_none() {
            self.start_queen_fashion_state();
        }
        let seed = self.elapsed_ms as u32;
        if self.queen_fashion.as_ref().is_some_and(|q| q.spinning) {
            return;
        }
        self.add_score(20_000);
        let started = self
            .queen_fashion
            .as_mut()
            .is_some_and(|qf| qf.start_spin(seed));
        if started {
            self.emit_ard("ard_fashion_rotate_start");
            self.set_banner("SPINNING WARDROBE! +20,000", 1_200);
        } else {
            self.complete_active_scene(Swe1Scene::QueenFashion);
            self.queen_fashion = None;
        }
    }

    pub(crate) fn render_queen_fashion(&mut self, secs: u32) {
        let f20 = ((self.elapsed_ms * 20) / 1000) as usize;
        let qf = self.queen_fashion.clone().unwrap_or_default();

        self.scene_sprite("anim_fashion_throne", 0, THRONE_POS.0, THRONE_POS.1, 2);
        let wheel_frame = (qf.wheel_step + 9) % 10;
        self.scene_sprite("anim_fashion_wheel", wheel_frame, WHEEL_POS.0, WHEEL_POS.1, 3);

        for wp in &WHEEL_POSITIONS {
            if (qf.cards_mask & wp.card_bit) != 0 {
                let step = (wp.start_step + qf.wheel_step) % 60;
                let (cx, cy) = CARD_POSITIONS[step];
                self.scene_sprite(wp.anim_name, step, WHEEL_POS.0 + cx - 1, WHEEL_POS.1 + cy - 1, 4);
            }
        }

        self.scene_sprite("anim_fashion_queen", 0, QUEEN_POS.0, QUEEN_POS.1, 5);
        let base_frame = usize::from(qf.spinning && qf.base_flash_steps > 0);
        self.scene_sprite("anim_fashion_base", base_frame, BASE_POS.0, BASE_POS.1, 6);

        if !qf.spinning && qf.is_left_card_clear() {
            self.scene_sprite("anim_fashion_sparkle_left", f20, SPARKLE_LEFT_POS.0, SPARKLE_LEFT_POS.1, 5);
        }
        if !qf.spinning && qf.is_right_card_clear() {
            self.scene_sprite("anim_fashion_sparkle_right", f20, SPARKLE_RIGHT_POS.0, SPARKLE_RIGHT_POS.1, 5);
        }

        self.render_scene_footer(qf.outfits_remaining, secs, "OUTFITS");
    }
}
