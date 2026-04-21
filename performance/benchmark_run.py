#!/usr/bin/env python3

import subprocess
import json
import csv
import shutil
from pathlib import Path

REPO_ROOT = Path(".").resolve()

import sys
sys.path.insert(0, str(REPO_ROOT))
from clippy_analysis.common import COMMON_PROBLEM_IDS

RUST_BASE   = REPO_ROOT / "repairs" / "translation_with_validated_specs_GPT_5_2"
TEST_BASE   = REPO_ROOT / "collected_dataset" / "problem_test_inputs_and_outputs"
BENCH_DIR   = REPO_ROOT / "performance" / "benchmark"
TEMP_BASE   = BENCH_DIR / "temp_projects"
BIN_BASE    = BENCH_DIR / "binaries"
RAW_DIR     = BENCH_DIR / "raw_json"
SUMMARY_DIR = BENCH_DIR / "summary"
LOG_DIR     = BENCH_DIR / "logs"

OUT_CSV     = SUMMARY_DIR / "performance_summary.csv"

TRANSLATION_DIRS = {
    "specs_only":    RUST_BASE / "specs_only"    / "optimized_prompt@1",
    "code_only":     RUST_BASE / "code_only"     / "optimized_prompt@1",
    "code_and_spec": RUST_BASE / "code_and_spec" / "optimized_prompt@1",
}

WARMUP_RUNS = 5
BENCH_RUNS  = 50

CARGO_TOML = """\
[package]
name = "{name}"
version = "0.1.0"
edition = "2021"
[dependencies]
"""


# ── helpers ──────────────────────────────────────────────────────────────────

def find_rs(problem_dir: Path) -> Path | None:
    files = sorted(problem_dir.glob("*.rs")) or sorted(problem_dir.rglob("*.rs"))
    return files[0] if files else None


def find_test_files(problem_id: str) -> list[tuple[Path, Path]]:
    """
    Returns list of (input_path, output_path) pairs for a problem.
    Naming convention: p00004_sample1_input.txt / p00004_sample1_output.txt
    """
    problem_test_dir = TEST_BASE / problem_id
    if not problem_test_dir.exists():
        print("Corresponding test not found")
        return []

    pairs = []
    for inp in sorted(problem_test_dir.glob(f"{problem_id}*_input.txt")):
        out = inp.parent / inp.name.replace("_input.txt", "_output.txt")
        if out.exists():
            pairs.append((inp, out))
    return pairs


def setup_temp_project(temp_dir: Path, rs_file: Path, tt: str,
                       problem_id: str, test_pairs: list) -> None:
    """Create a Cargo project with source + test files."""
    if temp_dir.exists():
        shutil.rmtree(temp_dir)

    src_dir     = temp_dir / "src"
    inputs_dir  = temp_dir / "inputs"
    outputs_dir = temp_dir / "outputs"

    for d in [src_dir, inputs_dir, outputs_dir]:
        d.mkdir(parents=True)

    (temp_dir / "Cargo.toml").write_text(
        CARGO_TOML.format(name=f"{tt}_{problem_id}"), encoding="utf-8"
    )
    shutil.copy(rs_file, src_dir / "main.rs")

    for inp, out in test_pairs:
        shutil.copy(inp, inputs_dir / inp.name)
        shutil.copy(out, outputs_dir / out.name)


def build_binary(temp_dir: Path, out_bin: Path, log: Path) -> bool:
    """cargo build --release and copy binary to out_bin."""
    result = subprocess.run(
        ["cargo", "build", "--release"],
        cwd=temp_dir,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        with log.open("a") as f:
            f.write(f"[{out_bin.parent.parent.name}/{out_bin.name}]\n")
            f.write(result.stderr + "\n")
        return False

    # cargo puts the binary at target/release/<name>
    name    = temp_dir.name  # e.g. "specs_only_p00004" — but Cargo uses package name
    # Package name is set in Cargo.toml as tt_problemid (underscores)
    package = temp_dir.name  # already set that way in CARGO_TOML
    built   = temp_dir / "target" / "release" / package

    if not built.exists():
        # fallback: find any binary in target/release
        candidates = [
            p for p in (temp_dir / "target" / "release").iterdir()
            if p.is_file() and not p.suffix and not p.name.startswith(".")
        ]
        if not candidates:
            return False
        built = candidates[0]

    out_bin.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy(built, out_bin)
    out_bin.chmod(0o755)
    return True


def run_hyperfine(problem_id: str,
                  binaries: dict[str, Path],
                  input_file: Path,
                  json_out: Path,
                  missing_log: Path) -> bool:

    cmd = [
        "hyperfine",
        "--warmup",      str(WARMUP_RUNS),
        "--runs",        str(BENCH_RUNS),
        "--export-json", str(json_out),
        "--input",       str(input_file),
        "--style",       "none",
    ]
    for label, bin_path in binaries.items():
        cmd += ["--command-name", label, str(bin_path)]

    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        with missing_log.open("a") as f:
            f.write(f"[{problem_id}] hyperfine error:\n{result.stderr}\n")
        return False
    return True


def parse_json(json_path: Path) -> list[dict]:
    data = json.loads(json_path.read_text())
    rows = []
    for r in data["results"]:
        rows.append({
            "command":  r["command"],
            "mean_s":   round(r["mean"],   6),
            "stddev_s": round(r["stddev"], 6),
            "median_s": round(r["median"], 6),
            "min_s":    round(r["min"],    6),
            "max_s":    round(r["max"],    6),
        })
    return rows


# ── main ─────────────────────────────────────────────────────────────────────

def main():
    for d in [TEMP_BASE, BIN_BASE, RAW_DIR, SUMMARY_DIR, LOG_DIR]:
        d.mkdir(parents=True, exist_ok=True)

    build_log   = LOG_DIR / "build_failures.log"
    missing_log = LOG_DIR / "missing_inputs.log"
    build_log.write_text("")
    missing_log.write_text("")

    common_ids = sorted(set(COMMON_PROBLEM_IDS))
    print(f"Problems: {len(common_ids)}  |  warmup={WARMUP_RUNS}  runs={BENCH_RUNS}\n")

    # ── Phase 1: build ───────────────────────────────────────────────
    print("=== Phase 1: building binaries ===")
    built: dict[str, dict[str, Path]] = {}  # problem_id -> {tt -> bin_path}

    for problem_id in common_ids:
        built[problem_id] = {}
        test_pairs = find_test_files(problem_id)

        for tt, base_dir in TRANSLATION_DIRS.items():
            problem_dir = base_dir / problem_id
            if not problem_dir.exists():
                continue

            rs_file = find_rs(problem_dir)
            if rs_file is None:
                continue

            temp_dir = TEMP_BASE / tt / problem_id
            out_bin  = BIN_BASE  / tt / problem_id / f"{tt}_{problem_id}"

            setup_temp_project(temp_dir, rs_file, tt, problem_id, test_pairs)

            if build_binary(temp_dir, out_bin, build_log):
                built[problem_id][tt] = out_bin
                print(f"  [OK]   {tt}/{problem_id}")
            else:
                print(f"  [FAIL] {tt}/{problem_id}  (see build_failures.log)")

    # ── Phase 2: benchmark ───────────────────────────────────────────
    print("\n=== Phase 2: benchmarking ===")
    all_rows = []
    skipped  = []

    for problem_id in common_ids:
        binaries = built.get(problem_id, {})

        if len(binaries) < 3:
            reason = f"only {len(binaries)}/3 variants built"
            print(f"  [SKIP] {problem_id}: {reason}")
            skipped.append((problem_id, reason))
            continue

        # Use first available input for timing
        # (all variants get the same input — fair comparison)
        test_pairs = find_test_files(problem_id)
        if not test_pairs:
            reason = "no test input found"
            print(f"  [SKIP] {problem_id}: {reason}")
            with missing_log.open("a") as f:
                f.write(f"{problem_id}: {reason}\n")
            skipped.append((problem_id, reason))
            continue

        input_file = test_pairs[0][0]   # first sample input
        json_out   = RAW_DIR / f"{problem_id}.json"

        print(f"  [BENCH] {problem_id}  ({input_file.name})")
        if not run_hyperfine(problem_id, binaries, input_file, json_out, missing_log):
            skipped.append((problem_id, "hyperfine failed"))
            continue

        for row in parse_json(json_out):
            all_rows.append({"problem_id": problem_id, **row})

    # ── Phase 3: save CSV ────────────────────────────────────────────
    with OUT_CSV.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=[
            "problem_id", "command",
            "mean_s", "stddev_s", "median_s", "min_s", "max_s",
        ])
        writer.writeheader()
        writer.writerows(all_rows)

    print(f"\n{'='*50}")
    print(f"Benchmarked : {len(common_ids) - len(skipped)}/{len(common_ids)} problems")
    print(f"Skipped     : {len(skipped)}")
    print(f"Saved       : {OUT_CSV}")
    if skipped:
        print("\nSkipped list:")
        for pid, reason in skipped:
            print(f"  {pid}: {reason}")


if __name__ == "__main__":
    main()