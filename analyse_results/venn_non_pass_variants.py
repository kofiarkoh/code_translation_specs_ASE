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
INPUT_CSV = PROJECT_ROOT/  "results" / Path(os.environ.get("DATASET_NAME")) / "translated_code_execution_statuses_output/non_pass_problems.csv"
TARGET_STATUS = "FAIL"


def slugify_status(status: str) -> str:
    return "".join(ch.lower() if ch.isalnum() else "_" for ch in status).strip("_") or "unknown"


def build_output_paths(status: str) -> dict[str, Path]:
    status_slug = slugify_status(status)
    output_dir = INPUT_CSV.parent.parent / f"{status_slug}_venn_analysis"
    return {
        "output_dir": output_dir,
        "summary_txt": output_dir / "venn_summary.txt",
        "variant_counts_csv": output_dir / f"variant_{status_slug}_counts.csv",
        "intersections_csv": output_dir / "variant_intersections.csv",
        "membership_csv": output_dir / "problem_membership_matrix.csv",
        "venn_png": output_dir / f"{status_slug}_variants_venn.png",
    }


def load_variant_problem_sets(target_status: str) -> dict[str, set[str]]:
    if not INPUT_CSV.is_file():
        raise FileNotFoundError(f"INPUT_CSV does not exist: {INPUT_CSV}")

    variant_problem_ids: dict[str, set[str]] = defaultdict(set)

    with INPUT_CSV.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        variant_column = "variant" if "variant" in (reader.fieldnames or []) else "variants"

        for row in reader:
            problem_id = row.get("problem_id", "").strip()
            variant = row.get(variant_column, "").strip()
            status = row.get("status", "").strip()
            if not problem_id or not variant or status != target_status:
                continue
            variant_problem_ids[variant].add(problem_id)

    return dict(sorted(variant_problem_ids.items()))


def write_variant_counts(variant_problem_ids: dict[str, set[str]], output_path: Path, target_status: str) -> None:
    with output_path.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["variant", f"{slugify_status(target_status)}_problem_count"])
        for variant, problem_ids in variant_problem_ids.items():
            writer.writerow([variant, len(problem_ids)])


def write_membership_matrix(variant_problem_ids: dict[str, set[str]], output_path: Path) -> None:
    all_problem_ids = sorted({problem_id for ids in variant_problem_ids.values() for problem_id in ids})
    variants = list(variant_problem_ids)

    with output_path.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["problem_id", *variants])
        for problem_id in all_problem_ids:
            writer.writerow([
                problem_id,
                *["1" if problem_id in variant_problem_ids[variant] else "0" for variant in variants],
            ])


def write_intersections(variant_problem_ids: dict[str, set[str]], output_path: Path) -> list[str]:
    variants = list(variant_problem_ids)
    summary_lines: list[str] = []

    with output_path.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["combination_size", "variants", "intersection_count", "problem_ids"])

        for size in range(2, len(variants) + 1):
            for combo in combinations(variants, size):
                overlap = set.intersection(*(variant_problem_ids[variant] for variant in combo))
                overlap_ids = sorted(overlap)
                writer.writerow([size, " | ".join(combo), len(overlap_ids), ";".join(overlap_ids)])
                summary_lines.append(f"{' INTERSECT '.join(combo)}: {len(overlap_ids)}")

    return summary_lines


def save_venn_plot(variant_problem_ids: dict[str, set[str]], output_path: Path, target_status: str) -> str:
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

    ax.set_title(f"{target_status} Problem Overlap by Variant")
    fig.tight_layout()
    fig.savefig(output_path, dpi=200)
    plt.close(fig)
    return f"Venn plot written to: {output_path}"


def main() -> None:
    output_paths = build_output_paths(TARGET_STATUS)
    output_paths["output_dir"].mkdir(parents=True, exist_ok=True)

    variant_problem_ids = load_variant_problem_sets(TARGET_STATUS)
    if not variant_problem_ids:
        raise ValueError(f"No variant/problem pairs were found in {INPUT_CSV} for status={TARGET_STATUS!r}")

    write_variant_counts(variant_problem_ids, output_paths["variant_counts_csv"], TARGET_STATUS)
    write_membership_matrix(variant_problem_ids, output_paths["membership_csv"])
    intersection_summary = write_intersections(variant_problem_ids, output_paths["intersections_csv"])
    venn_status = save_venn_plot(variant_problem_ids, output_paths["venn_png"], TARGET_STATUS)

    summary_lines = [
        f"INPUT_CSV: {INPUT_CSV}",
        f"STATUS_FILTER: {TARGET_STATUS}",
        f"Variants found: {len(variant_problem_ids)}",
        "",
        f"Per-variant {TARGET_STATUS} counts:",
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
    summary_lines.append(f"VARIANT_COUNTS_CSV: {output_paths['variant_counts_csv']}")
    summary_lines.append(f"INTERSECTIONS_CSV: {output_paths['intersections_csv']}")
    summary_lines.append(f"MEMBERSHIP_CSV: {output_paths['membership_csv']}")
    summary_lines.append(venn_status)

    output_paths["summary_txt"].write_text("\n".join(summary_lines) + "\n", encoding="utf-8")

    print(f"Summary written to: {output_paths['summary_txt']}")
    print(f"Variant counts CSV: {output_paths['variant_counts_csv']}")
    print(f"Intersections CSV: {output_paths['intersections_csv']}")
    print(f"Membership matrix CSV: {output_paths['membership_csv']}")
    print(venn_status)


if __name__ == "__main__":
    main()
