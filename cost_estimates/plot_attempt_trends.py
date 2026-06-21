#!/usr/bin/env python3
"""
Plot the trend of how many problems reach each attempt number, per pipeline and profile.

For each pipeline × profile, counts how many distinct problem_ids appear at attempt 1,
attempt 2, attempt 3, etc. — showing how many problems required multiple repair attempts.

Reads: cost_estimates/translation_cost_estimate.csv
Output: cost_estimates/attempt_trends.png

Usage:
    python cost_estimates/plot_attempt_trends.py
"""

from __future__ import annotations

from pathlib import Path

import matplotlib.pyplot as plt
import matplotlib.ticker as mticker
import pandas as pd

SCRIPT_DIR = Path(__file__).resolve().parent

INPUT_CSV = SCRIPT_DIR / "translation_cost_estimate.csv"
OUTPUT_FILE = SCRIPT_DIR / "attempt_trends.png"

# Pipelines to include — edit as needed
PIPELINES: list[str] = [
    "full_translation_loop_GPT_5_2",
    "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS",
    "translation_with_validated_specs_GPT_5_2",
]

PROFILE_COLORS = {
    "code_only":     "#4C72B0",
    "specs_only":    "#DD8452",
    "code_and_spec": "#55A868",
}

PIPELINE_LABELS: dict[str, str] = {
    "full_translation_loop":                              "full_loop (4o-mini)",
    "full_translation_loop_GPT_5_2":                      "full_loop (gpt-5.2)",
    "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS":  "full_loop_5.2 SPECS",
    "translation_with_validated_specs_GPT_5_2":           "validated_specs (gpt-5.2)",
    "translation_with_validated_specs_using_GPT_4O_mini": "validated_specs (4o-mini)",
}

LINE_STYLES = ["-", "--", "-."]


def compute_cumulative_counts(df: pd.DataFrame) -> pd.DataFrame:
    """For each (pipeline, profile, attempt), cumulative sum of distinct problem_ids."""
    grouped = (
        df.groupby(["directory", "profile", "attempt"])["problem_id"]
        .nunique()
        .reset_index()
        .rename(columns={"problem_id": "problem_count", "directory": "pipeline"})
    )
    grouped = grouped.sort_values("attempt")
    grouped["cumulative"] = grouped.groupby(["pipeline", "profile"])["problem_count"].cumsum()
    return grouped


def main() -> None:
    if not INPUT_CSV.exists():
        raise FileNotFoundError(f"Input CSV not found: {INPUT_CSV}")

    df = pd.read_csv(INPUT_CSV)
    df = df[df["directory"].isin(PIPELINES)]

    if df.empty:
        raise ValueError(f"No data found for pipelines: {PIPELINES}")

    counts = compute_cumulative_counts(df)
    attempts = sorted(counts["attempt"].unique())
    profiles = sorted(counts["profile"].unique())

    fig, axes = plt.subplots(
        1, len(PIPELINES),
        figsize=(5 * len(PIPELINES), 5),
        sharey=True,
    )
    if len(PIPELINES) == 1:
        axes = [axes]

    for ax, pipeline in zip(axes, PIPELINES):
        pipeline_data = counts[counts["pipeline"] == pipeline]
        label = PIPELINE_LABELS.get(pipeline, pipeline)

        for profile in profiles:
            profile_data = pipeline_data[pipeline_data["profile"] == profile]
            if profile_data.empty:
                continue

            profile_data = profile_data.set_index("attempt").reindex(attempts, fill_value=0)
            cumulative = profile_data["cumulative"].values
            ax.plot(
                attempts,
                cumulative,
                marker="o",
                color=PROFILE_COLORS.get(profile, "#999999"),
                label=profile,
                linewidth=2,
                markersize=7,
            )

            for attempt, count in zip(attempts, cumulative):
                ax.annotate(
                    str(int(count)),
                    (attempt, count),
                    textcoords="offset points",
                    xytext=(0, 8),
                    ha="center",
                    fontsize=10,
                )

        ax.set_title(label, fontsize=12)
        ax.set_xlabel("Attempt", fontsize=11)
        ax.set_xticks(attempts)
        ax.tick_params(axis="both", labelsize=10)
        ax.yaxis.set_major_formatter(mticker.FuncFormatter(lambda v, _: f"{int(v):,}"))
        ax.legend(title="Profile", fontsize=9, title_fontsize=9)
        ax.grid(axis="y", linestyle="--", alpha=0.4)

    axes[0].set_ylabel("Cumulative problems", fontsize=12)
    fig.suptitle("Cumulative repair attempts by pipeline and profile", fontsize=14)
    fig.tight_layout()
    fig.savefig(OUTPUT_FILE, dpi=150)
    plt.close(fig)
    print(f"Saved: {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
