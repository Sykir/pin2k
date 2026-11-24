//! Structured Pinball 2000 / *Star Wars Episode I* game & hardware event bus.

use serde::{Deserialize, Serialize};

use crate::swe1_rules::{JediRank, Swe1Scene};

/// Semantic game, switch, coil, and audio event emitted by `Pin2kMachine`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    CreditAdded { credits: u8 },
    GameStarted { players: usize },
    PlayerAdded { player_count: usize },
    BallStarted { player: usize, ball: u8 },
    BallSaved { player: usize },
    SwitchChanged { sw_id: u8, name: String, closed: bool },
    CoilFired { drive_id: u8, name: String, pulse_ms: u8 },
    SoundTriggered { sound_id: u16, looping: bool },
    SkillShotAwarded { points: u64 },
    SceneStarted { scene: Swe1Scene, title: String },
    SceneProgressed { scene: Swe1Scene, hits_remaining: u8, points: u64 },
    SceneCompleted { scene: Swe1Scene, points: u64 },
    MultiballStarted { mode: String },
    JackpotAwarded { super_jackpot: bool, points: u64 },
    JediRankPromoted { rank: JediRank },
    TiltWarning { warnings: u8 },
    Tilted,
    SlamTilted,
    BallEnded { player: usize, ball: u8, bonus: u64 },
    GameOver { final_score: u64 },
    AdminMenuEntered,
    AdminMenuExited,
    AdjustmentChanged { name: String, value: String },
    UtilityExecuted { action: String },
}

/// Timestamped event envelope returned by `Pin2kMachine::drain_events()`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimestampedEvent {
    pub elapsed_ms: u64,
    pub event: GameEvent,
}
