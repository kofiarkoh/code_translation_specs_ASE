#!/usr/bin/env python3
from __future__ import annotations

import csv
import html
import subprocess
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt@1"
ANALYSIS_SUBDIR = "rust-code-analysis"
PIPELINE_NAMES = [
    # "full_translation_loop",
    "full_translation_loop_GPT_5_2",
    "translation_with_validated_specs_GPT_5_2",
    "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS",
]
PIPELINE_LABEL_MAP = {
    "full_translation_loop": "Translations with GPT 4o-mini",
    "full_translation_loop_GPT_5_2": "Translation with GPT-5.2 using GPT 4o-mini generated specs",
    "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS": "Translation with GPT-5.2 using GPT-5.2 generated specs",
    "translation_with_validated_specs_GPT_5_2": "Translation With Validated Specs using GPT-5.2",
}
PROFILE_COLUMNS = [
    ("specs_and_code", "Specs+Code"),
    ("specs_only", "Specs Only"),
    ("code_only", "Code Only"),
]

OUTPUT_ROOT = PROJECT_ROOT / "repairs" / "multi_pipeline_metric_averages" / DATASET_NAME / ANALYSIS_SUBDIR
OUTPUT_CSV = OUTPUT_ROOT / "metrics_comparison_averages_multi_pipeline.csv"
OUTPUT_PNG = OUTPUT_ROOT / "metrics_comparison_averages_multi_pipeline.png"
OUTPUT_SVG = OUTPUT_ROOT / "metrics_comparison_averages_multi_pipeline.svg"
COUNTS_CSV = OUTPUT_ROOT / "metrics_comparison_program_counts_multi_pipeline.csv"
COUNTS_TXT = OUTPUT_ROOT / "metrics_comparison_program_counts_multi_pipeline.txt"

METRIC_FIELDS = [
    ("MI (original)", ("mi", "mi_original")),
    ("MI (SEI)", ("mi", "mi_sei")),
    ("Cyclomatic (sum)", ("cyclomatic", "sum")),
    ("Cognitive (sum)", ("cognitive", "sum")),
    ("Halstead Volume", ("halstead", "volume")),
    ("Halstead Effort", ("halstead", "effort")),
    ("SLOC", ("loc", "sloc")),
    ("PLOC", ("loc", "ploc")),
    ("LLOC", ("loc", "lloc")),
    ("Comments (CLOC)", ("loc", "cloc")),
    ("Blank Lines", ("loc", "blank")),
    ("Functions", ("nom", "functions")),
    ("Closures", ("nom", "closures")),
    ("Exits", ("nexits", "sum")),
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


def fmt_num(value: float | None) -> str:
    if value is None:
        return "N/A"
    return f"{value:.2f}"


def input_csv_for_pipeline(pipeline_name: str) -> Path:
    return (
        PROJECT_ROOT
        / "repairs"
        / f"{pipeline_name}_analysis"
        / DATASET_NAME
        / ANALYSIS_SUBDIR
        / "metrics_comparison.csv"
    )


def pass_counts_csv_for_pipeline(pipeline_name: str) -> Path:
    return (
        PROJECT_ROOT
        / "repairs"
        / f"{pipeline_name}_analysis"
        / DATASET_NAME
        / "pass_venn_analysis"
        / "profile_pass_counts.csv"
    )


def load_average_rows(input_csv: Path) -> dict[str, dict[str, str]]:
    if not input_csv.is_file():
        raise FileNotFoundError(f"Input CSV not found: {input_csv}")

    by_metric: dict[str, dict[str, str]] = {}
    with input_csv.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            if row.get("folder", "").strip() != "__AVERAGE__":
                continue
            metric = row.get("metric", "").strip()
            if metric:
                by_metric[metric] = row
    return by_metric


def load_program_counts(pipeline_name: str) -> dict[str, int]:
    input_csv = pass_counts_csv_for_pipeline(pipeline_name)
    if not input_csv.is_file():
        raise FileNotFoundError(f"Pass-counts CSV not found: {input_csv}")

    profile_name_map = {
        "code_and_spec": "specs_and_code",
        "specs_only": "specs_only",
        "code_only": "code_only",
    }
    counts = {profile_key: 0 for profile_key, _ in PROFILE_COLUMNS}

    with input_csv.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            source_profile = row.get("profile", "").strip()
            target_profile = profile_name_map.get(source_profile)
            if target_profile is None:
                continue
            counts[target_profile] = int(row.get("pass_problem_count", "0"))

    return counts


def pick_best_column(metric: str, values: dict[str, float]) -> str:
    higher_is_better = METRIC_BETTER_HIGHER[metric]
    ordered = sorted(
        values.items(),
        key=lambda item: (-item[1], item[0]) if higher_is_better else (item[1], item[0]),
    )
    return ordered[0][0]


def build_rows() -> tuple[list[dict[str, str]], list[tuple[str, str]]]:
    pipeline_average_rows = {
        pipeline_name: load_average_rows(input_csv_for_pipeline(pipeline_name))
        for pipeline_name in PIPELINE_NAMES
    }

    value_columns: list[tuple[str, str]] = []
    for pipeline_name in PIPELINE_NAMES:
        pipeline_label = PIPELINE_LABEL_MAP.get(pipeline_name, pipeline_name)
        for profile_key, profile_label in PROFILE_COLUMNS:
            value_columns.append((f"{pipeline_label} | {profile_label}", profile_key))

    rows: list[dict[str, str]] = []
    for metric, _ in METRIC_FIELDS:
        row_out = {
            "metric": metric,
            "preferred_direction": "Higher" if METRIC_BETTER_HIGHER[metric] else "Lower",
        }
        comparable_values: dict[str, float] = {}

        for pipeline_name in PIPELINE_NAMES:
            source_row = pipeline_average_rows[pipeline_name].get(metric)
            pipeline_label = PIPELINE_LABEL_MAP.get(pipeline_name, pipeline_name)
            for column_label, profile_key in value_columns:
                if not column_label.startswith(f"{pipeline_label} | "):
                    continue
                value = None
                if source_row is not None and source_row.get(profile_key, "").strip():
                    value = float(source_row[profile_key])
                    comparable_values[column_label] = value
                row_out[column_label] = fmt_num(value)

        row_out["best_column"] = pick_best_column(metric, comparable_values) if comparable_values else ""
        rows.append(row_out)

    return rows, value_columns


def write_csv(rows: list[dict[str, str]], value_columns: list[tuple[str, str]]) -> None:
    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    fieldnames = ["metric", "preferred_direction", "best_column", *[label for label, _ in value_columns]]
    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(csv_file, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def write_svg(rows: list[dict[str, str]], value_columns: list[tuple[str, str]]) -> None:
    headers = ["Metric", "Better", *[label for label, _ in value_columns]]
    column_widths = [260, 90, *([130] * len(value_columns))]
    total_width = sum(column_widths)
    row_height = 34
    title_height = 48
    footer_height = 24
    total_height = title_height + row_height * (len(rows) + 1) + footer_height

    x_positions = [0]
    for width in column_widths[:-1]:
        x_positions.append(x_positions[-1] + width)

    lines: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="{total_height}" viewBox="0 0 {total_width} {total_height}">',
        '<rect width="100%" height="100%" fill="#ffffff"/>',
        f'<text x="{total_width / 2}" y="28" text-anchor="middle" font-family="Helvetica" font-size="20" font-weight="700" fill="#1E2A3A">Multi-Pipeline Average Metrics</text>',
    ]

    top_y = title_height
    for col_index, header in enumerate(headers):
        x = x_positions[col_index]
        width = column_widths[col_index]
        lines.append(
            f'<rect x="{x}" y="{top_y}" width="{width}" height="{row_height}" fill="#DCE8F5" stroke="#B8C4D6" stroke-width="1"/>'
        )
        text_x = x + 8 if col_index == 0 else x + width / 2
        anchor = "start" if col_index == 0 else "middle"
        lines.append(
            f'<text x="{text_x}" y="{top_y + 22}" text-anchor="{anchor}" font-family="Helvetica" font-size="12" font-weight="700" fill="#1E2A3A">{html.escape(header)}</text>'
        )

    for row_index, row in enumerate(rows, start=1):
        y = title_height + row_height * row_index
        bg = "#F8FBFF" if row_index % 2 == 1 else "#EDF4FA"
        best_column = row["best_column"]
        rendered_values = [row["metric"], row["preferred_direction"], *[row[label] for label, _ in value_columns]]

        for col_index, value in enumerate(rendered_values):
            x = x_positions[col_index]
            width = column_widths[col_index]
            cell_bg = bg
            if col_index >= 2 and headers[col_index] == best_column:
                cell_bg = "#D7F0DD"
            lines.append(
                f'<rect x="{x}" y="{y}" width="{width}" height="{row_height}" fill="{cell_bg}" stroke="#B8C4D6" stroke-width="1"/>'
            )
            text_x = x + 8 if col_index == 0 else x + width / 2
            anchor = "start" if col_index == 0 else "middle"
            weight = "700" if col_index == 0 or cell_bg == "#D7F0DD" else "400"
            color = "#0B6E4F" if cell_bg == "#D7F0DD" else "#243447"
            lines.append(
                f'<text x="{text_x}" y="{y + 22}" text-anchor="{anchor}" font-family="Helvetica" font-size="12" font-weight="{weight}" fill="{color}">{html.escape(value)}</text>'
            )

    lines.append("</svg>")
    OUTPUT_SVG.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_counts_report(pipeline_counts: dict[str, dict[str, int]]) -> None:
    with COUNTS_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["pipeline_name", "pipeline_label", "variant", "program_count"])
        for pipeline_name in PIPELINE_NAMES:
            pipeline_label = PIPELINE_LABEL_MAP.get(pipeline_name, pipeline_name)
            counts = pipeline_counts[pipeline_name]
            for profile_key, profile_label in PROFILE_COLUMNS:
                writer.writerow([pipeline_name, pipeline_label, profile_label, counts.get(profile_key, 0)])

    lines: list[str] = []
    for pipeline_name in PIPELINE_NAMES:
        pipeline_label = PIPELINE_LABEL_MAP.get(pipeline_name, pipeline_name)
        counts = pipeline_counts[pipeline_name]
        lines.append(pipeline_label)
        for profile_key, profile_label in PROFILE_COLUMNS:
            lines.append(f"{profile_label}: {counts.get(profile_key, 0)}")
        lines.append("")
    COUNTS_TXT.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_png_from_svg() -> None:
    completed = subprocess.run(
        ["/usr/bin/sips", "-s", "format", "png", str(OUTPUT_SVG), "--out", str(OUTPUT_PNG)],
        capture_output=True,
        text=True,
        check=False,
    )
    if completed.returncode != 0:
        raise RuntimeError(f"sips failed to convert SVG to PNG: {completed.stderr.strip()}")


def main() -> None:
    pipeline_counts = {
        pipeline_name: load_program_counts(pipeline_name)
        for pipeline_name in PIPELINE_NAMES
    }
    rows, value_columns = build_rows()
    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)
    write_csv(rows, value_columns)
    write_counts_report(pipeline_counts)
    write_svg(rows, value_columns)
    try:
        write_png_from_svg()
    finally:
        if OUTPUT_SVG.exists():
            OUTPUT_SVG.unlink()

    print("Pipelines:")
    for pipeline_name in PIPELINE_NAMES:
        print(pipeline_name)
    print(f"Rows written: {len(rows)}")
    print(f"PNG written to: {OUTPUT_PNG}")
    print(f"CSV written to: {OUTPUT_CSV}")
    print(f"Counts CSV written to: {COUNTS_CSV}")
    print(f"Counts summary written to: {COUNTS_TXT}")


if __name__ == "__main__":
    main()
