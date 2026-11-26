# Custom Assets (`mods/darth_jarjar/custom_assets/`)

Drop any custom `.png` sprite, animation folder, or `.ogg` sound file here:

## Sprites & Animations
- **Single image**: `custom_assets/darth_jarjar.png`
- **Multi-frame animation**: `custom_assets/darth_jarjar/frame_0000.png`, `frame_0001.png`, ...

Map in `mods/darth_jarjar/mod.json` under `"anim_overrides"`:
```json
"anim_overrides": {
  "anim_jedi_battle_maul_5_move": "custom_assets/darth_jarjar.png",
  "anim_jedi_battle_maul_back_block_3": "custom_assets/darth_jarjar.png"
}
```

## Custom Audio (`.ogg`)
- **Sound / voice / music**: `custom_assets/darth_jarjar_laugh.ogg`

Map hex DCS-2 sound IDs in `mods/darth_jarjar/mod.json` under `"sound_overrides"` (works for both native OS audio and WebAudio):
```json
"sound_overrides": {
  "0x019A": "custom_assets/darth_jarjar_laugh.ogg"
}
```
