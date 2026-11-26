//! Authentic `MusicalChairsScene` (`MosEspaCantina`) + `DeffMusicalChairsBackground`
//! runtime & renderer (`swe1_0200.elf`).

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::mos_espa_cantina_state::{MosEspaCantinaState, CHAIR_X, CHAIR_Y, COUNCIL_ANIMS};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_mos_espa_cantina_state(&mut self) {
        self.mos_espa_cantina = Some(MosEspaCantinaState::new());
    }

    pub(crate) fn tick_mos_espa_cantina(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::MosEspaCantina && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        let mut stop_ard = false;
        if let Some(ref mut mc) = self.mos_espa_cantina {
            mc.jump_ms = mc.jump_ms.saturating_sub(dt_ms);
            mc.phase_ms = mc.phase_ms.saturating_add(dt_ms);
            if !mc.music_stopped && mc.phase_ms >= 2_500 {
                mc.music_stopped = true;
                mc.phase_ms = 0;
                stop_ard = true;
            } else if mc.music_stopped && mc.phase_ms >= 2_000 {
                mc.music_stopped = false;
                mc.phase_ms = 0;
                mc.shuffle();
            }
        }
        if stop_ard {
            self.emit_ard("ard_musical_chairs_music_stop");
        }
    }

    /// Handles authentic `MusicalChairsScene` switches (`0x1b`, `0x1c`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_mos_espa_cantina_switch(&mut self, sw_id: u8) -> bool {
        let is_mc = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::MosEspaCantina);
        if !is_mc {
            return false;
        }
        let valid = matches!(
            sw_id,
            sw::SHIELD_HIT
                | sw::LEFT_SHIELD_TARGET
                | sw::RIGHT_SHIELD_TARGET
                | sw::RAMP_MADE_LEFT
                | sw::RAMP_MADE_RIGHT
        );
        if !valid {
            return false;
        }
        if self.mos_espa_cantina.is_none() {
            self.start_mos_espa_cantina_state();
        }
        let mut completed = false;
        if let Some(ref mut mc) = self.mos_espa_cantina {
            mc.jump_ms = 400;
            mc.shuffle();
            mc.seats_remaining = mc.seats_remaining.saturating_sub(1);
            completed = mc.seats_remaining == 0;
        }
        self.add_score(scoring::MUSICAL_CHAIRS_CENTER);
        self.emit_ard("ard_musical_chairs_land");
        self.trigger_leff(LeffKind::SceneHit);
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = self
                .mos_espa_cantina
                .as_ref()
                .map_or(0, |m| m.seats_remaining);
        }
        if completed {
            self.complete_active_scene(Swe1Scene::MosEspaCantina);
            self.mos_espa_cantina = None;
        }
        true
    }

    pub(crate) fn render_mos_espa_cantina(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let mc = self.mos_espa_cantina.clone().unwrap_or_default();

        self.scene_sprite("anim_jedi_council_background", 0, 24, 3, 1);
        let bounce_y = if mc.jump_ms > 0 || !mc.music_stopped {
            ((f60 % 6) as i32) * 2
        } else {
            0
        };
        for (chair_idx, &member_idx) in mc.order.iter().enumerate() {
            let anim = COUNCIL_ANIMS[member_idx % COUNCIL_ANIMS.len()];
            let y = CHAIR_Y - if chair_idx % 2 == 0 { bounce_y } else { 0 };
            self.scene_sprite(anim, f60, CHAIR_X[chair_idx], y, 4);
        }

        self.render_scene_footer(mc.seats_remaining, secs, "SEATS");
    }
}
