//! Authentic `JarjarJuggleScene` + `DeffJarjarJuggleScene` runtime & renderer (`swe1_0200.elf`):
//! - Lane 0: `LEFT_RAMP_ENTER` (`0x1d`) -> `(100, 225)`
//! - Lane 1: `LEFT_SHIELD_TARGET` (`0x22`) -> `(175, 240)`
//! - Lane 2: `SHIELD_HIT` (`0x27`, center) -> `(285, 240)` + knocks Jar Jar down!
//! - Lane 3: `RIGHT_SHIELD_TARGET` (`0x23`) -> `(400, 240)`
//! - Lane 4: `RIGHT_RAMP_ENTER` (`0x1e`) -> `(475, 225)`

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};

use super::jarjar_juggle_state::{
    JarjarJuggleState, JuggleAnimState, JuggleAward, LANE_X, LANE_Y,
};
use crate::events::GameEvent;
use crate::machine::{LeffKind, Pin2kMachine};
use crate::swe1_rules::{scoring, Swe1Scene};
use crate::swe1_tables::sw;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_jarjar_juggle_state(&mut self) {
        self.jarjar_juggle = Some(JarjarJuggleState::new(
            self.elapsed_ms ^ ((self.current_player as u64 + 1) * 0x9E37_79B9),
        ));
    }

    pub(crate) fn tick_jarjar_juggle(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::JarJarJuggle && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        let mut emit_ard_name: Option<&'static str> = None;
        if let Some(ref mut jj) = self.jarjar_juggle {
            jj.intro_textbox_ms = jj.intro_textbox_ms.saturating_sub(dt_ms);
            jj.anim_ms = jj.anim_ms.saturating_add(dt_ms);

            for lane in &mut jj.lanes {
                lane.award_text_ms = lane.award_text_ms.saturating_sub(dt_ms);
                if lane.flying_ms > 0 {
                    lane.flying_ms = lane.flying_ms.saturating_sub(dt_ms);
                    let step = (dt_ms as i32).max(1);
                    lane.x_q8 = lane.x_q8.saturating_add((lane.vx_q8 * step) / 16);
                    lane.y_q8 = lane.y_q8.saturating_add((lane.vy_q8 * step) / 16);
                    lane.vy_q8 = lane.vy_q8.saturating_add((28 * step) / 16);
                }
            }

            let (_, _, _, frames, ms_per_f) = jj.anim.spec();
            let dur = (frames as u32) * ms_per_f;
            if jj.anim_ms >= dur {
                jj.anim_ms = 0;
                match jj.anim {
                    JuggleAnimState::Juggle1 => {
                        jj.juggle_cycles = jj.juggle_cycles.saturating_add(1);
                        if jj.juggle_cycles >= 2 {
                            jj.juggle_cycles = 0;
                            jj.anim = JuggleAnimState::Juggle2;
                            emit_ard_name = Some("ard_jarjar_juggle_scene_jarjar_juggle");
                        }
                    }
                    JuggleAnimState::Juggle2 => jj.anim = JuggleAnimState::Juggle1,
                    JuggleAnimState::Fall1 | JuggleAnimState::Fall2 => {
                        jj.anim = JuggleAnimState::JumpUp;
                        emit_ard_name = Some("ard_jarjar_juggle_scene_jarjar_jumpup");
                    }
                    JuggleAnimState::JumpUp => jj.anim = JuggleAnimState::Switch,
                    JuggleAnimState::Switch => {
                        jj.anim = JuggleAnimState::Stare;
                        emit_ard_name = Some("ard_jarjar_juggle_scene_jarjar_stare");
                    }
                    JuggleAnimState::Stare => {
                        jj.juggle_cycles = jj.juggle_cycles.saturating_add(1);
                        if jj.juggle_cycles >= 3 {
                            jj.juggle_cycles = 0;
                            emit_ard_name = Some("ard_jarjar_juggle_scene_jarjar_stare");
                        }
                    }
                }
            }
        }
        if let Some(ard) = emit_ard_name {
            self.emit_ard(ard);
        }
    }

    /// Handles authentic Jar Jar Juggle switch hits (`0x1d`, `0x22`, `0x27`, `0x23`, `0x1e`).
    pub(crate) fn handle_jarjar_juggle_switch(&mut self, sw_id: u8) -> bool {
        let is_jj = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::JarJarJuggle);
        if !is_jj {
            return false;
        }
        let lane_idx = match sw_id {
            sw::LEFT_RAMP_ENTER => 0usize,
            sw::LEFT_SHIELD_TARGET => 1,
            sw::SHIELD_HIT => 2,
            sw::RIGHT_SHIELD_TARGET => 3,
            sw::RIGHT_RAMP_ENTER => 4,
            _ => return false,
        };
        if self.jarjar_juggle.is_none() {
            self.start_jarjar_juggle_state();
        }

        let is_center = lane_idx == 2;
        let mut knocked_jarjar = false;
        let mut awarded: Option<(JuggleAward, u8)> = None;
        let mut missed = false;

        if let Some(ref mut jj) = self.jarjar_juggle {
            if is_center {
                match jj.anim {
                    JuggleAnimState::Juggle1 | JuggleAnimState::Juggle2 => {
                        jj.anim = JuggleAnimState::Fall1;
                        jj.anim_ms = 0;
                        jj.dropped = true;
                        knocked_jarjar = true;
                    }
                    JuggleAnimState::Stare | JuggleAnimState::Switch => {
                        jj.anim = JuggleAnimState::Fall2;
                        jj.anim_ms = 0;
                        jj.dropped = true;
                        knocked_jarjar = true;
                    }
                    _ => {}
                }
            }
            let lane = &mut jj.lanes[lane_idx];
            if !lane.collected {
                lane.collected = true;
                lane.flying_ms = 1_200;
                lane.award_text_ms = 3_000;
                let (_, w, h) = lane.award.sprite_info();
                lane.x_q8 = (LANE_X[lane_idx] - w / 2) << 8;
                lane.y_q8 = (LANE_Y[lane_idx] - h - 10) << 8;
                lane.vx_q8 = if lane_idx < 2 { -900 } else if lane_idx > 2 { 900 } else { 420 };
                lane.vy_q8 = -1_400;
                jj.remaining = jj.remaining.saturating_sub(1);
                awarded = Some((lane.award, jj.remaining));
            } else {
                missed = true;
            }
        }

        if knocked_jarjar {
            self.emit_ard("ard_jarjar_juggle_scene_jarjar_hit_sound");
            self.emit_ard("ard_jarjar_juggle_scene_jarjar_hit_speech");
            self.trigger_leff(LeffKind::ShieldHit);
        }
        if let Some((award, rem)) = awarded {
            let pts = match award {
                JuggleAward::Points500k => 500_000,
                JuggleAward::Points1m => 1_000_000,
                JuggleAward::Points1_5m => 1_500_000,
                JuggleAward::Points2m => 2_000_000,
                JuggleAward::FiveLasers => {
                    let p = &mut self.players[self.current_player];
                    p.laser_shots = (p.laser_shots + 5).min(9);
                    250_000
                }
                JuggleAward::IncreaseBonusX => {
                    let p = &mut self.players[self.current_player];
                    p.bonus_multiplier = (p.bonus_multiplier + 1).min(scoring::BONUS_MAX_MULTIPLIER);
                    250_000
                }
                JuggleAward::LightExtraBall => {
                    let p = &mut self.players[self.current_player];
                    p.extra_balls = p.extra_balls.saturating_add(1);
                    250_000
                }
            };
            self.add_score(pts);
            self.emit_ard("ard_jarjar_juggle_scene_junk_hit");
            self.trigger_leff(LeffKind::SceneHit);
            self.emit_event(GameEvent::SceneProgressed {
                scene: Swe1Scene::JarJarJuggle,
                hits_remaining: rem,
                points: pts,
            });
            self.set_banner(format!("JAR JAR AWARD: {}!", award.label()), 2_200);
            if let Some(ref mut sc) = self.active_scene {
                sc.hits_remaining = rem;
            }
            if rem == 0 {
                self.complete_active_scene(Swe1Scene::JarJarJuggle);
                self.jarjar_juggle = None;
            }
        } else if missed {
            self.emit_ard("ard_jarjar_juggle_scene_junk_miss");
        }
        true
    }

    /// Renders `DeffJarjarJuggleScene`: backdrop, Jar Jar animation state machine,
    /// 5 lane rewards (juggling / dropped / flying), and flashing `showoff_award_text`.
    pub(crate) fn render_jarjar_juggle(&mut self, secs: u32) {
        self.video.draw_named_sprite(SpriteDrawCmd::new(
            "anim_jarjar_juggle_backdrop", 0, 0, 50, 1,
        ));
        let Some(jj) = self.jarjar_juggle.clone() else {
            return;
        };

        let (jj_anim, jj_w, jj_h, jj_frames, ms_per_f) = jj.anim.spec();
        let jj_f = ((jj.anim_ms / ms_per_f) as usize).min(jj_frames.saturating_sub(1));
        self.video.draw_named_sprite(SpriteDrawCmd::new(
            jj_anim, jj_f, 315 - jj_w / 2, (220 - jj_h).max(0), 14,
        ));

        let f20 = ((self.elapsed_ms / 50) as usize) % 20;
        let blink = (self.elapsed_ms / 180) % 2 == 0;
        for (i, lane) in jj.lanes.iter().enumerate() {
            let (spr, w, h) = lane.award.sprite_info();
            if !lane.collected {
                let (x, y) = if jj.dropped {
                    (LANE_X[i] - w / 2, (LANE_Y[i] - h - 10).max(52))
                } else {
                    let phase = ((self.elapsed_ms / 45 + (i as u64) * 5) % 16) as i32;
                    let bob = if phase < 8 { phase * 3 } else { (16 - phase) * 3 };
                    (LANE_X[i] - w / 2, 56 + bob)
                };
                self.video.draw_named_sprite(SpriteDrawCmd::new(spr, (f20 + i * 4) % 20, x, y, 16));
            } else if lane.flying_ms > 0 {
                self.video.draw_named_sprite(SpriteDrawCmd::new(
                    spr, (f20 + i * 4) % 20, lane.x_q8 >> 8, lane.y_q8 >> 8, 18,
                ));
            }
            if lane.award_text_ms > 0 && blink {
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_system_7_data".into(),
                    text: lane.award.label().to_string(),
                    x: LANE_X[i],
                    y: 152,
                    z: 22,
                    color_rgb555: 0x7FE0,
                    shadow_rgb555: Some(0x0000),
                    align: TextAlign::Center,
                });
            }
        }

        if jj.intro_textbox_ms > 0 {
            let tb_f = (((3_500 - jj.intro_textbox_ms) / 33) as usize).min(29);
            self.video.draw_named_sprite(SpriteDrawCmd::new("anim_textbox_300", tb_f, 170, 52, 22));
            self.video.draw_font_text(TextDrawCmd {
                font_name: "font_system_7_data".into(),
                text: "HIT CENTER & SHOOT LANES!".into(),
                x: 320,
                y: 62,
                z: 23,
                color_rgb555: 0x7FE0,
                shadow_rgb555: Some(0x0000),
                align: TextAlign::Center,
            });
        }
        self.video.draw_font_text(TextDrawCmd {
            font_name: "font_shadow_9_narrow_data".into(),
            text: format!("ITEMS LEFT: {}   TIME: {secs}S", jj.remaining),
            x: 320,
            y: 222,
            z: 22,
            color_rgb555: 0x7FE0,
            shadow_rgb555: Some(0x0000),
            align: TextAlign::Center,
        });
    }
}
