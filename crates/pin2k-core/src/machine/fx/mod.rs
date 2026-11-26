//! ROM effect layers (`Deff*` one-shot sprites) and timed full-screen overlay Deffs.

pub mod anims;
mod overlays;
mod scene_extras;
mod triggers;

pub use anims::FxAnim;

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};

use super::Pin2kMachine;
use crate::swe1_rules::{ActiveMultiball, JediRank};
use anims::*;


/// One-shot ROM sprite effect queued on top of the current scene.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveFx {
    pub anim: FxAnim,
    pub x: i32,
    pub y: i32,
    pub z: u8,
    pub scale_x: u32,
    pub delay_ms: u32,
    pub hold_ms: u32,
    pub elapsed_ms: u32,
}

impl ActiveFx {
    /// Total lifetime: start delay + max(native playback length, minimum hold).
    pub fn duration_ms(&self) -> u32 {
        let play = (self.anim.frames as u32 * 1000).div_ceil(self.anim.fps.max(1) as u32);
        self.delay_ms + play.max(self.hold_ms)
    }

    /// Frame to display (clamped to the last frame), or `None` while still delayed.
    pub fn current_frame(&self) -> Option<usize> {
        if self.elapsed_ms < self.delay_ms {
            return None;
        }
        let t = (self.elapsed_ms - self.delay_ms) as u64;
        let f = (t * self.anim.fps as u64) / 1000;
        Some(f.min(self.anim.frames.saturating_sub(1) as u64) as usize)
    }
}

/// Timed full-screen overlay Deffs that pre-empt the scene and HUD while they run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayKind {
    /// `DeffWattoJunkShop` (`WattoRule::junk_shop_process`, random saucer award, 5 s).
    WattoJunkShop { award: u8 },
    /// `DeffMarketplace` (Mos Espa market — Jar Jar picks and swallows a chuba).
    Marketplace { award: u8 },
    /// `DeffJediMultiballIntro` (J-E-D-I letters shrink, bezel, movie and badge).
    JediMultiballIntro,
    /// Jedi rank promotion after defeating Darth Maul (Qui-Gon win icon, Yoda, fireworks).
    RankPromotion,
    /// Jedi Spirit rank reached (Sidious / Maul / Jedi spirits, electricity and sparks).
    JediSpirit,
}

impl OverlayKind {
    pub const fn duration_ms(self) -> u32 {
        match self {
            OverlayKind::WattoJunkShop { .. } | OverlayKind::Marketplace { .. } => 5_000,
            OverlayKind::JediMultiballIntro => 4_500,
            OverlayKind::RankPromotion => 3_800,
            OverlayKind::JediSpirit => 6_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveOverlay {
    pub kind: OverlayKind,
    pub elapsed_ms: u32,
}

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    fn fx_sprite(&mut self, name: &str, frame: usize, x: i32, y: i32, z: u8, scale_x: u32) {
        let mut cmd = SpriteDrawCmd::new(name, frame, x, y, z);
        cmd.scale_x = scale_x.max(1);
        self.video.draw_named_sprite(cmd);
    }

    fn fx_text(&mut self, font: &str, text: impl Into<String>, x: i32, y: i32, z: u8, color: u16) {
        self.video.draw_font_text(TextDrawCmd {
            font_name: font.to_string(),
            text: text.into(),
            x,
            y,
            z,
            color_rgb555: color,
            shadow_rgb555: Some(0x0000),
            align: TextAlign::Center,
        });
    }

    /// Queue a one-shot ROM effect with its top-left corner at `(x, y)`.
    pub fn spawn_fx(&mut self, anim: FxAnim, x: i32, y: i32, z: u8) {
        self.spawn_fx_ex(anim, x, y, z, 1, 0, 0);
    }

    /// Queue a one-shot ROM effect centered on `(cx, cy)`.
    pub fn spawn_fx_centered(&mut self, anim: FxAnim, cx: i32, cy: i32, z: u8) {
        self.spawn_fx(anim, cx - anim.w / 2, cy - anim.h / 2, z);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn spawn_fx_ex(
        &mut self,
        anim: FxAnim,
        x: i32,
        y: i32,
        z: u8,
        scale_x: u32,
        delay_ms: u32,
        hold_ms: u32,
    ) {
        // Re-triggering the same effect restarts it instead of stacking copies.
        self.fx_queue.retain(|f| f.anim.name != anim.name);
        self.fx_queue.push(ActiveFx {
            anim,
            x,
            y,
            z,
            scale_x: scale_x.max(1),
            delay_ms,
            hold_ms,
            elapsed_ms: 0,
        });
        if self.fx_queue.len() > 12 {
            self.fx_queue.remove(0);
        }
    }

    pub fn start_overlay(&mut self, kind: OverlayKind) {
        self.overlay = Some(ActiveOverlay { kind, elapsed_ms: 0 });
    }

    pub(super) fn tick_fx(&mut self, dt_ms: u32) {
        for f in &mut self.fx_queue {
            f.elapsed_ms = f.elapsed_ms.saturating_add(dt_ms);
        }
        self.fx_queue.retain(|f| f.elapsed_ms < f.duration_ms());
        if let Some(ref mut ov) = self.overlay {
            ov.elapsed_ms = ov.elapsed_ms.saturating_add(dt_ms);
            if ov.elapsed_ms >= ov.kind.duration_ms() {
                self.overlay = None;
            }
        }
    }

    /// Draws all queued one-shot effects.
    pub(super) fn render_fx(&mut self) {
        let items: Vec<(FxAnim, usize, i32, i32, u8, u32)> = self
            .fx_queue
            .iter()
            .filter_map(|f| f.current_frame().map(|fr| (f.anim, fr, f.x, f.y, f.z, f.scale_x)))
            .collect();
        for (anim, fr, x, y, z, sx) in items {
            self.fx_sprite(anim.name, fr, x, y, z, sx);
        }
    }


    // ------------------------------------------------------------------
    // `DeffJarJar` — JarJarSaucer during Super Jets
    // ------------------------------------------------------------------

    pub(super) fn jarjar_saucer_active(&self) -> bool {
        self.game_title == crate::rfm_rules::GameTitle::Swe1
            && self.active_scene.is_none()
            && self.multiball == ActiveMultiball::None
            && self
                .players
                .get(self.current_player)
                .is_some_and(|p| p.super_jets_ms > 0)
    }

    /// `JarJarSaucer::hook_jet` -> `DeffJarJar::hit` / `hook_right_ramp_enter` -> `DeffJarJar::spin`.
    pub(super) fn jarjar_saucer_event(&mut self, spin: bool, jet_idx: u8) {
        if !self.jarjar_saucer_active() {
            return;
        }
        if spin {
            self.spawn_fx(JARJAR_SURPRISE, 456, 106, 17);
        } else if jet_idx % 2 == 0 {
            self.spawn_fx(JARJAR_TONGUE, 430, 99, 16);
        } else {
            self.spawn_fx(JARJAR_FROG_FOUNTAIN, 432, 111, 16);
        }
    }

    pub(super) fn render_jarjar_saucer(&mut self) {
        if !self.jarjar_saucer_active() {
            return;
        }
        let active_ms = 20_000u32.saturating_sub(self.players[self.current_player].super_jets_ms);
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        if active_ms < 467 {
            let fr = ((active_ms as u64 * 60) / 1000) as usize;
            self.fx_sprite("anim_jarjar_appear", fr.min(27), 499, 79, 14, 1);
        } else {
            self.fx_sprite("anim_jarjar_idle_1", f60, 540, 105, 14, 1);
        }
    }


    /// Promotion overlay after Maul is defeated (`JediSpirit` at the top rank).
    pub(super) fn rank_promotion_overlay(&mut self, rank: JediRank) {
        if rank == JediRank::Spirit {
            self.start_overlay(OverlayKind::JediSpirit);
        } else {
            self.start_overlay(OverlayKind::RankPromotion);
        }
    }
}
