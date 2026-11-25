//! Data-driven Pinball 2000 gamecode script runtime (`game_script.json`).

mod schema;

pub use schema::*;

use std::collections::{BTreeMap, BTreeSet};

/// Live execution state for a loaded `GameScript`.
#[derive(Debug, Clone)]
pub struct ScriptRuntime {
    pub script: GameScript,
    pub vars: BTreeMap<String, i64>,
    /// Isolated per-player variables for 1–4 player games.
    pub player_vars: [BTreeMap<String, i64>; 4],
    pub active_player: usize,
    /// Active modes -> remaining duration in milliseconds (`None` = untimed).
    pub active_modes: BTreeMap<String, Option<u32>>,
    /// Active countdown timers -> remaining ticks.
    pub timers: BTreeMap<String, u32>,
    /// Lamps explicitly lit by script actions.
    pub lit_lamps: BTreeSet<u8>,
    pub attract_index: usize,
    pub attract_timer_ms: u32,
}

impl ScriptRuntime {
    pub fn new(script: GameScript) -> Self {
        let vars = script.vars.clone();
        let player_vars = [vars.clone(), vars.clone(), vars.clone(), vars.clone()];
        Self {
            script,
            vars,
            player_vars,
            active_player: 0,
            active_modes: BTreeMap::new(),
            timers: BTreeMap::new(),
            lit_lamps: BTreeSet::new(),
            attract_index: 0,
            attract_timer_ms: 0,
        }
    }

    pub fn get_var(&self, var: &str) -> i64 {
        let p = self.active_player % 4;
        self.player_vars[p]
            .get(var)
            .or_else(|| self.vars.get(var))
            .copied()
            .unwrap_or(0)
    }

    pub fn set_var(&mut self, var: &str, value: i64) {
        let p = self.active_player % 4;
        self.player_vars[p].insert(var.to_string(), value);
        if var.starts_with("global_") {
            self.vars.insert(var.to_string(), value);
        }
    }

    /// Advances active countdown timers and timed modes, returning `(expired_timers, expired_modes)`.
    pub fn tick_timers(&mut self, dt_ms: u32) -> (Vec<String>, Vec<String>) {
        let mut expired_timers = Vec::new();
        for (name, rem) in &mut self.timers {
            *rem = rem.saturating_sub(1);
            if *rem == 0 {
                expired_timers.push(name.clone());
            }
        }
        for name in &expired_timers {
            self.timers.remove(name);
        }

        let mut expired_modes = Vec::new();
        for (mode, dur) in &mut self.active_modes {
            if let Some(ref mut rem_ms) = dur {
                *rem_ms = rem_ms.saturating_sub(dt_ms);
                if *rem_ms == 0 {
                    expired_modes.push(mode.clone());
                }
            }
        }
        for mode in &expired_modes {
            self.active_modes.remove(mode);
        }
        (expired_timers, expired_modes)
    }

    pub fn eval_conditions(
        &self,
        conds: &[ScriptCondition],
        ball_in_play: bool,
        not_tilted: bool,
    ) -> bool {
        for c in conds {
            let ok = match c {
                ScriptCondition::BallInPlay => ball_in_play,
                ScriptCondition::NotTilted => not_tilted,
                ScriptCondition::ModeActive { mode } => self.active_modes.contains_key(mode),
                ScriptCondition::ModeInactive { mode } => !self.active_modes.contains_key(mode),
                ScriptCondition::VarEq { var, value } => self.get_var(var) == *value,
                ScriptCondition::VarGe { var, value } => self.get_var(var) >= *value,
                ScriptCondition::VarLt { var, value } => self.get_var(var) < *value,
                ScriptCondition::BitClear { var, bit } => {
                    (self.get_var(var) & (1i64 << (*bit as u32 & 63))) == 0
                }
                ScriptCondition::BitSet { var, bit } => {
                    (self.get_var(var) & (1i64 << (*bit as u32 & 63))) != 0
                }
            };
            if !ok {
                return false;
            }
        }
        true
    }

    pub fn current_attract_slide(&self) -> Option<&AttractSlideScript> {
        if self.script.attract.is_empty() {
            return None;
        }
        Some(&self.script.attract[self.attract_index % self.script.attract.len()])
    }

    pub fn tick_attract(&mut self, dt_ms: u32, last_game_played: bool) -> Option<&AttractSlideScript> {
        let len = self.script.attract.len();
        if len == 0 {
            return None;
        }
        self.attract_timer_ms = self.attract_timer_ms.saturating_add(dt_ms);
        let dur = self.script.attract[self.attract_index % len].duration_ms.max(500);
        if self.attract_timer_ms >= dur {
            self.attract_timer_ms = 0;
            for _ in 0..len {
                self.attract_index = (self.attract_index + 1) % len;
                let slide = &self.script.attract[self.attract_index];
                if !slide.require_last_game || last_game_played {
                    break;
                }
            }
        }
        Some(&self.script.attract[self.attract_index % len])
    }

    pub fn switch_name(&self, id: u8) -> Option<&str> {
        self.script
            .hardware
            .switches
            .iter()
            .find(|s| s.id == id)
            .map(|s| s.name.as_str())
    }

    pub fn drive_name(&self, id: u8) -> Option<&str> {
        self.script
            .hardware
            .drives
            .iter()
            .find(|d| d.id == id)
            .map(|d| d.name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn script_runtime_evaluates_conditions_and_attract() {
        let mut vars = BTreeMap::new();
        vars.insert("jedi_mask".into(), 0);
        let script = GameScript {
            schema_version: 1,
            meta: GameMetaScript {
                title: "Test".into(),
                game_number: 50069,
                version: "2.00".into(),
                xina_version: String::new(),
                build_timestamp: String::new(),
                opto_nc_mask_lo: 0,
                opto_nc_mask_hi: 0,
                balls_per_game: 3,
                ball_save_secs: 15,
                trough_switches: vec![],
                shooter_lane_switch: None,
                start_button_switch: None,
                trough_eject_drive: None,
                auto_plunge_drive: None,
            },
            hardware: HardwareTablesScript {
                switches: vec![],
                drives: vec![],
                lamps: vec![],
            },
            attract: vec![
                AttractSlideScript {
                    name: "logo".into(),
                    duration_ms: 1000,
                    banner: "LOGO".into(),
                    anim_hint: None,
                    require_last_game: false,
                },
                AttractSlideScript {
                    name: "scores".into(),
                    duration_ms: 1000,
                    banner: "SCORES".into(),
                    anim_hint: None,
                    require_last_game: true,
                },
            ],
            vars,
            modes: vec![],
            rules: vec![],
        };
        let mut rt = ScriptRuntime::new(script);
        assert!(rt.eval_conditions(&[ScriptCondition::BitClear { var: "jedi_mask".into(), bit: 0 }], true, true));
        rt.set_var("jedi_mask", 1);
        assert!(rt.eval_conditions(&[ScriptCondition::BitSet { var: "jedi_mask".into(), bit: 0 }], true, true));
        // Player 2 remains isolated at 0
        rt.active_player = 1;
        assert_eq!(rt.get_var("jedi_mask"), 0);
        // Timers & modes expire cleanly
        rt.timers.insert("t1".into(), 2);
        rt.active_modes.insert("m1".into(), Some(20));
        let (et, em) = rt.tick_timers(16);
        assert!(et.is_empty() && em.is_empty());
        let (et2, em2) = rt.tick_timers(16);
        assert_eq!(et2, vec!["t1"]);
        assert_eq!(em2, vec!["m1"]);
        let s = rt.tick_attract(1000, false).unwrap();
        assert_eq!(s.name, "logo");
    }
}
