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
