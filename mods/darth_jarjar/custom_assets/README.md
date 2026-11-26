# Custom Assets (`mods/darth_jarjar/custom_assets/`)

Drop any custom `.png` sprite or animation folder here:

- **Single image**: `custom_assets/darth_jarjar.png`
- **Multi-frame animation**: `custom_assets/darth_jarjar/frame_0000.png`, `frame_0001.png`, ...

Then map it in `mods/darth_jarjar/mod.json` under `"anim_overrides"`, e.g.:

```json
"anim_overrides": {
  "anim_jedi_battle_maul_5_move": "custom_assets/darth_jarjar.png",
  "anim_jedi_battle_maul_back_block_3": "custom_assets/darth_jarjar.png"
}
```
