#!/usr/bin/env python3
"""
Estimate token usage and API cost for spec/translation runs stored under
api_responses/open_ai/optimized_prompt@1/C/.

Two directory layouts are handled automatically:

  Dirs with attempt_N subdirs (validated spec runs):
    attempt_N/
      *_prompt.txt   — counted as input tokens (all prompt files summed)
      *_response.txt — counted as output tokens
      spec.txt       — counted as output tokens
      diagnosis.txt  — counted as output tokens

  Flat dirs (no attempt subdirs — prompt not saved):
    *_spec.txt  or  *.rs  per problem
    Only the output file is available; input tokens are reported as 0.

Usage:
    python cost_estimates/estimate_spec_generation_cost.py
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

try:
    from cost_estimates.cost_estimate_utils import (
        compute_cost,
        estimate_tokens,
        estimate_tokens_secondary,
        print_attempt_distribution,
        print_directory_table,
        print_matched_files,
        print_pricing_header,
        print_tokenizer_comparison,
        read,
        warn_unknown_models,
        write_csv,
        write_directory_summary_csv,
    )
except ImportError:
    from cost_estimate_utils import (
        compute_cost,
        estimate_tokens,
        estimate_tokens_secondary,
        print_attempt_distribution,
        print_directory_table,
        print_matched_files,
        print_pricing_header,
        print_tokenizer_comparison,
        read,
        warn_unknown_models,
        write_csv,
        write_directory_summary_csv,
    )

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SCRIPT_DIR = Path(__file__).resolve().parent
API_ROOT = PROJECT_ROOT / "api_responses" / "open_ai" / "optimized_prompt@1" / "C"

# =====================================================
# Configure directories here: (subdir name, model)
# =====================================================
DIRECTORIES: list[tuple[str, str]] = [
    ("specs_WITH_GPT_5_2",                                             "gpt-5.2"),
    ("validated_specs_WITH_GPT_gpt-5.2",                               "gpt-5.2"),
    ("specs",                           "gpt-4o-mini"),
]

# Inside attempt_N dirs, these named files are direct LLM outputs (no paired _response.txt)
ATTEMPT_OUTPUT_FILENAMES = {"spec.txt", "diagnosis.txt"}

# For flat problem dirs (no attempt subdirs), these patterns identify the output file
FLAT_OUTPUT_SUFFIXES = {".rs", ".c"}

CSV_FIELDNAMES = [
    "directory", "model", "problem_id", "attempt",
    "input_tokens", "output_tokens", "total_tokens",
    "input_tokens_cl100k", "output_tokens_cl100k",
    "cost_usd",
]


def scan_attempt_dir(
    attempt_dir: Path,
    file_hits: dict[str, dict],
) -> tuple[int, int, int, int]:
    """Return (input_tokens, output_tokens) for one attempt directory.

    Only files directly inside attempt_N/ are read — never the problem root.
    *_prompt.txt  → input.
    *_response.txt and known output filenames → output.
    Extracted artefacts (.c, .rs, run_report.txt, samples.csv) are skipped
    to avoid double-counting with the paired *_response.txt.

    file_hits is updated in-place: {filename: {role, count, tokens}}.
    """
    input_tokens = input_tokens_cl100k = 0
    output_tokens = output_tokens_cl100k = 0
    for f in sorted(attempt_dir.iterdir()):
        if not f.is_file():
            continue
        text = read(f)
        tokens     = estimate_tokens(text)
        tokens_alt = estimate_tokens_secondary(text)
        if f.name.endswith("_prompt.txt"):
            role = "input"
            input_tokens        += tokens
            input_tokens_cl100k += tokens_alt
        elif f.name.endswith("_response.txt") or f.name in ATTEMPT_OUTPUT_FILENAMES:
            role = "output"
            output_tokens        += tokens
            output_tokens_cl100k += tokens_alt
        else:
            continue
        entry = file_hits.setdefault(f.name, {"role": role, "count": 0, "tokens": 0})
        entry["count"]  += 1
        entry["tokens"] += tokens
    return input_tokens, output_tokens, input_tokens_cl100k, output_tokens_cl100k


def scan_directory(
    dir_name: str, model: str
) -> tuple[list[dict], dict[str, dict]]:
    """Return (rows, file_hits) for a single configured directory.

    file_hits: {filename: {role, count, tokens}} aggregated across all problems.
    """
    dir_path = API_ROOT / dir_name
    if not dir_path.is_dir():
        print(f"[WARN] Directory not found: {dir_path}", file=sys.stderr)
        return [], {}

    rows: list[dict] = []
    file_hits: dict[str, dict] = {}

    for problem_dir in sorted(dir_path.iterdir()):
        if not problem_dir.is_dir() or not problem_dir.name.startswith("p"):
            continue

        attempt_dirs = sorted(problem_dir.glob("attempt_*"))

        if attempt_dirs:
            for attempt_dir in attempt_dirs:
                if not attempt_dir.is_dir():
                    continue
                try:
                    attempt_number = int(attempt_dir.name.split("_")[1])
                except (IndexError, ValueError):
                    continue

                in_tok, out_tok, in_tok_alt, out_tok_alt = scan_attempt_dir(attempt_dir, file_hits)
                if in_tok == 0 and out_tok == 0:
                    continue

                cost = compute_cost(in_tok, out_tok, model)
                rows.append({
                    "directory": dir_name,
                    "model": model,
                    "problem_id": problem_dir.name,
                    "attempt": attempt_number,
                    "input_tokens": in_tok,
                    "output_tokens": out_tok,
                    "total_tokens": in_tok + out_tok,
                    "input_tokens_cl100k": in_tok_alt,
                    "output_tokens_cl100k": out_tok_alt,
                    "cost_usd": round(cost, 6),
                })
        else:
            # Flat dir — *_prompt.txt (input) and *_spec.txt (output)
            in_tok = out_tok = in_tok_alt = out_tok_alt = 0
            for f in sorted(problem_dir.iterdir()):
                if not f.is_file():
                    continue
                text = read(f)
                if f.name.endswith("_prompt.txt"):
                    t = estimate_tokens(text)
                    t_alt = estimate_tokens_secondary(text)
                    in_tok += t
                    in_tok_alt += t_alt
                    entry = file_hits.setdefault("*_prompt.txt", {"role": "input", "count": 0, "tokens": 0})
                    entry["count"] += 1
                    entry["tokens"] += t
                elif f.name.endswith("_spec.txt"):
                    t = estimate_tokens(text)
                    t_alt = estimate_tokens_secondary(text)
                    out_tok += t
                    out_tok_alt += t_alt
                    entry = file_hits.setdefault("*_spec.txt", {"role": "output", "count": 0, "tokens": 0})
                    entry["count"] += 1
                    entry["tokens"] += t

            if out_tok == 0:
                continue
            cost = compute_cost(in_tok, out_tok, model)
            rows.append({
                "directory": dir_name,
                "model": model,
                "problem_id": problem_dir.name,
                "attempt": 1,
                "input_tokens": in_tok,
                "output_tokens": out_tok,
                "total_tokens": in_tok + out_tok,
                "input_tokens_cl100k": in_tok_alt,
                "output_tokens_cl100k": out_tok_alt,
                "cost_usd": round(cost, 6),
            })

    return rows, file_hits


def print_summary(
    rows: list[dict],
    file_hits_by_dir: dict[str, dict[str, dict]],
) -> None:
    if not rows:
        print("No data found.")
        return

    print_pricing_header({row["model"] for row in rows})
    print_directory_table(rows)
    print_matched_files(file_hits_by_dir)
    print_tokenizer_comparison(rows)
    print_attempt_distribution(rows)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Estimate token cost for spec/translation runs in api_responses."
    )
    parser.add_argument(
        "--csv",
        metavar="PATH",
        default=str(SCRIPT_DIR / "spec_generation_cost_estimate.csv"),
        help="CSV output path",
    )
    args = parser.parse_args()

    if not DIRECTORIES:
        print("No directories configured.", file=sys.stderr)
        sys.exit(1)

    warn_unknown_models(DIRECTORIES)

    print(f"Root: {API_ROOT}")
    print(f"Scanning {len(DIRECTORIES)} director(ies):")
    all_rows: list[dict] = []
    file_hits_by_dir: dict[str, dict[str, dict]] = {}
    for dir_name, model in DIRECTORIES:
        rows, file_hits = scan_directory(dir_name, model)
        print(f"  [{model}] {dir_name}: {len(rows)} records")
        all_rows.extend(rows)
        file_hits_by_dir[dir_name] = file_hits

    print_summary(all_rows, file_hits_by_dir)
    detail_path = Path(args.csv)
    summary_path = detail_path.with_stem(detail_path.stem + "_summary")
    write_csv(all_rows, detail_path, CSV_FIELDNAMES)
    write_directory_summary_csv(all_rows, summary_path)


if __name__ == "__main__":
    main()
