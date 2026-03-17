#!/usr/bin/env python3
from __future__ import annotations

import csv
import os
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt"
MAX_PROBLEMS_TO_PROCESS: int | None = None
MODEL = "gpt-4o-mini"
API_TIMEOUT_SECONDS = 180
PARALLEL_REQUESTS = 20
SKIP_EXISTING_OUTPUTS = False

SYSTEM_PROMPT = "You are a careful Rust repair assistant."

USER_PROMPT_TEMPLATE = """You are an expert Rust engineer helping repair a generated Rust solution.

Task:
Fix the Rust code so it better matches the intended behavior, based on the execution report.

Instructions:
- Use the execution report to identify likely causes of failure.
- Preserve the program's intended behavior as much as possible.
- Keep the solution in Rust.
- Return only the repaired Rust code in a single ```rust code block.
- Do not include explanations outside the code block.

Problem ID: {problem_id}
Variant: {variant}
Observed status: {status}

Execution report:
```text
{report_text}
```

Current Rust code:
```rust
{code_text}
```
"""


@dataclass(frozen=True)
class RepairJob:
    problem_id: str
    variant: str
    status: str
    report_path: Path
    code_path: Path
    output_dir: Path
    prompt_path: Path
    response_path: Path
    repaired_code_path: Path


def get_input_csv() -> Path:
    return PROJECT_ROOT / "results" / DATASET_NAME / "translated_code_execution_statuses_output" / "non_pass_problems.csv"


def get_reports_root() -> Path:
    return PROJECT_ROOT / "results" / DATASET_NAME / "rust_run_reports"


def get_source_root() -> Path:
    return PROJECT_ROOT / "api_responses" / "open_ai" / DATASET_NAME / "C"


def get_output_root() -> Path:
    return PROJECT_ROOT / "repairs" / "generated_repairs" / DATASET_NAME


def get_report_csv() -> Path:
    return get_output_root() / "repair_report.csv"


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


def build_prompt(problem_id: str, variant: str, status: str, report_text: str, code_text: str) -> str:
    return USER_PROMPT_TEMPLATE.format(
        problem_id=problem_id,
        variant=variant,
        status=status,
        report_text=report_text.rstrip(),
        code_text=code_text.rstrip(),
    )


def variant_dir_name(variant: str) -> str:
    return Path(variant).stem


def resolve_report_path(problem_id: str, variant: str) -> Path:
    return get_reports_root() / problem_id / f"{variant_dir_name(variant)}_report.txt"


def resolve_code_path(problem_id: str, variant: str) -> Path:
    variant_dir = get_source_root() / variant_dir_name(variant) / problem_id
    matches = sorted(variant_dir.glob("*.rs"))
    if not matches:
        raise FileNotFoundError(f"No Rust source found in {variant_dir}")
    if len(matches) > 1:
        raise ValueError(f"Expected one Rust source in {variant_dir}, found {len(matches)}")
    return matches[0]


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

            key = (problem_id, variant)
            if key in seen:
                continue
            seen.add(key)

            if MAX_PROBLEMS_TO_PROCESS is not None:
                if problem_id not in selected_problem_ids and len(selected_problem_ids) >= MAX_PROBLEMS_TO_PROCESS:
                    continue
                selected_problem_ids.add(problem_id)

            report_path = resolve_report_path(problem_id, variant)
            code_path = resolve_code_path(problem_id, variant)
            output_dir = output_root / problem_id / variant_dir_name(variant)
            jobs.append(
                RepairJob(
                    problem_id=problem_id,
                    variant=variant,
                    status=status,
                    report_path=report_path,
                    code_path=code_path,
                    output_dir=output_dir,
                    prompt_path=output_dir / "repair_prompt.txt",
                    response_path=output_dir / "repair_response.txt",
                    repaired_code_path=output_dir / variant,
                )
            )

    return jobs


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


def process_job(client: OpenAI, job: RepairJob) -> dict[str, str]:
    job.output_dir.mkdir(parents=True, exist_ok=True)

    if SKIP_EXISTING_OUTPUTS and job.repaired_code_path.is_file():
        return {
            "problem_id": job.problem_id,
            "variant": job.variant,
            "status": job.status,
            "code_path": str(job.code_path),
            "report_path": str(job.report_path),
            "output_path": str(job.repaired_code_path),
            "result": "SKIPPED",
            "error": "",
        }

    report_text = job.report_path.read_text(encoding="utf-8", errors="ignore")
    code_text = job.code_path.read_text(encoding="utf-8", errors="ignore")
    prompt = build_prompt(job.problem_id, job.variant, job.status, report_text, code_text)
    job.prompt_path.write_text(prompt, encoding="utf-8")

    response_text = call_openai(client, prompt)
    repaired_code = extract_code_from_response(response_text)

    job.response_path.write_text(response_text.rstrip() + "\n", encoding="utf-8")
    job.repaired_code_path.write_text(repaired_code, encoding="utf-8")

    return {
        "problem_id": job.problem_id,
        "variant": job.variant,
        "status": job.status,
        "code_path": str(job.code_path),
        "report_path": str(job.report_path),
        "output_path": str(job.repaired_code_path),
        "result": "REPAIRED",
        "error": "",
    }


def save_report(rows: list[dict[str, str]]) -> None:
    output_root = get_output_root()
    report_csv = get_report_csv()
    output_root.mkdir(parents=True, exist_ok=True)
    with report_csv.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "variant",
                "status",
                "code_path",
                "report_path",
                "output_path",
                "result",
                "error",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)


def main() -> None:
    load_dotenv()
    api_key = os.environ.get("OPENAI_API_KEY", "").strip()
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set.")

    input_csv = get_input_csv()
    output_root = get_output_root()
    report_csv = get_report_csv()
    jobs = load_jobs()
    if not jobs:
        raise ValueError(f"No repair jobs found in {input_csv}")

    client = OpenAI(api_key=api_key)
    results: list[dict[str, str]] = []

    with ThreadPoolExecutor(max_workers=PARALLEL_REQUESTS) as executor:
        future_to_job = {executor.submit(process_job, client, job): job for job in jobs}
        for future in as_completed(future_to_job):
            job = future_to_job[future]
            try:
                row = future.result()
                results.append(row)
                print(f"[{row['result']}] {job.problem_id}/{job.variant}")
            except Exception as exc:
                results.append(
                    {
                        "problem_id": job.problem_id,
                        "variant": job.variant,
                        "status": job.status,
                        "code_path": str(job.code_path),
                        "report_path": str(job.report_path),
                        "output_path": str(job.repaired_code_path),
                        "result": "FAILED",
                        "error": repr(exc),
                    }
                )
                print(f"[FAILED] {job.problem_id}/{job.variant}: {exc}")

    results.sort(key=lambda row: (row["problem_id"], row["variant"]))
    save_report(results)

    repaired = sum(1 for row in results if row["result"] == "REPAIRED")
    skipped = sum(1 for row in results if row["result"] == "SKIPPED")
    failed = sum(1 for row in results if row["result"] == "FAILED")

    print("\nDone.")
    print(f"Jobs found: {len(results)}")
    print(f"Repaired: {repaired}")
    print(f"Skipped: {skipped}")
    print(f"Failed: {failed}")
    print(f"Dataset: {DATASET_NAME}")
    print(f"Output root: {output_root}")
    print(f"Report CSV: {report_csv}")


if __name__ == "__main__":
    main()
