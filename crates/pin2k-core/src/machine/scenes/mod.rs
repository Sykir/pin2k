//! Authentic ROM-lifted SWE1 scene state machines & renderers.
//!
//! Each scene module ports the exact gameplay logic, reward tables, switch-to-lane
//! mappings, animation state machines, and `(x, y, z)` coordinates from `swe1_0200.elf`.

pub mod battle_droid;
pub mod battle_droid_state;
pub mod destroyer_droid;
pub mod destroyer_droid_state;
pub mod federation_fighter;
pub mod federation_fighter_state;
pub mod gungan_battle;
pub mod gungan_battle_state;
pub mod hangar_escape;
pub mod hangar_escape_state;
pub mod hover_tank;
pub mod hover_tank_state;
pub mod probe_droid;
pub mod probe_droid_state;
pub mod question_mark;
pub mod question_mark_state;
pub mod queen_fashion;
pub mod queen_fashion_state;
pub mod r2d2;
pub mod r2d2_state;
pub mod wattos_chance;
pub mod wattos_chance_state;
pub mod jarjar_juggle;
pub mod jarjar_juggle_state;
pub mod mos_espa_cantina;
pub mod mos_espa_cantina_state;
pub mod podrace;
pub mod podrace_render;
pub mod podrace_state;
