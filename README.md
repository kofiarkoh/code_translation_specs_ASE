
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

## Root Scripts

### Dataset Preparation

| Script | Description |
|--------|-------------|
| `scan_projects.py` | Scans CodeNet project folders and copies only those containing the configured source language. |
| `keep_one_c_file_per_problem.py` | Retains only the first C file per problem directory and deletes all extras. |
| `delete_unpaired_source_files.py` | Filters a dataset to keep only source files that have a matching counterpart in the target language. |
| `copy_successful_programs.py` | Copies program files listed in a success CSV into directories grouped by problem ID. |
| `filter_successful_problem_ids.py` | Filters a run-summary CSV to keep only problem IDs where all samples passed with zero failures. |
| `extract_sample_test_input_and_output.py` | Parses problem HTML files to extract sample input/output blocks into per-problem text files. |
| `extract_sample_test_input_and_output_fallback.py` | Extracts sample input/output pairs from plain-text `_test.txt` files as an HTML-fallback method. |

### Execution

| Script | Description |
|--------|-------------|
| `run_c_program_with_samples.py` | Compiles C programs and runs them against sample inputs, reporting pass/fail per problem. |
| `run_java_program_with_samples.py` | Compiles Java programs and runs them against sample inputs, reporting pass/fail per problem. |

### Spec Generation

| Script | Description |
|--------|-------------|
| `generate_code_specs_with_openai.py` | Sends source code files to the OpenAI API and saves the generated behavioral specifications. |
| `generate_validated_code_specs_with_openai.py` | Generates specs, validates them by regenerating C code, and iteratively refines until tests pass. |
| `generate_missing_spec_prompts.py` | Backfills missing `_prompt.txt` files by reconstructing the prompt used to generate each spec. |
| `validated_spec_prompts.py` | Defines prompt templates (system, spec generation, diagnosis, fix) for validated spec generation. |

### Plotting

| Script | Description |
|--------|-------------|
| `plot_validation_pass_distribution.py` | Plots stacked-bar and cumulative-pass charts of spec validation outcomes by attempt number. |

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

## Scripts in `cost_estimates/`

| Script | Description |
|--------|-------------|
| `cost_estimate_utils.py` | Shared utilities: model pricing, token counting (o200k_base / cl100k_base), cost computation, and CSV/table helpers. |
| `estimate_spec_generation_cost.py` | Estimates token usage and API cost for spec generation runs by scanning prompt/response files. |
| `estimate_translation_cost.py` | Estimates token usage and API cost for translation repair attempts by scanning repair prompt/response files. |
| `combine_cost_summaries.py` | Combines translation and spec-generation costs into a unified summary with allocated spec costs per profile. |
| `plot_spec_generation_costs.py` | Plots a bar chart of spec generation cost per directory, colored by model. |
| `plot_translation_costs.py` | Plots grouped bar charts of translation cost by pipeline and profile. |
| `plot_attempt_trends.py` | Plots cumulative repair attempt counts per pipeline and profile across attempt levels. |

