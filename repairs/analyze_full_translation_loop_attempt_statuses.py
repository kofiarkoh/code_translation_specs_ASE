#!/usr/bin/env python3
from __future__ import annotations

import csv
import re
from collections import Counter, defaultdict
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "full_translation_loop_GPT_5_2"
DATASET_NAME = "optimized_prompt@1"
PIPELINE_ROOT = PROJECT_ROOT / "repairs" / PIPELINE_NAME
OUTPUT_ROOT = PROJECT_ROOT / "repairs" / f"{PIPELINE_NAME}_analysis" / DATASET_NAME
SUMMARY_TXT = OUTPUT_ROOT / "attempt_status_summary.txt"
SUMMARY_CSV = OUTPUT_ROOT / "attempt_status_summary.csv"

STATUS_ORDER = [
    "PASS",
    "FAIL",
    "RUNTIME_ERROR",
    "TIMEOUT",
    "COMPILE_FAILED",
    "NO_SAMPLES",
    "MISSING_REPORT",
]


def parse_status_from_report(report_path: Path) -> str:
    if not report_path.is_file():
        return "MISSING_REPORT"

    text = report_path.read_text(encoding="utf-8", errors="ignore")
    if "Status: NO_SAMPLES" in text:
        return "NO_SAMPLES"
    if "Compilation: FAILED" in text:
        return "COMPILE_FAILED"

    failed = extract_summary_count(text, "Failed")
    timed_out = extract_summary_count(text, "Timed out")
    runtime_errors = extract_summary_count(text, "Runtime errors")

    if failed > 0:
        return "FAIL"
    if timed_out > 0:
        return "TIMEOUT"
    if runtime_errors > 0:
        return "RUNTIME_ERROR"
    return "PASS"


def extract_summary_count(text: str, label: str) -> int:
    match = re.search(rf"^{re.escape(label)}:\s+(\d+)$", text, flags=re.MULTILINE)
    if match is None:
        return 0
    return int(match.group(1))


def collect_profile_attempts(profile_root: Path) -> dict[int, Counter[str]]:
    attempt_counters: dict[int, Counter[str]] = defaultdict(Counter)
    dataset_root = profile_root / DATASET_NAME
    if not dataset_root.is_dir():
        return {}

    for problem_dir in sorted(p for p in dataset_root.iterdir() if p.is_dir() and p.name.startswith("p")):
        for attempt_dir in sorted(p for p in problem_dir.iterdir() if p.is_dir() and p.name.startswith("attempt_")):
            try:
                attempt_number = int(attempt_dir.name.removeprefix("attempt_"))
            except ValueError:
                continue
            status = parse_status_from_report(attempt_dir / "run_report.txt")
            attempt_counters[attempt_number][status] += 1

    return dict(sorted(attempt_counters.items()))

def write_csv(profile_attempts: dict[str, dict[int, Counter[str]]]) -> None:
    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    with SUMMARY_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(
            [
                "profile",
                "attempt",
                "total",
                "pass_count",
                "fail_count",
                "runtime_error_count",
                "timeout_count",
                "compile_failed_count",
                "no_samples_count",
                "missing_report_count",
            ]
        )

        for profile_name, attempts in sorted(profile_attempts.items()):
            for attempt_number, counter in sorted(attempts.items()):
                total = sum(counter.values())
                writer.writerow(
                    [
                        profile_name,
                        attempt_number,
                        total,
                        counter["PASS"],
                        counter["FAIL"],
                        counter["RUNTIME_ERROR"],
                        counter["TIMEOUT"],
                        counter["COMPILE_FAILED"],
                        counter["NO_SAMPLES"],
                        counter["MISSING_REPORT"],
                    ]
                )


def write_summary(profile_attempts: dict[str, dict[int, Counter[str]]]) -> None:
    lines: list[str] = [
        f"PIPELINE_ROOT: {PIPELINE_ROOT}",
        f"DATASET_NAME: {DATASET_NAME}",
        "",
    ]

    for profile_name, attempts in sorted(profile_attempts.items()):
        lines.append(f"Profile: {profile_name}")
        if not attempts:
            lines.append("  No attempt data found.")
            lines.append("")
            continue

        for attempt_number, counter in sorted(attempts.items()):
            total = sum(counter.values())
            lines.append(f"  Attempt {attempt_number} (total={total})")
            for status in STATUS_ORDER:
                count = counter[status]
                lines.append(f"    {status}: {count}")
        lines.append("")

    lines.append(f"SUMMARY_CSV: {SUMMARY_CSV}")
    SUMMARY_TXT.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    if not PIPELINE_ROOT.is_dir():
        raise FileNotFoundError(f"Pipeline root not found: {PIPELINE_ROOT}")

    profile_attempts: dict[str, dict[int, Counter[str]]] = {}
    for profile_root in sorted(p for p in PIPELINE_ROOT.iterdir() if p.is_dir()):
        profile_attempts[profile_root.name] = collect_profile_attempts(profile_root)

    if not profile_attempts:
        raise ValueError(f"No profile directories found in {PIPELINE_ROOT}")

    write_csv(profile_attempts)
    write_summary(profile_attempts)

    print(f"Summary written to: {SUMMARY_TXT}")
    print(f"CSV written to: {SUMMARY_CSV}")


if __name__ == "__main__":
    main()
