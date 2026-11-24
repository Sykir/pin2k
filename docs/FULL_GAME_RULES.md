# Intégralité des règles de jeu extraites des ROMs Pinball 2000

Ce document rassemble l'ensemble des règles de gameplay, machines à états, conditions de qualification et formules de score extraites par rétro-ingénierie directe de `swe1_0166.elf` (*Star Wars Episode I v1.66*) et `rfm_0195.elf` (*Revenge From Mars 0195*).

---

## Partie A — *Star Wars Episode I* (`SWE1 v1.66`)

### 1. Cycle de partie et démarrage (`Game`, `BallStartBallSaver`)
- **Joueurs** : `1` à `4` joueurs par partie (`get_player_up()`, borné `0..=3`).
- **Attract Mode** : enchaîne `20` écrans définis dans `AttractProcInfo` (`0x002d94d0`) — logos *Williams*, *Pinball 2000*, *Kahuna*, *Star Wars Episode I*, *High Scores*, *Jedi Table*, *Ramp Combo Champion*, *Crédits*, etc.
- **Ball Saver (`BallStartBallSaver`)** : actif au lancement de chaque bille ; si la bille draine avant expiration du timer, relance automatique sans perte de bille (`auto_ball_serve()`).

### 2. Skill Shot (`SkillShot`, `0x17c634`)
- Armé au début de la séquence de lancement (`SkillShot::hook_ball_serve_start`) si aucune scène/multiball ne bloque.
- Désarmé dès que le plateau devient valide (`SkillShot::hook_valid_playfield_start` -> `SkillShot::stop()`).
- **Validation** : passage par la *Sneaky Lane* (`SkillShot::hook_sneaky_lane` -> `SkillShot::award()`).
- **Score** : progressif selon le nombre de Skill Shots réussis dans la partie, **plafonné à `3 000 000` pts** (`0x2dc6c0`), et incrémente `aud_skill_shot`.

### 3. Bouclier et qualification des Scènes (`Shield`, `Scenes`, `0x177bf0..0x1787c8`)
- **Disponibilité (`Scenes::are_available`)** : une scène ne peut être lancée que si :
  1. Aucun Multiball ou Rematch n'est actif (`!is_any_multiball_or_rematch_active()`),
  2. La séquence Jedi finale n'est pas déjà complétée (`!is_jedi_complete()`),
  3. Aucune autre scène n'est déjà active (`!Scenes::are_any_active()`).
- **Nombre de coups bouclier requis (`Scenes::reset_remaining_hits`, `0x177dd0`)** :
  - **1ère scène du joueur** (`scenes_started == 0`) : **`1`** passage/coup sur le bouclier suffit.
  - **Scènes suivantes** (`scenes_started > 0`) : **`3`** coups sur le bouclier sont requis (`min(counter + 3, 3)`).
- **Sélection de la scène (`Scenes::choose_next_scene`, `0x178130`)** :
  - Les boutons d'action gauche/droite (`hook_left_action_button` / `hook_right_action_button`) permettent de faire défiler les scènes éligibles avant de passer le bouclier (`hook_shield_pass` -> `start_valid_scene`).
  - Une scène n'est éligible (`scene_ok`) que si elle n'a pas encore été jouée dans le cycle courant et si `scenes_started >= min_scenes_started` :
    - `min = 0` : `BattleDroid`, `FederationFighter`, `Gungan`, `Podrace`, `HangarEscape`, `JarJarJuggle`, `ProbeDroid`, `QuestionMark`
    - `min = 1` : `MusicalChairs`, `R2D2`, `DestroyerDroid`, `HoverTank`, `WattosChance`
    - `min = 2` : `QueenFashion`
  - Lorsque toutes les `14` scènes ont été jouées, `Scenes::reset_scene_status()` réinitialise le masque.

### 4. Règles et scores de chaque scène (`SWE1 v1.66`)

| Scène | Mécanique principale extraite | Scores exacts extraits |
|---|---|---|
| **Battle Droid Invasion** | Abattre les Battle Droids et STAPs via bouclier, cibles bouclier et rampes | Tir bouclier : `400 000` pts ; Cible bouclier : `200 000` pts ; STAP : `250 000` / `500 000` pts ; Complétion : `2 250 000` pts |
| **Destroyer Droids** | Détruire les Destroyer Droids puis exposer le vaisseau de contrôle (`is_control_ship_exposed`) | Tir : `50 000` pts ; Destroyer détruit : `250 000` pts ; Control Ship : `450 000` pts |
| **Federation Fighters** | Tirs de lasers et rampes contre les chasseurs de la Fédération | Piloté par `DeffFedFighterSceneBackground` + `ClassFedFighterLeffLaserHit` |
| **Gungan Energy Battle** | Accumuler les tirs sur bouclier, rampes G/D, cibles bouclier et boucle gauche | `50 000` pts par tir qualifié |
| **Theed Hangar Escape** | Fuite du hangar par tirs rapides et lasers (`ClassHangarDroidLeffLaserHit`) | `20 000` pts par tir |
| **AAT Hover Tank** | Endommager puis détruire le char AAT | Tir : `50 000` pts ; Destruction du char : `250 000` pts |
| **Boonta Eve Podrace** | Course de Podracers : bouclier, cibles bouclier, rampes gauche et droite | `50 000` pts par tir |
| **Sith Probe Droids** | Abattre les sondes Sith | Tir : `70 000` pts ; Sonde détruite : `250 000` pts |
| **Queen Amidala Wardrobe** | Faire tourner la roue (`spin`) et assortir les paires de cartes (`process_card`, rampes G/D) | Spin : `20 000` pts ; Paire / Carte validée : `500 000` pts (`15 520` pts bonus carte) |
| **Jar Jar Juggle** | Jongler avec Jar Jar via les tirs et la soucoupe Jar Jar | Awards aléatoires via `ra_juggle_random_award_table` |
| **Watto's Chance Cube** | Lancer le dé de Watto (`spin`) pour gagner le pari | Spin : `20 000` pts ; Gain : `250 000` pts |
| **Mos Espa Cantina (`MusicalChairs`)** | Faire sauter les personnages (`jump`) et viser le siège central (`award_center_guy`) | Jump : `70 000` pts ; Center Guy : `200 000` pts |
| **R2-D2 Scene** | Atteindre le centre avec chrono *Hurry-Up* (`hurry_up_timer_process`, `boost_hurry_timer`) | Valeur Hurry-Up dégressive vers le centre |
| **Mystery (`QuestionMark`)** | Récompense mystère immédiate | Attribuée via `QuestionMarkScene::start()` |
| **Sub Escape** | Poursuite sous-marine : bouclier, rampes, standups bouclier, boucle gauche | Jauge de distance (`sub_set_distance`) : victoire ou dévoré |

### 5. Lasers, Drop Targets, Jet Bumpers et Rampes
- **Lasers (`Lasers::fire_left` / `fire_right`, `0x146354` / `0x14648c`)** :
  - Utilisables dès qu'aucune intro de scène n'est active et que `pd_laser_count > 0`, avec cooldown de `60` ticks (`1,0 s`).
  - Décrémente le compteur de lasers, marque **`1 000` pts**, et simule un coup sur la Drop Target correspondante (`left_drop_target` / `right_drop_target`). Si la cible était déjà abaissée, propage le tir laser à la scène active.
- **Drop Targets (`Drop::switch_hit`)** : **`2 310` pts** par cible.
- **Jet Bumpers (`JetBumpers::jet_bumper_hit`, `0x10dc84`)** :
  - **Normal** : **`10 310` pts** (`0x2846`).
  - **Super Jets** : **`51 550` pts** (`0xc95e`).
  - Chaque coup allume une lettre de `G-U-N-G-A-N` (`0..=5`) ; le 6e coup remet le compteur à `0` et déclenche **Super Jets**.
- **Rampes & Combos (`Ramps`, `RampBlocker`)** :
  - Les rampes gauche/droite déclenchent `leff_ramp_made_left` / `leff_ramp_made_right` et enchaînent les `next_combo_shot()` tant que le `RampBlocker` est en dessous de `adj_ramp_blocker_max`. Le meilleur combo est sauvegardé dans `nv_ramp_champ_data`.

### 6. Lettres J-E-D-I, Jedi Multiball & Main Multiball
- **Lettres J-E-D-I (`JediLetters::add_letter`, `0x13bc64`)** :
  - Collecte séquentielle des 4 lettres (`0..=3`).
  - À la 4e lettre : **`2 000 000` pts** (`0x1e8480`), `aud_jedi_letters_complete`, et déclenchement du hook `0x2d` qualifiant **Jedi Multiball**. Les lettres supplémentaires après complétion incrémentent le compteur d'overflow.
- **Jedi Multiball — Duel contre Darth Maul (`JediMultiball`, `0x13d89c..0x13e69c`)** :
  - Démarrage : **`500 000` pts**.
  - `attack_maul` : **`500 000` pts** et retire de la vie à Maul.
  - `attack_maul_combo` : **`700 000` pts**.
  - `attack_quigon` : **`1 000 000` pts**.
- **Main / Hangar Multiball (`Multiball`, `0x154c30..0x1562b8`)** :
  - Lock des billes dans les soucoupes gauche/droite (`is_left_saucer_ready`, `is_right_saucer_ready`).
  - Démarrage : **`500 000` pts** (`restart` : `2 000` pts).
  - Chaque Jackpot allumé (`check_jackpot`, `0x1559dc`) : **`1 000 000` pts** (`0xf4240`).
  - Lorsque tous les Jackpots sont collectés `4` fois de suite (`0x70 > 2`), attribution du **Super Jackpot** : **`5 000 000` pts** (`0x4c4b40`).

### 7. Extra Ball et Bonus de fin de bille
- **Extra Ball (`ExtraBall::award`, `0x118208`)** : décrémente le crédit d'Extra Ball allumé (temporaire ou joueur) et appelle `award_extra_ball()`, puis affiche `DeffExtraBallCollected`.
- **Bonus (`Bonus`, `DeffBonus::Run`, `0x10b554`)** :
  - `Total Bonus = build_bonus (0..=10) × build_bonus_x (1..=5) × 100 000` (plafonné à **`5 000 000` pts**).
  - Si `build_bonus == 10` et `build_bonus_x == 5`, joue la musique spéciale Bonus Maxed.

---

## Partie B — *Revenge From Mars* (`RFM 0195 / v1.09`)

### 1. Skill Shot & Super Skill Shot (`SkillShots`, `0x1a32a8..0x1a3eec`)
- **Skill Shot (`calc_skill_shot_value`, `0x1a3658`)** :
  - Formule exacte : `500 000 + (n - 1) × 100 000`, plafonné à **`1 500 000` pts** (`0x16e360`).
- **Super Skill Shot (`calc_super_skill_shot_value`, `0x1a3968`)** :
  - Activé via les boutons de flipper pendant le lancement (`allow_super_skill_shot`).
  - Formule exacte : `1 000 000 + (n - 1) × 200 000`, plafonné à **`3 000 000` pts** (`0x2dc6c0`).

### 2. Vagues de soucoupes et modes principaux (`RFM 0195`)
- **9 Vagues principales / modes** : `MarsKneadsWomen` (`Abduction`), `WeaponWave` / `SecretWeapon`, `TowerStruggle`, `Paratrooper`, `Barnyard`, `DriveIn`, `MartianBowling`, `MartianTank`, `MartianHappyHour` (+ `BonusWave`, `FuelWave`, `SaucersWave`, `Autopsy`, `Invaders`, `QuMarkManager`).
- **Martian Attack (`MartianAttack`, `0x112198..0x113c10`)** :
  - Cibles martiennes (`MartianTargets`) + bombes (`MartianBombs`).
  - Valeur de chaque Martien (`setup_martian_value`, `0x1129a0`) : `1 250 000 + (n - 1) × 250 000`, plafonnée à **`2 500 000` pts** (`0x2625a0`).

### 3. Multiballs (`CaptureMultiball`, `HypnoBeamMultiball`, `MartianMultiball`)
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

### 4. Finale *Attack Mars!* et *Victory Laps* (`AttackMars`, `0x191354..0x191d24`)
- **Qualification (`AttackMars::ok_for_qualify`, `0x191458`)** : requiert les **3 drapeaux** simultanément à `true` :
  1. `flag_all_modes` (tous les modes/vagues complétés),
  2. `flag_mothership` (Vaisseau-mère),
  3. `flag_all_saucer_lights` (toutes les lumières soucoupe).
- Une fois qualifié et lancé (`AttackMars::start`), le joueur affronte la finale puis enchaîne sur **Victory Laps** (`VictoryLaps::start`).

### 5. Bonus de fin de bille (`Bonus`, `0x180764..0x2e2242`)
- Suit 4 variables persistées par joueur (`PlayerData`) :
  - `multiplier` (`Bonus::add_to_multiplier`),
  - `multiplier_held_flag` (conserve le multiplicateur à la bille suivante),
  - `switch_value` (`Bonus::add_to_switch_value`),
  - `switch_value_held_flag` (conserve la valeur de base à la bille suivante).
