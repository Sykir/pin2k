use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::{LeffKind, MachineState, Pin2kMachine};
use crate::events::GameEvent;
use crate::swe1_rules::{scoring, ActiveMultiball, ActiveSceneState, Swe1Scene};
use crate::swe1_tables::{drive, sound, sw};

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(super) fn set_shield_raised(&mut self, raised: bool) {
        if raised && !self.shield_raised {
            self.hw.fire_coil(drive::SHIELD_POWER, 64);
            self.hw.set_drive_hold(drive::SHIELD_HOLD, true);
            self.shield_fx(true);
            self.holo_open_fx();
        } else if !raised && self.shield_raised {
            self.hw.set_drive_hold(drive::SHIELD_HOLD, false);
            self.shield_fx(false);
        }
        self.shield_raised = raised;
    }

    pub(super) fn cut_all_holds(&mut self) {
        for d in [
            drive::LEFT_FLIPPER_HOLD,
            drive::RIGHT_FLIPPER_HOLD,
            drive::SHIELD_HOLD,
            drive::MAGNET,
            drive::LIGHTSABER_NEON,
        ] {
            self.hw.set_drive_hold(d, false);
        }
    }

    pub(super) fn handle_ingame_switch_closed(&mut self, id: u8) {
        self.idle_ball_search_ms = 0;

        match id {
            sw::START_BUTTON => {
                self.try_start_or_add_player();
            }
            sw::PLUMB_TILT => {
                self.tilt_warnings += 1;
                if self.tilt_warnings >= self.adjustments.tilt_warnings_max {
                    self.cut_all_holds();
                    self.state = MachineState::Tilt;
                    self.emit_ard("ard_tilt_random_speech");
                    self.emit_event(GameEvent::Tilted);
                    self.set_banner("TILT! BONUS LOST", 4_000);
                } else {
                    self.emit_ard("ard_tilt_warning_speech");
                    self.emit_event(GameEvent::TiltWarning {
                        warnings: self.tilt_warnings,
                    });
                    self.set_banner(format!("DANGER! TILT WARNING {}/2", self.tilt_warnings), 2_000);
                }
            }
            // Flipper buttons: fire high-power kick + hold coil + lane-change JEDI letters
            sw::LEFT_FLIPPER_BUTTON => {
                self.hw.fire_coil(drive::LEFT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(drive::LEFT_FLIPPER_HOLD, true);
                self.players[self.current_player].rotate_jedi_letters(false);
                if self.players[self.current_player].scene_select_ready && self.active_scene.is_none() {
                    self.cycle_scene_selection(false);
                }
            }
            sw::RIGHT_FLIPPER_BUTTON => {
                self.hw.fire_coil(drive::RIGHT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(drive::RIGHT_FLIPPER_HOLD, true);
                self.players[self.current_player].rotate_jedi_letters(true);
                if self.players[self.current_player].scene_select_ready && self.active_scene.is_none() {
                    self.cycle_scene_selection(true);
                }
            }
            // Cabinet Laser Action Buttons (`Lasers::fire_left` / `Lasers::fire_right`)
            sw::LEFT_ACTION_BUTTON | sw::RIGHT_ACTION_BUTTON => {
                let right = id == sw::RIGHT_ACTION_BUTTON;
                if self.players[self.current_player].scene_select_ready && self.active_scene.is_none() {
                    self.cycle_scene_selection(right);
                    return;
                }
                if self.players[self.current_player].laser_shots > 0 {
                    self.players[self.current_player].laser_shots -= 1;
                    self.audio.play_sound(sound::LASER_FIRE, false);
                    self.trigger_leff(LeffKind::LaserFired { right });
                    if self.active_scene.is_some() {
                        self.advance_active_scene("LASER DIRECT HIT!");
                    } else {
                        self.add_score(scoring::LASER_FIRE);
                        self.set_banner(format!("LASER FIRED! +{}", scoring::LASER_FIRE), 1_500);
                    }
                } else {
                    self.emit_ard("ard_generic_laser_empty");
                    self.set_banner("NO LASERS CHARGED!", 1_200);
                }
            }
            // Slingshots & Jet Bumpers (`JetBumpers::jet_bumper_hit` + G-U-N-G-A-N Super Jets)
            sw::LEFT_SLING => {
                self.hw.fire_coil(drive::LEFT_SLING, 48);
                self.hw.fire_coil(drive::LOWER_LEFT_HOTDOG_FL, 24);
                self.add_score(5_110);
            }
            sw::RIGHT_SLING => {
                self.hw.fire_coil(drive::RIGHT_SLING, 48);
                self.hw.fire_coil(drive::LOWER_RIGHT_HOTDOG_FL, 24);
                self.add_score(5_110);
            }
            sw::UPPER_JET | sw::MIDDLE_JET | sw::LOWER_JET => {
                let coil = match id {
                    sw::UPPER_JET => drive::UPPER_JET,
                    sw::MIDDLE_JET => drive::MIDDLE_JET,
                    _ => drive::LOWER_JET,
                };
                self.hw.fire_coil(coil, 40);
                self.hw.fire_coil(drive::JET_FLASHER, 24);
                self.audio.play_sound(sound::JET_BUMPER_SFX, false);
                self.emit_ard("ard_jet_hit");
                let (pts, started_super_jets, gungan_step) = {
                    let p = &mut self.players[self.current_player];
                    p.jet_hits += 1;
                    if p.super_jets_ms > 0 {
                        (scoring::JET_SUPER, false, 6u8)
                    } else {
                        p.gungan_letters += 1;
                        if p.gungan_letters >= 6 {
                            p.gungan_letters = 0;
                            p.super_jets_ms = 20_000;
                            (scoring::JET_NORMAL, true, 6u8)
                        } else {
                            (scoring::JET_NORMAL, false, p.gungan_letters)
                        }
                    }
                };
                self.add_score(pts);
                let jet_idx = id.saturating_sub(sw::UPPER_JET);
                self.bumper_sparks_fx(jet_idx);
                self.jarjar_saucer_event(false, jet_idx);
                if started_super_jets {
                    self.emit_ard("ard_gungan_letters_done");
                    self.trigger_leff(LeffKind::SuperJetsStart);
                    self.set_banner("G-U-N-G-A-N COMPLETE! SUPER JETS!", 3_000);
                } else if gungan_step < 6 {
                    self.emit_ard("ard_gungan_letter_earn");
                } else {
                    self.set_banner(format!("SUPER JETS! +{}", scoring::JET_SUPER), 1_500);
                }
            }
            sw::JETS_ROLLOVER => {
                if self.skill_shot_ready {
                    self.award_skill_shot();
                } else {
                    let p = &mut self.players[self.current_player];
                    if p.bonus_multiplier < scoring::BONUS_MAX_MULTIPLIER {
                        p.bonus_multiplier += 1;
                    }
                    let bm = p.bonus_multiplier;
                    self.add_score(50_000);
                    self.set_banner(format!("BONUS {}X!", bm), 1_800);
                }
            }
            // Standup Banks charge Lasers
            sw::LEFT_BANK_UPPER | sw::LEFT_BANK_MIDDLE | sw::LEFT_BANK_LOWER
            | sw::RIGHT_BANK_UPPER | sw::RIGHT_BANK_MIDDLE | sw::RIGHT_BANK_LOWER => {
                self.skill_shot_ready = false;
                let p = &mut self.players[self.current_player];
                p.laser_shots = (p.laser_shots + 1).min(9);
                let ls = p.laser_shots;
                self.add_score(35_000);
                if ls == 1 {
                    self.laser_instruction_fx();
                }
                self.set_banner(format!("LASERS CHARGED: {ls}"), 1_500);
            }
            // Drop Targets (`Drop::hook_drop_target_hit`) & Saucers
            sw::LEFT_DROP_TARGET => {
                self.left_drop_down = true;
                self.add_score(scoring::DROP_TARGET);
                self.emit_ard("ard_hit_bdroids_speech");
                self.trigger_leff(LeffKind::ShieldHit);
                self.set_banner("LEFT SAUCER OPEN!", 1_800);
            }
            sw::RIGHT_DROP_TARGET => {
                self.right_drop_down = true;
                self.add_score(scoring::DROP_TARGET);
                self.emit_ard("ard_hit_bdroids_speech");
                self.trigger_leff(LeffKind::ShieldHit);
                self.set_banner("RIGHT SAUCER OPEN!", 1_800);
            }
            sw::LEFT_SAUCER | sw::RIGHT_SAUCER => {
                let idx = if id == sw::LEFT_SAUCER { 0 } else { 1 };
                self.saucer_hold_ms[idx] = 800;
                self.on_saucer_entered();
            }
            // Deflector Shield Targets & Center Shield Popper (`Scenes::start_valid_scene`)
            sw::LEFT_SHIELD_TARGET | sw::RIGHT_SHIELD_TARGET | sw::SHIELD_HIT => {
                self.skill_shot_ready = false;
                self.audio.play_sound(sound::SHIELD_HIT_SFX, false);
                self.trigger_leff(LeffKind::ShieldHit);
                if self.active_scene.is_some() {
                    self.advance_active_scene("SHIELD BLAST!");
                } else if let ActiveMultiball::DarthMaulDuel { .. } = self.multiball {
                    self.damage_darth_maul(20, false, "SABER STRIKE!");
                } else {
                    let rem = {
                        let p = &mut self.players[self.current_player];
                        p.shield_health = p.shield_health.saturating_sub(2);
                        p.shield_health
                    };
                    self.add_score(50_000);
                    if rem == 0 {
                        let selected = {
                            let p = &mut self.players[self.current_player];
                            p.scene_select_ready = true;
                            Swe1Scene::ALL[p.next_scene_idx % Swe1Scene::ALL.len()]
                        };
                        self.set_shield_raised(true);
                        self.emit_sound(sound::SHIELD_GONE_CALLOUT, false);
                        self.active_anim_hint = selected.holo_anim_hint().to_string();
                        self.set_banner(
                            format!("SHIELD DOWN! SELECT: {} (FLIPPERS) -> SHOOT SCOOP/RAMP!", selected.title()),
                            4_000,
                        );
                    } else {
                        self.active_anim_hint = "anim_shield".to_string();
                        self.set_banner(format!("DEFLECTOR SHIELD: {}%", (rem as u32) * 100 / 12), 1_500);
                    }
                }
            }
            sw::SHIELD_POPPER => {
                self.popper_hold_ms = 900;
                self.on_shield_popper_entered();
            }
            // Left & Right Ramps + Left Loop -> Virtual Spinners + Combos + JEDI + Scene/Multiball
            sw::LEFT_RAMP_ENTER => {
                self.skill_shot_ready = false;
                self.trigger_virtual_spinner(false, true);
                self.add_score(25_000);
            }
            sw::RIGHT_RAMP_ENTER => {
                self.skill_shot_ready = false;
                self.trigger_virtual_spinner(true, true);
                self.jarjar_saucer_event(true, 0);
                self.add_score(25_000);
            }
            sw::RAMP_MADE_LEFT | sw::RAMP_MADE_RIGHT | sw::LEFT_LOOP_UPPER | sw::LEFT_LOOP_ROLLOVER => {
                self.skill_shot_ready = false;
                if id == sw::RAMP_MADE_LEFT {
                    self.trigger_virtual_spinner(false, true);
                } else if id == sw::RAMP_MADE_RIGHT {
                    self.trigger_virtual_spinner(true, true);
                }
                self.on_major_shot_made(id);
            }
            // Inlanes, Sneaky Lane Skill Shot & Captive Ball (Build C-3PO)
            sw::SNEAKY_LANE => {
                if self.skill_shot_ready {
                    self.award_skill_shot();
                } else {
                    self.hw.fire_coil(drive::RIGHT_INLANE_FL, 24);
                    self.add_score(25_000);
                }
            }
            sw::LEFT_INLANE | sw::RIGHT_INLANE => {
                let fl = if id == sw::LEFT_INLANE {
                    drive::LEFT_INLANE_FL
                } else {
                    drive::RIGHT_INLANE_FL
                };
                self.hw.fire_coil(fl, 24);
                self.add_score(25_000);
                if self.active_scene.is_none() {
                    self.advance_c3p0();
                }
            }
            sw::CAPTIVE_BALL => {
                self.audio.play_sound(sound::EXPLOSION, false);
                self.add_score(100_000);
                self.advance_c3p0();
            }
            sw::LEFT_OUTLANE | sw::RIGHT_OUTLANE => {
                self.add_score(50_000);
                if self.ball_save_ms > 0 {
                    self.set_banner("FORCE SAVE READY!", 1_800);
                }
            }
            // Ball drained into trough!
            sw::TROUGH_BALL_1 | sw::TROUGH_BALL_2 | sw::TROUGH_BALL_3 | sw::TROUGH_BALL_4 => {
                if self.balls_in_play > 0 && self.eject_cooldown_ms == 0 {
                    self.on_ball_drained();
                }
            }
            _ => {}
        }
    }

    fn award_skill_shot(&mut self) {
        self.skill_shot_ready = false;
        let pts = {
            let p = &mut self.players[self.current_player];
            p.skill_shots_made = p.skill_shots_made.saturating_add(1);
            (500_000 * (p.skill_shots_made as u64)).min(scoring::SKILL_SHOT_CAP)
        };
        self.add_score(pts);
        self.emit_ard("ard_skill_shot_speech");
        self.emit_ard("ard_skill_shot_award");
        self.trigger_leff(LeffKind::SkillShotAward);
        self.emit_event(GameEvent::SkillShotAwarded { points: pts });
        self.set_banner(format!("SKILL SHOT! +{}", pts), 2_500);
    }

    fn advance_c3p0(&mut self) {
        let completed = {
            let p = &mut self.players[self.current_player];
            p.c3p0_parts = p.c3p0_parts.saturating_add(1);
            if p.c3p0_parts >= 6 {
                p.c3p0_parts = 0;
                p.extra_balls = p.extra_balls.saturating_add(1);
                true
            } else {
                false
            }
        };
        if completed {
            self.add_score(1_000_000);
            self.emit_ard("ard_extra_ball_lit_speech");
            self.trigger_leff(LeffKind::C3P0Complete);
            self.set_banner("C-3PO COMPLETE! EXTRA BALL +1,000,000!", 3_500);
        } else {
            let parts = self.players[self.current_player].c3p0_parts;
            self.set_banner(format!("BUILD C-3PO: PART {parts}/6"), 1_800);
        }
    }

    pub(super) fn on_saucer_entered(&mut self) {
        if !matches!(self.multiball, ActiveMultiball::None) {
            self.add_score(300_000);
            self.set_banner("SAUCER BONUS! +300,000", 2_000);
            return;
        }
        let p = &mut self.players[self.current_player];
        p.saucer_locks += 1;
        if p.saucer_locks >= 3 {
            p.saucer_locks = 0;
            self.multiball = ActiveMultiball::HangarMultiball {
                jackpots_lit: true,
                jackpots_collected: 0,
            };
            self.pending_eject_balls = self.pending_eject_balls.saturating_add(2);
            self.ball_save_ms = 20_000;
            self.special_screen_ms = 0;
            self.emit_sound(sound::MULTIBALL_MUSIC_LOOP, true);
            self.trigger_leff(LeffKind::MainMultiballStart);
            self.active_anim_hint = "anim_hangar_droid_explosion".to_string();
            self.audits.multiballs_started = self.audits.multiballs_started.saturating_add(1);
            self.add_score(scoring::MULTIBALL_START);
            self.emit_event(GameEvent::MultiballStarted {
                mode: "THEED HANGAR 3-BALL MULTIBALL".to_string(),
            });
            self.set_banner("THEED HANGAR 3-BALL MULTIBALL!", 4_000);
        } else {
            let locks = self.players[self.current_player].saucer_locks;
            self.add_score(150_000);
            self.emit_ard("ard_watto_flyup_speech");
            if locks == 1 {
                self.start_overlay(super::OverlayKind::WattoJunkShop {
                    award: (self.elapsed_ms / 16) as u8,
                });
            } else {
                self.start_overlay(super::OverlayKind::Marketplace {
                    award: (self.elapsed_ms / 16) as u8,
                });
            }
            self.set_banner(format!("BALL {locks} LOCKED FOR MULTIBALL!"), 2_500);
        }
    }

    pub(super) fn on_shield_popper_entered(&mut self) {
        if let ActiveMultiball::HangarMultiball {
            jackpots_lit,
            jackpots_collected,
        } = self.multiball
        {
            if jackpots_lit && jackpots_collected >= 2 {
                self.multiball = ActiveMultiball::HangarMultiball {
                    jackpots_lit: true,
                    jackpots_collected: 0,
                };
                self.emit_coil(drive::LIGHTSABER_NEON, 160);
                self.emit_sound(sound::EXPLOSION, false);
                self.emit_ard("ard_multiball_jackpot_cleared_speech");
                self.trigger_leff(LeffKind::MainMultiballStart);
                self.jackpot_fx(0, true);
                self.add_score(scoring::MULTIBALL_SUPER_JACKPOT);
                self.emit_event(GameEvent::JackpotAwarded {
                    super_jackpot: true,
                    points: scoring::MULTIBALL_SUPER_JACKPOT,
                });
                self.set_banner("SUPER JACKPOT!! +5,000,000", 3_500);
                return;
            }
        }

        if self.active_scene.is_some() {
            self.advance_active_scene("DIRECT CORE HIT!");
            return;
        }

        // Start the next SWE1 Holoprojector Scene (with intro movie before ball eject!)
        let scene = self.players[self.current_player].choose_next_scene();
        self.start_swe1_scene(scene);
    }

    pub fn cycle_scene_selection(&mut self, forward: bool) {
        let p = &mut self.players[self.current_player];
        let n = Swe1Scene::ALL.len();
        p.next_scene_idx = if forward {
            (p.next_scene_idx + 1) % n
        } else {
            (p.next_scene_idx + n - 1) % n
        };
        let scene = Swe1Scene::ALL[p.next_scene_idx];
        self.active_anim_hint = scene.holo_anim_hint().to_string();
        self.emit_sound(sound::LASER_SFX_OR_CLICK(), false);
        self.set_banner(format!("SELECT MODE: {} (SCOOP/RAMP TO START)", scene.title()), 3_000);
    }

    pub fn start_swe1_scene(&mut self, scene: Swe1Scene) {
        let intro_ms = scene.intro_duration_ms();
        {
            let p = &mut self.players[self.current_player];
            p.shield_health = 8;
            p.scene_select_ready = false;
            p.scenes_started = p.scenes_started.saturating_add(1);
        }
        self.set_shield_raised(false);
        self.holo_close_fx();
        self.popper_hold_ms = intro_ms;
        self.special_screen = None;
        self.special_screen_ms = 0;
        self.active_scene = Some(ActiveSceneState {
            scene,
            hits_remaining: scene.required_hits(),
            timer_ms: scene.duration_ms(),
            intro_remaining_ms: intro_ms,
            active_ms: 0,
        });
        self.active_anim_hint = scene.intro_anim_hint().to_string();
        self.audits.scenes_started = self.audits.scenes_started.saturating_add(1);
        self.emit_sound(scene.music_id(), true);
        self.emit_ard(scene.start_ard());
        self.trigger_leff(LeffKind::SceneHit);
        self.add_score(250_000);
        if scene == Swe1Scene::QuestionMark {
            let p = &mut self.players[self.current_player];
            match (self.elapsed_ms / 16) % 4 {
                0 => self.add_score(1_000_000),
                1 => p.laser_shots = (p.laser_shots + 3).min(9),
                2 => p.bonus_multiplier = (p.bonus_multiplier + 2).min(scoring::BONUS_MAX_MULTIPLIER),
                _ => p.extra_balls = p.extra_balls.saturating_add(1),
            }
        }
        self.emit_event(GameEvent::SceneStarted {
            scene,
            title: scene.title().to_string(),
        });
        self.set_banner(format!("INTRO: {}", scene.title()), intro_ms.max(2_500));
    }

    pub fn start_darth_maul_duel(&mut self) {
        self.players[self.current_player].jedi_letters = 0;
        self.right_saber_color = 1; // Red Sith lightsaber
        self.hw.set_drive_hold(drive::LIGHTSABER_NEON, true);
        self.multiball = ActiveMultiball::DarthMaulDuel {
            maul_health: 100,
            combo_window_ms: 5_000,
        };
        self.pending_eject_balls = self.pending_eject_balls.saturating_add(1);
        self.ball_save_ms = 20_000;
        self.emit_sound(sound::DUEL_OF_THE_FATES_LOOP, true);
        self.emit_sound(sound::SITH_LIGHTSABRE_ON, false);
        self.trigger_leff(LeffKind::JediMultiballClash);
        self.start_overlay(super::OverlayKind::JediMultiballIntro);
        self.add_score(scoring::JEDI_MB_START);
        self.active_anim_hint = "anim_jedi_battle_intro_movie".to_string();
        self.emit_event(GameEvent::MultiballStarted {
            mode: "DARTH MAUL SABER DUEL".to_string(),
        });
        self.set_banner("DARTH MAUL SABER DUEL!", 4_000);
    }

    pub(super) fn on_major_shot_made(&mut self, sw_id: u8) {
        let is_ramp = matches!(sw_id, sw::RAMP_MADE_LEFT | sw::RAMP_MADE_RIGHT);
        if is_ramp {
            self.trigger_leff(LeffKind::RampMade {
                right: sw_id == sw::RAMP_MADE_RIGHT,
            });
        } else {
            self.hw.fire_coil(drive::UPPER_HOTDOG_FL, 24);
        }

        // 1. Check active Multiball Jackpot / Darth Maul Duel!
        match self.multiball {
            ActiveMultiball::HangarMultiball {
                jackpots_lit,
                jackpots_collected,
            } => {
                if jackpots_lit {
                    let next_jp = jackpots_collected.saturating_add(1);
                    self.multiball = ActiveMultiball::HangarMultiball {
                        jackpots_lit: true,
                        jackpots_collected: next_jp,
                    };
                    let award = scoring::MULTIBALL_JACKPOT * (next_jp as u64);
                    self.add_score(award);
                    self.emit_sound(sound::EXPLOSION, false);
                    self.emit_ard("ard_multiball_jackpot_speech");
                    self.trigger_leff(LeffKind::MainMultiballJackpot);
                    self.jackpot_fx(next_jp, false);
                    self.emit_event(GameEvent::JackpotAwarded {
                        super_jackpot: false,
                        points: award,
                    });
                    self.set_banner(format!("JACKPOT! +{award}"), 2_500);
                    return;
                }
            }
            ActiveMultiball::DarthMaulDuel { combo_window_ms, .. } => {
                let is_combo = combo_window_ms > 0;
                let dmg = if is_combo { 35 } else { 25 };
                let msg = if is_combo {
                    "LIGHTSABER COMBO!"
                } else {
                    "LIGHTSABER SLASH!"
                };
                self.damage_darth_maul(dmg, is_combo, msg);
                return;
            }
            ActiveMultiball::None => {}
        }

        // 2. Check active Scene shot or Scene Selection via Ramp!
        if self.active_scene.is_some() {
            let shot_label = match sw_id {
                sw::RAMP_MADE_LEFT => "LEFT RAMP HIT!",
                sw::RAMP_MADE_RIGHT => "RIGHT RAMP HIT!",
                _ => "FORCE LOOP HIT!",
            };
            self.advance_active_scene(shot_label);
            return;
        }
        if self.players[self.current_player].scene_select_ready {
            if sw_id == sw::RAMP_MADE_LEFT {
                self.cycle_scene_selection(true);
                return;
            } else if sw_id == sw::RAMP_MADE_RIGHT {
                let scene = self.players[self.current_player].choose_next_scene();
                self.start_swe1_scene(scene);
                return;
            }
        }

        // 2b. Ramp Combo progression (`Ramps.c`)
        if is_ramp {
            let combo = {
                let p = &mut self.players[self.current_player];
                p.ramp_combo_count = if p.ramp_combo_window_ms > 0 {
                    p.ramp_combo_count.saturating_add(1)
                } else {
                    1
                };
                p.ramp_combo_window_ms = 6_000;
                p.ramp_combo_best = p.ramp_combo_best.max(p.ramp_combo_count);
                p.ramp_combo_count
            };
            if combo >= 2 {
                let combo_pts = 250_000 * (combo as u64);
                self.add_score(combo_pts);
                self.emit_ard("ard_ramp_combo_naboo_flyby");
                self.spinner_combo_fx(sw_id == sw::RAMP_MADE_RIGHT);
            } else {
                self.emit_ard("ard_ramp_made");
            }
        }

        // 3. Award J-E-D-I Letter -> Completing J-E-D-I starts Darth Maul Lightsaber Duel!
        let (completed_jedi, letter_pts) = {
            let p = &mut self.players[self.current_player];
            let pts = 500_000 + 50_000 * (p.jedi_letters_total as u64);
            p.jedi_letters_total = p.jedi_letters_total.saturating_add(1);
            (p.award_jedi_letter(), pts)
        };
        self.add_score(letter_pts);
        if completed_jedi {
            self.add_score(scoring::JEDI_LETTERS_COMPLETE);
            self.emit_ard("ard_jedi_multiball_ready_tune");
            self.start_darth_maul_duel();
        } else {
            let mask = self.players[self.current_player].jedi_letters;
            let spelled: String = ['J', 'E', 'D', 'I']
                .iter()
                .enumerate()
                .map(|(i, &c)| if (mask & (1 << i)) != 0 { c } else { '-' })
                .collect();
            self.set_banner(format!("JEDI SPIRIT: [{spelled}]"), 2_000);
        }
    }

    pub(super) fn advance_active_scene(&mut self, label: &str) {
        let pts = match self.active_scene {
            Some(ref sc) if sc.scene == Swe1Scene::R2D2 => (sc.timer_ms as u64 * 30).max(250_000),
            Some(ref sc) => sc.scene.hit_score(),
            None => 250_000,
        };
        self.add_score(pts);
        let mut finished_scene = None;
        let mut progressed_info = None;
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = sc.hits_remaining.saturating_sub(1);
            if sc.hits_remaining == 0 {
                finished_scene = Some(sc.scene);
            } else {
                progressed_info = Some((sc.scene, sc.hits_remaining));
                let left = sc.hits_remaining;
                self.set_banner(format!("{label} ({left} HITS LEFT)"), 2_000);
            }
        }
        if let Some((scene, hits_remaining)) = progressed_info {
            self.emit_ard(scene.hit_ard());
            self.trigger_leff(LeffKind::SceneHit);
            self.scene_hit_fx(scene, hits_remaining, label.contains("LASER"));
            self.emit_event(GameEvent::SceneProgressed {
                scene,
                hits_remaining,
                points: pts,
            });
        }
        if let Some(scene) = finished_scene {
            self.active_scene = None;
            let idx = Swe1Scene::ALL.iter().position(|&s| s == scene).unwrap_or(0);
            self.players[self.current_player].scenes_completed |= 1 << idx;
            let done_count = self.players[self.current_player].scenes_completed.count_ones();
            if done_count == 4 {
                self.players[self.current_player].extra_balls += 1;
            }
            let comp_pts = scene.completion_score();
            self.add_score(comp_pts);
            self.audits.scenes_completed = self.audits.scenes_completed.saturating_add(1);
            self.emit_coil(drive::LIGHTSABER_NEON, 160);
            self.emit_sound(sound::MAIN_THEME_LOOP, true);
            self.emit_ard(scene.complete_ard());
            self.trigger_leff(LeffKind::SceneComplete);
            self.scene_complete_fx(scene);
            self.emit_event(GameEvent::SceneCompleted {
                scene,
                points: comp_pts,
            });
            if scene == Swe1Scene::Podrace {
                self.active_anim_hint = "anim_podrace_closeup".to_string();
            } else {
                self.active_anim_hint.clear();
            }
            self.set_banner(format!("{} VICTORY! +{comp_pts}", scene.title()), 3_500);
        }
    }

    pub(super) fn damage_darth_maul(&mut self, dmg: u8, combo: bool, label: &str) {
        self.audio.play_sound(sound::SABER_CLASH, false);
        self.emit_ard("ard_jedi_battle_multiball_saber_hit");
        self.trigger_leff(LeffKind::JediMultiballClash);
        self.add_score(if combo {
            scoring::JEDI_MB_ATTACK_MAUL_COMBO
        } else {
            scoring::JEDI_MB_ATTACK_MAUL
        });

        if let ActiveMultiball::DarthMaulDuel { maul_health, .. } = self.multiball {
            let next_hp = maul_health.saturating_sub(dmg);
            if next_hp == 0 {
                self.multiball = ActiveMultiball::None;
                self.hw.set_drive_hold(drive::LIGHTSABER_NEON, false);
                let new_rank = self.players[self.current_player].jedi_rank.advance();
                self.players[self.current_player].jedi_rank = new_rank;
                self.audits.maul_duels_won = self.audits.maul_duels_won.saturating_add(1);
                self.add_score(5_000_000);
                self.emit_sound(sound::MAIN_THEME_LOOP, true);
                self.emit_sound(sound::DARTH_MAUL_DEATH_YELL, false);
                self.emit_ard("ard_jedi_battle_multiball_ending_speech");
                self.trigger_leff(LeffKind::JediMultiballFinish);
                self.duel_won_fx(combo);
                self.rank_promotion_overlay(new_rank);
                self.emit_event(GameEvent::JediRankPromoted { rank: new_rank });
                self.active_anim_hint = "anim_jedi_battle_maul_death_1".to_string();
                self.set_banner(
                    format!("MAUL DEFEATED! PROMOTED TO {}! +5,000,000", new_rank.label()),
                    4_500,
                );
            } else {
                self.multiball = ActiveMultiball::DarthMaulDuel {
                    maul_health: next_hp,
                    combo_window_ms: 4_000,
                };
                self.active_anim_hint = match (next_hp / 20) % 3 {
                    0 => "anim_jedi_battle_clash".to_string(),
                    1 => "anim_jedi_battle_quigon_high_right_single".to_string(),
                    _ => "anim_jedi_battle_maul_chop_single".to_string(),
                };
                self.set_banner(format!("{label} MAUL HP: {next_hp}%"), 2_000);
            }
        }
    }

}
