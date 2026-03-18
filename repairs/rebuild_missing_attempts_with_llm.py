#!/usr/bin/env python3
from __future__ import annotations

import os
import re
import shutil
from dataclasses import dataclass
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI

try:
    from repairs.full_translation_loop import (
        API_TIMEOUT_SECONDS,
        CODE_ROOT,
        DATASET_NAME,
        MAX_FILES_PER_PROBLEM,
        MAX_REPAIR_ATTEMPTS,
        MODEL,
        PIPELINE_NAME,
        SPECS_ROOT,
        TARGET_VARIANT_NAME,
        build_followup_prompt,
        build_initial_translation_prompt,
        call_openai,
        execute_candidate,
        get_initial_translation_profile,
        get_report_csv,
        list_profile_problem_code_files,
        list_profile_problem_specs,
        load_existing_report_rows,
        merge_report_rows,
        save_report,
    )
except ModuleNotFoundError:
    from full_translation_loop import (
        API_TIMEOUT_SECONDS,
        CODE_ROOT,
        DATASET_NAME,
        MAX_FILES_PER_PROBLEM,
        MAX_REPAIR_ATTEMPTS,
        MODEL,
        PIPELINE_NAME,
        SPECS_ROOT,
        TARGET_VARIANT_NAME,
        build_followup_prompt,
        build_initial_translation_prompt,
        call_openai,
        execute_candidate,
        get_initial_translation_profile,
        get_report_csv,
        list_profile_problem_code_files,
        list_profile_problem_specs,
        load_existing_report_rows,
        merge_report_rows,
        save_report,
    )


PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_ROOT = PROJECT_ROOT / "repairs" / PIPELINE_NAME
TARGET_PROFILES: list[str] | None = None
MAX_PROBLEMS_TO_RUN: int | None = None


@dataclass(frozen=True)
class RecoveryJob:
    profile_name: str
    problem_id: str
    output_dir: Path
    missing_attempt_numbers: list[int]
    spec_path: Path | None
    code_path: Path | None


def get_profile_names() -> list[str]:
    if TARGET_PROFILES is not None:
        return TARGET_PROFILES
    return sorted(path.name for path in PIPELINE_ROOT.iterdir() if path.is_dir())


def get_output_root(profile_name: str) -> Path:
    return PIPELINE_ROOT / profile_name / DATASET_NAME


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


def build_job(profile_name: str, problem_dir: Path) -> RecoveryJob | None:
    missing_attempt_numbers: list[int] = []
    for attempt_dir in sorted(path for path in problem_dir.iterdir() if path.is_dir() and path.name.startswith("attempt_")):
        report_path = attempt_dir / "run_report.txt"
        if report_path.is_file():
            continue
        try:
            attempt_number = int(attempt_dir.name.removeprefix("attempt_"))
        except ValueError:
            continue
        missing_attempt_numbers.append(attempt_number)

    if not missing_attempt_numbers:
        return None

    spec_problem_dir = SPECS_ROOT / problem_dir.name
    code_problem_dir = CODE_ROOT / problem_dir.name
    spec_files = list_profile_problem_specs("specs_only", spec_problem_dir)
    code_files = list_profile_problem_code_files("code_only", code_problem_dir)
    spec_path = spec_files[0] if spec_files else None
    code_path = code_files[0] if code_files else None

    profile = get_initial_translation_profile(profile_name)
    if profile.needs_spec and spec_path is None:
        print(f"[SKIP] {profile_name}/{problem_dir.name}: no spec file found")
        return None
    if profile.needs_code and code_path is None:
        print(f"[SKIP] {profile_name}/{problem_dir.name}: no C code file found")
        return None

    return RecoveryJob(
        profile_name=profile_name,
        problem_id=problem_dir.name,
        output_dir=problem_dir,
        missing_attempt_numbers=missing_attempt_numbers,
        spec_path=spec_path,
        code_path=code_path,
    )


def load_jobs() -> list[RecoveryJob]:
    jobs: list[RecoveryJob] = []
    selected_problem_ids: set[tuple[str, str]] = set()

    for profile_name in get_profile_names():
        output_root = get_output_root(profile_name)
        if not output_root.is_dir():
            print(f"[SKIP] output root not found: {output_root}")
            continue

        for problem_dir in sorted(path for path in output_root.iterdir() if path.is_dir() and path.name.startswith("p")):
            problem_key = (profile_name, problem_dir.name)
            if MAX_PROBLEMS_TO_RUN is not None and problem_key not in selected_problem_ids and len(selected_problem_ids) >= MAX_PROBLEMS_TO_RUN:
                continue

            job = build_job(profile_name, problem_dir)
            if job is None:
                continue

            selected_problem_ids.add(problem_key)
            jobs.append(job)

    return jobs


def load_attempt_state(output_dir: Path, attempt_number: int) -> tuple[str, str, str]:
    attempt_dir = output_dir / f"attempt_{attempt_number}"
    report_path = attempt_dir / "run_report.txt"
    code_path = attempt_dir / TARGET_VARIANT_NAME
    status = parse_status_from_report(report_path)
    report_text = report_path.read_text(encoding="utf-8", errors="ignore")
    code_text = code_path.read_text(encoding="utf-8", errors="ignore")
    return status, report_text, code_text


def rebuild_problem_report_row(profile_name: str, problem_id: str) -> dict[str, str | int] | None:
    problem_dir = get_output_root(profile_name) / problem_id
    if not problem_dir.is_dir():
        return None

    attempt_dirs = sorted(path for path in problem_dir.iterdir() if path.is_dir() and path.name.startswith("attempt_"))
    completed_attempts = [path for path in attempt_dirs if (path / "run_report.txt").is_file()]
    if not completed_attempts:
        return {
            "problem_id": problem_id,
            "initial_status": "INITIAL_TRANSLATION",
            "final_status": "ERROR",
            "attempts_used": 0,
            "final_code_path": "",
            "final_report_path": "",
            "result": "FAILED",
            "error": "Missing run_report.txt for all attempts.",
        }

    final_attempt_dir = max(completed_attempts, key=lambda path: int(path.name.removeprefix("attempt_")))
    final_status = parse_status_from_report(final_attempt_dir / "run_report.txt")
    attempts_used = int(final_attempt_dir.name.removeprefix("attempt_"))

    final_code_source = final_attempt_dir / TARGET_VARIANT_NAME
    final_report_source = final_attempt_dir / "run_report.txt"
    final_code_path = problem_dir / TARGET_VARIANT_NAME
    final_report_path = problem_dir / "final_run_report.txt"

    if final_code_source.is_file():
        shutil.copy2(final_code_source, final_code_path)
    if final_report_source.is_file():
        shutil.copy2(final_report_source, final_report_path)

    result = "PASS" if final_status == "PASS" else "MAX_ATTEMPTS_REACHED"
    return {
        "problem_id": problem_id,
        "initial_status": "INITIAL_TRANSLATION",
        "final_status": final_status,
        "attempts_used": attempts_used,
        "final_code_path": str(final_code_path) if final_code_path.is_file() else "",
        "final_report_path": str(final_report_path) if final_report_path.is_file() else "",
        "result": result,
        "error": "",
    }


def recover_job(client: OpenAI, job: RecoveryJob) -> dict[str, str | int]:
    profile = get_initial_translation_profile(job.profile_name)
    spec_text = job.spec_path.read_text(encoding="utf-8", errors="ignore") if job.spec_path is not None else ""
    code_text = job.code_path.read_text(encoding="utf-8", errors="ignore") if job.code_path is not None else ""
    previous_response_id: str | None = None

    for attempt_number in sorted(job.missing_attempt_numbers):
        attempt_dir = job.output_dir / f"attempt_{attempt_number}"
        if attempt_dir.exists():
            shutil.rmtree(attempt_dir)
        attempt_dir.mkdir(parents=True, exist_ok=True)

        if attempt_number == 1:
            prompt = build_initial_translation_prompt(profile, job.problem_id, spec_text=spec_text, code_text=code_text)
        else:
            previous_attempt_number = attempt_number - 1
            previous_status, previous_report_text, previous_code_text = load_attempt_state(job.output_dir, previous_attempt_number)
            prompt = build_followup_prompt(attempt_number, previous_status, previous_report_text, previous_code_text)

        prompt_path = attempt_dir / "repair_prompt.txt"
        prompt_path.write_text(prompt, encoding="utf-8")

        response = call_openai(client, prompt, previous_response_id=previous_response_id)
        previous_response_id = response.id
        response_text = response.output_text
        response_path = attempt_dir / "repair_response.txt"
        response_path.write_text(response_text.rstrip() + "\n", encoding="utf-8")

        repaired_code = extract_code_from_response(response_text)
        repaired_code_path = attempt_dir / TARGET_VARIANT_NAME
        repaired_code_path.write_text(repaired_code, encoding="utf-8")
        execution_result = execute_candidate(repaired_code_path, job.problem_id, attempt_dir)
        print(
            f"[ATTEMPT] {job.profile_name}/{job.problem_id} "
            f"attempt={attempt_number} status={execution_result['status']}"
        )

    rebuilt_row = rebuild_problem_report_row(job.profile_name, job.problem_id)
    if rebuilt_row is None:
        return {
            "problem_id": job.problem_id,
            "initial_status": "INITIAL_TRANSLATION",
            "final_status": "ERROR",
            "attempts_used": 0,
            "final_code_path": "",
            "final_report_path": "",
            "result": "FAILED",
            "error": "Could not rebuild report row after recovery.",
        }
    return rebuilt_row


def main() -> None:
    load_dotenv()

    api_key = os.environ.get("OPENAI_API_KEY", "").strip()
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set.")

    if API_TIMEOUT_SECONDS <= 0:
        raise ValueError(f"Invalid API timeout: {API_TIMEOUT_SECONDS}")

    print(f"PIPELINE_ROOT: {PIPELINE_ROOT}")
    print(f"DATASET_NAME: {DATASET_NAME}")
    print(f"TARGET_PROFILES: {TARGET_PROFILES}")
    print(f"MAX_PROBLEMS_TO_RUN: {MAX_PROBLEMS_TO_RUN}")
    print(f"MODEL: {MODEL}")
    print(f"MAX_REPAIR_ATTEMPTS: {MAX_REPAIR_ATTEMPTS}")

    jobs = load_jobs()
    if not jobs:
        print("No missing-attempt jobs found.")
        return

    client = OpenAI(api_key=api_key)
    rows_by_profile: dict[str, list[dict[str, str | int]]] = {}

    for job in jobs:
        print(f"[JOB] {job.profile_name}/{job.problem_id} missing_attempts={job.missing_attempt_numbers}")
        row = recover_job(client, job)
        rows_by_profile.setdefault(job.profile_name, []).append(row)

    for profile_name, new_rows in sorted(rows_by_profile.items()):
        existing_rows = load_existing_report_rows(profile_name)
        merged_rows = merge_report_rows(existing_rows, new_rows)
        save_report(profile_name, merged_rows)
        print(f"[OK] updated report CSV for {profile_name}: {len(new_rows)} recovered problems")
        print(f"[OK] report path: {get_report_csv(profile_name)}")


if __name__ == "__main__":
    main()
