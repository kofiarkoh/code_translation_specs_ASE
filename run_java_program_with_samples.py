#!/usr/bin/env python3
"""
Compile one or more Java programs and run them against sample inputs extracted
from HTML.

Expected dataset root shape:
  /.../dataset_root/
    <problem_id>/
      <filename>.java

Or:
  /.../dataset_root/
    <problem_id>/
      Java/
        <filename>.java

Expected sample I/O layout:
  SAMPLE_IO_ROOT/<problem_id>/<problem_id>_sample<N>_input.txt
  SAMPLE_IO_ROOT/<problem_id>/<problem_id>_sample<N>_output.txt
"""

from __future__ import annotations

import csv
import re
import shutil
import subprocess
from pathlib import Path


# =========================
# Configure these values
# =========================
DATASET_ROOT = Path("/Volumes/Extreme SSD/research/code translation/collected_dataset/Java/initial_projects")
SAMPLE_IO_ROOT = Path("/Users/lithe/Documents/research/code_translation_specs/codenet_statistics/collected_dataset/problem_test_inputs_and_outputs")
SOURCE_LANGUAGE = "Java"
SOURCE_EXTENSION = ".java"
COMPILER = "javac"
COMPILER_FLAGS: list[str] = []
RUNTIME = "java"
EXECUTION_TIMEOUT_SECONDS = 10
REPORT_ROOT = Path("/Volumes/Extreme SSD/research/code translation/collected_dataset/Java/java_run_reports/")
MAX_IDS_TO_RUN = 10_000_000
MAX_PROGRAMS_PER_ID = 5
RUN_SUMMARY_CSV = REPORT_ROOT / "run_summary.csv"


def infer_problem_id(path: Path, source_language: str) -> str:
    if path.parent.name == source_language:
        problem_id = path.parent.parent.name
    else:
        problem_id = path.parent.name

    if not problem_id:
        raise ValueError(f"Could not infer problem ID from path: {path}")
    return problem_id


def normalize_output(text: str) -> str:
    lines = [line.rstrip() for line in text.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    while lines and lines[-1] == "":
        lines.pop()
    return "\n".join(lines)


def decode_output(data: bytes | None) -> str:
    if not data:
        return ""
    return data.decode("utf-8", errors="replace")


def extract_package_name(java_source: str) -> str | None:
    match = re.search(r"^\s*package\s+([A-Za-z_][\w.]*)\s*;", java_source, flags=re.MULTILINE)
    if match:
        return match.group(1)
    return None


def extract_declared_class_name(java_source: str, fallback_name: str) -> str:
    patterns = [
        r"\bpublic\s+(?:class|interface|enum|record)\s+([A-Za-z_]\w*)\b",
        r"\b(?:class|interface|enum|record)\s+([A-Za-z_]\w*)\b",
    ]
    for pattern in patterns:
        match = re.search(pattern, java_source)
        if match:
            return match.group(1)
    return fallback_name


def class_name_from_source(java_file_path: Path, java_source: str) -> str:
    package_name = extract_package_name(java_source)
    class_name = extract_declared_class_name(java_source, java_file_path.stem)
    if package_name:
        return f"{package_name}.{class_name}"
    return class_name


def prepare_java_source_file(java_file_path: Path, java_source: str, build_dir: Path) -> Path:
    package_name = extract_package_name(java_source)
    class_name = extract_declared_class_name(java_source, java_file_path.stem)

    source_root = build_dir / "src"
    if package_name:
        package_dir = source_root.joinpath(*package_name.split("."))
    else:
        package_dir = source_root
    package_dir.mkdir(parents=True, exist_ok=True)

    staged_java_file = package_dir / f"{class_name}.java"
    staged_java_file.write_text(java_source, encoding="utf-8")
    return staged_java_file


def compile_java_program(staged_java_file: Path, build_dir: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [COMPILER, *COMPILER_FLAGS, "-d", str(build_dir), str(staged_java_file)],
        capture_output=True,
        text=True,
        errors="replace",
        check=False,
    )


def build_crash_result(java_file_path: Path, problem_id: str, error_message: str) -> dict[str, int | str]:
    work_dir = REPORT_ROOT / problem_id
    work_dir.mkdir(parents=True, exist_ok=True)
    report_path = work_dir / f"{java_file_path.stem}_report.txt"
    sample_csv_path = work_dir / f"{java_file_path.stem}_samples.csv"

    report_lines = [
        f"JAVA_FILE_PATH: {java_file_path}",
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
            fieldnames=["problem_id", "java_file", "sample_index", "status", "exit_code", "input_file", "output_file"],
        )
        writer.writeheader()

    return {
        "problem_id": problem_id,
        "status": "CRASHED",
        "report_path": str(report_path),
        "sample_csv_path": str(sample_csv_path),
        "java_file": str(java_file_path),
        "compiled": 0,
        "passed": 0,
        "failed": 0,
        "timed_out": 0,
        "runtime_errors": 0,
        "total_samples": 0,
    }


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


def find_java_files(dataset_root: Path) -> list[Path]:
    if not dataset_root.is_dir():
        raise FileNotFoundError(f"DATASET_ROOT does not exist or is not a directory: {dataset_root}")

    java_files: list[Path] = []
    ids_selected = 0
    for id_dir in sorted(dataset_root.iterdir()):
        if not id_dir.is_dir():
            continue

        source_dir = id_dir / SOURCE_LANGUAGE
        if source_dir.is_dir():
            search_dir = source_dir
        else:
            search_dir = id_dir

        id_added = False
        programs_added_for_id = 0
        for java_file in sorted(search_dir.glob(f"*{SOURCE_EXTENSION}")):
            if not java_file.is_file():
                continue
            if not id_added:
                ids_selected += 1
                id_added = True
            java_files.append(java_file)
            programs_added_for_id += 1
            if programs_added_for_id >= MAX_PROGRAMS_PER_ID:
                break
        if ids_selected >= MAX_IDS_TO_RUN:
            return java_files

    return java_files


def run_one_program(java_file_path: Path) -> dict[str, int | str]:
    problem_id = infer_problem_id(java_file_path, SOURCE_LANGUAGE)
    try:
        sample_pairs = find_sample_pairs(problem_id)
        if not sample_pairs:
            print(f"[SKIP] {java_file_path.name}: no sample input/output pairs for {problem_id}")
            return {
                "problem_id": problem_id,
                "status": "NO_SAMPLES",
                "report_path": "",
                "sample_csv_path": "",
                "java_file": str(java_file_path),
                "compiled": 0,
                "passed": 0,
                "failed": 0,
                "timed_out": 0,
                "runtime_errors": 0,
                "total_samples": 0,
            }

        work_dir = REPORT_ROOT / problem_id
        work_dir.mkdir(parents=True, exist_ok=True)
        build_dir = work_dir / f"{java_file_path.stem}_classes"
        if build_dir.exists():
            shutil.rmtree(build_dir)
        build_dir.mkdir(parents=True, exist_ok=True)

        report_path = work_dir / f"{java_file_path.stem}_report.txt"
        sample_csv_path = work_dir / f"{java_file_path.stem}_samples.csv"
        java_source = java_file_path.read_text(encoding="utf-8", errors="ignore")
        staged_java_file = prepare_java_source_file(java_file_path, java_source, build_dir)
        main_class = class_name_from_source(java_file_path, java_source)

        compile_result = compile_java_program(staged_java_file, build_dir)

        report_lines: list[str] = []
        sample_rows: list[dict[str, str | int]] = []
        report_lines.append(f"JAVA_FILE_PATH: {java_file_path}")
        report_lines.append(f"PROBLEM_ID: {problem_id}")
        report_lines.append(f"SAMPLE_IO_ROOT: {SAMPLE_IO_ROOT}")
        report_lines.append(f"COMPILER: {COMPILER}")
        report_lines.append(f"COMPILER_FLAGS: {' '.join(COMPILER_FLAGS)}")
        report_lines.append(f"RUNTIME: {RUNTIME}")
        report_lines.append(f"STAGED_JAVA_FILE: {staged_java_file}")
        report_lines.append(f"MAIN_CLASS: {main_class}")
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
                    fieldnames=["problem_id", "java_file", "sample_index", "status", "exit_code", "input_file", "output_file"],
                )
                writer.writeheader()
            print(f"[COMPILE FAILED] {java_file_path.name}")
            print(f"Report saved to: {report_path}")
            return {
                "problem_id": problem_id,
                "status": "COMPILE_FAILED",
                "report_path": str(report_path),
                "sample_csv_path": str(sample_csv_path),
                "java_file": str(java_file_path),
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
                    [RUNTIME, "-cp", str(build_dir), main_class],
                    input=input_text.encode("utf-8"),
                    capture_output=True,
                    timeout=EXECUTION_TIMEOUT_SECONDS,
                    check=False,
                )
            except subprocess.TimeoutExpired:
                timed_out += 1
                sample_rows.append({
                    "problem_id": problem_id,
                    "java_file": str(java_file_path),
                    "sample_index": sample_idx,
                    "status": "TIMEOUT",
                    "exit_code": "",
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                })
                report_lines.append(f"Sample {sample_idx}: TIMEOUT")
                report_lines.append(f"Input file: {input_path}")
                report_lines.append("")
                print(f"[TIMEOUT] {java_file_path.name} sample {sample_idx}")
                continue

            actual_output = normalize_output(decode_output(run_result.stdout))
            stderr_text = decode_output(run_result.stderr)
            expected_matches = actual_output == expected_output

            if run_result.returncode != 0:
                runtime_errors += 1
                sample_rows.append({
                    "problem_id": problem_id,
                    "java_file": str(java_file_path),
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
                print(f"[RUNTIME ERROR] {java_file_path.name} sample {sample_idx}")
                continue

            if expected_matches:
                passed += 1
                sample_rows.append({
                    "problem_id": problem_id,
                    "java_file": str(java_file_path),
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
                print(f"[PASS] {java_file_path.name} sample {sample_idx}")
            else:
                failed += 1
                sample_rows.append({
                    "problem_id": problem_id,
                    "java_file": str(java_file_path),
                    "sample_index": sample_idx,
                    "status": "FAIL",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                })
                report_lines.append(f"Sample {sample_idx}: FAIL")
                report_lines.append(f"Input file: {input_path}")
                report_lines.append(f"Expected output file: {output_path}")
                report_lines.append("Expected:")
                report_lines.append(expected_output)
                report_lines.append("Actual:")
                report_lines.append(actual_output)
                if stderr_text.strip():
                    report_lines.append("stderr:")
                    report_lines.append(stderr_text.rstrip())
                report_lines.append("")
                print(f"[FAIL] {java_file_path.name} sample {sample_idx}")

        report_lines.append("Summary:")
        report_lines.append(f"Total samples: {len(sample_pairs)}")
        report_lines.append(f"Passed: {passed}")
        report_lines.append(f"Failed: {failed}")
        report_lines.append(f"Timed out: {timed_out}")
        report_lines.append(f"Runtime errors: {runtime_errors}")

        report_path.write_text("\n".join(report_lines) + "\n", encoding="utf-8")
        with sample_csv_path.open("w", newline="", encoding="utf-8") as csv_file:
            writer = csv.DictWriter(
                csv_file,
                fieldnames=["problem_id", "java_file", "sample_index", "status", "exit_code", "input_file", "output_file"],
            )
            writer.writeheader()
            writer.writerows(sample_rows)

        print(f"[DONE] {java_file_path.name}: passed={passed}, failed={failed}, timed_out={timed_out}, runtime_errors={runtime_errors}")
        print(f"Report saved to: {report_path}")
        print(f"Sample CSV saved to: {sample_csv_path}")
        status = "PASS"
        if failed > 0:
            status = "FAIL"
        elif timed_out > 0:
            status = "TIMEOUT"
        elif runtime_errors > 0:
            status = "RUNTIME_ERROR"
        return {
            "problem_id": problem_id,
            "status": status,
            "report_path": str(report_path),
            "sample_csv_path": str(sample_csv_path),
            "java_file": str(java_file_path),
            "compiled": 1,
            "passed": passed,
            "failed": failed,
            "timed_out": timed_out,
            "runtime_errors": runtime_errors,
            "total_samples": len(sample_pairs),
        }
    except Exception as exc:
        print(f"[CRASHED] {java_file_path.name}: {exc}")
        return build_crash_result(java_file_path, problem_id, repr(exc))


def main() -> None:
    if not SAMPLE_IO_ROOT.is_dir():
        raise FileNotFoundError(f"SAMPLE_IO_ROOT does not exist or is not a directory: {SAMPLE_IO_ROOT}")

    REPORT_ROOT.mkdir(parents=True, exist_ok=True)
    java_files = find_java_files(DATASET_ROOT)
    if not java_files:
        raise FileNotFoundError(f"No {SOURCE_EXTENSION} files found under dataset root: {DATASET_ROOT}")

    compiled = 0
    total_samples = 0
    passed = 0
    failed = 0
    timed_out = 0
    runtime_errors = 0
    summary_rows: list[dict[str, str | int]] = []

    for java_file_path in java_files:
        result = run_one_program(java_file_path)
        compiled += int(result["compiled"])
        total_samples += int(result["total_samples"])
        passed += int(result["passed"])
        failed += int(result["failed"])
        timed_out += int(result["timed_out"])
        runtime_errors += int(result["runtime_errors"])
        summary_rows.append({
            "problem_id": result["problem_id"],
            "status": result.get("status", ""),
            "java_file": result.get("java_file", ""),
            "compiled": result["compiled"],
            "total_samples": result["total_samples"],
            "passed": result["passed"],
            "failed": result["failed"],
            "timed_out": result["timed_out"],
            "runtime_errors": result["runtime_errors"],
            "report_path": result["report_path"],
            "sample_csv_path": result.get("sample_csv_path", ""),
        })

    with RUN_SUMMARY_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "status",
                "java_file",
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
    print(f"Dataset root: {DATASET_ROOT}")
    print(f"IDs considered: {MAX_IDS_TO_RUN}")
    print(f"Programs per ID: {MAX_PROGRAMS_PER_ID}")
    print(f"Programs attempted: {len(java_files)}")
    print(f"Programs compiled successfully: {compiled}")
    print(f"Total samples run: {total_samples}")
    print(f"Passed: {passed}")
    print(f"Failed: {failed}")
    print(f"Timed out: {timed_out}")
    print(f"Runtime errors: {runtime_errors}")
    print(f"Run summary CSV: {RUN_SUMMARY_CSV}")


if __name__ == "__main__":
    main()
