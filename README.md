# Pinball 2000 → Portage Natif Rust (`pin2k`)

Réécriture native **clean-room** en Rust 64-bit de la plateforme **Williams Pinball 2000** (*Star Wars Episode I* [`#50069`, 1999] et *Revenge From Mars* [`#50070`, 1999]), conçue pour fonctionner sous **Linux et Windows** sans dépendance C externe.

> **Frontière légale / Clean-Room :** Ce dépôt ne contient aucune ROM propriétaire, image, police, animation ou piste audio. Toutes les ressources et scripts de gamecode sont extraits localement par l'utilisateur à partir de ses propres dumps légitimes (`swe1`, `rfm`, paquets de mise à jour `.exe`/`.zip` et `{jeu}_sound.bin`).

---

## Fonctionnalités principales

- **Reconstruction, Vérification & Extraction Complète des Assets (`pin2k-rom-tool`)** :
  - Entrelacement 16-bit des 8 Mask ROMs PRISM (`U100..U107` → 64 Mio) et des ROMs audio DCS-2 (`U109/U110` → 8 Mio).
  - Prise en charge native des **mises à jour officielles et communautaires** (dossiers décompressés, installeurs `.exe` auto-extractibles, archives `.zip`) : `list-updates`, `assemble-update`, `verify`, `wrap-elf` (ELF32 x86 symbolisé).
  - **Extraction complète des assets (`extract-assets`)** :
    - **Animations & vidéos** : décode **toutes** les frames par défaut (codecs `0, 1, 20, 21, 22, 23, 30, 42, 43, 44, 45` avec transparence RGB555 `0x7C1F`) → `gfx/*.png` + `gfx/gfx_manifest.json`.
    - **Polices bitmap (`FontData` / `Character`)** : extrait toutes les polices `font_*_data_ptr` en atlas PNG RGBA8888 + métriques par glyphe → `fonts/*.png` + `fonts/fonts_manifest.json`.
    - **Audio DCS-2 (`pb2kslib`)** : déchiffrement XOR `0x3A` et extraction des pistes Vorbis → `audio/*.ogg` + `audio/sound_manifest.json`.
- **Extraction du Gamecode en Scripts (`pin2k-rom-tool extract-gamecode`)** :
  - Lit `SwitchTable` (96 switchs), `DriveTable` (48 bobines/flashers/néon) et `LampTable` (128 lampes) directement depuis `.data` avec noms multi-langues et masques opto.
  - Désassembleur statique x86-32 (`lift_x86_calls`) analysant les handlers `swd_*` pour extraire les appels `score`, `generic_score`, `AudioRequest`, `traverse_and_call` et leurs arguments.
  - Génère un script portable structuré **`game_script.json`** (tables matérielles, séquence attract, variables, modes/scènes, règles déclenchées par événements).
- **Moteur de Jeu & Runtime Multiplateforme Windows / Linux (`pin2k-core` + `pin2k-desktop`)** :
  - **`ScriptRuntime`** : exécute `game_script.json` (auto-détecté dans `<assets-dir>/scripts/game_script.json` ou via `--game-script`) par-dessus le moteur temps réel 60 Hz.
  - Règles complètes *Star Wars Episode I* (`#50069`) et *Revenge From Mars* (`#50070`), support Mods/Homebrew (`mod.json`), bus d'événements JSONL (`--events-log`, `/api/events`), et Menu Opérateur complet (`Diagnostics`, `Audits`, `Adjustments`, `Utilities`).
  - Compositeur logiciel `640×480`, audio pur-Rust (`lewton` → `winmm.dll` sous Windows / `aplay` sous Linux), UI Web + WebAudio (`--web-port`).
- **Communication Matérielle Power Driver Board (`pin2k-hal`)** :
  - Protocole filaire exact PDB sur port parallèle Linux (`/dev/parportN` `ppdev`, `/dev/port` `0x378`), pont Série Windows/Linux (`COMx`, `/dev/ttyUSB0`), et émulateur filaire `VirtualPdbPort` + `--lpt-trace`.

---

## Architecture du Workspace

| Crate | Rôle | Documentation |
|---|---|---|
| `crates/pin2k-rom-tool` | Reconstruction PRISM/DCS, mises à jour `.exe`/`.zip`, ELF32, extraction `.png`/polices/`.ogg`, `extract-gamecode` | [ARCHITECTURE_GUIDE.md](docs/ARCHITECTURE_GUIDE.md) |
| `crates/pin2k-hal` | Traits matériel/vidéo/audio, `PdbDriver`, `LinuxPpdevPort`, `SerialPdbPort` (Win/Linux), `VirtualPdbPort` | [HARDWARE_NOTES.md](docs/HARDWARE_NOTES.md) |
| `crates/pin2k-core` | Machine 60 Hz, `ScriptRuntime` (`game_script.json`), tables/règles SWE1 & RFM, Mods, `GameEvent`, Admin | [GAME_SCRIPT_REFERENCE.md](docs/GAME_SCRIPT_REFERENCE.md) |
| `crates/pin2k-desktop` | Exécutable natif Windows/Linux, compositeur `640×480`, audio natif, serveur Web/WebAudio | [ARCHITECTURE_GUIDE.md](docs/ARCHITECTURE_GUIDE.md) |

---

## Démarrage Rapide (Pas à Pas)

### 1. Compiler le projet (Linux & Windows)

```bash
# Compilation native release (Linux ou Windows)
cargo build --release --workspace

# Lancer tous les tests unitaires
cargo test --workspace

# Compilation croisée Windows depuis Linux
cargo build --release --workspace --target x86_64-pc-windows-gnu
# (ou vérification MSVC : cargo check --workspace --target x86_64-pc-windows-msvc)
```

### 2. Reconstruire les ROMs, Extraire Tous les Assets & le Gamecode

```bash
# A. Reconstruire la ROM PRISM plate (64 Mio) et la ROM DCS (8 Mio)
./target/release/pin2k-rom-tool reconstruct \
  --input-dir /chemin/vers/roms --prefix swe1_ \
  --output ./work/swe1_prism_flat.bin \
  --dcs-output ./work/swe1_dcs_flat.bin

# B. Extraire TOUTES les animations (.png), polices (fonts/*.png) et pistes audio (.ogg)
./target/release/pin2k-rom-tool extract-assets \
  --prism-flat ./work/swe1_prism_flat.bin \
  --update /chemin/vers/updates/pin2000_50069_0200_02262016_B_10000000 \
  --sound-bin /chemin/vers/roms/swe1_sound.bin \
  --output-dir ./work/swe1_assets

# C. Extraire le gamecode (tables, handlers x86, attract, modes, règles) en script JSON
./target/release/pin2k-rom-tool extract-gamecode \
  --update /chemin/vers/updates/pin2000_50069_0200_02262016_B_10000000 \
  --prism-flat ./work/swe1_prism_flat.bin \
  --output-dir ./work/swe1_assets/scripts
```

### 3. Lancer le Jeu (`pin2k-desktop`)

```bash
# Mode bureau interactif avec assets + scripts auto-détectés (navigateur http://127.0.0.1:8080)
./target/release/pin2k-desktop \
  --assets-dir ./work/swe1_assets \
  --web-port 8080 \
  --frames 0

# Ou avec un fichier de script explicite
./target/release/pin2k-desktop \
  --assets-dir ./work/swe1_assets \
  --game-script ./work/swe1_assets/scripts/game_script.json \
  --web-port 8080 --frames 0

# Mode flipper réel Linux via port parallèle (/dev/parport0)
./target/release/pin2k-desktop \
  --lpt-device /dev/parport0 \
  --assets-dir ./work/swe1_assets \
  --frames 0 --realtime

# Mode pont Série (Windows COM3 ou Linux /dev/ttyUSB0)
./target/release/pin2k-desktop \
  --serial-device COM3 --serial-baud 115200 \
  --assets-dir ./work/swe1_assets \
  --events-log ./work/events.jsonl \
  --web-port 8080 --frames 0
```

### 4. Scénarios de Démonstration & Captures Automatisées

```bash
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --script demo --frames 280 --screenshot-out ./work/demo.png
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --script podrace --frames 285 --frames-out-dir ./work/podrace_frames
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --script admin --frames 48 --screenshot-out ./work/admin.png
```

---

## Documentation Complète

- **[Guide d'Architecture, Compilation & CLI (`docs/ARCHITECTURE_GUIDE.md`)](docs/ARCHITECTURE_GUIDE.md)**
- **[Référence du Format `game_script.json` & Extraction Gamecode (`docs/GAME_SCRIPT_REFERENCE.md`)](docs/GAME_SCRIPT_REFERENCE.md)**
- **[Tableau Croisé Complet des Assets & Utilisation (`docs/ASSETS_MATRIX.md`)](docs/ASSETS_MATRIX.md)**
- **[Référence Matérielle, Formats ROM & Protocole PDB (`docs/HARDWARE_NOTES.md`)](docs/HARDWARE_NOTES.md)**

---

## Licence & Mentions Légales

- **Licence du code source** : double licence **[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)**.
- **Non-affiliation** : ce projet communautaire est indépendant et n'est ni affilié, ni approuvé, ni sponsorisé par **Williams Electronics**, **Planetary Pinball**, **Lucasfilm Ltd.** ou **The Walt Disney Company**. *Pinball 2000*, *Star Wars* et *Revenge From Mars* sont des marques déposées de leurs propriétaires respectifs, citées uniquement à des fins d'interopérabilité et de préservation matérielle.
