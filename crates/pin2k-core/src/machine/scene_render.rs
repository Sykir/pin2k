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
    pub(crate) fn scene_sprite(&mut self, name: &str, frame: usize, x: i32, y: i32, z: u8) {
        self.video
            .draw_named_sprite(SpriteDrawCmd::new(name, frame, x, y, z));
    }

    pub(crate) fn scene_text(
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
                    self.render_battle_droid(secs);
                }
                Swe1Scene::DestroyerDroid => {
                    self.render_destroyer_droid(secs);
                }
                Swe1Scene::FederationFighter => {
                    self.render_federation_fighter(secs);
                }
                Swe1Scene::GunganBattle => {
                    self.render_gungan_battle(secs);
                }
                Swe1Scene::HangarEscape => {
                    self.render_hangar_escape(secs);
                }
                Swe1Scene::HoverTank => {
                    self.render_hover_tank(secs);
                }
                Swe1Scene::Podrace => {
                    self.render_podrace(secs);
                }
                Swe1Scene::ProbeDroid => {
                    self.render_probe_droid(secs);
                }
                Swe1Scene::QueenFashion => {
                    self.render_queen_fashion(secs);
                }
                Swe1Scene::JarJarJuggle => {
                    self.render_jarjar_juggle(secs);
                }
                Swe1Scene::WattosChance => {
                    self.render_wattos_chance(secs);
                }
                Swe1Scene::MosEspaCantina => {
                    self.render_mos_espa_cantina(secs);
                }
                Swe1Scene::R2D2 => {
                    self.render_r2d2(secs);
                }
                Swe1Scene::QuestionMark => {
                    self.render_question_mark(secs);
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
                    "LASER BUTTONS CHANGE - CENTER STARTS",
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

    pub(crate) fn render_scene_footer(&mut self, hits: u8, secs: u32, label: &str) {
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
