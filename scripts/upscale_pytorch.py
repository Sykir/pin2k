#!/usr/bin/env python3
"""Offline PyTorch HD sprite upscaler for Pinball 2000 (`gfx/` -> `gfx_hd/`).

Supports:
  - Real-ESRGAN / SwinIR `.pth` weights via `spandrel` (when `--model` is provided)
  - Built-in PyTorch bicubic antialias (`F.interpolate`) fallback when no `--model` is given

Always performs RGB edge-padding into `alpha == 0` pixels before upscaling and
upscales the 1-bit silhouette alpha mask separately so Pinball 2000 sprites remain
halo-free at 2x/3x/4x output resolutions (`pin2k-desktop --hd`).
"""

import argparse
import json
from pathlib import Path

import numpy as np
from PIL import Image
import torch
import torch.nn.functional as F


def edge_pad_rgb(rgba: np.ndarray, passes: int = 2) -> np.ndarray:
    """Dilates opaque RGB into adjacent `alpha == 0` pixels to prevent dark/magenta halos."""
    out = rgba.copy()
    known = out[:, :, 3] > 0
    rgb = out[:, :, :3].astype(np.float32)

    for _ in range(passes):
        if known.all():
            break
        acc = np.zeros_like(rgb)
        cnt = np.zeros(known.shape, dtype=np.float32)
        for dy in (-1, 0, 1):
            for dx in (-1, 0, 1):
                if dx == 0 and dy == 0:
                    continue
                shifted_k = np.roll(np.roll(known, dy, axis=0), dx, axis=1)
                shifted_rgb = np.roll(np.roll(rgb, dy, axis=0), dx, axis=1)
                # Zero out wrap-around borders
                if dy == -1:
                    shifted_k[-1, :] = False
                elif dy == 1:
                    shifted_k[0, :] = False
                if dx == -1:
                    shifted_k[:, -1] = False
                elif dx == 1:
                    shifted_k[:, 0] = False
                mask = (~known) & shifted_k
                acc[mask] += shifted_rgb[mask]
                cnt[mask] += 1.0
        fill = cnt > 0
        rgb[fill] = acc[fill] / cnt[fill, None]
        known = known | fill

    out[:, :, :3] = np.clip(rgb, 0, 255).astype(np.uint8)
    return out


def load_pytorch_model(model_path: str | None, device: torch.device):
    if not model_path:
        return None
    try:
        import spandrel  # type: ignore
    except ImportError as exc:
        raise SystemExit(
            "Install `spandrel` (`pip install spandrel`) to load .pth super-resolution models, "
            "or omit --model to use PyTorch bicubic antialias."
        ) from exc
    model = spandrel.ModelLoader().load_from_file(model_path)
    model.to(device).eval()
    return model


@torch.no_grad()
def upscale_frame(
    rgba: np.ndarray, scale: int, device: torch.device, model=None
) -> np.ndarray:
    padded = edge_pad_rgb(rgba, passes=2)
    h, w, _ = padded.shape
    dst_h, dst_w = h * scale, w * scale

    rgb_t = (
        torch.from_numpy(padded[:, :, :3])
        .to(device=device, dtype=torch.float32)
        .permute(2, 0, 1)
        .unsqueeze(0)
        / 255.0
    )
    alpha_t = (
        torch.from_numpy(padded[:, :, 3])
        .to(device=device, dtype=torch.float32)
        .unsqueeze(0)
        .unsqueeze(0)
        / 255.0
    )

    if model is not None:
        up_rgb = model(rgb_t)
        if up_rgb.shape[-2:] != (dst_h, dst_w):
            up_rgb = F.interpolate(
                up_rgb, size=(dst_h, dst_w), mode="bicubic", align_corners=False, antialias=True
            )
    else:
        up_rgb = F.interpolate(
            rgb_t, size=(dst_h, dst_w), mode="bicubic", align_corners=False, antialias=True
        )

    up_alpha = F.interpolate(
        alpha_t, size=(dst_h, dst_w), mode="bicubic", align_corners=False
    )
    # Keep 1-bit crisp alpha while preserving edge-padded RGB on transparent pixels.
    alpha_bin = (up_alpha.squeeze(0).squeeze(0) >= 0.5).to(torch.uint8) * 255
    rgb_u8 = (up_rgb.squeeze(0).clamp(0.0, 1.0) * 255.0).round().to(torch.uint8).permute(1, 2, 0)

    out = torch.cat([rgb_u8, alpha_bin.unsqueeze(-1)], dim=-1).cpu().numpy()
    return out


def main() -> None:
    ap = argparse.ArgumentParser(description="PyTorch HD upscaler for Pinball 2000 gfx/ -> gfx_hd/")
    ap.add_argument("--assets-dir", type=Path, default=Path("work/swe1_assets"))
    ap.add_argument("--scale", type=int, default=2, choices=[2, 3, 4, 8])
    ap.add_argument("--model", type=str, default=None, help="Optional Real-ESRGAN/SwinIR .pth path")
    ap.add_argument("--anim", type=str, default=None, help="Optional animation substring filter")
    args = ap.parse_args()

    device = torch.device(
        "cuda" if torch.cuda.is_available() else ("mps" if torch.backends.mps.is_available() else "cpu")
    )
    model = load_pytorch_model(args.model, device)
    print(f"Using PyTorch device={device}, scale={args.scale}x, model={args.model or 'bicubic'}")

    src_gfx = args.assets_dir / "gfx"
    dst_gfx = args.assets_dir / "gfx_hd"
    dst_gfx.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((src_gfx / "gfx_manifest.json").read_text())

    flt = args.anim.lower() if args.anim else None
    out_anims = []
    total_frames = 0
    for entry in manifest["animations"]:
        if flt and flt not in entry["symbol_name"].lower():
            continue
        exported = []
        for rel in entry["exported_frames"]:
            src_path = src_gfx / rel
            if not src_path.exists():
                continue
            rgba = np.array(Image.open(src_path).convert("RGBA"))
            up = upscale_frame(rgba, args.scale, device, model)
            dst_path = dst_gfx / rel
            dst_path.parent.mkdir(parents=True, exist_ok=True)
            Image.fromarray(up, mode="RGBA").save(dst_path)
            exported.append(rel)
            total_frames += 1
        if exported:
            entry["width"] *= args.scale
            entry["height"] *= args.scale
            entry["exported_frames"] = exported
            out_anims.append(entry)

    manifest["animations_exported"] = len(out_anims)
    manifest["animations"] = out_anims
    (dst_gfx / "gfx_manifest.json").write_text(json.dumps(manifest, indent=2))
    print(f"Upscaled {len(out_anims)} animations ({total_frames} frames) -> {dst_gfx / 'gfx_manifest.json'}")


if __name__ == "__main__":
    main()
