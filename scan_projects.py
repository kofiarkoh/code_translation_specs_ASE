#!/usr/bin/env python3
"""
Scan project subfolders and copy only those that contain
the configured source language folder.

Expected input shape:
SCAN_ROOT/
  p00881/
    C/
    Python/
    ...
  p00882/
    ...

Output shape:
OUTPUT_ROOT/
  p00881/
    C/
"""

from __future__ import annotations

import shutil
from pathlib import Path


# =========================
# Configure these values
# =========================
SCAN_ROOT = Path('/Volumes/Extreme SSD/research/code translation/Project_CodeNet/data')
SOURCE_LANGUAGE = "C"
OUTPUT_ROOT = Path(f"/Volumes/Extreme SSD/research/code translation/collected_dataset/{SOURCE_LANGUAGE}")
HTML_ROOT = Path('/Volumes/Extreme SSD/research/code translation/Project_CodeNet/problem_descriptions')

# If True, language folder matching is case-insensitive.
CASE_INSENSITIVE_MATCH = False


def find_language_dir(project_dir: Path, language_name: str, case_insensitive: bool) -> Path | None:
    """Return the matching language directory under project_dir, else None."""
    if not case_insensitive:
        candidate = project_dir / language_name
        return candidate if candidate.is_dir() else None

    target = language_name.casefold()
    for child in project_dir.iterdir():
        if child.is_dir() and child.name.casefold() == target:
            return child
    return None


def copy_lang_folder(src_lang_dir: Path, out_project_dir: Path) -> None:
    """Copy a language folder into the output project directory."""
    out_lang_dir = out_project_dir / src_lang_dir.name
    shutil.copytree(src_lang_dir, out_lang_dir, dirs_exist_ok=True)


def main() -> None:
    if not SCAN_ROOT.is_dir():
        raise FileNotFoundError(f"SCAN_ROOT does not exist or is not a directory: {SCAN_ROOT}")
    if not HTML_ROOT.is_dir():
        raise FileNotFoundError(f"HTML_ROOT does not exist or is not a directory: {HTML_ROOT}")

    OUTPUT_ROOT.mkdir(parents=True, exist_ok=True)

    scanned_projects = 0
    matched_projects = 0
    html_copied_count = 0
    report_lines: list[str] = []

    report_lines.append(f"SCAN_ROOT: {SCAN_ROOT}")
    report_lines.append(f"SOURCE_LANGUAGE: {SOURCE_LANGUAGE}")
    report_lines.append(f"HTML_ROOT: {HTML_ROOT}")
    report_lines.append(f"CASE_INSENSITIVE_MATCH: {CASE_INSENSITIVE_MATCH}")
    report_lines.append("")
    report_lines.append("Project Results:")

    for project_dir in sorted(SCAN_ROOT.iterdir()):
        if not project_dir.is_dir():
            continue

        scanned_projects += 1

        src_dir = find_language_dir(project_dir, SOURCE_LANGUAGE, CASE_INSENSITIVE_MATCH)

        if src_dir:
            out_project_dir = OUTPUT_ROOT / project_dir.name
            out_project_dir.mkdir(parents=True, exist_ok=True)
            copy_lang_folder(src_dir, out_project_dir)

            html_source = HTML_ROOT / f"{project_dir.name}.html"
            html_status = "html: missing"
            if html_source.is_file():
                shutil.copy2(html_source, out_project_dir / html_source.name)
                html_copied_count += 1
                html_status = f"html: copied ({html_source.name})"

            matched_projects += 1
            print(f"[COPIED] {project_dir.name}: {src_dir.name}, {html_status}")
            report_lines.append(f"[COPIED] {project_dir.name}: {src_dir.name}, {html_status}")
        else:
            print(f"[SKIP]   {project_dir.name}")
            report_lines.append(f"[SKIP]   {project_dir.name}: missing {SOURCE_LANGUAGE}")

    print("\nDone.")
    print(f"Scanned project folders: {scanned_projects}")
    print(f"Matched/copied folders: {matched_projects}")
    print(f"HTML files copied: {html_copied_count}")
    print(f"Output directory: {OUTPUT_ROOT}")

    report_lines.append("")
    report_lines.append("Summary:")
    report_lines.append(f"Scanned project folders: {scanned_projects}")
    report_lines.append(f"Matched/copied folders: {matched_projects}")
    report_lines.append(f"HTML files copied: {html_copied_count}")
    report_lines.append(f"Output directory: {OUTPUT_ROOT}")

    report_path = OUTPUT_ROOT / "scan_report.txt"
    report_path.write_text("\n".join(report_lines) + "\n", encoding="utf-8")
    print(f"Report saved to: {report_path}")


if __name__ == "__main__":
    main()
