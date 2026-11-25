# *Star Wars Episode I* (`SWE1 v1.66 / v2.00`) — Règles de jeu extraites

L'intégralité des règles, machines à états, conditions de qualification, formules de score, pools audio `ARD`/`ACD` et chorégraphies lumineuses `Leff*` ci-dessous provient de la rétro-ingénierie et de la décompilation headless **Ghidra 12.2** de `swe1_0166.elf` (`15 019` symboles) et `swe1_0200.elf` (`14 841` symboles, `1 867` fonctions décompilées sur `248` classes).

---

## 1. Cycle de partie, HUD & Spinners (`Game`, `DeffScoreDisplay`, `DeffLaserStatusBackground`, `DeffVSpinner`)
- **Joueurs & personnages uniques (`DeffScoreDisplay`, `0x1725d8`)** : `1` à `4` joueurs par partie (`get_player_up()`, borné `0..=3`), chacun recevant un nom Star Wars unique issu des `9` entrées de `player_name_table_data` (`Qui-Gon`, `Jar Jar`, `Anakin`, `Amidala`, `Obi-Wan`, `R2-D2`, `Yoda`, `Watto`, `Darth Maul`), affiché avec les sabres laser gauche/droite en haut de l'écran.
- **Indicateur de Lasers (`DeffLaserStatusBackground`, `0x1445f0`)** : affiché en haut à droite `(535, 10)` avec le Naboo Fighter rotatif (`LASER`/`LASERS`, ou `TORPEDOS` pendant *Theed Hangar Escape*), grisé monochrome lorsqu'il ne reste plus de tir.
- **Spinners virtuels sur rampe (`VirtualSpinners` / `DeffVSpinner`, `0x17ead0`)** : totalement absents au repos ; activés **uniquement** lors d'un passage rampe (`LEFT_RAMP_ENTER`, `RIGHT_RAMP_ENTER`, `RAMP_MADE_LEFT`, `RAMP_MADE_RIGHT`) aux positions `(43, 174)` / `(421, 174)`, avec décélération progressive (~1,6 s), popup `+10000` / `+25000` et son cyclique de tourniquet DCS (`0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`).
- **Attract Mode** : enchaîne `20` écrans définis dans `AttractProcInfo` (`0x002d94d0`) — logos *Williams*, *Pinball 2000*, *Kahuna / Star Wars* + rayons lumineux colorés (9 couleurs RGB555), *SlideShow*, *High Scores*, *Jedi Table*, *Ramp Combo Champion*, *Crédits*, etc.
- **Ball Saver (`BallStartBallSaver`)** : actif au lancement de chaque bille ; si la bille draine avant expiration du timer, relance automatique sans perte de bille (`auto_ball_serve()`).

## 2. Skill Shot (`SkillShot`, `0x17c634`)
- Armé au début de la séquence de lancement (`SkillShot::hook_ball_serve_start`) si aucune scène/multiball ne bloque.
- Désarmé dès que le plateau devient valide (`SkillShot::hook_valid_playfield_start` -> `SkillShot::stop()`).
- **Validation** : passage par la *Sneaky Lane* (`SkillShot::hook_sneaky_lane` -> `SkillShot::award()`).
- **Score** : progressif selon le nombre de Skill Shots réussis dans la partie, **plafonné à `3 000 000` pts** (`0x2dc6c0`), et incrémente `aud_skill_shot`.

## 3. Bouclier et qualification des Scènes (`Shield`, `Scenes`, `0x177bf0..0x1787c8`)
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

## 4. Règles et scores de chaque scène

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

## 5. Lasers, Drop Targets, Jet Bumpers et Rampes
- **Lasers (`Lasers::fire_left` / `fire_right`, `0x146354` / `0x14648c`)** :
  - Utilisables dès qu'aucune intro de scène n'est active et que `pd_laser_count > 0`, avec cooldown de `60` ticks (`1,0 s`).
  - Décrémente le compteur de lasers, marque **`1 000` pts**, et simule un coup sur la Drop Target correspondante (`left_drop_target` / `right_drop_target`). Si la cible était déjà abaissée, propage le tir laser à la scène active.
- **Drop Targets (`Drop::switch_hit`)** : **`2 310` pts** par cible.
- **Jet Bumpers (`JetBumpers::jet_bumper_hit`, `0x10dc84`)** :
  - **Normal** : **`10 310` pts** (`0x2846`).
  - **Super Jets** : **`51 550` pts** (`0xc95e`), actif pendant `20 s` (`super_jets_ms = 20_000`) avec `LeffKind::SuperJetsStart` et `ard_super_jets_activated`.
  - Chaque coup allume une lettre de `G-U-N-G-A-N` (`0..=5`) ; le 6e coup remet le compteur à `0` et déclenche **Super Jets**.
- **Rampes & Combos (`Ramps`, `RampBlocker`)** :
  - Les rampes gauche/droite déclenchent `LeffKind::RampMade`, tourniquet virtuel (`DeffVSpinner`), fenêtre combo (`ramp_combo_window_ms`) et enchaînent les `next_combo_shot()` (`ramp_combo_count` / `ramp_combo_best`).
- **Build C-3PO & R2-D2 Hurry-Up (`BuildC3P0`, `R2D2Scene`)** :
  - `BuildC3P0` : assemblage séquentiel de `6` pièces (`c3p0_parts`), déclenchant `LeffKind::C3P0Complete` et `ard_c3p0_complete` à complétion.
  - `R2-D2` : score *Hurry-Up* progressif couplé aux appels `start_ard` / `hit_ard` / `complete_ard` et `LeffKind::SceneHit` / `SceneComplete`.

## 6. Lettres J-E-D-I, Jedi Multiball & Main Multiball
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

## 7. Extra Ball et Bonus de fin de bille
- **Extra Ball (`ExtraBall::award`, `0x118208`)** : décrémente le crédit d'Extra Ball allumé (temporaire ou joueur) et appelle `award_extra_ball()`, puis affiche `DeffExtraBallCollected`.
- **Bonus (`Bonus`, `DeffBonus::Run`, `0x10b554`)** :
  - `Total Bonus = build_bonus (0..=10) × build_bonus_x (1..=5) × 100 000` (plafonné à **`5 000 000` pts**).
  - Si `build_bonus == 10` et `build_bonus_x == 5`, joue la musique spéciale Bonus Maxed.
