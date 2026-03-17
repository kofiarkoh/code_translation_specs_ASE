
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
├── analyse_results/.                     # contains the script to run and analyze the translated codes from the llm
│   ├── run_analysis_scripts.py           # main script to run all other scripts for analyses in required order and stores output in ./results folder

- translate_spec_only.py  # sends request to openai apis for specs only translations
- translate_code_with_spec.py # sends request to openai apis for both code and specs translations
- translate_code_only_batch.py # sends request to openai apis for code only
```


