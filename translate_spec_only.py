#!/usr/bin/env python3
"""
Translate saved specification text files into Rust code using the OpenAI API.

Expected spec input shape:
SPECS_ROOT/
  p00001/
    s123_spec.txt
  p00002/
    ...
"""

from __future__ import annotations

import csv
import os
from concurrent.futures import ThreadPoolExecutor, as_completed
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI


# =========================
# Configure these values
# =========================
SPECS_ROOT = Path("./api_responses/open_ai/C/specs")
OUTPUT_ROOT = Path("./api_responses/open_ai/C/translations_with_specs_only")
MODEL = "gpt-4o-mini"
TARGET_FILE_SUFFIX = ".rs"
MAX_PROBLEMS_TO_PROCESS = 300
MAX_FILES_PER_PROBLEM = 1
API_TIMEOUT_SECONDS = 120
PARALLEL_REQUESTS = 20
REPORT_CSV = OUTPUT_ROOT / "translation_report.csv"

SYSTEM_PROMPT = "You are a helpful coding assistant."

USER_PROMPT_TEMPLATE = """You are a senior systems engineer fluent in C and Rust, with strong expertise in program semantics and safe systems programming.

Task:
Translate the given Specification into Rust code.

Primary goal:
Preserve semantics based ONLY on the Specification provided. Do not assume any external code or source.

Instructions:
1) Treat the Specification as the only source of truth for behavior.
2) Preserve observable I/O behavior described in the Specification
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

5) You may add comments as needed to explain non-obvious translation choices, especially if they relate to preserving semantics or handling corner cases or just as a human developer would to clarify intent.
6) Any explanation must appear only as valid Rust comments inside the code and not after the code.
7) Output requirements:
   - Return ONLY the Rust code in a .rs file format code block.


Now translate the following.

=== SEPCIFICATION START ===
{spec}
=== SPECIFICATION END ===
"""


def list_spec_files(problem_dir: Path) -> list[Path]:
    return sorted(p for p in problem_dir.glob("*_spec.txt") if p.is_file())[:MAX_FILES_PER_PROBLEM]


def extract_code_from_response(text: str) -> str:
    stripped = text.strip()
    if stripped.startswith("```"):
        lines = stripped.splitlines()
        if lines:
            lines = lines[1:]
        if lines and lines[-1].strip() == "```":
            lines = lines[:-1]
        return "\n".join(lines).strip() + "\n"
    return stripped + "\n"


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


def process_spec_file(
    client: OpenAI,
    problem_id: str,
    base_stem: str,
    spec_file: Path,
    out_path: Path,
) -> dict[str, str]:
    spec_text = spec_file.read_text(encoding="utf-8", errors="ignore")
    prompt = USER_PROMPT_TEMPLATE.format(spec=spec_text)

    try:
        translated = call_openai(client, prompt)
        rust_code = extract_code_from_response(translated)
        out_path.write_text(rust_code, encoding="utf-8")
        return {
            "problem_id": problem_id,
            "file_stem": base_stem,
            "status": "SUCCESS",
            "spec_file": str(spec_file),
            "output_file": str(out_path),
            "error": "",
        }
    except Exception as exc:
        out_path.write_text(f"Translation failed: {repr(exc)}\n", encoding="utf-8")
        return {
            "problem_id": problem_id,
            "file_stem": base_stem,
            "status": "FAILED",
            "spec_file": str(spec_file),
            "output_file": str(out_path),
            "error": repr(exc),
        }


def main() -> None:
    load_dotenv()
    api_key = os.environ.get("OPENAI_API_KEY", "").strip()
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set.")
    if not SPECS_ROOT.is_dir():
        raise FileNotFoundError(f"SPECS_ROOT does not exist or is not a directory: {SPECS_ROOT}")

    client = OpenAI(api_key=api_key)
    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)

    processed_problems = 0
    translated_files = 0
    failed_files = 0
    report_rows: list[dict[str, str]] = []
    jobs: list[tuple[str, str, Path, Path]] = []

    for spec_problem_dir in sorted(SPECS_ROOT.iterdir()):
        if not spec_problem_dir.is_dir():
            continue
        if processed_problems >= MAX_PROBLEMS_TO_PROCESS:
            break

        out_problem_dir = OUTPUT_ROOT / spec_problem_dir.name
        if out_problem_dir.exists():
            print(f"[SKIP] {spec_problem_dir.name}: output directory already exists")
            continue

        spec_files = list_spec_files(spec_problem_dir)
        if not spec_files:
            continue

        processed_problems += 1
        out_problem_dir.mkdir(parents=True, exist_ok=True)

        for spec_file in spec_files:
            base_stem = spec_file.stem.removesuffix("_spec")
            out_path = out_problem_dir / f"{base_stem}{TARGET_FILE_SUFFIX}"
            jobs.append((spec_problem_dir.name, base_stem, spec_file, out_path))

    with ThreadPoolExecutor(max_workers=PARALLEL_REQUESTS) as executor:
        future_to_job = {
            executor.submit(process_spec_file, client, problem_id, base_stem, spec_file, out_path): (
                problem_id,
                base_stem,
            )
            for problem_id, base_stem, spec_file, out_path in jobs
        }

        for future in as_completed(future_to_job):
            problem_id, base_stem = future_to_job[future]
            try:
                row = future.result()
            except Exception as exc:
                failed_files += 1
                print(f"[FAILED] {problem_id}/{base_stem}: {exc}")
                continue

            report_rows.append(row)
            if row["status"] == "SUCCESS":
                translated_files += 1
                print(f"[DONE] {problem_id}/{base_stem}")
            else:
                failed_files += 1
                print(f"[FAILED] {problem_id}/{base_stem}: {row['error']}")

    with REPORT_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "file_stem",
                "status",
                "spec_file",
                "output_file",
                "error",
            ],
        )
        writer.writeheader()
        writer.writerows(report_rows)

    print("\nDone.")
    print(f"Problems processed: {processed_problems}")
    print(f"Files translated successfully: {translated_files}")
    print(f"Files failed: {failed_files}")
    print(f"Output root: {OUTPUT_ROOT}")
    print(f"Report CSV: {REPORT_CSV}")


if __name__ == "__main__":
    main()
