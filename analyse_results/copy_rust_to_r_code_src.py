#!/usr/bin/env python3
"""Copy Rust files from selected_dataset/* folders into r_code/src/*.

- Source root: selected_dataset
- Destination root: r_code/src
- Preserves one-level folder structure (e.g., p00074 -> r_code/src/p00074)
- Copies all .rs files found directly in each selected_dataset/p* folder
"""

from __future__ import annotations
import os
import argparse
import shutil
from pathlib import Path
from dotenv import load_dotenv

load_dotenv()

PROJECT_ROOT = Path(__file__).resolve().parent.parent
dataset_name = os.environ.get("DATASET_NAME")
SRC_ROOTS = [
    Path(f"api_responses/open_ai/{dataset_name}/C/translations_with_code_only"),
    Path(f"api_responses/open_ai/{dataset_name}/C/translations_with_specs_only"),
    Path(f"api_responses/open_ai/{dataset_name}/C/translations_with_specs_and_code"),
]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Copy Rust files from selected_dataset to r_code/src.")
    parser.add_argument("--dst", default="r_code/src", help="Destination root (default: r_code/src)")
    parser.add_argument("--dry-run", action="store_true", help="Show planned copies without writing files")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    dst_root = PROJECT_ROOT / args.dst

    dst_root.mkdir(parents=True, exist_ok=True)

    copied = 0
    skipped = 0

    for src_root in SRC_ROOTS:
        src_root = PROJECT_ROOT / src_root
        src_label = src_root.name

        if not src_root.exists() or not src_root.is_dir():
            raise FileNotFoundError(f"Source root not found or not a directory: {src_root}")

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

                # if target.exists() and target.read_bytes() == rust_file.read_bytes():
                #     print(f"[SKIP] {target} (unchanged)")
                #     skipped += 1
                #     continue

                shutil.copy2(rust_file, target)
                print(f"[COPY] {rust_file} -> {target}")
                copied += 1

    print(f"Done. copied={copied}, skipped={skipped}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
