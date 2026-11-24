use pin2k_hal::{AudioBackend, PinballHardware, SoundId, VideoBackend};

use super::{HstdEntrySession, MachineState, Pin2kMachine};
use crate::admin::AdminScreen;
use crate::mods::{HomebrewSwitchRule, ModManifest};
use crate::rfm_rules::{GameTitle, RfmMultiball, RfmWave};
use crate::script_engine::{GameScript, ScriptAction, ScriptRuntime, ScriptTrigger};
use crate::storage::{GameHistoryEntry, LocalDb};
use crate::swe1_rules::{ActiveMultiball, ActiveSceneState, JediRank, Swe1Scene};
use crate::swe1_tables::{drive, sw};

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
        match key.as_str() {
            "game:swe1" | "swe1" => {
                self.game_title = GameTitle::Swe1;
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.balls_in_play = 0;
                self.active_scene = None;
                self.set_banner("GAME: STAR WARS EPISODE I", 3_000);
                return true;
            }
            "game:rfm" | "rfm" => {
                self.game_title = GameTitle::Rfm;
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.balls_in_play = 0;
                self.active_scene = None;
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
                self.active_anim_hint = self.attract.current().anim_hint().unwrap_or("").to_string();
                self.set_banner("DEBUG: ATTRACT MODE", 3_000);
                return true;
            }
            "ingame" | "start" => {
                if self.state != MachineState::InGame {
                    if !matches!(self.state, MachineState::Boot | MachineState::Attract) {
                        self.state = MachineState::Attract;
                    }
                    self.try_start_or_add_player();
                }
                self.active_scene = None;
                self.multiball = ActiveMultiball::None;
                if self.game_title == GameTitle::Swe1 {
                    self.active_anim_hint = "anim_spinner_1".to_string();
                }
                self.set_banner("DEBUG: IN GAME", 3_000);
                return true;
            }
            "diagnostics" | "admin" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.admin_screen = AdminScreen::Root { cursor: 0 };
                self.set_banner("DEBUG: ADMIN MENU", 3_000);
                return true;
            }
            "debug" | "debug_menu" => {
                self.cut_all_holds();
                self.state = MachineState::Diagnostics;
                self.admin_screen = AdminScreen::DebugLauncher { cursor: 0 };
                self.set_banner("DEBUG LAUNCHER", 3_000);
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
                self.cut_all_holds();
                self.state = MachineState::Tilt;
                self.set_banner("DEBUG: TILT", 3_000);
                return true;
            }
            _ => {}
        }

        // RFM Waves (`rfm_wave:0`..`8`, `capture_multiball`, `martian_attack`)
        if let Some(wave_idx_str) = key.strip_prefix("rfm_wave:") {
            if let Some(wave) = wave_idx_str
                .parse::<usize>()
                .ok()
                .and_then(|i| RfmWave::ALL.get(i).copied())
            {
                self.game_title = GameTitle::Rfm;
                if self.state != MachineState::InGame {
                    if !matches!(self.state, MachineState::Boot | MachineState::Attract) {
                        self.state = MachineState::Attract;
                    }
                    self.try_start_or_add_player();
                }
                self.rfm.active_wave = Some(wave);
                self.rfm.wave_hits_left = wave.required_hits();
                self.set_banner(format!("RFM WAVE: {}", wave.title()), 4_000);
                return true;
            }
        }
        if key == "capture_multiball" || key == "martian_attack" {
            self.game_title = GameTitle::Rfm;
            if self.state != MachineState::InGame {
                self.try_start_or_add_player();
            }
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
        if self.state != MachineState::InGame {
            if !matches!(self.state, MachineState::Boot | MachineState::Attract) {
                self.state = MachineState::Attract;
            }
            self.try_start_or_add_player();
        }

        // Check SWE1 scenes (`scene:0`..`scene:11` or `podrace` etc.)
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

    pub fn debug_play_anim(&mut self, anim_name: &str) {
        self.active_anim_hint = anim_name.to_string();
        self.set_banner(format!("ANIM: {anim_name}"), 3_000);
    }

    pub fn debug_play_sound(&mut self, sound_id: u16) {
        self.emit_sound(SoundId(sound_id), false);
    }

    pub fn debug_add_score(&mut self, points: u64) {
        if self.state != MachineState::InGame {
            if !matches!(self.state, MachineState::Boot | MachineState::Attract) {
                self.state = MachineState::Attract;
            }
            self.try_start_or_add_player();
        }
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
                        rt.vars.insert(var.clone(), *value);
                    }
                }
                ScriptAction::IncVar { var, delta } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        let e = rt.vars.entry(var.clone()).or_insert(0);
                        *e = e.saturating_add(*delta);
                    }
                }
                ScriptAction::SetBit { var, bit } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        let e = rt.vars.entry(var.clone()).or_insert(0);
                        *e |= 1i64 << (*bit as u32 & 63);
                    }
                }
                ScriptAction::ClearVar { var } => {
                    if let Some(ref mut rt) = self.script_runtime {
                        rt.vars.insert(var.clone(), 0);
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
                        self.advance_active_scene("SCENE ADVANCE!", 250_000);
                    } else if let Some(p) = self.players.get_mut(self.current_player) {
                        let scene = p.choose_next_scene();
                        self.active_scene = Some(ActiveSceneState {
                            scene,
                            hits_remaining: scene.required_hits(),
                            timer_ms: scene.duration_ms(),
                            intro_remaining_ms: scene.intro_duration_ms(),
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
