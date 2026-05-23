#!/usr/bin/env python3
"""
Backfill missing *_prompt.txt files for flat spec directories where the
original run did not save the input prompt.

For each problem in SPEC_DIR:
  - finds the *_spec.txt output file
  - derives the source C filename from its stem  (e.g. s002492070_spec.txt -> s002492070.c)
  - reads the C source from C_SOURCE_ROOT/<problem_id>/<stem>.c
  - applies the same USER_PROMPT_TEMPLATE used by generate_code_specs_with_openai.py
  - writes the result as <stem>_prompt.txt alongside the spec file

Run:
    python generate_missing_spec_prompts.py
"""

from __future__ import annotations

from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent

# Directory whose *_spec.txt files need matching *_prompt.txt files
SPEC_DIR = PROJECT_ROOT / "api_responses" / "open_ai" / "optimized_prompt@1" / "C" / "specs"

# Where the original C source files live
C_SOURCE_ROOT = PROJECT_ROOT / "collected_dataset" / "C" / "final_problems"

SKIP_IF_EXISTS = False

# Copied verbatim from generate_code_specs_with_openai.py
SYSTEM_PROMPT = "You are a helpful coding assistant."

USER_PROMPT_TEMPLATE = """You are an expert senior software engineer.
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
- Do not reference language/library APIs, format strings, or identifiers.
- Describe I/O abstractly; never cite exact parsing patterns or format specifiers.
- Do not restate implementation structure or control flow.
- If behavior is ambiguous, state explicit assumptions.
- Avoid extreme assumptions. Capture only behavior implied by the code's observable contract.
- Return only the nine sections with the exact headings and numbering specified.

Now generate the specification for the code below:

```{language}
{code}
```"""


def build_prompt(c_code: str) -> str:
    return USER_PROMPT_TEMPLATE.format(language="C", code=c_code.rstrip())


def main() -> None:
    if not SPEC_DIR.is_dir():
        raise FileNotFoundError(f"SPEC_DIR not found: {SPEC_DIR}")

    written = 0
    skipped = 0
    missing_source = 0

    for problem_dir in sorted(SPEC_DIR.iterdir()):
        if not problem_dir.is_dir() or not problem_dir.name.startswith("p"):
            continue

        for spec_file in sorted(problem_dir.glob("*_spec.txt")):
            stem = spec_file.name[: -len("_spec.txt")]
            prompt_file = problem_dir / f"{stem}_prompt.txt"

            if SKIP_IF_EXISTS and prompt_file.exists():
                skipped += 1
                continue

            c_file = C_SOURCE_ROOT / problem_dir.name / f"{stem}.c"
            if not c_file.is_file():
                print(f"[MISSING SOURCE] {problem_dir.name}/{stem}.c")
                missing_source += 1
                continue

            c_code = c_file.read_text(encoding="utf-8", errors="ignore")
            prompt_text = build_prompt(c_code)
            prompt_file.write_text(prompt_text, encoding="utf-8")
            print(f"[WRITTEN] {problem_dir.name}/{prompt_file.name}")
            written += 1

    print(f"\nDone. Written: {written}  Skipped: {skipped}  Missing source: {missing_source}")


if __name__ == "__main__":
    main()
