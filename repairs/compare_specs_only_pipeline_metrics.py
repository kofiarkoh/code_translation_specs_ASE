#!/usr/bin/env python3
from __future__ import annotations

import csv
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt@1"
PIPELINE_A_ANALYSIS = "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS_analysis"
PIPELINE_B_ANALYSIS = "translation_with_validated_specs_GPT_5_2_analysis"
COMPARE_SUBDIR = "rust-code-analysis-all"
VARIANT_COLUMN = "spec_only"

PIPELINE_A_LABEL = PIPELINE_A_ANALYSIS.removesuffix("_analysis")
PIPELINE_B_LABEL = PIPELINE_B_ANALYSIS.removesuffix("_analysis")
INPUT_A_CSV = PROJECT_ROOT / "repairs" / PIPELINE_A_ANALYSIS / DATASET_NAME / COMPARE_SUBDIR / "metrics_comparison.csv"
INPUT_B_CSV = PROJECT_ROOT / "repairs" / PIPELINE_B_ANALYSIS / DATASET_NAME / COMPARE_SUBDIR / "metrics_comparison.csv"
OUTPUT_DIR = PROJECT_ROOT / "repairs" / f"{PIPELINE_A_LABEL}__vs__{PIPELINE_B_LABEL}_analysis" / DATASET_NAME / f"{VARIANT_COLUMN}_rust_code_analysis_all"
OUTPUT_MD = OUTPUT_DIR / f"metrics_comparison_{VARIANT_COLUMN}.md"
OUTPUT_CSV = OUTPUT_DIR / f"metrics_comparison_{VARIANT_COLUMN}.csv"

METRIC_ORDER = [
    "MI (original)",
    "MI (SEI)",
    "Cyclomatic (sum)",
    "Cognitive (sum)",
    "Halstead Volume",
    "Halstead Effort",
    "SLOC",
    "PLOC",
    "LLOC",
    "Comments (CLOC)",
    "Blank Lines",
    "Functions",
    "Closures",
    "Exits",
]

METRIC_BETTER_HIGHER = {
    "MI (original)": True,
    "MI (SEI)": True,
    "Cyclomatic (sum)": False,
    "Cognitive (sum)": False,
    "Halstead Volume": False,
    "Halstead Effort": False,
    "SLOC": False,
    "PLOC": False,
    "LLOC": False,
    "Comments (CLOC)": True,
    "Blank Lines": False,
    "Functions": False,
    "Closures": False,
    "Exits": False,
}


def fmt_num(value: float) -> str:
    if abs(value - round(value)) < 1e-12:
        return str(int(round(value)))
    return f"{value:.4f}"


def load_problem_metric_values(path: Path) -> dict[str, dict[str, float]]:
    if not path.is_file():
        raise FileNotFoundError(f"Input CSV not found: {path}")

    values_by_problem: dict[str, dict[str, float]] = {}
    with path.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        if reader.fieldnames is None or VARIANT_COLUMN not in reader.fieldnames:
            raise ValueError(
                f"Column {VARIANT_COLUMN!r} not found in {path}. "
                f"Available columns: {', '.join(reader.fieldnames or [])}"
            )
        for row in reader:
            folder = row.get("folder", "").strip()
            if not folder or folder == "__AVERAGE__":
                continue
            metric = row.get("metric", "").strip()
            raw_value = row.get(VARIANT_COLUMN, "").strip()
            if not metric or not raw_value:
                continue
            values_by_problem.setdefault(folder, {})[metric] = float(raw_value)
    return values_by_problem


def pick_better_pipeline(metric: str, a_value: float, b_value: float) -> str:
    higher_is_better = METRIC_BETTER_HIGHER[metric]
    if a_value == b_value:
        return "tie"
    if higher_is_better:
        return PIPELINE_A_LABEL if a_value > b_value else PIPELINE_B_LABEL
    return PIPELINE_A_LABEL if a_value < b_value else PIPELINE_B_LABEL


def build_rows() -> list[dict[str, str]]:
    values_a = load_problem_metric_values(INPUT_A_CSV)
    values_b = load_problem_metric_values(INPUT_B_CSV)
    rows: list[dict[str, str]] = []

    common_problems = sorted(set(values_a) & set(values_b))
    for problem_id in common_problems:
        metrics_a = values_a[problem_id]
        metrics_b = values_b[problem_id]
        for metric in METRIC_ORDER:
            if metric not in metrics_a or metric not in metrics_b:
                continue
            a_value = metrics_a[metric]
            b_value = metrics_b[metric]
            rows.append(
                {
                    "problem_id": problem_id,
                    "metric": metric,
                    "preferred_direction": "Higher" if METRIC_BETTER_HIGHER[metric] else "Lower",
                    PIPELINE_A_LABEL: fmt_num(a_value),
                    PIPELINE_B_LABEL: fmt_num(b_value),
                    "delta_a_minus_b": fmt_num(a_value - b_value),
                    "better_pipeline": pick_better_pipeline(metric, a_value, b_value),
                }
            )
    return rows


def build_average_rows(rows: list[dict[str, str]]) -> list[dict[str, str]]:
    grouped: dict[str, list[dict[str, str]]] = {}
    for row in rows:
        grouped.setdefault(row["metric"], []).append(row)

    average_rows: list[dict[str, str]] = []
    for metric in METRIC_ORDER:
        metric_rows = grouped.get(metric, [])
        if not metric_rows:
            continue

        a_values = [float(row[PIPELINE_A_LABEL]) for row in metric_rows]
        b_values = [float(row[PIPELINE_B_LABEL]) for row in metric_rows]
        a_avg = sum(a_values) / len(a_values)
        b_avg = sum(b_values) / len(b_values)
        average_rows.append(
            {
                "problem_id": "__AVERAGE__",
                "metric": metric,
                "preferred_direction": "Higher" if METRIC_BETTER_HIGHER[metric] else "Lower",
                PIPELINE_A_LABEL: fmt_num(a_avg),
                PIPELINE_B_LABEL: fmt_num(b_avg),
                "delta_a_minus_b": fmt_num(a_avg - b_avg),
                "better_pipeline": pick_better_pipeline(metric, a_avg, b_avg),
            }
        )

    return average_rows


def write_csv(rows: list[dict[str, str]]) -> None:
    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "problem_id",
                "metric",
                "preferred_direction",
                PIPELINE_A_LABEL,
                PIPELINE_B_LABEL,
                "delta_a_minus_b",
                "better_pipeline",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)


def write_markdown(rows: list[dict[str, str]]) -> None:
    grouped: dict[str, list[dict[str, str]]] = {}
    for row in rows:
        grouped.setdefault(row["problem_id"], []).append(row)
    average_rows = build_average_rows(rows)

    lines = [
        f"# {VARIANT_COLUMN} Rust Code Analysis Comparison",
        "",
        f"Pipeline A: `{PIPELINE_A_LABEL}`",
        f"Pipeline B: `{PIPELINE_B_LABEL}`",
        f"Variant column: `{VARIANT_COLUMN}`",
        f"Source A: `{INPUT_A_CSV}`",
        f"Source B: `{INPUT_B_CSV}`",
        "",
    ]

    lines.append("## Overall Averages")
    lines.append("")
    lines.append(
        f"| Metric | Better Direction | {PIPELINE_A_LABEL} | {PIPELINE_B_LABEL} | Delta (A-B) | Better Pipeline |"
    )
    lines.append("|---|---|---:|---:|---:|---|")
    for row in average_rows:
        pipeline_a_cell = row[PIPELINE_A_LABEL]
        pipeline_b_cell = row[PIPELINE_B_LABEL]
        if row["better_pipeline"] == PIPELINE_A_LABEL:
            pipeline_a_cell = f"**{pipeline_a_cell} ✅**"
        elif row["better_pipeline"] == PIPELINE_B_LABEL:
            pipeline_b_cell = f"**{pipeline_b_cell} ✅**"
        lines.append(
            f"| {row['metric']} | "
            f"{row['preferred_direction']} | "
            f"{pipeline_a_cell} | "
            f"{pipeline_b_cell} | "
            f"{row['delta_a_minus_b']} | "
            f"{row['better_pipeline']} |"
        )
    lines.append("")

    for problem_id in sorted(grouped):
        lines.append(f"## {problem_id}")
        lines.append("")
        lines.append(
            f"| Metric | Better Direction | {PIPELINE_A_LABEL} | {PIPELINE_B_LABEL} | Delta (A-B) | Better Pipeline |"
        )
        lines.append("|---|---|---:|---:|---:|---|")
        for row in grouped[problem_id]:
            pipeline_a_cell = row[PIPELINE_A_LABEL]
            pipeline_b_cell = row[PIPELINE_B_LABEL]
            if row["better_pipeline"] == PIPELINE_A_LABEL:
                pipeline_a_cell = f"**{pipeline_a_cell} ✅**"
            elif row["better_pipeline"] == PIPELINE_B_LABEL:
                pipeline_b_cell = f"**{pipeline_b_cell} ✅**"
            lines.append(
                f"| {row['metric']} | "
                f"{row['preferred_direction']} | "
                f"{pipeline_a_cell} | "
                f"{pipeline_b_cell} | "
                f"{row['delta_a_minus_b']} | "
                f"{row['better_pipeline']} |"
            )
        lines.append("")

    OUTPUT_MD.write_text("\n".join(lines) + "\n", encoding="utf-8")


def main() -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    rows = build_rows()
    write_csv(rows)
    write_markdown(rows)

    unique_problems = sorted({row["problem_id"] for row in rows})
    print(f"PIPELINE_A_CSV: {INPUT_A_CSV}")
    print(f"PIPELINE_B_CSV: {INPUT_B_CSV}")
    print(f"VARIANT_COLUMN: {VARIANT_COLUMN}")
    print(f"Problems compared: {len(unique_problems)}")
    print(f"Rows written: {len(rows)}")
    print(f"Markdown written to: {OUTPUT_MD}")
    print(f"CSV written to: {OUTPUT_CSV}")


if __name__ == "__main__":
    main()
