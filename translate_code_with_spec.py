#!/usr/bin/env python3
"""
Read saved specification text files and original source code files, submit
translation requests through the OpenAI Batch API, and save translated code in
the target language.

Expected spec output shape:
SPECS_ROOT/
  p00001/
    s123_spec.txt

Expected code input shape:
PROBLEMS_ROOT/
  p00001/
    s123.c

Or:
PROBLEMS_ROOT/
  p00001/
    C/
      s123.c
"""

from __future__ import annotations

import csv
import json
import os
import time
from pathlib import Path

from dotenv import load_dotenv
from openai import OpenAI


# =========================
# Configure these values
# =========================
SPECS_ROOT = Path("/Users/lithe/Documents/research/code_translation_specs/codenet_statistics/api_responses/open_ai/C/specs")
PROBLEMS_ROOT = Path("/Users/lithe/Documents/research/code_translation_specs/codenet_statistics/collected_dataset/C/final_problems")
OUTPUT_ROOT = Path("./api_responses/open_ai/C/translations_with_specs_and_code")
MODEL = "gpt-5-mini"
SOURCE_LANGUAGE = "C"
TARGET_LANGUAGE = "Rust"
SOURCE_FILE_SUFFIX = ".c"
TARGET_FILE_SUFFIX = ".rs"
MAX_PROBLEMS_TO_PROCESS = 140
MAX_FILES_PER_PROBLEM = 1
API_TIMEOUT_SECONDS = 120
USE_BATCH_API = False
DIRECT_REQUEST_DELAY_SECONDS = 10
BATCH_POLL_INTERVAL_SECONDS = 30
WAIT_FOR_BATCH_COMPLETION = False
REPORT_CSV = OUTPUT_ROOT / "translation_report.csv"
BATCH_INPUT_JSONL = OUTPUT_ROOT / "translation_batch_input.jsonl"
BATCH_OUTPUT_JSONL = OUTPUT_ROOT / "translation_batch_output.jsonl"
BATCH_ERROR_JSONL = OUTPUT_ROOT / "translation_batch_error.jsonl"
BATCH_METADATA_NAME = "spec_translation_batch"

SYSTEM_PROMPT = "You are a helpful coding assistant."

USER_PROMPT_TEMPLATE = """You are a senior systems engineer fluent in C and Rust, with strong expertise in program semantics and specification-driven translation.

Task:
Translate the given C code into Rust, using the provided code and specification as the source-of-truth for intended behavior.

Primary goal:
Preserve semantics. The Rust code must satisfy the specification (functional behavior, preconditions, postconditions, invariants, and error behavior).

Instructions:
1) Read the SPEC first and treat it as the definitive description of expected behavior.
2) Use the C code only to resolve ambiguities and edge cases and do not implement any non implemented function.
3) If the C code conflicts with the spec, follow the spec and explicitly note the mismatch.
4) Prefer safe, idiomatic Rust:
   - Strongly Avoid `unsafe` unless necessary; if used, justify it in a short comment. Prioritise converting unsafe code to safe code.

5) Preserve I/O behavior exactly (stdin/stdout formatting, whitespace, newlines).
6) Avoid unsafe unless absolutely necessary.
7) You may add comments as needed to explain non-obvious translation choices, especially if they relate to preserving semantics or handling corner cases or just as a human developer would to clarify intent.
8) Output requirements:
   - Return ONLY the Rust code in a .rs file format code block.

Now translate the following code using the specification.

=== SPECIFICATION START ===
{spec}
=== SPECIFICATION END ===

=== C CODE START ===
{code}
=== C CODE END ===
"""

TERMINAL_BATCH_STATUSES = {
    "completed",
    "failed",
    "expired",
    "cancelled",
}


def list_spec_files(problem_dir: Path) -> list[Path]:
    return sorted(p for p in problem_dir.glob("*_spec.txt") if p.is_file())[:MAX_FILES_PER_PROBLEM]


def find_code_file(problem_id: str, stem: str) -> Path | None:
    problem_dir = PROBLEMS_ROOT / problem_id
    direct_path = problem_dir / f"{stem}{SOURCE_FILE_SUFFIX}"
    if direct_path.is_file():
        return direct_path

    language_path = problem_dir / SOURCE_LANGUAGE / f"{stem}{SOURCE_FILE_SUFFIX}"
    if language_path.is_file():
        return language_path

    return None


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


def build_request_body(spec_text: str, code_text: str) -> dict[str, object]:
    prompt = USER_PROMPT_TEMPLATE.format(spec=spec_text, code=code_text)
    return {
        "model": MODEL,
        "input": [
            {"role": "system", "content": [{"type": "input_text", "text": SYSTEM_PROMPT}]},
            {"role": "user", "content": [{"type": "input_text", "text": prompt}]},
        ],
    }


def call_openai(client: OpenAI, request_body: dict[str, object]) -> str:
    response = client.responses.create(timeout=API_TIMEOUT_SECONDS, **request_body)
    if response.output_text:
        return response.output_text
    raise ValueError("OpenAI response did not include output_text.")


def build_manifest() -> tuple[list[dict[str, str]], list[dict[str, object]]]:
    manifest_rows: list[dict[str, str]] = []
    batch_requests: list[dict[str, object]] = []
    processed_problems = 0

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

        for spec_file in spec_files:
            base_stem = spec_file.stem.removesuffix("_spec")
            code_file = find_code_file(spec_problem_dir.name, base_stem)
            out_problem_dir.mkdir(parents=True, exist_ok=True)
            out_path = out_problem_dir / f"{base_stem}{TARGET_FILE_SUFFIX}"

            row = {
                "problem_id": spec_problem_dir.name,
                "file_stem": base_stem,
                "status": "PENDING",
                "spec_file": str(spec_file),
                "code_file": str(code_file) if code_file else "",
                "output_file": str(out_path),
                "error": "",
                "custom_id": f"{spec_problem_dir.name}::{base_stem}",
                "batch_id": "",
            }

            if code_file is None:
                row["status"] = "SKIPPED"
                row["error"] = "Missing source code file"
                out_path.write_text("Missing source code file.\n", encoding="utf-8")
                manifest_rows.append(row)
                print(f"[SKIP] {spec_problem_dir.name}/{spec_file.name}: missing code file")
                continue

            spec_text = spec_file.read_text(encoding="utf-8", errors="ignore")
            code_text = code_file.read_text(encoding="utf-8", errors="ignore")
            batch_requests.append(
                {
                    "custom_id": row["custom_id"],
                    "method": "POST",
                    "url": "/v1/responses",
                    "body": build_request_body(spec_text, code_text),
                }
            )
            manifest_rows.append(row)

    return manifest_rows, batch_requests


def write_batch_input_jsonl(batch_requests: list[dict[str, object]]) -> None:
    with BATCH_INPUT_JSONL.open("w", encoding="utf-8") as handle:
        for request in batch_requests:
            handle.write(json.dumps(request, ensure_ascii=False) + "\n")


def save_report(report_rows: list[dict[str, str]]) -> None:
    with REPORT_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "file_stem",
                "status",
                "spec_file",
                "code_file",
                "output_file",
                "error",
                "custom_id",
                "batch_id",
            ],
        )
        writer.writeheader()
        writer.writerows(report_rows)


def wait_for_batch(client: OpenAI, batch_id: str):
    while True:
        batch = client.batches.retrieve(batch_id)
        print(f"[BATCH] {batch.id}: {batch.status}")
        if batch.status in TERMINAL_BATCH_STATUSES:
            return batch
        time.sleep(BATCH_POLL_INTERVAL_SECONDS)


def parse_output_text(response_body: dict[str, object]) -> str:
    output_text = response_body.get("output_text")
    if isinstance(output_text, str) and output_text:
        return output_text
    raise ValueError(f"Batch response missing output_text: {response_body}")


def load_jsonl_by_custom_id(path: Path) -> dict[str, dict[str, object]]:
    records: dict[str, dict[str, object]] = {}
    if not path.is_file():
        return records

    with path.open("r", encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if not line:
                continue
            payload = json.loads(line)
            custom_id = str(payload.get("custom_id", ""))
            if custom_id:
                records[custom_id] = payload
    return records


def run_direct(client: OpenAI, report_rows: list[dict[str, str]], batch_requests: list[dict[str, object]]) -> None:
    requests_by_custom_id = {
        str(request["custom_id"]): request for request in batch_requests
    }

    translated_files = 0
    failed_files = 0
    skipped_files = 0

    for row in report_rows:
        if row["status"] == "SKIPPED":
            skipped_files += 1
            continue

        custom_id = row["custom_id"]
        request = requests_by_custom_id.get(custom_id)
        output_file = Path(row["output_file"])
        if request is None:
            row["status"] = "FAILED"
            row["error"] = "Missing request body for direct request"
            output_file.write_text(f"Translation failed: {row['error']}\n", encoding="utf-8")
            failed_files += 1
            continue

        try:
            translated = call_openai(client, request["body"])
            rust_code = extract_code_from_response(translated)
            output_file.write_text(rust_code, encoding="utf-8")
            row["status"] = "SUCCESS"
            row["error"] = ""
            translated_files += 1
            print(f"[DONE] {row['problem_id']}/{row['file_stem']}")
        except Exception as exc:
            row["status"] = "FAILED"
            row["error"] = repr(exc)
            output_file.write_text(f"Translation failed: {row['error']}\n", encoding="utf-8")
            failed_files += 1
            print(f"[FAILED] {row['problem_id']}/{row['file_stem']}: {exc}")

        if DIRECT_REQUEST_DELAY_SECONDS > 0:
            time.sleep(DIRECT_REQUEST_DELAY_SECONDS)

    save_report(report_rows)

    print("\nDone.")
    print("Mode: direct")
    print(f"Files translated successfully: {translated_files}")
    print(f"Files skipped: {skipped_files}")
    print(f"Files failed: {failed_files}")
    print(f"Output root: {OUTPUT_ROOT}")
    print(f"Report CSV: {REPORT_CSV}")


def run_batch(client: OpenAI, report_rows: list[dict[str, str]], batch_requests: list[dict[str, object]]) -> None:
    write_batch_input_jsonl(batch_requests)

    with BATCH_INPUT_JSONL.open("rb") as handle:
        input_file = client.files.create(file=handle, purpose="batch")

    batch = client.batches.create(
        input_file_id=input_file.id,
        endpoint="/v1/responses",
        completion_window="24h",
        metadata={"name": BATCH_METADATA_NAME},
        timeout=API_TIMEOUT_SECONDS,
    )

    for row in report_rows:
        if row["status"] == "PENDING":
            row["batch_id"] = batch.id

    save_report(report_rows)
    print(f"[BATCH CREATED] {batch.id}")
    print(f"Batch input JSONL: {BATCH_INPUT_JSONL}")

    if not WAIT_FOR_BATCH_COMPLETION:
        print(f"Report CSV: {REPORT_CSV}")
        return

    batch = wait_for_batch(client, batch.id)

    if batch.output_file_id:
        output_content = client.files.retrieve_content(batch.output_file_id)
        BATCH_OUTPUT_JSONL.write_text(output_content, encoding="utf-8")

    if batch.error_file_id:
        error_content = client.files.retrieve_content(batch.error_file_id)
        BATCH_ERROR_JSONL.write_text(error_content, encoding="utf-8")

    output_records = load_jsonl_by_custom_id(BATCH_OUTPUT_JSONL)
    error_records = load_jsonl_by_custom_id(BATCH_ERROR_JSONL)

    translated_files = 0
    failed_files = 0
    skipped_files = 0

    for row in report_rows:
        if row["status"] == "SKIPPED":
            skipped_files += 1
            continue

        custom_id = row["custom_id"]
        output_file = Path(row["output_file"])

        if custom_id in output_records:
            payload = output_records[custom_id]
            response = payload.get("response", {})
            body = response.get("body", {}) if isinstance(response, dict) else {}
            translated = parse_output_text(body if isinstance(body, dict) else {})
            rust_code = extract_code_from_response(translated)
            output_file.write_text(rust_code, encoding="utf-8")
            row["status"] = "SUCCESS"
            row["error"] = ""
            translated_files += 1
            print(f"[DONE] {row['problem_id']}/{row['file_stem']}")
            continue

        if custom_id in error_records:
            payload = error_records[custom_id]
            row["status"] = "FAILED"
            row["error"] = json.dumps(payload, ensure_ascii=False)
            output_file.write_text(f"Translation failed: {row['error']}\n", encoding="utf-8")
            failed_files += 1
            print(f"[FAILED] {row['problem_id']}/{row['file_stem']}: batch error")
            continue

        row["status"] = "FAILED"
        row["error"] = f"Batch finished with status {batch.status}, but no output/error record was found."
        output_file.write_text(f"Translation failed: {row['error']}\n", encoding="utf-8")
        failed_files += 1
        print(f"[FAILED] {row['problem_id']}/{row['file_stem']}: missing batch result")

    save_report(report_rows)

    print("\nDone.")
    print("Mode: batch")
    print(f"Batch ID: {batch.id}")
    print(f"Batch status: {batch.status}")
    print(f"Files translated successfully: {translated_files}")
    print(f"Files skipped: {skipped_files}")
    print(f"Files failed: {failed_files}")
    print(f"Output root: {OUTPUT_ROOT}")
    print(f"Report CSV: {REPORT_CSV}")


def main() -> None:
    load_dotenv()
    api_key = os.environ.get("OPENAI_API_KEY", "").strip()
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set.")
    if not SPECS_ROOT.is_dir():
        raise FileNotFoundError(f"SPECS_ROOT does not exist or is not a directory: {SPECS_ROOT}")
    if not PROBLEMS_ROOT.is_dir():
        raise FileNotFoundError(f"PROBLEMS_ROOT does not exist or is not a directory: {PROBLEMS_ROOT}")

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    client = OpenAI(api_key=api_key)

    report_rows, batch_requests = build_manifest()
    if not batch_requests:
        save_report(report_rows)
        print("No requests to submit.")
        print(f"Report CSV: {REPORT_CSV}")
        return

    if USE_BATCH_API:
        run_batch(client, report_rows, batch_requests)
    else:
        run_direct(client, report_rows, batch_requests)


if __name__ == "__main__":
    main()
