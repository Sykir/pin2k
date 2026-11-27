# Pin2k Project Conventions

1. **Active Repository**: Always work in `/workspace/pin2k` (persistent Kubernetes PVC).
2. **English-Only Documentation**: All Markdown documentation (`README.md`, `docs/*.md`) must be written in English, regardless of the conversation language.
3. **Safety**: Keep `pin2k-core` 100% safe Rust (`zero unsafe`).
4. **Secret Hygiene**: Never write tokens, PATs, or credentials into `.git/config`, tracked files, or responses. When pushing to an external remote requiring a token, pass it transiently via `git -c http.<url>.extraheader=...` only when requested.
5. **Audio & Debug Stack Invariants**:
   - Follow `upg_background_music` priorities (`0x0001` Ball 1 shooter -> `0x0003` Ball 2+ shooter -> `0x0002` `MAINPLAY_LOOP` once `playfield_valid` is true).
   - Starting any `looping = true` track must cancel any previous looping track regardless of `SoundId`; WebAudio must consume ordered `sound_events` (including `stop_all`) without dropping same-frame transitions, and `sound_stack` must prune expired SFX so only currently audible sounds appear active.
6. **Virtual Spinners & Mod Assets**:
   - Virtual spinners are active and rendered only in nominal SWE1 gameplay (no active scene, multiball, overlay, or special screen).
   - When adding multi-frame PNGs under `mods/<mod>/custom_assets/<anim>/`, ensure `.gitignore` un-ignores `custom_assets/**/*.png`.
7. **SWE1 Shield & Trap Polarity**:
   - `shield_raised = false` (DOWN) blocks `SHIELD_POPPER` (`Sw32`) so center shots hit `SHIELD_HIT` (`Sw39`).
   - `shield_raised = true` (UP) exposes `SHIELD_POPPER` (`Sw32`) behind the shield. Any mode or scene that raises the shield for a trap shot must wire `sw::SHIELD_POPPER` in `swe1_playfield.rs` and handle `SHIELD_POPPER` directly (not only `SHIELD_HIT`).
