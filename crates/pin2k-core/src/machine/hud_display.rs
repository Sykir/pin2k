//! Authentic `DeffLaserStatusBackground` (`0x001445f0..0x00144d64` in `swe1_0200.elf`):
//! - Draws `anim_naboo_fighter_tiny_rotate` (`100×50`) at `x = 535` (`0x27B - 100`), `y = 10`, `z = 20`.
//! - Uses normal animated frames when `laser_shots > 0`, or monochrome gray `0x294A` when `0`.
//! - Draws laser count & `"LASERS"` / `"TORPEDOS"` label in `font_system_7_data` (`z = 21`,
//!   yellow `0x7FE0` when ready, gray `0x294A` when depleted).

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};

use super::Pin2kMachine;
use crate::swe1_rules::Swe1Scene;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    /// Renders `DeffLaserStatusBackground` in the top-right corner (`x=535, y=10, z=20/21`)
    /// plus active in-game toast banners (`z=24`).
    pub fn render_laser_status_hud(&mut self) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let lasers = self
            .players
            .get(self.current_player)
            .map(|p| p.laser_shots)
            .unwrap_or(0);
        let available = lasers > 0;

        let mut ship_cmd = SpriteDrawCmd::new(
            "anim_naboo_fighter_tiny_rotate",
            if available { f60 % 60 } else { 0 },
            535, // 0x27B (635) - 100
            10,  // 0x0A
            20,  // 0x14
        );
        if !available {
            ship_cmd.monochrome_rgb555 = Some(0x294A);
        }
        self.video.draw_named_sprite(ship_cmd);

        let color = if available { 0x7FE0_u16 } else { 0x294A_u16 };
        let unit_label = if matches!(
            self.active_scene.as_ref().map(|s| s.scene),
            Some(Swe1Scene::HangarEscape)
        ) {
            "TORPEDOS"
        } else if lasers == 1 {
            "LASER"
        } else {
            "LASERS"
        };

        // Centered on the Naboo Fighter (`535 + 50 = 585`)
        self.video.draw_font_text(TextDrawCmd {
            font_name: "font_system_7_data".into(),
            text: format!("{lasers}"),
            x: 585,
            y: 35,
            z: 21,
            color_rgb555: color,
            shadow_rgb555: Some(0x0000),
            align: TextAlign::Center,
        });
        self.video.draw_font_text(TextDrawCmd {
            font_name: "font_system_7_data".into(),
            text: unit_label.to_string(),
            x: 585,
            y: 48,
            z: 21,
            color_rgb555: color,
            shadow_rgb555: Some(0x0000),
            align: TextAlign::Center,
        });
    }

    /// Renders active callout banner at the bottom of the 640×240 frame (`z=24`) when timed.
    pub fn render_ingame_banner(&mut self) {
        if self.banner_timer_ms > 0 && !self.status_banner.is_empty() {
            self.video.draw_font_text(TextDrawCmd {
                font_name: "font_shadow_11_narrow_data".into(),
                text: self.status_banner.clone(),
                x: 320,
                y: 204,
                z: 24,
                color_rgb555: 0x7FE0,
                shadow_rgb555: Some(0x0000),
                align: TextAlign::Center,
            });
        }
    }
}
