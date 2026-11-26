# Roadmap, Playability Score & TODO (`pin2k`)

Progress tracker for the native Rust **Pinball 2000** port (*Star Wars Episode I* `#50069` & *Revenge From Mars* `#50070`).

## Overall Progress: **~94%** | Playability Score: **93.5 / 100**

| Area | Progress | Status |
|---|---:|---|
| 1. ROM Tool & Extraction (`pin2k-rom-tool`) | **98%** | UI layouts + Gamecode + Assets + `upscale-assets` (`gfx_hd/`) + Ghidra 12.2 pipeline (`1,867` functions / `248` classes) |
| 2. Hardware Abstraction & PDB (`pin2k-hal`) | **98%** | Cabinet-ready, Serial UART bridge & Z-sorted compositor traits |
| 3. Game Engine & Native Screens (`pin2k-core`) | **95%** | 100% of SWE1 screens/scenes (unified Podrace), `ARD`/`ACD` audio bank, `Leff` & `FX` engines, complete `ScriptRuntime` (`player_vars` 1–4P, timers/modes) |
| 4. SWE1 Asset Coverage (`gfx` / `fonts` / `audio`) | **97%** | Fonts **100% (`19/19`)**, Audio **100% (`689/689`)**, Anims **90.3% (`354/392`)** |
| 5. Desktop Runtime, Debug Console & HD (`pin2k-desktop`) | **90%** | Native `--window`/`--fullscreen` (`winit`+`softbuffer`), `--hd` (`gfx_hd/`), native audio `StopAll`/looping + WebAudio mod overrides, 5-tab Web Console |
| 6. Asset Modding & Custom Scenes (`mod.json`) | **35%** | `gfx/` replacement & native + WebAudio overrides done; fonts, stateful rules & custom scenes remaining |
| 7. Documentation & Quality | **90%** | **44/44 unit tests passing**, **14/14 E2E video checks**, 0 warnings; full codebase & class reference documentation remaining |

### Playability Score: **93.5 / 100**

> *Native Desktop (`--window`/`--fullscreen`) & Web (`--web-port`): **~95 / 100** — Physical Cabinet (PDB): **~88 / 100***

| Playability Dimension | Weight | Score | Points | Current State |
|---|---:|---:|---:|---|
| **1. Game Loop & SWE1 Rules** | 25% | 94% | `23.5 / 25` | 1–4 players, Skill Shot, 14 scenes (unified Podrace), Super Jets, Combos, C-3PO, J-E-D-I, Multiballs, full `ScriptRuntime` |
| **2. Visual Fidelity & HUD (`640×240`)** | 20% | 92% | `18.4 / 20` | `DeffScoreDisplay`, `DeffLaserStatusBackground`, `DeffVSpinner`, 20 Attract slides, isolated intros, `354/392` anims, `19/19` fonts |
| **3. Audio & Sound Responsiveness** | 15% | 95% | `14.3 / 15` | `689/689` DCS-2 tracks (`382` `ARD` pools), OS `StopAll`/looping + WebAudio + mod `/audio/` overrides |
| **4. Controls, Latency & Display** | 15% | 92% | `13.8 / 15` | Native `--window` / `--fullscreen` 60 Hz (`4:3`, `F11`/`Alt+Enter`) + cabinet keyboard + Web Debug Console |
| **5. Lamps, Coils & PDB Hardware** | 15% | 90% | `13.5 / 15` | Wire-exact PDB protocol, `14` `Leff*`, NC optos, Linux LPT + Win/Linux Serial; long-term physical cabinet burn-in remaining |
| **6. Stability, Persistence & Diagnostics** | 10% | 100% | `10.0 / 10` | `pin2k_db.json` persistence, all Diagnostics/Admin screens, `43/43` unit tests + `14/14` E2E video passing |

---

## 1. ROM Tool & Extraction (`pin2k-rom-tool`) — **98%**
- [x] PRISM reconstruction (`U100..U107` → 64 MiB) and DCS (`U109/U110` → 8 MiB)
- [x] Official/community update bundle support (`.exe`, `.zip`, folders) + 12 hardware checksum verifications
- [x] Symbolized x86 ELF32 export (`wrap-elf`)
- [x] Complete animation extraction (`extract-assets`, all frames by default, codecs `0..45`)
- [x] **19 bitmap fonts** extraction (`FontData` / `Character` → PNG atlases + `fonts_manifest.json`)
- [x] DCS-2 audio extraction (`*_sound.bin` `pb2kslib` → `.ogg` + `sound_manifest.json`)
- [x] Gamecode extraction (`SwitchTable`, `DriveTable`, `LampTable`, x86 disassembler `lift_x86_calls` → `game_script.json`)
- [x] **Complete ROM UI layout extraction (`extract-layout`)**: pure-Rust ELF32/x86-32 parser (`layout_extractor.rs`) exporting `rom_layout_db.json` (138 `Deff*`, 14 scenes, 20 Attract slides, 9 player characters, 702 `ACD`, 383 `ARD`)
- [x] **HD sprite upscaling (`upscale-assets`)**: `upscaler.rs` (`Lanczos3`/`CatmullRom`/`Nearest` + transparent RGB edge-padding → `gfx_hd/gfx_manifest.json`)
- [x] Modular layout (`main.rs`, `cli.rs`, `commands.rs`, `layout_extractor.rs`, `upscaler.rs`)
- [ ] Extend `extract-gamecode` to export 100% of SWE1 & RFM scene rules into `game_script.json`
- [ ] Raw DCS-2 audio decoding directly from `u109`/`u110` (fallback without `*_sound.bin`)

---

## 2. Hardware Abstraction & PDB (`pin2k-hal`) — **98%**
- [x] Wire-exact Power Driver Board protocol (`PinIOWrite` / `PinIORead`, 96 switches, 48 coils, 128 lamps)
- [x] Active-low opto-interrupter polarity (`SWE1_OPTO_NC_MASK`) and coin door high-voltage interlock
- [x] Initial `hw.sync(0)` scan before seeding `SwitchDebouncer` (eliminates spurious frame-2 edges)
- [x] Z-sorted `VideoBackend` trait (`SpriteDrawCmd`, `TextDrawCmd`, `RectDrawCmd`, `TextAlign`, `monochrome_rgb555`, `flip_x`, `scale_x/y`)
- [x] Linux parallel port (`/dev/parportN` `ppdev`, `/dev/port`), `VirtualPdbPort` + `--lpt-trace`
- [x] Cross-platform Serial UART bridge for Windows (`COMx`) & Linux (`/dev/ttyUSB0`) with zero C dependencies
- [ ] Long-term physical field validation on real PDB hardware + UART microcontroller bridge

---

## 3. Game Engine & Native Screens (`pin2k-core`) — **95%**
- [x] Modular 60 Hz architecture (`machine/{mod, leff, fx/*, score_display, hud_display, vspinner, attract_render, scene_render, system_screens, render, script_exec, swe1_playfield, rfm_playfield, admin_nav, tests}.rs`)
- [x] **Complete ROM-extracted audio bank (`swe1_audio.rs`)**: `382` `ARD` pools (`407` unique DCS-2 IDs) + `695` `ACD` descriptors with `resolve_ard()`, `resolve_acd()`, and `Pin2kMachine::emit_ard()`
- [x] **Lamp/flasher choreography engine (`machine/leff.rs`, 14 `LeffKind`)** & **modular FX/overlay engine (`machine/fx/`)**
- [x] **SWE1 rules ported from Ghidra 12.2 decompilation** (`1,867` functions / `248` classes from `swe1_0200.elf`): progressive `SkillShot`, `G-U-N-G-A-N` Super Jets, Ramp Combos, `BuildC3P0`, `JediLetters`, `Multiball`, `JediMultiball`, `QuestionMark`, `R2D2` Hurry-Up, and authentic per-scene `start_ard`/`hit_ard`/`complete_ard` & scoring
- [x] Authentic HUD: `DeffScoreDisplay` (9 unique Star Wars characters, comma-formatted scores, lightsabers) & `DeffLaserStatusBackground` (Naboo ship + `LASERS`/`TORPEDOS`)
- [x] Ramp-only `VirtualSpinners` (`DeffVSpinner`) with 4 rotating turnstile DCS cues (`0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`)
- [x] **Complete Attract Mode (20 `AttractDeff` screens)** including `DeffAttractModeKahunaLogo` + 9-color RGB555 light flare
- [x] **All 14 SWE1 scenes** (strictly isolated 2× bezel intros + active gameplay backgrounds + unified Podrace), `DeffObiHolo`, `HangarMultiball`, `DarthMaulDuel`, `Super Skill Shot`
- [ ] **Authentic ROM-lifted per-scene gameplay logic (`machine/scenes/`)**: port exact state machines, reward tables, lane-to-switch mappings, timers, and `(x, y, z)` animation choreography from `swe1_0200.elf`:
  - [x] `JarJarJuggle` (`JarjarJuggleScene` + `DeffJarjarJuggleScene`: 5 lanes, `ra_juggle_random_award_table`, `junk_table`, `Juggle1/2 -> Fall1/2 -> JumpUp -> Switch -> Stare`, ballistic fly-off & flashing award text)
  - [ ] Remaining 13 SWE1 scenes (`BattleDroid`, `DestroyerDroid`, `FederationFighter`, `GunganBattle`, `HangarEscape`, `HoverTank`, `Podrace`, `ProbeDroid`, `QueenFashion`, `WattosChance`, `MosEspaCantina`, `R2D2`, `QuestionMark`)
- [x] **All special screens & Diagnostics**: `Boot`, 12 `SpecialScreen` variants, `DeffTilt`, `DeffHSTDinitials`, and `DiagPage` (`Align`, `ColorBar`, `Grid`, `Rgb`, `Fuse`, `Switch`, `Coil`, `Lamp`, Operator menus)
- [x] **Complete `ScriptRuntime` wiring** (`script_engine/{mod,schema}.rs`): `OnBallStart`, `OnBallEnd`, `OnTick`, `OnTimer`, `OnModeExit`, `tick_attract()` under `--strict-script`, and isolated 1–4P `player_vars`
- [x] **Unified scene compositing in `pin2k-core`**: `render_podrace_layers` moved to `fx/scene_extras.rs` and legacy duplicate `scenes_compositor.rs` deleted
- [ ] **100% Data-Driven mode**: migrate all remaining visual/rule definitions to `game_script.json` / `rom_layout_db.json` and enable `--strict-script` by default

---

## 4. SWE1 Asset Coverage ([ASSETS_MATRIX.md](ASSETS_MATRIX.md)) — **97%**
- [x] **Bitmap Fonts (`fonts/`)**: **19 / 19 (100%)** loaded and rendered with drop shadows & alignment
- [x] **DCS-2 Audio Tracks (`audio/`)**: **689 / 689 (100%)** indexed in `swe1_audio.rs` (`382` `ARD` pools / `695` `ACD`)
- [x] **Animations & Sprites (`gfx/`)**: **354 / 392 (90.3%)** wired across all 14 scenes (intros + backgrounds + explosions + victories), `DeffWattoJunkShop`, `DeffMarketplace`, `DeffJarJar`, `DeffBumperSparks`, `DeffJediMultiballIntro`, `JediSpirit`, Attract, HUD, spinners, special screens & diagnostics

---

## 5. Desktop Runtime, Debug Console & HD Rendering (`pin2k-desktop`) — **90%**
- [x] Z-sorted `BaseViewManager` software compositor `640×240` → `640×480` (`raster.rs`, `compositor.rs`)
- [x] **5-tab Web Debug Console** (`web_ui/{mod,state}.rs` + `web_ui.html`): direct mode/scene/diagnostic launch, Pause/Step, player/time mutations, `/api/assets` browser, coils/lamps, live Z-layer inspector
- [x] Native OS audio (`lewton` → `winmm.dll` / `aplay`) with atomic `StopAll`, `Stop`, and `looping` (`audio/{mod,platform}.rs`) + WebAudio mod overrides (`/audio/`)
- [x] **Native Desktop Window & Fullscreen (`--window` / `--fullscreen`)** (`window.rs`, pure-Rust `winit` + `softbuffer`, `4:3` aspect ratio, `F11` / `Alt+Enter`)
- [x] **`gfx_hd/` support via `--hd`** in `SoftwareCompositor::load_hd_assets()`
- [x] Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) and Linux builds
- [ ] **WebGL2 HD Shaders & LRU memory cache** (see [HD_UPSCALING_GUIDE.md](HD_UPSCALING_GUIDE.md))

---

## 6. Asset Modding & Custom Scenes (`mod.json`) — **35%**
- [x] PNG animation replacement (`<mod>/gfx/<name>/*.png` + `anim_overrides`) and `--mod` stacking
- [x] `.ogg` audio overrides for both native OS and WebAudio (`sound_overrides`) + `HomebrewSwitchRule`
- [ ] **Graphics modding (`gfx/` & `fonts/`)**: `<mod>/fonts/` override support and per-animation metadata (`fps`, `loop`, `scale`, `pivot`) in `mod.json`
- [ ] **Rule modding (`mod.json`)**: preserve `attract_banner` in SWE1 mode and extend `switch_rules` (conditions, lamp-off actions)
- [ ] **Dynamic custom scenes registry**: JSON-driven scene definitions selectable in `DeffObiHolo` (`intro`/`active`/`victory`/`timeout` phases, Z-layers, local switch rules)
- [ ] **Advanced scripting (mid-term)**: evaluate pure-Rust scripting engine (e.g. **Rhai**) bound to `VideoBackend` / `PinballHardware`

---

## 7. Documentation & Quality — **90%**
- [x] `RUSTFLAGS="-D warnings" cargo check --workspace --all-targets` (**0 warnings**), **44 / 44 unit tests** passing, and `cargo build --release --workspace` clean
- [x] **14 / 14 End-to-End functional test** recorded as MP4 video (`work/e2e_validation.mp4`, 1,455 frames, 48.5s @ 30 fps)
- [x] Visual verification across **11 PNG captures** (`attract:0`, `ingame`, `spinner_left`, `scene:6`, `scene:0`, `scene:12`, `bonus`, `match`, `hstd_entry`, `diag:colorbar`, `diag:switch`)
- [x] High-level English documentation: [README.md](../README.md), [SCREEN_LAYOUT_REFERENCE.md](SCREEN_LAYOUT_REFERENCE.md), [ARCHITECTURE_GUIDE.md](ARCHITECTURE_GUIDE.md), [HARDWARE_NOTES.md](HARDWARE_NOTES.md), [GAME_SCRIPT_REFERENCE.md](GAME_SCRIPT_REFERENCE.md), [ASSETS_MATRIX.md](ASSETS_MATRIX.md), [SWE1_EXTRACTION_INVENTORY.md](SWE1_EXTRACTION_INVENTORY.md), [SWE1_GAME_RULES.md](SWE1_GAME_RULES.md), [RFM_EXTRACTION_INVENTORY.md](RFM_EXTRACTION_INVENTORY.md), [RFM_GAME_RULES.md](RFM_GAME_RULES.md), [HD_UPSCALING_GUIDE.md](HD_UPSCALING_GUIDE.md), [TODO.md](TODO.md)
- [ ] **Complete codebase & class documentation**: write a thorough technical reference covering every crate (`pin2k-hal`, `pin2k-core`, `pin2k-rom-tool`, `pin2k-desktop`), module, struct/enum/trait, state machine lifecycle, 60 Hz tick & rendering/audio pipelines, PDB hardware protocol flow, and ROM-to-Rust class mapping (`Deff*`, `Leff*`, `Player`, `Scenes`, etc.), plus inline Rustdoc (`///`) across public APIs
