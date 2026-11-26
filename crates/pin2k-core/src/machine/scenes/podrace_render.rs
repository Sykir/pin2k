//! Authentic `DeffPodRaceSceneBackground` renderer (`swe1_0200.elf`, `0x00160a28`).

use pin2k_hal::{AudioBackend, PinballHardware, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend};

use super::podrace_state::{PodracePhase, PODRACE_LANE_X, PODRACE_LANE_Y};
use crate::machine::Pin2kMachine;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub(crate) fn render_podrace(&mut self, _secs: u32) {
        let Some(pr) = self.podrace.clone() else { return };

        // Sky (z=1) + Horizon (z=2)
        self.video.draw_named_sprite(SpriteDrawCmd::new("anim_podrace_sky", 0, 0, 0, 1));
        self.video.draw_named_sprite(SpriteDrawCmd::new(
            "anim_podrace_horizon", 0, -pr.horizon_x, 0, 2,
        ));

        if pr.phase == PodracePhase::StartingGate {
            let mut cmd = SpriteDrawCmd::new(
                "anim_podrace_starting",
                ((pr.phase_ms / 50) as usize).min(59),
                0,
                0,
                9,
            );
            cmd.scale_x = 2;
            self.video.draw_named_sprite(cmd);
            return;
        }

        // Ground (z=3)
        self.video.draw_named_sprite(SpriteDrawCmd::new(
            "anim_podrace_ground",
            ((pr.phase_ms / 50) as usize) % 21,
            0,
            112,
            3,
        ));

        if pr.phase == PodracePhase::Closeup {
            self.video.draw_named_sprite(SpriteDrawCmd::new(
                "anim_podrace_closeup",
                ((pr.phase_ms / 25) as usize).min(30),
                0,
                0,
                5,
            ));
        } else if pr.phase == PodracePhase::ZoomOut {
            let f = ((pr.phase_ms / 50) as usize).min(19);
            self.video.draw_named_sprite(SpriteDrawCmd::new(
                "anim_podrace_anakin_zoom",
                f,
                171,
                (132i32 - (f as i32) * 3).max(60),
                5,
            ));
        } else {
            if let Some(ref opp) = pr.opponent {
                let (idle_anim, exp_anim, _, _) = opp.sprite_info();
                let anim = if opp.exploding_ms > 0 { exp_anim } else { idle_anim };
                self.video.draw_named_sprite(SpriteDrawCmd::new(
                    anim,
                    (opp.anim_ms / 50) as usize,
                    opp.x_q8 >> 8,
                    opp.y_q8 >> 8,
                    4,
                ));
            }
            let (ax, ay, af) = match pr.anakin_bank {
                -1 => (9, 101, ((pr.phase_ms / 33) as usize) % 30),
                1 => (209, 101, 70 + ((pr.phase_ms / 33) as usize) % 25),
                _ => (109, 106, 30 + ((pr.phase_ms / 33) as usize) % 30),
            };
            self.video.draw_named_sprite(SpriteDrawCmd::new("anim_podrace_1", af, ax, ay, 5));
        }

        // Checkpoint badge (`z=6`) + number & name (`z=7`)
        let (bx, by) = if pr.checkpoint_banner_ms > 300 { (250, 51) } else { (10, 32) };
        self.video.draw_named_sprite(SpriteDrawCmd::new("anim_podrace_checkpoint", 0, bx, by, 6));
        self.video.draw_font_text(TextDrawCmd {
            font_name: "font_shadow_11_data".into(),
            text: format!("{}", pr.checkpoints),
            x: bx + 36,
            y: by + 28,
            z: 7,
            color_rgb555: 0x7FFF,
            shadow_rgb555: Some(0x6223),
            align: TextAlign::Center,
        });
        if pr.checkpoint_banner_ms > 0 {
            self.video.draw_font_text(TextDrawCmd {
                font_name: "font_shadow_7_data".into(),
                text: pr.checkpoint_label(),
                x: bx + 95,
                y: by + 61,
                z: 7,
                color_rgb555: 0x7FFF,
                shadow_rgb555: Some(0x7C00),
                align: TextAlign::Center,
            });
        }

        // Speed HUD (`z=8`)
        self.video.draw_font_text(TextDrawCmd {
            font_name: "font_shadow_7_data".into(),
            text: "SPEED".into(),
            x: 570,
            y: 51,
            z: 8,
            color_rgb555: 0x1CE7,
            shadow_rgb555: Some(0x7C00),
            align: TextAlign::Center,
        });
        self.video.draw_font_text(TextDrawCmd {
            font_name: "font_shadow_11_data".into(),
            text: format!("{}", pr.speed_mph),
            x: 570,
            y: 61,
            z: 8,
            color_rgb555: 0x1CE7,
            shadow_rgb555: Some(0x7C00),
            align: TextAlign::Center,
        });

        // 3-lane icons (`z=9`)
        for (i, lane) in pr.lanes.iter().enumerate() {
            if let Some(icon) = lane.icon {
                let (name, _, w, h) = icon.sprite_info();
                self.video.draw_named_sprite(SpriteDrawCmd::new(
                    name,
                    (lane.anim_ms / 50) as usize,
                    PODRACE_LANE_X[i] - w / 2,
                    PODRACE_LANE_Y[i] - h,
                    9,
                ));
            } else if lane.flying_ms > 0 {
                if let Some(icon) = lane.flying_kind {
                    let (_, blur_name, _, _) = icon.sprite_info();
                    self.video.draw_named_sprite(SpriteDrawCmd::new(
                        blur_name,
                        (lane.anim_ms / 50) as usize,
                        lane.x_q8 >> 8,
                        lane.y_q8 >> 8,
                        9,
                    ));
                }
            }
        }

        if pr.instruct_ms > 0 {
            self.video.draw_font_text(TextDrawCmd {
                font_name: "font_shadow_7_data".into(),
                text: "HIT CHECKPOINTS".into(),
                x: 320,
                y: 156,
                z: 8,
                color_rgb555: 0x7FE0,
                shadow_rgb555: Some(0x0000),
                align: TextAlign::Center,
            });
            self.video.draw_font_text(TextDrawCmd {
                font_name: "font_shadow_7_data".into(),
                text: "TO WIN RACE".into(),
                x: 320,
                y: 170,
                z: 8,
                color_rgb555: 0x7FE0,
                shadow_rgb555: Some(0x0000),
                align: TextAlign::Center,
            });
        }
    }
}
