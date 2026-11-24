use pin2k_hal::{AudioBackend, PinballHardware, SoundId, VideoBackend};

use super::{DiagPage, MachineState, Pin2kMachine};
use crate::admin::{AdminScreen, GameAdjustments, GameAudits, ADMIN_SOUND_PRESETS, DEBUG_LAUNCH_PRESETS};
use crate::events::GameEvent;
use crate::swe1_rules::default_high_scores;
use crate::swe1_tables::{drive_name, sw, switch_name};

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(super) fn handle_diagnostics_switch(&mut self, id: u8) {
        match id {
            sw::DIAG_ESCAPE => match self.admin_screen {
                AdminScreen::Root { .. } => {
                    self.cut_all_holds();
                    self.state = MachineState::Attract;
                    self.emit_event(GameEvent::AdminMenuExited);
                    self.set_banner("STAR WARS EPISODE I - PRESS START", 0);
                }
                AdminScreen::SwitchTest
                | AdminScreen::CoilTest { .. }
                | AdminScreen::LampTest { .. }
                | AdminScreen::SoundTest { .. } => {
                    self.admin_screen = AdminScreen::DiagnosticsMenu { cursor: 0 };
                    self.set_banner("DIAGNOSTICS MENU", 3_000);
                }
                AdminScreen::DiagnosticsMenu { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 0 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
                AdminScreen::Audits { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 1 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
                AdminScreen::Adjustments { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 2 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
                AdminScreen::Utilities { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 3 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
                AdminScreen::DebugLauncher { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 4 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
            },
            sw::DIAG_ENTER => match self.admin_screen {
                AdminScreen::Root { cursor } => {
                    self.admin_screen = match cursor {
                        0 => AdminScreen::DiagnosticsMenu { cursor: 0 },
                        1 => AdminScreen::Audits { cursor: 0 },
                        2 => AdminScreen::Adjustments { cursor: 0 },
                        3 => AdminScreen::Utilities { cursor: 0 },
                        _ => AdminScreen::DebugLauncher { cursor: 0 },
                    };
                }
                AdminScreen::DiagnosticsMenu { cursor } => {
                    self.admin_screen = match cursor {
                        0 => {
                            self.diag_page = DiagPage::SwitchTest;
                            AdminScreen::SwitchTest
                        }
                        1 => {
                            self.diag_page = DiagPage::CoilTest { selected_drive: 0 };
                            AdminScreen::CoilTest { selected_drive: 0 }
                        }
                        2 => {
                            self.diag_page = DiagPage::LampTest { selected_lamp: 2 };
                            AdminScreen::LampTest { selected_lamp: 2 }
                        }
                        _ => AdminScreen::SoundTest { selected_idx: 0 },
                    };
                }
                AdminScreen::SwitchTest => {
                    self.diag_page = DiagPage::CoilTest { selected_drive: 0 };
                    self.admin_screen = AdminScreen::CoilTest { selected_drive: 0 };
                }
                AdminScreen::CoilTest { selected_drive } => {
                    if self.coin_door_closed {
                        self.emit_coil(selected_drive, 48);
                        self.set_banner(
                            format!("PULSED COIL {:02}: {}", selected_drive, drive_name(selected_drive)),
                            3_000,
                        );
                    } else {
                        self.set_banner("CLOSE COIN DOOR FOR HV COILS", 3_000);
                    }
                }
                AdminScreen::LampTest { selected_lamp } => {
                    self.set_banner(format!("LAMP #{:03} ACTIVE", selected_lamp), 3_000);
                }
                AdminScreen::SoundTest { selected_idx } => {
                    let (sid, label) = ADMIN_SOUND_PRESETS[(selected_idx as usize) % ADMIN_SOUND_PRESETS.len()];
                    self.emit_sound(SoundId(sid), false);
                    self.set_banner(format!("PLAY SOUND 0x{:04X}: {}", sid, label), 3_000);
                }
                AdminScreen::Audits { .. } => {}
                AdminScreen::Adjustments { cursor } => {
                    self.toggle_adjustment(cursor);
                }
                AdminScreen::Utilities { cursor } => {
                    self.execute_utility(cursor);
                }
                AdminScreen::DebugLauncher { cursor } => {
                    if let Some(&(target, _)) = DEBUG_LAUNCH_PRESETS.get(cursor as usize) {
                        self.debug_launch_mode(target);
                    }
                }
            },
            sw::DIAG_UP | sw::DIAG_DOWN => {
                let step: i16 = if id == sw::DIAG_UP { 1 } else { -1 };
                match self.admin_screen {
                    AdminScreen::Root { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(5) as u8;
                        self.admin_screen = AdminScreen::Root { cursor: next };
                    }
                    AdminScreen::DiagnosticsMenu { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(4) as u8;
                        self.admin_screen = AdminScreen::DiagnosticsMenu { cursor: next };
                    }
                    AdminScreen::SwitchTest => {}
                    AdminScreen::CoilTest { selected_drive } => {
                        let next_drive = (selected_drive as i16 + step).rem_euclid(48) as u8;
                        self.diag_page = DiagPage::CoilTest { selected_drive: next_drive };
                        self.admin_screen = AdminScreen::CoilTest { selected_drive: next_drive };
                        if self.coin_door_closed {
                            self.emit_coil(next_drive, 48);
                        }
                        self.set_banner(
                            format!("COIL {:02}: {}", next_drive, drive_name(next_drive)),
                            4_000,
                        );
                    }
                    AdminScreen::LampTest { selected_lamp } => {
                        let next_lamp = (selected_lamp as i16 + step).rem_euclid(128) as u8;
                        self.diag_page = DiagPage::LampTest { selected_lamp: next_lamp };
                        self.admin_screen = AdminScreen::LampTest { selected_lamp: next_lamp };
                        self.set_banner(format!("LAMP TEST #{:03}", next_lamp), 4_000);
                    }
                    AdminScreen::SoundTest { selected_idx } => {
                        let len = ADMIN_SOUND_PRESETS.len() as i16;
                        let next = (selected_idx as i16 + step).rem_euclid(len) as u8;
                        let (sid, label) = ADMIN_SOUND_PRESETS[next as usize];
                        self.admin_screen = AdminScreen::SoundTest { selected_idx: next };
                        self.set_banner(format!("SND 0x{:04X}: {}", sid, label), 4_000);
                    }
                    AdminScreen::Audits { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(8) as u8;
                        self.admin_screen = AdminScreen::Audits { cursor: next };
                    }
                    AdminScreen::Adjustments { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(6) as u8;
                        self.admin_screen = AdminScreen::Adjustments { cursor: next };
                    }
                    AdminScreen::Utilities { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(3) as u8;
                        self.admin_screen = AdminScreen::Utilities { cursor: next };
                    }
                    AdminScreen::DebugLauncher { cursor } => {
                        let len = DEBUG_LAUNCH_PRESETS.len() as i16;
                        let next = (cursor as i16 + step).rem_euclid(len) as u8;
                        self.admin_screen = AdminScreen::DebugLauncher { cursor: next };
                    }
                }
            }
            other => {
                if matches!(self.admin_screen, AdminScreen::SwitchTest) {
                    self.set_banner(format!("SW {:02}: {}", other, switch_name(other)), 4_000);
                }
            }
        }
    }

    pub(super) fn toggle_adjustment(&mut self, cursor: u8) {
        let (name, val) = match cursor {
            0 => {
                self.adjustments.free_play = !self.adjustments.free_play;
                self.free_play = self.adjustments.free_play;
                ("FREE PLAY", if self.free_play { "YES" } else { "NO" }.to_string())
            }
            1 => {
                self.adjustments.balls_per_game = if self.adjustments.balls_per_game == 3 { 5 } else { 3 };
                self.balls_per_game = self.adjustments.balls_per_game;
                ("BALLS PER GAME", self.balls_per_game.to_string())
            }
            2 => {
                self.adjustments.ball_save_secs = match self.adjustments.ball_save_secs {
                    10 => 15,
                    15 => 20,
                    _ => 10,
                };
                ("BALL SAVE SECS", format!("{}S", self.adjustments.ball_save_secs))
            }
            3 => {
                self.adjustments.tilt_warnings_max = match self.adjustments.tilt_warnings_max {
                    2 => 3,
                    3 => 4,
                    _ => 2,
                };
                ("TILT WARNINGS", self.adjustments.tilt_warnings_max.to_string())
            }
            4 => {
                self.adjustments.master_volume_pct = match self.adjustments.master_volume_pct {
                    25 => 50,
                    50 => 75,
                    75 => 85,
                    85 => 100,
                    _ => 25,
                };
                self.audio
                    .set_master_volume((self.adjustments.master_volume_pct as f32) / 100.0);
                ("MASTER VOLUME", format!("{}%", self.adjustments.master_volume_pct))
            }
            _ => {
                self.adjustments.difficulty = self.adjustments.difficulty.next();
                ("DIFFICULTY", self.adjustments.difficulty.label().to_string())
            }
        };
        self.emit_event(GameEvent::AdjustmentChanged {
            name: name.to_string(),
            value: val.clone(),
        });
        self.set_banner(format!("{name}: {val}"), 3_000);
    }

    pub(super) fn execute_utility(&mut self, cursor: u8) {
        let action = match cursor {
            0 => {
                self.high_scores = default_high_scores();
                "HIGH SCORES RESET"
            }
            1 => {
                self.audits = GameAudits::default();
                "AUDITS CLEARED"
            }
            _ => {
                self.adjustments = GameAdjustments::default();
                self.free_play = self.adjustments.free_play;
                self.balls_per_game = self.adjustments.balls_per_game;
                self.audio
                    .set_master_volume((self.adjustments.master_volume_pct as f32) / 100.0);
                "FACTORY DEFAULTS RESTORED"
            }
        };
        self.emit_event(GameEvent::UtilityExecuted {
            action: action.to_string(),
        });
        self.set_banner(action, 3_500);
    }

    /// Returns `(header, rows)` for rendering the Pinball 2000 Operator Administration Menu.
    pub fn admin_display_lines(&self) -> (String, Vec<(String, bool)>) {
        match self.admin_screen {
            AdminScreen::Root { cursor } => (
                "PINBALL 2000 ADMIN MENU".to_string(),
                vec![
                    ("DIAGNOSTICS".into(), cursor == 0),
                    ("AUDITS".into(), cursor == 1),
                    ("ADJUSTMENTS".into(), cursor == 2),
                    ("UTILITIES".into(), cursor == 3),
                    ("DEBUG LAUNCHER".into(), cursor == 4),
                ],
            ),
            AdminScreen::DiagnosticsMenu { cursor } => (
                "DIAGNOSTICS".to_string(),
                vec![
                    ("SWITCH TEST".into(), cursor == 0),
                    ("COIL / SOLENOID TEST".into(), cursor == 1),
                    ("LAMP TEST".into(), cursor == 2),
                    ("SOUND / DCS TEST".into(), cursor == 3),
                ],
            ),
            AdminScreen::SwitchTest => {
                let sw_line = match self.last_switch_hit {
                    Some(id) => format!("LAST SW {:02}: {}", id, switch_name(id)),
                    None => "ACTIVATE ANY PLAYFIELD SWITCH".to_string(),
                };
                (
                    "DIAGNOSTICS: SWITCH TEST".to_string(),
                    vec![
                        (sw_line, true),
                        (
                            format!("COIN DOOR: {}", if self.coin_door_closed { "CLOSED" } else { "OPEN" }),
                            false,
                        ),
                    ],
                )
            }
            AdminScreen::CoilTest { selected_drive } => (
                "DIAGNOSTICS: COIL TEST".to_string(),
                vec![
                    (format!("COIL {:02}: {}", selected_drive, drive_name(selected_drive)), true),
                    (
                        format!("HV INTERLOCK: {}", if self.coin_door_closed { "READY" } else { "DOOR OPEN" }),
                        false,
                    ),
                ],
            ),
            AdminScreen::LampTest { selected_lamp } => (
                "DIAGNOSTICS: LAMP TEST".to_string(),
                vec![
                    (format!("LAMP #{:03} (MATRIX {})", selected_lamp, if selected_lamp < 64 { "A" } else { "B" }), true),
                ],
            ),
            AdminScreen::SoundTest { selected_idx } => {
                let (sid, label) = ADMIN_SOUND_PRESETS[(selected_idx as usize) % ADMIN_SOUND_PRESETS.len()];
                (
                    "DIAGNOSTICS: DCS SOUND TEST".to_string(),
                    vec![(format!("CMD 0x{:04X}: {}", sid, label), true)],
                )
            }
            AdminScreen::Audits { cursor } => {
                let rows = vec![
                    (format!("GAMES PLAYED : {}", self.audits.total_games_played), cursor == 0),
                    (format!("BALLS PLAYED : {}", self.audits.total_balls_played), cursor == 1),
                    (format!("CREDITS      : {}", self.audits.total_credits), cursor == 2),
                    (format!("SCENES START : {}", self.audits.scenes_started), cursor == 3),
                    (format!("SCENES WON   : {}", self.audits.scenes_completed), cursor == 4),
                    (format!("MULTIBALLS   : {}", self.audits.multiballs_started), cursor == 5),
                    (format!("MAUL DUELS   : {}", self.audits.maul_duels_won), cursor == 6),
                    (format!("HIGH SCORE   : {}", self.audits.highest_score), cursor == 7),
                ];
                let start = (cursor as usize / 4) * 4;
                ("SYSTEM AUDITS".to_string(), rows[start..(start + 4).min(rows.len())].to_vec())
            }
            AdminScreen::Adjustments { cursor } => {
                let rows = vec![
                    (format!("FREE PLAY    : {}", if self.adjustments.free_play { "YES" } else { "NO" }), cursor == 0),
                    (format!("BALLS/GAME   : {}", self.adjustments.balls_per_game), cursor == 1),
                    (format!("BALL SAVE    : {}S", self.adjustments.ball_save_secs), cursor == 2),
                    (format!("TILT WARNS   : {}", self.adjustments.tilt_warnings_max), cursor == 3),
                    (format!("VOLUME       : {}%", self.adjustments.master_volume_pct), cursor == 4),
                    (format!("DIFFICULTY   : {}", self.adjustments.difficulty.label()), cursor == 5),
                ];
                let start = (cursor as usize / 4) * 4;
                ("GAME ADJUSTMENTS".to_string(), rows[start..(start + 4).min(rows.len())].to_vec())
            }
            AdminScreen::Utilities { cursor } => (
                "SYSTEM UTILITIES".to_string(),
                vec![
                    ("RESET HIGH SCORES".into(), cursor == 0),
                    ("CLEAR AUDITS".into(), cursor == 1),
                    ("FACTORY DEFAULTS".into(), cursor == 2),
                ],
            ),
            AdminScreen::DebugLauncher { cursor } => {
                let start = (cursor as usize / 4) * 4;
                let end = (start + 4).min(DEBUG_LAUNCH_PRESETS.len());
                let rows = (start..end)
                    .map(|idx| (DEBUG_LAUNCH_PRESETS[idx].1.to_string(), idx == cursor as usize))
                    .collect();
                ("DEBUG LAUNCHER (SCENES/GAMES)".to_string(), rows)
            }
        }
    }

}
