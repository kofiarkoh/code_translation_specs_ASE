#!/usr/bin/env python3
"""
Plot translation cost estimates from translation_cost_estimate_summary.csv
as grouped bar charts — one group per pipeline, one bar per profile.

Produces one chart per metric in METRICS.

Usage:
    python plot_translation_costs.py
"""

from __future__ import annotations

from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import pandas as pd

# ============================================
# Configure here
# ============================================
SUMMARY_CSV = Path("translation_cost_estimate_summary.csv")
OUTPUT_DIR = Path(".")

# Metrics to plot: (column_name, y-axis label, output filename stem)
METRICS = [
    ("cost_usd", "Cost (USD)", "translation_cost_by_profile"),
]

# Optional: short display names for long pipeline names
PIPELINE_LABELS: dict[str, str] = {
    "full_translation_loop":                           "with \n(4o-mini)",
    "full_translation_loop_GPT_5_2":                   "with \n(gpt-5.2)",
    "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS": "with GPT 5.2 \n using GPT 4o mini SPECS",
    "translation_with_validated_specs_GPT_5_2":        "validated_specs\n(gpt-5.2)",
    "translation_with_validated_specs_using_GPT_4O_mini": "validated_specs\n(4o-mini)",
}

PROFILE_COLORS = {
    "code_only":    "#4C72B0",
    "specs_only":   "#DD8452",
    "code_and_spec": "#55A868",
}

BAR_WIDTH = 0.25
FIGURE_WIDTH = max(10, 2.5 * 5)  # scales with number of pipelines


def load_data(csv_path: Path) -> pd.DataFrame:
    df = pd.read_csv(csv_path)
    df["pipeline_label"] = df["pipeline"].map(
        lambda p: PIPELINE_LABELS.get(p, p)
    )
    return df


def plot_metric(
    df: pd.DataFrame,
    metric: str,
    ylabel: str,
    out_stem: str,
    output_dir: Path,
) -> Path:
    profiles = sorted(df["profile"].unique())
    pipelines = list(df["pipeline_label"].unique())
    n_pipelines = len(pipelines)
    n_profiles = len(profiles)

    fig, ax = plt.subplots(figsize=(max(12, n_pipelines * 3), 7))

    offsets = [
        (i - (n_profiles - 1) / 2) * BAR_WIDTH for i in range(n_profiles)
    ]
    x = range(n_pipelines)

    for offset, profile in zip(offsets, profiles):
        values = []
        for pipeline_label in pipelines:
            row = df[
                (df["pipeline_label"] == pipeline_label)
                & (df["profile"] == profile)
            ]
            values.append(float(row[metric].values[0]) if not row.empty else 0.0)

        bars = ax.bar(
            [xi + offset for xi in x],
            values,
            width=BAR_WIDTH,
            label=profile,
            color=PROFILE_COLORS.get(profile),
        )

        for bar, val in zip(bars, values):
            if val == 0:
                continue
            label = f"${val:.2f}" if metric == "cost_usd" else f"{val:,.0f}"
            ax.text(
                bar.get_x() + bar.get_width() / 2,
                bar.get_height() + ax.get_ylim()[1] * 0.005,
                label,
                ha="center", va="bottom", fontsize=11, rotation=45,
            )

    ax.set_xticks(list(x))
    ax.set_xticklabels(pipelines, fontsize=13)
    ax.set_ylabel(ylabel, fontsize=14)
    ax.set_title(f"Translation Cost — {ylabel} by Pipeline and Profile", fontsize=16)
    ax.legend(title="Profile", fontsize=13, title_fontsize=13)
    ax.tick_params(axis="y", labelsize=13)
    ax.yaxis.set_major_formatter(
        mticker.FuncFormatter(
            lambda v, _: f"${v:,.2f}" if metric == "cost_usd" else f"{v:,.0f}"
        )
    )
    ax.margins(x=0.05)
    fig.tight_layout()

    out_path = output_dir / f"{out_stem}.png"
    fig.savefig(out_path, dpi=150)
    plt.close(fig)
    return out_path


def main() -> None:
    if not SUMMARY_CSV.exists():
        raise FileNotFoundError(f"Summary CSV not found: {SUMMARY_CSV}")

    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    df = load_data(SUMMARY_CSV)

    print(f"Loaded {len(df)} rows from {SUMMARY_CSV}")
    print(f"Pipelines : {sorted(df['pipeline'].unique())}")
    print(f"Profiles  : {sorted(df['profile'].unique())}")
    print()

    for metric, ylabel, out_stem in METRICS:
        if metric not in df.columns:
            print(f"[SKIP] column '{metric}' not found in CSV")
            continue
        out_path = plot_metric(df, metric, ylabel, out_stem, OUTPUT_DIR)
        print(f"Saved: {out_path}")


if __name__ == "__main__":
    main()
