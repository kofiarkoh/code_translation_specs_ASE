
## Setup
- copy `.env.example` to `.env` file at the root level and add your openai api key
- Inside `.env`, make sure to set `DATASET_NAME` . This will be used to saved the results in a dedicated folder.
- create a new virtual environment and install the dependencies using the `requirements.txt` file


## Folder Structure

```text
project-root/
├── collected_dataset/                    # Datasets for each programming language
│   ├── C/                                # All files and resources related to C
│   │   └── final_problems/               # Final set of problems selected for C
│   └── problem_test_inputs_and_outputs/  # Test inputs and expected outputs
├── api_responses/                        # Raw LLM API responses (specs, translations)
├── repairs/                              # Translation loop, analysis, and plotting scripts
├── cost_estimates/                       # API cost estimation and plotting scripts
├── analyse_results/                      # Rust code analysis metric comparison
└── r_code/                               # Prepared source trees for rust-code-analysis
```

## Scripts in `repairs/`

### Translation & Repair

| Script | Description |
|--------|-------------|
| `full_translation_loop.py` | Runs the full C-to-Rust translation loop: initial LLM translation, compilation, testing, and iterative repair. |
| `initial_translation_profiles.py` | Defines the three translation prompt profiles (specs_only, code_only, code_and_spec) as dataclasses. |
| `rebuild_missing_attempts_with_llm.py` | Re-runs LLM translation for problems with missing or incomplete attempt data in a pipeline. |
| `repair_old_prompt_non_pass.py` | Sends non-passing Rust translations back to the LLM for repair using execution reports. |
| `copy_repaired_rust_to_r_code_src.py` | Copies repaired Rust output files into the `r_code/src` directory tree by problem ID. |

### Analysis & Metrics

| Script | Description |
|--------|-------------|
| `run_all_profile_metrics_analysis.py` | Prepares source trees for all problems and runs rust-code-analysis metric comparison across profiles. |
| `run_pass_profile_metrics_analysis.py` | Runs rust-code-analysis metric comparison on problems passing all three profiles within a pipeline. |
| `run_common_pass_profile_metrics_analysis.py` | Runs rust-code-analysis metric comparison only on problems that pass in all pipelines. |
| `analyze_full_translation_loop_attempt_statuses.py` | Parses test reports across attempts and summarizes pass/fail/error status counts per profile. |
| `export_attempt_failing_cases.py` | Exports non-passing cases for a specific profile and attempt to a CSV with failure details. |
| `export_attempt_status_matrix.py` | Builds a CSV matrix of per-problem attempt statuses across all profiles in a pipeline. |
| `show_failed_test_cases.py` | Parses test reports to extract and export detailed per-sample failure information to CSV. |
| `venn_pass_profiles.py` | Generates Venn diagrams and intersection CSVs showing pass overlap across translation profiles. |
| `read_profile_intersections.py` | Finds problems passing all three profiles across pipelines and computes their intersection. |
| `beautify_multi_pipeline_metric_averages.py` | Computes and formats average code-quality metric comparisons across multiple pipelines. |
| `beautify_pass_profile_metric_averages.py` | Formats average rust-code-analysis metrics per profile into a beautified CSV and image. |
| `compare_specs_only_pipeline_metrics.py` | Compares code-quality metrics for the specs-only variant between two pipelines side by side. |

### Statistical Tests

| Script | Description |
|--------|-------------|
| `wilcoxon_metrics_test.py` | Runs Wilcoxon signed-rank tests comparing code metrics between two pipelines (between-pipeline). |
| `within_pipeline_statistical_test.py` | Runs Friedman and pairwise Wilcoxon tests comparing metrics across profiles within a single pipeline. |

### Plotting

| Script | Description |
|--------|-------------|
| `plot_full_translation_loop_attempt_statuses.py` | Plots a grouped stacked bar chart of cumulative attempt statuses per profile. |
| `plot_metrics_boxplots.py` | Plots boxplots of rust-code-analysis metric distributions per translation profile. |
| `plot_spec_generation_attempt_statuses.py` | Plots grouped bar charts of pass/fail statuses across spec-generation validation attempts. |

