#!/usr/bin/env python3

from pathlib import Path
import csv
from clippy_analysis.common import COMMON_PROBLEM_IDS


REPO_ROOT = Path(".").resolve()

RUST_BASE = REPO_ROOT / "repairs" / "translation_with_validated_specs_GPT_5_2"
C_BASE = REPO_ROOT / "collected_dataset" / "C" / "final_problems"

OUTPUT_CSV = REPO_ROOT / "clippy_analysis" / "summary" / "loc_by_problem_and_translation.csv"
LOG_DIR = REPO_ROOT / "clippy_analysis" / "logs"

TRANSLATION_DIRS = {
    "code_and_spec": RUST_BASE / "code_and_spec" / "optimized_prompt@1",
    "code_only": RUST_BASE / "code_only" / "optimized_prompt@1",
    "specs_only": RUST_BASE / "specs_only" / "optimized_prompt@1",
}


# ---------------------------
# SIMPLE LOC (ALL LINES)
# ---------------------------
def count_total_lines(text: str) -> int:
    return len(text.splitlines())


def read_text_file(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")

def append_log(path: Path, msg: str):
    with path.open("a", encoding="utf-8") as f:
        f.write(msg + "\n")


def main():
    OUTPUT_CSV.parent.mkdir(parents=True, exist_ok=True)
    LOG_DIR.mkdir(parents=True, exist_ok=True)

    missing_c_log = LOG_DIR / "missing_c.log"
    missing_rust_log = LOG_DIR / "missing_rust.log"

    missing_c_log.write_text("")
    missing_rust_log.write_text("")

    rows = []
    problem_ids = sorted(COMMON_PROBLEM_IDS)

    for translation_type, rust_dir in TRANSLATION_DIRS.items():
        for problem_id in problem_ids:

            rust_problem_dir = rust_dir / problem_id
            c_problem_dir = C_BASE / problem_id

            # ---------------------------
            # FIND RUST FILE
            # ---------------------------
            rust_files = sorted(rust_problem_dir.glob("*.rs"))
            if not rust_files:
                rust_files = sorted(rust_problem_dir.rglob("*.rs"))

            if not rust_files:
                append_log(missing_rust_log, f"[{translation_type}][{problem_id}] missing rust")
                continue

            rust_file = rust_files[0]

            # ---------------------------
            # FIND C FILE
            # ---------------------------
            c_files = sorted(c_problem_dir.glob("*.c"))
            if not c_files:
                c_files = sorted(c_problem_dir.rglob("*.c"))

            if not c_files:
                append_log(missing_c_log, f"[{translation_type}][{problem_id}] missing C")
                continue

            c_file = c_files[0]

            # ---------------------------
            # COUNT LOC
            # ---------------------------
            rust_loc = count_total_lines(read_text_file(rust_file))
            c_loc = count_total_lines(read_text_file(c_file))

            rows.append({
                "translation_type": translation_type,
                "problem_id": problem_id,
                "c_loc": c_loc,
                "rust_loc": rust_loc,
            })

    # ---------------------------
    # SAVE CSV
    # ---------------------------
    with OUTPUT_CSV.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=["translation_type", "problem_id", "c_loc", "rust_loc"]
        )
        writer.writeheader()
        writer.writerows(rows)

    print(f"✅ Saved {len(rows)} rows to {OUTPUT_CSV}")


if __name__ == "__main__":
    main()