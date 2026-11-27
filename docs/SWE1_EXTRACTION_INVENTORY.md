# *Star Wars Episode I* (`SWE1 v1.66 / v2.00`) — ROM Extraction Inventory

Complete inventory of everything extracted from the **Star Wars Episode I (`#50069`)** ROMs: `swe1_0166.elf` (`v1.66`, XINA `1.13`) and `swe1_0200.elf` (`v2.00`, `14,841` C++ symbols, `1,867` Ghidra-decompiled functions across `248` classes).

---

## 1. General Extraction Summary (`SWE1 v1.66 / v2.00`)

| Category | ROM Source | Tool / Method | Extracted Count | Format / Destination |
|---|---|---|---|---|
| **x86 Native Code & C++ Symbols** | Decompressed PRISM image (`B_10000000`) | `pin2k-rom-tool wrap-elf` + Ghidra 12.2 | `14,841` symbols (`v2.00`) / `15,019` (`v1.66`), `1,867` decompiled functions | `ELF32 i386` (`swe1_0200.elf`, `swe1_0166.elf`) |
| **UI Layouts, `Deff*` & Coordinates** | `.text`/`.rodata`/`.data` | `pin2k-rom-tool extract-layout` | `138` `Deff*` classes, `702` `ACD`, `383` `ARD`, `9` characters | `rom_layout_db.json` + [SCREEN_LAYOUT_REFERENCE.md](SCREEN_LAYOUT_REFERENCE.md) |
| **Switch Table** | `SwitchTable` (`0x002e4580`) | ELF parser | `96` logical switches (matrix + optos + directs) | `crates/pin2k-core/src/swe1_tables.rs` |
| **Drive / Solenoid Table** | `DriveTable` (`0x002e2f00`) | ELF parser | `48` outputs (coils + flashers + neon) | `crates/pin2k-core/src/swe1_tables.rs` |
| **Lamp Table** | `LampTable` (`0x002e3380`) | ELF parser | Dual `8×8` matrices A & B (`128` lamps) | `crates/pin2k-core/src/swe1_tables.rs` |
| **Animations & Sprites** | `Animation` descriptors + RLE frames | `pin2k-rom-tool extract-assets` | `392` animations (`354 / 392` = **90.3%** wired in Rust) | `gfx/` (PNG) + `gfx_manifest.json` |
| **Bitmap Fonts** | `FontData` descriptors | `pin2k-rom-tool extract-assets` | `19` fonts (**19 / 19 = 100%** wired) | `fonts/` (PNG) + `fonts_manifest.json` |
| **DCS-2 Audio (Voices, SFX, Music)** | DCS / `pb2kslib` banks | `pin2k-rom-tool extract-assets` | `689` tracks (**689 / 689 = 100%** indexed in `swe1_audio.rs`) | `audio/*.ogg` + `sound_manifest.json` |
| **Attract Mode** | `AttractProcInfo` (`0x002d94d0`) | `extract-layout` + disassembly | `20` implemented sequences | `attract.rs` + `attract_render.rs` |
| **Holoprojector Scenes** | `Table<scene_info>` (`0x002dc95c` / `0x2e179c`) | `extract-layout` + Ghidra | `14` scenes (intros + backgrounds + FX) + `SubEscape` | `swe1_rules.rs` + `scene_render.rs` + `fx/` |
| **Multiballs, HUD & System** | `DeffScoreDisplay`, `DeffLaserStatusBackground`, `DeffVSpinner`, `Bonus`, `Match`, `HSTD`, `Diagnostic*Deff` | ELF disassembly & Ghidra | All coordinates, Z-orders, and scoring formulas | `SWE1_GAME_RULES.md`, `SCREEN_LAYOUT_REFERENCE.md` |

---

## 2. All Extracted Scenes (`Table<scene_info>` @ `0x2e179c`)

| # | Bitmask | ROM C++ Symbol | Display Title | `min_scenes_started` (`+0x08`) | `min_jedi_battles` (`+0x0C`) | Main C++ Class | Background `Deff` Class |
|---|---|---|---|---|---|---|---|
| 1 | `0x0001` | `jedi_scene_battle_droids` | Battle Droid Invasion | `0` | `0` | `BattleDroidScene` | `DeffBattleDroidSceneBackground` |
| 2 | `0x0002` | `jedi_scene_musical_chairs` | Mos Espa Cantina (Musical Chairs) | `1` | `0` | `MusicalChairsScene` | `MusicalChairsScene` |
| 3 | `0x0004` | `jedi_scene_federation_fighter` | Federation Fighters | `0` | `0` | `FederationFighterScene` | `DeffFedFighterSceneBackground` |
| 4 | `0x0008` | `jedi_scene_r2d2` | R2-D2 Scene | `1` | `0` | `R2D2Scene` | `DeffR2D2SceneBackground` |
| 5 | `0x0010` | `jedi_scene_gungan` | Gungan Energy Battle | `0` | `0` | `GunganScene` | `GunganScene` |
| 6 | `0x0020` | `jedi_scene_queen_fashion` | Queen Amidala Wardrobe | `2` | `0` | `QueenFashionScene` | `DeffQueenFashionSceneBackground` |
| 7 | `0x0040` | `jedi_scene_podrace` | Boonta Eve Podrace | `0` | `0` | `PodraceScene` | `DeffPodRaceSceneBackground` |
| 8 | `0x0080` | `jedi_scene_hangar_escape` | Theed Hangar Escape | `0` | `0` | `HangarEscapeScene` | `DeffHangarEscapeSceneBackground` |
| 9 | `0x0100` | `jedi_scene_jarjar_juggle` | Jar Jar Juggle | `0` | `0` | `JarjarJuggleScene` | `DeffJarjarJuggleScene` |
| 10 | `0x0200` | `jedi_scene_probe_droid` | Sith Probe Droids | `0` | `0` | `ProbeDroidScene` | `DeffProbeDroidSceneBackground` |
| 11 | `0x0400` | `jedi_scene_destroyer_droid` | Destroyer Droids | `0` | `1` | `DestroyerDroidScene` | `DeffDestroyerDroidSceneBackground` |
| 12 | `0x0800` | `jedi_scene_hover_tank` | AAT Hover Tank | `0` | `1` | `HoverTankScene` | `DeffHoverTankSceneBackground` |
| 13 | `0x1000` | `jedi_scene_wattos_chance` | Watto's Chance Cube | `0` | `1` | `WattosChanceScene` | `DeffWattosChanceSceneBackground` |
| 14 | `0x2000` | `jedi_scene_question_mark` | Mystery (`?`) | `0` | `1` (unused in 13-scene draw) | `QuestionMarkScene` | `DeffQuestionMarkSceneBackground` |

---

## 3. Special Modes, Multiballs & System Sequences

| Type | Extracted C++ Classes | Description |
|---|---|---|
| **Gungan Sub-Scene** | `DeffSubEscapeSceneIntro`, `DeffSubEscapeSceneBackground`, `ClassSubEscapeLeffHit` | Naboo underwater chase (*Sub Escape*) |
| **Main Multiball** | `Multiball`, `DeffMainMultiballStart`, `DeffMultiballBackground`, `DeffMainMultiballSuperJackpot` | Hangar / Naboo Starfighter Multiball (Jackpot + Super Jackpot) |
| **Jedi Multiball** | `JediMultiball`, `DeffJediMultiballIntro`, `DeffJediMultiballBackground`, `DeffJediMultiballWin` | Darth Maul lightsaber duel (triggered on J-E-D-I completion) |
| **Attract Mode** | `AttractProcInfo` (`20` entries), `DeffAttractModeBackground`, `DeffAttractModeKahunaLogo`, `DeffAttractModeP2KLogo` | Full attract loop, logos, HSTD, Jedi tables, Ramp Combo Champion |
| **Bonus / End of Ball** | `Bonus`, `DeffBonus`, `LeffBonus`, `BallStartBallSaver` | `build_bonus` × `build_bonus_x` + Ball Saver |
| **System & Diagnostics** | `Game`, `GameHooks`, `DeffTilt`, `DeffTiltWarning`, `DeffSlamTilt`, `Diagnostic*Deff` | 4-player management, Tilt, Slam Tilt, Operator menus & video test patterns |

---

## 4. Extracted UI Layouts (`swe1_0200.elf` → Pure Rust)

All screens below were reconstructed with exact `640×240` coordinates and Z-priorities (see [SCREEN_LAYOUT_REFERENCE.md](SCREEN_LAYOUT_REFERENCE.md)):
- **In-game HUD**: `DeffScoreDisplay` (`score_display.rs`) + `DeffLaserStatusBackground` (`hud_display.rs`).
- **Ramp-only virtual spinners**: `DeffVSpinner` (`vspinner.rs`) with turnstile cues `0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`.
- **20 Attract Mode screens**: `AttractProcInfo` (`attract_render.rs`), including `DeffAttractModeKahunaLogo` and its 9-color RGB555 flare cycle (`0x100fc0`).
- **All 14 scenes + Multiballs + Holo Select + FX/Overlays**: isolated 2× `Deff*SceneIntro` bezel intros + 14 active gameplay backgrounds (`scene_render.rs` + `fx/`).
- **Special screens & Diagnostics**: `PowerupTestingDeff`, `DeffBonus`, `DeffMatch`, `DeffGameOver`, `DeffGameIntro`, `DeffExtraBallCollected`, `DeffBSBallSaved`, `DeffBuildC3P0Status`, `DeffRampCombo`, `DeffJediMultiballReadyBackground`, `DeffHSTDinitials`, and `Diagnostic{ColorBar,Grid,RGB,Fuse,Switch,Coil,SingleLamp}Deff` (`system_screens.rs`).
