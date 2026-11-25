# Tableau Croise Complet des Assets (`swe1`) & Utilisation dans le Code

Ce document recense **100 % des assets extraits** de *Star Wars Episode I* (`#50069 v2.00`) et indique pour chacun s'il est reference dans le code source Rust (`crates/`), et ou.

## Resume de Couverture

| Categorie | Total extrait | Utilises | Non utilises |
|---|---:|---:|---:|
| **Animations & Sprites (`gfx/`)** | 392 | 354 | 38 |
| **Polices Bitmap (`fonts/`)** | 19 | 19 | 0 |
| **Pistes Audio DCS-2 (`audio/`)** | 689 | 689 | 0 |

---

## 1. Animations & Sprites (392 entrees)

| # | Asset | Frames | FPS | Dimensions | Utilise | Emplacement(s) dans le code |
|---:|---|---:|---:|---|---|---|
| 1 | `anim_battle_droid_intro_battle` | 55 | 15 | 220x160 | Oui | `crates/pin2k-core/src/attract.rs:74`, `crates/pin2k-core/src/mods.rs:64`, `crates/pin2k-core/src/swe1_rules.rs:126`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:530`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:693` |
| 2 | `anim_battle_droid_intro_badge` | 1 | 60 | 304x79 | Oui | `crates/pin2k-core/src/attract.rs:78`, `crates/pin2k-core/src/machine/attract_render.rs:370`, `crates/pin2k-core/src/swe1_rules.rs:146`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:547`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:572` |
| 3 | `anim_bdroid_explosion` | 20 | 60 | 245x129 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:17` |
| 4 | `anim_bdroid_laser_explosion` | 20 | 60 | 230x131 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:18` |
| 5 | `anim_bdroid_on_stap` | 26 | 60 | 163x76 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:99`, `crates/pin2k-core/src/swe1_rules.rs:207`, `crates/pin2k-desktop/src/scenes_compositor.rs:27`, `crates/pin2k-desktop/src/scenes_compositor.rs:31` |
| 6 | `anim_bdroid_tank_explosion` | 25 | 60 | 580x220 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:20` |
| 7 | `anim_bdroid_stap_zap` | 22 | 60 | 56x19 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:102`, `crates/pin2k-desktop/src/scenes_compositor.rs:34` |
| 8 | `anim_bdroid_stap_explosion` | 19 | 60 | 574x213 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:19` |
| 9 | `anim_bdroid_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:96`, `crates/pin2k-desktop/src/scenes_compositor.rs:28` |
| 10 | `anim_bdroid_zap` | 44 | 60 | 28x9 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:42`, `crates/pin2k-core/src/machine/fx/scene_extras.rs:43` |
| 11 | `anim_bonus_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:246`, `crates/pin2k-core/src/machine/system_screens.rs:123`, `crates/pin2k-core/src/machine/tests.rs:617` |
| 12 | `anim_explosion_2` | 30 | 60 | 150x63 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:67` |
| 13 | `anim_explosion_3` | 21 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:28` |
| 14 | `anim_explosion_4` | 30 | 60 | 160x53 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:36` |
| 15 | `anim_explosion_5` | 30 | 60 | 160x53 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:37` |
| 16 | `anim_explosion_6` | 30 | 60 | 160x53 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:38` |
| 17 | `anim_extra_ball_show` | 31 | 15 | 320x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:283`, `crates/pin2k-core/src/machine/system_screens.rs:258`, `crates/pin2k-core/src/machine/tests.rs:621` |
| 18 | `anim_fashion_flash` | 12 | 60 | 190x121 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:64` |
| 19 | `anim_fashion_wheel` | 10 | 60 | 497x152 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:192`, `crates/pin2k-desktop/src/scenes_compositor.rs:81` |
| 20 | `anim_fashion_base` | 2 | 60 | 220x30 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:189`, `crates/pin2k-desktop/src/scenes_compositor.rs:78` |
| 21 | `anim_fashion_queen` | 1 | 60 | 198x174 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:191`, `crates/pin2k-core/src/swe1_rules.rs:215`, `crates/pin2k-desktop/src/scenes_compositor.rs:77`, `crates/pin2k-desktop/src/scenes_compositor.rs:80` |
| 22 | `anim_fashion_throne` | 1 | 60 | 597x196 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:190`, `crates/pin2k-desktop/src/scenes_compositor.rs:79` |
| 23 | `anim_fashion_icon_black` | 60 | 60 | 127x63 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:158` |
| 24 | `anim_fashion_icon_fanhead` | 60 | 60 | 127x63 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:159` |
| 25 | `anim_fashion_icon_hornhead` | 60 | 60 | 127x63 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:160` |
| 26 | `anim_fashion_icon_nun` | 60 | 60 | 127x63 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:161` |
| 27 | `anim_fashion_icon_spade` | 60 | 60 | 127x63 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:162` |
| 28 | `anim_fashion_icon_victorian` | 60 | 60 | 127x63 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:163`, `crates/pin2k-desktop/src/scenes_compositor.rs:82` |
| 29 | `anim_fashion_icon_extra_ball` | 60 | 60 | 124x62 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:164` |
| 30 | `anim_federation_fighter_laser_explosion_2` | 30 | 60 | 324x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:35` |
| 31 | `anim_federation_fighter_intro_badge` | 1 | 60 | 295x58 | Oui | `crates/pin2k-core/src/swe1_rules.rs:148` |
| 32 | `anim_federation_fighter_intro_battle` | 59 | 12 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:128` |
| 33 | `anim_federation_fighter_cockpit` | 10 | 60 | 550x40 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:139` |
| 34 | `anim_federation_fighter` | 7 | 60 | 143x38 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:117`, `crates/pin2k-core/src/swe1_rules.rs:209`, `crates/pin2k-desktop/src/scenes_compositor.rs:45`, `crates/pin2k-desktop/src/scenes_compositor.rs:48` |
| 35 | `anim_federation_fighter_flyby` | 27 | 60 | 208x170 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:73` |
| 36 | `anim_federation_fighter_explosion` | 16 | 60 | 324x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:30` |
| 37 | `anim_federation_fighter_explosion_2` | 36 | 60 | 360x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:31` |
| 38 | `anim_federation_fighter_explosion_3` | 40 | 60 | 324x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:32` |
| 39 | `anim_federation_fighter_laser_explosion` | 16 | 60 | 324x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:33` |
| 40 | `anim_federation_fighter_side` | 1 | 60 | 127x280 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:70` |
| 41 | `anim_federation_fighter_background` | 1 | 60 | 900x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:115`, `crates/pin2k-desktop/src/scenes_compositor.rs:46` |
| 42 | `anim_federation_fighter_zap` | 58 | 60 | 88x19 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:79`, `crates/pin2k-core/src/machine/fx/scene_extras.rs:92` |
| 43 | `anim_gungan_sub_sub` | 80 | 60 | 464x122 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:124`, `crates/pin2k-core/src/machine/system_screens.rs:322`, `crates/pin2k-desktop/src/scenes_compositor.rs:55` |
| 44 | `anim_gungan_sub_opee` | 150 | 60 | 415x163 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:125` |
| 45 | `anim_gungan_sub_chomp` | 75 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:213`, `crates/pin2k-core/src/machine/system_screens.rs:323`, `crates/pin2k-core/src/machine/tests.rs:625`, `crates/pin2k-core/src/swe1_rules.rs:210`, `crates/pin2k-desktop/src/scenes_compositor.rs:52` (+1) |
| 46 | `anim_gungan_sub_bubbles` | 54 | 60 | 146x215 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:40`, `crates/pin2k-core/src/machine/fx/scene_extras.rs:83` |
| 47 | `anim_hangar_droid_run` | 90 | 60 | 85x127 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:140`, `crates/pin2k-core/src/swe1_rules.rs:211`, `crates/pin2k-desktop/src/scenes_compositor.rs:59`, `crates/pin2k-desktop/src/scenes_compositor.rs:60` |
| 48 | `anim_hangar_droid_explosion` | 30 | 60 | 251x145 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:43`, `crates/pin2k-core/src/machine/scene_render.rs:142`, `crates/pin2k-core/src/machine/swe1_playfield.rs:352`, `crates/pin2k-desktop/src/scenes_compositor.rs:61` |
| 49 | `anim_hangar_droid_explosion_2` | 30 | 60 | 238x145 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:44` |
| 50 | `anim_hangar_droid_explosion_3` | 30 | 60 | 306x127 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:45` |
| 51 | `anim_hangar_droid_explosion_4` | 30 | 60 | 262x145 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:46` |
| 52 | `anim_hangar_droid_laser_explosion` | 30 | 60 | 255x149 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:48`, `crates/pin2k-core/src/machine/scene_render.rs:143`, `crates/pin2k-desktop/src/scenes_compositor.rs:62` |
| 53 | `anim_hangar_droid_laser_explosion_2` | 30 | 60 | 268x131 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:50` |
| 54 | `anim_hangar_escape_intro_movie` | 74 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:130`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:582` |
| 55 | `anim_hangar_escape_intro_badge` | 1 | 60 | 299x78 | Oui | `crates/pin2k-core/src/swe1_rules.rs:150` |
| 56 | `anim_hangar_escape_exit_movie` | 47 | 15 | 320x240 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:51` |
| 57 | `anim_hangar_escape_torpedo` | 181 | 60 | 200x91 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:141` |
| 58 | `anim_jackpot_icon_idle` | 60 | 60 | 161x61 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:318` |
| 59 | `anim_jackpot_icon_boom` | 9 | 60 | 354x110 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:82`, `crates/pin2k-core/src/machine/fx/triggers.rs:139` |
| 60 | `anim_jarjar_juggle_intro_movie` | 67 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:135` |
| 61 | `anim_jarjar_juggle_intro_badge` | 1 | 60 | 330x79 | Oui | `crates/pin2k-core/src/swe1_rules.rs:155` |
| 62 | `anim_jarjar_frog_fountain` | 46 | 60 | 200x129 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:75` |
| 63 | `anim_jarjar_appear` | 28 | 60 | 141x161 | Oui | `crates/pin2k-core/src/machine/fx/mod.rs:220` |
| 64 | `anim_jarjar_juggle` | 19 | 60 | 407x191 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:200`, `crates/pin2k-core/src/swe1_rules.rs:216`, `crates/pin2k-desktop/src/scenes_compositor.rs:85`, `crates/pin2k-desktop/src/scenes_compositor.rs:87` |
| 65 | `anim_jarjar_fall` | 20 | 60 | 470x240 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:70` |
| 66 | `anim_jarjar_jumpup` | 40 | 60 | 400x213 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:68` |
| 67 | `anim_jarjar_switch` | 4 | 60 | 189x209 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:69` |
| 68 | `anim_jarjar_stare` | 11 | 60 | 189x209 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:199`, `crates/pin2k-desktop/src/scenes_compositor.rs:86` |
| 69 | `anim_jarjar_junk1` | 20 | 60 | 96x46 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:172` |
| 70 | `anim_jarjar_junk2` | 20 | 60 | 126x60 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:173` |
| 71 | `anim_jarjar_junk3` | 20 | 60 | 112x44 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:174` |
| 72 | `anim_jarjar_junk4` | 20 | 60 | 88x58 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:175` |
| 73 | `anim_jarjar_junk5` | 20 | 60 | 103x38 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:176` |
| 74 | `anim_jarjar_junk6` | 20 | 60 | 124x63 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:177` |
| 75 | `anim_jarjar_surprise` | 28 | 60 | 174x134 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:76` |
| 76 | `anim_jarjar_idle_1` | 20 | 60 | 94x135 | Oui | `crates/pin2k-core/src/machine/fx/mod.rs:222` |
| 77 | `anim_jarjar_junk_eb` | 20 | 60 | 116x61 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:186` |
| 78 | `anim_jarjar_juggle_2` | 32 | 60 | 466x213 | Non | - |
| 79 | `anim_jarjar_fall_2` | 18 | 60 | 318x220 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:71` |
| 80 | `anim_jarjar_tongue` | 35 | 60 | 202x141 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:74` |
| 81 | `anim_jarjar_juggle_backdrop` | 1 | 60 | 629x141 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:198` |
| 82 | `anim_jedi_battle_intro_movie` | 63 | 15 | 220x160 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:89`, `crates/pin2k-core/src/machine/swe1_playfield.rs:483`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:595` |
| 83 | `anim_jedi_battle_intro_badge` | 1 | 60 | 445x108 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:91` |
| 84 | `anim_jedi_battle_intro_frame` | 1 | 60 | 594x216 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:90` |
| 85 | `anim_jedi_battle_background_center` | 1 | 60 | 340x240 | Non | - |
| 86 | `anim_jedi_battle_background_left` | 10 | 60 | 150x240 | Non | - |
| 87 | `anim_jedi_battle_background_right` | 10 | 60 | 150x240 | Non | - |
| 88 | `anim_jedi_council_mace` | 1 | 60 | 93x58 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:230`, `crates/pin2k-desktop/src/scenes_compositor.rs:99` |
| 89 | `anim_jedi_council_background` | 1 | 60 | 593x234 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:229` |
| 90 | `anim_jedi_council_neck` | 12 | 60 | 90x58 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:234` |
| 91 | `anim_jedi_council_yoda` | 6 | 60 | 93x46 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:231`, `crates/pin2k-desktop/src/scenes_compositor.rs:100` |
| 92 | `anim_jedi_council_member_1` | 6 | 60 | 86x60 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:232` |
| 93 | `anim_jedi_council_member_2` | 5 | 60 | 96x58 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:233` |
| 94 | `anim_jedi_letters_loop` | 80 | 60 | 54x18 | Non | - |
| 95 | `anim_jedi_letters_shrink` | 88 | 60 | 247x82 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:86` |
| 96 | `anim_jedi_scene_intro_frame` | 1 | 60 | 522x195 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:60`, `crates/pin2k-core/src/machine/scene_render.rs:74`, `crates/pin2k-core/src/machine/tests.rs:562`, `crates/pin2k-core/src/swe1_rules.rs:164` |
| 97 | `anim_kahuna_logo_flare` | 17 | 60 | 595x226 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:5`, `crates/pin2k-core/src/machine/attract_render.rs:53`, `crates/pin2k-core/src/machine/tests.rs:459` |
| 98 | `anim_kahuna_logo_spinning` | 1 | 60 | 267x81 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:4`, `crates/pin2k-core/src/machine/attract_render.rs:57`, `crates/pin2k-core/src/machine/attract_render.rs:59`, `crates/pin2k-core/src/machine/tests.rs:466` |
| 99 | `anim_left_bdroid_shooting` | 10 | 60 | 55x91 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:39` |
| 100 | `anim_left_bdroid_standing` | 26 | 60 | 67x96 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:97`, `crates/pin2k-desktop/src/scenes_compositor.rs:29` |
| 101 | `anim_left_bdroid_unpacking` | 15 | 60 | 189x96 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:34` |
| 102 | `anim_mtt_frame` | 1 | 60 | 242x135 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:21` |
| 103 | `anim_mtt_doors_clean_anim` | 19 | 60 | 138x152 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:23` |
| 104 | `anim_mtt_doors_damage1_anim` | 19 | 60 | 138x152 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:24` |
| 105 | `anim_mtt_doors_damage2_anim` | 19 | 60 | 138x152 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:25` |
| 106 | `anim_mtt_doors_damage3_anim` | 19 | 60 | 138x152 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:26` |
| 107 | `anim_mtt_fires` | 60 | 60 | 242x150 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:30` |
| 108 | `anim_musical_chairs_intro_movie` | 56 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:137` |
| 109 | `anim_musical_chairs_intro_badge` | 1 | 60 | 394x69 | Oui | `crates/pin2k-core/src/swe1_rules.rs:157` |
| 110 | `anim_mos_espa_pipes` | 1 | 60 | 640x39 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:66` |
| 111 | `anim_mos_espa_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:53` |
| 112 | `anim_mos_espa_chuba_1` | 15 | 60 | 80x164 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:56` |
| 113 | `anim_mos_espa_chuba_2` | 15 | 60 | 132x170 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:57` |
| 114 | `anim_mos_espa_chuba_3` | 15 | 60 | 72x177 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:58` |
| 115 | `anim_mos_espa_chuba_4` | 15 | 60 | 149x182 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:59` |
| 116 | `anim_mos_espa_jarjar_choose` | 37 | 60 | 299x206 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:68` |
| 117 | `anim_mos_espa_jarjar_eat_1` | 14 | 60 | 413x205 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:71` |
| 118 | `anim_mos_espa_jarjar_eat_2` | 13 | 60 | 341x205 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:72` |
| 119 | `anim_mos_espa_jarjar_eat_3` | 14 | 60 | 329x203 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:73` |
| 120 | `anim_mos_espa_jarjar_eat_4` | 14 | 60 | 379x205 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:74` |
| 121 | `anim_mos_espa_jarjar_swallow` | 26 | 60 | 217x205 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:80` |
| 122 | `anim_multiball_start_movie` | 90 | 15 | 320x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:299`, `crates/pin2k-core/src/machine/script_exec.rs:179` |
| 123 | `anim_naboo_flyby_single` | 1 | 60 | 225x40 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:229`, `crates/pin2k-core/src/machine/system_screens.rs:360`, `crates/pin2k-core/src/machine/tests.rs:627` |
| 124 | `anim_naboo_fighter_tiny_rotate` | 60 | 60 | 100x50 | Oui | `crates/pin2k-core/src/machine/hud_display.rs:2`, `crates/pin2k-core/src/machine/hud_display.rs:25`, `crates/pin2k-core/src/machine/tests.rs:370`, `crates/pin2k-core/src/machine/tests.rs:386` |
| 125 | `anim_naboo_flyby_frame` | 1 | 60 | 640x134 | Non | - |
| 126 | `anim_naboo_flyby` | 64 | 15 | 276x120 | Non | - |
| 127 | `anim_naboo_xhair` | 1 | 60 | 150x75 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:118`, `crates/pin2k-desktop/src/scenes_compositor.rs:49` |
| 128 | `anim_naboo_xhair_blue` | 1 | 60 | 136x68 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:90` |
| 129 | `anim_obiwan_holo` | 60 | 60 | 320x160 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:392`, `crates/pin2k-desktop/src/scenes_compositor.rs:118` |
| 130 | `anim_obiwan_holo_glow` | 60 | 60 | 320x160 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:391`, `crates/pin2k-desktop/src/scenes_compositor.rs:117` |
| 131 | `anim_obiwan_holo_activate` | 18 | 60 | 255x81 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:95`, `crates/pin2k-core/src/machine/fx/triggers.rs:194` |
| 132 | `anim_obiwan_holo_anakin_pod` | 69 | 60 | 289x63 | Oui | `crates/pin2k-core/src/swe1_rules.rs:193` |
| 133 | `anim_obiwan_holo_battle_droid` | 71 | 60 | 198x138 | Oui | `crates/pin2k-core/src/swe1_rules.rs:187` |
| 134 | `anim_holo_burst` | 18 | 60 | 450x225 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:96`, `crates/pin2k-core/src/machine/fx/triggers.rs:194` |
| 135 | `anim_obiwan_holo_federation_fighter` | 72 | 60 | 400x75 | Oui | `crates/pin2k-core/src/swe1_rules.rs:189` |
| 136 | `anim_obiwan_holo_flatten` | 30 | 60 | 320x160 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:97`, `crates/pin2k-core/src/machine/fx/triggers.rs:200` |
| 137 | `anim_obiwan_holo_glow_loop` | 15 | 60 | 222x78 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:393`, `crates/pin2k-desktop/src/scenes_compositor.rs:119` |
| 138 | `anim_obiwan_holo_gungan_sub` | 72 | 60 | 400x64 | Oui | `crates/pin2k-core/src/swe1_rules.rs:190` |
| 139 | `anim_obiwan_holo_jarjar` | 71 | 60 | 174x138 | Oui | `crates/pin2k-core/src/swe1_rules.rs:196` |
| 140 | `anim_obiwan_holo_naboo_fighter` | 72 | 60 | 282x54 | Oui | `crates/pin2k-core/src/swe1_rules.rs:191` |
| 141 | `anim_obiwan_holo_queens_gun` | 71 | 60 | 213x75 | Oui | `crates/pin2k-core/src/swe1_rules.rs:194` |
| 142 | `anim_obiwan_holo_r2d2` | 73 | 60 | 202x113 | Oui | `crates/pin2k-core/src/swe1_rules.rs:199` |
| 143 | `anim_obiwan_holo_yoda` | 72 | 60 | 204x100 | Oui | `crates/pin2k-core/src/swe1_rules.rs:198` |
| 144 | `anim_p2k_logo_animated` | 90 | 15 | 200x200 | Oui | `crates/pin2k-core/src/attract.rs:20`, `crates/pin2k-core/src/attract.rs:75`, `crates/pin2k-core/src/machine/attract_render.rs:192`, `crates/pin2k-core/src/script_engine.rs:341`, `crates/pin2k-desktop/src/compositor.rs:243` (+1) |
| 145 | `anim_podrace_ground_slow` | 11 | 60 | 640x128 | Oui | `crates/pin2k-desktop/src/compositor.rs:567`, `crates/pin2k-desktop/src/compositor.rs:568` |
| 146 | `anim_podrace_sky` | 1 | 60 | 640x99 | Oui | `crates/pin2k-desktop/src/compositor.rs:564` |
| 147 | `anim_podrace_horizon` | 1 | 60 | 800x112 | Oui | `crates/pin2k-desktop/src/compositor.rs:566` |
| 148 | `anim_podrace_intro_badge` | 1 | 60 | 266x66 | Oui | `crates/pin2k-core/src/swe1_rules.rs:152` |
| 149 | `anim_probe_droid_intro_movie` | 83 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:133` |
| 150 | `anim_probe_droid_intro_badge` | 1 | 60 | 327x77 | Oui | `crates/pin2k-core/src/swe1_rules.rs:153` |
| 151 | `anim_queen_ship_intro_badge` | 1 | 60 | 304x74 | Oui | `crates/pin2k-core/src/swe1_rules.rs:154` |
| 152 | `anim_r2d2_intro_movie` | 53 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:138` |
| 153 | `anim_r2d2_intro_badge` | 1 | 60 | 355x70 | Oui | `crates/pin2k-core/src/swe1_rules.rs:158` |
| 154 | `anim_r2d2_scene_background_upper` | 1 | 60 | 640x135 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:247` |
| 155 | `anim_r2d2_scene_background_lower` | 1 | 60 | 640x131 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:248` |
| 156 | `anim_r2d2_scene_r2d2_unit` | 80 | 60 | 190x78 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:249`, `crates/pin2k-core/src/swe1_rules.rs:219` |
| 157 | `anim_r2d2_scene_dark_blue_r2_unit` | 60 | 60 | 222x60 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:216` |
| 158 | `anim_r2d2_scene_yellow_r2_unit` | 60 | 60 | 278x90 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:217` |
| 159 | `anim_r2d2_scene_blue_yellow_r2_unit` | 60 | 60 | 175x47 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:250` |
| 160 | `anim_r2d2_scene_red_r2_unit` | 60 | 60 | 214x59 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:251` |
| 161 | `anim_r2d2_scene_red_white_r2_unit` | 60 | 60 | 276x72 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:218` |
| 162 | `anim_republic_cruiser_intro` | 60 | 15 | 320x140 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:275`, `crates/pin2k-core/src/machine/system_screens.rs:235`, `crates/pin2k-core/src/machine/tests.rs:620` |
| 163 | `anim_right_bdroid_shooting` | 10 | 60 | 52x89 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:40` |
| 164 | `anim_right_bdroid_standing` | 26 | 60 | 64x96 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:98`, `crates/pin2k-desktop/src/scenes_compositor.rs:30` |
| 165 | `anim_right_bdroid_unpacking` | 15 | 60 | 186x96 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:35` |
| 166 | `anim_saber_handle` | 1 | 60 | 39x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:64`, `crates/pin2k-core/src/machine/tests.rs:351`, `crates/pin2k-core/src/machine/tests.rs:582`, `crates/pin2k-rom-tool/src/layout_extractor.rs:749` |
| 167 | `anim_saber_blue_loop` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:26`, `crates/pin2k-rom-tool/src/layout_extractor.rs:816` |
| 168 | `anim_saber_blue_flare` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:26`, `crates/pin2k-rom-tool/src/layout_extractor.rs:817` |
| 169 | `anim_saber_blue_start` | 4 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:26`, `crates/pin2k-rom-tool/src/layout_extractor.rs:815` |
| 170 | `anim_saber_green_loop` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:24`, `crates/pin2k-rom-tool/src/layout_extractor.rs:804` |
| 171 | `anim_saber_green_flare` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:24`, `crates/pin2k-rom-tool/src/layout_extractor.rs:805` |
| 172 | `anim_saber_green_start` | 4 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:24`, `crates/pin2k-rom-tool/src/layout_extractor.rs:803` |
| 173 | `anim_saber_red_loop` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:25`, `crates/pin2k-rom-tool/src/layout_extractor.rs:749`, `crates/pin2k-rom-tool/src/layout_extractor.rs:810` |
| 174 | `anim_saber_red_flare` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:25`, `crates/pin2k-rom-tool/src/layout_extractor.rs:811` |
| 175 | `anim_saber_red_start` | 4 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:25`, `crates/pin2k-rom-tool/src/layout_extractor.rs:809` |
| 176 | `anim_sith_probe_blast_1` | 2 | 60 | 253x43 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:25` |
| 177 | `anim_sith_probe_blast_2` | 2 | 60 | 137x40 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:26` |
| 178 | `anim_sith_probe_blast_3` | 2 | 60 | 109x42 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:27` |
| 179 | `anim_sith_probe_explosion_1` | 28 | 60 | 318x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:61` |
| 180 | `anim_sith_probe_explosion_2` | 56 | 60 | 347x153 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:62` |
| 181 | `anim_sith_probe_fly_1` | 36 | 60 | 110x103 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:143` |
| 182 | `anim_sith_probe_fly_2` | 12 | 60 | 108x89 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:143` |
| 183 | `anim_sith_probe_fly_3` | 12 | 60 | 114x92 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:143` |
| 184 | `anim_sith_probe_hover` | 24 | 60 | 135x99 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:183`, `crates/pin2k-desktop/src/scenes_compositor.rs:72` |
| 185 | `anim_sith_probe_hover_far` | 24 | 60 | 81x61 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:139` |
| 186 | `anim_sith_probe_miss_explosion` | 25 | 60 | 125x65 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:56` |
| 187 | `anim_sith_probe_queen` | 41 | 60 | 280x175 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:148` |
| 188 | `anim_sith_probe_queen_shoot_1` | 21 | 60 | 348x175 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:184`, `crates/pin2k-core/src/swe1_rules.rs:214`, `crates/pin2k-desktop/src/scenes_compositor.rs:70`, `crates/pin2k-desktop/src/scenes_compositor.rs:73` |
| 189 | `anim_sith_probe_queen_shoot_2` | 21 | 60 | 285x177 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:152` |
| 190 | `anim_sith_probe_queen_shoot_3` | 21 | 60 | 298x176 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:150` |
| 191 | `anim_sith_probe_target` | 20 | 60 | 125x65 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:48`, `crates/pin2k-core/src/machine/fx/scene_extras.rs:154` |
| 192 | `anim_sith_probe_xhair` | 29 | 60 | 144x77 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:185`, `crates/pin2k-desktop/src/scenes_compositor.rs:74` |
| 193 | `anim_stap_xhair` | 11 | 60 | 39x21 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:100`, `crates/pin2k-desktop/src/scenes_compositor.rs:32` |
| 194 | `anim_starwars_banner_1` | 1 | 60 | 720x240 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:64` |
| 195 | `anim_starwars_banner_2` | 1 | 60 | 720x240 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:65` |
| 196 | `anim_starwars_banner_3` | 1 | 60 | 720x240 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:66` |
| 197 | `anim_starwars_banner_4` | 1 | 60 | 720x240 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:67` |
| 198 | `anim_sub_escape_intro_movie` | 55 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:129` |
| 199 | `anim_sub_escape_intro_badge` | 1 | 60 | 386x66 | Oui | `crates/pin2k-core/src/swe1_rules.rs:149` |
| 200 | `anim_watto_cube_cube` | 22 | 60 | 269x133 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:33` |
| 201 | `anim_watto_cube` | 70 | 60 | 572x240 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:37`, `crates/pin2k-core/src/swe1_rules.rs:217`, `crates/pin2k-desktop/src/scenes_compositor.rs:90`, `crates/pin2k-desktop/src/scenes_compositor.rs:94` |
| 202 | `anim_watto_cube_flare` | 16 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:34`, `crates/pin2k-desktop/src/scenes_compositor.rs:95` |
| 203 | `anim_watto_junkshop_background` | 1 | 60 | 640x166 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:31`, `crates/pin2k-desktop/src/scenes_compositor.rs:92` |
| 204 | `anim_watto_junkshop_sky` | 1 | 60 | 640x125 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:30`, `crates/pin2k-desktop/src/scenes_compositor.rs:91` |
| 205 | `anim_watto_fly` | 30 | 60 | 121x63 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:93` |
| 206 | `menu_background_map_playfield` | 1 | 60 | 220x208 | Non | - |
| 207 | `anim_jedi_battle_idle` | 10 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:346`, `crates/pin2k-desktop/src/scenes_compositor.rs:104` |
| 208 | `anim_jedi_battle_icon_quigon` | 11 | 60 | 92x55 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:341`, `crates/pin2k-desktop/src/scenes_compositor.rs:125` |
| 209 | `anim_jedi_battle_icon_maul` | 11 | 60 | 92x55 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:342`, `crates/pin2k-desktop/src/scenes_compositor.rs:126` |
| 210 | `anim_jedi_battle_quigon_high_right_single` | 17 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:703`, `crates/pin2k-desktop/src/scenes_compositor.rs:106` |
| 211 | `anim_jedi_battle_quigon_low_left_single` | 13 | 60 | 396x178 | Non | - |
| 212 | `anim_jedi_battle_quigon_low_right_single` | 17 | 60 | 396x178 | Non | - |
| 213 | `anim_jedi_battle_quigon_middle_right_single` | 17 | 60 | 396x178 | Non | - |
| 214 | `anim_jedi_battle_quigon_4_move` | 40 | 60 | 396x178 | Non | - |
| 215 | `anim_jedi_battle_maul_chop_single` | 17 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:704`, `crates/pin2k-desktop/src/scenes_compositor.rs:107` |
| 216 | `anim_jedi_battle_maul_low_right_single` | 15 | 60 | 396x178 | Non | - |
| 217 | `anim_jedi_battle_quigon_death` | 55 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:99` |
| 218 | `anim_jedi_battle_maul_death_1` | 56 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:691`, `crates/pin2k-desktop/src/scenes_compositor.rs:108` |
| 219 | `anim_jedi_battle_maul_death_2` | 28 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:100` |
| 220 | `anim_jedi_battle_saber_sparks` | 11 | 60 | 264x148 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:350`, `crates/pin2k-desktop/src/scenes_compositor.rs:129` |
| 221 | `anim_jedi_battle_clash` | 42 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:349`, `crates/pin2k-core/src/machine/swe1_playfield.rs:702`, `crates/pin2k-desktop/src/scenes_compositor.rs:105`, `crates/pin2k-desktop/src/scenes_compositor.rs:128` |
| 222 | `anim_jedi_battle_background_hangar` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:340`, `crates/pin2k-desktop/src/scenes_compositor.rs:124` |
| 223 | `anim_jedi_battle_background_power` | 1 | 60 | 640x240 | Non | - |
| 224 | `anim_jedi_battle_background_dessert` | 1 | 60 | 640x240 | Non | - |
| 225 | `anim_sith_probe_background` | 1 | 60 | 800x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:182`, `crates/pin2k-desktop/src/scenes_compositor.rs:71` |
| 226 | `anim_sith_probe_background_2` | 1 | 60 | 800x240 | Non | - |
| 227 | `anim_spinner_1` | 56 | 60 | 150x57 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:166`, `crates/pin2k-core/src/machine/tests.rs:426`, `crates/pin2k-core/src/machine/vspinner.rs:2`, `crates/pin2k-core/src/machine/vspinner.rs:119` |
| 228 | `anim_spinner_2` | 56 | 60 | 150x57 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:168`, `crates/pin2k-core/src/machine/tests.rs:442`, `crates/pin2k-core/src/machine/vspinner.rs:3`, `crates/pin2k-core/src/machine/vspinner.rs:121` |
| 229 | `anim_sparks_1` | 15 | 60 | 200x100 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:80` |
| 230 | `anim_textbox_300` | 30 | 60 | 300x31 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:170` |
| 231 | `anim_build_c3p0` | 57 | 60 | 115x160 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:222`, `crates/pin2k-core/src/machine/system_screens.rs:336`, `crates/pin2k-core/src/machine/tests.rs:626` |
| 232 | `anim_fashion_sparkle_left` | 30 | 60 | 146x64 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:193` |
| 233 | `anim_fashion_sparkle_right` | 30 | 60 | 146x64 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:194` |
| 234 | `anim_saber_ball_loop` | 79 | 60 | 304x147 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:237`, `crates/pin2k-core/src/machine/system_screens.rs:373`, `crates/pin2k-core/src/machine/tests.rs:628` |
| 235 | `anim_saber_ball_ignite` | 73 | 60 | 640x205 | Non | - |
| 236 | `anim_podrace_intro_movie` | 55 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:132` |
| 237 | `anim_podrace_1` | 100 | 60 | 353x119 | Oui | `crates/pin2k-core/src/swe1_rules.rs:213`, `crates/pin2k-desktop/src/compositor.rs:590`, `crates/pin2k-desktop/src/scenes_compositor.rs:19` |
| 238 | `anim_podrace_2` | 40 | 60 | 147x43 | Oui | `crates/pin2k-desktop/src/compositor.rs:581` |
| 239 | `anim_podrace_icon1` | 21 | 60 | 70x30 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:123` |
| 240 | `anim_podrace_icon2` | 21 | 60 | 70x30 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:125` |
| 241 | `anim_podrace_icon3` | 20 | 60 | 70x30 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:127` |
| 242 | `anim_podrace_icon4` | 10 | 60 | 87x52 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:129` |
| 243 | `anim_podrace_icon1_blur` | 5 | 60 | 70x30 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:124` |
| 244 | `anim_podrace_icon2_blur` | 5 | 60 | 70x30 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:126` |
| 245 | `anim_podrace_icon3_blur` | 5 | 60 | 70x30 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:128` |
| 246 | `anim_podrace_icon4_blur` | 4 | 60 | 87x52 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:130` |
| 247 | `anim_podrace_checkpoint` | 1 | 60 | 191x58 | Oui | `crates/pin2k-desktop/src/compositor.rs:593` |
| 248 | `anim_podrace_ground` | 21 | 60 | 640x128 | Oui | `crates/pin2k-desktop/src/compositor.rs:568` |
| 249 | `anim_podrace_closeup` | 31 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:657`, `crates/pin2k-desktop/src/compositor.rs:554`, `crates/pin2k-desktop/src/compositor.rs:572`, `crates/pin2k-desktop/src/scenes_compositor.rs:23` |
| 250 | `anim_droid_control_ship` | 1 | 60 | 400x81 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:116`, `crates/pin2k-desktop/src/scenes_compositor.rs:47` |
| 251 | `anim_droid_control_ship2` | 1 | 60 | 150x30 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:69` |
| 252 | `anim_bonus_character_anakin` | 50 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:537` |
| 253 | `anim_bonus_character_jarjar` | 93 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:536` |
| 254 | `anim_bonus_character_maul1` | 57 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:543` |
| 255 | `anim_bonus_character_obiwan` | 39 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:539` |
| 256 | `anim_bonus_character_queen1` | 45 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:538` |
| 257 | `anim_bonus_character_quigon` | 57 | 15 | 88x76 | Oui | `crates/pin2k-core/src/machine/system_screens.rs:120`, `crates/pin2k-core/src/swe1_rules.rs:535` |
| 258 | `anim_bonus_character_r2d2` | 51 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:540` |
| 259 | `anim_bonus_character_watto` | 53 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:542` |
| 260 | `anim_bonus_character_yoda` | 77 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:541` |
| 261 | `anim_attract_anakin_web` | 1 | 60 | 202x101 | Oui | `crates/pin2k-core/src/attract.rs:24`, `crates/pin2k-core/src/attract.rs:77`, `crates/pin2k-core/src/machine/attract_render.rs:259`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:541` |
| 262 | `anim_klogo_zoom_back` | 41 | 60 | 640x240 | Oui | `crates/pin2k-core/src/attract.rs:12`, `crates/pin2k-core/src/attract.rs:73`, `crates/pin2k-desktop/src/compositor.rs:244`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:529` |
| 263 | `anim_queen_ship_intro_movie` | 55 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:134` |
| 264 | `anim_darth_maul_game_over` | 30 | 15 | 320x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:266`, `crates/pin2k-core/src/machine/system_screens.rs:212`, `crates/pin2k-core/src/machine/tests.rs:619` |
| 265 | `anim_gungan_seaweed` | 40 | 60 | 139x90 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:122`, `crates/pin2k-core/src/machine/system_screens.rs:320`, `crates/pin2k-desktop/src/scenes_compositor.rs:53` |
| 266 | `anim_gungan_bubbles` | 3 | 60 | 17x9 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:54` |
| 267 | `anim_jedi_council_chic` | 1 | 60 | 93x60 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:212` |
| 268 | `anim_attract_hasbro_ad` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/attract.rs:21`, `crates/pin2k-core/src/attract.rs:76`, `crates/pin2k-core/src/machine/attract_render.rs:201`, `crates/pin2k-desktop/src/compositor.rs:245`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:538` |
| 269 | `anim_gungan_sub_bubbles_2` | 29 | 60 | 571x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:123`, `crates/pin2k-core/src/machine/system_screens.rs:321` |
| 270 | `anim_gungan_sub_sando` | 85 | 60 | 397x156 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:41`, `crates/pin2k-core/src/machine/fx/scene_extras.rs:86` |
| 271 | `anim_spinner_lightning` | 22 | 60 | 137x76 | Oui | `crates/pin2k-core/src/machine/vspinner.rs:137` |
| 272 | `anim_podrace_2_explosion` | 26 | 60 | 350x200 | Oui | `crates/pin2k-desktop/src/compositor.rs:578` |
| 273 | `anim_podrace_3` | 11 | 60 | 60x21 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:135` |
| 274 | `anim_podrace_3_explosion` | 26 | 60 | 115x51 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:58` |
| 275 | `anim_podrace_4` | 11 | 60 | 65x22 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:136` |
| 276 | `anim_podrace_anakin_zoom` | 20 | 60 | 228x90 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:59` |
| 277 | `anim_podrace_starting` | 60 | 15 | 320x240 | Oui | `crates/pin2k-desktop/src/compositor.rs:551`, `crates/pin2k-desktop/src/compositor.rs:559` |
| 278 | `anim_bonus_icon_jedi_youth` | 47 | 60 | 140x70 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:113`, `crates/pin2k-core/src/machine/system_screens.rs:113`, `crates/pin2k-core/src/machine/system_screens.rs:120` |
| 279 | `anim_bonus_icon_jedi_knight` | 47 | 60 | 140x70 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:114`, `crates/pin2k-core/src/machine/system_screens.rs:114` |
| 280 | `anim_bonus_icon_jedi_master` | 47 | 60 | 140x70 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:115`, `crates/pin2k-core/src/machine/system_screens.rs:115` |
| 281 | `anim_bonus_icon_jedi_spirit` | 47 | 60 | 140x70 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:116`, `crates/pin2k-core/src/machine/system_screens.rs:116` |
| 282 | `anim_jedi_battle_win_icon_maul` | 1 | 60 | 370x175 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:101` |
| 283 | `anim_jedi_battle_win_icon_quigon` | 1 | 60 | 384x198 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:98` |
| 284 | `anim_obiwan_holo_queens_logo` | 72 | 60 | 216x98 | Oui | `crates/pin2k-core/src/swe1_rules.rs:195` |
| 285 | `anim_bonus_character_anakin2` | 50 | 15 | 88x76 | Non | - |
| 286 | `anim_bonus_character_maul2` | 67 | 15 | 88x76 | Non | - |
| 287 | `anim_bonus_character_queen2` | 50 | 15 | 88x76 | Non | - |
| 288 | `anim_drop_target` | 2 | 60 | 293x186 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:92` |
| 289 | `anim_shield` | 2 | 60 | 196x185 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:223` |
| 290 | `anim_action_button` | 1 | 60 | 211x100 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:93` |
| 291 | `anim_federation_fighter_redeye` | 7 | 60 | 143x38 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:76` |
| 292 | `anim_destroyer_droid_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:107`, `crates/pin2k-desktop/src/scenes_compositor.rs:39` |
| 293 | `anim_destroyer_droid_s_roll` | 65 | 60 | 341x77 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:51` |
| 294 | `anim_destroyer_droid_m_roll` | 65 | 60 | 405x88 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:57` |
| 295 | `anim_destroyer_droid_l_roll` | 61 | 60 | 609x136 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:110`, `crates/pin2k-core/src/swe1_rules.rs:208`, `crates/pin2k-desktop/src/scenes_compositor.rs:38`, `crates/pin2k-desktop/src/scenes_compositor.rs:42` |
| 296 | `anim_destroyer_droid_s_idle` | 20 | 60 | 115x74 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:108`, `crates/pin2k-desktop/src/scenes_compositor.rs:40` |
| 297 | `anim_destroyer_droid_m_idle` | 19 | 60 | 136x85 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:109`, `crates/pin2k-desktop/src/scenes_compositor.rs:41` |
| 298 | `anim_destroyer_droid_l_idle` | 20 | 60 | 209x132 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:65` |
| 299 | `anim_destroyer_droid_s_explosion` | 33 | 60 | 284x119 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:22` |
| 300 | `anim_destroyer_droid_m_explosion` | 34 | 60 | 308x171 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:23` |
| 301 | `anim_destroyer_droid_l_explosion` | 52 | 60 | 350x155 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:24` |
| 302 | `anim_destroyer_droid_s_shooting` | 11 | 60 | 276x74 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:53` |
| 303 | `anim_destroyer_droid_m_shooting` | 11 | 60 | 389x86 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:59` |
| 304 | `anim_destroyer_droid_l_shooting` | 11 | 60 | 528x132 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:63` |
| 305 | `anim_destroyer_droid_intro_badge` | 1 | 60 | 426x80 | Oui | `crates/pin2k-core/src/swe1_rules.rs:147` |
| 306 | `anim_destroyer_droid_intro_movie` | 67 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:127` |
| 307 | `anim_jedi_battle_quigon_6_move` | 57 | 60 | 396x178 | Non | - |
| 308 | `anim_jedi_battle_maul_5_move` | 43 | 60 | 396x178 | Non | - |
| 309 | `anim_jedi_battle_maul_back_block_3` | 36 | 60 | 396x178 | Non | - |
| 310 | `anim_match_movie` | 175 | 15 | 320x200 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:258`, `crates/pin2k-core/src/machine/system_screens.rs:168`, `crates/pin2k-core/src/machine/tests.rs:618` |
| 311 | `anim_hover_tank_intro_badge` | 1 | 60 | 407x53 | Oui | `crates/pin2k-core/src/swe1_rules.rs:151` |
| 312 | `anim_hover_tank_intro_movie` | 69 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:131` |
| 313 | `anim_hover_tank_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:147` |
| 314 | `anim_hover_tank_l_enter` | 25 | 60 | 288x120 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:98` |
| 315 | `anim_hover_tank_l_float` | 10 | 60 | 126x56 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:148` |
| 316 | `anim_hover_tank_l_explode` | 18 | 60 | 400x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:53` |
| 317 | `anim_hover_tank_l_shoot` | 10 | 60 | 288x117 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:104` |
| 318 | `anim_hover_tank_c_enter` | 25 | 60 | 246x162 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:100` |
| 319 | `anim_hover_tank_c_float` | 10 | 60 | 174x79 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:150` |
| 320 | `anim_hover_tank_c_explode` | 29 | 60 | 400x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:54` |
| 321 | `anim_hover_tank_c_shoot` | 10 | 60 | 200x102 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:106` |
| 322 | `anim_hover_tank_r_enter` | 25 | 60 | 267x122 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:99` |
| 323 | `anim_hover_tank_r_float` | 10 | 60 | 134x57 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:149` |
| 324 | `anim_hover_tank_r_explode` | 30 | 60 | 400x150 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:55` |
| 325 | `anim_hover_tank_r_shoot` | 10 | 60 | 399x116 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:108` |
| 326 | `anim_hover_tank_catapult_idle` | 26 | 60 | 175x87 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:151` |
| 327 | `anim_hover_tank_catapult_shoot` | 26 | 60 | 175x87 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:113` |
| 328 | `anim_hover_tank_boomer_ball` | 16 | 60 | 70x35 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:152` |
| 329 | `anim_jedi_spirit_sidious` | 40 | 60 | 246x110 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:113` |
| 330 | `anim_jedi_spirit_jedi` | 110 | 60 | 640x221 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:111` |
| 331 | `anim_jedi_spirit_electro1` | 17 | 60 | 191x147 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:115` |
| 332 | `anim_jedi_spirit_electro2` | 17 | 60 | 207x147 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:116` |
| 333 | `anim_jedi_spirit_sparks` | 18 | 60 | 357x143 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:117` |
| 334 | `anim_jedi_spirit_maul` | 40 | 60 | 246x110 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:114` |
| 335 | `anim_wattos_chance_intro_badge` | 1 | 60 | 410x95 | Oui | `crates/pin2k-core/src/swe1_rules.rs:156` |
| 336 | `anim_wattos_chance_intro_movie` | 71 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:136` |
| 337 | `anim_question_mark_intro_badge` | 1 | 60 | 446x97 | Oui | `crates/pin2k-core/src/swe1_rules.rs:159` |
| 338 | `anim_question_mark_intro_movie` | 77 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:139` |
| 339 | `anim_firework_2` | 60 | 60 | 284x240 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:81`, `crates/pin2k-core/src/machine/fx/overlays.rs:95`, `crates/pin2k-core/src/machine/fx/overlays.rs:96` |
| 340 | `anim_jackpot_icon_double` | 60 | 60 | 161x62 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:316` |
| 341 | `anim_jackpot_icon_super` | 60 | 60 | 161x62 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:314` |
| 342 | `anim_question_mark_green` | 60 | 60 | 122x62 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:265`, `crates/pin2k-core/src/swe1_rules.rs:220` |
| 343 | `anim_question_mark_blue` | 60 | 60 | 122x62 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:264` |
| 344 | `anim_watto_scene_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:213` |
| 345 | `anim_watto_scene_left_loop` | 30 | 60 | 203x147 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:192` |
| 346 | `anim_watto_scene_left_to_middle` | 10 | 60 | 384x160 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:193` |
| 347 | `anim_watto_scene_hand_to_chin_middle` | 15 | 60 | 169x164 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:194` |
| 348 | `anim_watto_scene_chin_loop_middle` | 31 | 60 | 173x162 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:195`, `crates/pin2k-core/src/machine/scene_render.rs:214` |
| 349 | `anim_watto_scene_hand_down_middle` | 14 | 60 | 170x166 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:196` |
| 350 | `anim_watto_scene_middle_to_right` | 14 | 60 | 396x153 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:197` |
| 351 | `anim_watto_scene_right_loop` | 32 | 60 | 184x158 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:198` |
| 352 | `anim_watto_scene_cube` | 30 | 60 | 126x64 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:215` |
| 353 | `anim_watto_scene_cube_flash` | 13 | 60 | 188x89 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:216` |
| 354 | `anim_watto_scene_cube_explode` | 30 | 60 | 256x118 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:78` |
| 355 | `anim_yoda_big` | 57 | 15 | 288x236 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:100` |
| 356 | `anim_obiwan_holo_destroyer` | 71 | 60 | 199x109 | Oui | `crates/pin2k-core/src/swe1_rules.rs:188` |
| 357 | `anim_obiwan_holo_hovertank` | 71 | 60 | 325x95 | Oui | `crates/pin2k-core/src/swe1_rules.rs:192` |
| 358 | `anim_obiwan_holo_question` | 72 | 60 | 200x100 | Oui | `crates/pin2k-core/src/swe1_rules.rs:200` |
| 359 | `anim_obiwan_holo_watto` | 71 | 60 | 178x104 | Oui | `crates/pin2k-core/src/swe1_rules.rs:197` |
| 360 | `anim_jedi_battle_background_hangar_movie` | 29 | 15 | 320x240 | Non | - |
| 361 | `anim_arrow_down` | 15 | 60 | 150x75 | Non | - |
| 362 | `anim_arrow_hard_right` | 10 | 60 | 100x50 | Non | - |
| 363 | `anim_arrow_soft_right` | 10 | 60 | 81x60 | Non | - |
| 364 | `anim_shield_block` | 15 | 60 | 200x25 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:90` |
| 365 | `anim_shield_block_wipe` | 9 | 60 | 200x25 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:91` |
| 366 | `anim_jedi_battle_quigon_force` | 39 | 60 | 396x178 | Non | - |
| 367 | `anim_super_skill_shot_icon` | 60 | 60 | 61x31 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:418`, `crates/pin2k-core/src/machine/scene_render.rs:421`, `crates/pin2k-core/src/machine/scene_render.rs:423`, `crates/pin2k-core/src/machine/script_exec.rs:165` |
| 368 | `anim_super_mystery_icon` | 60 | 60 | 91x31 | Oui | `crates/pin2k-core/src/machine/fx/scene_extras.rs:224` |
| 369 | `anim_bonus_icon` | 60 | 60 | 91x32 | Non | - |
| 370 | `anim_green_icon_boom_big` | 9 | 60 | 354x110 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:85` |
| 371 | `anim_blue_icon_boom_big` | 9 | 60 | 354x110 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:83` |
| 372 | `anim_blue_icon_boom_small` | 10 | 60 | 158x81 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:84` |
| 373 | `anim_bumper_bubbles_1` | 33 | 60 | 235x122 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:88` |
| 374 | `anim_bumper_bubbles_2` | 41 | 60 | 157x121 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:89` |
| 375 | `anim_jarjar_thumbs_up` | 26 | 60 | 115x60 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:72` |
| 376 | `anim_spinner_bracket_left` | 1 | 60 | 150x57 | Oui | `crates/pin2k-core/src/machine/tests.rs:423`, `crates/pin2k-core/src/machine/vspinner.rs:2`, `crates/pin2k-core/src/machine/vspinner.rs:114` |
| 377 | `anim_spinner_bracket_right` | 1 | 60 | 150x57 | Oui | `crates/pin2k-core/src/machine/tests.rs:439`, `crates/pin2k-core/src/machine/vspinner.rs:3`, `crates/pin2k-core/src/machine/vspinner.rs:116` |
| 378 | `anim_spinner_explosion` | 15 | 60 | 150x75 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:87` |
| 379 | `anim_queen_escape_misc_movie` | 28 | 15 | 320x140 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:65` |
| 380 | `anim_jedi_stand_misc_movie` | 33 | 15 | 320x140 | Oui | `crates/pin2k-core/src/machine/fx/anims.rs:66` |
| 381 | `anim_oola_girls` | 1 | 60 | 272x182 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:38` |
| 382 | `anim_rayboot` | 1 | 60 | 255x148 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:39` |
| 383 | `anim_jedi_battle_background_dessert_maul` | 1 | 60 | 640x240 | Non | - |
| 384 | `anim_flipinfo_d` | 1 | 60 | 180x142 | Non | - |
| 385 | `anim_flipinfo_e` | 1 | 60 | 640x240 | Non | - |
| 386 | `system_video_test_align` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:359`, `crates/pin2k-core/src/machine/system_screens.rs:518`, `crates/pin2k-core/src/machine/tests.rs:696` |
| 387 | `menu_background_script_bally` | 1 | 60 | 640x100 | Non | - |
| 388 | `menu_background_logo_bally` | 1 | 60 | 640x32 | Non | - |
| 389 | `menu_background_logo_williams` | 1 | 60 | 640x32 | Non | - |
| 390 | `menu_background_utility_lampgrid` | 1 | 60 | 420x208 | Non | - |
| 391 | `menu_background_utility_switchgrid` | 1 | 60 | 420x208 | Non | - |
| 392 | `menu_background_script_williams` | 1 | 60 | 640x100 | Non | - |

---

## 2. Polices Bitmap (19 entrees)

| # | Police | Glyphes | Utilise | Emplacement(s) dans le code |
|---:|---|---:|---|---|
| 1 | `font_basic_19` | 94 | Oui | `crates/pin2k-desktop/src/compositor.rs:627` |
| 2 | `font_kahuna_shadow_9` | 95 | Oui | `crates/pin2k-core/src/machine/score_display.rs:4`, `crates/pin2k-core/src/machine/score_display.rs:82`, `crates/pin2k-core/src/machine/score_display.rs:95`, `crates/pin2k-core/src/machine/score_display.rs:119`, `crates/pin2k-core/src/machine/system_screens.rs:325` (+1) |
| 3 | `font_shadow_7` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:218`, `crates/pin2k-desktop/src/compositor.rs:629` |
| 4 | `font_shadow_7_narrow` | 95 | Oui | `crates/pin2k-core/src/machine/system_screens.rs:348`, `crates/pin2k-desktop/src/compositor.rs:630` |
| 5 | `font_shadow_9` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:236`, `crates/pin2k-core/src/machine/system_screens.rs:137`, `crates/pin2k-core/src/machine/system_screens.rs:147`, `crates/pin2k-core/src/machine/system_screens.rs:247`, `crates/pin2k-desktop/src/compositor.rs:618` (+1) |
| 6 | `font_shadow_9_narrow` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:127`, `crates/pin2k-core/src/machine/scene_render.rs:373`, `crates/pin2k-core/src/machine/scene_render.rs:439`, `crates/pin2k-desktop/src/compositor.rs:632` |
| 7 | `font_shadow_11` | 95 | Oui | `crates/pin2k-core/src/machine/fx/overlays.rs:50`, `crates/pin2k-core/src/machine/scene_render.rs:80`, `crates/pin2k-core/src/machine/scene_render.rs:160`, `crates/pin2k-core/src/machine/scene_render.rs:170`, `crates/pin2k-core/src/machine/scene_render.rs:277` (+5) |
| 8 | `font_shadow_11_narrow` | 95 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:141`, `crates/pin2k-core/src/machine/attract_render.rs:153`, `crates/pin2k-core/src/machine/attract_render.rs:225`, `crates/pin2k-core/src/machine/attract_render.rs:247`, `crates/pin2k-core/src/machine/attract_render.rs:266` (+13) |
| 9 | `font_shadow_15` | 95 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:87`, `crates/pin2k-core/src/machine/attract_render.rs:97`, `crates/pin2k-core/src/machine/attract_render.rs:170`, `crates/pin2k-core/src/machine/attract_render.rs:205`, `crates/pin2k-core/src/machine/attract_render.rs:215` (+14) |
| 10 | `font_shadow_15_narrow` | 95 | Oui | `crates/pin2k-core/src/machine/system_screens.rs:375`, `crates/pin2k-desktop/src/compositor.rs:613`, `crates/pin2k-desktop/src/compositor.rs:636` |
| 11 | `font_shadow_22` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:303`, `crates/pin2k-desktop/src/compositor.rs:637` |
| 12 | `font_starwars_19` | 94 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:74`, `crates/pin2k-core/src/machine/attract_render.rs:123`, `crates/pin2k-core/src/machine/attract_render.rs:180`, `crates/pin2k-core/src/machine/attract_render.rs:237`, `crates/pin2k-core/src/machine/fx/overlays.rs:107` (+10) |
| 13 | `font_system_7` | 95 | Oui | `crates/pin2k-core/src/machine/hud_display.rs:4`, `crates/pin2k-core/src/machine/hud_display.rs:50`, `crates/pin2k-core/src/machine/hud_display.rs:60`, `crates/pin2k-core/src/machine/scene_render.rs:396`, `crates/pin2k-core/src/machine/scene_render.rs:406` (+17) |
| 14 | `font_system_9` | 95 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:6`, `crates/pin2k-core/src/machine/attract_render.rs:399`, `crates/pin2k-core/src/machine/attract_render.rs:415`, `crates/pin2k-core/src/machine/attract_render.rs:425`, `crates/pin2k-core/src/machine/attract_render.rs:441` (+8) |
| 15 | `font_system_10` | 95 | Oui | `crates/pin2k-desktop/src/compositor.rs:641` |
| 16 | `font_system_11` | 105 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:202`, `crates/pin2k-core/src/machine/system_screens.rs:68`, `crates/pin2k-core/src/machine/system_screens.rs:283`, `crates/pin2k-core/src/machine/system_screens.rs:647`, `crates/pin2k-core/src/machine/system_screens.rs:681` (+2) |
| 17 | `font_system_15` | 95 | Oui | `crates/pin2k-core/src/machine/system_screens.rs:58`, `crates/pin2k-desktop/src/compositor.rs:643` |
| 18 | `font_system_20` | 95 | Oui | `crates/pin2k-desktop/src/compositor.rs:644` |
| 19 | `font_system_fxd6x7` | 95 | Oui | `crates/pin2k-desktop/src/compositor.rs:645` |

---

## 3. Pistes Audio DCS-2 (689 entrees)

| # | Nom | DCS ID | Mode Loop | Fichier | Utilise | Emplacement(s) dans le code |
|---:|---|---|---|---|---|---|
| 1 | `dcs-bong` | `0x003A` | OneShot | `dcs-bong.ogg` | Oui | `crates/pin2k-core/src/admin.rs:141`, `crates/pin2k-core/src/rfm_tables.rs:111`, `crates/pin2k-core/src/swe1_tables.rs:163`, `crates/pin2k-rom-tool/src/asset_extractor.rs:586`, `crates/pin2k-rom-tool/src/asset_extractor.rs:587` (+2) |
| 2 | `S0001-LP1` | `0x0001` | LoopIntro | `S0001-LP1.ogg` | Oui | `crates/pin2k-core/src/admin.rs:142`, `crates/pin2k-core/src/mods.rs:61`, `crates/pin2k-core/src/rfm_tables.rs:112`, `crates/pin2k-core/src/swe1_audio.rs:25`, `crates/pin2k-core/src/swe1_audio.rs:350` (+11) |
| 3 | `S0001-LP2` | `0x0001` | LoopBody | `S0001-LP2.ogg` | Oui | `crates/pin2k-core/src/admin.rs:142`, `crates/pin2k-core/src/mods.rs:61`, `crates/pin2k-core/src/rfm_tables.rs:112`, `crates/pin2k-core/src/swe1_audio.rs:25`, `crates/pin2k-core/src/swe1_audio.rs:350` (+10) |
| 4 | `S0002-LP` | `0x0002` | SingleLoop | `S0002-LP.ogg` | Oui | `crates/pin2k-core/src/rfm_tables.rs:113`, `crates/pin2k-core/src/swe1_audio.rs:232`, `crates/pin2k-core/src/swe1_audio.rs:824`, `crates/pin2k-core/src/swe1_tables.rs:165`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:614` |
| 5 | `S0003-LP` | `0x0003` | SingleLoop | `S0003-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:349`, `crates/pin2k-core/src/swe1_audio.rs:1025` |
| 6 | `S0004` | `0x0004` | OneShot | `S0004.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:376`, `crates/pin2k-core/src/swe1_audio.rs:540` |
| 7 | `S0006-LP` | `0x0006` | SingleLoop | `S0006-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:42`, `crates/pin2k-core/src/swe1_audio.rs:1003` |
| 8 | `S0007-LP` | `0x0007` | SingleLoop | `S0007-LP.ogg` | Oui | `crates/pin2k-core/src/rfm_tables.rs:114`, `crates/pin2k-core/src/swe1_audio.rs:1004` |
| 9 | `S0009-LP` | `0x0009` | SingleLoop | `S0009-LP.ogg` | Oui | `crates/pin2k-core/src/admin.rs:144`, `crates/pin2k-core/src/swe1_audio.rs:297`, `crates/pin2k-core/src/swe1_audio.rs:900`, `crates/pin2k-core/src/swe1_tables.rs:166` |
| 10 | `S000A-LP` | `0x000A` | SingleLoop | `S000A-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:267`, `crates/pin2k-core/src/swe1_audio.rs:794`, `crates/pin2k-core/src/swe1_tables.rs:167` |
| 11 | `S000B-LP1` | `0x000B` | LoopIntro | `S000B-LP1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:819`, `crates/pin2k-core/src/swe1_tables.rs:175` |
| 12 | `S000B-LP2` | `0x000B` | LoopBody | `S000B-LP2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:819`, `crates/pin2k-core/src/swe1_tables.rs:175` |
| 13 | `S000C-LP` | `0x000C` | SingleLoop | `S000C-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:58`, `crates/pin2k-core/src/swe1_audio.rs:820` |
| 14 | `S000D` | `0x000D` | OneShot | `S000D.ogg` | Oui | `crates/pin2k-core/src/rfm_tables.rs:115`, `crates/pin2k-core/src/swe1_audio.rs:161`, `crates/pin2k-core/src/swe1_audio.rs:221`, `crates/pin2k-core/src/swe1_audio.rs:594`, `crates/pin2k-core/src/swe1_tables.rs:184` |
| 15 | `S000E-LP` | `0x000E` | SingleLoop | `S000E-LP.ogg` | Oui | `crates/pin2k-core/src/admin.rs:145`, `crates/pin2k-core/src/swe1_audio.rs:97`, `crates/pin2k-core/src/swe1_audio.rs:579`, `crates/pin2k-core/src/swe1_tables.rs:168`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:597` |
| 16 | `S000F-LP1` | `0x000F` | LoopIntro | `S000F-LP1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:112`, `crates/pin2k-core/src/swe1_audio.rs:220`, `crates/pin2k-core/src/swe1_audio.rs:580` |
| 17 | `S000F-LP2` | `0x000F` | LoopBody | `S000F-LP2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:112`, `crates/pin2k-core/src/swe1_audio.rs:220`, `crates/pin2k-core/src/swe1_audio.rs:580` |
| 18 | `S0010-LP` | `0x0010` | SingleLoop | `S0010-LP.ogg` | Oui | `crates/pin2k-core/src/machine/system_screens.rs:732`, `crates/pin2k-core/src/swe1_audio.rs:72`, `crates/pin2k-core/src/swe1_audio.rs:325`, `crates/pin2k-core/src/swe1_audio.rs:645`, `crates/pin2k-core/src/swe1_tables.rs:169` |
| 19 | `S0011-LP` | `0x0011` | SingleLoop | `S0011-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:244`, `crates/pin2k-core/src/swe1_audio.rs:247`, `crates/pin2k-core/src/swe1_audio.rs:827` |
| 20 | `S0012-LP` | `0x0012` | SingleLoop | `S0012-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:264`, `crates/pin2k-core/src/swe1_audio.rs:793` |
| 21 | `S0013-LP` | `0x0013` | SingleLoop | `S0013-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:197`, `crates/pin2k-core/src/swe1_audio.rs:723`, `crates/pin2k-core/src/swe1_tables.rs:170` |
| 22 | `S0014` | `0x0014` | OneShot | `S0014.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:216`, `crates/pin2k-core/src/swe1_audio.rs:631` |
| 23 | `S0015` | `0x0015` | OneShot | `S0015.ogg` | Oui | `crates/pin2k-core/src/machine/attract_render.rs:129`, `crates/pin2k-core/src/machine/attract_render.rs:231`, `crates/pin2k-core/src/machine/tests.rs:233`, `crates/pin2k-core/src/rfm_tables.rs:116`, `crates/pin2k-core/src/swe1_audio.rs:345` (+1) |
| 24 | `S0016` | `0x0016` | OneShot | `S0016.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:75`, `crates/pin2k-core/src/swe1_audio.rs:116`, `crates/pin2k-core/src/swe1_audio.rs:215`, `crates/pin2k-core/src/swe1_audio.rs:633` |
| 25 | `S0018` | `0x0018` | OneShot | `S0018.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:237`, `crates/pin2k-core/src/swe1_audio.rs:634` |
| 26 | `S0019` | `0x0019` | OneShot | `S0019.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:351`, `crates/pin2k-core/src/swe1_audio.rs:635` |
| 27 | `S001A` | `0x001A` | OneShot | `S001A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:27`, `crates/pin2k-core/src/swe1_audio.rs:636` |
| 28 | `S001C` | `0x001C` | OneShot | `S001C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:49`, `crates/pin2k-core/src/swe1_audio.rs:616` |
| 29 | `S001E-LP` | `0x001E` | SingleLoop | `S001E-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:368`, `crates/pin2k-core/src/swe1_audio.rs:537` |
| 30 | `S001F-LP1` | `0x001F` | LoopIntro | `S001F-LP1.ogg` | Oui | `crates/pin2k-core/src/machine/attract_render.rs:18`, `crates/pin2k-core/src/machine/scene_render.rs:273`, `crates/pin2k-core/src/machine/system_screens.rs:266`, `crates/pin2k-core/src/machine/system_screens.rs:523`, `crates/pin2k-core/src/machine/system_screens.rs:569` (+5) |
| 31 | `S001F-LP2` | `0x001F` | LoopBody | `S001F-LP2.ogg` | Oui | `crates/pin2k-core/src/machine/attract_render.rs:18`, `crates/pin2k-core/src/machine/scene_render.rs:273`, `crates/pin2k-core/src/machine/system_screens.rs:266`, `crates/pin2k-core/src/machine/system_screens.rs:523`, `crates/pin2k-core/src/machine/system_screens.rs:569` (+5) |
| 32 | `S0021-LP` | `0x0021` | SingleLoop | `S0021-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:318`, `crates/pin2k-core/src/swe1_audio.rs:920`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:634` |
| 33 | `S0022-LP` | `0x0022` | SingleLoop | `S0022-LP.ogg` | Oui | `crates/pin2k-core/src/admin.rs:143`, `crates/pin2k-core/src/swe1_audio.rs:137`, `crates/pin2k-core/src/swe1_audio.rs:160`, `crates/pin2k-core/src/swe1_audio.rs:607`, `crates/pin2k-core/src/swe1_tables.rs:171` |
| 34 | `S0023-LP` | `0x0023` | SingleLoop | `S0023-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:298`, `crates/pin2k-core/src/swe1_audio.rs:901` |
| 35 | `S0024-LP` | `0x0024` | SingleLoop | `S0024-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:334`, `crates/pin2k-core/src/swe1_audio.rs:539` |
| 36 | `S0025-LP` | `0x0025` | SingleLoop | `S0025-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:162`, `crates/pin2k-core/src/swe1_audio.rs:219`, `crates/pin2k-core/src/swe1_audio.rs:611` |
| 37 | `S0026-LP` | `0x0026` | SingleLoop | `S0026-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1023` |
| 38 | `S0027` | `0x0027` | OneShot | `S0027.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:229`, `crates/pin2k-core/src/swe1_audio.rs:630` |
| 39 | `S002D-LP` | `0x002D` | SingleLoop | `S002D-LP.ogg` | Oui | `crates/pin2k-core/src/admin.rs:146`, `crates/pin2k-core/src/swe1_audio.rs:231`, `crates/pin2k-core/src/swe1_audio.rs:829`, `crates/pin2k-core/src/swe1_tables.rs:172`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:584` |
| 40 | `S002E-LP` | `0x002E` | SingleLoop | `S002E-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:230`, `crates/pin2k-core/src/swe1_audio.rs:1024` |
| 41 | `S002F-LP` | `0x002F` | SingleLoop | `S002F-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:617`, `crates/pin2k-core/src/swe1_tables.rs:189` |
| 42 | `S0030` | `0x0030` | OneShot | `S0030.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:214`, `crates/pin2k-core/src/swe1_audio.rs:792` |
| 43 | `S0031-LP1` | `0x0031` | LoopIntro | `S0031-LP1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:393`, `crates/pin2k-core/src/swe1_audio.rs:1066`, `crates/pin2k-core/src/swe1_tables.rs:173` |
| 44 | `S0031-LP2` | `0x0031` | LoopBody | `S0031-LP2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:393`, `crates/pin2k-core/src/swe1_audio.rs:1066`, `crates/pin2k-core/src/swe1_tables.rs:173` |
| 45 | `S0032-LP` | `0x0032` | SingleLoop | `S0032-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:52`, `crates/pin2k-core/src/swe1_audio.rs:409` |
| 46 | `S0033` | `0x0033` | OneShot | `S0033.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:51`, `crates/pin2k-core/src/swe1_audio.rs:410` |
| 47 | `S005F` | `0x005F` | OneShot | `S005F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:834` |
| 48 | `S0060` | `0x0060` | OneShot | `S0060.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:176`, `crates/pin2k-core/src/swe1_audio.rs:835`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:702` |
| 49 | `S0061` | `0x0061` | OneShot | `S0061.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:177`, `crates/pin2k-core/src/swe1_audio.rs:836`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:713` |
| 50 | `S0062` | `0x0062` | OneShot | `S0062.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:178`, `crates/pin2k-core/src/swe1_audio.rs:837` |
| 51 | `S0063` | `0x0063` | OneShot | `S0063.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:179`, `crates/pin2k-core/src/swe1_audio.rs:838` |
| 52 | `S0070` | `0x0070` | OneShot | `S0070.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:227`, `crates/pin2k-core/src/swe1_audio.rs:340`, `crates/pin2k-core/src/swe1_audio.rs:1081` |
| 53 | `S007C` | `0x007C` | OneShot | `S007C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1033` |
| 54 | `S0081` | `0x0081` | OneShot | `S0081.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:47`, `crates/pin2k-core/src/swe1_audio.rs:804` |
| 55 | `S0082` | `0x0082` | OneShot | `S0082.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:326`, `crates/pin2k-core/src/swe1_audio.rs:330`, `crates/pin2k-core/src/swe1_audio.rs:354`, `crates/pin2k-core/src/swe1_audio.rs:962` |
| 56 | `S0083` | `0x0083` | OneShot | `S0083.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:326`, `crates/pin2k-core/src/swe1_audio.rs:354`, `crates/pin2k-core/src/swe1_audio.rs:963` |
| 57 | `S0084` | `0x0084` | OneShot | `S0084.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:326`, `crates/pin2k-core/src/swe1_audio.rs:354`, `crates/pin2k-core/src/swe1_audio.rs:964` |
| 58 | `S0085` | `0x0085` | OneShot | `S0085.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:326`, `crates/pin2k-core/src/swe1_audio.rs:354`, `crates/pin2k-core/src/swe1_audio.rs:965` |
| 59 | `S0086` | `0x0086` | OneShot | `S0086.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:326`, `crates/pin2k-core/src/swe1_audio.rs:354`, `crates/pin2k-core/src/swe1_audio.rs:966` |
| 60 | `S0087` | `0x0087` | OneShot | `S0087.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:326`, `crates/pin2k-core/src/swe1_audio.rs:354`, `crates/pin2k-core/src/swe1_audio.rs:967` |
| 61 | `S0088` | `0x0088` | OneShot | `S0088.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:326`, `crates/pin2k-core/src/swe1_audio.rs:354`, `crates/pin2k-core/src/swe1_audio.rs:968` |
| 62 | `S0089` | `0x0089` | OneShot | `S0089.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:332`, `crates/pin2k-core/src/swe1_audio.rs:971` |
| 63 | `S008A` | `0x008A` | OneShot | `S008A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:333`, `crates/pin2k-core/src/swe1_audio.rs:969` |
| 64 | `S00E1` | `0x00E1` | OneShot | `S00E1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:31`, `crates/pin2k-core/src/swe1_audio.rs:61`, `crates/pin2k-core/src/swe1_audio.rs:85`, `crates/pin2k-core/src/swe1_audio.rs:159`, `crates/pin2k-core/src/swe1_audio.rs:163` (+4) |
| 65 | `S00EC` | `0x00EC` | OneShot | `S00EC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:92`, `crates/pin2k-core/src/swe1_audio.rs:93`, `crates/pin2k-core/src/swe1_audio.rs:100`, `crates/pin2k-core/src/swe1_audio.rs:129`, `crates/pin2k-core/src/swe1_audio.rs:174` (+3) |
| 66 | `S00F9` | `0x00F9` | OneShot | `S00F9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:26`, `crates/pin2k-core/src/swe1_audio.rs:346`, `crates/pin2k-core/src/swe1_audio.rs:643` |
| 67 | `S0111` | `0x0111` | OneShot | `S0111.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:618` |
| 68 | `S0190` | `0x0190` | OneShot | `S0190.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:143`, `crates/pin2k-core/src/swe1_audio.rs:166`, `crates/pin2k-core/src/swe1_audio.rs:184`, `crates/pin2k-core/src/swe1_audio.rs:196`, `crates/pin2k-core/src/swe1_audio.rs:748` |
| 69 | `S0191` | `0x0191` | OneShot | `S0191.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:379`, `crates/pin2k-core/src/swe1_audio.rs:769` |
| 70 | `S0192` | `0x0192` | OneShot | `S0192.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:378`, `crates/pin2k-core/src/swe1_audio.rs:689` |
| 71 | `S0193` | `0x0193` | OneShot | `S0193.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:166`, `crates/pin2k-core/src/swe1_audio.rs:378`, `crates/pin2k-core/src/swe1_audio.rs:684` |
| 72 | `S0194` | `0x0194` | OneShot | `S0194.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:184`, `crates/pin2k-core/src/swe1_audio.rs:365`, `crates/pin2k-core/src/swe1_audio.rs:731` |
| 73 | `S0195` | `0x0195` | OneShot | `S0195.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:378`, `crates/pin2k-core/src/swe1_audio.rs:699` |
| 74 | `S0196` | `0x0196` | OneShot | `S0196.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:166`, `crates/pin2k-core/src/swe1_audio.rs:365`, `crates/pin2k-core/src/swe1_audio.rs:378`, `crates/pin2k-core/src/swe1_audio.rs:685` |
| 75 | `S0197` | `0x0197` | OneShot | `S0197.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:768` |
| 76 | `S0198` | `0x0198` | OneShot | `S0198.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:198`, `crates/pin2k-core/src/swe1_audio.rs:728` |
| 77 | `S0199` | `0x0199` | OneShot | `S0199.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:704` |
| 78 | `S019A` | `0x019A` | OneShot | `S019A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:190`, `crates/pin2k-core/src/swe1_audio.rs:200`, `crates/pin2k-core/src/swe1_audio.rs:366`, `crates/pin2k-core/src/swe1_audio.rs:374`, `crates/pin2k-core/src/swe1_audio.rs:729` |
| 79 | `S019B` | `0x019B` | OneShot | `S019B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:147`, `crates/pin2k-core/src/swe1_audio.rs:776` |
| 80 | `S019C` | `0x019C` | OneShot | `S019C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:380`, `crates/pin2k-core/src/swe1_audio.rs:746` |
| 81 | `S019D` | `0x019D` | OneShot | `S019D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:770` |
| 82 | `S019E` | `0x019E` | OneShot | `S019E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:379`, `crates/pin2k-core/src/swe1_audio.rs:717` |
| 83 | `S019F` | `0x019F` | OneShot | `S019F.ogg` | Oui | `crates/pin2k-core/src/machine/system_screens.rs:456`, `crates/pin2k-core/src/machine/system_screens.rs:704`, `crates/pin2k-core/src/machine/system_screens.rs:733`, `crates/pin2k-core/src/swe1_audio.rs:139`, `crates/pin2k-core/src/swe1_audio.rs:143` (+1) |
| 84 | `S01A0` | `0x01A0` | OneShot | `S01A0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:369`, `crates/pin2k-core/src/swe1_audio.rs:570` |
| 85 | `S01A1` | `0x01A1` | OneShot | `S01A1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:143`, `crates/pin2k-core/src/swe1_audio.rs:166`, `crates/pin2k-core/src/swe1_audio.rs:569` |
| 86 | `S01A2` | `0x01A2` | OneShot | `S01A2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:46`, `crates/pin2k-core/src/swe1_audio.rs:143`, `crates/pin2k-core/src/swe1_audio.rs:166`, `crates/pin2k-core/src/swe1_audio.rs:567` |
| 87 | `S01A3` | `0x01A3` | OneShot | `S01A3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:81`, `crates/pin2k-core/src/swe1_audio.rs:566` |
| 88 | `S01A4` | `0x01A4` | OneShot | `S01A4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:885` |
| 89 | `S01A6` | `0x01A6` | OneShot | `S01A6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:641` |
| 90 | `S01A7` | `0x01A7` | OneShot | `S01A7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:81`, `crates/pin2k-core/src/swe1_audio.rs:640` |
| 91 | `S01A8` | `0x01A8` | OneShot | `S01A8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:155`, `crates/pin2k-core/src/swe1_audio.rs:543` |
| 92 | `S01A9` | `0x01A9` | OneShot | `S01A9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:38`, `crates/pin2k-core/src/swe1_audio.rs:46`, `crates/pin2k-core/src/swe1_audio.rs:542` |
| 93 | `S01AA` | `0x01AA` | OneShot | `S01AA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:39`, `crates/pin2k-core/src/swe1_audio.rs:46`, `crates/pin2k-core/src/swe1_audio.rs:546` |
| 94 | `S01AB` | `0x01AB` | OneShot | `S01AB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:541` |
| 95 | `S01AC` | `0x01AC` | OneShot | `S01AC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:139`, `crates/pin2k-core/src/swe1_audio.rs:544` |
| 96 | `S01AD` | `0x01AD` | OneShot | `S01AD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:657` |
| 97 | `S01AE` | `0x01AE` | OneShot | `S01AE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:118`, `crates/pin2k-core/src/swe1_audio.rs:658` |
| 98 | `S01AF` | `0x01AF` | OneShot | `S01AF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:549` |
| 99 | `S01B0` | `0x01B0` | OneShot | `S01B0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:550` |
| 100 | `S01B1` | `0x01B1` | OneShot | `S01B1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:973` |
| 101 | `S01B2` | `0x01B2` | OneShot | `S01B2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:979` |
| 102 | `S01B3` | `0x01B3` | OneShot | `S01B3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:976` |
| 103 | `S01B4` | `0x01B4` | OneShot | `S01B4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:978` |
| 104 | `S01B5` | `0x01B5` | OneShot | `S01B5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:980`, `crates/pin2k-core/src/swe1_tables.rs:183` |
| 105 | `S01B6` | `0x01B6` | OneShot | `S01B6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:975` |
| 106 | `S01B7` | `0x01B7` | OneShot | `S01B7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:977` |
| 107 | `S01B8` | `0x01B8` | OneShot | `S01B8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:576` |
| 108 | `S01B9` | `0x01B9` | OneShot | `S01B9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:596` |
| 109 | `S01BA` | `0x01BA` | OneShot | `S01BA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:140`, `crates/pin2k-core/src/swe1_audio.rs:144`, `crates/pin2k-core/src/swe1_audio.rs:274`, `crates/pin2k-core/src/swe1_audio.rs:592` |
| 110 | `S01BB` | `0x01BB` | OneShot | `S01BB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:113`, `crates/pin2k-core/src/swe1_audio.rs:597` |
| 111 | `S01BC` | `0x01BC` | OneShot | `S01BC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:274`, `crates/pin2k-core/src/swe1_audio.rs:578` |
| 112 | `S01BD` | `0x01BD` | OneShot | `S01BD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:140`, `crates/pin2k-core/src/swe1_audio.rs:144`, `crates/pin2k-core/src/swe1_audio.rs:274`, `crates/pin2k-core/src/swe1_audio.rs:577` |
| 113 | `S01BF` | `0x01BF` | OneShot | `S01BF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:274`, `crates/pin2k-core/src/swe1_audio.rs:593` |
| 114 | `S01C0` | `0x01C0` | OneShot | `S01C0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:140`, `crates/pin2k-core/src/swe1_audio.rs:144`, `crates/pin2k-core/src/swe1_audio.rs:274`, `crates/pin2k-core/src/swe1_audio.rs:595` |
| 115 | `S01C1` | `0x01C1` | OneShot | `S01C1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:209`, `crates/pin2k-core/src/swe1_audio.rs:581`, `crates/pin2k-core/src/swe1_tables.rs:185` |
| 116 | `S01C2` | `0x01C2` | OneShot | `S01C2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:470` |
| 117 | `S01C3` | `0x01C3` | OneShot | `S01C3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:119`, `crates/pin2k-core/src/swe1_audio.rs:292`, `crates/pin2k-core/src/swe1_audio.rs:458` |
| 118 | `S01C5` | `0x01C5` | OneShot | `S01C5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:142`, `crates/pin2k-core/src/swe1_audio.rs:525` |
| 119 | `S01C7` | `0x01C7` | OneShot | `S01C7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:438` |
| 120 | `S01C8` | `0x01C8` | OneShot | `S01C8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:527` |
| 121 | `S01C9` | `0x01C9` | OneShot | `S01C9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:524` |
| 122 | `S01CA` | `0x01CA` | OneShot | `S01CA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:502` |
| 123 | `S01CB` | `0x01CB` | OneShot | `S01CB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:99`, `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:517` |
| 124 | `S01CC` | `0x01CC` | OneShot | `S01CC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:530` |
| 125 | `S01CD` | `0x01CD` | OneShot | `S01CD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:452` |
| 126 | `S01CE` | `0x01CE` | OneShot | `S01CE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:81`, `crates/pin2k-core/src/swe1_audio.rs:119`, `crates/pin2k-core/src/swe1_audio.rs:526` |
| 127 | `S01CF` | `0x01CF` | OneShot | `S01CF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:440` |
| 128 | `S01D0` | `0x01D0` | OneShot | `S01D0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:172`, `crates/pin2k-core/src/swe1_audio.rs:498` |
| 129 | `S01D1` | `0x01D1` | OneShot | `S01D1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:465` |
| 130 | `S01D5` | `0x01D5` | OneShot | `S01D5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:501` |
| 131 | `S01D6` | `0x01D6` | OneShot | `S01D6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:856` |
| 132 | `S01D7` | `0x01D7` | OneShot | `S01D7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:851`, `crates/pin2k-rom-tool/src/prism.rs:135` |
| 133 | `S01D8` | `0x01D8` | OneShot | `S01D8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:866` |
| 134 | `S01D9` | `0x01D9` | OneShot | `S01D9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:863`, `crates/pin2k-rom-tool/src/prism.rs:25` |
| 135 | `S01DA` | `0x01DA` | OneShot | `S01DA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:849` |
| 136 | `S01DB` | `0x01DB` | OneShot | `S01DB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:38`, `crates/pin2k-core/src/swe1_audio.rs:855` |
| 137 | `S01DC` | `0x01DC` | OneShot | `S01DC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:38`, `crates/pin2k-core/src/swe1_audio.rs:166`, `crates/pin2k-core/src/swe1_audio.rs:852` |
| 138 | `S01DD` | `0x01DD` | OneShot | `S01DD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:139`, `crates/pin2k-core/src/swe1_audio.rs:862` |
| 139 | `S01DE` | `0x01DE` | OneShot | `S01DE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:38`, `crates/pin2k-core/src/swe1_audio.rs:139`, `crates/pin2k-core/src/swe1_audio.rs:858` |
| 140 | `S01DF` | `0x01DF` | OneShot | `S01DF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:868` |
| 141 | `S01E0` | `0x01E0` | OneShot | `S01E0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:115`, `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:927` |
| 142 | `S01E1` | `0x01E1` | OneShot | `S01E1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:935` |
| 143 | `S01E2` | `0x01E2` | OneShot | `S01E2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:115`, `crates/pin2k-core/src/swe1_audio.rs:953` |
| 144 | `S01E3` | `0x01E3` | OneShot | `S01E3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:115`, `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:938` |
| 145 | `S01E4` | `0x01E4` | OneShot | `S01E4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:210`, `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:954` |
| 146 | `S01E5` | `0x01E5` | OneShot | `S01E5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:210`, `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:943` |
| 147 | `S01E6` | `0x01E6` | OneShot | `S01E6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:374`, `crates/pin2k-core/src/swe1_audio.rs:959` |
| 148 | `S01E7` | `0x01E7` | OneShot | `S01E7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:210`, `crates/pin2k-core/src/swe1_audio.rs:952` |
| 149 | `S01E8` | `0x01E8` | OneShot | `S01E8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:38`, `crates/pin2k-core/src/swe1_audio.rs:139`, `crates/pin2k-core/src/swe1_audio.rs:926` |
| 150 | `S01E9` | `0x01E9` | OneShot | `S01E9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:139`, `crates/pin2k-core/src/swe1_audio.rs:956` |
| 151 | `S01EA` | `0x01EA` | OneShot | `S01EA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:958` |
| 152 | `S01EB` | `0x01EB` | OneShot | `S01EB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:948` |
| 153 | `S01EC` | `0x01EC` | OneShot | `S01EC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:946` |
| 154 | `S01ED` | `0x01ED` | OneShot | `S01ED.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:28`, `crates/pin2k-core/src/swe1_audio.rs:942` |
| 155 | `S01EE` | `0x01EE` | OneShot | `S01EE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:947` |
| 156 | `S01EF` | `0x01EF` | OneShot | `S01EF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:929` |
| 157 | `S01F0` | `0x01F0` | OneShot | `S01F0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:937` |
| 158 | `S01F3` | `0x01F3` | OneShot | `S01F3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:960` |
| 159 | `S01F4` | `0x01F4` | OneShot | `S01F4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:374`, `crates/pin2k-core/src/swe1_audio.rs:883` |
| 160 | `S01F5` | `0x01F5` | OneShot | `S01F5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:167`, `crates/pin2k-core/src/swe1_audio.rs:324`, `crates/pin2k-core/src/swe1_audio.rs:875` |
| 161 | `S01F6` | `0x01F6` | OneShot | `S01F6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:167`, `crates/pin2k-core/src/swe1_audio.rs:171`, `crates/pin2k-core/src/swe1_audio.rs:324`, `crates/pin2k-core/src/swe1_audio.rs:874` |
| 162 | `S01F7` | `0x01F7` | OneShot | `S01F7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:167`, `crates/pin2k-core/src/swe1_audio.rs:171`, `crates/pin2k-core/src/swe1_audio.rs:877` |
| 163 | `S01F8` | `0x01F8` | OneShot | `S01F8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:310`, `crates/pin2k-core/src/swe1_audio.rs:873` |
| 164 | `S01F9` | `0x01F9` | OneShot | `S01F9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:882` |
| 165 | `S01FA` | `0x01FA` | OneShot | `S01FA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:878` |
| 166 | `S01FB` | `0x01FB` | OneShot | `S01FB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:880` |
| 167 | `S01FC` | `0x01FC` | OneShot | `S01FC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:881` |
| 168 | `S01FD` | `0x01FD` | OneShot | `S01FD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:67`, `crates/pin2k-core/src/swe1_audio.rs:876` |
| 169 | `S01FE` | `0x01FE` | OneShot | `S01FE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:879` |
| 170 | `S0203` | `0x0203` | OneShot | `S0203.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:171`, `crates/pin2k-core/src/swe1_audio.rs:915` |
| 171 | `S0204` | `0x0204` | OneShot | `S0204.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:59`, `crates/pin2k-core/src/swe1_audio.rs:311`, `crates/pin2k-core/src/swe1_audio.rs:910` |
| 172 | `S0205` | `0x0205` | OneShot | `S0205.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:323`, `crates/pin2k-core/src/swe1_audio.rs:907` |
| 173 | `S0206` | `0x0206` | OneShot | `S0206.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:324`, `crates/pin2k-core/src/swe1_audio.rs:905` |
| 174 | `S0207` | `0x0207` | OneShot | `S0207.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:919` |
| 175 | `S0208` | `0x0208` | OneShot | `S0208.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:171`, `crates/pin2k-core/src/swe1_audio.rs:914` |
| 176 | `S0209` | `0x0209` | OneShot | `S0209.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:918` |
| 177 | `S020A` | `0x020A` | OneShot | `S020A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:909` |
| 178 | `S020B` | `0x020B` | OneShot | `S020B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:375`, `crates/pin2k-core/src/swe1_audio.rs:904` |
| 179 | `S020C` | `0x020C` | OneShot | `S020C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:917` |
| 180 | `S020D` | `0x020D` | OneShot | `S020D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:916` |
| 181 | `S020E` | `0x020E` | OneShot | `S020E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:59`, `crates/pin2k-core/src/swe1_audio.rs:311`, `crates/pin2k-core/src/swe1_audio.rs:906` |
| 182 | `S020F` | `0x020F` | OneShot | `S020F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:799` |
| 183 | `S0210` | `0x0210` | OneShot | `S0210.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:800` |
| 184 | `S0212` | `0x0212` | OneShot | `S0212.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:274`, `crates/pin2k-core/src/swe1_audio.rs:598` |
| 185 | `S0213` | `0x0213` | OneShot | `S0213.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:140`, `crates/pin2k-core/src/swe1_audio.rs:144`, `crates/pin2k-core/src/swe1_audio.rs:600` |
| 186 | `S0214` | `0x0214` | OneShot | `S0214.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:140`, `crates/pin2k-core/src/swe1_audio.rs:144`, `crates/pin2k-core/src/swe1_audio.rs:274`, `crates/pin2k-core/src/swe1_audio.rs:602` |
| 187 | `S0216` | `0x0216` | OneShot | `S0216.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:604` |
| 188 | `S0217` | `0x0217` | OneShot | `S0217.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:601` |
| 189 | `S0218` | `0x0218` | OneShot | `S0218.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:599` |
| 190 | `S0219` | `0x0219` | OneShot | `S0219.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:274`, `crates/pin2k-core/src/swe1_audio.rs:603` |
| 191 | `S021A` | `0x021A` | OneShot | `S021A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:392`, `crates/pin2k-core/src/swe1_audio.rs:1057` |
| 192 | `S021B` | `0x021B` | OneShot | `S021B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:385`, `crates/pin2k-core/src/swe1_audio.rs:1072` |
| 193 | `S021C` | `0x021C` | OneShot | `S021C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:383`, `crates/pin2k-core/src/swe1_audio.rs:399`, `crates/pin2k-core/src/swe1_audio.rs:1079` |
| 194 | `S021D` | `0x021D` | OneShot | `S021D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:383`, `crates/pin2k-core/src/swe1_audio.rs:1062` |
| 195 | `S021E` | `0x021E` | OneShot | `S021E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:383`, `crates/pin2k-core/src/swe1_audio.rs:398`, `crates/pin2k-core/src/swe1_audio.rs:1065` |
| 196 | `S021F` | `0x021F` | OneShot | `S021F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:385`, `crates/pin2k-core/src/swe1_audio.rs:1061` |
| 197 | `S0220` | `0x0220` | OneShot | `S0220.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:385`, `crates/pin2k-core/src/swe1_audio.rs:390`, `crates/pin2k-core/src/swe1_audio.rs:397`, `crates/pin2k-core/src/swe1_audio.rs:1064` |
| 198 | `S0221` | `0x0221` | OneShot | `S0221.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:385`, `crates/pin2k-core/src/swe1_audio.rs:400`, `crates/pin2k-core/src/swe1_audio.rs:1068` |
| 199 | `S0222` | `0x0222` | OneShot | `S0222.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1060` |
| 200 | `S0223` | `0x0223` | OneShot | `S0223.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:402`, `crates/pin2k-core/src/swe1_audio.rs:1070` |
| 201 | `S0224` | `0x0224` | OneShot | `S0224.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1067` |
| 202 | `S0225` | `0x0225` | OneShot | `S0225.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:401`, `crates/pin2k-core/src/swe1_audio.rs:1058` |
| 203 | `S0226` | `0x0226` | OneShot | `S0226.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:404`, `crates/pin2k-core/src/swe1_audio.rs:1095` |
| 204 | `S0227` | `0x0227` | OneShot | `S0227.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:404`, `crates/pin2k-core/src/swe1_audio.rs:1101` |
| 205 | `S0228` | `0x0228` | OneShot | `S0228.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:404`, `crates/pin2k-core/src/swe1_audio.rs:1094` |
| 206 | `S0229` | `0x0229` | OneShot | `S0229.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1097` |
| 207 | `S022A` | `0x022A` | OneShot | `S022A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:374`, `crates/pin2k-core/src/swe1_audio.rs:1098` |
| 208 | `S022B` | `0x022B` | OneShot | `S022B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1102` |
| 209 | `S022C` | `0x022C` | OneShot | `S022C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1099` |
| 210 | `S022D` | `0x022D` | OneShot | `S022D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1103` |
| 211 | `S022E` | `0x022E` | OneShot | `S022E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:404`, `crates/pin2k-core/src/swe1_audio.rs:1100` |
| 212 | `S022F` | `0x022F` | OneShot | `S022F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1096` |
| 213 | `S0230` | `0x0230` | OneShot | `S0230.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:817` |
| 214 | `S0231` | `0x0231` | OneShot | `S0231.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:815` |
| 215 | `S0232` | `0x0232` | OneShot | `S0232.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:816` |
| 216 | `S0233` | `0x0233` | OneShot | `S0233.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:825` |
| 217 | `S0234` | `0x0234` | OneShot | `S0234.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:826` |
| 218 | `S0235` | `0x0235` | OneShot | `S0235.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:884` |
| 219 | `S0236` | `0x0236` | OneShot | `S0236.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:821` |
| 220 | `S0237` | `0x0237` | OneShot | `S0237.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:822` |
| 221 | `S0238` | `0x0238` | OneShot | `S0238.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:823` |
| 222 | `S023A` | `0x023A` | OneShot | `S023A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:56`, `crates/pin2k-core/src/swe1_audio.rs:329`, `crates/pin2k-core/src/swe1_audio.rs:554` |
| 223 | `S023B` | `0x023B` | OneShot | `S023B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:53`, `crates/pin2k-core/src/swe1_audio.rs:553` |
| 224 | `S023C` | `0x023C` | OneShot | `S023C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:53`, `crates/pin2k-core/src/swe1_audio.rs:560` |
| 225 | `S023D` | `0x023D` | OneShot | `S023D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:53`, `crates/pin2k-core/src/swe1_audio.rs:563` |
| 226 | `S023E` | `0x023E` | OneShot | `S023E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:562` |
| 227 | `S023F` | `0x023F` | OneShot | `S023F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:558` |
| 228 | `S0240` | `0x0240` | OneShot | `S0240.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:53`, `crates/pin2k-core/src/swe1_audio.rs:561` |
| 229 | `S0241` | `0x0241` | OneShot | `S0241.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:53`, `crates/pin2k-core/src/swe1_audio.rs:374`, `crates/pin2k-core/src/swe1_audio.rs:559` |
| 230 | `S0242` | `0x0242` | OneShot | `S0242.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:557` |
| 231 | `S0243` | `0x0243` | OneShot | `S0243.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:55`, `crates/pin2k-core/src/swe1_audio.rs:552` |
| 232 | `S0244` | `0x0244` | OneShot | `S0244.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:54`, `crates/pin2k-core/src/swe1_audio.rs:56`, `crates/pin2k-core/src/swe1_audio.rs:555` |
| 233 | `S0245` | `0x0245` | OneShot | `S0245.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:53`, `crates/pin2k-core/src/swe1_audio.rs:556` |
| 234 | `S024E` | `0x024E` | OneShot | `S024E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:839` |
| 235 | `S024F` | `0x024F` | OneShot | `S024F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:139`, `crates/pin2k-core/src/swe1_audio.rs:847` |
| 236 | `S0250` | `0x0250` | OneShot | `S0250.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:840` |
| 237 | `S0251` | `0x0251` | OneShot | `S0251.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:844` |
| 238 | `S0252` | `0x0252` | OneShot | `S0252.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:842` |
| 239 | `S0253` | `0x0253` | OneShot | `S0253.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:841` |
| 240 | `S0254` | `0x0254` | OneShot | `S0254.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:845` |
| 241 | `S0255` | `0x0255` | OneShot | `S0255.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:843` |
| 242 | `S0256` | `0x0256` | OneShot | `S0256.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:846` |
| 243 | `S0259` | `0x0259` | OneShot | `S0259.ogg` | Oui | `crates/pin2k-core/src/admin.rs:149`, `crates/pin2k-core/src/swe1_audio.rs:135`, `crates/pin2k-core/src/swe1_audio.rs:621`, `crates/pin2k-core/src/swe1_tables.rs:180`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:583` |
| 244 | `S025A` | `0x025A` | OneShot | `S025A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:29`, `crates/pin2k-core/src/swe1_audio.rs:101`, `crates/pin2k-core/src/swe1_audio.rs:622` |
| 245 | `S025B` | `0x025B` | OneShot | `S025B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:41`, `crates/pin2k-core/src/swe1_audio.rs:50`, `crates/pin2k-core/src/swe1_audio.rs:96`, `crates/pin2k-core/src/swe1_audio.rs:102`, `crates/pin2k-core/src/swe1_audio.rs:256` (+1) |
| 246 | `S025C` | `0x025C` | OneShot | `S025C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:624` |
| 247 | `S025D` | `0x025D` | OneShot | `S025D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:57`, `crates/pin2k-core/src/swe1_audio.rs:108`, `crates/pin2k-core/src/swe1_audio.rs:128`, `crates/pin2k-core/src/swe1_audio.rs:208`, `crates/pin2k-core/src/swe1_audio.rs:322` (+1) |
| 248 | `S025E` | `0x025E` | OneShot | `S025E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:626` |
| 249 | `S025F` | `0x025F` | OneShot | `S025F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:627` |
| 250 | `S0260` | `0x0260` | OneShot | `S0260.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:628` |
| 251 | `S0261` | `0x0261` | OneShot | `S0261.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:37`, `crates/pin2k-core/src/swe1_audio.rs:60`, `crates/pin2k-core/src/swe1_audio.rs:82`, `crates/pin2k-core/src/swe1_audio.rs:111` (+5) |
| 252 | `S0262` | `0x0262` | OneShot | `S0262.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:40`, `crates/pin2k-core/src/swe1_audio.rs:62`, `crates/pin2k-core/src/swe1_audio.rs:95`, `crates/pin2k-core/src/swe1_audio.rs:136` (+2) |
| 253 | `S0263` | `0x0263` | OneShot | `S0263.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:110`, `crates/pin2k-core/src/swe1_audio.rs:124`, `crates/pin2k-core/src/swe1_audio.rs:1032` |
| 254 | `S0264` | `0x0264` | OneShot | `S0264.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:83`, `crates/pin2k-core/src/swe1_audio.rs:605` |
| 255 | `S0265` | `0x0265` | OneShot | `S0265.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:84`, `crates/pin2k-core/src/swe1_audio.rs:1051` |
| 256 | `S02BC` | `0x02BC` | OneShot | `S02BC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:44`, `crates/pin2k-core/src/swe1_audio.rs:45`, `crates/pin2k-core/src/swe1_audio.rs:63`, `crates/pin2k-core/src/swe1_audio.rs:64` (+5) |
| 257 | `S02BD` | `0x02BD` | OneShot | `S02BD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:307`, `crates/pin2k-core/src/swe1_audio.rs:639` |
| 258 | `S02BE` | `0x02BE` | OneShot | `S02BE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:33`, `crates/pin2k-core/src/swe1_audio.rs:34`, `crates/pin2k-core/src/swe1_audio.rs:355`, `crates/pin2k-core/src/swe1_audio.rs:356` (+1) |
| 259 | `S02BF` | `0x02BF` | OneShot | `S02BF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:68`, `crates/pin2k-core/src/swe1_audio.rs:117`, `crates/pin2k-core/src/swe1_audio.rs:313`, `crates/pin2k-core/src/swe1_audio.rs:320`, `crates/pin2k-core/src/swe1_audio.rs:321` (+1) |
| 260 | `S02C0` | `0x02C0` | OneShot | `S02C0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:347`, `crates/pin2k-core/src/swe1_audio.rs:886` |
| 261 | `S02C1` | `0x02C1` | OneShot | `S02C1.ogg` | Oui | `crates/pin2k-core/src/admin.rs:147`, `crates/pin2k-core/src/swe1_audio.rs:88`, `crates/pin2k-core/src/swe1_audio.rs:103`, `crates/pin2k-core/src/swe1_audio.rs:104`, `crates/pin2k-core/src/swe1_audio.rs:105` (+3) |
| 262 | `S02C2` | `0x02C2` | OneShot | `S02C2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:106`, `crates/pin2k-core/src/swe1_audio.rs:107`, `crates/pin2k-core/src/swe1_audio.rs:122`, `crates/pin2k-core/src/swe1_audio.rs:123` (+2) |
| 263 | `S02C3` | `0x02C3` | OneShot | `S02C3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:125`, `crates/pin2k-core/src/swe1_audio.rs:126`, `crates/pin2k-core/src/swe1_audio.rs:149`, `crates/pin2k-core/src/swe1_audio.rs:150`, `crates/pin2k-core/src/swe1_audio.rs:157` (+2) |
| 264 | `S02C4` | `0x02C4` | OneShot | `S02C4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:203`, `crates/pin2k-core/src/swe1_audio.rs:217`, `crates/pin2k-core/src/swe1_audio.rs:798`, `crates/pin2k-core/src/swe1_tables.rs:187` |
| 265 | `S02C5` | `0x02C5` | OneShot | `S02C5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:797` |
| 266 | `S02C6` | `0x02C6` | OneShot | `S02C6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:207`, `crates/pin2k-core/src/swe1_audio.rs:213`, `crates/pin2k-core/src/swe1_audio.rs:309`, `crates/pin2k-core/src/swe1_audio.rs:795` |
| 267 | `S02C7` | `0x02C7` | OneShot | `S02C7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:206`, `crates/pin2k-core/src/swe1_audio.rs:218`, `crates/pin2k-core/src/swe1_audio.rs:796` |
| 268 | `S02C8` | `0x02C8` | OneShot | `S02C8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1028`, `crates/pin2k-core/src/swe1_tables.rs:186`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:596` |
| 269 | `S02C9` | `0x02C9` | OneShot | `S02C9.ogg` | Oui | `crates/pin2k-core/src/admin.rs:148`, `crates/pin2k-core/src/swe1_audio.rs:212`, `crates/pin2k-core/src/swe1_audio.rs:805`, `crates/pin2k-core/src/swe1_tables.rs:179` |
| 270 | `S02CA` | `0x02CA` | OneShot | `S02CA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:205`, `crates/pin2k-core/src/swe1_audio.rs:212`, `crates/pin2k-core/src/swe1_audio.rs:806` |
| 271 | `S02CB` | `0x02CB` | OneShot | `S02CB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:212`, `crates/pin2k-core/src/swe1_audio.rs:807` |
| 272 | `S02CC` | `0x02CC` | OneShot | `S02CC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:212`, `crates/pin2k-core/src/swe1_audio.rs:808` |
| 273 | `S02CD` | `0x02CD` | OneShot | `S02CD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:212`, `crates/pin2k-core/src/swe1_audio.rs:809` |
| 274 | `S02CE` | `0x02CE` | OneShot | `S02CE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:212`, `crates/pin2k-core/src/swe1_audio.rs:810` |
| 275 | `S02CF` | `0x02CF` | OneShot | `S02CF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:212`, `crates/pin2k-core/src/swe1_audio.rs:811` |
| 276 | `S02D0` | `0x02D0` | OneShot | `S02D0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:212`, `crates/pin2k-core/src/swe1_audio.rs:812` |
| 277 | `S0320` | `0x0320` | OneShot | `S0320.ogg` | Oui | `crates/pin2k-core/src/machine/attract_render.rs:18`, `crates/pin2k-core/src/swe1_audio.rs:148`, `crates/pin2k-core/src/swe1_audio.rs:151`, `crates/pin2k-core/src/swe1_audio.rs:638` |
| 278 | `S0321` | `0x0321` | OneShot | `S0321.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:43`, `crates/pin2k-core/src/swe1_audio.rs:547` |
| 279 | `S0322` | `0x0322` | OneShot | `S0322.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:133`, `crates/pin2k-core/src/swe1_audio.rs:259`, `crates/pin2k-core/src/swe1_audio.rs:268`, `crates/pin2k-core/src/swe1_audio.rs:319`, `crates/pin2k-core/src/swe1_audio.rs:830` |
| 280 | `S0323` | `0x0323` | OneShot | `S0323.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:109`, `crates/pin2k-core/src/swe1_audio.rs:269`, `crates/pin2k-core/src/swe1_audio.rs:832` |
| 281 | `S0324` | `0x0324` | OneShot | `S0324.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:303`, `crates/pin2k-core/src/swe1_audio.rs:370`, `crates/pin2k-core/src/swe1_audio.rs:889` |
| 282 | `S0325` | `0x0325` | OneShot | `S0325.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:301`, `crates/pin2k-core/src/swe1_audio.rs:304`, `crates/pin2k-core/src/swe1_audio.rs:305`, `crates/pin2k-core/src/swe1_audio.rs:306`, `crates/pin2k-core/src/swe1_audio.rs:371` (+1) |
| 283 | `S0329` | `0x0329` | OneShot | `S0329.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:300`, `crates/pin2k-core/src/swe1_audio.rs:301`, `crates/pin2k-core/src/swe1_audio.rs:893` |
| 284 | `S032A` | `0x032A` | OneShot | `S032A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:300`, `crates/pin2k-core/src/swe1_audio.rs:301`, `crates/pin2k-core/src/swe1_audio.rs:894` |
| 285 | `S032B` | `0x032B` | OneShot | `S032B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:911` |
| 286 | `S032D` | `0x032D` | OneShot | `S032D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:913` |
| 287 | `S032F` | `0x032F` | OneShot | `S032F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:24`, `crates/pin2k-core/src/swe1_audio.rs:912` |
| 288 | `S0330` | `0x0330` | OneShot | `S0330.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:242`, `crates/pin2k-core/src/swe1_audio.rs:291`, `crates/pin2k-core/src/swe1_audio.rs:890` |
| 289 | `S0331` | `0x0331` | OneShot | `S0331.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:291`, `crates/pin2k-core/src/swe1_audio.rs:891` |
| 290 | `S0332` | `0x0332` | OneShot | `S0332.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:170`, `crates/pin2k-core/src/swe1_audio.rs:283`, `crates/pin2k-core/src/swe1_audio.rs:1035` |
| 291 | `S0333` | `0x0333` | OneShot | `S0333.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:283`, `crates/pin2k-core/src/swe1_audio.rs:1036` |
| 292 | `S0334` | `0x0334` | OneShot | `S0334.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:35`, `crates/pin2k-core/src/swe1_audio.rs:357`, `crates/pin2k-core/src/swe1_audio.rs:1038` |
| 293 | `S0335` | `0x0335` | OneShot | `S0335.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:358`, `crates/pin2k-core/src/swe1_audio.rs:1039` |
| 294 | `S0336` | `0x0336` | OneShot | `S0336.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:70`, `crates/pin2k-core/src/swe1_audio.rs:1083` |
| 295 | `S0337` | `0x0337` | OneShot | `S0337.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:65`, `crates/pin2k-core/src/swe1_audio.rs:66`, `crates/pin2k-core/src/swe1_audio.rs:73`, `crates/pin2k-core/src/swe1_audio.rs:1082` |
| 296 | `S0339` | `0x0339` | OneShot | `S0339.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:308`, `crates/pin2k-core/src/swe1_audio.rs:315`, `crates/pin2k-core/src/swe1_audio.rs:341`, `crates/pin2k-core/src/swe1_audio.rs:903` |
| 297 | `S033A` | `0x033A` | OneShot | `S033A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:164`, `crates/pin2k-core/src/swe1_audio.rs:238`, `crates/pin2k-core/src/swe1_audio.rs:997` |
| 298 | `S033B` | `0x033B` | OneShot | `S033B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:165`, `crates/pin2k-core/src/swe1_audio.rs:239`, `crates/pin2k-core/src/swe1_audio.rs:998` |
| 299 | `S033E` | `0x033E` | OneShot | `S033E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:361`, `crates/pin2k-core/src/swe1_audio.rs:367`, `crates/pin2k-core/src/swe1_audio.rs:870` |
| 300 | `S033F` | `0x033F` | OneShot | `S033F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:363`, `crates/pin2k-core/src/swe1_audio.rs:871` |
| 301 | `S0340` | `0x0340` | OneShot | `S0340.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:902` |
| 302 | `S0341` | `0x0341` | OneShot | `S0341.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:241`, `crates/pin2k-core/src/swe1_audio.rs:892` |
| 303 | `S0342` | `0x0342` | OneShot | `S0342.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:240`, `crates/pin2k-core/src/swe1_audio.rs:888` |
| 304 | `S0352` | `0x0352` | OneShot | `S0352.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:384`, `crates/pin2k-core/src/swe1_audio.rs:828` |
| 305 | `S0353` | `0x0353` | OneShot | `S0353.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1034` |
| 306 | `S0354` | `0x0354` | OneShot | `S0354.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:187`, `crates/pin2k-core/src/swe1_audio.rs:199`, `crates/pin2k-core/src/swe1_audio.rs:234`, `crates/pin2k-core/src/swe1_audio.rs:245`, `crates/pin2k-core/src/swe1_audio.rs:749` |
| 307 | `S0355` | `0x0355` | OneShot | `S0355.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:250`, `crates/pin2k-core/src/swe1_audio.rs:328`, `crates/pin2k-core/src/swe1_audio.rs:667` |
| 308 | `S0356` | `0x0356` | OneShot | `S0356.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:377`, `crates/pin2k-core/src/swe1_audio.rs:386`, `crates/pin2k-core/src/swe1_audio.rs:387`, `crates/pin2k-core/src/swe1_audio.rs:388`, `crates/pin2k-core/src/swe1_audio.rs:672` |
| 309 | `S0357` | `0x0357` | OneShot | `S0357.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:394`, `crates/pin2k-core/src/swe1_audio.rs:1080` |
| 310 | `S0358` | `0x0358` | OneShot | `S0358.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:284`, `crates/pin2k-core/src/swe1_audio.rs:1002` |
| 311 | `S0359` | `0x0359` | OneShot | `S0359.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:290`, `crates/pin2k-core/src/swe1_audio.rs:1001` |
| 312 | `S035A` | `0x035A` | OneShot | `S035A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:287`, `crates/pin2k-core/src/swe1_audio.rs:289`, `crates/pin2k-core/src/swe1_audio.rs:479` |
| 313 | `S035B` | `0x035B` | OneShot | `S035B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:23`, `crates/pin2k-core/src/swe1_audio.rs:285`, `crates/pin2k-core/src/swe1_audio.rs:286`, `crates/pin2k-core/src/swe1_audio.rs:288`, `crates/pin2k-core/src/swe1_audio.rs:480` |
| 314 | `S035C` | `0x035C` | OneShot | `S035C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:94`, `crates/pin2k-core/src/swe1_audio.rs:257`, `crates/pin2k-core/src/swe1_audio.rs:608` |
| 315 | `S035D` | `0x035D` | OneShot | `S035D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:87`, `crates/pin2k-core/src/swe1_audio.rs:89`, `crates/pin2k-core/src/swe1_audio.rs:90`, `crates/pin2k-core/src/swe1_audio.rs:258`, `crates/pin2k-core/src/swe1_audio.rs:609` |
| 316 | `S035E` | `0x035E` | OneShot | `S035E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:290`, `crates/pin2k-core/src/swe1_audio.rs:299`, `crates/pin2k-core/src/swe1_audio.rs:895` |
| 317 | `S035F` | `0x035F` | OneShot | `S035F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:290`, `crates/pin2k-core/src/swe1_audio.rs:896` |
| 318 | `S0360` | `0x0360` | OneShot | `S0360.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:173`, `crates/pin2k-core/src/swe1_audio.rs:974` |
| 319 | `S0361` | `0x0361` | OneShot | `S0361.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:48`, `crates/pin2k-core/src/swe1_audio.rs:573` |
| 320 | `S0362` | `0x0362` | OneShot | `S0362.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:76`, `crates/pin2k-core/src/swe1_audio.rs:243`, `crates/pin2k-core/src/swe1_audio.rs:872` |
| 321 | `S0363` | `0x0363` | OneShot | `S0363.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:362`, `crates/pin2k-core/src/swe1_audio.rs:869` |
| 322 | `S0364` | `0x0364` | OneShot | `S0364.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:302`, `crates/pin2k-core/src/swe1_audio.rs:898` |
| 323 | `S0365` | `0x0365` | OneShot | `S0365.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:813`, `crates/pin2k-core/src/swe1_tables.rs:201` |
| 324 | `S0366` | `0x0366` | OneShot | `S0366.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1050` |
| 325 | `S0367` | `0x0367` | OneShot | `S0367.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:327`, `crates/pin2k-core/src/swe1_audio.rs:970` |
| 326 | `S0368` | `0x0368` | OneShot | `S0368.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1000` |
| 327 | `S0369` | `0x0369` | OneShot | `S0369.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:146`, `crates/pin2k-core/src/swe1_audio.rs:571` |
| 328 | `S036A` | `0x036A` | OneShot | `S036A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:551` |
| 329 | `S036B` | `0x036B` | OneShot | `S036B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:612` |
| 330 | `S036C` | `0x036C` | OneShot | `S036C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:613` |
| 331 | `S036D` | `0x036D` | OneShot | `S036D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:614` |
| 332 | `S036E` | `0x036E` | OneShot | `S036E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:615` |
| 333 | `S036F` | `0x036F` | OneShot | `S036F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:253`, `crates/pin2k-core/src/swe1_audio.rs:666` |
| 334 | `S0384` | `0x0384` | OneShot | `S0384.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:263`, `crates/pin2k-core/src/swe1_audio.rs:360`, `crates/pin2k-core/src/swe1_audio.rs:548` |
| 335 | `S0385` | `0x0385` | OneShot | `S0385.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:265`, `crates/pin2k-core/src/swe1_audio.rs:659` |
| 336 | `S0386` | `0x0386` | OneShot | `S0386.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:266`, `crates/pin2k-core/src/swe1_audio.rs:972` |
| 337 | `S0388` | `0x0388` | OneShot | `S0388.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:660` |
| 338 | `S0389` | `0x0389` | OneShot | `S0389.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:661` |
| 339 | `S038A` | `0x038A` | OneShot | `S038A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:803`, `crates/pin2k-core/src/swe1_tables.rs:182`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:645`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:656`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:667` |
| 340 | `S038B` | `0x038B` | OneShot | `S038B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:981`, `crates/pin2k-core/src/swe1_tables.rs:181`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:677` |
| 341 | `S038C` | `0x038C` | OneShot | `S038C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1042` |
| 342 | `S038F` | `0x038F` | OneShot | `S038F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:273`, `crates/pin2k-core/src/swe1_audio.rs:1043` |
| 343 | `S0390` | `0x0390` | OneShot | `S0390.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:152`, `crates/pin2k-core/src/swe1_audio.rs:153`, `crates/pin2k-core/src/swe1_audio.rs:154`, `crates/pin2k-core/src/swe1_audio.rs:255`, `crates/pin2k-core/src/swe1_audio.rs:353` (+1) |
| 344 | `S0392` | `0x0392` | OneShot | `S0392.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:271`, `crates/pin2k-core/src/swe1_audio.rs:1054` |
| 345 | `S0393` | `0x0393` | OneShot | `S0393.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:194`, `crates/pin2k-core/src/swe1_audio.rs:275`, `crates/pin2k-core/src/swe1_audio.rs:382`, `crates/pin2k-core/src/swe1_audio.rs:1029` |
| 346 | `S0396` | `0x0396` | OneShot | `S0396.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:887` |
| 347 | `S0398` | `0x0398` | OneShot | `S0398.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:224`, `crates/pin2k-core/src/swe1_audio.rs:1005` |
| 348 | `S0399` | `0x0399` | OneShot | `S0399.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1006` |
| 349 | `S039A` | `0x039A` | OneShot | `S039A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1007` |
| 350 | `S039B` | `0x039B` | OneShot | `S039B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1008` |
| 351 | `S039C` | `0x039C` | OneShot | `S039C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1009` |
| 352 | `S039D` | `0x039D` | OneShot | `S039D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1010` |
| 353 | `S039E` | `0x039E` | OneShot | `S039E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:98`, `crates/pin2k-core/src/swe1_audio.rs:138`, `crates/pin2k-core/src/swe1_audio.rs:1011` |
| 354 | `S03A0` | `0x03A0` | OneShot | `S03A0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:32`, `crates/pin2k-core/src/swe1_audio.rs:86`, `crates/pin2k-core/src/swe1_audio.rs:1012` |
| 355 | `S03A4` | `0x03A4` | OneShot | `S03A4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:629` |
| 356 | `S03A5` | `0x03A5` | OneShot | `S03A5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:690` |
| 357 | `S03A6` | `0x03A6` | OneShot | `S03A6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:202`, `crates/pin2k-core/src/swe1_audio.rs:226`, `crates/pin2k-core/src/swe1_audio.rs:339`, `crates/pin2k-core/src/swe1_audio.rs:1013` |
| 358 | `S03A7` | `0x03A7` | OneShot | `S03A7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:342`, `crates/pin2k-core/src/swe1_audio.rs:343`, `crates/pin2k-core/src/swe1_audio.rs:1014` |
| 359 | `S03AE` | `0x03AE` | OneShot | `S03AE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:225`, `crates/pin2k-core/src/swe1_audio.rs:338`, `crates/pin2k-core/src/swe1_audio.rs:1015` |
| 360 | `S03AF` | `0x03AF` | OneShot | `S03AF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:195`, `crates/pin2k-core/src/swe1_audio.rs:225`, `crates/pin2k-core/src/swe1_audio.rs:338`, `crates/pin2k-core/src/swe1_audio.rs:1016` |
| 361 | `S03B0` | `0x03B0` | OneShot | `S03B0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1017` |
| 362 | `S03B1` | `0x03B1` | OneShot | `S03B1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1018` |
| 363 | `S03B2` | `0x03B2` | OneShot | `S03B2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1019` |
| 364 | `S03B3` | `0x03B3` | OneShot | `S03B3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1020` |
| 365 | `S03B4` | `0x03B4` | OneShot | `S03B4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:114`, `crates/pin2k-core/src/swe1_audio.rs:1021` |
| 366 | `S03B7` | `0x03B7` | OneShot | `S03B7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1022` |
| 367 | `S03B8` | `0x03B8` | OneShot | `S03B8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:272`, `crates/pin2k-core/src/swe1_audio.rs:1055` |
| 368 | `S03B9` | `0x03B9` | OneShot | `S03B9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:280`, `crates/pin2k-core/src/swe1_audio.rs:1041`, `crates/pin2k-core/src/swe1_tables.rs:177` |
| 369 | `S03BB` | `0x03BB` | OneShot | `S03BB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:246`, `crates/pin2k-core/src/swe1_audio.rs:691` |
| 370 | `S03BC` | `0x03BC` | OneShot | `S03BC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:262`, `crates/pin2k-core/src/swe1_audio.rs:1047` |
| 371 | `S03BD` | `0x03BD` | OneShot | `S03BD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:254`, `crates/pin2k-core/src/swe1_audio.rs:1048` |
| 372 | `S03BE` | `0x03BE` | OneShot | `S03BE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:344`, `crates/pin2k-core/src/swe1_audio.rs:1049` |
| 373 | `S03BF` | `0x03BF` | OneShot | `S03BF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:78`, `crates/pin2k-core/src/swe1_audio.rs:921` |
| 374 | `S03C0` | `0x03C0` | OneShot | `S03C0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:79`, `crates/pin2k-core/src/swe1_audio.rs:922` |
| 375 | `S03C1` | `0x03C1` | OneShot | `S03C1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:80`, `crates/pin2k-core/src/swe1_audio.rs:923` |
| 376 | `S03C2` | `0x03C2` | OneShot | `S03C2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:127`, `crates/pin2k-core/src/swe1_audio.rs:373`, `crates/pin2k-core/src/swe1_audio.rs:924` |
| 377 | `S03C3` | `0x03C3` | OneShot | `S03C3.ogg` | Oui | `crates/pin2k-core/src/machine/vspinner.rs:5`, `crates/pin2k-core/src/swe1_audio.rs:381`, `crates/pin2k-core/src/swe1_audio.rs:1046`, `crates/pin2k-core/src/swe1_tables.rs:193` |
| 378 | `S03C9` | `0x03C9` | OneShot | `S03C9.ogg` | Oui | `crates/pin2k-core/src/machine/vspinner.rs:5`, `crates/pin2k-core/src/swe1_audio.rs:644`, `crates/pin2k-core/src/swe1_tables.rs:194` |
| 379 | `S03CA` | `0x03CA` | OneShot | `S03CA.ogg` | Oui | `crates/pin2k-core/src/machine/vspinner.rs:5`, `crates/pin2k-core/src/swe1_audio.rs:180`, `crates/pin2k-core/src/swe1_audio.rs:662`, `crates/pin2k-core/src/swe1_tables.rs:195` |
| 380 | `S03CB` | `0x03CB` | OneShot | `S03CB.ogg` | Oui | `crates/pin2k-core/src/machine/vspinner.rs:5`, `crates/pin2k-core/src/swe1_audio.rs:181`, `crates/pin2k-core/src/swe1_audio.rs:663`, `crates/pin2k-core/src/swe1_tables.rs:196` |
| 381 | `S03CC` | `0x03CC` | OneShot | `S03CC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:182`, `crates/pin2k-core/src/swe1_audio.rs:664` |
| 382 | `S03CD` | `0x03CD` | OneShot | `S03CD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:183`, `crates/pin2k-core/src/swe1_audio.rs:665` |
| 383 | `S03CE` | `0x03CE` | OneShot | `S03CE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:175`, `crates/pin2k-core/src/swe1_audio.rs:818` |
| 384 | `S03CF` | `0x03CF` | OneShot | `S03CF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:222`, `crates/pin2k-core/src/swe1_audio.rs:574` |
| 385 | `S03D0` | `0x03D0` | OneShot | `S03D0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:189`, `crates/pin2k-core/src/swe1_audio.rs:389`, `crates/pin2k-core/src/swe1_audio.rs:575` |
| 386 | `S03D1` | `0x03D1` | OneShot | `S03D1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:170`, `crates/pin2k-core/src/swe1_audio.rs:283`, `crates/pin2k-core/src/swe1_audio.rs:899` |
| 387 | `S03D2` | `0x03D2` | OneShot | `S03D2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:30`, `crates/pin2k-core/src/swe1_audio.rs:121`, `crates/pin2k-core/src/swe1_audio.rs:610` |
| 388 | `S03D3` | `0x03D3` | OneShot | `S03D3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:802` |
| 389 | `S03D4` | `0x03D4` | OneShot | `S03D4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1052`, `crates/pin2k-core/src/swe1_audio.rs:1092` |
| 390 | `S03D5` | `0x03D5` | OneShot | `S03D5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:606`, `crates/pin2k-core/src/swe1_audio.rs:1086` |
| 391 | `S03D6` | `0x03D6` | OneShot | `S03D6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:619`, `crates/pin2k-core/src/swe1_audio.rs:1087` |
| 392 | `S03D7` | `0x03D7` | OneShot | `S03D7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:620`, `crates/pin2k-core/src/swe1_audio.rs:1088` |
| 393 | `S03D8` | `0x03D8` | OneShot | `S03D8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1053`, `crates/pin2k-core/src/swe1_audio.rs:1093` |
| 394 | `S03D9` | `0x03D9` | OneShot | `S03D9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1026`, `crates/pin2k-core/src/swe1_audio.rs:1090` |
| 395 | `S03DA` | `0x03DA` | OneShot | `S03DA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1027`, `crates/pin2k-core/src/swe1_audio.rs:1091` |
| 396 | `S03DB` | `0x03DB` | OneShot | `S03DB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:565`, `crates/pin2k-core/src/swe1_audio.rs:1084` |
| 397 | `S03DC` | `0x03DC` | OneShot | `S03DC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:572`, `crates/pin2k-core/src/swe1_audio.rs:1085` |
| 398 | `S03DD` | `0x03DD` | OneShot | `S03DD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:999`, `crates/pin2k-core/src/swe1_audio.rs:1089` |
| 399 | `S03DE` | `0x03DE` | OneShot | `S03DE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:801`, `crates/pin2k-core/src/swe1_audio.rs:996` |
| 400 | `S044C` | `0x044C` | OneShot | `S044C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:656` |
| 401 | `S044D` | `0x044D` | OneShot | `S044D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:652`, `crates/pin2k-core/src/swe1_tables.rs:190` |
| 402 | `S044E` | `0x044E` | OneShot | `S044E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:648` |
| 403 | `S044F` | `0x044F` | OneShot | `S044F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:651` |
| 404 | `S0450` | `0x0450` | OneShot | `S0450.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:647` |
| 405 | `S0451` | `0x0451` | OneShot | `S0451.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:646` |
| 406 | `S0452` | `0x0452` | OneShot | `S0452.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:653` |
| 407 | `S0453` | `0x0453` | OneShot | `S0453.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:655` |
| 408 | `S0454` | `0x0454` | OneShot | `S0454.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:649` |
| 409 | `S0455` | `0x0455` | OneShot | `S0455.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:654` |
| 410 | `S0456` | `0x0456` | OneShot | `S0456.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:650` |
| 411 | `S04B0` | `0x04B0` | OneShot | `S04B0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:293`, `crates/pin2k-core/src/swe1_audio.rs:993` |
| 412 | `S04B1` | `0x04B1` | OneShot | `S04B1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:293`, `crates/pin2k-core/src/swe1_audio.rs:995` |
| 413 | `S04B2` | `0x04B2` | OneShot | `S04B2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:293`, `crates/pin2k-core/src/swe1_audio.rs:994` |
| 414 | `S04B3` | `0x04B3` | OneShot | `S04B3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:294`, `crates/pin2k-core/src/swe1_audio.rs:991` |
| 415 | `S04B4` | `0x04B4` | OneShot | `S04B4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:988` |
| 416 | `S04B5` | `0x04B5` | OneShot | `S04B5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:992` |
| 417 | `S04B6` | `0x04B6` | OneShot | `S04B6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:294`, `crates/pin2k-core/src/swe1_audio.rs:985` |
| 418 | `S04B7` | `0x04B7` | OneShot | `S04B7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:294`, `crates/pin2k-core/src/swe1_audio.rs:990` |
| 419 | `S04B8` | `0x04B8` | OneShot | `S04B8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:984` |
| 420 | `S04B9` | `0x04B9` | OneShot | `S04B9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:983` |
| 421 | `S04BA` | `0x04BA` | OneShot | `S04BA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:986` |
| 422 | `S04BB` | `0x04BB` | OneShot | `S04BB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:294`, `crates/pin2k-core/src/swe1_audio.rs:989` |
| 423 | `S04BC` | `0x04BC` | OneShot | `S04BC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:294`, `crates/pin2k-core/src/swe1_audio.rs:987` |
| 424 | `S04BD` | `0x04BD` | OneShot | `S04BD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:982` |
| 425 | `S04BE` | `0x04BE` | OneShot | `S04BE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:294`, `crates/pin2k-core/src/swe1_audio.rs:295`, `crates/pin2k-core/src/swe1_audio.rs:1045` |
| 426 | `S0514` | `0x0514` | OneShot | `S0514.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:391`, `crates/pin2k-core/src/swe1_audio.rs:1077` |
| 427 | `S0515` | `0x0515` | OneShot | `S0515.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1059` |
| 428 | `S0516` | `0x0516` | OneShot | `S0516.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1063` |
| 429 | `S0517` | `0x0517` | OneShot | `S0517.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:395`, `crates/pin2k-core/src/swe1_audio.rs:1069` |
| 430 | `S0518` | `0x0518` | OneShot | `S0518.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1071` |
| 431 | `S0519` | `0x0519` | OneShot | `S0519.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1073` |
| 432 | `S051A` | `0x051A` | OneShot | `S051A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1074` |
| 433 | `S051B` | `0x051B` | OneShot | `S051B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:403`, `crates/pin2k-core/src/swe1_audio.rs:1078` |
| 434 | `S051C` | `0x051C` | OneShot | `S051C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1056` |
| 435 | `S051D` | `0x051D` | OneShot | `S051D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:396`, `crates/pin2k-core/src/swe1_audio.rs:1075` |
| 436 | `S051E` | `0x051E` | OneShot | `S051E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:1076` |
| 437 | `S0578` | `0x0578` | OneShot | `S0578.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:168`, `crates/pin2k-core/src/swe1_audio.rs:375`, `crates/pin2k-core/src/swe1_audio.rs:750` |
| 438 | `S0579` | `0x0579` | OneShot | `S0579.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:751` |
| 439 | `S057A` | `0x057A` | OneShot | `S057A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:168`, `crates/pin2k-core/src/swe1_audio.rs:190`, `crates/pin2k-core/src/swe1_audio.rs:380`, `crates/pin2k-core/src/swe1_audio.rs:695` |
| 440 | `S05DC` | `0x05DC` | OneShot | `S05DC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:141`, `crates/pin2k-core/src/swe1_audio.rs:932` |
| 441 | `S05DD` | `0x05DD` | OneShot | `S05DD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:933` |
| 442 | `S05DE` | `0x05DE` | OneShot | `S05DE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:115`, `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:944` |
| 443 | `S05DF` | `0x05DF` | OneShot | `S05DF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:115`, `crates/pin2k-core/src/swe1_audio.rs:337`, `crates/pin2k-core/src/swe1_audio.rs:930` |
| 444 | `S05E0` | `0x05E0` | OneShot | `S05E0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:934` |
| 445 | `S05E1` | `0x05E1` | OneShot | `S05E1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:961` |
| 446 | `S05E2` | `0x05E2` | OneShot | `S05E2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:949` |
| 447 | `S05E3` | `0x05E3` | OneShot | `S05E3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:950` |
| 448 | `S05E4` | `0x05E4` | OneShot | `S05E4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:936` |
| 449 | `S05E5` | `0x05E5` | OneShot | `S05E5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:939` |
| 450 | `S05E6` | `0x05E6` | OneShot | `S05E6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:928` |
| 451 | `S05E7` | `0x05E7` | OneShot | `S05E7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:941` |
| 452 | `S05E8` | `0x05E8` | OneShot | `S05E8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:940` |
| 453 | `S05E9` | `0x05E9` | OneShot | `S05E9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:957` |
| 454 | `S05EA` | `0x05EA` | OneShot | `S05EA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:955` |
| 455 | `S05EB` | `0x05EB` | OneShot | `S05EB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:951` |
| 456 | `S05EC` | `0x05EC` | OneShot | `S05EC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:945` |
| 457 | `S05ED` | `0x05ED` | OneShot | `S05ED.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:925` |
| 458 | `S05EE` | `0x05EE` | OneShot | `S05EE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:931` |
| 459 | `S0640` | `0x0640` | OneShot | `S0640.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:860` |
| 460 | `S0641` | `0x0641` | OneShot | `S0641.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:859` |
| 461 | `S0642` | `0x0642` | OneShot | `S0642.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:850` |
| 462 | `S0643` | `0x0643` | OneShot | `S0643.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:854` |
| 463 | `S0644` | `0x0644` | OneShot | `S0644.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:861` |
| 464 | `S0645` | `0x0645` | OneShot | `S0645.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:867` |
| 465 | `S0646` | `0x0646` | OneShot | `S0646.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:848` |
| 466 | `S0647` | `0x0647` | OneShot | `S0647.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:857` |
| 467 | `S0648` | `0x0648` | OneShot | `S0648.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:853` |
| 468 | `S0649` | `0x0649` | OneShot | `S0649.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:865` |
| 469 | `S064A` | `0x064A` | OneShot | `S064A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:864` |
| 470 | `S06A4` | `0x06A4` | OneShot | `S06A4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:204`, `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:582` |
| 471 | `S06A5` | `0x06A5` | OneShot | `S06A5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:584` |
| 472 | `S06A6` | `0x06A6` | OneShot | `S06A6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:585` |
| 473 | `S06A7` | `0x06A7` | OneShot | `S06A7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:201`, `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:586` |
| 474 | `S06A8` | `0x06A8` | OneShot | `S06A8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:587` |
| 475 | `S06A9` | `0x06A9` | OneShot | `S06A9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:588` |
| 476 | `S06AA` | `0x06AA` | OneShot | `S06AA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:589` |
| 477 | `S06AB` | `0x06AB` | OneShot | `S06AB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:590` |
| 478 | `S06AC` | `0x06AC` | OneShot | `S06AC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:591` |
| 479 | `S06AD` | `0x06AD` | OneShot | `S06AD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:211`, `crates/pin2k-core/src/swe1_audio.rs:583` |
| 480 | `S07D0` | `0x07D0` | OneShot | `S07D0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:276`, `crates/pin2k-core/src/swe1_audio.rs:473` |
| 481 | `S07D1` | `0x07D1` | OneShot | `S07D1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:277`, `crates/pin2k-core/src/swe1_audio.rs:474` |
| 482 | `S07D2` | `0x07D2` | OneShot | `S07D2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:278`, `crates/pin2k-core/src/swe1_audio.rs:475` |
| 483 | `S07D3` | `0x07D3` | OneShot | `S07D3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:279`, `crates/pin2k-core/src/swe1_audio.rs:476` |
| 484 | `S07D9` | `0x07D9` | OneShot | `S07D9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:469` |
| 485 | `S07DA` | `0x07DA` | OneShot | `S07DA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:529` |
| 486 | `S07DB` | `0x07DB` | OneShot | `S07DB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:169`, `crates/pin2k-core/src/swe1_audio.rs:172`, `crates/pin2k-core/src/swe1_audio.rs:429` |
| 487 | `S07DC` | `0x07DC` | OneShot | `S07DC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:418` |
| 488 | `S07DD` | `0x07DD` | OneShot | `S07DD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:270`, `crates/pin2k-core/src/swe1_audio.rs:439` |
| 489 | `S07DE` | `0x07DE` | OneShot | `S07DE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:511` |
| 490 | `S07DF` | `0x07DF` | OneShot | `S07DF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:169`, `crates/pin2k-core/src/swe1_audio.rs:172`, `crates/pin2k-core/src/swe1_audio.rs:451` |
| 491 | `S07E1` | `0x07E1` | OneShot | `S07E1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:421` |
| 492 | `S07E2` | `0x07E2` | OneShot | `S07E2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:169`, `crates/pin2k-core/src/swe1_audio.rs:457` |
| 493 | `S07E3` | `0x07E3` | OneShot | `S07E3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:412` |
| 494 | `S07E4` | `0x07E4` | OneShot | `S07E4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:495` |
| 495 | `S07E5` | `0x07E5` | OneShot | `S07E5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:142`, `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:493` |
| 496 | `S07E6` | `0x07E6` | OneShot | `S07E6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:352`, `crates/pin2k-core/src/swe1_audio.rs:518` |
| 497 | `S07E7` | `0x07E7` | OneShot | `S07E7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:471` |
| 498 | `S07E8` | `0x07E8` | OneShot | `S07E8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:497` |
| 499 | `S07EA` | `0x07EA` | OneShot | `S07EA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:172`, `crates/pin2k-core/src/swe1_audio.rs:441` |
| 500 | `S07EB` | `0x07EB` | OneShot | `S07EB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:169`, `crates/pin2k-core/src/swe1_audio.rs:499` |
| 501 | `S07EC` | `0x07EC` | OneShot | `S07EC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:233`, `crates/pin2k-core/src/swe1_audio.rs:460` |
| 502 | `S07ED` | `0x07ED` | OneShot | `S07ED.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:223`, `crates/pin2k-core/src/swe1_audio.rs:510` |
| 503 | `S07EE` | `0x07EE` | OneShot | `S07EE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:456` |
| 504 | `S07EF` | `0x07EF` | OneShot | `S07EF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:450` |
| 505 | `S07F1` | `0x07F1` | OneShot | `S07F1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:169`, `crates/pin2k-core/src/swe1_audio.rs:172`, `crates/pin2k-core/src/swe1_audio.rs:431` |
| 506 | `S07F2` | `0x07F2` | OneShot | `S07F2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:482` |
| 507 | `S07F3` | `0x07F3` | OneShot | `S07F3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:487` |
| 508 | `S07F4` | `0x07F4` | OneShot | `S07F4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:423` |
| 509 | `S07F6` | `0x07F6` | OneShot | `S07F6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:488` |
| 510 | `S07F7` | `0x07F7` | OneShot | `S07F7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:489` |
| 511 | `S07F8` | `0x07F8` | OneShot | `S07F8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:142`, `crates/pin2k-core/src/swe1_audio.rs:427` |
| 512 | `S07F9` | `0x07F9` | OneShot | `S07F9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:532` |
| 513 | `S07FA` | `0x07FA` | OneShot | `S07FA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:422` |
| 514 | `S07FB` | `0x07FB` | OneShot | `S07FB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:36`, `crates/pin2k-core/src/swe1_audio.rs:69`, `crates/pin2k-core/src/swe1_audio.rs:91`, `crates/pin2k-core/src/swe1_audio.rs:134`, `crates/pin2k-core/src/swe1_audio.rs:145` (+3) |
| 515 | `S07FC` | `0x07FC` | OneShot | `S07FC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:142`, `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:352`, `crates/pin2k-core/src/swe1_audio.rs:426` |
| 516 | `S07FD` | `0x07FD` | OneShot | `S07FD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:142`, `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:512` |
| 517 | `S07FE` | `0x07FE` | OneShot | `S07FE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:119`, `crates/pin2k-core/src/swe1_audio.rs:145`, `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:523` |
| 518 | `S07FF` | `0x07FF` | OneShot | `S07FF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:519` |
| 519 | `S0800` | `0x0800` | OneShot | `S0800.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:296`, `crates/pin2k-core/src/swe1_audio.rs:508` |
| 520 | `S0801` | `0x0801` | OneShot | `S0801.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:142`, `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:417` |
| 521 | `S0802` | `0x0802` | OneShot | `S0802.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:413` |
| 522 | `S0803` | `0x0803` | OneShot | `S0803.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:142`, `crates/pin2k-core/src/swe1_audio.rs:145`, `crates/pin2k-core/src/swe1_audio.rs:248`, `crates/pin2k-core/src/swe1_audio.rs:533` |
| 523 | `S0804` | `0x0804` | OneShot | `S0804.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:142`, `crates/pin2k-core/src/swe1_audio.rs:248`, `crates/pin2k-core/src/swe1_audio.rs:415` |
| 524 | `S0805` | `0x0805` | OneShot | `S0805.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:36`, `crates/pin2k-core/src/swe1_audio.rs:69`, `crates/pin2k-core/src/swe1_audio.rs:91`, `crates/pin2k-core/src/swe1_audio.rs:134`, `crates/pin2k-core/src/swe1_audio.rs:248` (+2) |
| 525 | `S0807` | `0x0807` | OneShot | `S0807.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:500` |
| 526 | `S0808` | `0x0808` | OneShot | `S0808.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:521` |
| 527 | `S0809` | `0x0809` | OneShot | `S0809.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:531` |
| 528 | `S080A` | `0x080A` | OneShot | `S080A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:352`, `crates/pin2k-core/src/swe1_audio.rs:491` |
| 529 | `S080C` | `0x080C` | OneShot | `S080C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:352`, `crates/pin2k-core/src/swe1_audio.rs:536` |
| 530 | `S080D` | `0x080D` | OneShot | `S080D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:464` |
| 531 | `S080E` | `0x080E` | OneShot | `S080E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:424` |
| 532 | `S0810` | `0x0810` | OneShot | `S0810.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:411` |
| 533 | `S0811` | `0x0811` | OneShot | `S0811.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:496` |
| 534 | `S0815` | `0x0815` | OneShot | `S0815.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:516` |
| 535 | `S0816` | `0x0816` | OneShot | `S0816.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:77`, `crates/pin2k-core/src/swe1_audio.rs:428` |
| 536 | `S0817` | `0x0817` | OneShot | `S0817.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:77`, `crates/pin2k-core/src/swe1_audio.rs:445` |
| 537 | `S081A` | `0x081A` | OneShot | `S081A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:77`, `crates/pin2k-core/src/swe1_audio.rs:434` |
| 538 | `S081B` | `0x081B` | OneShot | `S081B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:74`, `crates/pin2k-core/src/swe1_audio.rs:528` |
| 539 | `S081C` | `0x081C` | OneShot | `S081C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:74`, `crates/pin2k-core/src/swe1_audio.rs:416` |
| 540 | `S0820` | `0x0820` | OneShot | `S0820.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:348`, `crates/pin2k-core/src/swe1_audio.rs:485` |
| 541 | `S0823` | `0x0823` | OneShot | `S0823.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:348`, `crates/pin2k-core/src/swe1_audio.rs:432` |
| 542 | `S0824` | `0x0824` | OneShot | `S0824.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:348`, `crates/pin2k-core/src/swe1_audio.rs:486` |
| 543 | `S0827` | `0x0827` | OneShot | `S0827.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:535` |
| 544 | `S0828` | `0x0828` | OneShot | `S0828.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:282`, `crates/pin2k-core/src/swe1_audio.rs:477` |
| 545 | `S0829` | `0x0829` | OneShot | `S0829.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:506` |
| 546 | `S082A` | `0x082A` | OneShot | `S082A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:282`, `crates/pin2k-core/src/swe1_audio.rs:478` |
| 547 | `S082C` | `0x082C` | OneShot | `S082C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:481` |
| 548 | `S082D` | `0x082D` | OneShot | `S082D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:509` |
| 549 | `S082E` | `0x082E` | OneShot | `S082E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:414` |
| 550 | `S082F` | `0x082F` | OneShot | `S082F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:419` |
| 551 | `S0830` | `0x0830` | OneShot | `S0830.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:437` |
| 552 | `S0831` | `0x0831` | OneShot | `S0831.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:436` |
| 553 | `S0832` | `0x0832` | OneShot | `S0832.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:145`, `crates/pin2k-core/src/swe1_audio.rs:169`, `crates/pin2k-core/src/swe1_audio.rs:448` |
| 554 | `S0833` | `0x0833` | OneShot | `S0833.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:520` |
| 555 | `S0834` | `0x0834` | OneShot | `S0834.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:494` |
| 556 | `S0836` | `0x0836` | OneShot | `S0836.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:248`, `crates/pin2k-core/src/swe1_audio.rs:446` |
| 557 | `S0837` | `0x0837` | OneShot | `S0837.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:522` |
| 558 | `S0838` | `0x0838` | OneShot | `S0838.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:169`, `crates/pin2k-core/src/swe1_audio.rs:442` |
| 559 | `S0839` | `0x0839` | OneShot | `S0839.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:468` |
| 560 | `S083A` | `0x083A` | OneShot | `S083A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:461` |
| 561 | `S083B` | `0x083B` | OneShot | `S083B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:462` |
| 562 | `S083C` | `0x083C` | OneShot | `S083C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:463` |
| 563 | `S083D` | `0x083D` | OneShot | `S083D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:261`, `crates/pin2k-core/src/swe1_audio.rs:467` |
| 564 | `S083E` | `0x083E` | OneShot | `S083E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:420` |
| 565 | `S083F` | `0x083F` | OneShot | `S083F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:252`, `crates/pin2k-core/src/swe1_audio.rs:455` |
| 566 | `S0840` | `0x0840` | OneShot | `S0840.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:425` |
| 567 | `S0841` | `0x0841` | OneShot | `S0841.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:505` |
| 568 | `S0842` | `0x0842` | OneShot | `S0842.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:249`, `crates/pin2k-core/src/swe1_audio.rs:492` |
| 569 | `S0843` | `0x0843` | OneShot | `S0843.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:430` |
| 570 | `S0844` | `0x0844` | OneShot | `S0844.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:449` |
| 571 | `S0845` | `0x0845` | OneShot | `S0845.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:352`, `crates/pin2k-core/src/swe1_audio.rs:472` |
| 572 | `S0847` | `0x0847` | OneShot | `S0847.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:459` |
| 573 | `S0848` | `0x0848` | OneShot | `S0848.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:466` |
| 574 | `S0849` | `0x0849` | OneShot | `S0849.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:504` |
| 575 | `S084A` | `0x084A` | OneShot | `S084A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:435` |
| 576 | `S084B` | `0x084B` | OneShot | `S084B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:453` |
| 577 | `S084C` | `0x084C` | OneShot | `S084C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:454` |
| 578 | `S084D` | `0x084D` | OneShot | `S084D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:513` |
| 579 | `S084E` | `0x084E` | OneShot | `S084E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:483` |
| 580 | `S084F` | `0x084F` | OneShot | `S084F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:484` |
| 581 | `S0850` | `0x0850` | OneShot | `S0850.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:352`, `crates/pin2k-core/src/swe1_audio.rs:490` |
| 582 | `S0851` | `0x0851` | OneShot | `S0851.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:515` |
| 583 | `S0852` | `0x0852` | OneShot | `S0852.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:36`, `crates/pin2k-core/src/swe1_audio.rs:69`, `crates/pin2k-core/src/swe1_audio.rs:91`, `crates/pin2k-core/src/swe1_audio.rs:134`, `crates/pin2k-core/src/swe1_audio.rs:314` (+1) |
| 584 | `S0853` | `0x0853` | OneShot | `S0853.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:514` |
| 585 | `S0855` | `0x0855` | OneShot | `S0855.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:443` |
| 586 | `S0856` | `0x0856` | OneShot | `S0856.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:433` |
| 587 | `S0857` | `0x0857` | OneShot | `S0857.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:36`, `crates/pin2k-core/src/swe1_audio.rs:69`, `crates/pin2k-core/src/swe1_audio.rs:91`, `crates/pin2k-core/src/swe1_audio.rs:134`, `crates/pin2k-core/src/swe1_audio.rs:314` (+1) |
| 588 | `S0858` | `0x0858` | OneShot | `S0858.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:503` |
| 589 | `S0898` | `0x0898` | OneShot | `S0898.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:702` |
| 590 | `S0899` | `0x0899` | OneShot | `S0899.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:738` |
| 591 | `S089A` | `0x089A` | OneShot | `S089A.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:276`, `crates/pin2k-core/src/swe1_audio.rs:741` |
| 592 | `S089B` | `0x089B` | OneShot | `S089B.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:277`, `crates/pin2k-core/src/swe1_audio.rs:742` |
| 593 | `S089C` | `0x089C` | OneShot | `S089C.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:278`, `crates/pin2k-core/src/swe1_audio.rs:743` |
| 594 | `S089D` | `0x089D` | OneShot | `S089D.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:279`, `crates/pin2k-core/src/swe1_audio.rs:744` |
| 595 | `S089E` | `0x089E` | OneShot | `S089E.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:754` |
| 596 | `S089F` | `0x089F` | OneShot | `S089F.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:718` |
| 597 | `S08A0` | `0x08A0` | OneShot | `S08A0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:673` |
| 598 | `S08A1` | `0x08A1` | OneShot | `S08A1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:141`, `crates/pin2k-core/src/swe1_audio.rs:359`, `crates/pin2k-core/src/swe1_audio.rs:762` |
| 599 | `S08A2` | `0x08A2` | OneShot | `S08A2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:668` |
| 600 | `S08A3` | `0x08A3` | OneShot | `S08A3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:669` |
| 601 | `S08A4` | `0x08A4` | OneShot | `S08A4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:670` |
| 602 | `S08A5` | `0x08A5` | OneShot | `S08A5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:671` |
| 603 | `S08A6` | `0x08A6` | OneShot | `S08A6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:753` |
| 604 | `S08A7` | `0x08A7` | OneShot | `S08A7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:789` |
| 605 | `S08A8` | `0x08A8` | OneShot | `S08A8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:785` |
| 606 | `S08A9` | `0x08A9` | OneShot | `S08A9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:77`, `crates/pin2k-core/src/swe1_audio.rs:697` |
| 607 | `S08AA` | `0x08AA` | OneShot | `S08AA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:74`, `crates/pin2k-core/src/swe1_audio.rs:721` |
| 608 | `S08AB` | `0x08AB` | OneShot | `S08AB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:77`, `crates/pin2k-core/src/swe1_audio.rs:778` |
| 609 | `S08AC` | `0x08AC` | OneShot | `S08AC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:74`, `crates/pin2k-core/src/swe1_audio.rs:730` |
| 610 | `S08AD` | `0x08AD` | OneShot | `S08AD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:348`, `crates/pin2k-core/src/swe1_audio.rs:788` |
| 611 | `S08AE` | `0x08AE` | OneShot | `S08AE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:790` |
| 612 | `S08AF` | `0x08AF` | OneShot | `S08AF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:745` |
| 613 | `S08B0` | `0x08B0` | OneShot | `S08B0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:252`, `crates/pin2k-core/src/swe1_audio.rs:781` |
| 614 | `S08B1` | `0x08B1` | OneShot | `S08B1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:249`, `crates/pin2k-core/src/swe1_audio.rs:779` |
| 615 | `S08B2` | `0x08B2` | OneShot | `S08B2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:780` |
| 616 | `S08B3` | `0x08B3` | OneShot | `S08B3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:747` |
| 617 | `S08B4` | `0x08B4` | OneShot | `S08B4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:190`, `crates/pin2k-core/src/swe1_audio.rs:724` |
| 618 | `S08B5` | `0x08B5` | OneShot | `S08B5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:190`, `crates/pin2k-core/src/swe1_audio.rs:725` |
| 619 | `S08B6` | `0x08B6` | OneShot | `S08B6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:185`, `crates/pin2k-core/src/swe1_audio.rs:191`, `crates/pin2k-core/src/swe1_audio.rs:705` |
| 620 | `S08B7` | `0x08B7` | OneShot | `S08B7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:191`, `crates/pin2k-core/src/swe1_audio.rs:706` |
| 621 | `S08B8` | `0x08B8` | OneShot | `S08B8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:191`, `crates/pin2k-core/src/swe1_audio.rs:707` |
| 622 | `S08B9` | `0x08B9` | OneShot | `S08B9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:191`, `crates/pin2k-core/src/swe1_audio.rs:708` |
| 623 | `S08BA` | `0x08BA` | OneShot | `S08BA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:379`, `crates/pin2k-core/src/swe1_audio.rs:710` |
| 624 | `S08BB` | `0x08BB` | OneShot | `S08BB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:186`, `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:379`, `crates/pin2k-core/src/swe1_audio.rs:711` |
| 625 | `S08BC` | `0x08BC` | OneShot | `S08BC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:168`, `crates/pin2k-core/src/swe1_audio.rs:248`, `crates/pin2k-core/src/swe1_audio.rs:787` |
| 626 | `S08BD` | `0x08BD` | OneShot | `S08BD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:678` |
| 627 | `S08BE` | `0x08BE` | OneShot | `S08BE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:36`, `crates/pin2k-core/src/swe1_audio.rs:69`, `crates/pin2k-core/src/swe1_audio.rs:156`, `crates/pin2k-core/src/swe1_audio.rs:248`, `crates/pin2k-core/src/swe1_audio.rs:314` (+1) |
| 628 | `S08BF` | `0x08BF` | OneShot | `S08BF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:248`, `crates/pin2k-core/src/swe1_audio.rs:764` |
| 629 | `S08C0` | `0x08C0` | OneShot | `S08C0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:141`, `crates/pin2k-core/src/swe1_audio.rs:251`, `crates/pin2k-core/src/swe1_audio.rs:786` |
| 630 | `S08C1` | `0x08C1` | OneShot | `S08C1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:36`, `crates/pin2k-core/src/swe1_audio.rs:69`, `crates/pin2k-core/src/swe1_audio.rs:156`, `crates/pin2k-core/src/swe1_audio.rs:314`, `crates/pin2k-core/src/swe1_audio.rs:782` |
| 631 | `S08C2` | `0x08C2` | OneShot | `S08C2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:696` |
| 632 | `S08C3` | `0x08C3` | OneShot | `S08C3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:141`, `crates/pin2k-core/src/swe1_audio.rs:733` |
| 633 | `S08C4` | `0x08C4` | OneShot | `S08C4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:168`, `crates/pin2k-core/src/swe1_audio.rs:196`, `crates/pin2k-core/src/swe1_audio.rs:359`, `crates/pin2k-core/src/swe1_audio.rs:752` |
| 634 | `S08C5` | `0x08C5` | OneShot | `S08C5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:693` |
| 635 | `S08C6` | `0x08C6` | OneShot | `S08C6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:692` |
| 636 | `S08C7` | `0x08C7` | OneShot | `S08C7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:168`, `crates/pin2k-core/src/swe1_audio.rs:372`, `crates/pin2k-core/src/swe1_audio.rs:677` |
| 637 | `S08C8` | `0x08C8` | OneShot | `S08C8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:675` |
| 638 | `S08C9` | `0x08C9` | OneShot | `S08C9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:261`, `crates/pin2k-core/src/swe1_audio.rs:726` |
| 639 | `S08CA` | `0x08CA` | OneShot | `S08CA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:261`, `crates/pin2k-core/src/swe1_audio.rs:760` |
| 640 | `S08CB` | `0x08CB` | OneShot | `S08CB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:766` |
| 641 | `S08CC` | `0x08CC` | OneShot | `S08CC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:372`, `crates/pin2k-core/src/swe1_audio.rs:773` |
| 642 | `S08CD` | `0x08CD` | OneShot | `S08CD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:168`, `crates/pin2k-core/src/swe1_audio.rs:775` |
| 643 | `S08CE` | `0x08CE` | OneShot | `S08CE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:791` |
| 644 | `S08CF` | `0x08CF` | OneShot | `S08CF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:193`, `crates/pin2k-core/src/swe1_audio.rs:737` |
| 645 | `S08D0` | `0x08D0` | OneShot | `S08D0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:774` |
| 646 | `S08D1` | `0x08D1` | OneShot | `S08D1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:765` |
| 647 | `S08D2` | `0x08D2` | OneShot | `S08D2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:372`, `crates/pin2k-core/src/swe1_audio.rs:758` |
| 648 | `S08D3` | `0x08D3` | OneShot | `S08D3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:714` |
| 649 | `S08D4` | `0x08D4` | OneShot | `S08D4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:36`, `crates/pin2k-core/src/swe1_audio.rs:69`, `crates/pin2k-core/src/swe1_audio.rs:156`, `crates/pin2k-core/src/swe1_audio.rs:314`, `crates/pin2k-core/src/swe1_audio.rs:777` |
| 650 | `S08D5` | `0x08D5` | OneShot | `S08D5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:687` |
| 651 | `S08D6` | `0x08D6` | OneShot | `S08D6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:252`, `crates/pin2k-core/src/swe1_audio.rs:703` |
| 652 | `S08D7` | `0x08D7` | OneShot | `S08D7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:235`, `crates/pin2k-core/src/swe1_audio.rs:720` |
| 653 | `S08D8` | `0x08D8` | OneShot | `S08D8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:767` |
| 654 | `S08D9` | `0x08D9` | OneShot | `S08D9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:674` |
| 655 | `S08DA` | `0x08DA` | OneShot | `S08DA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:719` |
| 656 | `S08DB` | `0x08DB` | OneShot | `S08DB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:192`, `crates/pin2k-core/src/swe1_audio.rs:736` |
| 657 | `S08DC` | `0x08DC` | OneShot | `S08DC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:196`, `crates/pin2k-core/src/swe1_audio.rs:713` |
| 658 | `S08DD` | `0x08DD` | OneShot | `S08DD.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:193`, `crates/pin2k-core/src/swe1_audio.rs:734` |
| 659 | `S08DE` | `0x08DE` | OneShot | `S08DE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:193`, `crates/pin2k-core/src/swe1_audio.rs:712` |
| 660 | `S08DF` | `0x08DF` | OneShot | `S08DF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:236`, `crates/pin2k-core/src/swe1_audio.rs:680` |
| 661 | `S08E0` | `0x08E0` | OneShot | `S08E0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:236`, `crates/pin2k-core/src/swe1_audio.rs:716` |
| 662 | `S08E1` | `0x08E1` | OneShot | `S08E1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:359`, `crates/pin2k-core/src/swe1_audio.rs:735` |
| 663 | `S08E2` | `0x08E2` | OneShot | `S08E2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:46`, `crates/pin2k-core/src/swe1_audio.rs:701` |
| 664 | `S08E3` | `0x08E3` | OneShot | `S08E3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:192`, `crates/pin2k-core/src/swe1_audio.rs:727` |
| 665 | `S08E4` | `0x08E4` | OneShot | `S08E4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:192`, `crates/pin2k-core/src/swe1_audio.rs:722` |
| 666 | `S08E5` | `0x08E5` | OneShot | `S08E5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:192`, `crates/pin2k-core/src/swe1_audio.rs:771` |
| 667 | `S08E6` | `0x08E6` | OneShot | `S08E6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:192`, `crates/pin2k-core/src/swe1_audio.rs:740` |
| 668 | `S08E7` | `0x08E7` | OneShot | `S08E7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:681` |
| 669 | `S08E8` | `0x08E8` | OneShot | `S08E8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:683` |
| 670 | `S08E9` | `0x08E9` | OneShot | `S08E9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:755` |
| 671 | `S08EA` | `0x08EA` | OneShot | `S08EA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:688` |
| 672 | `S08EB` | `0x08EB` | OneShot | `S08EB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:365`, `crates/pin2k-core/src/swe1_audio.rs:686` |
| 673 | `S08EC` | `0x08EC` | OneShot | `S08EC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:359`, `crates/pin2k-core/src/swe1_audio.rs:761` |
| 674 | `S08ED` | `0x08ED` | OneShot | `S08ED.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:676` |
| 675 | `S08EE` | `0x08EE` | OneShot | `S08EE.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:732` |
| 676 | `S08EF` | `0x08EF` | OneShot | `S08EF.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:759` |
| 677 | `S08F0` | `0x08F0` | OneShot | `S08F0.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:365`, `crates/pin2k-core/src/swe1_audio.rs:756` |
| 678 | `S08F1` | `0x08F1` | OneShot | `S08F1.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:147`, `crates/pin2k-core/src/swe1_audio.rs:757` |
| 679 | `S08F2` | `0x08F2` | OneShot | `S08F2.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:694` |
| 680 | `S08F3` | `0x08F3` | OneShot | `S08F3.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:715` |
| 681 | `S08F4` | `0x08F4` | OneShot | `S08F4.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:188`, `crates/pin2k-core/src/swe1_audio.rs:772` |
| 682 | `S08F5` | `0x08F5` | OneShot | `S08F5.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:739` |
| 683 | `S08F6` | `0x08F6` | OneShot | `S08F6.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:143`, `crates/pin2k-core/src/swe1_audio.rs:698` |
| 684 | `S08F7` | `0x08F7` | OneShot | `S08F7.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:369`, `crates/pin2k-core/src/swe1_audio.rs:783` |
| 685 | `S08F8` | `0x08F8` | OneShot | `S08F8.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:192`, `crates/pin2k-core/src/swe1_audio.rs:700` |
| 686 | `S08F9` | `0x08F9` | OneShot | `S08F9.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:682` |
| 687 | `S08FA` | `0x08FA` | OneShot | `S08FA.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:784` |
| 688 | `S08FB` | `0x08FB` | OneShot | `S08FB.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:709` |
| 689 | `S08FC` | `0x08FC` | OneShot | `S08FC.ogg` | Oui | `crates/pin2k-core/src/swe1_audio.rs:143`, `crates/pin2k-core/src/swe1_audio.rs:364`, `crates/pin2k-core/src/swe1_audio.rs:763` |
