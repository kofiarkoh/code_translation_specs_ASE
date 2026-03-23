#!/usr/bin/env python3
from __future__ import annotations

import csv
import re
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "translation_with_validated_specs_GPT_5_2"
DATASET_NAME = "optimized_prompt@1"
PROFILE_NAME = "specs_only"
ATTEMPT_NUMBER = 3
OUTPUT_CSV = (
    PROJECT_ROOT
    / "repairs"
    / f"{PIPELINE_NAME}_analysis"
    / DATASET_NAME
    / f"{PROFILE_NAME}_attempt_{ATTEMPT_NUMBER}_non_pass_cases.csv"
)


def get_profile_root() -> Path:
    return PROJECT_ROOT / "repairs" / PIPELINE_NAME / PROFILE_NAME / DATASET_NAME


def extract_summary_count(text: str, label: str) -> int:
    match = re.search(rf"^{re.escape(label)}:\s+(\d+)$", text, flags=re.MULTILINE)
    if match is None:
        return 0
    return int(match.group(1))


def first_matching_line(text: str, pattern: str) -> str:
    match = re.search(pattern, text, flags=re.MULTILINE)
    if match is None:
        return ""
    return match.group(0)


def parse_status_and_summary(report_path: Path) -> tuple[str, dict[str, int], str]:
    if not report_path.is_file():
        return "MISSING_REPORT", {"passed": 0, "failed": 0, "timed_out": 0, "runtime_errors": 0}, ""

    text = report_path.read_text(encoding="utf-8", errors="ignore")
    summary = {
        "passed": extract_summary_count(text, "Passed"),
        "failed": extract_summary_count(text, "Failed"),
        "timed_out": extract_summary_count(text, "Timed out"),
        "runtime_errors": extract_summary_count(text, "Runtime errors"),
    }

    if "Status: NO_SAMPLES" in text:
        return "NO_SAMPLES", summary, "No sample input/output pairs were found."
    if "Compilation: FAILED" in text:
        stderr_match = re.search(r"Compiler stderr:\n(.*)", text, flags=re.DOTALL)
        detail = stderr_match.group(1).strip() if stderr_match else "Compilation failed."
        return "COMPILE_FAILED", summary, detail
    if summary["failed"] > 0:
        return "FAIL", summary, first_matching_line(text, r"^Sample \d+: FAIL$")
    if summary["timed_out"] > 0:
        return "TIMEOUT", summary, first_matching_line(text, r"^Sample \d+: TIMEOUT$")
    if summary["runtime_errors"] > 0:
        return "RUNTIME_ERROR", summary, first_matching_line(text, r"^Sample \d+: RUNTIME ERROR$")
    return "PASS", summary, ""


def build_rows() -> list[dict[str, str | int]]:
    profile_root = get_profile_root()
    if not profile_root.is_dir():
        raise FileNotFoundError(f"Profile root not found: {profile_root}")

    rows: list[dict[str, str | int]] = []
    for problem_dir in sorted(path for path in profile_root.iterdir() if path.is_dir() and path.name.startswith("p")):
        attempt_dir = problem_dir / f"attempt_{ATTEMPT_NUMBER}"
        report_path = attempt_dir / "run_report.txt"
        if not attempt_dir.is_dir() or not report_path.is_file():
            continue
        status, summary, detail = parse_status_and_summary(report_path)
        if status == "PASS":
            continue

        rows.append(
            {
                "problem_id": problem_dir.name,
                "attempt": ATTEMPT_NUMBER,
                "status": status,
                "passed_samples": summary["passed"],
                "failed_samples": summary["failed"],
                "timed_out_samples": summary["timed_out"],
                "runtime_error_samples": summary["runtime_errors"],
                "report_path": str(report_path),
                "detail": detail,
            }
        )

    return rows


def write_rows(rows: list[dict[str, str | int]]) -> None:
    OUTPUT_CSV.parent.mkdir(parents=True, exist_ok=True)
    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "attempt",
                "status",
                "passed_samples",
                "failed_samples",
                "timed_out_samples",
                "runtime_error_samples",
                "report_path",
                "detail",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    rows = build_rows()
    write_rows(rows)
    unique_problem_ids = sorted({str(row["problem_id"]) for row in rows})

    print(f"Profile root: {get_profile_root()}")
    print(f"Profile: {PROFILE_NAME}")
    print(f"Attempt: {ATTEMPT_NUMBER}")
    print("Target status: all non-PASS statuses")
    print(f"Matching cases: {len(rows)}")
    print(f"Unique problems: {len(unique_problem_ids)}")
    if unique_problem_ids:
        print("Problem IDs:")
        for problem_id in unique_problem_ids:
            print(problem_id)
    print(f"CSV written to: {OUTPUT_CSV}")


if __name__ == "__main__":
    main()
