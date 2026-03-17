#!/usr/bin/env python3
"""
Copy successful program files listed in a filtered success CSV into a new
directory grouped by problem ID.

Expected CSV input:
- one row per successful program
- must include at least `problem_id` and the configured source file column

Output shape:
OUTPUT_ROOT/
  p00001/
    s123456789.c
    s987654321.c
  p00002/
    ...
"""

from __future__ import annotations

import csv
import shutil
from pathlib import Path


# =========================
# Configure these values
# =========================
SUCCESS_ROWS_CSV = Path('/Volumes/Extreme SSD/research/code translation/collected_dataset/Java/success_rows.csv')
OUTPUT_ROOT = Path('/Volumes/Extreme SSD/research/code translation/collected_dataset/Java/final_problems')
SOURCE_FILE_COLUMN = "java_file" # or c_file, depending on your CSV


def main() -> None:
    if not SUCCESS_ROWS_CSV.is_file():
        raise FileNotFoundError(
            f"SUCCESS_ROWS_CSV does not exist or is not a file: {SUCCESS_ROWS_CSV}"
        )

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)

    copied_files = 0
    skipped_missing_files = 0
    touched_problem_ids: set[str] = set()

    with SUCCESS_ROWS_CSV.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        if not reader.fieldnames:
            raise ValueError(f"CSV has no header row: {SUCCESS_ROWS_CSV}")
        if "problem_id" not in reader.fieldnames or SOURCE_FILE_COLUMN not in reader.fieldnames:
            raise ValueError(
                f"CSV must contain 'problem_id' and '{SOURCE_FILE_COLUMN}' columns: {SUCCESS_ROWS_CSV}"
            )

        for row in reader:
            problem_id = row.get("problem_id", "").strip()
            source_file_value = row.get(SOURCE_FILE_COLUMN, "").strip()
            if not problem_id or not source_file_value:
                continue

            src_path = Path(source_file_value)
            if not src_path.is_file():
                skipped_missing_files += 1
                print(f"[SKIP] missing file: {src_path}")
                continue

            out_problem_dir = OUTPUT_ROOT / problem_id
            out_problem_dir.mkdir(parents=True, exist_ok=True)

            dest_path = out_problem_dir / src_path.name
            shutil.copy2(src_path, dest_path)
            copied_files += 1
            touched_problem_ids.add(problem_id)
            print(f"[COPIED] {src_path} -> {dest_path}")

    print("\nDone.")
    print(f"Input CSV: {SUCCESS_ROWS_CSV}")
    print(f"Source file column: {SOURCE_FILE_COLUMN}")
    print(f"Copied files: {copied_files}")
    print(f"Problem ID folders created: {len(touched_problem_ids)}")
    print(f"Skipped missing files: {skipped_missing_files}")
    print(f"Output root: {OUTPUT_ROOT}")


if __name__ == "__main__":
    main()
