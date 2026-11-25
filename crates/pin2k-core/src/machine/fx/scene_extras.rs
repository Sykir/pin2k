use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use super::JUGGLE_ORBIT;
use crate::machine::Pin2kMachine;
use crate::swe1_rules::Swe1Scene;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    // ------------------------------------------------------------------
    // Extra authentic scene layers (`Deff*SceneBackground` owned animations)
    // ------------------------------------------------------------------

    /// Adds the remaining ROM-owned background layers for an active (post-intro) scene.
    pub(crate) fn render_scene_extras(&mut self, scene: Swe1Scene, active_ms: u32, hits: u8) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let a = active_ms;
        let req = scene.required_hits().max(1);
        let done = req.saturating_sub(hits);
        match scene {
            Swe1Scene::BattleDroid => {
                // Multi-Troop Transport frame + doors degrading with each hit + fires.
                self.fx_sprite("anim_mtt_frame", 0, 330, 40, 2, 1);
                let doors = match done {
                    0 => "anim_mtt_doors_clean_anim",
                    1 => "anim_mtt_doors_damage1_anim",
                    2 => "anim_mtt_doors_damage2_anim",
                    _ => "anim_mtt_doors_damage3_anim",
                };
                self.fx_sprite(doors, f60, 382, 32, 2, 1);
                if done >= 2 {
                    self.fx_sprite("anim_mtt_fires", f60, 330, 25, 3, 1);
                }
                if a < 1_000 {
                    let fr = ((a as u64 * 15) / 1000) as usize;
                    self.fx_sprite("anim_left_bdroid_unpacking", fr.min(14), -30, 75, 3, 1);
                    self.fx_sprite("anim_right_bdroid_unpacking", fr.min(14), 470, 75, 3, 1);
                } else {
                    let cyc = a % 2_400;
                    if cyc < 500 {
                        self.fx_sprite("anim_left_bdroid_shooting", f60, 40, 80, 3, 1);
                        self.fx_sprite("anim_right_bdroid_shooting", f60, 520, 80, 3, 1);
                        let dx = (cyc / 4) as i32;
                        self.fx_sprite("anim_bdroid_zap", f60, 96 + dx, 112, 6, 1);
                        self.fx_sprite("anim_bdroid_zap", f60, 516 - dx, 112, 6, 1);
                    }
                }
            }
            Swe1Scene::DestroyerDroid => {
                self.fx_sprite("anim_sith_probe_target", f60, 258, 100, 6, 1);
                let ps = a % 4_200;
                if ps < 1_083 {
                    self.fx_sprite("anim_destroyer_droid_s_roll", ((ps * 60) / 1000) as usize, 0, 100, 4, 1);
                } else if (ps / 700) % 3 == 0 {
                    self.fx_sprite("anim_destroyer_droid_s_shooting", f60, 30, 96, 4, 1);
                }
                let pm = (a + 1_400) % 4_200;
                if pm < 1_083 {
                    self.fx_sprite("anim_destroyer_droid_m_roll", ((pm * 60) / 1000) as usize, 235, 82, 4, 1);
                } else if (pm / 700) % 3 == 1 {
                    self.fx_sprite("anim_destroyer_droid_m_shooting", f60, 220, 84, 4, 1);
                }
                let pl = (a + 2_800) % 4_200;
                if (pl / 700) % 3 == 2 {
                    self.fx_sprite("anim_destroyer_droid_l_shooting", f60, 56, 62, 6, 1);
                } else {
                    self.fx_sprite("anim_destroyer_droid_l_idle", f60, 20, 62, 6, 1);
                }
            }
            Swe1Scene::FederationFighter => {
                self.fx_sprite("anim_droid_control_ship2", 0, 430, 20, 2, 1);
                self.fx_sprite("anim_federation_fighter_side", 0, 560, -20, 2, 1);
                let pf = a % 4_500;
                if pf < 450 {
                    self.fx_sprite("anim_federation_fighter_flyby", ((pf * 60) / 1000) as usize, 216, 35, 3, 1);
                }
                if (a / 1_500) % 3 == 2 {
                    self.fx_sprite("anim_federation_fighter_redeye", f60, 220, 74, 4, 1);
                }
                if (f60 % 40) < 12 {
                    self.fx_sprite("anim_federation_fighter_zap", f60, 300, 112, 5, 1);
                }
            }
            Swe1Scene::GunganBattle => {
                self.fx_sprite("anim_gungan_sub_bubbles", f60, 470, 10, 2, 1);
                let ps = a % 9_000;
                if ps < 1_417 {
                    self.fx_sprite("anim_gungan_sub_sando", ((ps * 60) / 1000) as usize, 240, 60, 3, 1);
                }
            }
            Swe1Scene::HangarEscape => {
                self.fx_sprite("anim_naboo_xhair_blue", f60, 252, 84, 6, 1);
                if (f60 % 50) < 14 {
                    self.fx_sprite("anim_federation_fighter_zap", f60, 200, 120, 5, 1);
                }
            }
            Swe1Scene::HoverTank => {
                if a < 417 {
                    let fr = ((a * 60) / 1000) as usize;
                    self.fx_sprite("anim_hover_tank_l_enter", fr, 60, 72, 3, 1);
                    self.fx_sprite("anim_hover_tank_r_enter", fr, 440, 72, 3, 1);
                    self.fx_sprite("anim_hover_tank_c_enter", fr, 233, 50, 4, 1);
                } else {
                    let ps = a % 3_000;
                    if ps < 167 {
                        self.fx_sprite("anim_hover_tank_l_shoot", ((ps * 60) / 1000) as usize, 60, 72, 3, 1);
                    } else if (1_000..1_167).contains(&ps) {
                        self.fx_sprite("anim_hover_tank_c_shoot", (((ps - 1_000) * 60) / 1000) as usize, 256, 76, 4, 1);
                    } else if (2_000..2_167).contains(&ps) {
                        self.fx_sprite("anim_hover_tank_r_shoot", (((ps - 2_000) * 60) / 1000) as usize, 300, 72, 3, 1);
                    }
                }
                let pc = a % 5_000;
                if pc < 433 {
                    self.fx_sprite("anim_hover_tank_catapult_shoot", ((pc * 60) / 1000) as usize, 456, 112, 5, 1);
                }
            }
            Swe1Scene::Podrace => {
                // Race position icons (`DeffPodRaceSceneBackground`) and rival pods — z=8
                // keeps `z<=7` free for `SoftwareCompositor::render_podrace_scene`.
                let lead = (f60 / 30) % 4;
                for i in 0..4usize {
                    let blur = i == lead;
                    let name = match (i, blur) {
                        (0, false) => "anim_podrace_icon1",
                        (0, true) => "anim_podrace_icon1_blur",
                        (1, false) => "anim_podrace_icon2",
                        (1, true) => "anim_podrace_icon2_blur",
                        (2, false) => "anim_podrace_icon3",
                        (2, true) => "anim_podrace_icon3_blur",
                        (_, false) => "anim_podrace_icon4",
                        (_, true) => "anim_podrace_icon4_blur",
                    };
                    self.fx_sprite(name, f60, 8, 108 + (i as i32) * 30, 8, 1);
                }
                let drift = ((a / 20) % 300) as i32;
                self.fx_sprite("anim_podrace_3", f60, 460 - drift, 150, 8, 1);
                self.fx_sprite("anim_podrace_4", f60, 180 + drift / 2, 164, 8, 1);
            }
            Swe1Scene::ProbeDroid => {
                self.fx_sprite("anim_sith_probe_hover_far", f60, 470, 40, 2, 1);
                let pf = a % 3_000;
                if pf < 600 {
                    let k = (a / 3_000) % 3;
                    let name = ["anim_sith_probe_fly_1", "anim_sith_probe_fly_2", "anim_sith_probe_fly_3"][k as usize];
                    self.fx_sprite(name, f60, 640 - (pf as i32), 30, 3, 1);
                }
                let pq = a % 3_600;
                if pq >= 2_150 {
                    self.fx_sprite("anim_sith_probe_queen", f60, 30, 65, 5, 1);
                } else if (1_800..2_150).contains(&pq) {
                    self.fx_sprite("anim_sith_probe_queen_shoot_3", f60, 30, 64, 5, 1);
                } else if pq < 350 {
                    self.fx_sprite("anim_sith_probe_queen_shoot_2", f60, 30, 63, 5, 1);
                }
                self.fx_sprite("anim_sith_probe_target", f60, 258, 150, 6, 1);
            }
            Swe1Scene::QueenFashion => {
                let icons = [
                    "anim_fashion_icon_black",
                    "anim_fashion_icon_fanhead",
                    "anim_fashion_icon_hornhead",
                    "anim_fashion_icon_nun",
                    "anim_fashion_icon_spade",
                    "anim_fashion_icon_victorian",
                    "anim_fashion_icon_extra_ball",
                ];
                let idx = (done as usize + (a / 1_500) as usize) % icons.len();
                self.fx_sprite(icons[idx], f60, 490, 92, 6, 1);
            }
            Swe1Scene::JarJarJuggle => {
                self.fx_sprite("anim_textbox_300", f60, 170, 198, 6, 1);
                let junk = [
                    "anim_jarjar_junk1",
                    "anim_jarjar_junk2",
                    "anim_jarjar_junk3",
                    "anim_jarjar_junk4",
                    "anim_jarjar_junk5",
                    "anim_jarjar_junk6",
                ];
                let step = f60 / 10;
                for (i, name) in junk.iter().enumerate() {
                    let (x, y) = JUGGLE_ORBIT[(step + i * 2) % JUGGLE_ORBIT.len()];
                    self.fx_sprite(name, f60, x - 50, y - 30, 6, 1);
                }
                if hits == 1 {
                    let (x, y) = JUGGLE_ORBIT[(step + 7) % JUGGLE_ORBIT.len()];
                    self.fx_sprite("anim_jarjar_junk_eb", f60, x - 58, y - 30, 6, 1);
                }
            }
            Swe1Scene::WattosChance => {
                // Watto timeline (`DeffWattosChanceSceneBackground`): left -> middle -> right.
                const SEG: [(&str, u32, i32, i32, bool); 7] = [
                    ("anim_watto_scene_left_loop", 1_000, 30, 46, true),
                    ("anim_watto_scene_left_to_middle", 167, 100, 40, false),
                    ("anim_watto_scene_hand_to_chin_middle", 250, 233, 40, false),
                    ("anim_watto_scene_chin_loop_middle", 1_000, 233, 44, true),
                    ("anim_watto_scene_hand_down_middle", 233, 233, 40, false),
                    ("anim_watto_scene_middle_to_right", 233, 160, 44, false),
                    ("anim_watto_scene_right_loop", 1_067, 380, 46, true),
                ];
                let total: u32 = SEG.iter().map(|s| s.1).sum();
                let mut t = a % total;
                for (name, len, x, y, looping) in SEG {
                    if t < len {
                        let fr = if looping { f60 } else { ((t * 60) / 1000) as usize };
                        self.fx_sprite(name, fr, x, y, 3, 1);
                        break;
                    }
                    t -= len;
                }
            }
            Swe1Scene::MosEspaCantina => {
                self.fx_sprite("anim_jedi_council_chic", f60, 444, 64, 4, 1);
            }
            Swe1Scene::R2D2 => {
                let units = [
                    ("anim_r2d2_scene_dark_blue_r2_unit", 400, 176),
                    ("anim_r2d2_scene_yellow_r2_unit", 0, 150),
                    ("anim_r2d2_scene_red_white_r2_unit", 360, 160),
                ];
                let (name, x, y) = units[((a / 2_500) % 3) as usize];
                self.fx_sprite(name, f60, x, y, 6, 1);
            }
            Swe1Scene::QuestionMark => {
                self.fx_sprite("anim_super_mystery_icon", f60, 275, 120, 5, 1);
            }
        }
    }

    /// Layered *Boonta Eve Podrace* canyon + Anakin + rival podracer scene (`z = 1..6`).
    pub(crate) fn render_podrace_layers(&mut self, active_ms: u32) {
        let t = (active_ms / 16) as usize;
        if t < 60 {
            self.fx_sprite("anim_podrace_starting", t, 0, 0, 2, 1);
            return;
        }
        let race_t = t.saturating_sub(60);
        let scroll_x = -(((race_t * 3) % 160) as i32);
        self.fx_sprite("anim_podrace_sky", 0, 0, 0, 1, 1);
        self.fx_sprite("anim_podrace_horizon", 0, scroll_x, 4, 2, 1);
        self.fx_sprite("anim_podrace_ground", race_t, 0, 112, 3, 1);

        let cycle = race_t % 55;
        if (28..54).contains(&cycle) {
            self.fx_sprite("anim_podrace_2_explosion", cycle - 28, 145, 18, 4, 1);
        } else {
            let rival_x = 245 + (((race_t as i32) * 5) % 90) - 45;
            self.fx_sprite("anim_podrace_2", race_t, rival_x, 90, 4, 1);
        }
        let sway = match (race_t / 20) % 4 {
            0 => (race_t % 20) as i32 * 2 - 20,
            1 => 20 - (race_t % 20) as i32 * 2,
            2 => -((race_t % 20) as i32),
            _ => -20 + (race_t % 20) as i32,
        };
        self.fx_sprite("anim_podrace_1", race_t, 143 + sway, 76, 5, 1);
        if (51..68).contains(&race_t) || (101..118).contains(&race_t) {
            self.fx_sprite("anim_podrace_checkpoint", 0, 224, 10, 6, 1);
        }
        let mph = 615 + ((race_t * 4) % 332);
        self.fx_text("font_shadow_9_data", format!("SPEED {mph} MPH"), 556, 12, 8, 0x03FF);
    }
}
