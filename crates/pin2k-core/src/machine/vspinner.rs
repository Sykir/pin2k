//! Authentic `VirtualSpinners` / `DeffVSpinner` (`0x0017ead0..0x0017fa04` in `swe1_0200.elf`):
//! - Left spinner at `(43, 174)` (`0x2b, 0xae`) `z = 22` (`anim_spinner_bracket_left` + `anim_spinner_1`).
//! - Right spinner at `(421, 174)` (`0x1a5, 0xae`) `z = 22` (`anim_spinner_bracket_right` + `anim_spinner_2`).
//! - **Only active when triggered by a ball passing a ramp entry/made switch!**
//! - Plays `acd_traditional_spinner_sound` (`0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`) on every
//!   half-turn and spawns a rising score popup (`z = 23`, `font_system_7_data`).

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};
use serde::{Deserialize, Serialize};

use super::Pin2kMachine;
use crate::swe1_tables::sound;

pub const LEFT_SPINNER_X: i32 = 43;
pub const RIGHT_SPINNER_X: i32 = 421;
pub const SPINNER_Y: i32 = 174;
const SPINNER_ANIM_FRAMES: u32 = 56;
const FRAMES_PER_TURNSTILE_CLICK: u32 = 28; // half-turn = 1 audible click

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VirtualSpinnerState {
    pub active: bool,
    pub forward: bool,
    pub frame_accum_q8: u32,
    pub velocity_q8: u32,
    pub tick_accum_q8: u32,
    pub total_spins: u32,
    pub sound_idx: usize,
    pub popup_ms: u32,
    pub last_award: u64,
    pub high_speed: bool,
}

impl VirtualSpinnerState {
    pub fn trigger(&mut self, forward: bool) {
        self.active = true;
        self.forward = forward;
        self.velocity_q8 = (self.velocity_q8 + 512).clamp(480, 960);
        self.high_speed = self.velocity_q8 >= 720;
        self.popup_ms = 1_100;
        self.last_award = if self.high_speed { 25_000 } else { 10_000 };
    }
}

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    /// Triggers the left or right virtual spinner (called on ramp entry/made).
    pub fn trigger_virtual_spinner(&mut self, right_side: bool, forward: bool) {
        let sp = if right_side {
            &mut self.right_spinner
        } else {
            &mut self.left_spinner
        };
        sp.trigger(forward);
        let sid = sound::SPINNER_TICKS[sp.sound_idx % sound::SPINNER_TICKS.len()];
        sp.sound_idx = sp.sound_idx.wrapping_add(1);
        sp.total_spins = sp.total_spins.saturating_add(1);
        let award = sp.last_award;
        self.emit_sound(sid, false);
        self.add_score(award);
    }

    pub(super) fn update_virtual_spinners(&mut self, dt_ms: u32) {
        for side in 0..2 {
            let mut click_sound = None;
            let mut click_award = 0u64;
            {
                let sp = if side == 0 {
                    &mut self.left_spinner
                } else {
                    &mut self.right_spinner
                };
                if !sp.active {
                    continue;
                }
                let step = (sp.velocity_q8 * dt_ms) / 16;
                sp.frame_accum_q8 = sp.frame_accum_q8.wrapping_add(step);
                sp.tick_accum_q8 = sp.tick_accum_q8.saturating_add(step);
                if sp.tick_accum_q8 >= FRAMES_PER_TURNSTILE_CLICK * 256 {
                    sp.tick_accum_q8 -= FRAMES_PER_TURNSTILE_CLICK * 256;
                    sp.total_spins = sp.total_spins.saturating_add(1);
                    click_sound = Some(sound::SPINNER_TICKS[sp.sound_idx % sound::SPINNER_TICKS.len()]);
                    sp.sound_idx = sp.sound_idx.wrapping_add(1);
                    click_award = if sp.high_speed { 25_000 } else { 10_000 };
                    sp.last_award = click_award;
                    sp.popup_ms = 900;
                }
                // Smooth friction decay (~1.6 s spin)
                let decay = (dt_ms / 3).max(1);
                sp.velocity_q8 = sp.velocity_q8.saturating_sub(decay);
                sp.popup_ms = sp.popup_ms.saturating_sub(dt_ms);
                if sp.velocity_q8 == 0 && sp.popup_ms == 0 {
                    sp.active = false;
                    sp.high_speed = false;
                }
            }
            if let Some(sid) = click_sound {
                self.emit_sound(sid, false);
                self.add_score(click_award);
            }
        }
    }

    /// Renders active left/right virtual spinners (`z = 22`) and rising score popups (`z = 23`).
    /// Does NOTHING when neither spinner has been triggered by a ramp shot!
    pub fn render_virtual_spinners(&mut self) {
        for (side, sp, x) in [
            (0usize, self.left_spinner.clone(), LEFT_SPINNER_X),
            (1usize, self.right_spinner.clone(), RIGHT_SPINNER_X),
        ] {
            if !sp.active {
                continue;
            }
            let bracket = if side == 0 {
                "anim_spinner_bracket_left"
            } else {
                "anim_spinner_bracket_right"
            };
            let wheel = if side == 0 {
                "anim_spinner_1"
            } else {
                "anim_spinner_2"
            };
            let raw_f = ((sp.frame_accum_q8 >> 8) % SPINNER_ANIM_FRAMES) as usize;
            let frame = if sp.forward {
                raw_f
            } else {
                (SPINNER_ANIM_FRAMES as usize - 1) - raw_f
            };

            self.video
                .draw_named_sprite(SpriteDrawCmd::new(bracket, 0, x, SPINNER_Y, 22));
            if sp.velocity_q8 > 0 {
                self.video
                    .draw_named_sprite(SpriteDrawCmd::new(wheel, frame, x, SPINNER_Y, 22));
                if sp.high_speed {
                    self.video.draw_named_sprite(SpriteDrawCmd::new(
                        "anim_spinner_lightning",
                        raw_f % 22,
                        x + 6,
                        SPINNER_Y - 12,
                        22,
                    ));
                }
            }

            if sp.popup_ms > 0 && sp.last_award > 0 {
                let lift = ((900 - sp.popup_ms.min(900)) / 30) as i32;
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_system_7_data".into(),
                    text: format!("+{}", sp.last_award),
                    x: x + 75,
                    y: (SPINNER_Y - 14 - lift).max(46),
                    z: 23,
                    color_rgb555: 0x7FE0,
                    shadow_rgb555: Some(0x0000),
                    align: TextAlign::Center,
                });
            }
        }
    }
}
