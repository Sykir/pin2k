# Roadmap & TODO (`pin2k`)

Suivi de l'avancement du portage natif Rust **Pinball 2000** (*Star Wars Episode I* `#50069` & *Revenge From Mars* `#50070`).

## Avancement Global : **~70 %**

| Chantier | Avancement | Statut |
|---|---:|---|
| 1. Outil ROM & Extraction (`pin2k-rom-tool`) | **90 %** | Quasiment complet |
| 2. Abstraction Matérielle & PDB (`pin2k-hal`) | **95 %** | Prêt cabinet & série |
| 3. Moteur de Jeu **100 % Data-Driven** (`pin2k-core`) | **45 %** | Hybride (VM prête, migration complète à finir) |
| 4. Couverture des Assets SWE1 (`gfx` / `fonts` / `audio`) | **35 %** | Polices 100 %, Anims 25 %, Audio 6 % |
| 5. Runtime Desktop, **Affichage Natif** & HD (`pin2k-desktop`) | **65 %** | Web OK, fenêtre/fullscreen natif à faire |
| 6. Documentation & Qualité | **95 %** | À jour |

---

## 1. Outil ROM & Extraction (`pin2k-rom-tool`) — **90 %**
- [x] Reconstruction PRISM (`U100..U107` → 64 Mio) et DCS (`U109/U110` → 8 Mio)
- [x] Lecture des mises à jour officielles/communautaires (`.exe`, `.zip`, dossiers) + vérification des 12 checksums
- [x] Export ELF32 x86 symbolisé (`wrap-elf`)
- [x] Extraction complète des animations (`extract-assets`, toutes les frames par défaut, codecs `0..45`)
- [x] Extraction des **19 polices bitmap** (`FontData` / `Character` → atlas PNG + `fonts_manifest.json`)
- [x] Extraction audio DCS-2 (`*_sound.bin` `pb2kslib` → `.ogg` + `sound_manifest.json`)
- [x] Extraction gamecode v1 (`SwitchTable`, `DriveTable`, `LampTable`, désassembleur `lift_x86_calls` → `game_script.json`)
- [x] Découpage modulaire (`main.rs` 11 lignes, `cli.rs`, `commands.rs`)
- [ ] Étendre `extract-gamecode` pour exporter 100 % des scènes, modes, couches visuelles et callouts SWE1 & RFM dans `game_script.json`
- [ ] Décodage audio DCS-2 brut depuis `u109`/`u110` (fallback sans `*_sound.bin`)
- [ ] Sous-commande `upscale-assets` (`xBRZ` / `Lanczos` + *edge-padding* alpha → `gfx_hd/`)

---

## 2. Abstraction Matérielle & PDB (`pin2k-hal`) — **95 %**
- [x] Protocole filaire exact Power Driver Board (`PinIOWrite` / `PinIORead`, 96 switchs, 48 bobines, 128 lampes)
- [x] Gestion des optos actifs-bas (`SWE1_OPTO_NC_MASK`) et verrou haute tension porte monnayeur
- [x] Port parallèle Linux (`/dev/parportN` `ppdev`, `/dev/port`), `VirtualPdbPort` + `--lpt-trace`
- [x] Pont Série multiplateforme Windows (`COMx`) & Linux (`/dev/ttyUSB0`) sans dépendance C
- [ ] Validation terrain longue durée sur carte PDB physique + pont microcontrôleur UART

---

## 3. Moteur de Jeu **100 % Data-Driven** (`pin2k-core`) — **45 %**
- [x] Machine 60 Hz découpée en sous-modules (`machine/{mod,script_exec,swe1_playfield,rfm_playfield,admin_nav,render,tests}.rs`)
- [x] VM de scripts `ScriptRuntime` (`vars`, `modes`, `timers`, `triggers`, `conditions`, `actions`, `--strict-script`)
- [x] Base de données locale persistante (`pin2k_db.json` : High Scores, Audits, Adjustments, historique des parties)
- [x] Mécaniques SWE1 : Bouclier → Sélection par hologramme → Intro 15 fps → Éjection bille → Scène active, Duel Darth Maul, Spinners
- [x] Menu Opérateur Admin complet + Menu Debug (lancement d'états, scènes, modes, jeux)
- [ ] **Passage en 100 % Data-Driven (règles)** : déplacer les 12 scènes SWE1 (intro, durée, hologramme, gameplay, coups requis, scores, musiques), le Duel Darth Maul et les vagues RFM intégralement dans `game_script.json`
- [ ] **Passage en 100 % Data-Driven (scènes visuelles)** : décrire la composition multi-couches (actuellement dans `scenes_compositor.rs`) dans un manifeste JSON de couches (`layers[]`)
- [ ] Activer le mode script pur (`--strict-script`) par défaut dès qu'un `game_script.json` complet est chargé, sans fallback sur des règles Rust codées en dur

---

## 4. Couverture des Assets SWE1 (`docs/ASSETS_MATRIX.md`) — **35 %**
- [x] **Polices Bitmap (`fonts/`)** : **19 / 19 (100 %)** chargées et utilisables dans `SoftwareCompositor`
- [ ] **Animations & Sprites (`gfx/`)** : **99 / 392 (25 %)** utilisées (12 scènes, hologrammes Obi-Wan, spinners, duel Maul, attract) → reste les écrans de bonus personnages, animations secondaires et effets avancés
- [ ] **Pistes Audio DCS-2 (`audio/`)** : **43 / 689 (6 %)** utilisées (toutes les musiques de scènes et SFX majeurs vérifiés contre `swe1_0200.elf`) → reste les callouts vocaux contextuels

---

## 5. Runtime Desktop, **Affichage Natif** & Rendu HD (`pin2k-desktop`) — **65 %**
- [x] Architecture modulaire (`main.rs` 19 lignes, `cli.rs`, `runner.rs`, `backend.rs`, `compositor.rs`, `scenes_compositor.rs`, `raster.rs`, `telemetry.rs`, `web_ui.rs` + `web_ui.html`, `audio.rs`)
- [x] Respect de l'horloge FPS native ROM + animations one-shot sans boucle
- [x] Interface Web interactive (`--web-port`), WebAudio, panneau Debug et vue DB locale
- [x] Audio natif pur-Rust (`lewton` → `winmm.dll` Windows / `aplay` Linux)
- [x] Compilation croisée Windows (`x86_64-pc-windows-msvc`, `x86_64-pc-windows-gnu`) et Linux (`x86_64-unknown-linux-gnu`)
- [ ] **Affichage Natif Desktop Fenêtré & Plein Écran (`--window` / `--fullscreen`)** :
  - Intégrer `winit` + `softbuffer` (ou `wgpu`) en pur Rust pour **Windows (Win32)** et **Linux (X11 / Wayland)**
  - Rendu direct du framebuffer `640×480` avec conservation du ratio `4:3`
  - Bascule fenêtre ↔ plein écran à chaud (`F11` / `Alt+Entrée`) et contrôles clavier natifs
  - Préserver le mode headless/CI/Docker quand ni `--window` ni `--fullscreen` n'est passé
- [ ] **Rendu HD & Upscaling** (voir [HD_UPSCALING_GUIDE.md](HD_UPSCALING_GUIDE.md)) :
  - Shaders WebGL2 côté navigateur (`Native`, `Bicubic`, `xBRZ`, `CRT Scanlines`)
  - Support dossier optionnel `gfx_hd/` + cache LRU borné dans `LoadedAnim`

---

## 6. Documentation — **95 %**
- [x] [README.md](../README.md)
- [x] [ARCHITECTURE_GUIDE.md](ARCHITECTURE_GUIDE.md)
- [x] [HARDWARE_NOTES.md](HARDWARE_NOTES.md)
- [x] [GAME_SCRIPT_REFERENCE.md](GAME_SCRIPT_REFERENCE.md)
- [x] [ASSETS_MATRIX.md](ASSETS_MATRIX.md)
- [x] [HD_UPSCALING_GUIDE.md](HD_UPSCALING_GUIDE.md)
- [x] [TODO.md](TODO.md)
- [ ] Mettre à jour les guides une fois l'affichage natif (`--window`/`--fullscreen`) et le 100 % data-driven finalisés
