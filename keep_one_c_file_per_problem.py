from __future__ import annotations

from pathlib import Path


PROBLEMS_ROOT = Path("/Users/lithe/Documents/research/code_translation_specs/codenet_statistics/collected_dataset/C/final_problems")
DRY_RUN = False
VERBOSE = True


def main() -> None:
    if not PROBLEMS_ROOT.exists():
        raise FileNotFoundError(f"Problems root does not exist: {PROBLEMS_ROOT}")
    if not PROBLEMS_ROOT.is_dir():
        raise NotADirectoryError(f"Problems root is not a directory: {PROBLEMS_ROOT}")

    total_problem_dirs = 0
    problem_dirs_with_c_files = 0
    kept_files = 0
    deleted_files = 0

    for problem_dir in sorted(path for path in PROBLEMS_ROOT.iterdir() if path.is_dir()):
        total_problem_dirs += 1
        c_files = sorted(path for path in problem_dir.rglob("*.c") if path.is_file())
        if not c_files:
            if VERBOSE:
                print(f"[SKIP] {problem_dir.name}: no .c files")
            continue

        problem_dirs_with_c_files += 1
        kept_file = c_files[0]
        extra_files = c_files[1:]
        kept_files += 1

        if VERBOSE:
            print(f"[KEEP] {problem_dir.name}: {kept_file.relative_to(problem_dir)}")

        for extra_file in extra_files:
            if DRY_RUN:
                if VERBOSE:
                    print(f"[DRY-RUN DELETE] {problem_dir.name}: {extra_file.relative_to(problem_dir)}")
            else:
                extra_file.unlink()
                if VERBOSE:
                    print(f"[DELETE] {problem_dir.name}: {extra_file.relative_to(problem_dir)}")
            deleted_files += 1

    print()
    print("Summary")
    print(f"Problems scanned: {total_problem_dirs}")
    print(f"Problems with .c files: {problem_dirs_with_c_files}")
    print(f"Files kept: {kept_files}")
    print(f"Files deleted: {deleted_files}")
    print(f"Dry run: {DRY_RUN}")


if __name__ == "__main__":
    main()
