//! Authentic `VirtualSpinners` / `DeffVSpinner` (`0x0017ead0..0x0017fa04` in `swe1_0200.elf`):
//! - Left spinner at `(43, 174)` (`0x2b, 0xae`) `z = 22` (`anim_spinner_bracket_left` + `anim_spinner_1`).
//! - Right spinner at `(421, 174)` (`0x1a5, 0xae`) `z = 22` (`anim_spinner_bracket_right` + `anim_spinner_2`).
//! - `DeffVSpinner::Run`: resets to frame `0`, plays frames `0..=55` sequentially (never wrapping):
//!   `sleep1000(72..87 ms)` for frames `0..=40`, `sleep1000(40..47 ms)` for frames `41..=55`.
//! - Sound clicks (`acd_traditional_spinner_sound`) fire on frame `0` (trigger), `1..=8`, `9, 11, 13, 15, 17`, and `22`
//!   (15 total clicks = initial + 14 deceleration clicks), while frames `23..=55` coast silently to rest.

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};
use serde::{Deserialize, Serialize};

use super::Pin2kMachine;
use crate::swe1_rules::ActiveMultiball;
use crate::swe1_tables::sound;

pub const LEFT_SPINNER_X: i32 = 43;
pub const RIGHT_SPINNER_X: i32 = 421;
pub const SPINNER_Y: i32 = 174;
const SPINNER_ANIM_FRAMES: usize = 56;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VirtualSpinnerState {
    pub active: bool,
    pub forward: bool,
    pub frame: usize,
    pub step_ms: u32,
    pub total_spins: u32,
    pub sound_idx: usize,
    pub popup_ms: u32,
    pub last_award: u64,
    pub high_speed: bool,
}

impl VirtualSpinnerState {
    pub fn trigger(&mut self, forward: bool, award_per_click: u64) {
        self.active = true;
        self.forward = forward;
        self.frame = 0;
        self.step_ms = 0;
        self.high_speed = award_per_click >= 5_000;
        self.popup_ms = 1_100;
        self.last_award = award_per_click;
    }

    /// Authentic `DeffVSpinner::Run` (`0x17f5c7..0x17f5e3`) click frame schedule.
    fn is_click_frame(frame: usize) -> bool {
        matches!(frame, 1..=8 | 9 | 11 | 13 | 15 | 17 | 22)
    }

    /// Authentic `DeffVSpinner::Run` (`0x17f61d`, `0x17f664`, `0x17f911`) `sleep1000` delay.
    fn frame_delay_ms(&self) -> u32 {
        if self.frame <= 40 {
            78
        } else {
            44
        }
    }
}

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    /// Virtual spinners only exist during nominal SWE1 gameplay (no active scene, multiball, or special screen/overlay).
    pub(super) fn is_nominal_spinner_mode(&self) -> bool {
        self.state == super::MachineState::InGame
            && self.game_title == crate::GameTitle::Swe1
            && self.active_scene.is_none()
            && self.multiball == ActiveMultiball::None
            && self.special_screen.is_none()
            && self.overlay.is_none()
    }

    /// Triggers the left or right virtual spinner (called on ramp entry in nominal play only).
    /// Marks `spinner_value` (1,000..10,000, +1,000 per ramp entry within 9.6s).
    pub fn trigger_virtual_spinner(&mut self, right_side: bool, forward: bool) {
        if !self.is_nominal_spinner_mode() {
            return;
        }
        if self.spinner_combo_ms > 0 {
            self.spinner_value = (self.spinner_value + 1_000).min(10_000);
        } else {
            self.spinner_value = 1_000;
        }
        self.spinner_combo_ms = 9_600;
        let award = self.spinner_value;
        let sp = if right_side {
            &mut self.right_spinner
        } else {
            &mut self.left_spinner
        };
        sp.trigger(forward, award);
        let sid = sound::SPINNER_TICKS[sp.sound_idx % sound::SPINNER_TICKS.len()];
        sp.sound_idx = sp.sound_idx.wrapping_add(1);
        sp.total_spins = sp.total_spins.saturating_add(1);
        self.emit_sound(sid, false);
        self.add_score(award);
    }

    pub(super) fn update_virtual_spinners(&mut self, dt_ms: u32) {
        self.spinner_combo_ms = self.spinner_combo_ms.saturating_sub(dt_ms);
        if !self.is_nominal_spinner_mode() {
            self.left_spinner.active = false;
            self.right_spinner.active = false;
            return;
        }
        let award = self.spinner_value.max(1_000);
        for side in 0..2 {
            let mut clicks = 0u32;
            let mut first_sid = None;
            {
                let sp = if side == 0 {
                    &mut self.left_spinner
                } else {
                    &mut self.right_spinner
                };
                if !sp.active {
                    continue;
                }
                sp.step_ms = sp.step_ms.saturating_add(dt_ms);
                while sp.frame + 1 < SPINNER_ANIM_FRAMES {
                    let delay = sp.frame_delay_ms();
                    if sp.step_ms < delay {
                        break;
                    }
                    sp.step_ms -= delay;
                    sp.frame += 1;
                    if VirtualSpinnerState::is_click_frame(sp.frame) {
                        clicks += 1;
                        sp.total_spins = sp.total_spins.saturating_add(1);
                        let sid = sound::SPINNER_TICKS[sp.sound_idx % sound::SPINNER_TICKS.len()];
                        sp.sound_idx = sp.sound_idx.wrapping_add(1);
                        if first_sid.is_none() {
                            first_sid = Some(sid);
                        }
                        sp.last_award = award;
                        sp.popup_ms = 900;
                    }
                }
                sp.popup_ms = sp.popup_ms.saturating_sub(dt_ms);
                if sp.frame + 1 >= SPINNER_ANIM_FRAMES && sp.popup_ms == 0 {
                    sp.active = false;
                    sp.high_speed = false;
                }
            }
            if let Some(sid) = first_sid {
                self.emit_sound(sid, false);
                self.add_score(award * (clicks as u64));
            }
        }
    }

    /// Renders left/right virtual spinners (`z = 22`) and rising score popups (`z = 23`).
    /// Only rendered in nominal gameplay (no active scene, multiball, or special screen/overlay).
    pub fn render_virtual_spinners(&mut self) {
        if !self.is_nominal_spinner_mode() {
            return;
        }

        for (side, sp, x) in [
            (0usize, self.left_spinner.clone(), LEFT_SPINNER_X),
            (1usize, self.right_spinner.clone(), RIGHT_SPINNER_X),
        ] {
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

            self.video
                .draw_named_sprite(SpriteDrawCmd::new(bracket, 0, x, SPINNER_Y, 22));

            if sp.active {
                let frame = if sp.forward {
                    sp.frame.min(SPINNER_ANIM_FRAMES - 1)
                } else {
                    (SPINNER_ANIM_FRAMES - 1).saturating_sub(sp.frame)
                };
                self.video
                    .draw_named_sprite(SpriteDrawCmd::new(wheel, frame, x, SPINNER_Y, 22));
                if sp.high_speed && sp.frame < 22 {
                    self.video.draw_named_sprite(SpriteDrawCmd::new(
                        "anim_spinner_lightning",
                        sp.frame % 22,
                        x + 6,
                        SPINNER_Y - 12,
                        22,
                    ));
                }
            } else {
                // Resting static spinner wheel at frame 55 until a ramp pass triggers it
                self.video.draw_named_sprite(SpriteDrawCmd::new(
                    wheel,
                    SPINNER_ANIM_FRAMES - 1,
                    x,
                    SPINNER_Y,
                    22,
                ));
            }

            if sp.active && sp.popup_ms > 0 && sp.last_award > 0 {
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
