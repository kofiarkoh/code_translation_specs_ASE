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
MODEL = "gpt-5-mini"
FILE_GLOB = "*.c"
MAX_PROBLEMS_TO_PROCESS = 140
MAX_FILES_PER_PROBLEM = 1
SOURCE_LANGUAGE = "C"
API_TIMEOUT_SECONDS = 120
REQUEST_DELAY_SECONDS = 10.0

SYSTEM_PROMPT = "You are a helpful coding assistant."

USER_PROMPT_TEMPLATE = """You are an expert senior software engineer.

Given the following source code, generate a precise and structured code specification.

The goal is to describe WHAT the code does (behavioral contract), not HOW it is implemented.

Return the specification using the following structured sections:

1. Functional Description
   - High-level description of the purpose
   - What problem it solves
   - Core behavior

2. Inputs
   - Parameters
   - Data types
   - Valid ranges or constraints
   - Assumptions about inputs

3. Outputs
   - Return values
   - Side effects
   - Output format/schema (if applicable)

4. Preconditions
   - Conditions that must hold before execution
   - Explicit logical constraints

5. Postconditions
   - Conditions guaranteed after execution
   - State changes
   - Return guarantees

6. Invariants
   - Conditions that must always remain true
   - Class-level invariants (if object-oriented)

7. Error Handling / Exceptional Behavior
   - What happens on invalid input?
   - Exceptions thrown?
   - Edge-case behavior

8. State Transitions (if applicable)
   - Valid object states
   - State changes caused by execution

9. Non-functional Constraints (if inferable)
   - Time complexity
   - Space complexity
   - Performance assumptions

Important constraints:
- Do NOT rewrite the code.
- Do NOT summarize implementation details.
- Focus strictly on behavioral specification.
- If behavior is ambiguous, state assumptions explicitly.
- If certain aspects cannot be inferred, state "Not inferable from code."
- Do not make any extreme assumptions.
- Your output should not be too wordy and must focus on core aspects

Now generate the specification for the code below:

```{language}
{code}
```"""


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
