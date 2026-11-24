# *Revenge From Mars* (`RFM 0195`) — Règles de jeu extraites

L'intégralité des règles, machines à états, conditions de qualification et formules de score ci-dessous provient de la rétro-ingénierie directe de `rfm_0195.elf` (`pin2000_50070_0195_03292018_B_10000000`, `19 290` symboles).

---

## 1. Skill Shot & Super Skill Shot (`SkillShots`, `0x1a32a8..0x1a3eec`)
- **Skill Shot (`calc_skill_shot_value`, `0x1a3658`)** :
  - Formule exacte : `500 000 + (n - 1) × 100 000`, plafonné à **`1 500 000` pts** (`0x16e360`).
- **Super Skill Shot (`calc_super_skill_shot_value`, `0x1a3968`)** :
  - Activé via les boutons de flipper pendant le lancement (`allow_super_skill_shot`).
  - Formule exacte : `1 000 000 + (n - 1) × 200 000`, plafonné à **`3 000 000` pts** (`0x2dc6c0`).

## 2. Vagues de soucoupes et modes principaux
- **Vagues principales / modes** : `MarsKneadsWomen` (`Abduction`), `WeaponWave` / `SecretWeapon`, `TowerStruggle`, `Paratrooper`, `Barnyard`, `DriveIn`, `MartianBowling`, `MartianTank`, `MartianHappyHour` (+ `BonusWave`, `FuelWave`, `SaucersWave`, `Autopsy`, `Invaders`, `QuMarkManager`).
- **Martian Attack (`MartianAttack`, `0x112198..0x113c10`)** :
  - Cibles martiennes (`MartianTargets`) + bombes (`MartianBombs`).
  - Valeur de chaque Martien (`setup_martian_value`, `0x1129a0`) : `1 250 000 + (n - 1) × 250 000`, plafonnée à **`2 500 000` pts** (`0x2625a0`).

## 3. Multiballs (`CaptureMultiball`, `HypnoBeamMultiball`, `MartianMultiball`)
- **Capture Multiball (`CaptureMultiball`)** :
  - `light_lock` : **`25 000` pts**.
  - `award_lock` : **`50 000` pts**.
  - `start` : **`100 000` pts**.
- **Hypno Beam Multiball (`HypnoBeamMultiball::award_beam_hit`, `0x19edd0`)** :
  - Chaque tir sur le rayon avant le coup final : **`250 000` pts** (`0x3d090`).
  - Coup final (`beam_strength == 1`) :
    - Si `jackpots == 0` : attribue une **Extra Ball** (`ExtraBall::award_eb`) + `aud_hypno_mb_eb`.
    - Score Jackpot : `10 000 000 + jackpots × 5 000 000`, plafonné à **`50 000 000` pts** (`0x2faf080`).
- **Martian Multiball (`MartianMultiball`)** :
  - Cibles martiennes animées (`VbMartian`, `VrMartian`) avec scores dédiés sur `DeffMartianMultiballBacky`.

## 4. Finale *Attack Mars!* et *Victory Laps* (`AttackMars`, `0x191354..0x191d24`)
- **Qualification (`AttackMars::ok_for_qualify`, `0x191458`)** : requiert les **3 drapeaux** simultanément à `true` :
  1. `flag_all_modes` (tous les modes/vagues complétés),
  2. `flag_mothership` (Vaisseau-mère),
  3. `flag_all_saucer_lights` (toutes les lumières soucoupe).
- Une fois qualifié et lancé (`AttackMars::start`), le joueur affronte la finale puis enchaîne sur **Victory Laps** (`VictoryLaps::start`).

## 5. Bonus de fin de bille (`Bonus`, `0x180764..0x2e2242`)
- Suit 4 variables persistées par joueur (`PlayerData`) :
  - `multiplier` (`Bonus::add_to_multiplier`),
  - `multiplier_held_flag` (conserve le multiplicateur à la bille suivante),
  - `switch_value` (`Bonus::add_to_switch_value`),
  - `switch_value_held_flag` (conserve la valeur de base à la bille suivante).
