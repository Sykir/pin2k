# *Star Wars Episode I* (`SWE1 v1.66 / v2.00`) — Inventaire d'extraction ROM

Ce document recense tout ce qui est extrait des ROMs **Star Wars Episode I (`#50069`)** : `swe1_0166.elf` (`v1.66`, XINA `1.13`) et `swe1_0200.elf` (`v2.00`, `14 841` symboles C++).

---

## 1. Tableau général d'extraction (`SWE1 v1.66 / v2.00`)

| Catégorie | Source dans la ROM | Outil / Méthode | Quantité extraite | Format / Destination |
|---|---|---|---|---|
| **Code natif x86 & symboles C++** | Image PRISM décompressée (`B_10000000`) | `pin2k-rom-tool wrap-elf` | `14 841` symboles (`v2.00`) / `15 019` (`v1.66`) | `ELF32 i386` (`swe1_0200.elf`, `swe1_0166.elf`) |
| **Layouts UI, `Deff*` & coordonnées** | Sections `.text`/`.rodata`/`.data` | `pin2k-rom-tool extract-layout` | `138` classes `Deff*`, `702` `ACD`, `383` `ARD`, `9` persos | `rom_layout_db.json` + [SCREEN_LAYOUT_REFERENCE.md](SCREEN_LAYOUT_REFERENCE.md) |
| **Table des switches** | `SwitchTable` (`0x002e4580`) | Lecture ELF | `96` switchs logiques (matrice + optos + directs) | `crates/pin2k-core/src/swe1_tables.rs` |
| **Table des bobines / solénoïdes** | `DriveTable` (`0x002e2f00`) | Lecture ELF | `48` sorties (bobines + flashers + néon) | `crates/pin2k-core/src/swe1_tables.rs` |
| **Table des lampes** | `LampTable` (`0x002e3380`) | Lecture ELF | Matrices duales `8×8` A & B (`128` lampes) | `crates/pin2k-core/src/swe1_tables.rs` |
| **Animations & Sprites** | Descripteurs `Animation` + frames RLE | `pin2k-rom-tool extract-assets` | `392` animations (`186` utilisées en Rust) | `gfx/` (PNG) + `gfx_manifest.json` |
| **Polices bitmap** | Descripteurs `FontData` | `pin2k-rom-tool extract-assets` | `19` polices (`19/19` utilisées) | `fonts/` (PNG) + `fonts_manifest.json` |
| **Audio (voix, SFX, musiques)** | Banques audio DCS / `pb2kslib` | `pin2k-rom-tool extract-assets` | `689` pistes (`47` utilisées + tourniquets) | `audio/*.ogg` + `sound_manifest.json` |
| **Attract Mode** | `AttractProcInfo` (`0x002d94d0`) | `extract-layout` + désassemblage | `20` séquences implémentées | `attract.rs` + `attract_render.rs` |
| **Scènes holoprojecteur** | `Table<scene_info>` (`0x002dc95c` / `0x2e179c`) | `extract-layout` + désassemblage | `14` scènes (intros + fonds) + `SubEscape` | `swe1_rules.rs` + `scene_render.rs` |
| **Multiballs, HUD & Système** | `DeffScoreDisplay`, `DeffLaserStatusBackground`, `DeffVSpinner`, `Bonus`, `Match`, `HSTD`, `Diagnostic*Deff` | Désassemblage ELF `swe1_0200` | Intégralité des coordonnées, Z et formules | `SWE1_GAME_RULES.md`, `SCREEN_LAYOUT_REFERENCE.md` |

---

## 2. Toutes les scènes extraites (`Table<scene_info>` à `0x2e179c`)

| # | Bitmask | Symbole C++ dans la ROM | Nom affiché | `min_scenes_started` | Classe C++ principale | Classe `Deff` d'arrière-plan |
|---|---|---|---|---|---|---|
| 1 | `0x0001` | `jedi_scene_battle_droids` | Battle Droid Invasion | `0` | `BattleDroidScene` | `DeffBattleDroidSceneBackground` |
| 2 | `0x0002` | `jedi_scene_musical_chairs` | Mos Espa Cantina (Musical Chairs) | `1` | `MusicalChairsScene` | `MusicalChairsScene` |
| 3 | `0x0004` | `jedi_scene_federation_fighter` | Federation Fighters | `0` | `FederationFighterScene` | `DeffFedFighterSceneBackground` |
| 4 | `0x0008` | `jedi_scene_r2d2` | R2-D2 Scene | `1` | `R2D2Scene` | `DeffR2D2SceneBackground` |
| 5 | `0x0010` | `jedi_scene_gungan` | Gungan Energy Battle | `0` | `GunganScene` | `GunganScene` |
| 6 | `0x0020` | `jedi_scene_queen_fashion` | Queen Amidala Wardrobe | `2` | `QueenFashionScene` | `DeffQueenFashionSceneBackground` |
| 7 | `0x0040` | `jedi_scene_podrace` | Boonta Eve Podrace | `0` | `PodraceScene` | `DeffPodRaceSceneBackground` |
| 8 | `0x0080` | `jedi_scene_hangar_escape` | Theed Hangar Escape | `0` | `HangarEscapeScene` | `DeffHangarEscapeSceneBackground` |
| 9 | `0x0100` | `jedi_scene_jarjar_juggle` | Jar Jar Juggle | `0` | `JarjarJuggleScene` | `DeffJarjarJuggleScene` |
| 10 | `0x0200` | `jedi_scene_probe_droid` | Sith Probe Droids | `0` | `ProbeDroidScene` | `DeffProbeDroidSceneBackground` |
| 11 | `0x0400` | `jedi_scene_destroyer_droid` | Destroyer Droids | `1` | `DestroyerDroidScene` | `DeffDestroyerDroidSceneBackground` |
| 12 | `0x0800` | `jedi_scene_hover_tank` | AAT Hover Tank | `1` | `HoverTankScene` | `DeffHoverTankSceneBackground` |
| 13 | `0x1000` | `jedi_scene_wattos_chance` | Watto's Chance Cube | `1` | `WattosChanceScene` | `DeffWattosChanceSceneBackground` |
| 14 | `0x2000` | `jedi_scene_question_mark` | Mystery (`?`) | `0` | `QuestionMarkScene` | `DeffQuestionMarkSceneBackground` |

---

## 3. Modes spéciaux, Multiballs et séquences système (`SWE1 v1.66`)

| Type | Classe(s) C++ extraites | Description |
|---|---|---|
| **Sous-scène Gungan** | `DeffSubEscapeSceneIntro`, `DeffSubEscapeSceneBackground`, `ClassSubEscapeLeffHit` | Poursuite sous-marine sur Naboo (*Sub Escape*) |
| **Multiball principal** | `Multiball`, `DeffMainMultiballStart`, `DeffMultiballBackground`, `DeffMainMultiballSuperJackpot` | Hangar / Naboo Starfighter Multiball (Jackpot + Super Jackpot) |
| **Jedi Multiball** | `JediMultiball`, `DeffJediMultiballIntro`, `DeffJediMultiballBackground`, `DeffJediMultiballWin` | Duel au sabre laser contre Darth Maul (complétion J-E-D-I) |
| **Attract Mode** | `AttractProcInfo` (`20` entrées), `DeffAttractModeBackground`, `DeffAttractModeKahunaLogo`, `DeffAttractModeP2KLogo` | Cycle complet d'attente, logos Williams/Pin2k/Kahuna, HSTD, Jedi Table, Ramp Combo Champion |
| **Bonus / Fin de bille** | `Bonus`, `DeffBonus`, `LeffBonus`, `BallStartBallSaver` | Calcul des vaisseaux (`build_bonus`) × multiplicateur (`build_bonus_x`) + Ball Saver |
| **Système & Diagnostics** | `Game`, `GameHooks`, `DeffTilt`, `DeffTiltWarning`, `DeffSlamTilt`, `Diagnostic*Deff` | Gestion des 4 joueurs, Tilt, Slam Tilt, menus opérateur / mires vidéo |

---

## 4. Layouts UI & Écrans Extraits (`swe1_0200.elf` → Rust pur)

L'ensemble des écrans suivants a été reconstruit avec ses coordonnées exactes `640×240` et priorités Z (voir [SCREEN_LAYOUT_REFERENCE.md](SCREEN_LAYOUT_REFERENCE.md)) :

- **HUD permanent en partie** : `DeffScoreDisplay` (`score_display.rs`) + `DeffLaserStatusBackground` (`hud_display.rs`).
- **Spinners virtuels sur rampe** : `DeffVSpinner` (`vspinner.rs`) avec déclenchement exclusif sur rampe + sons tourniquet `0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`.
- **Attract Mode complet (20 écrans)** : `AttractProcInfo` (`attract_render.rs`), incluant `DeffAttractModeKahunaLogo` et le cycle 9 couleurs RGB555 du flare (`0x100fc0`).
- **14 Scènes complètes + Multiballs + Holo Select** : `Deff*SceneIntro` (cadre `anim_jedi_scene_intro_frame` `z=202`) + 14 fonds actifs (`scene_render.rs`).
- **Écrans Spéciaux & Diagnostics** : `PowerupTestingDeff`, `DeffBonus`, `DeffMatch`, `DeffGameOver`, `DeffGameIntro`, `DeffExtraBallCollected`, `DeffBSBallSaved`, `DeffBuildC3P0Status`, `DeffRampCombo`, `DeffJediMultiballReadyBackground`, `DeffHSTDinitials`, et `Diagnostic{ColorBar,Grid,RGB,Fuse,Switch,Coil,SingleLamp}Deff` (`system_screens.rs`).
