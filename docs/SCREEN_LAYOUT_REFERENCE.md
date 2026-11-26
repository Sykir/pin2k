# Screen Layouts, Coordinates & Debug Console Reference (`pin2k`)

Complete reference of all screens, `640×240` logical coordinates, Z-priorities, bitmap fonts, RGB555 colors, lamp choreographies (`machine/leff.rs`), FX/overlays (`machine/fx/`), and audio requests (`swe1_audio.rs`, `382` `ARD` pools / `695` `ACD`) extracted via reverse engineering and Ghidra 12.2 decompilation of `work/swe1_0200.elf` and implemented in pure Rust inside `pin2k-core` + `pin2k-desktop`.

---

## 1. Compositor Architecture (`BaseViewManager` `640×240` → `640×480`)

Williams Pinball 2000 hardware (MediaGX + PRISM) composites each logical frame at **`640×240`** RGB555 (`0x7C1F` = transparent magenta) before scan-doubling vertically to **`640×480`**.

- **Traits & draw commands (`crates/pin2k-hal/src/lib.rs`)**:
  - `VideoBackend::begin_frame()`: resets the Z-ordered layer queue.
  - `SpriteDrawCmd`: `{ anim_name, frame, x, y, z, flip_x, scale_x, scale_y, monochrome_rgb555 }`.
  - `TextDrawCmd`: `{ font_name, text, x, y, z, color_rgb555, shadow_rgb555, align: TextAlign }` (`Left`, `Center`, `Right`).
  - `RectDrawCmd`: `{ x, y, w, h, z, color_rgb555, alpha }`.
- **Rendering (`crates/pin2k-desktop/src/compositor.rs`, `raster.rs`)**:
  - All sprites, rectangles, and bitmap texts are merged and sorted by ascending `z` priority on each `present()`.
  - All 14 SWE1 scenes (including *Boonta Eve Podrace* via `fx/scene_extras.rs::render_podrace_layers`) render uniformly through `VideoBackend`.

---

## 2. In-Game HUD (`InGame`)

### `DeffScoreDisplay` (`0x001725d8..0x001740fd`) — `crates/pin2k-core/src/machine/score_display.rs`
- **9 player characters (`player_name_table_data` @ `0x002dc798`)**: `Qui-Gon`, `Jar Jar`, `Anakin`, `Amidala`, `Obi-Wan`, `R2-D2`, `Yoda`, `Watto`, `Darth Maul` (unique per player 1..4 via `PlayerState::pick_unique_character`).
- **Exact `Table<score_coord>` positions (`0x00173f66..0x001740dd`)**, `z = 10`:
  - Player 1: `(36, 7)`, `TextAlign::Left` (keeps `x=535..635` clear on the right for Lasers)
  - Player 2: `(525, 7)`, `TextAlign::Right`
  - Player 3: `(36, 26)`, `TextAlign::Left`
  - Player 4: `(525, 26)`, `TextAlign::Right`
- **Fonts & colors**: name in `font_system_7_data`, comma-formatted score (`1,234,560`) in `font_kahuna_shadow_9_data`; active player `0x7FFF` (shadow `0x72E6`), inactive `0x294A` (shadow `0x18C6`).
- **Lightsabers (`saber_process` @ `y = 7, z = 9`, centered at `x = 280`)**:
  - Left blade: `anim_saber_{green,red,blue}_{start,loop,flare}` at `(16, 7)`
  - Center handle: `anim_saber_handle` at `(261, 7)`
  - Right blade: `anim_saber_{green,red,blue}_{start,loop,flare}` at `(300, 7)` with `flip_x = true`
  - Center info bar (`m_info_message_process`): `BALL N` (`x = 280, y = 9, z = 10`) and `FREE PLAY` / `CREDITS` (`x = 280, y = 26, z = 10`), `font_system_7_data`, `0x7E00` (shadow `0x6540`).

### `DeffLaserStatusBackground` (`0x001445f0..0x00144d64`) — `crates/pin2k-core/src/machine/hud_display.rs`
- Rotating Naboo ship reticle `anim_naboo_fighter_tiny_rotate` (`100×50`) at `(535, 10)` (`0x27B - 100`), `z = 20`.
- Remaining shot count `(585, 35)` and label `LASER` / `LASERS` / `TORPEDOS` (during *Hangar Escape*) `(585, 48)`, `font_system_7_data`, `z = 21`, yellow `0x7FE0` when ready; greyed out (`monochrome_rgb555 = Some(0x294A)`, text `0x294A`) at `0`.

### `VirtualSpinners` / `DeffVSpinner` (`0x0017ead0..0x0017fa04`) — `crates/pin2k-core/src/machine/vspinner.rs`
- **Authentic behavior**: in nominal gameplay (`InGame` with no active scene, multiball, or special screen), both spinners remain visible resting statically at frame `0`; spinning animation, lightning, score popups, and turnstile audio trigger **exclusively** on ramp passes (`LEFT_RAMP_ENTER`, `RIGHT_RAMP_ENTER`, `RAMP_MADE_LEFT`, `RAMP_MADE_RIGHT`).
- **Positions & layers**:
  - Left: `anim_spinner_bracket_left` + `anim_spinner_1` at `(43, 174)`, `z = 22`.
  - Right: `anim_spinner_bracket_right` + `anim_spinner_2` at `(421, 174)`, `z = 22`.
  - High speed: `anim_spinner_lightning` (`x + 6, y = 162, z = 22`).
  - Score popup `+10000` / `+25000`: `font_system_7_data`, `z = 23`, color `0x7FE0`.
- **Audio**: rotates through the 4 DCS turnstile cues `sound::SPINNER_TICKS` (`0x03C3`, `0x03C9`, `0x03CA`, `0x03CB`) per half-revolution.

### `DeffObiHolo` (Nominal Shield & Scene Select) — `crates/pin2k-core/src/machine/scene_render.rs`
- **Shield up (`!scene_select_ready`)**: renders `anim_obiwan_holo` at `(125, 80)`, `z = 3`, flashing `anim_obiwan_holo_glow` for `350 ms` on each deflector shield hit.
- **Shield defeated (`shield_health == 0`)**:
  1. `0..900 ms`: plays `anim_obiwan_holo_flatten` (`125, 80, z = 3`) + `ard_obi_holo_watch_descends`.
  2. `900..1500 ms`: chains into `anim_obiwan_holo_activate` (`162, 159, z = 4`) + `anim_holo_burst` (`60, 8, z = 5`) + `ard_scene_holo_burst`.
  3. `>= 1500 ms`: loops `anim_obiwan_holo_glow_loop` (`179, 157, z = 4`) with the selectable mini-game hologram (`sc.holo_anim_hint()`) floating above it (`x = 190, y = 44, z = 5`).

### `DeffGunganSeaweed` & `DeffGunganLetters` — `crates/pin2k-core/src/machine/hud_display.rs`
- Rendered on the right side above the jet bumpers during nominal gameplay (`InGame` with no active scene, multiball, or special screen):
  - `anim_gungan_seaweed` (`139×90`, 40 fr @ 40 fps) at `(496, 135)`, `z = 15`.
  - 5 rising `anim_gungan_bubbles` columns behind the seaweed at `z = 10` (`y = 195 → 125`).
  - `G-U-N-G-A-N` letter progress (`font_shadow_9_narrow_data`, `y = 112, z = 15`), blinking yellow/cyan during Super Jets.

---

## 3. Attract Mode — 20 Screens (`AttractProcInfo` @ `0x002d94d0`)

Implemented in `crates/pin2k-core/src/attract.rs` and `crates/pin2k-core/src/machine/attract_render.rs`. Every slide overlays `DeffAttractModePriceBackground` (`z = 250`, `font_system_9_data`, `FREE PLAY / PRESS START` or `INSERT COINS` at `y = 210 / 225`).

| Index | Debug Mode | `Deff` Class / Content | Primary Visual Layers |
|---:|---|---|---|
| 0 | `attract:0` | `DeffAttractModeKahunaLogo` | `anim_kahuna_logo_flare` `(23,7,z=7)` interpolated across 9 RGB555 colors (`0x100fc0`) + `anim_kahuna_logo_spinning` `(187,80,z=8)` |
| 1 | `attract:1` | `DeffAttractModeSlideShow` | Panoramic banners `anim_starwars_banner_1..4` |
| 2 | `attract:2` | `DeffAttractModeHSTD` | HSTD leaderboard with Jedi ranks (`font_starwars_19_data` + `font_shadow_15_data`) |
| 3..6 | `attract:3..6` | `DeffAtMJediTable` | `Youth`, `Knight`, `Master`, `Spirit` tables (`anim_bonus_icon_jedi_*`) |
| 7 | `attract:7` | `DeffAttractModeRampComboChampion` | Ramp Combo Champion + `anim_naboo_flyby_single` |
| 8 | `attract:8` | `DeffAttractModeP2KLogo` | `anim_p2k_logo_animated` |
| 9 | `attract:9` | `DeffAttractModeHasbroAd` | Hasbro / Jedi Knight promo |
| 10 | `attract:10` | `DeffAttractModeScoreAwards` | Replay & Extra Ball awards |
| 11 | `attract:11` | `DeffAttractModeEmailAddress` | `FEEDBACK@PINBALL.COM` |
| 12 | `attract:12` | `DeffAttractModeWebPage` | `WWW.PINBALL.COM` with Anakin portrait |
| 13 | `attract:13` | `DeffAttractModeScores` | Last game scores (1..4 players + character) |
| 14..17 | `attract:14..17` | `DeffAttractModeGameCredits1..4` | Williams Electronics, Lucasfilm Ltd., design & software credits |
| 18 | `attract:18` | `DeffCustomMessage` | Operator custom message |
| 19 | `attract:19` | `DeffHTPScene` | How To Play (shield → holoprojector → scenes) |

---

## 4. All 14 SWE1 Scenes (`scene:0`..`13`), Holo Select & Multiballs

Implemented in `crates/pin2k-core/src/swe1_rules.rs`, `crates/pin2k-core/src/machine/scene_render.rs`, and `crates/pin2k-core/src/machine/fx/`.

- **Scene Intro (`Deff*SceneIntro`, while `intro_remaining_ms > 0`) — strictly isolated from gameplay HUD**:
  - Opaque black fullscreen backdrop (`RectDrawCmd`, `640×240`, `z=0`) preventing any background bleed-through,
  - 15 fps intro movie (`anim_*_intro_{movie,battle}`, `220×160` horizontally doubled `scale_x = 2` → `440×160`) centered inside the oval aperture at `(100, 31, z=200)`, starting at frame `0`,
  - Metallic oval bezel `anim_jedi_scene_intro_frame` (`522×195`) centered at `(59, 14, z=202)`,
  - Scene badge (`intro_badge_anim()`) horizontally centered (`320 - intro_badge_width()/2`, `y=136`, `z=203`) across the lower rim (matching `DeffDestroyerDroidSceneIntro::Run()`),
  - Bottom prompt (`z=204`, `font_shadow_11_data`, `(320, 224)`). Once `intro_remaining_ms == 0`, transitions cleanly to the **active scene** + gameplay HUD.

| Index | Debug Mode | Title | Intro Badge (`z=203`) | Active Gameplay Background (`z=1..8`) |
|---:|---|---|---|---|
| 0 | `scene:0` | Battle Droid Invasion | `anim_battle_droid_intro_badge` | `anim_bdroid_background` + MTT + droids + `anim_bdroid_on_stap` + crosshair + explosions |
| 1 | `scene:1` | Destroyer Droids | `anim_destroyer_droid_intro_badge` | `anim_destroyer_droid_background` + `s/m/l` roll/idle/shooting + explosions |
| 2 | `scene:2` | Federation Fighters | `anim_federation_fighter_intro_badge` | `anim_federation_fighter_background` (scroll) + control ships + fighters + flyby + zap |
| 3 | `scene:3` | Gungan Energy Battle | `anim_sub_escape_intro_badge` | `anim_gungan_seaweed` + bubbles + sub + opee + sando + `G U N G A N` |
| 4 | `scene:4` | Theed Hangar Escape | `anim_hangar_escape_intro_badge` | `anim_federation_fighter_cockpit` + `hangar_droid_run` + torpedo + explosions + exit movie |
| 5 | `scene:5` | AAT Hover Tank | `anim_hover_tank_intro_badge` | `anim_hover_tank_background` + `l/r/c` enter/float/shoot/explode + catapult + boomer ball |
| 6 | `scene:6` | Boonta Eve Podrace | `anim_podrace_intro_badge` | Unified `render_podrace_layers` (`starting`, sky, horizon, ground, rivals, Anakin) + race icons + `LAP`/`SPEED` HUD |
| 7 | `scene:7` | Sith Probe Droids | `anim_probe_droid_intro_badge` | `anim_sith_probe_background` (scroll) + hover/fly + Queen shooting + blasts/explosions |
| 8 | `scene:8` | Queen Amidala Wardrobe | `anim_queen_ship_intro_badge` | `anim_fashion_base` + throne + queen + wheel + sparkles + outfit icons + flash |
| 9 | `scene:9` | Jar Jar Juggle | `anim_jarjar_juggle_intro_badge` | `anim_jarjar_juggle_backdrop` `(0,50)` + `Juggle1/2 → Fall1/2 → JumpUp → Switch → Stare` + 5 lane rewards (`anim_jarjar_junk1..6`/`_eb`) + ballistic fly-off & `showoff_award_text` |
| 10 | `scene:10` | Watto's Chance Cube | `anim_wattos_chance_intro_badge` | `anim_watto_scene_background` + full Watto timeline (left/middle/right) + cube + flash/explode |
| 11 | `scene:11` | Mos Espa Cantina | `anim_musical_chairs_intro_badge` | `anim_jedi_council_background` + Mace/Yoda/members/neck/chic |
| 12 | `scene:12` | R2-D2 Repair | `anim_r2d2_intro_badge` | `anim_r2d2_scene_background_{upper,lower}` + R2 units (blue/red/dark_blue/yellow/red_white) |
| 13 | `scene:13` | Mystery (`?`) | `anim_question_mark_intro_badge` | `anim_question_mark_{blue,green}` + `anim_super_mystery_icon` + `MYSTERY AWARD` |

- **Other in-game views (`scene_render.rs` & `fx/overlays.rs`)**:
  - `holo_select` (`DeffObiHolo`), `hangar_multiball`, `darth_maul_duel` (`DeffJediMultiballBackground` + `DeffJediMultiballIntro`), `skill_shot`, `DeffWattoJunkShop`, `DeffMarketplace`, `DeffJarJar`, `RankPromotion`, `JediSpirit`.

---

## 5. Special Screens, System & Diagnostics (`system_screens.rs`)

| Debug Mode | `Deff` Class | Content & Layers |
|---|---|---|
| `boot` | `PowerupTestingDeff` | Williams Pinball 2000 / PRISM / PDB power-up self-test (`font_system_*`) |
| `bonus` | `DeffBonus` | `anim_bonus_background` + `anim_bonus_character_*` + `anim_bonus_icon_jedi_*` + base × multiplier |
| `match` | `DeffMatch` | `anim_match_movie` (`scale_x=2`) + 2-digit match roll `font_starwars_19_data` |
| `game_over` | `DeffGameOver` | `anim_darth_maul_game_over` (`scale_x=2`) + `GAME OVER` + final score |
| `game_intro` | `DeffGameIntro` | `anim_republic_cruiser_intro` (`scale_x=2`) + `STAR WARS EPISODE I` |
| `extra_ball` | `DeffExtraBallCollected` | `anim_extra_ball_show` (`scale_x=2`) + `EXTRA BALL!` |
| `ball_save` | `DeffBSBallSaved` | Centered overlay `BALL SAVED / SHOOT AGAIN` (`z=23..24`) |
| `tilt_warning` | `DeffTiltWarning` | Alert overlay `DANGER! DANGER!` (`z=239..240`) |
| `tilt` | `DeffTilt` | Translucent veil + `TILT / BALL LOST` (`z=239..240`) |
| `slam_tilt` | `DeffSlamTilt` | Fullscreen `SLAM TILT` (`z=239..240`) |
| `sub_escape` | `DeffSubEscapeSceneBackground` | Naboo underwater chase (`seaweed`, `bubbles_2`, `sub`, `chomp`) |
| `build_c3p0` | `DeffBuildC3P0Status` | `anim_build_c3p0` + Build C-3PO status |
| `ramp_combo` | `DeffRampCombo` | `anim_naboo_flyby_single` + `RAMP COMBO!` |
| `jedi_ready` | `DeffJediMultiballReadyBackground` | `anim_saber_ball_loop` + `JEDI MULTIBALL READY!` |
| `hstd_entry` | `DeffHSTDinitials` | Navy frame, 3 initial slots (`font_starwars_19_data`), `A..Z 0..9` strip |
| `diag:align` | Video Align | ROM alignment grid `system_video_test_align` (`640×240`) |
| `diag:colorbar` | `DiagnosticColorBarDeff` | 8 full-height RGB555 SMPTE color bars |
| `diag:grid` | `DiagnosticGridDeff` | White convergence grid on black + red border |
| `diag:rgb` | `DiagnosticRGBDeff` | 3 pure Red / Green / Blue bands |
| `diag:fuse` | `DiagnosticFuseDeff` | `F100..F105` fuse status + coin door interlock |
| `diag:switch` | `DiagnosticSwitchTestDeff` | Live 12×8 switch matrix (green = closed) + last named switch |
| `diag:lamp` | `DiagnosticSingleLampDeff` | Matrices A & B (128 lamps) with selected lamp highlight |
| `diag:coil` / `diag:sound` / `admin` / `debug` | `MenuGUIBackgndDeff` | Full operator menus navigable via coin door buttons (`1/2/3/4`) |

---

## 6. Web Debug Console (`--web-port`)

Started with `--web-port 8080`, the console serves a 5-tab UI (`web_ui.html`, `web_ui/{mod,state}.rs` with URL `percent_decode`):
1. **Tab 1 (`Modes & Screens`)**: direct launch of all states, 20 Attract slides, 14 SWE1 scenes, Multiballs, special screens, diagnostics, and RFM waves.
2. **Tab 2 (`Players & Time`)**: Pause/Resume, `+1 Frame (16 ms)` step, score add, player count/switch (`1..4`), ball (`1..3`), lasers (`0/5/9`), shield, `J-E-D-I` letters, Jedi rank.
3. **Tab 3 (`Anims & Sounds`)**: interactive catalog backed by `/api/assets` (392 animations + 689 DCS sounds).
4. **Tab 4 (`Hardware`)**: pulse coils, toggle lamps, trigger playfield switches.
5. **Tab 5 (`Live Layers`)**: real-time Z-layer inspector for the current frame.

**HTTP Endpoints:**
- `GET /api/state`, `GET /api/assets`, `GET /api/events`, `GET /api/db`, `GET /frame.png`, `GET /audio/{file}`
- `POST /api/debug/mode/{target}`, `POST /api/debug/anim/{name}`, `POST /api/debug/sound/{id}`, `POST /api/debug/score/{pts}`, `POST /api/debug/drive/{id}/{ms}`, `POST /api/debug/lamp/{id}/{val}`, `POST /api/debug/player/{cmd}`, `POST /api/debug/pause`, `POST /api/debug/step`, `POST /api/switch/{id}`.

---

## 7. Automated Screenshots, Frame Sequences & Video Capture

`pin2k-desktop` supports deterministic headless PNG capture, 60 Hz frame-sequence export, and live HTTP frame snapshots. All captures render the exact `640×480` scan-doubled composite output (or `2×` HD sprites when `--hd` is enabled).

> **Repository policy:** Curated documentation screenshots live in `docs/screenshots/*.png`; ad-hoc captures and video exports belong in `./work/` (which is gitignored via `.gitignore`).

### 7.0 Runtime Gallery (`640×480`)

| Attract (`attract:0`) | Nominal HUD (`ingame`) |
|---|---|
| ![Attract Mode Kahuna Logo](screenshots/attract_kahuna.png) | ![Nominal Gameplay HUD](screenshots/ingame_nominal.png) |
| **Virtual Spinner (`spinner_left`)** | **Scene Intro (`scene:6` Intro)** |
| ![Left Virtual Spinner](screenshots/spinner_left.png) | ![Scene Intro Bezel](screenshots/scene_intro.png) |
| **Boonta Eve Podrace (`scene:6`)** | **Jar Jar Juggle (`scene:9`)** |
| ![Boonta Eve Podrace](screenshots/scene_podrace.png) | ![Jar Jar Juggle](screenshots/scene_jarjar_juggle.png) |
| **End-of-Ball Bonus (`bonus`)** | **Diagnostics (`diag:colorbar`)** |
| ![End-of-Ball Bonus](screenshots/bonus.png) | ![Diagnostic Color Bar](screenshots/diag_colorbar.png) |

### 7.1 Single-Frame Screenshot (`--screenshot-out`)

Use `--frames <N>` together with `--screenshot-out <path.png>` to advance the simulation deterministically for `N` 60 Hz frames (`16 ms` each) and save the final `640×480` PNG:

```bash
# Nominal gameplay HUD (score bar, sabers, Naboo lasers, static spinners, Obi-Wan holo, Gungan seaweed)
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --debug-mode ingame --frames 20 --screenshot-out ./work/verify_ingame_hud.png

# Attract slide 0 (Star Wars Kahuna logo + 9-color light flare)
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --debug-mode attract:0 --frames 90 --screenshot-out ./work/verify_kahuna.png

# Left virtual spinner spinning with score popup
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --debug-mode spinner_left --frames 25 --screenshot-out ./work/verify_spinner.png

# Specific SWE1 scene (example: scene:9 Jar Jar Juggle, after 4.5s intro -> frames > 280)
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --debug-mode scene:9 --frames 320 --screenshot-out ./work/verify_jarjar_juggle.png

# HD 2x upscaled sprites (`--hd`, requires `pin2k-rom-tool upscale-assets`)
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --hd --debug-mode scene:6 --frames 250 --screenshot-out ./work/verify_podrace_hd.png

# Include lower-half playfield lamp/coil debug telemetry overlay
./target/release/pin2k-desktop --assets-dir ./work/swe1_assets --telemetry-overlay --debug-mode ingame --frames 30 --screenshot-out ./work/verify_telemetry.png
```

### 7.2 Frame Sequence Export & Video Encoding (`--frames-out-dir`)

Use `--frames-out-dir <dir>` to dump every simulated 60 Hz frame as `frame_0000.png`, `frame_0001.png`, ... (`640×480` RGB):

```bash
# Export 300 frames (5 seconds @ 60 Hz) of Boonta Eve Podrace
./target/release/pin2k-desktop \
  --assets-dir ./work/swe1_assets \
  --script podrace --frames 300 \
  --frames-out-dir ./work/podrace_frames

# Encode to H.264 MP4 (30 fps or 60 fps) with ffmpeg
ffmpeg -y -framerate 60 -i ./work/podrace_frames/frame_%04d.png \
  -c:v libx264 -pix_fmt yuv420p -crf 18 ./work/podrace.mp4

# Or encode a lightweight animated GIF
ffmpeg -y -framerate 30 -i ./work/podrace_frames/frame_%04d.png \
  -vf "fps=30,scale=640:-1:flags=lanczos" ./work/podrace.gif
```

### 7.3 Built-in Automated Scenarios (`--script`)

| `--script` | Description | Recommended `--frames` |
|---|---|---|
| `none` | Default idle / manual `--debug-mode` | `10..300` |
| `demo` | Scripted SWE1 gameplay showcase (start, skill shot, shield hits, scene) | `280` |
| `podrace` | Queues *Boonta Eve Podrace* (`scene:6`) and runs scripted ramp/target hits | `300` |
| `all-sequences` | Sequentially triggers SWE1 scenes, *Darth Maul Duel*, and *Hangar Multiball* | `1455` |
| `diagnostics` | Walks through diagnostic test pages | `120` |
| `admin` | Opens and navigates the Operator Admin menu | `48` |
| `rfm-demo` | Scripted *Revenge From Mars* (`#50070`) gameplay showcase | `280` |

### 7.4 Live Capture via Web Debug Console (`--web-port`)

When running interactively with `--web-port 8080 --frames 0`, you can pause on an exact frame, step frame-by-frame (`16 ms`), and download the current framebuffer directly over HTTP:

```bash
# 1. Jump to a mode or scene (example: Jar Jar Juggle)
curl -s -X POST http://127.0.0.1:8080/api/debug/mode/scene:9

# 2. Pause & step 1 frame at a time (16 ms)
curl -s -X POST http://127.0.0.1:8080/api/debug/pause
curl -s -X POST http://127.0.0.1:8080/api/debug/step

# 3. Inspect live Z-layers or save the exact 640x480 PNG framebuffer
curl -s http://127.0.0.1:8080/api/state | jq .
curl -s http://127.0.0.1:8080/frame.png -o ./work/live_capture.png
```
