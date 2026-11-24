//! Parser for Pinball 2000 XINA/XINU `*_symbols.rom` symbol tables.
//!
//! Binary layout:
//! - `+0x00`: `"SYMBOL TABLE"` (12 bytes)
//! - `+0x0C`: `u32` checksum (LE)
//! - `+0x10`: `u32` num_entries (LE)
//! - `+0x14`: `u32` string_table_size (LE)
//! - `+0x18`: `num_entries` records of `(u32 addr, u32 name_offset)` (LE)
//! - `+0x18 + num_entries * 8`: NUL-terminated ASCII string table

use serde::Serialize;

use crate::RomToolError;

pub const SYMBOL_TABLE_MAGIC: &[u8; 12] = b"SYMBOL TABLE";
const HEADER_SIZE: usize = 24;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SymbolEntry {
    pub address: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SymbolTable {
    pub checksum: u32,
    pub string_table_size: u32,
    pub entries: Vec<SymbolEntry>,
}

impl SymbolTable {
    /// Parses a raw `*_symbols.rom` byte buffer.
    pub fn parse(data: &[u8]) -> Result<Self, RomToolError> {
        if data.len() < HEADER_SIZE || &data[..12] != SYMBOL_TABLE_MAGIC {
            return Err(RomToolError::InvalidFormat(
                "missing 'SYMBOL TABLE' magic header".to_string(),
            ));
        }

        let checksum = u32::from_le_bytes(data[12..16].try_into().unwrap());
        let num_entries = u32::from_le_bytes(data[16..20].try_into().unwrap()) as usize;
        let string_table_size = u32::from_le_bytes(data[20..24].try_into().unwrap());

        let str_base = HEADER_SIZE
            .checked_add(num_entries.saturating_mul(8))
            .ok_or_else(|| RomToolError::InvalidFormat("symbol table count overflow".into()))?;

        if str_base > data.len() {
            return Err(RomToolError::InvalidFormat(format!(
                "truncated symbol table: need {str_base} bytes, have {}",
                data.len()
            )));
        }

        let mut entries = Vec::with_capacity(num_entries);
        for i in 0..num_entries {
            let rec = HEADER_SIZE + i * 8;
            let address = u32::from_le_bytes(data[rec..rec + 4].try_into().unwrap());
            let name_off = u32::from_le_bytes(data[rec + 4..rec + 8].try_into().unwrap()) as usize;

            let start = str_base.saturating_add(name_off);
            if start >= data.len() {
                return Err(RomToolError::InvalidFormat(format!(
                    "symbol {i} name offset 0x{name_off:X} out of bounds"
                )));
            }
            let end = data[start..]
                .iter()
                .position(|&b| b == 0)
                .map(|rel| start + rel)
                .unwrap_or(data.len());
            let name = String::from_utf8_lossy(&data[start..end]).into_owned();
            entries.push(SymbolEntry { address, name });
        }

        Ok(Self {
            checksum,
            string_table_size,
            entries,
        })
    }

    /// Finds the first symbol whose exact name matches `name`.
    pub fn find_exact(&self, name: &str) -> Option<&SymbolEntry> {
        self.entries.iter().find(|e| e.name == name)
    }

    /// Resolves an address to the nearest symbol at or below `target` plus byte offset.
    pub fn resolve_address(&self, target: u32) -> Option<(&SymbolEntry, u32)> {
        self.entries
            .iter()
            .filter(|e| e.address <= target)
            .max_by_key(|e| e.address)
            .map(|e| (e, target - e.address))
    }

    /// Returns all symbols whose name contains `needle` (case-insensitive).
    pub fn filter_contains<'a>(&'a self, needle: &str) -> Vec<&'a SymbolEntry> {
        let lower = needle.to_ascii_lowercase();
        self.entries
            .iter()
            .filter(|e| e.name.to_ascii_lowercase().contains(&lower))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_symtab(items: &[(u32, &str)]) -> Vec<u8> {
        let mut strings = Vec::new();
        let mut offsets = Vec::new();
        for &(_, name) in items {
            offsets.push(strings.len() as u32);
            strings.extend_from_slice(name.as_bytes());
            strings.push(0);
        }

        let mut buf = Vec::new();
        buf.extend_from_slice(SYMBOL_TABLE_MAGIC);
        buf.extend_from_slice(&0x1234_5678u32.to_le_bytes());
        buf.extend_from_slice(&(items.len() as u32).to_le_bytes());
        buf.extend_from_slice(&(strings.len() as u32).to_le_bytes());
        for (&(addr, _), off) in items.iter().zip(offsets) {
            buf.extend_from_slice(&addr.to_le_bytes());
            buf.extend_from_slice(&off.to_le_bytes());
        }
        buf.extend_from_slice(&strings);
        buf
    }

    #[test]
    fn parses_and_resolves_symbols() {
        let raw = build_test_symtab(&[
            (0x0010_0000, "first(void)"),
            (0x0010_0358, "AttractMode::left_hit(void)"),
            (0x002C_4220, "etext"),
        ]);
        let table = SymbolTable::parse(&raw).unwrap();
        assert_eq!(table.entries.len(), 3);
        assert_eq!(table.find_exact("etext").unwrap().address, 0x002C_4220);

        let (sym, delta) = table.resolve_address(0x0010_0360).unwrap();
        assert_eq!(sym.name, "AttractMode::left_hit(void)");
        assert_eq!(delta, 8);
    }
}
