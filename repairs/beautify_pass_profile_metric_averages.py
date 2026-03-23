#!/usr/bin/env python3
from __future__ import annotations

import csv
import html
import subprocess
from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "full_translation_loop"
DATASET_NAME = "optimized_prompt@1"
ANALYSIS_ROOT = PROJECT_ROOT / "repairs" / f"{PIPELINE_NAME}_analysis" / DATASET_NAME / "rust-code-analysis"
INPUT_CSV = ANALYSIS_ROOT / "metrics_comparison.csv"
OUTPUT_CSV = ANALYSIS_ROOT / "metrics_comparison_averages_beautified.csv"
OUTPUT_PNG = ANALYSIS_ROOT / "metrics_comparison_averages_beautified.png"
OUTPUT_SVG = ANALYSIS_ROOT / "metrics_comparison_averages_beautified.svg"

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
    if abs(value - round(value)) < 1e-12:
        return str(int(round(value)))
    return f"{value:.4f}"


def load_average_rows() -> dict[str, dict[str, str]]:
    if not INPUT_CSV.is_file():
        raise FileNotFoundError(f"Input CSV not found: {INPUT_CSV}")

    by_metric: dict[str, dict[str, str]] = {}
    with INPUT_CSV.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            if row.get("folder", "").strip() != "__AVERAGE__":
                continue
            metric = row.get("metric", "").strip()
            if metric:
                by_metric[metric] = row
    return by_metric


def pick_best_profile(metric: str, values: dict[str, float]) -> str:
    higher_is_better = METRIC_BETTER_HIGHER[metric]
    ordered = sorted(
        values.items(),
        key=lambda item: (-item[1], item[0]) if higher_is_better else (item[1], item[0]),
    )
    return ordered[0][0]


def build_rows() -> list[dict[str, str]]:
    average_rows = load_average_rows()
    rows: list[dict[str, str]] = []

    for metric, _ in METRIC_FIELDS:
        source_row = average_rows.get(metric)
        if source_row is None:
            continue

        specs_and_code = float(source_row["specs_and_code"])
        specs_only = float(source_row["specs_only"])
        code_only = float(source_row["code_only"])

        label_to_value = {
            "specs_and_code": specs_and_code,
            "specs_only": specs_only,
            "code_only": code_only,
        }
        best_profile = pick_best_profile(metric, label_to_value)

        rows.append(
            {
                "metric": metric,
                "preferred_direction": "Higher" if METRIC_BETTER_HIGHER[metric] else "Lower",
                "best_profile": best_profile,
                "specs_and_code": fmt_num(specs_and_code),
                "specs_only": fmt_num(specs_only),
                "code_only": fmt_num(code_only),
            }
        )

    return rows


def write_csv(rows: list[dict[str, str]]) -> None:
    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.DictWriter(
            csv_file,
            fieldnames=[
                "metric",
                "preferred_direction",
                "best_profile",
                "specs_and_code",
                "specs_only",
                "code_only",
            ],
        )
        writer.writeheader()
        writer.writerows(rows)


def write_svg(rows: list[dict[str, str]]) -> None:
    headers = [
        "Metric",
        "Better",
        "Specs+Code",
        "Specs Only",
        "Code Only",
    ]
    cell_text = [
        [
            row["metric"],
            row["preferred_direction"],
            row["specs_and_code"],
            row["specs_only"],
            row["code_only"],
        ]
        for row in rows
    ]
    column_widths = [320, 90, 120, 120, 120]
    total_width = sum(column_widths)
    row_height = 34
    title_height = 48
    footer_height = 24
    total_height = title_height + row_height * (len(cell_text) + 1) + footer_height

    x_positions = [0]
    for width in column_widths[:-1]:
        x_positions.append(x_positions[-1] + width)

    lines: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{total_width}" height="{total_height}" viewBox="0 0 {total_width} {total_height}">',
        '<rect width="100%" height="100%" fill="#ffffff"/>',
        f'<text x="{total_width / 2}" y="28" text-anchor="middle" font-family="Helvetica" font-size="20" font-weight="700" fill="#1E2A3A">Average Metrics Table</text>',
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

    for row_index, row in enumerate(cell_text, start=1):
        y = title_height + row_height * row_index
        bg = "#F8FBFF" if row_index % 2 == 1 else "#EDF4FA"
        best_profile = rows[row_index - 1]["best_profile"]
        for col_index, value in enumerate(row):
            x = x_positions[col_index]
            width = column_widths[col_index]
            cell_bg = bg
            if (
                (best_profile == "specs_and_code" and col_index == 2)
                or (best_profile == "specs_only" and col_index == 3)
                or (best_profile == "code_only" and col_index == 4)
            ):
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
    ANALYSIS_ROOT.mkdir(parents=True, exist_ok=True)
    rows = build_rows()
    write_csv(rows)
    write_svg(rows)
    try:
        write_png_from_svg()
    finally:
        if OUTPUT_SVG.exists():
            OUTPUT_SVG.unlink()

    print(f"INPUT_CSV: {INPUT_CSV}")
    print(f"Rows written: {len(rows)}")
    print(f"PNG written to: {OUTPUT_PNG}")
    print(f"CSV written to: {OUTPUT_CSV}")


if __name__ == "__main__":
    main()
