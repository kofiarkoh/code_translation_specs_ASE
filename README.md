
## Setup
- copy `.env.example` to `.env` file at the root level and add your openai api key
- Inside `.env`, make sure to set `DATASET_NAME` . This will be used to saved the results in a dedicated folder.
- create a new virtual environment and install the dependencies using the `requirements.txt` file


## Folder Structure


```text
project-root/
├── collected_dataset/                    # Datasets for each programming language, organized by language name
│   ├── C/                                # All files and resources related to the C language
│   │   └── final_problems/               # Final set of problems selected for the C language
│   └── problem_test_inputs_and_outputs/  # Contains the test inputs and outputs for the problems
├── full_translation/.                     # contains the script to run the translations
│   ├── full_translation_loop.py           # main  the translation for for each of the three variants.
│   ├── venn_pass_profiles.py              # generate a venn diagram for the the problems that passed the test cases
│   ├── run_all_profile_metrics_analysis.py          # runs rust-code-analysis for alll translated code irrespective or whether they are passing or failing test cases 
│   ├── run_pass_profile_metrics_analysis.py         # runs rust-code-analysis for the test passing translated code only [this uses the output from the venn_pass_profiles.py] 

```


