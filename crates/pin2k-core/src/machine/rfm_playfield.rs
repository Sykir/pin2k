use pin2k_hal::{AudioBackend, LampState, PinballHardware, VideoBackend};

use super::{MachineState, Pin2kMachine};
use crate::events::GameEvent;
use crate::rfm_rules::{RfmMultiball, RfmWave};
use crate::rfm_tables::{rfm_drive, rfm_lamp, rfm_sound, rfm_sw};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(super) fn handle_rfm_ingame_switch_closed(&mut self, id: u8) {
        self.idle_ball_search_ms = 0;

        match id {
            rfm_sw::START_BUTTON => self.try_start_or_add_player(),
            rfm_sw::LAUNCH_BUTTON => {
                self.emit_coil(rfm_drive::AUTOPLUNGER, 48);
                self.emit_sound(rfm_sound::LASER_SFX, false);
                self.set_banner("AUTOPLUNGER LAUNCH!", 1_500);
            }
            sw::PLUMB_TILT => {
                self.tilt_warnings += 1;
                if self.tilt_warnings >= self.adjustments.tilt_warnings_max {
                    self.cut_all_holds();
                    self.state = MachineState::Tilt;
                    self.emit_event(GameEvent::Tilted);
                    self.set_banner("TILT! BONUS LOST", 4_000);
                } else {
                    self.emit_event(GameEvent::TiltWarning {
                        warnings: self.tilt_warnings,
                    });
                    self.set_banner(format!("DANGER! TILT WARNING {}/2", self.tilt_warnings), 2_000);
                }
            }
            sw::LEFT_FLIPPER_BUTTON => {
                self.emit_coil(rfm_drive::LEFT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(rfm_drive::LEFT_FLIPPER_HOLD, true);
            }
            sw::RIGHT_FLIPPER_BUTTON => {
                self.emit_coil(rfm_drive::RIGHT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(rfm_drive::RIGHT_FLIPPER_HOLD, true);
            }
            sw::LEFT_ACTION_BUTTON | sw::RIGHT_ACTION_BUTTON => {
                self.emit_coil(rfm_drive::CENTER_ARROW_FL, 32);
                self.emit_sound(rfm_sound::LASER_SFX, false);
                self.add_score(100_000);
                if self.rfm.active_wave.is_some() {
                    self.advance_rfm_wave("RAYGUN HIT!", 250_000);
                } else {
                    self.set_banner("MARTIAN RAYGUN FIRED!", 1_500);
                }
            }
            rfm_sw::LEFT_SLING => {
                self.emit_coil(rfm_drive::LEFT_SLING, 48);
                self.add_score(5_110);
            }
            rfm_sw::RIGHT_SLING => {
                self.emit_coil(rfm_drive::RIGHT_SLING, 48);
                self.add_score(5_110);
            }
            rfm_sw::LEFT_JET | rfm_sw::RIGHT_JET | rfm_sw::BOTTOM_JET => {
                let c = match id {
                    rfm_sw::LEFT_JET => rfm_drive::LEFT_JET,
                    rfm_sw::RIGHT_JET => rfm_drive::RIGHT_JET,
                    _ => rfm_drive::BOTTOM_JET,
                };
                self.emit_coil(c, 40);
                self.add_score(25_000);
            }
            rfm_sw::LEFT_TOP_LANE | rfm_sw::RIGHT_TOP_LANE => {
                if self.skill_shot_ready {
                    self.skill_shot_ready = false;
                    self.add_score(500_000);
                    self.emit_event(GameEvent::SkillShotAwarded { points: 500_000 });
                    self.set_banner("RFM SKILL SHOT! +500,000", 2_500);
                } else {
                    self.add_score(50_000);
                }
            }
            // 7 Martian Targets -> pulse Left/Right Martian toys & start Martian Attack Multiball!
            rfm_sw::MARTIAN_TARGET_1
            | rfm_sw::MARTIAN_TARGET_2
            | rfm_sw::MARTIAN_TARGET_3
            | rfm_sw::MARTIAN_TARGET_4
            | rfm_sw::MARTIAN_TARGET_5
            | rfm_sw::MARTIAN_TARGET_6
            | rfm_sw::MARTIAN_TARGET_7 => {
                self.skill_shot_ready = false;
                let bit = match id {
                    rfm_sw::MARTIAN_TARGET_1 => 0,
                    rfm_sw::MARTIAN_TARGET_2 => 1,
                    rfm_sw::MARTIAN_TARGET_3 => 2,
                    rfm_sw::MARTIAN_TARGET_4 => 3,
                    rfm_sw::MARTIAN_TARGET_5 => 4,
                    rfm_sw::MARTIAN_TARGET_6 => 5,
                    _ => 6,
                };
                self.rfm.martian_targets_mask |= 1 << bit;
                if bit <= 2 {
                    self.emit_coil(rfm_drive::LEFT_MARTIAN, 48);
                    self.emit_coil(rfm_drive::LEFT_MARTIAN_FL, 32);
                } else {
                    self.emit_coil(rfm_drive::RIGHT_MARTIAN, 48);
                    self.emit_coil(rfm_drive::RIGHT_MARTIAN_FL, 32);
                }
                self.add_score(75_000);
                if self.rfm.martian_targets_mask == 0x7F {
                    self.rfm.martian_targets_mask = 0;
                    self.rfm.multiball = RfmMultiball::MartianAttack;
                    self.pending_eject_balls = self.pending_eject_balls.saturating_add(1);
                    self.ball_save_ms = 20_000;
                    self.audits.multiballs_started = self.audits.multiballs_started.saturating_add(1);
                    self.emit_sound(rfm_sound::MULTIBALL_MUSIC, true);
                    self.emit_event(GameEvent::MultiballStarted {
                        mode: "MARTIAN ATTACK MULTIBALL".to_string(),
                    });
                    self.set_banner("MARTIAN ATTACK 2-BALL MULTIBALL!", 4_000);
                } else {
                    let count = self.rfm.martian_targets_mask.count_ones();
                    self.set_banner(format!("MARTIAN HIT! ({count}/7)"), 1_800);
                }
            }
            // Saucer Lockups -> Capture Multiball (3-ball)
            rfm_sw::RIGHT_LOCKUP_1 => {
                self.emit_coil(rfm_drive::RIGHT_LOCKUP, 64);
                self.rfm.saucer_locks += 1;
                if self.rfm.saucer_locks >= 3 {
                    self.rfm.saucer_locks = 0;
                    self.rfm.multiball = RfmMultiball::CaptureMultiball { jackpots: 0 };
                    self.pending_eject_balls = self.pending_eject_balls.saturating_add(2);
                    self.ball_save_ms = 20_000;
                    self.audits.multiballs_started = self.audits.multiballs_started.saturating_add(1);
                    self.emit_sound(rfm_sound::MULTIBALL_MUSIC, true);
                    self.emit_event(GameEvent::MultiballStarted {
                        mode: "CAPTURE MULTIBALL".to_string(),
                    });
                    self.set_banner("CAPTURE 3-BALL MULTIBALL!", 4_000);
                } else {
                    let l = self.rfm.saucer_locks;
                    self.add_score(150_000);
                    self.set_banner(format!("SAUCER LOCK {l}/3!"), 2_000);
                }
            }
            // Center Deflector Panel / Right Popper -> Start or Advance Attack Wave!
            rfm_sw::CENTER_DEFLECTOR_PANEL | rfm_sw::RIGHT_POPPER | rfm_sw::CENTER_TARGET_4..=rfm_sw::CENTER_TARGET_1 => {
                self.skill_shot_ready = false;
                self.emit_coil(rfm_drive::ATTACK_MARS_FL, 32);
                if id == rfm_sw::RIGHT_POPPER {
                    self.emit_coil(rfm_drive::RIGHT_POPPER, 64);
                }
                if self.rfm.active_wave.is_some() {
                    self.advance_rfm_wave("SAUCER HIT!", 250_000);
                } else {
                    let idx = (self.rfm.waves_completed.trailing_ones() as usize).min(8);
                    let wave = RfmWave::ALL[idx];
                    self.rfm.active_wave = Some(wave);
                    self.rfm.wave_hits_left = wave.required_hits();
                    self.audits.scenes_started = self.audits.scenes_started.saturating_add(1);
                    self.emit_sound(rfm_sound::WAVE_MUSIC, true);
                    self.add_score(250_000);
                    self.set_banner(format!("WAVE START: {}", wave.title()), 3_500);
                }
            }
            // Ramps & Loops -> Multiball Jackpots or Wave Progress
            rfm_sw::LEFT_RAMP_EXIT | rfm_sw::RIGHT_RAMP_EXIT | rfm_sw::LEFT_LOOP_HIGH | rfm_sw::RIGHT_LOOP_HIGH => {
                self.skill_shot_ready = false;
                if let RfmMultiball::CaptureMultiball { jackpots } = self.rfm.multiball {
                    let next_jp = jackpots + 1;
                    self.rfm.multiball = RfmMultiball::CaptureMultiball { jackpots: next_jp };
                    let award = 1_000_000 * (next_jp as u64);
                    self.add_score(award);
                    self.emit_sound(rfm_sound::EXPLOSION_SFX, false);
                    self.emit_event(GameEvent::JackpotAwarded {
                        super_jackpot: false,
                        points: award,
                    });
                    self.set_banner(format!("SAUCER JACKPOT! +{award}"), 2_500);
                } else if self.rfm.active_wave.is_some() {
                    self.advance_rfm_wave("RAMP SHOT!", 300_000);
                } else {
                    self.add_score(125_000);
                    self.set_banner("RAMP SHOT! +125,000", 1_500);
                }
            }
            rfm_sw::TROUGH_BALL_1 | rfm_sw::TROUGH_BALL_2 | rfm_sw::TROUGH_BALL_3 | rfm_sw::TROUGH_BALL_4 => {
                if self.balls_in_play > 0 && self.eject_cooldown_ms == 0 {
                    self.on_ball_drained();
                }
            }
            _ => {}
        }
    }

    pub(super) fn advance_rfm_wave(&mut self, label: &str, pts: u64) {
        self.add_score(pts);
        if let Some(wave) = self.rfm.active_wave {
            self.rfm.wave_hits_left = self.rfm.wave_hits_left.saturating_sub(1);
            if self.rfm.wave_hits_left == 0 {
                self.rfm.active_wave = None;
                let idx = RfmWave::ALL.iter().position(|&w| w == wave).unwrap_or(0);
                self.rfm.waves_completed |= 1 << idx;
                self.audits.scenes_completed = self.audits.scenes_completed.saturating_add(1);
                self.add_score(1_500_000);
                self.emit_sound(rfm_sound::MAIN_THEME, true);
                self.set_banner(format!("{} DESTROYED! +1,500,000", wave.title()), 3_500);
            } else {
                let left = self.rfm.wave_hits_left;
                self.set_banner(format!("{label} ({left} HITS LEFT)"), 2_000);
            }
        }
    }

    pub(super) fn update_rfm_lamps(&mut self) {
        self.hw.set_lamp(rfm_lamp::START_BUTTON, LampState::On);
        self.hw.set_lamp(rfm_lamp::SAUCER, LampState::On);
        self.hw.set_lamp(rfm_lamp::LEFT_MARTIAN_EYE, LampState::On);
        self.hw.set_lamp(rfm_lamp::RIGHT_MARTIAN_EYE, LampState::On);

        for (i, &lid) in rfm_lamp::SAUCER_RIM.iter().enumerate() {
            if (self.rfm.waves_completed & (1 << i)) != 0 {
                self.hw.set_lamp(lid, LampState::On);
            }
        }
        for (i, &lid) in rfm_lamp::MARTIAN_TARGETS.iter().enumerate() {
            if (self.rfm.martian_targets_mask & (1 << i)) != 0 {
                self.hw.set_lamp(lid, LampState::On);
            }
        }
        if self.ball_save_ms > 0 {
            self.hw.set_lamp(rfm_lamp::SHOOT_AGAIN, LampState::On);
        }
        if !matches!(self.rfm.multiball, RfmMultiball::None) {
            self.hw.set_lamp(rfm_lamp::MARTIAN_ATTACK, LampState::Blinking { period_ms: 200 });
        }
    }

}
