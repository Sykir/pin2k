# Tableau Croise Complet des Assets (`swe1`) & Utilisation dans le Code

Ce document recense **100 % des assets extraits** de *Star Wars Episode I* (`#50069 v2.00`) et indique pour chacun s'il est reference dans le code source Rust (`crates/`), et ou.

## Resume de Couverture

| Categorie | Total extrait | Utilises | Non utilises |
|---|---:|---:|---:|
| **Animations & Sprites (`gfx/`)** | 392 | 186 | 206 |
| **Polices Bitmap (`fonts/`)** | 19 | 19 | 0 |
| **Pistes Audio DCS-2 (`audio/`)** | 689 | 47 | 642 |

---

## 1. Animations & Sprites (392 entrees)

| # | Asset | Frames | FPS | Dimensions | Utilise | Emplacement(s) dans le code |
|---:|---|---:|---:|---|---|---|
| 1 | `anim_battle_droid_intro_battle` | 55 | 15 | 220x160 | Oui | `crates/pin2k-core/src/attract.rs:74`, `crates/pin2k-core/src/swe1_rules.rs:126`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:530`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:693` |
| 2 | `anim_battle_droid_intro_badge` | 1 | 60 | 304x79 | Oui | `crates/pin2k-core/src/attract.rs:78`, `crates/pin2k-core/src/swe1_rules.rs:146`, `crates/pin2k-core/src/machine/attract_render.rs:370`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:547`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:572` |
| 3 | `anim_bdroid_explosion` | 20 | 60 | 245x129 | Non | - |
| 4 | `anim_bdroid_laser_explosion` | 20 | 60 | 230x131 | Non | - |
| 5 | `anim_bdroid_on_stap` | 26 | 60 | 163x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:186`, `crates/pin2k-core/src/machine/scene_render.rs:83`, `crates/pin2k-desktop/src/scenes_compositor.rs:27`, `crates/pin2k-desktop/src/scenes_compositor.rs:31` |
| 6 | `anim_bdroid_tank_explosion` | 25 | 60 | 580x220 | Non | - |
| 7 | `anim_bdroid_stap_zap` | 22 | 60 | 56x19 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:86`, `crates/pin2k-desktop/src/scenes_compositor.rs:34` |
| 8 | `anim_bdroid_stap_explosion` | 19 | 60 | 574x213 | Non | - |
| 9 | `anim_bdroid_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:80`, `crates/pin2k-desktop/src/scenes_compositor.rs:28` |
| 10 | `anim_bdroid_zap` | 44 | 60 | 28x9 | Non | - |
| 11 | `anim_bonus_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:246`, `crates/pin2k-core/src/machine/system_screens.rs:123`, `crates/pin2k-core/src/machine/tests.rs:589` |
| 12 | `anim_explosion_2` | 30 | 60 | 150x63 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:67` |
| 13 | `anim_explosion_3` | 21 | 60 | 640x240 | Non | - |
| 14 | `anim_explosion_4` | 30 | 60 | 160x53 | Non | - |
| 15 | `anim_explosion_5` | 30 | 60 | 160x53 | Non | - |
| 16 | `anim_explosion_6` | 30 | 60 | 160x53 | Non | - |
| 17 | `anim_extra_ball_show` | 31 | 15 | 320x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:281`, `crates/pin2k-core/src/machine/system_screens.rs:258`, `crates/pin2k-core/src/machine/tests.rs:593` |
| 18 | `anim_fashion_flash` | 12 | 60 | 190x121 | Non | - |
| 19 | `anim_fashion_wheel` | 10 | 60 | 497x152 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:176`, `crates/pin2k-desktop/src/scenes_compositor.rs:81` |
| 20 | `anim_fashion_base` | 2 | 60 | 220x30 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:173`, `crates/pin2k-desktop/src/scenes_compositor.rs:78` |
| 21 | `anim_fashion_queen` | 1 | 60 | 198x174 | Oui | `crates/pin2k-core/src/swe1_rules.rs:194`, `crates/pin2k-core/src/machine/scene_render.rs:175`, `crates/pin2k-desktop/src/scenes_compositor.rs:77`, `crates/pin2k-desktop/src/scenes_compositor.rs:80` |
| 22 | `anim_fashion_throne` | 1 | 60 | 597x196 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:174`, `crates/pin2k-desktop/src/scenes_compositor.rs:79` |
| 23 | `anim_fashion_icon_black` | 60 | 60 | 127x63 | Non | - |
| 24 | `anim_fashion_icon_fanhead` | 60 | 60 | 127x63 | Non | - |
| 25 | `anim_fashion_icon_hornhead` | 60 | 60 | 127x63 | Non | - |
| 26 | `anim_fashion_icon_nun` | 60 | 60 | 127x63 | Non | - |
| 27 | `anim_fashion_icon_spade` | 60 | 60 | 127x63 | Non | - |
| 28 | `anim_fashion_icon_victorian` | 60 | 60 | 127x63 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:82` |
| 29 | `anim_fashion_icon_extra_ball` | 60 | 60 | 124x62 | Non | - |
| 30 | `anim_federation_fighter_laser_explosion_2` | 30 | 60 | 324x150 | Non | - |
| 31 | `anim_federation_fighter_intro_badge` | 1 | 60 | 295x58 | Oui | `crates/pin2k-core/src/swe1_rules.rs:148` |
| 32 | `anim_federation_fighter_intro_battle` | 59 | 12 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:128` |
| 33 | `anim_federation_fighter_cockpit` | 10 | 60 | 550x40 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:123` |
| 34 | `anim_federation_fighter` | 7 | 60 | 143x38 | Oui | `crates/pin2k-core/src/swe1_rules.rs:188`, `crates/pin2k-core/src/machine/scene_render.rs:101`, `crates/pin2k-desktop/src/scenes_compositor.rs:45`, `crates/pin2k-desktop/src/scenes_compositor.rs:48` |
| 35 | `anim_federation_fighter_flyby` | 27 | 60 | 208x170 | Non | - |
| 36 | `anim_federation_fighter_explosion` | 16 | 60 | 324x150 | Non | - |
| 37 | `anim_federation_fighter_explosion_2` | 36 | 60 | 360x150 | Non | - |
| 38 | `anim_federation_fighter_explosion_3` | 40 | 60 | 324x150 | Non | - |
| 39 | `anim_federation_fighter_laser_explosion` | 16 | 60 | 324x150 | Non | - |
| 40 | `anim_federation_fighter_side` | 1 | 60 | 127x280 | Non | - |
| 41 | `anim_federation_fighter_background` | 1 | 60 | 900x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:99`, `crates/pin2k-desktop/src/scenes_compositor.rs:46` |
| 42 | `anim_federation_fighter_zap` | 58 | 60 | 88x19 | Non | - |
| 43 | `anim_gungan_sub_sub` | 80 | 60 | 464x122 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:108`, `crates/pin2k-core/src/machine/system_screens.rs:322`, `crates/pin2k-desktop/src/scenes_compositor.rs:55` |
| 44 | `anim_gungan_sub_opee` | 150 | 60 | 415x163 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:109` |
| 45 | `anim_gungan_sub_chomp` | 75 | 60 | 640x240 | Oui | `crates/pin2k-core/src/swe1_rules.rs:189`, `crates/pin2k-core/src/machine/script_exec.rs:213`, `crates/pin2k-core/src/machine/system_screens.rs:323`, `crates/pin2k-core/src/machine/tests.rs:597`, `crates/pin2k-desktop/src/scenes_compositor.rs:52` (+1) |
| 46 | `anim_gungan_sub_bubbles` | 54 | 60 | 146x215 | Non | - |
| 47 | `anim_hangar_droid_run` | 90 | 60 | 85x127 | Oui | `crates/pin2k-core/src/swe1_rules.rs:190`, `crates/pin2k-core/src/machine/scene_render.rs:124`, `crates/pin2k-desktop/src/scenes_compositor.rs:59`, `crates/pin2k-desktop/src/scenes_compositor.rs:60` |
| 48 | `anim_hangar_droid_explosion` | 30 | 60 | 251x145 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:126`, `crates/pin2k-core/src/machine/swe1_playfield.rs:266`, `crates/pin2k-desktop/src/scenes_compositor.rs:61` |
| 49 | `anim_hangar_droid_explosion_2` | 30 | 60 | 238x145 | Non | - |
| 50 | `anim_hangar_droid_explosion_3` | 30 | 60 | 306x127 | Non | - |
| 51 | `anim_hangar_droid_explosion_4` | 30 | 60 | 262x145 | Non | - |
| 52 | `anim_hangar_droid_laser_explosion` | 30 | 60 | 255x149 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:127`, `crates/pin2k-desktop/src/scenes_compositor.rs:62` |
| 53 | `anim_hangar_droid_laser_explosion_2` | 30 | 60 | 268x131 | Non | - |
| 54 | `anim_hangar_escape_intro_movie` | 74 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:130`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:582` |
| 55 | `anim_hangar_escape_intro_badge` | 1 | 60 | 299x78 | Oui | `crates/pin2k-core/src/swe1_rules.rs:150` |
| 56 | `anim_hangar_escape_exit_movie` | 47 | 15 | 320x240 | Non | - |
| 57 | `anim_hangar_escape_torpedo` | 181 | 60 | 200x91 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:125` |
| 58 | `anim_jackpot_icon_idle` | 60 | 60 | 161x61 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:301` |
| 59 | `anim_jackpot_icon_boom` | 9 | 60 | 354x110 | Non | - |
| 60 | `anim_jarjar_juggle_intro_movie` | 67 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:135` |
| 61 | `anim_jarjar_juggle_intro_badge` | 1 | 60 | 330x79 | Oui | `crates/pin2k-core/src/swe1_rules.rs:155` |
| 62 | `anim_jarjar_frog_fountain` | 46 | 60 | 200x129 | Non | - |
| 63 | `anim_jarjar_appear` | 28 | 60 | 141x161 | Non | - |
| 64 | `anim_jarjar_juggle` | 19 | 60 | 407x191 | Oui | `crates/pin2k-core/src/swe1_rules.rs:195`, `crates/pin2k-core/src/machine/scene_render.rs:184`, `crates/pin2k-desktop/src/scenes_compositor.rs:85`, `crates/pin2k-desktop/src/scenes_compositor.rs:87` |
| 65 | `anim_jarjar_fall` | 20 | 60 | 470x240 | Non | - |
| 66 | `anim_jarjar_jumpup` | 40 | 60 | 400x213 | Non | - |
| 67 | `anim_jarjar_switch` | 4 | 60 | 189x209 | Non | - |
| 68 | `anim_jarjar_stare` | 11 | 60 | 189x209 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:183`, `crates/pin2k-desktop/src/scenes_compositor.rs:86` |
| 69 | `anim_jarjar_junk1` | 20 | 60 | 96x46 | Non | - |
| 70 | `anim_jarjar_junk2` | 20 | 60 | 126x60 | Non | - |
| 71 | `anim_jarjar_junk3` | 20 | 60 | 112x44 | Non | - |
| 72 | `anim_jarjar_junk4` | 20 | 60 | 88x58 | Non | - |
| 73 | `anim_jarjar_junk5` | 20 | 60 | 103x38 | Non | - |
| 74 | `anim_jarjar_junk6` | 20 | 60 | 124x63 | Non | - |
| 75 | `anim_jarjar_surprise` | 28 | 60 | 174x134 | Non | - |
| 76 | `anim_jarjar_idle_1` | 20 | 60 | 94x135 | Non | - |
| 77 | `anim_jarjar_junk_eb` | 20 | 60 | 116x61 | Non | - |
| 78 | `anim_jarjar_juggle_2` | 32 | 60 | 466x213 | Non | - |
| 79 | `anim_jarjar_fall_2` | 18 | 60 | 318x220 | Non | - |
| 80 | `anim_jarjar_tongue` | 35 | 60 | 202x141 | Non | - |
| 81 | `anim_jarjar_juggle_backdrop` | 1 | 60 | 629x141 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:182` |
| 82 | `anim_jedi_battle_intro_movie` | 63 | 15 | 220x160 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:364`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:595` |
| 83 | `anim_jedi_battle_intro_badge` | 1 | 60 | 445x108 | Non | - |
| 84 | `anim_jedi_battle_intro_frame` | 1 | 60 | 594x216 | Non | - |
| 85 | `anim_jedi_battle_background_center` | 1 | 60 | 340x240 | Non | - |
| 86 | `anim_jedi_battle_background_left` | 10 | 60 | 150x240 | Non | - |
| 87 | `anim_jedi_battle_background_right` | 10 | 60 | 150x240 | Non | - |
| 88 | `anim_jedi_council_mace` | 1 | 60 | 93x58 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:214`, `crates/pin2k-desktop/src/scenes_compositor.rs:99` |
| 89 | `anim_jedi_council_background` | 1 | 60 | 593x234 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:213` |
| 90 | `anim_jedi_council_neck` | 12 | 60 | 90x58 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:218` |
| 91 | `anim_jedi_council_yoda` | 6 | 60 | 93x46 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:215`, `crates/pin2k-desktop/src/scenes_compositor.rs:100` |
| 92 | `anim_jedi_council_member_1` | 6 | 60 | 86x60 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:216` |
| 93 | `anim_jedi_council_member_2` | 5 | 60 | 96x58 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:217` |
| 94 | `anim_jedi_letters_loop` | 80 | 60 | 54x18 | Non | - |
| 95 | `anim_jedi_letters_shrink` | 88 | 60 | 247x82 | Non | - |
| 96 | `anim_jedi_scene_intro_frame` | 1 | 60 | 522x195 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:52`, `crates/pin2k-core/src/machine/tests.rs:547` |
| 97 | `anim_kahuna_logo_flare` | 17 | 60 | 595x226 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:53`, `crates/pin2k-core/src/machine/tests.rs:459` |
| 98 | `anim_kahuna_logo_spinning` | 1 | 60 | 267x81 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:59`, `crates/pin2k-core/src/machine/tests.rs:466` |
| 99 | `anim_left_bdroid_shooting` | 10 | 60 | 55x91 | Non | - |
| 100 | `anim_left_bdroid_standing` | 26 | 60 | 67x96 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:81`, `crates/pin2k-desktop/src/scenes_compositor.rs:29` |
| 101 | `anim_left_bdroid_unpacking` | 15 | 60 | 189x96 | Non | - |
| 102 | `anim_mtt_frame` | 1 | 60 | 242x135 | Non | - |
| 103 | `anim_mtt_doors_clean_anim` | 19 | 60 | 138x152 | Non | - |
| 104 | `anim_mtt_doors_damage1_anim` | 19 | 60 | 138x152 | Non | - |
| 105 | `anim_mtt_doors_damage2_anim` | 19 | 60 | 138x152 | Non | - |
| 106 | `anim_mtt_doors_damage3_anim` | 19 | 60 | 138x152 | Non | - |
| 107 | `anim_mtt_fires` | 60 | 60 | 242x150 | Non | - |
| 108 | `anim_musical_chairs_intro_movie` | 56 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:137` |
| 109 | `anim_musical_chairs_intro_badge` | 1 | 60 | 394x69 | Oui | `crates/pin2k-core/src/swe1_rules.rs:157` |
| 110 | `anim_mos_espa_pipes` | 1 | 60 | 640x39 | Non | - |
| 111 | `anim_mos_espa_background` | 1 | 60 | 640x240 | Non | - |
| 112 | `anim_mos_espa_chuba_1` | 15 | 60 | 80x164 | Non | - |
| 113 | `anim_mos_espa_chuba_2` | 15 | 60 | 132x170 | Non | - |
| 114 | `anim_mos_espa_chuba_3` | 15 | 60 | 72x177 | Non | - |
| 115 | `anim_mos_espa_chuba_4` | 15 | 60 | 149x182 | Non | - |
| 116 | `anim_mos_espa_jarjar_choose` | 37 | 60 | 299x206 | Non | - |
| 117 | `anim_mos_espa_jarjar_eat_1` | 14 | 60 | 413x205 | Non | - |
| 118 | `anim_mos_espa_jarjar_eat_2` | 13 | 60 | 341x205 | Non | - |
| 119 | `anim_mos_espa_jarjar_eat_3` | 14 | 60 | 329x203 | Non | - |
| 120 | `anim_mos_espa_jarjar_eat_4` | 14 | 60 | 379x205 | Non | - |
| 121 | `anim_mos_espa_jarjar_swallow` | 26 | 60 | 217x205 | Non | - |
| 122 | `anim_multiball_start_movie` | 90 | 15 | 320x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:282`, `crates/pin2k-core/src/machine/script_exec.rs:179` |
| 123 | `anim_naboo_flyby_single` | 1 | 60 | 225x40 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:229`, `crates/pin2k-core/src/machine/system_screens.rs:360`, `crates/pin2k-core/src/machine/tests.rs:599` |
| 124 | `anim_naboo_fighter_tiny_rotate` | 60 | 60 | 100x50 | Oui | `crates/pin2k-core/src/machine/hud_display.rs:25`, `crates/pin2k-core/src/machine/tests.rs:370`, `crates/pin2k-core/src/machine/tests.rs:386` |
| 125 | `anim_naboo_flyby_frame` | 1 | 60 | 640x134 | Non | - |
| 126 | `anim_naboo_flyby` | 64 | 15 | 276x120 | Non | - |
| 127 | `anim_naboo_xhair` | 1 | 60 | 150x75 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:102`, `crates/pin2k-desktop/src/scenes_compositor.rs:49` |
| 128 | `anim_naboo_xhair_blue` | 1 | 60 | 136x68 | Non | - |
| 129 | `anim_obiwan_holo` | 60 | 60 | 320x160 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:375`, `crates/pin2k-desktop/src/scenes_compositor.rs:118` |
| 130 | `anim_obiwan_holo_glow` | 60 | 60 | 320x160 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:374`, `crates/pin2k-desktop/src/scenes_compositor.rs:117` |
| 131 | `anim_obiwan_holo_activate` | 18 | 60 | 255x81 | Non | - |
| 132 | `anim_obiwan_holo_anakin_pod` | 69 | 60 | 289x63 | Oui | `crates/pin2k-core/src/swe1_rules.rs:172` |
| 133 | `anim_obiwan_holo_battle_droid` | 71 | 60 | 198x138 | Oui | `crates/pin2k-core/src/swe1_rules.rs:166` |
| 134 | `anim_holo_burst` | 18 | 60 | 450x225 | Non | - |
| 135 | `anim_obiwan_holo_federation_fighter` | 72 | 60 | 400x75 | Oui | `crates/pin2k-core/src/swe1_rules.rs:168` |
| 136 | `anim_obiwan_holo_flatten` | 30 | 60 | 320x160 | Non | - |
| 137 | `anim_obiwan_holo_glow_loop` | 15 | 60 | 222x78 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:376`, `crates/pin2k-desktop/src/scenes_compositor.rs:119` |
| 138 | `anim_obiwan_holo_gungan_sub` | 72 | 60 | 400x64 | Oui | `crates/pin2k-core/src/swe1_rules.rs:169` |
| 139 | `anim_obiwan_holo_jarjar` | 71 | 60 | 174x138 | Oui | `crates/pin2k-core/src/swe1_rules.rs:175` |
| 140 | `anim_obiwan_holo_naboo_fighter` | 72 | 60 | 282x54 | Oui | `crates/pin2k-core/src/swe1_rules.rs:170` |
| 141 | `anim_obiwan_holo_queens_gun` | 71 | 60 | 213x75 | Non | - |
| 142 | `anim_obiwan_holo_r2d2` | 73 | 60 | 202x113 | Oui | `crates/pin2k-core/src/swe1_rules.rs:173`, `crates/pin2k-core/src/swe1_rules.rs:178` |
| 143 | `anim_obiwan_holo_yoda` | 72 | 60 | 204x100 | Oui | `crates/pin2k-core/src/swe1_rules.rs:177` |
| 144 | `anim_p2k_logo_animated` | 90 | 15 | 200x200 | Oui | `crates/pin2k-core/src/attract.rs:75`, `crates/pin2k-core/src/script_engine.rs:341`, `crates/pin2k-core/src/machine/attract_render.rs:192`, `crates/pin2k-desktop/src/compositor.rs:243`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:537` |
| 145 | `anim_podrace_ground_slow` | 11 | 60 | 640x128 | Non | - |
| 146 | `anim_podrace_sky` | 1 | 60 | 640x99 | Oui | `crates/pin2k-desktop/src/compositor.rs:565` |
| 147 | `anim_podrace_horizon` | 1 | 60 | 800x112 | Oui | `crates/pin2k-desktop/src/compositor.rs:567` |
| 148 | `anim_podrace_intro_badge` | 1 | 60 | 266x66 | Oui | `crates/pin2k-core/src/swe1_rules.rs:152`, `crates/pin2k-desktop/src/compositor.rs:560` |
| 149 | `anim_probe_droid_intro_movie` | 83 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:133` |
| 150 | `anim_probe_droid_intro_badge` | 1 | 60 | 327x77 | Oui | `crates/pin2k-core/src/swe1_rules.rs:153` |
| 151 | `anim_queen_ship_intro_badge` | 1 | 60 | 304x74 | Oui | `crates/pin2k-core/src/swe1_rules.rs:154` |
| 152 | `anim_r2d2_intro_movie` | 53 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:138` |
| 153 | `anim_r2d2_intro_badge` | 1 | 60 | 355x70 | Oui | `crates/pin2k-core/src/swe1_rules.rs:158` |
| 154 | `anim_r2d2_scene_background_upper` | 1 | 60 | 640x135 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:231` |
| 155 | `anim_r2d2_scene_background_lower` | 1 | 60 | 640x131 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:232` |
| 156 | `anim_r2d2_scene_r2d2_unit` | 80 | 60 | 190x78 | Oui | `crates/pin2k-core/src/swe1_rules.rs:198`, `crates/pin2k-core/src/machine/scene_render.rs:233` |
| 157 | `anim_r2d2_scene_dark_blue_r2_unit` | 60 | 60 | 222x60 | Non | - |
| 158 | `anim_r2d2_scene_yellow_r2_unit` | 60 | 60 | 278x90 | Non | - |
| 159 | `anim_r2d2_scene_blue_yellow_r2_unit` | 60 | 60 | 175x47 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:234` |
| 160 | `anim_r2d2_scene_red_r2_unit` | 60 | 60 | 214x59 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:235` |
| 161 | `anim_r2d2_scene_red_white_r2_unit` | 60 | 60 | 276x72 | Non | - |
| 162 | `anim_republic_cruiser_intro` | 60 | 15 | 320x140 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:273`, `crates/pin2k-core/src/machine/system_screens.rs:235`, `crates/pin2k-core/src/machine/tests.rs:592` |
| 163 | `anim_right_bdroid_shooting` | 10 | 60 | 52x89 | Non | - |
| 164 | `anim_right_bdroid_standing` | 26 | 60 | 64x96 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:82`, `crates/pin2k-desktop/src/scenes_compositor.rs:30` |
| 165 | `anim_right_bdroid_unpacking` | 15 | 60 | 186x96 | Non | - |
| 166 | `anim_saber_handle` | 1 | 60 | 39x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:64`, `crates/pin2k-core/src/machine/tests.rs:351` |
| 167 | `anim_saber_blue_loop` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:26`, `crates/pin2k-rom-tool/src/layout_extractor.rs:816` |
| 168 | `anim_saber_blue_flare` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:26`, `crates/pin2k-rom-tool/src/layout_extractor.rs:817` |
| 169 | `anim_saber_blue_start` | 4 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:26`, `crates/pin2k-rom-tool/src/layout_extractor.rs:815` |
| 170 | `anim_saber_green_loop` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:24`, `crates/pin2k-rom-tool/src/layout_extractor.rs:804` |
| 171 | `anim_saber_green_flare` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:24`, `crates/pin2k-rom-tool/src/layout_extractor.rs:805` |
| 172 | `anim_saber_green_start` | 4 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:24`, `crates/pin2k-rom-tool/src/layout_extractor.rs:803` |
| 173 | `anim_saber_red_loop` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:25`, `crates/pin2k-rom-tool/src/layout_extractor.rs:810` |
| 174 | `anim_saber_red_flare` | 10 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:25`, `crates/pin2k-rom-tool/src/layout_extractor.rs:811` |
| 175 | `anim_saber_red_start` | 4 | 60 | 245x32 | Oui | `crates/pin2k-core/src/machine/score_display.rs:25`, `crates/pin2k-rom-tool/src/layout_extractor.rs:809` |
| 176 | `anim_sith_probe_blast_1` | 2 | 60 | 253x43 | Non | - |
| 177 | `anim_sith_probe_blast_2` | 2 | 60 | 137x40 | Non | - |
| 178 | `anim_sith_probe_blast_3` | 2 | 60 | 109x42 | Non | - |
| 179 | `anim_sith_probe_explosion_1` | 28 | 60 | 318x150 | Non | - |
| 180 | `anim_sith_probe_explosion_2` | 56 | 60 | 347x153 | Non | - |
| 181 | `anim_sith_probe_fly_1` | 36 | 60 | 110x103 | Non | - |
| 182 | `anim_sith_probe_fly_2` | 12 | 60 | 108x89 | Non | - |
| 183 | `anim_sith_probe_fly_3` | 12 | 60 | 114x92 | Non | - |
| 184 | `anim_sith_probe_hover` | 24 | 60 | 135x99 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:167`, `crates/pin2k-desktop/src/scenes_compositor.rs:72` |
| 185 | `anim_sith_probe_hover_far` | 24 | 60 | 81x61 | Non | - |
| 186 | `anim_sith_probe_miss_explosion` | 25 | 60 | 125x65 | Non | - |
| 187 | `anim_sith_probe_queen` | 41 | 60 | 280x175 | Non | - |
| 188 | `anim_sith_probe_queen_shoot_1` | 21 | 60 | 348x175 | Oui | `crates/pin2k-core/src/swe1_rules.rs:193`, `crates/pin2k-core/src/machine/scene_render.rs:168`, `crates/pin2k-desktop/src/scenes_compositor.rs:70`, `crates/pin2k-desktop/src/scenes_compositor.rs:73` |
| 189 | `anim_sith_probe_queen_shoot_2` | 21 | 60 | 285x177 | Non | - |
| 190 | `anim_sith_probe_queen_shoot_3` | 21 | 60 | 298x176 | Non | - |
| 191 | `anim_sith_probe_target` | 20 | 60 | 125x65 | Non | - |
| 192 | `anim_sith_probe_xhair` | 29 | 60 | 144x77 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:169`, `crates/pin2k-desktop/src/scenes_compositor.rs:74` |
| 193 | `anim_stap_xhair` | 11 | 60 | 39x21 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:84`, `crates/pin2k-desktop/src/scenes_compositor.rs:32` |
| 194 | `anim_starwars_banner_1` | 1 | 60 | 720x240 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:64` |
| 195 | `anim_starwars_banner_2` | 1 | 60 | 720x240 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:65` |
| 196 | `anim_starwars_banner_3` | 1 | 60 | 720x240 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:66` |
| 197 | `anim_starwars_banner_4` | 1 | 60 | 720x240 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:67` |
| 198 | `anim_sub_escape_intro_movie` | 55 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:129` |
| 199 | `anim_sub_escape_intro_badge` | 1 | 60 | 386x66 | Oui | `crates/pin2k-core/src/swe1_rules.rs:149` |
| 200 | `anim_watto_cube_cube` | 22 | 60 | 269x133 | Non | - |
| 201 | `anim_watto_cube` | 70 | 60 | 572x240 | Oui | `crates/pin2k-core/src/swe1_rules.rs:196`, `crates/pin2k-desktop/src/scenes_compositor.rs:90`, `crates/pin2k-desktop/src/scenes_compositor.rs:94` |
| 202 | `anim_watto_cube_flare` | 16 | 60 | 640x240 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:95` |
| 203 | `anim_watto_junkshop_background` | 1 | 60 | 640x166 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:92` |
| 204 | `anim_watto_junkshop_sky` | 1 | 60 | 640x125 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:91` |
| 205 | `anim_watto_fly` | 30 | 60 | 121x63 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:93` |
| 206 | `menu_background_map_playfield` | 1 | 60 | 220x208 | Non | - |
| 207 | `anim_jedi_battle_idle` | 10 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:329`, `crates/pin2k-desktop/src/scenes_compositor.rs:104` |
| 208 | `anim_jedi_battle_icon_quigon` | 11 | 60 | 92x55 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:324`, `crates/pin2k-desktop/src/scenes_compositor.rs:125` |
| 209 | `anim_jedi_battle_icon_maul` | 11 | 60 | 92x55 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:325`, `crates/pin2k-desktop/src/scenes_compositor.rs:126` |
| 210 | `anim_jedi_battle_quigon_high_right_single` | 17 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:523`, `crates/pin2k-desktop/src/scenes_compositor.rs:106` |
| 211 | `anim_jedi_battle_quigon_low_left_single` | 13 | 60 | 396x178 | Non | - |
| 212 | `anim_jedi_battle_quigon_low_right_single` | 17 | 60 | 396x178 | Non | - |
| 213 | `anim_jedi_battle_quigon_middle_right_single` | 17 | 60 | 396x178 | Non | - |
| 214 | `anim_jedi_battle_quigon_4_move` | 40 | 60 | 396x178 | Non | - |
| 215 | `anim_jedi_battle_maul_chop_single` | 17 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:524`, `crates/pin2k-desktop/src/scenes_compositor.rs:107` |
| 216 | `anim_jedi_battle_maul_low_right_single` | 15 | 60 | 396x178 | Non | - |
| 217 | `anim_jedi_battle_quigon_death` | 55 | 60 | 396x178 | Non | - |
| 218 | `anim_jedi_battle_maul_death_1` | 56 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:511`, `crates/pin2k-desktop/src/scenes_compositor.rs:108` |
| 219 | `anim_jedi_battle_maul_death_2` | 28 | 60 | 396x178 | Non | - |
| 220 | `anim_jedi_battle_saber_sparks` | 11 | 60 | 264x148 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:333`, `crates/pin2k-desktop/src/scenes_compositor.rs:129` |
| 221 | `anim_jedi_battle_clash` | 42 | 60 | 396x178 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:332`, `crates/pin2k-core/src/machine/swe1_playfield.rs:522`, `crates/pin2k-desktop/src/scenes_compositor.rs:105`, `crates/pin2k-desktop/src/scenes_compositor.rs:128` |
| 222 | `anim_jedi_battle_background_hangar` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:323`, `crates/pin2k-desktop/src/scenes_compositor.rs:124` |
| 223 | `anim_jedi_battle_background_power` | 1 | 60 | 640x240 | Non | - |
| 224 | `anim_jedi_battle_background_dessert` | 1 | 60 | 640x240 | Non | - |
| 225 | `anim_sith_probe_background` | 1 | 60 | 800x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:166`, `crates/pin2k-desktop/src/scenes_compositor.rs:71` |
| 226 | `anim_sith_probe_background_2` | 1 | 60 | 800x240 | Non | - |
| 227 | `anim_spinner_1` | 56 | 60 | 150x57 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:166`, `crates/pin2k-core/src/machine/tests.rs:426`, `crates/pin2k-core/src/machine/vspinner.rs:119` |
| 228 | `anim_spinner_2` | 56 | 60 | 150x57 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:168`, `crates/pin2k-core/src/machine/tests.rs:442`, `crates/pin2k-core/src/machine/vspinner.rs:121` |
| 229 | `anim_sparks_1` | 15 | 60 | 200x100 | Non | - |
| 230 | `anim_textbox_300` | 30 | 60 | 300x31 | Non | - |
| 231 | `anim_build_c3p0` | 57 | 60 | 115x160 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:222`, `crates/pin2k-core/src/machine/system_screens.rs:336`, `crates/pin2k-core/src/machine/tests.rs:598` |
| 232 | `anim_fashion_sparkle_left` | 30 | 60 | 146x64 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:177` |
| 233 | `anim_fashion_sparkle_right` | 30 | 60 | 146x64 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:178` |
| 234 | `anim_saber_ball_loop` | 79 | 60 | 304x147 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:237`, `crates/pin2k-core/src/machine/system_screens.rs:373`, `crates/pin2k-core/src/machine/tests.rs:600` |
| 235 | `anim_saber_ball_ignite` | 73 | 60 | 640x205 | Non | - |
| 236 | `anim_podrace_intro_movie` | 55 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:132` |
| 237 | `anim_podrace_1` | 100 | 60 | 353x119 | Oui | `crates/pin2k-core/src/swe1_rules.rs:192`, `crates/pin2k-desktop/src/compositor.rs:589`, `crates/pin2k-desktop/src/scenes_compositor.rs:19` |
| 238 | `anim_podrace_2` | 40 | 60 | 147x43 | Oui | `crates/pin2k-desktop/src/compositor.rs:580` |
| 239 | `anim_podrace_icon1` | 21 | 60 | 70x30 | Non | - |
| 240 | `anim_podrace_icon2` | 21 | 60 | 70x30 | Non | - |
| 241 | `anim_podrace_icon3` | 20 | 60 | 70x30 | Non | - |
| 242 | `anim_podrace_icon4` | 10 | 60 | 87x52 | Non | - |
| 243 | `anim_podrace_icon1_blur` | 5 | 60 | 70x30 | Non | - |
| 244 | `anim_podrace_icon2_blur` | 5 | 60 | 70x30 | Non | - |
| 245 | `anim_podrace_icon3_blur` | 5 | 60 | 70x30 | Non | - |
| 246 | `anim_podrace_icon4_blur` | 4 | 60 | 87x52 | Non | - |
| 247 | `anim_podrace_checkpoint` | 1 | 60 | 191x58 | Oui | `crates/pin2k-desktop/src/compositor.rs:592` |
| 248 | `anim_podrace_ground` | 21 | 60 | 640x128 | Oui | `crates/pin2k-desktop/src/compositor.rs:568` |
| 249 | `anim_podrace_closeup` | 31 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:485`, `crates/pin2k-desktop/src/compositor.rs:571`, `crates/pin2k-desktop/src/scenes_compositor.rs:23` |
| 250 | `anim_droid_control_ship` | 1 | 60 | 400x81 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:100`, `crates/pin2k-desktop/src/scenes_compositor.rs:47` |
| 251 | `anim_droid_control_ship2` | 1 | 60 | 150x30 | Non | - |
| 252 | `anim_bonus_character_anakin` | 50 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:393` |
| 253 | `anim_bonus_character_jarjar` | 93 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:392` |
| 254 | `anim_bonus_character_maul1` | 57 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:399` |
| 255 | `anim_bonus_character_obiwan` | 39 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:395` |
| 256 | `anim_bonus_character_queen1` | 45 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:394` |
| 257 | `anim_bonus_character_quigon` | 57 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:391`, `crates/pin2k-core/src/machine/system_screens.rs:120` |
| 258 | `anim_bonus_character_r2d2` | 51 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:396` |
| 259 | `anim_bonus_character_watto` | 53 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:398` |
| 260 | `anim_bonus_character_yoda` | 77 | 15 | 88x76 | Oui | `crates/pin2k-core/src/swe1_rules.rs:397` |
| 261 | `anim_attract_anakin_web` | 1 | 60 | 202x101 | Oui | `crates/pin2k-core/src/attract.rs:77`, `crates/pin2k-core/src/machine/attract_render.rs:259`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:541` |
| 262 | `anim_klogo_zoom_back` | 41 | 60 | 640x240 | Oui | `crates/pin2k-core/src/attract.rs:73`, `crates/pin2k-desktop/src/compositor.rs:244`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:529` |
| 263 | `anim_queen_ship_intro_movie` | 55 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:134` |
| 264 | `anim_darth_maul_game_over` | 30 | 15 | 320x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:264`, `crates/pin2k-core/src/machine/system_screens.rs:212`, `crates/pin2k-core/src/machine/tests.rs:591` |
| 265 | `anim_gungan_seaweed` | 40 | 60 | 139x90 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:106`, `crates/pin2k-core/src/machine/system_screens.rs:320`, `crates/pin2k-desktop/src/scenes_compositor.rs:53` |
| 266 | `anim_gungan_bubbles` | 3 | 60 | 17x9 | Oui | `crates/pin2k-desktop/src/scenes_compositor.rs:54` |
| 267 | `anim_jedi_council_chic` | 1 | 60 | 93x60 | Non | - |
| 268 | `anim_attract_hasbro_ad` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/attract.rs:76`, `crates/pin2k-core/src/machine/attract_render.rs:201`, `crates/pin2k-desktop/src/compositor.rs:245`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:538` |
| 269 | `anim_gungan_sub_bubbles_2` | 29 | 60 | 571x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:107`, `crates/pin2k-core/src/machine/system_screens.rs:321` |
| 270 | `anim_gungan_sub_sando` | 85 | 60 | 397x156 | Non | - |
| 271 | `anim_spinner_lightning` | 22 | 60 | 137x76 | Oui | `crates/pin2k-core/src/machine/vspinner.rs:137` |
| 272 | `anim_podrace_2_explosion` | 26 | 60 | 350x200 | Oui | `crates/pin2k-desktop/src/compositor.rs:577` |
| 273 | `anim_podrace_3` | 11 | 60 | 60x21 | Non | - |
| 274 | `anim_podrace_3_explosion` | 26 | 60 | 115x51 | Non | - |
| 275 | `anim_podrace_4` | 11 | 60 | 65x22 | Non | - |
| 276 | `anim_podrace_anakin_zoom` | 20 | 60 | 228x90 | Non | - |
| 277 | `anim_podrace_starting` | 60 | 15 | 320x240 | Oui | `crates/pin2k-desktop/src/compositor.rs:559` |
| 278 | `anim_bonus_icon_jedi_youth` | 47 | 60 | 140x70 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:113`, `crates/pin2k-core/src/machine/system_screens.rs:113`, `crates/pin2k-core/src/machine/system_screens.rs:120` |
| 279 | `anim_bonus_icon_jedi_knight` | 47 | 60 | 140x70 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:114`, `crates/pin2k-core/src/machine/system_screens.rs:114` |
| 280 | `anim_bonus_icon_jedi_master` | 47 | 60 | 140x70 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:115`, `crates/pin2k-core/src/machine/system_screens.rs:115` |
| 281 | `anim_bonus_icon_jedi_spirit` | 47 | 60 | 140x70 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:116`, `crates/pin2k-core/src/machine/system_screens.rs:116` |
| 282 | `anim_jedi_battle_win_icon_maul` | 1 | 60 | 370x175 | Non | - |
| 283 | `anim_jedi_battle_win_icon_quigon` | 1 | 60 | 384x198 | Non | - |
| 284 | `anim_obiwan_holo_queens_logo` | 72 | 60 | 216x98 | Oui | `crates/pin2k-core/src/swe1_rules.rs:174` |
| 285 | `anim_bonus_character_anakin2` | 50 | 15 | 88x76 | Non | - |
| 286 | `anim_bonus_character_maul2` | 67 | 15 | 88x76 | Non | - |
| 287 | `anim_bonus_character_queen2` | 50 | 15 | 88x76 | Non | - |
| 288 | `anim_drop_target` | 2 | 60 | 293x186 | Non | - |
| 289 | `anim_shield` | 2 | 60 | 196x185 | Oui | `crates/pin2k-core/src/machine/swe1_playfield.rs:189` |
| 290 | `anim_action_button` | 1 | 60 | 211x100 | Non | - |
| 291 | `anim_federation_fighter_redeye` | 7 | 60 | 143x38 | Non | - |
| 292 | `anim_destroyer_droid_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:91`, `crates/pin2k-desktop/src/scenes_compositor.rs:39` |
| 293 | `anim_destroyer_droid_s_roll` | 65 | 60 | 341x77 | Non | - |
| 294 | `anim_destroyer_droid_m_roll` | 65 | 60 | 405x88 | Non | - |
| 295 | `anim_destroyer_droid_l_roll` | 61 | 60 | 609x136 | Oui | `crates/pin2k-core/src/swe1_rules.rs:187`, `crates/pin2k-core/src/machine/scene_render.rs:94`, `crates/pin2k-desktop/src/scenes_compositor.rs:38`, `crates/pin2k-desktop/src/scenes_compositor.rs:42` |
| 296 | `anim_destroyer_droid_s_idle` | 20 | 60 | 115x74 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:92`, `crates/pin2k-desktop/src/scenes_compositor.rs:40` |
| 297 | `anim_destroyer_droid_m_idle` | 19 | 60 | 136x85 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:93`, `crates/pin2k-desktop/src/scenes_compositor.rs:41` |
| 298 | `anim_destroyer_droid_l_idle` | 20 | 60 | 209x132 | Non | - |
| 299 | `anim_destroyer_droid_s_explosion` | 33 | 60 | 284x119 | Non | - |
| 300 | `anim_destroyer_droid_m_explosion` | 34 | 60 | 308x171 | Non | - |
| 301 | `anim_destroyer_droid_l_explosion` | 52 | 60 | 350x155 | Non | - |
| 302 | `anim_destroyer_droid_s_shooting` | 11 | 60 | 276x74 | Non | - |
| 303 | `anim_destroyer_droid_m_shooting` | 11 | 60 | 389x86 | Non | - |
| 304 | `anim_destroyer_droid_l_shooting` | 11 | 60 | 528x132 | Non | - |
| 305 | `anim_destroyer_droid_intro_badge` | 1 | 60 | 426x80 | Oui | `crates/pin2k-core/src/swe1_rules.rs:147` |
| 306 | `anim_destroyer_droid_intro_movie` | 67 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:127` |
| 307 | `anim_jedi_battle_quigon_6_move` | 57 | 60 | 396x178 | Non | - |
| 308 | `anim_jedi_battle_maul_5_move` | 43 | 60 | 396x178 | Non | - |
| 309 | `anim_jedi_battle_maul_back_block_3` | 36 | 60 | 396x178 | Non | - |
| 310 | `anim_match_movie` | 175 | 15 | 320x200 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:256`, `crates/pin2k-core/src/machine/system_screens.rs:168`, `crates/pin2k-core/src/machine/tests.rs:590` |
| 311 | `anim_hover_tank_intro_badge` | 1 | 60 | 407x53 | Oui | `crates/pin2k-core/src/swe1_rules.rs:151` |
| 312 | `anim_hover_tank_intro_movie` | 69 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:131` |
| 313 | `anim_hover_tank_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:131` |
| 314 | `anim_hover_tank_l_enter` | 25 | 60 | 288x120 | Non | - |
| 315 | `anim_hover_tank_l_float` | 10 | 60 | 126x56 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:132` |
| 316 | `anim_hover_tank_l_explode` | 18 | 60 | 400x150 | Non | - |
| 317 | `anim_hover_tank_l_shoot` | 10 | 60 | 288x117 | Non | - |
| 318 | `anim_hover_tank_c_enter` | 25 | 60 | 246x162 | Non | - |
| 319 | `anim_hover_tank_c_float` | 10 | 60 | 174x79 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:134` |
| 320 | `anim_hover_tank_c_explode` | 29 | 60 | 400x150 | Non | - |
| 321 | `anim_hover_tank_c_shoot` | 10 | 60 | 200x102 | Non | - |
| 322 | `anim_hover_tank_r_enter` | 25 | 60 | 267x122 | Non | - |
| 323 | `anim_hover_tank_r_float` | 10 | 60 | 134x57 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:133` |
| 324 | `anim_hover_tank_r_explode` | 30 | 60 | 400x150 | Non | - |
| 325 | `anim_hover_tank_r_shoot` | 10 | 60 | 399x116 | Non | - |
| 326 | `anim_hover_tank_catapult_idle` | 26 | 60 | 175x87 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:135` |
| 327 | `anim_hover_tank_catapult_shoot` | 26 | 60 | 175x87 | Non | - |
| 328 | `anim_hover_tank_boomer_ball` | 16 | 60 | 70x35 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:136` |
| 329 | `anim_jedi_spirit_sidious` | 40 | 60 | 246x110 | Non | - |
| 330 | `anim_jedi_spirit_jedi` | 110 | 60 | 640x221 | Non | - |
| 331 | `anim_jedi_spirit_electro1` | 17 | 60 | 191x147 | Non | - |
| 332 | `anim_jedi_spirit_electro2` | 17 | 60 | 207x147 | Non | - |
| 333 | `anim_jedi_spirit_sparks` | 18 | 60 | 357x143 | Non | - |
| 334 | `anim_jedi_spirit_maul` | 40 | 60 | 246x110 | Non | - |
| 335 | `anim_wattos_chance_intro_badge` | 1 | 60 | 410x95 | Oui | `crates/pin2k-core/src/swe1_rules.rs:156` |
| 336 | `anim_wattos_chance_intro_movie` | 71 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:136` |
| 337 | `anim_question_mark_intro_badge` | 1 | 60 | 446x97 | Oui | `crates/pin2k-core/src/swe1_rules.rs:159` |
| 338 | `anim_question_mark_intro_movie` | 77 | 15 | 220x160 | Oui | `crates/pin2k-core/src/swe1_rules.rs:139` |
| 339 | `anim_firework_2` | 60 | 60 | 284x240 | Non | - |
| 340 | `anim_jackpot_icon_double` | 60 | 60 | 161x62 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:299` |
| 341 | `anim_jackpot_icon_super` | 60 | 60 | 161x62 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:297` |
| 342 | `anim_question_mark_green` | 60 | 60 | 122x62 | Oui | `crates/pin2k-core/src/swe1_rules.rs:199`, `crates/pin2k-core/src/machine/scene_render.rs:249` |
| 343 | `anim_question_mark_blue` | 60 | 60 | 122x62 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:248` |
| 344 | `anim_watto_scene_background` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:197` |
| 345 | `anim_watto_scene_left_loop` | 30 | 60 | 203x147 | Non | - |
| 346 | `anim_watto_scene_left_to_middle` | 10 | 60 | 384x160 | Non | - |
| 347 | `anim_watto_scene_hand_to_chin_middle` | 15 | 60 | 169x164 | Non | - |
| 348 | `anim_watto_scene_chin_loop_middle` | 31 | 60 | 173x162 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:198` |
| 349 | `anim_watto_scene_hand_down_middle` | 14 | 60 | 170x166 | Non | - |
| 350 | `anim_watto_scene_middle_to_right` | 14 | 60 | 396x153 | Non | - |
| 351 | `anim_watto_scene_right_loop` | 32 | 60 | 184x158 | Non | - |
| 352 | `anim_watto_scene_cube` | 30 | 60 | 126x64 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:199` |
| 353 | `anim_watto_scene_cube_flash` | 13 | 60 | 188x89 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:200` |
| 354 | `anim_watto_scene_cube_explode` | 30 | 60 | 256x118 | Non | - |
| 355 | `anim_yoda_big` | 57 | 15 | 288x236 | Non | - |
| 356 | `anim_obiwan_holo_destroyer` | 71 | 60 | 199x109 | Oui | `crates/pin2k-core/src/swe1_rules.rs:167` |
| 357 | `anim_obiwan_holo_hovertank` | 71 | 60 | 325x95 | Oui | `crates/pin2k-core/src/swe1_rules.rs:171` |
| 358 | `anim_obiwan_holo_question` | 72 | 60 | 200x100 | Oui | `crates/pin2k-core/src/swe1_rules.rs:179` |
| 359 | `anim_obiwan_holo_watto` | 71 | 60 | 178x104 | Oui | `crates/pin2k-core/src/swe1_rules.rs:176` |
| 360 | `anim_jedi_battle_background_hangar_movie` | 29 | 15 | 320x240 | Non | - |
| 361 | `anim_arrow_down` | 15 | 60 | 150x75 | Non | - |
| 362 | `anim_arrow_hard_right` | 10 | 60 | 100x50 | Non | - |
| 363 | `anim_arrow_soft_right` | 10 | 60 | 81x60 | Non | - |
| 364 | `anim_shield_block` | 15 | 60 | 200x25 | Non | - |
| 365 | `anim_shield_block_wipe` | 9 | 60 | 200x25 | Non | - |
| 366 | `anim_jedi_battle_quigon_force` | 39 | 60 | 396x178 | Non | - |
| 367 | `anim_super_skill_shot_icon` | 60 | 60 | 61x31 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:404`, `crates/pin2k-core/src/machine/scene_render.rs:406`, `crates/pin2k-core/src/machine/script_exec.rs:165` |
| 368 | `anim_super_mystery_icon` | 60 | 60 | 91x31 | Non | - |
| 369 | `anim_bonus_icon` | 60 | 60 | 91x32 | Non | - |
| 370 | `anim_green_icon_boom_big` | 9 | 60 | 354x110 | Non | - |
| 371 | `anim_blue_icon_boom_big` | 9 | 60 | 354x110 | Non | - |
| 372 | `anim_blue_icon_boom_small` | 10 | 60 | 158x81 | Non | - |
| 373 | `anim_bumper_bubbles_1` | 33 | 60 | 235x122 | Non | - |
| 374 | `anim_bumper_bubbles_2` | 41 | 60 | 157x121 | Non | - |
| 375 | `anim_jarjar_thumbs_up` | 26 | 60 | 115x60 | Non | - |
| 376 | `anim_spinner_bracket_left` | 1 | 60 | 150x57 | Oui | `crates/pin2k-core/src/machine/tests.rs:423`, `crates/pin2k-core/src/machine/vspinner.rs:114` |
| 377 | `anim_spinner_bracket_right` | 1 | 60 | 150x57 | Oui | `crates/pin2k-core/src/machine/tests.rs:439`, `crates/pin2k-core/src/machine/vspinner.rs:116` |
| 378 | `anim_spinner_explosion` | 15 | 60 | 150x75 | Non | - |
| 379 | `anim_queen_escape_misc_movie` | 28 | 15 | 320x140 | Non | - |
| 380 | `anim_jedi_stand_misc_movie` | 33 | 15 | 320x140 | Non | - |
| 381 | `anim_oola_girls` | 1 | 60 | 272x182 | Non | - |
| 382 | `anim_rayboot` | 1 | 60 | 255x148 | Non | - |
| 383 | `anim_jedi_battle_background_dessert_maul` | 1 | 60 | 640x240 | Non | - |
| 384 | `anim_flipinfo_d` | 1 | 60 | 180x142 | Non | - |
| 385 | `anim_flipinfo_e` | 1 | 60 | 640x240 | Non | - |
| 386 | `system_video_test_align` | 1 | 60 | 640x240 | Oui | `crates/pin2k-core/src/machine/script_exec.rs:354`, `crates/pin2k-core/src/machine/system_screens.rs:518`, `crates/pin2k-core/src/machine/tests.rs:668` |
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
| 1 | `font_basic_19` | 94 | Oui | `crates/pin2k-desktop/src/compositor.rs:626` |
| 2 | `font_kahuna_shadow_9` | 95 | Oui | `crates/pin2k-core/src/machine/score_display.rs:95`, `crates/pin2k-core/src/machine/score_display.rs:119`, `crates/pin2k-core/src/machine/system_screens.rs:325`, `crates/pin2k-desktop/src/compositor.rs:627` |
| 3 | `font_shadow_7` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:202`, `crates/pin2k-desktop/src/compositor.rs:628` |
| 4 | `font_shadow_7_narrow` | 95 | Oui | `crates/pin2k-core/src/machine/system_screens.rs:348`, `crates/pin2k-desktop/src/compositor.rs:629` |
| 5 | `font_shadow_9` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:220`, `crates/pin2k-core/src/machine/system_screens.rs:137`, `crates/pin2k-core/src/machine/system_screens.rs:147`, `crates/pin2k-core/src/machine/system_screens.rs:247`, `crates/pin2k-desktop/src/compositor.rs:617` (+1) |
| 6 | `font_shadow_9_narrow` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:111`, `crates/pin2k-core/src/machine/scene_render.rs:356`, `crates/pin2k-core/src/machine/scene_render.rs:422`, `crates/pin2k-desktop/src/compositor.rs:631` |
| 7 | `font_shadow_11` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:64`, `crates/pin2k-core/src/machine/scene_render.rs:144`, `crates/pin2k-core/src/machine/scene_render.rs:154`, `crates/pin2k-core/src/machine/scene_render.rs:261`, `crates/pin2k-core/src/machine/scene_render.rs:408` (+4) |
| 8 | `font_shadow_11_narrow` | 95 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:141`, `crates/pin2k-core/src/machine/attract_render.rs:153`, `crates/pin2k-core/src/machine/attract_render.rs:225`, `crates/pin2k-core/src/machine/attract_render.rs:247`, `crates/pin2k-core/src/machine/attract_render.rs:266` (+13) |
| 9 | `font_shadow_15` | 95 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:87`, `crates/pin2k-core/src/machine/attract_render.rs:97`, `crates/pin2k-core/src/machine/attract_render.rs:170`, `crates/pin2k-core/src/machine/attract_render.rs:205`, `crates/pin2k-core/src/machine/attract_render.rs:215` (+13) |
| 10 | `font_shadow_15_narrow` | 95 | Oui | `crates/pin2k-core/src/machine/system_screens.rs:375`, `crates/pin2k-desktop/src/compositor.rs:612`, `crates/pin2k-desktop/src/compositor.rs:635` |
| 11 | `font_shadow_22` | 95 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:286`, `crates/pin2k-desktop/src/compositor.rs:636` |
| 12 | `font_starwars_19` | 94 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:74`, `crates/pin2k-core/src/machine/attract_render.rs:123`, `crates/pin2k-core/src/machine/attract_render.rs:180`, `crates/pin2k-core/src/machine/attract_render.rs:237`, `crates/pin2k-core/src/machine/system_screens.rs:185` (+8) |
| 13 | `font_system_7` | 95 | Oui | `crates/pin2k-core/src/machine/hud_display.rs:50`, `crates/pin2k-core/src/machine/hud_display.rs:60`, `crates/pin2k-core/src/machine/scene_render.rs:379`, `crates/pin2k-core/src/machine/scene_render.rs:389`, `crates/pin2k-core/src/machine/score_display.rs:84` (+13) |
| 14 | `font_system_9` | 95 | Oui | `crates/pin2k-core/src/machine/attract_render.rs:415`, `crates/pin2k-core/src/machine/attract_render.rs:425`, `crates/pin2k-core/src/machine/attract_render.rs:441`, `crates/pin2k-core/src/machine/system_screens.rs:78`, `crates/pin2k-core/src/machine/system_screens.rs:492` (+6) |
| 15 | `font_system_10` | 95 | Oui | `crates/pin2k-desktop/src/compositor.rs:640` |
| 16 | `font_system_11` | 105 | Oui | `crates/pin2k-core/src/machine/scene_render.rs:186`, `crates/pin2k-core/src/machine/system_screens.rs:68`, `crates/pin2k-core/src/machine/system_screens.rs:283`, `crates/pin2k-core/src/machine/system_screens.rs:647`, `crates/pin2k-core/src/machine/system_screens.rs:681` (+2) |
| 17 | `font_system_15` | 95 | Oui | `crates/pin2k-core/src/machine/system_screens.rs:58`, `crates/pin2k-desktop/src/compositor.rs:642` |
| 18 | `font_system_20` | 95 | Oui | `crates/pin2k-desktop/src/compositor.rs:643` |
| 19 | `font_system_fxd6x7` | 95 | Oui | `crates/pin2k-desktop/src/compositor.rs:644` |

---

## 3. Pistes Audio DCS-2 (689 entrees)

| # | Nom | DCS ID | Mode Loop | Fichier | Utilise | Emplacement(s) dans le code |
|---:|---|---|---|---|---|---|
| 1 | `dcs-bong` | `0x003A` | OneShot | `dcs-bong.ogg` | Oui | `crates/pin2k-core/src/admin.rs:141`, `crates/pin2k-core/src/rfm_tables.rs:111`, `crates/pin2k-core/src/swe1_tables.rs:163`, `crates/pin2k-rom-tool/src/asset_extractor.rs:586`, `crates/pin2k-rom-tool/src/asset_extractor.rs:587` (+2) |
| 2 | `S0001-LP1` | `0x0001` | LoopIntro | `S0001-LP1.ogg` | Oui | `crates/pin2k-hal/src/serial.rs:331`, `crates/pin2k-core/src/admin.rs:142`, `crates/pin2k-core/src/rfm_tables.rs:112`, `crates/pin2k-core/src/swe1_tables.rs:164`, `crates/pin2k-core/src/swe1_tables.rs:174` (+4) |
| 3 | `S0001-LP2` | `0x0001` | LoopBody | `S0001-LP2.ogg` | Oui | `crates/pin2k-hal/src/serial.rs:331`, `crates/pin2k-core/src/admin.rs:142`, `crates/pin2k-core/src/rfm_tables.rs:112`, `crates/pin2k-core/src/swe1_tables.rs:164`, `crates/pin2k-core/src/swe1_tables.rs:174` (+3) |
| 4 | `S0002-LP` | `0x0002` | SingleLoop | `S0002-LP.ogg` | Oui | `crates/pin2k-core/src/rfm_tables.rs:113`, `crates/pin2k-core/src/swe1_tables.rs:165`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:614` |
| 5 | `S0003-LP` | `0x0003` | SingleLoop | `S0003-LP.ogg` | Non | - |
| 6 | `S0004` | `0x0004` | OneShot | `S0004.ogg` | Non | - |
| 7 | `S0006-LP` | `0x0006` | SingleLoop | `S0006-LP.ogg` | Non | - |
| 8 | `S0007-LP` | `0x0007` | SingleLoop | `S0007-LP.ogg` | Oui | `crates/pin2k-core/src/rfm_tables.rs:114` |
| 9 | `S0009-LP` | `0x0009` | SingleLoop | `S0009-LP.ogg` | Oui | `crates/pin2k-core/src/admin.rs:144`, `crates/pin2k-core/src/swe1_tables.rs:166` |
| 10 | `S000A-LP` | `0x000A` | SingleLoop | `S000A-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:167` |
| 11 | `S000B-LP1` | `0x000B` | LoopIntro | `S000B-LP1.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:175` |
| 12 | `S000B-LP2` | `0x000B` | LoopBody | `S000B-LP2.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:175` |
| 13 | `S000C-LP` | `0x000C` | SingleLoop | `S000C-LP.ogg` | Non | - |
| 14 | `S000D` | `0x000D` | OneShot | `S000D.ogg` | Oui | `crates/pin2k-core/src/rfm_tables.rs:115`, `crates/pin2k-core/src/swe1_tables.rs:184` |
| 15 | `S000E-LP` | `0x000E` | SingleLoop | `S000E-LP.ogg` | Oui | `crates/pin2k-core/src/admin.rs:145`, `crates/pin2k-core/src/swe1_tables.rs:168`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:597` |
| 16 | `S000F-LP1` | `0x000F` | LoopIntro | `S000F-LP1.ogg` | Non | - |
| 17 | `S000F-LP2` | `0x000F` | LoopBody | `S000F-LP2.ogg` | Non | - |
| 18 | `S0010-LP` | `0x0010` | SingleLoop | `S0010-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:169`, `crates/pin2k-core/src/machine/system_screens.rs:732`, `crates/pin2k-rom-tool/src/cli.rs:232`, `crates/pin2k-rom-tool/src/commands.rs:569`, `crates/pin2k-rom-tool/src/commands.rs:635` (+9) |
| 19 | `S0011-LP` | `0x0011` | SingleLoop | `S0011-LP.ogg` | Non | - |
| 20 | `S0012-LP` | `0x0012` | SingleLoop | `S0012-LP.ogg` | Non | - |
| 21 | `S0013-LP` | `0x0013` | SingleLoop | `S0013-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:170` |
| 22 | `S0014` | `0x0014` | OneShot | `S0014.ogg` | Non | - |
| 23 | `S0015` | `0x0015` | OneShot | `S0015.ogg` | Oui | `crates/pin2k-core/src/rfm_tables.rs:116`, `crates/pin2k-core/src/machine/attract_render.rs:129`, `crates/pin2k-core/src/machine/attract_render.rs:231`, `crates/pin2k-core/src/machine/tests.rs:233` |
| 24 | `S0016` | `0x0016` | OneShot | `S0016.ogg` | Non | - |
| 25 | `S0018` | `0x0018` | OneShot | `S0018.ogg` | Non | - |
| 26 | `S0019` | `0x0019` | OneShot | `S0019.ogg` | Non | - |
| 27 | `S001A` | `0x001A` | OneShot | `S001A.ogg` | Non | - |
| 28 | `S001C` | `0x001C` | OneShot | `S001C.ogg` | Non | - |
| 29 | `S001E-LP` | `0x001E` | SingleLoop | `S001E-LP.ogg` | Non | - |
| 30 | `S001F-LP1` | `0x001F` | LoopIntro | `S001F-LP1.ogg` | Oui | `crates/pin2k-core/src/machine/attract_render.rs:18`, `crates/pin2k-core/src/machine/scene_render.rs:257`, `crates/pin2k-core/src/machine/system_screens.rs:266`, `crates/pin2k-core/src/machine/system_screens.rs:523`, `crates/pin2k-core/src/machine/system_screens.rs:569` (+3) |
| 31 | `S001F-LP2` | `0x001F` | LoopBody | `S001F-LP2.ogg` | Oui | `crates/pin2k-core/src/machine/attract_render.rs:18`, `crates/pin2k-core/src/machine/scene_render.rs:257`, `crates/pin2k-core/src/machine/system_screens.rs:266`, `crates/pin2k-core/src/machine/system_screens.rs:523`, `crates/pin2k-core/src/machine/system_screens.rs:569` (+3) |
| 32 | `S0021-LP` | `0x0021` | SingleLoop | `S0021-LP.ogg` | Oui | `crates/pin2k-rom-tool/src/gamecode_extractor.rs:634` |
| 33 | `S0022-LP` | `0x0022` | SingleLoop | `S0022-LP.ogg` | Oui | `crates/pin2k-core/src/admin.rs:143`, `crates/pin2k-core/src/swe1_tables.rs:171` |
| 34 | `S0023-LP` | `0x0023` | SingleLoop | `S0023-LP.ogg` | Non | - |
| 35 | `S0024-LP` | `0x0024` | SingleLoop | `S0024-LP.ogg` | Non | - |
| 36 | `S0025-LP` | `0x0025` | SingleLoop | `S0025-LP.ogg` | Non | - |
| 37 | `S0026-LP` | `0x0026` | SingleLoop | `S0026-LP.ogg` | Non | - |
| 38 | `S0027` | `0x0027` | OneShot | `S0027.ogg` | Non | - |
| 39 | `S002D-LP` | `0x002D` | SingleLoop | `S002D-LP.ogg` | Oui | `crates/pin2k-core/src/admin.rs:146`, `crates/pin2k-core/src/swe1_tables.rs:172`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:584`, `crates/pin2k-rom-tool/src/layout_extractor.rs:673` |
| 40 | `S002E-LP` | `0x002E` | SingleLoop | `S002E-LP.ogg` | Non | - |
| 41 | `S002F-LP` | `0x002F` | SingleLoop | `S002F-LP.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:189` |
| 42 | `S0030` | `0x0030` | OneShot | `S0030.ogg` | Non | - |
| 43 | `S0031-LP1` | `0x0031` | LoopIntro | `S0031-LP1.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:173` |
| 44 | `S0031-LP2` | `0x0031` | LoopBody | `S0031-LP2.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:173` |
| 45 | `S0032-LP` | `0x0032` | SingleLoop | `S0032-LP.ogg` | Non | - |
| 46 | `S0033` | `0x0033` | OneShot | `S0033.ogg` | Non | - |
| 47 | `S005F` | `0x005F` | OneShot | `S005F.ogg` | Non | - |
| 48 | `S0060` | `0x0060` | OneShot | `S0060.ogg` | Oui | `crates/pin2k-rom-tool/src/gamecode_extractor.rs:702` |
| 49 | `S0061` | `0x0061` | OneShot | `S0061.ogg` | Oui | `crates/pin2k-rom-tool/src/gamecode_extractor.rs:713` |
| 50 | `S0062` | `0x0062` | OneShot | `S0062.ogg` | Non | - |
| 51 | `S0063` | `0x0063` | OneShot | `S0063.ogg` | Non | - |
| 52 | `S0070` | `0x0070` | OneShot | `S0070.ogg` | Non | - |
| 53 | `S007C` | `0x007C` | OneShot | `S007C.ogg` | Non | - |
| 54 | `S0081` | `0x0081` | OneShot | `S0081.ogg` | Non | - |
| 55 | `S0082` | `0x0082` | OneShot | `S0082.ogg` | Non | - |
| 56 | `S0083` | `0x0083` | OneShot | `S0083.ogg` | Non | - |
| 57 | `S0084` | `0x0084` | OneShot | `S0084.ogg` | Non | - |
| 58 | `S0085` | `0x0085` | OneShot | `S0085.ogg` | Non | - |
| 59 | `S0086` | `0x0086` | OneShot | `S0086.ogg` | Non | - |
| 60 | `S0087` | `0x0087` | OneShot | `S0087.ogg` | Non | - |
| 61 | `S0088` | `0x0088` | OneShot | `S0088.ogg` | Non | - |
| 62 | `S0089` | `0x0089` | OneShot | `S0089.ogg` | Non | - |
| 63 | `S008A` | `0x008A` | OneShot | `S008A.ogg` | Non | - |
| 64 | `S00E1` | `0x00E1` | OneShot | `S00E1.ogg` | Oui | `crates/pin2k-rom-tool/src/asset_extractor.rs:618` |
| 65 | `S00EC` | `0x00EC` | OneShot | `S00EC.ogg` | Non | - |
| 66 | `S00F9` | `0x00F9` | OneShot | `S00F9.ogg` | Non | - |
| 67 | `S0111` | `0x0111` | OneShot | `S0111.ogg` | Non | - |
| 68 | `S0190` | `0x0190` | OneShot | `S0190.ogg` | Non | - |
| 69 | `S0191` | `0x0191` | OneShot | `S0191.ogg` | Non | - |
| 70 | `S0192` | `0x0192` | OneShot | `S0192.ogg` | Non | - |
| 71 | `S0193` | `0x0193` | OneShot | `S0193.ogg` | Non | - |
| 72 | `S0194` | `0x0194` | OneShot | `S0194.ogg` | Non | - |
| 73 | `S0195` | `0x0195` | OneShot | `S0195.ogg` | Non | - |
| 74 | `S0196` | `0x0196` | OneShot | `S0196.ogg` | Non | - |
| 75 | `S0197` | `0x0197` | OneShot | `S0197.ogg` | Non | - |
| 76 | `S0198` | `0x0198` | OneShot | `S0198.ogg` | Non | - |
| 77 | `S0199` | `0x0199` | OneShot | `S0199.ogg` | Non | - |
| 78 | `S019A` | `0x019A` | OneShot | `S019A.ogg` | Non | - |
| 79 | `S019B` | `0x019B` | OneShot | `S019B.ogg` | Non | - |
| 80 | `S019C` | `0x019C` | OneShot | `S019C.ogg` | Non | - |
| 81 | `S019D` | `0x019D` | OneShot | `S019D.ogg` | Non | - |
| 82 | `S019E` | `0x019E` | OneShot | `S019E.ogg` | Non | - |
| 83 | `S019F` | `0x019F` | OneShot | `S019F.ogg` | Oui | `crates/pin2k-core/src/machine/system_screens.rs:456`, `crates/pin2k-core/src/machine/system_screens.rs:704`, `crates/pin2k-core/src/machine/system_screens.rs:733` |
| 84 | `S01A0` | `0x01A0` | OneShot | `S01A0.ogg` | Non | - |
| 85 | `S01A1` | `0x01A1` | OneShot | `S01A1.ogg` | Non | - |
| 86 | `S01A2` | `0x01A2` | OneShot | `S01A2.ogg` | Non | - |
| 87 | `S01A3` | `0x01A3` | OneShot | `S01A3.ogg` | Non | - |
| 88 | `S01A4` | `0x01A4` | OneShot | `S01A4.ogg` | Non | - |
| 89 | `S01A6` | `0x01A6` | OneShot | `S01A6.ogg` | Non | - |
| 90 | `S01A7` | `0x01A7` | OneShot | `S01A7.ogg` | Non | - |
| 91 | `S01A8` | `0x01A8` | OneShot | `S01A8.ogg` | Non | - |
| 92 | `S01A9` | `0x01A9` | OneShot | `S01A9.ogg` | Non | - |
| 93 | `S01AA` | `0x01AA` | OneShot | `S01AA.ogg` | Non | - |
| 94 | `S01AB` | `0x01AB` | OneShot | `S01AB.ogg` | Non | - |
| 95 | `S01AC` | `0x01AC` | OneShot | `S01AC.ogg` | Non | - |
| 96 | `S01AD` | `0x01AD` | OneShot | `S01AD.ogg` | Non | - |
| 97 | `S01AE` | `0x01AE` | OneShot | `S01AE.ogg` | Non | - |
| 98 | `S01AF` | `0x01AF` | OneShot | `S01AF.ogg` | Non | - |
| 99 | `S01B0` | `0x01B0` | OneShot | `S01B0.ogg` | Non | - |
| 100 | `S01B1` | `0x01B1` | OneShot | `S01B1.ogg` | Non | - |
| 101 | `S01B2` | `0x01B2` | OneShot | `S01B2.ogg` | Non | - |
| 102 | `S01B3` | `0x01B3` | OneShot | `S01B3.ogg` | Non | - |
| 103 | `S01B4` | `0x01B4` | OneShot | `S01B4.ogg` | Non | - |
| 104 | `S01B5` | `0x01B5` | OneShot | `S01B5.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:183` |
| 105 | `S01B6` | `0x01B6` | OneShot | `S01B6.ogg` | Non | - |
| 106 | `S01B7` | `0x01B7` | OneShot | `S01B7.ogg` | Non | - |
| 107 | `S01B8` | `0x01B8` | OneShot | `S01B8.ogg` | Non | - |
| 108 | `S01B9` | `0x01B9` | OneShot | `S01B9.ogg` | Non | - |
| 109 | `S01BA` | `0x01BA` | OneShot | `S01BA.ogg` | Non | - |
| 110 | `S01BB` | `0x01BB` | OneShot | `S01BB.ogg` | Non | - |
| 111 | `S01BC` | `0x01BC` | OneShot | `S01BC.ogg` | Non | - |
| 112 | `S01BD` | `0x01BD` | OneShot | `S01BD.ogg` | Non | - |
| 113 | `S01BF` | `0x01BF` | OneShot | `S01BF.ogg` | Non | - |
| 114 | `S01C0` | `0x01C0` | OneShot | `S01C0.ogg` | Non | - |
| 115 | `S01C1` | `0x01C1` | OneShot | `S01C1.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:185` |
| 116 | `S01C2` | `0x01C2` | OneShot | `S01C2.ogg` | Non | - |
| 117 | `S01C3` | `0x01C3` | OneShot | `S01C3.ogg` | Non | - |
| 118 | `S01C5` | `0x01C5` | OneShot | `S01C5.ogg` | Non | - |
| 119 | `S01C7` | `0x01C7` | OneShot | `S01C7.ogg` | Non | - |
| 120 | `S01C8` | `0x01C8` | OneShot | `S01C8.ogg` | Non | - |
| 121 | `S01C9` | `0x01C9` | OneShot | `S01C9.ogg` | Non | - |
| 122 | `S01CA` | `0x01CA` | OneShot | `S01CA.ogg` | Non | - |
| 123 | `S01CB` | `0x01CB` | OneShot | `S01CB.ogg` | Non | - |
| 124 | `S01CC` | `0x01CC` | OneShot | `S01CC.ogg` | Non | - |
| 125 | `S01CD` | `0x01CD` | OneShot | `S01CD.ogg` | Non | - |
| 126 | `S01CE` | `0x01CE` | OneShot | `S01CE.ogg` | Non | - |
| 127 | `S01CF` | `0x01CF` | OneShot | `S01CF.ogg` | Non | - |
| 128 | `S01D0` | `0x01D0` | OneShot | `S01D0.ogg` | Non | - |
| 129 | `S01D1` | `0x01D1` | OneShot | `S01D1.ogg` | Non | - |
| 130 | `S01D5` | `0x01D5` | OneShot | `S01D5.ogg` | Non | - |
| 131 | `S01D6` | `0x01D6` | OneShot | `S01D6.ogg` | Non | - |
| 132 | `S01D7` | `0x01D7` | OneShot | `S01D7.ogg` | Oui | `crates/pin2k-rom-tool/src/prism.rs:135` |
| 133 | `S01D8` | `0x01D8` | OneShot | `S01D8.ogg` | Non | - |
| 134 | `S01D9` | `0x01D9` | OneShot | `S01D9.ogg` | Oui | `crates/pin2k-rom-tool/src/prism.rs:25` |
| 135 | `S01DA` | `0x01DA` | OneShot | `S01DA.ogg` | Non | - |
| 136 | `S01DB` | `0x01DB` | OneShot | `S01DB.ogg` | Non | - |
| 137 | `S01DC` | `0x01DC` | OneShot | `S01DC.ogg` | Non | - |
| 138 | `S01DD` | `0x01DD` | OneShot | `S01DD.ogg` | Non | - |
| 139 | `S01DE` | `0x01DE` | OneShot | `S01DE.ogg` | Non | - |
| 140 | `S01DF` | `0x01DF` | OneShot | `S01DF.ogg` | Non | - |
| 141 | `S01E0` | `0x01E0` | OneShot | `S01E0.ogg` | Non | - |
| 142 | `S01E1` | `0x01E1` | OneShot | `S01E1.ogg` | Non | - |
| 143 | `S01E2` | `0x01E2` | OneShot | `S01E2.ogg` | Non | - |
| 144 | `S01E3` | `0x01E3` | OneShot | `S01E3.ogg` | Non | - |
| 145 | `S01E4` | `0x01E4` | OneShot | `S01E4.ogg` | Non | - |
| 146 | `S01E5` | `0x01E5` | OneShot | `S01E5.ogg` | Non | - |
| 147 | `S01E6` | `0x01E6` | OneShot | `S01E6.ogg` | Non | - |
| 148 | `S01E7` | `0x01E7` | OneShot | `S01E7.ogg` | Non | - |
| 149 | `S01E8` | `0x01E8` | OneShot | `S01E8.ogg` | Non | - |
| 150 | `S01E9` | `0x01E9` | OneShot | `S01E9.ogg` | Non | - |
| 151 | `S01EA` | `0x01EA` | OneShot | `S01EA.ogg` | Non | - |
| 152 | `S01EB` | `0x01EB` | OneShot | `S01EB.ogg` | Non | - |
| 153 | `S01EC` | `0x01EC` | OneShot | `S01EC.ogg` | Non | - |
| 154 | `S01ED` | `0x01ED` | OneShot | `S01ED.ogg` | Non | - |
| 155 | `S01EE` | `0x01EE` | OneShot | `S01EE.ogg` | Non | - |
| 156 | `S01EF` | `0x01EF` | OneShot | `S01EF.ogg` | Non | - |
| 157 | `S01F0` | `0x01F0` | OneShot | `S01F0.ogg` | Non | - |
| 158 | `S01F3` | `0x01F3` | OneShot | `S01F3.ogg` | Non | - |
| 159 | `S01F4` | `0x01F4` | OneShot | `S01F4.ogg` | Non | - |
| 160 | `S01F5` | `0x01F5` | OneShot | `S01F5.ogg` | Non | - |
| 161 | `S01F6` | `0x01F6` | OneShot | `S01F6.ogg` | Non | - |
| 162 | `S01F7` | `0x01F7` | OneShot | `S01F7.ogg` | Non | - |
| 163 | `S01F8` | `0x01F8` | OneShot | `S01F8.ogg` | Non | - |
| 164 | `S01F9` | `0x01F9` | OneShot | `S01F9.ogg` | Non | - |
| 165 | `S01FA` | `0x01FA` | OneShot | `S01FA.ogg` | Non | - |
| 166 | `S01FB` | `0x01FB` | OneShot | `S01FB.ogg` | Non | - |
| 167 | `S01FC` | `0x01FC` | OneShot | `S01FC.ogg` | Non | - |
| 168 | `S01FD` | `0x01FD` | OneShot | `S01FD.ogg` | Non | - |
| 169 | `S01FE` | `0x01FE` | OneShot | `S01FE.ogg` | Non | - |
| 170 | `S0203` | `0x0203` | OneShot | `S0203.ogg` | Non | - |
| 171 | `S0204` | `0x0204` | OneShot | `S0204.ogg` | Non | - |
| 172 | `S0205` | `0x0205` | OneShot | `S0205.ogg` | Non | - |
| 173 | `S0206` | `0x0206` | OneShot | `S0206.ogg` | Non | - |
| 174 | `S0207` | `0x0207` | OneShot | `S0207.ogg` | Non | - |
| 175 | `S0208` | `0x0208` | OneShot | `S0208.ogg` | Non | - |
| 176 | `S0209` | `0x0209` | OneShot | `S0209.ogg` | Non | - |
| 177 | `S020A` | `0x020A` | OneShot | `S020A.ogg` | Non | - |
| 178 | `S020B` | `0x020B` | OneShot | `S020B.ogg` | Non | - |
| 179 | `S020C` | `0x020C` | OneShot | `S020C.ogg` | Non | - |
| 180 | `S020D` | `0x020D` | OneShot | `S020D.ogg` | Non | - |
| 181 | `S020E` | `0x020E` | OneShot | `S020E.ogg` | Non | - |
| 182 | `S020F` | `0x020F` | OneShot | `S020F.ogg` | Non | - |
| 183 | `S0210` | `0x0210` | OneShot | `S0210.ogg` | Non | - |
| 184 | `S0212` | `0x0212` | OneShot | `S0212.ogg` | Non | - |
| 185 | `S0213` | `0x0213` | OneShot | `S0213.ogg` | Non | - |
| 186 | `S0214` | `0x0214` | OneShot | `S0214.ogg` | Non | - |
| 187 | `S0216` | `0x0216` | OneShot | `S0216.ogg` | Non | - |
| 188 | `S0217` | `0x0217` | OneShot | `S0217.ogg` | Non | - |
| 189 | `S0218` | `0x0218` | OneShot | `S0218.ogg` | Non | - |
| 190 | `S0219` | `0x0219` | OneShot | `S0219.ogg` | Non | - |
| 191 | `S021A` | `0x021A` | OneShot | `S021A.ogg` | Non | - |
| 192 | `S021B` | `0x021B` | OneShot | `S021B.ogg` | Non | - |
| 193 | `S021C` | `0x021C` | OneShot | `S021C.ogg` | Non | - |
| 194 | `S021D` | `0x021D` | OneShot | `S021D.ogg` | Non | - |
| 195 | `S021E` | `0x021E` | OneShot | `S021E.ogg` | Non | - |
| 196 | `S021F` | `0x021F` | OneShot | `S021F.ogg` | Non | - |
| 197 | `S0220` | `0x0220` | OneShot | `S0220.ogg` | Non | - |
| 198 | `S0221` | `0x0221` | OneShot | `S0221.ogg` | Non | - |
| 199 | `S0222` | `0x0222` | OneShot | `S0222.ogg` | Non | - |
| 200 | `S0223` | `0x0223` | OneShot | `S0223.ogg` | Non | - |
| 201 | `S0224` | `0x0224` | OneShot | `S0224.ogg` | Non | - |
| 202 | `S0225` | `0x0225` | OneShot | `S0225.ogg` | Non | - |
| 203 | `S0226` | `0x0226` | OneShot | `S0226.ogg` | Non | - |
| 204 | `S0227` | `0x0227` | OneShot | `S0227.ogg` | Non | - |
| 205 | `S0228` | `0x0228` | OneShot | `S0228.ogg` | Non | - |
| 206 | `S0229` | `0x0229` | OneShot | `S0229.ogg` | Non | - |
| 207 | `S022A` | `0x022A` | OneShot | `S022A.ogg` | Non | - |
| 208 | `S022B` | `0x022B` | OneShot | `S022B.ogg` | Non | - |
| 209 | `S022C` | `0x022C` | OneShot | `S022C.ogg` | Non | - |
| 210 | `S022D` | `0x022D` | OneShot | `S022D.ogg` | Non | - |
| 211 | `S022E` | `0x022E` | OneShot | `S022E.ogg` | Non | - |
| 212 | `S022F` | `0x022F` | OneShot | `S022F.ogg` | Non | - |
| 213 | `S0230` | `0x0230` | OneShot | `S0230.ogg` | Non | - |
| 214 | `S0231` | `0x0231` | OneShot | `S0231.ogg` | Non | - |
| 215 | `S0232` | `0x0232` | OneShot | `S0232.ogg` | Non | - |
| 216 | `S0233` | `0x0233` | OneShot | `S0233.ogg` | Non | - |
| 217 | `S0234` | `0x0234` | OneShot | `S0234.ogg` | Non | - |
| 218 | `S0235` | `0x0235` | OneShot | `S0235.ogg` | Non | - |
| 219 | `S0236` | `0x0236` | OneShot | `S0236.ogg` | Non | - |
| 220 | `S0237` | `0x0237` | OneShot | `S0237.ogg` | Non | - |
| 221 | `S0238` | `0x0238` | OneShot | `S0238.ogg` | Non | - |
| 222 | `S023A` | `0x023A` | OneShot | `S023A.ogg` | Non | - |
| 223 | `S023B` | `0x023B` | OneShot | `S023B.ogg` | Non | - |
| 224 | `S023C` | `0x023C` | OneShot | `S023C.ogg` | Non | - |
| 225 | `S023D` | `0x023D` | OneShot | `S023D.ogg` | Non | - |
| 226 | `S023E` | `0x023E` | OneShot | `S023E.ogg` | Non | - |
| 227 | `S023F` | `0x023F` | OneShot | `S023F.ogg` | Non | - |
| 228 | `S0240` | `0x0240` | OneShot | `S0240.ogg` | Non | - |
| 229 | `S0241` | `0x0241` | OneShot | `S0241.ogg` | Non | - |
| 230 | `S0242` | `0x0242` | OneShot | `S0242.ogg` | Non | - |
| 231 | `S0243` | `0x0243` | OneShot | `S0243.ogg` | Non | - |
| 232 | `S0244` | `0x0244` | OneShot | `S0244.ogg` | Non | - |
| 233 | `S0245` | `0x0245` | OneShot | `S0245.ogg` | Non | - |
| 234 | `S024E` | `0x024E` | OneShot | `S024E.ogg` | Non | - |
| 235 | `S024F` | `0x024F` | OneShot | `S024F.ogg` | Non | - |
| 236 | `S0250` | `0x0250` | OneShot | `S0250.ogg` | Non | - |
| 237 | `S0251` | `0x0251` | OneShot | `S0251.ogg` | Non | - |
| 238 | `S0252` | `0x0252` | OneShot | `S0252.ogg` | Non | - |
| 239 | `S0253` | `0x0253` | OneShot | `S0253.ogg` | Non | - |
| 240 | `S0254` | `0x0254` | OneShot | `S0254.ogg` | Non | - |
| 241 | `S0255` | `0x0255` | OneShot | `S0255.ogg` | Non | - |
| 242 | `S0256` | `0x0256` | OneShot | `S0256.ogg` | Non | - |
| 243 | `S0259` | `0x0259` | OneShot | `S0259.ogg` | Oui | `crates/pin2k-core/src/admin.rs:149`, `crates/pin2k-core/src/swe1_tables.rs:180`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:583` |
| 244 | `S025A` | `0x025A` | OneShot | `S025A.ogg` | Non | - |
| 245 | `S025B` | `0x025B` | OneShot | `S025B.ogg` | Non | - |
| 246 | `S025C` | `0x025C` | OneShot | `S025C.ogg` | Non | - |
| 247 | `S025D` | `0x025D` | OneShot | `S025D.ogg` | Non | - |
| 248 | `S025E` | `0x025E` | OneShot | `S025E.ogg` | Non | - |
| 249 | `S025F` | `0x025F` | OneShot | `S025F.ogg` | Non | - |
| 250 | `S0260` | `0x0260` | OneShot | `S0260.ogg` | Non | - |
| 251 | `S0261` | `0x0261` | OneShot | `S0261.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:188` |
| 252 | `S0262` | `0x0262` | OneShot | `S0262.ogg` | Non | - |
| 253 | `S0263` | `0x0263` | OneShot | `S0263.ogg` | Non | - |
| 254 | `S0264` | `0x0264` | OneShot | `S0264.ogg` | Non | - |
| 255 | `S0265` | `0x0265` | OneShot | `S0265.ogg` | Non | - |
| 256 | `S02BC` | `0x02BC` | OneShot | `S02BC.ogg` | Non | - |
| 257 | `S02BD` | `0x02BD` | OneShot | `S02BD.ogg` | Non | - |
| 258 | `S02BE` | `0x02BE` | OneShot | `S02BE.ogg` | Non | - |
| 259 | `S02BF` | `0x02BF` | OneShot | `S02BF.ogg` | Non | - |
| 260 | `S02C0` | `0x02C0` | OneShot | `S02C0.ogg` | Non | - |
| 261 | `S02C1` | `0x02C1` | OneShot | `S02C1.ogg` | Oui | `crates/pin2k-core/src/admin.rs:147`, `crates/pin2k-core/src/swe1_tables.rs:178` |
| 262 | `S02C2` | `0x02C2` | OneShot | `S02C2.ogg` | Non | - |
| 263 | `S02C3` | `0x02C3` | OneShot | `S02C3.ogg` | Non | - |
| 264 | `S02C4` | `0x02C4` | OneShot | `S02C4.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:187` |
| 265 | `S02C5` | `0x02C5` | OneShot | `S02C5.ogg` | Non | - |
| 266 | `S02C6` | `0x02C6` | OneShot | `S02C6.ogg` | Non | - |
| 267 | `S02C7` | `0x02C7` | OneShot | `S02C7.ogg` | Non | - |
| 268 | `S02C8` | `0x02C8` | OneShot | `S02C8.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:186`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:596` |
| 269 | `S02C9` | `0x02C9` | OneShot | `S02C9.ogg` | Oui | `crates/pin2k-core/src/admin.rs:148`, `crates/pin2k-core/src/swe1_tables.rs:179` |
| 270 | `S02CA` | `0x02CA` | OneShot | `S02CA.ogg` | Non | - |
| 271 | `S02CB` | `0x02CB` | OneShot | `S02CB.ogg` | Non | - |
| 272 | `S02CC` | `0x02CC` | OneShot | `S02CC.ogg` | Non | - |
| 273 | `S02CD` | `0x02CD` | OneShot | `S02CD.ogg` | Non | - |
| 274 | `S02CE` | `0x02CE` | OneShot | `S02CE.ogg` | Non | - |
| 275 | `S02CF` | `0x02CF` | OneShot | `S02CF.ogg` | Non | - |
| 276 | `S02D0` | `0x02D0` | OneShot | `S02D0.ogg` | Non | - |
| 277 | `S0320` | `0x0320` | OneShot | `S0320.ogg` | Oui | `crates/pin2k-core/src/machine/attract_render.rs:18` |
| 278 | `S0321` | `0x0321` | OneShot | `S0321.ogg` | Non | - |
| 279 | `S0322` | `0x0322` | OneShot | `S0322.ogg` | Non | - |
| 280 | `S0323` | `0x0323` | OneShot | `S0323.ogg` | Non | - |
| 281 | `S0324` | `0x0324` | OneShot | `S0324.ogg` | Non | - |
| 282 | `S0325` | `0x0325` | OneShot | `S0325.ogg` | Non | - |
| 283 | `S0329` | `0x0329` | OneShot | `S0329.ogg` | Non | - |
| 284 | `S032A` | `0x032A` | OneShot | `S032A.ogg` | Non | - |
| 285 | `S032B` | `0x032B` | OneShot | `S032B.ogg` | Non | - |
| 286 | `S032D` | `0x032D` | OneShot | `S032D.ogg` | Non | - |
| 287 | `S032F` | `0x032F` | OneShot | `S032F.ogg` | Non | - |
| 288 | `S0330` | `0x0330` | OneShot | `S0330.ogg` | Non | - |
| 289 | `S0331` | `0x0331` | OneShot | `S0331.ogg` | Non | - |
| 290 | `S0332` | `0x0332` | OneShot | `S0332.ogg` | Non | - |
| 291 | `S0333` | `0x0333` | OneShot | `S0333.ogg` | Non | - |
| 292 | `S0334` | `0x0334` | OneShot | `S0334.ogg` | Non | - |
| 293 | `S0335` | `0x0335` | OneShot | `S0335.ogg` | Non | - |
| 294 | `S0336` | `0x0336` | OneShot | `S0336.ogg` | Non | - |
| 295 | `S0337` | `0x0337` | OneShot | `S0337.ogg` | Non | - |
| 296 | `S0339` | `0x0339` | OneShot | `S0339.ogg` | Non | - |
| 297 | `S033A` | `0x033A` | OneShot | `S033A.ogg` | Non | - |
| 298 | `S033B` | `0x033B` | OneShot | `S033B.ogg` | Non | - |
| 299 | `S033E` | `0x033E` | OneShot | `S033E.ogg` | Non | - |
| 300 | `S033F` | `0x033F` | OneShot | `S033F.ogg` | Non | - |
| 301 | `S0340` | `0x0340` | OneShot | `S0340.ogg` | Non | - |
| 302 | `S0341` | `0x0341` | OneShot | `S0341.ogg` | Non | - |
| 303 | `S0342` | `0x0342` | OneShot | `S0342.ogg` | Non | - |
| 304 | `S0352` | `0x0352` | OneShot | `S0352.ogg` | Non | - |
| 305 | `S0353` | `0x0353` | OneShot | `S0353.ogg` | Non | - |
| 306 | `S0354` | `0x0354` | OneShot | `S0354.ogg` | Non | - |
| 307 | `S0355` | `0x0355` | OneShot | `S0355.ogg` | Non | - |
| 308 | `S0356` | `0x0356` | OneShot | `S0356.ogg` | Non | - |
| 309 | `S0357` | `0x0357` | OneShot | `S0357.ogg` | Non | - |
| 310 | `S0358` | `0x0358` | OneShot | `S0358.ogg` | Non | - |
| 311 | `S0359` | `0x0359` | OneShot | `S0359.ogg` | Non | - |
| 312 | `S035A` | `0x035A` | OneShot | `S035A.ogg` | Non | - |
| 313 | `S035B` | `0x035B` | OneShot | `S035B.ogg` | Non | - |
| 314 | `S035C` | `0x035C` | OneShot | `S035C.ogg` | Non | - |
| 315 | `S035D` | `0x035D` | OneShot | `S035D.ogg` | Non | - |
| 316 | `S035E` | `0x035E` | OneShot | `S035E.ogg` | Non | - |
| 317 | `S035F` | `0x035F` | OneShot | `S035F.ogg` | Non | - |
| 318 | `S0360` | `0x0360` | OneShot | `S0360.ogg` | Non | - |
| 319 | `S0361` | `0x0361` | OneShot | `S0361.ogg` | Non | - |
| 320 | `S0362` | `0x0362` | OneShot | `S0362.ogg` | Non | - |
| 321 | `S0363` | `0x0363` | OneShot | `S0363.ogg` | Non | - |
| 322 | `S0364` | `0x0364` | OneShot | `S0364.ogg` | Non | - |
| 323 | `S0365` | `0x0365` | OneShot | `S0365.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:201` |
| 324 | `S0366` | `0x0366` | OneShot | `S0366.ogg` | Non | - |
| 325 | `S0367` | `0x0367` | OneShot | `S0367.ogg` | Non | - |
| 326 | `S0368` | `0x0368` | OneShot | `S0368.ogg` | Non | - |
| 327 | `S0369` | `0x0369` | OneShot | `S0369.ogg` | Non | - |
| 328 | `S036A` | `0x036A` | OneShot | `S036A.ogg` | Non | - |
| 329 | `S036B` | `0x036B` | OneShot | `S036B.ogg` | Non | - |
| 330 | `S036C` | `0x036C` | OneShot | `S036C.ogg` | Non | - |
| 331 | `S036D` | `0x036D` | OneShot | `S036D.ogg` | Non | - |
| 332 | `S036E` | `0x036E` | OneShot | `S036E.ogg` | Non | - |
| 333 | `S036F` | `0x036F` | OneShot | `S036F.ogg` | Non | - |
| 334 | `S0384` | `0x0384` | OneShot | `S0384.ogg` | Non | - |
| 335 | `S0385` | `0x0385` | OneShot | `S0385.ogg` | Non | - |
| 336 | `S0386` | `0x0386` | OneShot | `S0386.ogg` | Non | - |
| 337 | `S0388` | `0x0388` | OneShot | `S0388.ogg` | Non | - |
| 338 | `S0389` | `0x0389` | OneShot | `S0389.ogg` | Non | - |
| 339 | `S038A` | `0x038A` | OneShot | `S038A.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:182`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:645`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:656`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:667` |
| 340 | `S038B` | `0x038B` | OneShot | `S038B.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:181`, `crates/pin2k-rom-tool/src/gamecode_extractor.rs:677` |
| 341 | `S038C` | `0x038C` | OneShot | `S038C.ogg` | Non | - |
| 342 | `S038F` | `0x038F` | OneShot | `S038F.ogg` | Non | - |
| 343 | `S0390` | `0x0390` | OneShot | `S0390.ogg` | Non | - |
| 344 | `S0392` | `0x0392` | OneShot | `S0392.ogg` | Non | - |
| 345 | `S0393` | `0x0393` | OneShot | `S0393.ogg` | Non | - |
| 346 | `S0396` | `0x0396` | OneShot | `S0396.ogg` | Non | - |
| 347 | `S0398` | `0x0398` | OneShot | `S0398.ogg` | Non | - |
| 348 | `S0399` | `0x0399` | OneShot | `S0399.ogg` | Non | - |
| 349 | `S039A` | `0x039A` | OneShot | `S039A.ogg` | Non | - |
| 350 | `S039B` | `0x039B` | OneShot | `S039B.ogg` | Non | - |
| 351 | `S039C` | `0x039C` | OneShot | `S039C.ogg` | Non | - |
| 352 | `S039D` | `0x039D` | OneShot | `S039D.ogg` | Non | - |
| 353 | `S039E` | `0x039E` | OneShot | `S039E.ogg` | Non | - |
| 354 | `S03A0` | `0x03A0` | OneShot | `S03A0.ogg` | Non | - |
| 355 | `S03A4` | `0x03A4` | OneShot | `S03A4.ogg` | Non | - |
| 356 | `S03A5` | `0x03A5` | OneShot | `S03A5.ogg` | Non | - |
| 357 | `S03A6` | `0x03A6` | OneShot | `S03A6.ogg` | Non | - |
| 358 | `S03A7` | `0x03A7` | OneShot | `S03A7.ogg` | Non | - |
| 359 | `S03AE` | `0x03AE` | OneShot | `S03AE.ogg` | Non | - |
| 360 | `S03AF` | `0x03AF` | OneShot | `S03AF.ogg` | Non | - |
| 361 | `S03B0` | `0x03B0` | OneShot | `S03B0.ogg` | Non | - |
| 362 | `S03B1` | `0x03B1` | OneShot | `S03B1.ogg` | Non | - |
| 363 | `S03B2` | `0x03B2` | OneShot | `S03B2.ogg` | Non | - |
| 364 | `S03B3` | `0x03B3` | OneShot | `S03B3.ogg` | Non | - |
| 365 | `S03B4` | `0x03B4` | OneShot | `S03B4.ogg` | Non | - |
| 366 | `S03B7` | `0x03B7` | OneShot | `S03B7.ogg` | Non | - |
| 367 | `S03B8` | `0x03B8` | OneShot | `S03B8.ogg` | Non | - |
| 368 | `S03B9` | `0x03B9` | OneShot | `S03B9.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:177` |
| 369 | `S03BB` | `0x03BB` | OneShot | `S03BB.ogg` | Non | - |
| 370 | `S03BC` | `0x03BC` | OneShot | `S03BC.ogg` | Non | - |
| 371 | `S03BD` | `0x03BD` | OneShot | `S03BD.ogg` | Non | - |
| 372 | `S03BE` | `0x03BE` | OneShot | `S03BE.ogg` | Non | - |
| 373 | `S03BF` | `0x03BF` | OneShot | `S03BF.ogg` | Non | - |
| 374 | `S03C0` | `0x03C0` | OneShot | `S03C0.ogg` | Non | - |
| 375 | `S03C1` | `0x03C1` | OneShot | `S03C1.ogg` | Non | - |
| 376 | `S03C2` | `0x03C2` | OneShot | `S03C2.ogg` | Non | - |
| 377 | `S03C3` | `0x03C3` | OneShot | `S03C3.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:193` |
| 378 | `S03C9` | `0x03C9` | OneShot | `S03C9.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:194` |
| 379 | `S03CA` | `0x03CA` | OneShot | `S03CA.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:195` |
| 380 | `S03CB` | `0x03CB` | OneShot | `S03CB.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:196` |
| 381 | `S03CC` | `0x03CC` | OneShot | `S03CC.ogg` | Non | - |
| 382 | `S03CD` | `0x03CD` | OneShot | `S03CD.ogg` | Non | - |
| 383 | `S03CE` | `0x03CE` | OneShot | `S03CE.ogg` | Non | - |
| 384 | `S03CF` | `0x03CF` | OneShot | `S03CF.ogg` | Non | - |
| 385 | `S03D0` | `0x03D0` | OneShot | `S03D0.ogg` | Non | - |
| 386 | `S03D1` | `0x03D1` | OneShot | `S03D1.ogg` | Non | - |
| 387 | `S03D2` | `0x03D2` | OneShot | `S03D2.ogg` | Non | - |
| 388 | `S03D3` | `0x03D3` | OneShot | `S03D3.ogg` | Non | - |
| 389 | `S03D4` | `0x03D4` | OneShot | `S03D4.ogg` | Non | - |
| 390 | `S03D5` | `0x03D5` | OneShot | `S03D5.ogg` | Non | - |
| 391 | `S03D6` | `0x03D6` | OneShot | `S03D6.ogg` | Non | - |
| 392 | `S03D7` | `0x03D7` | OneShot | `S03D7.ogg` | Non | - |
| 393 | `S03D8` | `0x03D8` | OneShot | `S03D8.ogg` | Non | - |
| 394 | `S03D9` | `0x03D9` | OneShot | `S03D9.ogg` | Non | - |
| 395 | `S03DA` | `0x03DA` | OneShot | `S03DA.ogg` | Non | - |
| 396 | `S03DB` | `0x03DB` | OneShot | `S03DB.ogg` | Non | - |
| 397 | `S03DC` | `0x03DC` | OneShot | `S03DC.ogg` | Non | - |
| 398 | `S03DD` | `0x03DD` | OneShot | `S03DD.ogg` | Non | - |
| 399 | `S03DE` | `0x03DE` | OneShot | `S03DE.ogg` | Non | - |
| 400 | `S044C` | `0x044C` | OneShot | `S044C.ogg` | Non | - |
| 401 | `S044D` | `0x044D` | OneShot | `S044D.ogg` | Oui | `crates/pin2k-core/src/swe1_tables.rs:190` |
| 402 | `S044E` | `0x044E` | OneShot | `S044E.ogg` | Non | - |
| 403 | `S044F` | `0x044F` | OneShot | `S044F.ogg` | Non | - |
| 404 | `S0450` | `0x0450` | OneShot | `S0450.ogg` | Non | - |
| 405 | `S0451` | `0x0451` | OneShot | `S0451.ogg` | Non | - |
| 406 | `S0452` | `0x0452` | OneShot | `S0452.ogg` | Non | - |
| 407 | `S0453` | `0x0453` | OneShot | `S0453.ogg` | Non | - |
| 408 | `S0454` | `0x0454` | OneShot | `S0454.ogg` | Non | - |
| 409 | `S0455` | `0x0455` | OneShot | `S0455.ogg` | Non | - |
| 410 | `S0456` | `0x0456` | OneShot | `S0456.ogg` | Non | - |
| 411 | `S04B0` | `0x04B0` | OneShot | `S04B0.ogg` | Non | - |
| 412 | `S04B1` | `0x04B1` | OneShot | `S04B1.ogg` | Non | - |
| 413 | `S04B2` | `0x04B2` | OneShot | `S04B2.ogg` | Non | - |
| 414 | `S04B3` | `0x04B3` | OneShot | `S04B3.ogg` | Non | - |
| 415 | `S04B4` | `0x04B4` | OneShot | `S04B4.ogg` | Non | - |
| 416 | `S04B5` | `0x04B5` | OneShot | `S04B5.ogg` | Non | - |
| 417 | `S04B6` | `0x04B6` | OneShot | `S04B6.ogg` | Non | - |
| 418 | `S04B7` | `0x04B7` | OneShot | `S04B7.ogg` | Non | - |
| 419 | `S04B8` | `0x04B8` | OneShot | `S04B8.ogg` | Non | - |
| 420 | `S04B9` | `0x04B9` | OneShot | `S04B9.ogg` | Non | - |
| 421 | `S04BA` | `0x04BA` | OneShot | `S04BA.ogg` | Non | - |
| 422 | `S04BB` | `0x04BB` | OneShot | `S04BB.ogg` | Non | - |
| 423 | `S04BC` | `0x04BC` | OneShot | `S04BC.ogg` | Non | - |
| 424 | `S04BD` | `0x04BD` | OneShot | `S04BD.ogg` | Non | - |
| 425 | `S04BE` | `0x04BE` | OneShot | `S04BE.ogg` | Non | - |
| 426 | `S0514` | `0x0514` | OneShot | `S0514.ogg` | Non | - |
| 427 | `S0515` | `0x0515` | OneShot | `S0515.ogg` | Non | - |
| 428 | `S0516` | `0x0516` | OneShot | `S0516.ogg` | Non | - |
| 429 | `S0517` | `0x0517` | OneShot | `S0517.ogg` | Non | - |
| 430 | `S0518` | `0x0518` | OneShot | `S0518.ogg` | Non | - |
| 431 | `S0519` | `0x0519` | OneShot | `S0519.ogg` | Non | - |
| 432 | `S051A` | `0x051A` | OneShot | `S051A.ogg` | Non | - |
| 433 | `S051B` | `0x051B` | OneShot | `S051B.ogg` | Non | - |
| 434 | `S051C` | `0x051C` | OneShot | `S051C.ogg` | Non | - |
| 435 | `S051D` | `0x051D` | OneShot | `S051D.ogg` | Non | - |
| 436 | `S051E` | `0x051E` | OneShot | `S051E.ogg` | Non | - |
| 437 | `S0578` | `0x0578` | OneShot | `S0578.ogg` | Non | - |
| 438 | `S0579` | `0x0579` | OneShot | `S0579.ogg` | Non | - |
| 439 | `S057A` | `0x057A` | OneShot | `S057A.ogg` | Non | - |
| 440 | `S05DC` | `0x05DC` | OneShot | `S05DC.ogg` | Non | - |
| 441 | `S05DD` | `0x05DD` | OneShot | `S05DD.ogg` | Non | - |
| 442 | `S05DE` | `0x05DE` | OneShot | `S05DE.ogg` | Non | - |
| 443 | `S05DF` | `0x05DF` | OneShot | `S05DF.ogg` | Non | - |
| 444 | `S05E0` | `0x05E0` | OneShot | `S05E0.ogg` | Non | - |
| 445 | `S05E1` | `0x05E1` | OneShot | `S05E1.ogg` | Non | - |
| 446 | `S05E2` | `0x05E2` | OneShot | `S05E2.ogg` | Non | - |
| 447 | `S05E3` | `0x05E3` | OneShot | `S05E3.ogg` | Non | - |
| 448 | `S05E4` | `0x05E4` | OneShot | `S05E4.ogg` | Non | - |
| 449 | `S05E5` | `0x05E5` | OneShot | `S05E5.ogg` | Non | - |
| 450 | `S05E6` | `0x05E6` | OneShot | `S05E6.ogg` | Non | - |
| 451 | `S05E7` | `0x05E7` | OneShot | `S05E7.ogg` | Non | - |
| 452 | `S05E8` | `0x05E8` | OneShot | `S05E8.ogg` | Non | - |
| 453 | `S05E9` | `0x05E9` | OneShot | `S05E9.ogg` | Non | - |
| 454 | `S05EA` | `0x05EA` | OneShot | `S05EA.ogg` | Non | - |
| 455 | `S05EB` | `0x05EB` | OneShot | `S05EB.ogg` | Non | - |
| 456 | `S05EC` | `0x05EC` | OneShot | `S05EC.ogg` | Non | - |
| 457 | `S05ED` | `0x05ED` | OneShot | `S05ED.ogg` | Non | - |
| 458 | `S05EE` | `0x05EE` | OneShot | `S05EE.ogg` | Non | - |
| 459 | `S0640` | `0x0640` | OneShot | `S0640.ogg` | Non | - |
| 460 | `S0641` | `0x0641` | OneShot | `S0641.ogg` | Non | - |
| 461 | `S0642` | `0x0642` | OneShot | `S0642.ogg` | Non | - |
| 462 | `S0643` | `0x0643` | OneShot | `S0643.ogg` | Non | - |
| 463 | `S0644` | `0x0644` | OneShot | `S0644.ogg` | Non | - |
| 464 | `S0645` | `0x0645` | OneShot | `S0645.ogg` | Non | - |
| 465 | `S0646` | `0x0646` | OneShot | `S0646.ogg` | Non | - |
| 466 | `S0647` | `0x0647` | OneShot | `S0647.ogg` | Non | - |
| 467 | `S0648` | `0x0648` | OneShot | `S0648.ogg` | Non | - |
| 468 | `S0649` | `0x0649` | OneShot | `S0649.ogg` | Non | - |
| 469 | `S064A` | `0x064A` | OneShot | `S064A.ogg` | Non | - |
| 470 | `S06A4` | `0x06A4` | OneShot | `S06A4.ogg` | Non | - |
| 471 | `S06A5` | `0x06A5` | OneShot | `S06A5.ogg` | Non | - |
| 472 | `S06A6` | `0x06A6` | OneShot | `S06A6.ogg` | Non | - |
| 473 | `S06A7` | `0x06A7` | OneShot | `S06A7.ogg` | Non | - |
| 474 | `S06A8` | `0x06A8` | OneShot | `S06A8.ogg` | Non | - |
| 475 | `S06A9` | `0x06A9` | OneShot | `S06A9.ogg` | Non | - |
| 476 | `S06AA` | `0x06AA` | OneShot | `S06AA.ogg` | Non | - |
| 477 | `S06AB` | `0x06AB` | OneShot | `S06AB.ogg` | Non | - |
| 478 | `S06AC` | `0x06AC` | OneShot | `S06AC.ogg` | Non | - |
| 479 | `S06AD` | `0x06AD` | OneShot | `S06AD.ogg` | Non | - |
| 480 | `S07D0` | `0x07D0` | OneShot | `S07D0.ogg` | Non | - |
| 481 | `S07D1` | `0x07D1` | OneShot | `S07D1.ogg` | Non | - |
| 482 | `S07D2` | `0x07D2` | OneShot | `S07D2.ogg` | Non | - |
| 483 | `S07D3` | `0x07D3` | OneShot | `S07D3.ogg` | Non | - |
| 484 | `S07D9` | `0x07D9` | OneShot | `S07D9.ogg` | Non | - |
| 485 | `S07DA` | `0x07DA` | OneShot | `S07DA.ogg` | Non | - |
| 486 | `S07DB` | `0x07DB` | OneShot | `S07DB.ogg` | Non | - |
| 487 | `S07DC` | `0x07DC` | OneShot | `S07DC.ogg` | Non | - |
| 488 | `S07DD` | `0x07DD` | OneShot | `S07DD.ogg` | Non | - |
| 489 | `S07DE` | `0x07DE` | OneShot | `S07DE.ogg` | Non | - |
| 490 | `S07DF` | `0x07DF` | OneShot | `S07DF.ogg` | Non | - |
| 491 | `S07E1` | `0x07E1` | OneShot | `S07E1.ogg` | Non | - |
| 492 | `S07E2` | `0x07E2` | OneShot | `S07E2.ogg` | Non | - |
| 493 | `S07E3` | `0x07E3` | OneShot | `S07E3.ogg` | Non | - |
| 494 | `S07E4` | `0x07E4` | OneShot | `S07E4.ogg` | Non | - |
| 495 | `S07E5` | `0x07E5` | OneShot | `S07E5.ogg` | Non | - |
| 496 | `S07E6` | `0x07E6` | OneShot | `S07E6.ogg` | Non | - |
| 497 | `S07E7` | `0x07E7` | OneShot | `S07E7.ogg` | Non | - |
| 498 | `S07E8` | `0x07E8` | OneShot | `S07E8.ogg` | Non | - |
| 499 | `S07EA` | `0x07EA` | OneShot | `S07EA.ogg` | Non | - |
| 500 | `S07EB` | `0x07EB` | OneShot | `S07EB.ogg` | Non | - |
| 501 | `S07EC` | `0x07EC` | OneShot | `S07EC.ogg` | Non | - |
| 502 | `S07ED` | `0x07ED` | OneShot | `S07ED.ogg` | Non | - |
| 503 | `S07EE` | `0x07EE` | OneShot | `S07EE.ogg` | Non | - |
| 504 | `S07EF` | `0x07EF` | OneShot | `S07EF.ogg` | Non | - |
| 505 | `S07F1` | `0x07F1` | OneShot | `S07F1.ogg` | Non | - |
| 506 | `S07F2` | `0x07F2` | OneShot | `S07F2.ogg` | Non | - |
| 507 | `S07F3` | `0x07F3` | OneShot | `S07F3.ogg` | Non | - |
| 508 | `S07F4` | `0x07F4` | OneShot | `S07F4.ogg` | Non | - |
| 509 | `S07F6` | `0x07F6` | OneShot | `S07F6.ogg` | Non | - |
| 510 | `S07F7` | `0x07F7` | OneShot | `S07F7.ogg` | Non | - |
| 511 | `S07F8` | `0x07F8` | OneShot | `S07F8.ogg` | Non | - |
| 512 | `S07F9` | `0x07F9` | OneShot | `S07F9.ogg` | Non | - |
| 513 | `S07FA` | `0x07FA` | OneShot | `S07FA.ogg` | Non | - |
| 514 | `S07FB` | `0x07FB` | OneShot | `S07FB.ogg` | Non | - |
| 515 | `S07FC` | `0x07FC` | OneShot | `S07FC.ogg` | Non | - |
| 516 | `S07FD` | `0x07FD` | OneShot | `S07FD.ogg` | Non | - |
| 517 | `S07FE` | `0x07FE` | OneShot | `S07FE.ogg` | Non | - |
| 518 | `S07FF` | `0x07FF` | OneShot | `S07FF.ogg` | Non | - |
| 519 | `S0800` | `0x0800` | OneShot | `S0800.ogg` | Oui | `crates/pin2k-rom-tool/src/asset_extractor.rs:54`, `crates/pin2k-rom-tool/src/asset_extractor.rs:55` |
| 520 | `S0801` | `0x0801` | OneShot | `S0801.ogg` | Non | - |
| 521 | `S0802` | `0x0802` | OneShot | `S0802.ogg` | Non | - |
| 522 | `S0803` | `0x0803` | OneShot | `S0803.ogg` | Non | - |
| 523 | `S0804` | `0x0804` | OneShot | `S0804.ogg` | Non | - |
| 524 | `S0805` | `0x0805` | OneShot | `S0805.ogg` | Non | - |
| 525 | `S0807` | `0x0807` | OneShot | `S0807.ogg` | Non | - |
| 526 | `S0808` | `0x0808` | OneShot | `S0808.ogg` | Non | - |
| 527 | `S0809` | `0x0809` | OneShot | `S0809.ogg` | Non | - |
| 528 | `S080A` | `0x080A` | OneShot | `S080A.ogg` | Non | - |
| 529 | `S080C` | `0x080C` | OneShot | `S080C.ogg` | Non | - |
| 530 | `S080D` | `0x080D` | OneShot | `S080D.ogg` | Non | - |
| 531 | `S080E` | `0x080E` | OneShot | `S080E.ogg` | Non | - |
| 532 | `S0810` | `0x0810` | OneShot | `S0810.ogg` | Non | - |
| 533 | `S0811` | `0x0811` | OneShot | `S0811.ogg` | Non | - |
| 534 | `S0815` | `0x0815` | OneShot | `S0815.ogg` | Non | - |
| 535 | `S0816` | `0x0816` | OneShot | `S0816.ogg` | Non | - |
| 536 | `S0817` | `0x0817` | OneShot | `S0817.ogg` | Non | - |
| 537 | `S081A` | `0x081A` | OneShot | `S081A.ogg` | Non | - |
| 538 | `S081B` | `0x081B` | OneShot | `S081B.ogg` | Non | - |
| 539 | `S081C` | `0x081C` | OneShot | `S081C.ogg` | Non | - |
| 540 | `S0820` | `0x0820` | OneShot | `S0820.ogg` | Non | - |
| 541 | `S0823` | `0x0823` | OneShot | `S0823.ogg` | Non | - |
| 542 | `S0824` | `0x0824` | OneShot | `S0824.ogg` | Non | - |
| 543 | `S0827` | `0x0827` | OneShot | `S0827.ogg` | Non | - |
| 544 | `S0828` | `0x0828` | OneShot | `S0828.ogg` | Non | - |
| 545 | `S0829` | `0x0829` | OneShot | `S0829.ogg` | Non | - |
| 546 | `S082A` | `0x082A` | OneShot | `S082A.ogg` | Non | - |
| 547 | `S082C` | `0x082C` | OneShot | `S082C.ogg` | Non | - |
| 548 | `S082D` | `0x082D` | OneShot | `S082D.ogg` | Non | - |
| 549 | `S082E` | `0x082E` | OneShot | `S082E.ogg` | Non | - |
| 550 | `S082F` | `0x082F` | OneShot | `S082F.ogg` | Non | - |
| 551 | `S0830` | `0x0830` | OneShot | `S0830.ogg` | Non | - |
| 552 | `S0831` | `0x0831` | OneShot | `S0831.ogg` | Non | - |
| 553 | `S0832` | `0x0832` | OneShot | `S0832.ogg` | Non | - |
| 554 | `S0833` | `0x0833` | OneShot | `S0833.ogg` | Non | - |
| 555 | `S0834` | `0x0834` | OneShot | `S0834.ogg` | Non | - |
| 556 | `S0836` | `0x0836` | OneShot | `S0836.ogg` | Non | - |
| 557 | `S0837` | `0x0837` | OneShot | `S0837.ogg` | Non | - |
| 558 | `S0838` | `0x0838` | OneShot | `S0838.ogg` | Non | - |
| 559 | `S0839` | `0x0839` | OneShot | `S0839.ogg` | Non | - |
| 560 | `S083A` | `0x083A` | OneShot | `S083A.ogg` | Non | - |
| 561 | `S083B` | `0x083B` | OneShot | `S083B.ogg` | Non | - |
| 562 | `S083C` | `0x083C` | OneShot | `S083C.ogg` | Non | - |
| 563 | `S083D` | `0x083D` | OneShot | `S083D.ogg` | Non | - |
| 564 | `S083E` | `0x083E` | OneShot | `S083E.ogg` | Non | - |
| 565 | `S083F` | `0x083F` | OneShot | `S083F.ogg` | Non | - |
| 566 | `S0840` | `0x0840` | OneShot | `S0840.ogg` | Non | - |
| 567 | `S0841` | `0x0841` | OneShot | `S0841.ogg` | Non | - |
| 568 | `S0842` | `0x0842` | OneShot | `S0842.ogg` | Non | - |
| 569 | `S0843` | `0x0843` | OneShot | `S0843.ogg` | Non | - |
| 570 | `S0844` | `0x0844` | OneShot | `S0844.ogg` | Non | - |
| 571 | `S0845` | `0x0845` | OneShot | `S0845.ogg` | Non | - |
| 572 | `S0847` | `0x0847` | OneShot | `S0847.ogg` | Non | - |
| 573 | `S0848` | `0x0848` | OneShot | `S0848.ogg` | Non | - |
| 574 | `S0849` | `0x0849` | OneShot | `S0849.ogg` | Non | - |
| 575 | `S084A` | `0x084A` | OneShot | `S084A.ogg` | Non | - |
| 576 | `S084B` | `0x084B` | OneShot | `S084B.ogg` | Non | - |
| 577 | `S084C` | `0x084C` | OneShot | `S084C.ogg` | Non | - |
| 578 | `S084D` | `0x084D` | OneShot | `S084D.ogg` | Non | - |
| 579 | `S084E` | `0x084E` | OneShot | `S084E.ogg` | Non | - |
| 580 | `S084F` | `0x084F` | OneShot | `S084F.ogg` | Non | - |
| 581 | `S0850` | `0x0850` | OneShot | `S0850.ogg` | Non | - |
| 582 | `S0851` | `0x0851` | OneShot | `S0851.ogg` | Non | - |
| 583 | `S0852` | `0x0852` | OneShot | `S0852.ogg` | Non | - |
| 584 | `S0853` | `0x0853` | OneShot | `S0853.ogg` | Non | - |
| 585 | `S0855` | `0x0855` | OneShot | `S0855.ogg` | Non | - |
| 586 | `S0856` | `0x0856` | OneShot | `S0856.ogg` | Non | - |
| 587 | `S0857` | `0x0857` | OneShot | `S0857.ogg` | Non | - |
| 588 | `S0858` | `0x0858` | OneShot | `S0858.ogg` | Non | - |
| 589 | `S0898` | `0x0898` | OneShot | `S0898.ogg` | Non | - |
| 590 | `S0899` | `0x0899` | OneShot | `S0899.ogg` | Non | - |
| 591 | `S089A` | `0x089A` | OneShot | `S089A.ogg` | Non | - |
| 592 | `S089B` | `0x089B` | OneShot | `S089B.ogg` | Non | - |
| 593 | `S089C` | `0x089C` | OneShot | `S089C.ogg` | Non | - |
| 594 | `S089D` | `0x089D` | OneShot | `S089D.ogg` | Non | - |
| 595 | `S089E` | `0x089E` | OneShot | `S089E.ogg` | Non | - |
| 596 | `S089F` | `0x089F` | OneShot | `S089F.ogg` | Non | - |
| 597 | `S08A0` | `0x08A0` | OneShot | `S08A0.ogg` | Non | - |
| 598 | `S08A1` | `0x08A1` | OneShot | `S08A1.ogg` | Non | - |
| 599 | `S08A2` | `0x08A2` | OneShot | `S08A2.ogg` | Non | - |
| 600 | `S08A3` | `0x08A3` | OneShot | `S08A3.ogg` | Non | - |
| 601 | `S08A4` | `0x08A4` | OneShot | `S08A4.ogg` | Non | - |
| 602 | `S08A5` | `0x08A5` | OneShot | `S08A5.ogg` | Non | - |
| 603 | `S08A6` | `0x08A6` | OneShot | `S08A6.ogg` | Non | - |
| 604 | `S08A7` | `0x08A7` | OneShot | `S08A7.ogg` | Non | - |
| 605 | `S08A8` | `0x08A8` | OneShot | `S08A8.ogg` | Non | - |
| 606 | `S08A9` | `0x08A9` | OneShot | `S08A9.ogg` | Non | - |
| 607 | `S08AA` | `0x08AA` | OneShot | `S08AA.ogg` | Non | - |
| 608 | `S08AB` | `0x08AB` | OneShot | `S08AB.ogg` | Non | - |
| 609 | `S08AC` | `0x08AC` | OneShot | `S08AC.ogg` | Non | - |
| 610 | `S08AD` | `0x08AD` | OneShot | `S08AD.ogg` | Non | - |
| 611 | `S08AE` | `0x08AE` | OneShot | `S08AE.ogg` | Non | - |
| 612 | `S08AF` | `0x08AF` | OneShot | `S08AF.ogg` | Non | - |
| 613 | `S08B0` | `0x08B0` | OneShot | `S08B0.ogg` | Non | - |
| 614 | `S08B1` | `0x08B1` | OneShot | `S08B1.ogg` | Non | - |
| 615 | `S08B2` | `0x08B2` | OneShot | `S08B2.ogg` | Non | - |
| 616 | `S08B3` | `0x08B3` | OneShot | `S08B3.ogg` | Non | - |
| 617 | `S08B4` | `0x08B4` | OneShot | `S08B4.ogg` | Non | - |
| 618 | `S08B5` | `0x08B5` | OneShot | `S08B5.ogg` | Non | - |
| 619 | `S08B6` | `0x08B6` | OneShot | `S08B6.ogg` | Non | - |
| 620 | `S08B7` | `0x08B7` | OneShot | `S08B7.ogg` | Non | - |
| 621 | `S08B8` | `0x08B8` | OneShot | `S08B8.ogg` | Non | - |
| 622 | `S08B9` | `0x08B9` | OneShot | `S08B9.ogg` | Non | - |
| 623 | `S08BA` | `0x08BA` | OneShot | `S08BA.ogg` | Non | - |
| 624 | `S08BB` | `0x08BB` | OneShot | `S08BB.ogg` | Non | - |
| 625 | `S08BC` | `0x08BC` | OneShot | `S08BC.ogg` | Non | - |
| 626 | `S08BD` | `0x08BD` | OneShot | `S08BD.ogg` | Non | - |
| 627 | `S08BE` | `0x08BE` | OneShot | `S08BE.ogg` | Non | - |
| 628 | `S08BF` | `0x08BF` | OneShot | `S08BF.ogg` | Non | - |
| 629 | `S08C0` | `0x08C0` | OneShot | `S08C0.ogg` | Non | - |
| 630 | `S08C1` | `0x08C1` | OneShot | `S08C1.ogg` | Non | - |
| 631 | `S08C2` | `0x08C2` | OneShot | `S08C2.ogg` | Non | - |
| 632 | `S08C3` | `0x08C3` | OneShot | `S08C3.ogg` | Non | - |
| 633 | `S08C4` | `0x08C4` | OneShot | `S08C4.ogg` | Non | - |
| 634 | `S08C5` | `0x08C5` | OneShot | `S08C5.ogg` | Non | - |
| 635 | `S08C6` | `0x08C6` | OneShot | `S08C6.ogg` | Non | - |
| 636 | `S08C7` | `0x08C7` | OneShot | `S08C7.ogg` | Non | - |
| 637 | `S08C8` | `0x08C8` | OneShot | `S08C8.ogg` | Non | - |
| 638 | `S08C9` | `0x08C9` | OneShot | `S08C9.ogg` | Non | - |
| 639 | `S08CA` | `0x08CA` | OneShot | `S08CA.ogg` | Non | - |
| 640 | `S08CB` | `0x08CB` | OneShot | `S08CB.ogg` | Non | - |
| 641 | `S08CC` | `0x08CC` | OneShot | `S08CC.ogg` | Non | - |
| 642 | `S08CD` | `0x08CD` | OneShot | `S08CD.ogg` | Non | - |
| 643 | `S08CE` | `0x08CE` | OneShot | `S08CE.ogg` | Non | - |
| 644 | `S08CF` | `0x08CF` | OneShot | `S08CF.ogg` | Non | - |
| 645 | `S08D0` | `0x08D0` | OneShot | `S08D0.ogg` | Non | - |
| 646 | `S08D1` | `0x08D1` | OneShot | `S08D1.ogg` | Non | - |
| 647 | `S08D2` | `0x08D2` | OneShot | `S08D2.ogg` | Non | - |
| 648 | `S08D3` | `0x08D3` | OneShot | `S08D3.ogg` | Non | - |
| 649 | `S08D4` | `0x08D4` | OneShot | `S08D4.ogg` | Non | - |
| 650 | `S08D5` | `0x08D5` | OneShot | `S08D5.ogg` | Non | - |
| 651 | `S08D6` | `0x08D6` | OneShot | `S08D6.ogg` | Non | - |
| 652 | `S08D7` | `0x08D7` | OneShot | `S08D7.ogg` | Non | - |
| 653 | `S08D8` | `0x08D8` | OneShot | `S08D8.ogg` | Non | - |
| 654 | `S08D9` | `0x08D9` | OneShot | `S08D9.ogg` | Non | - |
| 655 | `S08DA` | `0x08DA` | OneShot | `S08DA.ogg` | Non | - |
| 656 | `S08DB` | `0x08DB` | OneShot | `S08DB.ogg` | Non | - |
| 657 | `S08DC` | `0x08DC` | OneShot | `S08DC.ogg` | Non | - |
| 658 | `S08DD` | `0x08DD` | OneShot | `S08DD.ogg` | Non | - |
| 659 | `S08DE` | `0x08DE` | OneShot | `S08DE.ogg` | Non | - |
| 660 | `S08DF` | `0x08DF` | OneShot | `S08DF.ogg` | Non | - |
| 661 | `S08E0` | `0x08E0` | OneShot | `S08E0.ogg` | Non | - |
| 662 | `S08E1` | `0x08E1` | OneShot | `S08E1.ogg` | Non | - |
| 663 | `S08E2` | `0x08E2` | OneShot | `S08E2.ogg` | Non | - |
| 664 | `S08E3` | `0x08E3` | OneShot | `S08E3.ogg` | Non | - |
| 665 | `S08E4` | `0x08E4` | OneShot | `S08E4.ogg` | Non | - |
| 666 | `S08E5` | `0x08E5` | OneShot | `S08E5.ogg` | Non | - |
| 667 | `S08E6` | `0x08E6` | OneShot | `S08E6.ogg` | Non | - |
| 668 | `S08E7` | `0x08E7` | OneShot | `S08E7.ogg` | Non | - |
| 669 | `S08E8` | `0x08E8` | OneShot | `S08E8.ogg` | Non | - |
| 670 | `S08E9` | `0x08E9` | OneShot | `S08E9.ogg` | Non | - |
| 671 | `S08EA` | `0x08EA` | OneShot | `S08EA.ogg` | Non | - |
| 672 | `S08EB` | `0x08EB` | OneShot | `S08EB.ogg` | Non | - |
| 673 | `S08EC` | `0x08EC` | OneShot | `S08EC.ogg` | Non | - |
| 674 | `S08ED` | `0x08ED` | OneShot | `S08ED.ogg` | Non | - |
| 675 | `S08EE` | `0x08EE` | OneShot | `S08EE.ogg` | Non | - |
| 676 | `S08EF` | `0x08EF` | OneShot | `S08EF.ogg` | Non | - |
| 677 | `S08F0` | `0x08F0` | OneShot | `S08F0.ogg` | Non | - |
| 678 | `S08F1` | `0x08F1` | OneShot | `S08F1.ogg` | Non | - |
| 679 | `S08F2` | `0x08F2` | OneShot | `S08F2.ogg` | Non | - |
| 680 | `S08F3` | `0x08F3` | OneShot | `S08F3.ogg` | Non | - |
| 681 | `S08F4` | `0x08F4` | OneShot | `S08F4.ogg` | Non | - |
| 682 | `S08F5` | `0x08F5` | OneShot | `S08F5.ogg` | Non | - |
| 683 | `S08F6` | `0x08F6` | OneShot | `S08F6.ogg` | Non | - |
| 684 | `S08F7` | `0x08F7` | OneShot | `S08F7.ogg` | Non | - |
| 685 | `S08F8` | `0x08F8` | OneShot | `S08F8.ogg` | Non | - |
| 686 | `S08F9` | `0x08F9` | OneShot | `S08F9.ogg` | Non | - |
| 687 | `S08FA` | `0x08FA` | OneShot | `S08FA.ogg` | Non | - |
| 688 | `S08FB` | `0x08FB` | OneShot | `S08FB.ogg` | Non | - |
| 689 | `S08FC` | `0x08FC` | OneShot | `S08FC.ogg` | Non | - |
