//! ROM effect animation metadata (`frames`, `fps`, `w`, `h`) lifted from `gfx_manifest.json`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FxAnim {
pub name: &'static str,
pub frames: u16,
pub fps: u16,
pub w: i32,
pub h: i32,
}

const fn fx(name: &'static str, frames: u16, fps: u16, w: i32, h: i32) -> FxAnim {
FxAnim { name, frames, fps, w, h }
}

// DeffBattleDroidSceneBackground
pub const BDROID_EXPLOSION: FxAnim = fx("anim_bdroid_explosion", 20, 60, 245, 129);
pub const BDROID_LASER_EXPLOSION: FxAnim = fx("anim_bdroid_laser_explosion", 20, 60, 230, 131);
pub const BDROID_STAP_EXPLOSION: FxAnim = fx("anim_bdroid_stap_explosion", 19, 60, 574, 213);
pub const BDROID_TANK_EXPLOSION: FxAnim = fx("anim_bdroid_tank_explosion", 25, 60, 580, 220);
// DeffDestroyerDroidSceneBackground
pub const DESTROYER_S_EXPLOSION: FxAnim = fx("anim_destroyer_droid_s_explosion", 33, 60, 284, 119);
pub const DESTROYER_M_EXPLOSION: FxAnim = fx("anim_destroyer_droid_m_explosion", 34, 60, 308, 171);
pub const DESTROYER_L_EXPLOSION: FxAnim = fx("anim_destroyer_droid_l_explosion", 52, 60, 350, 155);
pub const SITH_PROBE_BLAST_1: FxAnim = fx("anim_sith_probe_blast_1", 2, 60, 253, 43);
pub const SITH_PROBE_BLAST_2: FxAnim = fx("anim_sith_probe_blast_2", 2, 60, 137, 40);
pub const SITH_PROBE_BLAST_3: FxAnim = fx("anim_sith_probe_blast_3", 2, 60, 109, 42);
pub const EXPLOSION_3: FxAnim = fx("anim_explosion_3", 21, 60, 640, 240);
// DeffFedFighterSceneBackground
pub const FED_EXPLOSION: FxAnim = fx("anim_federation_fighter_explosion", 16, 60, 324, 150);
pub const FED_EXPLOSION_2: FxAnim = fx("anim_federation_fighter_explosion_2", 36, 60, 360, 150);
pub const FED_EXPLOSION_3: FxAnim = fx("anim_federation_fighter_explosion_3", 40, 60, 324, 150);
pub const FED_LASER_EXPLOSION: FxAnim = fx("anim_federation_fighter_laser_explosion", 16, 60, 324, 150);
pub const FED_LASER_EXPLOSION_2: FxAnim =
    fx("anim_federation_fighter_laser_explosion_2", 30, 60, 324, 150);
pub const EXPLOSION_4: FxAnim = fx("anim_explosion_4", 30, 60, 160, 53);
pub const EXPLOSION_5: FxAnim = fx("anim_explosion_5", 30, 60, 160, 53);
pub const EXPLOSION_6: FxAnim = fx("anim_explosion_6", 30, 60, 160, 53);
// DeffSubEscapeSceneBackground
pub const GUNGAN_SUB_BUBBLES: FxAnim = fx("anim_gungan_sub_bubbles", 54, 60, 146, 215);
pub const GUNGAN_SUB_SANDO: FxAnim = fx("anim_gungan_sub_sando", 85, 60, 397, 156);
// DeffHangarEscapeSceneBackground
pub const HANGAR_DROID_EXPLOSION: FxAnim = fx("anim_hangar_droid_explosion", 30, 60, 251, 145);
pub const HANGAR_DROID_EXPLOSION_2: FxAnim = fx("anim_hangar_droid_explosion_2", 30, 60, 238, 145);
pub const HANGAR_DROID_EXPLOSION_3: FxAnim = fx("anim_hangar_droid_explosion_3", 30, 60, 306, 127);
pub const HANGAR_DROID_EXPLOSION_4: FxAnim = fx("anim_hangar_droid_explosion_4", 30, 60, 262, 145);
pub const HANGAR_DROID_LASER_EXPLOSION: FxAnim =
    fx("anim_hangar_droid_laser_explosion", 30, 60, 255, 149);
pub const HANGAR_DROID_LASER_EXPLOSION_2: FxAnim =
    fx("anim_hangar_droid_laser_explosion_2", 30, 60, 268, 131);
pub const HANGAR_ESCAPE_EXIT_MOVIE: FxAnim = fx("anim_hangar_escape_exit_movie", 47, 15, 320, 240);
// DeffHoverTankSceneBackground
pub const HOVER_TANK_L_EXPLODE: FxAnim = fx("anim_hover_tank_l_explode", 18, 60, 400, 150);
pub const HOVER_TANK_C_EXPLODE: FxAnim = fx("anim_hover_tank_c_explode", 29, 60, 400, 150);
pub const HOVER_TANK_R_EXPLODE: FxAnim = fx("anim_hover_tank_r_explode", 30, 60, 400, 150);
pub const SITH_PROBE_MISS_EXPLOSION: FxAnim = fx("anim_sith_probe_miss_explosion", 25, 60, 125, 65);
// DeffPodRaceSceneBackground
pub const PODRACE_3_EXPLOSION: FxAnim = fx("anim_podrace_3_explosion", 26, 60, 115, 51);
pub const PODRACE_ANAKIN_ZOOM: FxAnim = fx("anim_podrace_anakin_zoom", 20, 60, 228, 90);
// DeffProbeDroidSceneBackground
pub const SITH_PROBE_EXPLOSION_1: FxAnim = fx("anim_sith_probe_explosion_1", 28, 60, 318, 150);
pub const SITH_PROBE_EXPLOSION_2: FxAnim = fx("anim_sith_probe_explosion_2", 56, 60, 347, 153);
// Queen / Jedi Council / misc cinematics
pub const FASHION_FLASH: FxAnim = fx("anim_fashion_flash", 12, 60, 190, 121);
pub const QUEEN_ESCAPE_MISC_MOVIE: FxAnim = fx("anim_queen_escape_misc_movie", 28, 15, 320, 140);
pub const JEDI_STAND_MISC_MOVIE: FxAnim = fx("anim_jedi_stand_misc_movie", 33, 15, 320, 140);
// DeffJarjarJuggleScene
pub const JARJAR_JUMPUP: FxAnim = fx("anim_jarjar_jumpup", 40, 60, 400, 213);
pub const JARJAR_SWITCH: FxAnim = fx("anim_jarjar_switch", 4, 60, 189, 209);
pub const JARJAR_FALL: FxAnim = fx("anim_jarjar_fall", 20, 60, 470, 240);
pub const JARJAR_FALL_2: FxAnim = fx("anim_jarjar_fall_2", 18, 60, 318, 220);
pub const JARJAR_THUMBS_UP: FxAnim = fx("anim_jarjar_thumbs_up", 26, 60, 115, 60);
// DeffJarJar (JarJarSaucer during Super Jets)
pub const JARJAR_TONGUE: FxAnim = fx("anim_jarjar_tongue", 35, 60, 202, 141);
pub const JARJAR_FROG_FOUNTAIN: FxAnim = fx("anim_jarjar_frog_fountain", 46, 60, 200, 129);
pub const JARJAR_SURPRISE: FxAnim = fx("anim_jarjar_surprise", 28, 60, 174, 134);
// DeffWattosChanceSceneBackground
pub const WATTO_CUBE_EXPLODE: FxAnim = fx("anim_watto_scene_cube_explode", 30, 60, 256, 118);
// R2-D2 / Mystery / generic sparks & award booms
pub const SPARKS_1: FxAnim = fx("anim_sparks_1", 15, 60, 200, 100);
pub const FIREWORK_2: FxAnim = fx("anim_firework_2", 60, 60, 284, 240);
pub const JACKPOT_ICON_BOOM: FxAnim = fx("anim_jackpot_icon_boom", 9, 60, 354, 110);
pub const BLUE_ICON_BOOM_BIG: FxAnim = fx("anim_blue_icon_boom_big", 9, 60, 354, 110);
pub const BLUE_ICON_BOOM_SMALL: FxAnim = fx("anim_blue_icon_boom_small", 10, 60, 158, 81);
pub const GREEN_ICON_BOOM_BIG: FxAnim = fx("anim_green_icon_boom_big", 9, 60, 354, 110);
// VirtualSpinners / JetBumpers (`DeffBumperSparks`) / Shield / Lasers
pub const SPINNER_EXPLOSION: FxAnim = fx("anim_spinner_explosion", 15, 60, 150, 75);
pub const BUMPER_BUBBLES_1: FxAnim = fx("anim_bumper_bubbles_1", 33, 60, 235, 122);
pub const BUMPER_BUBBLES_2: FxAnim = fx("anim_bumper_bubbles_2", 41, 60, 157, 121);
pub const SHIELD_BLOCK: FxAnim = fx("anim_shield_block", 15, 60, 200, 25);
pub const SHIELD_BLOCK_WIPE: FxAnim = fx("anim_shield_block_wipe", 9, 60, 200, 25);
pub const DROP_TARGET: FxAnim = fx("anim_drop_target", 2, 2, 293, 186);
pub const ACTION_BUTTON: FxAnim = fx("anim_action_button", 1, 1, 211, 100);
// DeffObiHolo
pub const OBIWAN_HOLO_ACTIVATE: FxAnim = fx("anim_obiwan_holo_activate", 18, 60, 255, 81);
pub const HOLO_BURST: FxAnim = fx("anim_holo_burst", 18, 60, 450, 225);
pub const OBIWAN_HOLO_FLATTEN: FxAnim = fx("anim_obiwan_holo_flatten", 30, 60, 320, 160);
// JediMultiball outcome (`JediMultiball` class)
pub const JEDI_BATTLE_QUIGON_DEATH: FxAnim = fx("anim_jedi_battle_quigon_death", 55, 60, 396, 178);
pub const JEDI_BATTLE_MAUL_DEATH_2: FxAnim = fx("anim_jedi_battle_maul_death_2", 28, 60, 396, 178);
pub const JEDI_BATTLE_WIN_ICON_MAUL: FxAnim = fx("anim_jedi_battle_win_icon_maul", 1, 1, 370, 175);
