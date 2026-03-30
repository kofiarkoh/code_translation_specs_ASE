#!/usr/bin/env python3
from __future__ import annotations

import csv
import itertools
from pathlib import Path
from statistics import median

import numpy as np
from scipy.stats import friedmanchisquare, wilcoxon
from scipy.stats import rankdata

try:
    from statsmodels.stats.multitest import multipletests
except ModuleNotFoundError as exc:
    raise SystemExit("statsmodels is required. Run: pip install statsmodels") from exc


# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

PROJECT_ROOT = Path(__file__).resolve().parent.parent
DATASET_NAME = "optimized_prompt@1"
ANALYSIS_SUBDIR = "rust-code-analysis"

PIPELINES = {
    "A": "translation_with_validated_specs_GPT_5_2",
    # "B": "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS",
}

VARIANT_COLUMNS = [
    "specs_and_code",
    "specs_only",
    "code_only",
]

# All 3 pairwise combinations of variants
VARIANT_PAIRS = list(itertools.combinations(VARIANT_COLUMNS, 2))

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
# Data helpers
# ---------------------------------------------------------------------------

def get_variant_values(
    rows: dict[tuple[str, str], dict[str, float]],
    metric: str,
    variant: str,
) -> list[float]:
    """Return values for a single variant across all folders, in sorted folder order."""
    result = []
    for (folder, row_metric) in sorted(rows):
        if row_metric != metric:
            continue
        if variant in rows[(folder, row_metric)]:
            result.append(rows[(folder, row_metric)][variant])
    return result


def get_aligned_trio(
    rows: dict[tuple[str, str], dict[str, float]],
    metric: str,
) -> tuple[list[float], list[float], list[float]]:
    """
    Return three aligned lists (one per variant) for folders that have
    ALL THREE variants present — required for Friedman.
    """
    v1, v2, v3 = [], [], []
    for (folder, row_metric) in sorted(rows):
        if row_metric != metric:
            continue
        d = rows[(folder, row_metric)]
        if all(v in d for v in VARIANT_COLUMNS):
            v1.append(d[VARIANT_COLUMNS[0]])
            v2.append(d[VARIANT_COLUMNS[1]])
            v3.append(d[VARIANT_COLUMNS[2]])
    return v1, v2, v3


def get_aligned_pair(
    rows: dict[tuple[str, str], dict[str, float]],
    metric: str,
    var_a: str,
    var_b: str,
) -> tuple[list[float], list[float]]:
    """Return two aligned lists for folders that have BOTH variants."""
    a_vals, b_vals = [], []
    for (folder, row_metric) in sorted(rows):
        if row_metric != metric:
            continue
        d = rows[(folder, row_metric)]
        if var_a in d and var_b in d:
            a_vals.append(d[var_a])
            b_vals.append(d[var_b])
    return a_vals, b_vals


def all_metrics(rows: dict[tuple[str, str], dict[str, float]]) -> list[str]:
    return sorted({metric for _, metric in rows})


# ---------------------------------------------------------------------------
# Statistical helpers
# ---------------------------------------------------------------------------

def rank_biserial(a: list[float], b: list[float]) -> float:
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
    if r_abs < 0.1: return "negligible"
    if r_abs < 0.3: return "small"
    if r_abs < 0.5: return "medium"
    return "large"


def format_p(p: float) -> str:
    if p < 0.0001:
        return f"{p:.2e}"
    return f"{p:.6f}"


def direction(r: float) -> str:
    if r > 0.01:  return f"{VARIANT_COLUMNS[0].split('_')[0]} tends higher"
    if r < -0.01: return "second variant tends higher"
    return "no clear direction"


def direction_pair(var_a: str, var_b: str, r: float) -> str:
    a_label = var_a.replace("_", " ")
    b_label = var_b.replace("_", " ")
    if r > 0.01:  return f"{a_label} > {b_label}"
    if r < -0.01: return f"{b_label} > {a_label}"
    return "no difference"


# ---------------------------------------------------------------------------
# Main analysis
# ---------------------------------------------------------------------------

def analyze_pipeline(pipeline_label: str, pipeline_name: str) -> None:
    csv_path = metrics_csv_for_pipeline(pipeline_name)
    rows = load_metric_values(csv_path)
    metrics = all_metrics(rows)

    print(f"\n{'=' * 80}")
    print(f"PIPELINE {pipeline_label}: {pipeline_name}")
    print(f"Friedman test across 3 variants, Wilcoxon post-hoc with Holm-Bonferroni")
    print(f"Alpha: {ALPHA}")
    print(f"{'=' * 80}")

    for metric in metrics:
        print(f"\n── {metric} ──")

        # ── Descriptive stats per variant ─────────────────────────────────
        for variant in VARIANT_COLUMNS:
            vals = get_variant_values(rows, metric, variant)
            if vals:
                print(f"   {variant:<20} n={len(vals):3d}  median={median(vals):.4f}")

        # ── Friedman test (needs all 3 variants aligned) ───────────────────
        v1, v2, v3 = get_aligned_trio(rows, metric)
        n_trio = len(v1)

        if n_trio == 0:
            print(f"   Friedman: skipped (no samples with all 3 variants)")
            continue

        # Check if all three are identical (Friedman would be undefined)
        if (all(x == v1[0] for x in v1) and
                all(x == v2[0] for x in v2) and
                all(x == v3[0] for x in v3)):
            print(f"   Friedman: skipped (all values identical)")
            continue

        friedman_stat, friedman_p = friedmanchisquare(v1, v2, v3)
        friedman_sig = friedman_p < ALPHA

        print(
            f"\n   Friedman (n={n_trio}): χ²={friedman_stat:.4f}  "
            f"p={format_p(friedman_p)}  "
            f"{'→ SIGNIFICANT — running post-hoc' if friedman_sig else '→ not significant'}"
        )

        if not friedman_sig:
            continue

        # ── Post-hoc: pairwise Wilcoxon + Holm-Bonferroni ────────────────
        pair_results = []
        for (var_a, var_b) in VARIANT_PAIRS:
            a_vals, b_vals = get_aligned_pair(rows, metric, var_a, var_b)
            n_pair = len(a_vals)

            if n_pair == 0:
                pair_results.append({"pair": (var_a, var_b), "status": "skipped (no paired data)", "n": 0})
                continue

            diffs = [a - b for a, b in zip(a_vals, b_vals)]
            if all(d == 0 for d in diffs):
                pair_results.append({"pair": (var_a, var_b), "status": "skipped (all differences zero)", "n": n_pair})
                continue

            result = wilcoxon(a_vals, b_vals, zero_method="wilcox", alternative="two-sided", method="auto")
            r = rank_biserial(a_vals, b_vals)

            pair_results.append({
                "pair": (var_a, var_b),
                "status": "ok",
                "n": n_pair,
                "statistic": result.statistic,
                "p_raw": result.pvalue,
                "median_a": median(a_vals),
                "median_b": median(b_vals),
                "median_delta": median(diffs),
                "effect_r": r,
                "effect_label": interpret_effect(r),
            })

        # Apply Holm-Bonferroni across the 3 pairs
        valid = [r for r in pair_results if r.get("status") == "ok"]
        if valid:
            raw_pvals = [r["p_raw"] for r in valid]
            reject, p_corrected, _, _ = multipletests(raw_pvals, alpha=ALPHA, method="holm")
            for i, res in enumerate(valid):
                res["p_corrected"] = p_corrected[i]
                res["significant"] = reject[i]

        print(f"   Post-hoc pairwise (Holm-Bonferroni corrected):")
        for res in pair_results:
            var_a, var_b = res["pair"]
            pair_label = f"{var_a.replace('_',' ')} vs {var_b.replace('_',' ')}"

            if res.get("status") != "ok":
                print(f"     {pair_label}: {res['status']}")
                continue

            sig_flag = "✅ YES" if res["significant"] else "❌ NO"
            dir_str = direction_pair(var_a, var_b, res["effect_r"])

            print(
                f"     {pair_label}\n"
                f"       n={res['n']}  W={res['statistic']:.2f}  "
                f"p_raw={format_p(res['p_raw'])}  p_corr={format_p(res['p_corrected'])}  "
                f"significant={sig_flag}\n"
                f"       median({var_a.replace('_',' ')})={res['median_a']:.4f}  "
                f"median({var_b.replace('_',' ')})={res['median_b']:.4f}  "
                f"Δ={res['median_delta']:.4f}  →  {dir_str}\n"
                f"       effect r={res['effect_r']:.4f} ({res['effect_label']})"
            )

    print(f"\n{'=' * 80}\nDone — Pipeline {pipeline_label}.\n")


def main() -> None:
    for label, name in PIPELINES.items():
        analyze_pipeline(label, name)


if __name__ == "__main__":
    main()