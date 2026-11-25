//! Authentic Pure-Rust Renderers for All SWE1 System, End-of-Ball/Game,
//! Tilt, High-Score Entry (`DeffHSTDinitials`), and Pinball 2000 Diagnostics/Admin Screens.

use pin2k_hal::{
    AudioBackend, PinballHardware, RectDrawCmd, SpriteDrawCmd, TextAlign, TextDrawCmd, VideoBackend,
};

use super::{DiagPage, Pin2kMachine, SpecialScreen, HSTD_ALPHABET};
use crate::swe1_rules::{format_score_commas, JediRank};
use crate::swe1_tables::switch_name;

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    fn sys_sprite(&mut self, name: &str, frame: usize, x: i32, y: i32, z: u8, scale_x: u32) {
        let mut cmd = SpriteDrawCmd::new(name, frame, x, y, z);
        cmd.scale_x = scale_x;
        self.video.draw_named_sprite(cmd);
    }

    fn sys_text(
        &mut self,
        font: &str,
        text: impl Into<String>,
        x: i32,
        y: i32,
        z: u8,
        color_rgb555: u16,
        shadow_rgb555: Option<u16>,
        align: TextAlign,
    ) {
        self.video.draw_font_text(TextDrawCmd {
            font_name: font.to_string(),
            text: text.into(),
            x,
            y,
            z,
            color_rgb555,
            shadow_rgb555,
            align,
        });
    }

    fn sys_rect(&mut self, x: i32, y: i32, w: i32, h: i32, z: u8, color_rgb555: u16, alpha: u8) {
        self.video.draw_rect(RectDrawCmd {
            x,
            y,
            w,
            h,
            z,
            color_rgb555,
            alpha,
        });
    }

    /// `PowerupTestingDeff`: authentic Pinball 2000 boot & ROM verification display.
    pub(super) fn render_boot_screen(&mut self) {
        self.active_anim_hint.clear();
        self.sys_text(
            "font_system_15_data",
            "WILLIAMS PINBALL 2000",
            320,
            48,
            10,
            0x7FE0,
            Some(0x0000),
            TextAlign::Center,
        );
        self.sys_text(
            "font_system_11_data",
            self.game_title.title(),
            320,
            92,
            10,
            0x7FFF,
            Some(0x0000),
            TextAlign::Center,
        );
        self.sys_text(
            "font_system_9_data",
            "XINA / PRISM ROM CHECK ... OK",
            320,
            140,
            10,
            0x03E0,
            Some(0x0000),
            TextAlign::Center,
        );
        self.sys_text(
            "font_system_7_data",
            "PDB PARALLEL BUS & COIL SELF-TEST ... OK",
            320,
            174,
            10,
            0x5EF7,
            Some(0x0000),
            TextAlign::Center,
        );
    }

    /// Renders `DeffBonus`, `DeffMatch`, `DeffGameOver`, `DeffGameIntro`, `DeffExtraBallCollected`,
    /// `DeffBSBallSaved`, `DeffTiltWarning`, `DeffSlamTilt`, `DeffSubEscapeSceneBackground`,
    /// `DeffBuildC3P0Status`, `DeffRampCombo`, and `DeffJediMultiballReadyBackground`.
    pub(super) fn render_special_screen(&mut self, screen: SpecialScreen) {
        let f60 = ((self.elapsed_ms * 60) / 1000) as usize;
        let f15 = ((self.special_screen_ms * 15) / 1000) as usize;

        match screen {
            SpecialScreen::Bonus => {
                let (char_anim, rank_anim, base_bonus, mult) = self
                    .players
                    .get(self.current_player)
                    .map(|p| {
                        let r_anim = match p.jedi_rank {
                            JediRank::Padawan | JediRank::Youth => "anim_bonus_icon_jedi_youth",
                            JediRank::Knight => "anim_bonus_icon_jedi_knight",
                            JediRank::Master => "anim_bonus_icon_jedi_master",
                            JediRank::Spirit => "anim_bonus_icon_jedi_spirit",
                        };
                        (p.bonus_character_anim(), r_anim, p.bonus_points, p.bonus_multiplier.max(1))
                    })
                    .unwrap_or(("anim_bonus_character_quigon", "anim_bonus_icon_jedi_youth", 10_000, 1));
                let total = base_bonus.saturating_mul(mult as u64);

                self.sys_sprite("anim_bonus_background", 0, 0, 0, 2, 1);
                self.sys_sprite(char_anim, f60, 36, 76, 4, 1);
                self.sys_sprite(rank_anim, f60, 464, 78, 4, 1);
                self.sys_text(
                    "font_shadow_15_data",
                    "BONUS",
                    320,
                    48,
                    6,
                    0x7E00,
                    Some(0x18C6),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_shadow_9_data",
                    format!("BASE: {}", format_score_commas(base_bonus)),
                    320,
                    104,
                    6,
                    0x7FFF,
                    Some(0x18C6),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_shadow_9_data",
                    format!("MULTIPLIER: {mult}X"),
                    320,
                    134,
                    6,
                    0x7FE0,
                    Some(0x18C6),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_shadow_11_data",
                    format!("TOTAL: {}", format_score_commas(total)),
                    320,
                    168,
                    6,
                    0x03FF,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
            SpecialScreen::Match => {
                self.sys_sprite("anim_match_movie", f15, 0, 20, 2, 2);
                let match_num = if self.special_screen_ms < 2_000 {
                    (((self.special_screen_ms / 80) % 10) * 10) as u64
                } else {
                    (self.score % 100 / 10) * 10
                };
                self.sys_text(
                    "font_shadow_15_data",
                    "MATCH",
                    320,
                    14,
                    6,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_starwars_19_data",
                    format!("{match_num:02}"),
                    450,
                    130,
                    6,
                    0x7FFF,
                    Some(0x3DEF),
                    TextAlign::Center,
                );
                let p_last = self.score % 100;
                let verdict = if self.special_screen_ms >= 2_000 && p_last / 10 == match_num / 10 {
                    format!("P1 {:02} MATCHES - FREE GAME!", p_last)
                } else {
                    format!("P1 LAST TWO DIGITS: {:02}", p_last)
                };
                self.sys_text(
                    "font_shadow_11_narrow_data",
                    verdict,
                    320,
                    206,
                    6,
                    0x7FFF,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
            SpecialScreen::GameOver => {
                self.sys_sprite("anim_darth_maul_game_over", f15, 0, 0, 2, 2);
                self.sys_text(
                    "font_starwars_19_data",
                    "GAME OVER",
                    320,
                    174,
                    6,
                    0x7C00,
                    Some(0x0000),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_shadow_11_narrow_data",
                    format!("FINAL SCORE: {}", format_score_commas(self.score)),
                    320,
                    214,
                    6,
                    0x7FFF,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
            SpecialScreen::GameIntro => {
                self.sys_sprite("anim_republic_cruiser_intro", f15, 0, 50, 2, 2);
                self.sys_text(
                    "font_shadow_15_data",
                    "STAR WARS EPISODE I",
                    320,
                    18,
                    6,
                    0x7FE7,
                    Some(0x0000),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_shadow_9_data",
                    "A LONG TIME AGO IN A GALAXY FAR, FAR AWAY...",
                    320,
                    202,
                    6,
                    0x03FF,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
            SpecialScreen::ExtraBall => {
                self.sys_sprite("anim_extra_ball_show", f15, 0, 0, 2, 2);
                self.sys_text(
                    "font_shadow_15_data",
                    "EXTRA BALL!",
                    320,
                    184,
                    6,
                    0x7FE0,
                    Some(0x001F),
                    TextAlign::Center,
                );
            }
            SpecialScreen::BallSave => {
                self.sys_rect(120, 84, 400, 88, 23, 0x0008, 210);
                self.sys_text(
                    "font_shadow_15_data",
                    "BALL SAVED",
                    320,
                    100,
                    24,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_system_11_data",
                    "SHOOT AGAIN",
                    320,
                    138,
                    24,
                    0x7FFF,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
            SpecialScreen::TiltWarning => {
                self.sys_rect(110, 92, 420, 56, 239, 0x3800, 200);
                self.sys_text(
                    "font_shadow_15_data",
                    "DANGER! DANGER!",
                    320,
                    108,
                    240,
                    0x7FE0,
                    Some(0x7C00),
                    TextAlign::Center,
                );
            }
            SpecialScreen::SlamTilt => {
                self.sys_rect(0, 0, 640, 240, 239, 0x0000, 225);
                self.sys_text(
                    "font_starwars_19_data",
                    "SLAM TILT",
                    320,
                    104,
                    240,
                    0x7C00,
                    Some(0x7FE0),
                    TextAlign::Center,
                );
            }
            SpecialScreen::SubEscape => {
                self.sys_sprite("anim_gungan_seaweed", f60, 0, 40, 1, 1);
                self.sys_sprite("anim_gungan_sub_bubbles_2", f60, 35, 0, 2, 1);
                self.sys_sprite("anim_gungan_sub_sub", f60, 80, 84, 4, 1);
                self.sys_sprite("anim_gungan_sub_chomp", f60, 0, 0, 5, 1);
                self.sys_text(
                    "font_kahuna_shadow_9_data",
                    "SUB ESCAPE! SHOOT RAMPS!",
                    320,
                    208,
                    7,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
            SpecialScreen::BuildC3P0 => {
                self.sys_sprite("anim_build_c3p0", f60, 10, 44, 5, 1);
                self.sys_text(
                    "font_shadow_15_data",
                    "BUILD C-3PO",
                    380,
                    72,
                    6,
                    0x7E00,
                    Some(0x0000),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_shadow_7_narrow_data",
                    "COMPLETE TOP LANES TO BUILD PARTS",
                    380,
                    124,
                    6,
                    0x7FFF,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
            SpecialScreen::RampCombo => {
                let x = 640 - ((f60 * 14) % 900) as i32;
                self.sys_sprite("anim_naboo_flyby_single", 0, x, 60, 6, 1);
                self.sys_text(
                    "font_shadow_11_data",
                    "RAMP COMBO!",
                    320,
                    130,
                    7,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
            SpecialScreen::JediMultiballReady => {
                self.sys_sprite("anim_saber_ball_loop", f60, 168, 54, 4, 1);
                self.sys_text(
                    "font_shadow_15_narrow_data",
                    "JEDI MULTIBALL READY!",
                    320,
                    204,
                    7,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
            }
        }
    }

    /// `DeffTilt`: full translucent TILT overlay above playfield HUD.
    pub(super) fn render_tilt_overlay(&mut self) {
        self.sys_rect(0, 42, 640, 198, 239, 0x0000, 210);
        self.sys_text(
            "font_starwars_19_data",
            "TILT",
            320,
            96,
            240,
            0x7C00,
            Some(0x7FE0),
            TextAlign::Center,
        );
        self.sys_text(
            "font_shadow_11_data",
            "BALL LOST",
            320,
            148,
            240,
            0x7FFF,
            Some(0x0000),
            TextAlign::Center,
        );
    }

    /// `DeffHSTDinitials`: authentic high score table initials entry UI (`z=2..10`).
    pub(super) fn render_hstd_entry_screen(&mut self) {
        self.sys_rect(24, 10, 592, 220, 2, 0x0008, 225);
        self.sys_rect(24, 10, 592, 3, 3, 0x7FE8, 255);
        self.sys_rect(24, 227, 592, 3, 3, 0x7FE8, 255);

        let (p_num, p_score, p_rank) = self
            .hstd_session
            .queue
            .first()
            .copied()
            .unwrap_or((1, self.score, JediRank::Padawan));

        self.sys_text(
            "font_starwars_19_data",
            "HIGHEST SCORE!",
            320,
            18,
            10,
            0x7FE8,
            Some(0x3DEF),
            TextAlign::Center,
        );
        self.sys_text(
            "font_shadow_11_narrow_data",
            format!("PLAYER {p_num}  -  {} ({})", format_score_commas(p_score), p_rank.label()),
            320,
            60,
            10,
            0x7FFF,
            Some(0x0000),
            TextAlign::Center,
        );

        for i in 0..3 {
            let bx = 235 + (i as i32) * 85;
            let active = i == self.hstd_session.cursor;
            self.sys_rect(
                bx - 26,
                92,
                52,
                46,
                8,
                if active { 0x019F } else { 0x18C6 },
                240,
            );
            let ch = self.hstd_session.initials[i];
            self.sys_text(
                "font_starwars_19_data",
                ch.to_string(),
                bx,
                98,
                10,
                if active { 0x7FE0 } else { 0x7FFF },
                Some(0x0000),
                TextAlign::Center,
            );
        }

        // Compact A..Z 0..9 selector window centered around active `char_idx`
        let len = HSTD_ALPHABET.len();
        let cur_idx = self.hstd_session.char_idx % len;
        let mut strip = String::new();
        for off in -7i32..=7 {
            let idx = (cur_idx as i32 + off).rem_euclid(len as i32) as usize;
            let ch = HSTD_ALPHABET[idx] as char;
            let display = if ch == ' ' { '_' } else { ch };
            if off == 0 {
                strip.push('[');
                strip.push(display);
                strip.push(']');
            } else {
                strip.push(display);
            }
            if off < 7 {
                strip.push(' ');
            }
        }
        self.sys_text(
            "font_system_9_data",
            strip,
            320,
            166,
            10,
            0x03FF,
            Some(0x0000),
            TextAlign::Center,
        );
        self.sys_text(
            "font_system_7_data",
            "FLIPPERS: SELECT LETTER   START: CONFIRM",
            320,
            208,
            10,
            0x5EF7,
            Some(0x0000),
            TextAlign::Center,
        );
    }

    /// Pinball 2000 Operator Administration & Diagnostics (`Diagnostic*Deff` & `MenuGUIBackgndDeff`).
    pub(super) fn render_diagnostics_screen(&mut self) {
        self.active_anim_hint.clear();
        match self.diag_page {
            DiagPage::AlignTest => {
                self.sys_sprite("system_video_test_align", 0, 0, 0, 5, 1);
                return;
            }
            DiagPage::ColorBarTest => {
                const BARS: [u16; 8] = [
                    0x7FFF, 0x7FE0, 0x03FF, 0x03E0, 0x7C1F, 0x7C00, 0x001F, 0x0000,
                ];
                for (i, &c) in BARS.iter().enumerate() {
                    self.sys_rect((i as i32) * 80, 0, 80, 240, 5, c, 255);
                }
                self.sys_rect(140, 206, 360, 26, 9, 0x0000, 200);
                self.sys_text(
                    "font_system_9_data",
                    "DIAGNOSTIC: SMPTE COLOR BARS",
                    320,
                    212,
                    10,
                    0x7FFF,
                    None,
                    TextAlign::Center,
                );
                return;
            }
            DiagPage::GridTest => {
                self.sys_rect(0, 0, 640, 240, 1, 0x0000, 255);
                for x in (0..640).step_by(40) {
                    self.sys_rect(x, 0, 1, 240, 5, 0x7FFF, 255);
                }
                for y in (0..240).step_by(24) {
                    self.sys_rect(0, y, 640, 1, 5, 0x7FFF, 255);
                }
                self.sys_rect(0, 0, 640, 2, 6, 0x7C00, 255);
                self.sys_rect(0, 238, 640, 2, 6, 0x7C00, 255);
                self.sys_rect(0, 0, 2, 240, 6, 0x7C00, 255);
                self.sys_rect(638, 0, 2, 240, 6, 0x7C00, 255);
                self.sys_rect(170, 212, 300, 20, 9, 0x0000, 210);
                self.sys_text(
                    "font_system_7_data",
                    "CONVERGENCE GRID TEST",
                    320,
                    216,
                    10,
                    0x7FE0,
                    None,
                    TextAlign::Center,
                );
                return;
            }
            DiagPage::RgbTest => {
                self.sys_rect(0, 0, 640, 80, 5, 0x7C00, 255);
                self.sys_rect(0, 80, 640, 80, 5, 0x03E0, 255);
                self.sys_rect(0, 160, 640, 80, 5, 0x001F, 255);
                self.sys_text(
                    "font_system_9_data",
                    "RED / GREEN / BLUE PURITY TEST",
                    320,
                    12,
                    10,
                    0x7FFF,
                    Some(0x0000),
                    TextAlign::Center,
                );
                return;
            }
            DiagPage::FuseTest => {
                self.render_menu_backdrop("DIAGNOSTICS: PDB FUSE STATUS");
                let hv_ok = self.coin_door_closed;
                const FUSES: [(&str, &str, bool); 6] = [
                    ("F100", "50V HIGH-POWER COILS", true),
                    ("F101", "50V FLIPPER SOLENOIDS", true),
                    ("F102", "20V FLASHER LAMPS", true),
                    ("F103", "18V LAMP MATRIX A+B", false),
                    ("F104", "12V OPTO & UNREGULATED", false),
                    ("F105", "5V DIGITAL LOGIC", false),
                ];
                for (i, (code, desc, hv)) in FUSES.iter().enumerate() {
                    let y = 56 + (i as i32) * 24;
                    let ok = !*hv || hv_ok;
                    self.sys_text(
                        "font_system_9_data",
                        format!("{code}  {desc}"),
                        48,
                        y,
                        10,
                        0x7FFF,
                        Some(0x0000),
                        TextAlign::Left,
                    );
                    self.sys_text(
                        "font_system_9_data",
                        if ok { "OK" } else { "DOOR OPEN (HV OFF)" },
                        592,
                        y,
                        10,
                        if ok { 0x03E0 } else { 0x7FE0 },
                        Some(0x0000),
                        TextAlign::Right,
                    );
                }
                return;
            }
            DiagPage::SwitchTest | DiagPage::CoilTest { .. } | DiagPage::LampTest { .. } => {}
        }

        match self.admin_screen {
            crate::admin::AdminScreen::SwitchTest => {
                self.render_menu_backdrop("DIAGNOSTIC: SWITCH MATRIX");
                // 8×8 Playfield Matrix + 4 Direct Columns
                for col in 0..12usize {
                    for row in 0..8usize {
                        let closed = (self.cached_switches[col] & (1 << row)) != 0;
                        let bx = 28 + (col as i32) * 24;
                        let by = 54 + (row as i32) * 18;
                        self.sys_rect(
                            bx,
                            by,
                            18,
                            13,
                            8,
                            if closed { 0x03E0 } else { 0x18C6 },
                            255,
                        );
                    }
                }
                let sw_label = match self.last_switch_hit {
                    Some(id) => format!("SW #{id:02}: {}", switch_name(id)),
                    None => "ACTIVATE ANY SWITCH".to_string(),
                };
                self.sys_text(
                    "font_system_11_data",
                    sw_label,
                    480,
                    58,
                    10,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
                self.sys_text(
                    "font_system_7_data",
                    format!("COIN DOOR: {}", if self.coin_door_closed { "CLOSED" } else { "OPEN" }),
                    480,
                    96,
                    10,
                    0x7FFF,
                    Some(0x0000),
                    TextAlign::Center,
                );
                return;
            }
            crate::admin::AdminScreen::LampTest { selected_lamp } => {
                self.render_menu_backdrop("DIAGNOSTIC: LAMP MATRIX A / B");
                for idx in 0..128u8 {
                    let bank = idx / 64;
                    let local = idx % 64;
                    let col = local / 8;
                    let row = local % 8;
                    let bx = 28 + (bank as i32) * 176 + (col as i32) * 20;
                    let by = 56 + (row as i32) * 17;
                    let sel = idx == selected_lamp;
                    self.sys_rect(bx, by, 14, 12, 8, if sel { 0x7FE0 } else { 0x18C6 }, 255);
                }
                self.sys_text(
                    "font_system_11_data",
                    format!(
                        "LAMP #{selected_lamp:03} (BANK {})",
                        if selected_lamp < 64 { 'A' } else { 'B' }
                    ),
                    496,
                    72,
                    10,
                    0x7FE0,
                    Some(0x0000),
                    TextAlign::Center,
                );
                return;
            }
            _ => {}
        }

        // Standard Pinball 2000 Operator Menu / CoilTest GUI (`MenuGUIBackgndDeff`)
        let (title, rows) = self.admin_display_lines();
        self.render_menu_backdrop(&title);
        for (i, (line, selected)) in rows.iter().enumerate() {
            let y = 58 + (i as i32) * 28;
            if *selected {
                self.sys_rect(44, y - 4, 552, 22, 8, 0x019F, 220);
            }
            let prefix = if *selected { "> " } else { "  " };
            let color = if *selected { 0x7FE0 } else { 0x7FFF };
            self.sys_text(
                "font_system_9_data",
                format!("{prefix}{line}"),
                64,
                y,
                10,
                color,
                Some(0x0000),
                TextAlign::Left,
            );
        }
        self.sys_text(
            "font_system_7_data",
            "UP/DN: SELECT   ENTER: OK   ESC: BACK",
            320,
            214,
            10,
            0x5EF7,
            Some(0x0000),
            TextAlign::Center,
        );
    }

    fn render_menu_backdrop(&mut self, title: &str) {
        self.sys_rect(12, 8, 616, 224, 2, 0x0010, 235);
        self.sys_rect(12, 8, 616, 30, 4, 0x019F, 245);
        self.sys_rect(12, 38, 616, 2, 5, 0x7FE0, 255);
        self.sys_text(
            "font_system_11_data",
            title,
            320,
            14,
            10,
            0x7FFF,
            Some(0x0000),
            TextAlign::Center,
        );
    }
}
