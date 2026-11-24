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
            }
        }
    }

    pub(super) fn render_frame(&mut self) {
        if self.state == MachineState::Boot {
            self.active_anim_hint.clear();
            self.video.draw_text("WILLIAMS PINBALL 2000", 320, 62, 0x7FE0, 2);
            self.video.draw_text(self.game_title.title(), 320, 100, 0x7FFF, 2);
            self.video.draw_text("XINA / PRISM ROM CHECK ... OK", 320, 150, 0x03E0, 1);
            self.video.present();
            return;
        }
        if self.state == MachineState::Diagnostics {
            self.active_anim_hint.clear();
            let (title, rows) = self.admin_display_lines();
            self.video.draw_text(&title, 320, 22, 0x7FE0, 2);
            for (i, (line, selected)) in rows.iter().enumerate() {
                let prefix = if *selected { "> " } else { "  " };
                let color = if *selected { 0x03FF } else { 0x7FFF };
                self.video.draw_text(
                    &format!("{prefix}{line}"),
                    320,
                    66 + (i as i32) * 26,
                    color,
                    2,
                );
            }
            self.video.draw_text(
                "UP/DN: SELECT   ENTER: OK   ESC: BACK",
                320,
                216,
                0x5EF7,
                1,
            );
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
                match self.attract.current() {
                    AttractDeff::Hstd => {
                        self.video.draw_text("HIGHEST SCORES", 320, 26, 0x7FE0, 2);
                        for (i, hs) in self.high_scores.iter().take(4).enumerate() {
                            let ini: String = hs.initials.iter().collect();
                            self.video.draw_text(
                                &format!("#{} {}  {:>10}", i + 1, ini, hs.score),
                                320,
                                68 + (i as i32) * 26,
                                0x7FFF,
                                2,
                            );
                        }
                    }
                    AttractDeff::JediTableYouth
                    | AttractDeff::JediTableKnight
                    | AttractDeff::JediTableMaster
                    | AttractDeff::JediTableSpirit => {
                        self.video.draw_text(&self.status_banner, 320, 32, 0x03FF, 2);
                        self.video.draw_text("QGJ   OBI   ANI   PAD", 320, 96, 0x7FFF, 2);
                        self.video.draw_text("DEFEAT DARTH MAUL TO ADVANCE", 320, 140, 0x7FE0, 1);
                    }
                    AttractDeff::ScoreAwards => {
                        self.video.draw_text("REPLAY AT 15,000,000", 320, 74, 0x7FE0, 2);
                        self.video.draw_text("EXTRA BALL AT 4 SCENES", 320, 114, 0x7FFF, 2);
                    }
                    AttractDeff::GameCredits1
                    | AttractDeff::GameCredits2
                    | AttractDeff::GameCredits3
                    | AttractDeff::GameCredits4 => {
                        self.video.draw_text(&self.status_banner, 320, 74, 0x7FE0, 2);
                        self.video.draw_text("STAR WARS EPISODE I PINBALL", 320, 116, 0x7FFF, 2);
                    }
                    _ => {
                        if !self.status_banner.is_empty() {
                            self.video.draw_text(&self.status_banner, 320, 190, 0x7FE0, 2);
                        }
                    }
                }
                // DeffAttractModePriceBackground: blink FREE PLAY / CREDITS every 500 ms
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
            MachineState::InGame | MachineState::Tilt => {
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
            MachineState::HighScoreEntry => {
                let (p_num, p_score, p_rank) = self
                    .hstd_session
                    .queue
                    .first()
                    .copied()
                    .unwrap_or((1, self.score, JediRank::Padawan));
                self.video.draw_text("GREAT SCORE - ENTER INITIALS", 320, 28, 0x7FE0, 2);
                self.video.draw_text(
                    &format!("PLAYER {p_num}  -  {p_score} ({})", p_rank.label()),
                    320,
                    64,
                    0x7FFF,
                    2,
                );
                let mut slots = String::new();
                for (i, &ch) in self.hstd_session.initials.iter().enumerate() {
                    if i > 0 {
                        slots.push_str("   ");
                    }
                    if i == self.hstd_session.cursor {
                        slots.push_str(&format!("[{ch}]"));
                    } else {
                        slots.push_str(&format!(" {ch} "));
                    }
                }
                self.video.draw_text(&slots, 320, 118, 0x03FF, 3);
                self.video.draw_text(
                    "FLIPPERS: SELECT LETTER   START: CONFIRM",
                    320,
                    196,
                    0x5EF7,
                    1,
                );
            }
            _ => {}
        }
        self.video.present();
    }
}
