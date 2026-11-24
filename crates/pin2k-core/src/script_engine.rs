//! Data-driven Pinball 2000 gamecode script runtime (`game_script.json`).
//!
//! Executes scripts extracted from Pinball 2000 ROMs by `pin2k-rom-tool extract-gamecode`
//! (or custom user-authored scripts) without requiring hardcoded game rules.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Top-level portable Pinball 2000 game script (`game_script.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameScript {
    pub schema_version: u32,
    pub meta: GameMetaScript,
    pub hardware: HardwareTablesScript,
    pub attract: Vec<AttractSlideScript>,
    pub vars: BTreeMap<String, i64>,
    pub modes: Vec<ModeScript>,
    pub rules: Vec<RuleScript>,
}

impl GameScript {
    pub fn load_from_file(path: &Path) -> Result<Self, std::io::Error> {
        let raw = fs::read_to_string(path)?;
        serde_json::from_str(&raw)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMetaScript {
    pub game_number: u32,
    pub title: String,
    pub version: String,
    #[serde(default)]
    pub xina_version: String,
    #[serde(default)]
    pub build_timestamp: String,
    pub opto_nc_mask_lo: u64,
    pub opto_nc_mask_hi: u32,
    pub balls_per_game: u8,
    pub ball_save_secs: u8,
    #[serde(default)]
    pub trough_switches: Vec<u8>,
    pub shooter_lane_switch: Option<u8>,
    pub start_button_switch: Option<u8>,
    pub trough_eject_drive: Option<u8>,
    pub auto_plunge_drive: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareTablesScript {
    pub switches: Vec<SwitchDefScript>,
    pub drives: Vec<DriveDefScript>,
    pub lamps: Vec<LampDefScript>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiftedCall {
    pub call_addr: u32,
    pub target_addr: u32,
    pub target_symbol: String,
    pub pushed_args: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchDefScript {
    pub id: u8,
    pub name: String,
    #[serde(default)]
    pub name_fr: String,
    #[serde(default)]
    pub sw_type: u32,
    #[serde(default)]
    pub arg: u32,
    #[serde(default)]
    pub debounce_ms: u32,
    #[serde(default)]
    pub handler_addr: u32,
    #[serde(default)]
    pub handler_symbol: String,
    #[serde(default)]
    pub opto_nc: bool,
    #[serde(default)]
    pub lifted_calls: Vec<LiftedCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveDefScript {
    pub id: u8,
    pub name: String,
    #[serde(default)]
    pub name_fr: String,
    #[serde(default)]
    pub default_pulse_ms: u32,
    #[serde(default)]
    pub flags: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LampDefScript {
    pub id: u8,
    pub name: String,
    #[serde(default)]
    pub name_fr: String,
    #[serde(default)]
    pub flags: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttractSlideScript {
    pub name: String,
    pub duration_ms: u32,
    pub anim_hint: Option<String>,
    pub banner: String,
    #[serde(default)]
    pub require_last_game: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeScript {
    pub id: String,
    pub name: String,
    pub duration_secs: Option<u32>,
    pub anim_hint: Option<String>,
    pub sound_id: Option<u16>,
    pub music_id: Option<u16>,
    #[serde(default)]
    pub on_enter: Vec<ScriptAction>,
    #[serde(default)]
    pub on_exit: Vec<ScriptAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleScript {
    pub id: String,
    pub trigger: ScriptTrigger,
    #[serde(default)]
    pub conditions: Vec<ScriptCondition>,
    pub actions: Vec<ScriptAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ScriptTrigger {
    OnBoot,
    OnBallStart,
    OnBallEnd,
    OnSwitch { sw_id: u8 },
    OnTick,
    OnModeEnter { mode: String },
    OnModeExit { mode: String },
    OnTimer { timer: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum ScriptCondition {
    BallInPlay,
    NotTilted,
    ModeActive { mode: String },
    ModeInactive { mode: String },
    VarEq { var: String, value: i64 },
    VarGe { var: String, value: i64 },
    VarLt { var: String, value: i64 },
    BitClear { var: String, bit: u8 },
    BitSet { var: String, bit: u8 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ScriptAction {
    AddScore { points: u64 },
    SetVar { var: String, value: i64 },
    IncVar { var: String, delta: i64 },
    SetBit { var: String, bit: u8 },
    ClearVar { var: String },
    PulseDrive { drive_id: u8, ms: u8 },
    SetLamp { lamp_id: u8, on: bool },
    PlaySound { sound_id: u16 },
    PlayMusic { sound_id: u16 },
    PlayAnim { anim: String },
    ShowBanner { text: String },
    StartMode { mode: String },
    EndMode { mode: String },
    StartTimer { name: String, ticks: u32 },
    StartMultiball { balls: u8 },
    AwardExtraBall,
    AdvanceScene,
    CollectBonus,
    EjectBall,
}

/// Live execution state for a loaded `GameScript`.
#[derive(Debug, Clone)]
pub struct ScriptRuntime {
    pub script: GameScript,
    pub vars: BTreeMap<String, i64>,
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
        Self {
            script,
            vars,
            active_modes: BTreeMap::new(),
            timers: BTreeMap::new(),
            lit_lamps: BTreeSet::new(),
            attract_index: 0,
            attract_timer_ms: 0,
        }
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
                ScriptCondition::VarEq { var, value } => {
                    self.vars.get(var).copied().unwrap_or(0) == *value
                }
                ScriptCondition::VarGe { var, value } => {
                    self.vars.get(var).copied().unwrap_or(0) >= *value
                }
                ScriptCondition::VarLt { var, value } => {
                    self.vars.get(var).copied().unwrap_or(0) < *value
                }
                ScriptCondition::BitClear { var, bit } => {
                    let v = self.vars.get(var).copied().unwrap_or(0);
                    (v & (1i64 << (*bit as u32 & 63))) == 0
                }
                ScriptCondition::BitSet { var, bit } => {
                    let v = self.vars.get(var).copied().unwrap_or(0);
                    (v & (1i64 << (*bit as u32 & 63))) != 0
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
        let script = GameScript {
            schema_version: 1,
            meta: GameMetaScript {
                game_number: 50069,
                title: "Test SWE1".into(),
                version: "2.00".into(),
                xina_version: "1.22".into(),
                build_timestamp: String::new(),
                opto_nc_mask_lo: 0,
                opto_nc_mask_hi: 0,
                balls_per_game: 3,
                ball_save_secs: 12,
                trough_switches: vec![25, 26, 27, 28],
                shooter_lane_switch: Some(7),
                start_button_switch: Some(2),
                trough_eject_drive: Some(8),
                auto_plunge_drive: Some(9),
            },
            hardware: HardwareTablesScript {
                switches: vec![],
                drives: vec![],
                lamps: vec![],
            },
            attract: vec![
                AttractSlideScript {
                    name: "Slide1".into(),
                    duration_ms: 1000,
                    anim_hint: Some("anim_p2k_logo_animated".into()),
                    banner: "BANNER 1".into(),
                    require_last_game: false,
                },
                AttractSlideScript {
                    name: "Slide2".into(),
                    duration_ms: 1000,
                    anim_hint: None,
                    banner: "BANNER 2".into(),
                    require_last_game: false,
                },
            ],
            vars: [("hits".into(), 2)].into_iter().collect(),
            modes: vec![],
            rules: vec![],
        };

        let mut rt = ScriptRuntime::new(script);
        assert!(rt.eval_conditions(
            &[
                ScriptCondition::BallInPlay,
                ScriptCondition::VarGe { var: "hits".into(), value: 2 },
            ],
            true,
            true,
        ));
        assert!(!rt.eval_conditions(
            &[ScriptCondition::VarGe { var: "hits".into(), value: 3 }],
            true,
            true,
        ));
        assert_eq!(rt.current_attract_slide().unwrap().name, "Slide1");
        rt.tick_attract(1000, false);
        assert_eq!(rt.current_attract_slide().unwrap().name, "Slide2");
    }
}
