    use super::*;
    use pin2k_hal::{HeadlessAudio, HeadlessVideo, PdbDriver, VirtualPdbPort};
    use crate::{AdminScreen, GameTitle, ModManifest, RfmWave, Swe1Scene};
    use crate::rfm_tables::rfm_sw;
    use crate::swe1_tables::{drive, lamp, sw};

    #[test]
    fn full_swe1_game_loop_runs_over_pdb_parallel_port_protocol() {
        let vboard = VirtualPdbPort::default();
        let pdb = PdbDriver::new(vboard);
        let video = HeadlessVideo::default();
        let mut machine = Pin2kMachine::new(pdb, video);

        // Press Start Button (Sw 2 = M13) for 2 ticks -> transitions Attract -> InGame & fires Trough Eject (Drive 8)
        machine.hw.port.set_logical_switch(sw::START_BUTTON, true);
        machine.tick(16);
        machine.tick(16);
        machine.hw.port.set_logical_switch(sw::START_BUTTON, false);
        machine.tick(16);

        assert_eq!(machine.state, MachineState::InGame);
        assert!(
            machine.hw.port.is_drive_active(drive::TROUGH_EJECT),
            "Trough eject solenoid energized over PDB register 0x0A"
        );

        // Spell J-E-D-I with 4 Left Ramp Made shots -> triggers Darth Maul Lightsaber Duel & Lightsaber Neon (Drive 40)!
        for _ in 0..4 {
            machine.hw.port.set_logical_switch(sw::RAMP_MADE_LEFT, true);
            machine.tick(16);
            machine.tick(16);
            machine.hw.port.set_logical_switch(sw::RAMP_MADE_LEFT, false);
            machine.tick(16);
            machine.tick(16);
        }

        assert!(matches!(
            machine.multiball,
            ActiveMultiball::DarthMaulDuel { .. }
        ));
        // Sync one more tick so Lightsaber Neon hold mask flushes to PDB register 0x0E
        machine.tick(16);
        assert!(
            machine.hw.port.is_drive_active(drive::LIGHTSABER_NEON),
            "Lightsaber Neon energized on PDB register 0x0E bit 0"
        );
        assert!(machine.score >= 500_000);
    }

    fn pulse_switch<H: PinballHardware, V: VideoBackend, A: AudioBackend>(
        m: &mut Pin2kMachine<H, V, A>,
        set_fn: impl Fn(&mut H, u8, bool),
        sw_id: u8,
    ) {
        set_fn(&mut m.hw, sw_id, true);
        m.tick(16);
        m.tick(16);
        set_fn(&mut m.hw, sw_id, false);
        m.tick(16);
        m.tick(16);
    }

    #[test]
    fn swe1_opto_polarity_shield_scene_hangar_multiball_and_diagnostics() {
        let vboard = VirtualPdbPort::default();
        let pdb = PdbDriver::new(vboard);
        let mut machine = Pin2kMachine::new(pdb, HeadlessVideo::default());

        // 1. Verify Operator Admin Menu -> Diagnostics -> Coil Test -> Adjustments -> Escape
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ENTER);
        assert_eq!(machine.state, MachineState::Diagnostics);
        assert_eq!(machine.admin_screen, AdminScreen::Root { cursor: 0 });

        // Enter Diagnostics -> Up to Coil Test -> Enter -> Up to Coil 01
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ENTER);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_UP);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ENTER);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_UP);
        assert_eq!(machine.diag_page, DiagPage::CoilTest { selected_drive: 1 });

        // Back out to Root -> Adjustments -> toggle Free Play off and back on -> Escape to Attract
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ESCAPE);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ESCAPE);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_UP);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_UP);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ENTER);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ENTER);
        assert!(!machine.free_play);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ENTER);
        assert!(machine.free_play);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ESCAPE);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::DIAG_ESCAPE);
        assert_eq!(machine.state, MachineState::Attract);

        // 2. Start 2-player game & verify Skill Shot
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        assert_eq!(machine.players.len(), 2);

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::JETS_ROLLOVER);
        assert!(machine.score >= 500_000, "Skill shot awarded");

        // 3. Deplete Deflector Shield & enter Shield Popper (Sw 32 active-low opto!) -> Battle Droid Scene
        for _ in 0..3 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        }
        assert!(machine.shield_raised, "Motorized Deflector Shield raised when health == 0");

        // Verify physical wire bit for opto Sw 32 (Col 4 bit 0) drops to 0 when blocked!
        machine.hw.port.set_logical_switch(sw::SHIELD_POPPER, true);
        assert_eq!(
            machine.hw.port.matrix_cols[4] & 0x01,
            0x00,
            "Blocked SWE1 opto Sw 32 drives active-low 0 on physical PDB wire"
        );
        machine.tick(16);
        machine.tick(16);
        machine.hw.port.set_logical_switch(sw::SHIELD_POPPER, false);
        assert_eq!(
            machine.hw.port.matrix_cols[4] & 0x01,
            0x01,
            "Unblocked SWE1 opto Sw 32 returns to 1 on physical PDB wire"
        );
        machine.tick(16);
        assert!(
            matches!(
                machine.active_scene,
                Some(ActiveSceneState {
                    scene: Swe1Scene::BattleDroid,
                    ..
                })
            ),
            "Shield Popper opto started Battle Droid Invasion scene"
        );

        // Charge Lasers on standup target & complete Battle Droid Invasion!
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_BANK_UPPER);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_ACTION_BUTTON);
        for _ in 0..4 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_RIGHT);
        }
        assert!(machine.active_scene.is_none(), "Scene completed");
        assert_eq!(machine.players[0].scenes_completed & 1, 1);

        // 4. Lock 3 balls in Saucers -> Theed Hangar 3-Ball Multiball -> 2x Jackpot -> Super Jackpot!
        for _ in 0..3 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_SAUCER);
        }
        assert!(matches!(
            machine.multiball,
            ActiveMultiball::HangarMultiball { .. }
        ));
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_RIGHT);
        let pre_super = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_POPPER);
        assert!(
            machine.score >= pre_super + 3_000_000,
            "Super Jackpot awarded in Shield Popper"
        );
    }

    #[test]
    fn full_rfm_game_loop_runs_over_pdb_parallel_port_protocol() {
        let vboard = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Rfm);
        let pdb = PdbDriver::new(vboard);
        let mut machine = Pin2kMachine::with_game(
            pdb,
            HeadlessVideo::default(),
            HeadlessAudio::default(),
            GameTitle::Rfm,
        );

        assert_eq!(machine.game_title, GameTitle::Rfm);
        assert_eq!(
            machine.hw.probe_playfield_signature(),
            pin2k_hal::PlayfieldSignature::Rfm
        );

        // Start RFM game -> Skill Shot -> Martian Attack -> Mars Kneads Women Wave!
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), rfm_sw::START_BUTTON);
        assert_eq!(machine.state, MachineState::InGame);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), rfm_sw::LEFT_TOP_LANE);
        assert!(machine.score >= 500_000);

        for t in [
            rfm_sw::MARTIAN_TARGET_1,
            rfm_sw::MARTIAN_TARGET_2,
            rfm_sw::MARTIAN_TARGET_3,
            rfm_sw::MARTIAN_TARGET_4,
            rfm_sw::MARTIAN_TARGET_5,
            rfm_sw::MARTIAN_TARGET_6,
            rfm_sw::MARTIAN_TARGET_7,
        ] {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), t);
        }
        assert!(matches!(machine.rfm.multiball, RfmMultiball::MartianAttack));

        pulse_switch(
            &mut machine,
            |hw, s, c| hw.port.set_logical_switch(s, c),
            rfm_sw::CENTER_DEFLECTOR_PANEL,
        );
        assert_eq!(machine.rfm.active_wave, Some(RfmWave::MarsKneadsWomen));
    }

    #[test]
    fn homebrew_mod_manifest_drives_custom_rules_and_scoring() {
        let vboard = VirtualPdbPort::default();
        let pdb = PdbDriver::new(vboard);
        let mut machine = Pin2kMachine::with_game(
            pdb,
            HeadlessVideo::default(),
            HeadlessAudio::default(),
            GameTitle::Homebrew,
        );

        let manifest = ModManifest {
            id: "space-cadet-hb".into(),
            name: "Space Cadet Homebrew".into(),
            author: "Pin2k Community".into(),
            version: "1.0.0".into(),
            base_game: Some("homebrew".into()),
            attract_banner: Some("SPACE CADET - INSERT COIN".into()),
            balls_per_game: Some(5),
            ball_save_secs: Some(20),
            score_multiplier: Some(2),
            switch_rules: vec![HomebrewSwitchRule {
                sw_id: sw::RAMP_MADE_LEFT,
                points: 100_000,
                coil_id: Some(drive::LIGHTSABER_NEON),
                pulse_ms: 80,
                sound_id: Some(0x0015),
                lamp_id: Some(lamp::EXTRA_BALL),
                banner: Some("WARP RAMP! +200,000".into()),
                anim_hint: Some("warp_anim".into()),
            }],
            ..Default::default()
        };
        machine.apply_mod(&manifest);
        assert_eq!(machine.balls_per_game, 5);
        assert_eq!(machine.status_banner, "SPACE CADET - INSERT COIN");

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        assert_eq!(machine.score, 200_000);
        assert_eq!(machine.active_anim_hint, "warp_anim");
        assert!(machine.hw.port.is_drive_active(drive::LIGHTSABER_NEON));
    }

    #[test]
    fn high_score_entry_lets_player_pick_initials_and_persists() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        // Start 1-player game with a top high score
        machine.adjustments.ball_save_secs = 0;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        machine.add_score(95_000_000);

        // Drain balls until Game Over -> HighScoreEntry (handles any replay/extra ball)
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, !c), sw::TROUGH_BALL_1);
        for _ in 0..10 {
            if machine.state != MachineState::InGame {
                break;
            }
            machine.eject_cooldown_ms = 0;
            machine.ball_save_ms = 0;
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::TROUGH_BALL_1);
            for _ in 0..10 {
                machine.tick(16);
            }
        }

        assert_eq!(machine.state, MachineState::HighScoreEntry);

        // Select 'B' (1 right flip) -> confirm, 'C' (2 right flips) -> confirm, 'A' -> confirm
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_FLIPPER_BUTTON);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_FLIPPER_BUTTON);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_FLIPPER_BUTTON);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);

        assert_eq!(machine.state, MachineState::Attract);
        assert_eq!(machine.high_scores[0].initials, ['B', 'C', 'A']);
        assert!(machine.high_scores[0].score >= 95_000_000);
    }

    #[test]
    fn debug_menu_launches_all_states_scenes_and_games() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        assert!(machine.debug_launch_mode("debug"));
        assert_eq!(machine.state, MachineState::Diagnostics);
        assert_eq!(machine.admin_screen, AdminScreen::DebugLauncher { cursor: 0 });

        assert!(machine.debug_launch_mode("scene:6"));
        assert_eq!(machine.state, MachineState::InGame);
        assert_eq!(machine.active_scene.as_ref().map(|s| s.scene), Some(Swe1Scene::Podrace));

        assert!(machine.debug_launch_mode("rfm_wave:8"));
        assert_eq!(machine.game_title, GameTitle::Rfm);
        assert_eq!(machine.rfm.active_wave, Some(RfmWave::AttackMars));

        assert!(machine.debug_launch_mode("hstd_entry"));
        assert_eq!(machine.state, MachineState::HighScoreEntry);
    }

    #[test]
    fn score_display_renders_character_names_scores_and_sabers() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        // Start 2-player SWE1 game
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        assert_eq!(machine.players.len(), 2);
        let p0_name = machine.players[0].character_name();
        let p1_name = machine.players[1].character_name();
        assert_ne!(p0_name, p1_name);
        assert!(crate::swe1_rules::PLAYER_CHARACTER_NAMES.contains(&p0_name));
        assert!(crate::swe1_rules::PLAYER_CHARACTER_NAMES.contains(&p1_name));

        machine.add_score(1_234_560);
        machine.tick(16);

        let texts = &machine.video.last_font_text_cmds;
        assert!(texts.iter().any(|t| {
            t.text == p0_name && t.x == 36 && t.y == 8 && t.z == 10 && t.align == pin2k_hal::TextAlign::Left
        }));
        assert!(texts.iter().any(|t| {
            t.text == "1,234,560" && t.x > 36 && t.y == 7 && t.z == 10 && t.align == pin2k_hal::TextAlign::Left
        }));
        assert!(texts.iter().any(|t| {
            t.text == p1_name && t.x < 525 && t.y == 8 && t.z == 10 && t.align == pin2k_hal::TextAlign::Right
        }));
        assert!(texts.iter().any(|t| {
            t.text == "0" && t.x == 525 && t.y == 7 && t.z == 10 && t.align == pin2k_hal::TextAlign::Right
        }));

        let sprites = &machine.video.last_sprite_cmds;
        assert!(sprites
            .iter()
            .any(|s| s.anim_name == "anim_saber_handle" && s.x == 261 && s.y == 7 && s.z == 9));
        assert!(sprites
            .iter()
            .any(|s| s.anim_name.starts_with("anim_saber_") && s.x == 16 && s.y == 7 && s.z == 9 && !s.flip_x));
        assert!(sprites
            .iter()
            .any(|s| s.anim_name.starts_with("anim_saber_") && s.x == 300 && s.y == 7 && s.z == 9 && s.flip_x));
    }

    #[test]
    fn laser_status_hud_shows_ready_and_depleted_states() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        machine.tick(16);

        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_naboo_fighter_tiny_rotate"
                && s.x == 535
                && s.y == 10
                && s.z == 20
                && s.monochrome_rgb555.is_none()
        }));
        assert!(machine
            .video
            .last_font_text_cmds
            .iter()
            .any(|t| (t.text == "LASER" || t.text == "LASERS") && t.z == 21));

        machine.players[0].laser_shots = 0;
        machine.tick(16);

        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_naboo_fighter_tiny_rotate"
                && s.x == 535
                && s.y == 10
                && s.z == 20
                && s.monochrome_rgb555 == Some(0x294A)
        }));
        assert!(machine
            .video
            .last_font_text_cmds
            .iter()
            .any(|t| t.text == "0" && t.x == 585 && t.y == 35 && t.z == 21));
    }

    #[test]
    fn virtual_spinners_stay_idle_until_ramp_pass_and_play_turnstile_sound() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        for _ in 0..20 {
            machine.tick(16);
        }
        assert!(!machine.left_spinner.active);
        assert!(!machine.right_spinner.active);
        assert!(!machine
            .video
            .last_sprite_cmds
            .iter()
            .any(|s| s.anim_name.starts_with("anim_spinner_")));

        // Left ramp pass triggers left virtual spinner + turnstile sound
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
        machine.tick(16);
        assert!(machine.left_spinner.active);
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_bracket_left" && s.x == 43 && s.y == 174 && s.z == 22
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_1" && s.x == 43 && s.y == 174 && s.z == 22
        }));
        assert!(machine
            .audio
            .played
            .iter()
            .any(|(sid, _)| *sid == crate::swe1_tables::sound::SPINNER_TICKS[0]));

        // Right ramp pass triggers right virtual spinner
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_RAMP_ENTER);
        machine.tick(16);
        assert!(machine.right_spinner.active);
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_bracket_right" && s.x == 421 && s.y == 174 && s.z == 22
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_2" && s.x == 421 && s.y == 174 && s.z == 22
        }));
    }

    #[test]
    fn attract_kahuna_logo_renders_tinted_light_flare_and_spinning_logo() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        assert!(machine.debug_launch_mode("attract:0"));
        for _ in 0..5 {
            machine.tick(16);
        }

        let sprites = &machine.video.last_sprite_cmds;
        assert!(sprites.iter().any(|s| {
            s.anim_name == "anim_kahuna_logo_flare"
                && s.x == 23
                && s.y == 7
                && s.z == 7
                && s.monochrome_rgb555.is_some()
        }));
        assert!(sprites.iter().any(|s| {
            s.anim_name == "anim_kahuna_logo_spinning" && s.x == 187 && s.y == 80 && s.z == 8
        }));
        assert!(machine.video.last_font_text_cmds.iter().any(|t| {
            t.z == 250 && (t.text.contains("FREE PLAY") || t.text.contains("PRESS START"))
        }));
    }

    #[test]
    fn debug_launch_mode_and_player_cmds_cover_all_modes() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        // Must transition directly from Boot -> InGame when debug-launching in-game modes
        assert_eq!(machine.state, MachineState::Boot);
        assert!(machine.debug_launch_mode("ingame"));
        assert_eq!(machine.state, MachineState::InGame);
        assert!(machine.debug_launch_mode("boot"));
        assert_eq!(machine.state, MachineState::Boot);
        assert!(machine.debug_launch_mode("scene:6"));
        assert_eq!(machine.state, MachineState::InGame);

        for mode in [
            "game:swe1",
            "game:rfm",
            "attract:0",
            "attract:19",
            "scene:0",
            "scene:12",
            "scene:13",
            "sub_escape",
            "skill_shot",
            "holo_select",
            "hangar_multiball",
            "darth_maul_duel",
            "spinner_left",
            "spinner_right",
            "bonus",
            "match",
            "game_over",
            "hstd_entry",
            "diag:align",
        ] {
            assert!(machine.debug_launch_mode(mode), "failed debug_launch_mode({mode})");
        }

        assert!(machine.debug_launch_mode("game:swe1"));
        assert!(machine.debug_launch_mode("ingame"));
        let prev_score = machine.players[0].score;

        machine.debug_add_score(1_000_000);
        assert!(machine.players[0].score >= prev_score + 1_000_000);
        assert!(machine.debug_apply_player_cmd("lasers:5"));
        assert_eq!(machine.players[0].laser_shots, 5);
        assert!(machine.debug_apply_player_cmd("jedi:7"));
        assert_eq!(machine.players[0].jedi_letters, 7);
        assert!(machine.debug_apply_player_cmd("shield:0"));
        assert_eq!(machine.players[0].shield_health, 0);
        assert!(machine.debug_apply_player_cmd("rank:3"));
        assert_eq!(machine.players[0].jedi_rank, crate::swe1_rules::JediRank::Master);
        assert!(machine.debug_apply_player_cmd("ball:2"));
        assert_eq!(machine.current_ball, 2);
        assert!(machine.debug_apply_player_cmd("add_player"));
        assert_eq!(machine.players.len(), 2);
        assert!(machine.debug_apply_player_cmd("next_player"));
        assert_eq!(machine.current_player, 1);
    }

    #[test]
    fn all_14_scenes_render_intro_and_active_background_layers() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        for (idx, &scene) in Swe1Scene::ALL.iter().enumerate() {
            assert!(machine.debug_launch_mode(&format!("scene:{idx}")));
            machine.tick(16);

            // Intro: black backdrop (z=0), 2x-stretched movie centered in oval (100,31,z=200),
            // bezel (59,14,z=202), centered badge on top (z=203), and no gameplay HUD overlap.
            assert!(
                machine.video.last_rect_cmds.iter().any(|r| r.z == 0 && r.w == 640 && r.h == 240),
                "missing opaque intro backdrop for {scene:?}"
            );
            assert!(
                machine.video.last_sprite_cmds.iter().any(|s| {
                    s.anim_name == scene.intro_anim_hint()
                        && s.x == 100
                        && s.y == 31
                        && s.scale_x == 2
                        && s.z == 200
                }),
                "missing centered 2x intro movie for {scene:?}"
            );
            assert!(
                machine.video.last_sprite_cmds.iter().any(|s| {
                    s.anim_name == "anim_jedi_scene_intro_frame"
                        && s.x == 59
                        && s.y == 14
                        && s.z == 202
                }),
                "missing intro frame for {scene:?}"
            );
            assert!(
                machine.video.last_sprite_cmds.iter().any(|s| {
                    s.anim_name == scene.intro_badge_anim()
                        && s.x == 320 - scene.intro_badge_width() / 2
                        && s.z == 203
                }),
                "missing centered intro badge for {scene:?}"
            );
            assert!(
                !machine
                    .video
                    .last_sprite_cmds
                    .iter()
                    .any(|s| s.anim_name == "anim_saber_handle"),
                "gameplay HUD must not overlap scene intro for {scene:?}"
            );

            // Skip intro and verify active scene rendering
            if let Some(ref mut sc) = machine.active_scene {
                sc.intro_remaining_ms = 0;
            }
            machine.active_anim_hint = scene.gameplay_anim_hint().to_string();
            machine.tick(16);

            if scene == Swe1Scene::Podrace {
                assert!(
                    machine
                        .video
                        .last_sprite_cmds
                        .iter()
                        .any(|s| s.anim_name.starts_with("anim_podrace_") && s.z <= 6),
                    "missing Podrace layers in pin2k-core"
                );
                assert!(machine
                    .video
                    .last_font_text_cmds
                    .iter()
                    .any(|t| t.z == 8 && t.text.contains("LAP")));
            } else {
                assert!(
                    machine.video.last_sprite_cmds.iter().any(|s| s.z <= 6),
                    "missing active background sprite for {scene:?}"
                );
            }
        }
    }

    #[test]
    fn all_special_screens_render_authentic_rom_layers() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        let expected: [(&str, Option<&str>, &str); 12] = [
            ("bonus", Some("anim_bonus_background"), "BONUS"),
            ("match", Some("anim_match_movie"), "MATCH"),
            ("game_over", Some("anim_darth_maul_game_over"), "GAME OVER"),
            ("game_intro", Some("anim_republic_cruiser_intro"), "STAR WARS EPISODE I"),
            ("extra_ball", Some("anim_extra_ball_show"), "EXTRA BALL!"),
            ("ball_save", None, "BALL SAVED"),
            ("tilt_warning", None, "DANGER! DANGER!"),
            ("slam_tilt", None, "SLAM TILT"),
            ("sub_escape", Some("anim_gungan_sub_chomp"), "SUB ESCAPE"),
            ("build_c3p0", Some("anim_build_c3p0"), "BUILD C-3PO"),
            ("ramp_combo", Some("anim_naboo_flyby_single"), "RAMP COMBO!"),
            ("jedi_ready", Some("anim_saber_ball_loop"), "JEDI MULTIBALL READY!"),
        ];

        for (mode, sprite_opt, text_frag) in expected {
            assert!(machine.debug_launch_mode(mode), "failed launch {mode}");
            machine.tick(16);
            if let Some(spr) = sprite_opt {
                assert!(
                    machine.video.last_sprite_cmds.iter().any(|s| s.anim_name == spr),
                    "missing sprite {spr} for {mode}"
                );
            }
            assert!(
                machine
                    .video
                    .last_font_text_cmds
                    .iter()
                    .any(|t| t.text.contains(text_frag)),
                "missing text '{text_frag}' for {mode}"
            );
        }
    }

    #[test]
    fn boot_tilt_hstd_and_all_diagnostics_use_rom_compositor_without_legacy_fallback() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        // Boot screen
        machine.video.text_calls.clear();
        machine.tick(16);
        assert!(machine.video.text_calls.is_empty());
        assert!(machine
            .video
            .last_font_text_cmds
            .iter()
            .any(|t| t.text.contains("WILLIAMS PINBALL 2000")));

        // Tilt overlay
        assert!(machine.debug_launch_mode("tilt"));
        machine.video.text_calls.clear();
        machine.tick(16);
        assert!(machine.video.text_calls.is_empty());
        assert!(machine
            .video
            .last_font_text_cmds
            .iter()
            .any(|t| t.text == "TILT" && t.z == 240));

        // High Score Entry (`DeffHSTDinitials`)
        assert!(machine.debug_launch_mode("hstd_entry"));
        machine.video.text_calls.clear();
        machine.tick(16);
        assert!(machine.video.text_calls.is_empty());
        assert!(machine
            .video
            .last_font_text_cmds
            .iter()
            .any(|t| t.font_name == "font_starwars_19_data" && t.text.contains("HIGHEST SCORE")));

        // Diagnostics (`align`, `colorbar`, `grid`, `rgb`, `switch`, `coil`, `lamp`, `fuse`)
        assert!(machine.debug_launch_mode("diag:align"));
        machine.tick(16);
        assert!(machine
            .video
            .last_sprite_cmds
            .iter()
            .any(|s| s.anim_name == "system_video_test_align"));

        assert!(machine.debug_launch_mode("diag:colorbar"));
        machine.tick(16);
        assert!(machine.video.last_rect_cmds.len() >= 8);

        assert!(machine.debug_launch_mode("diag:grid"));
        machine.tick(16);
        assert!(machine.video.last_rect_cmds.len() >= 20);

        assert!(machine.debug_launch_mode("diag:rgb"));
        machine.tick(16);
        assert_eq!(machine.video.last_rect_cmds.len(), 3);

        for (mode, needle) in [
            ("diag:switch", "SWITCH MATRIX"),
            ("diag:coil", "COIL TEST"),
            ("diag:lamp", "LAMP MATRIX"),
            ("diag:fuse", "FUSE STATUS"),
        ] {
            assert!(machine.debug_launch_mode(mode), "failed {mode}");
            machine.video.text_calls.clear();
            machine.tick(16);
            assert!(machine.video.text_calls.is_empty());
            assert!(
                machine
                    .video
                    .last_font_text_cmds
                    .iter()
                    .any(|t| t.text.contains(needle)),
                "missing '{needle}' in {mode}"
            );
        }
    }

    #[test]
    fn all_14_scenes_have_valid_ard_callouts_and_authentic_scoring() {
        for &scene in &Swe1Scene::ALL {
            assert!(
                crate::swe1_audio::resolve_ard(scene.start_ard(), 0).is_some(),
                "invalid start_ard for {scene:?}: {}",
                scene.start_ard()
            );
            assert!(
                crate::swe1_audio::resolve_ard(scene.hit_ard(), 0).is_some(),
                "invalid hit_ard for {scene:?}: {}",
                scene.hit_ard()
            );
            assert!(
                crate::swe1_audio::resolve_ard(scene.complete_ard(), 0).is_some(),
                "invalid complete_ard for {scene:?}: {}",
                scene.complete_ard()
            );
        }

        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());
        assert!(machine.debug_launch_mode("scene:0")); // BattleDroid
        let start_score = machine.score;
        machine.audio.played.clear();

        // 5 required hits for BattleDroid -> 4 * 400_000 + 2_250_000 completion
        for _ in 0..5 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_RIGHT);
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.score >= start_score + crate::swe1_rules::scoring::BATTLE_DROID_COMPLETE);
        let comp_sid = crate::swe1_audio::resolve_ard("ard_battle_droid_scene_complete", 0).unwrap();
        assert!(machine.audio.played.iter().any(|(sid, _)| *sid == comp_sid));
    }

    #[test]
    fn ghidra_lifted_skill_shot_super_jets_ramp_combo_c3p0_and_leffs() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("ingame"));
        assert!(machine.skill_shot_ready);

        // 1. Progressive Skill Shot on SNEAKY_LANE -> 500_000 + LeffKind::SkillShotAward + ard_skill_shot_award
        let base_score = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SNEAKY_LANE);
        assert_eq!(machine.players[0].skill_shots_made, 1);
        assert!(machine.score >= base_score + 500_000);
        assert_eq!(
            machine.active_leff.map(|l| l.kind),
            Some(LeffKind::SkillShotAward)
        );
        let ss_sid = crate::swe1_audio::resolve_ard("ard_skill_shot_award", 0).unwrap();
        assert!(machine.audio.played.iter().any(|(sid, _)| *sid == ss_sid));

        // 2. Second Skill Shot (caps cleanly up to 3_000_000)
        machine.players[0].skill_shots_made = 5;
        machine.skill_shot_ready = true;
        let pre_cap = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SNEAKY_LANE);
        assert_eq!(machine.players[0].skill_shots_made, 6);
        assert!(machine.score >= pre_cap + crate::swe1_rules::scoring::SKILL_SHOT_CAP);

        // 3. 6 Jet Bumper hits spell G-U-N-G-A-N -> 7th hit scores JET_SUPER (51_550)
        for _ in 0..6 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::UPPER_JET);
        }
        assert_eq!(machine.players[0].gungan_letters, 0);
        assert!(machine.players[0].super_jets_ms > 0);
        assert_eq!(
            machine.active_leff.map(|l| l.kind),
            Some(LeffKind::SuperJetsStart)
        );
        let sj_pre = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::MIDDLE_JET);
        assert_eq!(machine.score, sj_pre + crate::swe1_rules::scoring::JET_SUPER);

        // 4. Ramp Combo (Left -> Right within 6s window)
        machine.audio.played.clear();
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_RIGHT);
        assert_eq!(machine.players[0].ramp_combo_count, 2);
        assert_eq!(machine.players[0].ramp_combo_best, 2);
        assert_eq!(
            machine.active_leff.map(|l| l.kind),
            Some(LeffKind::RampMade { right: true })
        );
        let rc_sid = crate::swe1_audio::resolve_ard("ard_ramp_combo_naboo_flyby", 0).unwrap();
        assert!(machine.audio.played.iter().any(|(sid, _)| *sid == rc_sid));

        // 5. Build C-3PO (6 Captive Ball hits -> +1_000_000 + Extra Ball + C3P0Complete Leff)
        let prev_eb = machine.players[0].extra_balls;
        for _ in 0..6 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::CAPTIVE_BALL);
        }
        assert_eq!(machine.players[0].c3p0_parts, 0);
        assert_eq!(machine.players[0].extra_balls, prev_eb + 1);
        assert_eq!(
            machine.active_leff.map(|l| l.kind),
            Some(LeffKind::C3P0Complete)
        );

        // 6. Empty Laser -> ard_generic_laser_empty
        machine.players[0].laser_shots = 0;
        machine.audio.played.clear();
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_ACTION_BUTTON);
        let empty_sid = crate::swe1_audio::resolve_ard("ard_generic_laser_empty", 0).unwrap();
        assert!(machine.audio.played.iter().any(|(sid, _)| *sid == empty_sid));

        // 7. Verify Leff clears cleanly after its duration elapses
        assert!(machine.active_leff.is_some());
        for _ in 0..120 {
            machine.tick(16);
        }
        assert!(machine.active_leff.is_none());
    }
