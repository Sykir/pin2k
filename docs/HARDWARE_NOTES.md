# Pinball 2000 / Star Wars Episode I — Verified Hardware & Protocol Reference

Clean-room hardware documentation verified against `swe1_0200` x86 disassembly (`0x00100000`) and PRISM/MediaGX/PDB specifications.

---

## 1. PRISM & MediaGX Physical Memory Map

| Range | Size | Region | Notes |
|---|---:|---|---|
| `0x00000000–0x00FFFFFF` | 16 MiB | System RAM | `0x00080000` PRISM reset copy; `0x00100000` loaded `game.rom`; `0x00800000–0x00BFFFFF` framebuffer backing |
| `0x11000000–0x1102FFFF` | 192 KiB | BAR2 SRAM/NVRAM | Persistent audits, adjustments, high scores |
| `0x12000000–0x123FFFFF` | 4 MiB | BAR3 Update Flash | `bootdata` (`+0x0000`), `im_flsh0` (`+0x8000`), `game`, `symbols` |
| `0x13000000–0x13FFFFFF` | 16 MiB | BAR4 DCS MMIO | DCS-2 ADSP-2105 command/status transport |
| `0x14000000–0x17FFFFFF` | 64 MiB | BAR5 Mask ROMs | 4 × 16 MiB banks (`u100/u101`, `u102/u103`, `u104/u105`, `u106/u107`), 16-bit word-interleaved |
| `0x18000000–0x187FFFFF` | 8 MiB | Expansion ROM | DCS sound ROM (`u109/u110`, 4 MiB each, 16-bit word-interleaved) |
| `0x40000000–0x40FFFFFF` | 16 MiB | MediaGX GX | `0x40008100–0x4000820F` 2D blitter; `0x40800000–0x40BFFFFF` framebuffer alias |

---

## 2. Display & Compressed Graphics Formats

- **Display geometry:** `640×240` RGB555 (`15-bit`), scaled 2× vertically to `640×480` on the cabinet CRT for Pepper's Ghost playfield reflection; transparent key = `0x7C1F` (magenta).
- **ROM `OSBitmap` header (20 bytes):** `width: u16`, `height: u16`, `stride_or_size: u32`, `format: u8`, `flags: u8`, `pad: u16`, `palette_va: u32`, `pixels_va: u32` (`va - 0x14000000` indexes `prism_flat.bin`).
- **Supported codecs in `pin2k-rom-tool`:**
  - `0`: Raw 15-bit RGB555
  - `1`: Raw 8-bit indexed (`RGB555` palette)
  - `20`, `21`: Row-table 15-bit literal/transparent skip RLE (`L_RLE`)
  - `22`, `23`: Row-table 8-bit indexed skip + run/literal RLE (`B_RLE`)
  - `30`, `42`, `43`, `44`, `45`: Packed row-stream & Nibble-RLE (`NRLE`) 4-bit / 8-bit / 15-bit movie codecs

---

## 3. Power Driver Board (PDB) Parallel-Port Wire Protocol (`pin2k-hal`)

The Pinball 2000 PC communicates with the cabinet Power Driver Board over an IEEE-1284 SPP parallel port (`DATA = base+0`, `STATUS = base+1`, `CONTROL = base+2`, `/dev/parportN` via `ppdev`).

### Bus Transactions (`PinIOWrite` @ `0x001a8360`, `PinIORead` @ `0x001a8450`)

- **Presence check (`PinIOWork_16ms` @ `0x001a8540`):** `(inb(STATUS) & 0xF8) != 0x78` (`0x87` = connected PDB, `0x7F` = disconnected open bus).
- **`PinIOWrite(reg, val)`:**
  1. `DATA = reg`
  2. `CONTROL = 0x00`
  3. `CONTROL = 0x04` *(rising edge on bit 2 latches `reg` as active opcode)*
  4. `DATA = val`
  5. `CONTROL = 0x05` *(bit 0 high)*
  6. `CONTROL = 0x04` *(falling edge on bit 0 dispatches `val` into register `reg`)*
- **`PinIORead(reg)`:**
  1. `DATA = reg`
  2. `CONTROL = 0x00`
  3. `CONTROL = 0x04` *(latch `reg`)*
  4. `CONTROL = 0x2D` *(bit 5 reverse data direction + bits 3, 2, 0 gated read)*
  5. `val = DATA`
  6. `CONTROL = 0x04` *(restore forward idle)*

### PDB Register Map

| Opcode | Dir | Function |
|---|---|---|
| `0x00` | R | Direct coin switches (`Logical Col 8`: Left, Center, Right, 4th Coin) |
| `0x01` | R | Direct cabinet switches (`Logical Col 10`: bit 0 Slam Tilt, bit 1 Coin Door Closed interlock, bit 2 Plumb Tilt, bit 4/5 R/L Flippers, bit 6/7 R/L Action Buttons) |
| `0x02` | R | Board status nibble (`0xF0`) |
| `0x03` | R | Direct service & EOS switches (`Logical Col 9`: Escape, Down, Up, Enter, R/L Flipper EOS) |
| `0x04` | R | 8×8 Playfield Switch Matrix row input for column selected in `0x05` |
| `0x05` | W | 8×8 Playfield Switch Matrix one-hot column strobe (`1 << col`, `col = 0..7`) |
| `0x06` | W | Lamp Matrix A (`lamps 0..63`) row staging byte |
| `0x07` | W | Lamp Matrix B (`lamps 64..127`) row staging byte |
| `0x08` | W | Dual Lamp Matrix one-hot column strobe (`0x00` blank before `1 << col`) |
| `0x0B` | W | Solenoid/Drive Bank 0 (`Drives 0..7`: Saucers, Drop Targets, Magnet, Shield Popper) |
| `0x0A` | W | Solenoid/Drive Bank 1 (`Drives 8..15`: Trough Eject, Slings, Jets, Right Saucer) |
| `0x09` | W | Flasher Bank 2 (`Drives 16..23`: Playfield & Backpanel Flashers) |
| `0x0D` | W | Flasher Bank 3 (`Drives 24..31`: Backpanel Flashers) |
| `0x0C` | W | Flipper/Shield Bank 4 (`Drives 32..39`: R/L Flipper Power+Hold, Shield Power+Hold, Laser Flashers) |
| `0x0E` | W | Aux Bank 5 (`Drives 40..47`: Lightsaber Neon = Drive 40, Ticket Dispenser = Drive 47) |
| `0x10`, `0x11` | R | Playfield diode-matrix signature probe (`SWE1` vs `RFM`) & ready/busy toggle |
| `0x12`, `0x13` | R | Solenoid fuse fault registers (`0x00` = all fuses intact) |

### SWE1 Active-Low Opto-Interrupters (`D_NORM_CLOSED = 0x0800`)

In SWE1's `SwitchTable` (`0x002df6b4`), 9 playfield switches are infrared opto-interrupters (`Sw 24..30` Trough Jam/Balls 1–4 & Ramp Enters in Col 3 = `0x7F`; `Sw 32, 39` Shield Popper & Shield Hit in Col 4 = `0x81`). Unobstructed IR beams read `1` on register `0x04` and drop to `0` when blocked by a ball; `PdbDriver` XORs with `SWE1_OPTO_NC_MASK` so physical `/dev/parport0` cabinets and `VirtualPdbPort` share identical electrical levels.

---

## 4. ROM Binary Table & Font Layouts (`swe1_0200.elf` / `rfm_0180.elf`)

Verified directly from `game.rom` x86 disassembly (`Font::get_character_pointer` @ `0x0029862C`, `Font::plot_character_ptr` @ `0x002986C4`):

- **`FontData` (24 bytes, referenced by `font_*_data_ptr`):**
  - `+0x00`: `num_chars: u32`
  - `+0x04`: `ascent: u32`
  - `+0x08`: `line_height: u32`
  - `+0x0C`: `num_colors: u32`
  - `+0x10`: `default_spacing: i32`
  - `+0x14`: `chars_ptr: u32` → array of `num_chars` × `Character`
- **`Character` (28 bytes):**
  - `+0x00`: `code: u8` (+ 3 bytes padding)
  - `+0x04`: `x_offset: i32`
  - `+0x08`: `y_offset: i32`
  - `+0x0C`: `width: u32`
  - `+0x10`: `height: u32`
  - `+0x14`: `advance_extra: i32` (uses `FontData.default_spacing` when `0`)
  - `+0x18`: `pixels_ptr: u32` (`width × height` indexed bytes; `0` = transparent, `1..=num_colors` = glyph shade)
- **Hardware tables in `.data`:**
  - `DriveTable` (48 × 24 bytes): `+0x00 flags`, `+0x04 default_pulse_ms`, `+0x08..+0x14` EN/DE/FR/ES name pointers.
  - `LampTable` (128 × 36 bytes): `+0x00 flags`, `+0x14..+0x20` EN/DE/FR/ES name pointers.
  - `SwitchTable` (96 × 48 bytes): `+0x04 sw_type`, `+0x08 handler_addr`, `+0x0C arg`, `+0x10 debounce_ms`, `+0x20..+0x2C` EN/DE/FR/ES name pointers.
