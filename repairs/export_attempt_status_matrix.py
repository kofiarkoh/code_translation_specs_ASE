#!/usr/bin/env python3
from __future__ import annotations

import csv
import re
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "full_translation_loop"
DATASET_NAME = "optimized_prompt@1"
PROFILE_NAME = "specs_only"
MAX_ATTEMPT: int | None = None
OUTPUT_CSV = PROJECT_ROOT / "repairs" / f"{PIPELINE_NAME}_analysis" / DATASET_NAME / f"{PROFILE_NAME}_attempt_status_matrix.csv"

STATUS_COLUMNS = [
    ("PASS", "Pass"),
    ("FAIL", "Fail"),
    ("COMPILE_FAILED", "Compile Error"),
    ("RUNTIME_ERROR", "Runtime Error"),
    ("TIMEOUT", "Timeout"),
    ("NO_SAMPLES", "No Samples"),
    ("MISSING_REPORT", "Missing Report"),
]


def get_profile_root() -> Path:
    return PROJECT_ROOT / "repairs" / PIPELINE_NAME / PROFILE_NAME / DATASET_NAME


def parse_status_from_report(report_path: Path) -> str:
    if not report_path.is_file():
        return "MISSING_REPORT"

    text = report_path.read_text(encoding="utf-8", errors="ignore")
    if "Status: NO_SAMPLES" in text:
        return "NO_SAMPLES"
    if "Compilation: FAILED" in text:
        return "COMPILE_FAILED"
    if extract_summary_count(text, "Failed") > 0:
        return "FAIL"
    if extract_summary_count(text, "Timed out") > 0:
        return "TIMEOUT"
    if extract_summary_count(text, "Runtime errors") > 0:
        return "RUNTIME_ERROR"
    return "PASS"


def extract_summary_count(text: str, label: str) -> int:
    match = re.search(rf"^{re.escape(label)}:\s+(\d+)$", text, flags=re.MULTILINE)
    if match is None:
        return 0
    return int(match.group(1))


def collect_problem_attempt_statuses() -> tuple[list[str], dict[str, dict[int, str]], list[int]]:
    profile_root = get_profile_root()
    if not profile_root.is_dir():
        raise FileNotFoundError(f"Profile root not found: {profile_root}")

    problem_statuses: dict[str, dict[int, str]] = {}
    attempt_numbers: set[int] = set()

    for problem_dir in sorted(path for path in profile_root.iterdir() if path.is_dir() and path.name.startswith("p")):
        per_attempt: dict[int, str] = {}
        for attempt_dir in sorted(path for path in problem_dir.iterdir() if path.is_dir() and path.name.startswith("attempt_")):
            try:
                attempt_number = int(attempt_dir.name.removeprefix("attempt_"))
            except ValueError:
                continue
            if MAX_ATTEMPT is not None and attempt_number > MAX_ATTEMPT:
                continue
            per_attempt[attempt_number] = parse_status_from_report(attempt_dir / "run_report.txt")
            attempt_numbers.add(attempt_number)

        if per_attempt:
            problem_statuses[problem_dir.name] = per_attempt

    ordered_attempts = sorted(attempt_numbers)
    return sorted(problem_statuses), problem_statuses, ordered_attempts


def build_header_rows(attempt_numbers: list[int]) -> list[list[str]]:
    top_row = ["Problem"]
    middle_row = [""]
    bottom_row = [""]

    for attempt_number in attempt_numbers:
        top_row.extend([PROFILE_NAME] + [""] * (len(STATUS_COLUMNS) - 1))
        middle_row.extend([f"k={attempt_number}"] + [""] * (len(STATUS_COLUMNS) - 1))
        bottom_row.extend([label for _, label in STATUS_COLUMNS])

    return [top_row, middle_row, bottom_row]


def build_problem_rows(problem_ids: list[str], problem_statuses: dict[str, dict[int, str]], attempt_numbers: list[int]) -> list[list[str]]:
    rows: list[list[str]] = []

    for problem_id in problem_ids:
        row = [problem_id]
        statuses_by_attempt = problem_statuses.get(problem_id, {})
        for attempt_number in attempt_numbers:
            current_status = statuses_by_attempt.get(attempt_number, "")
            for status_name, _ in STATUS_COLUMNS:
                row.append("1" if current_status == status_name else "0")
        rows.append(row)

    return rows


def main() -> None:
    problem_ids, problem_statuses, attempt_numbers = collect_problem_attempt_statuses()
    if not problem_ids:
        raise ValueError(f"No attempt data found under {get_profile_root()}")

    OUTPUT_CSV.parent.mkdir(parents=True, exist_ok=True)
    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerows(build_header_rows(attempt_numbers))
        writer.writerows(build_problem_rows(problem_ids, problem_statuses, attempt_numbers))

    print(f"Profile root: {get_profile_root()}")
    print(f"Profile: {PROFILE_NAME}")
    print(f"Attempts included: {attempt_numbers}")
    print(f"CSV written to: {OUTPUT_CSV}")


if __name__ == "__main__":
    main()
