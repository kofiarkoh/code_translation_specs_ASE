#!/usr/bin/env python3
"""
Analyze and plot attempt statuses for spec generation (validated specs).

Scans api_responses/open_ai/optimized_prompt@1/C/<SPEC_DIR>/p*/attempt_*/run_report.txt
and classifies each attempt as PASS, FAIL, COMPILE_FAILED, TIMEOUT, RUNTIME_ERROR,
or NO_SAMPLES.

Produces a grouped bar chart (one bar per spec directory at each attempt k)
with stacked status segments + log scale.

Usage:
    python repairs/plot_spec_generation_attempt_statuses.py
"""

from __future__ import annotations

import re
from collections import defaultdict
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SCRIPT_DIR = Path(__file__).resolve().parent
API_ROOT = PROJECT_ROOT / "api_responses" / "open_ai" / "optimized_prompt@1" / "C"

SPEC_DIRS: list[str] = [
    "validated_specs_WITH_GPT_gpt-5.2",
    # "validated_specs_WITH_GPT_gpt-4o-mini",
]

SPEC_DIR_LABELS: dict[str, str] = {
    "validated_specs_WITH_GPT_gpt-5.2":     "GPT-5.2",
    "validated_specs_WITH_GPT_gpt-4o-mini":  "GPT-4o-mini",
}

STATUS_SERIES = [
    ("PASS",           "#009E73"),
    ("FAIL",           "#D55E00"),
    ("RUNTIME_ERROR",  "#E69F00"),
    ("TIMEOUT",        "#0072B2"),
    ("COMPILE_FAILED", "#CC79A7"),
    ("NO_SAMPLES",     "#888888"),
]

OUTPUT_DIR = SCRIPT_DIR
FIGURE_WIDTH = 14
FIGURE_HEIGHT = 7
BASE_FONT_SIZE = 18
BAR_WIDTH = 0.3
LEGEND_COLUMNS = 3


def classify_run_report(report_path: Path) -> str:
    if not report_path.is_file():
        return "NO_SAMPLES"

    text = report_path.read_text(encoding="utf-8", errors="ignore")

    if "Status: NO_SAMPLES" in text:
        return "NO_SAMPLES"

    if "Compilation: FAILED" in text:
        return "COMPILE_FAILED"

    passed = 0
    failed = 0
    timed_out = 0
    runtime_errors = 0

    for line in text.splitlines():
        m = re.match(r"^Passed:\s*(\d+)", line)
        if m:
            passed = int(m.group(1))
        m = re.match(r"^Failed:\s*(\d+)", line)
        if m:
            failed = int(m.group(1))
        m = re.match(r"^Timed out:\s*(\d+)", line)
        if m:
            timed_out = int(m.group(1))
        m = re.match(r"^Runtime errors:\s*(\d+)", line)
        if m:
            runtime_errors = int(m.group(1))

    if timed_out > 0:
        return "TIMEOUT"
    if runtime_errors > 0:
        return "RUNTIME_ERROR"
    if failed > 0:
        return "FAIL"
    if passed > 0:
        return "PASS"
    return "NO_SAMPLES"


CUMULATIVE_STATUSES = {"PASS"}


def scan_spec_dir(spec_dir_name: str) -> dict[int, dict[str, int]]:
    """Return {attempt_number: {status: count}} with PASS accumulated across attempts."""
    spec_path = API_ROOT / spec_dir_name
    if not spec_path.is_dir():
        print(f"[WARN] Not found: {spec_path}")
        return {}

    raw_counts: dict[int, dict[str, int]] = defaultdict(lambda: {s: 0 for s, _ in STATUS_SERIES})

    for problem_dir in sorted(spec_path.iterdir()):
        if not problem_dir.is_dir() or not problem_dir.name.startswith("p"):
            continue
        for attempt_dir in sorted(problem_dir.glob("attempt_*")):
            if not attempt_dir.is_dir():
                continue
            try:
                attempt_num = int(attempt_dir.name.split("_")[1])
            except (IndexError, ValueError):
                continue
            status = classify_run_report(attempt_dir / "run_report.txt")
            raw_counts[attempt_num][status] += 1

    carried = {s: 0 for s in CUMULATIVE_STATUSES}
    cumulative: dict[int, dict[str, int]] = {}
    for attempt in sorted(raw_counts):
        row = dict(raw_counts[attempt])
        for s in CUMULATIVE_STATUSES:
            row[s] = carried[s] + row[s]
            carried[s] = row[s]
        cumulative[attempt] = row

    return cumulative


def main() -> None:
    all_data: dict[str, dict[int, dict[str, int]]] = {}
    all_attempts: set[int] = set()

    for spec_dir in SPEC_DIRS:
        data = scan_spec_dir(spec_dir)
        all_data[spec_dir] = data
        all_attempts.update(data.keys())
        label = SPEC_DIR_LABELS.get(spec_dir, spec_dir)
        print(f"\n{'=' * 60}")
        print(f"  {label} ({spec_dir})")
        print(f"{'=' * 60}")
        for attempt in sorted(data):
            print(f"  Attempt {attempt}:")
            for status, _ in STATUS_SERIES:
                count = data[attempt].get(status, 0)
                if count > 0:
                    print(f"    {status:<18} {count}")

    attempts = sorted(all_attempts)
    n_attempts = len(attempts)
    n_dirs = len(SPEC_DIRS)
    x = np.arange(n_attempts)
    offsets = [(i - (n_dirs - 1) / 2) * BAR_WIDTH for i in range(n_dirs)]

    plt.rcParams.update({"font.size": BASE_FONT_SIZE})
    fig, ax = plt.subplots(figsize=(FIGURE_WIDTH, FIGURE_HEIGHT))

    for offset, spec_dir in zip(offsets, SPEC_DIRS):
        data = all_data.get(spec_dir, {})
        bottoms = [0] * n_attempts

        for status, color in STATUS_SERIES:
            values = [data.get(a, {}).get(status, 0) for a in attempts]
            if all(v == 0 for v in values):
                continue
            label_text = status if spec_dir == SPEC_DIRS[0] else None
            ax.bar(
                x + offset,
                values,
                width=BAR_WIDTH,
                bottom=bottoms,
                label=label_text,
                color=color,
                edgecolor="white",
                linewidth=0.7,
            )
            for i, value in enumerate(values):
                if value > 0:
                    ax.text(
                        x[i] + offset,
                        bottoms[i] + value / 2,
                        str(value),
                        ha="center",
                        va="center",
                        fontsize=BASE_FONT_SIZE,
                        color="white",
                    )
            bottoms = [b + v for b, v in zip(bottoms, values)]

    # X-axis: spec dir labels per bar, attempt labels per group
    tick_positions = []
    tick_labels = []
    for i in range(n_attempts):
        for offset, spec_dir in zip(offsets, SPEC_DIRS):
            tick_positions.append(x[i] + offset)
            tick_labels.append(SPEC_DIR_LABELS.get(spec_dir, spec_dir))

    ax.set_xticks(tick_positions)
    ax.set_xticklabels(tick_labels, fontsize=max(BASE_FONT_SIZE - 6, 1))

    ax2 = ax.secondary_xaxis("bottom")
    ax2.set_xticks(list(x))
    ax2.set_xticklabels([f"k={a}" for a in attempts], fontsize=BASE_FONT_SIZE)
    ax2.tick_params(length=0, pad=25)

    ax.set_yscale("symlog", linthresh=1)
    ax.set_ylabel("Count (log scale)")
    ax.grid(axis="y", linestyle="--", alpha=0.3)
    ax.legend(
        loc="upper center",
        bbox_to_anchor=(0.5, 1.15),
        ncol=LEGEND_COLUMNS,
        frameon=False,
        fontsize=max(BASE_FONT_SIZE - 2, 1),
    )

    fig.tight_layout()
    out_png = OUTPUT_DIR / "spec_generation_attempt_status_dashboard.png"
    out_pdf = OUTPUT_DIR / "spec_generation_attempt_status_dashboard.pdf"
    fig.savefig(out_png, dpi=200)
    fig.savefig(out_pdf)
    plt.close(fig)
    print(f"\nSaved: {out_png}")
    print(f"Saved: {out_pdf}")


if __name__ == "__main__":
    main()
