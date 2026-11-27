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

    pub(super) fn sync_shield_position(&mut self) {
        if self.state != MachineState::InGame || !self.playfield_valid {
            self.set_shield_raised(false);
            return;
        }
        let p = &self.players[self.current_player];
        let rematch_ready = matches!(
            self.multiball,
            ActiveMultiball::HangarMultiball {
                jackpots_lit: true,
                jackpots_collected
            } if jackpots_collected >= 2
        );
        let should_raise = p.jedi_letters == 0x0F || p.scene_select_ready || rematch_ready;
        self.set_shield_raised(should_raise);
    }

    fn activate_playfield_valid(&mut self) {
        self.playfield_valid = true;
        self.first_ball_serve = false;
        self.skill_shot_grace_ms = 160;
        self.sync_shield_position();
        if self.active_scene.is_none() && matches!(self.multiball, ActiveMultiball::None) {
            self.emit_sound(sound::MAINPLAY_LOOP, true);
        }
    }

    pub(super) fn record_playfield_switch(&mut self, id: u8) {
        if self.playfield_valid {
            return;
        }
        let immediate = matches!(
            id,
            sw::RIGHT_INLANE
                | sw::SNEAKY_LANE
                | sw::LEFT_SAUCER
                | sw::RIGHT_SAUCER
                | sw::SHIELD_POPPER
        );
        if immediate {
            self.activate_playfield_valid();
            return;
        }
        let counted = matches!(
            id,
            sw::LEFT_OUTLANE
                | sw::RIGHT_OUTLANE
                | sw::LEFT_INLANE
                | sw::CAPTIVE_BALL
                | sw::LEFT_BANK_UPPER
                | sw::LEFT_BANK_MIDDLE
                | sw::LEFT_BANK_LOWER
                | sw::RIGHT_BANK_UPPER
                | sw::RIGHT_BANK_MIDDLE
                | sw::RIGHT_BANK_LOWER
                | sw::LEFT_RAMP_ENTER
                | sw::RIGHT_RAMP_ENTER
                | sw::RAMP_MADE_LEFT
                | sw::RAMP_MADE_RIGHT
                | sw::LEFT_SHIELD_TARGET
                | sw::RIGHT_SHIELD_TARGET
                | sw::SHIELD_HIT
                | sw::LEFT_SLING
                | sw::RIGHT_SLING
                | sw::UPPER_JET
                | sw::MIDDLE_JET
                | sw::LOWER_JET
                | sw::JETS_ROLLOVER
                | sw::LEFT_LOOP_UPPER
                | sw::LEFT_LOOP_ROLLOVER
        );
        if counted && id < 64 {
            self.valid_playfield_switches |= 1u64 << id;
            if self.valid_playfield_switches.count_ones() >= 3 {
                self.activate_playfield_valid();
            }
        }
    }

    pub(super) fn handle_ingame_switch_closed(&mut self, id: u8) {
        self.idle_ball_search_ms = 0;
        self.record_playfield_switch(id);

        match id {
            sw::START_BUTTON => {
                self.try_start_or_add_player();
            }
            sw::PLUMB_TILT => {
                self.tilt_warnings += 1;
                if self.tilt_warnings >= self.adjustments.tilt_warnings_max {
                    self.cut_all_holds();
                    self.set_shield_raised(false);
                    self.active_scene = None;
                    self.state = MachineState::Tilt;
                    self.emit_ard("ard_tilt_random_speech");
                    self.emit_event(GameEvent::Tilted);
                    self.eval_mod_script_hook("on_tilt", ());
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
                self.handle_podrace_button(false);
            }
            sw::RIGHT_FLIPPER_BUTTON => {
                self.hw.fire_coil(drive::RIGHT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(drive::RIGHT_FLIPPER_HOLD, true);
                self.players[self.current_player].rotate_jedi_letters(true);
                self.handle_podrace_button(true);
            }
            // Cabinet Laser Action Buttons (`Lasers::fire_left` / `Lasers::fire_right`)
            sw::LEFT_ACTION_BUTTON | sw::RIGHT_ACTION_BUTTON => {
                let right = id == sw::RIGHT_ACTION_BUTTON;
                if self.players[self.current_player].scene_select_ready && self.active_scene.is_none() {
                    self.cycle_scene_selection(right);
                    return;
                }
                self.handle_podrace_button(right);
                if self.players[self.current_player].laser_shots > 0 {
                    self.players[self.current_player].laser_shots -= 1;
                    self.audio.play_sound(sound::LASER_FIRE, false);
                    self.trigger_leff(LeffKind::LaserFired { right });
                    if self.handle_battle_droid_laser(right)
                        || self.handle_mos_espa_cantina_laser()
                        || self.handle_queen_fashion_laser()
                    {
                        return;
                    }
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
                self.add_score(scoring::SWITCH_BASE);
            }
            sw::RIGHT_SLING => {
                self.hw.fire_coil(drive::RIGHT_SLING, 48);
                self.hw.fire_coil(drive::LOWER_RIGHT_HOTDOG_FL, 24);
                self.add_score(scoring::SWITCH_BASE);
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
                let gungan_blocked = self.active_scene.is_some()
                    || matches!(self.multiball, ActiveMultiball::DarthMaulDuel { .. })
                    || self.players[self.current_player].jedi_letters == 0x0F;
                let (pts, started_super_jets, gungan_step) = {
                    let p = &mut self.players[self.current_player];
                    p.jet_hits += 1;
                    if p.super_jets_ms > 0 {
                        (scoring::SWITCH_BASE + scoring::JET_SUPER, false, 6u8)
                    } else if gungan_blocked {
                        (scoring::SWITCH_BASE + scoring::JET_NORMAL, false, p.gungan_letters)
                    } else {
                        p.gungan_letters += 1;
                        if p.gungan_letters >= 6 {
                            p.gungan_letters = 0;
                            p.super_jets_ms = 20_000;
                            (scoring::SWITCH_BASE + scoring::JET_NORMAL, true, 6u8)
                        } else {
                            (scoring::SWITCH_BASE + scoring::JET_NORMAL, false, p.gungan_letters)
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
                    self.set_banner(
                        format!("SUPER JETS! +{}", scoring::SWITCH_BASE + scoring::JET_SUPER),
                        1_500,
                    );
                }
            }
            sw::JETS_ROLLOVER => {
                self.add_score(scoring::SWITCH_BASE);
            }
            // Standup Banks charge Lasers, light Naboo Ship Bonus & Hotdogs (`Bonus X + 1`)
            sw::LEFT_BANK_UPPER => self.handle_standup_target(false, 0),
            sw::LEFT_BANK_MIDDLE => self.handle_standup_target(false, 1),
            sw::LEFT_BANK_LOWER => self.handle_standup_target(false, 2),
            sw::RIGHT_BANK_UPPER => self.handle_standup_target(true, 0),
            sw::RIGHT_BANK_MIDDLE => self.handle_standup_target(true, 1),
            sw::RIGHT_BANK_LOWER => self.handle_standup_target(true, 2),
            // Drop Targets (`Drop::hook_drop_target_hit`) & Saucers
            sw::LEFT_DROP_TARGET => {
                self.left_drop_down = true;
                self.trigger_leff(LeffKind::ShieldHit);
                if self.handle_battle_droid_drop_target(false) {
                    return;
                }
                self.add_score(scoring::DROP_TARGET);
                self.emit_ard("ard_hit_bdroids_speech");
                self.set_banner("LEFT SAUCER OPEN!", 1_800);
            }
            sw::RIGHT_DROP_TARGET => {
                self.right_drop_down = true;
                self.trigger_leff(LeffKind::ShieldHit);
                if self.handle_battle_droid_drop_target(true) {
                    return;
                }
                self.add_score(scoring::DROP_TARGET);
                self.emit_ard("ard_hit_bdroids_speech");
                self.set_banner("RIGHT SAUCER OPEN!", 1_800);
            }
            sw::LEFT_SAUCER | sw::RIGHT_SAUCER => {
                let idx = if id == sw::LEFT_SAUCER { 0 } else { 1 };
                self.saucer_hold_ms[idx] = 800;
                self.add_score(scoring::SAUCER_ARRIVAL);
                self.on_saucer_entered();
            }
            // Deflector Shield Targets & Center Shield Popper (`Scenes::start_valid_scene`)
            sw::LEFT_SHIELD_TARGET | sw::RIGHT_SHIELD_TARGET | sw::SHIELD_HIT => {
                if self.handle_battle_droid_switch(id)
                    || self.handle_destroyer_droid_switch(id)
                    || self.handle_federation_fighter_switch(id)
                    || self.handle_gungan_battle_switch(id)
                    || self.handle_hangar_escape_switch(id)
                    || self.handle_hover_tank_switch(id)
                    || self.handle_probe_droid_switch(id)
                    || self.handle_queen_fashion_switch(id)
                    || self.handle_wattos_chance_switch(id)
                    || self.handle_mos_espa_cantina_switch(id)
                    || self.handle_r2d2_switch(id)
                    || self.handle_jarjar_juggle_switch(id)
                    || self.handle_podrace_switch(id)
                {
                    return;
                }
                self.audio.play_sound(sound::SHIELD_HIT_SFX, false);
                self.trigger_leff(LeffKind::ShieldHit);
                if self.active_scene.is_some() {
                    self.advance_active_scene("SHIELD BLAST!");
                } else if let ActiveMultiball::DarthMaulDuel { .. } = self.multiball {
                    self.damage_darth_maul(1, false, "SABER STRIKE!");
                } else if !matches!(self.multiball, ActiveMultiball::None)
                    || self.players[self.current_player].jedi_letters == 0x0F
                {
                    self.add_score(scoring::SWITCH_BASE);
                } else {
                    let rem = {
                        let p = &mut self.players[self.current_player];
                        p.shield_health = p.shield_health.saturating_sub(2);
                        p.shield_health
                    };
                    self.add_score(scoring::SWITCH_BASE);
                    if rem == 0 {
                        let (was_ready, selected) = {
                            let p = &mut self.players[self.current_player];
                            let was = p.scene_select_ready;
                            p.scene_select_ready = true;
                            (was, Swe1Scene::ALL[p.next_scene_idx % Swe1Scene::ALL.len()])
                        };
                        if !was_ready {
                            self.holo_transition_ms = 0;
                            self.emit_ard("ard_obi_holo_watch_descends");
                        }
                        self.sync_shield_position();
                        self.emit_sound(sound::SHIELD_GONE_CALLOUT, false);
                        self.active_anim_hint = selected.holo_anim_hint().to_string();
                        self.set_banner(
                            format!("SHIELD DOWN! SELECT: {} (LASER BUTTONS) -> SHOOT SCOOP!", selected.title()),
                            4_000,
                        );
                    } else {
                        self.shield_hit_glow_ms = 350;
                        self.active_anim_hint = "anim_shield".to_string();
                        self.set_banner(format!("DEFLECTOR SHIELD: {}%", (rem as u32) * 100 / 12), 1_500);
                    }
                }
            }
            sw::SHIELD_POPPER => {
                self.popper_hold_ms = 900;
                self.add_score(scoring::SAUCER_ARRIVAL);
                if self.handle_federation_fighter_switch(id)
                    || self.handle_hangar_escape_switch(id)
                    || self.handle_mos_espa_cantina_switch(id)
                {
                    return;
                }
                self.on_shield_popper_entered();
            }
            // Left & Right Ramps + Left Loop -> Virtual Spinners + Combos + C-3PO + JEDI + Scene/Multiball
            sw::LEFT_RAMP_ENTER => {
                self.trigger_virtual_spinner(false, true);
                if self.handle_destroyer_droid_switch(id)
                    || self.handle_federation_fighter_switch(id)
                    || self.handle_hangar_escape_switch(id)
                    || self.handle_hover_tank_switch(id)
                    || self.handle_probe_droid_switch(id)
                    || self.handle_queen_fashion_switch(id)
                    || self.handle_wattos_chance_switch(id)
                    || self.handle_jarjar_juggle_switch(id)
                    || self.handle_podrace_switch(id)
                {
                    return;
                }
                self.add_score(scoring::SWITCH_BASE);
            }
            sw::RIGHT_RAMP_ENTER => {
                self.trigger_virtual_spinner(true, true);
                self.jarjar_saucer_event(true, 0);
                if self.handle_destroyer_droid_switch(id)
                    || self.handle_federation_fighter_switch(id)
                    || self.handle_hangar_escape_switch(id)
                    || self.handle_hover_tank_switch(id)
                    || self.handle_probe_droid_switch(id)
                    || self.handle_queen_fashion_switch(id)
                    || self.handle_wattos_chance_switch(id)
                    || self.handle_jarjar_juggle_switch(id)
                    || self.handle_podrace_switch(id)
                {
                    return;
                }
                self.add_score(scoring::SWITCH_BASE);
            }
            sw::RAMP_MADE_LEFT | sw::RAMP_MADE_RIGHT | sw::LEFT_LOOP_UPPER | sw::LEFT_LOOP_ROLLOVER => {
                self.add_score(scoring::SWITCH_BASE);
                if self.handle_battle_droid_switch(id)
                    || self.handle_gungan_battle_switch(id)
                    || self.handle_mos_espa_cantina_switch(id)
                    || self.handle_r2d2_switch(id)
                {
                    return;
                }
                self.on_major_shot_made(id);
            }
            // Inlanes, Sneaky Lane Skill Shot & Captive Ball
            sw::SNEAKY_LANE => {
                self.add_score(scoring::SWITCH_BASE);
                if self.skill_shot_ready || self.skill_shot_grace_ms > 0 {
                    self.award_skill_shot();
                } else {
                    self.hw.fire_coil(drive::RIGHT_INLANE_FL, 24);
                }
            }
            sw::LEFT_INLANE | sw::RIGHT_INLANE => {
                let fl = if id == sw::LEFT_INLANE {
                    drive::LEFT_INLANE_FL
                } else {
                    drive::RIGHT_INLANE_FL
                };
                self.hw.fire_coil(fl, 24);
                self.add_score(scoring::SWITCH_BASE);
            }
            sw::CAPTIVE_BALL => {
                self.audio.play_sound(sound::EXPLOSION, false);
                let pts = {
                    let p = &mut self.players[self.current_player];
                    let val = scoring::captive_ball_score(p.captive_ball_hits);
                    p.captive_ball_hits = p.captive_ball_hits.saturating_add(1);
                    val
                };
                self.add_score(pts);
                self.set_banner(format!("CAPTIVE BALL! +{pts}"), 1_500);
            }
            sw::LEFT_OUTLANE | sw::RIGHT_OUTLANE => {
                self.add_score(scoring::SWITCH_BASE);
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

    fn light_naboo_ship_lamp(&mut self, count: u8) {
        let p = &mut self.players[self.current_player];
        p.naboo_ship_lamps = (p.naboo_ship_lamps + count).min(10);
        p.bonus_points = (p.naboo_ship_lamps as u64) * scoring::BONUS_PER_SHIP;
    }

    fn handle_standup_target(&mut self, is_right: bool, bit_idx: u8) {
        let bit = 1u8 << bit_idx;
        let (was_lit, completed_bank, hotdog_bonus_x) = {
            let p = &mut self.players[self.current_player];
            let mask = if is_right {
                &mut p.right_bank_mask
            } else {
                &mut p.left_bank_mask
            };
            let was_lit = (*mask & bit) != 0;
            *mask |= bit;
            let completed = *mask == 0b111;
            let mut hotdog_bonus_x = false;
            if completed {
                *mask = 0;
                p.laser_shots = (p.laser_shots + 1).min(scoring::MAX_LASER_SHOTS);
                if is_right {
                    p.right_hotdog_lit = true;
                } else {
                    p.left_hotdog_lit = true;
                }
                if p.left_hotdog_lit && p.right_hotdog_lit {
                    p.left_hotdog_lit = false;
                    p.right_hotdog_lit = false;
                    if p.bonus_multiplier < scoring::BONUS_MAX_MULTIPLIER {
                        p.bonus_multiplier += 1;
                        hotdog_bonus_x = true;
                    }
                }
            }
            (was_lit, completed, hotdog_bonus_x)
        };
        if completed_bank {
            self.light_naboo_ship_lamp(1);
            self.add_score(
                scoring::SWITCH_BASE + scoring::STANDUP_BANK_COMPLETE + scoring::STANDUP_LASER_AWARD,
            );
            let ls = self.players[self.current_player].laser_shots;
            if ls == 1 {
                self.laser_instruction_fx();
            }
            if hotdog_bonus_x {
                let bm = self.players[self.current_player].bonus_multiplier;
                self.set_banner(format!("BANK + HOTDOGS! LASER {ls} & BONUS {bm}X!"), 2_200);
            } else {
                self.set_banner(format!("BANK COMPLETE! LASERS: {ls}"), 1_800);
            }
        } else if !was_lit {
            self.light_naboo_ship_lamp(1);
            self.add_score(scoring::SWITCH_BASE + scoring::STANDUP_NEW);
        } else {
            self.add_score(scoring::SWITCH_BASE + scoring::STANDUP_LIT);
        }
    }

    pub(super) fn award_jedi_letter_with_score(&mut self) {
        let (completed_jedi, letter_pts) = {
            let p = &mut self.players[self.current_player];
            let pts = scoring::JEDI_LETTER_BASE
                + scoring::JEDI_LETTER_STEP * ((p.jedi_letters_total % 3) as u64);
            p.jedi_letters_total = p.jedi_letters_total.saturating_add(1);
            (p.award_jedi_letter(), pts)
        };
        if completed_jedi {
            self.add_score(scoring::JEDI_LETTERS_COMPLETE);
            self.emit_ard("ard_jedi_multiball_ready_tune");
            if self.active_scene.is_none() && matches!(self.multiball, ActiveMultiball::None) {
                self.start_darth_maul_duel();
            } else {
                self.sync_shield_position();
                self.set_banner("JEDI MULTIBALL READY! SHOOT SHIELD POPPER!", 3_000);
            }
        } else {
            self.add_score(letter_pts);
            let mask = self.players[self.current_player].jedi_letters;
            let spelled: String = ['J', 'E', 'D', 'I']
                .iter()
                .enumerate()
                .map(|(i, &c)| if (mask & (1 << i)) != 0 { c } else { '-' })
                .collect();
            self.set_banner(format!("JEDI SPIRIT: [{spelled}]"), 2_000);
        }
    }

    fn award_skill_shot(&mut self) {
        self.skill_shot_ready = false;
        self.skill_shot_grace_ms = 0;
        let (pts, award_jedi) = {
            let p = &mut self.players[self.current_player];
            p.skill_shots_made = p.skill_shots_made.saturating_add(1);
            let pts = match p.skill_shots_made {
                1 => scoring::SKILL_SHOT_FIRST,
                2 => scoring::SKILL_SHOT_SECOND,
                _ => scoring::SKILL_SHOT_CAP,
            };
            let award_jedi = p.skill_shots_made >= 3 && !p.skill_shot_jedi_awarded;
            if award_jedi {
                p.skill_shot_jedi_awarded = true;
            }
            (pts, award_jedi)
        };
        self.add_score(pts);
        self.emit_ard("ard_skill_shot_speech");
        self.emit_ard("ard_skill_shot_award");
        self.trigger_leff(LeffKind::SkillShotAward);
        self.emit_event(GameEvent::SkillShotAwarded { points: pts });
        self.set_banner(format!("SKILL SHOT! +{}", pts), 2_500);
        if award_jedi {
            self.award_jedi_letter_with_score();
        }
    }

    fn advance_c3p0(&mut self) {
        if matches!(self.multiball, ActiveMultiball::DarthMaulDuel { .. }) {
            self.add_score(100_000);
            return;
        }
        let (part, completed_reward_idx) = {
            let p = &mut self.players[self.current_player];
            p.c3p0_parts = p.c3p0_parts.saturating_add(1);
            let part = p.c3p0_parts;
            if part >= 8 {
                p.c3p0_parts = 0;
                let r_idx = p.c3p0_reward_idx % 11;
                p.c3p0_reward_idx = (r_idx + 1) % 11;
                (part, Some(r_idx))
            } else {
                (part, None)
            }
        };
        self.add_score((part as u64) * 100_000);
        if let Some(r_idx) = completed_reward_idx {
            self.trigger_leff(LeffKind::C3P0Complete);
            match r_idx {
                0 | 5 => {
                    self.players[self.current_player].extra_balls =
                        self.players[self.current_player].extra_balls.saturating_add(1);
                    self.emit_ard("ard_extra_ball_lit_speech");
                    self.set_banner("C-3PO COMPLETE! EXTRA BALL!", 3_500);
                }
                1 | 7 => {
                    self.award_jedi_letter_with_score();
                }
                2 => {
                    self.add_score(2_000_000);
                    self.set_banner("C-3PO COMPLETE! +2,000,000!", 3_500);
                }
                3 => {
                    let p = &mut self.players[self.current_player];
                    if p.hold_bonus {
                        self.add_score(scoring::C3P0_FALLBACK_AWARD);
                        self.set_banner("C-3PO COMPLETE! +1,000,000!", 3_500);
                    } else {
                        p.hold_bonus = true;
                        self.set_banner("C-3PO COMPLETE! HOLD BONUS!", 3_500);
                    }
                }
                4 => {
                    self.add_score(1_000_000);
                    self.set_banner("C-3PO COMPLETE! +1,000,000!", 3_500);
                }
                6 => {
                    let p = &mut self.players[self.current_player];
                    if p.bonus_multiplier < scoring::BONUS_MAX_MULTIPLIER {
                        p.bonus_multiplier += 1;
                        let bm = p.bonus_multiplier;
                        self.set_banner(format!("C-3PO COMPLETE! BONUS {bm}X!"), 3_500);
                    } else {
                        self.add_score(scoring::C3P0_FALLBACK_AWARD);
                        self.set_banner("C-3PO COMPLETE! +1,000,000!", 3_500);
                    }
                }
                8 => {
                    self.add_score(500_000);
                    self.set_banner("C-3PO COMPLETE! +500,000!", 3_500);
                }
                9 => {
                    let p = &mut self.players[self.current_player];
                    p.laser_shots = (p.laser_shots + 20).min(scoring::MAX_LASER_SHOTS);
                    self.add_score(1_000_000);
                    self.set_banner("C-3PO COMPLETE! 20 LASERS +1,000,000!", 3_500);
                }
                _ => {
                    self.add_score(10);
                    self.set_banner("C-3PO COMPLETE! +10!", 3_500);
                }
            }
        } else {
            self.set_banner(format!("BUILD C-3PO: PART {part}/8"), 1_800);
        }
    }

    pub(super) fn on_saucer_entered(&mut self) {
        if !matches!(self.multiball, ActiveMultiball::None) {
            self.add_score(300_000);
            self.set_banner("SAUCER BONUS! +300,000", 2_000);
            return;
        }
        if self.players[self.current_player].jedi_letters == 0x0F {
            // Locks are excluded when all 4 JEDI letters are collected / Jedi MB is ready
            self.add_score(150_000);
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

        if self.players[self.current_player].jedi_letters == 0x0F
            && matches!(self.multiball, ActiveMultiball::None)
        {
            self.start_darth_maul_duel();
            return;
        }

        if !matches!(self.multiball, ActiveMultiball::None) {
            return;
        }

        // Start the next SWE1 Holoprojector Scene (with intro movie before ball eject!)
        let scene = self.players[self.current_player].choose_next_scene();
        self.start_swe1_scene(scene);
    }

    pub fn cycle_scene_selection(&mut self, forward: bool) {
        let scene = self.players[self.current_player].cycle_eligible_scene(forward);
        self.active_anim_hint = scene.holo_anim_hint().to_string();
        self.emit_sound(sound::LASER_SFX_OR_CLICK(), false);
        self.set_banner(format!("SELECT MODE: {} (SHOOT SCOOP TO START)", scene.title()), 3_000);
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
        // Ball stays held in the behind-shield popper during intro + 1s after scene gameplay starts
        self.popper_hold_ms = intro_ms + 1_000;
        self.special_screen = None;
        self.special_screen_ms = 0;
        self.active_scene = Some(ActiveSceneState {
            scene,
            hits_remaining: scene.required_hits(),
            timer_ms: scene.duration_ms(),
            intro_remaining_ms: intro_ms,
            active_ms: 0,
        });
        if scene == Swe1Scene::BattleDroid {
            self.start_battle_droid_state();
        } else {
            self.battle_droid = None;
        }
        if scene == Swe1Scene::DestroyerDroid {
            self.start_destroyer_droid_state();
        } else {
            self.destroyer_droid = None;
        }
        if scene == Swe1Scene::FederationFighter {
            self.start_federation_fighter_state();
        } else {
            self.federation_fighter = None;
        }
        if scene == Swe1Scene::GunganBattle {
            self.start_gungan_battle_state();
        } else {
            self.gungan_battle = None;
        }
        if scene == Swe1Scene::HangarEscape {
            self.start_hangar_escape_state();
        } else {
            self.hangar_escape = None;
        }
        if scene == Swe1Scene::HoverTank {
            self.start_hover_tank_state();
        } else {
            self.hover_tank = None;
        }
        if scene == Swe1Scene::ProbeDroid {
            self.start_probe_droid_state();
        } else {
            self.probe_droid = None;
        }
        if scene == Swe1Scene::QueenFashion {
            self.start_queen_fashion_state();
        } else {
            self.queen_fashion = None;
        }
        if scene == Swe1Scene::WattosChance {
            self.start_wattos_chance_state();
        } else {
            self.wattos_chance = None;
        }
        if scene == Swe1Scene::MosEspaCantina {
            self.start_mos_espa_cantina_state();
        } else {
            self.mos_espa_cantina = None;
        }
        if scene == Swe1Scene::R2D2 {
            self.start_r2d2_state();
        } else {
            self.r2d2 = None;
        }
        if scene == Swe1Scene::QuestionMark {
            self.start_question_mark_state();
        } else {
            self.question_mark = None;
        }
        if scene == Swe1Scene::JarJarJuggle {
            self.start_jarjar_juggle_state();
        } else {
            self.jarjar_juggle = None;
        }
        if scene == Swe1Scene::Podrace {
            self.start_podrace_state();
        } else {
            self.podrace = None;
        }
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
        self.players[self.current_player].jedi_battles_started =
            self.players[self.current_player].jedi_battles_started.saturating_add(1);
        self.right_saber_color = 1; // Red Sith lightsaber
        self.hw.set_drive_hold(drive::LIGHTSABER_NEON, true);
        self.multiball = ActiveMultiball::DarthMaulDuel {
            maul_health: 15,
            quigon_health: 15,
            combo_window_ms: 5_000,
            maul_attack_ms: 5_500,
        };
        self.pending_eject_balls = self.pending_eject_balls.saturating_add(2);
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
                let is_combo = is_ramp && combo_window_ms > 0;
                let msg = if is_combo {
                    "LIGHTSABER COMBO!"
                } else {
                    "LIGHTSABER SLASH!"
                };
                self.damage_darth_maul(2, is_combo, msg);
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
                self.cycle_scene_selection(false);
                return;
            } else if sw_id == sw::RAMP_MADE_RIGHT {
                self.cycle_scene_selection(true);
                return;
            }
        }

        // 2b. Ramp Combo progression (`Ramps.c`: 6.72s window, n * 100,000 for n >= 2)
        if is_ramp {
            let combo = {
                let p = &mut self.players[self.current_player];
                p.ramp_combo_count = if p.ramp_combo_window_ms > 0 {
                    p.ramp_combo_count.saturating_add(1)
                } else {
                    1
                };
                p.ramp_combo_window_ms = scoring::RAMP_COMBO_WINDOW_MS;
                p.ramp_combo_best = p.ramp_combo_best.max(p.ramp_combo_count);
                p.ramp_combo_count
            };
            if combo >= 2 {
                let combo_pts = scoring::RAMP_COMBO_STEP * (combo as u64);
                self.add_score(combo_pts);
                self.emit_ard("ard_ramp_combo_naboo_flyby");
                self.spinner_combo_fx(sw_id == sw::RAMP_MADE_RIGHT);
            } else {
                self.emit_ard("ard_ramp_made");
            }
        }

        // 3. Left Loop builds C-3PO; Ramps award J-E-D-I Letters!
        if !is_ramp {
            self.advance_c3p0();
            return;
        }
        self.award_jedi_letter_with_score();
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
            self.complete_active_scene(scene);
        }
    }

    pub(crate) fn complete_active_scene(&mut self, scene: Swe1Scene) {
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
        self.emit_sound(sound::MAINPLAY_LOOP, true);
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

    pub(super) fn damage_darth_maul(&mut self, dmg: u8, combo: bool, label: &str) {
        self.emit_sound(sound::SABER_CLASH, false);
        self.emit_ard("ard_jedi_battle_multiball_saber_hit");
        self.trigger_leff(LeffKind::JediMultiballClash);
        self.add_score(if combo {
            scoring::JEDI_MB_ATTACK_MAUL_COMBO
        } else {
            scoring::JEDI_MB_ATTACK_MAUL
        });

        if let ActiveMultiball::DarthMaulDuel {
            maul_health,
            quigon_health,
            maul_attack_ms,
            ..
        } = self.multiball
        {
            let next_hp = maul_health.saturating_sub(dmg);
            if next_hp == 0 {
                self.multiball = ActiveMultiball::None;
                self.hw.set_drive_hold(drive::LIGHTSABER_NEON, false);
                let new_rank = self.players[self.current_player].jedi_rank.advance();
                self.players[self.current_player].jedi_rank = new_rank;
                self.audits.maul_duels_won = self.audits.maul_duels_won.saturating_add(1);
                self.add_score(5_000_000);
                self.emit_sound(sound::MAINPLAY_LOOP, true);
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
                    quigon_health,
                    combo_window_ms: 4_000,
                    maul_attack_ms,
                };
                self.active_anim_hint = match next_hp % 3 {
                    0 => "anim_jedi_battle_clash".to_string(),
                    1 => "anim_jedi_battle_quigon_high_right_single".to_string(),
                    _ => "anim_jedi_battle_maul_chop_single".to_string(),
                };
                self.set_banner(format!("{label} MAUL HP: {next_hp}/15"), 2_000);
            }
        }
    }

}
