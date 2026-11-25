# Référence Complète des Écrans, Layouts & Console de Debug (`pin2k`)

Ce document recense l'intégralité des écrans, coordonnées `640×240`, priorités Z, polices bitmap, couleurs RGB555, chorégraphies lampes (`machine/leff.rs`) et requêtes audio (`swe1_audio.rs`, `382` pools `ARD` / `695` `ACD`) extraits par rétro-ingénierie et décompilation Ghidra 12.2 de `work/swe1_0200.elf` et implémentés nativement en Rust pur dans `pin2k-core` + `pin2k-desktop`.

---

## 1. Architecture du Compositeur (`BaseViewManager` `640×240` → `640×480`)

Le matériel Williams Pinball 2000 (MediaGX + PRISM) compose chaque frame logique en **`640×240`** RGB555 (`0x7C1F` = magenta transparent) avant doublement vertical (`scan-doubler`) vers **`640×480`**.

- **Traits & commandes (`crates/pin2k-hal/src/lib.rs`)** :
  - `VideoBackend::begin_frame()` : réinitialise la pile des calques Z-ordonnés.
  - `SpriteDrawCmd` : `{ anim_name, frame, x, y, z, flip_x, scale_x, scale_y, monochrome_rgb555 }`.
  - `TextDrawCmd` : `{ font_name, text, x, y, z, color_rgb555, shadow_rgb555, align: TextAlign }` (`Left`, `Center`, `Right`).
  - `RectDrawCmd` : `{ x, y, w, h, z, color_rgb555, alpha }`.
- **Rendu (`crates/pin2k-desktop/src/compositor.rs`, `raster.rs`, `scenes_compositor.rs`)** :
  - Tous les sprites, rectangles et textes sont fusionnés et triés par priorité `z` croissante à chaque `present()`.
  - Le compositeur hérité (`render_specialized_scene`, utilisé notamment pour le rendu 3D multi-couches *Boonta Eve Podrace*) ne s'exécute que si **aucun** calque d'arrière-plan (`z <= 7`) n'a été empilé, ce qui garantit que le HUD (`z >= 9`), le Podrace HUD (`z = 8`) et les spinners (`z = 22..23`) se superposent proprement.

---

## 2. HUD en Partie (`InGame`)

### `DeffScoreDisplay` (`0x001725d8..0x001740fd`) — `crates/pin2k-core/src/machine/score_display.rs`
- **9 personnages joueurs (`player_name_table_data` @ `0x002dc798`)** : `Qui-Gon`, `Jar Jar`, `Anakin`, `Amidala`, `Obi-Wan`, `R2-D2`, `Yoda`, `Watto`, `Darth Maul` (attribution unique entre les 1..4 joueurs via `PlayerState::pick_unique_character`).
- **Positions exactes `Table<score_coord>` (`0x00173f66..0x001740dd`)**, `z = 10` :
  - Joueur 1 : `(36, 7)`, `TextAlign::Left` (laisser `x=535..635` libre à droite pour les Lasers)
  - Joueur 2 : `(525, 7)`, `TextAlign::Right`
  - Joueur 3 : `(36, 26)`, `TextAlign::Left`
  - Joueur 4 : `(525, 26)`, `TextAlign::Right`
- **Polices & couleurs** : nom en `font_system_7_data`, score formaté avec virgules (`1,234,560`) en `font_kahuna_shadow_9_data` ; joueur actif `0x7FFF` (ombre `0x72E6`), inactif `0x294A` (ombre `0x18C6`).
- **Sabres laser (`saber_process` @ `y = 7, z = 9`, centré `x = 280`)** :
  - Lame gauche : `anim_saber_{green,red,blue}_{start,loop,flare}` à `(16, 7)`
  - Poignée centrale : `anim_saber_handle` à `(261, 7)`
  - Lame droite : `anim_saber_{green,red,blue}_{start,loop,flare}` à `(300, 7)` avec `flip_x = true`
  - Barre d'info centrale (`m_info_message_process`) : `BALL N` (`x = 280, y = 9, z = 10`) et `FREE PLAY` / `CREDITS` (`x = 280, y = 26, z = 10`), `font_system_7_data`, `0x7E00` (ombre `0x6540`).

### `DeffLaserStatusBackground` (`0x001445f0..0x00144d64`) — `crates/pin2k-core/src/machine/hud_display.rs`
- Vaisseau cible rotatif `anim_naboo_fighter_tiny_rotate` (`100×50`) en `(535, 10)` (`0x27B - 100`), `z = 20`.
- Nombre de tirs restants `(585, 35)` et libellé `LASER` / `LASERS` / `TORPEDOS` (pendant *Hangar Escape*) `(585, 48)`, `font_system_7_data`, `z = 21`, jaune `0x7FE0` quand prêt ; grisé (`monochrome_rgb555 = Some(0x294A)` et texte `0x294A`) à `0`.

### `VirtualSpinners` / `DeffVSpinner` (`0x0017ead0..0x0017fa04`) — `crates/pin2k-core/src/machine/vspinner.rs`
- **Comportement d'origine** : totalement inactifs au repos ; déclenchés **uniquement** lors d'un passage rampe (`LEFT_RAMP_ENTER`, `RIGHT_RAMP_ENTER`, `RAMP_MADE_LEFT`, `RAMP_MADE_RIGHT`).
- **Positions & calques** :
  - Gauche : `anim_spinner_bracket_left` + `anim_spinner_1` en `(43, 174)`, `z = 22`.
  - Droite : `anim_spinner_bracket_right` + `anim_spinner_2` en `(421, 174)`, `z = 22`.
  - Haute vitesse : `anim_spinner_lightning` (`x + 6, y = 162, z = 22`).
  - Popup montant `+10000` / `+25000` : `font_system_7_data`, `z = 23`, couleur `0x7FE0`.
- **Audio** : rotation des 4 sons de tourniquet `sound::SPINNER_TICKS` (`0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`) à chaque demi-tour.

---

## 3. Attract Mode — 20 Écrans (`AttractProcInfo` @ `0x002d94d0`)

Implémenté dans `crates/pin2k-core/src/attract.rs` et `crates/pin2k-core/src/machine/attract_render.rs`. Chaque slide superpose `DeffAttractModePriceBackground` (`z = 250`, `font_system_9_data`, `FREE PLAY / PRESS START` ou `INSERT COINS` en `y = 210 / 225`).

| Index | Commande Debug | Classe `Deff` / Contenu | Éléments visuels principaux |
|---:|---|---|---|
| 0 | `attract:0` | `DeffAttractModeKahunaLogo` | `anim_kahuna_logo_flare` `(23,7,z=7)` interpolé sur 9 RGB555 (`0x100fc0`: `0x7C06, 0x0320, 0x001F, ...`) + `anim_kahuna_logo_spinning` `(187,80,z=8)` |
| 1 | `attract:1` | `DeffAttractModeSlideShow` | Bannières panoramiques `anim_starwars_banner_1..4` |
| 2 | `attract:2` | `DeffAttractModeHSTD` | Top scores HSTD avec rangs Jedi (`font_starwars_19_data` + `font_shadow_15_data`) |
| 3..6 | `attract:3..6` | `DeffAtMJediTable` | Classements `Youth`, `Knight`, `Master`, `Spirit` (`anim_bonus_icon_jedi_*`) |
| 7 | `attract:7` | `DeffAttractModeRampComboChampion` | Champion Ramp Combo + `anim_naboo_flyby_single` |
| 8 | `attract:8` | `DeffAttractModeP2KLogo` | Animation `anim_p2k_logo_animated` |
| 9 | `attract:9` | `DeffAttractModeHasbroAd` | Promotion Hasbro / Jedi Knight |
| 10 | `attract:10` | `DeffAttractModeScoreAwards` | Replay & Extra Ball awards |
| 11 | `attract:11` | `DeffAttractModeEmailAddress` | `WWW.PINBALL.COM` / coordonnées Williams |
| 12 | `attract:12` | `DeffAttractModeWebPage` | Page web avec portrait Anakin |
| 13 | `attract:13` | `DeffAttractModeScores` | Scores de la dernière partie (1..4 joueurs + personnage) |
| 14..17 | `attract:14..17` | `DeffAttractModeGameCredits1..4` | Crédits Williams Electronics, Lucasfilm Ltd., équipe & design |
| 18 | `attract:18` | `DeffCustomMessage` | Message opérateur personnalisé |
| 19 | `attract:19` | `DeffHTPScene` | How To Play (bouclier → holoprojecteur → scènes) |

---

## 4. Toutes les 14 Scènes SWE1 (`scene:0`..`13`), Holo Select & Multiballs

Implémenté dans `crates/pin2k-core/src/swe1_rules.rs` et `crates/pin2k-core/src/machine/scene_render.rs`.

- **Intro de scène (`Deff*SceneIntro`, tant que `intro_remaining_ms > 0`)** :
  - Film 15 fps (`anim_*_intro_{movie,battle}`) en `(76, 40, z=200)` derrière le cadre,
  - Badge de scène (`intro_badge_anim()`) en `(290, 74, z=201)`,
  - Cadre double ovale `anim_jedi_scene_intro_frame` en `(59, 22, z=202)` masquant les bords rectangulaires,
  - Titre (`z=203, font_shadow_15_data`) et consigne (`z=203, font_shadow_11_data`).

| Index | Debug Mode | Titre | Badge Intro (`z=201`) | Arrière-plan actif (`z=1..8`) |
|---:|---|---|---|---|
| 0 | `scene:0` | Battle Droid Invasion | `anim_battle_droid_intro_badge` | `anim_bdroid_background` + droids + `anim_bdroid_on_stap` + `anim_stap_xhair` + zap |
| 1 | `scene:1` | Destroyer Droids | `anim_destroyer_droid_intro_badge` | `anim_destroyer_droid_background` + `s_idle`/`m_idle`/`l_roll` |
| 2 | `scene:2` | Federation Fighters | `anim_federation_fighter_intro_badge` | `anim_federation_fighter_background` (scroll) + `anim_droid_control_ship` + chasseur + `anim_naboo_xhair` |
| 3 | `scene:3` | Gungan Energy Battle | `anim_sub_escape_intro_badge` | `anim_gungan_seaweed` + `anim_gungan_sub_bubbles_2` + `sub` + `opee` + texte `G U N G A N` |
| 4 | `scene:4` | Theed Hangar Escape | `anim_hangar_escape_intro_badge` | `anim_federation_fighter_cockpit` + `hangar_droid_run` + `torpedo` + explosions |
| 5 | `scene:5` | AAT Hover Tank | `anim_hover_tank_intro_badge` | `anim_hover_tank_background` + `l/r/c_float` + `catapult_idle` + `boomer_ball` |
| 6 | `scene:6` | Boonta Eve Podrace | `anim_podrace_intro_badge` | `SoftwareCompositor::render_podrace_scene` (`z<=7` libre) + HUD `LAP`/timer à `z=8` |
| 7 | `scene:7` | Sith Probe Droids | `anim_probe_droid_intro_badge` | `anim_sith_probe_background` (scroll) + `hover` + `queen_shoot_1` + `sith_probe_xhair` |
| 8 | `scene:8` | Queen Amidala Wardrobe | `anim_queen_ship_intro_badge` | `anim_fashion_base` + `throne` + `queen` + `wheel` + `sparkle_{left,right}` |
| 9 | `scene:9` | Jar Jar Juggle | `anim_jarjar_juggle_intro_badge` | `anim_jarjar_juggle_backdrop` + `jarjar_stare` + `jarjar_juggle` |
| 10 | `scene:10` | Watto's Chance Cube | `anim_wattos_chance_intro_badge` | `anim_watto_scene_background` + `chin_loop_middle` + `cube` + `cube_flash` |
| 11 | `scene:11` | Mos Espa Cantina | `anim_musical_chairs_intro_badge` | `anim_jedi_council_background` + Mace/Yoda/membres/neck |
| 12 | `scene:12` | R2-D2 Repair | `anim_r2d2_intro_badge` | `anim_r2d2_scene_background_{upper,lower}` + `r2d2_unit` + unités bleue/rouge |
| 13 | `scene:13` | Mystery (`?`) | `anim_question_mark_intro_badge` | `anim_question_mark_blue` + `anim_question_mark_green` + `MYSTERY AWARD` |

- **Autres vues in-game (`scene_render.rs`)** :
  - `holo_select` (`DeffObiHolo`) : `anim_obiwan_holo_glow` + `anim_obiwan_holo` + `anim_obiwan_holo_glow_loop` + miniature `anim_obiwan_holo_*`.
  - `hangar_multiball` : intro `anim_multiball_start_movie` (`scale_x=2`, 3 s) puis `anim_jackpot_icon_{idle,double,super}`.
  - `darth_maul_duel` (`DeffJediMultiballBackground`) : `anim_jedi_battle_background_hangar` + icônes Qui-Gon/Maul + animation combat + étincelles + jauge HP `RectDrawCmd`.
  - `skill_shot` : `anim_super_skill_shot_icon` `(240, 68, z=4)`.

---

## 5. Écrans Spéciaux, Système & Diagnostics (`system_screens.rs`)

| Commande Debug | Classe `Deff` | Contenu & calques |
|---|---|---|
| `boot` | `PowerupTestingDeff` | Écran de démarrage Williams Pinball 2000 / PRISM / PDB self-test (`font_system_*`) |
| `bonus` | `DeffBonus` | `anim_bonus_background` + portrait `anim_bonus_character_*` + badge `anim_bonus_icon_jedi_*` + détail base × mult |
| `match` | `DeffMatch` | `anim_match_movie` (`scale_x=2`) + roulement 2 chiffres `font_starwars_19_data` + comparaison joueur |
| `game_over` | `DeffGameOver` | `anim_darth_maul_game_over` (`scale_x=2`) + `GAME OVER` + score final |
| `game_intro` | `DeffGameIntro` | `anim_republic_cruiser_intro` (`scale_x=2`) + `STAR WARS EPISODE I` |
| `extra_ball` | `DeffExtraBallCollected` | `anim_extra_ball_show` (`scale_x=2`) + `EXTRA BALL!` |
| `ball_save` | `DeffBSBallSaved` | Overlay centré `BALL SAVED / SHOOT AGAIN` (`z=23..24`) |
| `tilt_warning` | `DeffTiltWarning` | Overlay alerte `DANGER! DANGER!` (`z=239..240`) |
| `tilt` | `DeffTilt` | Voile translucide + `TILT / BALL LOST` (`z=239..240`) |
| `slam_tilt` | `DeffSlamTilt` | Plein écran `SLAM TILT` (`z=239..240`) |
| `sub_escape` | `DeffSubEscapeSceneBackground` | Poursuite sous-marine Naboo (`seaweed`, `bubbles_2`, `sub`, `chomp`) |
| `build_c3p0` | `DeffBuildC3P0Status` | `anim_build_c3p0` + consigne Build C-3PO |
| `ramp_combo` | `DeffRampCombo` | Passage `anim_naboo_flyby_single` + `RAMP COMBO!` |
| `jedi_ready` | `DeffJediMultiballReadyBackground` | `anim_saber_ball_loop` + `JEDI MULTIBALL READY!` |
| `hstd_entry` | `DeffHSTDinitials` | Cadre navy, 3 cases d'initiales (`font_starwars_19_data`), bande `A..Z 0..9` |
| `diag:align` | Video Align | Mire ROM `system_video_test_align` (`640×240`) |
| `diag:colorbar` | `DiagnosticColorBarDeff` | 8 barres SMPTE RGB555 pleine hauteur |
| `diag:grid` | `DiagnosticGridDeff` | Grille de convergence blanche sur fond noir + bordure rouge |
| `diag:rgb` | `DiagnosticRGBDeff` | 3 bandes pureté Rouge / Vert / Bleu |
| `diag:fuse` | `DiagnosticFuseDeff` | État des fusibles `F100..F105` + verrou porte monnayeur |
| `diag:switch` | `DiagnosticSwitchTestDeff` | Matrice live 12×8 colonnes (verts = fermés) + dernier switch nommé |
| `diag:lamp` | `DiagnosticSingleLampDeff` | Matrices A & B (128 lampes) avec surbrillance sélection |
| `diag:coil` / `diag:sound` / `admin` / `debug` | `MenuGUIBackgndDeff` | Menus opérateur complets navigables par boutons de porte (`1/2/3/4`) |

---

## 6. Console de Debug Web (`--web-port`)

Démarrée avec `--web-port 8080`, la console sert une interface 5 onglets ([web_ui.html](file:///home/developer/pin2k/crates/pin2k-desktop/src/web_ui.html)) pilotée par [web_ui.rs](file:///home/developer/pin2k/crates/pin2k-desktop/src/web_ui.rs) (avec décodage URL `percent_decode` des paramètres tels que `scene%3A0`) :

1. **Onglet 1 (`Modes & Écrans`)** : lancement direct de tous les états, 20 slides Attract, 14 scènes SWE1, Multiballs, écrans spéciaux, mires & diagnostics, et modes RFM.
2. **Onglet 2 (`Joueurs & Temps`)** : Pause/Reprendre, Avancer `+1 Frame (16 ms)`, ajout de score, ajout/changement de joueur (`1..4`), sélection bille (`1..3`), lasers (`0/5/9`), bouclier, lettres `J-E-D-I`, rang Jedi.
3. **Onglet 3 (`Anims & Sons`)** : explorateur interactif alimenté par `/api/assets` (392 animations + sons DCS).
4. **Onglet 4 (`Matériel`)** : impulsion de bobines, pilotage de lampes, déclenchement de switchs plateau.
5. **Onglet 5 (`Couches Live`)** : inspection temps réel des calques Z actifs de la frame courante.

**Endpoints HTTP principaux :**
- `GET /api/state` : état complet JSON (jeu, état machine, joueurs, spinners, bannière, `layers[]` live, pause, bobines/lampes).
- `GET /api/assets` : catalogue JSON des animations, polices et sons DCS chargés.
- `GET /api/events` & `GET /api/db` : flux d'événements et base persistante `pin2k_db.json`.
- `POST /api/debug/mode/{target}`, `POST /api/debug/anim/{name}`, `POST /api/debug/sound/{id}`, `POST /api/debug/score/{pts}`, `POST /api/debug/drive/{id}/{ms}`, `POST /api/debug/lamp/{id}/{val}`, `POST /api/debug/player/{cmd}`, `POST /api/debug/pause`, `POST /api/debug/step`, `POST /api/switch/{id}`.
