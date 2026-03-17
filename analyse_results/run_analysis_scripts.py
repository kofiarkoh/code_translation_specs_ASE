#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

# Edit this list to control which steps run, and in what order.
RUN_ORDER = [
    # {"type": "python", "path": "copy_rust_to_r_code_src.py"},
    {"type": "python", "path": "run_rust_translated_codes.py"},
    {"type": "python", "path": "analyze_translated_code_statuses.py"},
    {
        "type": "command",
        "name": "rust-code-analysis-cli",
        "cmd": ["rust-code-analysis-cli", "-m", "-p", "src", "-l", "rust", "-O", "json"],
        "cwd": "r_code",
        "stdout": "metrics.json",
    },
    {"type": "python", "path": "compare_rust_code_analysis_metrics.py"},
    {"type": "python", "path": "venn_pass_variants.py"},
    {"type": "python", "path": "venn_non_pass_variants.py"},
]

# If True, stop immediately when a script fails.
STOP_ON_ERROR = True


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
