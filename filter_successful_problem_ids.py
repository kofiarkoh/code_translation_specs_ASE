#!/usr/bin/env python3
"""
Filter a run summary CSV down to problem IDs whose rows satisfy:

- failed == 0
- passed == total_samples

Outputs:
- a filtered CSV with matching rows
- a text file with one unique problem_id per line
"""

from __future__ import annotations

import csv
from pathlib import Path


# =========================
# Configure these values
# =========================
INPUT_CSV = Path('/Volumes/Extreme SSD/research/code translation/collected_dataset/Java/java_run_reports/run_summary.csv')
FILTERED_ROWS_CSV = Path("/Volumes/Extreme SSD/research/code translation/collected_dataset/Java/success_rows.csv")
FILTERED_PROBLEM_IDS_TXT = Path("/Volumes/Extreme SSD/research/code translation/collected_dataset/Java/filtered_problem_ids.txt")


def parse_int(row: dict[str, str], key: str) -> int:
    value = row.get(key, "").strip()
    if value == "":
        return 0
    return int(value)


def row_matches(row: dict[str, str]) -> bool:
    failed = parse_int(row, "failed")
    passed = parse_int(row, "passed")
    total_samples = parse_int(row, "total_samples")
    return failed == 0 and passed == total_samples and row.get("status") == "PASS"


def main() -> None:
    if not INPUT_CSV.is_file():
        raise FileNotFoundError(f"INPUT_CSV does not exist or is not a file: {INPUT_CSV}")

    FILTERED_ROWS_CSV.parent.mkdir(parents=True, exist_ok=True)
    FILTERED_PROBLEM_IDS_TXT.parent.mkdir(parents=True, exist_ok=True)

    with INPUT_CSV.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        fieldnames = reader.fieldnames
        if not fieldnames:
            raise ValueError(f"CSV has no header row: {INPUT_CSV}")

        matched_rows: list[dict[str, str]] = []
        matched_problem_ids: set[str] = set()

        for row in reader:
            if row_matches(row):
                matched_rows.append(row)
                problem_id = row.get("problem_id", "").strip()
                if problem_id:
                    matched_problem_ids.add(problem_id)

    with FILTERED_ROWS_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(csv_file, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(matched_rows)

    FILTERED_PROBLEM_IDS_TXT.write_text(
        "\n".join(sorted(matched_problem_ids)) + ("\n" if matched_problem_ids else ""),
        encoding="utf-8",
    )

    print("Done.")
    print(f"Input CSV: {INPUT_CSV}")
    print(f"Matched rows: {len(matched_rows)}")
    print(f"Matched unique problem IDs: {len(matched_problem_ids)}")
    print(f"Filtered rows CSV: {FILTERED_ROWS_CSV}")
    print(f"Problem IDs TXT: {FILTERED_PROBLEM_IDS_TXT}")


if __name__ == "__main__":
    main()
