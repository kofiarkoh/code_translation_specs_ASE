#!/usr/bin/env python3

import subprocess
import shutil
import csv
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]  # adjust depth to repo root

import sys
sys.path.insert(0, str(REPO_ROOT))
from clippy_analysis.common import COMMON_PROBLEM_IDS

BASE_INPUT  = REPO_ROOT / "repairs" / "translation_with_validated_specs_GPT_5_2"
BASE_OUTPUT = REPO_ROOT / "clippy_analysis"

TEMP_BASE     = BASE_OUTPUT / "temp_projects"
RAW_JSON_BASE = BASE_OUTPUT / "raw_json"
RAW_TXT_BASE  = BASE_OUTPUT / "raw_txt"
SUMMARY_DIR   = BASE_OUTPUT / "summary"
LOG_DIR       = BASE_OUTPUT / "logs"

SOURCE_DIRS = {
    "code_and_spec": BASE_INPUT / "code_and_spec" / "optimized_prompt@1",
    "code_only":     BASE_INPUT / "code_only"     / "optimized_prompt@1",
    "specs_only":    BASE_INPUT / "specs_only"    / "optimized_prompt@1",
}

CLIPPY_FLAGS = [
    "--all-targets", "--",
    "-W", "clippy::correctness",
    "-W", "clippy::suspicious",
    "-W", "clippy::complexity",
    "-W", "clippy::perf",
    "-W", "clippy::style",
]

CARGO_TOML_TEMPLATE = """\
[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
[dependencies]
"""


def make_dirs():
    for d in [TEMP_BASE, RAW_JSON_BASE, RAW_TXT_BASE, SUMMARY_DIR, LOG_DIR]:
        d.mkdir(parents=True, exist_ok=True)


def run_clippy_json(project_dir: Path, json_output: Path, json_stderr: Path) -> str:
    cmd = ["cargo", "clippy", "--message-format=json"] + CLIPPY_FLAGS
    result = subprocess.run(
        cmd,
        cwd=project_dir,
        stdout=json_output.open("w"),
        stderr=json_stderr.open("w"),
    )
    return "SUCCESS" if result.returncode == 0 else "FAIL"


def run_clippy_txt(project_dir: Path, txt_output: Path) -> str:
    # Touch main.rs to force re-lint
    (project_dir / "src" / "main.rs").touch()
    cmd = ["cargo", "clippy", "--color", "never"] + CLIPPY_FLAGS
    with txt_output.open("w") as out:
        result = subprocess.run(cmd, cwd=project_dir, stdout=out, stderr=subprocess.STDOUT)
    return "SUCCESS" if result.returncode == 0 else "FAIL"


def setup_cargo_project(project_dir: Path, source_rs: Path, name: str):
    if project_dir.exists():
        shutil.rmtree(project_dir)
    src_dir = project_dir / "src"
    src_dir.mkdir(parents=True)
    (project_dir / "Cargo.toml").write_text(CARGO_TOML_TEMPLATE.format(name=name))
    shutil.copy(source_rs, src_dir / "main.rs")


def main():
    make_dirs()

    common_ids = set(COMMON_PROBLEM_IDS)
    missing_dirs_log = LOG_DIR / "missing_dirs.log"
    summary_csv = SUMMARY_DIR / "clippy_run_summary.csv"

    print(f"Repo root:   {REPO_ROOT}")
    print(f"Base output: {BASE_OUTPUT}")
    print(f"Filtering to {len(common_ids)} common problem IDs\n")

    with summary_csv.open("w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=[
            "translation_type", "problem_id", "source_file",
            "json_status", "txt_status",
            "json_output", "txt_output", "json_stderr",
        ])
        writer.writeheader()

        for translation_type, input_dir in SOURCE_DIRS.items():
            json_out_dir  = RAW_JSON_BASE / translation_type
            txt_out_dir   = RAW_TXT_BASE  / translation_type
            temp_type_dir = TEMP_BASE     / translation_type

            for d in [json_out_dir, txt_out_dir, temp_type_dir]:
                d.mkdir(parents=True, exist_ok=True)

            if not input_dir.exists():
                msg = f"Skipping missing directory: {input_dir}"
                print(msg)
                with missing_dirs_log.open("a") as log:
                    log.write(msg + "\n")
                continue

            print(f"------------------------------------------")
            print(f"Processing: {translation_type}")
            print(f"------------------------------------------")

            problem_dirs = sorted(
                p for p in input_dir.iterdir()
                if p.is_dir() and p.name.startswith("p")
            )

            for problem_dir in problem_dirs:
                problem_id = problem_dir.name

                # ── Filter to common problem IDs ──────────────
                if problem_id not in common_ids:
                    continue
                # ──────────────────────────────────────────────

                rs_files = sorted(problem_dir.glob("*.rs"))
                if not rs_files:
                    rs_files = sorted(problem_dir.rglob("*.rs"))

                if not rs_files:
                    writer.writerow({
                        "translation_type": translation_type,
                        "problem_id": problem_id,
                        "source_file": "",
                        "json_status": "NO_RS_FILE",
                        "txt_status":  "NO_RS_FILE",
                        "json_output": "", "txt_output": "", "json_stderr": "",
                    })
                    continue

                source_rs       = rs_files[0]
                project_dir     = temp_type_dir / problem_id
                json_output     = json_out_dir  / f"{problem_id}.jsonl"
                txt_output      = txt_out_dir   / f"{problem_id}.txt"
                json_stderr     = json_out_dir  / f"{problem_id}.stderr.txt"
                cargo_name      = f"{translation_type}_{problem_id}"

                setup_cargo_project(project_dir, source_rs, cargo_name)
                json_status = run_clippy_json(project_dir, json_output, json_stderr)
                txt_status  = run_clippy_txt(project_dir, txt_output)

                print(f"  [{translation_type}][{problem_id}] json={json_status} txt={txt_status}")

                writer.writerow({
                    "translation_type": translation_type,
                    "problem_id":       problem_id,
                    "source_file":      str(source_rs),
                    "json_status":      json_status,
                    "txt_status":       txt_status,
                    "json_output":      str(json_output),
                    "txt_output":       str(txt_output),
                    "json_stderr":      str(json_stderr),
                })

    print(f"\nSummary written to: {summary_csv}")


if __name__ == "__main__":
    main()