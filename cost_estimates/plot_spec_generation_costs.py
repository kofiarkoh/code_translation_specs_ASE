#!/usr/bin/env python3
"""
Plot spec generation cost estimates from spec_generation_cost_estimate_summary.csv
as a bar chart — one bar per directory, coloured by model.

Usage:
    python plot_spec_generation_costs.py
"""

from __future__ import annotations

from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import pandas as pd

# ============================================
# Configure here
# ============================================
SUMMARY_CSV = Path("spec_generation_cost_estimate_summary.csv")
OUTPUT_FILE = Path("spec_generation_cost.png")

DIRECTORY_LABELS: dict[str, str] = {
    "specs":                    "specs\n(gpt-4o-mini)",
    "specs_WITH_GPT_5_2":                    "specs\n(gpt-5.2)",
    "validated_specs_WITH_GPT_gpt-5.2":      "validated_specs\n(gpt-5.2)",
    "validated_specs_WITH_GPT_gpt-4o-mini":  "validated_specs\n(4o-mini)",
    # "validated_specs_WITH_GPT_gpt-5.2_failing_cases_from_specs_only": "validated_specs_5.2\nfailing_only",
}

MODEL_COLORS: dict[str, str] = {
    "gpt-5.2":      "#4C72B0",
    "gpt-4o-mini":  "#DD8452",
    "gpt-4o":       "#55A868",
    "gpt-4.1":      "#C44E52",
    "gpt-4.1-mini": "#8172B2",
}


def main() -> None:
    if not SUMMARY_CSV.exists():
        raise FileNotFoundError(f"Summary CSV not found: {SUMMARY_CSV}")

    df = pd.read_csv(SUMMARY_CSV)
    df["label"] = df["directory"].map(lambda d: DIRECTORY_LABELS.get(d, d))
    df["color"] = df["model"].map(lambda m: MODEL_COLORS.get(m, "#999999"))

    fig, ax = plt.subplots(figsize=(max(8, len(df) * 2), 7))

    bars = ax.bar(
        df["label"],
        df["cost_usd"],
        color=df["color"],
        width=0.5,
    )

    for bar, val in zip(bars, df["cost_usd"]):
        ax.text(
            bar.get_x() + bar.get_width() / 2,
            bar.get_height() + ax.get_ylim()[1] * 0.01,
            f"${val:.2f}",
            ha="center", va="bottom", fontsize=13,
        )

    seen_models = df["model"].unique()
    handles = [
        plt.Rectangle((0, 0), 1, 1, color=MODEL_COLORS.get(m, "#999999"))
        for m in seen_models
    ]
    ax.legend(handles, seen_models, title="Model", fontsize=13, title_fontsize=13)

    ax.set_ylabel("Cost (USD)", fontsize=14)
    ax.set_title("Spec Generation Cost by Directory", fontsize=16)
    ax.tick_params(axis="both", labelsize=13)
    ax.yaxis.set_major_formatter(
        mticker.FuncFormatter(lambda v, _: f"${v:,.2f}")
    )
    ax.margins(x=0.1)
    fig.tight_layout()
    fig.savefig(OUTPUT_FILE, dpi=150)
    plt.close(fig)
    print(f"Saved: {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
