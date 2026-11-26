//! Authentic `PodraceScene` + `DeffPodRaceSceneBackground` runtime (`swe1_0200.elf`):
//! - Lane 0 (Left): `LEFT_RAMP_ENTER` (`0x1d`) / `LEFT_SHIELD_TARGET` (`0x22`) -> `(100, 230)`
//! - Lane 1 (Center): `SHIELD_HIT` (`0x27`) -> `(285, 235)`
//! - Lane 2 (Right): `RIGHT_RAMP_ENTER` (`0x1e`) / `RIGHT_SHIELD_TARGET` (`0x23`) -> `(470, 230)`
//! - 5 checkpoints (`CHECKPOINT_NAMES`) complete the race & award `min(scenes_started, 3) * 1M`.

use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::podrace_state::{
    PodraceIconKind, PodracePhase, PodraceState, PODRACE_LANE_X, PODRACE_LANE_Y,
};
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::Swe1Scene;
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_podrace_state(&mut self) {
        self.podrace = Some(PodraceState::new(
            self.elapsed_ms ^ ((self.current_player as u64 + 1) * 0x9E37_79B9),
        ));
    }

    pub(crate) fn tick_podrace(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::Podrace && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        let mut emit_ards: Vec<&'static str> = Vec::new();
        let mut finish_zoom = false;

        if let Some(ref mut pr) = self.podrace {
            pr.phase_ms = pr.phase_ms.saturating_add(dt_ms);
            pr.debounce_ms = pr.debounce_ms.saturating_sub(dt_ms);
            pr.checkpoint_banner_ms = pr.checkpoint_banner_ms.saturating_sub(dt_ms);

            match pr.phase {
                PodracePhase::StartingGate => {
                    let prev_frame = pr.phase_ms.saturating_sub(dt_ms) / 50;
                    let cur_frame = pr.phase_ms / 50;
                    if prev_frame < 15 && cur_frame >= 15 || prev_frame < 45 && cur_frame >= 45 {
                        emit_ards.push("ard_pod_race_engine_part_1");
                    } else if prev_frame < 30 && cur_frame >= 30 {
                        emit_ards.push("ard_pod_race_engine_return");
                    }
                    if pr.phase_ms >= 3_000 {
                        pr.phase = PodracePhase::Closeup;
                        pr.phase_ms = 0;
                        pr.try_place_opponent(true);
                        emit_ards.push("ard_podrace_scene_starting_flyby");
                        emit_ards.push("ard_pod_race_intro_speech_c1");
                    }
                }
                PodracePhase::Closeup => {
                    if pr.phase_ms >= 775 {
                        pr.phase = PodracePhase::Racing;
                        pr.phase_ms = 0;
                        pr.horizon_target = 80;
                    }
                }
                PodracePhase::Racing => {
                    pr.instruct_ms = pr.instruct_ms.saturating_sub(dt_ms);
                    pr.speed_decay_ms = pr.speed_decay_ms.saturating_add(dt_ms);
                    if pr.speed_decay_ms >= 9_000 {
                        pr.speed_decay_ms -= 9_000;
                        pr.speed_mph = pr.speed_mph.saturating_sub(5).max(150);
                    }
                    if pr.anakin_bank_ms > 0 {
                        pr.anakin_bank_ms = pr.anakin_bank_ms.saturating_sub(dt_ms);
                        if pr.anakin_bank_ms == 0 {
                            pr.anakin_bank = 0;
                            pr.horizon_target = 80;
                        }
                    }
                    // Smooth horizon scroll toward target (`0..=160`)
                    if pr.horizon_x < pr.horizon_target {
                        pr.horizon_x = (pr.horizon_x + 2).min(pr.horizon_target);
                    } else if pr.horizon_x > pr.horizon_target {
                        pr.horizon_x = (pr.horizon_x - 2).max(pr.horizon_target);
                    }

                    pr.try_place_icon();
                    pr.try_place_opponent(false);

                    let step = (dt_ms as i32).max(1);
                    for lane in &mut pr.lanes {
                        lane.anim_ms = lane.anim_ms.saturating_add(dt_ms);
                        if lane.flying_ms > 0 {
                            lane.flying_ms = lane.flying_ms.saturating_sub(dt_ms);
                            lane.x_q8 = lane.x_q8.saturating_add((lane.vx_q8 * step) / 16);
                            lane.y_q8 = lane.y_q8.saturating_add((lane.vy_q8 * step) / 16);
                            lane.vy_q8 = lane.vy_q8.saturating_add((28 * step) / 16);
                            if lane.flying_ms == 0 {
                                lane.flying_kind = None;
                            }
                        }
                    }

                    let mut clear_opp = false;
                    if let Some(ref mut opp) = pr.opponent {
                        opp.anim_ms = opp.anim_ms.saturating_add(dt_ms);
                        if opp.exploding_ms > 0 {
                            opp.exploding_ms = opp.exploding_ms.saturating_sub(dt_ms);
                            opp.x_q8 = opp.x_q8.saturating_add((opp.vx_q8 * step) / 16);
                            opp.y_q8 = opp.y_q8.saturating_add((opp.vy_q8 * step) / 16);
                            opp.vx_q8 = opp.vx_q8.saturating_add((opp.ax_q8 * step) / 16);
                            opp.vy_q8 = opp.vy_q8.saturating_add((opp.ay_q8 * step) / 16);
                            if opp.exploding_ms == 0 {
                                clear_opp = true;
                            }
                        } else {
                            let (_, _, w, _) = opp.sprite_info();
                            let half_w = w / 2;
                            let min_x = (205 - half_w) << 8;
                            let max_x = (365 - half_w) << 8;
                            let speed = (2 * step) << 8 / 16;
                            if opp.dir_right {
                                opp.x_q8 += speed.max(256);
                                if opp.x_q8 > max_x {
                                    opp.x_q8 = max_x;
                                    opp.dir_right = false;
                                }
                            } else {
                                opp.x_q8 -= speed.max(256);
                                if opp.x_q8 < min_x {
                                    opp.x_q8 = min_x;
                                    opp.dir_right = true;
                                }
                            }
                        }
                    }
                    if clear_opp {
                        pr.opponent = None;
                    }
                }
                PodracePhase::ZoomOut => {
                    if pr.phase_ms >= 1_000 {
                        finish_zoom = true;
                    }
                }
            }
        }

        for ard in emit_ards {
            self.emit_ard(ard);
        }
        if finish_zoom {
            self.complete_active_scene(Swe1Scene::Podrace);
            self.podrace = None;
        }
    }

    /// Handles authentic Podrace switch hits (`0x1d`, `0x22`, `0x27`, `0x23`, `0x1e`).
    pub(crate) fn handle_podrace_switch(&mut self, sw_id: u8) -> bool {
        if !self.active_scene.as_ref().is_some_and(|s| s.scene == Swe1Scene::Podrace) {
            return false;
        }
        let (lane_idx, is_ramp, is_shield_center) = match sw_id {
            sw::LEFT_RAMP_ENTER => (0usize, true, false),
            sw::LEFT_SHIELD_TARGET => (0, false, false),
            sw::SHIELD_HIT => (1, false, true),
            sw::RIGHT_SHIELD_TARGET => (2, false, false),
            sw::RIGHT_RAMP_ENTER => (2, true, false),
            _ => return false,
        };
        if self.podrace.is_none() {
            self.start_podrace_state();
        }

        let mut hit_icon: Option<PodraceIconKind> = None;
        let mut checkpoint_pts = 0u64;
        let mut completed_race = false;
        let mut speech_cue = false;
        let mut opp_boom_ard: Option<&'static str> = None;

        if let Some(ref mut pr) = self.podrace {
            if pr.phase != PodracePhase::Racing || pr.debounce_ms > 0 {
                return true;
            }
            pr.debounce_ms = 600;
            pr.anakin_bank = if lane_idx == 0 { -1 } else if lane_idx == 2 { 1 } else { 0 };
            pr.anakin_bank_ms = 750;
            pr.horizon_target = match lane_idx {
                0 => 0,
                2 => 160,
                _ => 80,
            };

            if let Some(icon) = pr.lanes[lane_idx].icon.take() {
                hit_icon = Some(icon);
                let (_, _, w, h) = icon.sprite_info();
                let dir = if (pr.next_rand() % 2) == 1 { -1 } else { 1 };
                let rand_vx = (pr.next_rand() % 100) as i32;
                let rand_vy = (pr.next_rand() % 100) as i32;
                let lane = &mut pr.lanes[lane_idx];
                lane.flying_kind = Some(icon);
                lane.flying_ms = 800;
                lane.x_q8 = (PODRACE_LANE_X[lane_idx] - w / 2) << 8;
                lane.y_q8 = (PODRACE_LANE_Y[lane_idx] - h) << 8;
                if icon == PodraceIconKind::Checkpoint {
                    lane.vx_q8 = ((285 - w / 2 - (PODRACE_LANE_X[lane_idx] - w / 2)) << 8) / 50;
                    lane.vy_q8 = ((50 - (PODRACE_LANE_Y[lane_idx] - h)) << 8) / 50;
                    pr.checkpoints = pr.checkpoints.saturating_add(1);
                    pr.checkpoint_banner_ms = 1_500;
                    checkpoint_pts = (pr.speed_mph as u64) * 10_000;
                    if pr.checkpoints >= 5 {
                        pr.phase = PodracePhase::ZoomOut;
                        pr.phase_ms = 0;
                        completed_race = true;
                    }
                } else {
                    lane.vx_q8 = dir * (200 + rand_vx);
                    lane.vy_q8 = -150 - rand_vy;
                    pr.speed_mph = (pr.speed_mph + icon.speed_boost()).min(750);
                }
            }

            if let Some(ref mut opp) = pr.opponent {
                if opp.exploding_ms == 0 {
                    opp.exploding_ms = 1_200;
                    opp.anim_ms = 0;
                    opp.vx_q8 = if (opp.x_q8 >> 8) < 285 { -40 } else { 40 };
                    opp.vy_q8 = -15;
                    opp.ax_q8 = if (opp.x_q8 >> 8) < 285 { -25 } else { 25 };
                    opp.ay_q8 = 8;
                    opp_boom_ard = Some(match opp.kind {
                        2 => "ard_fed_fight_scene_fighter_boom_2",
                        3 => "ard_fed_fight_scene_fighter_boom_3",
                        _ => "ard_fed_fight_scene_fighter_boom",
                    });
                }
            }

            pr.speech_toggle = !pr.speech_toggle;
            speech_cue = pr.speech_toggle;
        }

        // Authentic `+75,000` per switch hit (`PodraceScene::hook_*`)
        self.add_score(75_000);
        self.trigger_leff(LeffKind::ShieldHit);
        if is_shield_center {
            self.emit_ard("ard_pod_swoosh_2");
        } else if is_ramp {
            self.emit_ard("ard_pod_swoosh_1");
        } else {
            self.emit_ard("ard_pod_race_engine_part_1");
        }
        self.emit_ard("ard_pod_race_crowd");
        if speech_cue {
            self.emit_ard("ard_pod_race_random_speech");
        }
        if let Some(boom) = opp_boom_ard {
            self.emit_ard(boom);
        }

        if let Some(icon) = hit_icon {
            if icon == PodraceIconKind::Checkpoint {
                self.add_score(checkpoint_pts);
                self.emit_ard("ard_podrace_checkpoint_fanfare");
                if let Some(ref mut sc) = self.active_scene {
                    sc.hits_remaining = 5u8.saturating_sub(
                        self.podrace.as_ref().map_or(0, |p| p.checkpoints),
                    );
                }
                let cp_num = self.podrace.as_ref().map_or(1, |p| p.checkpoints);
                self.set_banner(format!("CHECKPOINT {cp_num}/5! +{checkpoint_pts}"), 2_200);
            } else {
                let spd = self.podrace.as_ref().map_or(150, |p| p.speed_mph);
                self.set_banner(format!("SPEED BOOST! {spd} MPH"), 1_500);
            }
        }
        if completed_race {
            self.emit_ard("ard_pod_race_exit_speech");
            self.emit_ard("ard_pod_race_zoom_away");
        }
        true
    }

    /// Revs pod engine on flipper/action button during `StartingGate` (`button_hit`, `0x1643a8`).
    pub(crate) fn handle_podrace_button(&mut self, right: bool) {
        let in_gate = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::Podrace && s.intro_remaining_ms == 0)
            && self
                .podrace
                .as_ref()
                .is_some_and(|p| p.phase == PodracePhase::StartingGate);
        if in_gate {
            self.emit_ard(if right {
                "ard_pod_race_engine_idle_r"
            } else {
                "ard_pod_race_engine_idle_l"
            });
        }
    }
}
