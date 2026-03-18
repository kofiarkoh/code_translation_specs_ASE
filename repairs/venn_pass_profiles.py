#!/usr/bin/env python3
from __future__ import annotations

import csv
from collections import defaultdict
from itertools import combinations
from pathlib import Path

import matplotlib.pyplot as plt

try:
    from matplotlib_venn import venn2, venn3
except ImportError:  # pragma: no cover - optional dependency
    venn2 = None
    venn3 = None


PROJECT_ROOT = Path(__file__).resolve().parent.parent
PIPELINE_NAME = "full_translation_loop_GPT_5_2"
DATASET_NAME = "optimized_prompt@1"
TARGET_STATUS = "PASS"

PIPELINE_ROOT = PROJECT_ROOT / "repairs" / PIPELINE_NAME
INPUT_ROOT = PIPELINE_ROOT
OUTPUT_DIR = PROJECT_ROOT / "repairs" / f"{PIPELINE_NAME}_analysis" / DATASET_NAME / "pass_venn_analysis"
SUMMARY_TXT = OUTPUT_DIR / "venn_summary.txt"
PROFILE_COUNTS_CSV = OUTPUT_DIR / "profile_pass_counts.csv"
INTERSECTIONS_CSV = OUTPUT_DIR / "profile_intersections.csv"
MEMBERSHIP_CSV = OUTPUT_DIR / "problem_membership_matrix.csv"
VENN_PNG = OUTPUT_DIR / "pass_profiles_venn.png"


def get_profile_report_csv(profile_name: str) -> Path:
    dataset_root = PIPELINE_ROOT / profile_name / DATASET_NAME
    candidates = sorted(dataset_root.glob("*_report.csv"))
    if not candidates:
        raise FileNotFoundError(f"No report CSV found in {dataset_root}")
    return candidates[0]


def row_matches_target_status(row: dict[str, str]) -> bool:
    final_status = row.get("final_status", "").strip()
    result = row.get("result", "").strip()
    return final_status == TARGET_STATUS or result == TARGET_STATUS


def load_profile_problem_sets() -> dict[str, set[str]]:
    if not INPUT_ROOT.is_dir():
        raise FileNotFoundError(f"Input root not found: {INPUT_ROOT}")

    profile_problem_ids: dict[str, set[str]] = defaultdict(set)

    for profile_root in sorted(p for p in INPUT_ROOT.iterdir() if p.is_dir()):
        profile_name = profile_root.name
        report_csv = get_profile_report_csv(profile_name)

        with report_csv.open("r", newline="", encoding="utf-8") as csv_file:
            reader = csv.DictReader(csv_file)
            for row in reader:
                problem_id = row.get("problem_id", "").strip()
                if not problem_id or not row_matches_target_status(row):
                    continue
                profile_problem_ids[profile_name].add(problem_id)

    return dict(sorted(profile_problem_ids.items()))


def write_profile_counts(profile_problem_ids: dict[str, set[str]]) -> None:
    with PROFILE_COUNTS_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["profile", "pass_problem_count"])
        for profile_name, problem_ids in profile_problem_ids.items():
            writer.writerow([profile_name, len(problem_ids)])


def write_membership_matrix(profile_problem_ids: dict[str, set[str]]) -> None:
    all_problem_ids = sorted({problem_id for ids in profile_problem_ids.values() for problem_id in ids})
    profiles = list(profile_problem_ids)

    with MEMBERSHIP_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["problem_id", *profiles])
        for problem_id in all_problem_ids:
            writer.writerow([
                problem_id,
                *["1" if problem_id in profile_problem_ids[profile_name] else "0" for profile_name in profiles],
            ])


def write_intersections(profile_problem_ids: dict[str, set[str]]) -> list[str]:
    profiles = list(profile_problem_ids)
    summary_lines: list[str] = []

    with INTERSECTIONS_CSV.open("w", newline="", encoding="utf-8") as csv_file:
        writer = csv.writer(csv_file)
        writer.writerow(["combination_size", "profiles", "intersection_count", "problem_ids"])

        for size in range(2, len(profiles) + 1):
            for combo in combinations(profiles, size):
                overlap = set.intersection(*(profile_problem_ids[profile_name] for profile_name in combo))
                overlap_ids = sorted(overlap)
                writer.writerow([size, " | ".join(combo), len(overlap_ids), ";".join(overlap_ids)])
                summary_lines.append(f"{' INTERSECT '.join(combo)}: {len(overlap_ids)}")

    return summary_lines


def save_venn_plot(profile_problem_ids: dict[str, set[str]]) -> str:
    profiles = list(profile_problem_ids)
    if len(profiles) not in {2, 3}:
        return f"Skipped Venn plot: expected 2 or 3 profiles, found {len(profiles)}."
    if len(profiles) == 2 and venn2 is None:
        return "Skipped Venn plot: matplotlib-venn is not installed."
    if len(profiles) == 3 and venn3 is None:
        return "Skipped Venn plot: matplotlib-venn is not installed."

    fig, ax = plt.subplots(figsize=(8, 8))
    sets = [profile_problem_ids[profile_name] for profile_name in profiles]

    if len(profiles) == 2:
        venn2(subsets=sets, set_labels=profiles, ax=ax)
    else:
        venn3(subsets=sets, set_labels=profiles, ax=ax)

    ax.set_title(f"{TARGET_STATUS} Problem Overlap by Initial Translation Profile")
    fig.tight_layout()
    fig.savefig(VENN_PNG, dpi=200)
    plt.close(fig)
    return f"Venn plot written to: {VENN_PNG}"


def main() -> None:
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    profile_problem_ids = load_profile_problem_sets()
    if not profile_problem_ids:
        raise ValueError(f"No {TARGET_STATUS} profile/problem pairs were found in {INPUT_ROOT}")

    write_profile_counts(profile_problem_ids)
    write_membership_matrix(profile_problem_ids)
    intersection_summary = write_intersections(profile_problem_ids)
    venn_status = save_venn_plot(profile_problem_ids)

    summary_lines = [
        f"PIPELINE_ROOT: {PIPELINE_ROOT}",
        f"DATASET_NAME: {DATASET_NAME}",
        f"TARGET_STATUS: {TARGET_STATUS}",
        f"Profiles found: {len(profile_problem_ids)}",
        "",
        f"Per-profile {TARGET_STATUS} counts:",
    ]

    for profile_name, problem_ids in profile_problem_ids.items():
        summary_lines.append(f"- {profile_name}: {len(problem_ids)}")

    summary_lines.append("")
    summary_lines.append("Intersections:")
    if intersection_summary:
        summary_lines.extend(intersection_summary)
    else:
        summary_lines.append("No multi-profile intersections to report.")

    summary_lines.append("")
    summary_lines.append(f"PROFILE_COUNTS_CSV: {PROFILE_COUNTS_CSV}")
    summary_lines.append(f"INTERSECTIONS_CSV: {INTERSECTIONS_CSV}")
    summary_lines.append(f"MEMBERSHIP_CSV: {MEMBERSHIP_CSV}")
    summary_lines.append(venn_status)

    SUMMARY_TXT.write_text("\n".join(summary_lines) + "\n", encoding="utf-8")

    print(f"Summary written to: {SUMMARY_TXT}")
    print(f"Profile counts CSV: {PROFILE_COUNTS_CSV}")
    print(f"Intersections CSV: {INTERSECTIONS_CSV}")
    print(f"Membership matrix CSV: {MEMBERSHIP_CSV}")
    print(venn_status)


if __name__ == "__main__":
    main()
