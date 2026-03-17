#!/usr/bin/env python3
from __future__ import annotations

import csv
import os
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt@2"
MAX_PROBLEMS_TO_RUN: int | None = 3
TARGET_STATUSES: list[str] | None = ["COMPILE_FAILED", "RUNTIME_ERROR", "TIMEOUT", "FAIL"]
MAX_REPAIR_ATTEMPTS = 3
MODEL = "gpt-4o-mini"
API_TIMEOUT_SECONDS = 180
USE_OPENAI_API = True

SAMPLE_IO_ROOT = PROJECT_ROOT / "collected_dataset/problem_test_inputs_and_outputs"
COMPILER = "rustc"
COMPILER_FLAGS = ["--edition=2021", "-O"]
EXECUTION_TIMEOUT_SECONDS = 10

SYSTEM_PROMPT = "You are a careful Rust repair assistant."

INITIAL_TRANSLATION_USER_PROMPT_TEMPLATE = """You are a senior systems engineer fluent in C and Rust, with strong expertise in program semantics and safe systems programming.

Task:
Translate the given Specification into Rust code.

Primary goal:
Preserve semantics based ONLY on the Specification provided. Do not assume any external code or source.

Instructions:
1) Treat the Specification as the only source of truth for behavior.
2) Preserve observable I/O behavior described in the Specification.
3) Preserve corner-case behavior implied by the Specification. For example, if the Specification relies on:
   - integer overflow / wraparound,
   - bit-level tricks,
   - undefined/implementation-defined behavior,
   then emulate the intended behavior as closely as possible in Rust.
4) If the Specification is incomplete, ambiguous, or inconsistent:
   - do not invent extra functionality
   - choose the narrowest reasonable interpretation justified by the Specification
   - produce compilable Rust that stays as faithful as possible to the stated behavior
5) Prefer safe, idiomatic Rust:
   - Avoid `unsafe` unless necessary; if used, justify it in a short comment.
6) You may add comments as needed to explain non-obvious translation choices, especially if they relate to preserving semantics or handling corner cases.
7) Any explanation must appear only as valid Rust comments inside the code and not after the code.
8) Return ONLY the Rust code in a single ```rust code block.

Problem ID: {problem_id}
Variant target: {variant}

Specification:
```text
{spec_text}
```
"""

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


@dataclass(frozen=True)
class RepairJob:
    problem_id: str
    variant: str
    status: str
    spec_path: Path
    initial_report_path: Path
    initial_code_path: Path
    output_dir: Path


def get_previous_dataset_name() -> str | None:
    match = re.fullmatch(r"(.+?)@(\d+)", DATASET_NAME)
    if not match:
        return None
    base_name, suffix_text = match.groups()
    suffix = int(suffix_text)
    if suffix <= 1:
        return None
    return f"{base_name}@{suffix - 1}"


def get_input_dataset_name() -> str:
    previous_dataset_name = get_previous_dataset_name()
    return previous_dataset_name if previous_dataset_name is not None else DATASET_NAME


def get_input_csv() -> Path:
    return PROJECT_ROOT / "results" / get_input_dataset_name() / "translated_code_execution_statuses_output" / "non_pass_problems.csv"


def get_reports_root() -> Path:
    return PROJECT_ROOT / "results" / get_input_dataset_name() / "rust_run_reports"


def get_specs_root() -> Path:
    return PROJECT_ROOT / "api_responses" / "open_ai" / get_input_dataset_name() / "C" / "specs"


def get_base_dataset_name() -> str:
    match = re.fullmatch(r"(.+?)@(\d+)", DATASET_NAME)
    if not match:
        return DATASET_NAME
    return match.group(1)


def get_source_root() -> Path:
    previous_dataset_name = get_previous_dataset_name()
    if previous_dataset_name is not None:
        return PROJECT_ROOT / "repairs" / "generated_repairs" / previous_dataset_name
    return PROJECT_ROOT / "api_responses" / "open_ai" / get_base_dataset_name() / "C"


def get_output_root() -> Path:
    return PROJECT_ROOT / "repairs" / "iterative_generated_repairs" / DATASET_NAME


def get_report_csv() -> Path:
    return get_output_root() / "iterative_repair_report.csv"


def variant_dir_name(variant: str) -> str:
    return Path(variant).stem


def resolve_report_path(problem_id: str, variant: str) -> Path:
    return get_reports_root() / problem_id / f"{variant_dir_name(variant)}_report.txt"


def resolve_spec_path(problem_id: str) -> Path:
    spec_dir = get_specs_root() / problem_id
    matches = sorted(spec_dir.glob("*_spec.txt"))
    if not matches:
        raise FileNotFoundError(f"No spec file found in {spec_dir}")
    if len(matches) > 1:
        raise ValueError(f"Expected one spec file in {spec_dir}, found {len(matches)}")
    return matches[0]


def resolve_code_path(problem_id: str, variant: str) -> Path:
    source_root = get_source_root()
    candidate_dirs = [
        source_root / variant_dir_name(variant) / problem_id,
        source_root / problem_id / variant_dir_name(variant),
    ]
    for candidate_dir in candidate_dirs:
        matches = sorted(candidate_dir.glob("*.rs"))
        if not matches:
            continue
        if len(matches) > 1:
            raise ValueError(f"Expected one Rust source in {candidate_dir}, found {len(matches)}")
        return matches[0]
    searched = ", ".join(str(path) for path in candidate_dirs)
    raise FileNotFoundError(f"No Rust source found in any of: {searched}")


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


def build_initial_translation_prompt(job: RepairJob, spec_text: str) -> str:
    return INITIAL_TRANSLATION_USER_PROMPT_TEMPLATE.format(
        problem_id=job.problem_id,
        variant=job.variant,
        spec_text=spec_text.rstrip(),
    )


def build_followup_prompt(attempt_number: int, status: str, report_text: str, code_text: str) -> str:
    return FOLLOWUP_USER_PROMPT_TEMPLATE.format(
        attempt_number=attempt_number,
        status=status,
        report_text=report_text.rstrip(),
        code_text=code_text.rstrip(),
    )


def load_jobs() -> list[RepairJob]:
    input_csv = get_input_csv()
    output_root = get_output_root()
    if not input_csv.is_file():
        raise FileNotFoundError(f"Input CSV not found: {input_csv}")

    jobs: list[RepairJob] = []
    seen: set[tuple[str, str]] = set()
    selected_problem_ids: set[str] = set()

    with input_csv.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            problem_id = row.get("problem_id", "").strip()
            variant = row.get("variant", "").strip()
            status = row.get("status", "").strip()
            if not problem_id or not variant:
                continue
            if TARGET_STATUSES is not None and status not in TARGET_STATUSES:
                continue

            key = (problem_id, variant)
            if key in seen:
                continue
            seen.add(key)

            if MAX_PROBLEMS_TO_RUN is not None and problem_id not in selected_problem_ids and len(selected_problem_ids) >= MAX_PROBLEMS_TO_RUN:
                continue

            try:
                spec_path = resolve_spec_path(problem_id)
                code_path = resolve_code_path(problem_id, variant)
            except FileNotFoundError as exc:
                print(f"[SKIP] {problem_id}/{variant}: {exc}")
                continue

            report_path = resolve_report_path(problem_id, variant)
            output_dir = output_root / problem_id / variant_dir_name(variant)
            selected_problem_ids.add(problem_id)
            print(f"[JOB] {problem_id}/{variant}")
            print(f"  spec: {spec_path}")
            print(f"  report: {report_path}")
            print(f"  code: {code_path}")
            jobs.append(
                RepairJob(
                    problem_id=problem_id,
                    variant=variant,
                    status=status,
                    spec_path=spec_path,
                    initial_report_path=report_path,
                    initial_code_path=code_path,
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


def process_job(client: OpenAI | None, job: RepairJob) -> dict[str, str | int]:
    job.output_dir.mkdir(parents=True, exist_ok=True)
    previous_response_id: str | None = None
    current_code_text = job.initial_code_path.read_text(encoding="utf-8", errors="ignore")
    current_report_text = job.initial_report_path.read_text(encoding="utf-8", errors="ignore")
    current_status = job.status
    spec_text = job.spec_path.read_text(encoding="utf-8", errors="ignore")

    for attempt_number in range(1, MAX_REPAIR_ATTEMPTS + 1):
        attempt_dir = job.output_dir / f"attempt_{attempt_number}"
        attempt_dir.mkdir(parents=True, exist_ok=True)

        prompt = (
            build_initial_translation_prompt(job, spec_text)
            if attempt_number == 1
            else build_followup_prompt(attempt_number, current_status, current_report_text, current_code_text)
        )
        prompt_path = attempt_dir / "repair_prompt.txt"
        prompt_path.write_text(prompt, encoding="utf-8")

        if not USE_OPENAI_API:
            return {
                "problem_id": job.problem_id,
                "variant": job.variant,
                "initial_status": job.status,
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
        repaired_code_path = attempt_dir / job.variant
        repaired_code_path.write_text(repaired_code, encoding="utf-8")

        execution_result = execute_candidate(repaired_code_path, job.problem_id, attempt_dir)
        current_status = str(execution_result["status"])
        current_report_text = Path(str(execution_result["report_path"])).read_text(encoding="utf-8", errors="ignore")
        current_code_text = repaired_code

        print(f"[ATTEMPT] {job.problem_id}/{job.variant} attempt={attempt_number} status={current_status}")

        if current_status == "PASS":
            final_code_path = job.output_dir / job.variant
            final_report_path = job.output_dir / "final_run_report.txt"
            final_code_path.write_text(repaired_code, encoding="utf-8")
            final_report_path.write_text(current_report_text, encoding="utf-8")
            return {
                "problem_id": job.problem_id,
                "variant": job.variant,
                "initial_status": job.status,
                "final_status": current_status,
                "attempts_used": attempt_number,
                "final_code_path": str(final_code_path),
                "final_report_path": str(final_report_path),
                "result": "PASS",
                "error": "",
            }

    final_code_path = job.output_dir / job.variant
    final_report_path = job.output_dir / "final_run_report.txt"
    final_code_path.write_text(current_code_text, encoding="utf-8")
    final_report_path.write_text(current_report_text, encoding="utf-8")
    return {
        "problem_id": job.problem_id,
        "variant": job.variant,
        "initial_status": job.status,
        "final_status": current_status,
        "attempts_used": MAX_REPAIR_ATTEMPTS,
        "final_code_path": str(final_code_path),
        "final_report_path": str(final_report_path),
        "result": "MAX_ATTEMPTS_REACHED",
        "error": "",
    }


def save_report(rows: list[dict[str, str | int]]) -> None:
    output_root = get_output_root()
    report_csv = get_report_csv()
    output_root.mkdir(parents=True, exist_ok=True)
    with report_csv.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "variant",
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


def main() -> None:
    load_dotenv()
    input_csv = get_input_csv()
    reports_root = get_reports_root()
    source_root = get_source_root()
    output_root = get_output_root()
    report_csv = get_report_csv()

    print(f"[CONFIG] DATASET_NAME={DATASET_NAME}")
    print(f"[CONFIG] MAX_PROBLEMS_TO_RUN={MAX_PROBLEMS_TO_RUN}")
    print(f"[CONFIG] TARGET_STATUSES={TARGET_STATUSES}")
    print(f"[CONFIG] MAX_REPAIR_ATTEMPTS={MAX_REPAIR_ATTEMPTS}")
    print(f"[CONFIG] USE_OPENAI_API={USE_OPENAI_API}")
    print(f"[CONFIG] INPUT_CSV={input_csv}")
    print(f"[CONFIG] REPORTS_ROOT={reports_root}")
    print(f"[CONFIG] SOURCE_ROOT={source_root}")
    print(f"[CONFIG] OUTPUT_ROOT={output_root}")

    jobs = load_jobs()
    if not jobs:
        raise ValueError(f"No repair jobs found in {input_csv}")

    client: OpenAI | None = None
    if USE_OPENAI_API:
        api_key = os.environ.get("OPENAI_API_KEY", "").strip()
        if not api_key:
            raise EnvironmentError("OPENAI_API_KEY is not set.")
        client = OpenAI(api_key=api_key)

    results: list[dict[str, str | int]] = []
    for job in jobs:
        try:
            row = process_job(client, job)
            results.append(row)
            print(f"[{row['result']}] {job.problem_id}/{job.variant}")
        except Exception as exc:
            results.append(
                {
                    "problem_id": job.problem_id,
                    "variant": job.variant,
                    "initial_status": job.status,
                    "final_status": "ERROR",
                    "attempts_used": 0,
                    "final_code_path": "",
                    "final_report_path": "",
                    "result": "FAILED",
                    "error": repr(exc),
                }
            )
            print(f"[FAILED] {job.problem_id}/{job.variant}: {exc}")

    results.sort(key=lambda row: (str(row["problem_id"]), str(row["variant"])))
    save_report(results)

    passed = sum(1 for row in results if row["result"] == "PASS")
    prompt_only = sum(1 for row in results if row["result"] == "PROMPT_ONLY")
    maxed = sum(1 for row in results if row["result"] == "MAX_ATTEMPTS_REACHED")
    failed = sum(1 for row in results if row["result"] == "FAILED")

    print("\nDone.")
    print(f"Jobs found: {len(results)}")
    print(f"Passed after repair: {passed}")
    print(f"Prompt only: {prompt_only}")
    print(f"Max attempts reached: {maxed}")
    print(f"Failed: {failed}")
    print(f"Dataset: {DATASET_NAME}")
    print(f"Output root: {output_root}")
    print(f"Report CSV: {report_csv}")


if __name__ == "__main__":
    main()
