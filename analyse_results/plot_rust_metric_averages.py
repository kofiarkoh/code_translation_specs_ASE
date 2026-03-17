#!/usr/bin/env python3
"""Helpers for plotting average Rust metric comparisons."""

from __future__ import annotations

from pathlib import Path
from typing import Dict, List, Optional, Tuple

import matplotlib.pyplot as plt


def plot_average_graphs(
    averages: Dict[str, Dict[str, Optional[float]]],
    metric_fields: List[Tuple[str, Tuple[str, ...]]],
    metric_better_higher: Dict[str, bool],
    values_plot_path: Path,
    deltas_plot_path: Path,
) -> None:
    metric_names = [display_name for display_name, _ in metric_fields]
    x = list(range(len(metric_names)))
    width = 0.25

    def normalize_metric(metric_name: str, raw_values: Dict[str, Optional[float]]) -> Dict[str, float]:
        present = [v for v in raw_values.values() if v is not None]
        if not present:
            return {k: 0.0 for k in raw_values}

        if metric_better_higher.get(metric_name, False):
            baseline = max(present)
            if baseline == 0:
                return {k: 0.0 for k in raw_values}
            return {
                k: ((v / baseline) * 100.0) if v is not None else 0.0
                for k, v in raw_values.items()
            }

        positive_present = [v for v in present if v > 0]
        if not positive_present:
            return {k: 0.0 for k in raw_values}
        baseline = min(positive_present)
        return {
            k: ((baseline / v) * 100.0) if (v is not None and v > 0) else 0.0
            for k, v in raw_values.items()
        }

    normalized_values = {
        name: normalize_metric(
            name,
            {
                "translations_with_specs_and_code.rs": averages.get(name, {}).get("translations_with_specs_and_code.rs__avg"),
                "translations_with_specs_only.rs": averages.get(name, {}).get("translations_with_specs_only.rs__avg"),
                "translations_with_code_only.rs": averages.get(name, {}).get("translations_with_code_only.rs__avg"),
            },
        )
        for name in metric_names
    }

    specs_and_code_vals = [normalized_values[name]["translations_with_specs_and_code.rs"] for name in metric_names]
    specs_only_vals = [normalized_values[name]["translations_with_specs_only.rs"] for name in metric_names]
    code_only_vals = [normalized_values[name]["translations_with_code_only.rs"] for name in metric_names]

    fig, ax = plt.subplots(figsize=(18, 8))
    ax.bar([i - width for i in x], specs_and_code_vals, width=width, label="specs_and_code")
    ax.bar(x, specs_only_vals, width=width, label="specs_only")
    ax.bar([i + width for i in x], code_only_vals, width=width, label="code_only")
    ax.set_title("Average Rust Metrics by Translation Variant (Normalized to 100%)")
    ax.set_xticks(x)
    ax.set_xticklabels(metric_names, rotation=45, ha="right")
    ax.set_ylim(0, max(110, max(specs_and_code_vals + specs_only_vals + code_only_vals, default=100)))
    ax.set_ylabel("Percent of best value for each metric")
    ax.grid(axis="y", linestyle="--", alpha=0.3)
    ax.legend()
    fig.tight_layout()
    fig.savefig(values_plot_path, dpi=200)
    plt.close(fig)

    pair_info = [
        (
            "Average Delta: specs_and_code - specs_only",
            "translations_with_specs_and_code.rs__minus__translations_with_specs_only.rs__avg",
        ),
        (
            "Average Delta: specs_and_code - code_only",
            "translations_with_specs_and_code.rs__minus__translations_with_code_only.rs__avg",
        ),
        (
            "Average Delta: specs_only - code_only",
            "translations_with_specs_only.rs__minus__translations_with_code_only.rs__avg",
        ),
    ]

    fig, axes = plt.subplots(3, 1, figsize=(18, 14), sharex=True)
    for ax, (title, key) in zip(axes, pair_info):
        vals = [averages.get(name, {}).get(key) for name in metric_names]
        ax.bar(x, [v if v is not None else 0 for v in vals])
        ax.set_title(title)
        ax.grid(axis="y", linestyle="--", alpha=0.3)
    axes[-1].set_xticks(x)
    axes[-1].set_xticklabels(metric_names, rotation=45, ha="right")
    fig.tight_layout()
    fig.savefig(deltas_plot_path, dpi=200)
    plt.close(fig)
