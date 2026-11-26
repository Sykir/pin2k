//! Authentic `GunganScene` + `DeffSubEscapeSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - Sub escape distance (`0..=10`, starts at `5`).
//! - `RAMP_MADE_LEFT` / `RAMP_MADE_RIGHT`: `+2` distance (`ard_sub_swoosh_1`).
//! - `SHIELD_HIT` / `LEFT/RIGHT_SHIELD_TARGET` / `LEFT_LOOP_UPPER/ROLLOVER`: `+1` distance (`ard_sub_swoosh_2`).
//! - Reaching `distance == 10` escapes the Opee Sea Killer and completes the scene!

use pin2k_hal::{AudioBackend, PinballHardware, TextAlign, VideoBackend};

use super::gungan_battle_state::GunganBattleState;
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_gungan_battle_state(&mut self) {
        self.gungan_battle = Some(GunganBattleState::new());
    }

    pub(crate) fn tick_gungan_battle(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::GunganBattle && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut gb) = self.gungan_battle {
            gb.boost_flash_ms = gb.boost_flash_ms.saturating_sub(dt_ms);
            gb.sando_ms = gb.sando_ms.saturating_add(dt_ms) % 9_000;
        }
    }

    /// Handles authentic `GunganScene` switches (`0x1b`, `0x1c`, `0x1f`, `0x20`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_gungan_battle_switch(&mut self, sw_id: u8) -> bool {
        let is_gb = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::GunganBattle);
        if !is_gb {
            return false;
        }
        let delta = match sw_id {
            sw::RAMP_MADE_LEFT | sw::RAMP_MADE_RIGHT => 2u8,
            sw::SHIELD_HIT
            | sw::LEFT_SHIELD_TARGET
            | sw::RIGHT_SHIELD_TARGET
            | sw::LEFT_LOOP_UPPER
            | sw::LEFT_LOOP_ROLLOVER => 1u8,
            _ => return false,
        };
        if self.gungan_battle.is_none() {
            self.start_gungan_battle_state();
        }
        let mut completed = false;
        let mut speech = false;
        if let Some(ref mut gb) = self.gungan_battle {
            gb.distance = gb.distance.saturating_add(delta).min(10);
            gb.boost_flash_ms = 400;
            gb.speech_counter = gb.speech_counter.wrapping_add(1);
            speech = (gb.speech_counter & 1) == 1;
            completed = gb.distance >= 10;
        }
        self.add_score(scoring::GUNGAN_SHOT);
        self.emit_ard(if delta >= 2 { "ard_sub_swoosh_1" } else { "ard_sub_swoosh_2" });
        if speech {
            self.emit_ard("ard_hit_gungan_sub_speech");
        }
        self.trigger_leff(LeffKind::SceneHit);
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = self
                .gungan_battle
                .as_ref()
                .map_or(0, |g| 10u8.saturating_sub(g.distance));
        }
        if completed {
            self.complete_active_scene(Swe1Scene::GunganBattle);
            self.gungan_battle = None;
        }
        true
    }

    pub(crate) fn render_gungan_battle(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let gb = self.gungan_battle.clone().unwrap_or_default();

        self.scene_sprite("anim_gungan_seaweed", f60, 0, 40, 1);
        self.scene_sprite("anim_gungan_sub_bubbles_2", f60, 35, 0, 2);
        self.scene_sprite("anim_gungan_sub_bubbles", f60, 470, 10, 2);
        if gb.sando_ms < 1_417 {
            let fr = ((gb.sando_ms * 60) / 1000) as usize;
            self.scene_sprite("anim_gungan_sub_sando", fr, 240, 60, 3);
        }

        let sub_x = 20 + (gb.distance as i32) * 14;
        let opee_x = -40 + (gb.distance as i32) * 8;
        self.scene_sprite("anim_gungan_sub_sub", f60, sub_x, 84, 4);
        self.scene_sprite("anim_gungan_sub_opee", f60, opee_x, 44, 5);

        self.scene_text(
            "font_shadow_9_narrow_data",
            "G U N G A N",
            540,
            80,
            7,
            0x7FE0,
            Some(0x0000),
            TextAlign::Center,
        );
        self.render_scene_footer(10u8.saturating_sub(gb.distance), secs, "BATTLE HITS");
    }
}
