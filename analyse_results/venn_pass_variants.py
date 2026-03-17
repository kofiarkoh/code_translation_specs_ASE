#!/usr/bin/env python3
from __future__ import annotations
import os
import csv
from collections import defaultdict
from itertools import combinations
from pathlib import Path

import matplotlib.pyplot as plt
from dotenv import load_dotenv
load_dotenv()


try:
    from matplotlib_venn import venn2, venn3
except ImportError:  # pragma: no cover - optional dependency
    venn2 = None
    venn3 = None


PROJECT_ROOT = Path(__file__).resolve().parent.parent
RUN_SUMMARY_CSV = PROJECT_ROOT / "results" / Path(os.environ.get("DATASET_NAME")) / "rust_run_reports" / "run_summary.csv"
INPUT_CSV = RUN_SUMMARY_CSV
OUTPUT_DIR = INPUT_CSV.parent.parent / "pass_venn_analysis"
SUMMARY_TXT = OUTPUT_DIR / "venn_summary.txt"
VARIANT_COUNTS_CSV = OUTPUT_DIR / "variant_pass_counts.csv"
INTERSECTIONS_CSV = OUTPUT_DIR / "variant_intersections.csv"
MEMBERSHIP_CSV = OUTPUT_DIR / "problem_membership_matrix.csv"
VENN_PNG = OUTPUT_DIR / "pass_variants_venn.png"


def variant_name_from_path(path_text: str) -> str:
    path = Path(path_text.strip())
    return path.name if path.name else "unknown"


def load_variant_problem_sets() -> dict[str, set[str]]:
    if not INPUT_CSV.is_file():
        raise FileNotFoundError(f"INPUT_CSV does not exist: {INPUT_CSV}")

    variant_problem_ids: dict[str, set[str]] = defaultdict(set)

    with INPUT_CSV.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            problem_id = row.get("problem_id", "").strip()
            status = row.get("status", "").strip()
            rust_file = row.get("rust_file", "").strip()
            if not problem_id or status != "PASS":
                continue

            variant = variant_name_from_path(rust_file)
            variant_problem_ids[variant].add(problem_id)

    return dict(sorted(variant_problem_ids.items()))


def write_variant_counts(variant_problem_ids: dict[str, set[str]]) -> None:
    with VARIANT_COUNTS_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["variant", "pass_problem_count"])
        for variant, problem_ids in variant_problem_ids.items():
            writer.writerow([variant, len(problem_ids)])


def write_membership_matrix(variant_problem_ids: dict[str, set[str]]) -> None:
    all_problem_ids = sorted({problem_id for ids in variant_problem_ids.values() for problem_id in ids})
    variants = list(variant_problem_ids)

    with MEMBERSHIP_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["problem_id", *variants])
        for problem_id in all_problem_ids:
            writer.writerow([
                problem_id,
                *["1" if problem_id in variant_problem_ids[variant] else "0" for variant in variants],
            ])


def write_intersections(variant_problem_ids: dict[str, set[str]]) -> list[str]:
    variants = list(variant_problem_ids)
    summary_lines: list[str] = []

    with INTERSECTIONS_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["combination_size", "variants", "intersection_count", "problem_ids"])

        for size in range(2, len(variants) + 1):
            for combo in combinations(variants, size):
                overlap = set.intersection(*(variant_problem_ids[variant] for variant in combo))
                overlap_ids = sorted(overlap)
                writer.writerow([size, " | ".join(combo), len(overlap_ids), ";".join(overlap_ids)])
                summary_lines.append(f"{' INTERSECT '.join(combo)}: {len(overlap_ids)}")

    return summary_lines


def save_venn_plot(variant_problem_ids: dict[str, set[str]]) -> str:
    variants = list(variant_problem_ids)
    if len(variants) not in {2, 3}:
        return f"Skipped Venn plot: expected 2 or 3 variants, found {len(variants)}."
    if len(variants) == 2 and venn2 is None:
        return "Skipped Venn plot: matplotlib-venn is not installed."
    if len(variants) == 3 and venn3 is None:
        return "Skipped Venn plot: matplotlib-venn is not installed."

    fig, ax = plt.subplots(figsize=(8, 8))
    sets = [variant_problem_ids[variant] for variant in variants]

    if len(variants) == 2:
        venn2(subsets=sets, set_labels=variants, ax=ax)
    else:
        venn3(subsets=sets, set_labels=variants, ax=ax)

    ax.set_title("PASS Problem Overlap by Variant")
    fig.tight_layout()
    fig.savefig(VENN_PNG, dpi=200)
    plt.close(fig)
    return f"Venn plot written to: {VENN_PNG}"


def main() -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    variant_problem_ids = load_variant_problem_sets()
    if not variant_problem_ids:
        raise ValueError(f"No PASS variant/problem pairs were found in {INPUT_CSV}")

    write_variant_counts(variant_problem_ids)
    write_membership_matrix(variant_problem_ids)
    intersection_summary = write_intersections(variant_problem_ids)
    venn_status = save_venn_plot(variant_problem_ids)

    summary_lines = [
        f"INPUT_CSV: {INPUT_CSV}",
        f"Variants found: {len(variant_problem_ids)}",
        "",
        "Per-variant PASS counts:",
    ]

    for variant, problem_ids in variant_problem_ids.items():
        summary_lines.append(f"- {variant}: {len(problem_ids)}")

    summary_lines.append("")
    summary_lines.append("Intersections:")
    if intersection_summary:
        summary_lines.extend(intersection_summary)
    else:
        summary_lines.append("No multi-variant intersections to report.")

    summary_lines.append("")
    summary_lines.append(f"VARIANT_COUNTS_CSV: {VARIANT_COUNTS_CSV}")
    summary_lines.append(f"INTERSECTIONS_CSV: {INTERSECTIONS_CSV}")
    summary_lines.append(f"MEMBERSHIP_CSV: {MEMBERSHIP_CSV}")
    summary_lines.append(venn_status)

    SUMMARY_TXT.write_text("\n".join(summary_lines) + "\n", encoding="utf-8")

    print(f"Summary written to: {SUMMARY_TXT}")
    print(f"Variant counts CSV: {VARIANT_COUNTS_CSV}")
    print(f"Intersections CSV: {INTERSECTIONS_CSV}")
    print(f"Membership matrix CSV: {MEMBERSHIP_CSV}")
    print(venn_status)


if __name__ == "__main__":
    main()
