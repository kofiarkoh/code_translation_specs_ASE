#!/usr/bin/env python3
from __future__ import annotations

import csv
import re
from dataclasses import dataclass, field
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS"
DATASET_NAME = "optimized_prompt@1"
PROFILE_NAME = "specs_only"
USE_FINAL_REPORTS = True
ATTEMPT_NUMBER: int | None = 3
TARGET_STATUS = "FAIL"

PIPELINE_ROOT = PROJECT_ROOT / "repairs" / PIPELINE_NAME
PROFILE_ROOT = PIPELINE_ROOT / PROFILE_NAME / DATASET_NAME
LOCAL_SAMPLE_ROOT = PROJECT_ROOT / "collected_dataset" / "problem_test_inputs_and_outputs"
OUTPUT_CSV = (
    PROJECT_ROOT
    / "repairs"
    / f"{PIPELINE_NAME}_analysis"
    / DATASET_NAME
    / (
        f"{PROFILE_NAME}_final_failed_test_cases.csv"
        if USE_FINAL_REPORTS
        else f"{PROFILE_NAME}_attempt_{ATTEMPT_NUMBER}_failed_test_cases.csv"
    )
)


@dataclass
class SampleResult:
    sample_index: int
    status: str
    input_file: str | None = None
    expected_output_file: str | None = None
    input_text: str | None = None
    expected_text: str | None = None
    actual_text: str | None = None
    stderr_text: str | None = None
    exit_code: str | None = None


@dataclass
class ParsedReport:
    report_path: Path
    problem_id: str
    compilation_status: str | None = None
    compile_error_text: str | None = None
    samples: list[SampleResult] = field(default_factory=list)


def normalize_text_block(lines: list[str]) -> str:
    return "\n".join(lines).rstrip("\n")


def extract_section(lines: list[str], start_index: int, stop_prefixes: tuple[str, ...]) -> tuple[str, int]:
    collected: list[str] = []
    i = start_index
    while i < len(lines):
        line = lines[i]
        if any(line.startswith(prefix) for prefix in stop_prefixes):
            break
        collected.append(line)
        i += 1
    return normalize_text_block(collected), i


def parse_report(report_path: Path) -> ParsedReport:
    text = report_path.read_text(encoding="utf-8", errors="replace")
    lines = text.splitlines()
    parsed = ParsedReport(report_path=report_path, problem_id=report_path.parent.name)
    if report_path.name == "run_report.txt" and report_path.parent.name.startswith("attempt_"):
        parsed.problem_id = report_path.parent.parent.name

    for i, line in enumerate(lines):
        if line.startswith("Compilation: "):
            parsed.compilation_status = line.split(":", 1)[1].strip()
            if parsed.compilation_status == "FAILED":
                compile_text, _ = extract_section(lines, i + 1, ("Sample ", "Summary"))
                parsed.compile_error_text = compile_text.strip()
            break

    sample_header_re = re.compile(r"^Sample\s+(\d+):\s+(.+)$")
    i = 0
    while i < len(lines):
        match = sample_header_re.match(lines[i])
        if not match:
            i += 1
            continue

        sample = SampleResult(sample_index=int(match.group(1)), status=match.group(2).strip())
        i += 1
        while i < len(lines):
            line = lines[i]
            if sample_header_re.match(line) or line == "Summary":
                break
            if line.startswith("Input file: "):
                sample.input_file = line.split(":", 1)[1].strip()
                i += 1
                continue
            if line.startswith("Expected output file: "):
                sample.expected_output_file = line.split(":", 1)[1].strip()
                i += 1
                continue
            if line.startswith("Exit code: "):
                sample.exit_code = line.split(":", 1)[1].strip()
                i += 1
                continue
            if line == "Input:":
                sample.input_text, i = extract_section(
                    lines,
                    i + 1,
                    ("Expected:", "Actual:", "stderr:", "Sample ", "Summary"),
                )
                continue
            if line == "Expected:":
                sample.expected_text, i = extract_section(
                    lines,
                    i + 1,
                    ("Actual:", "stderr:", "Sample ", "Summary"),
                )
                continue
            if line == "Actual:":
                sample.actual_text, i = extract_section(
                    lines,
                    i + 1,
                    ("stderr:", "Sample ", "Summary"),
                )
                continue
            if line == "stderr:":
                sample.stderr_text, i = extract_section(lines, i + 1, ("Sample ", "Summary"))
                continue
            i += 1
        parsed.samples.append(sample)

    return parsed


def is_failed_report(parsed: ParsedReport) -> bool:
    if parsed.compilation_status == "FAILED":
        return True
    return any(sample.status != "PASS" for sample in parsed.samples)


def normalize_sample_status(status: str) -> str:
    return status.strip().upper().replace(" ", "_")


def get_report_paths() -> list[Path]:
    if not PROFILE_ROOT.is_dir():
        raise FileNotFoundError(f"Profile root not found: {PROFILE_ROOT}")

    report_paths: list[Path] = []
    if USE_FINAL_REPORTS:
        for problem_dir in sorted(path for path in PROFILE_ROOT.iterdir() if path.is_dir() and path.name.startswith("p")):
            report_path = problem_dir / "final_run_report.txt"
            if report_path.is_file():
                report_paths.append(report_path)
        return report_paths

    if ATTEMPT_NUMBER is None:
        raise ValueError("ATTEMPT_NUMBER must be set when USE_FINAL_REPORTS is False.")

    for problem_dir in sorted(path for path in PROFILE_ROOT.iterdir() if path.is_dir() and path.name.startswith("p")):
        report_path = problem_dir / f"attempt_{ATTEMPT_NUMBER}" / "run_report.txt"
        if report_path.is_file():
            report_paths.append(report_path)
    return report_paths


def extract_summary_count(text: str, label: str) -> int:
    match = re.search(rf"^{re.escape(label)}:\s+(\d+)$", text, flags=re.MULTILINE)
    if match is None:
        return 0
    return int(match.group(1))


def summarize_report_counts(report_path: Path) -> dict[str, int]:
    text = report_path.read_text(encoding="utf-8", errors="replace")
    return {
        "passed_samples": extract_summary_count(text, "Passed"),
        "failed_samples": extract_summary_count(text, "Failed"),
        "timed_out_samples": extract_summary_count(text, "Timed out"),
        "runtime_error_samples": extract_summary_count(text, "Runtime errors"),
    }


def remap_dataset_path(raw_path: str | None, problem_id: str) -> Path | None:
    if not raw_path:
        return None

    path = Path(raw_path)
    if path.exists():
        return path

    local_candidate = LOCAL_SAMPLE_ROOT / problem_id / path.name
    if local_candidate.exists():
        return local_candidate

    return None


def read_file_if_present(path: Path | None) -> str | None:
    if path is None or not path.exists():
        return None
    return path.read_text(encoding="utf-8", errors="replace").rstrip("\n")


def single_line(text: str | None) -> str:
    if not text:
        return ""
    return text.replace("\r\n", "\n").replace("\r", "\n").strip()


def build_failure_detail(parsed: ParsedReport, sample: SampleResult) -> dict[str, str]:
    input_path = remap_dataset_path(sample.input_file, parsed.problem_id)
    expected_path = remap_dataset_path(sample.expected_output_file, parsed.problem_id)
    input_text = sample.input_text or read_file_if_present(input_path)
    expected_text = sample.expected_text or read_file_if_present(expected_path)
    return {
        "sample_index": str(sample.sample_index),
        "sample_status": sample.status,
        "exit_code": sample.exit_code or "",
        "input_file": str(input_path or sample.input_file or ""),
        "expected_output_file": str(expected_path or sample.expected_output_file or ""),
        "input_text": single_line(input_text),
        "expected_text": single_line(expected_text),
        "actual_text": single_line(sample.actual_text),
        "stderr_text": single_line(sample.stderr_text),
    }


def build_rows() -> list[dict[str, str | int]]:
    rows: list[dict[str, str | int]] = []
    for report_path in get_report_paths():
        parsed = parse_report(report_path)
        if not is_failed_report(parsed):
            continue

        counts = summarize_report_counts(report_path)
        failing_samples = [
            sample
            for sample in parsed.samples
            if normalize_sample_status(sample.status) == TARGET_STATUS
        ]

        if not failing_samples:
            continue

        for sample in failing_samples:
            detail = build_failure_detail(parsed, sample)
            rows.append(
                {
                    "problem_id": parsed.problem_id,
                    "profile_name": PROFILE_NAME,
                    "dataset_name": DATASET_NAME,
                    "pipeline_name": PIPELINE_NAME,
                    "report_kind": "final" if USE_FINAL_REPORTS else "attempt",
                    "attempt_number": "" if USE_FINAL_REPORTS else ATTEMPT_NUMBER,
                    "target_status": TARGET_STATUS,
                    "compilation_status": parsed.compilation_status or "",
                    "passed_samples": counts["passed_samples"],
                    "failed_samples": counts["failed_samples"],
                    "timed_out_samples": counts["timed_out_samples"],
                    "runtime_error_samples": counts["runtime_error_samples"],
                    "failing_sample_count": len(failing_samples),
                    "sample_index": detail["sample_index"],
                    "sample_status": detail["sample_status"],
                    "exit_code": detail["exit_code"],
                    "input_file": detail["input_file"],
                    "expected_output_file": detail["expected_output_file"],
                    "input_text": detail["input_text"],
                    "expected_text": detail["expected_text"],
                    "actual_text": detail["actual_text"],
                    "stderr_text": detail["stderr_text"],
                    "report_path": str(report_path),
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
                "profile_name",
                "dataset_name",
                "pipeline_name",
                "report_kind",
                "attempt_number",
                "target_status",
                "compilation_status",
                "passed_samples",
                "failed_samples",
                "timed_out_samples",
                "runtime_error_samples",
                "failing_sample_count",
                "sample_index",
                "sample_status",
                "exit_code",
                "input_file",
                "expected_output_file",
                "input_text",
                "expected_text",
                "actual_text",
                "stderr_text",
                "report_path",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    rows = build_rows()
    write_rows(rows)
    unique_problem_ids = sorted({str(row["problem_id"]) for row in rows})

    print(f"PIPELINE_ROOT: {PIPELINE_ROOT}")
    print(f"PIPELINE_NAME: {PIPELINE_NAME}")
    print(f"DATASET_NAME: {DATASET_NAME}")
    print(f"PROFILE_NAME: {PROFILE_NAME}")
    print(f"USE_FINAL_REPORTS: {USE_FINAL_REPORTS}")
    print(f"ATTEMPT_NUMBER: {ATTEMPT_NUMBER}")
    print(f"TARGET_STATUS: {TARGET_STATUS}")
    print(f"CSV rows written: {len(rows)}")
    print(f"Failed problems: {len(unique_problem_ids)}")
    print(f"CSV written to: {OUTPUT_CSV}")


if __name__ == "__main__":
    main()
