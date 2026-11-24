# Pinball 2000 Gamecode Reverse Engineering Specification

This document records the clean-room reverse-engineered rules, state machines, data tables, and exact scoring formulas extracted from the latest symbolized Williams Pinball 2000 ELF32 binaries:

- **Star Wars Episode I (`#50069`)**: `pin2000_50069_0166_04032022_B_10000000` (`v1.66`, XINA `1.13`, built `2022-04-03`, `15,019` symbols)
- **Revenge From Mars (`#50070`)**: `pin2000_50070_0195_03292018_B_10000000` (`0195 / v1.09`, XINA `1.22`, built `2018-03-29`, `19,290` symbols)

---

## 1. Star Wars Episode I (`SWE1 v1.66`)

### 1.1 Hardware I/O & Tables
- `DriveTable` (`0x002e2f00`), `LampTable` (`0x002e3380`), `SwitchTable` (`0x002e4580`)
- `SwitchWork_0_25ms` (`0x001a9984`), `PinIOWrite` (`0x001a8360`), `PinIORead` (`0x001a8450`)

### 1.2 Scene Selection & Qualification (`Scenes`, `0x345148` / `0x2e179c`)
- `Scenes::are_available()`: requires `!is_any_multiball_or_rematch_active() && !is_jedi_complete() && !Scenes::are_any_active()`.
- `Scenes::reset_remaining_hits()` (`0x177dd0`): requires **1** shield pass on the first scene (`scenes_started == 0`), and **3** shield hits/passes for all subsequent scenes (`min(counter + 3, 3)`).
- `Table<scene_info>` (`0x2e179c`, `14` entries of `20` bytes `{ mask, *Scene, min_scenes_started, ... }`):

| Bitmask | Scene Object | Min Scenes Started |
|---|---|---|
| `0x0001` | `jedi_scene_battle_droids` | 0 |
| `0x0002` | `jedi_scene_musical_chairs` | 1 |
| `0x0004` | `jedi_scene_federation_fighter` | 0 |
| `0x0008` | `jedi_scene_r2d2` | 1 |
| `0x0010` | `jedi_scene_gungan` | 0 |
| `0x0020` | `jedi_scene_queen_fashion` | 2 |
| `0x0040` | `jedi_scene_podrace` | 0 |
| `0x0080` | `jedi_scene_hangar_escape` | 0 |
| `0x0100` | `jedi_scene_jarjar_juggle` | 0 |
| `0x0200` | `jedi_scene_probe_droid` | 0 |
| `0x0400` | `jedi_scene_destroyer_droid` | 1 |
| `0x0800` | `jedi_scene_hover_tank` | 1 |
| `0x1000` | `jedi_scene_wattos_chance` | 1 |
| `0x2000` | `jedi_scene_question_mark` | 0 |

### 1.3 Core Mechanics & Exact Scoring
- **Bonus (`DeffBonus::Run`, `0x10b554`)**:
  - `build_bonus` (`0..=10` ships) × `build_bonus_x` (`1..=5` multiplier) × **`100,000`** pts (max `5,000,000`).
- **Skill Shot (`SkillShot::award`, `0x17c634`)**:
  - Progressive award capped at `0x2dc6c0` = **`3,000,000`** pts.
- **Jet Bumpers (`JetBumpers::jet_bumper_hit`, `0x10dc84`)**:
  - Normal hit: **`10,310`** pts (`0x2846`).
  - Super Jets active: **`51,550`** pts (`0xc95e`).
  - Collects 6 G-U-N-G-A-N letters (`0..=5`); 6th hit starts Super Jets.
- **Lasers (`Lasers::fire_left`/`fire_right`, `0x146354`/`0x14648c`)**:
  - Cooldown `60` ticks (`1.0 s`, `AProc` `0xf5`/`0xf6`).
  - Awards **`1,000`** pts (`0x3e8`), decrements `pd_laser_count`, and triggers `Drop::simulate_hit()` on the corresponding drop target (or scene laser hook if drop target was already down).
- **Drop Targets (`Drop::switch_hit`)**: **`2,310`** pts.
- **J-E-D-I Letters (`JediLetters::add_letter`, `0x13bc64`)**:
  - Completing all 4 letters (`J`, `E`, `D`, `I`) awards **`2,000,000`** pts (`0x1e8480`) and fires hook `0x2d` to qualify `JediMultiball`.
- **Main / Hangar Multiball (`Multiball::check_jackpot`, `0x1559dc`)**:
  - `Multiball::start`: **`500,000`** pts.
  - Each lit Jackpot: **`1,000,000`** pts (`0xf4240`).
  - Clearing all Jackpot bits 4 times awards **Super Jackpot**: **`5,000,000`** pts (`0x4c4b40`).
- **Jedi Multiball / Darth Maul Duel (`JediMultiball`, `0x13e4f8`)**:
  - `JediMultiball::start`: **`500,000`** pts.
  - `attack_maul`: **`500,000`** pts (`0x7a120`).
  - `attack_maul_combo`: **`700,000`** pts (`0xaae60`).
  - `attack_quigon`: **`1,000,000`** pts.
- **Scene Shot Values**:
  - `BattleDroidScene`: shield hit `400,000`, shield target `200,000`, STAP `250,000` / `500,000`, completion `2,250,000`.
  - `DestroyerDroidScene`: hit `50,000`, destroyer `250,000`, control ship `450,000`.
  - `GunganScene`: shot `50,000`.
  - `HangarEscapeScene`: hit `20,000`.
  - `HoverTankScene`: hit `50,000`, tank `250,000`.
  - `MusicalChairsScene`: jump `70,000`, center guy `200,000`.
  - `PodraceScene`: shot `50,000`.
  - `ProbeDroidScene`: hit `70,000`, probe `250,000`.
  - `QueenFashionScene`: spin `20,000`, card match `500,000`.
  - `WattosChanceScene`: spin `20,000`, win `250,000`.

---

## 2. Revenge From Mars (`RFM 0195 / v1.09`)

### 2.1 Hardware Tables
- `DriveTable` (`0x002faed4`), `LampTable` (`0x002fb354`), `SwitchTable` (`0x002fc554`)

### 2.2 Core Mechanics & Exact Scoring
- **Skill Shot (`SkillShots::calc_skill_shot_value`, `0x1a3658`)**:
  - Formula: `500,000 + (n - 1) * 100,000`, capped at **`1,500,000`** (`0x16e360`).
- **Super Skill Shot (`SkillShots::calc_super_skill_shot_value`, `0x1a3968`)**:
  - Formula: `1,000,000 + (n - 1) * 200,000`, capped at **`3,000,000`** (`0x2dc6c0`).
- **Martian Attack (`MartianAttack::setup_martian_value`, `0x1129a0`)**:
  - Formula: `1,250,000 + (n - 1) * 250,000`, capped at **`2,500,000`** (`0x2625a0`).
- **Capture Multiball (`CaptureMultiball`)**:
  - `light_lock`: **`25,000`** pts.
  - `award_lock`: **`50,000`** pts.
  - `start`: **`100,000`** pts.
- **Hypno Beam Multiball (`HypnoBeamMultiball::award_beam_hit`, `0x19edd0`)**:
  - Beam hit: **`250,000`** pts (`0x3d090`).
  - Final beam hit (`beam_strength == 1`): awards Extra Ball on first jackpot (`jackpots == 0`), and scores `10,000,000 + jackpots * 5,000,000`, capped at **`50,000,000`** (`0x2faf080`).
- **Attack Mars Qualification (`AttackMars::ok_for_qualify`, `0x191458`)**:
  - Requires all 3 flags: `flag_all_modes`, `flag_mothership`, and `flag_all_saucer_lights`.
