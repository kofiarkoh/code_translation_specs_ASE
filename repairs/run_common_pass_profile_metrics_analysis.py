#!/usr/bin/env python3
from __future__ import annotations

import csv
import shutil
import subprocess
import sys
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "full_translation_loop_GPT_5_2"
DATASET_NAME = "optimized_prompt@1"

PROFILE_TO_COMPARE_FILENAME = {
    "code_and_spec": "translations_with_specs_and_code.rs",
    "code_only": "translations_with_code_only.rs",
    "specs_only": "translations_with_specs_only.rs",
}

PIPELINE_ROOT = PROJECT_ROOT / "repairs" / PIPELINE_NAME
ANALYSIS_ROOT = PROJECT_ROOT / "repairs" / f"{PIPELINE_NAME}_analysis" / DATASET_NAME
COMMON_PASS_CSV = PROJECT_ROOT / "repairs" / "common_pass_problems.csv"
R_CODE_ROOT = PROJECT_ROOT / "r_code_copy"
PREPARED_SRC_DIRNAME = "common_pass_intersection_src"
PREPARED_SRC_ROOT = R_CODE_ROOT / PREPARED_SRC_DIRNAME
METRICS_JSON = R_CODE_ROOT / f"{PIPELINE_NAME}_metrics.json"
COMPARE_OUTPUT_DIR = ANALYSIS_ROOT / "rust-code-analysis-common-pass"
COMPARE_SCRIPT = PROJECT_ROOT / "analyse_results" / "compare_rust_code_analysis_metrics.py"


def load_problem_ids() -> list[str]:
    if not COMMON_PASS_CSV.is_file():
        raise FileNotFoundError(f"Common pass CSV not found: {COMMON_PASS_CSV}")

    with COMMON_PASS_CSV.open("r", newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        return [row["problem_id"] for row in reader if row.get("problem_id", "").strip()]


def prepare_source_tree(problem_ids: list[str]) -> int:
    if PREPARED_SRC_ROOT.exists():
        shutil.rmtree(PREPARED_SRC_ROOT)
        print(f"[DEL] removed existing {PREPARED_SRC_ROOT}")
    PREPARED_SRC_ROOT.mkdir(parents=True, exist_ok=True)

    copied_files = 0
    for problem_id in sorted(problem_ids):
        target_problem_dir = PREPARED_SRC_ROOT / problem_id
        target_problem_dir.mkdir(parents=True, exist_ok=True)

        for profile_name, target_filename in PROFILE_TO_COMPARE_FILENAME.items():
            source_file = PIPELINE_ROOT / profile_name / DATASET_NAME / problem_id / f"{PIPELINE_NAME}.rs"
            if not source_file.is_file():
                print(f"[SKIP] missing source file: {source_file}")
                continue

            destination_file = target_problem_dir / target_filename
            shutil.copy2(source_file, destination_file)
            copied_files += 1

    return copied_files


def run_rust_code_analysis() -> None:
    if not R_CODE_ROOT.is_dir():
        raise FileNotFoundError(f"r_code directory not found: {R_CODE_ROOT}")

    command = [
        "rust-code-analysis-cli",
        "-m",
        "-p",
        PREPARED_SRC_DIRNAME,
        "-l",
        "rust",
        "-O",
        "json",
    ]

    print(f"[RUN] rust-code-analysis-cli (cwd={R_CODE_ROOT})")
    with METRICS_JSON.open("w", encoding="utf-8") as stdout_handle:
        completed = subprocess.run(command, cwd=R_CODE_ROOT, stdout=stdout_handle)

    if completed.returncode != 0:
        raise RuntimeError(f"rust-code-analysis-cli failed with exit code {completed.returncode}")

    print(f"[OK] rust-code-analysis-cli -> {METRICS_JSON}")


def run_metrics_compare() -> None:
    if not COMPARE_SCRIPT.is_file():
        raise FileNotFoundError(f"Compare script not found: {COMPARE_SCRIPT}")

    COMPARE_OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

    command = [
        sys.executable,
        str(COMPARE_SCRIPT),
        "--metrics",
        str(METRICS_JSON),
        "--output",
        str(COMPARE_OUTPUT_DIR / "metrics_comparison.md"),
        "--output-csv",
        str(COMPARE_OUTPUT_DIR / "metrics_comparison.csv"),
        "--output-csv-wide",
        str(COMPARE_OUTPUT_DIR / "metrics_comparison_wide.csv"),
        "--no-plots",
    ]

    print(f"[RUN] {COMPARE_SCRIPT}")
    completed = subprocess.run(command, cwd=PROJECT_ROOT)
    if completed.returncode != 0:
        raise RuntimeError(f"compare_rust_code_analysis_metrics.py failed with exit code {completed.returncode}")

    print(f"[OK] compare_rust_code_analysis_metrics.py -> {COMPARE_OUTPUT_DIR}")


def main() -> int:
    print(f"PIPELINE_ROOT:    {PIPELINE_ROOT}")
    print(f"DATASET_NAME:     {DATASET_NAME}")
    print(f"COMMON_PASS_CSV:  {COMMON_PASS_CSV}")
    print(f"PREPARED_SRC_ROOT:{PREPARED_SRC_ROOT}")
    print(f"METRICS_JSON:     {METRICS_JSON}")
    print(f"COMPARE_OUTPUT_DIR:{COMPARE_OUTPUT_DIR}")

    problem_ids = load_problem_ids()
    if not problem_ids:
        raise ValueError(f"No problem IDs found in: {COMMON_PASS_CSV}")

    copied_files = prepare_source_tree(problem_ids)
    print(f"[OK] prepared compare source tree -> {PREPARED_SRC_ROOT} (problem_count={len(problem_ids)}, copied_files={copied_files})")

    run_rust_code_analysis()
    run_metrics_compare()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
