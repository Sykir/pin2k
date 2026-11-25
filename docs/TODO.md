# Roadmap & TODO (`pin2k`)

Suivi de l'avancement du portage natif Rust **Pinball 2000** (*Star Wars Episode I* `#50069` & *Revenge From Mars* `#50070`).

## Avancement Global : **~84 %**

| Chantier | Avancement | Statut |
|---|---:|---|
| 1. Outil ROM & Extraction (`pin2k-rom-tool`) | **95 %** | Layouts UI + Gamecode + Assets complets |
| 2. Abstraction Matérielle & PDB (`pin2k-hal`) | **98 %** | Prêt cabinet, série & compositeur Z-trié |
| 3. Moteur de Jeu & Écrans Natifs (`pin2k-core`) | **78 %** | 100 % des écrans/scènes SWE1 natifs ; finalisation VM `ScriptRuntime` et migration data-driven en cours |
| 4. Couverture des Assets SWE1 (`gfx` / `fonts` / `audio`) | **47 %** | Polices 100 % (`19/19`), Anims 47 % (`186/392`), Audio 6 % (`47/689`) |
| 5. Runtime Desktop, Console Debug & HD (`pin2k-desktop`) | **78 %** | Web Console 5 onglets & PNG OK ; audio natif/Web et fenêtre native `--window`/`--fullscreen` à finaliser |
| 6. Modding d'Assets & Scènes Custom (`mod.json`) | **30 %** | Remplacement `gfx/` & audio natif OK ; WebAudio mods, polices, règles à états et scènes custom à faire |
| 7. Documentation & Qualité | **100 %** | 39/39 tests OK, 0 avertissement, documentation complète à jour |

---

## 1. Outil ROM & Extraction (`pin2k-rom-tool`) — **95 %**
- [x] Reconstruction PRISM (`U100..U107` → 64 Mio) et DCS (`U109/U110` → 8 Mio)
- [x] Lecture des mises à jour officielles/communautaires (`.exe`, `.zip`, dossiers) + vérification des 12 checksums
- [x] Export ELF32 x86 symbolisé (`wrap-elf`)
- [x] Extraction complète des animations (`extract-assets`, toutes les frames par défaut, codecs `0..45`)
- [x] Extraction des **19 polices bitmap** (`FontData` / `Character` → atlas PNG + `fonts_manifest.json`)
- [x] Extraction audio DCS-2 (`*_sound.bin` `pb2kslib` → `.ogg` + `sound_manifest.json`)
- [x] Extraction gamecode (`SwitchTable`, `DriveTable`, `LampTable`, désassembleur `lift_x86_calls` → `game_script.json`)
- [x] **Extraction complète des layouts UI ROM (`extract-layout`)** : parseur ELF32/x86-32 pur-Rust (`layout_extractor.rs`) exportant `rom_layout_db.json` (138 `Deff*`, 14 scènes, 20 slides Attract, 9 noms joueurs, 702 `ACD`, 383 `ARD`)
- [x] Découpage modulaire (`main.rs`, `cli.rs`, `commands.rs`, `layout_extractor.rs`)
- [ ] Étendre `extract-gamecode` pour exporter 100 % des règles scénaristiques SWE1 & RFM dans `game_script.json`
- [ ] Décodage audio DCS-2 brut depuis `u109`/`u110` (fallback sans `*_sound.bin`)
- [ ] Sous-commande `upscale-assets` (`xBRZ` / `Lanczos` + *edge-padding* alpha → `gfx_hd/`)

---

## 2. Abstraction Matérielle & PDB (`pin2k-hal`) — **98 %**
- [x] Protocole filaire exact Power Driver Board (`PinIOWrite` / `PinIORead`, 96 switchs, 48 bobines, 128 lampes)
- [x] Gestion des optos actifs-bas (`SWE1_OPTO_NC_MASK`) et verrou haute tension porte monnayeur
- [x] Synchronisation initiale `hw.sync(0)` avant amorçage `SwitchDebouncer` (suppression des faux fronts à la frame 2)
- [x] Interface `VideoBackend` Z-ordonnée (`SpriteDrawCmd`, `TextDrawCmd`, `RectDrawCmd`, `TextAlign`, `monochrome_rgb555`, `flip_x`, `scale_x/y`)
- [x] Port parallèle Linux (`/dev/parportN` `ppdev`, `/dev/port`), `VirtualPdbPort` + `--lpt-trace`
- [x] Pont Série multiplateforme Windows (`COMx`) & Linux (`/dev/ttyUSB0`) sans dépendance C
- [ ] Validation terrain longue durée sur carte PDB physique + pont microcontrôleur UART

---

## 3. Moteur de Jeu & Écrans Natifs (`pin2k-core`) — **78 %**
- [x] Architecture modulaire complète (`machine/{mod, score_display, hud_display, vspinner, attract_render, scene_render, system_screens, render, script_exec, swe1_playfield, rfm_playfield, admin_nav, tests}.rs`)
- [x] HUD authentique : `DeffScoreDisplay` (9 noms Star Wars uniques, scores avec virgules, sabres gauche/poignée/droite) et `DeffLaserStatusBackground` (vaisseau Naboo + `LASERS`/`TORPEDOS`)
- [x] Spinners virtuels fidèles (`VirtualSpinners` / `DeffVSpinner`) : déclenchés **uniquement sur passage rampe** avec rotation des 4 sons tourniquet DCS (`0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`)
- [x] **Attract Mode complet (20 écrans `AttractDeff`)** incluant `DeffAttractModeKahunaLogo` + rayons lumineux 9 couleurs RGB555
- [x] **Toutes les 14 scènes SWE1** (intros bezel `anim_jedi_scene_intro_frame` + 14 arrière-plans actifs incluant `R2D2` et `QuestionMark`), `DeffObiHolo`, `HangarMultiball`, `DarthMaulDuel`, `Super Skill Shot`
- [x] **Tous les écrans spéciaux & Diagnostics** : `Boot` (`PowerupTestingDeff`), 12 `SpecialScreen` (`Bonus`, `Match`, `GameOver`, `GameIntro`, `ExtraBall`, `BallSave`, `TiltWarning`, `SlamTilt`, `SubEscape`, `BuildC3P0`, `RampCombo`, `JediMultiballReady`), `DeffTilt`, `DeffHSTDinitials`, et `DiagPage` (`Align`, `ColorBar`, `Grid`, `Rgb`, `Fuse`, `Switch`, `Coil`, `Lamp`, menus Opérateur)
- [x] VM de scripts `ScriptRuntime` + persistance `pin2k_db.json`
- [ ] **Finaliser le câblage de `ScriptRuntime` dans `Pin2kMachine`** :
  - Déclencher `OnBallStart`, `OnBallEnd`, `OnTick` et décrémenter `rt.timers` (`OnTimer`) ainsi que `rt.active_modes` (`OnModeExit`) dans `update_timers()`, et brancher `tick_attract()` lorsque le script pilote l'Attract
  - Isoler les variables par joueur (`player_vars` vs `global_vars`) pour le support multijoueur 1–4 joueurs dans `ScriptRuntime`
  - Clarifier la cohabitation natif/script par défaut : éviter que `game_script.json` court-circuite silencieusement les switchs SWE1 natifs (ex. rampes `36`/`37`, bouclier `39`) tant que `--strict-script` (ou un mode explicite) n'est pas activé
- [ ] **Unifier le compositing des scènes dans `pin2k-core`** : rapatrier `render_podrace_scene` dans `crates/pin2k-core/src/machine/scene_render.rs` et supprimer le doublon historique `crates/pin2k-desktop/src/scenes_compositor.rs`
- [ ] **Passage en 100 % Data-Driven** : déplacer toutes les descriptions visuelles et règles restantes vers `game_script.json` / `rom_layout_db.json` / manifestes JSON et activer `--strict-script` par défaut

---

## 4. Couverture des Assets SWE1 (`docs/ASSETS_MATRIX.md`) — **47 %**
- [x] **Polices Bitmap (`fonts/`)** : **19 / 19 (100 %)** chargées et utilisées avec ombres & alignement dans `SoftwareCompositor`
- [x] **Animations & Sprites (`gfx/`)** : **186 / 392 (47 %)** utilisées (toutes les 14 scènes intros+fonds, Attract complet, sabres, Naboo HUD, spinners, Bonus personnages/rangs, Match, Game Over, Extra Ball, Multiballs, Holo Obi-Wan, mire alignement)
- [ ] **Pistes Audio DCS-2 (`audio/`)** : **47 / 689 (6 %)** utilisées (thèmes, musiques de scènes, tourniquets, tirs, explosions) → reste les répliques vocales contextuelles secondaires

---

## 5. Runtime Desktop, Console Debug & Rendu HD (`pin2k-desktop`) — **78 %**
- [x] Compositeur logiciel `BaseViewManager` `640×240` → `640×480` trié par Z (`raster.rs`, `compositor.rs`, `scenes_compositor.rs`)
- [x] **Console de Debug Web 5 onglets** (`web_ui.rs` + `web_ui.html`) : lancement direct de tous modes/scènes/diagnostics (avec décodage `%3A`), Pause/Step, mutations joueurs/temps, explorateur `/api/assets`, bobines/lampes, inspecteur `Couches Live`
- [x] Audio natif pur-Rust (`lewton` → `winmm.dll` Windows / `aplay` Linux) + WebAudio
- [x] Compilation croisée Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) et Linux
- [ ] Corriger `AudioCmd::StopAll` et le support `looping` dans le worker audio natif (`crates/pin2k-desktop/src/audio.rs`)
- [ ] Exposer `override_paths` audio des mods dans `WebAudioBridge` et la route HTTP `/audio/` (`crates/pin2k-desktop/src/web_ui.rs`)
- [ ] **Affichage Natif Desktop Fenêtré & Plein Écran (`--window` / `--fullscreen`)** (`winit` + `softbuffer`/`wgpu`, ratio `4:3`, `F11` / `Alt+Entrée`)
- [ ] **Rendu HD & Upscaling** (voir [HD_UPSCALING_GUIDE.md](HD_UPSCALING_GUIDE.md)) : shaders WebGL2 côté navigateur + support `gfx_hd/` + cache LRU

---

## 6. Modding d'Assets & Scènes Custom (`mod.json`) — **30 %**
- [x] Remplacement d'animations PNG (`<mod>/gfx/<nom>/*.png` + `anim_overrides`) et empilement de `--mod`
- [x] Overrides audio `.ogg` côté lecture native OS (`sound_overrides`) et règles simples `HomebrewSwitchRule`
- [ ] **Modding graphique (`gfx/` & `fonts/`)** :
  - Supporter le chargement et l'override de polices bitmap `<mod>/fonts/` dans `SoftwareCompositor`
  - Ajouter un mini-manifeste par animation dans `mod.json` (`fps`, `loop`, `scale` / densité HD, `pivot`) au lieu de figer les animations mod à `30` FPS
- [ ] **Modding audio (`audio/`)** : support complet des `sound_overrides` dans l'interface Web (`--web-port`, en lien avec la section 5) et adressage optionnel par nom symbolique en plus des IDs DCS numériques
- [ ] **Modding de règles (`mod.json`)** : préserver `attract_banner` en mode SWE1 (sans écrasement dans `update_timers()`) et étendre `switch_rules` (conditions, extinction de lampes)
- [ ] **Registre dynamique & format de Scènes Custom déclaratif** :
  - Remplacer/compléter l'enum fermé `Swe1Scene` par un registre alimenté par JSON (et à terme `rom_layout_db.json`) sélectionnable dans l'holoprojecteur (`DeffObiHolo`)
  - Décrire chaque scène via ses **phases** (`intro`, `active`, `victory`, `timeout`), ses **calques Z-triés** (`SpriteDrawCmd`, `TextDrawCmd` avec interpolation `{hits}`/`{secs}`/`{score}`, `RectDrawCmd`) et ses **règles locales de switchs**
- [ ] **Scripting avancé (moyen terme)** : évaluer/intégrer un moteur de script pur-Rust (ex. **Rhai**, zéro dépendance C) branché sur `VideoBackend` / `PinballHardware` pour les mini-jeux vidéo et scènes custom à logique arbitraire

---

## 7. Documentation & Qualité — **100 %**
- [x] `RUSTFLAGS="-D warnings" cargo check --workspace --all-targets` (**0 avertissement**) et **39 / 39 tests unitaires** verts
- [x] Validation visuelle sur **11 captures PNG** (`attract:0`, `ingame`, `spinner_left`, `scene:6`, `scene:0`, `scene:12`, `bonus`, `match`, `hstd_entry`, `diag:colorbar`, `diag:switch`)
- [x] [README.md](../README.md) & [SCREEN_LAYOUT_REFERENCE.md](SCREEN_LAYOUT_REFERENCE.md)
- [x] [ARCHITECTURE_GUIDE.md](ARCHITECTURE_GUIDE.md) & [HARDWARE_NOTES.md](HARDWARE_NOTES.md)
- [x] [GAME_SCRIPT_REFERENCE.md](GAME_SCRIPT_REFERENCE.md) & [ASSETS_MATRIX.md](ASSETS_MATRIX.md)
- [x] [SWE1_EXTRACTION_INVENTORY.md](SWE1_EXTRACTION_INVENTORY.md) & [SWE1_GAME_RULES.md](SWE1_GAME_RULES.md)
- [x] [RFM_EXTRACTION_INVENTORY.md](RFM_EXTRACTION_INVENTORY.md) & [RFM_GAME_RULES.md](RFM_GAME_RULES.md)
- [x] [HD_UPSCALING_GUIDE.md](HD_UPSCALING_GUIDE.md) & [TODO.md](TODO.md)
