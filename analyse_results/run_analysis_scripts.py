#!/usr/bin/env python3
from __future__ import annotations

import csv
import os
import shutil
import subprocess
import sys
from pathlib import Path

from dotenv import load_dotenv

# Edit this list to control which steps run, and in what order.
RUN_ORDER = [
    # {"type": "python", "path": "copy_rust_to_r_code_src.py"},
    {"type": "python", "path": "run_rust_translated_codes.py"},
    {"type": "python", "path": "analyze_translated_code_statuses.py"},
    {"type": "python", "path": "venn_pass_variants.py"},
    {"type": "prepare_pass_intersection_src"},
    {
        "type": "command",
        "name": "rust-code-analysis-cli",
        "cmd": ["rust-code-analysis-cli", "-m", "-p", "pass_intersection_src", "-l", "rust", "-O", "json"],
        "cwd": "r_code",
        "stdout": "metrics.json",
    },
    {"type": "python", "path": "compare_rust_code_analysis_metrics.py"},
    {"type": "python", "path": "venn_non_pass_variants.py"},
]

# If True, stop immediately when a script fails.
STOP_ON_ERROR = True
TARGET_INTERSECTION = (
    "translations_with_code_only.rs | "
    "translations_with_specs_and_code.rs | "
    "translations_with_specs_only.rs"
)
PASS_INTERSECTION_SRC_DIRNAME = "pass_intersection_src"


def resolve_script_path(base_dir: Path, script_name: str) -> Path:
    script_path = Path(script_name)
    if not script_path.is_absolute():
        script_path = base_dir / script_path
    return script_path


def run_python_step(base_dir: Path, step: dict[str, str]) -> tuple[str, int]:
    script_path = resolve_script_path(base_dir, step["path"])
    if not script_path.is_file():
        print(f"[MISSING] {script_path}")
        return step["path"], 1

    print(f"[RUN] {script_path}")
    completed = subprocess.run([sys.executable, str(script_path)], cwd=base_dir)
    if completed.returncode != 0:
        print(f"[FAILED] {script_path} (exit code {completed.returncode})")
    else:
        print(f"[OK] {script_path}")
    return step["path"], completed.returncode


def run_command_step(project_root: Path, step: dict[str, object]) -> tuple[str, int]:
    command = [str(part) for part in step["cmd"]]  # type: ignore[index]
    command_cwd = project_root / str(step["cwd"])
    stdout_path = command_cwd / str(step["stdout"])
    step_name = str(step.get("name", " ".join(command)))

    if not command_cwd.is_dir():
        print(f"[MISSING] command cwd does not exist: {command_cwd}")
        return step_name, 1

    print(f"[RUN] {step_name} (cwd={command_cwd})")
    with stdout_path.open("w", encoding="utf-8") as stdout_handle:
        completed = subprocess.run(command, cwd=command_cwd, stdout=stdout_handle)

    if completed.returncode != 0:
        print(f"[FAILED] {step_name} (exit code {completed.returncode})")
    else:
        print(f"[OK] {step_name} -> {stdout_path}")
    return step_name, completed.returncode


def prepare_pass_intersection_src(project_root: Path) -> tuple[str, int]:
    load_dotenv()
    dataset_name = os.environ.get("DATASET_NAME", "").strip()
    if not dataset_name:
        print("[MISSING] DATASET_NAME is not set.")
        return "prepare_pass_intersection_src", 1

    intersections_csv = project_root / "results" / dataset_name / "pass_venn_analysis" / "variant_intersections.csv"
    source_root = project_root / "r_code" / "src"
    target_root = project_root / "r_code" / PASS_INTERSECTION_SRC_DIRNAME

    if not intersections_csv.is_file():
        print(f"[MISSING] intersections CSV not found: {intersections_csv}")
        return "prepare_pass_intersection_src", 1
    if not source_root.is_dir():
        print(f"[MISSING] source root not found: {source_root}")
        return "prepare_pass_intersection_src", 1

    problem_ids: list[str] = []
    with intersections_csv.open("r", newline="", encoding="utf-8") as csv_file:
        reader = csv.DictReader(csv_file)
        for row in reader:
            if row.get("variants", "").strip() != TARGET_INTERSECTION:
                continue
            problem_ids_text = row.get("problem_ids", "").strip()
            if problem_ids_text:
                problem_ids = [problem_id for problem_id in problem_ids_text.split(";") if problem_id]
            break

    if not problem_ids:
        print(f"[MISSING] no problem IDs found for intersection: {TARGET_INTERSECTION}")
        return "prepare_pass_intersection_src", 1

    if target_root.exists():
        shutil.rmtree(target_root)
    target_root.mkdir(parents=True, exist_ok=True)

    copied = 0
    for problem_id in sorted(problem_ids):
        source_dir = source_root / problem_id
        target_dir = target_root / problem_id
        if not source_dir.is_dir():
            print(f"[SKIP] missing source problem dir: {source_dir}")
            continue
        shutil.copytree(source_dir, target_dir)
        copied += 1

    print(f"[OK] prepare_pass_intersection_src -> {target_root} (copied {copied} problem folders)")
    return "prepare_pass_intersection_src", 0


def main() -> int:
    base_dir = Path(__file__).resolve().parent
    project_root = base_dir.parent
    failures: list[tuple[str, int]] = []

    if not RUN_ORDER:
        print("No steps configured in RUN_ORDER.")
        return 0

    for step in RUN_ORDER:
        step_type = step["type"]
        if step_type == "python":
            step_name, returncode = run_python_step(base_dir, step)
        elif step_type == "command":
            step_name, returncode = run_command_step(project_root, step)
        elif step_type == "prepare_pass_intersection_src":
            step_name, returncode = prepare_pass_intersection_src(project_root)
        else:
            print(f"[MISSING] unsupported step type: {step_type}")
            failures.append((str(step), 1))
            if STOP_ON_ERROR:
                return 1
            continue

        if returncode != 0:
            failures.append((step_name, returncode))
            if STOP_ON_ERROR:
                return returncode

    if failures:
        print("Finished with failures:")
        for step_name, returncode in failures:
            print(f"- {step_name}: exit code {returncode}")
        return 1

    print("All scripts completed successfully.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
