#!/usr/bin/env python3
"""
Combine translation cost and spec generation cost into a unified summary.

For profiles that consume specs (specs_only, code_and_spec), the spec
generation cost is split equally between them and added to their
translation cost.  code_only profiles are unaffected.

Outputs:
  - combined_cost_summary.csv   — per pipeline × profile with breakdown
  - prints a formatted table

Usage:
    python combine_cost_summaries.py
"""

from __future__ import annotations

import csv
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

# ================================================================
# Configure here
# ================================================================
TRANSLATION_CSV = Path("translation_cost_estimate_summary.csv")
SPEC_GEN_CSV    = Path("spec_generation_cost_estimate_summary.csv")
OUTPUT_CSV      = Path("combined_cost_summary.csv")

# Map each pipeline to the spec-generation directory whose cost it consumed.
# Pipelines not listed here are assumed to have no external spec cost.
PIPELINE_TO_SPEC_DIR: dict[str, str] = {
    "full_translation_loop": "specs",
    "full_translation_loop_GPT_5_2":  "specs",
    "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS":  "specs_WITH_GPT_5_2",
    "translation_with_validated_specs_GPT_5_2": "validated_specs_WITH_GPT_gpt-5.2",
     "translation_with_validated_specs_using_GPT_4O_mini":  "validated_specs_WITH_GPT_gpt-4o-mini",
}

# Profiles that consume specs — spec gen cost is split equally among these
SPEC_CONSUMING_PROFILES = {"specs_only", "code_and_spec"}

# Set to False to hide the spec gen overhead segment in the plot
SHOW_SPEC_GEN_COST = True


def load_spec_costs(path: Path) -> dict[str, float]:
    """Return {directory: total_cost_usd}."""
    df = pd.read_csv(path)
    return dict(zip(df["directory"], df["cost_usd"]))


def build_combined(
    trans_df: pd.DataFrame,
    spec_costs: dict[str, float],
) -> pd.DataFrame:
    rows = []
    for _, row in trans_df.iterrows():
        pipeline = row["pipeline"]
        profile  = row["profile"]
        trans_cost = float(row["cost_usd"])

        spec_dir = PIPELINE_TO_SPEC_DIR.get(pipeline)
        if spec_dir and profile in SPEC_CONSUMING_PROFILES:
            spec_allocated = spec_costs.get(spec_dir, 0.0)
        else:
            spec_allocated = 0.0

        rows.append({
            "pipeline":        pipeline,
            "model":           row["model"],
            "profile":         profile,
            "problems":        int(row["problems"]),
            "translation_cost": round(trans_cost, 4),
            "spec_gen_cost":   round(spec_allocated, 4),
            "total_cost":      round(trans_cost + spec_allocated, 4),
        })

    return pd.DataFrame(rows)


def print_table(df: pd.DataFrame) -> None:
    col = 55
    print(
        f"\n{'Pipeline':<{col}} {'Model':<14} {'Profile':<15}"
        f" {'Trans $':>10} {'Spec $ (alloc)':>15} {'Total $':>10}"
    )
    print("-" * (col + 68))

    for pipeline, grp in df.groupby("pipeline", sort=False):
        for _, row in grp.iterrows():
            print(
                f"{row['pipeline']:<{col}} {row['model']:<14} {row['profile']:<15}"
                f" ${row['translation_cost']:>9.4f} ${row['spec_gen_cost']:>14.4f}"
                f" ${row['total_cost']:>9.4f}"
            )

        # Pipeline subtotal
        t = grp["translation_cost"].sum()
        s = grp["spec_gen_cost"].sum()
        print(
            f"{'  SUBTOTAL':<{col}} {'':<14} {'':<15}"
            f" ${t:>9.4f} ${s:>14.4f} ${t+s:>9.4f}"
        )
        print()

    # Grand total
    gt = df["translation_cost"].sum()
    gs = df["spec_gen_cost"].sum()
    print("-" * (col + 68))
    print(
        f"{'GRAND TOTAL':<{col}} {'':<14} {'':<15}"
        f" ${gt:>9.4f} ${gs:>14.4f} ${gt+gs:>9.4f}"
    )


PIPELINE_LABELS: dict[str, str] = {
    "full_translation_loop":                              "full_loop\n(4o-mini)",
    "full_translation_loop_GPT_5_2":                      "full_loop\n(gpt-5.2)",
    "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS":  "full_loop_5.2\nSPECS",
    "translation_with_validated_specs_GPT_5_2":           "validated_specs\n(gpt-5.2)",
}

# Pipelines to exclude from the plot (still appear in the CSV)
EXCLUDED_PIPELINES = {"translation_with_validated_specs_using_GPT_4O_mini"}

PROFILE_COLORS = {
    "code_only":     "#4C72B0",
    "specs_only":    "#DD8452",
    "code_and_spec": "#55A868",
}

BAR_WIDTH = 0.22


def plot_combined(df: pd.DataFrame, output_file: Path) -> None:
    df = df[~df["pipeline"].isin(EXCLUDED_PIPELINES)].copy()
    df["label"] = df["pipeline"].map(lambda p: PIPELINE_LABELS.get(p, p))

    pipelines = list(df["label"].unique())
    profiles  = list(PROFILE_COLORS)
    n_pip     = len(pipelines)
    n_prof    = len(profiles)

    fig, ax = plt.subplots(figsize=(max(12, n_pip * 3), 7))

    offsets = [(i - (n_prof - 1) / 2) * BAR_WIDTH for i in range(n_prof)]
    x = np.arange(n_pip)

    for offset, profile in zip(offsets, profiles):
        trans_vals = []
        spec_vals  = []
        for label in pipelines:
            row = df[(df["label"] == label) & (df["profile"] == profile)]
            if row.empty:
                trans_vals.append(0.0)
                spec_vals.append(0.0)
            else:
                trans_vals.append(float(row["translation_cost"].values[0]))
                spec_vals.append(float(row["spec_gen_cost"].values[0]))

        color = PROFILE_COLORS[profile]
        if SHOW_SPEC_GEN_COST:
            # Spec gen cost — hatched segment at the bottom
            ax.bar(
                x + offset, spec_vals,
                width=BAR_WIDTH, color=color, alpha=0.45, hatch="///", edgecolor="white",
            )
            # Label inside the hatched segment
            for xi, sv in enumerate(spec_vals):
                if sv == 0:
                    continue
                ax.text(
                    x[xi] + offset, sv / 2,
                    f"${sv:.2f}",
                    ha="center", va="center", fontsize=8,
                    fontweight="bold", color="black",
                )
            bottoms = spec_vals
        else:
            bottoms = [0.0] * len(trans_vals)

        # Translation cost — solid bar
        ax.bar(
            x + offset, trans_vals,
            width=BAR_WIDTH, bottom=bottoms,
            color=color, label=f"{profile}",
        )

        # Value label on top of full bar
        for xi, (tv, sv) in enumerate(zip(trans_vals, spec_vals)):
            total = (tv + sv) if SHOW_SPEC_GEN_COST else tv
            if total == 0:
                continue
            ax.text(
                x[xi] + offset, total + ax.get_ylim()[1] * 0.005,
                f"${total:.2f}",
                ha="center", va="bottom", fontsize=10, rotation=45,
            )

    ax.set_xticks(x)
    ax.set_xticklabels(pipelines, fontsize=13)
    ax.set_ylabel("Cost (USD)", fontsize=14)
    subtitle = "(solid = translation, hatched = spec generation)" if SHOW_SPEC_GEN_COST else "(translation cost only)"
    ax.set_title(f"Total Cost by Pipeline and Profile\n{subtitle}", fontsize=14)
    ax.tick_params(axis="y", labelsize=13)

    handles, labels = ax.get_legend_handles_labels()
    if SHOW_SPEC_GEN_COST:
        from matplotlib.patches import Patch
        hatch_patch = Patch(facecolor="grey", alpha=0.45, hatch="///", edgecolor="white",
                            label="spec gen (allocated)")
        handles += [hatch_patch]
        labels  += ["spec gen (allocated)"]
    ax.legend(handles, labels, fontsize=12, title="Profile", title_fontsize=12)

    ax.margins(x=0.05)
    import matplotlib.ticker as mticker
    ax.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"${v:,.2f}"))
    fig.tight_layout()
    fig.savefig(output_file, dpi=150)
    plt.close(fig)
    print(f"Plot saved: {output_file}")


def main() -> None:
    for p in (TRANSLATION_CSV, SPEC_GEN_CSV):
        if not p.exists():
            raise FileNotFoundError(f"Not found: {p}")

    trans_df   = pd.read_csv(TRANSLATION_CSV)
    spec_costs = load_spec_costs(SPEC_GEN_CSV)

    print("Spec generation costs loaded:")
    for d, c in sorted(spec_costs.items()):
        print(f"  {d}: ${c:.4f}")

    combined = build_combined(trans_df, spec_costs)
    print_table(combined)

    combined.to_csv(OUTPUT_CSV, index=False)
    print(f"\nCombined summary written to: {OUTPUT_CSV}")

    plot_file = OUTPUT_CSV.with_suffix(".png")
    plot_combined(combined, plot_file)


if __name__ == "__main__":
    main()
