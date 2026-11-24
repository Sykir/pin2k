//! Minimal clean-room 32-bit x86 ELF (`ET_EXEC`, `EM_386`) wrapper for raw
//! Pinball 2000 `*_game.rom` binaries + optional `*_symbols.rom` tables.
//!
//! Wrapping `game.rom` at its true load address (`0x00100000`) with a full
//! `.symtab` lets Ghidra, IDA, Binary Ninja, objdump, and REA (`rea-agents`)
//! auto-load the game code with every C++ class/method name intact.

use crate::symbols::SymbolTable;
use crate::RomToolError;

const ELF_HDR_SIZE: usize = 52;
const PHDR_SIZE: usize = 32;
const SHDR_SIZE: usize = 40;
const SYM_SIZE: usize = 16;

// ELF constants
const ET_EXEC: u16 = 2;
const EM_386: u16 = 3;
const PT_LOAD: u32 = 1;
const PF_X: u32 = 1;
const PF_W: u32 = 2;
const PF_R: u32 = 4;

const SHT_NULL: u32 = 0;
const SHT_PROGBITS: u32 = 1;
const SHT_SYMTAB: u32 = 2;
const SHT_STRTAB: u32 = 3;
const SHT_NOBITS: u32 = 8;

const SHF_WRITE: u32 = 1;
const SHF_ALLOC: u32 = 2;
const SHF_EXECINSTR: u32 = 4;

const STB_GLOBAL: u8 = 1;
const STT_OBJECT: u8 = 1;
const STT_FUNC: u8 = 2;

/// Options for wrapping a raw 32-bit x86 game binary into an ELF32 executable.
#[derive(Debug, Clone)]
pub struct ElfWrapOptions {
    /// Virtual load address of byte 0 of `game_bytes` (Pinball 2000 default: `0x0010_0000`).
    pub load_addr: u32,
    /// Entry point virtual address (defaults to `load_addr`, i.e. `first(void)`).
    pub entry_addr: u32,
    /// Optional explicit `.text` end address. If `None`, uses `etext` symbol if present.
    pub etext_addr: Option<u32>,
    /// Optional `.bss` size in bytes appended after the loaded file image.
    pub bss_size: u32,
}

impl Default for ElfWrapOptions {
    fn default() -> Self {
        Self {
            load_addr: 0x0010_0000,
            entry_addr: 0x0010_0000,
            etext_addr: None,
            bss_size: 0x0010_0000, // 1 MiB default BSS headroom when unknown
        }
    }
}

/// Summary of the generated ELF32 layout.
#[derive(Debug, Clone)]
pub struct ElfWrapSummary {
    pub load_addr: u32,
    pub entry_addr: u32,
    pub text_size: u32,
    pub data_size: u32,
    pub bss_size: u32,
    pub symbol_count: usize,
    pub total_elf_bytes: usize,
}

/// Builds a valid 32-bit little-endian x86 ELF executable containing `game_bytes`
/// and (optionally) all symbols from `symbols`.
pub fn wrap_raw_x86_to_elf(
    game_bytes: &[u8],
    symbols: Option<&SymbolTable>,
    opts: &ElfWrapOptions,
) -> Result<(Vec<u8>, ElfWrapSummary), RomToolError> {
    if game_bytes.is_empty() {
        return Err(RomToolError::InvalidFormat(
            "cannot wrap an empty game binary".into(),
        ));
    }

    let load_addr = opts.load_addr;
    let file_len = u32::try_from(game_bytes.len())
        .map_err(|_| RomToolError::InvalidFormat("game binary exceeds 4 GiB".into()))?;
    let image_end = load_addr
        .checked_add(file_len)
        .ok_or_else(|| RomToolError::InvalidFormat("load_addr + size overflows u32".into()))?;

    // Determine .text / .data split using explicit option or `etext` symbol
    let detected_etext = opts
        .etext_addr
        .or_else(|| symbols.and_then(|st| st.find_exact("etext").map(|e| e.address)));

    let text_end = match detected_etext {
        Some(addr) if addr > load_addr && addr <= image_end => addr,
        _ => image_end,
    };
    let text_size = text_end - load_addr;
    let data_size = image_end - text_end;

    // Infer .bss size from symbols in RAM above `image_end` (capped below 16 MiB RAM top)
    let inferred_bss = symbols
        .and_then(|st| {
            st.entries
                .iter()
                .map(|e| e.address)
                .filter(|&a| a >= image_end && a < 0x0100_0000)
                .max()
                .map(|max_a| (max_a - image_end).saturating_add(0x1000))
        })
        .unwrap_or(opts.bss_size);
    let bss_size = inferred_bss.max(opts.bss_size);

    // Build section header string table (.shstrtab)
    let mut shstrtab = Vec::new();
    shstrtab.push(0);
    let sh_name_text = push_cstr(&mut shstrtab, ".text");
    let sh_name_data = push_cstr(&mut shstrtab, ".data");
    let sh_name_bss = push_cstr(&mut shstrtab, ".bss");
    let sh_name_symtab = push_cstr(&mut shstrtab, ".symtab");
    let sh_name_strtab = push_cstr(&mut shstrtab, ".strtab");
    let sh_name_shstrtab = push_cstr(&mut shstrtab, ".shstrtab");

    // Section indices:
    // 0: NULL
    // 1: .text
    // 2: .data
    // 3: .bss
    // 4: .symtab
    // 5: .strtab
    // 6: .shstrtab
    const SHN_TEXT: u16 = 1;
    const SHN_DATA: u16 = 2;
    const SHN_BSS: u16 = 3;
    const SHN_ABS: u16 = 0xFFF1;

    // Build .strtab and .symtab
    let mut strtab = Vec::new();
    strtab.push(0);
    let mut symtab = Vec::new();
    // Symbol 0: STN_UNDEF
    symtab.extend_from_slice(&[0u8; SYM_SIZE]);

    let mut written_symbols = 0usize;
    if let Some(st) = symbols {
        // Sort a copy of symbol addresses to estimate function sizes in .text
        let mut sorted_addrs: Vec<u32> = st.entries.iter().map(|e| e.address).collect();
        sorted_addrs.sort_unstable();
        sorted_addrs.dedup();

        for entry in &st.entries {
            let name_off = push_cstr(&mut strtab, &entry.name);
            let addr = entry.address;

            let (shndx, st_type, st_size) = if addr >= load_addr && addr < text_end {
                let next = sorted_addrs
                    .partition_point(|&a| a <= addr)
                    .checked_sub(0)
                    .and_then(|idx| sorted_addrs.get(idx).copied())
                    .filter(|&a| a <= text_end)
                    .unwrap_or(text_end);
                (SHN_TEXT, STT_FUNC, next.saturating_sub(addr))
            } else if addr >= text_end && addr < image_end {
                (SHN_DATA, STT_OBJECT, 0)
            } else if addr >= image_end && addr < image_end.saturating_add(bss_size) {
                (SHN_BSS, STT_OBJECT, 0)
            } else {
                (SHN_ABS, STT_OBJECT, 0)
            };

            let st_info = (STB_GLOBAL << 4) | (st_type & 0x0F);
            symtab.extend_from_slice(&name_off.to_le_bytes());
            symtab.extend_from_slice(&addr.to_le_bytes());
            symtab.extend_from_slice(&st_size.to_le_bytes());
            symtab.push(st_info);
            symtab.push(0); // st_other
            symtab.extend_from_slice(&shndx.to_le_bytes());
            written_symbols += 1;
        }
    }

    // Layout inside ELF file:
    // 0x0000: ELF header (52 B) + 2 Program Headers (64 B) -> padded to 0x1000
    // 0x1000: .text (text_size) + .data (data_size)
    // aligned: .symtab
    // aligned: .strtab
    // aligned: .shstrtab
    // aligned: Section Header Table (7 * 40 B)
    let ph_count: u16 = if data_size > 0 || bss_size > 0 { 2 } else { 1 };
    let text_file_off: u32 = 0x1000;
    let data_file_off: u32 = text_file_off + text_size;

    let mut elf = vec![0u8; text_file_off as usize];
    elf.extend_from_slice(game_bytes);

    align_vec(&mut elf, 4);
    let symtab_off = elf.len() as u32;
    elf.extend_from_slice(&symtab);

    align_vec(&mut elf, 4);
    let strtab_off = elf.len() as u32;
    elf.extend_from_slice(&strtab);

    align_vec(&mut elf, 4);
    let shstrtab_off = elf.len() as u32;
    elf.extend_from_slice(&shstrtab);

    align_vec(&mut elf, 4);
    let shoff = elf.len() as u32;

    // Write 7 section headers
    let sh_count: u16 = 7;
    // 0: NULL
    write_shdr(&mut elf, 0, SHT_NULL, 0, 0, 0, 0, 0, 0, 0, 0);
    // 1: .text
    write_shdr(
        &mut elf,
        sh_name_text,
        SHT_PROGBITS,
        SHF_ALLOC | SHF_EXECINSTR,
        load_addr,
        text_file_off,
        text_size,
        0,
        0,
        16,
        0,
    );
    // 2: .data
    write_shdr(
        &mut elf,
        sh_name_data,
        SHT_PROGBITS,
        SHF_ALLOC | SHF_WRITE,
        text_end,
        data_file_off,
        data_size,
        0,
        0,
        16,
        0,
    );
    // 3: .bss
    write_shdr(
        &mut elf,
        sh_name_bss,
        SHT_NOBITS,
        SHF_ALLOC | SHF_WRITE,
        image_end,
        data_file_off + data_size,
        bss_size,
        0,
        0,
        16,
        0,
    );
    // 4: .symtab (sh_link = 5 (.strtab), sh_info = 1 (first non-local symbol))
    write_shdr(
        &mut elf,
        sh_name_symtab,
        SHT_SYMTAB,
        0,
        0,
        symtab_off,
        symtab.len() as u32,
        5,
        1,
        4,
        SYM_SIZE as u32,
    );
    // 5: .strtab
    write_shdr(
        &mut elf,
        sh_name_strtab,
        SHT_STRTAB,
        0,
        0,
        strtab_off,
        strtab.len() as u32,
        0,
        0,
        1,
        0,
    );
    // 6: .shstrtab
    write_shdr(
        &mut elf,
        sh_name_shstrtab,
        SHT_STRTAB,
        0,
        0,
        shstrtab_off,
        shstrtab.len() as u32,
        0,
        0,
        1,
        0,
    );

    // Patch ELF32 header at offset 0
    let mut ehdr = Vec::with_capacity(ELF_HDR_SIZE);
    ehdr.extend_from_slice(&[
        0x7F, b'E', b'L', b'F', // e_ident[0..4]
        1,    // ELFCLASS32
        1,    // ELFDATA2LSB
        1,    // EV_CURRENT
        0,    // ELFOSABI_NONE
        0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    ehdr.extend_from_slice(&ET_EXEC.to_le_bytes());
    ehdr.extend_from_slice(&EM_386.to_le_bytes());
    ehdr.extend_from_slice(&1u32.to_le_bytes()); // e_version
    ehdr.extend_from_slice(&opts.entry_addr.to_le_bytes());
    ehdr.extend_from_slice(&(ELF_HDR_SIZE as u32).to_le_bytes()); // e_phoff
    ehdr.extend_from_slice(&shoff.to_le_bytes()); // e_shoff
    ehdr.extend_from_slice(&0u32.to_le_bytes()); // e_flags
    ehdr.extend_from_slice(&(ELF_HDR_SIZE as u16).to_le_bytes());
    ehdr.extend_from_slice(&(PHDR_SIZE as u16).to_le_bytes());
    ehdr.extend_from_slice(&ph_count.to_le_bytes());
    ehdr.extend_from_slice(&(SHDR_SIZE as u16).to_le_bytes());
    ehdr.extend_from_slice(&sh_count.to_le_bytes());
    ehdr.extend_from_slice(&6u16.to_le_bytes()); // e_shstrndx = .shstrtab
    elf[0..ELF_HDR_SIZE].copy_from_slice(&ehdr);

    // Patch Program Headers right after ELF header
    let mut phdrs = Vec::with_capacity((ph_count as usize) * PHDR_SIZE);
    // PT_LOAD 0: .text (R|X)
    write_phdr(
        &mut phdrs,
        PT_LOAD,
        text_file_off,
        load_addr,
        load_addr,
        text_size,
        text_size,
        PF_R | PF_X,
        0x1000,
    );
    if ph_count == 2 {
        // PT_LOAD 1: .data + .bss (R|W)
        write_phdr(
            &mut phdrs,
            PT_LOAD,
            data_file_off,
            text_end,
            text_end,
            data_size,
            data_size.saturating_add(bss_size),
            PF_R | PF_W,
            0x1000,
        );
    }
    elf[ELF_HDR_SIZE..ELF_HDR_SIZE + phdrs.len()].copy_from_slice(&phdrs);

    let summary = ElfWrapSummary {
        load_addr,
        entry_addr: opts.entry_addr,
        text_size,
        data_size,
        bss_size,
        symbol_count: written_symbols,
        total_elf_bytes: elf.len(),
    };

    Ok((elf, summary))
}

fn push_cstr(table: &mut Vec<u8>, s: &str) -> u32 {
    let off = table.len() as u32;
    table.extend_from_slice(s.as_bytes());
    table.push(0);
    off
}

fn align_vec(buf: &mut Vec<u8>, align: usize) {
    let rem = buf.len() % align;
    if rem != 0 {
        buf.resize(buf.len() + (align - rem), 0);
    }
}

#[allow(clippy::too_many_arguments)]
fn write_phdr(
    out: &mut Vec<u8>,
    p_type: u32,
    p_offset: u32,
    p_vaddr: u32,
    p_paddr: u32,
    p_filesz: u32,
    p_memsz: u32,
    p_flags: u32,
    p_align: u32,
) {
    out.extend_from_slice(&p_type.to_le_bytes());
    out.extend_from_slice(&p_offset.to_le_bytes());
    out.extend_from_slice(&p_vaddr.to_le_bytes());
    out.extend_from_slice(&p_paddr.to_le_bytes());
    out.extend_from_slice(&p_filesz.to_le_bytes());
    out.extend_from_slice(&p_memsz.to_le_bytes());
    out.extend_from_slice(&p_flags.to_le_bytes());
    out.extend_from_slice(&p_align.to_le_bytes());
}

#[allow(clippy::too_many_arguments)]
fn write_shdr(
    out: &mut Vec<u8>,
    sh_name: u32,
    sh_type: u32,
    sh_flags: u32,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
) {
    out.extend_from_slice(&sh_name.to_le_bytes());
    out.extend_from_slice(&sh_type.to_le_bytes());
    out.extend_from_slice(&sh_flags.to_le_bytes());
    out.extend_from_slice(&sh_addr.to_le_bytes());
    out.extend_from_slice(&sh_offset.to_le_bytes());
    out.extend_from_slice(&sh_size.to_le_bytes());
    out.extend_from_slice(&sh_link.to_le_bytes());
    out.extend_from_slice(&sh_info.to_le_bytes());
    out.extend_from_slice(&sh_addralign.to_le_bytes());
    out.extend_from_slice(&sh_entsize.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbols::SymbolEntry;

    #[test]
    fn wraps_raw_x86_with_symbols_into_valid_elf32() {
        let code = vec![0x90u8; 0x200];
        let syms = SymbolTable {
            checksum: 0,
            string_table_size: 0,
            entries: vec![
                SymbolEntry {
                    address: 0x0010_0000,
                    name: "first(void)".into(),
                },
                SymbolEntry {
                    address: 0x0010_0100,
                    name: "etext".into(),
                },
                SymbolEntry {
                    address: 0x0010_0120,
                    name: "g_score".into(),
                },
            ],
        };

        let (elf, summary) =
            wrap_raw_x86_to_elf(&code, Some(&syms), &ElfWrapOptions::default()).unwrap();
        assert_eq!(&elf[0..4], b"\x7fELF");
        assert_eq!(elf[4], 1); // 32-bit
        assert_eq!(elf[5], 1); // LE
        assert_eq!(summary.text_size, 0x100);
        assert_eq!(summary.data_size, 0x100);
        assert_eq!(summary.symbol_count, 3);
    }
}
