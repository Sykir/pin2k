//! Authentic `QuestionMarkScene` + `DeffQuestionMarkSceneBackground` runtime & renderer (`swe1_0200.elf`):
//! - Displays the Mystery Award (`1M`, `+3 Lasers`, `Bonus +2X`, `Extra Ball`) for 5 seconds
//!   (`sleep(5)` in `DeffQuestionMarkSceneBackground::Run`) then completes the scene.

use pin2k_hal::{AudioBackend, PinballHardware, TextAlign, VideoBackend};

use super::question_mark_state::QuestionMarkState;
use crate::machine::Pin2kMachine;
use crate::swe1_rules::Swe1Scene;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn start_question_mark_state(&mut self) {
        self.question_mark = Some(QuestionMarkState::new(self.elapsed_ms));
    }

    pub(crate) fn tick_question_mark(&mut self, dt_ms: u32) {
        let active_gameplay = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.scene == Swe1Scene::QuestionMark && s.intro_remaining_ms == 0);
        if !active_gameplay {
            return;
        }
        let mut complete = false;
        if let Some(ref mut qm) = self.question_mark {
            qm.elapsed_ms = qm.elapsed_ms.saturating_add(dt_ms);
            if qm.elapsed_ms >= 5_000 {
                complete = true;
            }
        }
        if complete {
            self.complete_active_scene(Swe1Scene::QuestionMark);
            self.question_mark = None;
        }
    }

    pub(crate) fn render_question_mark(&mut self, _secs: u32) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let qm = self.question_mark.clone().unwrap_or_default();

        self.scene_sprite("anim_question_mark_blue", f60, 140, 88, 4);
        self.scene_sprite("anim_question_mark_green", f60, 378, 88, 4);
        self.scene_sprite("anim_super_mystery_icon", f60, 275, 120, 5);
        self.scene_text(
            "font_shadow_15_data",
            "MYSTERY AWARD",
            320,
            54,
            6,
            0x7FE0,
            Some(0x001F),
            TextAlign::Center,
        );
        self.scene_text(
            "font_shadow_11_narrow_data",
            qm.award.label(),
            320,
            182,
            7,
            0x7FFF,
            Some(0x0000),
            TextAlign::Center,
        );
    }
}
