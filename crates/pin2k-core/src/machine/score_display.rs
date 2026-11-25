//! Authentic `DeffScoreDisplay` (`0x001725d8..0x001740fd` in `swe1_0200.elf`):
//! - 1..4 player scores at exact `Table<score_coord>` positions with Star Wars
//!   character names (`player_name_table_data`), `font_system_7_data`, and
//!   `font_kahuna_shadow_9_data`.
//! - Animated Left & Right Lightsabers (`saber_process` @ `y=7, z=9`, centered at `x=280`).
//! - Center info bar (`m_info_message_process` @ `x=280, y=9` and `y=26, z=10`).

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};

use super::Pin2kMachine;
use crate::swe1_rules::format_score_commas;

/// Exact `Table<score_coord>` entries lifted from `0x00173f66..0x001740dd`:
/// `(x, y, align)`.
pub const SCORE_COORDS: [(i32, i32, TextAlign); 4] = [
    (36, 7, TextAlign::Left),
    (525, 7, TextAlign::Right),
    (36, 26, TextAlign::Left),
    (525, 26, TextAlign::Right),
];

/// Saber animation triplets `(start, loop, flare)` from `saber_table` (`0x00173e88`).
const SABER_TRIPLETS: [(&str, &str, &str); 3] = [
    ("anim_saber_green_start", "anim_saber_green_loop", "anim_saber_green_flare"),
    ("anim_saber_red_start", "anim_saber_red_loop", "anim_saber_red_flare"),
    ("anim_saber_blue_start", "anim_saber_blue_loop", "anim_saber_blue_flare"),
];

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    /// Renders the authentic SWE1 top score bar (`DeffScoreDisplay`):
    /// players' Star Wars names + comma-formatted scores, animated lightsabers, and info bar.
    pub fn render_score_display(&mut self, show_sabers: bool) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let player_count = self.players.len().min(4);

        // 1. Lightsabers behind player names/scores (`z = 9`, `y = 7`, centered at `x = 280`)
        if show_sabers {
            let left_idx = self.left_saber_color % SABER_TRIPLETS.len();
            let right_idx = self.right_saber_color % SABER_TRIPLETS.len();

            let (l_anim, l_frame) = if self.saber_flare_ms > 0 {
                (SABER_TRIPLETS[left_idx].2, f60 % 10)
            } else if f60 < 4 {
                (SABER_TRIPLETS[left_idx].0, f60)
            } else {
                (SABER_TRIPLETS[left_idx].1, f60 % 10)
            };

            let (r_anim, r_frame) = if self.saber_flare_ms > 0 {
                (SABER_TRIPLETS[right_idx].2, (f60 + 3) % 10)
            } else if f60 < 4 {
                (SABER_TRIPLETS[right_idx].0, f60)
            } else {
                (SABER_TRIPLETS[right_idx].1, (f60 + 5) % 10)
            };

            // Left blade (`245x32`) at x=16, center handle (`39x32`) at x=261, right blade (`245x32`, flip_x) at x=300
            self.video
                .draw_named_sprite(SpriteDrawCmd::new(l_anim, l_frame, 16, 7, 9));
            let mut right_cmd = SpriteDrawCmd::new(r_anim, r_frame, 300, 7, 9);
            right_cmd.flip_x = true;
            self.video.draw_named_sprite(right_cmd);
            self.video
                .draw_named_sprite(SpriteDrawCmd::new("anim_saber_handle", 0, 261, 7, 9));
        }

        // 2. Player scores & Star Wars character names (`z = 10`)
        for (idx, p) in self.players.iter().take(player_count).enumerate() {
            let (base_x, base_y, align) = SCORE_COORDS[idx];
            let is_active = idx == self.current_player && show_sabers;
            let (fg, shadow) = if is_active {
                (0x7FFF_u16, Some(0x72E6_u16))
            } else {
                (0x294A_u16, Some(0x18C6_u16))
            };

            let name = p.character_name();
            let score_str = format_score_commas(p.score);

            match align {
                TextAlign::Left => {
                    // Name (`font_system_7_data`) left-aligned, Score (`font_kahuna_shadow_9_data`) right next to it
                    self.video.draw_font_text(TextDrawCmd {
                        font_name: "font_system_7_data".into(),
                        text: name.to_string(),
                        x: base_x,
                        y: base_y + 1,
                        z: 10,
                        color_rgb555: fg,
                        shadow_rgb555: shadow,
                        align: TextAlign::Left,
                    });
                    let name_px = (name.len() as i32) * 9 + 10;
                    self.video.draw_font_text(TextDrawCmd {
                        font_name: "font_kahuna_shadow_9_data".into(),
                        text: score_str,
                        x: base_x + name_px,
                        y: base_y,
                        z: 10,
                        color_rgb555: fg,
                        shadow_rgb555: shadow,
                        align: TextAlign::Left,
                    });
                }
                TextAlign::Right => {
                    // Right-aligned block ending at `base_x` (`525`)
                    let score_px = (score_str.len() as i32) * 10 + 10;
                    self.video.draw_font_text(TextDrawCmd {
                        font_name: "font_system_7_data".into(),
                        text: name.to_string(),
                        x: base_x - score_px,
                        y: base_y + 1,
                        z: 10,
                        color_rgb555: fg,
                        shadow_rgb555: shadow,
                        align: TextAlign::Right,
                    });
                    self.video.draw_font_text(TextDrawCmd {
                        font_name: "font_kahuna_shadow_9_data".into(),
                        text: score_str,
                        x: base_x,
                        y: base_y,
                        z: 10,
                        color_rgb555: fg,
                        shadow_rgb555: shadow,
                        align: TextAlign::Right,
                    });
                }
                TextAlign::Center => {}
            }
        }

        // 3. Center info messages (`m_info_message_process` @ `x = 280, y = 9` and `y = 26`, `z = 10`)
        let top_msg = format!("BALL {}", self.current_ball);
        let bot_msg = if self.free_play {
            "FREE PLAY".to_string()
        } else {
            format!("CREDITS {}", self.credits)
        };
        self.video.draw_font_text(TextDrawCmd {
            font_name: "font_system_7_data".into(),
            text: top_msg,
            x: 280,
            y: 9,
            z: 10,
            color_rgb555: 0x7E00,
            shadow_rgb555: Some(0x6540),
            align: TextAlign::Center,
        });
        self.video.draw_font_text(TextDrawCmd {
            font_name: "font_system_7_data".into(),
            text: bot_msg,
            x: 280,
            y: 26,
            z: 10,
            color_rgb555: 0x7E00,
            shadow_rgb555: Some(0x6540),
            align: TextAlign::Center,
        });
    }
}
