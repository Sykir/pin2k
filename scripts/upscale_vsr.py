#!/usr/bin/env python3
"""Temporal Multi-Frame Video Super-Resolution (VSR) upscaler for Pinball 2000 (`gfx/` -> `gfx_hd/`).

Designed to eliminate temporal flickering across multi-frame animations and pre-rendered
codec-30 movies using two complementary approaches:

1. **Multi-frame VSR model (`--vsr-model`)**:
   Loads a TorchScript (`.pt`/`.pth`) multi-frame Video Super-Resolution network (e.g.
   RealBasicVSR, BasicVSR++, RealVSR) accepting a sliding window of `2 * radius + 1`
   consecutive RGB frames (`btchw`, `bcthw`, or channel-concatenated `concat`).

2. **Optical-Flow Temporal Fusion (default when `--vsr-model` is omitted)**:
   Upscales each frame with an optional single-image `spandrel` checkpoint (`--model`) or
   PyTorch bicubic antialias, computes dense Farneback optical flow (`cv2`) between
   consecutive 1x frames, warps neighboring upscaled frames onto the center frame, and
   performs occlusion-aware photometric fusion across `[t - radius .. t + radius]`.

Always applies RGB edge-padding into `alpha == 0` pixels and supports `--soft-alpha`,
`--blend`, `--fp16`, `--tile`, `--output-dir`, `--resume`, and a live percentage progress bar.
"""

import argparse
import json
import re
import time
from pathlib import Path

import cv2
import numpy as np
from PIL import Image
import torch
import torch.nn.functional as F

from upscale_pytorch import (
    edge_pad_rgb,
    load_pytorch_model,
    print_progress,
    tensors_to_rgba_numpy,
    upscale_frame_tensors,
)


def load_vsr_model(vsr_path: str, device: torch.device, use_fp16: bool):
    """Loads a TorchScript multi-frame VSR checkpoint."""
    model = torch.jit.load(vsr_path, map_location=device)
    model.eval()
    if use_fp16 and device.type == "cuda":
        model.half()
    return model


@torch.no_grad()
def run_vsr_window(
    vsr_model,
    window_rgb: list[torch.Tensor],
    layout: str,
    use_fp16: bool,
    device: torch.device,
) -> torch.Tensor:
    """Runs a TorchScript multi-frame VSR model over `window_rgb` ([1, 3, H, W] each) and returns center frame."""
    t_len = len(window_rgb)
    center_idx = t_len // 2
    seq = torch.cat(window_rgb, dim=0)  # [T, 3, H, W]
    _, _, h, w = seq.shape
    pad_h = (8 - h % 8) % 8
    pad_w = (8 - w % 8) % 8
    if pad_h or pad_w:
        seq = F.pad(seq, (0, pad_w, 0, pad_h), mode="replicate")

    if layout == "btchw":
        inp = seq.unsqueeze(0)  # [1, T, 3, H, W]
    elif layout == "bcthw":
        inp = seq.permute(1, 0, 2, 3).unsqueeze(0)  # [1, 3, T, H, W]
    else:  # concat
        inp = seq.reshape(1, t_len * 3, seq.shape[-2], seq.shape[-1])

    is_half = any(p.dtype == torch.float16 for p in vsr_model.parameters())
    if is_half:
        inp = inp.half()

    if use_fp16 and not is_half and device.type == "cuda":
        with torch.amp.autocast("cuda", dtype=torch.float16):
            out = vsr_model(inp)
    else:
        out = vsr_model(inp)

    if out.ndim == 5:
        # [B, T, C, H, W] or [B, C, T, H, W]
        if out.shape[1] == t_len:
            center = out[:, center_idx]
        else:
            center = out[:, :, center_idx]
    else:
        center = out
    return center.float()


def warp_upscaled_by_flow(
    src_prev_gray: np.ndarray,
    src_curr_gray: np.ndarray,
    up_prev_rgb: np.ndarray,
    scale_x: float,
    scale_y: float,
) -> np.ndarray:
    """Computes dense Farneback optical flow on 1x luma and warps `up_prev_rgb` onto current frame."""
    flow = cv2.calcOpticalFlowFarneback(
        src_curr_gray,
        src_prev_gray,
        None,
        pyr_scale=0.5,
        levels=3,
        winsize=11,
        iterations=3,
        poly_n=5,
        poly_sigma=1.1,
        flags=0,
    )
    dst_h, dst_w = up_prev_rgb.shape[:2]
    flow_up = cv2.resize(flow, (dst_w, dst_h), interpolation=cv2.INTER_LINEAR)
    flow_up[:, :, 0] *= scale_x
    flow_up[:, :, 1] *= scale_y

    grid_x, grid_y = np.meshgrid(
        np.arange(dst_w, dtype=np.float32),
        np.arange(dst_h, dtype=np.float32),
    )
    map_x = grid_x + flow_up[:, :, 0]
    map_y = grid_y + flow_up[:, :, 1]
    warped = cv2.remap(
        up_prev_rgb,
        map_x,
        map_y,
        interpolation=cv2.INTER_LINEAR,
        borderMode=cv2.BORDER_REPLICATE,
    )
    return warped


def fuse_temporal_neighbors(
    center_rgb: np.ndarray,
    neighbor_pairs: list[tuple[np.ndarray, np.ndarray, np.ndarray]],
    temporal_weight: float,
    sigma: float,
) -> np.ndarray:
    """Occlusion-aware temporal fusion of warped upscaled neighbor frames onto `center_rgb` (float32 0..1)."""
    if not neighbor_pairs or temporal_weight <= 0.0:
        return center_rgb

    acc = center_rgb.copy()
    w_sum = np.ones(center_rgb.shape[:2], dtype=np.float32)
    inv_2sig2 = 1.0 / (2.0 * max(sigma, 1e-4) ** 2)

    for src_nbr_gray, src_curr_gray, up_nbr_rgb in neighbor_pairs:
        if src_nbr_gray.shape != src_curr_gray.shape or up_nbr_rgb.shape != center_rgb.shape:
            continue
        sx = center_rgb.shape[1] / float(src_curr_gray.shape[1])
        sy = center_rgb.shape[0] / float(src_curr_gray.shape[0])
        warped = warp_upscaled_by_flow(src_nbr_gray, src_curr_gray, up_nbr_rgb, sx, sy)
        diff2 = np.mean((warped - center_rgb) ** 2, axis=2)
        w = temporal_weight * np.exp(-diff2 * inv_2sig2)
        acc += warped * w[:, :, None]
        w_sum += w

    return np.clip(acc / w_sum[:, :, None], 0.0, 1.0)


def default_vsr_output_dir(assets_dir: Path, vsr_model: str | None, spandrel_model: str | None) -> Path:
    ref = vsr_model or spandrel_model
    if ref:
        tag = re.sub(r"[^A-Za-z0-9._-]+", "_", Path(ref).stem).strip("_") or "model"
    else:
        tag = "bicubic"
    return assets_dir / f"gfx_hd_vsr_{tag}"


def main() -> None:
    ap = argparse.ArgumentParser(
        description="Temporal Multi-Frame VSR upscaler for Pinball 2000 gfx/ -> gfx_hd/",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog=(
            "Examples:\n"
            "  python scripts/upscale_vsr.py --scale 4 --model work/models/4xNomos2_realplksr_dysample.safetensors --anim podrace --fp16\n"
            "  python scripts/upscale_vsr.py --scale 4 --vsr-model work/models/RealBasicVSR.pt --layout btchw --fp16\n"
        ),
    )
    ap.add_argument("--assets-dir", type=Path, default=Path("work/swe1_assets"), help="Root assets folder (default: work/swe1_assets)")
    ap.add_argument("--output-dir", type=Path, default=None, help="Destination directory (default: <assets-dir>/gfx_hd_vsr_<model_name>)")
    ap.add_argument("--scale", type=int, default=4, choices=[2, 3, 4, 8], help="Target output scale factor (default: 4)")
    ap.add_argument("--model", type=str, default=None, help="Optional single-frame spandrel checkpoint combined with optical-flow fusion for multi-frame animations")
    ap.add_argument("--single-model", type=str, default=None, help="Optional dedicated spandrel checkpoint for 1-frame animations (defaults to --model when omitted)")
    ap.add_argument("--vsr-model", type=str, default=None, help="Optional TorchScript multi-frame VSR model (.pt/.pth)")
    ap.add_argument("--layout", choices=["btchw", "bcthw", "concat"], default="btchw", help="Input tensor layout for --vsr-model (default: btchw)")
    ap.add_argument("--window-radius", type=int, default=1, help="Temporal window radius in frames (1 = 3-frame window [t-1, t, t+1])")
    ap.add_argument("--temporal-weight", type=float, default=0.5, help="Neighbor fusion weight for optical-flow VSR (default: 0.5)")
    ap.add_argument("--sigma", type=float, default=0.06, help="Photometric consistency sigma for occlusion rejection (default: 0.06)")
    ap.add_argument("--blend", type=float, default=1.0, help="AI vs bicubic blend factor in [0.0, 1.0] (default: 1.0)")
    ap.add_argument("--soft-alpha", action="store_true", help="Keep smooth bicubic alpha instead of 1-bit 0.5 threshold")
    ap.add_argument("--anim", type=str, default=None, help="Case-insensitive substring filter on animation symbol_name")
    ap.add_argument("--fp16", action="store_true", help="Use float16 weights/autocast on CUDA")
    ap.add_argument("--tile", type=int, default=256, help="Tile size in pixels for single-frame model (default: 256, 0 to disable)")
    ap.add_argument("--resume", action="store_true", help="Skip frames that already exist in --output-dir")
    args = ap.parse_args()
    args.blend = max(0.0, min(1.0, args.blend))

    device = torch.device(
        "cuda" if torch.cuda.is_available() else ("mps" if torch.backends.mps.is_available() else "cpu")
    )
    vsr_model = load_vsr_model(args.vsr_model, device, args.fp16) if args.vsr_model else None
    spandrel_model = None if vsr_model is not None else load_pytorch_model(args.model, device, use_fp16=args.fp16)
    spandrel_scale = getattr(spandrel_model, "scale", args.scale) if spandrel_model is not None else args.scale
    if args.single_model:
        single_model = load_pytorch_model(args.single_model, device, use_fp16=args.fp16)
    elif spandrel_model is not None:
        single_model = spandrel_model
    else:
        single_model = load_pytorch_model(args.model, device, use_fp16=args.fp16) if args.model else None
    single_scale = getattr(single_model, "scale", args.scale) if single_model is not None else args.scale

    src_gfx = args.assets_dir / "gfx"
    dst_gfx = args.output_dir if args.output_dir is not None else default_vsr_output_dir(args.assets_dir, args.vsr_model, args.model)
    dst_gfx.mkdir(parents=True, exist_ok=True)
    mode_desc = f"TorchScript VSR ({args.vsr_model})" if vsr_model is not None else f"Optical-Flow VSR ({args.model or 'bicubic'})"
    single_desc = args.single_model or args.model or "bicubic"
    print(
        f"Using device={device}, multi_frame={mode_desc}, single_frame={single_desc}, "
        f"scale={args.scale}x, radius={args.window_radius}, fp16={args.fp16 and device.type == 'cuda'}, output={dst_gfx}"
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
        rel_frames = [rel for rel in entry["exported_frames"] if (src_gfx / rel).exists()]
        if not rel_frames:
            continue

        # Single-frame animations bypass VSR/optical-flow and use the dedicated single-image model directly
        if len(rel_frames) == 1:
            rel = rel_frames[0]
            dst_path = dst_gfx / rel
            if not (args.resume and dst_path.exists()):
                rgba = np.array(Image.open(src_gfx / rel).convert("RGBA"))
                u_rgb, u_alpha = upscale_frame_tensors(
                    rgba,
                    args.scale,
                    device,
                    model=single_model,
                    model_scale=single_scale,
                    use_fp16=args.fp16,
                    tile_size=args.tile,
                    blend=args.blend,
                )
                out_rgba = tensors_to_rgba_numpy(u_rgb, u_alpha, soft_alpha=args.soft_alpha)
                dst_path.parent.mkdir(parents=True, exist_ok=True)
                Image.fromarray(out_rgba, mode="RGBA").save(dst_path)
            done_frames += 1
            if done_frames % 5 == 0 or done_frames == total_target:
                print_progress(done_frames, total_target, start_t)
            entry["width"] *= args.scale
            entry["height"] *= args.scale
            entry["exported_frames"] = rel_frames
            out_anims.append(entry)
            continue

        raw_frames = [np.array(Image.open(src_gfx / rel).convert("RGBA")) for rel in rel_frames]
        gray_frames = [cv2.cvtColor(f[:, :, :3], cv2.COLOR_RGB2GRAY) for f in raw_frames]

        if vsr_model is not None:
            # Multi-frame TorchScript VSR mode
            padded_rgb = [
                torch.from_numpy(edge_pad_rgb(f, passes=2)[:, :, :3])
                .to(device=device, dtype=torch.float32)
                .permute(2, 0, 1)
                .unsqueeze(0)
                / 255.0
                for f in raw_frames
            ]
            exported = []
            for idx, rel in enumerate(rel_frames):
                dst_path = dst_gfx / rel
                if args.resume and dst_path.exists():
                    exported.append(rel)
                    done_frames += 1
                    continue
                h, w = raw_frames[idx].shape[:2]
                dst_h, dst_w = h * args.scale, w * args.scale
                win = [
                    padded_rgb[min(max(idx + d, 0), len(raw_frames) - 1)]
                    for d in range(-args.window_radius, args.window_radius + 1)
                ]
                up_rgb = run_vsr_window(vsr_model, win, args.layout, args.fp16, device)
                up_rgb = up_rgb[:, :, :dst_h, :dst_w] if up_rgb.shape[-2] >= dst_h and up_rgb.shape[-1] >= dst_w else F.interpolate(
                    up_rgb, size=(dst_h, dst_w), mode="bicubic", align_corners=False, antialias=True
                )
                alpha_t = (
                    torch.from_numpy(raw_frames[idx][:, :, 3])
                    .to(device=device, dtype=torch.float32)
                    .unsqueeze(0)
                    .unsqueeze(0)
                    / 255.0
                )
                up_alpha = F.interpolate(alpha_t, size=(dst_h, dst_w), mode="bicubic", align_corners=False)
                out_rgba = tensors_to_rgba_numpy(up_rgb.clamp(0.0, 1.0), up_alpha.clamp(0.0, 1.0), soft_alpha=args.soft_alpha)
                dst_path.parent.mkdir(parents=True, exist_ok=True)
                Image.fromarray(out_rgba, mode="RGBA").save(dst_path)
                exported.append(rel)
                done_frames += 1
                if done_frames % 5 == 0 or done_frames == total_target:
                    print_progress(done_frames, total_target, start_t)
        else:
            # Optical-flow temporal VSR fusion mode
            up_rgb_list = []
            up_alpha_list = []
            for f in raw_frames:
                u_rgb, u_alpha = upscale_frame_tensors(
                    f,
                    args.scale,
                    device,
                    model=spandrel_model,
                    model_scale=spandrel_scale,
                    use_fp16=args.fp16,
                    tile_size=args.tile,
                    blend=args.blend,
                )
                up_rgb_list.append(u_rgb.squeeze(0).permute(1, 2, 0).cpu().numpy())
                up_alpha_list.append(u_alpha)

            exported = []
            for idx, rel in enumerate(rel_frames):
                dst_path = dst_gfx / rel
                if args.resume and dst_path.exists():
                    exported.append(rel)
                    done_frames += 1
                    continue
                nbrs = []
                for d in range(-args.window_radius, args.window_radius + 1):
                    if d == 0:
                        continue
                    j = idx + d
                    if 0 <= j < len(raw_frames):
                        nbrs.append((gray_frames[j], gray_frames[idx], up_rgb_list[j]))

                fused_rgb = fuse_temporal_neighbors(
                    up_rgb_list[idx],
                    nbrs,
                    temporal_weight=args.temporal_weight,
                    sigma=args.sigma,
                )
                fused_rgb_t = torch.from_numpy(fused_rgb).to(device=device).permute(2, 0, 1).unsqueeze(0)
                out_rgba = tensors_to_rgba_numpy(fused_rgb_t, up_alpha_list[idx], soft_alpha=args.soft_alpha)
                dst_path.parent.mkdir(parents=True, exist_ok=True)
                Image.fromarray(out_rgba, mode="RGBA").save(dst_path)
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
    print(f"VSR upscaled {len(out_anims)} animations ({done_frames} frames) -> {dst_gfx / 'gfx_manifest.json'}")


if __name__ == "__main__":
    main()
