//! ASCII string and executable/asset header scanner for reconstructed PRISM ROM images.

use serde::Serialize;

/// A printable ASCII string found inside a binary image.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AsciiMatch {
    pub offset: usize,
    pub text: String,
}

/// A recognized binary header or magic signature found inside a ROM image.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeaderMatch {
    pub offset: usize,
    pub kind: String,
    pub detail: String,
}

/// Combined scan output for serialization or CLI display.
#[derive(Debug, Clone, Serialize)]
pub struct ScanReport {
    pub rom_size: usize,
    pub base_offset: usize,
    pub headers: Vec<HeaderMatch>,
    pub strings: Vec<AsciiMatch>,
}

/// Extracts contiguous printable ASCII strings (`0x20..=0x7E`) of at least `min_len` bytes.
pub fn scan_ascii_strings(
    data: &[u8],
    base_offset: usize,
    min_len: usize,
    filter_substring: Option<&str>,
    limit: Option<usize>,
) -> Vec<AsciiMatch> {
    let min_len = min_len.max(1);
    let filter_lower = filter_substring.map(|s| s.to_ascii_lowercase());
    let mut results = Vec::new();

    let mut run_start: Option<usize> = None;

    for (idx, &byte) in data.iter().enumerate() {
        let is_printable = (0x20..=0x7E).contains(&byte);
        match (run_start, is_printable) {
            (None, true) => run_start = Some(idx),
            (Some(start), false) => {
                if idx - start >= min_len {
                    if let Some(m) = build_match(data, start, idx, base_offset, filter_lower.as_deref())
                    {
                        results.push(m);
                        if limit.is_some_and(|max| results.len() >= max) {
                            return results;
                        }
                    }
                }
                run_start = None;
            }
            _ => {}
        }
    }

    if let Some(start) = run_start {
        if data.len() - start >= min_len {
            if let Some(m) = build_match(data, start, data.len(), base_offset, filter_lower.as_deref())
            {
                results.push(m);
            }
        }
    }

    results
}

fn build_match(
    data: &[u8],
    start: usize,
    end: usize,
    base_offset: usize,
    filter_lower: Option<&str>,
) -> Option<AsciiMatch> {
    let text = String::from_utf8_lossy(&data[start..end]).into_owned();
    if let Some(needle) = filter_lower {
        if !text.to_ascii_lowercase().contains(needle) {
            return None;
        }
    }
    Some(AsciiMatch {
        offset: base_offset + start,
        text,
    })
}

/// Scans for high-signal executable, archive, audio, and Pinball 2000 / XINA signatures.
pub fn scan_headers(data: &[u8], base_offset: usize) -> Vec<HeaderMatch> {
    let mut matches = Vec::new();

    // Known magic byte patterns
    let signatures: &[(&[u8], &str, &str)] = &[
        (b"\x7fELF", "ELF", "ELF executable/object header"),
        (b"\x89PNG\r\n\x1a\n", "PNG", "PNG image signature"),
        (b"RIFF", "RIFF", "RIFF container (WAV/AVI)"),
        (b"PK\x03\x04", "ZIP", "ZIP local file header"),
        (b"\x1f\x8b\x08", "GZIP", "GZIP compressed stream"),
        (b"XINA", "XINA", "XINA RTOS marker"),
        (b"PRISM", "PRISM", "PRISM board/firmware marker"),
        (b"PIN2000", "PIN2000", "Pinball 2000 platform marker"),
    ];

    for (sig, kind, detail) in signatures {
        for pos in find_all(data, sig) {
            // Extra validation for RIFF to cut false positives
            if *kind == "RIFF" {
                if pos + 12 > data.len() {
                    continue;
                }
                let fourcc = &data[pos + 8..pos + 12];
                if !fourcc.iter().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || *b == b' ')
                {
                    continue;
                }
                let fourcc_str = String::from_utf8_lossy(fourcc);
                matches.push(HeaderMatch {
                    offset: base_offset + pos,
                    kind: (*kind).to_string(),
                    detail: format!("RIFF container (type '{}')", fourcc_str.trim()),
                });
                continue;
            }

            matches.push(HeaderMatch {
                offset: base_offset + pos,
                kind: (*kind).to_string(),
                detail: (*detail).to_string(),
            });
        }
    }

    // Inspect x86 reset vector at the last 16 bytes of the ROM image (0xFFFFFFF0 alias)
    if data.len() >= 16 {
        let reset_pos = data.len() - 16;
        let first_opcode = data[reset_pos];
        if matches!(first_opcode, 0xEA | 0xE9 | 0xEB) {
            matches.push(HeaderMatch {
                offset: base_offset + reset_pos,
                kind: "X86_RESET_VECTOR".to_string(),
                detail: format!(
                    "Plausible x86 reset jump at ROM top-16 (opcode 0x{first_opcode:02X})"
                ),
            });
        }
    }

    matches.sort_by_key(|m| m.offset);
    matches
}

fn find_all(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 0;
    let last = haystack.len() - needle.len();
    while i <= last {
        if &haystack[i..i + needle.len()] == needle {
            out.push(i);
            i += needle.len();
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_ascii_strings_and_filters() {
        let blob = b"\x00\x01XINA_KERNEL_V1\x00\x02hi\x00RFM_SAUCER_ANIM\x00";
        let all = scan_ascii_strings(blob, 0x1000, 6, None, None);
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].offset, 0x1002);
        assert_eq!(all[0].text, "XINA_KERNEL_V1");

        let filtered = scan_ascii_strings(blob, 0x1000, 6, Some("saucer"), None);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].text, "RFM_SAUCER_ANIM");
    }

    #[test]
    fn finds_elf_and_reset_vector() {
        let mut blob = vec![0u8; 64];
        blob[8..12].copy_from_slice(b"\x7fELF");
        blob[48] = 0xEA; // x86 far jmp at top-16
        let headers = scan_headers(&blob, 0);
        assert!(headers.iter().any(|h| h.kind == "ELF" && h.offset == 8));
        assert!(headers
            .iter()
            .any(|h| h.kind == "X86_RESET_VECTOR" && h.offset == 48));
    }
}
