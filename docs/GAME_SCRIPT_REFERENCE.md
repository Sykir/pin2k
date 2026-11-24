# Référence du Format `game_script.json` & Extraction Gamecode

Ce document décrit le format de script **`game_script.json`** (schéma `v1`) généré par `pin2k-rom-tool extract-gamecode` et exécuté par `ScriptRuntime` dans `pin2k-core`.

---

## 1. Pipeline d'Extraction (`extract-gamecode`)

`pin2k-rom-tool extract-gamecode` inspecte directement `game.rom` + `symbols.rom` (et optionnellement `prism_flat.bin`) :

1. **Résolution des tables matérielles dans `.data`** :
   - `DriveTable` : 48 entrées × 24 octets (`flags`, `default_pulse_ms`, noms EN/DE/FR/ES).
   - `LampTable` : 128 entrées × 36 octets (`flags`, noms EN/DE/FR/ES).
   - `SwitchTable` : 96 entrées × 48 octets (`sw_type`, `handler_addr`, `arg`, `debounce_ms`, noms EN/DE/FR/ES).
2. **Désassemblage statique x86-32 (`lift_x86_calls`)** :
   - Parcourt le code machine de chaque handler `swd_*` jusqu'au `RET` (`0xC3`/`0xC2`).
   - Enregistre chaque `CALL rel32` (`0xE8`) résolu via `symbols.rom` et les arguments empilés (`PUSH imm8` `0x6A`, `PUSH imm32` `0x68`).
   - Traduit automatiquement les appels `generic_score`, `score(u64)` et `AudioRequest(desc*)` en `ScriptAction` (`AddScore`, `PlaySound`).
3. **Export des règles, modes et attract** :
   - Génère la section `meta` (détection auto des switchs Trough, Shooter Lane, Start Button), `hardware`, `attract`, `vars`, `modes` et `rules`.

---

## 2. Structure de `game_script.json`

```json
{
  "schema_version": 1,
  "meta": {
    "game_number": 50069,
    "title": "Star Wars Episode I",
    "version": "2.00",
    "opto_nc_mask_lo": 1031312277632,
    "opto_nc_mask_hi": 0,
    "balls_per_game": 3,
    "ball_save_secs": 12,
    "trough_switches": [25, 26, 27, 28],
    "shooter_lane_switch": 7,
    "start_button_switch": 2
  },
  "hardware": {
    "switches": [ ... ],
    "drives": [ ... ],
    "lamps": [ ... ]
  },
  "attract": [
    {
      "name": "P2kLogo",
      "duration_ms": 6000,
      "anim_hint": "anim_p2k_logo_animated",
      "banner": "PINBALL 2000",
      "require_last_game": false
    }
  ],
  "vars": {
    "shield_hits": 0,
    "hangar_locks": 0
  },
  "modes": [ ... ],
  "rules": [ ... ]
}
```

---

## 3. Triggers, Conditions et Actions

### `ScriptTrigger` (`trigger.type`)

| Type | Paramètres | Déclenchement |
|---|---|---|
| `on_boot` | — | Au chargement du script dans `Pin2kMachine::load_game_script` |
| `on_ball_start` | — | Au lancement d'une nouvelle bille |
| `on_ball_end` | — | À la perte de la bille |
| `on_switch` | `sw_id: u8` | À la fermeture logique du switch `sw_id` (en jeu) |
| `on_tick` | — | À chaque tick 60 Hz (`16 ms`) |
| `on_mode_enter` | `mode: String` | À l'activation du mode `mode` |
| `on_mode_exit` | `mode: String` | À la sortie ou expiration du mode `mode` |
| `on_timer` | `timer: String` | Lorsque le compteur `timer` atteint `0` |

### `ScriptCondition` (`conditions[].op`)

| Opérateur | Paramètres | Sémantique |
|---|---|---|
| `ball_in_play` | — | Vrai si `balls_in_play > 0` et état `InGame` |
| `not_tilted` | — | Vrai si la machine n'est pas en état `Tilt` |
| `mode_active` | `mode: String` | Vrai si `mode` est actif |
| `mode_inactive` | `mode: String` | Vrai si `mode` est inactif |
| `var_eq` / `var_ge` / `var_lt` | `var: String, value: i64` | Compare `vars[var]` à `value` (`==`, `>=`, `<`) |
| `bit_set` / `bit_clear` | `var: String, bit: u8` | Teste le bit `bit` (`0..63`) de `vars[var]` |

### `ScriptAction` (`actions[].action`)

| Action | Paramètres | Effet |
|---|---|---|
| `add_score` | `points: u64` | Ajoute `points` (× multiplicateur) au joueur actif |
| `set_var` / `inc_var` | `var: String, value / delta: i64` | Affecte ou incrémente `vars[var]` |
| `set_bit` / `clear_var` | `var: String, [bit: u8]` | Positionne un bit ou remet `vars[var]` à `0` |
| `pulse_drive` | `drive_id: u8, ms: u8` | Pulse la bobine/flasher `drive_id` pendant `ms` ms |
| `set_lamp` | `lamp_id: u8, on: bool` | Allume ou éteint la lampe `lamp_id` |
| `play_sound` / `play_music` | `sound_id: u16` | Joue un effet sonore ou une musique DCS |
| `play_anim` | `anim: String` | Affiche l'animation `anim` (clé `gfx_manifest.json`) |
| `show_banner` | `text: String` | Affiche le bandeau texte pendant 3 s |
| `start_mode` / `end_mode` | `mode: String` | Démarre ou termine un mode (`modes[]`) |
| `start_timer` | `name: String, ticks: u32` | Démarre un timer décrémenté à chaque frame 60 Hz |
| `start_multiball` | `balls: u8` | Éjecte les billes supplémentaires jusqu'à `balls` en jeu |
| `award_extra_ball` | — | Accorde une Extra Ball au joueur actif |
| `advance_scene` | — | Avance ou démarre la scène Holoprojecteur suivante |
| `collect_bonus` / `eject_ball` | — | Encaisse le bonus / demande l'éjection d'une bille |
