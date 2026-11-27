---
name: pin2k-scene-lifting
description: >-
  Extract authentic Pinball 2000 scene logic, reward tables, switch-to-lane mappings,
  animation state machines, and exact (x, y, z) coordinates from swe1_0200.elf and port
  them into modular Rust files in pin2k-core. Use when asked to reverse-engineer or
  implement an authentic SWE1 or RFM scene.
---

# Pinball 2000 ROM Scene Lifting Runbook

Use this workflow when porting a Pinball 2000 scene (e.g. `JarJarJuggle`, `WattosChance`,
`BattleDroid`, etc.) from `work/swe1_0200.elf` into `crates/pin2k-core/src/machine/scenes/`.

## 1. Inventory Classes, Symbols, Rulebooks, and Data Tables
1. Read the Ghidra decompilations in `work/ghidra_out/classes/` for the scene's
   `<Scene>.c`, `Deff<Scene>.c`, `Deff<Scene>Intro.c`, and `Class<Scene>Leff*.c`.
2. **Cross-check the rulebook docs in `work/` (if present)**:
   Search `work/swe1-regles/` (e.g., `livre_regles_swe1_14_essentiel.md`,
   `livre_regles_swe1_210_essentiel.md`, `livre_regles_swe1_14.md`) for the scene's
   gameplay rules, progressive/combo scoring windows, miss/raté flags, shield/drop-target
   interactions, and perfect-completion bonuses to ensure nothing is missed in Ghidra.
3. **Ghidra `Font::Font` truncation check**: if `Deff*::Run` stops right after
   `Font__Font_FontData_const___` with `/* WARNING: Subroutine does not return */`,
   the decompilation is truncated. Disassemble the full address range up to the next
   symbol using `objdump -d -C --start-address=<Run> --stop-address=<NextSym> work/swe1_0200.elf`.
3. Resolve data tables with `nm -C work/swe1_0200.elf`:
   - `.rodata` tables (e.g., `ra_*_random_award_table`): read directly from ELF sections.
   - `.bss` tables (e.g., `*_table_data`): disassemble `global constructors keyed to *_cpp_rsrc`
     to recover exact ID -> `anim_*_ptr` mappings.

## 2. Extract Exact Sprite Dimensions & Timings
- Query `work/swe1_assets/gfx/gfx_manifest.json` (`m["animations"]` entries with keys
  `symbol_name`, `width`, `height`, `frame_count`, `fps`) for all `anim_*` used by the scene.
- Convert ROM `sleep100(n)` delays using `n * 10 ms` per frame.
- **Exact `(x, y, z)` & anchor math**: Never estimate sprite positions. Disassemble
  `Deff*::Run`, `*_process`, and helper positioning methods (`Member::position_slot`,
  `Member::jump_to_slot`, `SetPosStyle`) to recover exact coordinates, parent-relative
  offsets, and center/bottom-aligned anchors (`center_x - width / 2`, `bottom_y - height`).
- **Hardware & playfield invariants**: Check `<Scene>.c` and `work/swe1-regles/` for
  shield state transitions (`set_shield_raised`), drop-target hooks (`LEFT/RIGHT_DROP_TARGET`),
  and laser action button hooks (`hook_left_laser_fired` / `hook_right_laser_fired`).

## 3. Implement Modular Rust Scene (`<= 300` Lines per File)
1. Split into two cohesive files under `crates/pin2k-core/src/machine/scenes/`:
   - `<scene>_state.rs`: pure data structs, award tables, animation state specs, and PRNG/init.
   - `<scene>.rs`: `start_*`, `tick_*`, `handle_*_switch`, and `render_*` methods on `Pin2kMachine`.
2. Wire into `machine/mod.rs`, `machine/swe1_playfield.rs` (switch hooks, start, completion),
   and `machine/scene_render.rs`.
3. **Remove legacy decorative stubs** for that scene in `machine/fx/scene_extras.rs` and
   `machine/fx/mod.rs` to avoid double-rendering sprites or dead-code warnings.

## 4. Verify & Track
1. Add a unit test in `crates/pin2k-core/src/machine/tests.rs` asserting exact sprite counts
   (catches double-rendering!), switch-to-lane/target mapping, animation state transitions,
   duplicate-hit handling, and scene completion.
2. Check line counts (`wc -l`) on all new/modified files (`<= 300` lines).
3. Run:
   `RUSTFLAGS="-D warnings" cargo check --workspace --all-targets && cargo test --workspace && cargo build --release --workspace`
4. Mark the scene `[x]` in `docs/TODO.md`.
