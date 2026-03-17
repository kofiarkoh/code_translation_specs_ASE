#!/usr/bin/env python3
"""Compare Rust metrics across three Rust translation variants per folder.

Default input:
  r_code/metrics.json

The input can be either:
- newline-delimited JSON (one object per line), or
- a single JSON object/list.

Outputs:
- Markdown report printed to stdout
- Optional markdown file via --output (default: metrics_comparison.md)
- All output files are written under the `COMPARE_RUST_METRICS_OUTPUT_DIR` env var
"""

from __future__ import annotations

import argparse
import csv
import json
import os
import re
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Tuple

from dotenv import load_dotenv

try:
    from analyse_results.plot_rust_metric_averages import plot_average_graphs
except ModuleNotFoundError:
    from plot_rust_metric_averages import plot_average_graphs

PROJECT_ROOT = Path(__file__).resolve().parent.parent

TARGET_FILES = {
    "translations_with_specs_and_code.rs",
    "translations_with_specs_only.rs",
    "translations_with_code_only.rs",
}
FILE_LABELS: Dict[str, str] = {
    "translations_with_specs_and_code.rs": "specs_and_code",
    "translations_with_specs_only.rs": "specs_only",
    "translations_with_code_only.rs": "code_only",
}
PAIRWISE_COMPARISONS: List[Tuple[str, str]] = [
    ("translations_with_specs_and_code.rs", "translations_with_specs_only.rs"),
    ("translations_with_specs_and_code.rs", "translations_with_code_only.rs"),
    ("translations_with_specs_only.rs", "translations_with_code_only.rs"),
]
NAME_RE = re.compile(r"(?:^|/)(p\d+)/([^/]+\.rs)$")

# (display_name, key_path)
METRIC_FIELDS: List[Tuple[str, Tuple[str, ...]]] = [
    ("MI (original)", ("mi", "mi_original")),
    ("MI (SEI)", ("mi", "mi_sei")),
    ("Cyclomatic (sum)", ("cyclomatic", "sum")),
    ("Cognitive (sum)", ("cognitive", "sum")),
    ("Halstead Volume", ("halstead", "volume")),
    ("Halstead Effort", ("halstead", "effort")),
    ("SLOC", ("loc", "sloc")),
    ("PLOC", ("loc", "ploc")),
    ("LLOC", ("loc", "lloc")),
    ("Comments (CLOC)", ("loc", "cloc")),
    ("Blank Lines", ("loc", "blank")),
    ("Functions", ("nom", "functions")),
    ("Closures", ("nom", "closures")),
    ("Exits", ("nexits", "sum")),
]

# True => higher is better, False => lower is better
METRIC_BETTER_HIGHER: Dict[str, bool] = {
    "MI (original)": True,
    "MI (SEI)": True,
    "Cyclomatic (sum)": False,
    "Cognitive (sum)": False,
    "Halstead Volume": False,
    "Halstead Effort": False,
    "SLOC": False,
    "PLOC": False,
    "LLOC": False,
    "Comments (CLOC)": True,
    "Blank Lines": False,
    "Functions": False,
    "Closures": False,
    "Exits": False,
}

# If True, the markdown report only includes the overall averages section.
MARKDOWN_AVERAGE_ONLY = False
# If True, write plots for overall-average metrics only.
PLOT_AVERAGE_ONLY = True
AVERAGE_VALUES_PLOT = "metrics_comparison_averages.png"
AVERAGE_DELTAS_PLOT = "metrics_comparison_average_deltas.png"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Compare Rust metrics across three Rust translation variants.")
    parser.add_argument(
        "--metrics",
        default="r_code/metrics.json",
        help="Path to metrics JSON/NDJSON file (default: r_code/metrics.json)",
    )
    parser.add_argument(
        "--output",
        default="metrics_comparison.md",
        help="Where to write markdown report (default: metrics_comparison.md)",
    )
    parser.add_argument(
        "--output-csv",
        default="metrics_comparison.csv",
        help="Where to write long-form CSV report (default: metrics_comparison.csv)",
    )
    parser.add_argument(
        "--output-csv-wide",
        default="metrics_comparison_wide.csv",
        help="Where to write wide CSV report (default: metrics_comparison_wide.csv)",
    )
    parser.add_argument(
        "--green-box",
        choices=["all", "average", "none"],
        default="all",
        help="Where to apply Markdown highlight markers for better values (default: all).",
    )
    return parser.parse_args()


def resolve_output_path(output_dir: Path, raw_path: str) -> Path:
    path = Path(raw_path)
    if path.is_absolute():
        return path
    return output_dir / path


def get_output_dir() -> Path:
    load_dotenv()
    base_path = Path(os.environ.get("DATASET_NAME", "."))
    full_path = PROJECT_ROOT / "results" / base_path / "rust-code-analysis"
    return full_path


def load_records(path: Path) -> List[Dict[str, Any]]:
    text = path.read_text(encoding="utf-8")

    # Try normal JSON first.
    try:
        obj = json.loads(text)
        if isinstance(obj, list):
            return [x for x in obj if isinstance(x, dict)]
        if isinstance(obj, dict):
            return [obj]
        return []
    except json.JSONDecodeError:
        pass

    # Fallback: NDJSON (one JSON object per line).
    records: List[Dict[str, Any]] = []
    for i, line in enumerate(text.splitlines(), 1):
        line = line.strip()
        if not line:
            continue
        try:
            obj = json.loads(line)
        except json.JSONDecodeError as exc:
            raise ValueError(f"Invalid JSON on line {i}: {exc}") from exc
        if isinstance(obj, dict):
            records.append(obj)
    return records


def nested_get(data: Dict[str, Any], path: Iterable[str]) -> Optional[float]:
    cur: Any = data
    for key in path:
        if not isinstance(cur, dict) or key not in cur:
            return None
        cur = cur[key]
    if isinstance(cur, (int, float)):
        return float(cur)
    return None


def fmt_num(value: Optional[float]) -> str:
    if value is None:
        return "N/A"
    if abs(value - round(value)) < 1e-12:
        return str(int(round(value)))
    return f"{value:.4f}"


def fmt_green_box(value: Optional[float]) -> str:
    """Render highlighted value with GitHub-compatible markdown."""
    inner = fmt_num(value)
    return f"**{inner} ✅**"


def left_is_better(metric_name: str, left_val: Optional[float], right_val: Optional[float]) -> bool:
    if left_val is None or right_val is None:
        return False
    higher_better = METRIC_BETTER_HIGHER.get(metric_name, False)
    if higher_better:
        return left_val > right_val
    return left_val < right_val


def _should_apply_green(green_box_mode: str, is_average_row: bool) -> bool:
    if green_box_mode == "none":
        return False
    if green_box_mode == "all":
        return True
    return is_average_row


def best_value(metric_name: str, values: Dict[str, Optional[float]]) -> Optional[float]:
    present = [v for v in values.values() if v is not None]
    if not present:
        return None
    if METRIC_BETTER_HIGHER.get(metric_name, False):
        return max(present)
    return min(present)


def fmt_variant_cells(
    metric_name: str,
    values: Dict[str, Optional[float]],
    green_box_mode: str,
    is_average_row: bool,
) -> Dict[str, str]:
    formatted = {name: fmt_num(value) for name, value in values.items()}
    if not _should_apply_green(green_box_mode, is_average_row):
        return formatted
    winner = best_value(metric_name, values)
    if winner is None:
        return formatted
    winners = [name for name, value in values.items() if value == winner]
    if len(winners) != 1:
        return formatted
    winner_name = winners[0]
    formatted[winner_name] = fmt_green_box(values[winner_name])
    return formatted


def fmt_delta(left_val: Optional[float], right_val: Optional[float]) -> str:
    if left_val is None or right_val is None:
        return "N/A"
    d = left_val - right_val
    sign = "+" if d > 0 else ""
    if abs(d - round(d)) < 1e-12:
        return f"{sign}{int(round(d))}"
    return f"{sign}{d:.4f}"


def change_label(left_val: Optional[float], right_val: Optional[float]) -> str:
    if left_val is None or right_val is None:
        return "N/A"
    if left_val > right_val:
        return "Higher"
    if left_val < right_val:
        return "Lower"
    return "Same"


def compute_averages(by_folder: Dict[str, Dict[str, Dict[str, Any]]]) -> Dict[str, Dict[str, Optional[float]]]:
    out: Dict[str, Dict[str, Optional[float]]] = {}
    for display_name, key_path in METRIC_FIELDS:
        variant_vals: Dict[str, List[float]] = {name: [] for name in TARGET_FILES}
        pair_delta_vals: Dict[str, List[float]] = {
            f"{left}__minus__{right}": [] for left, right in PAIRWISE_COMPARISONS
        }

        for folder in sorted(by_folder.keys()):
            files = by_folder[folder]
            metric_values: Dict[str, Optional[float]] = {}
            missing_variant = False
            for variant in TARGET_FILES:
                variant_metrics = files.get(variant)
                if variant_metrics is None:
                    missing_variant = True
                    break
                metric_values[variant] = nested_get(variant_metrics, key_path)
            if missing_variant:
                continue
            if any(value is None for value in metric_values.values()):
                continue
            for variant, value in metric_values.items():
                variant_vals[variant].append(value if value is not None else 0.0)
            for left, right in PAIRWISE_COMPARISONS:
                left_val = metric_values[left]
                right_val = metric_values[right]
                if left_val is not None and right_val is not None:
                    pair_delta_vals[f"{left}__minus__{right}"].append(left_val - right_val)

        result: Dict[str, Optional[float]] = {}
        for variant in TARGET_FILES:
            values = variant_vals[variant]
            result[f"{variant}__avg"] = (sum(values) / len(values)) if values else None
        for left, right in PAIRWISE_COMPARISONS:
            key = f"{left}__minus__{right}"
            deltas = pair_delta_vals[key]
            result[f"{key}__avg"] = (sum(deltas) / len(deltas)) if deltas else None
        out[display_name] = result
    return out


def write_long_csv(
    by_folder: Dict[str, Dict[str, Dict[str, Any]]],
    output_path: Path,
    averages: Dict[str, Dict[str, Optional[float]]],
) -> None:
    with output_path.open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow([
            "folder",
            "metric",
            "specs_and_code",
            "specs_only",
            "code_only",
            "delta_specs_and_code_minus_specs_only",
            "change_specs_and_code_vs_specs_only",
            "delta_specs_and_code_minus_code_only",
            "change_specs_and_code_vs_code_only",
            "delta_specs_only_minus_code_only",
            "change_specs_only_vs_code_only",
        ])

        for folder in sorted(by_folder.keys()):
            files = by_folder[folder]
            variant_metrics = {variant: files.get(variant) for variant in TARGET_FILES}
            if any(variant_metrics[variant] is None for variant in TARGET_FILES):
                continue

            for display_name, key_path in METRIC_FIELDS:
                values = {
                    variant: nested_get(variant_metrics[variant], key_path)  # type: ignore[arg-type]
                    for variant in TARGET_FILES
                }
                w.writerow(
                    [
                        folder,
                        display_name,
                        "" if values["translations_with_specs_and_code.rs"] is None else values["translations_with_specs_and_code.rs"],
                        "" if values["translations_with_specs_only.rs"] is None else values["translations_with_specs_only.rs"],
                        "" if values["translations_with_code_only.rs"] is None else values["translations_with_code_only.rs"],
                        "" if values["translations_with_specs_and_code.rs"] is None or values["translations_with_specs_only.rs"] is None else values["translations_with_specs_and_code.rs"] - values["translations_with_specs_only.rs"],
                        change_label(values["translations_with_specs_and_code.rs"], values["translations_with_specs_only.rs"]),
                        "" if values["translations_with_specs_and_code.rs"] is None or values["translations_with_code_only.rs"] is None else values["translations_with_specs_and_code.rs"] - values["translations_with_code_only.rs"],
                        change_label(values["translations_with_specs_and_code.rs"], values["translations_with_code_only.rs"]),
                        "" if values["translations_with_specs_only.rs"] is None or values["translations_with_code_only.rs"] is None else values["translations_with_specs_only.rs"] - values["translations_with_code_only.rs"],
                        change_label(values["translations_with_specs_only.rs"], values["translations_with_code_only.rs"]),
                    ]
                )

        # Cross-folder averages row per metric.
        for display_name, _ in METRIC_FIELDS:
            avg = averages.get(display_name, {})
            w.writerow(
                [
                    "__AVERAGE__",
                    display_name,
                    "" if avg.get("translations_with_specs_and_code.rs__avg") is None else avg.get("translations_with_specs_and_code.rs__avg"),
                    "" if avg.get("translations_with_specs_only.rs__avg") is None else avg.get("translations_with_specs_only.rs__avg"),
                    "" if avg.get("translations_with_code_only.rs__avg") is None else avg.get("translations_with_code_only.rs__avg"),
                    "" if avg.get("translations_with_specs_and_code.rs__minus__translations_with_specs_only.rs__avg") is None else avg.get("translations_with_specs_and_code.rs__minus__translations_with_specs_only.rs__avg"),
                    change_label(avg.get("translations_with_specs_and_code.rs__avg"), avg.get("translations_with_specs_only.rs__avg")),
                    "" if avg.get("translations_with_specs_and_code.rs__minus__translations_with_code_only.rs__avg") is None else avg.get("translations_with_specs_and_code.rs__minus__translations_with_code_only.rs__avg"),
                    change_label(avg.get("translations_with_specs_and_code.rs__avg"), avg.get("translations_with_code_only.rs__avg")),
                    "" if avg.get("translations_with_specs_only.rs__minus__translations_with_code_only.rs__avg") is None else avg.get("translations_with_specs_only.rs__minus__translations_with_code_only.rs__avg"),
                    change_label(avg.get("translations_with_specs_only.rs__avg"), avg.get("translations_with_code_only.rs__avg")),
                ]
            )


def write_wide_csv(
    by_folder: Dict[str, Dict[str, Dict[str, Any]]],
    output_path: Path,
    averages: Dict[str, Dict[str, Optional[float]]],
) -> None:
    headers = ["folder"]
    for display_name, _ in METRIC_FIELDS:
        base = display_name.lower().replace(" ", "_").replace("(", "").replace(")", "").replace("/", "_").replace("-", "_")
        headers.extend(
            [
                f"{base}__specs_and_code",
                f"{base}__specs_only",
                f"{base}__code_only",
                f"{base}__delta_specs_and_code_minus_specs_only",
                f"{base}__change_specs_and_code_vs_specs_only",
                f"{base}__delta_specs_and_code_minus_code_only",
                f"{base}__change_specs_and_code_vs_code_only",
                f"{base}__delta_specs_only_minus_code_only",
                f"{base}__change_specs_only_vs_code_only",
            ]
        )

    with output_path.open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(headers)

        for folder in sorted(by_folder.keys()):
            files = by_folder[folder]
            variant_metrics = {variant: files.get(variant) for variant in TARGET_FILES}

            row: List[Any] = [folder]
            for _, key_path in METRIC_FIELDS:
                if any(variant_metrics[variant] is None for variant in TARGET_FILES):
                    row.extend(["", "", "", "", "N/A", "", "N/A", "", "N/A"])
                    continue

                values = {
                    variant: nested_get(variant_metrics[variant], key_path)  # type: ignore[arg-type]
                    for variant in TARGET_FILES
                }
                row.extend(
                    [
                        "" if values["translations_with_specs_and_code.rs"] is None else values["translations_with_specs_and_code.rs"],
                        "" if values["translations_with_specs_only.rs"] is None else values["translations_with_specs_only.rs"],
                        "" if values["translations_with_code_only.rs"] is None else values["translations_with_code_only.rs"],
                        "" if values["translations_with_specs_and_code.rs"] is None or values["translations_with_specs_only.rs"] is None else values["translations_with_specs_and_code.rs"] - values["translations_with_specs_only.rs"],
                        change_label(values["translations_with_specs_and_code.rs"], values["translations_with_specs_only.rs"]),
                        "" if values["translations_with_specs_and_code.rs"] is None or values["translations_with_code_only.rs"] is None else values["translations_with_specs_and_code.rs"] - values["translations_with_code_only.rs"],
                        change_label(values["translations_with_specs_and_code.rs"], values["translations_with_code_only.rs"]),
                        "" if values["translations_with_specs_only.rs"] is None or values["translations_with_code_only.rs"] is None else values["translations_with_specs_only.rs"] - values["translations_with_code_only.rs"],
                        change_label(values["translations_with_specs_only.rs"], values["translations_with_code_only.rs"]),
                    ]
                )
            w.writerow(row)

        avg_row: List[Any] = ["__AVERAGE__"]
        for display_name, _ in METRIC_FIELDS:
            avg = averages.get(display_name, {})
            avg_row.extend(
                [
                    "" if avg.get("translations_with_specs_and_code.rs__avg") is None else avg.get("translations_with_specs_and_code.rs__avg"),
                    "" if avg.get("translations_with_specs_only.rs__avg") is None else avg.get("translations_with_specs_only.rs__avg"),
                    "" if avg.get("translations_with_code_only.rs__avg") is None else avg.get("translations_with_code_only.rs__avg"),
                    "" if avg.get("translations_with_specs_and_code.rs__minus__translations_with_specs_only.rs__avg") is None else avg.get("translations_with_specs_and_code.rs__minus__translations_with_specs_only.rs__avg"),
                    change_label(avg.get("translations_with_specs_and_code.rs__avg"), avg.get("translations_with_specs_only.rs__avg")),
                    "" if avg.get("translations_with_specs_and_code.rs__minus__translations_with_code_only.rs__avg") is None else avg.get("translations_with_specs_and_code.rs__minus__translations_with_code_only.rs__avg"),
                    change_label(avg.get("translations_with_specs_and_code.rs__avg"), avg.get("translations_with_code_only.rs__avg")),
                    "" if avg.get("translations_with_specs_only.rs__minus__translations_with_code_only.rs__avg") is None else avg.get("translations_with_specs_only.rs__minus__translations_with_code_only.rs__avg"),
                    change_label(avg.get("translations_with_specs_only.rs__avg"), avg.get("translations_with_code_only.rs__avg")),
                ]
            )
        w.writerow(avg_row)


def collect_pairs(records: List[Dict[str, Any]]) -> Dict[str, Dict[str, Dict[str, Any]]]:
    by_folder: Dict[str, Dict[str, Dict[str, Any]]] = {}

    for rec in records:
        name = rec.get("name")
        if not isinstance(name, str):
            continue
        m = NAME_RE.search(name)
        if not m:
            continue
        folder, filename = m.group(1), m.group(2)
        if filename not in TARGET_FILES:
            continue
        by_folder.setdefault(folder, {})[filename] = rec.get("metrics", {}) if isinstance(rec.get("metrics"), dict) else {}

    return by_folder


def build_report(
    by_folder: Dict[str, Dict[str, Dict[str, Any]]],
    source_path: Path,
    averages: Dict[str, Dict[str, Optional[float]]],
    green_box_mode: str,
) -> str:
    lines: List[str] = []
    lines.append("# Rust Metrics Comparison")
    lines.append("")
    lines.append(f"Source: `{source_path}`")
    lines.append("")

    if not by_folder:
        lines.append("No matching three-variant Rust metric records found.")
        return "\n".join(lines) + "\n"

    if not MARKDOWN_AVERAGE_ONLY:
        missing_rows: List[Tuple[str, str]] = []

        for folder in sorted(by_folder.keys()):
            files = by_folder[folder]
            variant_metrics = {variant: files.get(variant) for variant in TARGET_FILES}

            lines.append(f"## {folder}")
            lines.append("")

            if any(variant_metrics[variant] is None for variant in TARGET_FILES):
                missing = []
                for variant in TARGET_FILES:
                    if variant_metrics[variant] is None:
                        missing.append(variant)
                missing_rows.append((folder, ", ".join(missing)))
                lines.append(f"Missing metrics for: {', '.join(missing)}")
                lines.append("")
                continue

            lines.append("| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |")
            lines.append("|---|---:|---:|---:|---:|---|---:|---|---:|---|")

            for display_name, key_path in METRIC_FIELDS:
                values = {
                    variant: nested_get(variant_metrics[variant], key_path)  # type: ignore[arg-type]
                    for variant in TARGET_FILES
                }
                display_cells = fmt_variant_cells(display_name, values, green_box_mode, False)
                lines.append(
                    f"| {display_name} | "
                    f"{display_cells['translations_with_specs_and_code.rs']} | "
                    f"{display_cells['translations_with_specs_only.rs']} | "
                    f"{display_cells['translations_with_code_only.rs']} | "
                    f"{fmt_delta(values['translations_with_specs_and_code.rs'], values['translations_with_specs_only.rs'])} | "
                    f"{change_label(values['translations_with_specs_and_code.rs'], values['translations_with_specs_only.rs'])} | "
                    f"{fmt_delta(values['translations_with_specs_and_code.rs'], values['translations_with_code_only.rs'])} | "
                    f"{change_label(values['translations_with_specs_and_code.rs'], values['translations_with_code_only.rs'])} | "
                    f"{fmt_delta(values['translations_with_specs_only.rs'], values['translations_with_code_only.rs'])} | "
                    f"{change_label(values['translations_with_specs_only.rs'], values['translations_with_code_only.rs'])} |"
                )

            lines.append("")

        if missing_rows:
            lines.append("## Missing File Metrics")
            lines.append("")
            lines.append("| Folder | Missing |")
            lines.append("|---|---|")
            for folder, missing in missing_rows:
                lines.append(f"| {folder} | {missing} |")
            lines.append("")

    lines.append("## Overall Averages")
    lines.append("")
    lines.append("| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |")
    lines.append("|---|---:|---:|---:|---:|---|---:|---|---:|---|")
    for display_name, _ in METRIC_FIELDS:
        avg = averages.get(display_name, {})
        avg_values = {
            variant: avg.get(f"{variant}__avg") for variant in TARGET_FILES
        }
        display_cells = fmt_variant_cells(display_name, avg_values, green_box_mode, True)
        lines.append(
            f"| {display_name} | "
            f"{display_cells['translations_with_specs_and_code.rs']} | "
            f"{display_cells['translations_with_specs_only.rs']} | "
            f"{display_cells['translations_with_code_only.rs']} | "
            f"{fmt_num(avg.get('translations_with_specs_and_code.rs__minus__translations_with_specs_only.rs__avg'))} | "
            f"{change_label(avg.get('translations_with_specs_and_code.rs__avg'), avg.get('translations_with_specs_only.rs__avg'))} | "
            f"{fmt_num(avg.get('translations_with_specs_and_code.rs__minus__translations_with_code_only.rs__avg'))} | "
            f"{change_label(avg.get('translations_with_specs_and_code.rs__avg'), avg.get('translations_with_code_only.rs__avg'))} | "
            f"{fmt_num(avg.get('translations_with_specs_only.rs__minus__translations_with_code_only.rs__avg'))} | "
            f"{change_label(avg.get('translations_with_specs_only.rs__avg'), avg.get('translations_with_code_only.rs__avg'))} |"
        )
    lines.append("")

    return "\n".join(lines) + "\n"
def main() -> int:
    args = parse_args()
    metrics_path = Path(args.metrics)
    if not metrics_path.is_absolute():
        metrics_path = PROJECT_ROOT / metrics_path
    output_dir = get_output_dir()
    output_path = resolve_output_path(output_dir, args.output)
    output_csv_path = resolve_output_path(output_dir, args.output_csv)
    output_csv_wide_path = resolve_output_path(output_dir, args.output_csv_wide)

    if not metrics_path.exists():
        raise FileNotFoundError(f"Metrics file not found: {metrics_path}")

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_csv_path.parent.mkdir(parents=True, exist_ok=True)
    output_csv_wide_path.parent.mkdir(parents=True, exist_ok=True)

    records = load_records(metrics_path)
    by_folder = collect_pairs(records)
    averages = compute_averages(by_folder)
    report = build_report(by_folder, metrics_path, averages, args.green_box)
    values_plot_path = output_path.with_name(AVERAGE_VALUES_PLOT)
    deltas_plot_path = output_path.with_name(AVERAGE_DELTAS_PLOT)

    output_path.write_text(report, encoding="utf-8")
    write_long_csv(by_folder, output_csv_path, averages)
    write_wide_csv(by_folder, output_csv_wide_path, averages)
    if PLOT_AVERAGE_ONLY:
        plot_average_graphs(
            averages,
            METRIC_FIELDS,
            METRIC_BETTER_HIGHER,
            values_plot_path,
            deltas_plot_path,
        )
    print(report)
    print(f"Wrote report to: {output_path.resolve()}")
    print(f"Wrote CSV (long) to: {output_csv_path.resolve()}")
    print(f"Wrote CSV (wide) to: {output_csv_wide_path.resolve()}")
    if PLOT_AVERAGE_ONLY:
        print(f"Wrote average values plot to: {values_plot_path.resolve()}")
        print(f"Wrote average deltas plot to: {deltas_plot_path.resolve()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
