//! Multi-layer Holoprojector compositors for all 12 SWE1 Scenes,
//! Obi-Wan Hologram Scene Selection, and Darth Maul Saber Duel.

use crate::compositor::SoftwareCompositor;

impl SoftwareCompositor {
    /// Dispatches specialized multi-layer scene compositors for `active_name`.
    /// Returns `true` if a multi-layer compositor rendered the frame.
    pub(crate) fn render_specialized_scene(&mut self, active_name: &str) -> bool {
        let f60 = ((self.anim_elapsed_ms * 60) / 1000) as usize;
        let f15 = ((self.anim_elapsed_ms * 15) / 1000) as usize;

        if active_name.starts_with("anim_obiwan_holo_") {
            self.render_holo_scene_select(active_name, f60);
            return true;
        }

        match active_name {
            "anim_podrace_1" => {
                self.render_podrace_scene(false);
                true
            }
            "anim_podrace_closeup" => {
                self.render_podrace_scene(true);
                true
            }
            "anim_spinner_1" => {
                self.render_ingame_spinners();
                true
            }
            "anim_bdroid_on_stap" => {
                let _ = self.blit_named("anim_bdroid_background", 0, 0, 0, 1);
                let _ = self.blit_named("anim_left_bdroid_standing", f60, 40, 80, 1);
                let _ = self.blit_named("anim_right_bdroid_standing", f60, 520, 80, 1);
                let _ = self.blit_named("anim_bdroid_on_stap", f60, 238, 68, 1);
                let _ = self.blit_named("anim_stap_xhair", f60, 275, 76, 1);
                if (f60 % 48) < 18 {
                    let _ = self.blit_named("anim_bdroid_stap_zap", f60, 280, 96, 1);
                }
                true
            }
            "anim_destroyer_droid_l_roll" => {
                let _ = self.blit_named("anim_destroyer_droid_background", 0, 0, 0, 1);
                let _ = self.blit_named("anim_destroyer_droid_s_idle", f60, 80, 92, 1);
                let _ = self.blit_named("anim_destroyer_droid_m_idle", f60, 430, 82, 1);
                let _ = self.blit_named("anim_destroyer_droid_l_roll", f60, 20, 62, 1);
                true
            }
            "anim_federation_fighter" => {
                let _ = self.blit_named("anim_federation_fighter_background", 0, 0, 0, 1);
                let _ = self.blit_named("anim_droid_control_ship", 0, 120, 8, 1);
                let _ = self.blit_named("anim_federation_fighter", f60, 220, 74, 1);
                let _ = self.blit_named("anim_naboo_xhair", 0, 245, 78, 1);
                true
            }
            "anim_gungan_sub_chomp" => {
                let _ = self.blit_named("anim_gungan_seaweed", f60, 0, 40, 1);
                let _ = self.blit_named("anim_gungan_bubbles", f60, 120, 20, 1);
                let _ = self.blit_named("anim_gungan_sub_sub", f60, 80, 84, 1);
                let _ = self.blit_named("anim_gungan_sub_chomp", f60, 240, 42, 1);
                true
            }
            "anim_hangar_droid_run" => {
                let _ = self.blit_named("anim_hangar_droid_run", f60, 275, 48, 1);
                let _ = self.blit_named("anim_hangar_droid_explosion", f60, 140, 42, 1);
                let _ = self.blit_named("anim_hangar_droid_laser_explosion", f60, 220, 40, 1);
                true
            }
            "anim_hover_tank" => {
                let _ = self.blit_named("anim_hover_tank", f60, 160, 48, 1);
                let _ = self.blit_named("anim_explosion_2", f60, 210, 42, 1);
                true
            }
            "anim_sith_probe_queen_shoot_1" => {
                let _ = self.blit_named("anim_sith_probe_background", 0, 0, 0, 1);
                let _ = self.blit_named("anim_sith_probe_hover", f60, 210, 40, 1);
                let _ = self.blit_named("anim_sith_probe_queen_shoot_1", f60, 40, 84, 1);
                let _ = self.blit_named("anim_sith_probe_xhair", f60, 240, 56, 1);
                true
            }
            "anim_fashion_queen" => {
                let _ = self.blit_named("anim_fashion_base", 0, 0, 0, 1);
                let _ = self.blit_named("anim_fashion_throne", f60, 190, 28, 1);
                let _ = self.blit_named("anim_fashion_queen", f60, 210, 34, 1);
                let _ = self.blit_named("anim_fashion_wheel", f60, 40, 36, 1);
                let _ = self.blit_named("anim_fashion_icon_victorian", f60, 460, 76, 1);
                true
            }
            "anim_jarjar_juggle" => {
                let _ = self.blit_named("anim_jarjar_stare", f60, 40, 16, 1);
                let _ = self.blit_named("anim_jarjar_juggle", f60, 200, 24, 1);
                true
            }
            "anim_watto_cube" => {
                let _ = self.blit_named("anim_watto_junkshop_sky", 0, 0, 0, 1);
                let _ = self.blit_named("anim_watto_junkshop_background", 0, 0, 0, 1);
                let _ = self.blit_named("anim_watto_fly", f60, 220, 24, 1);
                let _ = self.blit_named("anim_watto_cube", f60, 250, 96, 1);
                let _ = self.blit_named("anim_watto_cube_flare", f60, 250, 96, 1);
                true
            }
            "anim_chairs" => {
                let _ = self.blit_named("anim_jedi_council_mace", f60, 40, 86, 1);
                let _ = self.blit_named("anim_jedi_council_yoda", f60, 500, 92, 1);
                let _ = self.blit_named("anim_chairs", f60, 180, 36, 1);
                true
            }
            "anim_jedi_battle_idle"
            | "anim_jedi_battle_clash"
            | "anim_jedi_battle_quigon_high_right_single"
            | "anim_jedi_battle_maul_chop_single"
            | "anim_jedi_battle_maul_death_1" => {
                self.render_jedi_battle_scene(active_name, f60, f15);
                true
            }
            _ => false,
        }
    }

    fn render_holo_scene_select(&mut self, holo_name: &str, f60: usize) {
        let _ = self.blit_named("anim_obiwan_holo_glow", f60, 160, 22, 1);
        let _ = self.blit_named("anim_obiwan_holo", f60, 160, 22, 1);
        let _ = self.blit_named("anim_obiwan_holo_glow_loop", f60, 209, 64, 1);
        let _ = self.blit_named(holo_name, f60, 180, 40, 1);
    }

    fn render_jedi_battle_scene(&mut self, active_name: &str, f60: usize, _f15: usize) {
        let _ = self.blit_named("anim_jedi_battle_background_hangar", 0, 0, 0, 1);
        let _ = self.blit_named("anim_jedi_battle_icon_quigon", f60, 16, 12, 1);
        let _ = self.blit_named("anim_jedi_battle_icon_maul", f60, 532, 12, 1);
        let _ = self.blit_named(active_name, f60, 122, 36, 1);
        if active_name == "anim_jedi_battle_clash" {
            let _ = self.blit_named("anim_jedi_battle_saber_sparks", f60, 188, 42, 1);
        }
    }
}
