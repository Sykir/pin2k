//! Authentic ROM-lifted SWE1 scene state machines & renderers.
//!
//! Each scene module ports the exact gameplay logic, reward tables, switch-to-lane
//! mappings, animation state machines, and `(x, y, z)` coordinates from `swe1_0200.elf`.

pub mod jarjar_juggle;
pub mod jarjar_juggle_state;
pub mod podrace;
pub mod podrace_render;
pub mod podrace_state;
