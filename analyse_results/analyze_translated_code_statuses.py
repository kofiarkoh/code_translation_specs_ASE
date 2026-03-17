#!/usr/bin/env python3
from __future__ import annotations

import csv
import os
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
from dotenv import load_dotenv
load_dotenv()

# =========================
# Configure these values
# =========================
PROJECT_ROOT = Path(__file__).resolve().parent.parent
REPORT_ROOT = PROJECT_ROOT / "results"/ Path(os.environ.get("DATASET_NAME")) / "rust_run_reports"
RUN_SUMMARY_CSV = REPORT_ROOT / "run_summary.csv"
INPUT_CSV = RUN_SUMMARY_CSV


def get_output_dir() -> Path:
    return PROJECT_ROOT / "results"/ Path(os.environ.get("DATASET_NAME")) / "translated_code_execution_statuses_output"


def variant_name_from_path(path_text: str) -> str:
    path = Path(path_text.strip())
    return path.name if path.name else "unknown"


def plot_status_counts(
    variant_status_problem_ids: dict[str, dict[str, set[str]]],
    all_variants: list[str],
    all_statuses: list[str],
    status_plot_png: Path,
) -> None:
    if not all_variants or not all_statuses:
        return

    x_positions = list(range(len(all_variants)))
    total_statuses = len(all_statuses)
    bar_width = 0.8 / max(total_statuses, 1)

    fig, ax = plt.subplots(figsize=(max(10, len(all_variants) * 2.2), 6))

    for idx, status in enumerate(all_statuses):
        counts = [len(variant_status_problem_ids[variant].get(status, set())) for variant in all_variants]
        offset = (idx - (total_statuses - 1) / 2) * bar_width
        bars = ax.bar(
            [x + offset for x in x_positions],
            counts,
            width=bar_width,
            label=status,
        )
        for bar, count in zip(bars, counts):
            ax.text(
                bar.get_x() + bar.get_width() / 2,
                bar.get_height(),
                str(count),
                ha="center",
                va="bottom",
                fontsize=8,
            )

    ax.set_title("Unique Problem IDs by Rust File and Status")
    ax.set_xlabel("Rust File")
    ax.set_ylabel("Unique Problem IDs")
    ax.set_xticks(x_positions)
    ax.set_xticklabels(all_variants, rotation=20, ha="right")
    ax.legend(title="Status")
    ax.grid(axis="y", linestyle="--", alpha=0.3)
    fig.tight_layout()
    fig.savefig(status_plot_png, dpi=200)
    plt.close(fig)


def main() -> None:
    if not INPUT_CSV.is_file():
        raise FileNotFoundError(f"INPUT_CSV does not exist: {INPUT_CSV}")

    output_dir = get_output_dir()
    summary_txt = output_dir / "status_summary.txt"
    unique_id_status_by_variant_csv = output_dir / "unique_id_status_by_variant.csv"
    problem_status_matrix_csv = output_dir / "problem_status_matrix.csv"
    non_pass_problems_csv = output_dir / "non_pass_problems.csv"
    status_plot_png = output_dir / "unique_id_status_by_variant.png"

    output_dir.mkdir(parents=True, exist_ok=True)

    variant_status_problem_ids: dict[str, dict[str, set[str]]] = defaultdict(lambda: defaultdict(set))
    problem_statuses: dict[str, dict[str, str]] = defaultdict(dict)
    row_count = 0

    with INPUT_CSV.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            row_count += 1
            problem_id = row.get("problem_id", "").strip()
            rust_file = row.get("rust_file", "").strip()
            status = row.get("status", "").strip() or "UNKNOWN"
            if not problem_id:
                continue

            variant = variant_name_from_path(rust_file)
            variant_status_problem_ids[variant][status].add(problem_id)
            problem_statuses[problem_id][variant] = status

    all_variants = sorted(variant_status_problem_ids)
    all_statuses = sorted({status for status_map in variant_status_problem_ids.values() for status in status_map})

    with unique_id_status_by_variant_csv.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["variant", "status", "unique_problem_ids"])
        for variant in all_variants:
            for status in all_statuses:
                writer.writerow([variant, status, len(variant_status_problem_ids[variant].get(status, set()))])

    with problem_status_matrix_csv.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["problem_id", *all_variants])
        for problem_id in sorted(problem_statuses):
            writer.writerow([
                problem_id,
                *[problem_statuses[problem_id].get(variant, "") for variant in all_variants],
            ])

    non_pass_rows: list[dict[str, str]] = []
    for problem_id in sorted(problem_statuses):
        for variant in all_variants:
            status = problem_statuses[problem_id].get(variant, "")
            if status and status != "PASS":
                non_pass_rows.append({
                    "problem_id": problem_id,
                    "variant": variant,
                    "status": status,
                })

    with non_pass_problems_csv.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(csv_file, fieldnames=["problem_id", "variant", "status"])
        writer.writeheader()
        writer.writerows(non_pass_rows)

    plot_status_counts(variant_status_problem_ids, all_variants, all_statuses, status_plot_png)

    unique_problem_ids = sorted(problem_statuses)
    non_pass_problem_ids = sorted({row["problem_id"] for row in non_pass_rows})

    summary_lines = [
        f"INPUT_CSV: {INPUT_CSV}",
        f"Total rows read: {row_count}",
        f"Unique problem IDs: {len(unique_problem_ids)}",
        f"Problem IDs with any non-PASS status: {len(non_pass_problem_ids)}",
        "",
        "Unique problem IDs by variant and status:",
    ]

    print(f"all variants: {all_variants}")
    print(f"all statuses: {all_statuses}")
    for variant in all_variants:
        summary_lines.append(f"- {variant}")
        for status in all_statuses:
            count = len(variant_status_problem_ids[variant].get(status, set()))
            summary_lines.append(f"  {status}: {count}")

    summary_lines.append("")
    summary_lines.append(f"UNIQUE_ID_STATUS_BY_VARIANT_CSV: {unique_id_status_by_variant_csv}")
    summary_lines.append(f"PROBLEM_STATUS_MATRIX_CSV: {problem_status_matrix_csv}")
    summary_lines.append(f"NON_PASS_PROBLEMS_CSV: {non_pass_problems_csv}")
    summary_lines.append(f"STATUS_PLOT_PNG: {status_plot_png}")

    summary_txt.write_text("\n".join(summary_lines) + "\n", encoding="utf-8")

    print(f"Summary written to: {summary_txt}")
    print(f"Unique-id status CSV: {unique_id_status_by_variant_csv}")
    print(f"Problem status matrix CSV: {problem_status_matrix_csv}")
    print(f"Non-pass problems CSV: {non_pass_problems_csv}")
    print(f"Status plot: {status_plot_png}")


if __name__ == "__main__":
    main()
