//! Authentic `ProbeDroidScene` + `DeffProbeDroidSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 3 Sith Probe Droids (`probes_remaining = 3`).
//! - Sweeping `anim_sith_probe_target` (125x65) + Queen Amidala blaster shots.

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::probe_droid_state::ProbeDroidState;
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_probe_droid_state(&mut self) {
        self.probe_droid = Some(ProbeDroidState::new());
    }

    pub(crate) fn tick_probe_droid(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::ProbeDroid && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut pd) = self.probe_droid {
            pd.scroll_x = (pd.scroll_x + ((dt_ms as i32) * 45) / 1000) % 160;
            pd.fly_ms = pd.fly_ms.saturating_add(dt_ms) % 9_000;
            pd.queen_shoot_ms = pd.queen_shoot_ms.saturating_sub(dt_ms);
            pd.explosion_ms = pd.explosion_ms.saturating_sub(dt_ms);
            pd.miss_blast_ms = pd.miss_blast_ms.saturating_sub(dt_ms);

            let step = ((dt_ms as i32) * 180) / 1000;
            pd.target_x += pd.target_dir * step.max(1);
            if pd.target_x >= 440 {
                pd.target_x = 440;
                pd.target_dir = -1;
            } else if pd.target_x <= 28 {
                pd.target_x = 28;
                pd.target_dir = 1;
            }
        }
    }

    /// Handles authentic `ProbeDroidScene` switches (`0x1d`, `0x1e`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_probe_droid_switch(&mut self, sw_id: u8) -> bool {
        let is_pd = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::ProbeDroid);
        if !is_pd {
            return false;
        }
        let (min_x, max_x) = match sw_id {
            sw::LEFT_RAMP_ENTER => (28, 180),
            sw::LEFT_SHIELD_TARGET => (38, 260),
            sw::SHIELD_HIT => (98, 348),
            sw::RIGHT_SHIELD_TARGET => (248, 440),
            sw::RIGHT_RAMP_ENTER => (280, 440),
            _ => return false,
        };
        if self.probe_droid.is_none() {
            self.start_probe_droid_state();
        }
        let mut hit = false;
        let mut completed = false;
        if let Some(ref mut pd) = self.probe_droid {
            pd.queen_shoot_ms = 350;
            if (min_x..=max_x).contains(&pd.target_x) {
                hit = true;
                pd.explosion_ms = 500;
                pd.hit_variant = pd.hit_variant.wrapping_add(1);
                pd.probes_remaining = pd.probes_remaining.saturating_sub(1);
                completed = pd.probes_remaining == 0;
            } else {
                pd.probes_missed = pd.probes_missed.saturating_add(1);
                pd.miss_blast_ms = 350;
            }
        }
        if hit {
            self.add_score(scoring::PROBE_DROID_DESTROY);
            self.emit_ard("ard_probe_droid_scene_probe_boom");
            self.trigger_leff(LeffKind::SceneHit);
            if let Some(ref mut sc) = self.active_scene {
                sc.hits_remaining = self
                    .probe_droid
                    .as_ref()
                    .map_or(0, |p| p.probes_remaining);
            }
            if completed {
                self.complete_active_scene(Swe1Scene::ProbeDroid);
                self.probe_droid = None;
            }
        } else {
            self.emit_ard("ard_probe_droid_scene_probe_miss");
        }
        true
    }

    pub(crate) fn render_probe_droid(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let pd = self.probe_droid.clone().unwrap_or_default();

        self.scene_sprite("anim_sith_probe_background", 0, -pd.scroll_x, 0, 1);
        self.scene_sprite("anim_sith_probe_hover_far", f60, 470, 40, 2);

        let pf = pd.fly_ms % 3_000;
        if pf < 600 {
            let k = (pd.fly_ms / 3_000) % 3;
            let name = [
                "anim_sith_probe_fly_1",
                "anim_sith_probe_fly_2",
                "anim_sith_probe_fly_3",
            ][k as usize];
            self.scene_sprite(name, f60, 640 - (pf as i32), 30, 3);
        }

        self.scene_sprite("anim_sith_probe_hover", f60, 258, 30, 4);
        if pd.queen_shoot_ms > 0 {
            self.scene_sprite("anim_sith_probe_queen_shoot_2", f60, 30, 63, 5);
        } else {
            self.scene_sprite("anim_sith_probe_queen", f60, 30, 65, 5);
        }
        self.scene_sprite("anim_sith_probe_target", f60, pd.target_x, 150, 6);
        self.scene_sprite("anim_sith_probe_xhair", f60, 248, 30, 6);

        if pd.explosion_ms > 0 {
            let fr = ((500 - pd.explosion_ms) * 60 / 1000) as usize;
            let anim = if pd.hit_variant % 2 == 0 {
                "anim_sith_probe_explosion_1"
            } else {
                "anim_sith_probe_explosion_2"
            };
            self.scene_sprite(anim, fr, 120, 20, 7);
        } else if pd.miss_blast_ms > 0 {
            let fr = ((350 - pd.miss_blast_ms) * 60 / 1000) as usize;
            self.scene_sprite("anim_sith_probe_miss_explosion", fr, pd.target_x, 120, 7);
        }

        self.render_scene_footer(pd.probes_remaining, secs, "PROBES");
    }
}
