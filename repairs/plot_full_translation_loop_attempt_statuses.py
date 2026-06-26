#!/usr/bin/env python3
from __future__ import annotations

import csv
from pathlib import Path

import matplotlib.pyplot as plt

PIPELINE_NAME = "full_translation_loop"
SCRIPT_DIR = Path(__file__).resolve().parent
INPUT_CSV = SCRIPT_DIR / f"{PIPELINE_NAME}_analysis" / "optimized_prompt@1" / "attempt_status_summary.csv"
OUTPUT_PNG = INPUT_CSV.with_name("attempt_status_dashboard.png")
OUTPUT_PDF = INPUT_CSV.with_name("attempt_status_dashboard.pdf")
OUTPUT_TXT = INPUT_CSV.with_name("attempt_status_table.txt")
FIGURE_WIDTH = 14
FIGURE_HEIGHT = 7
BASE_FONT_SIZE = 18
BAR_WIDTH = 0.25
LEGEND_COLUMNS = 3

PROFILE_LABELS: dict[str, str] = {
    "code_only":     "Code Only",
    "specs_only":    "Specs Only",
    "code_and_spec": "Code & Specs",
}
STATUS_SERIES = [
    ("pass_count", "PASS", "#009E73"),
    ("fail_count", "FAIL", "#D55E00"),
    ("runtime_error_count", "RUNTIME_ERROR", "#E69F00"),
    ("timeout_count", "TIMEOUT", "#0072B2"),
    ("compile_failed_count", "COMPILE_FAILED", "#CC79A7"),
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
    import numpy as np

    rows = load_rows()
    profiles = sorted({row["profile"] for row in rows})
    attempts = sorted({int(row["attempt"]) for row in rows})
    plt.rcParams.update({"font.size": BASE_FONT_SIZE})

    # Build cumulative data per profile
    profile_data: dict[str, list[dict[str, int]]] = {}
    for profile in profiles:
        profile_rows = sorted(
            (row for row in rows if row["profile"] == profile),
            key=lambda row: int(row["attempt"]),
        )
        profile_data[profile] = build_cumulative_rows(profile_rows)

    n_attempts = len(attempts)
    n_profiles = len(profiles)
    x = np.arange(n_attempts)
    offsets = [(i - (n_profiles - 1) / 2) * BAR_WIDTH for i in range(n_profiles)]

    fig, ax = plt.subplots(figsize=(FIGURE_WIDTH, FIGURE_HEIGHT))
    table_lines: list[str] = []

    for offset, profile in zip(offsets, profiles):
        cum_rows = profile_data[profile]
        bottoms = [0] * n_attempts

        for col_idx, (column, status_label, color) in enumerate(STATUS_SERIES):
            values = [row[column] for row in cum_rows]
            label = status_label if profile == profiles[0] else None
            ax.bar(
                x + offset,
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
                        x[i] + offset,
                        bottoms[i] + value / 2,
                        str(value),
                        ha="center",
                        va="center",
                        fontsize=BASE_FONT_SIZE,
                        color="white",
                    )
            bottoms = [b + v for b, v in zip(bottoms, values)]

        table_lines.append(profile)
        table_lines.append(
            "attempt,cumulative_total,PASS,FAIL,RUNTIME_ERROR,TIMEOUT,COMPILE_FAILED"
        )
        for row in cum_rows:
            table_lines.append(
                ",".join(
                    str(row[k]) for k in
                    ["attempt", "total", "pass_count", "fail_count",
                     "runtime_error_count", "timeout_count", "compile_failed_count"]
                )
            )
        table_lines.append("")

    # Profile labels under each bar (minor ticks), k=N labels per group (major ticks)
    tick_positions = []
    tick_labels = []
    for i in range(n_attempts):
        for offset, profile in zip(offsets, profiles):
            tick_positions.append(x[i] + offset)
            tick_labels.append(PROFILE_LABELS.get(profile, profile))

    ax.set_xticks(tick_positions)
    ax.set_xticklabels(tick_labels, fontsize=max(BASE_FONT_SIZE - 6, 1))

    # Add k=N group labels below using secondary x-axis
    ax2 = ax.secondary_xaxis("bottom")
    ax2.set_xticks(list(x))
    ax2.set_xticklabels([f"k={a}" for a in attempts], fontsize=BASE_FONT_SIZE)
    ax2.tick_params(length=0, pad=25)

    ax.set_yscale("symlog", linthresh=1)
    ax.set_ylabel("Count (log scale)")
    ax.grid(axis="y", linestyle="--", alpha=0.3)
    ax.legend(
        loc="upper center",
        bbox_to_anchor=(0.5, 1.15),
        ncol=LEGEND_COLUMNS,
        frameon=False,
        fontsize=max(BASE_FONT_SIZE - 2, 1),
    )

    fig.tight_layout()
    fig.savefig(OUTPUT_PNG, dpi=200)
    fig.savefig(OUTPUT_PDF)
    plt.close(fig)
    OUTPUT_TXT.write_text("\n".join(table_lines) + "\n", encoding="utf-8")
    print(f"Dashboard written to: {OUTPUT_PNG}")
    print(f"Dashboard written to: {OUTPUT_PDF}")
    print(f"Table written to: {OUTPUT_TXT}")


if __name__ == "__main__":
    main()
