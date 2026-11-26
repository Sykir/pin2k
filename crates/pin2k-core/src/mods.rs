//! Modding & Homebrew game support (`mod.json`) for Pinball 2000 (`pin2k-core`).
//!
//! Allows:
//! - **Mods** on top of *Star Wars Episode I* or *Revenge From Mars* (custom graphics,
//!   custom `.ogg` audio replacements, scoring multipliers, banner & rule overrides).
//! - **Standalone Homebrew** Pinball 2000 games running on real cabinet hardware (`PdbDriver`)
//!   or desktop simulator via declarative `switch_rules` or custom Rust code.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Declarative switch-triggered rule for Homebrew games or Mod overrides.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HomebrewSwitchRule {
    pub sw_id: u8,
    #[serde(default)]
    pub points: u64,
    #[serde(default)]
    pub coil_id: Option<u8>,
    #[serde(default = "default_pulse_ms")]
    pub pulse_ms: u8,
    #[serde(default)]
    pub sound_id: Option<u16>,
    #[serde(default)]
    pub lamp_id: Option<u8>,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default)]
    pub anim_hint: Option<String>,
}

const fn default_pulse_ms() -> u8 {
    48
}

/// Manifest (`mod.json`) describing a Pinball 2000 Mod or Homebrew game package.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModManifest {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: String,
    /// `"swe1"`, `"rfm"`, or `"homebrew"` (default: inherit active base game).
    #[serde(default)]
    pub base_game: Option<String>,
    #[serde(default)]
    pub attract_banner: Option<String>,
    #[serde(default)]
    pub balls_per_game: Option<u8>,
    #[serde(default)]
    pub ball_save_secs: Option<u8>,
    #[serde(default)]
    pub score_multiplier: Option<u64>,
    /// Maps hex DCS ID (e.g. `"0x0001"`) to a custom `.ogg` file inside `<mod>/audio/`.
    #[serde(default)]
    pub sound_overrides: HashMap<String, String>,
    /// Maps animation hint (e.g. `"anim_battle_droid_intro_battle"`) to a custom folder inside `<mod>/gfx/`.
    #[serde(default)]
    pub anim_overrides: HashMap<String, String>,
    /// Relative path to a custom Rhai script inside the mod directory (e.g. `"main.rhai"`).
    #[serde(default)]
    pub script: Option<String>,
    /// Loaded Rhai source code (populated by `load_from_dir` or set directly in tests).
    #[serde(default, skip_serializing)]
    pub script_source: Option<String>,
    /// Custom or override switch rules evaluated on switch closure.
    #[serde(default)]
    pub switch_rules: Vec<HomebrewSwitchRule>,
}

impl ModManifest {
    pub fn load_from_dir(dir: &Path) -> Result<Self, std::io::Error> {
        let p = dir.join("mod.json");
        let raw = fs::read_to_string(p)?;
        let mut manifest: Self = serde_json::from_str(&raw)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        if let Some(ref script_rel) = manifest.script {
            let script_path = dir.join(script_rel);
            manifest.script_source = Some(fs::read_to_string(script_path)?);
        }
        Ok(manifest)
    }
}
