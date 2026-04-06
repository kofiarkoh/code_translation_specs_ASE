#!/usr/bin/env python3

import csv
import json
from pathlib import Path

RAW_JSON_BASE = Path("clippy_analysis/raw_json")
MAPPING_CSV = Path("clippy_analysis/metadata/clippy_lint_group_map_rust_1_87_0.csv")
LOC_CSV = Path("clippy_analysis/summary/loc_by_problem_and_translation.csv")
OUTPUT_CSV = Path("clippy_analysis/summary/clippy_lints_detailed.csv")

TRANSLATION_TYPES = ["code_and_spec", "code_only", "specs_only"]


def load_lint_mapping(mapping_csv_path: Path) -> dict[str, str]:
    mapping = {}
    with mapping_csv_path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            lint_code = (row.get("lint_code") or "").strip()
            clippy_group = (row.get("clippy_group") or "").strip()
            if lint_code:
                mapping[lint_code] = clippy_group if clippy_group else "unknown"
    return mapping


def load_loc_mapping(loc_csv_path: Path) -> dict[tuple[str, str], dict[str, str]]:
    loc_map = {}
    with loc_csv_path.open("r", encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            translation_type = (row.get("translation_type") or "").strip()
            problem_id = (row.get("problem_id") or "").strip()
            c_loc = (row.get("c_loc") or "").strip()
            rust_loc = (row.get("rust_loc") or "").strip()

            if translation_type and problem_id:
                loc_map[(translation_type, problem_id)] = {
                    "c_loc": c_loc,
                    "rust_loc": rust_loc,
                }
    return loc_map


def extract_primary_line(message_obj: dict) -> str:
    spans = message_obj.get("spans", [])
    for span in spans:
        if span.get("is_primary"):
            line_start = span.get("line_start")
            if line_start is not None:
                return str(line_start)

    if spans:
        line_start = spans[0].get("line_start")
        if line_start is not None:
            return str(line_start)

    return ""


def parse_jsonl_file(
    jsonl_path: Path,
    translation_type: str,
    lint_map: dict[str, str],
    loc_map: dict[tuple[str, str], dict[str, str]],
) -> list[dict]:
    rows = []
    problem_id = jsonl_path.stem

    loc_info = loc_map.get((translation_type, problem_id), {})
    c_loc = loc_info.get("c_loc", "")
    rust_loc = loc_info.get("rust_loc", "")

    with jsonl_path.open("r", encoding="utf-8") as f:
        for raw_line in f:
            raw_line = raw_line.strip()
            if not raw_line:
                continue

            try:
                obj = json.loads(raw_line)
            except json.JSONDecodeError:
                continue

            if obj.get("reason") != "compiler-message":
                continue

            message_obj = obj.get("message", {})
            code_obj = message_obj.get("code") or {}
            lint_code = code_obj.get("code")

            if not lint_code or not isinstance(lint_code, str):
                continue

            if not lint_code.startswith("clippy::"):
                continue

            level = message_obj.get("level", "")
            message = message_obj.get("message", "")
            line = extract_primary_line(message_obj)
            clippy_category = lint_map.get(lint_code, "unknown")

            rows.append(
                {
                    "translation_type": translation_type,
                    "problem_id": problem_id,
                    "lint_code": lint_code,
                    "clippy_category": clippy_category,
                    "level": level,
                    "line": line,
                    "message": message,
                    "c_loc": c_loc,
                    "rust_loc": rust_loc,
                }
            )

    return rows


def main() -> None:
    if not MAPPING_CSV.exists():
        raise FileNotFoundError(f"Mapping CSV not found: {MAPPING_CSV}")

    if not LOC_CSV.exists():
        raise FileNotFoundError(f"LOC CSV not found: {LOC_CSV}")

    lint_map = load_lint_mapping(MAPPING_CSV)
    loc_map = load_loc_mapping(LOC_CSV)

    all_rows = []

    for translation_type in TRANSLATION_TYPES:
        translation_dir = RAW_JSON_BASE / translation_type
        if not translation_dir.exists():
            print(f"Skipping missing directory: {translation_dir}")
            continue

        for jsonl_path in sorted(translation_dir.glob("*.jsonl")):
            rows = parse_jsonl_file(jsonl_path, translation_type, lint_map, loc_map)
            all_rows.extend(rows)

    OUTPUT_CSV.parent.mkdir(parents=True, exist_ok=True)

    with OUTPUT_CSV.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=[
                "translation_type",
                "problem_id",
                "lint_code",
                "clippy_category",
                "level",
                "line",
                "message",
                "c_loc",
                "rust_loc",
            ],
        )
        writer.writeheader()
        writer.writerows(all_rows)

    print(f"Saved {len(all_rows)} rows to {OUTPUT_CSV}")


if __name__ == "__main__":
    main()