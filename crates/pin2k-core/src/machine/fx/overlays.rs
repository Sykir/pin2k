use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::OverlayKind;
use crate::machine::Pin2kMachine;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    // ------------------------------------------------------------------
    // Timed full-screen overlay Deffs
    // ------------------------------------------------------------------

    /// Renders the active overlay. Returns `true` when an overlay owns the screen.
    pub(crate) fn render_overlay(&mut self) -> bool {
        let Some(ov) = self.overlay else {
            return false;
        };
        let t = ov.elapsed_ms;
        let f60 = ((t as u64 * 60) / 1000) as usize;
        let f15 = ((t as u64 * 15) / 1000) as usize;
        self.video.draw_rect(pin2k_hal::RectDrawCmd {
            x: 0,
            y: 0,
            w: 640,
            h: 240,
            z: 0,
            color_rgb555: 0x0000,
            alpha: 255,
        });
        match ov.kind {
            OverlayKind::WattoJunkShop { award } => {
                self.fx_sprite("anim_watto_junkshop_sky", 0, 0, 0, 1, 1);
                self.fx_sprite("anim_watto_junkshop_background", 0, 0, 0, 2, 1);
                if t < 2_500 {
                    self.fx_sprite("anim_watto_cube_cube", f60, 186, 54, 4, 1);
                    self.fx_sprite("anim_watto_cube_flare", f60, 220, 60, 5, 1);
                } else {
                    let (img, x, y) = match award % 3 {
                        0 => ("anim_watto_cube", 257, 80),
                        1 => ("anim_oola_girls", 184, 20),
                        _ => ("anim_rayboot", 193, 30),
                    };
                    self.fx_sprite(img, f60, x, y, 4, 1);
                }
                let label = match award % 4 {
                    0 => "1,000,000 POINTS",
                    1 => "LIGHT EXTRA BALL",
                    2 => "3 LASERS",
                    _ => "BONUS X +2",
                };
                self.fx_text("font_shadow_15_data", "WATTO'S JUNK SHOP", 320, 6, 6, 0x7E00);
                self.fx_text("font_shadow_11_data", label, 320, 206, 6, 0x7FE0);
            }
            OverlayKind::Marketplace { award } => {
                self.fx_sprite("anim_mos_espa_background", 0, 0, 0, 1, 1);
                let chosen = (award % 4) as usize;
                let chubas = [
                    ("anim_mos_espa_chuba_1", 36, 50),
                    ("anim_mos_espa_chuba_2", 120, 44),
                    ("anim_mos_espa_chuba_3", 468, 40),
                    ("anim_mos_espa_chuba_4", 532, 40),
                ];
                for (i, (name, x, y)) in chubas.iter().enumerate() {
                    if i != chosen || t < 617 {
                        self.fx_sprite(name, f60, *x, *y, 2, 1);
                    }
                }
                self.fx_sprite("anim_mos_espa_pipes", 0, 0, 201, 3, 1);
                if t < 617 {
                    self.fx_sprite("anim_mos_espa_jarjar_choose", f60.min(36), 171, 34, 4, 1);
                } else if t < 850 {
                    let eat = [
                        "anim_mos_espa_jarjar_eat_1",
                        "anim_mos_espa_jarjar_eat_2",
                        "anim_mos_espa_jarjar_eat_3",
                        "anim_mos_espa_jarjar_eat_4",
                    ][chosen];
                    let fr = (((t - 617) as u64 * 60) / 1000) as usize;
                    self.fx_sprite(eat, fr.min(13), 120, 34, 4, 1);
                } else {
                    let fr = (((t - 850) as u64 * 60) / 1000) as usize;
                    self.fx_sprite("anim_mos_espa_jarjar_swallow", fr.min(25), 211, 34, 4, 1);
                }
                self.fx_text("font_shadow_15_data", "MOS ESPA MARKETPLACE", 320, 6, 6, 0x7E00);
            }
            OverlayKind::JediMultiballIntro => {
                if t < 1_467 {
                    self.fx_sprite("anim_jedi_letters_shrink", f60, 196, 79, 4, 1);
                } else {
                    let mt = (t - 1_467) as u64;
                    self.fx_sprite("anim_jedi_battle_intro_movie", ((mt * 15) / 1000) as usize, 90, 41, 2, 2);
                    self.fx_sprite("anim_jedi_battle_intro_frame", 0, 23, 12, 3, 1);
                    self.fx_sprite("anim_jedi_battle_intro_badge", 0, 97, 122, 4, 1);
                }
            }
            OverlayKind::RankPromotion => {
                self.fx_sprite("anim_firework_2", f60, 0, 0, 1, 1);
                self.fx_sprite("anim_firework_2", f60 + 20, 356, 0, 1, 1);
                if t < 1_200 {
                    self.fx_sprite("anim_jedi_battle_win_icon_quigon", 0, 128, 21, 3, 1);
                } else {
                    self.fx_sprite("anim_yoda_big", f15, 176, 4, 3, 1);
                }
                let rank = self
                    .players
                    .get(self.current_player)
                    .map(|p| p.jedi_rank.label())
                    .unwrap_or("JEDI");
                self.fx_text("font_starwars_19_data", format!("PROMOTED TO {rank}"), 320, 200, 6, 0x7FE0);
            }
            OverlayKind::JediSpirit => {
                if t < 1_833 {
                    self.fx_sprite("anim_jedi_spirit_jedi", f60, 0, 10, 2, 1);
                } else {
                    self.fx_sprite("anim_jedi_spirit_sidious", f60, 20, 20, 3, 1);
                    self.fx_sprite("anim_jedi_spirit_maul", f60, 374, 20, 3, 1);
                    self.fx_sprite("anim_jedi_spirit_electro1", f60, 40, 80, 4, 1);
                    self.fx_sprite("anim_jedi_spirit_electro2", f60, 393, 80, 4, 1);
                    self.fx_sprite("anim_jedi_spirit_sparks", f60, 141, 90, 5, 1);
                }
                self.fx_text("font_starwars_19_data", "JEDI SPIRIT", 320, 204, 6, 0x03FF);
            }
        }
        true
    }

}
