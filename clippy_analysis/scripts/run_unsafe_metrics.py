#!/usr/bin/env python3

import csv
import json
import shutil
import subprocess
from pathlib import Path

REPO_ROOT = Path(".").resolve()

import sys
sys.path.insert(0, str(REPO_ROOT))
from clippy_analysis.common import COMMON_PROBLEM_IDS

RUST_BASE = REPO_ROOT / "repairs" / "translation_with_validated_specs_GPT_5_2"
OUT_CSV = REPO_ROOT / "clippy_analysis" / "summary" / "unsafe_constructs_detailed.csv"
TEMP_BASE = REPO_ROOT / "clippy_analysis" / "temp_projects" / "unsafe_metrics"
ANALYZER_MANIFEST = REPO_ROOT / "clippy_analysis" / "tools" / "unsafe_metrics_hir" / "Cargo.toml"

TRANSLATION_DIRS = {
    "code_and_spec": RUST_BASE / "code_and_spec" / "optimized_prompt@1",
    "code_only":     RUST_BASE / "code_only"     / "optimized_prompt@1",
    "specs_only":    RUST_BASE / "specs_only"    / "optimized_prompt@1",
}


def count_total_lines(path: Path) -> int:
    return len(path.read_text(encoding="utf-8", errors="replace").splitlines())


def find_rs(problem_dir: Path):
    files = sorted(problem_dir.glob("*.rs"))
    if files:
        return files[0]
    files = sorted(problem_dir.rglob("*.rs"))
    return files[0] if files else None


def make_temp_crate(temp_dir: Path, src_file: Path):
    if temp_dir.exists():
        shutil.rmtree(temp_dir)
    (temp_dir / "src").mkdir(parents=True)
    (temp_dir / "Cargo.toml").write_text(
        "[package]\n"
        f'name = "{temp_dir.name}"\n'
        'version = "0.1.0"\n'
        'edition = "2021"\n\n'
        "[dependencies]\n",
        encoding="utf-8",
    )
    shutil.copy(src_file, temp_dir / "src" / "main.rs")


def main():
    OUT_CSV.parent.mkdir(parents=True, exist_ok=True)
    TEMP_BASE.mkdir(parents=True, exist_ok=True)

    # ── Filter to common problem IDs ──────────────────────────
    common_ids = set(COMMON_PROBLEM_IDS)
    print(f"Filtering to {len(common_ids)} common problem IDs")
    # ──────────────────────────────────────────────────────────

    rows = []

    all_jobs = []
    for translation_type, base in TRANSLATION_DIRS.items():
        for problem_dir in sorted(p for p in base.iterdir() if p.is_dir() and p.name.startswith("p")):
            if problem_dir.name not in common_ids:  # ← filter here
                continue
            all_jobs.append((translation_type, problem_dir))

    total_jobs = len(all_jobs)
    print(f"\n🚀 Starting unsafe metrics extraction")
    print(f"Total jobs: {total_jobs}\n")

    success = 0
    failed  = 0

    for idx, (translation_type, problem_dir) in enumerate(all_jobs, start=1):
        rs_file = find_rs(problem_dir)
        progress = f"[{idx}/{total_jobs}]"

        if rs_file is None:
            print(f"{progress} ⚠️  {translation_type}/{problem_dir.name} -> NO RUST FILE")
            continue

        problem_id = problem_dir.name
        rust_loc   = count_total_lines(rs_file)

        print(f"{progress} 🔄 Processing {translation_type}/{problem_id} (LOC={rust_loc})")

        temp_dir = TEMP_BASE / translation_type / problem_id
        make_temp_crate(temp_dir, rs_file)

        out_json = temp_dir / "unsafe_metrics.json"

        cmd = [
            "bash", "-lc",
            f"cd {REPO_ROOT} && "
            f"RUSTC_BOOTSTRAP=1 cargo run --quiet --manifest-path {ANALYZER_MANIFEST} -- "
            f"--crate-root {temp_dir / 'src' / 'main.rs'} "
            f"--output-json {out_json}"
        ]

        result = subprocess.run(cmd, capture_output=True, text=True)

        if result.returncode != 0:
            failed += 1
            print(f"{progress} ❌ FAILED {translation_type}/{problem_id}")
            print(result.stderr.strip())
            continue

        try:
            metrics = json.loads(out_json.read_text(encoding="utf-8"))
        except Exception as e:
            failed += 1
            print(f"{progress} ❌ JSON ERROR {translation_type}/{problem_id}: {e}")
            continue

        rows.append({
            "translation_type": translation_type,
            "problem_id":       problem_id,
            "rust_loc":         rust_loc,
            "unsafe_blocks":    metrics["unsafe_blocks"],
            "unsafe_lines":     metrics["unsafe_lines"],
            "type_casts":       metrics["type_casts"],
            "unsafe_calls":     metrics["unsafe_calls"],
        })

        success += 1
        print(f"{progress} ✅ DONE ({success} ok / {failed} failed)\n")

    with OUT_CSV.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=[
            "translation_type", "problem_id", "rust_loc",
            "unsafe_blocks", "unsafe_lines", "type_casts", "unsafe_calls",
        ])
        writer.writeheader()
        writer.writerows(rows)

    print("\n🎉 Finished!")
    print(f"Total: {total_jobs} | Success: {success} | Failed: {failed}")
    print(f"Saved {len(rows)} rows to {OUT_CSV}")


if __name__ == "__main__":
    main()