"""Shared utilities for cost estimation scripts."""

from __future__ import annotations

import csv
import sys
from functools import lru_cache
from pathlib import Path

import tiktoken

MODEL_PRICING: dict[str, dict[str, float]] = {
    "gpt-5.2":      {"input": 1.75,  "output": 14.00},
    "gpt-4o":       {"input": 2.50,  "output": 10.00},
    "gpt-4o-mini":  {"input": 0.15,  "output": 0.60},
    "gpt-4.1":      {"input": 2.00,  "output": 8.00},
    "gpt-4.1-mini": {"input": 0.40,  "output": 1.60},
}

# Primary encoding used for cost calculation (OpenAI GPT-4o+ tokenizer).
# gpt-5.2 not yet in tiktoken — approximated with o200k_base.
PRIMARY_ENCODING = "o200k_base"

# Secondary encoding used only for comparison (GPT-4 / Claude approximation).
SECONDARY_ENCODING = "cl100k_base"


@lru_cache(maxsize=8)
def _get_encoding(encoding_name: str) -> tiktoken.Encoding:
    return tiktoken.get_encoding(encoding_name)


def estimate_tokens(text: str) -> int:
    """Count tokens using the primary encoding (o200k_base)."""
    return max(1, len(_get_encoding(PRIMARY_ENCODING).encode(text)))


def estimate_tokens_secondary(text: str) -> int:
    """Count tokens using the secondary encoding (cl100k_base) for comparison."""
    return max(1, len(_get_encoding(SECONDARY_ENCODING).encode(text)))


def compute_cost(input_tokens: int, output_tokens: int, model: str) -> float:
    pricing = MODEL_PRICING.get(model)
    if pricing is None:
        return 0.0
    return (
        input_tokens * pricing["input"] + output_tokens * pricing["output"]
    ) / 1_000_000


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="ignore") if path.is_file() else ""


def warn_unknown_models(entries: list[tuple[str, str]]) -> None:
    unknown = {m for _, m in entries if m not in MODEL_PRICING}
    if unknown:
        print(
            f"[WARN] Unknown model(s) — cost will be $0: {', '.join(sorted(unknown))}",
            file=sys.stderr,
        )


def print_pricing_header(models_used: set[str]) -> None:
    print(
        f"Token counting: primary={PRIMARY_ENCODING} (cost basis), "
        f"secondary={SECONDARY_ENCODING} (comparison only)"
    )
    for m in sorted(models_used):
        pricing = MODEL_PRICING.get(m, {})
        if pricing:
            print(
                f"  {m}: ${pricing['input']:.2f}/1M input, "
                f"${pricing['output']:.2f}/1M output"
            )
    print()


def print_tokenizer_comparison(rows: list[dict]) -> None:
    agg: dict[str, dict] = {}
    for row in rows:
        d = row["directory"]
        if d not in agg:
            agg[d] = {"primary": 0, "secondary": 0}
        agg[d]["primary"]   += row["input_tokens"]   + row["output_tokens"]
        agg[d]["secondary"] += row.get("input_tokens_cl100k", 0) + row.get("output_tokens_cl100k", 0)

    print(f"\n--- Tokenizer comparison ({PRIMARY_ENCODING} vs {SECONDARY_ENCODING}) ---")
    col = 60
    print(
        f"{'Directory':<{col}} {PRIMARY_ENCODING:>12} {SECONDARY_ENCODING:>12} {'diff':>8} {'diff%':>8}"
    )
    print("-" * (col + 46))
    for d, s in sorted(agg.items()):
        p, sec = s["primary"], s["secondary"]
        diff = sec - p
        pct = (diff / p * 100) if p else 0.0
        sign = "+" if diff >= 0 else ""
        print(
            f"{d:<{col}} {p:>12,} {sec:>12,} {sign}{diff:>7,} {sign}{pct:>7.2f}%"
        )


def print_directory_table(rows: list[dict]) -> None:
    agg: dict[str, dict] = {}
    for row in rows:
        d = row["directory"]
        if d not in agg:
            agg[d] = {
                "model": row["model"],
                "input": 0, "output": 0, "total": 0, "cost": 0.0,
                "calls": 0, "problems": set(),
            }
        agg[d]["input"] += row["input_tokens"]
        agg[d]["output"] += row["output_tokens"]
        agg[d]["total"] += row["total_tokens"]
        agg[d]["cost"] += row["cost_usd"]
        agg[d]["calls"] += 1
        agg[d]["problems"].add(row["problem_id"])

    col = 60
    hdr = (
        f"{'Directory':<{col}} {'Model':<14} {'Problems':>9} {'Calls':>6}"
        f" {'Input Tok':>12} {'Output Tok':>12} {'Cost (USD)':>12}"
    )
    sep = "-" * len(hdr)
    print(hdr)
    print(sep)
    for name, s in sorted(agg.items()):
        print(
            f"{name:<{col}} {s['model']:<14} {len(s['problems']):>9,} {s['calls']:>6,}"
            f" {s['input']:>12,} {s['output']:>12,} ${s['cost']:>11.4f}"
        )
    print(sep)
    print(
        f"{'TOTAL':<{col}} {'':<14}"
        f" {sum(len(s['problems']) for s in agg.values()):>9,}"
        f" {sum(s['calls'] for s in agg.values()):>6,}"
        f" {sum(s['input'] for s in agg.values()):>12,}"
        f" {sum(s['output'] for s in agg.values()):>12,}"
        f" ${sum(s['cost'] for s in agg.values()):>11.4f}"
    )


def print_attempt_distribution(rows: list[dict]) -> None:
    counts: dict[int, int] = {}
    for row in rows:
        a = row["attempt"]
        counts[a] = counts.get(a, 0) + 1

    print("\n--- Attempt distribution ---")
    for attempt_num in sorted(counts):
        print(f"  Attempt {attempt_num}: {counts[attempt_num]:,} API calls")


def print_matched_files(file_hits_by_dir: dict[str, dict[str, dict]]) -> None:
    print("\n--- Matched files per directory ---")
    for dir_name, hits in sorted(file_hits_by_dir.items()):
        print(f"\n  {dir_name}")
        col = 40
        print(f"    {'File':<{col}} {'Role':<8} {'Matches':>8} {'Tokens':>12}")
        print(f"    {'-' * (col + 32)}")
        for filename, s in sorted(hits.items()):
            print(
                f"    {filename:<{col}} {s['role']:<8} {s['count']:>8,} {s['tokens']:>12,}"
            )


def write_directory_summary_csv(rows: list[dict], output_path: Path) -> None:
    agg: dict[str, dict] = {}
    for row in rows:
        d = row["directory"]
        if d not in agg:
            agg[d] = {
                "directory": d,
                "model": row["model"],
                "problems": set(),
                "calls": 0,
                "input_tokens": 0,
                "output_tokens": 0,
                "total_tokens": 0,
                "cost_usd": 0.0,
            }
        agg[d]["problems"].add(row["problem_id"])
        agg[d]["calls"] += 1
        agg[d]["input_tokens"] += row["input_tokens"]
        agg[d]["output_tokens"] += row["output_tokens"]
        agg[d]["total_tokens"] += row["total_tokens"]
        agg[d]["cost_usd"] += row["cost_usd"]

    fieldnames = ["directory", "model", "problems", "calls",
                  "input_tokens", "output_tokens", "total_tokens", "cost_usd"]
    summary_rows = [
        {**{k: v for k, v in s.items() if k != "problems"},
         "problems": len(s["problems"])}
        for s in sorted(agg.values(), key=lambda x: x["directory"])
    ]

    with output_path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(summary_rows)
    print(f"Summary CSV written to:  {output_path}")


def write_csv(rows: list[dict], output_path: Path, fieldnames: list[str]) -> None:
    if not rows:
        return
    with output_path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames, extrasaction="ignore")
        writer.writeheader()
        writer.writerows(rows)
    print(f"\nDetailed CSV written to: {output_path}")
