#!/usr/bin/env python3
from __future__ import annotations

import csv
import shutil
from pathlib import Path

from dotenv import load_dotenv

load_dotenv()

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt@2"
SOURCE_ROOT = PROJECT_ROOT / "repairs" / "generated_repairs" / DATASET_NAME
REPORT_CSV = SOURCE_ROOT / "repair_report.csv"
DESTINATION_ROOT = PROJECT_ROOT / "r_code" / "src"


def main() -> int:
    if not SOURCE_ROOT.is_dir():
        raise FileNotFoundError(f"Source root not found or not a directory: {SOURCE_ROOT}")
    if not REPORT_CSV.is_file():
        raise FileNotFoundError(f"Repair report not found: {REPORT_CSV}")

    DESTINATION_ROOT.mkdir(parents=True, exist_ok=True)

    copied = 0

    with REPORT_CSV.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            output_path_text = row.get("output_path", "").strip()
            if not output_path_text:
                continue
            rust_file = Path(output_path_text)
            if not rust_file.is_file():
                print(f"[SKIP] missing output file: {rust_file}")
                continue

            problem_id = row.get("problem_id", "").strip()
            if not problem_id:
                print(f"[SKIP] missing problem_id for output file: {rust_file}")
                continue

            target_dir = DESTINATION_ROOT / problem_id
            target_dir.mkdir(parents=True, exist_ok=True)
            target = target_dir / rust_file.name
            shutil.copy2(rust_file, target)
            print(f"[COPY] {rust_file} -> {target}")
            copied += 1

    print(f"Done. copied={copied}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
