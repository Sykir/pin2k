//! Authentic `MusicalChairsScene` (`MosEspaCantina`) + `DeffMusicalChairsBackground`
//! runtime & renderer (`swe1_0200.elf`).

use pin2k_hal::{AudioBackend, PinballHardware, TextAlign, VideoBackend};

use super::mos_espa_cantina_state::{
    CouncilAward, MosEspaCantinaState, BG_X, BG_Y, CHAIR_X, CHAIR_Y, COUNCIL_ANIMS,
};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_mos_espa_cantina_state(&mut self) {
        let mut mc = MosEspaCantinaState::new();
        let p = &self.players[self.current_player];
        let seed = (self.elapsed_ms as u32) ^ (p.score as u32).wrapping_add(1);
        mc.select_awards(seed, p.jedi_letters != 0x0F, p.bonus_multiplier < 5);
        self.mos_espa_cantina = Some(mc);
    }

    pub(crate) fn tick_mos_espa_cantina(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::MosEspaCantina && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        let mut stopped_now = false;
        let mut timeout_departed = false;
        let mut completed = false;
        let next_dur = 3_000 + ((self.elapsed_ms % 6_000) as u32);
        if let Some(ref mut mc) = self.mos_espa_cantina {
            mc.jump_ms = mc.jump_ms.saturating_sub(dt_ms);
            mc.tick_member_anims(dt_ms);
            mc.phase_ms = mc.phase_ms.saturating_add(dt_ms);
            if !mc.music_stopped && mc.phase_ms >= mc.music_duration_ms {
                mc.music_stopped = true;
                mc.phase_ms = 0;
                stopped_now = true;
            } else if mc.music_stopped && mc.phase_ms >= 20_000 {
                mc.music_stopped = false;
                mc.phase_ms = 0;
                mc.music_duration_ms = next_dur;
                mc.mark_center_departed();
                mc.shuffle();
                timeout_departed = true;
                completed = mc.seats_remaining == 0;
            }
        }
        if stopped_now {
            self.set_shield_raised(true);
            self.emit_ard("ard_musical_chairs_music_stop");
            self.set_banner("MUSIC STOPPED! COLLECT CENTER!", 2_500);
        }
        if timeout_departed {
            self.set_shield_raised(false);
            if let Some(ref mut sc) = self.active_scene {
                sc.hits_remaining = self.mos_espa_cantina.as_ref().map_or(0, |m| m.seats_remaining);
            }
            if completed {
                self.complete_active_scene(Swe1Scene::MosEspaCantina);
                self.mos_espa_cantina = None;
            }
        }
    }

    /// Handles `MusicalChairsScene` switches (`0x1b`, `0x1c`, `0x20`, `0x22`, `0x23`, `0x27`).
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
            sw::SHIELD_POPPER
                | sw::SHIELD_HIT
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
        let music_stopped = self.mos_espa_cantina.as_ref().is_some_and(|m| m.music_stopped);
        if !music_stopped {
            if sw_id == sw::SHIELD_POPPER {
                return false;
            }
            self.trigger_musical_chairs_jump();
            return true;
        }
        // During silence (20s), shield is UP: trapping in SHIELD_POPPER (or SHIELD_HIT opto) collects center member
        if !matches!(sw_id, sw::SHIELD_POPPER | sw::SHIELD_HIT) {
            return true;
        }
        let next_dur = 3_000 + ((self.elapsed_ms % 6_000) as u32);
        let (award, completed) = if let Some(ref mut mc) = self.mos_espa_cantina {
            let aw = mc.awards[2];
            mc.mark_center_departed();
            mc.music_stopped = false;
            mc.phase_ms = 0;
            mc.music_duration_ms = next_dur;
            mc.shuffle();
            (aw, mc.seats_remaining == 0)
        } else {
            return true;
        };
        self.set_shield_raised(false);
        self.add_score(scoring::MUSICAL_CHAIRS_CENTER);
        let mut jedi_completed = false;
        match award {
            CouncilAward::Points(p) => self.add_score(p),
            CouncilAward::AddJediLetter => {
                self.award_jedi_letter_with_score();
                jedi_completed = self.players[self.current_player].jedi_letters == 0x0F;
            }
            CouncilAward::AdvanceBonusX => {
                let p = &mut self.players[self.current_player];
                p.bonus_multiplier = p.bonus_multiplier.saturating_add(1).min(5);
            }
        }
        self.emit_ard("ard_musical_chairs_award");
        self.trigger_leff(LeffKind::SceneHit);
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = self.mos_espa_cantina.as_ref().map_or(0, |m| m.seats_remaining);
        }
        if completed || jedi_completed {
            self.complete_active_scene(Swe1Scene::MosEspaCantina);
            self.mos_espa_cantina = None;
        } else {
            self.set_banner(format!("COLLECTED! {}", award.label()), 2_200);
        }
        true
    }

    /// Lasers trigger a Musical Chairs shuffle jump (`70,000`) while music is playing.
    pub(crate) fn handle_mos_espa_cantina_laser(&mut self) -> bool {
        let playing = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::MosEspaCantina)
            && self.mos_espa_cantina.as_ref().is_some_and(|m| !m.music_stopped);
        if !playing {
            return false;
        }
        self.trigger_musical_chairs_jump();
        true
    }

    fn trigger_musical_chairs_jump(&mut self) {
        if let Some(ref mut mc) = self.mos_espa_cantina {
            mc.jump_ms = 400;
            mc.shuffle();
        }
        self.add_score(scoring::MUSICAL_CHAIRS_JUMP);
        self.emit_ard("ard_musical_chairs_bounce");
        self.trigger_leff(LeffKind::SceneHit);
        self.set_banner("MUSICAL CHAIRS! +70,000", 1_400);
    }

    pub(crate) fn render_mos_espa_cantina(&mut self, secs: u32) {
        let mc = self.mos_espa_cantina.clone().unwrap_or_default();

        self.scene_sprite("anim_jedi_council_background", 0, BG_X, BG_Y, 1);
        let jump_offset = if mc.jump_ms > 0 {
            let phase = (400 - mc.jump_ms) as i32;
            (20 - ((phase - 200).abs() / 10)).max(0)
        } else {
            0
        };
        for (chair_idx, &member_idx) in mc.order.iter().enumerate() {
            if mc.is_seat_departed(chair_idx) {
                continue;
            }
            let anim = COUNCIL_ANIMS[member_idx % COUNCIL_ANIMS.len()];
            let frame = mc.member_frame(member_idx % COUNCIL_ANIMS.len());
            let y = CHAIR_Y - jump_offset;
            self.scene_sprite(anim, frame, CHAIR_X[chair_idx], y, 4);

            // `Member::plot_award_text`: centered below each chair; grey (`0x3DEF`) on non-center during silence
            let (line1, line2) = mc.awards[chair_idx].lines();
            let color = if mc.music_stopped && chair_idx != 2 { 0x3DEF } else { 0x07FF };
            let cx = CHAIR_X[chair_idx] + 46;
            self.scene_text("font_shadow_9_data", line1, cx, 173, 5, color, Some(0x0000), TextAlign::Center);
            self.scene_text("font_shadow_9_data", line2, cx, 185, 5, color, Some(0x0000), TextAlign::Center);
        }
        let status = if mc.music_stopped {
            format!("COLLECT CENTER: {}", mc.awards[2].label())
        } else {
            "SHOOT TO SHUFFLE! +70,000".to_string()
        };
        self.scene_text("font_shadow_9_data", status, 320, 44, 7, 0x07FF, Some(0x0000), TextAlign::Center);
        self.render_scene_footer(mc.seats_remaining, secs, "LEFT");
    }
}
