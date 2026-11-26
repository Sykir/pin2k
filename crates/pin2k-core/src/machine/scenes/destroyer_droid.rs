//! Authentic `DestroyerDroidScene` + `DeffDestroyerDroidSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - 6 Droidekas (`S`, `M`, `L` slots cycling `Rolling -> Idle -> Shooting -> Exploding`)
//! - Sweeping `anim_sith_probe_target` (125x65) + hit windows for `LEFT_RAMP_ENTER`,
//!   `LEFT_SHIELD_TARGET`, `SHIELD_HIT`, `RIGHT_SHIELD_TARGET`, `RIGHT_RAMP_ENTER`.

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::destroyer_droid_state::{DestroyerDroidPhase, DestroyerDroidState, DestroyerSize};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_destroyer_droid_state(&mut self) {
        self.destroyer_droid = Some(DestroyerDroidState::new());
    }

    pub(crate) fn tick_destroyer_droid(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::DestroyerDroid && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        if let Some(ref mut dd) = self.destroyer_droid {
            dd.miss_blast_ms = dd.miss_blast_ms.saturating_sub(dt_ms);
            // Sweep target cursor between x=28 and x=440
            let step = ((dt_ms as i32) * 180) / 1000;
            dd.target_x += dd.target_dir * step.max(1);
            if dd.target_x >= 440 {
                dd.target_x = 440;
                dd.target_dir = -1;
            } else if dd.target_x <= 28 {
                dd.target_x = 28;
                dd.target_dir = 1;
            }
            for slot in &mut dd.slots {
                slot.phase_ms = slot.phase_ms.saturating_add(dt_ms);
                match slot.phase {
                    DestroyerDroidPhase::Rolling if slot.phase_ms >= 1_083 => {
                        slot.phase = DestroyerDroidPhase::Idle;
                        slot.phase_ms = 0;
                    }
                    DestroyerDroidPhase::Idle if slot.phase_ms >= 1_800 => {
                        slot.phase = DestroyerDroidPhase::Shooting;
                        slot.phase_ms = 0;
                    }
                    DestroyerDroidPhase::Shooting if slot.phase_ms >= 450 => {
                        slot.phase = DestroyerDroidPhase::Idle;
                        slot.phase_ms = 0;
                    }
                    DestroyerDroidPhase::Exploding if slot.phase_ms >= 550 => {
                        slot.phase = DestroyerDroidPhase::Rolling;
                        slot.phase_ms = 0;
                    }
                    _ => {}
                }
            }
        }
    }

    /// Handles authentic `DestroyerDroidScene` switches (`0x1d`, `0x1e`, `0x22`, `0x23`, `0x27`).
    pub(crate) fn handle_destroyer_droid_switch(&mut self, sw_id: u8) -> bool {
        let is_dd = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::DestroyerDroid);
        if !is_dd {
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
        if self.destroyer_droid.is_none() {
            self.start_destroyer_droid_state();
        }
        let mut hit = false;
        let mut completed = false;
        let mut speech = false;
        if let Some(ref mut dd) = self.destroyer_droid {
            dd.speech_counter = dd.speech_counter.wrapping_add(1);
            speech = (dd.speech_counter & 1) == 1;
            if (min_x..=max_x).contains(&dd.target_x) {
                hit = true;
                dd.no_hits = false;
                dd.destroyers_remaining = dd.destroyers_remaining.saturating_sub(1);
                let slot_idx = (5 - dd.destroyers_remaining as usize) % 3;
                dd.slots[slot_idx].phase = DestroyerDroidPhase::Exploding;
                dd.slots[slot_idx].phase_ms = 0;
                if dd.destroyers_remaining == 0 {
                    completed = true;
                }
            } else {
                dd.destroyers_missed = dd.destroyers_missed.saturating_add(1);
                dd.miss_blast_ms = 350;
            }
        }
        if speech {
            self.emit_ard("ard_hit_destroyer_droid_speech");
        }
        if hit {
            self.add_score(scoring::DESTROYER_DROID_DESTROY);
            self.emit_ard("ard_destroyer_droid_scene_destroyer_boom");
            self.trigger_leff(LeffKind::SceneHit);
            if let Some(ref mut sc) = self.active_scene {
                sc.hits_remaining = self
                    .destroyer_droid
                    .as_ref()
                    .map_or(0, |d| d.destroyers_remaining);
            }
            if completed {
                self.complete_active_scene(Swe1Scene::DestroyerDroid);
                self.destroyer_droid = None;
            }
        } else {
            self.emit_ard("ard_destroyer_droid_scene_destroyer_miss");
        }
        true
    }

    pub(crate) fn render_destroyer_droid(&mut self, secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        self.scene_sprite("anim_destroyer_droid_background", 0, 0, 0, 1);
        let dd = self.destroyer_droid.clone().unwrap_or_default();

        for slot in &dd.slots {
            let (base_x, base_y, z) = slot.size.pos();
            match slot.phase {
                DestroyerDroidPhase::Rolling => {
                    let fr = ((slot.phase_ms * 60) / 1000) as usize;
                    let rx = match slot.size {
                        DestroyerSize::Small => 0,
                        DestroyerSize::Medium => 235,
                        DestroyerSize::Large => 20,
                    };
                    self.scene_sprite(slot.size.anim_roll(), fr, rx, base_y, z);
                }
                DestroyerDroidPhase::Idle => {
                    self.scene_sprite(slot.size.anim_idle(), f60, base_x, base_y, z);
                }
                DestroyerDroidPhase::Shooting => {
                    self.scene_sprite(slot.size.anim_shooting(), f60, base_x, base_y, z + 1);
                }
                DestroyerDroidPhase::Exploding => {
                    let fr = ((slot.phase_ms * 60) / 1000) as usize;
                    self.scene_sprite(slot.size.anim_explosion(), fr, base_x, base_y.saturating_sub(15), 7);
                }
            }
        }

        // Sweeping target reticle (`anim_sith_probe_target`, 125x65)
        self.scene_sprite("anim_sith_probe_target", f60, dd.target_x, 100, 6);
        if dd.miss_blast_ms > 0 {
            let fr = ((350 - dd.miss_blast_ms) * 60 / 1000) as usize;
            self.scene_sprite("anim_sith_probe_miss_explosion", fr, dd.target_x, 120, 7);
        }

        self.render_scene_footer(dd.destroyers_remaining, secs, "DROIDEKAS");
    }
}
