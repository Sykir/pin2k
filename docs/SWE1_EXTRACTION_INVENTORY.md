# *Star Wars Episode I* (`SWE1 v1.66`) — Inventaire d'extraction ROM

Ce document recense tout ce qui est extrait de la ROM **Star Wars Episode I (`#50069`)** : `pin2000_50069_0166_04032022_B_10000000` (`v1.66`, XINA `1.13`, compilée le `2022-04-03`).

---

## 1. Tableau général d'extraction (`SWE1 v1.66`)

| Catégorie | Source dans la ROM | Outil / Méthode | Quantité extraite | Format / Destination |
|---|---|---|---|---|
| **Code natif x86 & symboles C++** | Image PRISM décompressée (`B_10000000`) | `pin2k-rom-tool wrap-elf` | `3 088 400` octets, `15 019` symboles | `ELF32 i386` (`swe1_0166.elf`) |
| **Table des switches** | `SwitchTable` (`0x002e4580`) | Lecture ELF | `64` entrées (matrice + optos + directs) | `crates/pin2k-core/src/swe1_tables.rs` |
| **Table des bobines / solénoïdes** | `DriveTable` (`0x002e2f00`) | Lecture ELF | `40` bobines + Flashers | `crates/pin2k-core/src/swe1_tables.rs` |
| **Table des lampes** | `LampTable` (`0x002e3380`) | Lecture ELF | Matrice `8×8` + lampes dédiées | `crates/pin2k-core/src/swe1_tables.rs` |
| **Animations & Sprites** | Descripteurs `Animation` + frames RLE | `pin2k-rom-tool extract-assets` | `393` animations | `gfx/` (PNG) + `gfx_manifest.json` |
| **Polices bitmap** | Descripteurs `FontData` | `pin2k-rom-tool extract-assets` | `19` polices | `fonts/` (PNG) + `fonts_manifest.json` |
| **Audio (voix, SFX, musiques)** | Banques audio DCS / `pb2kslib` | `pin2k-rom-tool extract-assets` | `689` pistes | `audio/*.ogg` + `sound_manifest.json` |
| **Attract Mode** | `AttractProcInfo` (`0x002d94d0`) | Désassemblage ELF | `20` séquences | `crates/pin2k-core/src/attract.rs` |
| **Scènes holoprojecteur** | `Table<scene_info>` (`0x2e179c`) | Désassemblage ELF | `14` scènes + `SubEscape` | `crates/pin2k-core/src/swe1_rules.rs` |
| **Multiballs & Mécaniques** | `Multiball`, `JediMultiball`, `Bonus`, `SkillShot`, `Lasers`, `JetBumpers`, `Drop`, `JediLetters`, `Ramps` | Désassemblage ELF | Intégralité des formules et seuils | `SWE1_GAME_RULES.md` + `swe1_rules.rs` |

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
| **Système & Diagnostics** | `Game`, `GameHooks`, `DeffTilt`, `DeffTiltWarning`, `DeffSlamTilt`, `SlamTiltStuckDeff` | Gestion des 4 joueurs, Tilt, Slam Tilt, menus opérateur / tests |
