#!/usr/bin/env python3
from __future__ import annotations

import csv
import os
import re
import shutil
import subprocess
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI

try:
    from repairs.initial_translation_profiles import (
        build_initial_translation_prompt,
        get_initial_translation_profile,
        list_profile_code_files,
        list_profile_specs,
    )
except ModuleNotFoundError:
    from initial_translation_profiles import (
        build_initial_translation_prompt,
        get_initial_translation_profile,
        list_profile_code_files,
        list_profile_specs,
    )

PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "full_translation_loop_GPT_5_2"
DATASET_NAME = "optimized_prompt@1"
INITIAL_TRANSLATION_PROFILES = ["specs_only", "code_only", "code_and_spec"]
SPECS_ROOT = PROJECT_ROOT / "api_responses" / "open_ai" / DATASET_NAME / "C" / "specs"
CODE_ROOT = PROJECT_ROOT / "collected_dataset" / "C" / "final_problems"
MAX_PROBLEMS_TO_RUN: int | None = None
MAX_REPAIR_ATTEMPTS = 3
PARALLEL_PROBLEMS = 10
REDO_ONLY_API_ERRORS = False
MODEL = "gpt-5.2"
 #"gpt-4o-mini"
API_TIMEOUT_SECONDS = 180
USE_OPENAI_API = True

SAMPLE_IO_ROOT = PROJECT_ROOT / "collected_dataset/problem_test_inputs_and_outputs"
MAX_FILES_PER_PROBLEM = 1
TARGET_VARIANT_NAME = f"{PIPELINE_NAME}.rs"
COMPILER = "rustc"
COMPILER_FLAGS = ["--edition=2021", "-O"]
EXECUTION_TIMEOUT_SECONDS = 10

SYSTEM_PROMPT = "You are a careful Rust repair assistant."

FOLLOWUP_USER_PROMPT_TEMPLATE = """The previous repair attempt was executed and did not fully pass.

Attempt number: {attempt_number}
Observed status: {status}

Latest execution report:
```text
{report_text}
```

Latest Rust code:
```rust
{code_text}
```

Please produce an improved repaired Rust file. Return only the Rust code in a single ```rust code block.
"""

API_ERROR_PATTERNS = [
    "apierror",
    "apiconnectionerror",
    "apitimeouterror",
    "apistatuserror",
    "internalservererror",
    "ratelimiterror",
    "timeout",
    "timed out",
    "connection",
    "connect",
    "server",
    "service unavailable",
    "bad gateway",
    "gateway",
    "502",
    "503",
    "504",
    "500",
]


@dataclass(frozen=True)
class RepairJob:
    problem_id: str
    spec_path: Path | None
    code_path: Path | None
    output_dir: Path


def get_output_root(profile_name: str) -> Path:
    return PROJECT_ROOT / "repairs" / PIPELINE_NAME / profile_name / DATASET_NAME


def get_report_csv(profile_name: str) -> Path:
    return get_output_root(profile_name) / f"{PIPELINE_NAME}_{profile_name}_report.csv"


def load_existing_report_rows(profile_name: str) -> list[dict[str, str]]:
    report_csv = get_report_csv(profile_name)
    if not report_csv.is_file():
        return []
    with report_csv.open("r", newline="", encoding="utf-8") as csv_file:
        return list(csv.DictReader(csv_file))


def row_has_api_related_error(row: dict[str, str]) -> bool:
    if row.get("result", "").strip() != "FAILED":
        return False

    error_text = row.get("error", "").strip().lower()
    if not error_text:
        return False

    return any(pattern in error_text for pattern in API_ERROR_PATTERNS)


def list_profile_problem_specs(profile_name: str, problem_dir: Path) -> list[Path]:
    profile = get_initial_translation_profile(profile_name)
    return list_profile_specs(profile, problem_dir, MAX_FILES_PER_PROBLEM)


def list_profile_problem_code_files(profile_name: str, problem_dir: Path) -> list[Path]:
    profile = get_initial_translation_profile(profile_name)
    return list_profile_code_files(profile, problem_dir, MAX_FILES_PER_PROBLEM)


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


def compile_rust_program(rust_file_path: Path, executable_path: Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [COMPILER, *COMPILER_FLAGS, str(rust_file_path), "-o", str(executable_path)],
        capture_output=True,
        text=True,
        errors="replace",
        check=False,
    )


def execute_candidate(rust_file_path: Path, problem_id: str, attempt_dir: Path) -> dict[str, str | int]:
    report_path = attempt_dir / "run_report.txt"
    sample_csv_path = attempt_dir / "samples.csv"
    executable_path = attempt_dir / f"{rust_file_path.stem}.out"
    sample_pairs = find_sample_pairs(problem_id)

    if not sample_pairs:
        report_text = "Status: NO_SAMPLES\nNo sample input/output pairs were found.\n"
        report_path.write_text(report_text, encoding="utf-8")
        with sample_csv_path.open("w", newline="", encoding="utf-8") as csv_file:
            writer = csv.DictWriter(
                csv_file,
                fieldnames=["problem_id", "rust_file", "sample_index", "status", "exit_code", "input_file", "output_file"],
            )
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

    compile_result = compile_rust_program(rust_file_path, executable_path)
    report_lines: list[str] = [f"COMPILER: {COMPILER}", f"COMPILER_FLAGS: {' '.join(COMPILER_FLAGS)}", ""]
    sample_rows: list[dict[str, str | int]] = []

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
                    "rust_file": str(rust_file_path),
                    "sample_index": sample_idx,
                    "status": "TIMEOUT",
                    "exit_code": "",
                    "input_file": str(input_path),
                    "output_file": str(output_path),
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
                    "rust_file": str(rust_file_path),
                    "sample_index": sample_idx,
                    "status": "RUNTIME_ERROR",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
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
                    "rust_file": str(rust_file_path),
                    "sample_index": sample_idx,
                    "status": "PASS",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
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
                    "rust_file": str(rust_file_path),
                    "sample_index": sample_idx,
                    "status": "FAIL",
                    "exit_code": run_result.returncode,
                    "input_file": str(input_path),
                    "output_file": str(output_path),
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
        "status": overall_status,
        "report_path": str(report_path),
        "sample_csv_path": str(sample_csv_path),
        "passed": passed,
        "failed": failed,
        "timed_out": timed_out,
        "runtime_errors": runtime_errors,
        "total_samples": len(sample_pairs),
    }


def build_followup_prompt(attempt_number: int, status: str, report_text: str, code_text: str) -> str:
    return FOLLOWUP_USER_PROMPT_TEMPLATE.format(
        attempt_number=attempt_number,
        status=status,
        report_text=report_text.rstrip(),
        code_text=code_text.rstrip(),
    )


def load_jobs(profile_name: str) -> list[RepairJob]:
    output_root = get_output_root(profile_name)
    profile = get_initial_translation_profile(profile_name)
    if profile.needs_spec and not SPECS_ROOT.is_dir():
        raise FileNotFoundError(f"Specs root not found: {SPECS_ROOT}")
    if profile.needs_code and not CODE_ROOT.is_dir():
        raise FileNotFoundError(f"Code root not found: {CODE_ROOT}")

    jobs: list[RepairJob] = []
    selected_problem_ids: set[str] = set()
    rerun_problem_ids: set[str] | None = None

    if REDO_ONLY_API_ERRORS:
        existing_rows = load_existing_report_rows(profile_name)
        rerun_problem_ids = {
            row["problem_id"].strip()
            for row in existing_rows
            if row.get("problem_id", "").strip() and row_has_api_related_error(row)
        }
        print(f"[CONFIG] REDO_ONLY_API_ERRORS={REDO_ONLY_API_ERRORS}")
        print(f"[CONFIG] existing_report_rows={len(existing_rows)}")
        print(f"[CONFIG] api_error_problem_ids={len(rerun_problem_ids)}")

        if existing_rows and not rerun_problem_ids:
            return []

    needs_spec = profile.needs_spec
    root_for_iteration = SPECS_ROOT if needs_spec else CODE_ROOT
    for problem_dir in sorted(root_for_iteration.iterdir()):
        if not problem_dir.is_dir():
            continue
        problem_id = problem_dir.name
        if rerun_problem_ids is not None and problem_id not in rerun_problem_ids:
            continue

        if MAX_PROBLEMS_TO_RUN is not None and problem_id not in selected_problem_ids and len(selected_problem_ids) >= MAX_PROBLEMS_TO_RUN:
            continue

        spec_problem_dir = SPECS_ROOT / problem_id
        code_problem_dir = CODE_ROOT / problem_id
        spec_files = list_profile_problem_specs("specs_only", spec_problem_dir)
        code_files = list_profile_problem_code_files("code_only", code_problem_dir)

        if profile.needs_spec and not spec_files:
            print(f"[SKIP] {problem_id}: no spec file found")
            continue
        if profile.needs_code and not code_files:
            print(f"[SKIP] {problem_id}: no C code file found")
            continue

        selected_problem_ids.add(problem_id)
        num_jobs = max(len(spec_files), len(code_files), 1)
        for index in range(num_jobs):
            spec_path = spec_files[index] if index < len(spec_files) else None
            code_path = code_files[index] if index < len(code_files) else None
            output_dir = output_root / problem_id
            print(f"[JOB] {problem_id}")
            if spec_path is not None:
                print(f"  spec: {spec_path}")
            if code_path is not None:
                print(f"  code: {code_path}")
            jobs.append(
                RepairJob(
                    problem_id=problem_id,
                    spec_path=spec_path,
                    code_path=code_path,
                    output_dir=output_dir,
                )
            )
    return jobs


def call_openai(client: OpenAI, prompt: str, previous_response_id: str | None = None):
    kwargs = {
        "model": MODEL,
        "temperature": 0,
        "input": [
            {"role": "system", "content": [{"type": "input_text", "text": SYSTEM_PROMPT}]},
            {"role": "user", "content": [{"type": "input_text", "text": prompt}]},
        ],
        "timeout": API_TIMEOUT_SECONDS,
    }
    if previous_response_id is not None:
        kwargs["previous_response_id"] = previous_response_id
    response = client.responses.create(**kwargs)
    if response.output_text:
        return response
    raise ValueError("OpenAI response did not include output_text.")


def process_job(client: OpenAI | None, profile_name: str, job: RepairJob) -> dict[str, str | int]:
    if REDO_ONLY_API_ERRORS and job.output_dir.exists():
        shutil.rmtree(job.output_dir)
    job.output_dir.mkdir(parents=True, exist_ok=True)
    previous_response_id: str | None = None
    current_code_text = ""
    current_report_text = ""
    current_status = "INITIAL_TRANSLATION"
    initial_profile = get_initial_translation_profile(profile_name)
    spec_text = job.spec_path.read_text(encoding="utf-8", errors="ignore") if job.spec_path is not None else ""
    code_text = job.code_path.read_text(encoding="utf-8", errors="ignore") if job.code_path is not None else ""

    for attempt_number in range(1, MAX_REPAIR_ATTEMPTS + 1):
        attempt_dir = job.output_dir / f"attempt_{attempt_number}"
        attempt_dir.mkdir(parents=True, exist_ok=True)

        if attempt_number == 1:
            prompt = build_initial_translation_prompt(
                initial_profile,
                job.problem_id,
                spec_text=spec_text,
                code_text=code_text,
            )
        else:
            prompt = build_followup_prompt(attempt_number, current_status, current_report_text, current_code_text)
        prompt_path = attempt_dir / "repair_prompt.txt"
        prompt_path.write_text(prompt, encoding="utf-8")

        if not USE_OPENAI_API:
            return {
                "problem_id": job.problem_id,
                "initial_status": "INITIAL_TRANSLATION",
                "final_status": "PROMPT_ONLY",
                "attempts_used": attempt_number,
                "final_code_path": "",
                "final_report_path": "",
                "result": "PROMPT_ONLY",
                "error": "",
            }

        if client is None:
            raise ValueError("OpenAI client is required when USE_OPENAI_API is True.")

        response = call_openai(client, prompt, previous_response_id=previous_response_id)
        previous_response_id = response.id
        response_text = response.output_text
        response_path = attempt_dir / "repair_response.txt"
        response_path.write_text(response_text.rstrip() + "\n", encoding="utf-8")

        repaired_code = extract_code_from_response(response_text)
        repaired_code_path = attempt_dir / TARGET_VARIANT_NAME
        repaired_code_path.write_text(repaired_code, encoding="utf-8")

        execution_result = execute_candidate(repaired_code_path, job.problem_id, attempt_dir)
        current_status = str(execution_result["status"])
        current_report_text = Path(str(execution_result["report_path"])).read_text(encoding="utf-8", errors="ignore")
        current_code_text = repaired_code

        print(f"[ATTEMPT] {job.problem_id} attempt={attempt_number} status={current_status}")

        if current_status == "PASS":
            final_code_path = job.output_dir / TARGET_VARIANT_NAME
            final_report_path = job.output_dir / "final_run_report.txt"
            final_code_path.write_text(repaired_code, encoding="utf-8")
            final_report_path.write_text(current_report_text, encoding="utf-8")
            return {
                "problem_id": job.problem_id,
                "initial_status": "INITIAL_TRANSLATION",
                "final_status": current_status,
                "attempts_used": attempt_number,
                "final_code_path": str(final_code_path),
                "final_report_path": str(final_report_path),
                "result": "PASS",
                "error": "",
            }

    final_code_path = job.output_dir / TARGET_VARIANT_NAME
    final_report_path = job.output_dir / "final_run_report.txt"
    final_code_path.write_text(current_code_text, encoding="utf-8")
    final_report_path.write_text(current_report_text, encoding="utf-8")
    return {
        "problem_id": job.problem_id,
        "initial_status": "INITIAL_TRANSLATION",
        "final_status": current_status,
        "attempts_used": MAX_REPAIR_ATTEMPTS,
        "final_code_path": str(final_code_path),
        "final_report_path": str(final_report_path),
        "result": "MAX_ATTEMPTS_REACHED",
        "error": "",
    }


def save_report(profile_name: str, rows: list[dict[str, str | int]]) -> None:
    output_root = get_output_root(profile_name)
    report_csv = get_report_csv(profile_name)
    output_root.mkdir(parents=True, exist_ok=True)
    with report_csv.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "initial_status",
                "final_status",
                "attempts_used",
                "final_code_path",
                "final_report_path",
                "result",
                "error",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)


def merge_report_rows(
    existing_rows: list[dict[str, str]],
    new_rows: list[dict[str, str | int]],
) -> list[dict[str, str | int]]:
    merged_by_problem_id: dict[str, dict[str, str | int]] = {}

    for row in existing_rows:
        problem_id = row.get("problem_id", "").strip()
        if problem_id:
            merged_by_problem_id[problem_id] = dict(row)

    for row in new_rows:
        problem_id = str(row["problem_id"]).strip()
        if problem_id:
            merged_by_problem_id[problem_id] = row

    return [merged_by_problem_id[problem_id] for problem_id in sorted(merged_by_problem_id)]


def main() -> None:
    load_dotenv()

    print(f"[CONFIG] DATASET_NAME={DATASET_NAME}")
    print(f"[CONFIG] INITIAL_TRANSLATION_PROFILES={INITIAL_TRANSLATION_PROFILES}")
    print(f"[CONFIG] SPECS_ROOT={SPECS_ROOT}")
    print(f"[CONFIG] CODE_ROOT={CODE_ROOT}")
    print(f"[CONFIG] MAX_PROBLEMS_TO_RUN={MAX_PROBLEMS_TO_RUN}")
    print(f"[CONFIG] MAX_REPAIR_ATTEMPTS={MAX_REPAIR_ATTEMPTS}")
    print(f"[CONFIG] PARALLEL_PROBLEMS={PARALLEL_PROBLEMS}")
    print(f"[CONFIG] REDO_ONLY_API_ERRORS={REDO_ONLY_API_ERRORS}")
    print(f"[CONFIG] USE_OPENAI_API={USE_OPENAI_API}")
    print(f"[CONFIG] TARGET_VARIANT_NAME={TARGET_VARIANT_NAME}")

    if not INITIAL_TRANSLATION_PROFILES:
        raise ValueError("INITIAL_TRANSLATION_PROFILES is empty.")

    client: OpenAI | None = None
    if USE_OPENAI_API:
        api_key = os.environ.get("OPENAI_API_KEY", "").strip()
        if not api_key:
            raise EnvironmentError("OPENAI_API_KEY is not set.")
        client = OpenAI(api_key=api_key)

    for profile_name in INITIAL_TRANSLATION_PROFILES:
        output_root = get_output_root(profile_name)
        report_csv = get_report_csv(profile_name)
        existing_rows = load_existing_report_rows(profile_name)

        print(f"\n[PROFILE] {profile_name}")
        print(f"[CONFIG] OUTPUT_ROOT={output_root}")

        jobs = load_jobs(profile_name)
        if not jobs:
            print(f"[SKIP] no jobs found for profile: {profile_name}")
            continue

        results: list[dict[str, str | int]] = []
        with ThreadPoolExecutor(max_workers=PARALLEL_PROBLEMS) as executor:
            future_to_job = {executor.submit(process_job, client, profile_name, job): job for job in jobs}
            for future in as_completed(future_to_job):
                job = future_to_job[future]
                try:
                    row = future.result()
                    results.append(row)
                    print(f"[{row['result']}] {profile_name}/{job.problem_id}")
                except Exception as exc:
                    results.append(
                        {
                            "problem_id": job.problem_id,
                            "initial_status": "INITIAL_TRANSLATION",
                            "final_status": "ERROR",
                            "attempts_used": 0,
                            "final_code_path": "",
                            "final_report_path": "",
                            "result": "FAILED",
                            "error": repr(exc),
                        }
                    )
                    print(f"[FAILED] {profile_name}/{job.problem_id}: {exc}")

        results.sort(key=lambda row: str(row["problem_id"]))
        merged_results = merge_report_rows(existing_rows, results) if REDO_ONLY_API_ERRORS else results
        save_report(profile_name, merged_results)

        passed = sum(1 for row in merged_results if row["result"] == "PASS")
        prompt_only = sum(1 for row in merged_results if row["result"] == "PROMPT_ONLY")
        maxed = sum(1 for row in merged_results if row["result"] == "MAX_ATTEMPTS_REACHED")
        failed = sum(1 for row in merged_results if row["result"] == "FAILED")

        print(f"[SUMMARY] profile={profile_name}")
        print(f"Jobs run this pass: {len(results)}")
        print(f"Rows in merged report: {len(merged_results)}")
        print(f"Passed after repair: {passed}")
        print(f"Prompt only: {prompt_only}")
        print(f"Max attempts reached: {maxed}")
        print(f"Failed: {failed}")
        print(f"Dataset: {DATASET_NAME}")
        print(f"Output root: {output_root}")
        print(f"Report CSV: {report_csv}")


if __name__ == "__main__":
    main()
