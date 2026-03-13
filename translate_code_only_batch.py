#!/usr/bin/env python3
"""
Submit batch translation requests to OpenAI using only source code, then save the
translated target-language files and a CSV report.

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
PROBLEMS_ROOT =  Path("/Users/lithe/Documents/research/code_translation_specs/codenet_statistics/collected_dataset/C/final_problems") 
OUTPUT_ROOT = Path("./api_responses/open_ai/C/translations_with_code_only")
MODEL = "gpt-4o-mini"
SOURCE_LANGUAGE = "C"
TARGET_LANGUAGE = "Rust"
SOURCE_FILE_SUFFIX = ".c"
TARGET_FILE_SUFFIX = ".rs"
MAX_PROBLEMS_TO_PROCESS = 2
MAX_FILES_PER_PROBLEM = 1
API_TIMEOUT_SECONDS = 120
USE_BATCH_API = False
DIRECT_REQUEST_DELAY_SECONDS = 10
BATCH_POLL_INTERVAL_SECONDS = 30
WAIT_FOR_BATCH_COMPLETION = True
REPORT_CSV = OUTPUT_ROOT / "translation_report.csv"
BATCH_INPUT_JSONL = OUTPUT_ROOT / "translation_batch_input.jsonl"
BATCH_OUTPUT_JSONL = OUTPUT_ROOT / "translation_batch_output.jsonl"
BATCH_ERROR_JSONL = OUTPUT_ROOT / "translation_batch_error.jsonl"
BATCH_METADATA_NAME = "code_only_translation_batch"

SYSTEM_PROMPT = "You are a helpful coding assistant."

USER_PROMPT_TEMPLATE = """You are a senior systems engineer fluent in C and Rust, with strong expertise in program semantics and safe systems programming.

Task:
Translate the given C code into Rust.

Primary goal:
Preserve semantics based ONLY on the C code provided. Do not assume any external specification.

Instructions:
1) Treat the C code as the only source of truth for behavior.
2) Preserve I/O behavior exactly (stdin/stdout formatting, whitespace, newlines).
3) Preserve corner-case behavior implied by the C code. For example, if the C code relies on:
   - integer overflow / wraparound,
   - bit-level tricks,
   - undefined/implementation-defined behavior,
   then emulate the intended behavior as closely as possible in Rust.
4) Prefer safe, idiomatic Rust:
   - Avoid `unsafe` unless necessary; if used, justify it in a short comment.

5) You may add comments as needed to explain non-obvious translation choices, especially if they relate to preserving semantics or handling corner cases or just as a human developer would to clarify intent.
6) Output requirements:
   - Return ONLY the Rust code in a .rs file format code block.

Now translate the following.

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


def list_code_files(problem_dir: Path) -> list[Path]:
    direct_files = sorted(
        p for p in problem_dir.glob(f"*{SOURCE_FILE_SUFFIX}") if p.is_file()
    )
    if direct_files:
        return direct_files[:MAX_FILES_PER_PROBLEM]

    language_dir = problem_dir / SOURCE_LANGUAGE
    if language_dir.is_dir():
        nested_files = sorted(
            p for p in language_dir.glob(f"*{SOURCE_FILE_SUFFIX}") if p.is_file()
        )
        return nested_files[:MAX_FILES_PER_PROBLEM]

    return []


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


def build_request_body(code_text: str) -> dict[str, object]:
    prompt = USER_PROMPT_TEMPLATE.format(code=code_text)
    return {
        "model": MODEL,
        "temperature": 0,
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

    for problem_dir in sorted(PROBLEMS_ROOT.iterdir()):
        if not problem_dir.is_dir():
            continue
        if processed_problems >= MAX_PROBLEMS_TO_PROCESS:
            break

        out_problem_dir = OUTPUT_ROOT / problem_dir.name
        if out_problem_dir.exists():
            print(f"[SKIP] {problem_dir.name}: output directory already exists")
            continue

        code_files = list_code_files(problem_dir)
        if not code_files:
            continue

        processed_problems += 1
        out_problem_dir.mkdir(parents=True, exist_ok=True)

        for code_file in code_files:
            base_stem = code_file.stem
            out_path = out_problem_dir / f"{base_stem}{TARGET_FILE_SUFFIX}"
            row = {
                "problem_id": problem_dir.name,
                "file_stem": base_stem,
                "status": "PENDING",
                "code_file": str(code_file),
                "output_file": str(out_path),
                "error": "",
                "custom_id": f"{problem_dir.name}::{base_stem}",
                "batch_id": "",
            }

            code_text = code_file.read_text(encoding="utf-8", errors="ignore")
            batch_requests.append(
                {
                    "custom_id": row["custom_id"],
                    "method": "POST",
                    "url": "/v1/responses",
                    "body": build_request_body(code_text),
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

    for row in report_rows:
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

    for row in report_rows:
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
    print(f"Files failed: {failed_files}")
    print(f"Output root: {OUTPUT_ROOT}")
    print(f"Report CSV: {REPORT_CSV}")


def main() -> None:
    load_dotenv()
    api_key = os.environ.get("OPENAI_API_KEY", "").strip()
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set.")
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
