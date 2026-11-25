use pin2k_hal::{AssetId, AudioBackend, LampState, PinballHardware, VideoBackend};

use super::{DiagPage, MachineState, Pin2kMachine};
use crate::attract::AttractDeff;
use crate::rfm_rules::GameTitle;
use crate::rfm_tables::rfm_lamp;
use crate::swe1_rules::JediRank;
use crate::swe1_tables::lamp;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(super) fn update_lamps(&mut self) {
        for id in 0..128u8 {
            self.hw.set_lamp(id, LampState::Off);
        }

        match self.state {
            MachineState::Boot => {
                self.hw.set_lamp(lamp::COIN_DOOR, LampState::On);
            }
            MachineState::Attract => {
                // Attract lamp sweep (`LeffAttractMode`)
                if self.game_title == GameTitle::Rfm {
                    let rim_idx = ((self.elapsed_ms / 120) as usize) % 9;
                    self.hw.set_lamp(rfm_lamp::SAUCER_RIM[rim_idx], LampState::On);
                    self.hw.set_lamp(rfm_lamp::SAUCER, LampState::On);
                } else {
                    let phase = ((self.elapsed_ms / 120) as usize) % 12;
                    self.hw
                        .set_lamp(lamp::SHIELD_LAMPS[phase], LampState::On);
                    let jedi_idx = ((self.elapsed_ms / 250) as usize) % 4;
                    self.hw
                        .set_lamp(lamp::JEDI_LETTERS[jedi_idx], LampState::On);
                }
                if self.free_play || self.credits > 0 {
                    self.hw.set_lamp(
                        lamp::START_BUTTON,
                        LampState::Blinking { period_ms: 400 },
                    );
                }
            }
            MachineState::Diagnostics => {
                if let DiagPage::LampTest { selected_lamp } = self.diag_page {
                    self.hw
                        .set_lamp(selected_lamp, LampState::Blinking { period_ms: 250 });
                }
            }
            MachineState::Tilt => {
                self.hw.set_lamp(lamp::COIN_DOOR, LampState::On);
            }
            MachineState::HighScoreEntry => {
                self.hw.set_lamp(lamp::START_BUTTON, LampState::Blinking { period_ms: 250 });
                self.hw.set_lamp(lamp::LEFT_FLIPPER, LampState::On);
                self.hw.set_lamp(lamp::RIGHT_FLIPPER, LampState::On);
            }
            MachineState::InGame => {
                for &lid in &self.mod_lit_lamps {
                    self.hw.set_lamp(lid, LampState::On);
                }
                if let Some(ref rt) = self.script_runtime {
                    for &lid in &rt.lit_lamps {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }
                if self.game_title == GameTitle::Homebrew {
                    self.hw.set_lamp(lamp::START_BUTTON, LampState::On);
                    if self.ball_save_ms > 0 {
                        self.hw.set_lamp(lamp::SHOOT_AGAIN, LampState::On);
                    }
                    return;
                }
                if self.game_title == GameTitle::Rfm {
                    self.update_rfm_lamps();
                    return;
                }
                let p = &self.players[self.current_player];
                self.hw.set_lamp(lamp::START_BUTTON, LampState::On);
                self.hw.set_lamp(lamp::LEFT_FLIPPER, LampState::On);
                self.hw.set_lamp(lamp::RIGHT_FLIPPER, LampState::On);

                // J-E-D-I letters
                for (i, &lid) in lamp::JEDI_LETTERS.iter().enumerate() {
                    if (p.jedi_letters & (1 << i)) != 0 {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }

                // 12-lamp Deflector Shield meter
                for (i, &lid) in lamp::SHIELD_LAMPS.iter().enumerate() {
                    if (i as u8) < p.shield_health {
                        self.hw.set_lamp(lid, LampState::On);
                    } else if p.shield_health == 0 {
                        self.hw
                            .set_lamp(lid, LampState::Blinking { period_ms: 200 });
                    }
                }

                // Jedi Rank lamps
                if p.jedi_rank >= JediRank::Youth {
                    self.hw.set_lamp(lamp::JEDI_YOUTH, LampState::On);
                }
                if p.jedi_rank >= JediRank::Knight {
                    self.hw.set_lamp(lamp::JEDI_KNIGHT, LampState::On);
                }
                if p.jedi_rank >= JediRank::Master {
                    self.hw.set_lamp(lamp::JEDI_MASTER, LampState::On);
                }
                if p.jedi_rank >= JediRank::Spirit {
                    self.hw
                        .set_lamp(lamp::JEDI_SPIRIT, LampState::Blinking { period_ms: 200 });
                }

                // Lasers ready lamps
                if p.laser_shots > 0 {
                    self.hw.set_lamp(
                        lamp::FIRE_LASERS_LEFT,
                        LampState::Blinking { period_ms: 300 },
                    );
                    self.hw.set_lamp(
                        lamp::FIRE_LASERS_RIGHT,
                        LampState::Blinking { period_ms: 300 },
                    );
                }

                // Bonus multiplier lamps
                if p.bonus_multiplier >= 2 {
                    self.hw.set_lamp(lamp::BONUS_X2, LampState::On);
                }
                if p.bonus_multiplier >= 3 {
                    self.hw.set_lamp(lamp::BONUS_X3, LampState::On);
                }
                if p.bonus_multiplier >= 4 {
                    self.hw.set_lamp(lamp::BONUS_X4, LampState::On);
                }
                if p.bonus_multiplier >= 5 {
                    self.hw.set_lamp(lamp::BONUS_X5, LampState::On);
                }

                // Shoot Again / Ball Saver
                if self.ball_save_ms > 4_000 {
                    self.hw.set_lamp(lamp::SHOOT_AGAIN, LampState::On);
                } else if self.ball_save_ms > 0 {
                    self.hw.set_lamp(
                        lamp::SHOOT_AGAIN,
                        LampState::Blinking { period_ms: 180 },
                    );
                }

                if p.extra_balls > 0 {
                    self.hw.set_lamp(lamp::EXTRA_BALL, LampState::On);
                }

                // Completed Scenes light Naboo Starfighter body lamps!
                for (i, &lid) in lamp::NABOO_SHIP_BODY.iter().enumerate() {
                    if (p.scenes_completed & (1 << i)) != 0 {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }
                self.apply_leff_lamps();
            }
        }
    }

    pub(super) fn render_frame(&mut self) {
        self.video.begin_frame();

        if self.state == MachineState::Boot {
            self.render_boot_screen();
            self.video.present();
            return;
        }
        if self.state == MachineState::Diagnostics {
            if self.debug_anim_override.is_none() {
                self.render_diagnostics_screen();
            }
            self.video.present();
            return;
        }

        let frame_idx = match self.attract.current() {
            AttractDeff::P2kLogo if self.state == MachineState::Attract => (self.attract.timer_ms / 66) as usize,
            AttractDeff::KahunaLogo if self.state == MachineState::Attract => (self.attract.timer_ms / 40) as usize,
            _ => (self.elapsed_ms / 32) as usize,
        };
        if !self.active_anim_hint.is_empty() {
            self.video.draw_sprite(AssetId(0), frame_idx, 210, 20, 0);
        }

        match self.state {
            MachineState::Attract => {
                if let Some(screen) = self.special_screen {
                    self.render_special_screen(screen);
                } else if self.game_title == GameTitle::Swe1 && self.debug_anim_override.is_none() {
                    self.render_swe1_attract_frame();
                } else if self.debug_anim_override.is_none() {
                    if !self.status_banner.is_empty() {
                        self.video.draw_text(&self.status_banner, 320, 190, 0x7FE0, 2);
                    }
                    if (self.elapsed_ms / 500) % 2 == 0 {
                        let price_txt = if self.free_play {
                            "FREE PLAY - PRESS START".to_string()
                        } else if self.credits > 0 {
                            format!("CREDITS {} - PRESS START", self.credits)
                        } else {
                            "INSERT COIN".to_string()
                        };
                        self.video.draw_text(&price_txt, 320, 222, 0x03FF, 1);
                    }
                }
            }
            MachineState::InGame | MachineState::Tilt => {
                if self.game_title == GameTitle::Swe1 {
                    let is_scene_intro = self
                        .active_scene
                        .as_ref()
                        .is_some_and(|s| s.intro_remaining_ms > 0);
                    let is_fullscreen_special = matches!(
                        self.special_screen,
                        Some(
                            super::SpecialScreen::Bonus
                                | super::SpecialScreen::Match
                                | super::SpecialScreen::GameOver
                                | super::SpecialScreen::GameIntro
                                | super::SpecialScreen::ExtraBall
                                | super::SpecialScreen::SlamTilt
                        )
                    );
                    if self.render_overlay() {
                        // Full-screen timed overlay (`DeffWattoJunkShop`, `DeffMarketplace`,
                        // `DeffJediMultiballIntro`, `RankPromotion`, `JediSpirit`).
                        self.render_fx();
                        if self.state == MachineState::Tilt {
                            self.render_tilt_overlay();
                        }
                        self.video.present();
                        return;
                    }
                    if let Some(screen) = self.special_screen {
                        self.render_special_screen(screen);
                    } else if self.debug_anim_override.is_none() {
                        self.render_swe1_ingame_scene();
                    }
                    // Keep scene intros & fullscreen cutscenes clean without HUD overlap;
                    // show score display, laser HUD, spinners, and banners during active gameplay.
                    if !is_scene_intro && !is_fullscreen_special {
                        self.render_jarjar_saucer();
                        self.render_fx();
                        self.render_score_display(true);
                        self.render_laser_status_hud();
                        self.render_virtual_spinners();
                        self.render_ingame_banner();
                    }
                    if self.state == MachineState::Tilt {
                        self.render_tilt_overlay();
                    }
                } else {
                    if !self.status_banner.is_empty() {
                        self.video.draw_text(&self.status_banner, 320, 190, 0x7FE0, 2);
                    }
                    self.video.draw_text(
                        &format!("P{}  BALL {}   {:>10}", self.current_player + 1, self.current_ball, self.score),
                        320,
                        218,
                        0x7FFF,
                        2,
                    );
                }
            }
            MachineState::HighScoreEntry => {
                self.render_hstd_entry_screen();
            }
            _ => {}
        }
        self.video.present();
    }
}
