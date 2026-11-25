//! Authentic `DeffLaserStatusBackground` (`0x001445f0..0x00144d64` in `swe1_0200.elf`):
//! - Draws `anim_naboo_fighter_tiny_rotate` (`100×50`) at `x = 535` (`0x27B - 100`), `y = 10`, `z = 20`.
//! - Uses normal animated frames when `laser_shots > 0`, or monochrome gray `0x294A` when `0`.
//! - Draws laser count & `"LASERS"` / `"TORPEDOS"` label in `font_system_7_data` (`z = 21`,
//!   yellow `0x7FE0` when ready, gray `0x294A` when depleted).

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};

use super::Pin2kMachine;
use crate::swe1_rules::{ActiveMultiball, Swe1Scene};

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

    /// Renders active callout banner above bottom scene footer (`z=24`) when timed.
    pub fn render_ingame_banner(&mut self) {
        let in_scene_intro = self
            .active_scene
            .as_ref()
            .is_some_and(|s| s.intro_remaining_ms > 0);
        if self.special_screen.is_some() || in_scene_intro {
            return;
        }
        if self.banner_timer_ms > 0 && !self.status_banner.is_empty() {
            self.video.draw_font_text(TextDrawCmd {
                font_name: "font_shadow_11_narrow_data".into(),
                text: self.status_banner.clone(),
                x: 320,
                y: 186,
                z: 24,
                color_rgb555: 0x7FE0,
                shadow_rgb555: Some(0x0000),
                align: TextAlign::Center,
            });
        }
    }

    /// Renders authentic `DeffGunganSeaweed` + `DeffGunganLetters` on the right side
    /// above the jet bumpers during nominal gameplay (no active scene, multiball, or special screen).
    /// - `anim_gungan_seaweed` (`139×90`, 40 fr @ 40fps) at `(496, 135)`, `z = 15`.
    /// - 5 rising `anim_gungan_bubbles` (`17×9`, 3 fr) at `z = 10` (`y = 195 -> 125`).
    /// - `"GUNGAN"` letter progress (`font_shadow_9_narrow_data`, `z = 15`) above the bumpers.
    pub fn render_gungan_nominal_hud(&mut self) {
        if self.active_scene.is_some()
            || self.multiball != ActiveMultiball::None
            || self.special_screen.is_some()
        {
            return;
        }

        // `DeffGunganSeaweed`: x = 0x27b - 139 = 496, y = 0xe1 - 90 = 135, z = 15
        let seaweed_frame = ((self.elapsed_ms * 40) / 1000) as usize % 40;
        self.video.draw_named_sprite(SpriteDrawCmd::new(
            "anim_gungan_seaweed",
            seaweed_frame,
            496,
            135,
            15,
        ));

        // 5 rising `anim_gungan_bubbles` columns behind seaweed (`z = 10`, y: 195 -> 125)
        const BUBBLE_SPEEDS: [u64; 5] = [28, 36, 24, 42, 32];
        const BUBBLE_PHASES: [u64; 5] = [0, 19, 43, 11, 57];
        for i in 0..5usize {
            let travel = ((self.elapsed_ms * BUBBLE_SPEEDS[i]) / 1000 + BUBBLE_PHASES[i]) % 70;
            let bx = 501 + (i as i32) * 22;
            let by = 195 - (travel as i32);
            self.video.draw_named_sprite(SpriteDrawCmd::new(
                "anim_gungan_bubbles",
                i % 3,
                bx,
                by,
                10,
            ));
        }

        // `DeffGunganLetters`: G-U-N-G-A-N progress on the right above the bumpers (`z = 15`)
        let (letters_lit, super_jets) = self
            .players
            .get(self.current_player)
            .map(|p| (p.gungan_letters as usize, p.super_jets_ms > 0))
            .unwrap_or((0, false));
        let blink_on = (self.elapsed_ms / 180) % 2 == 0;
        const GUNGAN: [&str; 6] = ["G", "U", "N", "G", "A", "N"];
        for (i, ch) in GUNGAN.iter().enumerate() {
            let lit = super_jets || i < letters_lit;
            let color = if super_jets {
                if blink_on { 0x7FE0 } else { 0x03FF }
            } else if lit {
                0x7FE0
            } else {
                0x218C
            };
            self.video.draw_font_text(TextDrawCmd {
                font_name: "font_shadow_9_narrow_data".into(),
                text: (*ch).to_string(),
                x: 508 + (i as i32) * 21,
                y: 112,
                z: 15,
                color_rgb555: color,
                shadow_rgb555: Some(0x0000),
                align: TextAlign::Center,
            });
        }
    }
}
