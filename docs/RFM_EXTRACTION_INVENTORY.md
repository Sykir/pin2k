# *Revenge From Mars* (`RFM 0195`) — ROM Extraction Inventory

Inventory of everything extracted from the **Revenge From Mars (`#50070`)** ROM: `pin2000_50070_0195_03292018_B_10000000` (`0195 / v1.09`, XINA `1.22`, built `2018-03-29`).

---

## 1. General Extraction Summary (`RFM 0195`)

| Category | ROM Source | Tool / Method | Extracted Count | Format / Destination |
|---|---|---|---|---|
| **x86 Native Code & C++ Symbols** | Decompressed PRISM image (`B_10000000`) | `pin2k-rom-tool wrap-elf` | `3,432,816` bytes, `19,290` symbols | `ELF32 i386` (`rfm_0195.elf`) |
| **Switch Table** | `SwitchTable` (`0x002fc554`) | ELF parser | `64` entries (matrix + optos + directs) | `crates/pin2k-core/src/rfm_tables.rs` |
| **Drive / Solenoid Table** | `DriveTable` (`0x002faed4`) | ELF parser | `40` coils + flashers | `crates/pin2k-core/src/rfm_tables.rs` |
| **Lamp Table** | `LampTable` (`0x002fb354`) | ELF parser | `8×8` matrix + dedicated lamps | `crates/pin2k-core/src/rfm_tables.rs` |
| **Animations, Fonts & Audio** | `Animation`, `FontData`, `pb2kslib` | `pin2k-rom-tool extract-assets` | Complete extraction from RFM ROM | Multi-frame PNG, `.ogg`, JSON |
| **Waves, Multiballs & Mechanics** | `AttackMars`, `MartianAttack`, `CaptureMultiball`, `HypnoBeamMultiball`, `MartianMultiball`, `SkillShots`, `Bonus`, etc. | ELF disassembly | All scoring formulas & flags | `RFM_GAME_RULES.md` + `rfm_rules.rs` |

---

## 2. Extracted Waves, Scenes & Modes (`RFM 0195`)

| Category | Extracted C++ Classes | Gameplay Role |
|---|---|---|
| **Wave — Mars Kneads Women / Abduction** | `MarsKneadsWomen`, `Abduction` | Rescue abducted Earthlings from saucers |
| **Wave — Big-O-Beam / Secret Weapon** | `WeaponWave`, `SecretWeapon` | Secret weapon / beam wave |
| **Wave — Paris in Peril / Tower Struggle** | `TowerStruggle` | Defend Paris / Eiffel Tower |
| **Wave — Paratrooper** | `Paratrooper`, `ParatrooperBacky` | Martian paratroopers |
| **Wave — Barnyard** | `Barnyard`, `DeffBarnyardBacky` | Barnyard abduction |
| **Wave — Alien Drive-In** | `DriveIn` | Drive-In theater wave |
| **Wave — Martian Bowling** | `MartianBowling` | Martian bowling wave |
| **Wave — Martian Tank** | `MartianTank`, `MartianTankBacky` | Martian tank battle |
| **Wave — Martian Happy Hour / Autopsy / Invaders** | `MartianHappyHour`, `Autopsy`, `Invaders` | Special modes & Martian sub-waves |
| **Bonus Wave & Mystery** | `BonusWave`, `QuMarkManager`, `FuelWave`, `SaucersWave` | Bonus wave, Mystery (`?`), fuel & saucers |
| **Wizard Mode — Attack Mars!** | `AttackMars`, `AttackMarsBacky`, `DeffAttackMarsChampion` | Finale qualified by `all_modes + mothership + all_saucer_lights` |
| **Martian Attack** | `MartianAttack`, `MartianTargets`, `MartianBombs` | Knock down Martian targets (progressive value) |
| **Capture Multiball** | `CaptureMultiball`, `DeffCaptureMultiballStart`, `DeffCaptureMultiballBacky` | Saucer locks → 3-ball multiball |
| **Hypno Beam Multiball** | `HypnoBeamMultiball`, `DeffHypnoBeamBacky` | Hypno Beam multiball + Extra Ball on 1st Jackpot |
| **Martian Multiball** | `MartianMultiball`, `DeffMartianMultiballStart`, `DeffMartianMultiballBacky` | Martian multiball (`VbMartian` / `VrMartian` targets) |
| **Victory Laps** | `VictoryLaps` | Victory laps after *Attack Mars!* |
| **Skill Shots, Bonus & Ball Saver** | `SkillShots`, `Bonus`, `DeffBonus`, `BallSaver`, `SceneBallSaver` | Skill Shot, Super Skill Shot, Multiplier, Bonus Held |
| **Attract Mode & System** | `AttractMode`, `RfmGame`, `Game`, `GameHooks` | Attract Mode, 4 players, Tilt, Slam Tilt, diagnostics |

---

## 3. Direct Launch & Layout Extraction (`pin2k-rom-tool` / Debug Console)

- **`pin2k-rom-tool extract-layout --elf rfm_0195.elf --output-dir <DIR>`** supports the *Revenge From Mars* ELF32 binary to export its `Deff*` classes and screen metadata to `rom_layout_db.json`.
- `pin2k-desktop` (via `--debug-mode` or the Web Debug Console) supports hot-switching to `game:rfm`, launching all 9 waves (`rfm_wave:0`..`8`), `capture_multiball`, `martian_attack`, and full hardware diagnostics.
