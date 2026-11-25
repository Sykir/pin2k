# Étude Technique : Upscaling & Rendu « HD » dans `pin2k`

Ce document analyse les options techniques, contraintes (transparence RGB555, mémoire, horloge FPS) et recommandations pour passer du rendu natif **`640×240` → `640×480`** à un rendu **HD (`1280×960`, `1920×1080` ou `4K`)** dans `pin2k`.

---

## 1. Contraintes du Pipeline Actuel

- **Résolution native ROM & Compositeur Z-trié** : la carte PRISM / MediaGX génère une image **`640×240` RGB555 (15 bits)** doublée verticalement en **`640×480`** (scan-doubler pour la vitre Pepper's Ghost du flipper). `SoftwareCompositor` implémente le tri Z complet (`BaseViewManager`) avec miroir `flip_x`, teinte monochrome RGB555 et rendu des **19 polices bitmap ROM** avec ombres.
- **Films vs Sprites** :
  - Les films d'intro (codec `30`, `220×160` @ 15 fps, ex. `anim_podrace_intro_movie`, `anim_battle_droid_intro_battle`) sont affichés derrière le cadre métallique `anim_jedi_scene_intro_frame` (`z=202`) qui masque proprement leurs bords.
  - Les cinématiques plein écran `320×240` (ex. `anim_darth_maul_game_over`, `anim_match_movie`, `anim_extra_ball_show`, `anim_multiball_start_movie`) sont étirées **`scale_x = 2`** en `640×240`.
  - Les sprites et décors (codecs `0, 1, 20..23, 42..45`) utilisent la couleur clé **`0x7C1F` (magenta)** convertie en canal alpha RGBA (`0` ou `255`).
- **Mémoire (`LoadedAnim`)** : actuellement `compositor.rs` décode à la demande toutes les frames d'une animation et les conserve en RAM (`decoded_frames: Vec<RgbaImage>`).

---

## 2. Les Deux Approches d'Upscaling

### Approche A — Upscaling Offline des Assets (`gfx_hd/` + Packs HD)

Consiste à pré-calculer des versions HD (`2×` = `1280×480`, `3×` = `1920×720`) des `.png` extraits :

| Type d'asset | Algorithme recommandé | Pourquoi |
|---|---|---|
| **Sprites 2D / UI / Icônes** (`comp 0, 1, 20..23`) | **xBRZ** / **Scale2x** / **HQx** | Lisse les escaliers pixel-art en préservant les contours nets, sans flou. |
| **Films 3D pré-rendus** (`comp 30` @ 15 fps, `220×160`) | **Real-ESRGAN / Waifu2x** ou **Lanczos + Deband** | Reconstruit les textures 3D et réduit le banding 15-bit RGB555. |
| **Polices Bitmap (`fonts/`)** | **xBRZ 2×/3×** ou remplacement TTF/OTF | Garde des textes parfaitement lisibles à haute résolution. |

#### Piège critique : Franges Alpha (`0x7C1F` / halos colorés)
Un rescale classique (bilinéaire, bicubique, Lanczos, IA) appliqué directement sur un PNG RGBA mélange les pixels opaques avec le noir/magenta des pixels transparents (`alpha = 0`), créant un halo sale autour des sprites.

**Solution obligatoire** pour tout upscaling offline :
1. **Edge-padding (dilatation RGB)** : propager la couleur des pixels opaques sur les pixels transparents voisins avant de redimensionner.
2. **Séparation RGB / Alpha** : upscaler la couche RGB (avec l'algorithme choisi) et le canal Alpha séparément (ou en alpha pré-multiplié).
3. **Recomposition** : réassembler RGBA et, pour les sprites nets, appliquer un léger seuillage alpha si nécessaire.

#### Contrainte Mémoire (RAM)
- En `640×240`, 1 frame RGBA = `614 Ko`.
- En `3×` (`1920×720`), 1 frame RGBA = **`5,5 Mo`** (`9×` plus lourd). Une animation de 100 frames (ex. `anim_podrace_1`) passerait à **550 Mo** en RAM si toutes les frames restaient en cache.
- **Prérequis** : ajouter un **cache LRU borné** (ex. max 128 Mo ou 64 frames en mémoire) dans `LoadedAnim` avant d'activer des packs `3×`/`4×`.

---

### Approche B — Upscaling Temps Réel dans `pin2k-desktop`

| Variante | Où | Avantages | Limites |
|---|---|---|---|
| **WebGL2 Shader dans le navigateur (`web_ui.rs`)** | Côté client HTTP | **0 coût CPU serveur**, flux `/frame.png` reste en `640×240` léger, shaders **xBRZ / Bicubic / Scanlines CRT** instantanés à 60 Hz. | Uniquement pour le mode Web (`--web-port`). |
| **Compositor CPU HD (`compositor.rs`)** | CPU Rust | Fonctionne sans GPU ni fenêtre OS (`--screenshot-out`, `--frames-out-dir`). | Coût CPU `4×` à `9×` plus élevé sur l'encodage PNG et le blitting logiciel. |
| **Post-traitement GPU natif (`wgpu`)** | GPU Rust (Win/Linux) | Rendu natif `1080p`/`4K` à 60 fps avec shaders xBRZ + CRT. | Ajoute une dépendance GPU (`wgpu`/`winit`) au runtime headless actuel. |

---

## 3. Recommandation pour `pin2k` (Plan d'évolution)

1. **Étape 1 — Gain immédiat sans surcoût CPU : Shaders WebGL2 dans `web_ui.rs`**
   - Conserver le flux interne `640×240` côté serveur et proposer un sélecteur de rendu dans la page Web : `Native Pixel`, `HD Smooth (Bicubic)`, `HD Pixel (xBRZ)`, `CRT Scanlines`.
2. **Étape 2 — Support des Packs HD (`gfx_hd/` + cache LRU)**
   - Permettre à `SoftwareCompositor` de charger un dossier optionnel `gfx_hd/` (prioritaire sur `gfx/`, avec facteur d'échelle détecté automatiquement via `width / native_width`) et borner `decoded_frames` avec un cache LRU.
3. **Étape 3 — Outil d'upscaling intégré (`pin2k-rom-tool upscale-assets`)**
   - Ajouter une sous-commande pur-Rust (ex. `xBRZ` + `Lanczos3` avec *edge-padding* alpha automatique) générant `gfx_hd/` et `gfx_manifest.json` directement depuis les assets extraits.

---

## Conclusion

**Oui, un rendu HD est tout à fait réalisable** sans casser l'architecture clean-room ni les performances :
- Pour les **sprites** : **xBRZ + edge-padding alpha** (offline ou shader).
- Pour les **films `220×160` 15 fps** : **Lanczos / IA (Real-ESRGAN)** en pack `gfx_hd/` optionnel + **cache LRU** dans `compositor.rs`.
- Pour l'**affichage Web** : un **shader WebGL2** côté client permet déjà d'obtenir un rendu HD lissé ou CRT immédiatement sans alourdir `pin2k-desktop`.
