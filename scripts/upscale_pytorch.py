#!/usr/bin/env python3
"""Offline PyTorch HD sprite upscaler for Pinball 2000 (`gfx/` -> `gfx_hd/`).

Supports:
  - Neural super-resolution `.pth` / `.safetensors` weights via `spandrel`
    (Real-ESRGAN, MoSR, PLKSR, SwinIR, DAT, ATD, etc.) when `--model` is provided.
  - Built-in PyTorch bicubic antialias (`F.interpolate`) fallback when `--model` is omitted.

Pipeline per frame:
  1. `edge_pad_rgb`: dilates opaque RGB into adjacent `alpha == 0` pixels (2 passes)
     to prevent dark or magenta (`0x7C1F`) border fringes.
  2. `infer_tiled`: splits large frames using `--tile` (16 px overlap) and replicate-pads
     tiles to multiples of 8 so odd-sized sprites never fail model shape checks.
  3. Separate 1-bit silhouette alpha upscaling (`>= 0.5` threshold) for crisp sprites.
  4. Real-time percentage progress bar with throughput (`img/s`) and ETA.

Note: `pin2k-desktop --hd` reads `<assets-dir>/gfx_hd/`. Use `--output-dir` to target
`gfx_hd/` directly, or compare model outputs in their default `gfx_hd_<model_name>/`
folders before renaming your preferred pack to `gfx_hd/`.
"""

import argparse
import json
import re
import time
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


def load_pytorch_model(model_path: str | None, device: torch.device, use_fp16: bool = False):
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
    if use_fp16 and device.type == "cuda" and getattr(model, "supports_half", False):
        model.half()
    return model


def infer_tiled(
    model,
    rgb_in: torch.Tensor,
    model_scale: int,
    tile_size: int,
    tile_pad: int,
    use_fp16: bool,
) -> torch.Tensor:
    _, _, h, w = rgb_in.shape
    use_half_weights = next(model.model.parameters()).dtype == torch.float16
    if tile_size <= 0 or (h <= tile_size and w <= tile_size):
        x = rgb_in.half() if use_half_weights else rgb_in
        if use_fp16 and not use_half_weights and rgb_in.device.type == "cuda":
            amp_dtype = torch.bfloat16 if torch.cuda.is_bf16_supported() else torch.float32
            with torch.amp.autocast("cuda", dtype=amp_dtype, enabled=amp_dtype != torch.float32):
                return model(x).float()
        return model(x).float()

    out = torch.zeros((1, 3, h * model_scale, w * model_scale), device=rgb_in.device, dtype=torch.float32)
    for y in range(0, h, tile_size):
        for x in range(0, w, tile_size):
            x0 = max(x - tile_pad, 0)
            y0 = max(y - tile_pad, 0)
            x1 = min(x + tile_size + tile_pad, w)
            y1 = min(y + tile_size + tile_pad, h)
            # Pad tile to multiple of 8
            th, tw = y1 - y0, x1 - x0
            ph = (8 - th % 8) % 8
            pw = (8 - tw % 8) % 8
            tile = rgb_in[:, :, y0:y1, x0:x1]
            if ph or pw:
                tile = F.pad(tile, (0, pw, 0, ph), mode="replicate")
            tin = tile.half() if use_half_weights else tile
            if use_fp16 and not use_half_weights and rgb_in.device.type == "cuda":
                amp_dtype = torch.bfloat16 if torch.cuda.is_bf16_supported() else torch.float32
                with torch.amp.autocast("cuda", dtype=amp_dtype, enabled=amp_dtype != torch.float32):
                    tout = model(tin).float()
            else:
                tout = model(tin).float()
            if ph or pw:
                tout = tout[:, :, : th * model_scale, : tw * model_scale]

            ox0 = (x - x0) * model_scale
            oy0 = (y - y0) * model_scale
            ox1 = ox0 + min(tile_size, w - x) * model_scale
            oy1 = oy0 + min(tile_size, h - y) * model_scale
            out[
                :,
                :,
                y * model_scale : min(y + tile_size, h) * model_scale,
                x * model_scale : min(x + tile_size, w) * model_scale,
            ] = tout[:, :, oy0:oy1, ox0:ox1]
    return out


@torch.no_grad()
def upscale_frame_tensors(
    rgba: np.ndarray,
    scale: int,
    device: torch.device,
    model=None,
    model_scale: int = 4,
    use_fp16: bool = False,
    tile_size: int = 256,
    blend: float = 1.0,
) -> tuple[torch.Tensor, torch.Tensor]:
    """Upscales a single RGBA frame into `(up_rgb [1,3,H,W], up_alpha [1,1,H,W])` float32 tensors on `device`."""
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

    bicubic_rgb = F.interpolate(
        rgb_t, size=(dst_h, dst_w), mode="bicubic", align_corners=False, antialias=True
    )

    if model is not None:
        pad_h = (8 - h % 8) % 8
        pad_w = (8 - w % 8) % 8
        rgb_in = F.pad(rgb_t, (0, pad_w, 0, pad_h), mode="replicate") if (pad_h or pad_w) else rgb_t
        up_rgb = infer_tiled(model, rgb_in, model_scale, tile_size, tile_pad=16, use_fp16=use_fp16)
        if pad_h or pad_w:
            up_rgb = up_rgb[:, :, : h * model_scale, : w * model_scale]
        if up_rgb.shape[-2:] != (dst_h, dst_w):
            up_rgb = F.interpolate(
                up_rgb, size=(dst_h, dst_w), mode="bicubic", align_corners=False, antialias=True
            )
        if blend < 1.0:
            up_rgb = blend * up_rgb + (1.0 - blend) * bicubic_rgb
    else:
        up_rgb = bicubic_rgb

    up_alpha = F.interpolate(
        alpha_t, size=(dst_h, dst_w), mode="bicubic", align_corners=False
    )
    return up_rgb.clamp(0.0, 1.0), up_alpha.clamp(0.0, 1.0)


@torch.no_grad()
def apply_temporal_stabilization(
    cur_src_rgba: np.ndarray,
    prev_src_rgba: np.ndarray | None,
    up_rgb: torch.Tensor,
    up_alpha: torch.Tensor,
    prev_up_rgb: torch.Tensor | None,
    prev_up_alpha: torch.Tensor | None,
    strength: float,
    threshold: float,
) -> tuple[torch.Tensor, torch.Tensor]:
    """Motion-aware temporal EMA guided by 1x source deltas to suppress static-region flicker without ghosting."""
    if prev_src_rgba is None or prev_up_rgb is None or prev_up_alpha is None:
        return up_rgb, up_alpha
    if cur_src_rgba.shape != prev_src_rgba.shape or up_rgb.shape != prev_up_rgb.shape:
        return up_rgb, up_alpha

    # Measure per-pixel change in 1x source space (max across RGBA channels, normalized 0..1)
    diff_1x = np.abs(cur_src_rgba.astype(np.float32) - prev_src_rgba.astype(np.float32)).max(axis=2) / 255.0
    diff_t = torch.from_numpy(diff_1x).to(device=up_rgb.device, dtype=torch.float32).unsqueeze(0).unsqueeze(0)
    # Slightly dilate motion regions (3x3 max pool) so edges of moving objects stay ghost-free
    diff_t = F.max_pool2d(diff_t, kernel_size=3, stride=1, padding=1)
    diff_up = F.interpolate(diff_t, size=up_rgb.shape[-2:], mode="bilinear", align_corners=False)

    static_mask = (1.0 - diff_up / max(threshold, 1e-4)).clamp(0.0, 1.0)
    w = (strength * static_mask).clamp(0.0, 0.95)
    stab_rgb = (1.0 - w) * up_rgb + w * prev_up_rgb
    stab_alpha = (1.0 - w) * up_alpha + w * prev_up_alpha
    return stab_rgb, stab_alpha


def tensors_to_rgba_numpy(up_rgb: torch.Tensor, up_alpha: torch.Tensor, soft_alpha: bool) -> np.ndarray:
    if soft_alpha:
        alpha_u8 = (up_alpha.squeeze(0).squeeze(0).clamp(0.0, 1.0) * 255.0).round().to(torch.uint8)
    else:
        alpha_u8 = (up_alpha.squeeze(0).squeeze(0) >= 0.5).to(torch.uint8) * 255
    rgb_u8 = (up_rgb.squeeze(0).clamp(0.0, 1.0) * 255.0).round().to(torch.uint8).permute(1, 2, 0)
    return torch.cat([rgb_u8, alpha_u8.unsqueeze(-1)], dim=-1).cpu().numpy()


@torch.no_grad()
def upscale_frame(
    rgba: np.ndarray,
    scale: int,
    device: torch.device,
    model=None,
    model_scale: int = 4,
    use_fp16: bool = False,
    tile_size: int = 256,
    blend: float = 1.0,
    soft_alpha: bool = False,
) -> np.ndarray:
    up_rgb, up_alpha = upscale_frame_tensors(
        rgba,
        scale,
        device,
        model=model,
        model_scale=model_scale,
        use_fp16=use_fp16,
        tile_size=tile_size,
        blend=blend,
    )
    return tensors_to_rgba_numpy(up_rgb, up_alpha, soft_alpha=soft_alpha)


def format_eta(seconds: float) -> str:
    if seconds <= 0 or not np.isfinite(seconds):
        return "--:--"
    secs = int(seconds)
    mins, secs = divmod(secs, 60)
    hours, mins = divmod(mins, 60)
    if hours > 0:
        return f"{hours:d}h{mins:02d}m{secs:02d}s"
    return f"{mins:02d}m{secs:02d}s"


def print_progress(done: int, total: int, start_t: float, width: int = 28) -> None:
    if total <= 0:
        return
    frac = min(1.0, done / total)
    filled = int(round(frac * width))
    bar = "#" * filled + "-" * (width - filled)
    elapsed = max(1e-6, time.perf_counter() - start_t)
    rate = done / elapsed
    eta = (total - done) / rate if rate > 0 else 0.0
    print(
        f"\r[{bar}] {frac * 100:6.2f}% ({done}/{total}) | {rate:5.1f} img/s | ETA {format_eta(eta)}",
        end="",
        flush=True,
    )


def default_output_dir(assets_dir: Path, model_path: str | None) -> Path:
    if model_path:
        tag = re.sub(r"[^A-Za-z0-9._-]+", "_", Path(model_path).stem).strip("_") or "model"
    else:
        tag = "bicubic"
    return assets_dir / f"gfx_hd_{tag}"


def main() -> None:
    ap = argparse.ArgumentParser(
        description="PyTorch HD upscaler for Pinball 2000 gfx/ -> gfx_hd/",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=(
            "Examples:\n"
            "  python scripts/upscale_pytorch.py --scale 4 --model work/models/4xNomos2_otf_esrgan.safetensors --anim kahuna --fp16\n"
            "  python scripts/upscale_pytorch.py --scale 4 --model work/models/4xNomos2_otf_esrgan.safetensors --output-dir work/swe1_assets/gfx_hd --fp16 --resume\n"
            "  python scripts/upscale_pytorch.py --scale 2\n"
        ),
    )
    ap.add_argument(
        "--assets-dir",
        type=Path,
        default=Path("work/swe1_assets"),
        help="Root extracted assets directory containing gfx/gfx_manifest.json (default: work/swe1_assets)",
    )
    ap.add_argument(
        "--output-dir",
        type=Path,
        default=None,
        help="Destination directory (default: <assets-dir>/gfx_hd_<model_name>, or gfx_hd_bicubic without --model)",
    )
    ap.add_argument("--scale", type=int, default=2, choices=[2, 3, 4, 8], help="Target output scale factor (default: 2)")
    ap.add_argument("--model", type=str, default=None, help="Optional .pth/.safetensors super-resolution checkpoint")
    ap.add_argument("--anim", type=str, default=None, help="Case-insensitive substring filter on animation symbol_name")
    ap.add_argument("--fp16", action="store_true", help="Use float16 weights/autocast on CUDA for lower VRAM & faster inference")
    ap.add_argument("--tile", type=int, default=256, help="Tile size in pixels with 16px overlap (default: 256, 0 to disable)")
    ap.add_argument("--blend", type=float, default=1.0, help="AI vs bicubic blend factor in [0.0, 1.0] (default: 1.0 = 100%% AI)")
    ap.add_argument("--soft-alpha", action="store_true", help="Keep smooth bicubic alpha instead of 1-bit 0.5 threshold (use with pin2k-desktop --aa)")
    ap.add_argument("--temporal-stabilize", action="store_true", help="Enable motion-aware temporal EMA stabilization across consecutive animation frames")
    ap.add_argument("--temporal-strength", type=float, default=0.65, help="Temporal blend strength for static regions (default: 0.65)")
    ap.add_argument("--temporal-threshold", type=float, default=0.03, help="1x source delta threshold for static detection (default: 0.03)")
    ap.add_argument("--resume", action="store_true", help="Skip frames that already exist in --output-dir")
    args = ap.parse_args()
    args.blend = max(0.0, min(1.0, args.blend))

    device = torch.device(
        "cuda" if torch.cuda.is_available() else ("mps" if torch.backends.mps.is_available() else "cpu")
    )
    model = load_pytorch_model(args.model, device, use_fp16=args.fp16)
    model_scale = getattr(model, "scale", args.scale) if model is not None else args.scale

    src_gfx = args.assets_dir / "gfx"
    dst_gfx = args.output_dir if args.output_dir is not None else default_output_dir(args.assets_dir, args.model)
    dst_gfx.mkdir(parents=True, exist_ok=True)
    print(
        f"Using PyTorch device={device}, scale={args.scale}x, fp16={args.fp16 and device.type == 'cuda'}, "
        f"blend={args.blend:.2f}, soft_alpha={args.soft_alpha}, temporal={args.temporal_stabilize}, "
        f"model={args.model or 'bicubic'}, output={dst_gfx}"
    )

    manifest = json.loads((src_gfx / "gfx_manifest.json").read_text())

    flt = args.anim.lower() if args.anim else None
    selected_entries = [
        entry
        for entry in manifest["animations"]
        if not flt or flt in entry["symbol_name"].lower()
    ]
    total_target = sum(
        1
        for entry in selected_entries
        for rel in entry["exported_frames"]
        if (src_gfx / rel).exists()
    )

    out_anims = []
    done_frames = 0
    start_t = time.perf_counter()
    print_progress(0, total_target, start_t)

    for entry in selected_entries:
        exported = []
        prev_src_rgba = None
        prev_up_rgb = None
        prev_up_alpha = None
        for rel in entry["exported_frames"]:
            src_path = src_gfx / rel
            if not src_path.exists():
                continue
            dst_path = dst_gfx / rel
            if args.resume and dst_path.exists():
                prev_src_rgba = None
                prev_up_rgb = None
                prev_up_alpha = None
                exported.append(rel)
                done_frames += 1
                if done_frames % 10 == 0 or done_frames == total_target:
                    print_progress(done_frames, total_target, start_t)
                continue

            rgba = np.array(Image.open(src_path).convert("RGBA"))
            up_rgb, up_alpha = upscale_frame_tensors(
                rgba,
                args.scale,
                device,
                model=model,
                model_scale=model_scale,
                use_fp16=args.fp16,
                tile_size=args.tile,
                blend=args.blend,
            )
            if args.temporal_stabilize:
                up_rgb, up_alpha = apply_temporal_stabilization(
                    rgba,
                    prev_src_rgba,
                    up_rgb,
                    up_alpha,
                    prev_up_rgb,
                    prev_up_alpha,
                    strength=args.temporal_strength,
                    threshold=args.temporal_threshold,
                )
                prev_src_rgba = rgba
                prev_up_rgb = up_rgb
                prev_up_alpha = up_alpha

            up = tensors_to_rgba_numpy(up_rgb, up_alpha, soft_alpha=args.soft_alpha)
            dst_path.parent.mkdir(parents=True, exist_ok=True)
            Image.fromarray(up, mode="RGBA").save(dst_path)
            exported.append(rel)
            done_frames += 1
            if done_frames % 5 == 0 or done_frames == total_target:
                print_progress(done_frames, total_target, start_t)
        if exported:
            entry["width"] *= args.scale
            entry["height"] *= args.scale
            entry["exported_frames"] = exported
            out_anims.append(entry)

    if total_target > 0:
        print()

    manifest["animations_exported"] = len(out_anims)
    manifest["animations"] = out_anims
    (dst_gfx / "gfx_manifest.json").write_text(json.dumps(manifest, indent=2))
    print(f"Upscaled {len(out_anims)} animations ({done_frames} frames) -> {dst_gfx / 'gfx_manifest.json'}")


if __name__ == "__main__":
    main()
