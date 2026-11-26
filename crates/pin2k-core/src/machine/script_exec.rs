use pin2k_hal::{AudioBackend, PinballHardware, SoundId, VideoBackend};

use super::{DiagPage, HstdEntrySession, MachineState, Pin2kMachine, VirtualSpinnerState};
use crate::admin::AdminScreen;
use crate::attract::AttractDeff;
use crate::mods::{HomebrewSwitchRule, ModManifest};
use crate::rfm_rules::{GameTitle, RfmMultiball, RfmWave};
use crate::script_engine::{GameScript, ScriptAction, ScriptRuntime, ScriptTrigger};
use crate::storage::{GameHistoryEntry, LocalDb};
use crate::swe1_rules::{ActiveMultiball, ActiveSceneState, JediRank, PlayerState, Swe1Scene};
use crate::swe1_tables::{drive, sound, sw};

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub fn apply_mod(&mut self, manifest: &ModManifest) {
        if let Some(ref banner) = manifest.attract_banner {
            self.status_banner = banner.clone();
        }
        if let Some(bpg) = manifest.balls_per_game {
            self.balls_per_game = bpg.max(1);
            self.adjustments.balls_per_game = self.balls_per_game;
        }
        if let Some(bs) = manifest.ball_save_secs {
            self.adjustments.ball_save_secs = bs;
        }
        if let Some(mult) = manifest.score_multiplier {
            self.score_multiplier = mult.max(1);
        }
        for rule in &manifest.switch_rules {
            self.mod_switch_rules.push(rule.clone());
        }
        if let Some(ref src) = manifest.script_source {
            match crate::mod_script::ModScriptRuntime::compile(src) {
                Ok(rt) => {
                    self.mod_script = Some(rt);
                    self.eval_mod_script_hook("on_init", ());
                }
                Err(err) => {
                    eprintln!("[pin2k-mod] Rhai script compile error: {err}");
                    self.set_banner(format!("MOD SCRIPT ERROR: {err}"), 5_000);
                }
            }
        }
    }

    pub(super) fn mod_snapshot(&self) -> crate::mod_script::ModSnapshot {
        crate::mod_script::ModSnapshot {
            current_player: self.current_player,
            current_ball: self.current_ball,
            score: self.score,
            balls_in_play: self.balls_in_play,
            playfield_valid: self.playfield_valid,
            in_scene: self.active_scene.is_some(),
            in_multiball: !matches!(self.multiball, ActiveMultiball::None),
        }
    }

    fn execute_mod_commands(&mut self, cmds: Vec<crate::mod_script::ModCommand>) {
        use crate::mod_script::ModCommand;
        for cmd in cmds {
            match cmd {
                ModCommand::AddScore(pts) => self.add_score(pts),
                ModCommand::FireCoil { id, pulse_ms } => self.emit_coil(id, pulse_ms),
                ModCommand::SetHold { id, on } => self.hw.set_drive_hold(id, on),
                ModCommand::SetLamp { id, on } => {
                    if on {
                        if !self.mod_lit_lamps.contains(&id) {
                            self.mod_lit_lamps.push(id);
                        }
                    } else {
                        self.mod_lit_lamps.retain(|&l| l != id);
                    }
                }
                ModCommand::PlaySound(sid) => self.emit_sound(SoundId(sid), false),
                ModCommand::PlayArd(pool) => {
                    let _ = self.emit_ard(&pool);
                }
                ModCommand::SetBanner { text, ms } => self.set_banner(text, ms),
                ModCommand::ShowAnim(anim) => self.active_anim_hint = anim,
                ModCommand::EjectBall => {
                    self.pending_eject_balls = self.pending_eject_balls.saturating_add(1);
                }
            }
        }
    }

    pub(super) fn eval_mod_script_hook(&mut self, fn_name: &str, args: impl rhai::FuncArgs) {
        if self.mod_script.is_none() {
            return;
        }
        let snap = self.mod_snapshot();
        let (_, cmds) = self
            .mod_script
            .as_mut()
            .unwrap()
            .call_hook(fn_name, args, snap);
        self.execute_mod_commands(cmds);
    }

    pub(super) fn eval_mod_script_switch(&mut self, sw_id: u8) -> bool {
        if self.mod_script.is_none() {
            return false;
        }
        let snap = self.mod_snapshot();
        let (override_default, cmds) = self
            .mod_script
            .as_mut()
            .unwrap()
            .call_hook("on_switch", (sw_id as i64,), snap);
        self.execute_mod_commands(cmds);
        override_default
    }

    /// Loads an extracted `GameScript` (`game_script.json`) and activates script-driven execution.
    pub fn load_game_script(&mut self, script: GameScript) {
        self.balls_per_game = script.meta.balls_per_game.max(1);
        self.adjustments.balls_per_game = self.balls_per_game;
        self.adjustments.ball_save_secs = script.meta.ball_save_secs;
        self.status_banner = format!("{} v{} - PRESS START", script.meta.title, script.meta.version);
        if script.meta.game_number == 50069 {
            self.game_title = GameTitle::Swe1;
        } else if script.meta.game_number == 50070 {
            self.game_title = GameTitle::Rfm;
        }
        self.script_runtime = Some(ScriptRuntime::new(script));
        self.eval_script_trigger(ScriptTrigger::OnBoot);
    }

    pub fn active_game_number(&self) -> u32 {
        if let Some(ref rt) = self.script_runtime {
            rt.script.meta.game_number
        } else {
            match self.game_title {
                GameTitle::Swe1 | GameTitle::Homebrew => 50069,
                GameTitle::Rfm => 50070,
            }
        }
    }

    /// Restores persisted high scores, audits, and adjustments from `LocalDb`.
    pub fn apply_db(&mut self, db: &LocalDb) {
        let gnum = self.active_game_number();
        if let Some(rec) = db.games.get(&gnum) {
            if !rec.high_scores.is_empty() {
                self.high_scores = rec.high_scores.clone();
            }
            self.audits = rec.audits.clone();
            self.adjustments = rec.adjustments.clone();
            self.free_play = self.adjustments.free_play;
            self.balls_per_game = self.adjustments.balls_per_game.max(1);
        }
    }

    /// Snapshots current high scores, audits, and adjustments into `LocalDb`.
    pub fn sync_to_db(&self, db: &mut LocalDb) {
        let gnum = self.active_game_number();
        let rec = db.get_or_insert_game(gnum);
        rec.high_scores = self.high_scores.clone();
        rec.audits = self.audits.clone();
        rec.adjustments = self.adjustments.clone();
    }

    /// Builds a `GameHistoryEntry` for the current session.
    pub fn snapshot_game_history(&self) -> GameHistoryEntry {
        let scores: Vec<u64> = self.players.iter().map(|p| p.score).collect();
        let best_score = scores.iter().copied().max().unwrap_or(0);
        let scenes_completed = self
            .players
            .iter()
            .map(|p| p.scenes_completed)
            .max()
            .unwrap_or(0);
        GameHistoryEntry {
            timestamp_ms: self.elapsed_ms,
            game_number: self.active_game_number(),
            players: self.players.len(),
            scores,
            best_score,
            scenes_completed,
        }
    }

    /// Debug helper: immediately jumps to a state, SWE1 scene, or scripted mode.
    pub fn debug_launch_mode(&mut self, target: &str) -> bool {
        let key = target.trim().to_ascii_lowercase();
        self.debug_anim_override = None;
        self.special_screen = None;
        self.special_screen_ms = 0;

        match key.as_str() {
            "game:swe1" | "swe1" => {
                self.game_title = GameTitle::Swe1;
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.balls_in_play = 0;
                self.active_scene = None;
                self.active_anim_hint.clear();
                self.set_banner("GAME: STAR WARS EPISODE I", 3_000);
                return true;
            }
            "game:rfm" | "rfm" => {
                self.game_title = GameTitle::Rfm;
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.balls_in_play = 0;
                self.active_scene = None;
                self.active_anim_hint.clear();
                self.set_banner("GAME: REVENGE FROM MARS", 3_000);
                return true;
            }
            "boot" => {
                self.cut_all_holds();
                self.state = MachineState::Boot;
                self.elapsed_ms = 0;
                self.balls_in_play = 0;
                self.active_scene = None;
                self.active_anim_hint.clear();
                self.set_banner("DEBUG: BOOT SEQUENCE", 2_000);
                return true;
            }
            "attract" => {
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.balls_in_play = 0;
                self.active_scene = None;
                self.multiball = ActiveMultiball::None;
                self.active_anim_hint.clear();
                self.set_banner("DEBUG: ATTRACT MODE", 3_000);
                return true;
            }
            "ingame" | "start" => {
                self.ensure_ingame_for(self.game_title);
                self.active_scene = None;
                self.multiball = ActiveMultiball::None;
                self.left_spinner = VirtualSpinnerState::default();
                self.right_spinner = VirtualSpinnerState::default();
                self.active_anim_hint.clear();
                self.set_banner("DEBUG: IN GAME", 3_000);
                return true;
            }
            "skill_shot" => {
                self.ensure_ingame_swe1();
                self.active_scene = None;
                self.multiball = ActiveMultiball::None;
                self.skill_shot_ready = true;
                self.active_anim_hint = "anim_super_skill_shot_icon".to_string();
                self.set_banner("SKILL SHOT READY - SHOOT JETS!", 4_000);
                return true;
            }
            "hangar_multiball" => {
                self.ensure_ingame_swe1();
                self.active_scene = None;
                self.multiball = ActiveMultiball::HangarMultiball {
                    jackpots_lit: true,
                    jackpots_collected: 0,
                };
                self.pending_eject_balls = self.pending_eject_balls.saturating_add(2);
                self.ball_save_ms = 20_000;
                self.emit_sound(sound::MULTIBALL_MUSIC_LOOP, true);
                self.active_anim_hint = "anim_multiball_start_movie".to_string();
                self.set_banner("THEED HANGAR 3-BALL MULTIBALL!", 4_000);
                return true;
            }
            "holo_select" => {
                self.ensure_ingame_swe1();
                self.active_scene = None;
                self.multiball = ActiveMultiball::None;
                self.players[self.current_player].shield_health = 0;
                self.players[self.current_player].scene_select_ready = true;
                self.set_shield_raised(true);
                let idx = self.players[self.current_player].next_scene_idx % Swe1Scene::ALL.len();
                let sc = Swe1Scene::ALL[idx];
                self.active_anim_hint = sc.holo_anim_hint().to_string();
                self.set_banner(format!("SELECT SCENE: {}", sc.title()), 4_000);
                return true;
            }
            "spinner_left" => {
                self.ensure_ingame_swe1();
                self.trigger_virtual_spinner(false, true);
                self.set_banner("LEFT RAMP SPINNER ACTIVE", 2_500);
                return true;
            }
            "spinner_right" => {
                self.ensure_ingame_swe1();
                self.trigger_virtual_spinner(true, true);
                self.set_banner("RIGHT RAMP SPINNER ACTIVE", 2_500);
                return true;
            }
            "sub_escape" => {
                self.ensure_ingame_swe1();
                self.active_scene = None;
                self.multiball = ActiveMultiball::None;
                self.special_screen = Some(super::SpecialScreen::SubEscape);
                self.active_anim_hint = "anim_gungan_sub_chomp".to_string();
                self.emit_sound(sound::BATTLE_MUSIC_LOOP, true);
                self.set_banner("SUB ESCAPE!", 4_000);
                return true;
            }
            "build_c3p0" => {
                self.ensure_ingame_swe1();
                self.active_scene = None;
                self.special_screen = Some(super::SpecialScreen::BuildC3P0);
                self.active_anim_hint = "anim_build_c3p0".to_string();
                self.set_banner("BUILD C-3PO", 4_000);
                return true;
            }
            "ramp_combo" => {
                self.ensure_ingame_swe1();
                self.special_screen = Some(super::SpecialScreen::RampCombo);
                self.active_anim_hint = "anim_naboo_flyby_single".to_string();
                self.set_banner("RAMP COMBO!", 3_000);
                return true;
            }
            "jedi_ready" | "jedi_multiball_ready" => {
                self.ensure_ingame_swe1();
                self.active_scene = None;
                self.special_screen = Some(super::SpecialScreen::JediMultiballReady);
                self.active_anim_hint = "anim_saber_ball_loop".to_string();
                self.set_banner("JEDI MULTIBALL READY!", 4_000);
                return true;
            }
            "bonus" => {
                self.ensure_ingame_swe1();
                self.active_scene = None;
                self.multiball = ActiveMultiball::None;
                self.special_screen = Some(super::SpecialScreen::Bonus);
                self.active_anim_hint = "anim_bonus_background".to_string();
                self.emit_ard("ard_bonus_tune");
                self.trigger_leff(super::LeffKind::BonusShow);
                let p = &self.players[self.current_player];
                let total = p.bonus_points.saturating_mul(p.bonus_multiplier as u64);
                self.set_banner(format!("BONUS: {} X {} = {}", p.bonus_points, p.bonus_multiplier, total), 4_000);
                return true;
            }
            "match" => {
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.special_screen = Some(super::SpecialScreen::Match);
                self.active_anim_hint = "anim_match_movie".to_string();
                self.set_banner("MATCH SEQUENCE", 4_000);
                return true;
            }
            "game_over" => {
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.special_screen = Some(super::SpecialScreen::GameOver);
                self.active_anim_hint = "anim_darth_maul_game_over".to_string();
                self.emit_sound(sound::GAME_OVER_MUSIC, false);
                self.set_banner("GAME OVER", 4_000);
                return true;
            }
            "game_intro" => {
                self.ensure_ingame_swe1();
                self.active_scene = None;
                self.special_screen = Some(super::SpecialScreen::GameIntro);
                self.active_anim_hint = "anim_republic_cruiser_intro".to_string();
                self.set_banner("EPISODE I INTRO", 4_000);
                return true;
            }
            "extra_ball" => {
                self.ensure_ingame_swe1();
                self.players[self.current_player].extra_balls += 1;
                self.special_screen = Some(super::SpecialScreen::ExtraBall);
                self.active_anim_hint = "anim_extra_ball_show".to_string();
                self.emit_ard("ard_extra_ball_fanfare");
                self.set_banner("EXTRA BALL!", 3_500);
                return true;
            }
            "ball_save" => {
                self.ensure_ingame_swe1();
                self.ball_save_ms = 15_000;
                self.special_screen = Some(super::SpecialScreen::BallSave);
                self.emit_ard("ard_ball_saved");
                self.set_banner("SHOOT AGAIN! FORCE BALL SAVE!", 3_000);
                return true;
            }
            "tilt_warning" => {
                self.ensure_ingame_swe1();
                self.special_screen = Some(super::SpecialScreen::TiltWarning);
                self.emit_ard("ard_tilt_warning_speech");
                self.set_banner("DANGER! TILT WARNING!", 3_000);
                return true;
            }
            "slam_tilt" => {
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.balls_in_play = 0;
                self.special_screen = Some(super::SpecialScreen::SlamTilt);
                self.set_banner("SLAM TILT!", 4_000);
                return true;
            }
            "diagnostics" | "admin" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::SwitchTest;
                self.admin_screen = AdminScreen::Root { cursor: 0 };
                self.set_banner("DEBUG: ADMIN MENU", 3_000);
                return true;
            }
            "debug" | "debug_menu" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::SwitchTest;
                self.admin_screen = AdminScreen::DebugLauncher { cursor: 0 };
                self.set_banner("DEBUG LAUNCHER", 3_000);
                return true;
            }
            "diag:switch" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::SwitchTest;
                self.admin_screen = AdminScreen::SwitchTest;
                return true;
            }
            "diag:coil" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::CoilTest { selected_drive: 0 };
                self.admin_screen = AdminScreen::CoilTest { selected_drive: 0 };
                return true;
            }
            "diag:lamp" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::LampTest { selected_lamp: 2 };
                self.admin_screen = AdminScreen::LampTest { selected_lamp: 2 };
                return true;
            }
            "diag:sound" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::SwitchTest;
                self.admin_screen = AdminScreen::SoundTest { selected_idx: 0 };
                return true;
            }
            "diag:align" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::AlignTest;
                self.active_anim_hint = "system_video_test_align".to_string();
                return true;
            }
            "diag:colorbar" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::ColorBarTest;
                return true;
            }
            "diag:grid" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::GridTest;
                return true;
            }
            "diag:rgb" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::RgbTest;
                return true;
            }
            "diag:fuse" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.diag_page = DiagPage::FuseTest;
                return true;
            }
            "hstd_entry" | "high_score_entry" => {
                self.cut_all_holds();
                let score = self.score.max(95_000_000);
                self.hstd_session = HstdEntrySession {
                    queue: vec![(1, score, JediRank::Knight)],
                    initials: ['A', 'A', 'A'],
                    cursor: 0,
                    char_idx: 0,
                };
                self.state = MachineState::HighScoreEntry;
                self.set_banner("DEBUG: HIGH SCORE ENTRY", 3_000);
                return true;
            }
            "tilt" => {
                self.ensure_ingame_swe1();
                self.cut_all_holds();
                self.state = MachineState::Tilt;
                self.emit_ard("ard_tilt_random_speech");
                self.set_banner("DEBUG: TILT", 3_000);
                return true;
            }
            _ => {}
        }

        // Attract specific slide (`attract:0`..`attract:19`)
        if let Some(idx_str) = key.strip_prefix("attract:") {
            if let Ok(idx) = idx_str.parse::<usize>() {
                if idx < AttractDeff::ALL.len() {
                    self.cut_all_holds();
                    self.game_title = GameTitle::Swe1;
                    self.state = MachineState::Attract;
                    self.balls_in_play = 0;
                    self.active_scene = None;
                    self.attract.index = idx;
                    self.attract.timer_ms = 0;
                    self.attract.last_game_played = true;
                    self.active_anim_hint.clear();
                    return true;
                }
            }
        }

        // RFM Waves (`rfm_wave:0`..`8`, `capture_multiball`, `martian_attack`)
        if let Some(wave_idx_str) = key.strip_prefix("rfm_wave:") {
            if let Some(wave) = wave_idx_str
                .parse::<usize>()
                .ok()
                .and_then(|i| RfmWave::ALL.get(i).copied())
            {
                self.ensure_ingame_for(GameTitle::Rfm);
                self.rfm.active_wave = Some(wave);
                self.rfm.wave_hits_left = wave.required_hits();
                self.set_banner(format!("RFM WAVE: {}", wave.title()), 4_000);
                return true;
            }
        }
        if key == "capture_multiball" || key == "martian_attack" {
            self.ensure_ingame_for(GameTitle::Rfm);
            self.rfm.multiball = if key == "capture_multiball" {
                RfmMultiball::CaptureMultiball { jackpots: 0 }
            } else {
                RfmMultiball::MartianAttack
            };
            self.balls_in_play = 3;
            self.set_banner(format!("RFM: {}", key.to_ascii_uppercase()), 4_000);
            return true;
        }

        // Ensure we are InGame for scene/mode launches
        self.ensure_ingame_for(self.game_title);

        // Check SWE1 scenes (`scene:0`..`scene:13` or `podrace` etc.)
        let scene_opt = if let Some(idx_str) = key.strip_prefix("scene:") {
            idx_str
                .parse::<usize>()
                .ok()
                .and_then(|i| Swe1Scene::ALL.get(i).copied())
        } else {
            match key.as_str() {
                "podrace" | "boonta" => Some(Swe1Scene::Podrace),
                "battle_droid" => Some(Swe1Scene::BattleDroid),
                "destroyer_droid" => Some(Swe1Scene::DestroyerDroid),
                "federation_fighter" => Some(Swe1Scene::FederationFighter),
                "gungan_battle" => Some(Swe1Scene::GunganBattle),
                "hangar_escape" => Some(Swe1Scene::HangarEscape),
                "hover_tank" => Some(Swe1Scene::HoverTank),
                "probe_droid" => Some(Swe1Scene::ProbeDroid),
                "queen_fashion" => Some(Swe1Scene::QueenFashion),
                "jar_jar_juggle" => Some(Swe1Scene::JarJarJuggle),
                "wattos_chance" => Some(Swe1Scene::WattosChance),
                "mos_espa_cantina" => Some(Swe1Scene::MosEspaCantina),
                "r2d2" | "r2_d2" => Some(Swe1Scene::R2D2),
                "question_mark" | "mystery" => Some(Swe1Scene::QuestionMark),
                _ => None,
            }
        };

        if let Some(scene) = scene_opt {
            self.game_title = GameTitle::Swe1;
            self.multiball = ActiveMultiball::None;
            self.start_swe1_scene(scene);
            self.set_banner(format!("DEBUG SCENE: {}", scene.title()), 4_000);
            return true;
        }

        if key == "darth_maul_duel" || key == "maul" || key == "jedi_battle" {
            self.game_title = GameTitle::Swe1;
            self.active_scene = None;
            self.start_darth_maul_duel();
            return true;
        }

        // Check scripted modes in `game_script.json`
        let has_script_mode = self
            .script_runtime
            .as_ref()
            .is_some_and(|rt| rt.script.modes.iter().any(|m| m.id.eq_ignore_ascii_case(&key)));
        if has_script_mode {
            let exact_id = self
                .script_runtime
                .as_ref()
                .unwrap()
                .script
                .modes
                .iter()
                .find(|m| m.id.eq_ignore_ascii_case(&key))
                .unwrap()
                .id
                .clone();
            self.execute_script_actions(&[ScriptAction::StartMode { mode: exact_id }]);
            return true;
        }

        false
    }

    fn ensure_ingame_for(&mut self, game: GameTitle) {
        self.game_title = game;
        if self.state != MachineState::InGame {
            self.state = MachineState::Attract;
            self.try_start_or_add_player();
        }
    }

    fn ensure_ingame_swe1(&mut self) {
        self.ensure_ingame_for(GameTitle::Swe1);
    }

    /// Applies debug player/gameplay mutators (`add_player`, `next_player`, `ball:N`, `lasers:N`, `shield:N`, `jedi:N`, `rank:N`).
    pub fn debug_apply_player_cmd(&mut self, cmd: &str) -> bool {
        let key = cmd.trim().to_ascii_lowercase();
        if key == "add_player" {
            self.ensure_ingame_swe1();
            if self.players.len() < 4 {
                let used: Vec<usize> = self.players.iter().map(|p| p.character_idx).collect();
                let char_idx = PlayerState::pick_unique_character(self.players.len() + 1, &used);
                self.players.push(PlayerState {
                    character_idx: char_idx,
                    ..PlayerState::default()
                });
                return true;
            }
            return false;
        }
        if key == "next_player" && !self.players.is_empty() {
            self.current_player = (self.current_player + 1) % self.players.len();
            self.score = self.players[self.current_player].score;
            return true;
        }
        if let Some(v) = key.strip_prefix("ball:").and_then(|s| s.parse::<u8>().ok()) {
            self.current_ball = v.clamp(1, 5);
            return true;
        }
        if let Some(v) = key.strip_prefix("lasers:").and_then(|s| s.parse::<u8>().ok()) {
            self.players[self.current_player].laser_shots = v.min(9);
            return true;
        }
        if let Some(v) = key.strip_prefix("shield:").and_then(|s| s.parse::<u8>().ok()) {
            self.players[self.current_player].shield_health = v.min(12);
            self.set_shield_raised(v == 0);
            return true;
        }
        if let Some(v) = key.strip_prefix("jedi:").and_then(|s| s.parse::<u8>().ok()) {
            self.players[self.current_player].jedi_letters = v & 0x0F;
            return true;
        }
        if let Some(v) = key.strip_prefix("rank:").and_then(|s| s.parse::<u8>().ok()) {
            self.players[self.current_player].jedi_rank = match v {
                0 => JediRank::Padawan,
                1 => JediRank::Youth,
                2 => JediRank::Knight,
                3 => JediRank::Master,
                _ => JediRank::Spirit,
            };
            return true;
        }
        false
    }

    pub fn debug_play_anim(&mut self, anim_name: &str) {
        self.debug_anim_override = Some(anim_name.to_string());
        self.active_anim_hint = anim_name.to_string();
        self.set_banner(format!("ANIM: {anim_name}"), 3_000);
    }

    pub fn debug_play_sound(&mut self, sound_id: u16) {
        self.emit_sound(SoundId(sound_id), false);
    }

    pub fn debug_add_score(&mut self, points: u64) {
        self.ensure_ingame_for(self.game_title);
        self.add_score(points);
    }

    pub fn debug_pulse_drive(&mut self, drive_id: u8, ms: u8) {
        self.emit_coil(drive_id, ms);
    }

    /// Evaluates all `GameScript` rules matching `trigger` whose conditions hold.
    pub fn eval_script_trigger(&mut self, trigger: ScriptTrigger) -> bool {
        let Some(ref rt) = self.script_runtime else {
            return false;
        };
        let ball_in_play = self.balls_in_play > 0 && self.state == MachineState::InGame;
        let not_tilted = self.state != MachineState::Tilt;
        let matched_actions: Vec<Vec<ScriptAction>> = rt
            .script
            .rules
            .iter()
            .filter(|r| r.trigger == trigger)
            .filter(|r| rt.eval_conditions(&r.conditions, ball_in_play, not_tilted))
            .map(|r| r.actions.clone())
            .collect();

        if matched_actions.is_empty() {
            return false;
        }
        for actions in matched_actions {
            self.execute_script_actions(&actions);
        }
        true
    }

    pub(super) fn execute_script_actions(&mut self, actions: &[ScriptAction]) {
        for act in actions {
            match act {
                ScriptAction::AddScore { points } => self.add_score(*points),
                ScriptAction::SetVar { var, value } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        rt.set_var(var, *value);
                    }
                }
                ScriptAction::IncVar { var, delta } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        let next = rt.get_var(var).saturating_add(*delta);
                        rt.set_var(var, next);
                    }
                }
                ScriptAction::SetBit { var, bit } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        let next = rt.get_var(var) | (1i64 << (*bit as u32 & 63));
                        rt.set_var(var, next);
                    }
                }
                ScriptAction::ClearVar { var } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        rt.set_var(var, 0);
                    }
                }
                ScriptAction::PulseDrive { drive_id, ms } => {
                    self.emit_coil(*drive_id, *ms);
                }
                ScriptAction::SetLamp { lamp_id, on } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        if *on {
                            rt.lit_lamps.insert(*lamp_id);
                        } else {
                            rt.lit_lamps.remove(lamp_id);
                        }
                    }
                }
                ScriptAction::PlaySound { sound_id } => {
                    self.emit_sound(SoundId(*sound_id), false);
                }
                ScriptAction::PlayMusic { sound_id } => {
                    self.emit_sound(SoundId(*sound_id), true);
                }
                ScriptAction::PlayAnim { anim } => {
                    self.active_anim_hint = anim.clone();
                }
                ScriptAction::ShowBanner { text } => {
                    self.set_banner(text.clone(), 3_000);
                }
                ScriptAction::StartMode { mode } => {
                    let mode_def = self
                        .script_runtime
                        .as_ref()
                        .and_then(|rt| rt.script.modes.iter().find(|m| m.id == *mode).cloned());
                    if let Some(ref mut rt) = self.script_runtime {
                        let dur = mode_def
                            .as_ref()
                            .and_then(|m| m.duration_secs.map(|s| s.saturating_mul(1000)));
                        rt.active_modes.insert(mode.clone(), dur);
                    }
                    if let Some(m) = mode_def {
                        if let Some(anim) = m.anim_hint {
                            self.active_anim_hint = anim;
                        }
                        if let Some(sid) = m.sound_id {
                            self.emit_sound(SoundId(sid), false);
                        }
                        if let Some(mid) = m.music_id {
                            self.emit_sound(SoundId(mid), true);
                        }
                        self.execute_script_actions(&m.on_enter);
                    }
                    self.eval_script_trigger(ScriptTrigger::OnModeEnter { mode: mode.clone() });
                }
                ScriptAction::EndMode { mode } => {
                    let on_exit = self.script_runtime.as_mut().and_then(|rt| {
                        rt.active_modes.remove(mode);
                        rt.script
                            .modes
                            .iter()
                            .find(|m| m.id == *mode)
                            .map(|m| m.on_exit.clone())
                    });
                    if let Some(acts) = on_exit {
                        self.execute_script_actions(&acts);
                    }
                    self.eval_script_trigger(ScriptTrigger::OnModeExit { mode: mode.clone() });
                }
                ScriptAction::StartTimer { name, ticks } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        rt.timers.insert(name.clone(), *ticks);
                    }
                }
                ScriptAction::StartMultiball { balls } => {
                    let extra = balls.saturating_sub(self.balls_in_play);
                    if extra > 0 {
                        self.pending_eject_balls = self.pending_eject_balls.saturating_add(extra);
                    }
                    self.set_banner(format!("{balls}-BALL MULTIBALL!"), 4_000);
                }
                ScriptAction::AwardExtraBall => {
                    if let Some(p) = self.players.get_mut(self.current_player) {
                        p.extra_balls = p.extra_balls.saturating_add(1);
                    }
                    self.set_banner("EXTRA BALL AWARDED!", 3_500);
                }
                ScriptAction::AdvanceScene => {
                    if self.active_scene.is_some() {
                        self.advance_active_scene("SCENE ADVANCE!");
                    } else if let Some(p) = self.players.get_mut(self.current_player) {
                        let scene = p.choose_next_scene();
                        self.active_scene = Some(ActiveSceneState {
                            scene,
                            hits_remaining: scene.required_hits(),
                            timer_ms: scene.duration_ms(),
                            intro_remaining_ms: scene.intro_duration_ms(),
            active_ms: 0,
                        });
                        self.active_anim_hint = scene.anim_hint().to_string();
                        self.set_banner(format!("SCENE: {}", scene.title()), 3_000);
                    }
                }
                ScriptAction::CollectBonus => {
                    self.add_score(250_000);
                }
                ScriptAction::EjectBall => {
                    self.pending_eject_balls = self.pending_eject_balls.saturating_add(1);
                }
            }
        }
    }

    pub(super) fn apply_mod_switch_rules(&mut self, id: u8) -> bool {
        let mut matched = false;
        let rules: Vec<HomebrewSwitchRule> = self
            .mod_switch_rules
            .iter()
            .filter(|r| r.sw_id == id)
            .cloned()
            .collect();
        for rule in rules {
            matched = true;
            if rule.points > 0 {
                self.add_score(rule.points);
            }
            if let Some(coil) = rule.coil_id {
                self.emit_coil(coil, rule.pulse_ms);
            }
            if let Some(sid) = rule.sound_id {
                self.emit_sound(SoundId(sid), false);
            }
            if let Some(lid) = rule.lamp_id {
                if !self.mod_lit_lamps.contains(&lid) {
                    self.mod_lit_lamps.push(lid);
                }
            }
            if let Some(ref b) = rule.banner {
                self.set_banner(b.clone(), 2_500);
            }
            if let Some(ref anim) = rule.anim_hint {
                self.active_anim_hint = anim.clone();
            }
        }
        matched
    }

    pub(super) fn handle_homebrew_builtin_switch(&mut self, id: u8) {
        match id {
            sw::START_BUTTON => self.try_start_or_add_player(),
            sw::LEFT_FLIPPER_BUTTON => {
                self.emit_coil(drive::LEFT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(drive::LEFT_FLIPPER_HOLD, true);
            }
            sw::RIGHT_FLIPPER_BUTTON => {
                self.emit_coil(drive::RIGHT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(drive::RIGHT_FLIPPER_HOLD, true);
            }
            sw::TROUGH_BALL_1 | sw::TROUGH_BALL_2 | sw::TROUGH_BALL_3 | sw::TROUGH_BALL_4 => {
                if self.balls_in_play > 0 && self.eject_cooldown_ms == 0 {
                    self.on_ball_drained();
                }
            }
            _ => {}
        }
    }

}
