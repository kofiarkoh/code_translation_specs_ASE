import csv
import os

REPAIRS_DIR = os.path.dirname(os.path.abspath(__file__))

PIPELINES = [

    "full_translation_loop_GPT_5_2",
    "full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS",
    "translation_with_validated_specs_GPT_5_2",

]


def get_all_pass_problems(pipeline_name: str) -> set[str] | None:
    csv_path = os.path.join(
        REPAIRS_DIR,
        f"{pipeline_name}_analysis",
        "optimized_prompt@1",
        "pass_venn_analysis",
        "profile_intersections.csv",
    )

    if not os.path.exists(csv_path):
        print(f"Warning: no profile_intersections.csv found for '{pipeline_name}' (skipping)")
        print(f"  Path checked: {csv_path}\n")
        return None

    with open(csv_path, newline="") as f:
        reader = csv.DictReader(f)
        rows = list(reader)

    for row in rows:
        if row["profiles"] == "code_and_spec | code_only | specs_only":
            problems = set(row["problem_ids"].split(";"))
            print(f"  {pipeline_name}: {len(problems)} problems")
            return problems

    return None


if __name__ == "__main__":
    print("Problems passing all profiles per pipeline:")
    pipeline_problem_sets = []
    for pipeline in PIPELINES:
        problems = get_all_pass_problems(pipeline)
        if problems is not None:
            pipeline_problem_sets.append(problems)

    if pipeline_problem_sets:
        common = sorted(set.intersection(*pipeline_problem_sets))
        print(f"\nIntersection across all {len(pipeline_problem_sets)} pipelines: {len(common)} problems")
        print(", ".join(common))

        out_path = os.path.join(REPAIRS_DIR, "common_pass_problems.csv")
        with open(out_path, "w", newline="") as f:
            writer = csv.writer(f)
            writer.writerow(["problem_id"])
            writer.writerows([[p] for p in common])
        print(f"\nSaved to: {out_path}")
