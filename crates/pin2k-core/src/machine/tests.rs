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

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SNEAKY_LANE);
        assert!(machine.score >= 1_000_000, "Skill shot awarded");

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
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
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
            machine.playfield_valid = true;
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
        // Nominal mode displays both spinners statically at the last frame (55) without playing turnstile sound
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_bracket_left" && s.x == 43 && s.y == 174 && s.z == 22
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_1" && s.frame == 55 && s.x == 43 && s.y == 174 && s.z == 22
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_bracket_right" && s.x == 421 && s.y == 174 && s.z == 22
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_2" && s.frame == 55 && s.x == 421 && s.y == 174 && s.z == 22
        }));
        assert!(!machine
            .audio
            .played
            .iter()
            .any(|(sid, _)| crate::swe1_tables::sound::SPINNER_TICKS.contains(sid)));

        // Left ramp enter triggers left virtual spinner (15 clicks across frames 0..=22); RAMP_MADE_LEFT does NOT re-trigger
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
        machine.tick(16);
        assert!(machine.left_spinner.active);
        for _ in 0..120 {
            machine.tick(16);
        }
        let spins_after_enter = machine.left_spinner.total_spins;
        assert_eq!(spins_after_enter, 15);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        assert_eq!(machine.left_spinner.total_spins, spins_after_enter);
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_bracket_left" && s.x == 43 && s.y == 174 && s.z == 22
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_1" && s.frame > 0 && s.x == 43 && s.y == 174 && s.z == 22
        }));
        assert!(machine
            .audio
            .played
            .iter()
            .any(|(sid, _)| *sid == crate::swe1_tables::sound::SPINNER_TICKS[0]));

        // Right ramp pass triggers right virtual spinner animation
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_RAMP_ENTER);
        for _ in 0..6 {
            machine.tick(16);
        }
        assert!(machine.right_spinner.active);
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_bracket_right" && s.x == 421 && s.y == 174 && s.z == 22
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_spinner_2" && s.frame > 0 && s.x == 421 && s.y == 174 && s.z == 22
        }));

        // During a mini-game (active scene) or multiball, ramps must NOT trigger or render spinners
        assert!(machine.debug_launch_mode("scene:0"));
        machine.audio.played.clear();
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_RAMP_ENTER);
        machine.tick(16);
        assert!(!machine.left_spinner.active);
        assert!(!machine.right_spinner.active);
        assert!(!machine
            .audio
            .played
            .iter()
            .any(|(sid, _)| crate::swe1_tables::sound::SPINNER_TICKS.contains(sid)));
        assert!(!machine
            .video
            .last_sprite_cmds
            .iter()
            .any(|s| s.anim_name.starts_with("anim_spinner_")));
    }

    #[test]
    fn nominal_obiwan_holo_sequence_and_gungan_animations() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        machine.tick(16);

        // 1. Nominal gameplay with shield up: `anim_obiwan_holo` + right-side Gungan seaweed/bubbles/letters
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_obiwan_holo" && s.x == 125 && s.y == 80
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_gungan_seaweed" && s.x == 496 && s.y == 135 && s.z == 15
        }));
        let bubble_count = machine
            .video
            .last_sprite_cmds
            .iter()
            .filter(|s| s.anim_name == "anim_gungan_bubbles" && s.z == 10)
            .count();
        assert_eq!(bubble_count, 5);
        assert!(machine
            .video
            .last_font_text_cmds
            .iter()
            .any(|t| t.text == "G" && t.x >= 500 && t.z == 15));

        // 2. First shield hit flashes `anim_obiwan_holo_glow`
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        machine.tick(16);
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_obiwan_holo_glow" && s.x == 125 && s.y == 80
        }));

        // 3. Remaining 5 shield hits deplete shield -> `anim_obiwan_holo_flatten`
        for _ in 0..5 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        }
        machine.tick(16);
        assert!(machine.players[0].scene_select_ready);
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_obiwan_holo_flatten" && s.x == 125 && s.y == 80
        }));

        // 4. After 900ms -> chains into `anim_obiwan_holo_activate` + `anim_holo_burst`
        machine.tick(900);
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_obiwan_holo_activate" && s.x == 162 && s.y == 159
        }));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| {
            s.anim_name == "anim_holo_burst"
        }));

        // 5. After 1500ms total -> `anim_obiwan_holo_glow_loop` with mini-game holo above it
        machine.tick(650);
        let glow_loop = machine
            .video
            .last_sprite_cmds
            .iter()
            .find(|s| s.anim_name == "anim_obiwan_holo_glow_loop")
            .expect("expected anim_obiwan_holo_glow_loop");
        let sc = Swe1Scene::ALL[machine.players[0].next_scene_idx % Swe1Scene::ALL.len()];
        let mini_holo = machine
            .video
            .last_sprite_cmds
            .iter()
            .find(|s| s.anim_name == sc.holo_anim_hint())
            .expect("expected selectable mini-game hologram");
        assert!(mini_holo.y < glow_loop.y);

        // 6. Flippers do NOT change scene selection; Laser action buttons and ramps DO (by side)
        let initial_idx = machine.players[0].next_scene_idx;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_FLIPPER_BUTTON);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_FLIPPER_BUTTON);
        assert_eq!(machine.players[0].next_scene_idx, initial_idx);

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_ACTION_BUTTON);
        // Index 1 (MosEspaCantina) requires min_scenes_started = 1, so cycling skips to index 2 (FederationFighter)
        assert_eq!(machine.players[0].next_scene_idx, 2);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_ACTION_BUTTON);
        assert_eq!(machine.players[0].next_scene_idx, initial_idx);

        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_RIGHT);
        assert_eq!(machine.players[0].next_scene_idx, 2);
        assert!(machine.active_scene.is_none());
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        assert_eq!(machine.players[0].next_scene_idx, initial_idx);
        assert!(machine.active_scene.is_none());
    }

    #[test]
    fn jarjar_juggle_authentic_lanes_rewards_and_state_machine() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        // Launch Jar Jar Juggle directly and skip intro
        assert!(machine.debug_launch_mode("scene:9"));
        assert_eq!(
            machine.active_scene.as_ref().map(|s| s.scene),
            Some(Swe1Scene::JarJarJuggle)
        );
        if let Some(ref mut sc) = machine.active_scene {
            sc.intro_remaining_ms = 0;
        }
        let jj = machine.jarjar_juggle.as_ref().expect("jarjar_juggle state initialized");
        assert_eq!(jj.remaining, 5);
        assert!(!jj.dropped);
        assert_eq!(jj.anim, super::JuggleAnimState::Juggle1);
        // Max 1 LightExtraBall across the 5 lanes (`no_jarjar_juggle_extra_ball`)
        let eb_count = jj
            .lanes
            .iter()
            .filter(|l| l.award == super::JuggleAward::LightExtraBall)
            .count();
        assert!(eb_count <= 1);

        // Render initial frame: backdrop + Jar Jar + 5 junk sprites
        machine.tick(16);
        assert!(machine
            .video
            .last_sprite_cmds
            .iter()
            .any(|s| s.anim_name == "anim_jarjar_juggle_backdrop" && s.x == 0 && s.y == 50));
        assert!(machine
            .video
            .last_sprite_cmds
            .iter()
            .any(|s| s.anim_name == "anim_jarjar_juggle"));
        let junk_count = machine
            .video
            .last_sprite_cmds
            .iter()
            .filter(|s| s.anim_name.starts_with("anim_jarjar_junk"))
            .count();
        assert_eq!(junk_count, 5);

        // Center shield hit knocks Jar Jar down (`Fall1`), drops items into lanes, and collects lane 2
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        machine.tick(16);
        {
            let jj = machine.jarjar_juggle.as_ref().unwrap();
            assert!(jj.dropped);
            assert_eq!(jj.anim, super::JuggleAnimState::Fall1);
            assert!(jj.lanes[2].collected);
            assert_eq!(jj.remaining, 4);
        }
        assert!(machine
            .video
            .last_sprite_cmds
            .iter()
            .any(|s| s.anim_name == "anim_jarjar_fall"));

        // Re-hitting already collected center lane does not reduce remaining count
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        machine.tick(16);
        assert_eq!(machine.jarjar_juggle.as_ref().unwrap().remaining, 4);

        // Advance time through Fall1 -> JumpUp -> Switch -> Stare
        for _ in 0..70 {
            machine.tick(50);
        }
        assert_eq!(
            machine.jarjar_juggle.as_ref().unwrap().anim,
            super::JuggleAnimState::Stare
        );

        // Collect remaining 4 lanes (0, 1, 3, 4) -> completes the scene!
        for sw_id in [
            sw::LEFT_RAMP_ENTER,
            sw::LEFT_SHIELD_TARGET,
            sw::RIGHT_SHIELD_TARGET,
            sw::RIGHT_RAMP_ENTER,
        ] {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw_id);
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.jarjar_juggle.is_none());
    }

    #[test]
    fn podrace_authentic_lanes_checkpoints_and_state_machine() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine = Pin2kMachine::new(driver, HeadlessVideo::default());

        assert!(machine.debug_launch_mode("scene:6"));
        assert_eq!(
            machine.active_scene.as_ref().map(|s| s.scene),
            Some(Swe1Scene::Podrace)
        );
        let pr = machine.podrace.as_ref().expect("podrace state initialized");
        assert_eq!(pr.phase, super::PodracePhase::StartingGate);
        assert_eq!(pr.speed_mph, 150);
        assert_eq!(pr.checkpoints, 0);

        // Skip intro -> StartingGate -> Closeup -> Racing
        if let Some(ref mut sc) = machine.active_scene {
            sc.intro_remaining_ms = 0;
        }
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_podrace_starting").count(),
            1
        );
        for _ in 0..78 {
            machine.tick(50);
        }
        let pr = machine.podrace.as_ref().unwrap();
        assert_eq!(pr.phase, super::PodracePhase::Racing);
        // First icon placed is always a Checkpoint (`PodraceIconKind::Checkpoint`)
        let cp_lane = pr
            .lanes
            .iter()
            .position(|l| l.icon == Some(super::PodraceIconKind::Checkpoint))
            .expect("first icon must be checkpoint");
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_podrace_1").count(),
            1
        );

        let lane_sw = [sw::LEFT_RAMP_ENTER, sw::SHIELD_HIT, sw::RIGHT_RAMP_ENTER][cp_lane];
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), lane_sw);
        assert_eq!(machine.podrace.as_ref().unwrap().checkpoints, 1);

        // Debounce blocks immediate duplicate hit within 600ms
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), lane_sw);
        assert_eq!(machine.podrace.as_ref().unwrap().checkpoints, 1);
    }

    #[test]
    fn battle_droid_authentic_tank_and_stap_state_machine() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:0"));
        assert_eq!(machine.battle_droid.as_ref().unwrap().tank_hits_remaining, 8);
        for _ in 0..270 {
            machine.tick(16);
        }
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_mtt_frame").count(),
            1
        );
        assert!(machine.video.last_sprite_cmds.iter().any(|s| s.anim_name == "anim_mtt_doors_clean_anim" && s.x == 239 && s.y == 89));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| s.anim_name == "anim_left_bdroid_standing" && s.x == 68 && s.y == 135 && s.z == 5));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| s.anim_name == "anim_right_bdroid_standing" && s.x == 421 && s.y == 134 && s.z == 5));
        assert!(machine.video.last_sprite_cmds.iter().any(|s| s.anim_name == "anim_bdroid_on_stap" && (s.y == 84 || s.y == 80)));

        // Ramp shot (200_000) & quick laser within 0.8s (1_000_000) destroy STAP without reducing tank_hits_remaining
        let pre_ramp = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        assert_eq!(machine.battle_droid.as_ref().unwrap().staps_destroyed, 1);
        assert_eq!(machine.battle_droid.as_ref().unwrap().tank_hits_remaining, 8);
        assert_eq!(machine.score, pre_ramp + crate::swe1_rules::scoring::SWITCH_BASE + 200_000);

        machine.players[0].laser_shots = 1;
        let pre_laser = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_ACTION_BUTTON);
        assert_eq!(machine.battle_droid.as_ref().unwrap().staps_destroyed, 2);
        assert_eq!(machine.battle_droid.as_ref().unwrap().tank_hits_remaining, 8);
        assert_eq!(machine.score, pre_laser + 1_000_000);

        // Drop target hits side droid (100_000) without reducing tank_hits_remaining
        let pre_drop = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_DROP_TARGET);
        assert_eq!(machine.battle_droid.as_ref().unwrap().tank_hits_remaining, 8);
        assert_eq!(machine.score, pre_drop + 100_000);

        // Side shield target deals 1 damage (down to 7) + 200_000 pts
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_SHIELD_TARGET);
        assert_eq!(machine.battle_droid.as_ref().unwrap().tank_hits_remaining, 7);

        // Center shield hits deal 2 damage each (7 -> 5 -> 3 -> 1); shield rises when < 3 HP remain
        for expected in [5, 3, 1] {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
            assert_eq!(machine.battle_droid.as_ref().unwrap().tank_hits_remaining, expected);
        }
        assert!(machine.shield_raised);
        // Final center shield hit defeats the tank & completes the scene
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        assert!(machine.active_scene.is_none());
        assert!(machine.battle_droid.is_none());
    }

    #[test]
    fn destroyer_droid_authentic_target_sweep_and_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:1"));
        assert_eq!(machine.destroyer_droid.as_ref().unwrap().destroyers_remaining, 6);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_sith_probe_target").count(),
            1
        );

        // Place target in center window and destroy all 6 droidekas
        for expected in (0..6).rev() {
            machine.destroyer_droid.as_mut().unwrap().target_x = 220;
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
            if expected > 0 {
                assert_eq!(machine.destroyer_droid.as_ref().unwrap().destroyers_remaining, expected);
            }
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.destroyer_droid.is_none());
    }

    #[test]
    fn federation_fighter_authentic_fighters_and_control_ship_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:2"));
        assert_eq!(machine.federation_fighter.as_ref().unwrap().fighters_remaining, 9);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_droid_control_ship").count(),
            1
        );

        // 3 SHIELD_POPPER hits (3 damage each) defeat all 9 fighters
        for expected in [6, 3, 0] {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_POPPER);
            if expected > 0 {
                assert_eq!(machine.federation_fighter.as_ref().unwrap().fighters_remaining, expected);
            }
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.federation_fighter.is_none());
    }

    #[test]
    fn gungan_battle_authentic_sub_escape_distance_and_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:3"));
        assert_eq!(machine.gungan_battle.as_ref().unwrap().distance, 5);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_gungan_sub_sub").count(),
            1
        );

        // Shield hit (+1 -> 6), then 2 ramp shots (+2 each -> 8 -> 10) complete the escape
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        assert_eq!(machine.gungan_battle.as_ref().unwrap().distance, 6);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        assert_eq!(machine.gungan_battle.as_ref().unwrap().distance, 8);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_RIGHT);
        assert!(machine.active_scene.is_none());
        assert!(machine.gungan_battle.is_none());
    }

    #[test]
    fn hangar_escape_authentic_droids_and_popper_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:4"));
        assert_eq!(machine.hangar_escape.as_ref().unwrap().droids_remaining, 8);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_hangar_droid_run").count(),
            1
        );

        // First droid starts alone at center: empty-lane miss (+20_000) increments droids_missed
        assert_eq!(machine.hangar_escape.as_ref().unwrap().lanes, [0, 1, 0]);
        let s0 = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
        assert_eq!(machine.score, s0 + 20_000);
        assert_eq!(machine.hangar_escape.as_ref().unwrap().droids_missed, 1);

        // Defeat all 8 droids by hitting active lanes (+200_000 each)
        while machine.hangar_escape.as_ref().is_some_and(|h| !h.door_open()) {
            let lanes = machine.hangar_escape.as_ref().unwrap().lanes;
            let sw_id = if lanes[1] > 0 {
                sw::SHIELD_HIT
            } else if lanes[0] > 0 {
                sw::LEFT_RAMP_ENTER
            } else {
                sw::RIGHT_RAMP_ENTER
            };
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw_id);
        }
        assert!(machine.shield_raised);
        assert!(machine.hangar_escape.as_ref().unwrap().door_open());

        // Shoot SHIELD_POPPER behind raised shield to exit hangar
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_POPPER);
        assert!(!machine.shield_raised);
        assert!(machine.active_scene.is_none());
        assert!(machine.hangar_escape.is_none());
    }

    #[test]
    fn hover_tank_authentic_lanes_catapult_and_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:5"));
        assert_eq!(machine.hover_tank.as_ref().unwrap().tanks_remaining, 6);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_hover_tank_boomer_ball").count(),
            1
        );

        for expected in (0..6).rev() {
            let sw_id = [sw::LEFT_RAMP_ENTER, sw::SHIELD_HIT, sw::RIGHT_RAMP_ENTER][expected as usize % 3];
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw_id);
            if expected > 0 {
                assert_eq!(machine.hover_tank.as_ref().unwrap().tanks_remaining, expected);
            }
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.hover_tank.is_none());
    }

    #[test]
    fn probe_droid_authentic_target_and_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:7"));
        assert_eq!(machine.probe_droid.as_ref().unwrap().probes_remaining, 3);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_sith_probe_target").count(),
            1
        );

        // Miss when target_x is outside window -> +70_000 and probes_missed == 1
        machine.probe_droid.as_mut().unwrap().target_x = 400;
        let s_miss = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
        assert_eq!(machine.score, s_miss + 70_000);
        assert_eq!(machine.probe_droid.as_ref().unwrap().probes_missed, 1);

        // Cooldown (0.48s) ignores immediate repeat shot
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        assert_eq!(machine.probe_droid.as_ref().unwrap().probes_remaining, 3);

        for expected in [2, 1, 0] {
            for _ in 0..32 {
                machine.tick(16);
            }
            machine.probe_droid.as_mut().unwrap().target_x = 220;
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
            if expected > 0 {
                assert_eq!(machine.probe_droid.as_ref().unwrap().probes_remaining, expected);
            }
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.probe_droid.is_none());
    }

    #[test]
    fn queen_fashion_authentic_wheel_and_outfits_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:8"));
        assert_eq!(machine.queen_fashion.as_ref().unwrap().outfits_remaining, 6);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert!(machine.video.last_sprite_cmds.iter().any(|s| s.anim_name == "anim_fashion_throne" && s.x == 5 && s.y == 44));

        // Expose both windows at initial position (0x20 left, 0x04 right) -> 500_000 each
        let s0 = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_RAMP_ENTER);
        assert_eq!(machine.queen_fashion.as_ref().unwrap().outfits_remaining, 4);
        assert_eq!(
            machine.score,
            s0 + 2 * crate::swe1_rules::scoring::QUEEN_FASHION_MATCH
        );

        // Duplicate left hit awards 15_520 instead of 500_000
        let s_dup = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
        assert_eq!(machine.score, s_dup + 15_520);
        assert_eq!(machine.queen_fashion.as_ref().unwrap().outfits_remaining, 4);

        // Spin wheel with SHIELD_HIT (+20_000) and finish remaining 4 outfits
        while machine.queen_fashion.is_some() {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
            for _ in 0..40 {
                machine.tick(30);
                if machine.queen_fashion.as_ref().is_none_or(|q| !q.spinning) {
                    break;
                }
            }
            if machine.queen_fashion.as_ref().is_some_and(|q| q.is_left_card_clear()) {
                pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
            }
            if machine.queen_fashion.as_ref().is_some_and(|q| q.is_right_card_clear()) {
                pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_RAMP_ENTER);
            }
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.queen_fashion.is_none());
    }

    #[test]
    fn wattos_chance_authentic_cubes_and_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:10"));
        assert_eq!(machine.wattos_chance.as_ref().unwrap().hits_remaining, 3);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_watto_scene_cube").count(),
            3
        );

        // Active lane starts at 0 (left); hit left 3 times to complete
        for expected in [2, 1, 0] {
            machine.wattos_chance.as_mut().unwrap().timeline_ms = 0;
            machine.wattos_chance.as_mut().unwrap().active_lane = 0;
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_RAMP_ENTER);
            if expected > 0 {
                assert_eq!(machine.wattos_chance.as_ref().unwrap().hits_remaining, expected);
            }
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.wattos_chance.is_none());
    }

    #[test]
    fn mos_espa_cantina_authentic_chairs_and_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("musical_chairs"));
        assert_eq!(machine.mos_espa_cantina.as_ref().unwrap().seats_remaining, 3);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert!(machine
            .video
            .last_sprite_cmds
            .iter()
            .any(|s| s.anim_name == "anim_jedi_council_background" && s.x == 6 && s.y == 3));
        assert!(machine
            .video
            .last_sprite_cmds
            .iter()
            .filter(|s| s.anim_name.starts_with("anim_jedi_council_") && s.anim_name != "anim_jedi_council_background")
            .all(|s| s.frame == 0));
        // All 5 chairs display their award text underneath (y = 173 / 185)
        assert_eq!(
            machine.video.last_font_text_cmds.iter().filter(|t| t.y == 173).count(),
            5
        );

        // During music: ramp/shield/laser shuffle and award 70_000 without departing a seat
        let pre_jump = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        assert_eq!(machine.score, pre_jump + crate::swe1_rules::scoring::SWITCH_BASE + 70_000);
        assert_eq!(machine.mos_espa_cantina.as_ref().unwrap().seats_remaining, 3);

        // Advance until music stops (3..9s), shield rises, then 3 center collects complete the scene
        for round in (0..3).rev() {
            for _ in 0..600 {
                machine.tick(16);
                if machine.mos_espa_cantina.as_ref().is_some_and(|m| m.music_stopped) {
                    break;
                }
            }
            assert!(machine.mos_espa_cantina.as_ref().unwrap().music_stopped);
            assert!(machine.shield_raised);
            let sw_collect = if round == 1 { sw::SHIELD_POPPER } else { sw::SHIELD_HIT };
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw_collect);
            if round > 0 {
                assert_eq!(machine.mos_espa_cantina.as_ref().unwrap().seats_remaining, round);
            }
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.mos_espa_cantina.is_none());
    }

    #[test]
    fn r2d2_authentic_units_hurry_up_and_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:12"));
        assert_eq!(machine.r2d2.as_ref().unwrap().units_remaining, 6);
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_r2d2_scene_r2d2_unit").count(),
            1
        );

        // Hurry-up stays frozen at 500_000 during 3s, then decays down toward 50_000
        for _ in 0..180 {
            machine.tick(16);
        }
        assert_eq!(machine.r2d2.as_ref().unwrap().hurry_up_value, 500_000);
        for _ in 0..400 {
            machine.tick(16);
        }
        assert!(machine.r2d2.as_ref().unwrap().hurry_up_value < 500_000);
        assert!(machine.r2d2.as_ref().unwrap().hurry_up_value >= 50_000);

        // Ramp made boosts hurry-up (500_000 + 250_000 = 750_000) without boarding a unit
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        assert_eq!(machine.r2d2.as_ref().unwrap().units_remaining, 6);
        assert_eq!(machine.r2d2.as_ref().unwrap().hurry_up_value, 750_000);

        for expected in (0..6).rev() {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
            if expected > 0 {
                assert_eq!(machine.r2d2.as_ref().unwrap().units_remaining, expected);
            }
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.r2d2.is_none());
    }

    #[test]
    fn question_mark_authentic_mystery_award_and_timer_completion() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("scene:13"));
        assert!(machine.question_mark.is_some());
        machine.active_scene.as_mut().unwrap().intro_remaining_ms = 0;
        machine.tick(16);
        assert_eq!(
            machine.video.last_sprite_cmds.iter().filter(|s| s.anim_name == "anim_super_mystery_icon").count(),
            1
        );

        // 5-second sleep completes QuestionMarkScene
        for _ in 0..320 {
            machine.tick(16);
        }
        assert!(machine.active_scene.is_none());
        assert!(machine.question_mark.is_none());
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

        // 4 center SHIELD_HITs (-2 HP each = 8 HP) complete BattleDroid + 2_250_000 completion
        for _ in 0..4 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
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
        assert_eq!(
            machine.score,
            sj_pre + crate::swe1_rules::scoring::SWITCH_BASE + crate::swe1_rules::scoring::JET_SUPER
        );

        // 4. Ramp Combo (Left -> Right within 6.72s window)
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

        // 5. Build C-3PO (starts at 4/8 -> 4 Left Loop hits -> Extra Ball + C3P0Complete Leff)
        let prev_eb = machine.players[0].extra_balls;
        for _ in 0..4 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_LOOP_ROLLOVER);
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

    #[test]
    fn swe1_v14_rulebook_scoring_priorities_and_playfield_validation() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("ingame"));
        assert!(!machine.playfield_valid);
        assert!(machine.skill_shot_ready);

        // 1. Draining before playfield is valid re-serves the ball without losing ball or Ball Saver
        machine.ball_save_ms = 5_000;
        machine.eject_cooldown_ms = 0;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::TROUGH_BALL_1);
        assert_eq!(machine.current_ball, 1);
        assert_eq!(machine.ball_save_ms, 5_000);
        assert!(machine.skill_shot_ready);
        machine.tick(16);
        machine.eject_cooldown_ms = 0;

        // 2. Captive ball exact 1.4 cycle (50, 40, 30, 20, 10 -> 500...) without 2,310 base
        let pre_cb = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::CAPTIVE_BALL);
        assert_eq!(machine.score, pre_cb + 50);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::CAPTIVE_BALL);
        assert_eq!(machine.score, pre_cb + 90);

        // 3. Standup 3-bank + Hotdogs -> Bonus X + 1 + Naboo ship lamps
        let pre_bank = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_BANK_UPPER);
        assert_eq!(machine.score, pre_bank + 12_310);
        // 3rd distinct switch (CAPTIVE_BALL, LEFT_BANK_UPPER, LEFT_BANK_MIDDLE) validates playfield!
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_BANK_MIDDLE);
        assert!(machine.playfield_valid);
        assert!(machine.skill_shot_grace_ms > 0 && machine.skill_shot_grace_ms <= 160);
        // SNEAKY_LANE within 160ms grace still awards Skill Shot (1,000,000 + 2,310)!
        let pre_ss = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SNEAKY_LANE);
        assert_eq!(machine.score, pre_ss + 1_000_000 + 2_310);

        // Complete both standup banks -> both hotdogs -> Bonus X becomes 2
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_BANK_LOWER);
        assert!(machine.players[0].left_hotdog_lit);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_BANK_UPPER);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_BANK_MIDDLE);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_BANK_LOWER);
        assert_eq!(machine.players[0].bonus_multiplier, 2);
        assert!(!machine.players[0].left_hotdog_lit);
        assert!(!machine.players[0].right_hotdog_lit);
        assert_eq!(machine.players[0].naboo_ship_lamps, 6);
        assert_eq!(machine.players[0].bonus_points, 600_000);

        // 4. 3rd Skill Shot awards 3,000,000 + 1 JEDI letter (once per game)
        machine.players[0].skill_shots_made = 2;
        machine.skill_shot_ready = true;
        let pre_3rd_ss = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SNEAKY_LANE);
        assert_eq!(machine.players[0].jedi_letters, 1);
        assert!(machine.players[0].skill_shot_jedi_awarded);
        assert_eq!(machine.score, pre_3rd_ss + 2_310 + 3_000_000 + 550_000);

        // 5. Tilt lowers shield, blocks scoring, and loses bonus
        machine.tilt_warnings = machine.adjustments.tilt_warnings_max - 1;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::PLUMB_TILT);
        assert_eq!(machine.state, MachineState::Tilt);
        assert!(!machine.shield_raised);
        let tilt_score = machine.score;
        machine.add_score(100_000);
        assert_eq!(machine.score, tilt_score);
    }

    #[test]
    fn rhai_custom_code_mod_drives_modes_timers_and_switch_overrides() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        let mod_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../mods/example_droid_hurry_up");
        let manifest = ModManifest::load_from_dir(&mod_dir)
            .expect("example_droid_hurry_up mod should load from disk");
        assert!(manifest.script_source.is_some());

        machine.apply_mod(&manifest);
        assert!(machine.mod_script.is_some());

        assert!(machine.debug_launch_mode("ingame"));

        // Sneaky Lane starts "droid_hurry_up" mode AND still awards base Skill Shot (return false)
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SNEAKY_LANE);
        assert!(machine.score >= 1_000_000);
        assert!(machine.mod_lit_lamps.contains(&79));

        // Tick 1 second -> hurry_up_value decays by 10_000 (from 500_000 to ~490_000)
        for _ in 0..60 {
            machine.tick(16);
        }

        // Captive Ball during mode collects Hurry Up and overrides base Captive Ball (return true)
        let pre_cb = machine.score;
        let pre_hits = machine.players[0].captive_ball_hits;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::CAPTIVE_BALL);
        assert_eq!(machine.players[0].captive_ball_hits, pre_hits); // base handler was suppressed!
        assert!(machine.score >= pre_cb + 480_000);
        assert!(!machine.mod_lit_lamps.contains(&79));
    }

    #[test]
    fn swe1_210_mod_implements_quick_mb_droid_hurry_up_and_double_scoring() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        let mod_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../mods/swe1_210");
        let manifest = ModManifest::load_from_dir(&mod_dir)
            .expect("swe1_210 mod should load from disk");
        machine.apply_mod(&manifest);
        assert!(machine.debug_launch_mode("ingame"));

        // 1. Quick Multiball: 4 Captive Ball hits -> 15,010 each, then 500,000 + 2nd ball ejected!
        let pre_qmb = machine.score;
        for _ in 0..3 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::CAPTIVE_BALL);
        }
        assert_eq!(machine.score, pre_qmb + 3 * 15_010);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::CAPTIVE_BALL);
        assert_eq!(machine.score, pre_qmb + 4 * 15_010 + 500_000);
        // Tick to eject the 2nd ball -> balls_in_play == 2 -> next Captive Ball scores 1,000,000 Jackpot!
        machine.eject_cooldown_ms = 0;
        machine.tick(16);
        assert!(machine.balls_in_play >= 2);
        let pre_jp = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::CAPTIVE_BALL);
        assert_eq!(machine.score, pre_jp + 1_000_000);

        // 2. Droid Hurry Up: 5 Sneaky Lane hits -> raises Left Drop Target & starts 1,000,000 hurry-up
        machine.left_drop_down = true;
        for _ in 0..5 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SNEAKY_LANE);
        }
        assert!(!machine.left_drop_down);
        let pre_dhu = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_DROP_TARGET);
        assert!(machine.score >= pre_dhu + 900_000);

        // 3. Double Scoring (X2): 4th saucer visit activates 2X multiplier for 25s
        for _ in 0..4 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_SAUCER);
        }
        assert_eq!(machine.score_multiplier, 2);
    }

    #[test]
    fn swe1_authentic_shooter_and_mainplay_music_transitions() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        // Boot into attract mode, then start game (Ball 1)
        machine.tick(1_250);
        assert_eq!(machine.state, MachineState::Attract);
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::START_BUTTON);
        assert_eq!(machine.state, MachineState::InGame);
        assert!(machine.first_ball_serve);
        assert!(!machine.playfield_valid);
        assert!(
            machine.audio.played.iter().any(|(id, looping)| *id == sound::MAIN_THEME_LOOP && *looping),
            "Ball 1 shooter lane should play ard_shooter_tune_theme (0x0001)"
        );

        // Ball leaves shooter lane and validates playfield -> switches to MAINPLAY_LOOP (0x0002)
        machine.audio.played.clear();
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RIGHT_INLANE);
        assert!(machine.playfield_valid);
        assert!(!machine.first_ball_serve);
        assert!(
            machine.audio.played.iter().any(|(id, looping)| *id == sound::MAINPLAY_LOOP && *looping),
            "Valid playfield should switch to ard_mainplay_tune (0x0002)"
        );

        // Drain Ball 1 after ball save expires -> Ball 2 shooter lane plays SHOOTER_GROOVE_LOOP (0x0003)
        machine.audio.played.clear();
        machine.ball_save_ms = 0;
        machine.on_ball_drained();
        assert_eq!(machine.current_ball, 2);
        assert!(!machine.first_ball_serve);
        assert!(
            machine.audio.played.iter().any(|(id, looping)| *id == sound::SHOOTER_GROOVE_LOOP && *looping),
            "Ball 2 shooter lane should play ard_shooter_tune (0x0003)"
        );
    }

    #[test]
    fn jedi_multiball_authentic_15hp_combat_and_maul_counterattack() {
        let port = VirtualPdbPort::new(pin2k_hal::PlayfieldSignature::Swe1);
        let driver = PdbDriver::new(port);
        let mut machine =
            Pin2kMachine::with_audio(driver, HeadlessVideo::default(), HeadlessAudio::default());

        assert!(machine.debug_launch_mode("darth_maul_duel"));
        assert!(matches!(
            machine.multiball,
            ActiveMultiball::DarthMaulDuel {
                maul_health: 15,
                quigon_health: 15,
                ..
            }
        ));
        assert!(machine.pending_eject_balls >= 2);
        machine.playfield_valid = true;

        // 1. Shield hit deals 1 dmg (15 -> 14) and awards 500,000
        let pre_shield = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::SHIELD_HIT);
        assert_eq!(machine.score, pre_shield + 500_000);
        assert!(matches!(
            machine.multiball,
            ActiveMultiball::DarthMaulDuel { maul_health: 14, .. }
        ));

        // 2. Ramp combo deals 2 dmg (14 -> 12) and awards 700,000 (+ SWITCH_BASE 2,310)
        let pre_ramp = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_LEFT);
        assert_eq!(machine.score, pre_ramp + crate::swe1_rules::scoring::SWITCH_BASE + 700_000);
        assert!(matches!(
            machine.multiball,
            ActiveMultiball::DarthMaulDuel { maul_health: 12, .. }
        ));

        // 3. Left loop deals 2 dmg (12 -> 10) and awards 500,000 (+ SWITCH_BASE 2,310)
        let pre_loop = machine.score;
        pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::LEFT_LOOP_UPPER);
        assert_eq!(machine.score, pre_loop + crate::swe1_rules::scoring::SWITCH_BASE + 500_000);
        assert!(matches!(
            machine.multiball,
            ActiveMultiball::DarthMaulDuel { maul_health: 10, .. }
        ));

        // 4. Maul counter-attack after intro overlay finishes (-2 HP to Qui-Gon: 15 -> 13)
        machine.overlay = None;
        for _ in 0..360 {
            machine.tick(16);
        }
        assert!(matches!(
            machine.multiball,
            ActiveMultiball::DarthMaulDuel { quigon_health: 13, .. }
        ));

        // 5. Remaining hits defeat Maul -> +5,000,000 and Jedi Rank promotion
        let pre_rank = machine.players[0].jedi_rank;
        for _ in 0..5 {
            pulse_switch(&mut machine, |hw, s, c| hw.port.set_logical_switch(s, c), sw::RAMP_MADE_RIGHT);
        }
        assert_eq!(machine.multiball, ActiveMultiball::None);
        assert_ne!(machine.players[0].jedi_rank, pre_rank);
        assert!(machine.score >= 5_000_000);
    }
