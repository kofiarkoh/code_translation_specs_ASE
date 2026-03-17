#!/usr/bin/env python3
"""
Extract sample input/output blocks from plain-text *_test.txt files.

Expected input shape:
TEXT_ROOT/
  p00001/
    anything_test.txt
  p00002/
    something_else_test.txt

For each extracted pair, this script writes:
  OUTPUT_ROOT/<problem_id>/<problem_id>_sample<N>_input.txt
  OUTPUT_ROOT/<problem_id>/<problem_id>_sample<N>_output.txt
"""

from __future__ import annotations

import re
from pathlib import Path


# =========================
# Configure these values
# =========================
TEXT_ROOT = Path("/Users/lithe/Downloads/icse26-main1/spectra/data/c_spec_store")
OUTPUT_ROOT = Path("./collected_dataset/problem_test_inputs_and_outputs")
RECURSIVE = False


INPUT_HEADING_PATTERN = re.compile(r"^\s*Input(?:\s+\d+)?\s*:\s*$", flags=re.IGNORECASE)
OUTPUT_HEADING_PATTERN = re.compile(r"^\s*Output(?:\s+\d+)?\s*:\s*$", flags=re.IGNORECASE)


def normalize_block(lines: list[str]) -> str:
    while lines and not lines[0].strip():
        lines.pop(0)
    while lines and not lines[-1].strip():
        lines.pop()
    return "\n".join(lines) + ("\n" if lines else "")


def extract_samples_from_text(content: str) -> list[tuple[str, str]]:
    samples: list[tuple[str, str]] = []
    lines = content.replace("\r\n", "\n").replace("\r", "\n").split("\n")

    state: str | None = None
    current_input: list[str] = []
    current_output: list[str] = []

    for line in lines:
        if INPUT_HEADING_PATTERN.match(line):
            if current_input or current_output:
                input_text = normalize_block(current_input)
                output_text = normalize_block(current_output)
                if input_text and output_text:
                    samples.append((input_text, output_text))
            state = "input"
            current_input = []
            current_output = []
            continue

        if OUTPUT_HEADING_PATTERN.match(line):
            state = "output"
            continue

        if state == "input":
            current_input.append(line)
        elif state == "output":
            current_output.append(line)

    input_text = normalize_block(current_input)
    output_text = normalize_block(current_output)
    if input_text and output_text:
        samples.append((input_text, output_text))

    return samples


def iter_test_files(problem_dir: Path) -> list[Path]:
    pattern = "**/*_test.txt" if RECURSIVE else "*_test.txt"
    return sorted(path for path in problem_dir.glob(pattern) if path.is_file())


def main() -> None:
    if not TEXT_ROOT.is_dir():
        raise FileNotFoundError(f"TEXT_ROOT does not exist or is not a directory: {TEXT_ROOT}")

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)

    scanned_problem_dirs = 0
    scanned_test_files = 0
    files_with_pairs = 0
    wrote_pairs = 0

    for problem_dir in sorted(TEXT_ROOT.iterdir()):
        if not problem_dir.is_dir():
            continue

        scanned_problem_dirs += 1
        problem_id = problem_dir.name
        test_files = iter_test_files(problem_dir)

        if not test_files:
            print(f"[SKIP] {problem_id}: no *_test.txt files")
            continue

        out_problem_dir = OUTPUT_ROOT / problem_id
        out_problem_dir.mkdir(parents=True, exist_ok=True)

        next_sample_idx = 1
        local_pairs = 0

        for test_file in test_files:
            scanned_test_files += 1
            content = test_file.read_text(encoding="utf-8", errors="ignore")
            samples = extract_samples_from_text(content)

            for input_text, output_text in samples:
                input_path = out_problem_dir / f"{problem_id}_sample{next_sample_idx}_input.txt"
                output_path = out_problem_dir / f"{problem_id}_sample{next_sample_idx}_output.txt"
                input_path.write_text(input_text, encoding="utf-8")
                output_path.write_text(output_text, encoding="utf-8")
                next_sample_idx += 1
                local_pairs += 1
                wrote_pairs += 1

            print(f"[DONE] {problem_id}/{test_file.name}: extracted {len(samples)} sample pair(s)")

        if local_pairs > 0:
            files_with_pairs += 1

    print("\nDone.")
    print(f"Scanned problem directories: {scanned_problem_dirs}")
    print(f"Scanned test files: {scanned_test_files}")
    print(f"Problem directories with extracted pairs: {files_with_pairs}")
    print(f"Extracted sample pairs: {wrote_pairs}")
    print(f"Output directory: {OUTPUT_ROOT}")


if __name__ == "__main__":
    main()
