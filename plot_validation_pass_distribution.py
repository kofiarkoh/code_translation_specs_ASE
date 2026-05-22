#!/usr/bin/env python3
"""
Plot the distribution of spec validation outcomes across attempt levels.

Reads validation_summary.csv and produces two panels:
  - Stacked bar: count of each final_status per attempt number
  - Cumulative PASS line: how many problems have passed by each attempt

Usage:
    python plot_validation_pass_distribution.py
"""

from __future__ import annotations

from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import pandas as pd

# ============================================
# Configure here
# ============================================
CSV_PATH = Path(
    "api_responses/open_ai/optimized_prompt@1/C/"
    "validated_specs_WITH_GPT_gpt-4o-mini/validation_summary.csv"
)
# CSV_PATH = Path(
#     "api_responses/open_ai/optimized_prompt@1/C/"
#     "validated_specs_WITH_GPT_gpt-5.2/validation_summary.csv"
# )
OUTPUT_FILE = Path("validation_pass_distribution_gpt-4o-mini.png")
TITLE = "Spec Validation — Pass Distribution by Attempt (gpt-4o-mini)"

STATUS_COLORS: dict[str, str] = {
    "PASS":           "#55A868",
    "FAIL":           "#C44E52",
    "COMPILE_FAILED": "#DD8452",
    "RUNTIME_ERROR":  "#8172B2",
    "TIMEOUT":        "#937860",
    "NO_SAMPLES":     "#CCCCCC",
}
STATUS_ORDER = ["PASS", "FAIL", "COMPILE_FAILED", "RUNTIME_ERROR", "TIMEOUT", "NO_SAMPLES"]


def main() -> None:
    if not CSV_PATH.exists():
        raise FileNotFoundError(f"CSV not found: {CSV_PATH}")

    df = pd.read_csv(CSV_PATH)
    total = len(df)

    # Count per (attempts_used, final_status)
    counts = (
        df.groupby(["attempts_used", "final_status"])
        .size()
        .reset_index(name="count")
    )

    attempts = sorted(df["attempts_used"].unique())
    statuses = [s for s in STATUS_ORDER if s in df["final_status"].unique()]

    # Cumulative passes: problems that passed at attempt <= k
    cum_pass = []
    running = 0
    for a in attempts:
        passed_at_a = len(df[(df["attempts_used"] == a) & (df["final_status"] == "PASS")])
        running += passed_at_a
        cum_pass.append(running)

    fig, (ax_bar, ax_cum) = plt.subplots(
        1, 2, figsize=(14, 6), gridspec_kw={"width_ratios": [2, 1]}
    )
    fig.suptitle(TITLE, fontsize=16, fontweight="bold")

    # ── Stacked bar ──────────────────────────────────────────────────────────
    bottoms = [0] * len(attempts)
    bars_by_status: dict[str, list] = {}

    for status in statuses:
        vals = []
        for a in attempts:
            row = counts[(counts["attempts_used"] == a) & (counts["final_status"] == status)]
            vals.append(int(row["count"].values[0]) if not row.empty else 0)

        bars = ax_bar.bar(
            [str(a) for a in attempts],
            vals,
            bottom=bottoms,
            label=status,
            color=STATUS_COLORS.get(status, "#999999"),
            width=0.5,
        )
        bars_by_status[status] = (bars, vals)
        bottoms = [b + v for b, v in zip(bottoms, vals)]

    # Value labels inside bars
    for status, (bars, vals) in bars_by_status.items():
        for bar, val in zip(bars, vals):
            if val == 0:
                continue
            ax_bar.text(
                bar.get_x() + bar.get_width() / 2,
                bar.get_y() + bar.get_height() / 2,
                str(val),
                ha="center", va="center",
                fontsize=13, fontweight="bold", color="white",
            )

    ax_bar.set_xlabel("Attempt Number", fontsize=14)
    ax_bar.set_ylabel("Number of Problems", fontsize=14)
    ax_bar.set_title("Status Distribution per Attempt", fontsize=14)
    ax_bar.tick_params(labelsize=13)
    ax_bar.legend(fontsize=12, title="Status", title_fontsize=12)
    ax_bar.yaxis.set_major_locator(mticker.MaxNLocator(integer=True))

    # ── Cumulative PASS line ─────────────────────────────────────────────────
    ax_cum.plot(
        [str(a) for a in attempts],
        cum_pass,
        marker="o", markersize=10, linewidth=2.5,
        color=STATUS_COLORS["PASS"], label="Cumulative PASS",
    )

    for a, cp in zip(attempts, cum_pass):
        ax_cum.annotate(
            f"{cp}\n({cp/total*100:.1f}%)",
            xy=(str(a), cp),
            xytext=(0, 14), textcoords="offset points",
            ha="center", fontsize=13, fontweight="bold",
        )

    ax_cum.set_xlabel("Attempt Number", fontsize=14)
    ax_cum.set_ylabel("Cumulative Problems Passed", fontsize=14)
    ax_cum.set_title("Cumulative PASS", fontsize=14)
    ax_cum.tick_params(labelsize=13)
    ax_cum.set_ylim(0, total * 1.1)
    ax_cum.axhline(total, linestyle="--", color="#AAAAAA", linewidth=1)
    ax_cum.text(
        len(attempts) - 1, total * 1.02,
        f"Total: {total}", ha="right", fontsize=11, color="#888888",
    )
    ax_cum.yaxis.set_major_locator(mticker.MaxNLocator(integer=True))

    fig.tight_layout()
    fig.savefig(OUTPUT_FILE, dpi=150)
    plt.close(fig)
    print(f"Saved: {OUTPUT_FILE}")

    # Print summary
    print(f"\nTotal problems : {total}")
    for a, cp in zip(attempts, cum_pass):
        print(f"  After attempt {a}: {cp} passed ({cp/total*100:.1f}%)")
    print(f"  Final non-pass : {total - cum_pass[-1]}")


if __name__ == "__main__":
    main()
