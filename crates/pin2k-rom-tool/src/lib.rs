//! Library modules for `pin2k-rom-tool`.

pub mod asset_extractor;
pub mod elf_writer;
pub mod gamecode_extractor;
pub mod gfx_decoder;
pub mod interleave;
pub mod prism;
pub mod scanner;
pub mod symbols;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RomToolError {
    #[error("invalid binary format: {0}")]
    InvalidFormat(String),

    #[error("ROM pair size mismatch: low chip '{low_name}' is {low_len} bytes, high chip '{high_name}' is {high_len} bytes")]
    PairSizeMismatch {
        low_name: String,
        low_len: usize,
        high_name: String,
        high_len: usize,
    },

    #[error("chip size {size} bytes is not aligned to {alignment}-byte interleave width")]
    UnalignedChipSize { size: usize, alignment: usize },

    #[error("no PRISM ROM chip pairs found in '{0}'")]
    NoChipPairsFound(String),

    #[error("offset 0x{offset:X} with length {length} exceeds ROM image size {rom_size}")]
    OutOfBounds {
        offset: usize,
        length: usize,
        rom_size: usize,
    },

    #[error("invalid pixel dimensions: width={width}, height={height}")]
    InvalidDimensions { width: u32, height: u32 },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Parses a decimal or `0x`-prefixed hexadecimal unsigned integer.
pub fn parse_hex_or_dec(input: &str) -> Result<usize, String> {
    let trimmed = input.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        usize::from_str_radix(hex, 16).map_err(|e| format!("invalid hex value '{input}': {e}"))
    } else {
        trimmed
            .parse::<usize>()
            .map_err(|e| format!("invalid integer value '{input}': {e}"))
    }
}

/// Parses a decimal or `0x`-prefixed hexadecimal `u32` (used for 32-bit addresses).
pub fn parse_hex_or_dec_u32(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16).map_err(|e| format!("invalid hex u32 '{input}': {e}"))
    } else {
        trimmed
            .parse::<u32>()
            .map_err(|e| format!("invalid u32 '{input}': {e}"))
    }
}

/// Parses a decimal or `0x`-prefixed hexadecimal `u16` (used for 16-bit color keys).
pub fn parse_hex_or_dec_u16(input: &str) -> Result<u16, String> {
    let trimmed = input.trim();
    if let Some(hex) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        u16::from_str_radix(hex, 16).map_err(|e| format!("invalid hex u16 '{input}': {e}"))
    } else {
        trimmed
            .parse::<u16>()
            .map_err(|e| format!("invalid u16 '{input}': {e}"))
    }
}
