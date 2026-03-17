#!/usr/bin/env python3
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

# Edit this list to control which scripts run, and in what order.
SCRIPT_ORDER = [
    # "copy_rust_to_r_code_src.py",
    "run_rust_translated_codes.py",
    "analyze_translated_code_statuses.py",
    "compare_rust_code_analysis_metrics.py",
    "venn_pass_variants.py",
    "venn_non_pass_variants.py",
]

# If True, stop immediately when a script fails.
STOP_ON_ERROR = True


def resolve_script_path(base_dir: Path, script_name: str) -> Path:
    script_path = Path(script_name)
    if not script_path.is_absolute():
        script_path = base_dir / script_path
    return script_path


def main() -> int:
    base_dir = Path(__file__).resolve().parent
    failures: list[tuple[str, int]] = []

    if not SCRIPT_ORDER:
        print("No scripts configured in SCRIPT_ORDER.")
        return 0

    for script_name in SCRIPT_ORDER:
        script_path = resolve_script_path(base_dir, script_name)
        if not script_path.is_file():
            print(f"[MISSING] {script_path}")
            failures.append((script_name, 1))
            if STOP_ON_ERROR:
                return 1
            continue

        print(f"[RUN] {script_path}")
        completed = subprocess.run([sys.executable, str(script_path)], cwd=base_dir)
        if completed.returncode != 0:
            print(f"[FAILED] {script_path} (exit code {completed.returncode})")
            failures.append((script_name, completed.returncode))
            if STOP_ON_ERROR:
                return completed.returncode
        else:
            print(f"[OK] {script_path}")

    if failures:
        print("Finished with failures:")
        for script_name, returncode in failures:
            print(f"- {script_name}: exit code {returncode}")
        return 1

    print("All scripts completed successfully.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
