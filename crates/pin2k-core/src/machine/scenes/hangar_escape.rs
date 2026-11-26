//! Authentic `HangarEscapeScene` + `DeffHangarEscapeSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 8..14 Hangar Droids (`((attempts - 1) * 6 + 8).min(14)`).
//! - Handles `SHIELD_HIT` (`0x27`), `LEFT/RIGHT_SHIELD_TARGET` (`0x22`/`0x23`),
//!   `LEFT/RIGHT_RAMP_ENTER` (`0x1d`/`0x1e`), and `SHIELD_POPPER` (`0x24`).

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::hangar_escape_state::HangarEscapeState;
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
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

    /// Handles authentic `HangarEscapeScene` switches (`0x1d`, `0x1e`, `0x22`, `0x23`, `0x24`, `0x27`).
    pub(crate) fn handle_hangar_escape_switch(&mut self, sw_id: u8) -> bool {
        let is_he = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::HangarEscape);
        if !is_he {
            return false;
        }
        let dmg = match sw_id {
            sw::SHIELD_POPPER => 2u8,
            sw::SHIELD_HIT
            | sw::LEFT_SHIELD_TARGET
            | sw::RIGHT_SHIELD_TARGET
            | sw::LEFT_RAMP_ENTER
            | sw::RIGHT_RAMP_ENTER => 1u8,
            _ => return false,
        };
        if self.hangar_escape.is_none() {
            self.start_hangar_escape_state();
        }
        let mut completed = false;
        if let Some(ref mut he) = self.hangar_escape {
            he.droids_remaining = he.droids_remaining.saturating_sub(dmg);
            he.explosion_ms = 500;
            he.hit_variant = he.hit_variant.wrapping_add(1);
            completed = he.droids_remaining == 0;
        }
        self.add_score(scoring::HANGAR_ESCAPE_HIT * 5);
        self.emit_ard("ard_hangar_escape_scene_droid_boom");
        self.trigger_leff(LeffKind::SceneHit);
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = self
                .hangar_escape
                .as_ref()
                .map_or(0, |h| h.droids_remaining);
        }
        if completed {
            self.complete_active_scene(Swe1Scene::HangarEscape);
            self.hangar_escape = None;
        }
        true
    }

    pub(crate) fn render_hangar_escape(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let he = self.hangar_escape.clone().unwrap_or_default();

        self.scene_sprite("anim_federation_fighter_cockpit", 0, 0, 0, 1);
        self.scene_sprite("anim_hangar_droid_run", f60, 275, 48, 3);
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
            self.scene_sprite(anim, fr, 140, 42, 7);
        }

        self.render_scene_footer(he.droids_remaining, secs, "TARGETS");
    }
}
