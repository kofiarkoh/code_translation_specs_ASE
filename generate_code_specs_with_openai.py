#!/usr/bin/env python3
"""
Loop through problem folders, read code files, send each file to the OpenAI API,
and save the generated specification text.

Expected input shape:
PROBLEMS_ROOT/
  p00001/
    file1.c
    file2.c
  p00002/
    ...

Or:
PROBLEMS_ROOT/
  p00001/
    C/
      file1.c
  p00002/
    C/
      file2.c
"""

from __future__ import annotations

import os
import time
from pathlib import Path

from openai import OpenAI
from dotenv import load_dotenv


# =========================
# Configure these values
# =========================
PROBLEMS_ROOT = Path("/Users/lithe/Documents/research/code_translation_specs/codenet_statistics/collected_dataset/C/final_problems")
OUTPUT_ROOT = Path("./api_responses/open_ai/C/specs")
MODEL = "gpt-4o-mini"
FILE_GLOB = "*.c"
MAX_PROBLEMS_TO_PROCESS = 140
MAX_FILES_PER_PROBLEM = 1
SOURCE_LANGUAGE = "C"
API_TIMEOUT_SECONDS = 120
REQUEST_DELAY_SECONDS = 10.0

SYSTEM_PROMPT = "You are a helpful coding assistant."

USER_PROMPT_TEMPLATE = """ 
You are an expert senior software engineer.
Given source code, generate a precise, structured behavioral specification that states WHAT the code guarantees, not HOW it achieves it.

Output must use exactly these sections and headings:
1. Functional Description
2. Inputs
3. Outputs
4. Preconditions
5. Postconditions
6. Invariants
7. Error Handling / Exceptional Behavior
8. State Transitions (if applicable)
9. Non-functional Constraints (if inferable)

Content requirements:
- Be concise and contract-focused; omit incidental or low-level details.
- Do not reference language/library APIs, format strings, or identifiers (e.g., no mentions of scanf, printf, fgets, variable or function names, loop types, or recursion).
- Describe I/O abstractly (e.g., "reads values from standard input" or "returns a value"); never cite exact parsing patterns or format specifiers.
- Do not restate implementation structure or control flow. Focus on observable behavior, data constraints, and guarantees.
- If behavior is ambiguous, state explicit assumptions. If something cannot be deduced, write "Not inferable from code."
- Avoid extreme assumptions. Capture only behavior implied by the code’s observable contract.
- If the program is an executable, treat printing/writing as side effects in Outputs.
- No code, no pseudocode, no preamble or epilogue text. Use only the nine sections above. 
- Return only the nine sections with the exact headings and numbering specified. Do not include any other text, examples, or code.
Now generate the specification for the code below:
```{language}
{code}
```
"""


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

    output_text = response.output_text
    if output_text:
        return output_text

    raise ValueError("OpenAI response did not include output_text.")


def main() -> None:
    load_dotenv()
    api_key = os.environ.get("OPENAI_API_KEY", "").strip()
    if not api_key:
        raise EnvironmentError("OPENAI_API_KEY is not set.")
    if not PROBLEMS_ROOT.is_dir():
        raise FileNotFoundError(f"PROBLEMS_ROOT does not exist or is not a directory: {PROBLEMS_ROOT}")

    client = OpenAI(api_key=api_key)
    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)

    processed_problems = 0
    processed_files = 0
    failed_files = 0

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
            print(f"[SKIP] {problem_dir.name}: no matching code files")
            continue

        processed_problems += 1
        out_problem_dir.mkdir(parents=True, exist_ok=True)

        for code_file in code_files:
            code_text = code_file.read_text(encoding="utf-8", errors="ignore")
            prompt = USER_PROMPT_TEMPLATE.format(language=SOURCE_LANGUAGE, code=code_text)
            out_spec_path = out_problem_dir / f"{code_file.stem}_spec.txt"

            try:
                spec_text = call_openai(client, prompt)
                out_spec_path.write_text(spec_text.rstrip() + "\n", encoding="utf-8")
                processed_files += 1
                print(f"[DONE] {problem_dir.name}/{code_file.name}")
            except Exception as exc:
                failed_files += 1
                out_spec_path.write_text(
                    f"Status: FAILED\nError: {repr(exc)}\n",
                    encoding="utf-8",
                )
                print(f"[FAILED] {problem_dir.name}/{code_file.name}: {exc}")

            if REQUEST_DELAY_SECONDS > 0:
                time.sleep(REQUEST_DELAY_SECONDS)

    print("\nDone.")
    print(f"Problems processed: {processed_problems}")
    print(f"Files processed successfully: {processed_files}")
    print(f"Files failed: {failed_files}")
    print(f"Output root: {OUTPUT_ROOT}")


if __name__ == "__main__":
    main()
