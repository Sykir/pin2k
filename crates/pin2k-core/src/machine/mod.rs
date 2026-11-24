//! Deterministic fixed-timestep *Star Wars Episode I* (`#50069`) Pinball 2000 machine runtime.

mod admin_nav;
mod render;
mod rfm_playfield;
mod script_exec;
mod swe1_playfield;
#[cfg(test)]
mod tests;

use pin2k_hal::{
    AudioBackend, HeadlessAudio, PinballHardware, VideoBackend,
};
use serde::{Deserialize, Serialize};

use crate::admin::{
    AdminScreen, GameAdjustments, GameAudits,
};
use crate::attract::{AttractController, AttractDeff};
use crate::events::{GameEvent, TimestampedEvent};
use crate::mods::HomebrewSwitchRule;
use crate::rfm_rules::{GameTitle, RfmMultiball, RfmState};
use crate::rfm_tables::{rfm_drive_name, rfm_switch_name};
use crate::script_engine::{ScriptRuntime, ScriptTrigger};
use crate::swe1_rules::{
    default_high_scores, ActiveMultiball, ActiveSceneState, HighScoreEntry, JediRank, PlayerState,
};
use crate::swe1_tables::{drive, sound, sw, drive_name, switch_name};
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
pub(super) struct HstdEntrySession {
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

    pub(super) debouncer: SwitchDebouncer,
    pub(super) event_log: Vec<TimestampedEvent>,
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
    pub(super) fn game_switch_name(&self, id: u8) -> &'static str {
        match self.game_title {
            GameTitle::Swe1 | GameTitle::Homebrew => switch_name(id),
            GameTitle::Rfm => rfm_switch_name(id),
        }
    }

    pub(super) fn game_drive_name(&self, id: u8) -> &'static str {
        match self.game_title {
            GameTitle::Swe1 | GameTitle::Homebrew => drive_name(id),
            GameTitle::Rfm => rfm_drive_name(id),
        }
    }

    pub(super) fn emit_event(&mut self, event: GameEvent) {
        self.event_log.push(TimestampedEvent {
            elapsed_ms: self.elapsed_ms,
            event,
        });
    }

    /// Drains all structured `GameEvent`s emitted since the last call.
    pub fn drain_events(&mut self) -> Vec<TimestampedEvent> {
        std::mem::take(&mut self.event_log)
    }

    pub(super) fn emit_coil(&mut self, drive_id: u8, pulse_ms: u8) {
        self.hw.fire_coil(drive_id, pulse_ms);
        self.emit_event(GameEvent::CoilFired {
            drive_id,
            name: self.game_drive_name(drive_id).to_string(),
            pulse_ms,
        });
    }

    pub(super) fn emit_sound(&mut self, id: pin2k_hal::SoundId, looping: bool) {
        self.audio.play_sound(id, looping);
        self.emit_event(GameEvent::SoundTriggered {
            sound_id: id.0,
            looping,
        });
    }

    pub(super) fn set_banner(&mut self, msg: impl Into<String>, duration_ms: u32) {
        self.status_banner = msg.into();
        self.banner_timer_ms = duration_ms;
    }

    pub(super) fn add_score(&mut self, pts: u64) {
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

    pub(super) fn handle_switch_closed(&mut self, id: u8) {
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

    pub(super) fn handle_switch_opened(&mut self, id: u8) {
        if id == sw::LEFT_FLIPPER_BUTTON {
            self.hw.set_drive_hold(drive::LEFT_FLIPPER_HOLD, false);
        } else if id == sw::RIGHT_FLIPPER_BUTTON {
            self.hw.set_drive_hold(drive::RIGHT_FLIPPER_HOLD, false);
        }
    }

    pub(super) fn try_start_or_add_player(&mut self) {
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

    pub(super) fn start_ball(&mut self) {
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

    pub(super) fn on_ball_drained(&mut self) {
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

    pub(super) fn finish_game(&mut self) {
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

    pub(super) fn handle_hstd_entry_switch(&mut self, id: u8) {
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

    pub(super) fn update_timers(&mut self, dt_ms: u32) {
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

        // Scene intro -> eject ball -> gameplay timer
        let mut switch_to_gameplay_anim = None;
        if let Some(ref mut sc) = self.active_scene {
            if sc.intro_remaining_ms > 0 {
                sc.intro_remaining_ms = sc.intro_remaining_ms.saturating_sub(dt_ms);
                if sc.intro_remaining_ms == 0 {
                    switch_to_gameplay_anim = Some((
                        sc.scene.gameplay_anim_hint().to_string(),
                        format!("SCENE ACTIVE: {}", sc.scene.title()),
                    ));
                }
            } else {
                sc.timer_ms = sc.timer_ms.saturating_sub(dt_ms);
                if sc.timer_ms == 0 {
                    let title = sc.scene.title();
                    self.active_scene = None;
                    self.audio.play_sound(sound::MAIN_THEME_LOOP, true);
                    self.set_banner(format!("{title} TIME EXPIRED"), 2_500);
                }
            }
        }
        if let Some((anim, banner)) = switch_to_gameplay_anim {
            self.active_anim_hint = anim;
            self.set_banner(banner, 3_000);
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

}
