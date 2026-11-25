//! Authentic 20-slide Attract Mode renderer (`AttractProcInfo` @ `0x002d94d0` + `DeffAttractModePriceBackground`).
//!
//! All coordinates, fonts, RGB555 colors, and Z-orders are lifted directly from `swe1_0200.elf`:
//! - `DeffAttractModeKahunaLogo` (`0x002b517c`): `anim_kahuna_logo_spinning` `(187, 80, z=8)` +
//!   `anim_kahuna_logo_flare` `(23, 7, z=7)` with 9-step RGB555 color cycling (`0x100fc0`).
//! - `DeffAttractModePriceBackground` (`0x001008c0`): `font_system_9_data`, `z=250`, blinking `y=210/225`.
//! - All 20 `AttractDeff` variants (`Hstd`, `JediTable*`, `RampComboChampion`, `P2kLogo`, `HasbroAd`,
//!   `ScoreAwards`, `EmailAddress`, `WebPage`, `Scores`, `GameCredits1..4`, `CustomMessage`, `HtpScene`).

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};

use super::Pin2kMachine;
use crate::attract::AttractDeff;
use crate::swe1_rules::{format_score_commas, JediRank};

/// Exact 9 RGB555 flare colors from `DeffAttractModeKahunaLogo::Run` (`0x100fc0`).
pub const KAHUNA_FLARE_COLORS: [u16; 9] = [
    0x7C06, 0x0320, 0x001F, 0x0320, 0x001F, 0x7C06, 0x0320, 0x7C06, 0x001F,
];

fn lerp_rgb555(a: u16, b: u16, t: u32, steps: u32) -> u16 {
    let ar = (a >> 10) & 0x1F;
    let ag = (a >> 5) & 0x1F;
    let ab = a & 0x1F;
    let br = (b >> 10) & 0x1F;
    let bg = (b >> 5) & 0x1F;
    let bb = b & 0x1F;
    let inv = steps.saturating_sub(t);
    let r = (ar as u32 * inv + br as u32 * t) / steps;
    let g = (ag as u32 * inv + bg as u32 * t) / steps;
    let bl = (ab as u32 * inv + bb as u32 * t) / steps;
    (((r & 0x1F) << 10) | ((g & 0x1F) << 5) | (bl & 0x1F)) as u16
}

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(super) fn render_swe1_attract_frame(&mut self) {
        let cur = self.attract.current();
        let t_ms = self.attract.timer_ms;
        let f60 = (t_ms * 60 / 1000) as usize;
        let f15 = (t_ms * 15 / 1000) as usize;
        let f12 = (t_ms * 12 / 1000) as usize;

        match cur {
            AttractDeff::KahunaLogo => {
                // Smooth 32-step RGB555 interpolation across the 9 flare colors (`0x2b5340..0x2b5484`)
                let step_ms = 50u32;
                let total_steps = (t_ms / step_ms) as usize;
                let idx0 = (total_steps / 32) % KAHUNA_FLARE_COLORS.len();
                let idx1 = (idx0 + 1) % KAHUNA_FLARE_COLORS.len();
                let frac = (total_steps % 32) as u32;
                let flare_color = lerp_rgb555(KAHUNA_FLARE_COLORS[idx0], KAHUNA_FLARE_COLORS[idx1], frac, 32);

                let mut flare = SpriteDrawCmd::new("anim_kahuna_logo_flare", f12 % 17, 23, 7, 7);
                flare.monochrome_rgb555 = Some(flare_color);
                self.video.draw_named_sprite(flare);

                // `anim_kahuna_logo_spinning` (`267x81`) centered at `(320, 120)` -> `(187, 80)`
                self.video
                    .draw_named_sprite(SpriteDrawCmd::new("anim_kahuna_logo_spinning", 0, 187, 80, 8));
            }
            AttractDeff::SlideShow => {
                let banner_idx = (t_ms / 1_250) % 4;
                let anim = match banner_idx {
                    0 => "anim_starwars_banner_1",
                    1 => "anim_starwars_banner_2",
                    2 => "anim_starwars_banner_3",
                    _ => "anim_starwars_banner_4",
                };
                let pan = -(((t_ms % 1_250) * 80 / 1_250) as i32);
                self.video.draw_named_sprite(SpriteDrawCmd::new(anim, 0, pan, 0, 7));
            }
            AttractDeff::Hstd => {
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_starwars_19_data".into(),
                    text: "HIGHEST SCORES".into(),
                    x: 320,
                    y: 12,
                    z: 8,
                    color_rgb555: 0x7E00,
                    shadow_rgb555: Some(0x3DEF),
                    align: TextAlign::Center,
                });
                for (i, hs) in self.high_scores.iter().take(4).enumerate() {
                    let ini: String = hs.initials.iter().collect();
                    let y = 58 + (i as i32) * 34;
                    self.video.draw_font_text(TextDrawCmd {
                        font_name: "font_shadow_15_data".into(),
                        text: format!("#{}  {}  ({})", i + 1, ini, hs.rank.label()),
                        x: 24,
                        y,
                        z: 8,
                        color_rgb555: 0x7E00,
                        shadow_rgb555: Some(0x3DEF),
                        align: TextAlign::Left,
                    });
                    self.video.draw_font_text(TextDrawCmd {
                        font_name: "font_shadow_15_data".into(),
                        text: format_score_commas(hs.score),
                        x: 616,
                        y,
                        z: 8,
                        color_rgb555: 0x7FFF,
                        shadow_rgb555: Some(0x5294),
                        align: TextAlign::Right,
                    });
                }
            }
            AttractDeff::JediTableYouth
            | AttractDeff::JediTableKnight
            | AttractDeff::JediTableMaster
            | AttractDeff::JediTableSpirit => {
                let (rank, icon_anim) = match cur {
                    AttractDeff::JediTableYouth => (JediRank::Youth, "anim_bonus_icon_jedi_youth"),
                    AttractDeff::JediTableKnight => (JediRank::Knight, "anim_bonus_icon_jedi_knight"),
                    AttractDeff::JediTableMaster => (JediRank::Master, "anim_bonus_icon_jedi_master"),
                    _ => (JediRank::Spirit, "anim_bonus_icon_jedi_spirit"),
                };
                self.video
                    .draw_named_sprite(SpriteDrawCmd::new(icon_anim, f60 % 47, 20, 14, 9));
                self.video
                    .draw_named_sprite(SpriteDrawCmd::new(icon_anim, (f60 + 23) % 47, 480, 14, 9));
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_starwars_19_data".into(),
                    text: rank.label().to_string(),
                    x: 320,
                    y: 26,
                    z: 8,
                    color_rgb555: 0x03FF,
                    shadow_rgb555: Some(0x0015),
                    align: TextAlign::Center,
                });
                let champs: Vec<_> = self
                    .high_scores
                    .iter()
                    .filter(|h| h.rank >= rank)
                    .take(3)
                    .collect();
                for (i, hs) in champs.iter().enumerate() {
                    let ini: String = hs.initials.iter().collect();
                    self.video.draw_font_text(TextDrawCmd {
                        font_name: "font_shadow_11_narrow_data".into(),
                        text: format!("{}  -  {}", ini, format_score_commas(hs.score)),
                        x: 320,
                        y: 92 + (i as i32) * 28,
                        z: 8,
                        color_rgb555: 0x7FFF,
                        shadow_rgb555: Some(0x294A),
                        align: TextAlign::Center,
                    });
                }
                if champs.is_empty() {
                    self.video.draw_font_text(TextDrawCmd {
                        font_name: "font_shadow_11_narrow_data".into(),
                        text: "DEFEAT DARTH MAUL TO QUALIFY".into(),
                        x: 320,
                        y: 112,
                        z: 8,
                        color_rgb555: 0x7E00,
                        shadow_rgb555: Some(0x294A),
                        align: TextAlign::Center,
                    });
                }
            }
            AttractDeff::RampComboChampion => {
                self.video
                    .draw_named_sprite(SpriteDrawCmd::new("anim_spinner_1", f60 % 56, 480, 40, 7));
                self.video
                    .draw_named_sprite(SpriteDrawCmd::new("anim_spinner_2", f60 % 56, 480, 140, 7));
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_15_data".into(),
                    text: "RAMP COMBO CHAMPION".into(),
                    x: 320,
                    y: 36,
                    z: 8,
                    color_rgb555: 0x7E00,
                    shadow_rgb555: Some(0x3DEF),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_starwars_19_data".into(),
                    text: "QGJ  -  12 RAMPS".into(),
                    x: 320,
                    y: 108,
                    z: 8,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x5294),
                    align: TextAlign::Center,
                });
            }
            AttractDeff::P2kLogo => {
                self.video.draw_named_sprite(SpriteDrawCmd::new(
                    "anim_p2k_logo_animated",
                    f15.min(89),
                    220,
                    16,
                    7,
                ));
            }
            AttractDeff::HasbroAd => {
                self.video
                    .draw_named_sprite(SpriteDrawCmd::new("anim_attract_hasbro_ad", 0, 0, 0, 7));
            }
            AttractDeff::ScoreAwards => {
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_15_data".into(),
                    text: "REPLAY AT 15,000,000".into(),
                    x: 320,
                    y: 54,
                    z: 8,
                    color_rgb555: 0x7E00,
                    shadow_rgb555: Some(0x3DEF),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_15_data".into(),
                    text: "EXTRA BALL AT 4 SCENES".into(),
                    x: 320,
                    y: 102,
                    z: 8,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x294A),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_11_narrow_data".into(),
                    text: "SPECIAL AT JEDI SPIRIT".into(),
                    x: 320,
                    y: 150,
                    z: 8,
                    color_rgb555: 0x03FF,
                    shadow_rgb555: Some(0x0015),
                    align: TextAlign::Center,
                });
            }
            AttractDeff::EmailAddress => {
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_starwars_19_data".into(),
                    text: "PINBALL 2000".into(),
                    x: 320,
                    y: 56,
                    z: 8,
                    color_rgb555: 0x7E00,
                    shadow_rgb555: Some(0x3DEF),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_11_narrow_data".into(),
                    text: "FEEDBACK@PINBALL.COM".into(),
                    x: 320,
                    y: 120,
                    z: 8,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x294A),
                    align: TextAlign::Center,
                });
            }
            AttractDeff::WebPage => {
                self.video.draw_named_sprite(SpriteDrawCmd::new(
                    "anim_attract_anakin_web",
                    0,
                    219,
                    66,
                    7,
                ));
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_11_narrow_data".into(),
                    text: "VISIT OUR WEBSITE".into(),
                    x: 320,
                    y: 26,
                    z: 8,
                    color_rgb555: 0x3DEF,
                    shadow_rgb555: Some(0x18C6),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_11_narrow_data".into(),
                    text: "WWW.PINBALL.COM".into(),
                    x: 320,
                    y: 176,
                    z: 8,
                    color_rgb555: 0x7CA5,
                    shadow_rgb555: Some(0x3C00),
                    align: TextAlign::Center,
                });
            }
            AttractDeff::Scores => {
                self.render_score_display(false);
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_15_data".into(),
                    text: "LAST GAME SCORES".into(),
                    x: 320,
                    y: 96,
                    z: 8,
                    color_rgb555: 0x7E00,
                    shadow_rgb555: Some(0x3DEF),
                    align: TextAlign::Center,
                });
            }
            AttractDeff::GameCredits1
            | AttractDeff::GameCredits2
            | AttractDeff::GameCredits3
            | AttractDeff::GameCredits4 => {
                let (h, l1, l2) = match cur {
                    AttractDeff::GameCredits1 => (
                        "WILLIAMS ELECTRONICS",
                        "STAR WARS EPISODE I",
                        "PINBALL 2000 PLATFORM",
                    ),
                    AttractDeff::GameCredits2 => (
                        "LUCASFILM LTD.",
                        "JOHN POPADIUK - DESIGN",
                        "CAMERON SILVER - SOFTWARE",
                    ),
                    AttractDeff::GameCredits3 => (
                        "ART & ANIMATION",
                        "PAUL BARKER  -  DAVE MUELLER",
                        "ADAM RHINE  -  BRIAN MORRIS",
                    ),
                    _ => (
                        "SOUND & MUSIC",
                        "DAN FORDEN  -  CHRIS GRANNER",
                        "JOHN WILLIAMS SCORE",
                    ),
                };
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_15_data".into(),
                    text: h.into(),
                    x: 320,
                    y: 38,
                    z: 8,
                    color_rgb555: 0x7E00,
                    shadow_rgb555: Some(0x3DEF),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_11_narrow_data".into(),
                    text: l1.into(),
                    x: 320,
                    y: 92,
                    z: 8,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x294A),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_11_narrow_data".into(),
                    text: l2.into(),
                    x: 320,
                    y: 126,
                    z: 8,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x294A),
                    align: TextAlign::Center,
                });
            }
            AttractDeff::CustomMessage => {
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_15_data".into(),
                    text: "MAY THE FORCE BE WITH YOU".into(),
                    x: 320,
                    y: 96,
                    z: 8,
                    color_rgb555: 0x7E00,
                    shadow_rgb555: Some(0x3DEF),
                    align: TextAlign::Center,
                });
            }
            AttractDeff::HtpScene => {
                self.video.draw_named_sprite(SpriteDrawCmd::new(
                    "anim_battle_droid_intro_badge",
                    0,
                    168,
                    66,
                    7,
                ));
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_11_narrow_data".into(),
                    text: "SHOOT DEFLECTOR SHIELD".into(),
                    x: 320,
                    y: 26,
                    z: 8,
                    color_rgb555: 0x7E00,
                    shadow_rgb555: Some(0x3DEF),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_shadow_11_narrow_data".into(),
                    text: "ENTER HOLOPROJECTOR SCENES".into(),
                    x: 320,
                    y: 162,
                    z: 8,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x294A),
                    align: TextAlign::Center,
                });
            }
        }

        // `DeffAttractModePriceBackground` (`0x001008c0`, `font_system_9_data`, `z=250`, `y=210/225`)
        if (self.elapsed_ms / 500) % 2 == 0 {
            let line1 = if self.free_play {
                "FREE PLAY"
            } else if self.credits > 0 {
                "PRESS START"
            } else {
                "INSERT COIN"
            };
            let line2 = if self.free_play {
                "PRESS START"
            } else {
                ""
            };
            if !line2.is_empty() {
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_system_9_data".into(),
                    text: line1.into(),
                    x: 320,
                    y: 210,
                    z: 250,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x2108),
                    align: TextAlign::Center,
                });
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_system_9_data".into(),
                    text: line2.into(),
                    x: 320,
                    y: 225,
                    z: 250,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x2108),
                    align: TextAlign::Center,
                });
            } else {
                let txt = if self.credits > 0 {
                    format!("CREDITS {} - PRESS START", self.credits)
                } else {
                    line1.to_string()
                };
                self.video.draw_font_text(TextDrawCmd {
                    font_name: "font_system_9_data".into(),
                    text: txt,
                    x: 320,
                    y: 222,
                    z: 250,
                    color_rgb555: 0x7FFF,
                    shadow_rgb555: Some(0x2108),
                    align: TextAlign::Center,
                });
            }
        }
    }
}
