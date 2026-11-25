//! Core Pinball 2000 & *Star Wars Episode I* (`#50069`) game engine.

pub mod admin;
pub mod attract;
pub mod events;
pub mod machine;
pub mod mods;
pub mod rfm_rules;
pub mod rfm_tables;
pub mod script_engine;
pub mod storage;
pub mod swe1_audio;
pub mod swe1_rules;
pub mod swe1_tables;
pub mod switch_matrix;

use thiserror::Error;

pub use admin::{AdminScreen, Difficulty, GameAdjustments, GameAudits};
pub use attract::{AttractController, AttractDeff};
pub use events::{GameEvent, TimestampedEvent};
pub use machine::{DiagPage, MachineState, Pin2kMachine};
pub use mods::{HomebrewSwitchRule, ModManifest};
pub use script_engine::{GameScript, ScriptAction, ScriptRuntime, ScriptTrigger};
pub use storage::{GameDbRecord, GameHistoryEntry, LocalDb};
pub use rfm_rules::{GameTitle, RfmMultiball, RfmState, RfmWave};
pub use swe1_rules::{
    ActiveMultiball, ActiveSceneState, HighScoreEntry, JediRank, PlayerState, Swe1Scene,
};
pub use switch_matrix::{SwitchDebouncer, SwitchEvent, SwitchId};

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("invalid machine state transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: MachineState,
        to: MachineState,
    },
}
