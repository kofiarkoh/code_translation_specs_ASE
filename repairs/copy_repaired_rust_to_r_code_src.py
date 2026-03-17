#!/usr/bin/env python3
from __future__ import annotations

import os
import shutil
from pathlib import Path

from dotenv import load_dotenv

load_dotenv()

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt@1"
SOURCE_ROOT = PROJECT_ROOT / "repairs" / "generated_repairs" / DATASET_NAME
DESTINATION_ROOT = PROJECT_ROOT / "r_code" / "src"


def main() -> int:
    if not SOURCE_ROOT.is_dir():
        raise FileNotFoundError(f"Source root not found or not a directory: {SOURCE_ROOT}")

    DESTINATION_ROOT.mkdir(parents=True, exist_ok=True)

    copied = 0

    for problem_dir in sorted(p for p in SOURCE_ROOT.iterdir() if p.is_dir() and p.name.startswith("p")):
        for variant_dir in sorted(p for p in problem_dir.iterdir() if p.is_dir()):
            rust_files = sorted(variant_dir.glob("*.rs"))
            if not rust_files:
                continue

            target_dir = DESTINATION_ROOT / problem_dir.name
            target_dir.mkdir(parents=True, exist_ok=True)

            for rust_file in rust_files:
                target = target_dir / rust_file.name
                shutil.copy2(rust_file, target)
                print(f"[COPY] {rust_file} -> {target}")
                copied += 1

    print(f"Done. copied={copied}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
