#!/usr/bin/env python3
from __future__ import annotations

import csv
import re
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "full_translation_loop_GPT_5_2"
DATASET_NAME = "optimized_prompt@1"
PROFILE_NAMES = ["specs_only", "code_and_spec", "code_only"]
MAX_ATTEMPT: int | None = None
OUTPUT_CSV = PROJECT_ROOT / "repairs" / f"{PIPELINE_NAME}_analysis" / DATASET_NAME / "all_profiles_attempt_status_matrix.csv"

STATUS_COLUMNS = [
    ("PASS", "Pass"),
    ("FAIL", "Fail"),
    ("COMPILE_FAILED", "Compile Error"),
    ("RUNTIME_ERROR", "Runtime Error"),
    ("TIMEOUT", "Timeout"),
    ("NO_SAMPLES", "No Samples"),
    ("MISSING_REPORT", "Missing Report"),
]


def get_profile_root(profile_name: str) -> Path:
    return PROJECT_ROOT / "repairs" / PIPELINE_NAME / profile_name / DATASET_NAME


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


def collect_profile_attempt_statuses(profile_name: str) -> tuple[dict[str, dict[int, str]], set[int]]:
    profile_root = get_profile_root(profile_name)
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

    return problem_statuses, attempt_numbers


def build_header_rows(attempt_numbers: list[int]) -> list[list[str]]:
    top_row = ["Problem"]
    middle_row = [""]
    bottom_row = [""]

    for profile_name in PROFILE_NAMES:
        for attempt_number in attempt_numbers:
            top_row.extend([profile_name] + [""] * (len(STATUS_COLUMNS) - 1))
            middle_row.extend([f"k={attempt_number}"] + [""] * (len(STATUS_COLUMNS) - 1))
            bottom_row.extend([label for _, label in STATUS_COLUMNS])

    return [top_row, middle_row, bottom_row]


def build_problem_rows(
    problem_ids: list[str],
    all_profile_statuses: dict[str, dict[str, dict[int, str]]],
    attempt_numbers: list[int],
) -> list[list[str]]:
    rows: list[list[str]] = []

    for problem_id in problem_ids:
        row = [problem_id]
        for profile_name in PROFILE_NAMES:
            statuses_by_attempt = all_profile_statuses.get(profile_name, {}).get(problem_id, {})
            for attempt_number in attempt_numbers:
                current_status = statuses_by_attempt.get(attempt_number, "")
                for status_name, _ in STATUS_COLUMNS:
                    row.append("1" if current_status == status_name else "0")
        rows.append(row)

    return rows


def main() -> None:
    all_profile_statuses: dict[str, dict[str, dict[int, str]]] = {}
    all_problem_ids: set[str] = set()
    all_attempt_numbers: set[int] = set()

    for profile_name in PROFILE_NAMES:
        profile_statuses, attempt_numbers = collect_profile_attempt_statuses(profile_name)
        all_profile_statuses[profile_name] = profile_statuses
        all_problem_ids.update(profile_statuses)
        all_attempt_numbers.update(attempt_numbers)

    problem_ids = sorted(all_problem_ids)
    attempt_numbers = sorted(all_attempt_numbers)
    if not problem_ids:
        raise ValueError("No attempt data found for any configured profiles.")

    OUTPUT_CSV.parent.mkdir(parents=True, exist_ok=True)
    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerows(build_header_rows(attempt_numbers))
        writer.writerows(build_problem_rows(problem_ids, all_profile_statuses, attempt_numbers))

    print(f"Profiles: {PROFILE_NAMES}")
    print(f"Attempts included: {attempt_numbers}")
    print(f"CSV written to: {OUTPUT_CSV}")


if __name__ == "__main__":
    main()
