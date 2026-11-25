# Roadmap & TODO (`pin2k`)

Suivi de l'avancement du portage natif Rust **Pinball 2000** (*Star Wars Episode I* `#50069` & *Revenge From Mars* `#50070`).

## Avancement Global : **~91 %** | Indice de Jouabilité : **87,4 / 100**

| Chantier | Avancement | Statut |
|---|---:|---|
| 1. Outil ROM & Extraction (`pin2k-rom-tool`) | **96 %** | Layouts UI + Gamecode + Assets complets + pipeline Ghidra 12.2 (`1 867` fonctions / `248` classes) |
| 2. Abstraction Matérielle & PDB (`pin2k-hal`) | **98 %** | Prêt cabinet, série & compositeur Z-trié |
| 3. Moteur de Jeu & Écrans Natifs (`pin2k-core`) | **90 %** | 100 % des écrans/scènes SWE1 + banque audio `ARD`/`ACD` + moteurs `Leff` & `FX` + règles portées depuis Ghidra |
| 4. Couverture des Assets SWE1 (`gfx` / `fonts` / `audio`) | **97 %** | Polices **100 % (`19/19`)**, Audio **100 % (`689/689`)**, Anims **90 % (`354/392`)** |
| 5. Runtime Desktop, Console Debug & HD (`pin2k-desktop`) | **78 %** | Web Console 5 onglets & PNG OK ; audio natif/Web et fenêtre native `--window`/`--fullscreen` à finaliser |
| 6. Modding d'Assets & Scènes Custom (`mod.json`) | **30 %** | Remplacement `gfx/` & audio natif OK ; WebAudio mods, polices, règles à états et scènes custom à faire |
| 7. Documentation & Qualité | **100 %** | **42/42 tests OK**, E2E vidéo **14/14**, 0 avertissement, documentation complète à jour |

### Indice de Jouabilité (`Playability Score`) : **87,4 / 100**

> *Web / Émulation (`--web-port`) : **~90 / 100** — Cabinet physique (PDB + écran natif) : **~82 / 100***

| Critère de jouabilité | Poids | Niveau | Points | État actuel |
|---|---:|---:|---:|---|
| **1. Boucle de partie & Règles SWE1** | 25 % | 92 % | `23,0 / 25` | 1–4 joueurs, Skill Shot, 14 scènes, Super Jets, Combos, C-3PO, J-E-D-I, Multiballs, Bonus/Match/HSTD, Tilt, Ball Save |
| **2. Fidélité Visuelle & HUD (`640×240`)** | 20 % | 92 % | `18,4 / 20` | `DeffScoreDisplay`, `DeffLaserStatusBackground`, `DeffVSpinner`, 20 slides Attract, intros isolées, `354/392` anims, `19/19` polices |
| **3. Audio & Réactivité Sonore** | 15 % | 80 % | `12,0 / 15` | `689/689` pistes DCS-2 (`382` pools `ARD`), WebAudio OK ; reste fix `StopAll`/bouclage en audio natif OS |
| **4. Contrôles, Latence & Affichage** | 15 % | 75 % | `11,3 / 15` | Clavier Web + Debug Console OK ; manque la fenêtre native `--window`/`--fullscreen` (60 Hz direct) |
| **5. Lampes, Bobines & Matériel PDB** | 15 % | 88 % | `13,2 / 15` | Protocole PDB exact, `14` `Leff*`, optos NC, LPT Linux + Série Win/Linux ; validation terrain longue durée restante |
| **6. Stabilité, Sauvegarde & Diagnostics** | 10 % | 95 % | `9,5 / 10` | Persistance `pin2k_db.json`, tous écrans Diagnostics/Admin, `42/42` tests + E2E vidéo `14/14` verts |

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

## 3. Moteur de Jeu & Écrans Natifs (`pin2k-core`) — **86 %**
- [x] Architecture modulaire complète (`machine/{mod, leff, score_display, hud_display, vspinner, attract_render, scene_render, system_screens, render, script_exec, swe1_playfield, rfm_playfield, admin_nav, tests}.rs`)
- [x] **Banque audio complète extraite de la ROM (`swe1_audio.rs`)** : `382` pools `ARD` (`407` IDs DCS-2 uniques) + `695` descripteurs `ACD` avec `resolve_ard()`, `resolve_acd()` et `Pin2kMachine::emit_ard()`
- [x] **Moteur de chorégraphie lampes/flashers (`machine/leff.rs`, 14 `LeffKind`)** : `RampMade`, `MainMultiballStart`, `MainMultiballJackpot`, `JediMultiballClash`, `JediMultiballFinish`, `SkillShotAward`, `SuperJetsStart`, `ShieldHit`, `LaserFired`, `SceneHit`, `SceneComplete`, `C3P0Complete`, `ExtraBallCollected`, `BonusShow`
- [x] **Règles SWE1 portées depuis la décompilation Ghidra 12.2** (`1 867` fonctions / `248` classes de `swe1_0200.elf`) : `SkillShot` progressif (`500 000`..`3 000 000`), `G-U-N-G-A-N` Super Jets (`20 s`, `51 550` pts), combos de rampes, `BuildC3P0` (`6` pièces), `JediLetters` (`500 000 + 50 000*n` + `2 000 000`), `Multiball` (`5 000 000` Super Jackpot), `JediMultiball`, `QuestionMark`, `R2D2` Hurry-Up, et `start_ard`/`hit_ard`/`complete_ard` + scores authentiques des 14 scènes
- [x] HUD authentique : `DeffScoreDisplay` (9 noms Star Wars uniques, scores avec virgules, sabres gauche/poignée/droite) et `DeffLaserStatusBackground` (vaisseau Naboo + `LASERS`/`TORPEDOS`)
- [x] Spinners virtuels fidèles (`VirtualSpinners` / `DeffVSpinner`) : déclenchés **uniquement sur passage rampe** avec rotation des 4 sons tourniquet DCS (`0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`)
- [x] **Attract Mode complet (20 écrans `AttractDeff`)** incluant `DeffAttractModeKahunaLogo` + rayons lumineux 9 couleurs RGB555
- [x] **Toutes les 14 scènes SWE1** (intros bezel `anim_jedi_scene_intro_frame` + 14 arrière-plans actifs incluant `R2D2` et `QuestionMark`), `DeffObiHolo`, `HangarMultiball`, `DarthMaulDuel`, `Super Skill Shot`
- [x] **Tous les écrans spéciaux & Diagnostics** : `Boot` (`PowerupTestingDeff`), 12 `SpecialScreen` (`Bonus`, `Match`, `GameOver`, `GameIntro`, `ExtraBall`, `BallSave`, `TiltWarning`, `SlamTilt`, `SubEscape`, `BuildC3P0`, `RampCombo`, `JediMultiballReady`), `DeffTilt`, `DeffHSTDinitials`, et `DiagPage` (`Align`, `ColorBar`, `Grid`, `Rgb`, `Fuse`, `Switch`, `Coil`, `Lamp`, menus Opérateur)
- [x] VM de scripts `ScriptRuntime` + persistance `pin2k_db.json`
- [x] **Cohabitation natif / `game_script.json`** : `game_script.json` ne court-circuite plus les switchs SWE1/RFM natifs tant que `--strict-script` n'est pas activé (modes `Homebrew` et `mod.json` inchangés)
- [x] **Séparation stricte Intro de scène / Scène active** : fond opaque `z=0`, vidéo `220×160` étirée `2×` centrée dans le cadre ovale `anim_jedi_scene_intro_frame`, badge centré sur le rebord inférieur, HUD masqué pendant les intros & écrans plein écran puis affiché dès l'entrée en scène active
- [ ] **Finaliser le câblage de `ScriptRuntime` dans `Pin2kMachine`** :
  - Déclencher `OnBallStart`, `OnBallEnd`, `OnTick` et décrémenter `rt.timers` (`OnTimer`) ainsi que `rt.active_modes` (`OnModeExit`) dans `update_timers()`, et brancher `tick_attract()` lorsque le script pilote l'Attract
  - Isoler les variables par joueur (`player_vars` vs `global_vars`) pour le support multijoueur 1–4 joueurs dans `ScriptRuntime`
- [ ] **Unifier le compositing des scènes dans `pin2k-core`** : rapatrier `render_podrace_scene` dans `crates/pin2k-core/src/machine/scene_render.rs` et supprimer le doublon historique `crates/pin2k-desktop/src/scenes_compositor.rs`
- [ ] **Passage en 100 % Data-Driven** : déplacer toutes les descriptions visuelles et règles restantes vers `game_script.json` / `rom_layout_db.json` / manifestes JSON et activer `--strict-script` par défaut

---

## 4. Couverture des Assets SWE1 (`docs/ASSETS_MATRIX.md`) — **97 %**
- [x] **Polices Bitmap (`fonts/`)** : **19 / 19 (100 %)** chargées et utilisées avec ombres & alignement dans `SoftwareCompositor`
- [x] **Pistes Audio DCS-2 (`audio/`)** : **689 / 689 (100 %)** indexées dans `swe1_audio.rs` (`382` pools `ARD` / `695` `ACD`) et déclenchables via `Pin2kMachine::emit_ard()`
- [x] **Animations & Sprites (`gfx/`)** : **354 / 392 (90,3 %)** utilisées (toutes les 14 scènes intros+fonds+explosions+victoires, `DeffWattoJunkShop`, `DeffMarketplace`, `DeffJarJar`, `DeffBumperSparks`, `DeffJediMultiballIntro`, `JediSpirit`, Attract complet, sabres, Naboo HUD, spinners, Bonus, Match, Game Over, Extra Ball, Multiballs, Holo Obi-Wan, mire alignement)

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
- [x] `RUSTFLAGS="-D warnings" cargo check --workspace --all-targets` (**0 avertissement**), **42 / 42 tests unitaires** verts et `cargo build --release --workspace` OK
- [x] Validation fonctionnelle **End-to-End 14 / 14 étapes** enregistrée en vidéo MP4 (`work/e2e_validation.mp4`, 1 455 frames, 48,5 s @ 30 fps) : Attract, 2 joueurs + sabres + lasers, Skill Shot, Rampes + Spinners + Combo, G-U-N-G-A-N Super Jets, tirs Laser, Intros complètes → Scènes actives (*Battle Droid* & *Podrace 3D*), Multiballs, Bonus/Match/HSTD, et Diagnostics
- [x] Validation visuelle sur **11 captures PNG** (`attract:0`, `ingame`, `spinner_left`, `scene:6`, `scene:0`, `scene:12`, `bonus`, `match`, `hstd_entry`, `diag:colorbar`, `diag:switch`)
- [x] [README.md](../README.md) & [SCREEN_LAYOUT_REFERENCE.md](SCREEN_LAYOUT_REFERENCE.md)
- [x] [ARCHITECTURE_GUIDE.md](ARCHITECTURE_GUIDE.md) & [HARDWARE_NOTES.md](HARDWARE_NOTES.md)
- [x] [GAME_SCRIPT_REFERENCE.md](GAME_SCRIPT_REFERENCE.md) & [ASSETS_MATRIX.md](ASSETS_MATRIX.md)
- [x] [SWE1_EXTRACTION_INVENTORY.md](SWE1_EXTRACTION_INVENTORY.md) & [SWE1_GAME_RULES.md](SWE1_GAME_RULES.md)
- [x] [RFM_EXTRACTION_INVENTORY.md](RFM_EXTRACTION_INVENTORY.md) & [RFM_GAME_RULES.md](RFM_GAME_RULES.md)
- [x] [HD_UPSCALING_GUIDE.md](HD_UPSCALING_GUIDE.md) & [TODO.md](TODO.md)
