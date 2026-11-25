# *Star Wars Episode I* (`SWE1 v1.66 / v2.00`) — Extracted Game Rules

All rules, state machines, qualification rules, scoring formulas, `ARD`/`ACD` audio pools, and `Leff*` lamp choreographies below are lifted directly from reverse engineering and headless **Ghidra 12.2** decompilation of `swe1_0166.elf` (`15,019` symbols) and `swe1_0200.elf` (`14,841` symbols, `1,867` decompiled functions across `248` classes).

---

## 1. Game Cycle, HUD & Virtual Spinners (`Game`, `DeffScoreDisplay`, `DeffLaserStatusBackground`, `DeffVSpinner`)
- **Players & Unique Characters (`DeffScoreDisplay`, `0x1725d8`)**: `1` to `4` players per game (`get_player_up()`, `0..=3`), each assigned a unique Star Wars character from the `9` entries of `player_name_table_data` (`Qui-Gon`, `Jar Jar`, `Anakin`, `Amidala`, `Obi-Wan`, `R2-D2`, `Yoda`, `Watto`, `Darth Maul`), displayed alongside the left/right lightsabers at the top of the screen.
- **Laser Reticle HUD (`DeffLaserStatusBackground`, `0x1445f0`)**: top-right `(535, 10)` rotating Naboo Fighter (`LASER`/`LASERS`, or `TORPEDOS` during *Theed Hangar Escape*), rendered monochrome grey when depleted.
- **Ramp-Only Virtual Spinners (`VirtualSpinners` / `DeffVSpinner`, `0x17ead0`)**: idle at rest; activated **only** on ramp passes (`LEFT_RAMP_ENTER`, `RIGHT_RAMP_ENTER`, `RAMP_MADE_LEFT`, `RAMP_MADE_RIGHT`) at `(43, 174)` / `(421, 174)`, with deceleration (~1.6 s), `+10000` / `+25000` popup, and rotating DCS turnstile audio (`0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`).
- **Attract Mode**: cycles through all `20` screens defined in `AttractProcInfo` (`0x002d94d0`) — *Williams*, *Pinball 2000*, *Kahuna / Star Wars* logo + 9-color light rays, *SlideShow*, *High Scores*, *Jedi Tables*, *Ramp Combo Champion*, *Credits*, etc.
- **Ball Saver (`BallStartBallSaver`)**: active on ball launch; draining before timer expiration auto-serves the ball without losing the ball in play.

## 2. Skill Shot (`SkillShot`, `0x17c634`)
- Armed on ball serve (`SkillShot::hook_ball_serve_start`) when no scene/multiball blocks it; disarmed once the playfield becomes valid.
- **Award**: passing the *Sneaky Lane* (`Sw 15`) or *Jets Rollover* (`Sw 45`) calls `SkillShot::award()`.
- **Scoring**: progressive `500,000 × skill_shots_made`, **capped at `3,000,000` pts** (`0x2dc6c0`), triggers `LeffKind::SkillShotAward` & `ard_skill_shot_award`.

## 3. Deflector Shield & Scene Qualification (`Shield`, `Scenes`, `0x177bf0..0x1787c8`)
- **Availability (`Scenes::are_available`)**: a scene can start only when no multiball/rematch is active, the final Jedi sequence is not complete, and no other scene is running.
- **Shield hits required (`Scenes::reset_remaining_hits`, `0x177dd0`)**:
  - **1st scene** (`scenes_started == 0`): **`1`** shield hit.
  - **Subsequent scenes** (`scenes_started > 0`): **`3`** shield hits.
- **Scene Selection (`Scenes::choose_next_scene`, `0x178130`)**:
  - Action/flipper buttons cycle eligible scenes in `DeffObiHolo` before shooting the center scoop/ramp.
  - Minimum `scenes_started` thresholds:
    - `min = 0`: `BattleDroid`, `FederationFighter`, `Gungan`, `Podrace`, `HangarEscape`, `JarJarJuggle`, `ProbeDroid`, `QuestionMark`
    - `min = 1`: `MusicalChairs`, `R2D2`, `DestroyerDroid`, `HoverTank`, `WattosChance`
    - `min = 2`: `QueenFashion`
  - Completing all `14` scenes resets the bitmask via `Scenes::reset_scene_status()`.

## 4. Per-Scene Rules & Extracted Scoring

| Scene | Core Extracted Mechanics | Exact Extracted Scores |
|---|---|---|
| **Battle Droid Invasion** | Destroy Battle Droids, STAPs & MTT via shield, shield targets, and ramps | Shield: `400,000`; Target: `200,000`; STAP: `250,000`/`500,000`; Completion: `2,250,000` |
| **Destroyer Droids** | Destroy Droidekas then expose the Droid Control Ship | Hit: `50,000`; Droideka: `250,000`; Control Ship: `450,000` |
| **Federation Fighters** | Laser shots & ramps against Federation Fighters | `DeffFedFighterSceneBackground` + `ClassFedFighterLeffLaserHit` |
| **Gungan Energy Battle** | Accumulate hits on shield, L/R ramps, shield standups & left loop | `50,000` per qualified hit |
| **Theed Hangar Escape** | Fast shots and torpedoes (`ClassHangarDroidLeffLaserHit`) | `20,000` per hit |
| **AAT Hover Tank** | Damage and destroy the AAT Hover Tanks | Hit: `50,000`; Tank destroyed: `250,000` |
| **Boonta Eve Podrace** | 3-lap Podrace via shield, targets & L/R ramps | `50,000` per hit |
| **Sith Probe Droids** | Shoot down Sith Probe Droids | Hit: `70,000`; Probe destroyed: `250,000` |
| **Queen Amidala Wardrobe** | Spin the wheel & match outfit cards via L/R ramps | Spin: `20,000`; Matched card: `500,000` (`15,520` card bonus) |
| **Jar Jar Juggle** | Juggle junk with Jar Jar via shots & saucer | Random awards via `ra_juggle_random_award_table` |
| **Watto's Chance Cube** | Roll Watto's cube (`spin`) to win the bet | Spin: `20,000`; Win: `250,000` |
| **Mos Espa Cantina (`MusicalChairs`)** | Jump characters & hit the center seat (`award_center_guy`) | Jump: `70,000`; Center guy: `200,000` |
| **R2-D2 Scene** | Center scoop with decaying *Hurry-Up* timer | Decaying Hurry-Up value + `start_ard`/`hit_ard`/`complete_ard` |
| **Mystery (`QuestionMark`)** | Instant mystery award | Random award (`1,000,000`, +3 Lasers, +2× Bonus, or Extra Ball) |
| **Sub Escape** | Naboo underwater chase (`sub_set_distance`) | Distance gauge: escape or chomp |

## 5. Lasers, Drop Targets, Jet Bumpers, Ramps & Build C-3PO
- **Lasers (`Lasers::fire_left` / `fire_right`, `0x146354` / `0x14648c`)**: requires `laser_shots > 0` (else plays `ard_generic_laser_empty`), scores **`1,000` pts**, simulates a drop target hit or forwards a direct hit to the active scene.
- **Drop Targets (`Drop::switch_hit`)**: **`2,310` pts** per target.
- **Jet Bumpers (`JetBumpers::jet_bumper_hit`, `0x10dc84`)**:
  - **Normal**: **`10,310` pts** (`0x2846`).
  - **Super Jets**: **`51,550` pts** (`0xc95e`), active for `20 s` (`super_jets_ms = 20_000`) after spelling `G-U-N-G-A-N` (`6` hits), triggering `LeffKind::SuperJetsStart`, `ard_gungan_letters_done`, and `DeffJarJar`.
- **Ramps & Combos (`Ramps`, `RampBlocker`)**: L/R ramps trigger `LeffKind::RampMade`, `DeffVSpinner`, combo window (`ramp_combo_window_ms`), and track `ramp_combo_best`.
- **Build C-3PO (`BuildC3P0`)**: collects `6` parts (`c3p0_parts`), triggering `LeffKind::C3P0Complete` and `ard_c3p0_complete` on completion.

## 6. J-E-D-I Letters, Jedi Multiball & Main Multiball
- **J-E-D-I Letters (`JediLetters::add_letter`, `0x13bc64`)**: `500,000 + 50,000 × n` per letter; completing the 4th letter awards **`2,000,000` pts** (`0x1e8480`), plays `ard_jedi_multiball_ready_tune`, and starts **Jedi Multiball**.
- **Jedi Multiball — Darth Maul Duel (`JediMultiball`, `0x13d89c..0x13e69c`)**: Start **`500,000`**, `attack_maul` **`500,000`**, `attack_maul_combo` **`700,000`**, Defeat Maul **`5,000,000`** (`ard_jedi_battle_multiball_ending_speech`, `LeffKind::JediMultiballFinish`, rank promotion).
- **Main / Hangar Multiball (`Multiball`, `0x154c30..0x1562b8`)**: Start **`500,000`** (`LeffKind::MainMultiballStart`), Jackpot **`1,000,000 × next_jp`** (`ard_multiball_jackpot_speech`, `LeffKind::MainMultiballJackpot`), **Super Jackpot** **`5,000,000`** (`0x4c4b40`, `ard_multiball_jackpot_cleared_speech`).

## 7. Extra Ball & End-of-Ball Bonus
- **Extra Ball (`ExtraBall::award`, `0x118208`)**: awards extra ball and displays `DeffExtraBallCollected`.
- **Bonus (`Bonus`, `DeffBonus::Run`, `0x10b554`)**: `Total Bonus = build_bonus (0..=10) × build_bonus_x (1..=5) × 100,000`, capped at **`5,000,000` pts**.
