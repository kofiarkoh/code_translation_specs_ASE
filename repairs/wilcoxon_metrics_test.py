#!/usr/bin/env python3
from __future__ import annotations

import csv
from pathlib import Path
from statistics import median

import numpy as np
from scipy.stats import wilcoxon, rankdata

try:
    from statsmodels.stats.multitest import multipletests
except ModuleNotFoundError as exc:
    raise SystemExit(
        "statsmodels is required. Run: pip install statsmodels"
    ) from exc


# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt@1"
ANALYSIS_SUBDIR = "rust-code-analysis"

PIPELINE_A = "translation_with_validated_specs_GPT_5_2"
PIPELINE_B = "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS"

VARIANT_COLUMNS = [
    "specs_and_code",
    "specs_only",
    "code_only",
]

ALPHA = 0.05


# ---------------------------------------------------------------------------
# File loading
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


# ---------------------------------------------------------------------------
# Pairing helpers
# ---------------------------------------------------------------------------

def paired_values_for_variant(
    rows_a: dict[tuple[str, str], dict[str, float]],
    rows_b: dict[tuple[str, str], dict[str, float]],
    metric: str,
    variant: str,
) -> tuple[list[float], list[float]]:
    paired_a: list[float] = []
    paired_b: list[float] = []

    for (folder, row_metric) in sorted(set(rows_a) & set(rows_b)):
        if row_metric != metric:
            continue
        if variant not in rows_a[(folder, row_metric)] or variant not in rows_b[(folder, row_metric)]:
            continue
        paired_a.append(rows_a[(folder, row_metric)][variant])
        paired_b.append(rows_b[(folder, row_metric)][variant])

    return paired_a, paired_b


def all_metrics(
    rows_a: dict[tuple[str, str], dict[str, float]],
    rows_b: dict[tuple[str, str], dict[str, float]],
) -> list[str]:
    return sorted({metric for _, metric in set(rows_a) & set(rows_b)})


# ---------------------------------------------------------------------------
# Statistical helpers
# ---------------------------------------------------------------------------

def rank_biserial(a: list[float], b: list[float]) -> float:
    """
    Rank-biserial correlation as effect size for Wilcoxon signed-rank test.
    Range: -1 to +1.
      Positive → A tends to be higher than B
      Negative → B tends to be higher than A
    """
    diff = np.array(a) - np.array(b)
    diff = diff[diff != 0]
    if len(diff) == 0:
        return 0.0
    ranks = rankdata(np.abs(diff))
    w_plus = float(ranks[diff > 0].sum())
    w_minus = float(ranks[diff < 0].sum())
    return (w_plus - w_minus) / (w_plus + w_minus)


def interpret_effect(r: float) -> str:
    r_abs = abs(r)
    if r_abs < 0.1:
        return "negligible"
    if r_abs < 0.3:
        return "small"
    if r_abs < 0.5:
        return "medium"
    return "large"


def format_p(p: float) -> str:
    if p < 0.0001:
        return f"{p:.2e}"
    return f"{p:.6f}"


# ---------------------------------------------------------------------------
# Main analysis
# ---------------------------------------------------------------------------

def main() -> None:
    csv_a = metrics_csv_for_pipeline(PIPELINE_A)
    csv_b = metrics_csv_for_pipeline(PIPELINE_B)
    rows_a = load_metric_values(csv_a)
    rows_b = load_metric_values(csv_b)
    metrics = all_metrics(rows_a, rows_b)

    print(f"Pipeline A : {PIPELINE_A}")
    print(f"Pipeline B : {PIPELINE_B}")
    print(f"Alpha      : {ALPHA}")
    print(f"Correction : Holm-Bonferroni (applied per metric across variants)")
    print("=" * 80)

    for metric in metrics:
        print(f"\n{'=' * 80}")
        print(f"METRIC: {metric}")
        print(f"{'=' * 80}")

        # ── 1. Collect raw Wilcoxon results for all variants ──────────────
        variant_results: list[dict] = []

        for variant in VARIANT_COLUMNS:
            values_a, values_b = paired_values_for_variant(rows_a, rows_b, metric, variant)
            n = len(values_a)

            if n == 0:
                variant_results.append({"variant": variant, "status": "skipped (no paired values)", "n": 0})
                continue

            differences = [a - b for a, b in zip(values_a, values_b)]

            if all(d == 0 for d in differences):
                variant_results.append({"variant": variant, "status": "skipped (all differences zero)", "n": n})
                continue

            result = wilcoxon(
                values_a,
                values_b,
                zero_method="wilcox",
                alternative="two-sided",
                method="auto",
            )

            r = rank_biserial(values_a, values_b)

            variant_results.append({
                "variant": variant,
                "status": "ok",
                "n": n,
                "statistic": result.statistic,
                "p_raw": result.pvalue,
                "median_a": median(values_a),
                "median_b": median(values_b),
                "median_delta": median(differences),   # ✅ fixed: use statistics.median
                "effect_r": r,
                "effect_label": interpret_effect(r),
            })

        # ── 2. Holm-Bonferroni correction across variants ─────────────────
        valid = [r for r in variant_results if r.get("status") == "ok"]
        if valid:
            raw_pvals = [r["p_raw"] for r in valid]
            reject, p_corrected, _, _ = multipletests(raw_pvals, alpha=ALPHA, method="holm")
            for i, res in enumerate(valid):
                res["p_corrected"] = p_corrected[i]
                res["significant"] = reject[i]

        # ── 3. Print results ──────────────────────────────────────────────
        for res in variant_results:
            variant = res["variant"]
            if res.get("status") != "ok":
                print(f"\n  [{variant}]  n={res['n']} | {res['status']}")
                continue

            sig_flag = "✅ YES" if res["significant"] else "❌ NO"
            direction = "A > B" if res["median_delta"] > 0 else ("B > A" if res["median_delta"] < 0 else "no difference")

            print(
                f"\n  [{variant}]\n"
                f"    n                 : {res['n']}\n"
                f"    Median (A)        : {res['median_a']:.4f}\n"
                f"    Median (B)        : {res['median_b']:.4f}\n"
                f"    Median delta (A-B): {res['median_delta']:.4f}  →  {direction}\n"
                f"    W statistic       : {res['statistic']:.4f}\n"
                f"    p (raw)           : {format_p(res['p_raw'])}\n"
                f"    p (Holm-corrected): {format_p(res['p_corrected'])}\n"
                f"    Significant       : {sig_flag}\n"
                f"    Effect size r     : {res['effect_r']:.4f}  ({res['effect_label']})"
            )

    print(f"\n{'=' * 80}")
    print("Done.")


if __name__ == "__main__":
    main()