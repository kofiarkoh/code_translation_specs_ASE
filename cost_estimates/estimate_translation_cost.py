#!/usr/bin/env python3
"""
Estimate token usage and API cost for translation repair runs stored under
repairs/<pipeline>/<profile>/<dataset>/<problem_id>/attempt_N/.

Reads repair_prompt.txt (input) and repair_response.txt (output) from each
attempt directory.

Usage:
    python cost_estimates/estimate_translation_cost.py
"""

from __future__ import annotations

import argparse
import csv
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
    )

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SCRIPT_DIR = Path(__file__).resolve().parent
REPAIRS_ROOT = PROJECT_ROOT / "repairs"

DEFAULT_DATASET = "optimized_prompt@1"

# ============================================
# Configure pipelines here: (name, model)
# ============================================
PIPELINES: list[tuple[str, str]] = [
    ("full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS", "gpt-5.2"),
    ("translation_with_validated_specs_GPT_5_2",          "gpt-5.2"),
    ("full_translation_loop_GPT_5_2",                     "gpt-5.2"),
    # ("full_translation_loop",                     "gpt-4o-mini"),
    # ("translation_with_validated_specs_using_GPT_4O_mini",                     "gpt-4o-mini"),
]

CSV_FIELDNAMES = [
    "directory", "model", "profile", "problem_id", "attempt",
    "input_tokens", "output_tokens", "total_tokens",
    "input_tokens_cl100k", "output_tokens_cl100k",
    "cost_usd",
]


def scan_pipeline(
    pipeline_name: str, dataset_name: str, model: str
) -> tuple[list[dict], dict[str, dict]]:
    pipeline_root = REPAIRS_ROOT / pipeline_name
    if not pipeline_root.is_dir():
        print(f"[WARN] Pipeline directory not found: {pipeline_root}", file=sys.stderr)
        return [], {}

    rows: list[dict] = []
    file_hits: dict[str, dict] = {}

    for profile_dir in sorted(pipeline_root.iterdir()):
        if not profile_dir.is_dir():
            continue
        profile_name = profile_dir.name

        dataset_dir = profile_dir / dataset_name
        if not dataset_dir.is_dir():
            dataset_dir = profile_dir

        for problem_dir in sorted(dataset_dir.iterdir()):
            if not problem_dir.is_dir() or not problem_dir.name.startswith("p"):
                continue

            for attempt_dir in sorted(problem_dir.glob("attempt_*")):
                if not attempt_dir.is_dir():
                    continue
                try:
                    attempt_number = int(attempt_dir.name.split("_")[1])
                except (IndexError, ValueError):
                    continue

                prompt_path = attempt_dir / "repair_prompt.txt"
                response_path = attempt_dir / "repair_response.txt"

                prompt_text = read(prompt_path)
                response_text = read(response_path)

                input_tokens      = estimate_tokens(prompt_text) if prompt_text else 0
                output_tokens     = estimate_tokens(response_text) if response_text else 0
                input_tokens_alt  = estimate_tokens_secondary(prompt_text) if prompt_text else 0
                output_tokens_alt = estimate_tokens_secondary(response_text) if response_text else 0
                cost = compute_cost(input_tokens, output_tokens, model)

                rows.append({
                    "directory": pipeline_name,
                    "model": model,
                    "profile": profile_name,
                    "problem_id": problem_dir.name,
                    "attempt": attempt_number,
                    "input_tokens": input_tokens,
                    "output_tokens": output_tokens,
                    "total_tokens": input_tokens + output_tokens,
                    "input_tokens_cl100k": input_tokens_alt,
                    "output_tokens_cl100k": output_tokens_alt,
                    "cost_usd": round(cost, 6),
                })

                for fname, role, tokens in [
                    ("repair_prompt.txt", "input", input_tokens),
                    ("repair_response.txt", "output", output_tokens),
                ]:
                    entry = file_hits.setdefault(fname, {"role": role, "count": 0, "tokens": 0})
                    entry["count"] += 1
                    entry["tokens"] += tokens

    return rows, file_hits


def print_profile_table(rows: list[dict]) -> None:
    agg: dict[tuple[str, str], dict] = {}
    for row in rows:
        key = (row["directory"], row["profile"])
        if key not in agg:
            agg[key] = {
                "model": row["model"],
                "input": 0, "output": 0, "total": 0,
                "cost": 0.0, "calls": 0, "problems": set(),
            }
        agg[key]["input"] += row["input_tokens"]
        agg[key]["output"] += row["output_tokens"]
        agg[key]["total"] += row["total_tokens"]
        agg[key]["cost"] += row["cost_usd"]
        agg[key]["calls"] += 1
        agg[key]["problems"].add(row["problem_id"])

    print("\n--- Per profile ---")
    col = 55
    print(
        f"{'Pipeline':<{col}} {'Model':<14} {'Profile':<15} {'Problems':>9}"
        f" {'Calls':>6} {'Input Tok':>12} {'Output Tok':>12} {'Cost (USD)':>12}"
    )
    print("-" * 145)
    for (pipeline_name, profile_name), s in sorted(agg.items()):
        print(
            f"{pipeline_name:<{col}} {s['model']:<14} {profile_name:<15}"
            f" {len(s['problems']):>9,} {s['calls']:>6,}"
            f" {s['input']:>12,} {s['output']:>12,} ${s['cost']:>11.4f}"
        )


def write_profile_summary_csv(rows: list[dict], output_path: Path) -> None:
    agg: dict[tuple[str, str], dict] = {}
    for row in rows:
        key = (row["directory"], row["profile"])
        if key not in agg:
            agg[key] = {
                "pipeline": row["directory"],
                "model": row["model"],
                "profile": row["profile"],
                "problems": set(),
                "calls": 0,
                "input_tokens": 0,
                "output_tokens": 0,
                "total_tokens": 0,
                "cost_usd": 0.0,
            }
        agg[key]["problems"].add(row["problem_id"])
        agg[key]["calls"] += 1
        agg[key]["input_tokens"] += row["input_tokens"]
        agg[key]["output_tokens"] += row["output_tokens"]
        agg[key]["total_tokens"] += row["total_tokens"]
        agg[key]["cost_usd"] += row["cost_usd"]

    fieldnames = ["pipeline", "model", "profile", "problems", "calls",
                  "input_tokens", "output_tokens", "total_tokens", "cost_usd"]
    summary_rows = [
        {**{k: v for k, v in s.items() if k != "problems"},
         "problems": len(s["problems"])}
        for s in sorted(agg.values(), key=lambda x: (x["pipeline"], x["profile"]))
    ]

    with output_path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(summary_rows)
    print(f"Profile summary CSV written to: {output_path}")


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
    print_profile_table(rows)
    print_attempt_distribution(rows)


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Estimate token cost for translation repair runs in repairs/."
    )
    parser.add_argument(
        "--dataset",
        default=DEFAULT_DATASET,
        help=f"Dataset subfolder inside each profile dir (default: {DEFAULT_DATASET})",
    )
    parser.add_argument(
        "--csv",
        metavar="PATH",
        default=str(SCRIPT_DIR / "translation_cost_estimate.csv"),
        help="CSV output path",
    )
    args = parser.parse_args()

    if not PIPELINES:
        print("No pipelines configured.", file=sys.stderr)
        sys.exit(1)

    warn_unknown_models(PIPELINES)

    print(f"Root: {REPAIRS_ROOT}")
    print(f"Scanning {len(PIPELINES)} pipeline(s):")
    all_rows: list[dict] = []
    file_hits_by_dir: dict[str, dict[str, dict]] = {}
    for pipeline_name, model in PIPELINES:
        rows, file_hits = scan_pipeline(pipeline_name, args.dataset, model)
        print(f"  [{model}] {pipeline_name}: {len(rows)} attempt records")
        all_rows.extend(rows)
        file_hits_by_dir[pipeline_name] = file_hits

    print_summary(all_rows, file_hits_by_dir)
    detail_path = Path(args.csv)
    summary_path = detail_path.with_stem(detail_path.stem + "_summary")
    write_csv(all_rows, detail_path, CSV_FIELDNAMES)
    write_profile_summary_csv(all_rows, summary_path)


if __name__ == "__main__":
    main()
