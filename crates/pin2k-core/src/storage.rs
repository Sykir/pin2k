//! Local JSON database (`pin2k_db.json`) for persisting high scores, audits,
//! operator adjustments, and recent game history across sessions.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::admin::{GameAdjustments, GameAudits};
use crate::swe1_rules::{default_high_scores, HighScoreEntry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHistoryEntry {
    pub timestamp_ms: u64,
    pub game_number: u32,
    pub players: usize,
    pub scores: Vec<u64>,
    pub best_score: u64,
    pub scenes_completed: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDbRecord {
    pub game_number: u32,
    pub high_scores: Vec<HighScoreEntry>,
    pub audits: GameAudits,
    pub adjustments: GameAdjustments,
    #[serde(default)]
    pub recent_games: Vec<GameHistoryEntry>,
}

impl GameDbRecord {
    pub fn new_default(game_number: u32) -> Self {
        Self {
            game_number,
            high_scores: default_high_scores(),
            audits: GameAudits::default(),
            adjustments: GameAdjustments::default(),
            recent_games: Vec::new(),
        }
    }
}

/// Persistent local database storing per-game scores, audits, adjustments, and history.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalDb {
    pub schema_version: u32,
    pub games: BTreeMap<u32, GameDbRecord>,
}

impl LocalDb {
    pub fn load_or_create(path: &Path) -> Result<Self, std::io::Error> {
        if !path.exists() {
            return Ok(Self {
                schema_version: 1,
                games: BTreeMap::new(),
            });
        }
        let raw = fs::read_to_string(path)?;
        serde_json::from_str(&raw)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn save_to_file(&self, path: &Path) -> Result<(), std::io::Error> {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, json)?;
        fs::rename(tmp, path)?;
        Ok(())
    }

    pub fn get_or_insert_game(&mut self, game_number: u32) -> &mut GameDbRecord {
        self.games
            .entry(game_number)
            .or_insert_with(|| GameDbRecord::new_default(game_number))
    }

    pub fn record_game(&mut self, entry: GameHistoryEntry) {
        let rec = self.get_or_insert_game(entry.game_number);
        rec.recent_games.push(entry);
        if rec.recent_games.len() > 50 {
            let excess = rec.recent_games.len() - 50;
            rec.recent_games.drain(0..excess);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_db_roundtrip_persists_scores_and_audits() {
        let dir = std::env::temp_dir().join("pin2k_db_test");
        let _ = fs::remove_dir_all(&dir);
        let db_path = dir.join("pin2k_db.json");

        let mut db = LocalDb::load_or_create(&db_path).unwrap();
        let rec = db.get_or_insert_game(50069);
        rec.audits.total_games_played = 7;
        rec.audits.highest_score = 12_345_670;
        db.record_game(GameHistoryEntry {
            timestamp_ms: 1000,
            game_number: 50069,
            players: 1,
            scores: vec![12_345_670],
            best_score: 12_345_670,
            scenes_completed: 2,
        });
        db.save_to_file(&db_path).unwrap();

        let loaded = LocalDb::load_or_create(&db_path).unwrap();
        let loaded_rec = loaded.games.get(&50069).unwrap();
        assert_eq!(loaded_rec.audits.total_games_played, 7);
        assert_eq!(loaded_rec.audits.highest_score, 12_345_670);
        assert_eq!(loaded_rec.recent_games.len(), 1);

        let _ = fs::remove_dir_all(&dir);
    }
}
