#!/usr/bin/env python3
"""
Build a new filtered dataset folder from an existing dataset root.

For each ID folder, keep only source files that have a same-path
counterpart in the target folder, and copy both files to NEW_OUTPUT_ROOT.

Expected input structure:
INPUT_ROOT/
  p00001/
    C/
      abc.c
    Rust/
      abc.c
"""

from __future__ import annotations

import shutil
from pathlib import Path


# =========================
# Configure these values
# =========================

SOURCE_LANGUAGE = "C"
TARGET_LANGUAGE = "Rust"
INPUT_ROOT = Path(f"/Volumes/Extreme SSD/research/code translation/collected_dataset/{SOURCE_LANGUAGE}_to_{TARGET_LANGUAGE}")
NEW_OUTPUT_ROOT = Path(f"/Volumes/Extreme SSD/research/code translation/collected_dataset/{SOURCE_LANGUAGE}_to_{TARGET_LANGUAGE}_paired_only")
# Match modes:
# - "exact_name": require same relative path and filename in target
# - "same_stem":  require same relative path parent and same stem (extension can differ)
MATCH_MODE = "exact_name"

def main() -> None:
    if not INPUT_ROOT.is_dir():
        raise FileNotFoundError(f"INPUT_ROOT does not exist or is not a directory: {INPUT_ROOT}")

    NEW_OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)

    scanned_ids = 0
    scanned_source_files = 0
    kept_pairs = 0
    dropped_source_files = 0
    skipped_missing_lang_dirs = 0
    unmatched_examples: list[str] = []

    for id_dir in sorted(INPUT_ROOT.iterdir()):
        if not id_dir.is_dir():
            continue

        scanned_ids += 1
        src_dir = id_dir / SOURCE_LANGUAGE
        tgt_dir = id_dir / TARGET_LANGUAGE

        if not src_dir.is_dir() or not tgt_dir.is_dir():
            skipped_missing_lang_dirs += 1
            print(f"[SKIP]   {id_dir.name}: missing {SOURCE_LANGUAGE} or {TARGET_LANGUAGE} folder")
            continue

        out_src_dir = NEW_OUTPUT_ROOT / id_dir.name / SOURCE_LANGUAGE
        out_tgt_dir = NEW_OUTPUT_ROOT / id_dir.name / TARGET_LANGUAGE

        local_kept = 0
        local_dropped = 0
        for src_file in src_dir.rglob("*"):
            if not src_file.is_file():
                continue

            scanned_source_files += 1
            rel_path = src_file.relative_to(src_dir)
            counterpart: Path | None = None

            if MATCH_MODE == "exact_name":
                candidate = tgt_dir / rel_path
                if candidate.is_file():
                    counterpart = candidate
            elif MATCH_MODE == "same_stem":
                target_parent = (tgt_dir / rel_path.parent)
                if target_parent.is_dir():
                    stem = src_file.stem
                    for p in target_parent.iterdir():
                        if p.is_file() and p.stem == stem:
                            counterpart = p
                            break
            else:
                raise ValueError(f"Unsupported MATCH_MODE: {MATCH_MODE}")

            if counterpart is None:
                dropped_source_files += 1
                local_dropped += 1
                if len(unmatched_examples) < 10:
                    unmatched_examples.append(str(src_file))
                continue

            out_src_path = out_src_dir / rel_path
            if MATCH_MODE == "same_stem":
                out_tgt_rel_path = rel_path.parent / counterpart.name
            else:
                out_tgt_rel_path = rel_path
            out_tgt_path = out_tgt_dir / out_tgt_rel_path
            out_src_path.parent.mkdir(parents=True, exist_ok=True)
            out_tgt_path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src_file, out_src_path)
            shutil.copy2(counterpart, out_tgt_path)
            kept_pairs += 1
            local_kept += 1

        print(f"[DONE]   {id_dir.name}: kept {local_kept} pair(s), dropped {local_dropped} source file(s)")

    print("\nDone.")
    print(f"Scanned ID folders: {scanned_ids}")
    print(f"Scanned source files: {scanned_source_files}")
    print(f"Kept source-target pairs: {kept_pairs}")
    print(f"Dropped source files without counterpart: {dropped_source_files}")
    print(f"Skipped IDs (missing language folders): {skipped_missing_lang_dirs}")
    print(f"Match mode: {MATCH_MODE}")
    print(f"Input root: {INPUT_ROOT}")
    print(f"New output root: {NEW_OUTPUT_ROOT}")
    if unmatched_examples:
        print("Unmatched source examples:")
        for ex in unmatched_examples:
            print(f"  - {ex}")


if __name__ == "__main__":
    main()
