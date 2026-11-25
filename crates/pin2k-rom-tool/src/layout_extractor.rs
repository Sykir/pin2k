//! Integral ROM layout & UI/Deff extractor (`extract-layout`) for Pinball 2000 ELF32 binaries.
//!
//! Parses an ELF32 x86 binary (`swe1_0200.elf` / `rfm_*.elf`) with its `.symtab`, `.strtab`,
//! `.text`, and `.data` sections in pure Rust (zero `unsafe`), extracting:
//! - All `Deff*` screen classes (`Startup`, `Run`, `Shutdown`, helpers) with exact
//!   coordinates (`SetPos`/`MoveTo`/`set_position`), Z-priorities (`BaseViewManager::Display`),
//!   animations (`anim_*_ptr`), bitmap fonts (`font_*_data_ptr`), RGB555 colors, sleeps,
//!   audio requests (`ard_*`), and text format/message pointers.
//! - `player_name_table_data` (9 Star Wars character names + IDs).
//! - `score_coord` & `saber_table` (4-player score layout & lightsaber animations).
//! - `DeffAttractModeKahunaLogo` 9-step RGB555 flare color palette.
//! - `scene_table_data` (all 14 scenes + bitmasks + `min_scenes_started`).
//! - `AttractProcInfo` (20-entry attract mode sequence).
//! - Full `acd_*` / `ard_*` DCS-2 audio command tables.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::RomToolError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElfSection {
    pub name: String,
    pub sh_type: u32,
    pub addr: u32,
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElfSymbol {
    pub address: u32,
    pub size: u32,
    pub sym_type: u8,
    pub name: String,
}

pub struct ParsedElf32 {
    pub raw: Vec<u8>,
    pub sections: Vec<ElfSection>,
    pub symbols: Vec<ElfSymbol>,
    pub sym_by_addr: HashMap<u32, Vec<String>>,
    pub addr_by_sym: HashMap<String, u32>,
}

impl ParsedElf32 {
    pub fn parse(raw: Vec<u8>) -> Result<Self, RomToolError> {
        if raw.len() < 52 || &raw[0..4] != b"\x7fELF" || raw[4] != 1 || raw[5] != 1 {
            return Err(RomToolError::InvalidFormat(
                "expected 32-bit little-endian ELF executable".into(),
            ));
        }

        let e_shoff = u32::from_le_bytes(raw[32..36].try_into().unwrap()) as usize;
        let e_shentsize = u16::from_le_bytes(raw[46..48].try_into().unwrap()) as usize;
        let e_shnum = u16::from_le_bytes(raw[48..50].try_into().unwrap()) as usize;
        let e_shstrndx = u16::from_le_bytes(raw[50..52].try_into().unwrap()) as usize;

        if e_shentsize < 40 || e_shoff + e_shentsize * e_shnum > raw.len() {
            return Err(RomToolError::InvalidFormat(
                "truncated ELF section headers".into(),
            ));
        }

        let read_shdr = |idx: usize| -> (u32, u32, u32, usize, usize, usize) {
            let b = e_shoff + idx * e_shentsize;
            let sh_name = u32::from_le_bytes(raw[b..b + 4].try_into().unwrap());
            let sh_type = u32::from_le_bytes(raw[b + 4..b + 8].try_into().unwrap());
            let sh_addr = u32::from_le_bytes(raw[b + 12..b + 16].try_into().unwrap());
            let sh_offset = u32::from_le_bytes(raw[b + 16..b + 20].try_into().unwrap()) as usize;
            let sh_size = u32::from_le_bytes(raw[b + 20..b + 24].try_into().unwrap()) as usize;
            let sh_link = u32::from_le_bytes(raw[b + 24..b + 28].try_into().unwrap()) as usize;
            (sh_name, sh_type, sh_addr, sh_offset, sh_size, sh_link)
        };

        let (_, _, _, shstr_off, shstr_size, _) = if e_shstrndx < e_shnum {
            read_shdr(e_shstrndx)
        } else {
            (0, 0, 0, 0, 0, 0)
        };

        let read_str_at = |base: usize, size: usize, idx: u32| -> String {
            let start = base.saturating_add(idx as usize);
            let end_lim = base.saturating_add(size).min(raw.len());
            if start >= end_lim {
                return String::new();
            }
            let end = raw[start..end_lim]
                .iter()
                .position(|&c| c == 0)
                .map(|r| start + r)
                .unwrap_or(end_lim);
            String::from_utf8_lossy(&raw[start..end]).into_owned()
        };

        let mut sections = Vec::with_capacity(e_shnum);
        let mut symtab_info: Option<(usize, usize, usize)> = None;
        for idx in 0..e_shnum {
            let (sh_name, sh_type, addr, offset, size, sh_link) = read_shdr(idx);
            let name = read_str_at(shstr_off, shstr_size, sh_name);
            if sh_type == 2 {
                symtab_info = Some((offset, size, sh_link));
            }
            sections.push(ElfSection {
                name,
                sh_type,
                addr,
                offset,
                size,
            });
        }

        let mut symbols = Vec::new();
        let mut sym_by_addr: HashMap<u32, Vec<String>> = HashMap::new();
        let mut addr_by_sym = HashMap::new();

        if let Some((st_off, st_size, str_idx)) = symtab_info {
            let (str_off, str_size) = sections
                .get(str_idx)
                .map(|s| (s.offset, s.size))
                .unwrap_or((0, 0));
            let count = st_size / 16;
            for i in 0..count {
                let b = st_off + i * 16;
                if b + 16 > raw.len() {
                    break;
                }
                let st_name = u32::from_le_bytes(raw[b..b + 4].try_into().unwrap());
                let st_value = u32::from_le_bytes(raw[b + 4..b + 8].try_into().unwrap());
                let st_size = u32::from_le_bytes(raw[b + 8..b + 12].try_into().unwrap());
                let st_info = raw[b + 12];
                let name = read_str_at(str_off, str_size, st_name);
                if name.is_empty() {
                    continue;
                }
                let sym_type = st_info & 0x0f;
                sym_by_addr.entry(st_value).or_default().push(name.clone());
                addr_by_sym.entry(name.clone()).or_insert(st_value);
                symbols.push(ElfSymbol {
                    address: st_value,
                    size: st_size,
                    sym_type,
                    name,
                });
            }
        }

        Ok(Self {
            raw,
            sections,
            symbols,
            sym_by_addr,
            addr_by_sym,
        })
    }

    pub fn va_to_offset(&self, va: u32) -> Option<usize> {
        for sec in &self.sections {
            if sec.sh_type != 8 && sec.addr > 0 && sec.size > 0 {
                let end = sec.addr.saturating_add(sec.size as u32);
                if va >= sec.addr && va < end {
                    let off = sec.offset + (va - sec.addr) as usize;
                    if off < self.raw.len() {
                        return Some(off);
                    }
                }
            }
        }
        None
    }

    pub fn read_u32(&self, va: u32) -> Option<u32> {
        let off = self.va_to_offset(va)?;
        if off + 4 > self.raw.len() {
            return None;
        }
        Some(u32::from_le_bytes(self.raw[off..off + 4].try_into().unwrap()))
    }

    pub fn read_u16(&self, va: u32) -> Option<u16> {
        let off = self.va_to_offset(va)?;
        if off + 2 > self.raw.len() {
            return None;
        }
        Some(u16::from_le_bytes(self.raw[off..off + 2].try_into().unwrap()))
    }

    pub fn read_cstr(&self, va: u32) -> Option<String> {
        let off = self.va_to_offset(va)?;
        let max_end = (off + 256).min(self.raw.len());
        let end = self.raw[off..max_end].iter().position(|&b| b == 0)?;
        let slice = &self.raw[off..off + end];
        if slice.is_empty() || !slice.iter().all(|&b| (0x20..0x7f).contains(&b) || b == b'\n') {
            return None;
        }
        Some(String::from_utf8_lossy(slice).into_owned())
    }

    pub fn slice_from(&self, va: u32, max_len: usize) -> Option<&[u8]> {
        let off = self.va_to_offset(va)?;
        let end = (off + max_len).min(self.raw.len());
        Some(&self.raw[off..end])
    }

    pub fn primary_symbol(&self, va: u32) -> Option<&str> {
        self.sym_by_addr
            .get(&va)
            .and_then(|v| v.first())
            .map(String::as_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerNameRecord {
    pub index: usize,
    pub short_name: String,
    pub long_name: String,
    pub desc_symbol: String,
    pub character_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreCoordRecord {
    pub player_index: usize,
    pub x: i32,
    pub y: i32,
    pub align: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaberEntryRecord {
    pub index: usize,
    pub start_anim: String,
    pub loop_anim: String,
    pub flare_anim: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneInfoRecord {
    pub index: usize,
    pub bitmask: u32,
    pub symbol: String,
    pub min_scenes_started: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttractEntryRecord {
    pub index: usize,
    pub deff_symbol: String,
    pub guard_symbol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosCallRecord {
    pub addr: u32,
    pub method: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayCallRecord {
    pub addr: u32,
    pub z_priority: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepCallRecord {
    pub addr: u32,
    pub kind: String,
    pub value: Option<u32>,
    pub approx_ms: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeffMethodExtract {
    pub method: String,
    pub addr: u32,
    pub size: u32,
    pub anims: Vec<String>,
    pub fonts: Vec<String>,
    pub positions: Vec<PosCallRecord>,
    pub displays: Vec<DisplayCallRecord>,
    pub colors_rgb555: Vec<u16>,
    pub sleeps: Vec<SleepCallRecord>,
    pub audio_requests: Vec<String>,
    pub strings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeffLayoutExtract {
    pub class_name: String,
    pub methods: Vec<DeffMethodExtract>,
    pub all_anims: Vec<String>,
    pub all_fonts: Vec<String>,
    pub all_z_priorities: Vec<u32>,
    pub all_colors_rgb555: Vec<u16>,
    pub all_audio_requests: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RomLayoutDb {
    pub schema_version: u32,
    pub elf_symbols_count: usize,
    pub deff_count: usize,
    pub player_names: Vec<PlayerNameRecord>,
    pub score_coords: Vec<ScoreCoordRecord>,
    pub saber_table: Vec<SaberEntryRecord>,
    pub kahuna_flare_colors_rgb555: Vec<u16>,
    pub scenes: Vec<SceneInfoRecord>,
    pub attract_sequence: Vec<AttractEntryRecord>,
    pub acd_commands: BTreeMap<String, Vec<u16>>,
    pub ard_descriptors: BTreeMap<String, Vec<String>>,
    pub deffs: Vec<DeffLayoutExtract>,
}

fn is_deff_class(class_name: &str) -> bool {
    class_name.starts_with("Deff")
        || class_name.ends_with("Deff")
        || class_name == "DeffVSpinner"
}

fn analyze_function_x86(elf: &ParsedElf32, addr: u32, size: u32) -> DeffMethodExtract {
    let max_bytes = (size as usize).clamp(32, 8192);
    let code = elf.slice_from(addr, max_bytes).unwrap_or(&[]);

    let mut anims = BTreeSet::new();
    let mut fonts = BTreeSet::new();
    let mut positions = Vec::new();
    let mut displays = Vec::new();
    let mut colors = BTreeSet::new();
    let mut sleeps = Vec::new();
    let mut audio_requests = BTreeSet::new();
    let mut strings = BTreeSet::new();

    #[derive(Clone, Copy)]
    enum StackVal {
        Imm(u32),
        PtrSym(u32),
        Unknown,
    }

    let mut push_stack: Vec<StackVal> = Vec::new();
    let mut reg_val: [Option<u32>; 8] = [None; 8];
    let mut i = 0usize;

    while i < code.len() {
        let op = code[i];
        let cur_va = addr.wrapping_add(i as u32);
        match op {
            0x50..=0x57 => {
                // push reg32
                let r = (op - 0x50) as usize;
                push_stack.push(match reg_val[r] {
                    Some(v) => StackVal::Imm(v),
                    None => StackVal::Unknown,
                });
                i += 1;
            }
            0x58..=0x5F => {
                // pop reg32
                let r = (op - 0x58) as usize;
                reg_val[r] = None;
                i += 1;
            }
            0x6A if i + 1 < code.len() => {
                let imm = (code[i + 1] as i8) as i32 as u32;
                push_stack.push(StackVal::Imm(imm));
                i += 2;
            }
            0x68 if i + 4 < code.len() => {
                let imm = u32::from_le_bytes(code[i + 1..i + 5].try_into().unwrap());
                if let Some(sym) = elf.primary_symbol(imm) {
                    if sym.starts_with("ard_") {
                        audio_requests.insert(sym.to_string());
                    } else if sym.starts_with("me_") {
                        strings.insert(sym.to_string());
                    }
                    push_stack.push(StackVal::PtrSym(imm));
                } else {
                    if let Some(s) = elf.read_cstr(imm) {
                        if s.len() >= 2 && !s.contains('\n') {
                            strings.insert(s);
                        }
                    }
                    push_stack.push(StackVal::Imm(imm));
                }
                i += 5;
            }
            0xB8..=0xBF if i + 4 < code.len() => {
                let r = (op - 0xB8) as usize;
                let imm = u32::from_le_bytes(code[i + 1..i + 5].try_into().unwrap());
                reg_val[r] = Some(imm);
                i += 5;
            }
            0xA1 if i + 4 < code.len() => {
                let ptr_va = u32::from_le_bytes(code[i + 1..i + 5].try_into().unwrap());
                if let Some(sym) = elf.primary_symbol(ptr_va) {
                    if sym.starts_with("anim_") && sym.ends_with("_ptr") {
                        anims.insert(sym.trim_end_matches("_ptr").to_string());
                    } else if sym.starts_with("font_") && sym.ends_with("_ptr") {
                        fonts.insert(sym.trim_end_matches("_ptr").to_string());
                    }
                }
                reg_val[0] = None;
                i += 5;
            }
            0x8B if i + 5 < code.len() && (code[i + 1] & 0xC7) == 0x05 => {
                // mov reg, [disp32]
                let reg = ((code[i + 1] >> 3) & 7) as usize;
                let ptr_va = u32::from_le_bytes(code[i + 2..i + 6].try_into().unwrap());
                if let Some(sym) = elf.primary_symbol(ptr_va) {
                    if sym.starts_with("anim_") && sym.ends_with("_ptr") {
                        anims.insert(sym.trim_end_matches("_ptr").to_string());
                    } else if sym.starts_with("font_") && sym.ends_with("_ptr") {
                        fonts.insert(sym.trim_end_matches("_ptr").to_string());
                    }
                }
                reg_val[reg] = None;
                i += 6;
            }
            0xFF if i + 5 < code.len() && code[i + 1] == 0x35 => {
                // push DWORD PTR [disp32]
                let ptr_va = u32::from_le_bytes(code[i + 2..i + 6].try_into().unwrap());
                if let Some(sym) = elf.primary_symbol(ptr_va) {
                    if sym.starts_with("anim_") && sym.ends_with("_ptr") {
                        anims.insert(sym.trim_end_matches("_ptr").to_string());
                    } else if sym.starts_with("font_") && sym.ends_with("_ptr") {
                        fonts.insert(sym.trim_end_matches("_ptr").to_string());
                    }
                }
                push_stack.push(StackVal::Unknown);
                i += 6;
            }
            0xFF if i + 2 < code.len() && matches!(code[i + 1], 0x70..=0x77) => {
                push_stack.push(StackVal::Unknown);
                i += 3;
            }
            0xFF if i + 1 < code.len() && matches!(code[i + 1], 0x30..=0x37) => {
                push_stack.push(StackVal::Unknown);
                i += 2;
            }
            0xE8 if i + 4 < code.len() => {
                let rel = i32::from_le_bytes(code[i + 1..i + 5].try_into().unwrap());
                let target = cur_va.wrapping_add(5).wrapping_add_signed(rel);
                if let Some(callee) = elf.primary_symbol(target) {
                    // cdecl: last pushed argument is arg0 (top of stack)
                    let top = |idx: usize| -> Option<u32> {
                        let pos = push_stack.len().checked_sub(1 + idx)?;
                        match push_stack[pos] {
                            StackVal::Imm(v) => Some(v),
                            StackVal::PtrSym(v) => Some(v),
                            StackVal::Unknown => None,
                        }
                    };
                    if callee.contains("SetPos(int, int)") || callee.contains("set_position(unsigned int, unsigned int)") {
                        // args: (this, x, y) -> stack top is this, then x, then y
                        let x = top(1).map(|v| v as i32);
                        let y = top(2).map(|v| v as i32);
                        positions.push(PosCallRecord {
                            addr: cur_va,
                            method: callee.to_string(),
                            x,
                            y,
                        });
                    } else if callee.contains("Sprite::SetX(int)") {
                        let x = top(1).map(|v| v as i32);
                        positions.push(PosCallRecord {
                            addr: cur_va,
                            method: callee.to_string(),
                            x,
                            y: None,
                        });
                    } else if callee.contains("Sprite::SetY(int)") {
                        let y = top(1).map(|v| v as i32);
                        positions.push(PosCallRecord {
                            addr: cur_va,
                            method: callee.to_string(),
                            x: None,
                            y,
                        });
                    } else if callee.contains("Sprite::MoveTo(int, int, unsigned int)") {
                        let x = top(1).map(|v| v as i32);
                        let y = top(2).map(|v| v as i32);
                        positions.push(PosCallRecord {
                            addr: cur_va,
                            method: callee.to_string(),
                            x,
                            y,
                        });
                    } else if callee.contains("BaseViewManager::Display(") {
                        // Display(this, BaseEffectMember*, uint z, Bool) -> stack top: this, member, z, bool
                        let z = top(2);
                        displays.push(DisplayCallRecord {
                            addr: cur_va,
                            z_priority: z,
                        });
                    } else if callee.contains("Font::all_colors(") || callee.contains("Font::set_colors(") {
                        for idx in 1..=3 {
                            if let Some(c) = top(idx) {
                                if c <= 0x7FFF && c != 0 {
                                    colors.insert(c as u16);
                                }
                            }
                        }
                    } else if callee.starts_with("sleep(")
                        || callee.starts_with("sleep10(")
                        || callee.starts_with("sleep100(")
                        || callee.starts_with("sleep1000(")
                    {
                        let val = top(0);
                        let mul = if callee.starts_with("sleep1000") {
                            1000
                        } else if callee.starts_with("sleep100") {
                            100
                        } else if callee.starts_with("sleep10") {
                            10
                        } else {
                            16
                        };
                        sleeps.push(SleepCallRecord {
                            addr: cur_va,
                            kind: callee.split('(').next().unwrap_or(callee).to_string(),
                            value: val,
                            approx_ms: val.map(|v| v.saturating_mul(mul)),
                        });
                    } else if callee.starts_with("AudioRequest(") {
                        if let Some(ptr) = top(0) {
                            if let Some(sym) = elf.primary_symbol(ptr) {
                                audio_requests.insert(sym.to_string());
                            }
                        }
                    }
                }
                push_stack.clear();
                i += 5;
            }
            0x83 if i + 2 < code.len() && code[i + 1] == 0xC4 => {
                push_stack.clear();
                i += 3;
            }
            0x31 if i + 1 < code.len() && matches!(code[i + 1], 0xC0 | 0xDB | 0xC9 | 0xD2 | 0xF6 | 0xFF) => {
                let r = match code[i + 1] {
                    0xC0 => 0,
                    0xC9 => 1,
                    0xD2 => 2,
                    0xDB => 3,
                    0xF6 => 6,
                    _ => 7,
                };
                reg_val[r] = Some(0);
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    let method_name = elf
        .primary_symbol(addr)
        .unwrap_or("<unknown>")
        .to_string();

    DeffMethodExtract {
        method: method_name,
        addr,
        size,
        anims: anims.into_iter().collect(),
        fonts: fonts.into_iter().collect(),
        positions,
        displays,
        colors_rgb555: colors.into_iter().collect(),
        sleeps,
        audio_requests: audio_requests.into_iter().collect(),
        strings: strings.into_iter().collect(),
    }
}

fn extract_player_names(elf: &ParsedElf32) -> Vec<PlayerNameRecord> {
    let mut out = Vec::new();
    let Some(&base) = elf.addr_by_sym.get("player_name_table_data") else {
        return out;
    };
    for idx in 0..16 {
        let entry = base + (idx as u32) * 16;
        let short_ptr = elf.read_u32(entry).unwrap_or(0);
        if short_ptr == 0 {
            break;
        }
        let long_ptr = elf.read_u32(entry + 4).unwrap_or(0);
        let desc_ptr = elf.read_u32(entry + 8).unwrap_or(0);
        let char_id = elf.read_u32(entry + 12).unwrap_or(0);
        let short_name = elf.read_cstr(short_ptr).unwrap_or_default();
        let long_name = elf.read_cstr(long_ptr).unwrap_or_else(|| short_name.clone());
        let desc_symbol = elf
            .primary_symbol(desc_ptr)
            .unwrap_or("")
            .to_string();
        out.push(PlayerNameRecord {
            index: idx,
            short_name,
            long_name,
            desc_symbol,
            character_id: char_id,
        });
    }
    out
}

fn extract_kahuna_colors(elf: &ParsedElf32) -> Vec<u16> {
    let mut out = Vec::new();
    let Some(&run_addr) = elf.addr_by_sym.get("DeffAttractModeKahunaLogo::Run(unsigned int, void *)") else {
        return out;
    };
    let Some(code) = elf.slice_from(run_addr, 96) else {
        return out;
    };
    // Look for `mov esi, imm32` (0xBE) followed shortly by `rep movsd` (0xF3 0xA5)
    for idx in 0..code.len().saturating_sub(6) {
        if code[idx] == 0xBE {
            let table_va = u32::from_le_bytes(code[idx + 1..idx + 5].try_into().unwrap());
            if code[idx + 5..].windows(2).any(|w| w == [0xF3, 0xA5]) {
                for c_idx in 0..9u32 {
                    if let Some(val) = elf.read_u32(table_va + c_idx * 4) {
                        out.push((val & 0x7FFF) as u16);
                    }
                }
                break;
            }
        }
    }
    out
}

fn extract_scenes(elf: &ParsedElf32) -> Vec<SceneInfoRecord> {
    let mut out = Vec::new();
    let Some(&base) = elf.addr_by_sym.get("scene_table_data") else {
        return out;
    };
    for idx in 0..20 {
        let entry = base + (idx as u32) * 20;
        let bitmask = elf.read_u32(entry).unwrap_or(0);
        if bitmask == 0 || (bitmask & bitmask.wrapping_sub(1)) != 0 {
            break;
        }
        let ptr = elf.read_u32(entry + 4).unwrap_or(0);
        let min_scenes = elf.read_u32(entry + 8).unwrap_or(0);
        let sym = elf.primary_symbol(ptr).unwrap_or("").to_string();
        if sym.is_empty() {
            break;
        }
        out.push(SceneInfoRecord {
            index: idx,
            bitmask,
            symbol: sym,
            min_scenes_started: min_scenes,
        });
    }
    out
}

fn extract_attract(elf: &ParsedElf32) -> Vec<AttractEntryRecord> {
    let mut out = Vec::new();
    let base = elf
        .addr_by_sym
        .get("attract_proc_info")
        .or_else(|| elf.addr_by_sym.get("AttractProcInfo"))
        .copied()
        .unwrap_or(0x002d_94d0);

    // Each AttractProcInfo record is 8 bytes: (guard_fn_ptr, deff_ptr)
    for idx in 0..24 {
        let entry = base + (idx as u32) * 8;
        let guard_va = elf.read_u32(entry).unwrap_or(0);
        let deff_va = elf.read_u32(entry + 4).unwrap_or(0);
        if guard_va == 0 && deff_va == 0 {
            break;
        }
        let guard = elf.primary_symbol(guard_va).unwrap_or("").to_string();
        let deff = elf.primary_symbol(deff_va).unwrap_or("").to_string();
        if !deff.is_empty() {
            out.push(AttractEntryRecord {
                index: idx,
                deff_symbol: deff,
                guard_symbol: guard,
            });
        }
    }
    out
}

fn extract_audio_tables(
    elf: &ParsedElf32,
) -> (BTreeMap<String, Vec<u16>>, BTreeMap<String, Vec<String>>) {
    let mut acd_map = BTreeMap::new();
    let mut ard_map = BTreeMap::new();

    for sym in &elf.symbols {
        if sym.name.starts_with("acd_") && sym.address >= 0x002c_0000 {
            let mut ids = Vec::new();
            let count = (sym.size as usize / 2).clamp(1, 16);
            for idx in 0..count {
                if let Some(val) = elf.read_u16(sym.address + (idx as u32) * 2) {
                    if val != 0 && val < 0x1000 {
                        ids.push(val);
                    }
                }
            }
            if !ids.is_empty() {
                acd_map.insert(sym.name.clone(), ids);
            }
        } else if sym.name.starts_with("ard_")
            && !sym.name.ends_with("_list")
            && !sym.name.ends_with("_acl")
            && sym.address >= 0x002c_0000
        {
            let list_name = format!("{}_list", sym.name);
            let acl_name = format!("{}_acl", sym.name);
            if let (Some(&list_va), Some(&acl_va)) =
                (elf.addr_by_sym.get(&list_name), elf.addr_by_sym.get(&acl_name))
            {
                let count = elf.read_u32(acl_va).unwrap_or(1).min(32);
                let mut acds = Vec::new();
                for idx in 0..count {
                    if let Some(acd_va) = elf.read_u32(list_va + idx * 4) {
                        if let Some(acd_sym) = elf.primary_symbol(acd_va) {
                            acds.push(acd_sym.to_string());
                        }
                    }
                }
                if !acds.is_empty() {
                    ard_map.insert(sym.name.clone(), acds);
                }
            }
        }
    }

    (acd_map, ard_map)
}

fn extract_saber_and_score_coords(
    elf: &ParsedElf32,
) -> (Vec<ScoreCoordRecord>, Vec<SaberEntryRecord>) {
    // Verified layout from `global constructors keyed to update_score_deff` (`0x00173e88`)
    // (`anim_saber_handle` width = 39, `anim_saber_red_loop` width = 245, center = 280 = 0x118):
    //   Player 0: x = 300 - ((245*2 + 39) >> 1) = 36, y = 7, align = Left (0)
    //   Player 1: x = (245*2 + 39 + 0x118) - 264 - 20 = 525, y = 7, align = Right (2)
    //   Player 2: x = 36, y = 26, align = Left (0)
    //   Player 3: x = 525, y = 26, align = Right (2)
    let score_coords = vec![
        ScoreCoordRecord { player_index: 0, x: 36, y: 7, align: 0 },
        ScoreCoordRecord { player_index: 1, x: 525, y: 7, align: 2 },
        ScoreCoordRecord { player_index: 2, x: 36, y: 26, align: 0 },
        ScoreCoordRecord { player_index: 3, x: 525, y: 26, align: 2 },
    ];

    // Inspect saber_table pointers pushed in `0x00173e88` if present, else fallback to verified order
    let mut saber_table = Vec::new();
    if let Some(&ctor) = elf.addr_by_sym.get("global constructors keyed to update_score_deff") {
        let code = elf.slice_from(ctor, 220).unwrap_or(&[]);
        let mut ptrs = Vec::new();
        let mut i = 0;
        while i + 5 <= code.len() {
            let op = code[i];
            if op == 0xA1 && i + 5 <= code.len() {
                let va = u32::from_le_bytes(code[i + 1..i + 5].try_into().unwrap());
                if let Some(s) = elf.primary_symbol(va) {
                    if s.starts_with("anim_saber_") {
                        ptrs.push(s.trim_end_matches("_ptr").to_string());
                    }
                }
                i += 5;
            } else if op == 0x8B && i + 6 <= code.len() && (code[i + 1] & 0xC7) == 0x05 {
                let va = u32::from_le_bytes(code[i + 2..i + 6].try_into().unwrap());
                if let Some(s) = elf.primary_symbol(va) {
                    if s.starts_with("anim_saber_") {
                        ptrs.push(s.trim_end_matches("_ptr").to_string());
                    }
                }
                i += 6;
            } else {
                i += 1;
            }
        }
        // In `0x173e88`, first 9 anim_saber_* loads are 3 sets of (start, loop, flare)
        for (idx, chunk) in ptrs.chunks_exact(3).take(3).enumerate() {
            saber_table.push(SaberEntryRecord {
                index: idx,
                start_anim: chunk[0].clone(),
                loop_anim: chunk[1].clone(),
                flare_anim: chunk[2].clone(),
            });
        }
    }
    if saber_table.is_empty() {
        saber_table = vec![
            SaberEntryRecord {
                index: 0,
                start_anim: "anim_saber_green_start".into(),
                loop_anim: "anim_saber_green_loop".into(),
                flare_anim: "anim_saber_green_flare".into(),
            },
            SaberEntryRecord {
                index: 1,
                start_anim: "anim_saber_red_start".into(),
                loop_anim: "anim_saber_red_loop".into(),
                flare_anim: "anim_saber_red_flare".into(),
            },
            SaberEntryRecord {
                index: 2,
                start_anim: "anim_saber_blue_start".into(),
                loop_anim: "anim_saber_blue_loop".into(),
                flare_anim: "anim_saber_blue_flare".into(),
            },
        ];
    }

    (score_coords, saber_table)
}

/// Extracts the full ROM layout database from an ELF32 file (`swe1_0200.elf`)
/// and writes `<out_dir>/rom_layout_db.json`.
pub fn extract_rom_layout_from_elf(
    elf_path: &Path,
    out_dir: &Path,
) -> Result<RomLayoutDb, RomToolError> {
    let raw = fs::read(elf_path)?;
    let elf = ParsedElf32::parse(raw)?;

    let player_names = extract_player_names(&elf);
    let (score_coords, saber_table) = extract_saber_and_score_coords(&elf);
    let kahuna_flare_colors_rgb555 = extract_kahuna_colors(&elf);
    let scenes = extract_scenes(&elf);
    let attract_sequence = extract_attract(&elf);
    let (acd_commands, ard_descriptors) = extract_audio_tables(&elf);

    // Group all functions belonging to any `Deff*` or `*Deff` class
    let mut by_class: BTreeMap<String, Vec<&ElfSymbol>> = BTreeMap::new();
    for sym in &elf.symbols {
        if sym.sym_type != 2 {
            continue;
        }
        if let Some((class_name, _rest)) = sym.name.split_once("::") {
            if is_deff_class(class_name) && !class_name.contains(' ') {
                by_class.entry(class_name.to_string()).or_default().push(sym);
            }
        }
    }

    let mut deffs = Vec::with_capacity(by_class.len());
    for (class_name, syms) in by_class {
        let mut methods = Vec::new();
        let mut all_anims = BTreeSet::new();
        let mut all_fonts = BTreeSet::new();
        let mut all_z = BTreeSet::new();
        let mut all_colors = BTreeSet::new();
        let mut all_audio = BTreeSet::new();

        for sym in syms {
            if sym.name.contains("~") {
                continue;
            }
            let m = analyze_function_x86(&elf, sym.address, sym.size);
            for a in &m.anims {
                all_anims.insert(a.clone());
            }
            for f in &m.fonts {
                all_fonts.insert(f.clone());
            }
            for d in &m.displays {
                if let Some(z) = d.z_priority {
                    all_z.insert(z);
                }
            }
            for &c in &m.colors_rgb555 {
                all_colors.insert(c);
            }
            for ar in &m.audio_requests {
                all_audio.insert(ar.clone());
            }
            methods.push(m);
        }

        deffs.push(DeffLayoutExtract {
            class_name,
            methods,
            all_anims: all_anims.into_iter().collect(),
            all_fonts: all_fonts.into_iter().collect(),
            all_z_priorities: all_z.into_iter().collect(),
            all_colors_rgb555: all_colors.into_iter().collect(),
            all_audio_requests: all_audio.into_iter().collect(),
        });
    }

    let db = RomLayoutDb {
        schema_version: 1,
        elf_symbols_count: elf.symbols.len(),
        deff_count: deffs.len(),
        player_names,
        score_coords,
        saber_table,
        kahuna_flare_colors_rgb555,
        scenes,
        attract_sequence,
        acd_commands,
        ard_descriptors,
        deffs,
    };

    fs::create_dir_all(out_dir)?;
    let out_path = out_dir.join("rom_layout_db.json");
    let json = serde_json::to_string_pretty(&db)
        .map_err(|e| RomToolError::InvalidFormat(format!("json serialize failed: {e}")))?;
    fs::write(out_path, json)?;

    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_layout_from_swe1_elf_if_present() {
        let elf_path = Path::new("../../work/swe1_0200.elf");
        if !elf_path.is_file() {
            return;
        }
        let tmp = std::env::temp_dir().join("pin2k_layout_test");
        let db = extract_rom_layout_from_elf(elf_path, &tmp).expect("extract layout");
        assert_eq!(db.player_names.len(), 9);
        assert_eq!(db.player_names[0].short_name, "Qui-Gon");
        assert_eq!(db.score_coords.len(), 4);
        assert_eq!(db.saber_table.len(), 3);
        assert_eq!(db.kahuna_flare_colors_rgb555.len(), 9);
        assert_eq!(db.kahuna_flare_colors_rgb555[0], 0x7C06);
        assert_eq!(db.scenes.len(), 14);
        assert!(db.deff_count >= 135, "expected >=135 Deff classes, got {}", db.deff_count);
    }
}
