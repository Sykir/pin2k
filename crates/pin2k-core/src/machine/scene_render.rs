//! Authentic Pure-Rust Compositors for all 14 SWE1 Holoprojector Scenes
//! (`Deff*SceneIntro` `z=200..202` + active `Deff*SceneBackground` `z=1..8`),
//! Obi-Wan Holo Select (`DeffObiHolo`), Hangar Multiball, Darth Maul Duel
//! (`DeffJediMultiballBackground`), and Super Skill Shot.

use pin2k_hal::{
    AudioBackend, PinballHardware, RectDrawCmd, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend,
};

use super::Pin2kMachine;
use crate::swe1_rules::{ActiveMultiball, Swe1Scene};

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    fn scene_sprite(&mut self, name: &str, frame: usize, x: i32, y: i32, z: u8) {
        self.video
            .draw_named_sprite(SpriteDrawCmd::new(name, frame, x, y, z));
    }

    fn scene_text(
        &mut self,
        font: &str,
        text: impl Into<String>,
        x: i32,
        y: i32,
        z: u8,
        color_rgb555: u16,
        shadow_rgb555: Option<u16>,
        align: TextAlign,
    ) {
        self.video.draw_font_text(TextDrawCmd {
            font_name: font.to_string(),
            text: text.into(),
            x,
            y,
            z,
            color_rgb555,
            shadow_rgb555,
            align,
        });
    }

    /// Renders active SWE1 scenes (intros + all 14 backgrounds), multiballs, holo-select, or skill shot.
    pub(super) fn render_swe1_ingame_scene(&mut self) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;

        if let Some(sc) = self.active_scene.clone() {
            if sc.intro_remaining_ms > 0 {
                // Phase 1 — Authentic Scene Intro (`Deff*SceneIntro`), cleanly isolated from gameplay:
                // 1. Opaque black backdrop at z=0 so no legacy/background mode bleeds through.
                self.video.draw_rect(RectDrawCmd {
                    x: 0,
                    y: 0,
                    w: 640,
                    h: 240,
                    z: 0,
                    color_rgb555: 0x0000,
                    alpha: 255,
                });
                // 2. Intro movie (220x160 doubled horizontally -> 440x160) centered inside the
                //    434x157 oval aperture of `anim_jedi_scene_intro_frame` (522x195 at (59, 14)).
                let intro_total = sc.scene.intro_duration_ms().max(sc.intro_remaining_ms);
                let intro_elapsed_ms = intro_total.saturating_sub(sc.intro_remaining_ms) as u64;
                let intro_frame = ((intro_elapsed_ms * 15) / 1000) as usize;
                let mut movie = SpriteDrawCmd::new(
                    sc.scene.intro_anim_hint(),
                    intro_frame,
                    100,
                    31,
                    200,
                );
                movie.scale_x = 2;
                self.video.draw_named_sprite(movie);
                // 3. Double-oval metallic bezel on top of the movie edges.
                self.scene_sprite("anim_jedi_scene_intro_frame", 0, 59, 14, 202);
                // 4. Scene badge horizontally centered on x=320 across the lower rim (y=14+0x7a=136),
                //    matching Ghidra `DeffDestroyerDroidSceneIntro::Run()`.
                let badge_x = 320 - (sc.scene.intro_badge_width() / 2);
                self.scene_sprite(sc.scene.intro_badge_anim(), 0, badge_x, 136, 203);
                self.scene_text(
                    "font_shadow_11_data",
                    "GET READY - SHOOT FLASHING SHOTS!",
                    320,
                    224,
                    204,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
                return;
            }

            let secs = sc.timer_ms.div_ceil(1000);
            let hits = sc.hits_remaining;
            match sc.scene {
                Swe1Scene::BattleDroid => {
                    self.scene_sprite("anim_bdroid_background", 0, 0, 0, 1);
                    self.scene_sprite("anim_left_bdroid_standing", f60, 40, 80, 3);
                    self.scene_sprite("anim_right_bdroid_standing", f60, 520, 80, 3);
                    self.scene_sprite("anim_bdroid_on_stap", f60, 238, 68, 4);
                    self.scene_sprite("anim_stap_xhair", f60, 275, 76, 5);
                    if (f60 % 48) < 18 {
                        self.scene_sprite("anim_bdroid_stap_zap", f60, 280, 96, 6);
                    }
                    self.render_scene_footer(hits, secs, "DROIDS LEFT");
                }
                Swe1Scene::DestroyerDroid => {
                    self.scene_sprite("anim_destroyer_droid_background", 0, 0, 0, 1);
                    self.scene_sprite("anim_destroyer_droid_s_idle", f60, 80, 92, 3);
                    self.scene_sprite("anim_destroyer_droid_m_idle", f60, 430, 82, 3);
                    self.scene_sprite("anim_destroyer_droid_l_roll", f60, 20, 62, 5);
                    self.render_scene_footer(hits, secs, "DROIDEKAS");
                }
                Swe1Scene::FederationFighter => {
                    let scroll_x = -((f60 % 260) as i32);
                    self.scene_sprite("anim_federation_fighter_background", 0, scroll_x, 0, 1);
                    self.scene_sprite("anim_droid_control_ship", 0, 120, 8, 2);
                    self.scene_sprite("anim_federation_fighter", f60, 220, 74, 4);
                    self.scene_sprite("anim_naboo_xhair", 0, 245, 78, 5);
                    self.render_scene_footer(hits, secs, "FIGHTERS");
                }
                Swe1Scene::GunganBattle => {
                    self.scene_sprite("anim_gungan_seaweed", f60, 0, 40, 1);
                    self.scene_sprite("anim_gungan_sub_bubbles_2", f60, 35, 0, 2);
                    self.scene_sprite("anim_gungan_sub_sub", f60, 80, 84, 4);
                    self.scene_sprite("anim_gungan_sub_opee", f60, 220, 44, 5);
                    self.scene_text(
                        "font_shadow_9_narrow_data",
                        "G U N G A N",
                        540,
                        80,
                        7,
                        0x7FE0,
                        Some(0x0000),
                        TextAlign::Center,
                    );
                    self.render_scene_footer(hits, secs, "BATTLE HITS");
                }
                Swe1Scene::HangarEscape => {
                    self.scene_sprite("anim_federation_fighter_cockpit", 0, 0, 0, 1);
                    self.scene_sprite("anim_hangar_droid_run", f60, 275, 48, 3);
                    self.scene_sprite("anim_hangar_escape_torpedo", f60, 220, 80, 4);
                    self.scene_sprite("anim_hangar_droid_explosion", f60, 140, 42, 5);
                    self.scene_sprite("anim_hangar_droid_laser_explosion", f60, 220, 40, 6);
                    self.render_scene_footer(hits, secs, "TARGETS");
                }
                Swe1Scene::HoverTank => {
                    self.scene_sprite("anim_hover_tank_background", 0, 0, 0, 1);
                    self.scene_sprite("anim_hover_tank_l_float", f60, 60, 72, 3);
                    self.scene_sprite("anim_hover_tank_r_float", f60, 440, 72, 3);
                    self.scene_sprite("anim_hover_tank_c_float", f60, 233, 66, 4);
                    self.scene_sprite("anim_hover_tank_catapult_idle", f60, 456, 120, 5);
                    self.scene_sprite("anim_hover_tank_boomer_ball", f60, 360, 115, 6);
                    self.render_scene_footer(hits, secs, "TANKS");
                }
                Swe1Scene::Podrace => {
                    self.render_podrace_layers(sc.active_ms);
                    let lap = 4u8.saturating_sub(hits).clamp(1, 3);
                    self.scene_text(
                        "font_shadow_11_data",
                        format!("LAP {lap}/3"),
                        12,
                        90,
                        8,
                        0x7FE0,
                        Some(0x0000),
                        TextAlign::Left,
                    );
                    self.scene_text(
                        "font_shadow_11_data",
                        format!("{secs}S"),
                        628,
                        72,
                        8,
                        0x7FE0,
                        Some(0x0000),
                        TextAlign::Right,
                    );
                }
                Swe1Scene::ProbeDroid => {
                    let scroll_x = -((f60 % 160) as i32);
                    self.scene_sprite("anim_sith_probe_background", 0, scroll_x, 0, 1);
                    self.scene_sprite("anim_sith_probe_hover", f60, 210, 44, 4);
                    self.scene_sprite("anim_sith_probe_queen_shoot_1", f60, 40, 84, 5);
                    self.scene_sprite("anim_sith_probe_xhair", f60, 240, 56, 6);
                    self.render_scene_footer(hits, secs, "PROBES");
                }
                Swe1Scene::QueenFashion => {
                    self.scene_sprite("anim_fashion_base", 0, 210, 205, 2);
                    self.scene_sprite("anim_fashion_throne", f60, 190, 40, 3);
                    self.scene_sprite("anim_fashion_queen", f60, 210, 44, 4);
                    self.scene_sprite("anim_fashion_wheel", f60, 40, 46, 5);
                    self.scene_sprite("anim_fashion_sparkle_left", f60, 130, 60, 6);
                    self.scene_sprite("anim_fashion_sparkle_right", f60, 420, 60, 6);
                    self.render_scene_footer(hits, secs, "OUTFITS");
                }
                Swe1Scene::JarJarJuggle => {
                    self.render_jarjar_juggle(secs);
                }
                Swe1Scene::WattosChance => {
                    self.scene_sprite("anim_watto_scene_background", 0, 0, 0, 1);
                    self.scene_sprite("anim_watto_scene_chin_loop_middle", f60, 233, 44, 3);
                    self.scene_sprite("anim_watto_scene_cube", f60, 257, 120, 5);
                    self.scene_sprite("anim_watto_scene_cube_flash", f60, 226, 108, 6);
                    self.scene_text(
                        "font_shadow_7_data",
                        format!("SHOOT CUBE!  HITS LEFT: {hits}   {secs}S"),
                        320,
                        208,
                        7,
                        0x7FE0,
                        Some(0x0000),
                        TextAlign::Center,
                    );
                }
                Swe1Scene::MosEspaCantina => {
                    self.scene_sprite("anim_jedi_council_background", 0, 23, 4, 1);
                    self.scene_sprite("anim_jedi_council_mace", f60, 40, 86, 3);
                    self.scene_sprite("anim_jedi_council_yoda", f60, 500, 92, 3);
                    self.scene_sprite("anim_jedi_council_member_1", f60, 170, 84, 4);
                    self.scene_sprite("anim_jedi_council_member_2", f60, 370, 84, 4);
                    self.scene_sprite("anim_jedi_council_neck", f60, 275, 78, 5);
                    self.scene_text(
                        "font_shadow_9_data",
                        format!("JEDI COUNCIL: {hits} LEFT   {secs}S"),
                        320,
                        206,
                        7,
                        0x7FFF,
                        Some(0x0000),
                        TextAlign::Center,
                    );
                }
                Swe1Scene::R2D2 => {
                    self.scene_sprite("anim_r2d2_scene_background_upper", 0, 0, 0, 1);
                    self.scene_sprite("anim_r2d2_scene_background_lower", 0, 0, 109, 2);
                    self.scene_sprite("anim_r2d2_scene_r2d2_unit", f60, 225, 105, 4);
                    self.scene_sprite("anim_r2d2_scene_blue_yellow_r2_unit", f60, 25, 135, 5);
                    self.scene_sprite("anim_r2d2_scene_red_r2_unit", f60, 410, 130, 5);
                    self.scene_text(
                        "font_shadow_11_narrow_data",
                        format!("REPAIR SHIELD!  {hits} LEFT  ({secs}S)"),
                        320,
                        214,
                        7,
                        0x7E00,
                        Some(0x0000),
                        TextAlign::Center,
                    );
                }
                Swe1Scene::QuestionMark => {
                    self.scene_sprite("anim_question_mark_blue", f60, 140, 88, 4);
                    self.scene_sprite("anim_question_mark_green", f60, 378, 88, 4);
                    self.scene_text(
                        "font_shadow_15_data",
                        "MYSTERY AWARD",
                        320,
                        62,
                        6,
                        0x7FE0,
                        Some(0x001F),
                        TextAlign::Center,
                    );
                    self.scene_text(
                        "font_shadow_11_data",
                        format!("HITS LEFT: {hits}   TIME: {secs}S"),
                        320,
                        174,
                        7,
                        0x7FFF,
                        Some(0x0000),
                        TextAlign::Center,
                    );
                }
            }
            self.render_scene_extras(sc.scene, sc.active_ms, hits);
            return;
        }

        // Multiballs (`HangarMultiball` and `DarthMaulDuel`)
        match self.multiball {
            ActiveMultiball::HangarMultiball {
                jackpots_lit,
                jackpots_collected,
            } => {
                if self.special_screen_ms < 3_000 {
                    let mut start_cmd = SpriteDrawCmd::new("anim_multiball_start_movie", f60, 0, 0, 2);
                    start_cmd.scale_x = 2;
                    self.video.draw_named_sprite(start_cmd);
                    self.scene_text(
                        "font_shadow_22_data",
                        "MULTIBALL!",
                        320,
                        110,
                        7,
                        0x7FE0,
                        Some(0x7C00),
                        TextAlign::Center,
                    );
                } else {
                    let icon = if jackpots_collected >= 2 {
                        "anim_jackpot_icon_super"
                    } else if jackpots_collected == 1 {
                        "anim_jackpot_icon_double"
                    } else {
                        "anim_jackpot_icon_idle"
                    };
                    self.scene_sprite(icon, f60, 240, 78, 4);
                    let status = if jackpots_lit {
                        format!("JACKPOT LIT!   COLLECTED: {jackpots_collected}")
                    } else {
                        format!("JACKPOTS COLLECTED: {jackpots_collected}")
                    };
                    self.scene_text(
                        "font_shadow_11_narrow_data",
                        status,
                        320,
                        164,
                        7,
                        0x7FE0,
                        Some(0x0000),
                        TextAlign::Center,
                    );
                }
                return;
            }
            ActiveMultiball::DarthMaulDuel { maul_health, .. } => {
                self.scene_sprite("anim_jedi_battle_background_hangar", 0, 0, 0, 1);
                self.scene_sprite("anim_jedi_battle_icon_quigon", f60, 10, 48, 4);
                self.scene_sprite("anim_jedi_battle_icon_maul", f60, 540, 48, 4);
                let duel_anim = if self.active_anim_hint.starts_with("anim_jedi_battle_") {
                    self.active_anim_hint.clone()
                } else {
                    "anim_jedi_battle_idle".to_string()
                };
                self.scene_sprite(&duel_anim, f60, 122, 42, 5);
                if duel_anim == "anim_jedi_battle_clash" {
                    self.scene_sprite("anim_jedi_battle_saber_sparks", f60, 188, 48, 6);
                }
                let max_hp = 12i32;
                let hp = (maul_health as i32).clamp(0, max_hp);
                self.video.draw_rect(RectDrawCmd {
                    x: 200,
                    y: 44,
                    w: 240,
                    h: 8,
                    z: 6,
                    color_rgb555: 0x2108,
                    alpha: 220,
                });
                self.video.draw_rect(RectDrawCmd {
                    x: 202,
                    y: 45,
                    w: (236 * hp) / max_hp,
                    h: 6,
                    z: 7,
                    color_rgb555: 0x7C00,
                    alpha: 255,
                });
                self.scene_text(
                    "font_shadow_9_narrow_data",
                    format!("DARTH MAUL HP: {hp}/{max_hp}"),
                    320,
                    56,
                    7,
                    0x7FE8,
                    Some(0x0000),
                    TextAlign::Center,
                );
                return;
            }
            ActiveMultiball::None => {}
        }

        // Authentic `DeffObiHolo` nominal background & shield-defeat hologram sequence:
        // 1. Shield up (`!scene_select_ready`): `anim_obiwan_holo` (flashes `anim_obiwan_holo_glow` on hit)
        // 2. Shield defeated (`scene_select_ready`):
        //    - `0..900ms` (30 fr @ 33fps): `anim_obiwan_holo_flatten`
        //    - `900..1500ms` (18 fr @ 30fps): `anim_obiwan_holo_activate` + `anim_holo_burst`
        //    - `>= 1500ms`: `anim_obiwan_holo_glow_loop` with the selectable mini-game hologram above it
        if self.special_screen.is_none() {
            let p = &self.players[self.current_player];
            let ready = p.scene_select_ready;
            let sc = Swe1Scene::ALL[p.next_scene_idx % Swe1Scene::ALL.len()];

            if !ready {
                let holo_anim = if self.shield_hit_glow_ms > 0 {
                    "anim_obiwan_holo_glow"
                } else {
                    "anim_obiwan_holo"
                };
                let holo_frame = ((self.elapsed_ms / 30) as usize) % 60;
                // Ghidra `DeffObiHolo::Run`: x = 0x11d - 320/2 = 125, y = 0xf0 - 160 = 80
                self.scene_sprite(holo_anim, holo_frame, 125, 80, 3);
            } else if self.holo_transition_ms < 900 {
                let flatten_f = ((self.holo_transition_ms / 30) as usize).min(29);
                self.scene_sprite("anim_obiwan_holo_flatten", flatten_f, 125, 80, 3);
            } else if self.holo_transition_ms < 1_500 {
                let act_f = (((self.holo_transition_ms - 900) / 33) as usize).min(17);
                // Ghidra: activate x = 0x122 - 255/2 = 162, y = 0xf0 - 81 = 159
                // burst x = 0x11d - 450/2 = 60, y = 0x78 - 225/2 = 8
                self.scene_sprite("anim_obiwan_holo_activate", act_f, 162, 159, 4);
                self.scene_sprite("anim_holo_burst", act_f, 60, 8, 5);
            } else {
                let loop_f = ((self.elapsed_ms / 33) as usize) % 15;
                // Ghidra: glow_loop x = 0x122 - 222/2 = 179, y = 0xeb - 78 = 157
                self.scene_sprite("anim_obiwan_holo_glow_loop", loop_f, 179, 157, 4);
                // Selectable mini-game hologram floating above the glowing projector base
                self.scene_sprite(sc.holo_anim_hint(), f60, 190, 44, 5);
                self.scene_text(
                    "font_system_7_data",
                    format!("SELECT SCENE: {}", sc.title()),
                    320,
                    206,
                    7,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
                self.scene_text(
                    "font_system_7_data",
                    "FLIPPERS CHANGE - CENTER STARTS",
                    320,
                    222,
                    7,
                    0x03FF,
                    Some(0x0000),
                    TextAlign::Center,
                );
                return;
            }
        }

        // Skill Shot (`anim_super_skill_shot_icon`)
        if self.skill_shot_ready
            && self.special_screen.is_none()
            && self.active_anim_hint == "anim_super_skill_shot_icon"
        {
            self.scene_sprite("anim_super_skill_shot_icon", f60, 240, 68, 4);
            self.scene_text(
                "font_shadow_11_data",
                "SUPER SKILL SHOT - SHOOT JETS!",
                320,
                188,
                7,
                0x7FE0,
                Some(0x0000),
                TextAlign::Center,
            );
        }
    }

    fn render_scene_footer(&mut self, hits: u8, secs: u32, label: &str) {
        self.scene_text(
            "font_shadow_9_narrow_data",
            format!("{label}: {hits}   TIME: {secs}S"),
            320,
            216,
            7,
            0x7FE0,
            Some(0x0000),
            TextAlign::Center,
        );
    }
}
