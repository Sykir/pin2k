# Inventaire complet d'extraction des ROMs Pinball 2000

Ce document recense **tout ce qui est extrait avec certitude** des mises à jour ROM Williams Pinball 2000 par `pin2k-rom-tool` et l'analyse ELF32 (`swe1_0166.elf` et `rfm_0195.elf`).

---

## 1. Tableau général de tout ce que l'on extrait des ROMs

| Catégorie | Source dans la ROM | Outil / Méthode d'extraction | Quantité extraite (`SWE1 v1.66`) | Quantité extraite (`RFM 0195`) | Format de sortie |
|---|---|---|---|---|---|
| **Code natif x86 & symboles C++** | Image PRISM décompressée (`B_10000000`) | `pin2k-rom-tool wrap-elf` | `3 088 400` octets, `15 019` symboles | `3 432 816` octets, `19 290` symboles | `ELF32 i386` (`.elf`) |
| **Table des switches** | Symbole `SwitchTable` | Lecture ELF (`0x002e4580` / `0x002fc554`) | `64` entrées (matrice + optos + directs) | `64` entrées (matrice + optos + directs) | Rust (`swe1_tables.rs`, `rfm_tables.rs`) |
| **Table des bobines / solénoïdes** | Symbole `DriveTable` | Lecture ELF (`0x002e2f00` / `0x002faed4`) | `40` bobines + Flashers | `40` bobines + Flashers | Rust (`swe1_tables.rs`, `rfm_tables.rs`) |
| **Table des lampes** | Symbole `LampTable` | Lecture ELF (`0x002e3380` / `0x002fb354`) | Matrice `8×8` + lampes dédiées | Matrice `8×8` + lampes dédiées | Rust (`swe1_tables.rs`, `rfm_tables.rs`) |
| **Animations & Sprites** | Descripteurs `Animation` + frames RLE RGB555/565/Indexed8 | `pin2k-rom-tool extract-assets` | `393` animations (`gfx/` + `gfx_manifest.json`) | Pris en charge par `extract-assets` | PNG multi-frames + JSON |
| **Polices bitmap** | Descripteurs `FontData` | `pin2k-rom-tool extract-assets` | `19` polices (`fonts/` + `fonts_manifest.json`) | Pris en charge par `extract-assets` | PNG + métadonnées glyphes JSON |
| **Audio (voix, SFX, musiques)** | Banques audio DCS / `pb2kslib` | `pin2k-rom-tool extract-assets` | `689` pistes (`audio/*.ogg` + `sound_manifest.json`) | Pris en charge par `extract-assets` | Ogg Vorbis (`.ogg`) + JSON |
| **Table Attract Mode** | `AttractProcInfo` (`0x002d94d0`) | Désassemblage ELF | `20` séquences Attract Mode | `AttractMode` complet | Rust (`attract.rs`) |
| **Table des scènes / vagues** | `Table<scene_info>` (`0x2e179c`) / Classes `Wave` | Désassemblage ELF | `14` scènes principales + `SubEscape` + `2` Multiballs | `9` vagues + `4` Multiballs + sous-modes | Rust + `FULL_GAME_RULES.md` |
| **Règles, scores, timers, hooks** | Méthodes C++ (`score()`, `Buildup`, `AProc`, `Resource`) | Désassemblage ELF (`objdump` / `rea`) | Intégralité des formules & seuils | Intégralité des formules & seuils | Rust + `FULL_GAME_RULES.md` |

---

## 2. Toutes les scènes extraites — *Star Wars Episode I* (`SWE1 v1.66`)

### 2.1 Les 14 scènes de la table `Table<scene_info>` (`0x2e179c`)

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

### 2.2 Modes spéciaux, Multiballs et séquences système (`SWE1 v1.66`)

| Type | Classe(s) C++ extraites | Description |
|---|---|---|
| **Sous-scène Gungan** | `DeffSubEscapeSceneIntro`, `DeffSubEscapeSceneBackground`, `ClassSubEscapeLeffHit` | Poursuite sous-marine sur Naboo (*Sub Escape*) |
| **Multiball principal** | `Multiball`, `DeffMainMultiballStart`, `DeffMultiballBackground`, `DeffMainMultiballSuperJackpot` | Hangar / Naboo Starfighter Multiball (Jackpot + Super Jackpot) |
| **Jedi Multiball** | `JediMultiball`, `DeffJediMultiballIntro`, `DeffJediMultiballBackground`, `DeffJediMultiballWin` | Duel au sabre laser contre Darth Maul (complétion J-E-D-I) |
| **Attract Mode** | `AttractProcInfo` (`20` entrées), `DeffAttractModeBackground`, `DeffAttractModeKahunaLogo`, `DeffAttractModeP2KLogo` | Cycle complet d'attente, logos Williams/Pin2k/Kahuna, HSTD, Jedi Table, Ramp Combo Champion |
| **Bonus / Fin de bille** | `Bonus`, `DeffBonus`, `LeffBonus`, `BallStartBallSaver` | Calcul des vaisseaux (`build_bonus`) × multiplicateur (`build_bonus_x`) + Ball Saver |
| **Système & Diagnostics** | `Game`, `GameHooks`, `DeffTilt`, `DeffTiltWarning`, `DeffSlamTilt`, `SlamTiltStuckDeff` | Gestion des 4 joueurs, Tilt, Slam Tilt, menus opérateur / tests |

---

## 3. Toutes les vagues, scènes et modes extraits — *Revenge From Mars* (`RFM 0195`)

| Catégorie | Classe(s) C++ extraites | Rôle dans le jeu |
|---|---|---|
| **Vague 1 — Mars Kneads Women** | `MarsKneadsWomen`, `Abduction` | Vague soucoupe : sauver les terriennes enlevées |
| **Vague 2 — Big-O-Beam** | `WeaponWave`, `SecretWeapon` | Vague arme secrète / rayon |
| **Vague 3 — Paris in Peril** | `TowerStruggle` | Vague défense de Paris / Tour Eiffel |
| **Vague 4 — Tower of Terror / Paratrooper** | `Paratrooper`, `ParatrooperBacky` | Parachutistes martiens |
| **Vague 5 — Barnyard / Cows** | `Barnyard`, `DeffBarnyardBacky` | Enlèvement à la ferme |
| **Vague 6 — Alien Drive-In** | `DriveIn` | Vague Drive-In |
| **Vague 7 — Martian Bowling** | `MartianBowling` | Vague bowling martien |
| **Vague 8 — Martian Tank** | `MartianTank`, `MartianTankBacky` | Vague char martien |
| **Vague 9 — Martian Happy Hour / Autopsy / Invaders** | `MartianHappyHour`, `Autopsy`, `Invaders` | Modes spéciaux & sous-vagues martiennes |
| **Bonus Wave / Mystery** | `BonusWave`, `QuMarkManager`, `FuelWave`, `SaucersWave` | Vague bonus, Mystery (`?`), carburant et soucoupes |
| **Finale — Attack Mars!** | `AttackMars`, `AttackMarsBacky`, `DeffAttackMarsChampion` | Mode final qualifié par `all_modes + mothership + all_saucer_lights` |
| **Martian Attack** | `MartianAttack`, `MartianTargets`, `MartianBombs` | Abattre les cibles martiennes (valeur progressive) |
| **Capture Multiball** | `CaptureMultiball`, `DeffCaptureMultiballStart`, `DeffCaptureMultiballBacky` | Locks dans les soucoupes puis multiball 3 billes |
| **Hypno Beam Multiball** | `HypnoBeamMultiball`, `DeffHypnoBeamBacky` | Multiball Hypno Beam + Extra Ball au 1er Jackpot |
| **Martian Multiball** | `MartianMultiball`, `DeffMartianMultiballStart`, `DeffMartianMultiballBacky` | Multiball martien (cibles `VbMartian` / `VrMartian`) |
| **Victory Laps** | `VictoryLaps` | Tours de victoire après *Attack Mars!* |
| **Skill Shots & Bonus** | `SkillShots`, `Bonus`, `DeffBonus`, `BallSaver`, `SceneBallSaver` | Skill Shot, Super Skill Shot, Multiplicateur, Bonus Held |
