# *Revenge From Mars* (`RFM 0195`) — Extracted Game Rules

All rules, state machines, qualification flags, and scoring formulas below come from direct reverse engineering of `rfm_0195.elf` (`pin2000_50070_0195_03292018_B_10000000`, `19,290` symbols).

---

## 1. Skill Shot & Super Skill Shot (`SkillShots`, `0x1a32a8..0x1a3eec`)
- **Skill Shot (`calc_skill_shot_value`, `0x1a3658`)**:
  - Exact formula: `500,000 + (n - 1) × 100,000`, capped at **`1,500,000` pts** (`0x16e360`).
- **Super Skill Shot (`calc_super_skill_shot_value`, `0x1a3968`)**:
  - Armed via flipper buttons during launch (`allow_super_skill_shot`).
  - Exact formula: `1,000,000 + (n - 1) × 200,000`, capped at **`3,000,000` pts** (`0x2dc6c0`).

## 2. Saucer Waves & Core Modes
- **Main waves / modes**: `MarsKneadsWomen` (`Abduction`), `WeaponWave` / `SecretWeapon`, `TowerStruggle`, `Paratrooper`, `Barnyard`, `DriveIn`, `MartianBowling`, `MartianTank`, `MartianHappyHour` (+ `BonusWave`, `FuelWave`, `SaucersWave`, `Autopsy`, `Invaders`, `QuMarkManager`).
- **Martian Attack (`MartianAttack`, `0x112198..0x113c10`)**:
  - Martian targets (`MartianTargets`) + bombs (`MartianBombs`).
  - Per-Martian value (`setup_martian_value`, `0x1129a0`): `1,250,000 + (n - 1) × 250,000`, capped at **`2,500,000` pts** (`0x2625a0`).

## 3. Multiballs (`CaptureMultiball`, `HypnoBeamMultiball`, `MartianMultiball`)
- **Capture Multiball (`CaptureMultiball`)**:
  - `light_lock`: **`25,000` pts** | `award_lock`: **`50,000` pts** | `start`: **`100,000` pts**.
- **Hypno Beam Multiball (`HypnoBeamMultiball::award_beam_hit`, `0x19edd0`)**:
  - Each beam hit before the finishing blow: **`250,000` pts** (`0x3d090`).
  - Finishing blow (`beam_strength == 1`):
    - If `jackpots == 0`: awards an **Extra Ball** (`ExtraBall::award_eb`) + `aud_hypno_mb_eb`.
    - Jackpot score: `10,000,000 + jackpots × 5,000,000`, capped at **`50,000,000` pts** (`0x2faf080`).
- **Martian Multiball (`MartianMultiball`)**:
  - Animated Martian targets (`VbMartian`, `VrMartian`) with dedicated scoring on `DeffMartianMultiballBacky`.

## 4. *Attack Mars!* Wizard Mode & *Victory Laps* (`AttackMars`, `0x191354..0x191d24`)
- **Qualification (`AttackMars::ok_for_qualify`, `0x191458`)**: requires all **3 flags** set simultaneously:
  1. `flag_all_modes` (all modes/waves completed),
  2. `flag_mothership` (Mothership defeated),
  3. `flag_all_saucer_lights` (all saucer lights completed).
- Once qualified and started (`AttackMars::start`), defeating Mars transitions into **Victory Laps** (`VictoryLaps::start`).

## 5. End-of-Ball Bonus (`Bonus`, `0x180764..0x2e2242`)
- Tracks 4 per-player variables (`PlayerData`):
  - `multiplier` (`Bonus::add_to_multiplier`),
  - `multiplier_held_flag` (carries multiplier to next ball),
  - `switch_value` (`Bonus::add_to_switch_value`),
  - `switch_value_held_flag` (carries base bonus to next ball).
