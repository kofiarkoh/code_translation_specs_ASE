#!/usr/bin/env python3
"""Copy Rust files from selected_dataset/* folders into r_code/src/*.

- Source root: selected_dataset
- Destination root: r_code/src
- Preserves one-level folder structure (e.g., p00074 -> r_code/src/p00074)
- Copies all .rs files found directly in each selected_dataset/p* folder
"""

from __future__ import annotations

import argparse
import shutil
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Copy Rust files from selected_dataset to r_code/src.")
    parser.add_argument("--src", default="/Users/lithe/Documents/research/code_translation_specs/codenet_statistics/api_responses/open_ai/C/translations_with_code_only", help="Source root (default: selected_dataset)")
    parser.add_argument("--dst", default="r_code/src", help="Destination root (default: r_code/src)")
    parser.add_argument("--dry-run", action="store_true", help="Show planned copies without writing files")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    src_root = Path(args.src)
    dst_root = Path(args.dst)
    src_label = src_root.name

    if not src_root.exists() or not src_root.is_dir():
        raise FileNotFoundError(f"Source root not found or not a directory: {src_root}")

    dst_root.mkdir(parents=True, exist_ok=True)

    copied = 0
    skipped = 0

    for folder in sorted(p for p in src_root.iterdir() if p.is_dir() and p.name.startswith("p")):
        rust_files = sorted(folder.glob("*.rs"))
        if not rust_files:
            continue

        target_dir = dst_root / folder.name
        if not args.dry_run:
            target_dir.mkdir(parents=True, exist_ok=True)

        for rust_file in rust_files:
            target = target_dir / f"{src_label}.rs"
            if args.dry_run:
                print(f"[DRY-RUN] {rust_file} -> {target}")
                copied += 1
                continue

            if target.exists() and target.read_bytes() == rust_file.read_bytes():
                print(f"[SKIP] {target} (unchanged)")
                skipped += 1
                continue

            shutil.copy2(rust_file, target)
            print(f"[COPY] {rust_file} -> {target}")
            copied += 1

    print(f"Done. copied={copied}, skipped={skipped}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
