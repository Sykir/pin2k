//! Deterministic fixed-timestep *Star Wars Episode I* (`#50069`) Pinball 2000 machine runtime.

use pin2k_hal::{
    AssetId, AudioBackend, HeadlessAudio, LampState, PinballHardware, SoundId, VideoBackend,
};
use serde::{Deserialize, Serialize};

use crate::admin::{
    AdminScreen, GameAdjustments, GameAudits, ADMIN_SOUND_PRESETS, DEBUG_LAUNCH_PRESETS,
};
use crate::attract::{AttractController, AttractDeff};
use crate::events::{GameEvent, TimestampedEvent};
use crate::mods::{HomebrewSwitchRule, ModManifest};
use crate::rfm_rules::{GameTitle, RfmMultiball, RfmState, RfmWave};
use crate::rfm_tables::{rfm_drive, rfm_drive_name, rfm_lamp, rfm_sound, rfm_sw, rfm_switch_name};
use crate::script_engine::{GameScript, ScriptAction, ScriptRuntime, ScriptTrigger};
use crate::storage::{GameHistoryEntry, LocalDb};
use crate::swe1_rules::{
    default_high_scores, ActiveMultiball, ActiveSceneState, HighScoreEntry, JediRank, PlayerState,
    Swe1Scene,
};
use crate::swe1_tables::{drive, lamp, sound, sw, drive_name, switch_name};
use crate::switch_matrix::{SwitchDebouncer, SwitchEvent};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MachineState {
    Boot,
    Attract,
    InGame,
    Tilt,
    HighScoreEntry,
    Diagnostics,
}

const HSTD_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 ";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HstdEntrySession {
    pub queue: Vec<(usize, u64, JediRank)>,
    pub initials: [char; 3],
    pub cursor: usize,
    pub char_idx: usize,
}

impl Default for HstdEntrySession {
    fn default() -> Self {
        Self {
            queue: Vec::new(),
            initials: ['A', 'A', 'A'],
            cursor: 0,
            char_idx: 0,
        }
    }
}

/// Operator diagnostic menu page (`DiagnosticSwitchTestDeff`, `DiagnosticCoilTestDeff`, `DiagnosticSingleLampDeff`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagPage {
    SwitchTest,
    CoilTest { selected_drive: u8 },
    LampTest { selected_lamp: u8 },
}

/// Deterministic Pinball 2000 / *Star Wars Episode I* machine controller.
pub struct Pin2kMachine<H: PinballHardware, V: VideoBackend, A: AudioBackend = HeadlessAudio> {
    pub hw: H,
    pub video: V,
    pub audio: A,
    pub game_title: GameTitle,
    pub state: MachineState,
    pub elapsed_ms: u64,

    // Cabinet & economy settings
    pub free_play: bool,
    pub credits: u8,
    pub balls_per_game: u8,
    pub coin_door_closed: bool,

    // Active game & multi-player state
    pub players: Vec<PlayerState>,
    pub current_player: usize,
    pub current_ball: u8,
    pub score: u64, // mirror of active player's score for quick inspection
    pub high_scores: Vec<HighScoreEntry>,

    // Ball tracking & playfield mechanisms
    pub balls_in_play: u8,
    pub ball_save_ms: u32,
    pub skill_shot_ready: bool,
    pub shield_raised: bool,
    pub left_drop_down: bool,
    pub right_drop_down: bool,
    pub idle_ball_search_ms: u32,
    pub pending_eject_balls: u8,
    pub eject_cooldown_ms: u32,
    pub saucer_hold_ms: [u32; 2], // [left_saucer, right_saucer]
    pub popper_hold_ms: u32,
    pub tilt_warnings: u8,

    // Active scene / multiball / HUD banner
    pub active_scene: Option<ActiveSceneState>,
    pub multiball: ActiveMultiball,
    pub rfm: RfmState,
    pub attract: AttractController,
    pub status_banner: String,
    pub banner_timer_ms: u32,
    pub active_anim_hint: String,
    pub diag_page: DiagPage,
    pub admin_screen: AdminScreen,
    pub hstd_session: HstdEntrySession,
    pub adjustments: GameAdjustments,
    pub audits: GameAudits,
    pub last_switch_hit: Option<u8>,
    pub score_multiplier: u64,
    pub mod_switch_rules: Vec<HomebrewSwitchRule>,
    pub mod_lit_lamps: Vec<u8>,
    pub script_runtime: Option<ScriptRuntime>,
    pub strict_script_mode: bool,

    debouncer: SwitchDebouncer,
    event_log: Vec<TimestampedEvent>,
}

impl<H: PinballHardware, V: VideoBackend> Pin2kMachine<H, V, HeadlessAudio> {
    pub fn new(hw: H, video: V) -> Self {
        Self::with_audio(hw, video, HeadlessAudio::default())
    }
}

impl<H: PinballHardware, V: VideoBackend, A: AudioBackend> Pin2kMachine<H, V, A> {
    pub fn with_audio(hw: H, video: V, audio: A) -> Self {
        Self::with_game(hw, video, audio, GameTitle::Swe1)
    }

    pub fn with_game(mut hw: H, video: V, mut audio: A, game_title: GameTitle) -> Self {
        let initial_sw = hw.read_logical_switches();
        let mut debouncer = SwitchDebouncer::new(2);
        debouncer.seed_state(initial_sw);
        audio.set_master_volume(0.85);
        audio.play_sound(sound::BOOT_BONG, false);

        let banner = format!("{} - PRESS START", game_title.title());

        Self {
            hw,
            video,
            audio,
            game_title,
            state: MachineState::Boot,
            elapsed_ms: 0,
            free_play: true,
            credits: 0,
            balls_per_game: 3,
            coin_door_closed: (initial_sw[10] & 0x02) != 0,
            players: vec![PlayerState::default()],
            current_player: 0,
            current_ball: 1,
            score: 0,
            high_scores: default_high_scores(),
            balls_in_play: 0,
            ball_save_ms: 0,
            skill_shot_ready: false,
            shield_raised: false,
            left_drop_down: false,
            right_drop_down: false,
            idle_ball_search_ms: 0,
            pending_eject_balls: 0,
            eject_cooldown_ms: 0,
            saucer_hold_ms: [0; 2],
            popper_hold_ms: 0,
            tilt_warnings: 0,
            active_scene: None,
            multiball: ActiveMultiball::None,
            rfm: RfmState::default(),
            attract: AttractController::default(),
            status_banner: banner,
            banner_timer_ms: 0,
            active_anim_hint: String::new(),
            diag_page: DiagPage::SwitchTest,
            admin_screen: AdminScreen::Root { cursor: 0 },
            hstd_session: HstdEntrySession::default(),
            adjustments: GameAdjustments::default(),
            audits: GameAudits::default(),
            last_switch_hit: None,
            score_multiplier: 1,
            mod_switch_rules: Vec::new(),
            mod_lit_lamps: Vec::new(),
            script_runtime: None,
            strict_script_mode: false,
            debouncer,
            event_log: Vec::new(),
        }
    }

    /// Applies a Mod or Homebrew `ModManifest` (`mod.json`) onto the running machine.
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
                self.set_banner("DEBUG: BOOT SEQUENCE", 2_000);
                return true;
            }
            "attract" => {
                self.cut_all_holds();
                self.state = MachineState::Attract;
                self.balls_in_play = 0;
                self.active_scene = None;
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
            self.active_scene = Some(ActiveSceneState {
                scene,
                hits_remaining: scene.required_hits(),
                timer_ms: scene.duration_ms(),
            });
            self.active_anim_hint = scene.anim_hint().to_string();
            self.audits.scenes_started = self.audits.scenes_started.saturating_add(1);
            self.emit_sound(scene.music_id(), true);
            self.emit_event(GameEvent::SceneStarted {
                scene,
                title: scene.title().to_string(),
            });
            self.set_banner(format!("DEBUG SCENE: {}", scene.title()), 4_000);
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

    fn execute_script_actions(&mut self, actions: &[ScriptAction]) {
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

    fn game_switch_name(&self, id: u8) -> &'static str {
        match self.game_title {
            GameTitle::Swe1 | GameTitle::Homebrew => switch_name(id),
            GameTitle::Rfm => rfm_switch_name(id),
        }
    }

    fn game_drive_name(&self, id: u8) -> &'static str {
        match self.game_title {
            GameTitle::Swe1 | GameTitle::Homebrew => drive_name(id),
            GameTitle::Rfm => rfm_drive_name(id),
        }
    }

    fn emit_event(&mut self, event: GameEvent) {
        self.event_log.push(TimestampedEvent {
            elapsed_ms: self.elapsed_ms,
            event,
        });
    }

    /// Drains all structured `GameEvent`s emitted since the last call.
    pub fn drain_events(&mut self) -> Vec<TimestampedEvent> {
        std::mem::take(&mut self.event_log)
    }

    fn emit_coil(&mut self, drive_id: u8, pulse_ms: u8) {
        self.hw.fire_coil(drive_id, pulse_ms);
        self.emit_event(GameEvent::CoilFired {
            drive_id,
            name: self.game_drive_name(drive_id).to_string(),
            pulse_ms,
        });
    }

    fn emit_sound(&mut self, id: pin2k_hal::SoundId, looping: bool) {
        self.audio.play_sound(id, looping);
        self.emit_event(GameEvent::SoundTriggered {
            sound_id: id.0,
            looping,
        });
    }

    fn set_banner(&mut self, msg: impl Into<String>, duration_ms: u32) {
        self.status_banner = msg.into();
        self.banner_timer_ms = duration_ms;
    }

    fn add_score(&mut self, pts: u64) {
        let scaled = pts.saturating_mul(self.score_multiplier.max(1));
        if let Some(p) = self.players.get_mut(self.current_player) {
            p.score = p.score.saturating_add(scaled);
            p.bonus_points = p.bonus_points.saturating_add(scaled / 10);
            self.score = p.score;
            if p.score > self.audits.highest_score {
                self.audits.highest_score = p.score;
            }
        }
    }

    /// Advances the deterministic *Star Wars Episode I* simulation by `dt_ms` milliseconds.
    pub fn tick(&mut self, dt_ms: u32) -> Vec<SwitchEvent> {
        self.elapsed_ms = self.elapsed_ms.saturating_add(dt_ms as u64);

        // Sync hardware bus (PDB parallel-port scan + drive pulse decay + lamp multiplex)
        self.hw.sync(dt_ms);
        let raw_switches = self.hw.read_logical_switches();
        self.coin_door_closed = (raw_switches[10] & 0x02) != 0;
        let events = self.debouncer.update_logical(raw_switches);

        for &ev in &events {
            match ev {
                SwitchEvent::Closed(sw_id) => {
                    let id = sw_id.linear_id();
                    self.emit_event(GameEvent::SwitchChanged {
                        sw_id: id,
                        name: self.game_switch_name(id).to_string(),
                        closed: true,
                    });
                    self.handle_switch_closed(id);
                }
                SwitchEvent::Opened(sw_id) => {
                    let id = sw_id.linear_id();
                    self.emit_event(GameEvent::SwitchChanged {
                        sw_id: id,
                        name: self.game_switch_name(id).to_string(),
                        closed: false,
                    });
                    self.handle_switch_opened(id);
                }
            }
        }

        self.update_timers(dt_ms);
        self.update_lamps();
        self.render_frame();

        events
    }

    fn handle_switch_closed(&mut self, id: u8) {
        self.last_switch_hit = Some(id);

        // Coin slots work in every state (`hook_coin_credit`)
        if matches!(id, sw::LEFT_COIN | sw::CENTER_COIN | sw::RIGHT_COIN | sw::FOURTH_COIN) {
            self.credits = self.credits.saturating_add(1);
            self.audits.total_credits = self.audits.total_credits.saturating_add(1);
            self.emit_sound(sound::COIN_CREDIT, false);
            self.emit_event(GameEvent::CreditAdded { credits: self.credits });
            self.set_banner(format!("CREDITS: {}", self.credits), 3_000);
            return;
        }

        // Slam tilt aborts game immediately
        if id == sw::SLAM_TILT {
            self.cut_all_holds();
            self.state = MachineState::Attract;
            self.balls_in_play = 0;
            self.emit_event(GameEvent::SlamTilted);
            self.set_banner("SLAM TILT!", 4_000);
            return;
        }

        // Service / Diagnostics menu entry (`Sw 72..75`) — accessible from Boot, Attract, InGame, or Tilt
        if id == sw::DIAG_ENTER && self.state != MachineState::Diagnostics {
            self.state = MachineState::Diagnostics;
            self.diag_page = DiagPage::SwitchTest;
            self.admin_screen = AdminScreen::Root { cursor: 0 };
            self.emit_event(GameEvent::AdminMenuEntered);
            self.set_banner("OPERATOR ADMIN MENU", 5_000);
            return;
        }

        match self.state {
            MachineState::Boot => {
                if id == sw::START_BUTTON {
                    self.state = MachineState::Attract;
                    self.try_start_or_add_player();
                }
            }
            MachineState::Attract => {
                if id == sw::START_BUTTON {
                    self.try_start_or_add_player();
                } else if id == sw::LEFT_FLIPPER_BUTTON {
                    // Lifted AttractMode::left_hit (`0x00100358`)
                    self.attract.advance(-1);
                    self.emit_sound(sound::LASER_SFX_OR_CLICK(), false);
                } else if id == sw::RIGHT_FLIPPER_BUTTON {
                    // Lifted AttractMode::right_hit (`0x001003e0`)
                    self.attract.advance(1);
                    self.emit_sound(sound::LASER_SFX_OR_CLICK(), false);
                }
            }
            MachineState::Diagnostics => self.handle_diagnostics_switch(id),
            MachineState::HighScoreEntry => self.handle_hstd_entry_switch(id),
            MachineState::Tilt => {
                if matches!(id, sw::TROUGH_BALL_1 | sw::TROUGH_BALL_2 | sw::TROUGH_BALL_3 | sw::TROUGH_BALL_4) {
                    self.on_ball_drained();
                }
            }
            MachineState::InGame => {
                // Evaluate mod / homebrew switch rules first
                let matched_mod = self.apply_mod_switch_rules(id);
                let matched_script = self.eval_script_trigger(ScriptTrigger::OnSwitch { sw_id: id });
                if self.game_title == GameTitle::Homebrew {
                    if !matched_mod && !matched_script {
                        self.handle_homebrew_builtin_switch(id);
                    }
                    return;
                }
                if (matched_script || self.strict_script_mode) && self.script_runtime.is_some() {
                    // Script handled this switch (or strict script mode is active); only run core hardware ball/flipper/trough mechanics
                    if matches!(
                        id,
                        sw::TROUGH_BALL_1
                            | sw::TROUGH_BALL_2
                            | sw::TROUGH_BALL_3
                            | sw::TROUGH_BALL_4
                            | sw::LEFT_FLIPPER_BUTTON
                            | sw::RIGHT_FLIPPER_BUTTON
                            | sw::SHOOTER_LANE
                            | sw::PLUMB_TILT
                            | sw::START_BUTTON
                    ) {
                        match self.game_title {
                            GameTitle::Swe1 => self.handle_ingame_switch_closed(id),
                            GameTitle::Rfm => self.handle_rfm_ingame_switch_closed(id),
                            GameTitle::Homebrew => {}
                        }
                    }
                    return;
                }
                match self.game_title {
                    GameTitle::Swe1 => self.handle_ingame_switch_closed(id),
                    GameTitle::Rfm => self.handle_rfm_ingame_switch_closed(id),
                    GameTitle::Homebrew => {}
                }
            }
        }
    }

    fn handle_switch_opened(&mut self, id: u8) {
        if id == sw::LEFT_FLIPPER_BUTTON {
            self.hw.set_drive_hold(drive::LEFT_FLIPPER_HOLD, false);
        } else if id == sw::RIGHT_FLIPPER_BUTTON {
            self.hw.set_drive_hold(drive::RIGHT_FLIPPER_HOLD, false);
        }
    }

    fn handle_diagnostics_switch(&mut self, id: u8) {
        match id {
            sw::DIAG_ESCAPE => match self.admin_screen {
                AdminScreen::Root { .. } => {
                    self.cut_all_holds();
                    self.state = MachineState::Attract;
                    self.emit_event(GameEvent::AdminMenuExited);
                    self.set_banner("STAR WARS EPISODE I - PRESS START", 0);
                }
                AdminScreen::SwitchTest
                | AdminScreen::CoilTest { .. }
                | AdminScreen::LampTest { .. }
                | AdminScreen::SoundTest { .. } => {
                    self.admin_screen = AdminScreen::DiagnosticsMenu { cursor: 0 };
                    self.set_banner("DIAGNOSTICS MENU", 3_000);
                }
                AdminScreen::DiagnosticsMenu { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 0 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
                AdminScreen::Audits { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 1 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
                AdminScreen::Adjustments { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 2 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
                AdminScreen::Utilities { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 3 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
                AdminScreen::DebugLauncher { .. } => {
                    self.admin_screen = AdminScreen::Root { cursor: 4 };
                    self.set_banner("OPERATOR ADMIN MENU", 3_000);
                }
            },
            sw::DIAG_ENTER => match self.admin_screen {
                AdminScreen::Root { cursor } => {
                    self.admin_screen = match cursor {
                        0 => AdminScreen::DiagnosticsMenu { cursor: 0 },
                        1 => AdminScreen::Audits { cursor: 0 },
                        2 => AdminScreen::Adjustments { cursor: 0 },
                        3 => AdminScreen::Utilities { cursor: 0 },
                        _ => AdminScreen::DebugLauncher { cursor: 0 },
                    };
                }
                AdminScreen::DiagnosticsMenu { cursor } => {
                    self.admin_screen = match cursor {
                        0 => {
                            self.diag_page = DiagPage::SwitchTest;
                            AdminScreen::SwitchTest
                        }
                        1 => {
                            self.diag_page = DiagPage::CoilTest { selected_drive: 0 };
                            AdminScreen::CoilTest { selected_drive: 0 }
                        }
                        2 => {
                            self.diag_page = DiagPage::LampTest { selected_lamp: 2 };
                            AdminScreen::LampTest { selected_lamp: 2 }
                        }
                        _ => AdminScreen::SoundTest { selected_idx: 0 },
                    };
                }
                AdminScreen::SwitchTest => {
                    self.diag_page = DiagPage::CoilTest { selected_drive: 0 };
                    self.admin_screen = AdminScreen::CoilTest { selected_drive: 0 };
                }
                AdminScreen::CoilTest { selected_drive } => {
                    if self.coin_door_closed {
                        self.emit_coil(selected_drive, 48);
                        self.set_banner(
                            format!("PULSED COIL {:02}: {}", selected_drive, drive_name(selected_drive)),
                            3_000,
                        );
                    } else {
                        self.set_banner("CLOSE COIN DOOR FOR HV COILS", 3_000);
                    }
                }
                AdminScreen::LampTest { selected_lamp } => {
                    self.set_banner(format!("LAMP #{:03} ACTIVE", selected_lamp), 3_000);
                }
                AdminScreen::SoundTest { selected_idx } => {
                    let (sid, label) = ADMIN_SOUND_PRESETS[(selected_idx as usize) % ADMIN_SOUND_PRESETS.len()];
                    self.emit_sound(SoundId(sid), false);
                    self.set_banner(format!("PLAY SOUND 0x{:04X}: {}", sid, label), 3_000);
                }
                AdminScreen::Audits { .. } => {}
                AdminScreen::Adjustments { cursor } => {
                    self.toggle_adjustment(cursor);
                }
                AdminScreen::Utilities { cursor } => {
                    self.execute_utility(cursor);
                }
                AdminScreen::DebugLauncher { cursor } => {
                    if let Some(&(target, _)) = DEBUG_LAUNCH_PRESETS.get(cursor as usize) {
                        self.debug_launch_mode(target);
                    }
                }
            },
            sw::DIAG_UP | sw::DIAG_DOWN => {
                let step: i16 = if id == sw::DIAG_UP { 1 } else { -1 };
                match self.admin_screen {
                    AdminScreen::Root { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(5) as u8;
                        self.admin_screen = AdminScreen::Root { cursor: next };
                    }
                    AdminScreen::DiagnosticsMenu { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(4) as u8;
                        self.admin_screen = AdminScreen::DiagnosticsMenu { cursor: next };
                    }
                    AdminScreen::SwitchTest => {}
                    AdminScreen::CoilTest { selected_drive } => {
                        let next_drive = (selected_drive as i16 + step).rem_euclid(48) as u8;
                        self.diag_page = DiagPage::CoilTest { selected_drive: next_drive };
                        self.admin_screen = AdminScreen::CoilTest { selected_drive: next_drive };
                        if self.coin_door_closed {
                            self.emit_coil(next_drive, 48);
                        }
                        self.set_banner(
                            format!("COIL {:02}: {}", next_drive, drive_name(next_drive)),
                            4_000,
                        );
                    }
                    AdminScreen::LampTest { selected_lamp } => {
                        let next_lamp = (selected_lamp as i16 + step).rem_euclid(128) as u8;
                        self.diag_page = DiagPage::LampTest { selected_lamp: next_lamp };
                        self.admin_screen = AdminScreen::LampTest { selected_lamp: next_lamp };
                        self.set_banner(format!("LAMP TEST #{:03}", next_lamp), 4_000);
                    }
                    AdminScreen::SoundTest { selected_idx } => {
                        let len = ADMIN_SOUND_PRESETS.len() as i16;
                        let next = (selected_idx as i16 + step).rem_euclid(len) as u8;
                        let (sid, label) = ADMIN_SOUND_PRESETS[next as usize];
                        self.admin_screen = AdminScreen::SoundTest { selected_idx: next };
                        self.set_banner(format!("SND 0x{:04X}: {}", sid, label), 4_000);
                    }
                    AdminScreen::Audits { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(8) as u8;
                        self.admin_screen = AdminScreen::Audits { cursor: next };
                    }
                    AdminScreen::Adjustments { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(6) as u8;
                        self.admin_screen = AdminScreen::Adjustments { cursor: next };
                    }
                    AdminScreen::Utilities { cursor } => {
                        let next = (cursor as i16 + step).rem_euclid(3) as u8;
                        self.admin_screen = AdminScreen::Utilities { cursor: next };
                    }
                    AdminScreen::DebugLauncher { cursor } => {
                        let len = DEBUG_LAUNCH_PRESETS.len() as i16;
                        let next = (cursor as i16 + step).rem_euclid(len) as u8;
                        self.admin_screen = AdminScreen::DebugLauncher { cursor: next };
                    }
                }
            }
            other => {
                if matches!(self.admin_screen, AdminScreen::SwitchTest) {
                    self.set_banner(format!("SW {:02}: {}", other, switch_name(other)), 4_000);
                }
            }
        }
    }

    fn toggle_adjustment(&mut self, cursor: u8) {
        let (name, val) = match cursor {
            0 => {
                self.adjustments.free_play = !self.adjustments.free_play;
                self.free_play = self.adjustments.free_play;
                ("FREE PLAY", if self.free_play { "YES" } else { "NO" }.to_string())
            }
            1 => {
                self.adjustments.balls_per_game = if self.adjustments.balls_per_game == 3 { 5 } else { 3 };
                self.balls_per_game = self.adjustments.balls_per_game;
                ("BALLS PER GAME", self.balls_per_game.to_string())
            }
            2 => {
                self.adjustments.ball_save_secs = match self.adjustments.ball_save_secs {
                    10 => 15,
                    15 => 20,
                    _ => 10,
                };
                ("BALL SAVE SECS", format!("{}S", self.adjustments.ball_save_secs))
            }
            3 => {
                self.adjustments.tilt_warnings_max = match self.adjustments.tilt_warnings_max {
                    2 => 3,
                    3 => 4,
                    _ => 2,
                };
                ("TILT WARNINGS", self.adjustments.tilt_warnings_max.to_string())
            }
            4 => {
                self.adjustments.master_volume_pct = match self.adjustments.master_volume_pct {
                    25 => 50,
                    50 => 75,
                    75 => 85,
                    85 => 100,
                    _ => 25,
                };
                self.audio
                    .set_master_volume((self.adjustments.master_volume_pct as f32) / 100.0);
                ("MASTER VOLUME", format!("{}%", self.adjustments.master_volume_pct))
            }
            _ => {
                self.adjustments.difficulty = self.adjustments.difficulty.next();
                ("DIFFICULTY", self.adjustments.difficulty.label().to_string())
            }
        };
        self.emit_event(GameEvent::AdjustmentChanged {
            name: name.to_string(),
            value: val.clone(),
        });
        self.set_banner(format!("{name}: {val}"), 3_000);
    }

    fn execute_utility(&mut self, cursor: u8) {
        let action = match cursor {
            0 => {
                self.high_scores = default_high_scores();
                "HIGH SCORES RESET"
            }
            1 => {
                self.audits = GameAudits::default();
                "AUDITS CLEARED"
            }
            _ => {
                self.adjustments = GameAdjustments::default();
                self.free_play = self.adjustments.free_play;
                self.balls_per_game = self.adjustments.balls_per_game;
                self.audio
                    .set_master_volume((self.adjustments.master_volume_pct as f32) / 100.0);
                "FACTORY DEFAULTS RESTORED"
            }
        };
        self.emit_event(GameEvent::UtilityExecuted {
            action: action.to_string(),
        });
        self.set_banner(action, 3_500);
    }

    /// Returns `(header, rows)` for rendering the Pinball 2000 Operator Administration Menu.
    pub fn admin_display_lines(&self) -> (String, Vec<(String, bool)>) {
        match self.admin_screen {
            AdminScreen::Root { cursor } => (
                "PINBALL 2000 ADMIN MENU".to_string(),
                vec![
                    ("DIAGNOSTICS".into(), cursor == 0),
                    ("AUDITS".into(), cursor == 1),
                    ("ADJUSTMENTS".into(), cursor == 2),
                    ("UTILITIES".into(), cursor == 3),
                    ("DEBUG LAUNCHER".into(), cursor == 4),
                ],
            ),
            AdminScreen::DiagnosticsMenu { cursor } => (
                "DIAGNOSTICS".to_string(),
                vec![
                    ("SWITCH TEST".into(), cursor == 0),
                    ("COIL / SOLENOID TEST".into(), cursor == 1),
                    ("LAMP TEST".into(), cursor == 2),
                    ("SOUND / DCS TEST".into(), cursor == 3),
                ],
            ),
            AdminScreen::SwitchTest => {
                let sw_line = match self.last_switch_hit {
                    Some(id) => format!("LAST SW {:02}: {}", id, switch_name(id)),
                    None => "ACTIVATE ANY PLAYFIELD SWITCH".to_string(),
                };
                (
                    "DIAGNOSTICS: SWITCH TEST".to_string(),
                    vec![
                        (sw_line, true),
                        (
                            format!("COIN DOOR: {}", if self.coin_door_closed { "CLOSED" } else { "OPEN" }),
                            false,
                        ),
                    ],
                )
            }
            AdminScreen::CoilTest { selected_drive } => (
                "DIAGNOSTICS: COIL TEST".to_string(),
                vec![
                    (format!("COIL {:02}: {}", selected_drive, drive_name(selected_drive)), true),
                    (
                        format!("HV INTERLOCK: {}", if self.coin_door_closed { "READY" } else { "DOOR OPEN" }),
                        false,
                    ),
                ],
            ),
            AdminScreen::LampTest { selected_lamp } => (
                "DIAGNOSTICS: LAMP TEST".to_string(),
                vec![
                    (format!("LAMP #{:03} (MATRIX {})", selected_lamp, if selected_lamp < 64 { "A" } else { "B" }), true),
                ],
            ),
            AdminScreen::SoundTest { selected_idx } => {
                let (sid, label) = ADMIN_SOUND_PRESETS[(selected_idx as usize) % ADMIN_SOUND_PRESETS.len()];
                (
                    "DIAGNOSTICS: DCS SOUND TEST".to_string(),
                    vec![(format!("CMD 0x{:04X}: {}", sid, label), true)],
                )
            }
            AdminScreen::Audits { cursor } => {
                let rows = vec![
                    (format!("GAMES PLAYED : {}", self.audits.total_games_played), cursor == 0),
                    (format!("BALLS PLAYED : {}", self.audits.total_balls_played), cursor == 1),
                    (format!("CREDITS      : {}", self.audits.total_credits), cursor == 2),
                    (format!("SCENES START : {}", self.audits.scenes_started), cursor == 3),
                    (format!("SCENES WON   : {}", self.audits.scenes_completed), cursor == 4),
                    (format!("MULTIBALLS   : {}", self.audits.multiballs_started), cursor == 5),
                    (format!("MAUL DUELS   : {}", self.audits.maul_duels_won), cursor == 6),
                    (format!("HIGH SCORE   : {}", self.audits.highest_score), cursor == 7),
                ];
                let start = (cursor as usize / 4) * 4;
                ("SYSTEM AUDITS".to_string(), rows[start..(start + 4).min(rows.len())].to_vec())
            }
            AdminScreen::Adjustments { cursor } => {
                let rows = vec![
                    (format!("FREE PLAY    : {}", if self.adjustments.free_play { "YES" } else { "NO" }), cursor == 0),
                    (format!("BALLS/GAME   : {}", self.adjustments.balls_per_game), cursor == 1),
                    (format!("BALL SAVE    : {}S", self.adjustments.ball_save_secs), cursor == 2),
                    (format!("TILT WARNS   : {}", self.adjustments.tilt_warnings_max), cursor == 3),
                    (format!("VOLUME       : {}%", self.adjustments.master_volume_pct), cursor == 4),
                    (format!("DIFFICULTY   : {}", self.adjustments.difficulty.label()), cursor == 5),
                ];
                let start = (cursor as usize / 4) * 4;
                ("GAME ADJUSTMENTS".to_string(), rows[start..(start + 4).min(rows.len())].to_vec())
            }
            AdminScreen::Utilities { cursor } => (
                "SYSTEM UTILITIES".to_string(),
                vec![
                    ("RESET HIGH SCORES".into(), cursor == 0),
                    ("CLEAR AUDITS".into(), cursor == 1),
                    ("FACTORY DEFAULTS".into(), cursor == 2),
                ],
            ),
            AdminScreen::DebugLauncher { cursor } => {
                let start = (cursor as usize / 4) * 4;
                let end = (start + 4).min(DEBUG_LAUNCH_PRESETS.len());
                let rows = (start..end)
                    .map(|idx| (DEBUG_LAUNCH_PRESETS[idx].1.to_string(), idx == cursor as usize))
                    .collect();
                ("DEBUG LAUNCHER (SCENES/GAMES)".to_string(), rows)
            }
        }
    }

    /// Reproduces `Game::m_credit_button_pressed()` (`docs/26-lpt-board.md`).
    fn try_start_or_add_player(&mut self) {
        if !self.free_play && self.credits == 0 {
            self.set_banner("INSERT COINS", 2_500);
            return;
        }

        if self.state == MachineState::Attract {
            if !self.free_play {
                self.credits -= 1;
            }
            self.players = vec![PlayerState::default()];
            self.current_player = 0;
            self.current_ball = 1;
            self.score = 0;
            self.audits.total_games_played = self.audits.total_games_played.saturating_add(1);
            self.state = MachineState::InGame;
            self.audio.stop_all();
            self.emit_sound(sound::MAIN_THEME_LOOP, true);
            self.emit_event(GameEvent::GameStarted { players: 1 });
            self.start_ball();
        } else if self.state == MachineState::InGame
            && self.current_ball == 1
            && self.players.len() < 4
        {
            if !self.free_play {
                self.credits -= 1;
            }
            self.players.push(PlayerState::default());
            let count = self.players.len();
            self.emit_event(GameEvent::PlayerAdded { player_count: count });
            self.set_banner(format!("PLAYER {count} ADDED"), 2_500);
        }
    }

    fn start_ball(&mut self) {
        self.tilt_warnings = 0;
        self.active_scene = None;
        self.multiball = ActiveMultiball::None;
        self.skill_shot_ready = true;
        self.ball_save_ms = (self.adjustments.ball_save_secs as u32) * 1000;
        self.idle_ball_search_ms = 0;
        self.audits.total_balls_played = self.audits.total_balls_played.saturating_add(1);

        // Reset drop targets up & lower shield unless ready for scene
        self.hw.fire_coil(drive::LEFT_DROP_UP, 64);
        self.hw.fire_coil(drive::RIGHT_DROP_UP, 64);
        self.left_drop_down = false;
        self.right_drop_down = false;

        if self.game_title == GameTitle::Swe1 {
            let shield_zero = self.players[self.current_player].shield_health == 0;
            self.set_shield_raised(shield_zero);
        } else {
            self.rfm.active_wave = None;
            self.rfm.wave_hits_left = 0;
            self.rfm.multiball = RfmMultiball::None;
        }

        // Eject 1 ball to shooter lane
        self.pending_eject_balls = 1;
        self.eject_cooldown_ms = 0;
        self.active_anim_hint = "anim_battle_droid_intro_badge".to_string();
        self.emit_event(GameEvent::BallStarted {
            player: self.current_player + 1,
            ball: self.current_ball,
        });
        self.set_banner(
            format!(
                "PLAYER {} - BALL {} - SHOOT THE GALAXY!",
                self.current_player + 1,
                self.current_ball
            ),
            3_500,
        );
    }

    fn set_shield_raised(&mut self, raised: bool) {
        if raised && !self.shield_raised {
            self.hw.fire_coil(drive::SHIELD_POWER, 64);
            self.hw.set_drive_hold(drive::SHIELD_HOLD, true);
        } else if !raised && self.shield_raised {
            self.hw.set_drive_hold(drive::SHIELD_HOLD, false);
        }
        self.shield_raised = raised;
    }

    fn cut_all_holds(&mut self) {
        for d in [
            drive::LEFT_FLIPPER_HOLD,
            drive::RIGHT_FLIPPER_HOLD,
            drive::SHIELD_HOLD,
            drive::MAGNET,
            drive::LIGHTSABER_NEON,
        ] {
            self.hw.set_drive_hold(d, false);
        }
    }

    fn handle_ingame_switch_closed(&mut self, id: u8) {
        self.idle_ball_search_ms = 0;

        match id {
            sw::START_BUTTON => {
                self.try_start_or_add_player();
            }
            sw::PLUMB_TILT => {
                self.tilt_warnings += 1;
                if self.tilt_warnings >= self.adjustments.tilt_warnings_max {
                    self.cut_all_holds();
                    self.state = MachineState::Tilt;
                    self.emit_event(GameEvent::Tilted);
                    self.set_banner("TILT! BONUS LOST", 4_000);
                } else {
                    self.emit_event(GameEvent::TiltWarning {
                        warnings: self.tilt_warnings,
                    });
                    self.set_banner(format!("DANGER! TILT WARNING {}/2", self.tilt_warnings), 2_000);
                }
            }
            // Flipper buttons: fire high-power kick + hold coil + lane-change JEDI letters
            sw::LEFT_FLIPPER_BUTTON => {
                self.hw.fire_coil(drive::LEFT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(drive::LEFT_FLIPPER_HOLD, true);
                self.players[self.current_player].rotate_jedi_letters(false);
            }
            sw::RIGHT_FLIPPER_BUTTON => {
                self.hw.fire_coil(drive::RIGHT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(drive::RIGHT_FLIPPER_HOLD, true);
                self.players[self.current_player].rotate_jedi_letters(true);
            }
            // Cabinet Laser Action Buttons (`Lasers::fire_left` / `Lasers::fire_right`)
            sw::LEFT_ACTION_BUTTON | sw::RIGHT_ACTION_BUTTON => {
                let fl = if id == sw::LEFT_ACTION_BUTTON {
                    drive::LEFT_LASER_FL
                } else {
                    drive::RIGHT_LASER_FL
                };
                if self.players[self.current_player].laser_shots > 0 {
                    self.players[self.current_player].laser_shots -= 1;
                    self.hw.fire_coil(fl, 32);
                    self.audio.play_sound(sound::LASER_FIRE, false);
                    self.add_score(75_000);
                    if self.active_scene.is_some() {
                        self.advance_active_scene("LASER DIRECT HIT!", 250_000);
                    } else {
                        self.set_banner("LASER FIRED! +75,000", 1_500);
                    }
                }
            }
            // Slingshots & Jet Bumpers
            sw::LEFT_SLING => {
                self.hw.fire_coil(drive::LEFT_SLING, 48);
                self.hw.fire_coil(drive::LOWER_LEFT_HOTDOG_FL, 24);
                self.add_score(5_110);
            }
            sw::RIGHT_SLING => {
                self.hw.fire_coil(drive::RIGHT_SLING, 48);
                self.hw.fire_coil(drive::LOWER_RIGHT_HOTDOG_FL, 24);
                self.add_score(5_110);
            }
            sw::UPPER_JET | sw::MIDDLE_JET | sw::LOWER_JET => {
                let coil = match id {
                    sw::UPPER_JET => drive::UPPER_JET,
                    sw::MIDDLE_JET => drive::MIDDLE_JET,
                    _ => drive::LOWER_JET,
                };
                self.hw.fire_coil(coil, 40);
                self.hw.fire_coil(drive::JET_FLASHER, 24);
                self.audio.play_sound(sound::JET_BUMPER_SFX, false);
                self.players[self.current_player].jet_hits += 1;
                let mult = self.players[self.current_player].bonus_multiplier as u64;
                self.add_score(10_000 * mult);
            }
            sw::JETS_ROLLOVER => {
                if self.skill_shot_ready {
                    self.skill_shot_ready = false;
                    self.add_score(500_000);
                    self.emit_coil(drive::UPPER_HOTDOG_FL, 32);
                    self.emit_event(GameEvent::SkillShotAwarded { points: 500_000 });
                    self.set_banner("SKILL SHOT! +500,000", 2_500);
                } else {
                    let p = &mut self.players[self.current_player];
                    if p.bonus_multiplier < 5 {
                        p.bonus_multiplier += 1;
                    }
                    let bm = p.bonus_multiplier;
                    self.add_score(50_000);
                    self.set_banner(format!("BONUS {}X!", bm), 1_800);
                }
            }
            // Standup Banks charge Lasers
            sw::LEFT_BANK_UPPER | sw::LEFT_BANK_MIDDLE | sw::LEFT_BANK_LOWER
            | sw::RIGHT_BANK_UPPER | sw::RIGHT_BANK_MIDDLE | sw::RIGHT_BANK_LOWER => {
                self.skill_shot_ready = false;
                let p = &mut self.players[self.current_player];
                p.laser_shots = (p.laser_shots + 1).min(9);
                let ls = p.laser_shots;
                self.add_score(35_000);
                self.set_banner(format!("LASERS CHARGED: {ls}"), 1_500);
            }
            // Drop Targets & Saucers (Hangar Multiball Locks)
            sw::LEFT_DROP_TARGET => {
                self.left_drop_down = true;
                self.add_score(40_000);
                self.set_banner("LEFT SAUCER OPEN!", 1_800);
            }
            sw::RIGHT_DROP_TARGET => {
                self.right_drop_down = true;
                self.add_score(40_000);
                self.set_banner("RIGHT SAUCER OPEN!", 1_800);
            }
            sw::LEFT_SAUCER | sw::RIGHT_SAUCER => {
                let idx = if id == sw::LEFT_SAUCER { 0 } else { 1 };
                self.saucer_hold_ms[idx] = 800;
                self.on_saucer_entered();
            }
            // Deflector Shield Targets & Center Shield Popper (`Scenes::start_valid_scene`)
            sw::LEFT_SHIELD_TARGET | sw::RIGHT_SHIELD_TARGET | sw::SHIELD_HIT => {
                self.skill_shot_ready = false;
                self.audio.play_sound(sound::SHIELD_HIT_SFX, false);
                self.hw.fire_coil(drive::BACK_MIDDLE_FL, 24);
                if self.active_scene.is_some() {
                    self.advance_active_scene("SHIELD BLAST!", 200_000);
                } else if let ActiveMultiball::DarthMaulDuel { .. } = self.multiball {
                    self.damage_darth_maul(20, "SABER STRIKE!");
                } else {
                    let p = &mut self.players[self.current_player];
                    p.shield_health = p.shield_health.saturating_sub(2);
                    let rem = p.shield_health;
                    self.add_score(50_000);
                    if rem == 0 {
                        self.set_shield_raised(true);
                        self.set_banner("SHIELDS DOWN! SHOOT CENTER SCOOP!", 3_000);
                    } else {
                        self.set_banner(format!("DEFLECTOR SHIELD: {}%", (rem as u32) * 100 / 12), 1_500);
                    }
                }
            }
            sw::SHIELD_POPPER => {
                self.popper_hold_ms = 900;
                self.on_shield_popper_entered();
            }
            // Left & Right Ramps + Left Loop -> JEDI Letters, Scene Shots, Multiball Jackpots!
            sw::LEFT_RAMP_ENTER | sw::RIGHT_RAMP_ENTER => {
                self.skill_shot_ready = false;
                self.add_score(25_000);
            }
            sw::RAMP_MADE_LEFT | sw::RAMP_MADE_RIGHT | sw::LEFT_LOOP_UPPER | sw::LEFT_LOOP_ROLLOVER => {
                self.skill_shot_ready = false;
                self.on_major_shot_made(id);
            }
            // Inlanes & Outlanes & Captive Ball
            sw::LEFT_INLANE | sw::RIGHT_INLANE | sw::SNEAKY_LANE => {
                let fl = if id == sw::LEFT_INLANE {
                    drive::LEFT_INLANE_FL
                } else {
                    drive::RIGHT_INLANE_FL
                };
                self.hw.fire_coil(fl, 24);
                self.add_score(25_000);
            }
            sw::CAPTIVE_BALL => {
                self.audio.play_sound(sound::EXPLOSION, false);
                self.add_score(100_000);
                self.set_banner("CAPTIVE BALL SMASH! +100,000", 1_800);
            }
            sw::LEFT_OUTLANE | sw::RIGHT_OUTLANE => {
                self.add_score(50_000);
                if self.ball_save_ms > 0 {
                    self.set_banner("FORCE SAVE READY!", 1_800);
                }
            }
            // Ball drained into trough!
            sw::TROUGH_BALL_1 | sw::TROUGH_BALL_2 | sw::TROUGH_BALL_3 | sw::TROUGH_BALL_4 => {
                if self.balls_in_play > 0 && self.eject_cooldown_ms == 0 {
                    self.on_ball_drained();
                }
            }
            _ => {}
        }
    }

    fn on_saucer_entered(&mut self) {
        if !matches!(self.multiball, ActiveMultiball::None) {
            self.add_score(300_000);
            self.set_banner("SAUCER BONUS! +300,000", 2_000);
            return;
        }
        let p = &mut self.players[self.current_player];
        p.saucer_locks += 1;
        if p.saucer_locks >= 3 {
            p.saucer_locks = 0;
            self.multiball = ActiveMultiball::HangarMultiball {
                jackpots_lit: true,
                jackpots_collected: 0,
            };
            self.pending_eject_balls = self.pending_eject_balls.saturating_add(2);
            self.ball_save_ms = 20_000;
            self.emit_sound(sound::MULTIBALL_MUSIC_LOOP, true);
            self.active_anim_hint = "anim_hangar_droid_explosion".to_string();
            self.audits.multiballs_started = self.audits.multiballs_started.saturating_add(1);
            self.add_score(500_000);
            self.emit_event(GameEvent::MultiballStarted {
                mode: "THEED HANGAR 3-BALL MULTIBALL".to_string(),
            });
            self.set_banner("THEED HANGAR 3-BALL MULTIBALL!", 4_000);
        } else {
            let locks = self.players[self.current_player].saucer_locks;
            self.add_score(150_000);
            self.set_banner(format!("BALL {locks} LOCKED FOR MULTIBALL!"), 2_500);
        }
    }

    fn on_shield_popper_entered(&mut self) {
        if let ActiveMultiball::HangarMultiball {
            jackpots_lit,
            jackpots_collected,
        } = self.multiball
        {
            if jackpots_lit && jackpots_collected >= 2 {
                self.multiball = ActiveMultiball::HangarMultiball {
                    jackpots_lit: true,
                    jackpots_collected: 0,
                };
                self.emit_coil(drive::LIGHTSABER_NEON, 160);
                self.emit_sound(sound::EXPLOSION, false);
                self.add_score(3_000_000);
                self.emit_event(GameEvent::JackpotAwarded {
                    super_jackpot: true,
                    points: 3_000_000,
                });
                self.set_banner("SUPER JACKPOT!! +3,000,000", 3_500);
                return;
            }
        }

        if self.active_scene.is_some() {
            self.advance_active_scene("DIRECT CORE HIT!", 350_000);
            return;
        }

        // Start the next SWE1 Holoprojector Scene!
        let scene = self.players[self.current_player].choose_next_scene();
        self.players[self.current_player].shield_health = 8;
        self.set_shield_raised(false);
        self.active_scene = Some(ActiveSceneState {
            scene,
            hits_remaining: scene.required_hits(),
            timer_ms: scene.duration_ms(),
        });
        self.active_anim_hint = scene.anim_hint().to_string();
        self.audits.scenes_started = self.audits.scenes_started.saturating_add(1);
        self.emit_sound(scene.music_id(), true);
        self.add_score(250_000);
        self.emit_event(GameEvent::SceneStarted {
            scene,
            title: scene.title().to_string(),
        });
        self.set_banner(format!("SCENE START: {}", scene.title()), 3_500);
    }

    fn on_major_shot_made(&mut self, sw_id: u8) {
        self.hw.fire_coil(drive::UPPER_HOTDOG_FL, 24);

        // 1. Check active Multiball Jackpot / Darth Maul Duel!
        match self.multiball {
            ActiveMultiball::HangarMultiball {
                jackpots_lit,
                jackpots_collected,
            } => {
                if jackpots_lit {
                    let next_jp = jackpots_collected.saturating_add(1);
                    self.multiball = ActiveMultiball::HangarMultiball {
                        jackpots_lit: true,
                        jackpots_collected: next_jp,
                    };
                    let award = 1_000_000 * (next_jp as u64);
                    self.add_score(award);
                    self.emit_sound(sound::EXPLOSION, false);
                    self.emit_event(GameEvent::JackpotAwarded {
                        super_jackpot: false,
                        points: award,
                    });
                    self.set_banner(format!("JACKPOT! +{award}"), 2_500);
                    return;
                }
            }
            ActiveMultiball::DarthMaulDuel { combo_window_ms, .. } => {
                let dmg = if combo_window_ms > 0 { 35 } else { 25 };
                let msg = if combo_window_ms > 0 {
                    "LIGHTSABER COMBO!"
                } else {
                    "LIGHTSABER SLASH!"
                };
                self.damage_darth_maul(dmg, msg);
                return;
            }
            ActiveMultiball::None => {}
        }

        // 2. Check active Scene shot!
        if self.active_scene.is_some() {
            let shot_label = match sw_id {
                sw::RAMP_MADE_LEFT => "LEFT RAMP HIT!",
                sw::RAMP_MADE_RIGHT => "RIGHT RAMP HIT!",
                _ => "FORCE LOOP HIT!",
            };
            self.advance_active_scene(shot_label, 250_000);
            return;
        }

        // 3. Award J-E-D-I Letter -> Completing J-E-D-I starts Darth Maul Lightsaber Duel!
        let completed_jedi = self.players[self.current_player].award_jedi_letter();
        self.add_score(125_000);
        if completed_jedi {
            self.players[self.current_player].jedi_letters = 0;
            self.hw.set_drive_hold(drive::LIGHTSABER_NEON, true);
            self.multiball = ActiveMultiball::DarthMaulDuel {
                maul_health: 100,
                combo_window_ms: 5_000,
            };
            self.pending_eject_balls = self.pending_eject_balls.saturating_add(1);
            self.ball_save_ms = 20_000;
            self.emit_sound(sound::DUEL_OF_THE_FATES_LOOP, true);
            self.active_anim_hint = "anim_jedi_battle_maul_chop_single".to_string();
            self.emit_event(GameEvent::MultiballStarted {
                mode: "DARTH MAUL SABER DUEL".to_string(),
            });
            self.set_banner("J-E-D-I COMPLETE! DARTH MAUL SABER DUEL!", 4_000);
        } else {
            let mask = self.players[self.current_player].jedi_letters;
            let spelled: String = ['J', 'E', 'D', 'I']
                .iter()
                .enumerate()
                .map(|(i, &c)| if (mask & (1 << i)) != 0 { c } else { '-' })
                .collect();
            self.set_banner(format!("JEDI SPIRIT: [{spelled}]"), 2_000);
        }
    }

    fn advance_active_scene(&mut self, label: &str, pts: u64) {
        self.add_score(pts);
        let mut finished_scene = None;
        let mut progressed_info = None;
        if let Some(ref mut sc) = self.active_scene {
            sc.hits_remaining = sc.hits_remaining.saturating_sub(1);
            if sc.hits_remaining == 0 {
                finished_scene = Some(sc.scene);
            } else {
                progressed_info = Some((sc.scene, sc.hits_remaining));
                let left = sc.hits_remaining;
                self.set_banner(format!("{label} ({left} HITS LEFT)"), 2_000);
            }
        }
        if let Some((scene, hits_remaining)) = progressed_info {
            self.emit_event(GameEvent::SceneProgressed {
                scene,
                hits_remaining,
                points: pts,
            });
        }
        if let Some(scene) = finished_scene {
            self.active_scene = None;
            let idx = Swe1Scene::ALL.iter().position(|&s| s == scene).unwrap_or(0);
            self.players[self.current_player].scenes_completed |= 1 << idx;
            let done_count = self.players[self.current_player].scenes_completed.count_ones();
            if done_count == 4 {
                self.players[self.current_player].extra_balls += 1;
            }
            self.add_score(1_500_000);
            self.audits.scenes_completed = self.audits.scenes_completed.saturating_add(1);
            self.emit_coil(drive::LIGHTSABER_NEON, 160);
            self.emit_sound(sound::MAIN_THEME_LOOP, true);
            self.emit_event(GameEvent::SceneCompleted {
                scene,
                points: 1_500_000,
            });
            if scene == Swe1Scene::Podrace {
                self.active_anim_hint = "anim_podrace_closeup".to_string();
            }
            self.set_banner(format!("{} VICTORY! +1,500,000", scene.title()), 3_500);
        }
    }

    fn damage_darth_maul(&mut self, dmg: u8, label: &str) {
        self.audio.play_sound(sound::SABER_CLASH, false);
        self.hw.fire_coil(drive::LEFT_LASER_FL, 32);
        self.hw.fire_coil(drive::RIGHT_LASER_FL, 32);
        self.add_score(400_000);

        if let ActiveMultiball::DarthMaulDuel { maul_health, .. } = self.multiball {
            let next_hp = maul_health.saturating_sub(dmg);
            if next_hp == 0 {
                self.multiball = ActiveMultiball::None;
                self.hw.set_drive_hold(drive::LIGHTSABER_NEON, false);
                let new_rank = self.players[self.current_player].jedi_rank.advance();
                self.players[self.current_player].jedi_rank = new_rank;
                self.audits.maul_duels_won = self.audits.maul_duels_won.saturating_add(1);
                self.add_score(5_000_000);
                self.emit_sound(sound::MAIN_THEME_LOOP, true);
                self.emit_event(GameEvent::JediRankPromoted { rank: new_rank });
                self.active_anim_hint = "anim_jedi_battle_maul_death_1".to_string();
                self.set_banner(
                    format!("MAUL DEFEATED! PROMOTED TO {}! +5,000,000", new_rank.label()),
                    4_500,
                );
            } else {
                self.multiball = ActiveMultiball::DarthMaulDuel {
                    maul_health: next_hp,
                    combo_window_ms: 4_000,
                };
                self.set_banner(format!("{label} MAUL HP: {next_hp}%"), 2_000);
            }
        }
    }

    fn on_ball_drained(&mut self) {
        if self.ball_save_ms > 0 && self.state == MachineState::InGame {
            // Ball Saver (`BallSaver`): re-serve immediately without losing ball!
            self.pending_eject_balls = self.pending_eject_balls.saturating_add(1);
            self.balls_in_play = self.balls_in_play.saturating_sub(1);
            self.emit_event(GameEvent::BallSaved {
                player: self.current_player + 1,
            });
            self.set_banner("SHOOT AGAIN! FORCE BALL SAVE!", 2_500);
            return;
        }

        self.balls_in_play = self.balls_in_play.saturating_sub(1);
        if self.balls_in_play >= 1 {
            // Dropped from Multiball down to single-ball play
            if self.balls_in_play == 1 && !matches!(self.multiball, ActiveMultiball::None) {
                self.multiball = ActiveMultiball::None;
                self.hw.set_drive_hold(drive::LIGHTSABER_NEON, false);
                self.audio.play_sound(sound::MAIN_THEME_LOOP, true);
                self.set_banner("MULTIBALL ENDED", 2_000);
            }
            return;
        }

        // End of ball! Award bonus if not tilted
        self.cut_all_holds();
        self.active_scene = None;
        self.multiball = ActiveMultiball::None;

        if self.state != MachineState::Tilt {
            let bonus = {
                let p = &self.players[self.current_player];
                p.bonus_points.saturating_mul(p.bonus_multiplier as u64)
            };
            self.add_score(bonus);
            self.emit_event(GameEvent::BallEnded {
                player: self.current_player + 1,
                ball: self.current_ball,
                bonus,
            });
            self.players[self.current_player].bonus_points = 10_000;
            self.players[self.current_player].bonus_multiplier = 1;
        }
        self.state = MachineState::InGame;

        // Extra ball check
        if self.players[self.current_player].extra_balls > 0 {
            self.players[self.current_player].extra_balls -= 1;
            self.start_ball();
            self.set_banner("SHOOT AGAIN - EXTRA BALL!", 3_000);
            return;
        }

        // Advance to next player or next ball
        if self.current_player + 1 < self.players.len() {
            self.current_player += 1;
            self.score = self.players[self.current_player].score;
            self.start_ball();
        } else if self.current_ball < self.balls_per_game {
            self.current_player = 0;
            self.current_ball += 1;
            self.score = self.players[0].score;
            self.start_ball();
        } else {
            self.finish_game();
        }
    }

    fn handle_rfm_ingame_switch_closed(&mut self, id: u8) {
        self.idle_ball_search_ms = 0;

        match id {
            rfm_sw::START_BUTTON => self.try_start_or_add_player(),
            rfm_sw::LAUNCH_BUTTON => {
                self.emit_coil(rfm_drive::AUTOPLUNGER, 48);
                self.emit_sound(rfm_sound::LASER_SFX, false);
                self.set_banner("AUTOPLUNGER LAUNCH!", 1_500);
            }
            sw::PLUMB_TILT => {
                self.tilt_warnings += 1;
                if self.tilt_warnings >= self.adjustments.tilt_warnings_max {
                    self.cut_all_holds();
                    self.state = MachineState::Tilt;
                    self.emit_event(GameEvent::Tilted);
                    self.set_banner("TILT! BONUS LOST", 4_000);
                } else {
                    self.emit_event(GameEvent::TiltWarning {
                        warnings: self.tilt_warnings,
                    });
                    self.set_banner(format!("DANGER! TILT WARNING {}/2", self.tilt_warnings), 2_000);
                }
            }
            sw::LEFT_FLIPPER_BUTTON => {
                self.emit_coil(rfm_drive::LEFT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(rfm_drive::LEFT_FLIPPER_HOLD, true);
            }
            sw::RIGHT_FLIPPER_BUTTON => {
                self.emit_coil(rfm_drive::RIGHT_FLIPPER_POWER, 40);
                self.hw.set_drive_hold(rfm_drive::RIGHT_FLIPPER_HOLD, true);
            }
            sw::LEFT_ACTION_BUTTON | sw::RIGHT_ACTION_BUTTON => {
                self.emit_coil(rfm_drive::CENTER_ARROW_FL, 32);
                self.emit_sound(rfm_sound::LASER_SFX, false);
                self.add_score(100_000);
                if self.rfm.active_wave.is_some() {
                    self.advance_rfm_wave("RAYGUN HIT!", 250_000);
                } else {
                    self.set_banner("MARTIAN RAYGUN FIRED!", 1_500);
                }
            }
            rfm_sw::LEFT_SLING => {
                self.emit_coil(rfm_drive::LEFT_SLING, 48);
                self.add_score(5_110);
            }
            rfm_sw::RIGHT_SLING => {
                self.emit_coil(rfm_drive::RIGHT_SLING, 48);
                self.add_score(5_110);
            }
            rfm_sw::LEFT_JET | rfm_sw::RIGHT_JET | rfm_sw::BOTTOM_JET => {
                let c = match id {
                    rfm_sw::LEFT_JET => rfm_drive::LEFT_JET,
                    rfm_sw::RIGHT_JET => rfm_drive::RIGHT_JET,
                    _ => rfm_drive::BOTTOM_JET,
                };
                self.emit_coil(c, 40);
                self.add_score(25_000);
            }
            rfm_sw::LEFT_TOP_LANE | rfm_sw::RIGHT_TOP_LANE => {
                if self.skill_shot_ready {
                    self.skill_shot_ready = false;
                    self.add_score(500_000);
                    self.emit_event(GameEvent::SkillShotAwarded { points: 500_000 });
                    self.set_banner("RFM SKILL SHOT! +500,000", 2_500);
                } else {
                    self.add_score(50_000);
                }
            }
            // 7 Martian Targets -> pulse Left/Right Martian toys & start Martian Attack Multiball!
            rfm_sw::MARTIAN_TARGET_1
            | rfm_sw::MARTIAN_TARGET_2
            | rfm_sw::MARTIAN_TARGET_3
            | rfm_sw::MARTIAN_TARGET_4
            | rfm_sw::MARTIAN_TARGET_5
            | rfm_sw::MARTIAN_TARGET_6
            | rfm_sw::MARTIAN_TARGET_7 => {
                self.skill_shot_ready = false;
                let bit = match id {
                    rfm_sw::MARTIAN_TARGET_1 => 0,
                    rfm_sw::MARTIAN_TARGET_2 => 1,
                    rfm_sw::MARTIAN_TARGET_3 => 2,
                    rfm_sw::MARTIAN_TARGET_4 => 3,
                    rfm_sw::MARTIAN_TARGET_5 => 4,
                    rfm_sw::MARTIAN_TARGET_6 => 5,
                    _ => 6,
                };
                self.rfm.martian_targets_mask |= 1 << bit;
                if bit <= 2 {
                    self.emit_coil(rfm_drive::LEFT_MARTIAN, 48);
                    self.emit_coil(rfm_drive::LEFT_MARTIAN_FL, 32);
                } else {
                    self.emit_coil(rfm_drive::RIGHT_MARTIAN, 48);
                    self.emit_coil(rfm_drive::RIGHT_MARTIAN_FL, 32);
                }
                self.add_score(75_000);
                if self.rfm.martian_targets_mask == 0x7F {
                    self.rfm.martian_targets_mask = 0;
                    self.rfm.multiball = RfmMultiball::MartianAttack;
                    self.pending_eject_balls = self.pending_eject_balls.saturating_add(1);
                    self.ball_save_ms = 20_000;
                    self.audits.multiballs_started = self.audits.multiballs_started.saturating_add(1);
                    self.emit_sound(rfm_sound::MULTIBALL_MUSIC, true);
                    self.emit_event(GameEvent::MultiballStarted {
                        mode: "MARTIAN ATTACK MULTIBALL".to_string(),
                    });
                    self.set_banner("MARTIAN ATTACK 2-BALL MULTIBALL!", 4_000);
                } else {
                    let count = self.rfm.martian_targets_mask.count_ones();
                    self.set_banner(format!("MARTIAN HIT! ({count}/7)"), 1_800);
                }
            }
            // Saucer Lockups -> Capture Multiball (3-ball)
            rfm_sw::RIGHT_LOCKUP_1 => {
                self.emit_coil(rfm_drive::RIGHT_LOCKUP, 64);
                self.rfm.saucer_locks += 1;
                if self.rfm.saucer_locks >= 3 {
                    self.rfm.saucer_locks = 0;
                    self.rfm.multiball = RfmMultiball::CaptureMultiball { jackpots: 0 };
                    self.pending_eject_balls = self.pending_eject_balls.saturating_add(2);
                    self.ball_save_ms = 20_000;
                    self.audits.multiballs_started = self.audits.multiballs_started.saturating_add(1);
                    self.emit_sound(rfm_sound::MULTIBALL_MUSIC, true);
                    self.emit_event(GameEvent::MultiballStarted {
                        mode: "CAPTURE MULTIBALL".to_string(),
                    });
                    self.set_banner("CAPTURE 3-BALL MULTIBALL!", 4_000);
                } else {
                    let l = self.rfm.saucer_locks;
                    self.add_score(150_000);
                    self.set_banner(format!("SAUCER LOCK {l}/3!"), 2_000);
                }
            }
            // Center Deflector Panel / Right Popper -> Start or Advance Attack Wave!
            rfm_sw::CENTER_DEFLECTOR_PANEL | rfm_sw::RIGHT_POPPER | rfm_sw::CENTER_TARGET_4..=rfm_sw::CENTER_TARGET_1 => {
                self.skill_shot_ready = false;
                self.emit_coil(rfm_drive::ATTACK_MARS_FL, 32);
                if id == rfm_sw::RIGHT_POPPER {
                    self.emit_coil(rfm_drive::RIGHT_POPPER, 64);
                }
                if self.rfm.active_wave.is_some() {
                    self.advance_rfm_wave("SAUCER HIT!", 250_000);
                } else {
                    let idx = (self.rfm.waves_completed.trailing_ones() as usize).min(8);
                    let wave = RfmWave::ALL[idx];
                    self.rfm.active_wave = Some(wave);
                    self.rfm.wave_hits_left = wave.required_hits();
                    self.audits.scenes_started = self.audits.scenes_started.saturating_add(1);
                    self.emit_sound(rfm_sound::WAVE_MUSIC, true);
                    self.add_score(250_000);
                    self.set_banner(format!("WAVE START: {}", wave.title()), 3_500);
                }
            }
            // Ramps & Loops -> Multiball Jackpots or Wave Progress
            rfm_sw::LEFT_RAMP_EXIT | rfm_sw::RIGHT_RAMP_EXIT | rfm_sw::LEFT_LOOP_HIGH | rfm_sw::RIGHT_LOOP_HIGH => {
                self.skill_shot_ready = false;
                if let RfmMultiball::CaptureMultiball { jackpots } = self.rfm.multiball {
                    let next_jp = jackpots + 1;
                    self.rfm.multiball = RfmMultiball::CaptureMultiball { jackpots: next_jp };
                    let award = 1_000_000 * (next_jp as u64);
                    self.add_score(award);
                    self.emit_sound(rfm_sound::EXPLOSION_SFX, false);
                    self.emit_event(GameEvent::JackpotAwarded {
                        super_jackpot: false,
                        points: award,
                    });
                    self.set_banner(format!("SAUCER JACKPOT! +{award}"), 2_500);
                } else if self.rfm.active_wave.is_some() {
                    self.advance_rfm_wave("RAMP SHOT!", 300_000);
                } else {
                    self.add_score(125_000);
                    self.set_banner("RAMP SHOT! +125,000", 1_500);
                }
            }
            rfm_sw::TROUGH_BALL_1 | rfm_sw::TROUGH_BALL_2 | rfm_sw::TROUGH_BALL_3 | rfm_sw::TROUGH_BALL_4 => {
                if self.balls_in_play > 0 && self.eject_cooldown_ms == 0 {
                    self.on_ball_drained();
                }
            }
            _ => {}
        }
    }

    fn advance_rfm_wave(&mut self, label: &str, pts: u64) {
        self.add_score(pts);
        if let Some(wave) = self.rfm.active_wave {
            self.rfm.wave_hits_left = self.rfm.wave_hits_left.saturating_sub(1);
            if self.rfm.wave_hits_left == 0 {
                self.rfm.active_wave = None;
                let idx = RfmWave::ALL.iter().position(|&w| w == wave).unwrap_or(0);
                self.rfm.waves_completed |= 1 << idx;
                self.audits.scenes_completed = self.audits.scenes_completed.saturating_add(1);
                self.add_score(1_500_000);
                self.emit_sound(rfm_sound::MAIN_THEME, true);
                self.set_banner(format!("{} DESTROYED! +1,500,000", wave.title()), 3_500);
            } else {
                let left = self.rfm.wave_hits_left;
                self.set_banner(format!("{label} ({left} HITS LEFT)"), 2_000);
            }
        }
    }

    fn update_rfm_lamps(&mut self) {
        self.hw.set_lamp(rfm_lamp::START_BUTTON, LampState::On);
        self.hw.set_lamp(rfm_lamp::SAUCER, LampState::On);
        self.hw.set_lamp(rfm_lamp::LEFT_MARTIAN_EYE, LampState::On);
        self.hw.set_lamp(rfm_lamp::RIGHT_MARTIAN_EYE, LampState::On);

        for (i, &lid) in rfm_lamp::SAUCER_RIM.iter().enumerate() {
            if (self.rfm.waves_completed & (1 << i)) != 0 {
                self.hw.set_lamp(lid, LampState::On);
            }
        }
        for (i, &lid) in rfm_lamp::MARTIAN_TARGETS.iter().enumerate() {
            if (self.rfm.martian_targets_mask & (1 << i)) != 0 {
                self.hw.set_lamp(lid, LampState::On);
            }
        }
        if self.ball_save_ms > 0 {
            self.hw.set_lamp(rfm_lamp::SHOOT_AGAIN, LampState::On);
        }
        if !matches!(self.rfm.multiball, RfmMultiball::None) {
            self.hw.set_lamp(rfm_lamp::MARTIAN_ATTACK, LampState::Blinking { period_ms: 200 });
        }
    }

    fn apply_mod_switch_rules(&mut self, id: u8) -> bool {
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

    fn handle_homebrew_builtin_switch(&mut self, id: u8) {
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

    fn finish_game(&mut self) {
        self.attract.last_game_played = true;
        self.audio.stop_all();
        self.emit_sound(sound::GAME_OVER_MUSIC, false);
        self.emit_event(GameEvent::GameOver {
            final_score: self.score,
        });

        let threshold = if self.high_scores.len() < 4 {
            0
        } else {
            self.high_scores.last().map(|h| h.score).unwrap_or(0)
        };
        let qualifiers: Vec<(usize, u64, JediRank)> = self
            .players
            .iter()
            .enumerate()
            .filter(|(_, p)| p.score > threshold)
            .map(|(idx, p)| (idx + 1, p.score, p.jedi_rank))
            .collect();

        if !qualifiers.is_empty() {
            let first_p = qualifiers[0].0;
            self.hstd_session = HstdEntrySession {
                queue: qualifiers,
                initials: ['A', 'A', 'A'],
                cursor: 0,
                char_idx: 0,
            };
            self.state = MachineState::HighScoreEntry;
            self.set_banner(format!("PLAYER {first_p} HIGH SCORE! ENTER INITIALS"), 0);
        } else {
            self.state = MachineState::Attract;
            self.attract.index = 13; // DeffAttractModeScores
            self.attract.timer_ms = 0;
            self.set_banner(format!("GAME OVER - FINAL SCORE: {}", self.score), 5_000);
        }
    }

    fn handle_hstd_entry_switch(&mut self, id: u8) {
        if self.hstd_session.queue.is_empty() {
            self.state = MachineState::Attract;
            return;
        }
        let n = HSTD_ALPHABET.len();
        match id {
            sw::LEFT_FLIPPER_BUTTON | sw::DIAG_DOWN => {
                self.hstd_session.char_idx = (self.hstd_session.char_idx + n - 1) % n;
                let ch = HSTD_ALPHABET[self.hstd_session.char_idx] as char;
                self.hstd_session.initials[self.hstd_session.cursor] = ch;
                self.emit_sound(sound::LASER_SFX_OR_CLICK(), false);
            }
            sw::RIGHT_FLIPPER_BUTTON | sw::DIAG_UP => {
                self.hstd_session.char_idx = (self.hstd_session.char_idx + 1) % n;
                let ch = HSTD_ALPHABET[self.hstd_session.char_idx] as char;
                self.hstd_session.initials[self.hstd_session.cursor] = ch;
                self.emit_sound(sound::LASER_SFX_OR_CLICK(), false);
            }
            sw::START_BUTTON | sw::LEFT_ACTION_BUTTON | sw::RIGHT_ACTION_BUTTON => {
                self.emit_sound(sound::COIN_CREDIT, false);
                if self.hstd_session.cursor < 2 {
                    self.hstd_session.cursor += 1;
                    let cur_ch = self.hstd_session.initials[self.hstd_session.cursor] as u8;
                    self.hstd_session.char_idx =
                        HSTD_ALPHABET.iter().position(|&c| c == cur_ch).unwrap_or(0);
                } else {
                    let (_, score, rank) = self.hstd_session.queue.remove(0);
                    self.high_scores.push(HighScoreEntry {
                        initials: self.hstd_session.initials,
                        score,
                        rank,
                    });
                    self.high_scores.sort_by(|a, b| b.score.cmp(&a.score));
                    self.high_scores.truncate(4);

                    if let Some(&(next_p, _, _)) = self.hstd_session.queue.first() {
                        self.hstd_session.initials = ['A', 'A', 'A'];
                        self.hstd_session.cursor = 0;
                        self.hstd_session.char_idx = 0;
                        self.set_banner(format!("PLAYER {next_p} HIGH SCORE! ENTER INITIALS"), 0);
                    } else {
                        self.state = MachineState::Attract;
                        self.attract.index = 13; // DeffAttractModeScores
                        self.attract.timer_ms = 0;
                        self.set_banner(format!("HIGH SCORE SAVED!"), 4_000);
                    }
                }
            }
            _ => {}
        }
    }

    fn update_timers(&mut self, dt_ms: u32) {
        if self.state == MachineState::Boot && self.elapsed_ms >= 1_200 {
            self.state = MachineState::Attract;
            self.attract.index = 0; // Start at DeffAttractModeKahunaLogo
            self.attract.timer_ms = 0;
            if !self.attract.attract_music_started {
                self.attract.attract_music_started = true;
                self.emit_sound(sound::ATTRACT_MUSIC, false);
            }
            self.set_banner(format!("{} - PRESS START", self.game_title.title()), 0);
        }

        if self.state == MachineState::Attract {
            self.attract.tick(dt_ms);
            let cur = self.attract.current();
            if self.game_title == GameTitle::Swe1 {
                self.active_anim_hint = cur.anim_hint().unwrap_or("").to_string();
                self.status_banner = match cur {
                    AttractDeff::KahunaLogo | AttractDeff::P2kLogo | AttractDeff::HasbroAd => String::new(),
                    AttractDeff::SlideShow => "STAR WARS EPISODE I".to_string(),
                    AttractDeff::Hstd => "HIGHEST SCORES".to_string(),
                    AttractDeff::JediTableYouth => "JEDI YOUTH".to_string(),
                    AttractDeff::JediTableKnight => "JEDI KNIGHT".to_string(),
                    AttractDeff::JediTableMaster => "JEDI MASTER".to_string(),
                    AttractDeff::JediTableSpirit => "JEDI SPIRIT".to_string(),
                    AttractDeff::RampComboChampion => "RAMP COMBO CHAMPION".to_string(),
                    AttractDeff::ScoreAwards => "REPLAY & EXTRA BALL".to_string(),
                    AttractDeff::EmailAddress => "PINBALL 2000".to_string(),
                    AttractDeff::WebPage => "WWW.PINBALL.COM".to_string(),
                    AttractDeff::Scores => "LAST GAME SCORES".to_string(),
                    AttractDeff::GameCredits1 => "WILLIAMS ELECTRONICS".to_string(),
                    AttractDeff::GameCredits2 => "LUCASFILM LTD.".to_string(),
                    AttractDeff::GameCredits3 => "DESIGN & SOFTWARE".to_string(),
                    AttractDeff::GameCredits4 => "PINBALL 2000 TEAM".to_string(),
                    AttractDeff::CustomMessage => String::new(),
                    AttractDeff::HtpScene => "SHOOT SHIELD -> START SCENE".to_string(),
                };
            }
        }

        if self.banner_timer_ms > 0 {
            self.banner_timer_ms = self.banner_timer_ms.saturating_sub(dt_ms);
        }
        if self.ball_save_ms > 0 {
            self.ball_save_ms = self.ball_save_ms.saturating_sub(dt_ms);
        }
        if self.eject_cooldown_ms > 0 {
            self.eject_cooldown_ms = self.eject_cooldown_ms.saturating_sub(dt_ms);
        }

        // Serve queued balls from Trough (`Drive 8`)
        if self.pending_eject_balls > 0 && self.eject_cooldown_ms == 0 && self.coin_door_closed {
            self.pending_eject_balls -= 1;
            self.balls_in_play = self.balls_in_play.saturating_add(1);
            self.eject_cooldown_ms = 650;
            self.hw.fire_coil(drive::TROUGH_EJECT, 64);
        }

        // Kick balls out of Left/Right Saucers & Shield Popper after hold timer
        for (i, coil) in [drive::LEFT_SAUCER, drive::RIGHT_SAUCER].iter().enumerate() {
            if self.saucer_hold_ms[i] > 0 {
                self.saucer_hold_ms[i] = self.saucer_hold_ms[i].saturating_sub(dt_ms);
                if self.saucer_hold_ms[i] == 0 {
                    self.hw.fire_coil(*coil, 64);
                }
            }
        }
        if self.popper_hold_ms > 0 {
            self.popper_hold_ms = self.popper_hold_ms.saturating_sub(dt_ms);
            if self.popper_hold_ms == 0 {
                self.hw.fire_coil(drive::SHIELD_POPPER, 64);
            }
        }

        // Scene countdown timer
        if let Some(ref mut sc) = self.active_scene {
            sc.timer_ms = sc.timer_ms.saturating_sub(dt_ms);
            if sc.timer_ms == 0 {
                let title = sc.scene.title();
                self.active_scene = None;
                self.audio.play_sound(sound::MAIN_THEME_LOOP, true);
                self.set_banner(format!("{title} TIME EXPIRED"), 2_500);
            }
        }

        // Darth Maul combo window decay
        if let ActiveMultiball::DarthMaulDuel {
            maul_health,
            combo_window_ms,
        } = self.multiball
        {
            if combo_window_ms > 0 {
                self.multiball = ActiveMultiball::DarthMaulDuel {
                    maul_health,
                    combo_window_ms: combo_window_ms.saturating_sub(dt_ms),
                };
            }
        }

        // Missing-ball search (`BallSearch`) after 15 seconds of playfield silence
        if self.state == MachineState::InGame && self.balls_in_play > 0 && self.coin_door_closed {
            self.idle_ball_search_ms = self.idle_ball_search_ms.saturating_add(dt_ms);
            if self.idle_ball_search_ms >= 15_000 {
                self.idle_ball_search_ms = 0;
                self.hw.fire_coil(drive::SHIELD_POPPER, 48);
                self.hw.fire_coil(drive::LEFT_SAUCER, 48);
                self.hw.fire_coil(drive::RIGHT_SAUCER, 48);
            }
        }
    }

    fn update_lamps(&mut self) {
        for id in 0..128u8 {
            self.hw.set_lamp(id, LampState::Off);
        }

        match self.state {
            MachineState::Boot => {
                self.hw.set_lamp(lamp::COIN_DOOR, LampState::On);
            }
            MachineState::Attract => {
                // Attract lamp sweep (`LeffAttractMode`)
                if self.game_title == GameTitle::Rfm {
                    let rim_idx = ((self.elapsed_ms / 120) as usize) % 9;
                    self.hw.set_lamp(rfm_lamp::SAUCER_RIM[rim_idx], LampState::On);
                    self.hw.set_lamp(rfm_lamp::SAUCER, LampState::On);
                } else {
                    let phase = ((self.elapsed_ms / 120) as usize) % 12;
                    self.hw
                        .set_lamp(lamp::SHIELD_LAMPS[phase], LampState::On);
                    let jedi_idx = ((self.elapsed_ms / 250) as usize) % 4;
                    self.hw
                        .set_lamp(lamp::JEDI_LETTERS[jedi_idx], LampState::On);
                }
                if self.free_play || self.credits > 0 {
                    self.hw.set_lamp(
                        lamp::START_BUTTON,
                        LampState::Blinking { period_ms: 400 },
                    );
                }
            }
            MachineState::Diagnostics => {
                if let DiagPage::LampTest { selected_lamp } = self.diag_page {
                    self.hw
                        .set_lamp(selected_lamp, LampState::Blinking { period_ms: 250 });
                }
            }
            MachineState::Tilt => {
                self.hw.set_lamp(lamp::COIN_DOOR, LampState::On);
            }
            MachineState::HighScoreEntry => {
                self.hw.set_lamp(lamp::START_BUTTON, LampState::Blinking { period_ms: 250 });
                self.hw.set_lamp(lamp::LEFT_FLIPPER, LampState::On);
                self.hw.set_lamp(lamp::RIGHT_FLIPPER, LampState::On);
            }
            MachineState::InGame => {
                for &lid in &self.mod_lit_lamps {
                    self.hw.set_lamp(lid, LampState::On);
                }
                if let Some(ref rt) = self.script_runtime {
                    for &lid in &rt.lit_lamps {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }
                if self.game_title == GameTitle::Homebrew {
                    self.hw.set_lamp(lamp::START_BUTTON, LampState::On);
                    if self.ball_save_ms > 0 {
                        self.hw.set_lamp(lamp::SHOOT_AGAIN, LampState::On);
                    }
                    return;
                }
                if self.game_title == GameTitle::Rfm {
                    self.update_rfm_lamps();
                    return;
                }
                let p = &self.players[self.current_player];
                self.hw.set_lamp(lamp::START_BUTTON, LampState::On);
                self.hw.set_lamp(lamp::LEFT_FLIPPER, LampState::On);
                self.hw.set_lamp(lamp::RIGHT_FLIPPER, LampState::On);

                // J-E-D-I letters
                for (i, &lid) in lamp::JEDI_LETTERS.iter().enumerate() {
                    if (p.jedi_letters & (1 << i)) != 0 {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }

                // 12-lamp Deflector Shield meter
                for (i, &lid) in lamp::SHIELD_LAMPS.iter().enumerate() {
                    if (i as u8) < p.shield_health {
                        self.hw.set_lamp(lid, LampState::On);
                    } else if p.shield_health == 0 {
                        self.hw
                            .set_lamp(lid, LampState::Blinking { period_ms: 200 });
                    }
                }

                // Jedi Rank lamps
                if p.jedi_rank >= JediRank::Youth {
                    self.hw.set_lamp(lamp::JEDI_YOUTH, LampState::On);
                }
                if p.jedi_rank >= JediRank::Knight {
                    self.hw.set_lamp(lamp::JEDI_KNIGHT, LampState::On);
                }
                if p.jedi_rank >= JediRank::Master {
                    self.hw.set_lamp(lamp::JEDI_MASTER, LampState::On);
                }
                if p.jedi_rank >= JediRank::Spirit {
                    self.hw
                        .set_lamp(lamp::JEDI_SPIRIT, LampState::Blinking { period_ms: 200 });
                }

                // Lasers ready lamps
                if p.laser_shots > 0 {
                    self.hw.set_lamp(
                        lamp::FIRE_LASERS_LEFT,
                        LampState::Blinking { period_ms: 300 },
                    );
                    self.hw.set_lamp(
                        lamp::FIRE_LASERS_RIGHT,
                        LampState::Blinking { period_ms: 300 },
                    );
                }

                // Bonus multiplier lamps
                if p.bonus_multiplier >= 2 {
                    self.hw.set_lamp(lamp::BONUS_X2, LampState::On);
                }
                if p.bonus_multiplier >= 3 {
                    self.hw.set_lamp(lamp::BONUS_X3, LampState::On);
                }
                if p.bonus_multiplier >= 4 {
                    self.hw.set_lamp(lamp::BONUS_X4, LampState::On);
                }
                if p.bonus_multiplier >= 5 {
                    self.hw.set_lamp(lamp::BONUS_X5, LampState::On);
                }

                // Shoot Again / Ball Saver
                if self.ball_save_ms > 4_000 {
                    self.hw.set_lamp(lamp::SHOOT_AGAIN, LampState::On);
                } else if self.ball_save_ms > 0 {
                    self.hw.set_lamp(
                        lamp::SHOOT_AGAIN,
                        LampState::Blinking { period_ms: 180 },
                    );
                }

                if p.extra_balls > 0 {
                    self.hw.set_lamp(lamp::EXTRA_BALL, LampState::On);
                }

                // Completed Scenes light Naboo Starfighter body lamps!
                for (i, &lid) in lamp::NABOO_SHIP_BODY.iter().enumerate() {
                    if (p.scenes_completed & (1 << i)) != 0 {
                        self.hw.set_lamp(lid, LampState::On);
                    }
                }
            }
        }
    }

    fn render_frame(&mut self) {
        if self.state == MachineState::Boot {
            self.active_anim_hint.clear();
            self.video.draw_text("WILLIAMS PINBALL 2000", 320, 62, 0x7FE0, 2);
            self.video.draw_text(self.game_title.title(), 320, 100, 0x7FFF, 2);
            self.video.draw_text("XINA / PRISM ROM CHECK ... OK", 320, 150, 0x03E0, 1);
            self.video.present();
            return;
        }
        if self.state == MachineState::Diagnostics {
            self.active_anim_hint.clear();
            let (title, rows) = self.admin_display_lines();
            self.video.draw_text(&title, 320, 22, 0x7FE0, 2);
            for (i, (line, selected)) in rows.iter().enumerate() {
                let prefix = if *selected { "> " } else { "  " };
                let color = if *selected { 0x03FF } else { 0x7FFF };
                self.video.draw_text(
                    &format!("{prefix}{line}"),
                    320,
                    66 + (i as i32) * 26,
                    color,
                    2,
                );
            }
            self.video.draw_text(
                "UP/DN: SELECT   ENTER: OK   ESC: BACK",
                320,
                216,
                0x5EF7,
                1,
            );
            self.video.present();
            return;
        }
        let frame_idx = match self.attract.current() {
            AttractDeff::P2kLogo if self.state == MachineState::Attract => (self.attract.timer_ms / 66) as usize,
            AttractDeff::KahunaLogo if self.state == MachineState::Attract => (self.attract.timer_ms / 40) as usize,
            _ => (self.elapsed_ms / 32) as usize,
        };
        if !self.active_anim_hint.is_empty() {
            self.video.draw_sprite(AssetId(0), frame_idx, 210, 20, 0);
        }

        match self.state {
            MachineState::Attract => {
                match self.attract.current() {
                    AttractDeff::Hstd => {
                        self.video.draw_text("HIGHEST SCORES", 320, 26, 0x7FE0, 2);
                        for (i, hs) in self.high_scores.iter().take(4).enumerate() {
                            let ini: String = hs.initials.iter().collect();
                            self.video.draw_text(
                                &format!("#{} {}  {:>10}", i + 1, ini, hs.score),
                                320,
                                68 + (i as i32) * 26,
                                0x7FFF,
                                2,
                            );
                        }
                    }
                    AttractDeff::JediTableYouth
                    | AttractDeff::JediTableKnight
                    | AttractDeff::JediTableMaster
                    | AttractDeff::JediTableSpirit => {
                        self.video.draw_text(&self.status_banner, 320, 32, 0x03FF, 2);
                        self.video.draw_text("QGJ   OBI   ANI   PAD", 320, 96, 0x7FFF, 2);
                        self.video.draw_text("DEFEAT DARTH MAUL TO ADVANCE", 320, 140, 0x7FE0, 1);
                    }
                    AttractDeff::ScoreAwards => {
                        self.video.draw_text("REPLAY AT 15,000,000", 320, 74, 0x7FE0, 2);
                        self.video.draw_text("EXTRA BALL AT 4 SCENES", 320, 114, 0x7FFF, 2);
                    }
                    AttractDeff::GameCredits1
                    | AttractDeff::GameCredits2
                    | AttractDeff::GameCredits3
                    | AttractDeff::GameCredits4 => {
                        self.video.draw_text(&self.status_banner, 320, 74, 0x7FE0, 2);
                        self.video.draw_text("STAR WARS EPISODE I PINBALL", 320, 116, 0x7FFF, 2);
                    }
                    _ => {
                        if !self.status_banner.is_empty() {
                            self.video.draw_text(&self.status_banner, 320, 190, 0x7FE0, 2);
                        }
                    }
                }
                // DeffAttractModePriceBackground: blink FREE PLAY / CREDITS every 500 ms
                if (self.elapsed_ms / 500) % 2 == 0 {
                    let price_txt = if self.free_play {
                        "FREE PLAY - PRESS START".to_string()
                    } else if self.credits > 0 {
                        format!("CREDITS {} - PRESS START", self.credits)
                    } else {
                        "INSERT COIN".to_string()
                    };
                    self.video.draw_text(&price_txt, 320, 222, 0x03FF, 1);
                }
            }
            MachineState::InGame | MachineState::Tilt => {
                if !self.status_banner.is_empty() {
                    self.video.draw_text(&self.status_banner, 320, 190, 0x7FE0, 2);
                }
                self.video.draw_text(
                    &format!("P{}  BALL {}   {:>10}", self.current_player + 1, self.current_ball, self.score),
                    320,
                    218,
                    0x7FFF,
                    2,
                );
            }
            MachineState::HighScoreEntry => {
                let (p_num, p_score, p_rank) = self
                    .hstd_session
                    .queue
                    .first()
                    .copied()
                    .unwrap_or((1, self.score, JediRank::Padawan));
                self.video.draw_text("GREAT SCORE - ENTER INITIALS", 320, 28, 0x7FE0, 2);
                self.video.draw_text(
                    &format!("PLAYER {p_num}  -  {p_score} ({})", p_rank.label()),
                    320,
                    64,
                    0x7FFF,
                    2,
                );
                let mut slots = String::new();
                for (i, &ch) in self.hstd_session.initials.iter().enumerate() {
                    if i > 0 {
                        slots.push_str("   ");
                    }
                    if i == self.hstd_session.cursor {
                        slots.push_str(&format!("[{ch}]"));
                    } else {
                        slots.push_str(&format!(" {ch} "));
                    }
                }
                self.video.draw_text(&slots, 320, 118, 0x03FF, 3);
                self.video.draw_text(
                    "FLIPPERS: SELECT LETTER   START: CONFIRM",
                    320,
                    196,
                    0x5EF7,
                    1,
                );
            }
            _ => {}
        }
        self.video.present();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pin2k_hal::{HeadlessVideo, PdbDriver, VirtualPdbPort};

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
}
