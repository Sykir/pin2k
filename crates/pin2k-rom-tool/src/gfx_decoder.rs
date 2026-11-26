//! Pinball 2000 graphics decoders:
//! - Raw pixel & palette decoders (`RGB555`, `RGB565`, `BGR555`, `BGR565`, `Indexed8`)
//! - Lifted XINA/Pinball 2000 `Animation` & `Movie` decompression routines
//!   covering all codecs used by *Star Wars Episode I* and *Revenge From Mars*:
//!   - Code `0`:  8-bit paletted raw -> RGB555
//!   - Code `1`:  8-bit paletted L-RLE -> RGB555
//!   - Code `20`: 15-bit RGB555 raw
//!   - Code `21`: 15-bit RGB555 L-RLE
//!   - Code `22`: 15-bit 64-color + 64-pair dictionary RLE
//!   - Code `23`: 15-bit 32-color + 128-pair dictionary RLE
//!   - Code `30`: 15-bit 4×4 block-quantized movie (`decompress_movie`)
//!   - Code `42`: 15-bit 64-color row-delta dictionary RLE
//!   - Code `43`: 15-bit 32-color row-delta dictionary RLE
//!   - Code `44`: 15-bit 64-color run-delta dictionary RLE
//!   - Code `45`: 15-bit 32-color run-delta dictionary RLE

use std::path::Path;

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::RomToolError;

/// Pinball 2000 hardware RGB555 transparency color key (bright magenta).
pub const P2K_TRANSPARENT_KEY_RGB555: u16 = 0x7C1F;

/// Supported raw pixel formats for ROM graphic inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    Rgb565,
    Rgb555,
    Bgr565,
    Bgr555,
    Indexed8,
}

impl PixelFormat {
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            PixelFormat::Rgb565
            | PixelFormat::Rgb555
            | PixelFormat::Bgr565
            | PixelFormat::Bgr555 => 2,
            PixelFormat::Indexed8 => 1,
        }
    }
}

/// Palette encoding when previewing 8-bit indexed sprites with a ROM palette slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteFormat {
    Rgb565,
    Rgb555,
    Rgb888,
}

/// Configuration for decoding a raw ROM byte slice into an RGBA8888 image.
#[derive(Debug, Clone)]
pub struct PreviewOptions {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub stride: Option<usize>,
    pub big_endian: bool,
    pub transparent_key: Option<u16>,
    pub scale: u32,
}

/// Converts a 5-bit channel (0..=31) to 8-bit (0..=255) with bit replication.
#[inline]
pub const fn expand_5_to_8(v: u16) -> u8 {
    let v = (v & 0x1F) as u8;
    (v << 3) | (v >> 2)
}

/// Converts a 6-bit channel (0..=63) to 8-bit (0..=255) with bit replication.
#[inline]
pub const fn expand_6_to_8(v: u16) -> u8 {
    let v = (v & 0x3F) as u8;
    (v << 2) | (v >> 4)
}

/// Decodes a single 16-bit RGB565/RGB555/BGR565/BGR555 pixel to `[R, G, B, A]`.
#[inline]
pub fn decode_16bit_pixel(
    raw: u16,
    format: PixelFormat,
    transparent_key: Option<u16>,
) -> [u8; 4] {
    if transparent_key == Some(raw) {
        return [0, 0, 0, 0];
    }

    match format {
        PixelFormat::Rgb565 => {
            let r = expand_5_to_8(raw >> 11);
            let g = expand_6_to_8(raw >> 5);
            let b = expand_5_to_8(raw);
            [r, g, b, 255]
        }
        PixelFormat::Bgr565 => {
            let b = expand_5_to_8(raw >> 11);
            let g = expand_6_to_8(raw >> 5);
            let r = expand_5_to_8(raw);
            [r, g, b, 255]
        }
        PixelFormat::Rgb555 => {
            let r = expand_5_to_8(raw >> 10);
            let g = expand_5_to_8(raw >> 5);
            let b = expand_5_to_8(raw);
            [r, g, b, 255]
        }
        PixelFormat::Bgr555 => {
            let b = expand_5_to_8(raw >> 10);
            let g = expand_5_to_8(raw >> 5);
            let r = expand_5_to_8(raw);
            [r, g, b, 255]
        }
        PixelFormat::Indexed8 => [raw as u8, raw as u8, raw as u8, 255],
    }
}

/// Decodes up to 256 palette entries from a ROM slice.
pub fn decode_palette(data: &[u8], format: PaletteFormat, big_endian: bool) -> [[u8; 4]; 256] {
    let mut palette = default_grayscale_palette();
    match format {
        PaletteFormat::Rgb565 | PaletteFormat::Rgb555 => {
            let pf = if format == PaletteFormat::Rgb565 {
                PixelFormat::Rgb565
            } else {
                PixelFormat::Rgb555
            };
            for (i, chunk) in data.chunks_exact(2).take(256).enumerate() {
                let raw = if big_endian {
                    u16::from_be_bytes([chunk[0], chunk[1]])
                } else {
                    u16::from_le_bytes([chunk[0], chunk[1]])
                };
                palette[i] = decode_16bit_pixel(raw, pf, None);
            }
        }
        PaletteFormat::Rgb888 => {
            for (i, chunk) in data.chunks_exact(3).take(256).enumerate() {
                palette[i] = [chunk[0], chunk[1], chunk[2], 255];
            }
        }
    }
    palette
}

pub fn default_grayscale_palette() -> [[u8; 4]; 256] {
    let mut pal = [[0, 0, 0, 255]; 256];
    for (i, entry) in pal.iter_mut().enumerate() {
        let v = i as u8;
        *entry = [v, v, v, 255];
    }
    pal
}

/// Decodes raw ROM pixel bytes into an `RgbaImage`.
pub fn decode_raw_image(
    data: &[u8],
    opts: &PreviewOptions,
    palette: Option<&[[u8; 4]; 256]>,
) -> Result<RgbaImage, RomToolError> {
    if opts.width == 0 || opts.height == 0 {
        return Err(RomToolError::InvalidDimensions {
            width: opts.width,
            height: opts.height,
        });
    }

    let bpp = opts.format.bytes_per_pixel();
    let row_bytes = (opts.width as usize) * bpp;
    let stride = opts.stride.unwrap_or(row_bytes).max(row_bytes);
    let required = stride
        .saturating_mul((opts.height as usize).saturating_sub(1))
        .saturating_add(row_bytes);

    if data.len() < required {
        return Err(RomToolError::OutOfBounds {
            offset: 0,
            length: required,
            rom_size: data.len(),
        });
    }

    let fallback_pal = default_grayscale_palette();
    let pal = palette.unwrap_or(&fallback_pal);

    let mut img: RgbaImage = ImageBuffer::new(opts.width, opts.height);

    for y in 0..opts.height {
        let row_start = (y as usize) * stride;
        let row_slice = &data[row_start..row_start + row_bytes];

        for x in 0..opts.width {
            let rgba = match opts.format {
                PixelFormat::Indexed8 => {
                    let idx = row_slice[x as usize];
                    if opts.transparent_key == Some(idx as u16) {
                        [0, 0, 0, 0]
                    } else {
                        pal[idx as usize]
                    }
                }
                _ => {
                    let px_off = (x as usize) * 2;
                    let raw = if opts.big_endian {
                        u16::from_be_bytes([row_slice[px_off], row_slice[px_off + 1]])
                    } else {
                        u16::from_le_bytes([row_slice[px_off], row_slice[px_off + 1]])
                    };
                    decode_16bit_pixel(raw, opts.format, opts.transparent_key)
                }
            };
            img.put_pixel(x, y, Rgba(rgba));
        }
    }

    let scale = opts.scale.max(1);
    if scale > 1 {
        Ok(upscale_nearest(&img, scale))
    } else {
        Ok(img)
    }
}

fn upscale_nearest(src: &RgbaImage, scale: u32) -> RgbaImage {
    let w = src.width() * scale;
    let h = src.height() * scale;
    let mut dst = ImageBuffer::new(w, h);
    for y in 0..h {
        let sy = y / scale;
        for x in 0..w {
            let sx = x / scale;
            dst.put_pixel(x, y, *src.get_pixel(sx, sy));
        }
    }
    dst
}

pub fn render_palette_swatch(palette: &[[u8; 4]; 256], cell_size: u32) -> RgbaImage {
    let cell = cell_size.max(1);
    let dim = 16 * cell;
    let mut img: RgbaImage = ImageBuffer::new(dim, dim);
    for idx in 0..256u32 {
        let gx = idx % 16;
        let gy = idx / 16;
        let color = Rgba(palette[idx as usize]);
        for dy in 0..cell {
            for dx in 0..cell {
                img.put_pixel(gx * cell + dx, gy * cell + dy, color);
            }
        }
    }
    img
}

pub fn save_png(img: &RgbaImage, output_path: &Path) -> Result<(), image::ImageError> {
    img.save(output_path)
}

// ============================================================================
// Lifted XINA / Pinball 2000 Animation & Movie Decompressors
// ============================================================================

/// Converts an RGB555 (`u16`) framebuffer into an `RgbaImage`.
/// Returns `true` if `px` matches the Pinball 2000 `0x7C1F` magenta transparency key
/// (including slightly quantized near-magenta such as `0x7CDF` in `anim_watto_scene_cube`).
#[inline]
pub const fn is_p2k_transparent_rgb555(px: u16) -> bool {
    if px == P2K_TRANSPARENT_KEY_RGB555 {
        return true;
    }
    let r = (px >> 10) & 0x1F;
    let g = (px >> 5) & 0x1F;
    let b = px & 0x1F;
    r >= 30 && g <= 6 && b >= 30
}

pub fn rgb555_canvas_to_rgba(
    canvas: &[u16],
    width: u32,
    height: u32,
    transparent_key: Option<u16>,
) -> RgbaImage {
    let mut img = ImageBuffer::new(width, height);
    let use_p2k_key = transparent_key == Some(P2K_TRANSPARENT_KEY_RGB555);
    for y in 0..height {
        let row_off = (y as usize) * (width as usize);
        for x in 0..width {
            let px = canvas[row_off + (x as usize)];
            let rgba = if use_p2k_key && is_p2k_transparent_rgb555(px) {
                [0, 0, 0, 0]
            } else {
                decode_16bit_pixel(px, PixelFormat::Rgb555, transparent_key)
            };
            img.put_pixel(x, y, Rgba(rgba));
        }
    }
    img
}

/// Decodes a single Pinball 2000 `Animation_Frame` into an existing RGB555 `canvas`
/// (`width * height` `u16` pixels), preserving untouched pixels for delta codecs.
pub fn decompress_p2k_frame_into_rgb555(
    comp_type: u32,
    width: u32,
    height: u32,
    palette_data: Option<&[u8]>,
    palette_entries: u32,
    frame_data: &[u8],
    canvas: &mut [u16],
) -> Result<(), RomToolError> {
    let w = width as usize;
    let h = height as usize;
    if w == 0 || h == 0 || canvas.len() < w * h {
        return Err(RomToolError::InvalidDimensions { width, height });
    }

    match comp_type {
        0 => decode_8bit_raw(w, h, palette_data, palette_entries, frame_data, canvas),
        1 => decode_8bit_l_rle(w, h, palette_data, palette_entries, frame_data, canvas),
        20 => decode_15bit_raw(w, h, frame_data, canvas),
        21 => decode_15bit_l_rle(w, h, frame_data, canvas),
        22 | 42 | 44 => decode_15bit_64col_dict_rle(
            w,
            h,
            comp_type == 42 || comp_type == 44,
            comp_type == 44,
            palette_data,
            frame_data,
            canvas,
        ),
        23 | 43 | 45 => decode_15bit_32col_dict_rle(
            w,
            h,
            comp_type == 43 || comp_type == 45,
            comp_type == 45,
            palette_data,
            frame_data,
            canvas,
        ),
        30 => decode_movie_blocks(w, h, frame_data, canvas),
        other => Err(RomToolError::InvalidFormat(format!(
            "unsupported Pinball 2000 frame compression code {other}"
        ))),
    }
}

fn build_8bit_pal_rgb555(palette_data: Option<&[u8]>, entries: u32) -> [u16; 256] {
    let mut pal = [0u16; 256];
    pal[0] = P2K_TRANSPARENT_KEY_RGB555;
    if let Some(bytes) = palette_data {
        let count = (entries as usize).min(256);
        for idx in 1..count {
            let off = idx * 3;
            if off + 2 < bytes.len() {
                let r = ((bytes[off] >> 1) as u16) & 0x1F;
                let g = ((bytes[off + 1] >> 1) as u16) & 0x1F;
                let b = ((bytes[off + 2] >> 1) as u16) & 0x1F;
                pal[idx] = (r << 10) | (g << 5) | b;
            }
        }
    }
    pal
}

fn decode_8bit_raw(
    w: usize,
    h: usize,
    palette_data: Option<&[u8]>,
    palette_entries: u32,
    data: &[u8],
    canvas: &mut [u16],
) -> Result<(), RomToolError> {
    if data.len() < w * h {
        return Err(RomToolError::InvalidFormat("truncated 8bit_raw frame".into()));
    }
    let pal = build_8bit_pal_rgb555(palette_data, palette_entries);
    for (dst, &idx) in canvas[..w * h].iter_mut().zip(&data[..w * h]) {
        *dst = pal[idx as usize];
    }
    Ok(())
}

fn decode_8bit_l_rle(
    w: usize,
    h: usize,
    palette_data: Option<&[u8]>,
    palette_entries: u32,
    data: &[u8],
    canvas: &mut [u16],
) -> Result<(), RomToolError> {
    let pal = build_8bit_pal_rgb555(palette_data, palette_entries);
    let mut pos = 0usize;
    for y in 0..h {
        let row = &mut canvas[y * w..(y + 1) * w];
        let mut x = 0usize;
        while x < w && pos < data.len() {
            let ctrl = data[pos];
            pos += 1;
            let count = (ctrl & 0x7F) as usize;
            if (ctrl & 0x80) == 0 {
                if pos >= data.len() {
                    break;
                }
                let px = pal[data[pos] as usize];
                pos += 1;
                let end = (x + count).min(w);
                row[x..end].fill(px);
                x += count;
            } else {
                for _ in 0..count {
                    if pos >= data.len() {
                        break;
                    }
                    if x < w {
                        row[x] = pal[data[pos] as usize];
                    }
                    pos += 1;
                    x += 1;
                }
            }
        }
    }
    Ok(())
}

fn decode_15bit_raw(
    w: usize,
    h: usize,
    data: &[u8],
    canvas: &mut [u16],
) -> Result<(), RomToolError> {
    if data.len() < w * h * 2 {
        return Err(RomToolError::InvalidFormat("truncated 15bit_raw frame".into()));
    }
    for (dst, chunk) in canvas[..w * h].iter_mut().zip(data.chunks_exact(2)) {
        *dst = u16::from_le_bytes([chunk[0], chunk[1]]) & 0x7FFF;
    }
    Ok(())
}

/// Lifted from `decompress_15bit_image_l_rle_to_15bit` (`0x002a0f10`).
fn decode_15bit_l_rle(
    w: usize,
    h: usize,
    data: &[u8],
    canvas: &mut [u16],
) -> Result<(), RomToolError> {
    let mut pos = 0usize;
    for y in 0..h {
        let row = &mut canvas[y * w..(y + 1) * w];
        let mut x = 0usize;
        while x < w && pos + 1 < data.len() {
            let ctrl = u16::from_le_bytes([data[pos], data[pos + 1]]);
            pos += 2;
            let count = (ctrl & 0x7FFF) as usize;
            if (ctrl & 0x8000) == 0 {
                if pos + 1 >= data.len() {
                    break;
                }
                let px = u16::from_le_bytes([data[pos], data[pos + 1]]) & 0x7FFF;
                pos += 2;
                let end = (x + count).min(w);
                row[x..end].fill(px);
                x += count;
            } else {
                for _ in 0..count {
                    if pos + 1 >= data.len() {
                        break;
                    }
                    let px = u16::from_le_bytes([data[pos], data[pos + 1]]) & 0x7FFF;
                    pos += 2;
                    if x < w {
                        row[x] = px;
                    }
                    x += 1;
                }
            }
        }
    }
    Ok(())
}

struct BitReader32<'a> {
    data: &'a [u8],
    pos: usize,
    cur: u32,
    bits_left: u32,
}

impl<'a> BitReader32<'a> {
    fn new(data: &'a [u8], start: usize) -> Self {
        let mut s = Self {
            data,
            pos: start,
            cur: 0,
            bits_left: 0,
        };
        s.refill_dword();
        s
    }

    fn refill_dword(&mut self) {
        let mut b = [0u8; 4];
        for slot in &mut b {
            if self.pos < self.data.len() {
                *slot = self.data[self.pos];
                self.pos += 1;
            }
        }
        self.cur = u32::from_le_bytes(b);
        self.bits_left = 32;
    }

    #[inline]
    fn next_bit(&mut self) -> bool {
        if self.bits_left == 0 {
            self.refill_dword();
        }
        let bit = (self.cur & 1) != 0;
        self.cur >>= 1;
        self.bits_left -= 1;
        bit
    }
}

/// Lifted from `do_15bit_64col_dict_rle` (`0x002a1100`).
fn decode_15bit_64col_dict_rle(
    w: usize,
    h: usize,
    has_row_mask: bool,
    is_run_delta: bool,
    palette_data: Option<&[u8]>,
    data: &[u8],
    canvas: &mut [u16],
) -> Result<(), RomToolError> {
    let pal_bytes = palette_data.ok_or_else(|| {
        RomToolError::InvalidFormat("64col_dict_rle requires 384-byte palette".into())
    })?;
    if pal_bytes.len() < 384 || data.len() < 2 {
        return Err(RomToolError::InvalidFormat(
            "truncated 64col_dict_rle palette or frame header".into(),
        ));
    }

    let mut dict64 = [0u16; 64];
    for (i, chunk) in pal_bytes[..128].chunks_exact(2).enumerate() {
        dict64[i] = u16::from_le_bytes([chunk[0], chunk[1]]);
    }
    let mut pair64 = [(0u16, 0u16); 64];
    for (i, chunk) in pal_bytes[128..384].chunks_exact(4).enumerate() {
        pair64[i] = (
            u16::from_le_bytes([chunk[0], chunk[1]]),
            u16::from_le_bytes([chunk[2], chunk[3]]),
        );
    }

    let cmd_rel = u16::from_le_bytes([data[0], data[1]]) as usize;
    if cmd_rel > data.len() {
        return Err(RomToolError::InvalidFormat(
            "64col_dict_rle cmd offset out of bounds".into(),
        ));
    }

    let mut pair_bits = BitReader32::new(data, 2);
    let mut cmd_pos = cmd_rel;

    let mut row_mask_bits = if has_row_mask {
        let mask_words = (h + 31) / 32;
        let mask_start = cmd_pos;
        cmd_pos = cmd_pos.saturating_add(mask_words * 4);
        Some(BitReader32::new(data, mask_start))
    } else {
        None
    };

    for y in 0..h {
        if let Some(ref mut rm) = row_mask_bits {
            // Bit = 1 skips the unchanged scanline
            if rm.next_bit() {
                continue;
            }
        }
        let row = &mut canvas[y * w..(y + 1) * w];
        let mut x = 0usize;
        while x < w && cmd_pos < data.len() {
            let b = data[cmd_pos];
            cmd_pos += 1;
            let idx = (b & 0x3F) as usize;
            match b & 0xC0 {
                0x00 => {
                    row[x] = dict64[idx];
                    x += 1;
                }
                0x40 => {
                    if is_run_delta {
                        x += idx + 2;
                    } else {
                        let px = dict64[idx];
                        row[x] = px;
                        if x + 1 < w {
                            row[x + 1] = px;
                        }
                        x += 2;
                    }
                }
                0x80 => {
                    let (p0, p1) = pair64[idx];
                    let (first, second) = if pair_bits.next_bit() {
                        (p0, p1)
                    } else {
                        (p1, p0)
                    };
                    row[x] = first;
                    if x + 1 < w {
                        row[x + 1] = second;
                    }
                    x += 2;
                }
                _ => {
                    if cmd_pos >= data.len() {
                        break;
                    }
                    let count = (data[cmd_pos] as usize) + 3;
                    cmd_pos += 1;
                    let px = dict64[idx];
                    let end = (x + count).min(w);
                    row[x..end].fill(px);
                    x += count;
                }
            }
        }
    }

    Ok(())
}

/// Lifted from `do_15bit_32col_dict_rle` (`0x002a1b8c`).
fn decode_15bit_32col_dict_rle(
    w: usize,
    h: usize,
    has_row_mask: bool,
    is_run_delta: bool,
    palette_data: Option<&[u8]>,
    data: &[u8],
    canvas: &mut [u16],
) -> Result<(), RomToolError> {
    let pal_bytes = palette_data.ok_or_else(|| {
        RomToolError::InvalidFormat("32col_dict_rle requires 576-byte palette".into())
    })?;
    if pal_bytes.len() < 576 || data.len() < 2 {
        return Err(RomToolError::InvalidFormat(
            "truncated 32col_dict_rle palette or frame header".into(),
        ));
    }

    let mut dict32 = [0u16; 32];
    for (i, chunk) in pal_bytes[..64].chunks_exact(2).enumerate() {
        dict32[i] = u16::from_le_bytes([chunk[0], chunk[1]]);
    }
    let mut pair128 = [(0u16, 0u16); 128];
    for (i, chunk) in pal_bytes[64..576].chunks_exact(4).enumerate() {
        pair128[i] = (
            u16::from_le_bytes([chunk[0], chunk[1]]),
            u16::from_le_bytes([chunk[2], chunk[3]]),
        );
    }

    let cmd_rel = u16::from_le_bytes([data[0], data[1]]) as usize;
    if cmd_rel > data.len() {
        return Err(RomToolError::InvalidFormat(
            "32col_dict_rle cmd offset out of bounds".into(),
        ));
    }

    let mut pair_bits = BitReader32::new(data, 2);
    let mut cmd_pos = cmd_rel;

    let mut row_mask_bits = if has_row_mask {
        let mask_words = (h + 31) / 32;
        let mask_start = cmd_pos;
        cmd_pos = cmd_pos.saturating_add(mask_words * 4);
        Some(BitReader32::new(data, mask_start))
    } else {
        None
    };

    for y in 0..h {
        if let Some(ref mut rm) = row_mask_bits {
            if rm.next_bit() {
                continue;
            }
        }
        let row = &mut canvas[y * w..(y + 1) * w];
        let mut x = 0usize;
        while x < w && cmd_pos < data.len() {
            let b = data[cmd_pos];
            cmd_pos += 1;
            if b >= 0x80 {
                let (p0, p1) = pair128[(b & 0x7F) as usize];
                let (first, second) = if pair_bits.next_bit() {
                    (p0, p1)
                } else {
                    (p1, p0)
                };
                row[x] = first;
                if x + 1 < w {
                    row[x + 1] = second;
                }
                x += 2;
            } else {
                let idx = (b & 0x1F) as usize;
                match b & 0xE0 {
                    0x00 => {
                        row[x] = dict32[idx];
                        x += 1;
                    }
                    0x20 => {
                        if is_run_delta {
                            x += idx + 2;
                        } else {
                            let px = dict32[idx];
                            row[x] = px;
                            if x + 1 < w {
                                row[x + 1] = px;
                            }
                            x += 2;
                        }
                    }
                    0x40 => {
                        let count = idx + 3;
                        let px = dict32[0];
                        let end = (x + count).min(w);
                        row[x..end].fill(px);
                        x += count;
                    }
                    _ => {
                        // 0x60: long RLE
                        if cmd_pos >= data.len() {
                            break;
                        }
                        let count = (data[cmd_pos] as usize) + 3;
                        cmd_pos += 1;
                        let px = dict32[idx];
                        let end = (x + count).min(w);
                        row[x..end].fill(px);
                        x += count;
                    }
                }
            }
        }
    }

    Ok(())
}

/// Lifted from `decompress_movie_2x1_double` / `decompress_movie_1x1` (`0x0029c720`).
/// Decodes 4×4 RGB555 blocks at native `(width × height)` resolution.
fn decode_movie_blocks(
    w: usize,
    h: usize,
    data: &[u8],
    canvas: &mut [u16],
) -> Result<(), RomToolError> {
    let mut bitbuf: u64 = 0;
    let mut bits_in_buf: u32 = 0;
    let mut pos: usize = 0;

    let refill = |bitbuf: &mut u64, bits_in_buf: &mut u32, pos: &mut usize| {
        while *bits_in_buf <= 32 && *pos < data.len() {
            *bitbuf |= (data[*pos] as u64) << *bits_in_buf;
            *pos += 1;
            *bits_in_buf += 8;
        }
    };

    let take_bits = |n: u32, bitbuf: &mut u64, bits_in_buf: &mut u32, pos: &mut usize| -> u32 {
        refill(bitbuf, bits_in_buf, pos);
        let mask = if n == 32 { u32::MAX as u64 } else { (1u64 << n) - 1 };
        let val = (*bitbuf & mask) as u32;
        *bitbuf >>= n;
        *bits_in_buf = bits_in_buf.saturating_sub(n);
        val
    };

    let mut by = 0usize;
    while by + 4 <= h {
        let mut bx = 0usize;
        while bx + 4 <= w {
            let op = take_bits(2, &mut bitbuf, &mut bits_in_buf, &mut pos);
            match op {
                1 => {
                    // Inter-frame skip: leave 4x4 block unchanged
                }
                2 => {
                    // Solid 15-bit color block
                    let c = (take_bits(15, &mut bitbuf, &mut bits_in_buf, &mut pos) & 0x7FFF) as u16;
                    for dy in 0..4 {
                        let row = &mut canvas[(by + dy) * w + bx..(by + dy) * w + bx + 4];
                        row.fill(c);
                    }
                }
                3 => {
                    // 2-color 4x4 block
                    let r0 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let r1 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let g0 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let g1 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let b0 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let b1 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let colors = [
                        (r0 << 10) | (g0 << 5) | b0,
                        (r1 << 10) | (g1 << 5) | b1,
                    ];
                    let mask16 = take_bits(16, &mut bitbuf, &mut bits_in_buf, &mut pos);
                    for dy in 0..4 {
                        for dx in 0..4 {
                            let idx = ((mask16 >> (dy * 4 + dx)) & 1) as usize;
                            canvas[(by + dy) * w + bx + dx] = colors[idx];
                        }
                    }
                }
                _ => {
                    // op == 0: 4-color linearly interpolated 4x4 block
                    let r0 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let r3 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let g0 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let g3 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let b0 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;
                    let b3 = take_bits(5, &mut bitbuf, &mut bits_in_buf, &mut pos) as u16;

                    let r1 = (2 * r0 + r3) / 3;
                    let r2 = (r0 + 2 * r3) / 3;
                    let g1 = (2 * g0 + g3) / 3;
                    let g2 = (g0 + 2 * g3) / 3;
                    let b1 = (2 * b0 + b3) / 3;
                    let b2 = (b0 + 2 * b3) / 3;

                    let colors = [
                        (r0 << 10) | (g0 << 5) | b0,
                        (r1 << 10) | (g1 << 5) | b1,
                        (r2 << 10) | (g2 << 5) | b2,
                        (r3 << 10) | (g3 << 5) | b3,
                    ];
                    let mask32 = take_bits(32, &mut bitbuf, &mut bits_in_buf, &mut pos);
                    for dy in 0..4 {
                        for dx in 0..4 {
                            let shift = (dy * 4 + dx) * 2;
                            let idx = ((mask32 >> shift) & 3) as usize;
                            canvas[(by + dy) * w + bx + dx] = colors[idx];
                        }
                    }
                }
            }
            bx += 4;
        }
        by += 4;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_rgb565_primaries_and_transparency() {
        let mut raw = Vec::new();
        for px in [0xF800u16, 0x07E0, 0x001F, 0xF81F] {
            raw.extend_from_slice(&px.to_le_bytes());
        }

        let opts = PreviewOptions {
            width: 2,
            height: 2,
            format: PixelFormat::Rgb565,
            stride: None,
            big_endian: false,
            transparent_key: Some(0xF81F),
            scale: 1,
        };

        let img = decode_raw_image(&raw, &opts, None).unwrap();
        assert_eq!(img.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(img.get_pixel(1, 0).0, [0, 255, 0, 255]);
        assert_eq!(img.get_pixel(0, 1).0, [0, 0, 255, 255]);
        assert_eq!(img.get_pixel(1, 1).0, [0, 0, 0, 0]);
    }

    #[test]
    fn decodes_rgb555_white_and_red() {
        assert_eq!(
            decode_16bit_pixel(0x7C00, PixelFormat::Rgb555, None),
            [255, 0, 0, 255]
        );
        assert_eq!(
            decode_16bit_pixel(0x7FFF, PixelFormat::Rgb555, None),
            [255, 255, 255, 255]
        );
    }

    #[test]
    fn decodes_indexed8_with_rgb555_palette() {
        let mut pal_bytes = Vec::new();
        pal_bytes.extend_from_slice(&0x0000u16.to_le_bytes());
        pal_bytes.extend_from_slice(&0x03E0u16.to_le_bytes());
        let palette = decode_palette(&pal_bytes, PaletteFormat::Rgb555, false);

        let pixels = [0u8, 1u8];
        let opts = PreviewOptions {
            width: 2,
            height: 1,
            format: PixelFormat::Indexed8,
            stride: None,
            big_endian: false,
            transparent_key: Some(0),
            scale: 1,
        };

        let img = decode_raw_image(&pixels, &opts, Some(&palette)).unwrap();
        assert_eq!(img.get_pixel(0, 0).0, [0, 0, 0, 0]);
        assert_eq!(img.get_pixel(1, 0).0, [0, 255, 0, 255]);
    }

    #[test]
    fn decodes_15bit_l_rle_runs() {
        // 4x1 row: repeat 3x 0x7C00 (red), literal 1x 0x001F (blue)
        let mut stream = Vec::new();
        stream.extend_from_slice(&3u16.to_le_bytes());
        stream.extend_from_slice(&0x7C00u16.to_le_bytes());
        stream.extend_from_slice(&(0x8000u16 | 1).to_le_bytes());
        stream.extend_from_slice(&0x001Fu16.to_le_bytes());

        let mut canvas = [0u16; 4];
        decompress_p2k_frame_into_rgb555(21, 4, 1, None, 0, &stream, &mut canvas).unwrap();
        assert_eq!(canvas, [0x7C00, 0x7C00, 0x7C00, 0x001F]);
    }

    #[test]
    fn rgb555_canvas_to_rgba_keys_quantized_magenta() {
        // 0x7C1F = exact magenta (255,0,255), 0x7CDF = quantized magenta (255,49,255), 0x7C00 = red
        let canvas = [P2K_TRANSPARENT_KEY_RGB555, 0x7CDF, 0x7C00];
        let img = rgb555_canvas_to_rgba(&canvas, 3, 1, Some(P2K_TRANSPARENT_KEY_RGB555));
        assert_eq!(img.get_pixel(0, 0).0, [0, 0, 0, 0]);
        assert_eq!(img.get_pixel(1, 0).0, [0, 0, 0, 0]);
        assert_eq!(img.get_pixel(2, 0).0, [255, 0, 0, 255]);
    }
}
