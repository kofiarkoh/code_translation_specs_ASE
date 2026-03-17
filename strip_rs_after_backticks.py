#!/usr/bin/env python3
from __future__ import annotations

import argparse
from pathlib import Path


def strip_after_backticks(file_path: Path) -> bool:
    original_text = file_path.read_text(encoding="utf-8", errors="ignore")
    lines = original_text.splitlines(keepends=True)

    for index, line in enumerate(lines):
        if "```" in line:
            updated_text = "".join(lines[:index])
            if updated_text != original_text:
                file_path.write_text(updated_text, encoding="utf-8")
                return True
            return False

    return False


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Strip everything from the first triple-backtick line onward in all .rs files under a folder."
    )
    parser.add_argument("folder", type=Path, help="Root folder containing .rs files")
    args = parser.parse_args()

    root = args.folder.expanduser().resolve()
    if not root.is_dir():
        raise FileNotFoundError(f"Folder does not exist or is not a directory: {root}")

    scanned = 0
    modified = 0

    for rust_file in sorted(root.rglob("*.rs")):
        if not rust_file.is_file():
            continue

        scanned += 1
        if strip_after_backticks(rust_file):
            modified += 1
            print(f"[UPDATED] {rust_file}")
        else:
            print(f"[SKIP] {rust_file}")

    print("\nDone.")
    print(f"Scanned .rs files: {scanned}")
    print(f"Modified .rs files: {modified}")
    print(f"Root folder: {root}")


if __name__ == "__main__":
    main()
