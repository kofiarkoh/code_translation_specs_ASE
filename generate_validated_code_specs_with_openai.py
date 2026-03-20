#!/usr/bin/env python3
"""
Generate behavioral specifications from C programs, validate them by regenerating C,
and iteratively refine the specification until the regenerated program passes the
sample tests or the attempt budget is exhausted.

Expected input shape:
PROBLEMS_ROOT/
  p00001/
    file1.c
  p00002/
    C/
      file2.c

Expected sample I/O layout:
SAMPLE_IO_ROOT/<problem_id>/<problem_id>_sample<N>_input.txt
SAMPLE_IO_ROOT/<problem_id>/<problem_id>_sample<N>_output.txt
"""

from __future__ import annotations

import csv
import os
import re
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI
from validated_spec_prompts import (
    DIAGNOSIS_PROMPT_TEMPLATE,
    INITIAL_SPEC_PROMPT_TEMPLATE,
    SPEC_FIX_PROMPT_TEMPLATE,
    SPEC_TO_C_PROMPT_TEMPLATE,
    SYSTEM_PROMPT,
)


# =========================
# Configure these values
# =========================
PROBLEMS_ROOT = Path("./collected_dataset/C/final_problems")
SAMPLE_IO_ROOT = Path("./collected_dataset/problem_test_inputs_and_outputs")
MODEL = "gpt-4o-mini"
DATASET_NAME = "optimized_prompt@1"
OUTPUT_ROOT = Path(f"./api_responses/open_ai/{DATASET_NAME}/C/validated_specs_WITH_GPT_{MODEL}")

FILE_GLOB = "*.c"
MAX_PROBLEMS_TO_PROCESS = 2
MAX_FILES_PER_PROBLEM = 1
SOURCE_LANGUAGE = "C"
API_TIMEOUT_SECONDS = 180
PARALLEL_REQUESTS = 4
MAX_VALIDATION_ATTEMPTS = 3
COMPILER = "gcc"
COMPILER_FLAGS = ["-O2", "-std=c11"]
EXECUTION_TIMEOUT_SECONDS = 10
SKIP_IF_OUTPUT_EXISTS = True


def list_code_files(problem_dir: Path) -> list[Path]:
    direct_files = sorted(p for p in problem_dir.glob(FILE_GLOB) if p.is_file())
    if direct_files:
        return direct_files[:MAX_FILES_PER_PROBLEM]

    language_dir = problem_dir / SOURCE_LANGUAGE
    if language_dir.is_dir():
        nested_files = sorted(p for p in language_dir.glob(FILE_GLOB) if p.is_file())
        return nested_files[:MAX_FILES_PER_PROBLEM]

    return []


def call_openai(client: OpenAI, prompt: str) -> str:
    response = client.responses.create(
        model=MODEL,
        temperature=0,
        input=[
            {"role": "system", "content": [{"type": "input_text", "text": SYSTEM_PROMPT}]},
            {"role": "user", "content": [{"type": "input_text", "text": prompt}]},
        ],
        timeout=API_TIMEOUT_SECONDS,
    )
    if response.output_text:
        return response.output_text
    raise ValueError("OpenAI response did not include output_text.")


def extract_code_from_response(text: str) -> str:
    stripped = text.strip()
    if stripped.startswith("```"):
        lines = stripped.splitlines()
        if lines:
            lines = lines[1:]
        if lines and lines[-1].strip() == "```":
            lines = lines[:-1]
        return "\n".join(lines).rstrip() + "\n"
    return stripped.rstrip() + "\n"


def normalize_output(text: str) -> str:
    lines = [line.rstrip() for line in text.replace("\r\n", "\n").replace("\r", "\n").split("\n")]
    while lines and lines[-1] == "":
        lines.pop()
    return "\n".join(lines)


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


def compile_c_program(c_file_path: Path, executable_path: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [COMPILER, *COMPILER_FLAGS, str(c_file_path), "-o", str(executable_path)],
        capture_output=True,
        text=True,
        errors="replace",
        check=False,
    )


def execute_candidate(c_file_path: Path, problem_id: str, work_dir: Path) -> dict[str, str | int]:
    report_path = work_dir / "run_report.txt"
    sample_csv_path = work_dir / "samples.csv"
    executable_path = work_dir / f"{c_file_path.stem}.out"
    sample_pairs = find_sample_pairs(problem_id)
    sample_csv_fieldnames = [
        "problem_id",
        "c_file",
        "sample_index",
        "status",
        "exit_code",
        "input_file",
        "output_file",
        "input_text",
        "expected_output",
        "actual_output",
        "stderr_text",
    ]

    if not sample_pairs:
        report_text = "Status: NO_SAMPLES\nNo sample input/output pairs were found.\n"
        report_path.write_text(report_text, encoding="utf-8")
        with sample_csv_path.open("w", newline="", encoding="utf-8") as csv_file:
            writer = csv.DictWriter(csv_file, fieldnames=sample_csv_fieldnames)
            writer.writeheader()
        return {
            "status": "NO_SAMPLES",
            "report_path": str(report_path),
            "sample_csv_path": str(sample_csv_path),
            "passed": 0,
            "failed": 0,
            "timed_out": 0,
            "runtime_errors": 0,
            "total_samples": 0,
        }

    compile_result = compile_c_program(c_file_path, executable_path)
    report_lines: list[str] = [f"COMPILER: {COMPILER}", f"COMPILER_FLAGS: {' '.join(COMPILER_FLAGS)}", ""]
    sample_rows: list[dict[str, str | int]] = []

    if compile_result.returncode != 0:
        report_lines.append("Compilation: FAILED")
        report_lines.append("")
        report_lines.append("Compiler stderr:")
        report_lines.append(compile_result.stderr.rstrip())
        report_path.write_text("\n".join(report_lines) + "\n", encoding="utf-8")
        with sample_csv_path.open("w", newline="", encoding="utf-8") as csv_file:
            writer = csv.DictWriter(csv_file, fieldnames=sample_csv_fieldnames)
            writer.writeheader()
        return {
            "status": "COMPILE_FAILED",
            "report_path": str(report_path),
            "sample_csv_path": str(sample_csv_path),
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
            sample_rows.append(
                {
                    "problem_id": problem_id,
                    "c_file": str(c_file_path),
                    "sample_index": sample_idx,
                    "status": "TIMEOUT",
                    "exit_code": "",
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                    "input_text": normalize_output(input_text),
                    "expected_output": expected_output,
                    "actual_output": "",
                    "stderr_text": "",
                }
            )
            report_lines.extend([f"Sample {sample_idx}: TIMEOUT", f"Input file: {input_path}", ""])
            continue

        actual_output = normalize_output(decode_output(run_result.stdout))
        stderr_text = decode_output(run_result.stderr)
        expected_matches = actual_output == expected_output

        if run_result.returncode != 0:
            runtime_errors += 1
            sample_rows.append(
                {
                    "problem_id": problem_id,
                    "c_file": str(c_file_path),
                    "sample_index": sample_idx,
                    "status": "RUNTIME_ERROR",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                    "input_text": normalize_output(input_text),
                    "expected_output": expected_output,
                    "actual_output": actual_output,
                    "stderr_text": stderr_text.rstrip(),
                }
            )
            report_lines.extend(
                [
                    f"Sample {sample_idx}: RUNTIME ERROR",
                    f"Input file: {input_path}",
                    f"Expected output file: {output_path}",
                    f"Exit code: {run_result.returncode}",
                ]
            )
            if stderr_text.strip():
                report_lines.extend(["stderr:", stderr_text.rstrip()])
            report_lines.append("")
            continue

        if expected_matches:
            passed += 1
            sample_rows.append(
                {
                    "problem_id": problem_id,
                    "c_file": str(c_file_path),
                    "sample_index": sample_idx,
                    "status": "PASS",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                    "input_text": normalize_output(input_text),
                    "expected_output": expected_output,
                    "actual_output": actual_output,
                    "stderr_text": stderr_text.rstrip(),
                }
            )
            report_lines.extend(
                [
                    f"Sample {sample_idx}: PASS",
                    f"Input file: {input_path}",
                    f"Expected output file: {output_path}",
                    "Input:",
                    normalize_output(input_text),
                    "Expected:",
                    expected_output,
                    "Actual:",
                    actual_output,
                    "",
                ]
            )
        else:
            failed += 1
            sample_rows.append(
                {
                    "problem_id": problem_id,
                    "c_file": str(c_file_path),
                    "sample_index": sample_idx,
                    "status": "FAIL",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
                    "input_text": normalize_output(input_text),
                    "expected_output": expected_output,
                    "actual_output": actual_output,
                    "stderr_text": stderr_text.rstrip(),
                }
            )
            report_lines.extend(
                [
                    f"Sample {sample_idx}: FAIL",
                    f"Input file: {input_path}",
                    f"Expected output file: {output_path}",
                    "Input:",
                    normalize_output(input_text),
                    "Expected:",
                    expected_output,
                    "Actual:",
                    actual_output,
                ]
            )
            if stderr_text.strip():
                report_lines.extend(["stderr:", stderr_text.rstrip()])
            report_lines.append("")

    report_lines.extend(
        [
            "Summary",
            f"Total samples: {len(sample_pairs)}",
            f"Passed: {passed}",
            f"Failed: {failed}",
            f"Timed out: {timed_out}",
            f"Runtime errors: {runtime_errors}",
        ]
    )
    report_path.write_text("\n".join(report_lines) + "\n", encoding="utf-8")

    with sample_csv_path.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(csv_file, fieldnames=sample_csv_fieldnames)
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
        "status": overall_status,
        "report_path": str(report_path),
        "sample_csv_path": str(sample_csv_path),
        "passed": passed,
        "failed": failed,
        "timed_out": timed_out,
        "runtime_errors": runtime_errors,
        "total_samples": len(sample_pairs),
    }


def process_code_file(
    client: OpenAI,
    problem_name: str,
    code_file: Path,
    out_problem_dir: Path,
) -> dict[str, str | int]:
    code_text = code_file.read_text(encoding="utf-8", errors="ignore")
    current_spec = ""
    final_status = "ERROR"
    attempts_used = 0
    best_attempt_number = 0
    best_passed = -1
    best_total_samples = 0
    best_status = "ERROR"

    for attempt_number in range(1, MAX_VALIDATION_ATTEMPTS + 1):
        attempt_dir = out_problem_dir / f"attempt_{attempt_number}"
        attempt_dir.mkdir(parents=True, exist_ok=True)

        if attempt_number == 1:
            spec_prompt = INITIAL_SPEC_PROMPT_TEMPLATE.format(code=code_text)
            spec_prompt_path = attempt_dir / "initial_translation_prompt.txt"
            spec_prompt_path.write_text(spec_prompt, encoding="utf-8")
            spec_response = call_openai(client, spec_prompt)
            current_spec = spec_response.rstrip() + "\n"
        else:
            previous_attempt_dir = attempt_dir.parent / f"attempt_{attempt_number - 1}"
            report_text = (previous_attempt_dir / "run_report.txt").read_text(
                encoding="utf-8",
                errors="ignore",
            )
            generated_code = (previous_attempt_dir / "generated_from_spec.c").read_text(
                encoding="utf-8",
                errors="ignore",
            )
            diagnosis_prompt = DIAGNOSIS_PROMPT_TEMPLATE.format(
                original_code=code_text,
                current_spec=current_spec,
                generated_code=generated_code,
                report_text=report_text,
            )
            diagnosis_prompt_path = attempt_dir / "diagnosis_prompt.txt"
            diagnosis_prompt_path.write_text(diagnosis_prompt, encoding="utf-8")
            diagnosis_response = call_openai(client, diagnosis_prompt)
            diagnosis_text = diagnosis_response.rstrip() + "\n"
            diagnosis_path = attempt_dir / "diagnosis.txt"
            diagnosis_path.write_text(diagnosis_text, encoding="utf-8")

            spec_fix_prompt = SPEC_FIX_PROMPT_TEMPLATE.format(
                original_code=code_text,
                current_spec=current_spec,
                diagnosis_text=diagnosis_text,
            )
            spec_fix_prompt_path = attempt_dir / "spec_fix_prompt.txt"
            spec_fix_prompt_path.write_text(spec_fix_prompt, encoding="utf-8")
            spec_response = call_openai(client, spec_fix_prompt)
            current_spec = spec_response.rstrip() + "\n"

        spec_path = attempt_dir / "spec.txt"
        spec_path.write_text(current_spec, encoding="utf-8")

        c_prompt = SPEC_TO_C_PROMPT_TEMPLATE.format(spec=current_spec)
        c_prompt_path = attempt_dir / "c_generation_prompt.txt"
        c_prompt_path.write_text(c_prompt, encoding="utf-8")
        c_response = call_openai(client, c_prompt)
        c_response_path = attempt_dir / "c_generation_response.txt"
        c_response_path.write_text(c_response.rstrip() + "\n", encoding="utf-8")

        generated_c = extract_code_from_response(c_response)
        generated_c_path = attempt_dir / "generated_from_spec.c"
        generated_c_path.write_text(generated_c, encoding="utf-8")

        execution_result = execute_candidate(generated_c_path, problem_name, attempt_dir)
        final_status = str(execution_result["status"])
        attempts_used = attempt_number
        passed_samples = int(execution_result["passed"])
        total_samples = int(execution_result["total_samples"])

        if (
            passed_samples > best_passed
            or (passed_samples == best_passed and total_samples > best_total_samples)
            or (
                passed_samples == best_passed
                and total_samples == best_total_samples
                and attempt_number > best_attempt_number
            )
        ):
            best_attempt_number = attempt_number
            best_passed = passed_samples
            best_total_samples = total_samples
            best_status = final_status

        print(f"[ATTEMPT] {problem_name}/{code_file.name} attempt={attempt_number} status={final_status}")

        if final_status == "PASS":
            break

    selected_attempt_number = attempts_used if final_status == "PASS" else best_attempt_number
    selected_attempt_dir = out_problem_dir / f"attempt_{selected_attempt_number}"

    final_spec_src = selected_attempt_dir / "spec.txt"
    final_spec_path = out_problem_dir / f"{code_file.stem}_spec.txt"
    if final_spec_src.is_file():
        final_spec_path.write_text(final_spec_src.read_text(encoding="utf-8", errors="ignore"), encoding="utf-8")
    else:
        final_spec_path.write_text(current_spec, encoding="utf-8")

    final_report_src = selected_attempt_dir / "run_report.txt"
    if final_report_src.is_file():
        (out_problem_dir / "final_run_report.txt").write_text(
            final_report_src.read_text(encoding="utf-8", errors="ignore"),
            encoding="utf-8",
        )

    final_generated_c_src = selected_attempt_dir / "generated_from_spec.c"
    if final_generated_c_src.is_file():
        (out_problem_dir / "final_generated_from_spec.c").write_text(
            final_generated_c_src.read_text(encoding="utf-8", errors="ignore"),
            encoding="utf-8",
        )

    return {
        "problem_id": problem_name,
        "source_file": code_file.name,
        "attempts_used": attempts_used,
        "final_status": final_status,
        "selected_attempt_number": selected_attempt_number,
        "selected_status": final_status if final_status == "PASS" else best_status,
        "best_passed_samples": best_passed,
        "best_total_samples": best_total_samples,
        "final_spec_path": str(final_spec_path),
        "final_report_path": str(out_problem_dir / "final_run_report.txt"),
        "final_generated_c_path": str(out_problem_dir / "final_generated_from_spec.c"),
    }


def write_summary(rows: list[dict[str, str | int]]) -> None:
    summary_csv = OUTPUT_ROOT / "validation_summary.csv"
    with summary_csv.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "source_file",
                "attempts_used",
                "final_status",
                "selected_attempt_number",
                "selected_status",
                "best_passed_samples",
                "best_total_samples",
                "final_spec_path",
                "final_report_path",
                "final_generated_c_path",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    load_dotenv()
    api_key = os.environ.get("OPENAI_API_KEY", "").strip()
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set.")
    if not PROBLEMS_ROOT.is_dir():
        raise FileNotFoundError(f"PROBLEMS_ROOT does not exist or is not a directory: {PROBLEMS_ROOT}")

    client = OpenAI(api_key=api_key)
    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)

    jobs: list[tuple[str, Path, Path]] = []
    scheduled_problems = 0

    for problem_dir in sorted(PROBLEMS_ROOT.iterdir()):
        if not problem_dir.is_dir():
            continue
        if scheduled_problems >= MAX_PROBLEMS_TO_PROCESS:
            break

        code_files = list_code_files(problem_dir)
        if not code_files:
            print(f"[SKIP] {problem_dir.name}: no matching code files")
            continue

        out_problem_dir = OUTPUT_ROOT / problem_dir.name
        if SKIP_IF_OUTPUT_EXISTS and out_problem_dir.exists():
            print(f"[SKIP] {problem_dir.name}: output directory already exists")
            continue

        out_problem_dir.mkdir(parents=True, exist_ok=True)
        scheduled_problems += 1
        for code_file in code_files:
            jobs.append((problem_dir.name, code_file, out_problem_dir))

    results: list[dict[str, str | int]] = []
    failed_jobs = 0
    with ThreadPoolExecutor(max_workers=PARALLEL_REQUESTS) as executor:
        future_to_job = {
            executor.submit(process_code_file, client, problem_name, code_file, out_problem_dir): (
                problem_name,
                code_file,
            )
            for problem_name, code_file, out_problem_dir in jobs
        }

        for future in as_completed(future_to_job):
            problem_name, code_file = future_to_job[future]
            try:
                row = future.result()
                results.append(row)
                print(f"[DONE] {problem_name}/{code_file.name}: {row['final_status']}")
            except Exception as exc:
                failed_jobs += 1
                print(f"[FAILED] {problem_name}/{code_file.name}: {exc}")

    results.sort(key=lambda row: str(row["problem_id"]))
    write_summary(results)

    passed = sum(1 for row in results if row["final_status"] == "PASS")
    print("\nDone.")
    print(f"Problems scheduled: {scheduled_problems}")
    print(f"Files completed: {len(results)}")
    print(f"Files failed with script/API errors: {failed_jobs}")
    print(f"Files with PASS: {passed}")
    print(f"Output root: {OUTPUT_ROOT}")


if __name__ == "__main__":
    main()
