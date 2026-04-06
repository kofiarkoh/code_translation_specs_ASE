# common.py
from pathlib import Path
import csv

REPO_ROOT = Path(__file__).resolve().parent.parent

PROFILE_INTERSECTIONS_CSV = (
    REPO_ROOT
    / "repairs/translation_with_validated_specs_GPT_5_2_analysis"
    / "optimized_prompt@1/pass_venn_analysis/profile_intersections.csv"
)

def load_common_problem_ids(
    csv_path: Path = PROFILE_INTERSECTIONS_CSV,
    combination_size: int = 3
) -> list[str]:
    """
    Returns problem IDs that pass for all translation types
    (rows where combination_size == 3 by default).
    """
    with csv_path.open(encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for row in reader:
            if int(row["combination_size"]) == combination_size:
                return row["problem_ids"].split(";")
    return []

# Optionally expose it as a module-level constant so scripts
# can just do: from common import COMMON_PROBLEM_IDS
COMMON_PROBLEM_IDS = load_common_problem_ids()