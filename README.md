# Pinball 2000 → Portage Natif Rust (`pin2k`)

Réécriture native **clean-room** en Rust 64-bit de la plateforme **Williams Pinball 2000** (*Star Wars Episode I* [`#50069`, 1999] et outils *Revenge From Mars* [`#50070`, 1999]), conçue pour fonctionner sous **Linux et Windows** sans dépendance C externe.

> **Frontière légale / Clean-Room :** Ce dépôt ne contient aucune ROM propriétaire, image, animation ou piste audio. Toutes les ressources sont reconstruites et extraites localement par l'utilisateur à partir de ses propres dumps légitimes (`swe1`, `rfm`, paquets de mise à jour `.exe`/`.zip` et `{jeu}_sound.bin`).

---

## Fonctionnalités principales

- **Reconstruction & Vérification ROM (`pin2k-rom-tool`)** :
  - Entrelacement 16-bit des 8 Mask ROMs PRISM (`U100..U107` → 64 Mio) et des ROMs audio DCS-2 (`U109/U110` → 8 Mio).
  - Prise en charge native des **mises à jour officielles et communautaires** (dossiers décompressés, installeurs `.exe` auto-extractibles, archives `.zip`) : `list-updates`, `assemble-update`, `verify`, `wrap-elf` (ELF32 x86 avec ~14 841 symboles C++ démanglés).
  - Décodage complet des sprites et vidéos Pinball 2000 (codecs `0, 1, 20, 21, 22, 23, 30, 42, 43, 44, 45` avec transparence RGB555 `0x7C1F`) et extraction des pistes `.ogg` depuis `{jeu}_sound.bin` (`pb2kslib`).
- **Communication Matérielle Power Driver Board (`pin2k-hal`)** :
  - Protocole filaire exact de la carte **Power Driver Board (PDB)** : 96 switchs (matrice 8×8 + colonnes directes + inversion des optos actifs-bas `SWE1_OPTO_NC_MASK`), 48 sorties bobines/flashers/néon avec verrou haute tension porte monnayeur, et double matrice de 128 lampes.
  - **Port Parallèle Linux** : `/dev/parportN` (`ppdev`), `/dev/port` (`0x378`), et émulateur filaire `VirtualPdbPort` + traceur `--lpt-trace`.
  - **Port Série Windows & Linux** : `NativeSerialPort` (`COM3` via Win32 sous Windows, `/dev/ttyUSB0` via `termios` sous Linux) + protocole pont UART `SerialPdbPort`.
- **Moteur de Jeu, Événements & Menu Opérateur (`pin2k-core`)** :
  - Règles complètes *Star Wars Episode I* : Attract, 1–4 joueurs, Trough & Ball Saver, Skill Shot, Bouclier Déflecteur motorisé, les **12 Scènes Holoprojecteur**, Lasers, Multiball Hangar 3 billes (Jackpots + Super Jackpot), `J-E-D-I` → Duel au Sabre contre Darth Maul (`Lightsaber Neon`), Rangs Jedi, Tilt/Slam Tilt, High Scores.
  - **Bus d'événements structurés (`GameEvent`)** exportable en JSON Lines (`--events-log`) et en HTTP (`/api/events`).
  - **Menu d'Administration Opérateur** complet (`Diagnostics`, `Audits`, `Adjustments`, `Utilities`) piloté par les 4 boutons de porte monnayeur (`Escape`, `Down`, `Up`, `Enter`).
- **Runtime Multiplateforme Windows & Linux (`pin2k-desktop`)** :
  - Compositeur logiciel `640×480` (holoprojecteur `640×240` + télémétrie plateau temps réel), son natif pur-Rust (`lewton` → `winmm.dll` sous Windows / `aplay` sous Linux), interface navigateur Web + WebAudio (`--web-port`), et export de captures/frames (`--screenshot-out`, `--frames-out-dir`).

---

## Architecture du Workspace

| Crate | Rôle | Documentation détaillée |
|---|---|---|
| `crates/pin2k-rom-tool` | Reconstruction PRISM/DCS, mises à jour `.exe`/`.zip`, vérification checksums, ELF32, extraction `.png`/`.ogg` | [ARCHITECTURE_GUIDE.md](docs/ARCHITECTURE_GUIDE.md) |
| `crates/pin2k-hal` | Traits matériel/vidéo/audio, pilote `PdbDriver`, `LinuxPpdevPort`, `SerialPdbPort` (Win/Linux), `VirtualPdbPort` | [HARDWARE_NOTES.md](docs/HARDWARE_NOTES.md) |
| `crates/pin2k-core` | Tables SWE1 `#50069`, anti-rebond, règles de jeu, `GameEvent` bus, menu d'administration | [ARCHITECTURE_GUIDE.md](docs/ARCHITECTURE_GUIDE.md) |
| `crates/pin2k-desktop` | Exécutable natif Windows/Linux, compositeur `640×480`, audio natif, serveur Web/WebAudio, scripts de test | [ARCHITECTURE_GUIDE.md](docs/ARCHITECTURE_GUIDE.md) |

---

## Démarrage Rapide (Pas à Pas)

### 1. Compiler le projet (Linux ou Windows)

```bash
# Compilation native en mode release (Linux ou Windows)
cargo build --release --workspace

# Lancer tous les tests unitaires et d'intégration
cargo test --workspace
```

*(Vérification croisée Windows depuis Linux : `cargo check --workspace --target x86_64-pc-windows-msvc`)*

### 2. Préparer les ROMs et Extraire les Ressources

```bash
# A. Reconstruire la ROM PRISM plate (64 Mio) et la ROM DCS (8 Mio)
./target/release/pin2k-rom-tool reconstruct \
  --input-dir ./roms --prefix swe1_ \
  --output ./work/swe1_prism_flat.bin \
  --dcs-output ./work/swe1_dcs_flat.bin

# B. Lister toutes les mises à jour disponibles (.exe, .zip, dossiers)
./target/release/pin2k-rom-tool list-updates --dir ./updates

# C. Extraire les animations (.png) et l'audio (.ogg) directement depuis une mise à jour .exe ou .zip
./target/release/pin2k-rom-tool extract-assets \
  --prism-flat ./work/swe1_prism_flat.bin \
  --update ./updates/pin2000_50069_0200_02262016_B_10000000.exe \
  --sound-bin ./roms/swe1_sound.bin \
  --output-dir ./work/swe1_assets
```

### 3. Lancer le Jeu (`pin2k-desktop`)

```bash
# Mode bureau interactif (navigateur http://127.0.0.1:8080 + WebAudio + son natif)
./target/release/pin2k-desktop \
  --rom-update ./updates/pin2000_50069_0200_02262016_B_10000000.exe \
  --assets-dir ./work/swe1_assets \
  --web-port 8080 \
  --frames 0

# Mode flipper réel Linux via port parallèle (/dev/parport0)
./target/release/pin2k-desktop \
  --lpt-device /dev/parport0 \
  --assets-dir ./work/swe1_assets \
  --frames 0 --realtime

# Mode pont Série (Windows COM3 ou Linux /dev/ttyUSB0) + journal d'événements JSONL
./target/release/pin2k-desktop \
  --serial-device COM3 --serial-baud 115200 \
  --assets-dir ./work/swe1_assets \
  --events-log ./work/events.jsonl \
  --web-port 8080 --frames 0
```

### 4. Scénarios de Démonstration & Captures Automatisées

```bash
# Démonstration complète (Attract -> Skill Shot -> Battle Droid -> Duel Darth Maul -> Hangar Multiball)
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --script demo --frames 280 --screenshot-out ./demo.png

# Scène Boonta Eve Podrace (export de toutes les frames PNG)
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --script podrace --frames 285 --frames-out-dir ./podrace_frames

# Menu d'Administration Opérateur
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --script admin --frames 48 --screenshot-out ./admin.png
```

---

## Documentation Complète

- **[Guide d'Architecture, Principes & Manuel Complet (`docs/ARCHITECTURE_GUIDE.md`)](docs/ARCHITECTURE_GUIDE.md)**
- **[Référence Matérielle & Protocole PDB (`docs/HARDWARE_NOTES.md`)](docs/HARDWARE_NOTES.md)**

---

## Licence & Mentions Légales

- **Licence du code source** : double licence **[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)**.
- **Non-affiliation** : ce projet communautaire est indépendant et n'est ni affilié, ni approuvé, ni sponsorisé par **Williams Electronics**, **Planetary Pinball**, **Lucasfilm Ltd.** ou **The Walt Disney Company**. *Pinball 2000*, *Star Wars* et *Revenge From Mars* sont des marques déposées de leurs propriétaires respectifs, citées uniquement à des fins d'interopérabilité et de préservation matérielle.

