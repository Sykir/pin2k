//! Core Pinball 2000 & *Star Wars Episode I* (`#50069`) game engine.

pub mod admin;
pub mod events;
pub mod machine;
pub mod swe1_rules;
pub mod swe1_tables;
pub mod switch_matrix;

use thiserror::Error;

pub use admin::{AdminScreen, Difficulty, GameAdjustments, GameAudits};
pub use events::{GameEvent, TimestampedEvent};
pub use machine::{DiagPage, MachineState, Pin2kMachine};
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
