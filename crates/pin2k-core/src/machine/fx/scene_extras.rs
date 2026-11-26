use pin2k_hal::{AudioBackend, PinballHardware, VideoBackend};

use crate::machine::Pin2kMachine;
use crate::swe1_rules::Swe1Scene;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    // ------------------------------------------------------------------
    // Extra authentic scene layers (`Deff*SceneBackground` owned animations)
    // ------------------------------------------------------------------

    /// Adds the remaining ROM-owned background layers for an active (post-intro) scene.
    pub(crate) fn render_scene_extras(&mut self, _scene: Swe1Scene, _active_ms: u32, _hits: u8) {
        // All 14 SWE1 scenes now render authentically in `machine/scenes/*.rs`.
    }
}
