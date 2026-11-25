use std::collections::BTreeMap;
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

