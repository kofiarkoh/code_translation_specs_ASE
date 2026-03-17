#!/usr/bin/env python3
from __future__ import annotations

import csv
import os
import re
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path
from dotenv import load_dotenv

load_dotenv()

# =========================
# Configure these values
# =========================
PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_ROOT = PROJECT_ROOT / "r_code/src"
SAMPLE_IO_ROOT = PROJECT_ROOT / "collected_dataset/problem_test_inputs_and_outputs"
COMPILER = "rustc"
COMPILER_FLAGS = ["--edition=2021", "-O"]
EXECUTION_TIMEOUT_SECONDS = 10

MAX_IDS_TO_RUN = 10_000_000
MAX_PROGRAMS_PER_ID = 10

PARALLEL_EXECUTIONS = 5

dataset_name = os.environ.get("DATASET_NAME")
if not dataset_name:
    raise ValueError("DATASET_NAME environment variable is not set")

REPORT_ROOT = "results"/ Path(dataset_name) / "rust_run_reports"
RUN_SUMMARY_CSV = REPORT_ROOT / "run_summary.csv"
REPORT_ROOT = PROJECT_ROOT / REPORT_ROOT
RUN_SUMMARY_CSV = PROJECT_ROOT / RUN_SUMMARY_CSV

def normalize_output(text: str) -> str:
    lines = [line.rstrip() for line in text.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    while lines and lines[-1] == "":
        lines.pop()
    return "\n".join(lines)


def compile_rust_program(rust_file_path: Path, executable_path: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [COMPILER, *COMPILER_FLAGS, str(rust_file_path), "-o", str(executable_path)],
        capture_output=True,
        text=True,
        errors="replace",
        check=False,
    )


def decode_output(data: bytes | None) -> str:
    if not data:
        return ""
    return data.decode("utf-8", errors="replace")


def find_sample_pairs(problem_id: str) -> list[tuple[int, Path, Path]]:
    problem_dir = SAMPLE_IO_ROOT / problem_id
    if not problem_dir.is_dir():
        return []

    sample_pairs: list[tuple[int, Path, Path]] = []
    pattern = re.compile(rf"^{re.escape(problem_id)}_sample(\d+)_input\.txt$")
    for input_path in sorted(problem_dir.glob(f"{problem_id}_sample*_input.txt")):
        match = pattern.match(input_path.name)
        if match is None:
            continue
        sample_idx = int(match.group(1))
        output_path = problem_dir / f"{problem_id}_sample{sample_idx}_output.txt"
        if output_path.is_file():
            sample_pairs.append((sample_idx, input_path, output_path))
    return sample_pairs


def find_rust_files(dataset_root: Path) -> list[Path]:
    if not dataset_root.is_dir():
        raise FileNotFoundError(f"DATASET_ROOT does not exist or is not a directory: {dataset_root}")

    rust_files: list[Path] = []
    ids_selected = 0

    for id_dir in sorted(dataset_root.iterdir()):
        if not id_dir.is_dir():
            continue

        id_files = [path for path in sorted(id_dir.glob("*.rs")) if path.is_file()]
        if not id_files:
            continue

        ids_selected += 1
        rust_files.extend(id_files[:MAX_PROGRAMS_PER_ID])
        if ids_selected >= MAX_IDS_TO_RUN:
            break

    return rust_files


def build_crash_result(rust_file_path: Path, problem_id: str, error_message: str) -> dict[str, int | str]:
    work_dir = REPORT_ROOT / problem_id
    work_dir.mkdir(parents=True, exist_ok=True)
    report_path = work_dir / f"{rust_file_path.stem}_report.txt"
    sample_csv_path = work_dir / f"{rust_file_path.stem}_samples.csv"

    report_lines = [
        f"RUST_FILE_PATH: {rust_file_path}",
        f"PROBLEM_ID: {problem_id}",
        "Status: CRASHED",
        "",
        "Error:",
        error_message,
        "",
    ]
    report_path.write_text("\n".join(report_lines), encoding="utf-8")

    with sample_csv_path.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=["problem_id", "rust_file", "sample_index", "status", "exit_code", "input_file", "output_file"],
        )
        writer.writeheader()

    return {
        "problem_id": problem_id,
        "status": "CRASHED",
        "report_path": str(report_path),
        "sample_csv_path": str(sample_csv_path),
        "rust_file": str(rust_file_path),
        "compiled": 0,
        "passed": 0,
        "failed": 0,
        "timed_out": 0,
        "runtime_errors": 0,
        "total_samples": 0,
    }


def run_one_program(rust_file_path: Path) -> dict[str, int | str]:
    problem_id = rust_file_path.parent.name
    try:
        sample_pairs = find_sample_pairs(problem_id)
        if not sample_pairs:
            print(f"[SKIP] {rust_file_path.name}: no sample input/output pairs for {problem_id}")
            return {
                "problem_id": problem_id,
                "status": "NO_SAMPLES",
                "report_path": "",
                "sample_csv_path": "",
                "rust_file": str(rust_file_path),
                "compiled": 0,
                "passed": 0,
                "failed": 0,
                "timed_out": 0,
                "runtime_errors": 0,
                "total_samples": 0,
            }

        work_dir = REPORT_ROOT / problem_id
        work_dir.mkdir(parents=True, exist_ok=True)
        executable_path = work_dir / f"{rust_file_path.stem}.out"
        report_path = work_dir / f"{rust_file_path.stem}_report.txt"
        sample_csv_path = work_dir / f"{rust_file_path.stem}_samples.csv"

        compile_result = compile_rust_program(rust_file_path, executable_path)

        report_lines: list[str] = []
        sample_rows: list[dict[str, str | int]] = []
        report_lines.append(f"RUST_FILE_PATH: {rust_file_path}")
        report_lines.append(f"PROBLEM_ID: {problem_id}")
        report_lines.append(f"SAMPLE_IO_ROOT: {SAMPLE_IO_ROOT}")
        report_lines.append(f"COMPILER: {COMPILER}")
        report_lines.append(f"COMPILER_FLAGS: {' '.join(COMPILER_FLAGS)}")
        report_lines.append("")

        if compile_result.returncode != 0:
            report_lines.append("Compilation: FAILED")
            report_lines.append("")
            report_lines.append("Compiler stderr:")
            report_lines.append(compile_result.stderr.rstrip())
            report_path.write_text("\n".join(report_lines) + "\n", encoding="utf-8")
            with sample_csv_path.open("w", newline="", encoding="utf-8") as csv_file:
                writer = csv.DictWriter(
                    csv_file,
                    fieldnames=["problem_id", "rust_file", "sample_index", "status", "exit_code", "input_file", "output_file"],
                )
                writer.writeheader()
            print(f"[COMPILE FAILED] {rust_file_path.name}")
            return {
                "problem_id": problem_id,
                "status": "COMPILE_FAILED",
                "report_path": str(report_path),
                "sample_csv_path": str(sample_csv_path),
                "rust_file": str(rust_file_path),
                "compiled": 0,
                "passed": 0,
                "failed": 0,
                "timed_out": 0,
                "runtime_errors": 0,
                "total_samples": len(sample_pairs),
            }

        report_lines.append("Compilation: PASSED")
        report_lines.append("")

        passed = 0
        failed = 0
        timed_out = 0
        runtime_errors = 0

        for sample_idx, input_path, output_path in sample_pairs:
            input_text = input_path.read_text(encoding="utf-8", errors="ignore")
            expected_output = normalize_output(output_path.read_text(encoding="utf-8", errors="ignore"))

            try:
                run_result = subprocess.run(
                    [str(executable_path)],
                    input=input_text.encode("utf-8"),
                    capture_output=True,
                    timeout=EXECUTION_TIMEOUT_SECONDS,
                    check=False,
                )
            except subprocess.TimeoutExpired:
                timed_out += 1
                sample_rows.append({
                    "problem_id": problem_id,
                    "rust_file": str(rust_file_path),
                    "sample_index": sample_idx,
                    "status": "TIMEOUT",
                    "exit_code": "",
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                })
                report_lines.append(f"Sample {sample_idx}: TIMEOUT")
                report_lines.append(f"Input file: {input_path}")
                report_lines.append("")
                print(f"[TIMEOUT] {rust_file_path.name} sample {sample_idx}")
                continue

            actual_output = normalize_output(decode_output(run_result.stdout))
            stderr_text = decode_output(run_result.stderr)
            expected_matches = actual_output == expected_output

            if run_result.returncode != 0:
                runtime_errors += 1
                sample_rows.append({
                    "problem_id": problem_id,
                    "rust_file": str(rust_file_path),
                    "sample_index": sample_idx,
                    "status": "RUNTIME_ERROR",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                })
                report_lines.append(f"Sample {sample_idx}: RUNTIME ERROR")
                report_lines.append(f"Input file: {input_path}")
                report_lines.append(f"Expected output file: {output_path}")
                report_lines.append(f"Exit code: {run_result.returncode}")
                if stderr_text.strip():
                    report_lines.append("stderr:")
                    report_lines.append(stderr_text.rstrip())
                report_lines.append("")
                print(f"[RUNTIME ERROR] {rust_file_path.name} sample {sample_idx}")
                continue

            if expected_matches:
                passed += 1
                sample_rows.append({
                    "problem_id": problem_id,
                    "rust_file": str(rust_file_path),
                    "sample_index": sample_idx,
                    "status": "PASS",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                })
                report_lines.append(f"Sample {sample_idx}: PASS")
                report_lines.append(f"Input file: {input_path}")
                report_lines.append(f"Expected output file: {output_path}")
                report_lines.append("Input:")
                report_lines.append(normalize_output(input_text))
                report_lines.append("Expected:")
                report_lines.append(expected_output)
                report_lines.append("Actual:")
                report_lines.append(actual_output)
                report_lines.append("")
                print(f"[PASS] {rust_file_path.name} sample {sample_idx}")
            else:
                failed += 1
                sample_rows.append({
                    "problem_id": problem_id,
                    "rust_file": str(rust_file_path),
                    "sample_index": sample_idx,
                    "status": "FAIL",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                })
                report_lines.append(f"Sample {sample_idx}: FAIL")
                report_lines.append(f"Input file: {input_path}")
                report_lines.append(f"Expected output file: {output_path}")
                report_lines.append("Input:")
                report_lines.append(normalize_output(input_text))
                report_lines.append("Expected:")
                report_lines.append(expected_output)
                report_lines.append("Actual:")
                report_lines.append(actual_output)
                if stderr_text.strip():
                    report_lines.append("stderr:")
                    report_lines.append(stderr_text.rstrip())
                report_lines.append("")
                print(f"[FAIL] {rust_file_path.name} sample {sample_idx}")

        report_lines.append("Summary")
        report_lines.append(f"Total samples: {len(sample_pairs)}")
        report_lines.append(f"Passed: {passed}")
        report_lines.append(f"Failed: {failed}")
        report_lines.append(f"Timed out: {timed_out}")
        report_lines.append(f"Runtime errors: {runtime_errors}")
        report_path.write_text("\n".join(report_lines) + "\n", encoding="utf-8")

        with sample_csv_path.open("w", newline="", encoding="utf-8") as csv_file:
            writer = csv.DictWriter(
                csv_file,
                fieldnames=["problem_id", "rust_file", "sample_index", "status", "exit_code", "input_file", "output_file"],
            )
            writer.writeheader()
            writer.writerows(sample_rows)

        if failed > 0:
            overall_status = "FAIL"
        elif timed_out > 0:
            overall_status = "TIMEOUT"
        elif runtime_errors > 0:
            overall_status = "RUNTIME_ERROR"
        else:
            overall_status = "PASS"

        return {
            "problem_id": problem_id,
            "status": overall_status,
            "report_path": str(report_path),
            "sample_csv_path": str(sample_csv_path),
            "rust_file": str(rust_file_path),
            "compiled": 1,
            "passed": passed,
            "failed": failed,
            "timed_out": timed_out,
            "runtime_errors": runtime_errors,
            "total_samples": len(sample_pairs),
        }
    except Exception as exc:
        print(f"[CRASHED] {rust_file_path}: {exc}")
        return build_crash_result(rust_file_path, problem_id, repr(exc))


def main() -> None:
    REPORT_ROOT.mkdir(parents=True, exist_ok=True)
    rust_files = find_rust_files(DATASET_ROOT)
    if not rust_files:
        raise FileNotFoundError(f"No .rs files found under DATASET_ROOT: {DATASET_ROOT}")

    summary_rows: list[dict[str, int | str]] = []
    with ThreadPoolExecutor(max_workers=PARALLEL_EXECUTIONS) as executor:
        future_to_rust_file = {
            executor.submit(run_one_program, rust_file_path): rust_file_path
            for rust_file_path in rust_files
        }
        for future in as_completed(future_to_rust_file):
            rust_file_path = future_to_rust_file[future]
            try:
                summary_rows.append(future.result())
            except Exception as exc:
                print(f"[CRASHED] {rust_file_path}: {exc}")
                summary_rows.append(
                    build_crash_result(rust_file_path, rust_file_path.parent.name, repr(exc))
                )

    with RUN_SUMMARY_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "rust_file",
                "status",
                "compiled",
                "total_samples",
                "passed",
                "failed",
                "timed_out",
                "runtime_errors",
                "report_path",
                "sample_csv_path",
            ],
        )
        writer.writeheader()
        writer.writerows(summary_rows)

    print("\nDone.")
    print(f"Programs processed: {len(summary_rows)}")
    print(f"Run summary CSV: {RUN_SUMMARY_CSV}")
    print(f"Report root: {REPORT_ROOT}")


if __name__ == "__main__":
    main()
