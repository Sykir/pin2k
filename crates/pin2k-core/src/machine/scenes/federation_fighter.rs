//! Authentic `FederationFighterScene` + `DeffFedFighterSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 9 Vulture Droids (`fighters_remaining = 9`) + Droid Control Ship (`SHIELD_POPPER` `0x24`).
//! - Handles `SHIELD_HIT` (`0x27`), `LEFT/RIGHT_SHIELD_TARGET` (`0x22`/`0x23`),
//!   `LEFT/RIGHT_RAMP_ENTER` (`0x1d`/`0x1e`), and `SHIELD_POPPER` (`0x24`).

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::federation_fighter_state::{FederationFighterState, FighterPhase};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::Swe1Scene;
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_federation_fighter_state(&mut self) {
        self.federation_fighter = Some(FederationFighterState::new());
    }

    pub(crate) fn tick_federation_fighter(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::FederationFighter && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut ff) = self.federation_fighter {
            ff.scroll_x = (ff.scroll_x + ((dt_ms as i32) * 60) / 1000) % 260;
            ff.zap_ms = ff.zap_ms.saturating_add(dt_ms) % 667;
            ff.phase_ms = ff.phase_ms.saturating_add(dt_ms);
            match ff.phase {
                FighterPhase::Approaching if ff.phase_ms >= 1_500 => {
                    ff.phase = FighterPhase::RedEye;
                    ff.phase_ms = 0;
                }
                FighterPhase::RedEye if ff.phase_ms >= 1_000 => {
                    ff.phase = FighterPhase::Flyby;
                    ff.phase_ms = 0;
                }
                FighterPhase::Flyby if ff.phase_ms >= 450 => {
                    ff.phase = FighterPhase::Approaching;
                    ff.phase_ms = 0;
                }
                FighterPhase::Exploding if ff.phase_ms >= 500 => {
                    ff.phase = FighterPhase::Approaching;
                    ff.phase_ms = 0;
                }
                _ => {}
            }
        }
    }

    /// Handles authentic `FederationFighterScene` switches (`0x1d`, `0x1e`, `0x22`, `0x23`, `0x24`, `0x27`).
    pub(crate) fn handle_federation_fighter_switch(&mut self, sw_id: u8) -> bool {
        let is_ff = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::FederationFighter);
        if !is_ff {
            return false;
        }
        let is_popper = sw_id == sw::SHIELD_POPPER;
        let valid = matches!(
            sw_id,
            sw::LEFT_RAMP_ENTER
                | sw::RIGHT_RAMP_ENTER
                | sw::LEFT_SHIELD_TARGET
                | sw::RIGHT_SHIELD_TARGET
                | sw::SHIELD_HIT
                | sw::SHIELD_POPPER
        );
        if !valid {
            return false;
        }
        if self.federation_fighter.is_none() {
            self.start_federation_fighter_state();
        }
        let dmg = if is_popper { 3 } else { 1 };
        let mut completed = false;
        if let Some(ref mut ff) = self.federation_fighter {
            ff.fighters_remaining = ff.fighters_remaining.saturating_sub(dmg);
            ff.phase = FighterPhase::Exploding;
            ff.phase_ms = 0;
            ff.hit_variant = ff.hit_variant.wrapping_add(1);
            completed = ff.fighters_remaining == 0;
        }
        self.add_score(if is_popper { 500_000 } else { 250_000 });
        self.emit_ard("ard_fed_fight_scene_fighter_boom");
        self.trigger_leff(LeffKind::SceneHit);
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = self
                .federation_fighter
                .as_ref()
                .map_or(0, |f| f.fighters_remaining);
        }
        if completed {
            self.complete_active_scene(Swe1Scene::FederationFighter);
            self.federation_fighter = None;
        }
        true
    }

    pub(crate) fn render_federation_fighter(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let ff = self.federation_fighter.clone().unwrap_or_default();

        self.scene_sprite("anim_federation_fighter_background", 0, -ff.scroll_x, 0, 1);
        self.scene_sprite("anim_droid_control_ship", 0, 120, 8, 2);
        self.scene_sprite("anim_droid_control_ship2", 0, 430, 20, 2);
        self.scene_sprite("anim_federation_fighter_side", 0, 560, -20, 2);

        match ff.phase {
            FighterPhase::Approaching => {
                self.scene_sprite("anim_federation_fighter", f60, 220, 74, 4);
            }
            FighterPhase::RedEye => {
                self.scene_sprite("anim_federation_fighter_redeye", f60, 220, 74, 4);
            }
            FighterPhase::Flyby => {
                let fr = ((ff.phase_ms * 60) / 1000) as usize;
                self.scene_sprite("anim_federation_fighter_flyby", fr, 216, 35, 3);
            }
            FighterPhase::Exploding => {
                let fr = ((ff.phase_ms * 60) / 1000) as usize;
                let anim = if ff.hit_variant % 2 == 0 {
                    "anim_federation_fighter_explosion"
                } else {
                    "anim_federation_fighter_explosion_2"
                };
                self.scene_sprite(anim, fr, 129, 18, 7);
            }
        }

        self.scene_sprite("anim_naboo_xhair", 0, 245, 78, 5);
        if ff.zap_ms < 200 {
            self.scene_sprite("anim_federation_fighter_zap", f60, 300, 112, 5);
        }

        self.render_scene_footer(ff.fighters_remaining, secs, "FIGHTERS");
    }
}
