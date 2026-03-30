#!/usr/bin/env python3
"""
check_distribution.py

Checks the distribution of metric values for each variant across both pipelines.
Runs:
  - Shapiro-Wilk normality test (per metric × variant)
  - Descriptive stats (mean, median, std, skewness, kurtosis)
  - Levene's test for equal variance across variants (per metric)
  - Plots histograms + Q-Q plots for visual inspection
"""
from __future__ import annotations

import csv
import itertools
from pathlib import Path
from statistics import median, mean, stdev

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from scipy import stats

# ---------------------------------------------------------------------------
# Configuration — must match your main analysis script
# ---------------------------------------------------------------------------

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt@1"
ANALYSIS_SUBDIR = "rust-code-analysis"

PIPELINES = {
    "A": "translation_with_validated_specs_GPT_5_2",
    "B": "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS",
}

VARIANT_COLUMNS = [
    "specs_and_code",
    "specs_only",
    "code_only",
]

ALPHA = 0.05

# ---------------------------------------------------------------------------
# File loading (copied from main script)
# ---------------------------------------------------------------------------

def metrics_csv_for_pipeline(pipeline_name: str) -> Path:
    return (
        PROJECT_ROOT
        / "repairs"
        / f"{pipeline_name}_analysis"
        / DATASET_NAME
        / ANALYSIS_SUBDIR
        / "metrics_comparison.csv"
    )


def load_metric_values(csv_path: Path) -> dict[tuple[str, str], dict[str, float]]:
    if not csv_path.is_file():
        raise FileNotFoundError(f"Metrics CSV not found: {csv_path}")

    rows: dict[tuple[str, str], dict[str, float]] = {}
    with csv_path.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            folder = row.get("folder", "").strip()
            metric = row.get("metric", "").strip()
            if not folder or folder == "__AVERAGE__" or not metric:
                continue
            values: dict[str, float] = {}
            for variant in VARIANT_COLUMNS:
                raw_value = row.get(variant, "").strip()
                if raw_value:
                    values[variant] = float(raw_value)
            rows[(folder, metric)] = values
    return rows


def get_variant_values(
    rows: dict[tuple[str, str], dict[str, float]],
    metric: str,
    variant: str,
) -> list[float]:
    result = []
    for (folder, row_metric) in sorted(rows):
        if row_metric != metric:
            continue
        if variant in rows[(folder, row_metric)]:
            result.append(rows[(folder, row_metric)][variant])
    return result


def all_metrics(rows: dict[tuple[str, str], dict[str, float]]) -> list[str]:
    return sorted({metric for _, metric in rows})


# ---------------------------------------------------------------------------
# Distribution checks
# ---------------------------------------------------------------------------

def shapiro_label(p: float) -> str:
    return "normal ✅" if p >= ALPHA else "NOT normal ❌"


def describe(vals: list[float]) -> dict:
    arr = np.array(vals)
    return {
        "n":        len(vals),
        "mean":     float(np.mean(arr)),
        "median":   float(np.median(arr)),
        "std":      float(np.std(arr, ddof=1)),
        "skew":     float(stats.skew(arr)),
        "kurtosis": float(stats.kurtosis(arr)),   # excess kurtosis (0 = normal)
        "min":      float(np.min(arr)),
        "max":      float(np.max(arr)),
    }


def check_distributions(pipeline_label: str, pipeline_name: str, save_plots: bool = True) -> None:
    csv_path = metrics_csv_for_pipeline(pipeline_name)
    rows = load_metric_values(csv_path)
    metrics = all_metrics(rows)

    print(f"\n{'=' * 80}")
    print(f"PIPELINE {pipeline_label}: {pipeline_name}")
    print(f"{'=' * 80}")

    for metric in metrics:
        print(f"\n── {metric} ──")

        variant_data: dict[str, list[float]] = {}
        for variant in VARIANT_COLUMNS:
            vals = get_variant_values(rows, metric, variant)
            if vals:
                variant_data[variant] = vals

        if not variant_data:
            print("   No data found.")
            continue

        # ── Descriptive stats + Shapiro-Wilk ──────────────────────────────
        print(f"   {'Variant':<20} {'n':>4}  {'mean':>8}  {'median':>8}  "
              f"{'std':>8}  {'skew':>7}  {'kurt':>7}  {'Shapiro p':>10}  {'normality'}")
        print(f"   {'-'*105}")

        for variant, vals in variant_data.items():
            d = describe(vals)
            if d["n"] >= 3:
                sw_stat, sw_p = stats.shapiro(vals)
            else:
                sw_stat, sw_p = float("nan"), float("nan")

            sw_str  = f"{sw_p:.4f}" if not np.isnan(sw_p) else "   n/a"
            norm_str = shapiro_label(sw_p) if not np.isnan(sw_p) else "n/a"

            print(
                f"   {variant:<20} {d['n']:>4}  {d['mean']:>8.4f}  {d['median']:>8.4f}  "
                f"{d['std']:>8.4f}  {d['skew']:>7.3f}  {d['kurtosis']:>7.3f}  "
                f"{sw_str:>10}  {norm_str}"
            )

        # ── Levene's test (equal variances across variants) ───────────────
        groups = list(variant_data.values())
        if len(groups) >= 2 and all(len(g) >= 2 for g in groups):
            lev_stat, lev_p = stats.levene(*groups)
            lev_sig = lev_p < ALPHA
            print(
                f"\n   Levene's test (equal variances): "
                f"W={lev_stat:.4f}  p={lev_p:.6f}  "
                f"→ {'variances differ ❌' if lev_sig else 'variances equal ✅'}"
            )

        # ── Plots ──────────────────────────────────────────────────────────
        if save_plots:
            n_variants = len(variant_data)
            fig = plt.figure(figsize=(6 * n_variants, 8))
            fig.suptitle(f"Pipeline {pipeline_label} — {metric}", fontsize=13, fontweight="bold")
            gs = gridspec.GridSpec(2, n_variants, figure=fig, hspace=0.45, wspace=0.35)

            for col, (variant, vals) in enumerate(variant_data.items()):
                arr = np.array(vals)

                # Histogram + KDE
                ax_hist = fig.add_subplot(gs[0, col])
                ax_hist.hist(arr, bins="auto", density=True, alpha=0.6, color="steelblue", edgecolor="white")
                if len(arr) >= 3:
                    kde_x = np.linspace(arr.min(), arr.max(), 300)
                    kde = stats.gaussian_kde(arr)
                    ax_hist.plot(kde_x, kde(kde_x), color="navy", linewidth=1.5, label="KDE")
                ax_hist.set_title(variant.replace("_", " "), fontsize=10)
                ax_hist.set_xlabel("value")
                ax_hist.set_ylabel("density")

                # Q-Q plot
                ax_qq = fig.add_subplot(gs[1, col])
                (osm, osr), (slope, intercept, r) = stats.probplot(arr, dist="norm")
                ax_qq.scatter(osm, osr, s=12, alpha=0.6, color="steelblue")
                line_x = np.array([min(osm), max(osm)])
                ax_qq.plot(line_x, slope * line_x + intercept, color="red", linewidth=1.5)
                ax_qq.set_title(f"Q-Q  (r={r:.3f})", fontsize=9)
                ax_qq.set_xlabel("theoretical quantiles")
                ax_qq.set_ylabel("sample quantiles")

            plot_dir = Path("distribution_plots")
            plot_dir.mkdir(exist_ok=True)
            safe_metric = metric.replace("/", "_").replace(" ", "_")
            plot_path = plot_dir / f"pipeline_{pipeline_label}_{safe_metric}.png"
            fig.savefig(plot_path, dpi=120, bbox_inches="tight")
            plt.close(fig)
            print(f"   Plot saved → {plot_path}")

    print(f"\n{'=' * 80}\nDone — Pipeline {pipeline_label}.\n")


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

def main() -> None:
    for label, name in PIPELINES.items():
        check_distributions(label, name, save_plots=True)


if __name__ == "__main__":
    main()