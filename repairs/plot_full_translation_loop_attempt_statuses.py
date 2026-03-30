#!/usr/bin/env python3
from __future__ import annotations

import csv
from pathlib import Path

import matplotlib.pyplot as plt

PIPELINE_NAME = "translation_with_old_prompt_using_GPT_4O_mini"
INPUT_CSV = Path(f"./repairs/{PIPELINE_NAME}_analysis/old_prompt/attempt_status_summary.csv")
OUTPUT_PNG = INPUT_CSV.with_name("attempt_status_dashboard.png")
OUTPUT_PDF = INPUT_CSV.with_name("attempt_status_dashboard.pdf")
OUTPUT_TXT = INPUT_CSV.with_name("attempt_status_table.txt")
FIGURE_WIDTH = 9
FIGURE_HEIGHT_PER_PROFILE = 4.5
MIN_FIGURE_HEIGHT = 4
BASE_FONT_SIZE = 16
BAR_WIDTH = 0.4
LEGEND_COLUMNS = 3
STATUS_SERIES = [
    ("pass_count", "PASS", "#2f7d4a"),
    ("fail_count", "FAIL", "#d1495b"),
    ("runtime_error_count", "RUNTIME_ERROR", "#f08a24"),
    ("timeout_count", "TIMEOUT", "#7a6ff0"),
    ("compile_failed_count", "COMPILE_FAILED", "#4c78a8"),
]
TERMINAL_CARRY_COLUMNS = {"pass_count"}


def load_rows() -> list[dict[str, str]]:
    if not INPUT_CSV.is_file():
        raise FileNotFoundError(f"Input CSV not found: {INPUT_CSV}")
    with INPUT_CSV.open("r", newline="", encoding="utf-8") as csv_file:
        return list(csv.DictReader(csv_file))


def build_cumulative_rows(profile_rows: list[dict[str, str]]) -> list[dict[str, int]]:
    cumulative_rows: list[dict[str, int]] = []
    carried_totals = {column: 0 for column, _, _ in STATUS_SERIES}

    for row in profile_rows:
        cumulative_row: dict[str, int] = {"attempt": int(row["attempt"])}
        for column, _, _ in STATUS_SERIES:
            current_value = int(row.get(column, "0"))
            if column in TERMINAL_CARRY_COLUMNS:
                cumulative_row[column] = carried_totals[column] + current_value
            else:
                cumulative_row[column] = current_value
        cumulative_row["total"] = sum(cumulative_row[column] for column, _, _ in STATUS_SERIES)
        cumulative_rows.append(cumulative_row)

        for column in TERMINAL_CARRY_COLUMNS:
            carried_totals[column] = cumulative_row[column]

    return cumulative_rows


def main() -> None:
    rows = load_rows()
    profiles = sorted({row["profile"] for row in rows})
    plt.rcParams.update({"font.size": BASE_FONT_SIZE})
    figure_height = max(MIN_FIGURE_HEIGHT, FIGURE_HEIGHT_PER_PROFILE * len(profiles))
    fig, axes = plt.subplots(len(profiles), 1, figsize=(FIGURE_WIDTH, figure_height), squeeze=False)
    table_lines: list[str] = []
    legend_handles = None
    legend_labels = None

    for idx, profile in enumerate(profiles):
        profile_rows = sorted((row for row in rows if row["profile"] == profile), key=lambda row: int(row["attempt"]))
        cumulative_rows = build_cumulative_rows(profile_rows)
        labels = [f"k={row['attempt']}" for row in cumulative_rows]
        ax = axes[idx][0]
        bottoms = [0] * len(cumulative_rows)

        for column, label, color in STATUS_SERIES:
            values = [row[column] for row in cumulative_rows]
            ax.bar(
                labels,
                values,
                width=BAR_WIDTH,
                bottom=bottoms,
                label=label,
                color=color,
                edgecolor="white",
                linewidth=0.7,
            )
            for i, value in enumerate(values):
                if value > 0:
                    ax.text(
                        i,
                        bottoms[i] + value / 2,
                        str(value),
                        ha="center",
                        va="center",
                        fontsize=max(BASE_FONT_SIZE - 2, 1),
                        color="white",
                    )
            bottoms = [bottom + value for bottom, value in zip(bottoms, values)]

        if idx == 0:
            legend_handles, legend_labels = ax.get_legend_handles_labels()

        ax.set_title(f"{profile}")
        ax.set_ylabel("Count")
        ax.grid(axis="y", linestyle="--", alpha=0.3)

        table_lines.append(profile)
        table_lines.append(
            "attempt,cumulative_total,PASS,FAIL,RUNTIME_ERROR,TIMEOUT,COMPILE_FAILED"
        )
        for row in cumulative_rows:
            table_lines.append(
                ",".join(
                    [
                        str(row["attempt"]),
                        str(row["total"]),
                        str(row["pass_count"]),
                        str(row["fail_count"]),
                        str(row["runtime_error_count"]),
                        str(row["timeout_count"]),
                        str(row["compile_failed_count"]),
                    ]
                )
            )
        table_lines.append("")

    if legend_handles and legend_labels:
        fig.legend(
            legend_handles,
            legend_labels,
            loc="upper center",
            bbox_to_anchor=(0.5, 0.99),
            ncol=LEGEND_COLUMNS,
            frameon=False,
            fontsize=max(BASE_FONT_SIZE - 2, 1),
        )
    fig.tight_layout(rect=(0, 0, 1, 0.94))
    fig.savefig(OUTPUT_PNG, dpi=200)
    fig.savefig(OUTPUT_PDF)
    plt.close(fig)
    OUTPUT_TXT.write_text("\n".join(table_lines) + "\n", encoding="utf-8")
    print(f"Dashboard written to: {OUTPUT_PNG}")
    print(f"Dashboard written to: {OUTPUT_PDF}")
    print(f"Table written to: {OUTPUT_TXT}")


if __name__ == "__main__":
    main()
