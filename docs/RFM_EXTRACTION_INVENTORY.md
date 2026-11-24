# *Revenge From Mars* (`RFM 0195`) — Inventaire d'extraction ROM

Ce document recense tout ce qui est extrait de la ROM **Revenge From Mars (`#50070`)** : `pin2000_50070_0195_03292018_B_10000000` (`0195 / v1.09`, XINA `1.22`, compilée le `2018-03-29`).

---

## 1. Tableau général d'extraction (`RFM 0195`)

| Catégorie | Source dans la ROM | Outil / Méthode | Quantité extraite | Format / Destination |
|---|---|---|---|---|
| **Code natif x86 & symboles C++** | Image PRISM décompressée (`B_10000000`) | `pin2k-rom-tool wrap-elf` | `3 432 816` octets, `19 290` symboles | `ELF32 i386` (`rfm_0195.elf`) |
| **Table des switches** | `SwitchTable` (`0x002fc554`) | Lecture ELF | `64` entrées (matrice + optos + directs) | `crates/pin2k-core/src/rfm_tables.rs` |
| **Table des bobines / solénoïdes** | `DriveTable` (`0x002faed4`) | Lecture ELF | `40` bobines + Flashers | `crates/pin2k-core/src/rfm_tables.rs` |
| **Table des lampes** | `LampTable` (`0x002fb354`) | Lecture ELF | Matrice `8×8` + lampes dédiées | `crates/pin2k-core/src/rfm_tables.rs` |
| **Animations, polices & audio** | Descripteurs `Animation`, `FontData`, `pb2kslib` | `pin2k-rom-tool extract-assets` | Extraction complète depuis la ROM RFM | PNG multi-frames, `.ogg`, JSON |
| **Vagues, Multiballs & Mécaniques** | `AttackMars`, `MartianAttack`, `CaptureMultiball`, `HypnoBeamMultiball`, `MartianMultiball`, `SkillShots`, `Bonus`, etc. | Désassemblage ELF | Intégralité des formules et drapeaux | `RFM_GAME_RULES.md` + `rfm_rules.rs` |

---

## 2. Toutes les vagues, scènes et modes extraits (`RFM 0195`)

| Catégorie | Classe(s) C++ extraites | Rôle dans le jeu |
|---|---|---|
| **Vague — Mars Kneads Women / Abduction** | `MarsKneadsWomen`, `Abduction` | Sauver les terriennes enlevées par les soucoupes |
| **Vague — Big-O-Beam / Secret Weapon** | `WeaponWave`, `SecretWeapon` | Vague arme secrète / rayon |
| **Vague — Paris in Peril / Tower Struggle** | `TowerStruggle` | Défense de Paris / Tour Eiffel |
| **Vague — Paratrooper** | `Paratrooper`, `ParatrooperBacky` | Parachutistes martiens |
| **Vague — Barnyard** | `Barnyard`, `DeffBarnyardBacky` | Enlèvement à la ferme |
| **Vague — Alien Drive-In** | `DriveIn` | Vague Drive-In |
| **Vague — Martian Bowling** | `MartianBowling` | Vague bowling martien |
| **Vague — Martian Tank** | `MartianTank`, `MartianTankBacky` | Vague char martien |
| **Vague — Martian Happy Hour / Autopsy / Invaders** | `MartianHappyHour`, `Autopsy`, `Invaders` | Modes spéciaux & sous-vagues martiennes |
| **Bonus Wave & Mystery** | `BonusWave`, `QuMarkManager`, `FuelWave`, `SaucersWave` | Vague bonus, Mystery (`?`), carburant et soucoupes |
| **Finale — Attack Mars!** | `AttackMars`, `AttackMarsBacky`, `DeffAttackMarsChampion` | Mode final qualifié par `all_modes + mothership + all_saucer_lights` |
| **Martian Attack** | `MartianAttack`, `MartianTargets`, `MartianBombs` | Abattre les cibles martiennes (valeur progressive) |
| **Capture Multiball** | `CaptureMultiball`, `DeffCaptureMultiballStart`, `DeffCaptureMultiballBacky` | Locks dans les soucoupes puis multiball 3 billes |
| **Hypno Beam Multiball** | `HypnoBeamMultiball`, `DeffHypnoBeamBacky` | Multiball Hypno Beam + Extra Ball au 1er Jackpot |
| **Martian Multiball** | `MartianMultiball`, `DeffMartianMultiballStart`, `DeffMartianMultiballBacky` | Multiball martien (cibles `VbMartian` / `VrMartian`) |
| **Victory Laps** | `VictoryLaps` | Tours de victoire après *Attack Mars!* |
| **Skill Shots, Bonus & Ball Saver** | `SkillShots`, `Bonus`, `DeffBonus`, `BallSaver`, `SceneBallSaver` | Skill Shot, Super Skill Shot, Multiplicateur, Bonus Held |
| **Attract Mode & Système** | `AttractMode`, `RfmGame`, `Game`, `GameHooks` | Attract Mode, 4 joueurs, Tilt, Slam Tilt, diagnostics |
