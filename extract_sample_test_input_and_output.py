#!/usr/bin/env python3
"""
Extract sample input/output blocks from problem HTML files.

For each HTML file found under HTML_ROOT, this script writes:
  OUTPUT_ROOT/<problem_id>/<problem_id>_sample<N>_input.txt
  OUTPUT_ROOT/<problem_id>/<problem_id>_sample<N>_output.txt
"""

from __future__ import annotations

import html
import re
from pathlib import Path


# =========================
# Configure these values
# =========================
HTML_ROOT = Path("/Volumes/Extreme SSD/research/code translation/Project_CodeNet/problem_descriptions")
OUTPUT_ROOT = Path("/Volumes/Extreme SSD/research/code translation/collected_dataset/problem_test_inputs_and_outputs")
RECURSIVE = True


HEADING_PRE_PATTERN = re.compile(
    r"<h[1-6][^>]*>\s*(.*?)\s*</h[1-6]>\s*<pre[^>]*>(.*?)</pre>",
    flags=re.IGNORECASE | re.DOTALL,
)

SAMPLE_INPUT_PATTERN = re.compile(r"sample\s*input(?:\s*(\d+))?", flags=re.IGNORECASE)
SAMPLE_OUTPUT_PATTERN_1 = re.compile(
    r"output\s*for\s*the\s*sample\s*input(?:\s*(\d+))?", flags=re.IGNORECASE
)
SAMPLE_OUTPUT_PATTERN_2 = re.compile(r"sample\s*output(?:\s*(\d+))?", flags=re.IGNORECASE)


def normalize_pre_text(raw: str) -> str:
    """Decode HTML entities and trim only outer blank lines."""
    text = html.unescape(raw)
    lines = text.splitlines()

    while lines and not lines[0].strip():
        lines.pop(0)
    while lines and not lines[-1].strip():
        lines.pop()

    return "\n".join(lines) + ("\n" if lines else "")


def extract_samples_from_html(content: str) -> dict[int, dict[str, str]]:
    """
    Return a mapping like:
      {1: {"input": "...", "output": "..."}, 2: {...}}
    """
    samples: dict[int, dict[str, str]] = {}

    next_input_idx = 1
    next_output_idx = 1

    for heading_raw, pre_raw in HEADING_PRE_PATTERN.findall(content):
        heading = html.unescape(re.sub(r"<[^>]+>", "", heading_raw)).strip()
        pre_text = normalize_pre_text(pre_raw)

        output_match = SAMPLE_OUTPUT_PATTERN_1.search(heading)
        if not output_match:
            output_match = SAMPLE_OUTPUT_PATTERN_2.search(heading)
        if output_match:
            number = output_match.group(1)
            if number is not None:
                idx = int(number)
                next_output_idx = max(next_output_idx, idx + 1)
            else:
                idx = next_output_idx
                next_output_idx += 1
            samples.setdefault(idx, {})["output"] = pre_text
            continue

        input_match = SAMPLE_INPUT_PATTERN.search(heading)
        if input_match:
            number = input_match.group(1)
            if number is not None:
                idx = int(number)
                next_input_idx = max(next_input_idx, idx + 1)
            else:
                idx = next_input_idx
                next_input_idx += 1
            samples.setdefault(idx, {})["input"] = pre_text

    return samples


def main() -> None:
    if not HTML_ROOT.is_dir():
        raise FileNotFoundError(f"HTML_ROOT does not exist or is not a directory: {HTML_ROOT}")

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)

    html_files = sorted(HTML_ROOT.rglob("*.html") if RECURSIVE else HTML_ROOT.glob("*.html"))

    scanned = 0
    wrote_pairs = 0
    files_with_pairs = 0

    for html_file in html_files:
        scanned += 1
        problem_id = html_file.stem

        content = html_file.read_text(encoding="utf-8", errors="ignore")
        samples = extract_samples_from_html(content)

        local_pairs = 0
        for idx in sorted(samples):
            input_text = samples[idx].get("input")
            output_text = samples[idx].get("output")
            if input_text is None or output_text is None:
                continue

            out_problem_dir = OUTPUT_ROOT / problem_id
            out_problem_dir.mkdir(parents=True, exist_ok=True)
            input_path = out_problem_dir / f"{problem_id}_sample{idx}_input.txt"
            output_path = out_problem_dir / f"{problem_id}_sample{idx}_output.txt"
            input_path.write_text(input_text, encoding="utf-8")
            output_path.write_text(output_text, encoding="utf-8")
            local_pairs += 1
            wrote_pairs += 1

        if local_pairs > 0:
            files_with_pairs += 1
        print(f"[DONE] {html_file.name}: extracted {local_pairs} sample pair(s)")

    print("\nDone.")
    print(f"Scanned HTML files: {scanned}")
    print(f"HTML files with extracted pairs: {files_with_pairs}")
    print(f"Extracted sample pairs: {wrote_pairs}")
    print(f"Output directory: {OUTPUT_ROOT}")


if __name__ == "__main__":
    main()
