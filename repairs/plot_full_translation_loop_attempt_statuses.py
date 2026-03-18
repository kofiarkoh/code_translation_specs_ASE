#!/usr/bin/env python3
from __future__ import annotations

import csv
from pathlib import Path

import matplotlib.pyplot as plt

INPUT_CSV = Path("./repairs/full_translation_loop_GPT_5_2_analysis/optimized_prompt@1/attempt_status_summary.csv")
OUTPUT_PNG = INPUT_CSV.with_name("attempt_status_dashboard.png")
OUTPUT_TXT = INPUT_CSV.with_name("attempt_status_table.txt")
STATUS_SERIES = [
    ("pass_count", "PASS", "#2f7d4a"),
    ("fail_count", "FAIL", "#d1495b"),
    ("runtime_error_count", "RUNTIME_ERROR", "#f08a24"),
    ("timeout_count", "TIMEOUT", "#7a6ff0"),
    ("compile_failed_count", "COMPILE_FAILED", "#4c78a8"),
    ("no_samples_count", "NO_SAMPLES", "#9aa0a6"),
    ("missing_report_count", "MISSING_REPORT", "#6c757d"),
]
TERMINAL_CARRY_COLUMNS = {"pass_count", "missing_report_count"}


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
    fig, axes = plt.subplots(len(profiles), 1, figsize=(13, max(4, 4.5 * len(profiles))), squeeze=False)
    table_lines: list[str] = []

    for idx, profile in enumerate(profiles):
        profile_rows = sorted((row for row in rows if row["profile"] == profile), key=lambda row: int(row["attempt"]))
        cumulative_rows = build_cumulative_rows(profile_rows)
        labels = [f"k={row['attempt']}" for row in cumulative_rows]
        ax = axes[idx][0]
        bottoms = [0] * len(cumulative_rows)

        for column, label, color in STATUS_SERIES:
            values = [row[column] for row in cumulative_rows]
            ax.bar(labels, values, bottom=bottoms, label=label, color=color, edgecolor="white", linewidth=0.7)
            for i, value in enumerate(values):
                if value > 0:
                    ax.text(i, bottoms[i] + value / 2, str(value), ha="center", va="center", fontsize=8, color="white")
            bottoms = [bottom + value for bottom, value in zip(bottoms, values)]

        ax.set_title(f"{profile} (cumulative)")
        ax.set_ylabel("Count")
        ax.grid(axis="y", linestyle="--", alpha=0.3)
        ax.legend(loc="upper right", fontsize=8)

        table_lines.append(profile)
        table_lines.append(
            "attempt,cumulative_total,PASS,FAIL,RUNTIME_ERROR,TIMEOUT,COMPILE_FAILED,NO_SAMPLES,MISSING_REPORT"
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
                        str(row["no_samples_count"]),
                        str(row["missing_report_count"]),
                    ]
                )
            )
        table_lines.append("")

    fig.suptitle("Full Translation Loop Status After Up To Attempt k", fontsize=16)
    fig.tight_layout()
    fig.savefig(OUTPUT_PNG, dpi=200)
    plt.close(fig)
    OUTPUT_TXT.write_text("\n".join(table_lines) + "\n", encoding="utf-8")
    print(f"Dashboard written to: {OUTPUT_PNG}")
    print(f"Table written to: {OUTPUT_TXT}")


if __name__ == "__main__":
    main()
