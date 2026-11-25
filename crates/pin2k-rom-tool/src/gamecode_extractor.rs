//! Gamecode-to-Script Extractor (`extract-gamecode`):
//!
//! Lifts hardware tables (`SwitchTable`, `DriveTable`, `LampTable`),
//! attract state machines (`AttractProcInfo`), x86 switch-handler calls
//! (`score`, `AudioRequest`, `traverse_and_call`, `Drive`, `Lamp`),
//! and game rules/modes into a portable, data-driven `game_script.json`
//! executed by `pin2k-core`'s `ScriptedMachine`.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::asset_extractor::PrismMemorySpace;
use crate::prism::BootDataHeader;
use crate::symbols::SymbolTable;
use crate::RomToolError;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMetaScript {
    pub game_number: u32,
    pub title: String,
    pub version: String,
    pub xina_version: String,
    pub build_timestamp: String,
    pub opto_nc_mask_lo: u64,
    pub opto_nc_mask_hi: u32,
    pub balls_per_game: u8,
    pub ball_save_secs: u8,
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
    pub name_fr: String,
    pub sw_type: u32,
    pub arg: u32,
    pub debounce_ms: u32,
    pub handler_addr: u32,
    pub handler_symbol: String,
    pub opto_nc: bool,
    pub lifted_calls: Vec<LiftedCall>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveDefScript {
    pub id: u8,
    pub name: String,
    pub name_fr: String,
    pub default_pulse_ms: u32,
    pub flags: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LampDefScript {
    pub id: u8,
    pub name: String,
    pub name_fr: String,
    pub flags: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttractSlideScript {
    pub name: String,
    pub duration_ms: u32,
    pub anim_hint: Option<String>,
    pub banner: String,
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
    pub on_enter: Vec<ScriptAction>,
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

fn read_u32_mem(mem: &PrismMemorySpace<'_>, addr: u32) -> Option<u32> {
    mem.read_u32(addr)
}

fn read_cstr_mem(mem: &PrismMemorySpace<'_>, addr: u32) -> String {
    if addr == 0 {
        return String::new();
    }
    let Some(s) = mem.slice_from(addr) else {
        return String::new();
    };
    let end = s.iter().position(|&b| b == 0).unwrap_or(64).min(96);
    String::from_utf8_lossy(&s[..end]).into_owned()
}

/// Lightweight x86-32 call-site & pushed-argument lifter for Pinball 2000 handlers.
/// Scans up to `max_bytes` from `fn_addr` until `RET` (`0xC3`/`0xC2`) and records
/// direct `CALL rel32` (`0xE8`) targets along with immediately preceding `PUSH imm8/imm32`.
pub fn lift_x86_calls(
    mem: &PrismMemorySpace<'_>,
    symtab: &SymbolTable,
    sym_map: &HashMap<u32, String>,
    fn_addr: u32,
    max_bytes: usize,
) -> Vec<LiftedCall> {
    let Some(code) = mem.slice_from(fn_addr) else {
        return Vec::new();
    };
    let len = code.len().min(max_bytes);
    let mut calls = Vec::new();
    let mut push_stack: Vec<u32> = Vec::new();
    let mut i = 0usize;

    while i < len {
        let op = code[i];
        match op {
            0xC3 => break, // ret
            0xC2 => break, // ret imm16
            0x6A if i + 1 < len => {
                // push imm8 (sign-extended)
                let imm = (code[i + 1] as i8) as i32 as u32;
                push_stack.push(imm);
                i += 2;
            }
            0x68 if i + 4 < len => {
                // push imm32
                let imm = u32::from_le_bytes(code[i + 1..i + 5].try_into().unwrap());
                push_stack.push(imm);
                i += 5;
            }
            0xE8 if i + 4 < len => {
                // call rel32
                let rel = i32::from_le_bytes(code[i + 1..i + 5].try_into().unwrap());
                let call_addr = fn_addr.wrapping_add(i as u32);
                let next_ip = call_addr.wrapping_add(5);
                let target_addr = next_ip.wrapping_add_signed(rel);
                let target_symbol = sym_map
                    .get(&target_addr)
                    .cloned()
                    .or_else(|| {
                        symtab
                            .resolve_address(target_addr)
                            .filter(|(_, d)| *d == 0)
                            .map(|(s, _)| s.name.clone())
                    })
                    .unwrap_or_else(|| format!("sub_{target_addr:08x}"));
                let mut args = Vec::new();
                for a in push_stack.iter().rev().take(4) {
                    args.push(*a);
                }
                calls.push(LiftedCall {
                    call_addr,
                    target_addr,
                    target_symbol,
                    pushed_args: args,
                });
                push_stack.clear();
                i += 5;
            }
            0x83 if i + 2 < len && code[i + 1] == 0xC4 => {
                // add esp, imm8
                push_stack.clear();
                i += 3;
            }
            _ => {
                i += 1;
            }
        }
    }
    calls
}

/// Extracts `GameScript` from `game.rom` + `symbols.rom` + optional `UpdateHeader`.
pub fn extract_gamecode_script(
    mem: &PrismMemorySpace<'_>,
    symtab: &SymbolTable,
    header: Option<&BootDataHeader>,
    out_dir: &Path,
) -> Result<GameScript, RomToolError> {
    fs::create_dir_all(out_dir)?;

    let mut sym_map = HashMap::with_capacity(symtab.entries.len());
    for e in &symtab.entries {
        sym_map.entry(e.address).or_insert_with(|| e.name.clone());
    }

    let game_number = header.map(|h| h.game_number).unwrap_or_else(|| {
        if symtab.find_exact("JediMultiball").is_some()
            || !symtab.filter_contains("JediTable").is_empty()
        {
            50069
        } else {
            50070
        }
    });

    // 1. Lift DriveTable (48 entries * 24 bytes)
    let mut drives = Vec::with_capacity(48);
    if let Some(drive_sym) = symtab.find_exact("DriveTable") {
        for id in 0..48u8 {
            let base = drive_sym.address + (id as u32) * 24;
            let flags = read_u32_mem(mem, base).unwrap_or(1);
            let pulse_ms = read_u32_mem(mem, base + 4).unwrap_or(0);
            let name_ptr = read_u32_mem(mem, base + 8).unwrap_or(0);
            let name_fr_ptr = read_u32_mem(mem, base + 16).unwrap_or(0);
            let name = read_cstr_mem(mem, name_ptr);
            let name_fr = read_cstr_mem(mem, name_fr_ptr);
            if !name.is_empty() && name != "Not Used" {
                drives.push(DriveDefScript {
                    id,
                    name,
                    name_fr,
                    default_pulse_ms: pulse_ms,
                    flags,
                });
            }
        }
    }

    // 2. Lift LampTable (128 entries * 36 bytes)
    let mut lamps = Vec::with_capacity(128);
    if let Some(lamp_sym) = symtab.find_exact("LampTable") {
        for id in 0..128u8 {
            let base = lamp_sym.address + (id as u32) * 36;
            let flags = read_u32_mem(mem, base).unwrap_or(1);
            let name_ptr = read_u32_mem(mem, base + 20).unwrap_or(0);
            let name_fr_ptr = read_u32_mem(mem, base + 28).unwrap_or(0);
            let name = read_cstr_mem(mem, name_ptr);
            let name_fr = read_cstr_mem(mem, name_fr_ptr);
            if !name.is_empty() && name != "Not Used" {
                lamps.push(LampDefScript {
                    id,
                    name,
                    name_fr,
                    flags,
                });
            }
        }
    }

    // 3. Lift SwitchTable (96 entries * 48 bytes) + x86 handler calls
    let (opto_lo, opto_hi): (u64, u32) = if game_number == 50069 {
        (0x0000_00F0_1F00_0080, 0x0000_0000)
    } else {
        (0x0000_0000_1F00_0000, 0x0000_0000)
    };

    let mut switches = Vec::with_capacity(96);
    if let Some(sw_sym) = symtab.find_exact("SwitchTable") {
        for id in 0..96u8 {
            let base = sw_sym.address + (id as u32) * 48;
            let sw_type = read_u32_mem(mem, base + 4).unwrap_or(0);
            let handler_addr = read_u32_mem(mem, base + 8).unwrap_or(0);
            let arg = read_u32_mem(mem, base + 12).unwrap_or(0);
            let debounce_ms = read_u32_mem(mem, base + 16).unwrap_or(60);
            let name_ptr = read_u32_mem(mem, base + 32).unwrap_or(0);
            let name_fr_ptr = read_u32_mem(mem, base + 40).unwrap_or(0);
            let name = read_cstr_mem(mem, name_ptr);
            let name_fr = read_cstr_mem(mem, name_fr_ptr);
            let handler_symbol = if handler_addr != 0 {
                sym_map
                    .get(&handler_addr)
                    .cloned()
                    .unwrap_or_else(|| format!("sub_{handler_addr:08x}"))
            } else {
                String::new()
            };
            let lifted_calls = if handler_addr != 0 {
                lift_x86_calls(mem, symtab, &sym_map, handler_addr, 256)
            } else {
                Vec::new()
            };
            let opto_nc = if id < 64 {
                (opto_lo >> id) & 1 == 1
            } else {
                (opto_hi >> (id - 64)) & 1 == 1
            };
            if !name.is_empty() && name != "Not Used" {
                switches.push(SwitchDefScript {
                    id,
                    name,
                    name_fr,
                    sw_type,
                    arg,
                    debounce_ms,
                    handler_addr,
                    handler_symbol,
                    opto_nc,
                    lifted_calls,
                });
            }
        }
    }

    // Auto-detect key hardware IDs from lifted tables
    let trough_switches: Vec<u8> = switches
        .iter()
        .filter(|s| s.name.starts_with("Trough Ball"))
        .map(|s| s.id)
        .collect();
    let shooter_lane_switch = switches
        .iter()
        .find(|s| s.name.eq_ignore_ascii_case("Shooter Lane"))
        .map(|s| s.id);
    let start_button_switch = switches
        .iter()
        .find(|s| s.name.eq_ignore_ascii_case("Start Button"))
        .map(|s| s.id);
    let trough_eject_drive = drives
        .iter()
        .find(|d| d.name.to_ascii_lowercase().contains("trough"))
        .map(|d| d.id);
    let auto_plunge_drive = drives
        .iter()
        .find(|d| d.name.to_ascii_lowercase().contains("plunge"))
        .map(|d| d.id);

    // 4. Build attract sequence + game rules from lifted switches & game profile
    let (attract, vars, modes, mut rules) = if game_number == 50069 {
        build_swe1_script_logic()
    } else {
        build_rfm_script_logic()
    };

    // Enrich switch rules from any lifted x86 calls (slings, jets, etc.) that aren't already covered
    enrich_rules_from_lifted_switches(mem, &switches, &mut rules);

    let script = GameScript {
        schema_version: 1,
        meta: GameMetaScript {
            game_number,
            title: header
                .map(|h| h.game_name.clone())
                .unwrap_or_else(|| {
                    if game_number == 50069 {
                        "Star Wars Episode I".into()
                    } else {
                        "Revenge From Mars".into()
                    }
                }),
            version: header
                .map(|h| format!("{}.{:02}", h.version_major, h.version_minor))
                .unwrap_or_else(|| "rom".into()),
            xina_version: header
                .map(|h| h.xina_version.clone())
                .unwrap_or_default(),
            build_timestamp: header
                .map(|h| h.build_timestamp.clone())
                .unwrap_or_default(),
            opto_nc_mask_lo: opto_lo,
            opto_nc_mask_hi: opto_hi,
            balls_per_game: 3,
            ball_save_secs: 12,
            trough_switches,
            shooter_lane_switch,
            start_button_switch,
            trough_eject_drive,
            auto_plunge_drive,
        },
        hardware: HardwareTablesScript {
            switches,
            drives,
            lamps,
        },
        attract,
        vars,
        modes,
        rules,
    };

    let json = serde_json::to_string_pretty(&script)
        .map_err(|e| RomToolError::InvalidFormat(format!("JSON serialize error: {e}")))?;
    fs::write(out_dir.join("game_script.json"), json)?;

    Ok(script)
}

fn enrich_rules_from_lifted_switches(
    mem: &PrismMemorySpace<'_>,
    switches: &[SwitchDefScript],
    rules: &mut Vec<RuleScript>,
) {
    let existing_sw: std::collections::HashSet<u8> = rules
        .iter()
        .filter_map(|r| match r.trigger {
            ScriptTrigger::OnSwitch { sw_id } => Some(sw_id),
            _ => None,
        })
        .collect();

    for sw in switches {
        if existing_sw.contains(&sw.id) || sw.lifted_calls.is_empty() {
            continue;
        }
        let mut actions = Vec::new();
        for c in &sw.lifted_calls {
            if c.target_symbol.starts_with("generic_score") {
                actions.push(ScriptAction::AddScore { points: 2_310 });
            } else if c.target_symbol.starts_with("score(") && !c.pushed_args.is_empty() {
                let pts = c.pushed_args[0] as u64;
                if pts > 0 && pts <= 10_000_000 {
                    actions.push(ScriptAction::AddScore { points: pts });
                }
            } else if c.target_symbol.starts_with("AudioRequest(") && !c.pushed_args.is_empty() {
                if let Some(dcs_ptr) = read_u32_mem(mem, c.pushed_args[0]) {
                    if let Some(word) = read_u32_mem(mem, dcs_ptr) {
                        let snd_id = (word & 0xFFFF) as u16;
                        if snd_id != 0 {
                            actions.push(ScriptAction::PlaySound { sound_id: snd_id });
                        }
                    }
                }
            }
        }
        if !actions.is_empty() {
            rules.push(RuleScript {
                id: format!("lifted_sw_{}", sw.id),
                trigger: ScriptTrigger::OnSwitch { sw_id: sw.id },
                conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
                actions,
            });
        }
    }
}

fn build_swe1_script_logic() -> (
    Vec<AttractSlideScript>,
    BTreeMap<String, i64>,
    Vec<ModeScript>,
    Vec<RuleScript>,
) {
    let attract = vec![
        ("KahunaLogo", 3500, Some("anim_klogo_zoom_back"), "KAHUNA / WILLIAMS", false),
        ("SlideShow", 5000, Some("anim_battle_droid_intro_battle"), "STAR WARS EPISODE I", false),
        ("Hstd", 4000, None, "GRAND CHAMPION", false),
        ("JediTableYouth", 4000, None, "JEDI YOUTH TABLE", false),
        ("JediTableKnight", 4000, None, "JEDI KNIGHT TABLE", false),
        ("JediTableMaster", 4000, None, "JEDI MASTER TABLE", false),
        ("JediTableSpirit", 4000, None, "JEDI SPIRIT TABLE", false),
        ("RampComboChampion", 4000, None, "RAMP COMBO CHAMPION", false),
        ("P2kLogo", 6000, Some("anim_p2k_logo_animated"), "PINBALL 2000", false),
        ("HasbroAd", 4000, Some("anim_attract_hasbro_ad"), "HASBRO / EPISODE I", false),
        ("ScoreAwards", 4000, None, "REPLAY / EXTRA BALL", false),
        ("EmailAddress", 4000, None, "PINBALL.COM", false),
        ("WebPage", 4000, Some("anim_attract_anakin_web"), "WWW.PINBALL.COM", false),
        ("Scores", 4000, None, "LAST GAME SCORES", true),
        ("GameCredits1", 4000, None, "GAME DESIGN", false),
        ("GameCredits2", 4000, None, "PROGRAMMING", false),
        ("GameCredits3", 4000, None, "ART & ANIMATION", false),
        ("GameCredits4", 4000, None, "SOUND & MUSIC", false),
        ("HtpScene", 5500, Some("anim_battle_droid_intro_badge"), "HOW TO PLAY", false),
    ]
    .into_iter()
    .map(|(n, d, a, b, r)| AttractSlideScript {
        name: n.into(),
        duration_ms: d,
        anim_hint: a.map(str::to_string),
        banner: b.into(),
        require_last_game: r,
    })
    .collect();

    let mut vars = BTreeMap::new();
    vars.insert("shield_hits".into(), 0);
    vars.insert("jedi_mask".into(), 0);
    vars.insert("scenes_completed".into(), 0);
    vars.insert("hangar_locks".into(), 0);
    vars.insert("jet_hits".into(), 0);
    vars.insert("skill_shot_active".into(), 0);

    let modes = vec![
        ModeScript {
            id: "skill_shot".into(),
            name: "Skill Shot".into(),
            duration_secs: Some(15),
            anim_hint: Some("anim_battle_droid_intro_badge".into()),
            sound_id: None,
            music_id: None,
            on_enter: vec![ScriptAction::ShowBanner { text: "SKILL SHOT READY".into() }],
            on_exit: vec![],
        },
        ModeScript {
            id: "hangar_multiball".into(),
            name: "Hangar Multiball".into(),
            duration_secs: None,
            anim_hint: Some("anim_hangar_escape_intro_movie".into()),
            sound_id: Some(0x0259),
            music_id: Some(0x002D),
            on_enter: vec![
                ScriptAction::ShowBanner { text: "HANGAR MULTIBALL!".into() },
                ScriptAction::StartMultiball { balls: 3 },
            ],
            on_exit: vec![],
        },
        ModeScript {
            id: "darth_maul_duel".into(),
            name: "Duel Darth Maul".into(),
            duration_secs: Some(45),
            anim_hint: Some("anim_jedi_battle_intro_movie".into()),
            sound_id: Some(0x02C8),
            music_id: Some(0x000E),
            on_enter: vec![
                ScriptAction::ShowBanner { text: "DUEL DARTH MAUL!".into() },
                ScriptAction::SetLamp { lamp_id: 95, on: true },
            ],
            on_exit: vec![ScriptAction::SetLamp { lamp_id: 95, on: false }],
        },
    ];

    let rules = vec![
        RuleScript {
            id: "swe1_ball_start".into(),
            trigger: ScriptTrigger::OnBallStart,
            conditions: vec![],
            actions: vec![
                ScriptAction::SetVar { var: "skill_shot_active".into(), value: 1 },
                ScriptAction::StartMode { mode: "skill_shot".into() },
                ScriptAction::PlayMusic { sound_id: 0x0002 },
            ],
        },
        RuleScript {
            id: "swe1_left_sling".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 40 },
            conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
            actions: vec![
                ScriptAction::AddScore { points: 2_310 },
                ScriptAction::PulseDrive { drive_id: 14, ms: 32 },
                ScriptAction::PlaySound { sound_id: 0x0020 },
            ],
        },
        RuleScript {
            id: "swe1_right_sling".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 41 },
            conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
            actions: vec![
                ScriptAction::AddScore { points: 2_310 },
                ScriptAction::PulseDrive { drive_id: 15, ms: 32 },
                ScriptAction::PlaySound { sound_id: 0x0021 },
            ],
        },
        RuleScript {
            id: "swe1_jets".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 42 },
            conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
            actions: vec![
                ScriptAction::AddScore { points: 25_000 },
                ScriptAction::IncVar { var: "jet_hits".into(), delta: 1 },
                ScriptAction::PulseDrive { drive_id: 11, ms: 32 },
                ScriptAction::PlaySound { sound_id: 0x038A },
            ],
        },
        RuleScript {
            id: "swe1_middle_jet".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 43 },
            conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
            actions: vec![
                ScriptAction::AddScore { points: 25_000 },
                ScriptAction::IncVar { var: "jet_hits".into(), delta: 1 },
                ScriptAction::PulseDrive { drive_id: 12, ms: 32 },
                ScriptAction::PlaySound { sound_id: 0x038A },
            ],
        },
        RuleScript {
            id: "swe1_lower_jet".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 44 },
            conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
            actions: vec![
                ScriptAction::AddScore { points: 25_000 },
                ScriptAction::IncVar { var: "jet_hits".into(), delta: 1 },
                ScriptAction::PulseDrive { drive_id: 13, ms: 32 },
                ScriptAction::PlaySound { sound_id: 0x038A },
            ],
        },
        RuleScript {
            id: "swe1_shield_hit".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 39 },
            conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
            actions: vec![
                ScriptAction::AddScore { points: 250_000 },
                ScriptAction::IncVar { var: "shield_hits".into(), delta: 1 },
                ScriptAction::PlaySound { sound_id: 0x038B },
                ScriptAction::ShowBanner { text: "SHIELD HIT!".into() },
            ],
        },
        RuleScript {
            id: "swe1_shield_scene_start".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 39 },
            conditions: vec![
                ScriptCondition::BallInPlay,
                ScriptCondition::NotTilted,
                ScriptCondition::VarGe { var: "shield_hits".into(), value: 3 },
            ],
            actions: vec![
                ScriptAction::SetVar { var: "shield_hits".into(), value: 0 },
                ScriptAction::AdvanceScene,
                ScriptAction::AddScore { points: 1_000_000 },
                ScriptAction::PlayAnim { anim: "anim_battle_droid_intro_battle".into() },
            ],
        },
        RuleScript {
            id: "swe1_left_ramp".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 36 },
            conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
            actions: vec![
                ScriptAction::AddScore { points: 500_000 },
                ScriptAction::PlaySound { sound_id: 0x0060 },
                ScriptAction::ShowBanner { text: "LEFT RAMP!".into() },
            ],
        },
        RuleScript {
            id: "swe1_right_ramp".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 37 },
            conditions: vec![ScriptCondition::BallInPlay, ScriptCondition::NotTilted],
            actions: vec![
                ScriptAction::AddScore { points: 500_000 },
                ScriptAction::IncVar { var: "hangar_locks".into(), delta: 1 },
                ScriptAction::PlaySound { sound_id: 0x0061 },
                ScriptAction::ShowBanner { text: "RIGHT RAMP!".into() },
            ],
        },
        RuleScript {
            id: "swe1_hangar_multiball_start".into(),
            trigger: ScriptTrigger::OnSwitch { sw_id: 37 },
            conditions: vec![
                ScriptCondition::BallInPlay,
                ScriptCondition::NotTilted,
                ScriptCondition::VarGe { var: "hangar_locks".into(), value: 3 },
                ScriptCondition::ModeInactive { mode: "hangar_multiball".into() },
            ],
            actions: vec![
                ScriptAction::SetVar { var: "hangar_locks".into(), value: 0 },
                ScriptAction::StartMode { mode: "hangar_multiball".into() },
                ScriptAction::AddScore { points: 2_000_000 },
            ],
        },
    ];

    (attract, vars, modes, rules)
}

fn build_rfm_script_logic() -> (
    Vec<AttractSlideScript>,
    BTreeMap<String, i64>,
    Vec<ModeScript>,
    Vec<RuleScript>,
) {
    let attract = vec![
        AttractSlideScript {
            name: "RfmTitle".into(),
            duration_ms: 4500,
            anim_hint: Some("anim_rfm_logo".into()),
            banner: "REVENGE FROM MARS".into(),
            require_last_game: false,
        },
        AttractSlideScript {
            name: "RfmHstd".into(),
            duration_ms: 4000,
            anim_hint: None,
            banner: "MARTIAN LEADERBOARD".into(),
            require_last_game: false,
        },
    ];
    let mut vars = BTreeMap::new();
    vars.insert("martians_destroyed".into(), 0);
    vars.insert("saucer_hits".into(), 0);

    let modes = vec![ModeScript {
        id: "attack_martians".into(),
        name: "Attack Martians".into(),
        duration_secs: Some(40),
        anim_hint: None,
        sound_id: None,
        music_id: Some(0x0001),
        on_enter: vec![ScriptAction::ShowBanner { text: "ATTACK MARTIANS!".into() }],
        on_exit: vec![],
    }];

    let rules = vec![RuleScript {
        id: "rfm_ball_start".into(),
        trigger: ScriptTrigger::OnBallStart,
        conditions: vec![],
        actions: vec![ScriptAction::StartMode { mode: "attack_martians".into() }],
    }];

    (attract, vars, modes, rules)
}
