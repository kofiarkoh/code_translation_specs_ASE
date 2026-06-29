#!/usr/bin/env python3
"""
Plot boxplots of rust-code-analysis metrics per profile, showing
distributions, variance, and outliers.

Reads the long-form metrics_comparison.csv from a pipeline's analysis folder.

Usage:
    python repairs/plot_metrics_boxplots.py
"""

from __future__ import annotations

from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import pandas as pd

SCRIPT_DIR = Path(__file__).resolve().parent

PIPELINE_NAME = "translation_with_validated_specs_GPT_5_2"
DATASET_NAME = "optimized_prompt@1"
ANALYSIS_SUBDIR = "rust-code-analysis-common-pass"

INPUT_CSV = (
    SCRIPT_DIR
    / f"{PIPELINE_NAME}_analysis"
    / DATASET_NAME
    / ANALYSIS_SUBDIR
    / "metrics_comparison.csv"
)
OUTPUT_DIR = INPUT_CSV.parent

PROFILE_COLUMNS = ["specs_and_code", "specs_only", "code_only"]

PROFILE_LABELS: dict[str, str] = {
    "specs_and_code": "Code & Specs",
    "specs_only":     "Specs Only",
    "code_only":      "Code Only",
}

PROFILE_COLORS: dict[str, str] = {
    "specs_and_code": "#009E73",
    "specs_only":     "#E69F00",
    "code_only":      "#0072B2",
}

# Metrics to plot — set to None to plot all found in the CSV
METRICS: list[str] | None = None

COLS_PER_ROW = 3
BASE_FONT_SIZE = 14
FIG_CELL_WIDTH = 5
FIG_CELL_HEIGHT = 5


def load_data(csv_path: Path) -> pd.DataFrame:
    if not csv_path.is_file():
        raise FileNotFoundError(f"Input CSV not found: {csv_path}")
    df = pd.read_csv(csv_path)
    df = df[df["folder"] != "__AVERAGE__"]
    return df


def main() -> None:
    df = load_data(INPUT_CSV)
    metrics = METRICS or sorted(df["metric"].unique())
    n_metrics = len(metrics)
    n_rows = (n_metrics + COLS_PER_ROW - 1) // COLS_PER_ROW

    plt.rcParams.update({"font.size": BASE_FONT_SIZE})
    fig, axes = plt.subplots(
        n_rows, COLS_PER_ROW,
        figsize=(FIG_CELL_WIDTH * COLS_PER_ROW, FIG_CELL_HEIGHT * n_rows),
        squeeze=False,
    )

    import numpy as np

    for idx, metric in enumerate(metrics):
        row_idx, col_idx = divmod(idx, COLS_PER_ROW)
        ax = axes[row_idx][col_idx]
        metric_df = df[df["metric"] == metric]

        print(f"\n{'=' * 70}")
        print(f"  {metric}")
        print(f"{'=' * 70}")

        data = []
        labels = []
        colors = []
        for col in PROFILE_COLUMNS:
            vals = pd.to_numeric(metric_df[col], errors="coerce").dropna()
            data.append(vals.values)
            labels.append(PROFILE_LABELS.get(col, col))
            colors.append(PROFILE_COLORS.get(col, "#999999"))

            prof_label = PROFILE_LABELS.get(col, col)
            q1 = np.percentile(vals, 25)
            q3 = np.percentile(vals, 75)
            iqr = q3 - q1
            lower_fence = q1 - 1.5 * iqr
            upper_fence = q3 + 1.5 * iqr
            outliers = vals[(vals < lower_fence) | (vals > upper_fence)]

            print(f"\n  {prof_label}  (n={len(vals)})")
            print(f"    Mean     : {vals.mean():.4f}")
            print(f"    Median   : {np.median(vals):.4f}")
            print(f"    Std Dev  : {vals.std():.4f}")
            print(f"    Variance : {vals.var():.4f}")
            print(f"    Min      : {vals.min():.4f}")
            print(f"    Q1       : {q1:.4f}")
            print(f"    Q3       : {q3:.4f}")
            print(f"    Max      : {vals.max():.4f}")
            print(f"    IQR      : {iqr:.4f}")
            print(f"    Fences   : [{lower_fence:.4f}, {upper_fence:.4f}]")
            print(f"    Outliers : {len(outliers)}", end="")
            if len(outliers) > 0:
                sorted_out = sorted(outliers)
                preview = ", ".join(f"{v:.2f}" for v in sorted_out[:10])
                if len(sorted_out) > 10:
                    preview += f" ... ({len(sorted_out)} total)"
                print(f"  [{preview}]")
            else:
                print()

        bp = ax.boxplot(
            data,
            tick_labels=labels,
            patch_artist=True,
            showfliers=True,
            flierprops=dict(marker="o", markersize=4, alpha=0.5),
            medianprops=dict(color="black", linewidth=1.5),
            whiskerprops=dict(linewidth=1.2),
            capprops=dict(linewidth=1.2),
        )
        for patch, color in zip(bp["boxes"], colors):
            patch.set_facecolor(color)
            patch.set_alpha(0.7)

        ax.set_title(metric, fontsize=BASE_FONT_SIZE)
        ax.tick_params(axis="x", labelsize=BASE_FONT_SIZE - 3, rotation=15)
        ax.tick_params(axis="y", labelsize=BASE_FONT_SIZE - 2)
        ax.grid(axis="y", linestyle="--", alpha=0.3)

    # Hide unused subplots
    for idx in range(n_metrics, n_rows * COLS_PER_ROW):
        row_idx, col_idx = divmod(idx, COLS_PER_ROW)
        axes[row_idx][col_idx].set_visible(False)

    fig.suptitle(
        f"{PIPELINE_NAME.replace('_', ' ')} — Metric Distributions by Profile",
        fontsize=BASE_FONT_SIZE + 2,
        y=1.01,
    )
    fig.tight_layout()

    out_png = OUTPUT_DIR / "metrics_boxplots.png"
    out_pdf = OUTPUT_DIR / "metrics_boxplots.pdf"
    fig.savefig(out_png, dpi=200, bbox_inches="tight")
    fig.savefig(out_pdf, bbox_inches="tight")
    plt.close(fig)
    print(f"Saved: {out_png}")
    print(f"Saved: {out_pdf}")


if __name__ == "__main__":
    main()
