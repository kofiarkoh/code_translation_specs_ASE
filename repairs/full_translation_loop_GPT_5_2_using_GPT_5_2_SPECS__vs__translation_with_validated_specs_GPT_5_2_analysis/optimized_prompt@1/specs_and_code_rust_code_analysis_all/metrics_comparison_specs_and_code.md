# specs_and_code Rust Code Analysis Comparison

Pipeline A: `full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS`
Pipeline B: `translation_with_validated_specs_GPT_5_2`
Variant column: `specs_and_code`
Source A: `/Users/lithe/Documents/research/code_translation_specs/code_translation_specs_ASE/repairs/full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS_analysis/optimized_prompt@1/rust-code-analysis-all/metrics_comparison.csv`
Source B: `/Users/lithe/Documents/research/code_translation_specs/code_translation_specs_ASE/repairs/translation_with_validated_specs_GPT_5_2_analysis/optimized_prompt@1/rust-code-analysis-all/metrics_comparison.csv`

## Overall Averages

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.0361 | **59.9721 ✅** | -0.9359 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 36.7958 | **37.8016 ✅** | -1.0058 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17.6867 | **17.0900 ✅** | 0.5967 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22.6400 | **22.2767 ✅** | 0.3633 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2619.5529 | **2517.0589 ✅** | 102.4940 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 168419.2300 | **154876.3752 ✅** | 13542.8547 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 89.1000 | **84.9833 ✅** | 4.1167 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 68.2233 | **65.1333 ✅** | 3.0900 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 49.3567 | **47.5633 ✅** | 1.7933 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10.1300 ✅** | 9.3467 | 0.7833 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11.7567 | **11.6400 ✅** | 0.1167 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2.3500 | **2.2033 ✅** | 0.1467 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0.4833 | **0.4133 ✅** | 0.0700 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2.0033 | **1.8300 ✅** | 0.1733 | translation_with_validated_specs_GPT_5_2 |

## p00004

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 63.0001 | **67.0036 ✅** | -4.0036 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 45.5077 | **48.0555 ✅** | -2.5478 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | **15 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 8 | **6 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1585.9236 | **1272.0476 ✅** | 313.8760 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 43740.7958 | **31703.3394 ✅** | 12037.4564 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 54 | **50 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 42 | **38 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 17 | **16 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **8 ✅** | 6 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 5 | **0 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00010

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.0182 ✅** | 64.1400 | 2.8782 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **45.0219 ✅** | 37.6619 | 7.3600 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1974.1396 ✅** | 2208.0565 | -233.9169 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **68162.6533 ✅** | 80830.6385 | -12667.9852 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **50 ✅** | 56 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **35 ✅** | 40 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **32 ✅** | 34 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **5 ✅** | 4 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00013

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **81.3134 ✅** | 80.3959 | 0.9174 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 73.5169 | **76.8745 ✅** | -3.3577 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 4 | 4 | 0 | tie |
| Halstead Volume | Lower | 585.4121 | **574.7682 ✅** | 10.6439 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 9339.9838 | **9170.1659 ✅** | 169.8179 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **31 ✅** | 33 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 19 | 19 | 0 | tie |
| LLOC | Lower | 16 | 16 | 0 | tie |
| Comments (CLOC) | Higher | 6 | **9 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00015

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.5119 | **53.7846 ✅** | -1.2728 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 27.9453 | **31.1370 ✅** | -3.1917 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | **20 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 34 | **26 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2345.0064 | **2307.1838 ✅** | 37.8227 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 119595.3284 | **110895.2887 ✅** | 8700.0397 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 91 | **87 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 67 | **61 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 45 | **41 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | **12 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00023

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.4386 ✅** | 66.1493 | 2.2893 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 35.1542 | **47.4373 ✅** | -12.2831 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1526.2790 ✅** | 1559.7839 | -33.5050 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **40700.7724 ✅** | 40975.9451 | -275.1728 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **47 ✅** | 53 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 38 | **37 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 25 | **23 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 1 | **7 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00024

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 82.2554 | **84.1737 ✅** | -1.9183 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **84.2680 ✅** | 77.6603 | 6.6077 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 575.1841 | **558.9053 ✅** | 16.2788 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 7661.4517 | **7444.6182 ✅** | 216.8335 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 29 | **26 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 16 | 16 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | **11 ✅** | 5 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00056

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.8796 ✅** | 66.9813 | 0.8984 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 44.5712 | **47.1544 ✅** | -2.5832 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | 13 | 0 | tie |
| Cognitive (sum) | Lower | 17 | 17 | 0 | tie |
| Halstead Volume | Lower | **1104 ✅** | 1164 | -60 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **31558.6286 ✅** | 35684.9143 | -4126.2857 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **51 ✅** | 53 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **38 ✅** | 39 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **24 ✅** | 25 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **6 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00071

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **60.9230 ✅** | 56.8871 | 4.0359 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **40.0283 ✅** | 36.1193 | 3.9090 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 16 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **18 ✅** | 22 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2190.3234 ✅** | 2510.5806 | -320.2572 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **65285.0467 ✅** | 95496.8017 | -30211.7550 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **62 ✅** | 74 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **45 ✅** | 51 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **33 ✅** | 40 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **11 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 13 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00074

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **64.0297 ✅** | 62.4028 | 1.6269 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.3382 ✅** | 42.7631 | 1.5751 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 18 | 18 | 0 | tie |
| Cognitive (sum) | Lower | 11 | 11 | 0 | tie |
| Halstead Volume | Lower | **1312.1144 ✅** | 1452.3896 | -140.2753 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **46376.4555 ✅** | 49925.8930 | -3549.4375 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **57 ✅** | 61 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **47 ✅** | 49 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **15 ✅** | 17 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00077

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 69.6801 | **76.2968 ✅** | -6.6167 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 60.9831 | **67.6057 ✅** | -6.6227 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | **6 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22 | **11 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 932.0418 | **745.6809 ✅** | 186.3610 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 23162.9656 | **17430.2902 ✅** | 5732.6754 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 51 | **38 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 32 | **26 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 24 | **21 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 8 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **6 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00080

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 74.8248 | **75.2350 ✅** | -0.4101 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **63.2457 ✅** | 62.2016 | 1.0442 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 7 | 7 | 0 | tie |
| Halstead Volume | Lower | 772.0879 | 772.0879 | 0 | tie |
| Halstead Effort | Lower | **16985.9327 ✅** | 17325.6514 | -339.7187 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 40 | **39 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 26 | 26 | 0 | tie |
| LLOC | Lower | 17 | 17 | 0 | tie |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00105

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **70.3818 ✅** | 54.0388 | 16.3430 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **53.2933 ✅** | 24.9219 | 28.3713 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **9 ✅** | 18 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **10 ✅** | 33 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1123.1402 ✅** | 2086.6543 | -963.5140 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **23187.4109 ✅** | 65729.6092 | -42542.1983 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **46 ✅** | 91 | -45 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **33 ✅** | 72 | -39 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **21 ✅** | 44 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **7 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 12 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00108

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.6038 | **57.5874 ✅** | -4.9836 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 31.3023 | **36.8351 ✅** | -5.5328 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | **15 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 25 | **22 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2014.9502 | **1665.7431 ✅** | 349.2071 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 74970.8910 | **61275.5515 ✅** | 13695.3395 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 95 | **82 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 68 | **58 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **42 ✅** | 46 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **15 ✅** | 12 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | **13 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00118

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.0005 | **53.2628 ✅** | -6.2622 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 21.3243 | **31.6634 ✅** | -10.3391 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 29 | **24 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **32 ✅** | 35 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3480 | **2670 ✅** | 810 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 292037.8378 | **158021.8421 ✅** | 134015.9957 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 102 | **81 ✅** | 21 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 76 | **60 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 56 | **51 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 12 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | **10 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00124

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 66.0279 | **66.8657 ✅** | -0.8377 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **47.6803 ✅** | 46.6822 | 0.9982 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | **1260.1960 ✅** | 1359.0516 | -98.8556 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **27667.0308 ✅** | 29918.5500 | -2251.5193 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 58 | **53 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **39 ✅** | 40 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 26 | 26 | 0 | tie |
| Comments (CLOC) | Higher | **8 ✅** | 6 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00136

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 60.6201 | **61.9053 ✅** | -1.2852 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 31.4269 | **33.7158 ✅** | -2.2889 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 12 | **9 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1819.8617 | **1794.4727 ✅** | 25.3890 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 54595.8505 | **50220.9852 ✅** | 4374.8654 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 68 | **64 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 51 | **49 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 46 | **45 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | 4 | 0 | tie |
| Blank Lines | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00137

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **79.3107 ✅** | 71.3018 | 8.0090 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **65.2667 ✅** | 59.2652 | 6.0015 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **5 ✅** | 11 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **4 ✅** | 7 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **745.6809 ✅** | 1062.7460 | -317.0651 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **14093.3683 ✅** | 26568.6488 | -12475.2804 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **32 ✅** | 43 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **22 ✅** | 30 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **19 ✅** | 21 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **8 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00146

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **47.5854 ✅** | 47.5777 | 0.0077 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **23.5901 ✅** | 18.0831 | 5.5070 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **20 ✅** | 22 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **37 ✅** | 38 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3659.7622 ✅** | 3765.2748 | -105.5126 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **235329.6162 ✅** | 248037.4772 | -12707.8609 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 110 | **106 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **82 ✅** | 85 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 70 | 70 | 0 | tie |
| Comments (CLOC) | Higher | **17 ✅** | 10 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00156

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 34.3871 | **37.1248 ✅** | -2.7377 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -2.6027 | **1.9924 ✅** | -4.5951 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 40 | **37 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 46 | **39 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 5306.4388 | **4908.1135 ✅** | 398.3253 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 382063.5930 | **307897.7100 ✅** | 74165.8830 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 166 | **150 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 135 | **121 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 108 | **88 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | 11 | 0 | tie |
| Blank Lines | Lower | 21 | 21 | 0 | tie |
| Functions | Lower | 7 | **5 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00167

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 64.5574 | **71.1291 ✅** | -6.5718 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 49.4834 | **52.6226 ✅** | -3.1391 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **10 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 18 | **9 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1185.5068 | **996.6582 ✅** | 188.8487 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 35258.6082 | **24332.2061 ✅** | 10926.4021 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 57 | **45 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 44 | **32 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 23 | **21 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 5 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | **0 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |

## p00187

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 43.6803 | **45.8590 ✅** | -2.1786 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 10.5123 | **13.4777 ✅** | -2.9654 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 21 | **19 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22 | **19 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **4741.9310 ✅** | 4770.0956 | -28.1646 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **263551.5349 ✅** | 274344.9579 | -10793.4229 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 127 | **114 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 99 | **89 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 77 | **55 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 9 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 18 | **16 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00190

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 28.1362 | **29.9597 ✅** | -1.8235 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -7.0997 | **-2.3477 ✅** | -4.7521 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 39 | **36 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 64 | **62 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6772.8455 | **6533.9185 ✅** | 238.9270 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 465780.3659 | **443839.7498 ✅** | 21940.6161 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 229 | **216 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 190 | **176 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 119 | **84 ✅** | 35 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 24 | **28 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 29 | **28 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 11 | **7 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p00203

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **49.3326 ✅** | 48.8166 | 0.5160 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 25.1483 | **28.1940 ✅** | -3.0457 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **26 ✅** | 28 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **66 ✅** | 67 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2461.7354 | **2343.6732 ✅** | 118.0622 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 143916.8362 | **143344.1199 ✅** | 572.7163 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **103 ✅** | 105 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 79 | **77 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 56 | **54 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 14 | **19 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 11 | **9 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00208

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 69.2240 | **73.3048 ✅** | -4.0808 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **58.7957 ✅** | 58.3185 | 0.4772 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 17 | **15 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 863.5100 | **789.8694 ✅** | 73.6406 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 21122.7830 | **18956.8646 ✅** | 2165.9184 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 53 | **43 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 33 | **30 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 25 | **23 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 6 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **7 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00244

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.3087 | **46.4067 ✅** | -0.0980 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **22.3653 ✅** | 17.4802 | 4.8851 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **21 ✅** | 22 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **24 ✅** | 28 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3596.5373 ✅** | 3658.7894 | -62.2520 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **173532.9269 ✅** | 192622.6448 | -19089.7178 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 118 | **115 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **82 ✅** | 86 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **58 ✅** | 60 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **19 ✅** | 12 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 17 | 17 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00252

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **79.8230 ✅** | 71.1653 | 8.6577 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **62.4049 ✅** | 47.9642 | 14.4407 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **17 ✅** | 26 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **6 ✅** | 9 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **857.5470 ✅** | 1167.9463 | -310.3993 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **21081.3638 ✅** | 38834.2157 | -17752.8519 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **25 ✅** | 34 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **20 ✅** | 29 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 12 | 12 | 0 | tie |
| Comments (CLOC) | Higher | 2 | 2 | 0 | tie |
| Blank Lines | Lower | 3 | 3 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00256

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **45.0190 ✅** | 40.8373 | 4.1817 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **20.2225 ✅** | 14.1629 | 6.0596 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **20 ✅** | 22 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **42 ✅** | 52 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3926.9481 ✅** | 4299.1412 | -372.1930 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **237362.1988 ✅** | 239044.0275 | -1681.8288 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **126 ✅** | 154 | -28 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **88 ✅** | 105 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **93 ✅** | 110 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 20 | **24 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **19 ✅** | 27 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00261

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 67.6064 | **69.9724 ✅** | -2.3660 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 57.5187 | **60.7489 ✅** | -3.2302 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 11 | 11 | 0 | tie |
| Halstead Volume | Lower | 1133.7128 | **1043.1805 ✅** | 90.5324 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **18491.7756 ✅** | 23673.4664 | -5181.6908 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 56 | **49 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 41 | **33 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 19 | **16 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **14 ✅** | 12 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **7 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00270

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 24.3935 | **34.1510 ✅** | -9.7575 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -21.9945 | **-3.9833 ✅** | -18.0113 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 51 | **35 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 65 | **55 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 7963.0641 | **5773.2612 ✅** | 2189.8029 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 622340.6092 | **368289.0800 ✅** | 254051.5292 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 231 | **176 ✅** | 55 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 207 | **144 ✅** | 63 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 138 | **107 ✅** | 31 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | **11 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **18 ✅** | 22 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 10 | **5 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 8 | **4 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p00271

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 90.3083 | **92.2658 ✅** | -1.9575 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 86.8274 | **87.2723 ✅** | -0.4449 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 1 | 1 | 0 | tie |
| Halstead Volume | Lower | 425 | **405 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **4131.9444 ✅** | 4288.2353 | -156.2908 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 20 | **18 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **4 ✅** | 3 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00283

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **50.4871 ✅** | 48.9853 | 1.5018 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 18.3552 | **22.0022 ✅** | -3.6470 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | 22 | 0 | tie |
| Cognitive (sum) | Lower | **21 ✅** | 24 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3027.2059 | **2789.5340 ✅** | 237.6719 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 156493.3824 | **123067.6765 ✅** | 33425.7059 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **95 ✅** | 107 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **76 ✅** | 85 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 54 | **45 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **12 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 15 | 15 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 3 | 3 | 0 | tie |

## p00288

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 21.3545 | **22.3611 ✅** | -1.0067 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -25.3271 | **-21.6448 ✅** | -3.6822 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 45 | 45 | 0 | tie |
| Cognitive (sum) | Lower | 87 | **84 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 9681 | **9226 ✅** | 455 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 874412.9032 | **839365.4348 ✅** | 35047.4684 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 285 | **272 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 239 | **222 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 168 | **162 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | **14 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **36 ✅** | 37 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 6 | 6 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p00292

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 71.6773 | **84.6054 ✅** | -12.9281 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 56.9588 | **78.5998 ✅** | -21.6410 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | **3 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 7 | **1 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1049.7573 | **634.9992 ✅** | 414.7580 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 29393.2030 | **8081.8084 ✅** | 21311.3947 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 41 | **25 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 28 | **15 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 14 | **11 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **6 ✅** | 5 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **6 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00315

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **53.0236 ✅** | 52.2778 | 0.7459 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **29.7717 ✅** | 24.8760 | 4.8957 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 16 | 16 | 0 | tie |
| Cognitive (sum) | Lower | 19 | 19 | 0 | tie |
| Halstead Volume | Lower | **3304.5790 ✅** | 3310.5331 | -5.9542 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **220764.2328 ✅** | 221162.0062 | -397.7734 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **86 ✅** | 90 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **63 ✅** | 68 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **52 ✅** | 53 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **12 ✅** | 9 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | 13 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 4 | 4 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00317

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 31.1430 | **32.0681 ✅** | -0.9251 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -2.7947 | **5.4645 ✅** | -8.2592 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 41 | **35 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **73 ✅** | 77 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 5737.0175 | **5440.3639 ✅** | 296.6536 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 405689.0913 | **340868.2031 ✅** | 64820.8882 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **195 ✅** | 204 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 162 | **143 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 126 | **125 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 20 | **39 ✅** | -19 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **16 ✅** | 23 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 7 | **3 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00320

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **46.7954 ✅** | 46.2278 | 0.5676 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **14.5413 ✅** | 13.7380 | 0.8033 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **24 ✅** | 25 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **22 ✅** | 23 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3569.3907 ✅** | 3702.9352 | -133.5445 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **241573.6685 ✅** | 265399.4611 | -23825.7927 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **110 ✅** | 111 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **88 ✅** | 92 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **51 ✅** | 56 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | 8 | 0 | tie |
| Blank Lines | Lower | 14 | **12 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p00329

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 38.2392 | **39.6947 ✅** | -1.4555 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 4.3467 | **4.3911 ✅** | -0.0445 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 36 | **26 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 45 | **28 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **4140.4665 ✅** | 4579.1894 | -438.7228 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 267405.1310 | **248490.4044 ✅** | 18914.7266 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **150 ✅** | 153 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 126 | 126 | 0 | tie |
| LLOC | Lower | **82 ✅** | 90 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **12 ✅** | 11 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **13 ✅** | 16 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 8 | 8 | 0 | tie |

## p00336

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **73.0832 ✅** | 72.8108 | 0.2724 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 60.5294 | **61.5611 ✅** | -1.0317 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 7 | 7 | 0 | tie |
| Halstead Volume | Lower | 1179.0891 | **1150.5023 ✅** | 28.5868 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 37583.4640 | **36688.2392 ✅** | 895.2248 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **40 ✅** | 41 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 26 | 26 | 0 | tie |
| LLOC | Lower | 21 | 21 | 0 | tie |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 7 | 7 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00378

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **78.2998 ✅** | 78.1398 | 0.1600 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **68.9542 ✅** | 56.5263 | 12.4279 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 9 | 9 | 0 | tie |
| Halstead Volume | Lower | **866.5159 ✅** | 893.5940 | -27.0781 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **22604.7616 ✅** | 23233.4440 | -628.6824 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 32 | 32 | 0 | tie |
| PLOC | Lower | **25 ✅** | 26 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **21 ✅** | 22 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **6 ✅** | 2 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00428

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 66.8960 | **68.1709 ✅** | -1.2749 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **50.5782 ✅** | 49.9081 | 0.6702 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | 9 | 9 | 0 | tie |
| Halstead Volume | Lower | **1312.3687 ✅** | 1317.8692 | -5.5006 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **34544.9298 ✅** | 35846.0433 | -1301.1135 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 52 | **48 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 37 | **35 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 26 | 26 | 0 | tie |
| Comments (CLOC) | Higher | **8 ✅** | 6 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00435

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 77.7237 | **86.9210 ✅** | -9.1973 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 72.1148 | **76.7469 ✅** | -4.6321 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 7 | **5 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 9 | **5 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 641.6858 | **482.8093 ✅** | 158.8764 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 9625.2866 | **6448.9532 ✅** | 3176.3334 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 36 | **23 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 24 | **16 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 11 | **8 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **9 ✅** | 3 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00445

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **79.1276 ✅** | 77.5661 | 1.5615 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **62.0065 ✅** | 61.9215 | 0.0850 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 7 | 7 | 0 | tie |
| Cognitive (sum) | Lower | 14 | 14 | 0 | tie |
| Halstead Volume | Lower | **707.0159 ✅** | 722.0966 | -15.0806 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 15230.3015 | **14289.1798 ✅** | 941.1217 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **32 ✅** | 35 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 3 | **4 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00450

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.1349 | **67.2683 ✅** | -2.1333 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 44.9233 | **50.8784 ✅** | -5.9551 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | 16 | **13 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1242.4056 | **1151.3120 ✅** | 91.0936 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **35427.9714 ✅** | 41247.9669 | -5819.9955 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 59 | **53 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 45 | **39 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 36 | **32 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00453

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 26.9852 | **44.2014 ✅** | -17.2162 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -1.7744 | **17.2554 ✅** | -19.0297 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 51 | **29 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 116 | **58 ✅** | 58 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6911.8909 | **3786.2441 ✅** | 3125.6468 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 806582.5185 | **252442.0283 ✅** | 554140.4902 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 206 | **118 ✅** | 88 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 150 | **93 ✅** | 57 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 112 | **69 ✅** | 43 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **35 ✅** | 15 | 20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 21 | **12 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 5 | **0 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00465

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 27.4034 | **43.6759 ✅** | -16.2725 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -17.3179 | **15.1110 ✅** | -32.4289 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 44 | **27 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 51 | **30 ✅** | 21 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6512.8182 | **4124.9455 ✅** | 2387.8726 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 508949.6018 | **216028.3971 ✅** | 292921.2047 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 226 | **122 ✅** | 104 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 200 | **94 ✅** | 106 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 119 | **68 ✅** | 51 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 8 | **14 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 19 | **17 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 9 | **4 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | **3 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p00468

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.4550 | **62.8204 ✅** | -16.3655 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 20.0040 | **40.9411 ✅** | -20.9371 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 26 | **13 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 30 | **17 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3297.8614 | **1512.1710 ✅** | 1785.6904 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 224127.7325 | **45152.4810 ✅** | 178975.2516 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 112 | **63 ✅** | 49 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 83 | **46 ✅** | 37 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 60 | **37 ✅** | 23 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **14 ✅** | 7 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | **11 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00479

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 57.1494 | **75.1355 ✅** | -17.9861 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 31.5135 | **58.0970 ✅** | -26.5835 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **7 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 23 | **3 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2185.0081 | **1055.5655 ✅** | 1129.4426 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 99990.1312 | **19387.2188 ✅** | 80602.9124 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 74 | **36 ✅** | 38 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 58 | **25 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 44 | **15 ✅** | 29 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 4 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | **2 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00492

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.6502 | **52.6747 ✅** | -0.0244 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 26.9034 | **32.6704 ✅** | -5.7670 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **16 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **27 ✅** | 30 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2920.7308 | **2769.4162 ✅** | 151.3146 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 157063.7879 | **147240.6280 ✅** | 9823.1598 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **89 ✅** | 93 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 67 | **66 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **53 ✅** | 56 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | **17 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **12 ✅** | 13 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00495

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 50.5372 | **60.8284 ✅** | -10.2911 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 24.8654 | **41.1880 ✅** | -16.3226 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17 | **13 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 27 | **15 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2738.3689 | **1670.6706 ✅** | 1067.6983 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 122512.2449 | **69071.7887 ✅** | 53440.4562 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 105 | **69 ✅** | 36 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 81 | **47 ✅** | 34 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 68 | **36 ✅** | 32 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 10 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | 14 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00507

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 53.8200 | **68.6180 ✅** | -14.7980 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 32.2285 | **58.3460 ✅** | -26.1174 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | **9 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 26 | **17 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2024.1721 | **1143.2464 ✅** | 880.9256 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 65036.6481 | **33559.8149 ✅** | 31476.8331 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 88 | **51 ✅** | 37 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 63 | **31 ✅** | 32 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 27 | **22 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 12 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | **8 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00514

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 42.2112 | **48.8718 ✅** | -6.6606 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 7.8926 | **22.4415 ✅** | -14.5489 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 26 | **20 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 18 | **16 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3640.1475 | **3025.8148 ✅** | 614.3328 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 257464.6003 | **166104.6227 ✅** | 91359.9776 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 141 | **108 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 115 | **83 ✅** | 32 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 84 | **60 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 10 | **13 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 19 | **15 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 8 | **3 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |

## p00518

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **55.5263 ✅** | 54.8380 | 0.6882 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **39.1011 ✅** | 37.8047 | 1.2964 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 17 | 17 | 0 | tie |
| Cognitive (sum) | Lower | **27 ✅** | 32 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1818.8151 ✅** | 1935.8132 | -116.9981 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **62492.1130 ✅** | 69504.9114 | -7012.7984 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **88 ✅** | 90 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **57 ✅** | 61 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **45 ✅** | 54 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 19 | 19 | 0 | tie |
| Blank Lines | Lower | 13 | 13 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00519

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **45.4862 ✅** | 42.9543 | 2.5319 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **20.1961 ✅** | 14.8691 | 5.3270 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 33 | 33 | 0 | tie |
| Cognitive (sum) | Lower | 32 | 32 | 0 | tie |
| Halstead Volume | Lower | **3672.3369 ✅** | 3826.4064 | -154.0695 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 255500.8873 | **233758.6448 ✅** | 21742.2425 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **104 ✅** | 120 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **77 ✅** | 92 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 50 | **49 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 14 | 14 | 0 | tie |
| Blank Lines | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 8 | 8 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00528

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **39.1914 ✅** | 35.8739 | 3.3176 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **8.0728 ✅** | 3.6366 | 4.4362 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **28 ✅** | 34 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **32 ✅** | 40 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5120.9961 ✅** | 5718.2685 | -597.2724 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **350032.6785 ✅** | 389495.7756 | -39463.0972 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **148 ✅** | 161 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **115 ✅** | 131 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **85 ✅** | 108 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 16 | **17 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 20 | **19 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00534

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 68.9880 | **70.0199 ✅** | -1.0319 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 55.7238 | **56.5175 ✅** | -0.7936 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | **1344.2105 ✅** | 1347.7491 | -3.5386 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 44309.1610 | **42204.5140 ✅** | 2104.6470 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 48 | **45 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 29 | 29 | 0 | tie |
| LLOC | Lower | 25 | 25 | 0 | tie |
| Comments (CLOC) | Higher | **9 ✅** | 8 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00543

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 73.1688 | **76.1393 ✅** | -2.9705 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **63.7599 ✅** | 62.4901 | 1.2698 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 7 | 7 | 0 | tie |
| Cognitive (sum) | Lower | 8 | 8 | 0 | tie |
| Halstead Volume | Lower | 1027.4919 | **950.0906 ✅** | 77.4013 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 30578.1590 | **24524.2132 ✅** | 6053.9457 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 41 | **35 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 27 | **25 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **18 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **9 ✅** | 5 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00554

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 62.8548 | **78.4551 ✅** | -15.6002 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 48.9284 | **64.0322 ✅** | -15.1038 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | **5 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 17 | **5 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1556.2200 | **879.0549 ✅** | 677.1652 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 56490.7876 | **16816.7022 ✅** | 39674.0853 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 65 | **32 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 43 | **24 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 41 | **15 ✅** | 26 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **14 ✅** | 4 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **5 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00571

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **58.7776 ✅** | 52.1370 | 6.6406 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **33.8486 ✅** | 25.6265 | 8.2221 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **15 ✅** | 21 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 17 | **15 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1986.8409 ✅** | 2229.9171 | -243.0761 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **72562.8868 ✅** | 83646.1291 | -11083.2423 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **72 ✅** | 96 | -24 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **55 ✅** | 72 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 38 | 38 | 0 | tie |
| Comments (CLOC) | Higher | 7 | **10 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 16 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 6 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | **1 ✅** | 4 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00594

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 63.9767 | **64.2445 ✅** | -0.2678 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **47.1659 ✅** | 45.0059 | 2.1600 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | 11 | 11 | 0 | tie |
| Halstead Volume | Lower | 1399.2361 | 1399.2361 | 0 | tie |
| Halstead Effort | Lower | 43716.1346 | 43716.1346 | 0 | tie |
| SLOC | Lower | 61 | **60 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 45 | 45 | 0 | tie |
| LLOC | Lower | 37 | 37 | 0 | tie |
| Comments (CLOC) | Higher | **10 ✅** | 8 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00629

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **56.2131 ✅** | 53.8357 | 2.3774 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 24.3092 | **36.3118 ✅** | -12.0026 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | 18 | 0 | tie |
| Cognitive (sum) | Lower | **20 ✅** | 22 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2133.9623 ✅** | 2169.7662 | -35.8039 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 61031.3204 | **54151.8242 ✅** | 6879.4961 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **79 ✅** | 91 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **64 ✅** | 71 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **36 ✅** | 38 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **19 ✅** | -15 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 3 | 3 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00641

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 38.3444 | **39.8830 ✅** | -1.5386 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -2.1729 | **0.6981 ✅** | -2.8710 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **31 ✅** | 32 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 26 | **25 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4663.3704 | **4180.7534 ✅** | 482.6170 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **297128.4412 ✅** | 307285.3745 | -10156.9333 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 154 | **143 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 129 | **119 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 84 | 84 | 0 | tie |
| Comments (CLOC) | Higher | 6 | 6 | 0 | tie |
| Blank Lines | Lower | 19 | 19 | 0 | tie |
| Functions | Lower | 8 | **6 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00649

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **44.9852 ✅** | 40.7344 | 4.2508 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **14.1737 ✅** | 6.2365 | 7.9372 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **25 ✅** | 30 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **36 ✅** | 41 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3503.3718 ✅** | 4142.4618 | -639.0901 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **142236.8945 ✅** | 226454.5807 | -84217.6862 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **122 ✅** | 140 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **95 ✅** | 113 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **63 ✅** | 80 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **11 ✅** | 10 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 17 | 17 | 0 | tie |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00653

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 27.7780 | **34.0572 ✅** | -6.2792 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -15.6282 | **-1.0362 ✅** | -14.5920 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 43 | **35 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 30 | 30 | 0 | tie |
| Halstead Volume | Lower | 7507.7901 | **6091.4020 ✅** | 1416.3881 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 637089.6151 | **533497.7137 ✅** | 103591.9014 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 214 | **174 ✅** | 40 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 188 | **134 ✅** | 54 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 103 | **97 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | **15 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **18 ✅** | 25 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 12 | **8 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 14 | **9 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |

## p00655

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 19.1645 | **23.3360 ✅** | -4.1715 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -28.5202 | **-22.1161 ✅** | -6.4041 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 61 | **56 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 55 | **49 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 9226.5055 | **7822.6860 ✅** | 1403.8195 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 1188748.3163 | **981541.2309 ✅** | 207207.0854 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 264 | **231 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 238 | **203 ✅** | 35 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 155 | **122 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 8 | 8 | 0 | tie |
| Blank Lines | Lower | 20 | 20 | 0 | tie |
| Functions | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00658

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **-10.0371 ✅** | -10.2447 | 0.2076 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **-59.4154 ✅** | -59.6579 | 0.2424 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 140 | **139 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **178 ✅** | 181 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **15133.5808 ✅** | 15152.4918 | -18.9109 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 1998396.9911 | **1989769.5545 ✅** | 8627.4366 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **445 ✅** | 457 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **396 ✅** | 407 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **260 ✅** | 265 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 21 | **22 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **28 ✅** | 29 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 12 | 12 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **21 ✅** | 29 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00680

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 18.2286 | **19.2441 ✅** | -1.0155 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -26.8374 | **-26.4471 ✅** | -0.3904 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 61 | **60 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 82 | **76 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 9094.2591 | **8855 ✅** | 239.2591 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **791481.8039 ✅** | 793485 | -2003.1961 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 281 | **270 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 237 | **229 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 152 | **145 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 11 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 32 | 32 | 0 | tie |
| Functions | Lower | 10 | 10 | 0 | tie |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | 10 | 10 | 0 | tie |

## p00690

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 43.5044 | **45.4699 ✅** | -1.9655 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 12.0791 | **12.2509 ✅** | -0.1718 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **23 ✅** | 26 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 42 | **34 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4174.8662 | **3478.6814 ✅** | 696.1848 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 209064.4548 | **179929.2444 ✅** | 29135.2104 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 130 | **117 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 103 | **96 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 81 | **63 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 8 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 15 | **13 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00692

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 44.4131 | **46.2490 ✅** | -1.8360 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 15.1557 | **20.0618 ✅** | -4.9061 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **31 ✅** | 33 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **25 ✅** | 31 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 4260.4707 | **3697.4041 ✅** | 563.0667 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 304344.2826 | **269045.1464 ✅** | 35299.1362 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 109 | **99 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 85 | **78 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 64 | **60 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | **12 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 15 | **9 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 5 | **0 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |

## p00723

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 50.1937 | **58.7881 ✅** | -8.5944 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 20.4503 | **35.0411 ✅** | -14.5908 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17 | **8 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 11 | **5 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3013.9182 | **2105.9862 ✅** | 907.9320 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 140996.5718 | **71375.8570 ✅** | 69620.7148 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 104 | **78 ✅** | 26 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 81 | **52 ✅** | 29 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 51 | **38 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | 9 | 0 | tie |
| Blank Lines | Lower | **14 ✅** | 18 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00740

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **62.9353 ✅** | 62.7368 | 0.1985 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 39.3587 | **40.3024 ✅** | -0.9438 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 14 | **13 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 18 | 18 | 0 | tie |
| Halstead Volume | Lower | 1347.3976 | **1329.3790 ✅** | 18.0186 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 38217.0954 | **37973.0683 ✅** | 244.0271 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **64 ✅** | 66 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **50 ✅** | 51 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **40 ✅** | 43 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **7 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00749

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 20.2244 | **22.4827 ✅** | -2.2583 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -20.1852 | **-20.0481 ✅** | -0.1371 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 57 | **53 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 78 | **70 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 8570.5535 | **7927.9652 ✅** | 642.5884 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 796849.8610 | **760650.2478 ✅** | 36199.6131 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 268 | **253 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 225 | **215 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 166 | **153 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **20 ✅** | 14 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 28 | **26 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | 6 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 13 | 13 | 0 | tie |

## p00762

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 37.5597 | **37.6087 ✅** | -0.0490 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 5.5855 | **5.6562 ✅** | -0.0707 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 31 | 31 | 0 | tie |
| Cognitive (sum) | Lower | 19 | 19 | 0 | tie |
| Halstead Volume | Lower | 5422.9919 | **5372.1314 ✅** | 50.8604 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 422993.3651 | **419026.2526 ✅** | 3967.1125 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 154 | 154 | 0 | tie |
| PLOC | Lower | 126 | 126 | 0 | tie |
| LLOC | Lower | 100 | 100 | 0 | tie |
| Comments (CLOC) | Higher | 16 | 16 | 0 | tie |
| Blank Lines | Lower | 21 | 21 | 0 | tie |
| Functions | Lower | 8 | 8 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | 5 | 0 | tie |

## p00792

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 50.2288 | **50.5392 ✅** | -0.3104 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **26.4328 ✅** | 22.0858 | 4.3470 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 20 | 20 | 0 | tie |
| Cognitive (sum) | Lower | 22 | 22 | 0 | tie |
| Halstead Volume | Lower | **3154.8203 ✅** | 3169.1902 | -14.3699 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 186941.4431 | **168346.0041 ✅** | 18595.4390 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 98 | **96 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **73 ✅** | 74 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 70 | 70 | 0 | tie |
| Comments (CLOC) | Higher | **14 ✅** | 9 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **12 ✅** | 14 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00799

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **43.8726 ✅** | 41.7823 | 2.0903 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **18.5701 ✅** | 13.0592 | 5.5110 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **44 ✅** | 62 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2954.1390 ✅** | 3417.6163 | -463.4773 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **131089.9200 ✅** | 179843.3420 | -48753.4220 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **142 ✅** | 152 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **104 ✅** | 117 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **68 ✅** | 91 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **22 ✅** | 19 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00811

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **52.7468 ✅** | 51.3897 | 1.3571 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 18.3724 | **19.6510 ✅** | -1.2786 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **19 ✅** | 21 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23 ✅** | 25 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2649.1213 ✅** | 2749.0881 | -99.9668 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **133221.3650 ✅** | 142036.2188 | -8814.8538 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **90 ✅** | 94 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **70 ✅** | 74 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **60 ✅** | 62 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **6 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 16 | 16 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00816

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.3241 | **52.5304 ✅** | -0.2063 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **26.6390 ✅** | 24.9674 | 1.6716 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 19 | 19 | 0 | tie |
| Cognitive (sum) | Lower | 31 | **28 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **2275.4064 ✅** | 2333.5639 | -58.1574 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **96307.9002 ✅** | 108266.5093 | -11958.6090 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 97 | **95 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 76 | **74 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 58 | **52 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 9 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00820

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.7303 ✅** | 72.3656 | 4.3647 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 48.9406 | **53.0818 ✅** | -4.1412 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **8 ✅** | 11 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **7 ✅** | 10 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **888.0090 ✅** | 1004.6724 | -116.6634 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **24509.0494 ✅** | 28298.2726 | -3789.2232 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **34 ✅** | 41 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **29 ✅** | 34 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **21 ✅** | 22 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 1 | **4 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00837

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 40.1650 | **45.3209 ✅** | -5.1559 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 8.4587 | **19.1390 ✅** | -10.6803 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 23 | **17 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 44 | **27 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4587.3758 | **3660.3964 ✅** | 926.9794 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 325945.1204 | **184608.2922 ✅** | 141336.8282 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 155 | **132 ✅** | 23 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 114 | **95 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 111 | **81 ✅** | 30 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 16 | **19 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 26 | **21 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00846

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 40.6167 | **41.3133 ✅** | -0.6966 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **6.3645 ✅** | 5.0507 | 1.3138 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 20 | 20 | 0 | tie |
| Cognitive (sum) | Lower | 21 | **18 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4436.0221 | **4117.0409 ✅** | 318.9812 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 222216.9836 | **208085.0160 ✅** | 14131.9676 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 159 | **156 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 128 | **126 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 49 | **48 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 10 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 20 | 20 | 0 | tie |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00855

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **69.5496 ✅** | 67.6483 | 1.9013 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 49.5039 | **54.9505 ✅** | -5.4466 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 11 | 11 | 0 | tie |
| Cognitive (sum) | Lower | 12 | 12 | 0 | tie |
| Halstead Volume | Lower | **990.9302 ✅** | 996.6582 | -5.7279 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 29581.1027 | **28031.0107 ✅** | 1550.0920 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **49 ✅** | 55 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **36 ✅** | 37 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **25 ✅** | 26 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | **11 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00856

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.4361 | **52.8127 ✅** | -5.3766 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 19.2095 | **24.8022 ✅** | -5.5928 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 26 | **24 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 27 | **23 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3148.3202 | **2802.2137 ✅** | 346.1065 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 178265.5994 | **152195.2317 ✅** | 26070.3678 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 107 | **82 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 81 | **66 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 50 | **38 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 7 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 15 | **12 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00866

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **47.1700 ✅** | 46.3486 | 0.8213 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 13.9691 | **17.3112 ✅** | -3.3421 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | 18 | 0 | tie |
| Cognitive (sum) | Lower | 25 | 25 | 0 | tie |
| Halstead Volume | Lower | **3670.3962 ✅** | 3768.8886 | -98.4924 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **171679.8212 ✅** | 181083.3190 | -9403.4978 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **116 ✅** | 121 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **93 ✅** | 94 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **59 ✅** | 60 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **13 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00867

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **11.9614 ✅** | 6.8085 | 5.1529 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **-31.7969 ✅** | -39.2737 | 7.4768 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **74 ✅** | 85 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **94 ✅** | 103 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **12327.6897 ✅** | 13534.6306 | -1206.9409 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **1304680.4942 ✅** | 1557523.6405 | -252843.1463 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **312 ✅** | 356 | -44 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **263 ✅** | 305 | -42 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **188 ✅** | 207 | -19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 20 | 20 | 0 | tie |
| Blank Lines | Lower | **33 ✅** | 37 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **8 ✅** | 12 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **11 ✅** | 16 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00870

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **56.8888 ✅** | 53.1554 | 3.7334 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **30.7548 ✅** | 22.5996 | 8.1552 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **17 ✅** | 19 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **27 ✅** | 32 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2209.7674 ✅** | 2626.5162 | -416.7489 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **96356.1352 ✅** | 117008.7237 | -20652.5885 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **76 ✅** | 88 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **58 ✅** | 69 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **45 ✅** | 51 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **11 ✅** | 13 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00874

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **69.8558 ✅** | 68.8323 | 1.0235 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **48.0515 ✅** | 45.9353 | 2.1162 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 8 | 8 | 0 | tie |
| Halstead Volume | Lower | 1242.6959 | 1242.6959 | 0 | tie |
| Halstead Effort | Lower | 31440.2052 | 31440.2052 | 0 | tie |
| SLOC | Lower | **46 ✅** | 49 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 35 | 35 | 0 | tie |
| LLOC | Lower | 29 | 29 | 0 | tie |
| Comments (CLOC) | Higher | 4 | 4 | 0 | tie |
| Blank Lines | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00889

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **40.5993 ✅** | 40.1946 | 0.4047 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **10.7156 ✅** | 7.4379 | 3.2776 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 31 | **28 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **43 ✅** | 44 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 4555.4562 | **4405.3399 ✅** | 150.1163 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 390166.3916 | **285068.1247 ✅** | 105098.2669 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **135 ✅** | 146 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **108 ✅** | 119 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **100 ✅** | 107 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **15 ✅** | 13 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **16 ✅** | 19 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 6 | **2 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00897

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 33.1711 | **45.3504 ✅** | -12.1793 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -2.9871 | **14.0051 ✅** | -16.9922 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 35 | **19 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 39 | **25 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 5868.0158 | **4151.3452 ✅** | 1716.6706 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 348727.7967 | **189598.7499 ✅** | 159129.0468 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 186 | **123 ✅** | 63 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 154 | **97 ✅** | 57 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 95 | **71 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **15 ✅** | 11 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 20 | **17 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | **2 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 7 | **2 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |

## p00907

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 58.0497 | **60.0051 ✅** | -1.9554 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **36.1207 ✅** | 35.4774 | 0.6434 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 20 | **19 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 28 | 28 | 0 | tie |
| Halstead Volume | Lower | 1831.9589 | **1807.9765 ✅** | 23.9825 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 85186.0905 | **80283.9278 ✅** | 4902.1627 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 72 | **65 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 57 | **53 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 42 | **39 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **9 ✅** | 6 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00924

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.5681 | **53.9377 ✅** | -1.3696 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 23.0279 | **28.4976 ✅** | -5.4697 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **21 ✅** | 23 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3212.5189 | **2943.0542 ✅** | 269.4647 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **162162.3694 ✅** | 164742.5935 | -2580.2241 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 88 | **82 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 70 | **64 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 58 | **57 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | **9 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 12 | **9 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 5 | 5 | 0 | tie |

## p00949

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 41.4781 | **44.7902 ✅** | -3.3121 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 8.9841 | **10.7824 ✅** | -1.7983 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 25 | 25 | 0 | tie |
| Cognitive (sum) | Lower | 26 | 26 | 0 | tie |
| Halstead Volume | Lower | 3930.1955 | **3637.2286 ✅** | 292.9669 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 269000.0498 | **221253.3028 ✅** | 47746.7469 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 146 | **122 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 112 | **96 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 98 | **77 ✅** | 21 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 8 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 22 | **18 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **5 ✅** | 8 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00971

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **42.3871 ✅** | 38.1760 | 4.2111 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **17.7000 ✅** | 5.3465 | 12.3535 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 22 | **21 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 42 | **28 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **4391.3172 ✅** | 5033.8409 | -642.5237 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **344852.8265 ✅** | 416194.1667 | -71341.3402 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **139 ✅** | 175 | -36 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **102 ✅** | 142 | -40 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **96 ✅** | 106 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **24 ✅** | 18 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 15 | 15 | 0 | tie |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01050

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **74.4892 ✅** | 73.6513 | 0.8379 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 62.3417 | **65.2435 ✅** | -2.9018 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 7 | 7 | 0 | tie |
| Cognitive (sum) | Lower | 12 | 12 | 0 | tie |
| Halstead Volume | Lower | **797.0770 ✅** | 807.3024 | -10.2254 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 20013.5642 | **18719.3245 ✅** | 1294.2397 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **41 ✅** | 43 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 30 | **29 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 29 | **27 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | **10 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01065

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **57.4138 ✅** | 56.7351 | 0.6786 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **35.8827 ✅** | 34.4217 | 1.4610 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 16 | 16 | 0 | tie |
| Cognitive (sum) | Lower | **36 ✅** | 43 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2367.0028 ✅** | 2378.9574 | -11.9546 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 111342.5683 | **104674.1260 ✅** | 6668.4422 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **73 ✅** | 76 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **52 ✅** | 56 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **40 ✅** | 43 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | 10 | 0 | tie |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01106

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 57.5268 | **58.5003 ✅** | -0.9735 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 32.2616 | **40.4470 ✅** | -8.1854 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 20 | **18 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 16 | **14 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1860 ✅** | 2003.5960 | -143.5960 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **63054 ✅** | 72773.4698 | -9719.4698 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 74 | **70 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 59 | **53 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 39 | **38 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | **12 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01119

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 27.4188 | **31.0116 ✅** | -3.5928 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -10.2685 | **-1.2355 ✅** | -9.0330 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 50 | **42 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **63 ✅** | 70 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 5264.8929 | **5043.5408 ✅** | 221.3520 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 323366.3228 | **320834.2758 ✅** | 2532.0471 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 222 | **202 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 190 | **159 ✅** | 31 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 130 | **126 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 17 | **24 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **17 ✅** | 19 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 10 | **6 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 14 | **12 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p01125

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **58.3753 ✅** | 56.4215 | 1.9539 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 31.2904 | **31.9130 ✅** | -0.6225 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **14 ✅** | 15 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 14 | 14 | 0 | tie |
| Halstead Volume | Lower | **2060.1653 ✅** | 2165.6437 | -105.4784 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **70162.6745 ✅** | 74065.0137 | -3902.3392 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **74 ✅** | 81 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **56 ✅** | 60 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **44 ✅** | 50 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **9 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **14 ✅** | 15 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01128

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.6512 | **48.8493 ✅** | -2.1981 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 18.7213 | **20.1238 ✅** | -1.4025 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **30 ✅** | 37 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3749.2248 | **3631.2422 ✅** | 117.9826 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 173417.0137 | **165013.1712 ✅** | 8403.8425 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 114 | **102 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 81 | **78 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **52 ✅** | 53 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **13 ✅** | 10 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 21 | **15 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01138

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.1324 | **65.1531 ✅** | -0.0207 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 39.2319 | **41.4584 ✅** | -2.2265 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | 10 | 0 | tie |
| Cognitive (sum) | Lower | 8 | 8 | 0 | tie |
| Halstead Volume | Lower | 1512 | **1506 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 33079.6098 | **32948.3415 ✅** | 131.2683 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 57 | 57 | 0 | tie |
| PLOC | Lower | 43 | 43 | 0 | tie |
| LLOC | Lower | 31 | 31 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01150

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 27.9663 | **34.2719 ✅** | -6.3056 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -4.8979 | **4.9629 ✅** | -9.8607 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 42 | **41 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 43 | 43 | 0 | tie |
| Halstead Volume | Lower | 4965.4678 | **4908.8991 ✅** | 56.5686 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 493443.3579 | **438462.8706 ✅** | 54980.4874 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 245 | **169 ✅** | 76 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 193 | **122 ✅** | 71 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **82 ✅** | 84 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **31 ✅** | 23 | 8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 33 | **24 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01153

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **63.2677 ✅** | 60.6993 | 2.5684 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **38.7933 ✅** | 38.6309 | 0.1624 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 14 | 14 | 0 | tie |
| Cognitive (sum) | Lower | 33 | **32 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1628.5187 ✅** | 1714.8085 | -86.2898 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **70128.0872 ✅** | 76258.5444 | -6130.4571 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **59 ✅** | 68 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **45 ✅** | 49 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **35 ✅** | 40 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | **8 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01184

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 38.1140 | **38.4773 ✅** | -0.3632 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 5.1286 | **8.0378 ✅** | -2.9092 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 34 | **32 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **45 ✅** | 47 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 4100.6691 | **4015.1289 ✅** | 85.5402 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 257111.9530 | **249629.1190 ✅** | 7482.8340 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **156 ✅** | 158 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 126 | **124 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **95 ✅** | 97 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 14 | **18 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 17 | **16 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p01255

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | -0.8446 | **0.2065 ✅** | -1.0512 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -54.9102 | **-50.8028 ✅** | -4.1074 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 102 | **98 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 179 | **168 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 14784.2087 | **14415.8926 ✅** | 368.3161 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 1555575.9575 | **1458482.8814 ✅** | 97093.0760 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 436 | 436 | 0 | tie |
| PLOC | Lower | 384 | **381 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 219 | **211 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 10 | **16 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 43 | **42 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 17 | 17 | 0 | tie |
| Closures | Lower | 3 | 3 | 0 | tie |
| Exits | Lower | 20 | 20 | 0 | tie |

## p01275

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 56.7444 | **57.0489 ✅** | -0.3045 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 26.9211 | **27.4798 ✅** | -0.5587 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | 15 | 0 | tie |
| Cognitive (sum) | Lower | 12 | 12 | 0 | tie |
| Halstead Volume | Lower | 2383.0825 | **2340.9639 ✅** | 42.1185 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 106805.4228 | **99568.9987 ✅** | 7236.4241 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 77 | **76 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 62 | **61 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 41 | **40 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | 5 | 0 | tie |
| Blank Lines | Lower | 12 | 12 | 0 | tie |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p01288

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 58.3081 | **60.1955 ✅** | -1.8874 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 38.1886 | **40.9990 ✅** | -2.8105 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | 14 | 14 | 0 | tie |
| Halstead Volume | Lower | 1935.1138 | **1804.2581 ✅** | 130.8557 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 67830.8311 | **61654.0784 ✅** | 6176.7527 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 78 | **71 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 55 | **53 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 50 | **48 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 11 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **8 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01316

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 53.7692 | **59.1052 ✅** | -5.3361 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 25.1795 | **35.9422 ✅** | -10.7627 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 20 | **15 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 25 | 25 | 0 | tie |
| Halstead Volume | Lower | 2314.0453 | **1948.6193 ✅** | 365.4259 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 88234.5454 | **64213.8035 ✅** | 24020.7419 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 87 | **71 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 68 | **56 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 47 | 47 | 0 | tie |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 14 | **10 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01317

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 43.3651 | **45.3302 ✅** | -1.9651 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **18.3724 ✅** | 12.7842 | 5.5883 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **22 ✅** | 23 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **44 ✅** | 48 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3984.9450 | **3973.6819 ✅** | 11.2631 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 198305.3542 | **185101.3277 ✅** | 13204.0265 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 135 | **118 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 97 | 97 | 0 | tie |
| LLOC | Lower | 81 | **79 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **22 ✅** | 9 | 13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 17 | **13 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01339

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **44.3403 ✅** | 43.0927 | 1.2476 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **15.6818 ✅** | 11.1311 | 4.5507 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **25 ✅** | 26 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **40 ✅** | 48 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3770.0283 ✅** | 4255.8356 | -485.8072 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **246345.4792 ✅** | 296844.5318 | -50499.0526 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **124 ✅** | 127 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **97 ✅** | 101 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **76 ✅** | 87 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **14 ✅** | 11 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **14 ✅** | 15 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01353

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 49.9023 | **54.4071 ✅** | -4.5048 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 23.3499 | **32.2610 ✅** | -8.9111 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **14 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22 | **16 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3142.8944 | **2575.6493 ✅** | 567.2452 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 182411.1283 | **117548.2485 ✅** | 64862.8797 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 103 | **88 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 77 | **66 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 60 | **48 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 12 | **13 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 15 | **13 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p01368

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **39.9488 ✅** | 39.4986 | 0.4502 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 5.7868 | **7.8128 ✅** | -2.0260 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **34 ✅** | 35 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 55 | 55 | 0 | tie |
| Halstead Volume | Lower | **4418.2512 ✅** | 4505.3460 | -87.0948 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **363255.2778 ✅** | 401018.3016 | -37763.0238 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **136 ✅** | 137 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 113 | **112 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 84 | **83 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 10 | **13 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 14 | 14 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p01452

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **55.4852 ✅** | 52.3397 | 3.1455 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **24.6685 ✅** | 23.7201 | 0.9485 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **13 ✅** | 16 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3062.1476 ✅** | 3271.3626 | -209.2151 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **164815.5888 ✅** | 177880.3415 | -13064.7527 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **79 ✅** | 90 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **62 ✅** | 68 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **50 ✅** | 52 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | **8 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **12 ✅** | 14 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01459

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **32.3298 ✅** | 31.6685 | 0.6613 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **1.6962 ✅** | -4.4732 | 6.1694 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **35 ✅** | 36 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **61 ✅** | 69 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5860.0187 ✅** | 6072.4846 | -212.4658 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **372419.6105 ✅** | 435917.6414 | -63498.0309 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **196 ✅** | 199 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **149 ✅** | 161 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **80 ✅** | 103 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **27 ✅** | 17 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 22 | 22 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01471

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 55.6841 | **56.8889 ✅** | -1.2048 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **36.1523 ✅** | 34.6704 | 1.4820 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 18 | 18 | 0 | tie |
| Halstead Volume | Lower | 1997.4875 | **1991.3790 ✅** | 6.1085 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 75305.2787 | **70718.2386 ✅** | 4587.0400 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 87 | **82 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 62 | **61 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 44 | 44 | 0 | tie |
| Comments (CLOC) | Higher | **15 ✅** | 11 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | 13 | 0 | tie |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01521

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 76.8241 | **81.0221 ✅** | -4.1980 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 69.9696 | **71.3567 ✅** | -1.3871 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 6 | 6 | 0 | tie |
| Halstead Volume | Lower | 673.7652 | **627.6685 ✅** | 46.0966 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 12077.8647 | **10139.2610 ✅** | 1938.6037 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 38 | **30 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 22 | **19 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 11 | 11 | 0 | tie |
| Comments (CLOC) | Higher | **9 ✅** | 5 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01529

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **56.7780 ✅** | 56.0072 | 0.7708 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **30.3914 ✅** | 28.6198 | 1.7717 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 15 | 15 | 0 | tie |
| Cognitive (sum) | Lower | **26 ✅** | 28 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2466.1128 | **2345.2249 ✅** | 120.8879 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 154631.9378 | **119665.1015 ✅** | 34966.8363 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **76 ✅** | 81 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 62 | **61 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 56 | **52 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | 7 | 0 | tie |
| Blank Lines | Lower | **8 ✅** | 13 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p01545

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **55.0292 ✅** | 52.7576 | 2.2716 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **35.4058 ✅** | 32.8930 | 2.5128 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 20 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **17 ✅** | 26 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2545.9953 ✅** | 3135.6763 | -589.6810 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **156676.6343 ✅** | 221337.8491 | -64661.2148 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 85 | **84 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **60 ✅** | 62 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 58 | **49 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 15 | 15 | 0 | tie |
| Blank Lines | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **1 ✅** | 4 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01570

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **55.8499 ✅** | 52.3319 | 3.5180 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **30.5102 ✅** | 26.4793 | 4.0310 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 15 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **20 ✅** | 23 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2558.2948 ✅** | 2893.6053 | -335.3105 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **109105.4590 ✅** | 139144.6707 | -30039.2117 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **83 ✅** | 95 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **59 ✅** | 72 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **53 ✅** | 62 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **11 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 16 | 16 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01600

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **61.5343 ✅** | 58.3251 | 3.2092 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **42.0304 ✅** | 32.0837 | 9.9468 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **13 ✅** | 17 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **20 ✅** | 24 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1936.5195 ✅** | 2166 | -229.4805 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **88727.8018 ✅** | 117084.3333 | -28356.5315 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **63 ✅** | 70 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **47 ✅** | 57 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **39 ✅** | 44 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **9 ✅** | 6 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | 7 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01601

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.5981 | **76.0301 ✅** | -0.4320 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 51.5774 | **52.4242 ✅** | -0.8467 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | 780.6987 | 780.6987 | 0 | tie |
| Halstead Effort | Lower | 25680.8790 | 25680.8790 | 0 | tie |
| SLOC | Lower | 38 | **37 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 30 | 30 | 0 | tie |
| LLOC | Lower | 22 | 22 | 0 | tie |
| Comments (CLOC) | Higher | 2 | 2 | 0 | tie |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01656

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 77.1426 | **82.4925 ✅** | -5.3498 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 69.3543 | **76.5794 ✅** | -7.2251 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 819.3660 | **640.7628 ✅** | 178.6032 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 14594.9565 | **8543.5035 ✅** | 6051.4530 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 36 | **28 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 20 | **18 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 18 | **13 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **8 ✅** | 6 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **5 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01658

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **63.2886 ✅** | 63.2731 | 0.0155 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.4596 ✅** | 43.6113 | 0.8483 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 14 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **27 ✅** | 28 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1444.4768 ✅** | 1466.3476 | -21.8709 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **65334.7951 ✅** | 67000.8074 | -1666.0123 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 63 | **61 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 47 | **44 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 43 | **41 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **9 ✅** | 8 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01668

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **33.7634 ✅** | 33.1152 | 0.6481 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **1.0163 ✅** | -0.6377 | 1.6539 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 44 | **40 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 46 | **34 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **5621.6944 ✅** | 5748.9773 | -127.2829 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 461670.8428 | **428865.6078 ✅** | 32805.2350 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **160 ✅** | 175 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **124 ✅** | 138 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **66 ✅** | 77 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 16 | **17 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 25 | 25 | 0 | tie |
| Functions | Lower | 6 | 6 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 10 | 10 | 0 | tie |

## p01725

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **39.6949 ✅** | 39.1181 | 0.5767 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 2.8808 | **5.1441 ✅** | -2.2633 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 28 | 28 | 0 | tie |
| Cognitive (sum) | Lower | **47 ✅** | 51 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 4366.7302 | **4321.1756 ✅** | 45.5546 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 319733.4658 | **312882.4125 ✅** | 6851.0532 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **151 ✅** | 157 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **128 ✅** | 131 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **100 ✅** | 101 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **13 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 15 | 15 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p01767

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 62.4119 | 62.4119 | 0 | tie |
| MI (SEI) | Higher | 36.7549 | 36.7549 | 0 | tie |
| Cyclomatic (sum) | Lower | 11 | 11 | 0 | tie |
| Cognitive (sum) | Lower | 12 | 12 | 0 | tie |
| Halstead Volume | Lower | 1878.3912 | 1878.3912 | 0 | tie |
| Halstead Effort | Lower | 70439.6704 | 70439.6704 | 0 | tie |
| SLOC | Lower | 62 | 62 | 0 | tie |
| PLOC | Lower | 48 | 48 | 0 | tie |
| LLOC | Lower | 35 | 35 | 0 | tie |
| Comments (CLOC) | Higher | 5 | 5 | 0 | tie |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01768

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.4689 | **55.8292 ✅** | -3.3603 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 33.2243 | **33.5539 ✅** | -0.3296 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 14 | **12 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 23 | **15 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2434.1057 | **2384.8577 ✅** | 49.2481 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **82404.6207 ✅** | 87948.8969 | -5544.2762 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 101 | **85 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 64 | **61 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 46 | **45 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **20 ✅** | 12 | 8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 18 | **14 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01782

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 21.4147 | **28.5226 ✅** | -7.1079 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -18.9250 | **-13.7797 ✅** | -5.1453 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 46 | **38 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 52 | **47 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 8861.6522 | **7239.5587 ✅** | 1622.0935 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 649577.5668 | **502408.9169 ✅** | 147168.6499 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 288 | **222 ✅** | 66 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 235 | **190 ✅** | 45 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 139 | **127 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **23 ✅** | 11 | 12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 33 | **24 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 13 | **10 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **7 ✅** | 9 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01841

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **42.4027 ✅** | 37.7135 | 4.6893 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **14.0918 ✅** | 4.8041 | 9.2877 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **28 ✅** | 32 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 32 | **29 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **4038.6463 ✅** | 4471.6875 | -433.0412 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **185888.3785 ✅** | 230412.7634 | -44524.3849 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **131 ✅** | 160 | -29 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **100 ✅** | 125 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **65 ✅** | 75 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **16 ✅** | 15 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **15 ✅** | 21 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01847

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.5778 | **53.8393 ✅** | -7.2615 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 8.1266 | **24.6484 ✅** | -16.5218 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 24 | **16 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 24 | **18 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3154.4023 | **2538.6455 ✅** | 615.7568 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 165807.4661 | **109904.7760 ✅** | 55902.6901 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 116 | **89 ✅** | 27 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 95 | **70 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 72 | **54 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | **7 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 17 | **14 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p01907

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **38.3063 ✅** | 34.3834 | 3.9230 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **-0.6493 ✅** | -1.1369 | 0.4876 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **32 ✅** | 40 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **32 ✅** | 55 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5086.7119 ✅** | 5115.7693 | -29.0574 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **308315.4762 ✅** | 315372.1289 | -7056.6527 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **148 ✅** | 168 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **130 ✅** | 138 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **106 ✅** | 107 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **13 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **12 ✅** | 17 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 6 | 6 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 4 | 4 | 0 | tie |

## p01916

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 88.0004 | **89.0496 ✅** | -1.0492 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 79.2967 | **88.1492 ✅** | -8.8525 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 1 | 1 | 0 | tie |
| Halstead Volume | Lower | 544.3964 | **517.9694 ✅** | 26.4270 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 7151.3894 | **6403.9855 ✅** | 747.4039 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 14 | **13 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 3 | **5 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01917

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 50.7920 | **51.5147 ✅** | -0.7227 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 21.4393 | **21.5811 ✅** | -0.1419 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | 22 | 0 | tie |
| Cognitive (sum) | Lower | 51 | 51 | 0 | tie |
| Halstead Volume | Lower | 2763.2067 | **2745.6166 ✅** | 17.5901 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **140442.9864 ✅** | 154616.5270 | -14173.5405 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 96 | **92 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 76 | 76 | 0 | tie |
| LLOC | Lower | 60 | 60 | 0 | tie |
| Comments (CLOC) | Higher | **8 ✅** | 7 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02035

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 49.0370 | **52.5220 ✅** | -3.4850 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 13.8535 | **21.5951 ✅** | -7.7416 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **16 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 18 | **15 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4069.9632 | **3639.2479 ✅** | 430.7154 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 309943.3548 | **290897.2143 ✅** | 19046.1405 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 100 | **86 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 83 | **71 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 71 | **61 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | **6 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 12 | **9 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02038

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 68.6968 | **70.1885 ✅** | -1.4917 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 57.4968 | **57.8627 ✅** | -0.3660 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | 13 | 0 | tie |
| Cognitive (sum) | Lower | 9 | 9 | 0 | tie |
| Halstead Volume | Lower | 1216.8195 | **1205.0057 ✅** | 11.8138 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 31637.3060 | **30928.4792 ✅** | 708.8269 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 47 | **43 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 31 | 31 | 0 | tie |
| LLOC | Lower | 17 | 17 | 0 | tie |
| Comments (CLOC) | Higher | **10 ✅** | 8 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02045

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.8154 | **46.8941 ✅** | -0.0786 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **18.8352 ✅** | 17.4543 | 1.3808 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **33 ✅** | 36 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 22 | 22 | 0 | tie |
| Halstead Volume | Lower | **3315.8481 ✅** | 3361.3850 | -45.5369 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **201834.2320 ✅** | 218124.6572 | -16290.4252 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 99 | **94 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 79 | 79 | 0 | tie |
| LLOC | Lower | 49 | **44 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 8 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **9 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 6 | 6 | 0 | tie |
| Exits | Lower | **1 ✅** | 4 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02046

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.8783 ✅** | 59.2981 | 9.5802 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **51.6333 ✅** | 38.6265 | 13.0068 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 22 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **8 ✅** | 14 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1202.1414 ✅** | 1505.9566 | -303.8153 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **40502.9169 ✅** | 41189.9487 | -687.0318 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **46 ✅** | 69 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **33 ✅** | 53 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 23 | **20 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **9 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 11 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02056

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 43.1719 | **44.0295 ✅** | -0.8576 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 12.8271 | **15.0726 ✅** | -2.2455 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | **21 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 20 | **17 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4232.6882 | **4122.8912 ✅** | 109.7971 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 203578.6495 | **178965.1907 ✅** | 24613.4588 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 134 | **130 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 101 | **99 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 65 | **64 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 14 | **15 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 19 | **18 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02061

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **45.9794 ✅** | 45.9343 | 0.0450 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 18.3605 | **18.4446 ✅** | -0.0841 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 25 | **24 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 39 | **36 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3777.1424 | **3384.7422 ✅** | 392.4002 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 228643.0183 | **223520.7092 ✅** | 5122.3091 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **112 ✅** | 118 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **89 ✅** | 93 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **69 ✅** | 75 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 13 | **14 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02104

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **61.2544 ✅** | 60.4591 | 0.7953 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.3754 ✅** | 43.7010 | 0.6744 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | 18 | 18 | 0 | tie |
| Halstead Volume | Lower | **1763.2523 ✅** | 1792.5422 | -29.2899 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **72811.9471 ✅** | 75286.7717 | -2474.8247 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **67 ✅** | 70 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **46 ✅** | 47 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **40 ✅** | 41 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 12 | **13 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02114

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.4872 | **54.7356 ✅** | -7.2485 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 15.8047 | **30.8690 ✅** | -15.0643 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 24 | **23 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 43 | **40 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3405.9203 | **2461.6834 ✅** | 944.2369 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 201463.4001 | **122013.8749 ✅** | 79449.5252 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 107 | **77 ✅** | 30 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 86 | **60 ✅** | 26 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 69 | **52 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 8 | **9 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 13 | **8 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | **1 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p02153

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 42.1236 | **42.4812 ✅** | -0.3576 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 11.6164 | **13.1250 ✅** | -1.5086 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 28 | 28 | 0 | tie |
| Cognitive (sum) | Lower | 44 | 44 | 0 | tie |
| Halstead Volume | Lower | 4580.3395 | **4490.9444 ✅** | 89.3951 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 260464.0827 | **243952.1229 ✅** | 16511.9598 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 128 | **126 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 102 | **101 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 82 | **80 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 13 | **14 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02162

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.6924 | **66.6948 ✅** | -1.0024 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **39.7330 ✅** | 22.6576 | 17.0754 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 21 | 21 | 0 | tie |
| Cognitive (sum) | Lower | 16 | 16 | 0 | tie |
| Halstead Volume | Lower | 1255.3371 | 1255.3371 | 0 | tie |
| Halstead Effort | Lower | 35351.1889 | 35351.1889 | 0 | tie |
| SLOC | Lower | 50 | **47 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 43 | 43 | 0 | tie |
| LLOC | Lower | 12 | 12 | 0 | tie |
| Comments (CLOC) | Higher | **3 ✅** | 0 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 4 | 4 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02166

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **61.0023 ✅** | 59.9582 | 1.0441 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.8658 ✅** | 34.5333 | 10.3325 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 15 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **19 ✅** | 23 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1767.3597 ✅** | 1891.8954 | -124.5356 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **71578.0689 ✅** | 79331.7743 | -7753.7054 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 68 | 68 | 0 | tie |
| PLOC | Lower | **50 ✅** | 56 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **39 ✅** | 41 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **13 ✅** | 6 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02186

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 30.0499 | **33.3048 ✅** | -3.2549 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -3.5643 | **-1.0580 ✅** | -2.5063 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 29 | 29 | 0 | tie |
| Cognitive (sum) | Lower | 14 | 14 | 0 | tie |
| Halstead Volume | Lower | 7395.2696 | **7212.8918 ✅** | 182.3778 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 635222.8436 | **625134.2205 ✅** | 10088.6230 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 228 | **188 ✅** | 40 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 178 | **150 ✅** | 28 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 88 | **83 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **28 ✅** | 19 | 9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 29 | **25 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 12 | 12 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 12 | 12 | 0 | tie |

## p02190

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **78.8663 ✅** | 78.3826 | 0.4836 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 64.2676 | **65.8782 ✅** | -1.6106 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 4 | 4 | 0 | tie |
| Halstead Volume | Lower | 737.9720 | 737.9720 | 0 | tie |
| Halstead Effort | Lower | 13796.8680 | 13796.8680 | 0 | tie |
| SLOC | Lower | **33 ✅** | 34 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 22 | 22 | 0 | tie |
| LLOC | Lower | 18 | 18 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 7 | 7 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02205

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **80.3229 ✅** | 80.1701 | 0.1528 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 66.9982 | **75.3388 ✅** | -8.3406 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 708.2439 | **660.6753 ✅** | 47.5686 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 15283.1578 | **11925.1888 ✅** | 3357.9690 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **31 ✅** | 32 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 21 | **18 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **14 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | **8 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02242

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **44.6951 ✅** | 41.8807 | 2.8145 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **10.5989 ✅** | 1.1488 | 9.4501 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **26 ✅** | 31 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **24 ✅** | 28 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3368.9756 ✅** | 3730.0225 | -361.0469 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 230391.9897 | **201356.9019 ✅** | 29035.0878 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **124 ✅** | 133 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **105 ✅** | 114 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **64 ✅** | 69 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **8 ✅** | 4 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **14 ✅** | 16 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 3 | 3 | 0 | tie |

## p02249

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 40.5838 | **40.7779 ✅** | -0.1941 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **7.4028 ✅** | 6.8200 | 0.5828 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 31 | 31 | 0 | tie |
| Cognitive (sum) | Lower | 40 | **39 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4569.0742 | **4504.8116 ✅** | 64.2626 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 287851.6772 | **279453.6571 ✅** | 8398.0201 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 135 | **134 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **111 ✅** | 112 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 79 | 79 | 0 | tie |
| Comments (CLOC) | Higher | **11 ✅** | 10 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | 6 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p02256

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.7813 | **70.0425 ✅** | -4.2612 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 47.1938 | **52.5064 ✅** | -5.3126 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | **11 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1266.0753 | **961.0998 ✅** | 304.9755 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 38043.5192 | **23288.1870 ✅** | 14755.3322 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 54 | **48 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 39 | **35 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02266

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.3466 | **62.7873 ✅** | -3.4407 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 44.2620 | **52.2185 ✅** | -7.9565 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 14 | **12 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 32 | **18 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1394.1995 | **1145.5841 ✅** | 248.6154 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 35731.3409 | **26015.8452 ✅** | 9715.4957 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 79 | **70 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 54 | **42 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 29 | **25 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 17 | **19 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02276

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 64.4935 | **67.1469 ✅** | -2.6534 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 43.5000 | **47.2021 ✅** | -3.7021 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | 1523.0751 | **1427.9958 ✅** | 95.0792 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **48917.5872 ✅** | 50574.8530 | -1657.2658 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 60 | **52 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 43 | **40 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 31 | **29 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **7 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02277

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **55.9031 ✅** | 55.4445 | 0.4586 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **26.8226 ✅** | 26.1470 | 0.6756 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **15 ✅** | 16 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2302.9142 ✅** | 2317.2461 | -14.3319 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **91976.1448 ✅** | 96250.4892 | -4274.3444 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **82 ✅** | 83 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **66 ✅** | 67 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 43 | 43 | 0 | tie |
| Comments (CLOC) | Higher | 6 | 6 | 0 | tie |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02279

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **50.5531 ✅** | 49.8340 | 0.7191 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **23.7591 ✅** | 19.5481 | 4.2110 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **19 ✅** | 25 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2770.4359 ✅** | 2871.4854 | -101.0495 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **112021.9743 ✅** | 115577.2880 | -3555.3137 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **106 ✅** | 108 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **82 ✅** | 85 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **53 ✅** | 55 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **12 ✅** | 9 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | 14 | 0 | tie |
| Functions | Lower | 3 | **2 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02297

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 79.7569 | **79.9974 ✅** | -0.2405 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 66.9779 | **70.1089 ✅** | -3.1309 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | 972.0530 | **928.1232 ✅** | 43.9299 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **21241.1592 ✅** | 21903.7069 | -662.5477 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 29 | 29 | 0 | tie |
| PLOC | Lower | 20 | **19 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 17 | 17 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02305

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **72.3019 ✅** | 70.7349 | 1.5670 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **52.5826 ✅** | 46.8414 | 5.7412 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1213.8990 ✅** | 1259.7972 | -45.8982 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **25844.3018 ✅** | 38339.8274 | -12495.5257 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **41 ✅** | 44 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **31 ✅** | 33 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **18 ✅** | 22 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **4 ✅** | 3 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02317

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.4417 | 65.4417 | 0 | tie |
| MI (SEI) | Higher | 40.0166 | **42.2447 ✅** | -2.2280 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | 10 | 0 | tie |
| Cognitive (sum) | Lower | 11 | 11 | 0 | tie |
| Halstead Volume | Lower | 1592.3790 | 1592.3790 | 0 | tie |
| Halstead Effort | Lower | 64106.0955 | 64106.0955 | 0 | tie |
| SLOC | Lower | 55 | 55 | 0 | tie |
| PLOC | Lower | 44 | 44 | 0 | tie |
| LLOC | Lower | 36 | 36 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p02328

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **65.1639 ✅** | 61.5824 | 3.5815 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **41.2958 ✅** | 36.7788 | 4.5170 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1423.5996 ✅** | 1813.3182 | -389.7186 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **43743.3337 ✅** | 80843.7713 | -37100.4376 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **58 ✅** | 66 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **43 ✅** | 50 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **26 ✅** | 40 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | **6 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02333

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **57.5631 ✅** | 56.8791 | 0.6840 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **26.9685 ✅** | 20.6526 | 6.3159 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 15 | 15 | 0 | tie |
| Cognitive (sum) | Lower | 11 | 11 | 0 | tie |
| Halstead Volume | Lower | **2739.7895 ✅** | 2862.3752 | -122.5857 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **171579.3180 ✅** | 182476.4170 | -10897.0990 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **70 ✅** | 72 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **56 ✅** | 57 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **42 ✅** | 43 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **4 ✅** | 2 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **10 ✅** | 13 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | 5 | 0 | tie |

## p02340

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **73.2609 ✅** | 72.4249 | 0.8360 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **56.6404 ✅** | 53.1276 | 3.5128 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 6 | 6 | 0 | tie |
| Halstead Volume | Lower | **1139.4677 ✅** | 1224.9069 | -85.4392 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **31280.2269 ✅** | 40066.3097 | -8786.0828 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 40 | 40 | 0 | tie |
| PLOC | Lower | **28 ✅** | 29 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **19 ✅** | 21 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **5 ✅** | 4 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | 7 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02347

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **40.1735 ✅** | 39.6915 | 0.4820 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 1.0269 | **1.5347 ✅** | -0.5079 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **29 ✅** | 30 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 29 | 29 | 0 | tie |
| Halstead Volume | Lower | **4931.8128 ✅** | 4951.3835 | -19.5707 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **344569.3237 ✅** | 345936.6623 | -1367.3386 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **139 ✅** | 141 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **115 ✅** | 117 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **59 ✅** | 60 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **7 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 21 | 21 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 3 | 3 | 0 | tie |
| Exits | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02353

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 36.4978 | **40.7009 ✅** | -4.2031 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -1.9871 | **3.5615 ✅** | -5.5486 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 33 | **21 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 24 | **19 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4910.9083 | **4804.7365 ✅** | 106.1717 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 326575.3998 | **291123.3555 ✅** | 35452.0443 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 165 | **152 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 136 | **123 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **65 ✅** | 71 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | 9 | 0 | tie |
| Blank Lines | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 6 | 6 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 10 | 10 | 0 | tie |

## p02355

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **52.0059 ✅** | 51.0001 | 1.0059 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **25.1635 ✅** | 23.8471 | 1.3164 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 25 | **24 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **33 ✅** | 36 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2187.6845 | **2139.8662 ✅** | 47.8182 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 145173.3729 | **129364.6392 ✅** | 15808.7337 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **92 ✅** | 100 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **72 ✅** | 76 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 62 | **60 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | **10 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **13 ✅** | 15 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02369

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **65.7760 ✅** | 58.0496 | 7.7264 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.7961 ✅** | 34.1981 | 10.5981 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **11 ✅** | 13 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 16 | 16 | 0 | tie |
| Halstead Volume | Lower | **1428.6093 ✅** | 1798.1673 | -369.5580 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **41251.0939 ✅** | 55648.5455 | -14397.4516 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **55 ✅** | 80 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **42 ✅** | 61 | -19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **32 ✅** | 34 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **9 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 12 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 4 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02370

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 67.9770 | **69.6000 ✅** | -1.6229 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 44.3984 | **49.9670 ✅** | -5.5686 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | 1437.7080 | **1364.4009 ✅** | 73.3071 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 40812.3571 | **34110.0223 ✅** | 6702.3348 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 50 | **46 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 37 | **34 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 31 | **26 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 9 | **7 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02371

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **57.5706 ✅** | 56.3462 | 1.2244 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **39.1547 ✅** | 27.5380 | 11.6167 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 11 | 11 | 0 | tie |
| Cognitive (sum) | Lower | 17 | **10 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **2426.4357 ✅** | 2949.6643 | -523.2287 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **92435.6448 ✅** | 129373.6495 | -36938.0047 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **77 ✅** | 78 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **53 ✅** | 64 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **44 ✅** | 50 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **14 ✅** | 6 | 8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **9 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **2 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02393

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 89.9348 | **90.8107 ✅** | -0.8759 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **83.1232 ✅** | 80.2025 | 2.9207 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | 560 | 560 | 0 | tie |
| Halstead Effort | Lower | 6130.5263 | 6130.5263 | 0 | tie |
| SLOC | Lower | 19 | **18 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 12 | 12 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | **3 ✅** | 2 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02396

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **88.4883 ✅** | 85.8755 | 2.6128 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 74.8829 | **77.8409 ✅** | -2.9579 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **1 ✅** | 5 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **428.7735 ✅** | 435.5780 | -6.8045 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **5447.9456 ✅** | 5880.3028 | -432.3571 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **22 ✅** | 25 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 2 | **4 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02412

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **71.4423 ✅** | 71.0699 | 0.3724 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **51.5498 ✅** | 47.9357 | 3.6141 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 14 | 14 | 0 | tie |
| Cognitive (sum) | Lower | 19 | 19 | 0 | tie |
| Halstead Volume | Lower | 905.8642 | 905.8642 | 0 | tie |
| Halstead Effort | Lower | 23250.5135 | 23250.5135 | 0 | tie |
| SLOC | Lower | **43 ✅** | 44 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 33 | 33 | 0 | tie |
| LLOC | Lower | 23 | 23 | 0 | tie |
| Comments (CLOC) | Higher | **4 ✅** | 3 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02418

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **89.8663 ✅** | 89.1126 | 0.7536 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 81.8867 | **83.8457 ✅** | -1.9590 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | 397.4581 | 397.4581 | 0 | tie |
| Halstead Effort | Lower | 3444.6372 | 3444.6372 | 0 | tie |
| SLOC | Lower | **21 ✅** | 22 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 14 | 14 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | 3 | **4 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02430

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 70.3280 | **77.2405 ✅** | -6.9125 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 59.5446 | **59.9264 ✅** | -0.3818 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | 10 | 0 | tie |
| Cognitive (sum) | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1015.3641 | **982.5742 ✅** | 32.7899 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 23770.3997 | **23217.8650 ✅** | 552.5347 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 47 | **31 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 31 | **23 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 22 | **16 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 3 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p02433

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **37.8866 ✅** | 28.4072 | 9.4794 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **4.8915 ✅** | -8.4433 | 13.3348 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **30 ✅** | 41 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **20 ✅** | 36 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3912.0983 ✅** | 5884.6708 | -1972.5725 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **193648.8640 ✅** | 346648.1652 | -152999.3012 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **170 ✅** | 229 | -59 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **131 ✅** | 180 | -49 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **77 ✅** | 122 | -45 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 16 | **20 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **25 ✅** | 31 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02444

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 67.7002 | **68.7026 ✅** | -1.0024 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 48.3477 | **50.5148 ✅** | -2.1671 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | 1516.3170 | 1516.3170 | 0 | tie |
| Halstead Effort | Lower | 47258.5454 | 47258.5454 | 0 | tie |
| SLOC | Lower | 50 | **47 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 35 | 35 | 0 | tie |
| LLOC | Lower | 28 | 28 | 0 | tie |
| Comments (CLOC) | Higher | 6 | 6 | 0 | tie |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02449

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.2913 | **47.3467 ✅** | -1.0554 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **21.0585 ✅** | 19.2379 | 1.8206 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 25 | 25 | 0 | tie |
| Cognitive (sum) | Lower | 22 | 22 | 0 | tie |
| Halstead Volume | Lower | 2796 | **2748 ✅** | 48 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 157104.9730 | **148392 ✅** | 8712.9730 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 121 | **114 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 88 | **85 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 57 | **54 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **17 ✅** | 12 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 18 | **17 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 8 | 8 | 0 | tie |

## p02450

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.9001 ✅** | 67.8629 | 0.0373 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **46.9207 ✅** | 46.8669 | 0.0537 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 13 | 13 | 0 | tie |
| Halstead Volume | Lower | **1486.6829 ✅** | 1497.3722 | -10.6893 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **51554.3250 ✅** | 54279.7410 | -2725.4160 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 49 | 49 | 0 | tie |
| PLOC | Lower | 35 | 35 | 0 | tie |
| LLOC | Lower | 24 | 24 | 0 | tie |
| Comments (CLOC) | Higher | 5 | 5 | 0 | tie |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02457

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 38.5717 | **49.0993 ✅** | -10.5276 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 7.5832 | **23.6267 ✅** | -16.0435 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 36 | **23 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 44 | **31 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4410.6990 | **2940.0024 ✅** | 1470.6966 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 321072.9435 | **144060.1187 ✅** | 177012.8248 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 144 | **103 ✅** | 41 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 117 | **82 ✅** | 35 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 65 | **50 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **15 ✅** | 13 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | **8 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | **3 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 11 | **4 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |

## p02542

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 13.1618 | **14.2478 ✅** | -1.0860 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **-26.4017 ✅** | -26.6593 | 0.2576 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 57 | **55 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 146 | **144 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 11704.1360 | **11682.5156 ✅** | 21.6204 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **1391542.2318 ✅** | 1464602.1825 | -73059.9506 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 375 | **361 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 295 | **290 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 237 | **224 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **41 ✅** | 34 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 44 | **43 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02562

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 17.9192 | **18.5659 ✅** | -0.6467 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **-18.6855 ✅** | -19.3500 | 0.6644 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 55 | **54 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **107 ✅** | 114 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 10264.4367 | **9983.3359 ✅** | 281.1008 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 1191577.0255 | **1121576.1458 ✅** | 70000.8797 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 300 | **295 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 233 | 233 | 0 | tie |
| LLOC | Lower | **180 ✅** | 186 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **36 ✅** | 31 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 32 | 32 | 0 | tie |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02584

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 77.1392 | **81.6739 ✅** | -4.5347 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 63.6272 | **67.3263 ✅** | -3.6991 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | 895.1247 | **803.6328 ✅** | 91.4919 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 18486.2708 | **17603.3852 ✅** | 882.8857 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 35 | **27 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 25 | **19 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **12 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **5 ✅** | 3 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02585

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **51.6169 ✅** | 49.5270 | 2.0899 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 28.0281 | **34.2123 ✅** | -6.1841 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 16 | 16 | 0 | tie |
| Cognitive (sum) | Lower | 26 | 26 | 0 | tie |
| Halstead Volume | Lower | 2883.2235 | **2842.8451 ✅** | 40.3784 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 216393.5144 | **207602.5074 ✅** | 8791.0069 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **98 ✅** | 112 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 68 | **67 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 71 | **70 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 14 | **32 ✅** | -18 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02606

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **91.5001 ✅** | 90.5720 | 0.9282 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 86.0659 | **87.7933 ✅** | -1.7273 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | **490.4654 ✅** | 495.4196 | -4.9542 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **5452.8216 ✅** | 5507.9006 | -55.0790 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **18 ✅** | 19 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 11 | 11 | 0 | tie |
| LLOC | Lower | 8 | 8 | 0 | tie |
| Comments (CLOC) | Higher | 3 | **4 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02617

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **81.5152 ✅** | 79.8383 | 1.6769 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 70.4885 | **74.0114 ✅** | -3.5230 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | 828.5343 | **823.7780 ✅** | 4.7563 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 19884.8230 | **18911.0767 ✅** | 973.7463 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **27 ✅** | 30 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 18 | 18 | 0 | tie |
| LLOC | Lower | 15 | 15 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **7 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02618

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 40.8183 | **42.5369 ✅** | -1.7186 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 11.0607 | **11.6688 ✅** | -0.6082 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **21 ✅** | 22 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **16 ✅** | 18 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4332.2657 ✅** | 4463.6750 | -131.4093 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **223565.8709 ✅** | 270158.6160 | -46592.7451 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 156 | **137 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 121 | **105 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **68 ✅** | 75 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **19 ✅** | 14 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 18 | 18 | 0 | tie |
| Functions | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02639

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 69.2230 | **81.1291 ✅** | -11.9061 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 45.6785 | **62.5191 ✅** | -16.8407 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **7 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 10 | **5 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1082.6181 | **816.8482 ✅** | 265.7700 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 27065.4536 | **11175.9679 ✅** | 15889.4857 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 44 | **27 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 37 | **22 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 14 | 14 | 0 | tie |
| Comments (CLOC) | Higher | **3 ✅** | 2 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 5 | **0 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |

## p02644

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **54.1894 ✅** | 51.7903 | 2.3991 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **29.3749 ✅** | 20.7540 | 8.6209 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 14 | 14 | 0 | tie |
| Cognitive (sum) | Lower | **26 ✅** | 28 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3104.6442 ✅** | 3248.7091 | -144.0649 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **122492.3258 ✅** | 161185.9502 | -38693.6244 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **84 ✅** | 96 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **59 ✅** | 72 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **47 ✅** | 67 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **10 ✅** | 7 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **15 ✅** | 17 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02647

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.1540 | **61.4010 ✅** | -2.2470 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 38.8170 | **44.1911 ✅** | -5.3741 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22 | **14 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1858.3536 | **1788.3235 ✅** | 70.0301 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 77065.3596 | **66832.2035 ✅** | 10233.1562 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 75 | **68 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 54 | **46 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 49 | **43 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | **12 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02668

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **65.3521 ✅** | 64.0605 | 1.2916 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **51.3737 ✅** | 44.7469 | 6.6268 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 8 | 8 | 0 | tie |
| Halstead Volume | Lower | **2047.3444 ✅** | 2217.7564 | -170.4120 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 101250.4864 | **95603.2839 ✅** | 5647.2025 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **54 ✅** | 57 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **36 ✅** | 40 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **34 ✅** | 38 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **11 ✅** | 8 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02678

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **69.0720 ✅** | 67.8500 | 1.2220 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **52.8058 ✅** | 48.4620 | 4.3438 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 7 | 7 | 0 | tie |
| Cognitive (sum) | Lower | 8 | 8 | 0 | tie |
| Halstead Volume | Lower | **1476.2112 ✅** | 1539.8830 | -63.6718 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **39773.3478 ✅** | 47596.3825 | -7823.0348 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **47 ✅** | 50 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **33 ✅** | 36 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **27 ✅** | 31 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02703

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **41.5970 ✅** | 36.0497 | 5.5473 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **19.3301 ✅** | 3.8726 | 15.4575 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **24 ✅** | 26 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **25 ✅** | 28 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4283.3434 ✅** | 4405.3968 | -122.0533 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **220300.1407 ✅** | 230329.9229 | -10029.7822 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **143 ✅** | 194 | -51 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **95 ✅** | 106 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **73 ✅** | 76 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **30 ✅** | 22 | 8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **21 ✅** | 70 | -49 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p02727

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 43.4085 | **43.7312 ✅** | -0.3227 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **15.0279 ✅** | 12.6912 | 2.3367 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **42 ✅** | 43 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **31 ✅** | 32 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3366 ✅** | 3408 | -42 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **254223.9730 ✅** | 262369.9459 | -8145.9730 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 107 | **103 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 87 | **86 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **50 ✅** | 51 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **11 ✅** | 8 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 11 | 11 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02728

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 28.3202 | **46.5754 ✅** | -18.2552 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 7.6724 | **17.5349 ✅** | -9.8625 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 40 | **22 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 60 | **28 ✅** | 32 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 5847.5228 | **3356.7736 ✅** | 2490.7492 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 549619.9837 | **203496.4802 ✅** | 346123.5035 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 234 | **117 ✅** | 117 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 137 | **88 ✅** | 49 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 111 | **71 ✅** | 40 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **75 ✅** | 12 | 63 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 23 | **17 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02731

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 95.0944 | **96.1400 ✅** | -1.0455 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **92.7765 ✅** | 90.0032 | 2.7733 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | 354.6326 | 354.6326 | 0 | tie |
| Halstead Effort | Lower | 3806.3900 | 3806.3900 | 0 | tie |
| SLOC | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 9 | 9 | 0 | tie |
| LLOC | Lower | 6 | 6 | 0 | tie |
| Comments (CLOC) | Higher | **3 ✅** | 2 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02733

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 48.9010 | **51.1582 ✅** | -2.2573 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 22.5853 | **25.0588 ✅** | -2.4734 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 52 | **51 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2878.7142 | **2724.0964 ✅** | 154.6178 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 134594.5769 | **114030.6751 ✅** | 20563.9018 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 108 | **97 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 80 | **74 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 72 | **65 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 11 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | **12 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02734

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 62.0996 | **63.5029 ✅** | -1.4032 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 42.2472 | **45.2715 ✅** | -3.0243 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 12 | 12 | 0 | tie |
| Halstead Volume | Lower | 1973.9264 | **1941.7650 ✅** | 32.1614 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **92281.0598 ✅** | 101975.0270 | -9693.9672 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 64 | **59 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 43 | **39 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 43 | **37 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | 9 | 0 | tie |
| Blank Lines | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02743

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **88.3326 ✅** | 87.5422 | 0.7904 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **75.6703 ✅** | 74.0074 | 1.6628 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 594.5414 | 594.5414 | 0 | tie |
| Halstead Effort | Lower | 9043.2872 | 9043.2872 | 0 | tie |
| SLOC | Lower | **20 ✅** | 21 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 15 | 15 | 0 | tie |
| LLOC | Lower | 10 | 10 | 0 | tie |
| Comments (CLOC) | Higher | 2 | 2 | 0 | tie |
| Blank Lines | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02761

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **56.3913 ✅** | 55.1040 | 1.2873 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **41.1128 ✅** | 36.9559 | 4.1569 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **19 ✅** | 22 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **27 ✅** | 28 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1897.0459 ✅** | 2047.1805 | -150.1347 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **75482.4564 ✅** | 85432.3386 | -9949.8822 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **80 ✅** | 81 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **53 ✅** | 56 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 40 | 40 | 0 | tie |
| Comments (CLOC) | Higher | **18 ✅** | 15 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | 12 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02762

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **48.6213 ✅** | 46.9064 | 1.7150 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **20.8617 ✅** | 20.7051 | 0.1566 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 19 | 19 | 0 | tie |
| Cognitive (sum) | Lower | 17 | 17 | 0 | tie |
| Halstead Volume | Lower | **3134.3219 ✅** | 3323.8974 | -189.5755 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **165549.1840 ✅** | 170626.7351 | -5077.5511 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **110 ✅** | 120 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **84 ✅** | 89 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **65 ✅** | 71 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 12 | **16 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **14 ✅** | 15 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p02783

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 88.3768 | **89.9209 ✅** | -1.5440 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **85.7662 ✅** | 82.5651 | 3.2011 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 1 | 1 | 0 | tie |
| Halstead Volume | Lower | 457.8717 | 457.8717 | 0 | tie |
| Halstead Effort | Lower | 6181.2674 | 6181.2674 | 0 | tie |
| SLOC | Lower | 22 | **20 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 13 | 13 | 0 | tie |
| LLOC | Lower | 10 | 10 | 0 | tie |
| Comments (CLOC) | Higher | **5 ✅** | 3 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02812

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 78.4679 | **80.1038 ✅** | -1.6359 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **68.7941 ✅** | 67.7839 | 1.0101 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **762.2515 ✅** | 796.3187 | -34.0671 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **15366.9912 ✅** | 16244.9015 | -877.9102 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 33 | **29 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 21 | 21 | 0 | tie |
| LLOC | Lower | 18 | 18 | 0 | tie |
| Comments (CLOC) | Higher | **6 ✅** | 4 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | **5 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02831

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 87.0774 | **88.4417 ✅** | -1.3643 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **85.8944 ✅** | 79.9332 | 5.9612 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | **489.6886 ✅** | 500.1075 | -10.4189 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **7494.9563 ✅** | 7918.3691 | -423.4128 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 23 | **21 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 15 | 15 | 0 | tie |
| LLOC | Lower | 10 | 10 | 0 | tie |
| Comments (CLOC) | Higher | **6 ✅** | 3 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 3 | 3 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02837

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 58.4721 | **58.8139 ✅** | -0.3418 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **38.5742 ✅** | 35.4663 | 3.1079 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 16 | 16 | 0 | tie |
| Cognitive (sum) | Lower | 26 | 26 | 0 | tie |
| Halstead Volume | Lower | 1931.1400 | **1887.6738 ✅** | 43.4662 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 74302.9103 | **72630.4976 ✅** | 1672.4128 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 73 | **72 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 56 | 56 | 0 | tie |
| LLOC | Lower | 41 | 41 | 0 | tie |
| Comments (CLOC) | Higher | **11 ✅** | 8 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02840

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 42.7989 | **48.0590 ✅** | -5.2601 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 14.8395 | **21.9140 ✅** | -7.0745 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 23 | **21 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 35 | **31 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3881.4639 | **3107.7639 ✅** | 773.7000 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 279166.8275 | **205457.7251 ✅** | 73709.1024 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 139 | **111 ✅** | 28 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 106 | **82 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 84 | **65 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **18 ✅** | 14 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 15 | 15 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02846

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **70.0153 ✅** | 69.7016 | 0.3137 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 48.3042 | **52.0064 ✅** | -3.7022 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 7 | 7 | 0 | tie |
| Halstead Volume | Lower | **1409.9474 ✅** | 1461.7588 | -51.8113 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 43412.7364 | **38980.2334 ✅** | 4432.5030 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **45 ✅** | 46 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 35 | **34 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 26 | **25 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | **6 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02862

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **59.7363 ✅** | 57.2855 | 2.4508 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **30.0780 ✅** | 23.9178 | 6.1602 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **11 ✅** | 15 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2467.8439 ✅** | 2535.8392 | -67.9953 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **118192.0944 ✅** | 130831.6096 | -12639.5151 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **67 ✅** | 73 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **53 ✅** | 57 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 46 | **44 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **4 ✅** | 3 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **10 ✅** | 13 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02884

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 53.1186 | **53.9643 ✅** | -0.8456 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 29.0744 | **31.1926 ✅** | -2.1182 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **33 ✅** | 35 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2432.7529 ✅** | 2434.9851 | -2.2322 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 142072.7710 | **126132.2301 ✅** | 15940.5410 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 93 | **87 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **64 ✅** | 65 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 61 | 61 | 0 | tie |
| Comments (CLOC) | Higher | 12 | 12 | 0 | tie |
| Blank Lines | Lower | 17 | **12 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02885

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 89.7849 | **91.4146 ✅** | -1.6297 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **89.1083 ✅** | 86.0444 | 3.0639 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | **470 ✅** | 477.0233 | -7.0233 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **5117.7778 ✅** | 5824.7058 | -706.9280 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 20 | **18 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 11 | 11 | 0 | tie |
| LLOC | Lower | 8 | 8 | 0 | tie |
| Comments (CLOC) | Higher | **5 ✅** | 3 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02923

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.6758 | **77.7520 ✅** | -2.0762 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **62.9591 ✅** | 59.2879 | 3.6712 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 7 | 7 | 0 | tie |
| Halstead Volume | Lower | 840.2535 | **797.0770 ✅** | 43.1765 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 18748.1567 | **17830.2663 ✅** | 917.8904 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 38 | **34 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 26 | **25 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **6 ✅** | 3 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02945

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **87.9495 ✅** | 87.2294 | 0.7201 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 82.0657 | **83.4043 ✅** | -1.3386 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | 519.5726 | 519.5726 | 0 | tie |
| Halstead Effort | Lower | 6927.6346 | 6927.6346 | 0 | tie |
| SLOC | Lower | **22 ✅** | 23 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 13 | 13 | 0 | tie |
| LLOC | Lower | 10 | 10 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02951

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 80.2454 | **80.6686 ✅** | -0.4232 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 77.0873 | **79.1746 ✅** | -2.0873 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 651.1817 | **600.2829 ✅** | 50.8988 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 9922.7683 | **9604.5264 ✅** | 318.2419 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 32 | 32 | 0 | tie |
| PLOC | Lower | 19 | **18 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **10 ✅** | 14 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **10 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02953

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 78.9840 | **79.4667 ✅** | -0.4826 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **67.6005 ✅** | 65.9666 | 1.6339 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 6 | 6 | 0 | tie |
| Halstead Volume | Lower | **759.6771 ✅** | 764.3204 | -4.6434 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 17040.9378 | **16399.6578 ✅** | 641.2800 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 32 | **31 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 22 | 22 | 0 | tie |
| LLOC | Lower | 16 | 16 | 0 | tie |
| Comments (CLOC) | Higher | **5 ✅** | 4 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02956

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 30.3392 | **30.5833 ✅** | -0.2441 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **-0.2891 ✅** | -3.0783 | 2.7892 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **36 ✅** | 38 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **34 ✅** | 40 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **6533.8258 ✅** | 6848.3610 | -314.5352 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **726859.4652 ✅** | 748548.3110 | -21688.8458 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 211 | **199 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **161 ✅** | 164 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **157 ✅** | 158 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **31 ✅** | 22 | 9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 22 | **19 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | 3 | 3 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02957

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **88.9044 ✅** | 88.0531 | 0.8512 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **80.4990 ✅** | 78.7074 | 1.7915 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **478.2215 ✅** | 487.2819 | -9.0604 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **6163.7439 ✅** | 6673.0547 | -509.3109 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **21 ✅** | 22 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 14 | 14 | 0 | tie |
| LLOC | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02959

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **70.3374 ✅** | 68.8965 | 1.4409 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 54.0375 | **59.8753 ✅** | -5.8378 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 15 | 15 | 0 | tie |
| Halstead Volume | Lower | 1397.6145 | 1397.6145 | 0 | tie |
| Halstead Effort | Lower | 58816.2766 | 58816.2766 | 0 | tie |
| SLOC | Lower | **43 ✅** | 47 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 27 | 27 | 0 | tie |
| LLOC | Lower | 29 | 29 | 0 | tie |
| Comments (CLOC) | Higher | 6 | **12 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02988

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 72.6798 | **73.8901 ✅** | -1.2103 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **58.0996 ✅** | 53.3681 | 4.7315 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 18 | **17 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **988.5336 ✅** | 1031.7796 | -43.2460 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 31676.0536 | **31526.5986 ✅** | 149.4550 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 41 | **37 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **27 ✅** | 30 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 20 | **19 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **6 ✅** | 3 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **5 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03003

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.8914 ✅** | 67.8675 | 0.0239 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.8866 ✅** | 44.8522 | 0.0344 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 10 | 10 | 0 | tie |
| Cognitive (sum) | Lower | 16 | 16 | 0 | tie |
| Halstead Volume | Lower | **1519.2903 ✅** | 1526.2790 | -6.9887 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 80171.7805 | **77557.5829 ✅** | 2614.1975 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 48 | 48 | 0 | tie |
| PLOC | Lower | 36 | 36 | 0 | tie |
| LLOC | Lower | 32 | 32 | 0 | tie |
| Comments (CLOC) | Higher | 4 | 4 | 0 | tie |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03007

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **45.8889 ✅** | 45.8280 | 0.0610 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **14.4042 ✅** | 14.2287 | 0.1755 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 19 | 19 | 0 | tie |
| Cognitive (sum) | Lower | 36 | 36 | 0 | tie |
| Halstead Volume | Lower | 4492.5476 | **4425.5855 ✅** | 66.9621 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 547665.8337 | **533940.9083 ✅** | 13724.9253 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **116 ✅** | 117 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 96 | **95 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 79 | **78 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 10 | 10 | 0 | tie |
| Blank Lines | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | 5 | 0 | tie |

## p03029

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **95.3225 ✅** | 94.2770 | 1.0455 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 81.4951 | **86.5530 ✅** | -5.0579 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | 415 | 415 | 0 | tie |
| Halstead Effort | Lower | 4394.1176 | 4394.1176 | 0 | tie |
| SLOC | Lower | **15 ✅** | 16 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 10 | 10 | 0 | tie |
| LLOC | Lower | 7 | 7 | 0 | tie |
| Comments (CLOC) | Higher | 1 | **2 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03034

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 64.7312 | **65.7233 ✅** | -0.9921 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **48.4648 ✅** | 46.6900 | 1.7748 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 13 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1274.1980 ✅** | 1320.9789 | -46.7809 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **44198.7435 ✅** | 46234.2602 | -2035.5167 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 60 | **55 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 39 | 39 | 0 | tie |
| LLOC | Lower | 31 | 31 | 0 | tie |
| Comments (CLOC) | Higher | **10 ✅** | 7 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03043

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.2141 ✅** | 74.9799 | 1.2341 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **73.6526 ✅** | 63.8308 | 9.8218 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **763.3676 ✅** | 1004.0193 | -240.6517 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **13435.2694 ✅** | 26327.6168 | -12892.3474 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 39 | **38 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **21 ✅** | 26 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **18 ✅** | 24 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **13 ✅** | 7 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03051

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **57.3605 ✅** | 57.1887 | 0.1719 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 31.0654 | **31.9696 ✅** | -0.9041 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | 13 | 13 | 0 | tie |
| Halstead Volume | Lower | 2623.6965 | **2498.3865 ✅** | 125.3100 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 168814.1554 | **148529.0776 ✅** | 20285.0778 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **75 ✅** | 77 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 57 | **56 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 48 | **46 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 14 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p03053

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.0589 | **60.1363 ✅** | -1.0774 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **35.0344 ✅** | 33.7203 | 1.3140 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 15 | 15 | 0 | tie |
| Cognitive (sum) | Lower | **20 ✅** | 21 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2355.3531 | **2319.3233 ✅** | 36.0298 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 95260.9495 | **88555.9822 ✅** | 6704.9673 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 67 | **63 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 49 | **48 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 38 | **37 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 5 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03055

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.4835 | **48.9134 ✅** | -2.4298 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **23.9294 ✅** | 22.9603 | 0.9692 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 16 | 16 | 0 | tie |
| Cognitive (sum) | Lower | 28 | **23 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3717.2937 | **3582.6793 ✅** | 134.6145 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 218176.5468 | **193464.6803 ✅** | 24711.8665 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 124 | **108 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 85 | **79 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 81 | **71 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **23 ✅** | 14 | 9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 18 | **16 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03064

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 58.9974 | **65.7868 ✅** | -6.7894 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 50.6214 | **55.6055 ✅** | -4.9842 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | **7 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1656.1511 | **1522.3843 ✅** | 133.7669 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 74526.8017 | **63841.9220 ✅** | 10684.8797 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 82 | **57 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 39 | **36 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 36 | 36 | 0 | tie |
| Comments (CLOC) | Higher | **30 ✅** | 15 | 15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | **10 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03074

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 60.8790 | **62.2110 ✅** | -1.3319 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 45.3766 | **47.6914 ✅** | -2.3148 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 11 | **9 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 16 | **11 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1807.5085 | **1755.4983 ✅** | 52.0103 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 61235.4576 | **53658.1900 ✅** | 7577.2676 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 69 | **66 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 46 | 46 | 0 | tie |
| LLOC | Lower | 40 | **39 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 14 | 14 | 0 | tie |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03080

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **73.1718 ✅** | 71.3758 | 1.7960 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **57.0210 ✅** | 51.2502 | 5.7708 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **929.1717 ✅** | 1002.3861 | -73.2144 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **23892.9859 ✅** | 26545.9486 | -2652.9627 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **40 ✅** | 43 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 32 | 32 | 0 | tie |
| LLOC | Lower | 18 | 18 | 0 | tie |
| Comments (CLOC) | Higher | **5 ✅** | 4 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03105

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 90.0902 | **91.8920 ✅** | -1.8019 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **87.2000 ✅** | 75.5246 | 11.6754 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | 520 | 520 | 0 | tie |
| Halstead Effort | Lower | 6652.9412 | 6652.9412 | 0 | tie |
| SLOC | Lower | 19 | **17 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 12 | 12 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | **4 ✅** | 1 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03106

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **77.5845 ✅** | 68.9511 | 8.6334 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **69.2656 ✅** | 55.8919 | 13.3738 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **7 ✅** | 17 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **8 ✅** | 12 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **787.5601 ✅** | 1111.7125 | -324.1524 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **16876.2888 ✅** | 33618.1875 | -16741.8987 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **34 ✅** | 45 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **22 ✅** | 33 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **18 ✅** | 19 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **0 ✅** | 4 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03108

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 49.5379 | **51.1254 ✅** | -1.5876 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 26.6174 | **30.3506 ✅** | -3.7332 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3136.4157 | **2964.9260 ✅** | 171.4897 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **136434.0833 ✅** | 136954.3461 | -520.2628 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 110 | **103 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 78 | **72 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 56 | **53 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 18 | **19 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03115

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.2032 | **60.4233 ✅** | -1.2201 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 33.5459 | **33.5769 ✅** | -0.0310 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 16 | **14 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2092.8831 | **1984.5129 ✅** | 108.3702 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 85522.8131 | **82637.1520 ✅** | 2885.6612 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 68 | **66 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 56 | **54 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **37 ✅** | 38 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **6 ✅** | 5 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03123

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 41.1328 | **41.1409 ✅** | -0.0081 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **5.7426 ✅** | 0.4325 | 5.3101 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **24 ✅** | 25 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 24 | 24 | 0 | tie |
| Halstead Volume | Lower | **4297.6067 ✅** | 4377.5958 | -79.9890 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **246196.5047 ✅** | 256016.3943 | -9819.8896 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 147 | **144 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **117 ✅** | 119 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **87 ✅** | 90 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **10 ✅** | 5 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 20 | 20 | 0 | tie |
| Functions | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03142

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 48.8118 | **49.3248 ✅** | -0.5130 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 19.9228 | **25.2961 ✅** | -5.3733 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 32 | **28 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 43 | 43 | 0 | tie |
| Halstead Volume | Lower | 2868.5045 | **2809.9213 ✅** | 58.5832 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 181791.4696 | **169087.0112 ✅** | 12704.4584 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **93 ✅** | 96 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 75 | **74 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **48 ✅** | 49 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **13 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03145

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 87.2232 | **89.8432 ✅** | -2.6200 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 88.0565 | **91.6345 ✅** | -3.5781 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | 520.1906 | **485.7822 ✅** | 34.4084 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 5259.7051 | **4829.2463 ✅** | 430.4588 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 23 | **20 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03157

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 50.0039 | **51.0022 ✅** | -0.9982 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 22.7155 | **24.5493 ✅** | -1.8338 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 27 | **26 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 39 | **38 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3151.4125 | **3132.2355 ✅** | 19.1770 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 179388.0956 | **175405.1901 ✅** | 3982.9055 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 90 | **86 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 71 | **67 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 49 | **47 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | 9 | 0 | tie |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03164

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 53.0184 | **59.2300 ✅** | -6.2116 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 29.0454 | **36.3994 ✅** | -7.3540 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 26 | **21 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 24 | **17 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2374.1105 | **2016 ✅** | 358.1105 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 114912.7391 | **80769.2308 ✅** | 34143.5083 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 83 | **64 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 63 | **50 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 40 | **31 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 7 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03165

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 61.4522 | **61.9190 ✅** | -0.4668 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 42.0060 | **44.6998 ✅** | -2.6938 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 21 | **19 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2161.3733 | **1870.6805 ✅** | 290.6928 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 113171.9055 | **81454.7733 ✅** | 31717.1321 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **62 ✅** | 64 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 46 | **45 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **40 ✅** | 41 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **11 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03173

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **64.6076 ✅** | 64.5463 | 0.0613 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 44.5637 | **46.0705 ✅** | -1.5068 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | 10 | 0 | tie |
| Cognitive (sum) | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1767.3657 ✅** | 1788.3235 | -20.9578 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 92256.4882 | **80311.9821 ✅** | 11944.5061 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 56 | 56 | 0 | tie |
| PLOC | Lower | 38 | 38 | 0 | tie |
| LLOC | Lower | **26 ✅** | 29 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03183

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.0786 ✅** | 61.1396 | 5.9391 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **47.1036 ✅** | 43.9157 | 3.1878 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **6 ✅** | 14 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1446.8697 ✅** | 1799.1613 | -352.2916 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **33704.6442 ✅** | 49656.8507 | -15952.2064 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **52 ✅** | 68 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **38 ✅** | 47 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **22 ✅** | 32 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **12 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03194

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **75.0669 ✅** | 74.6461 | 0.4208 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **60.0543 ✅** | 59.1397 | 0.9146 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | 864.6653 | 864.6653 | 0 | tie |
| Halstead Effort | Lower | 28121.2743 | 28121.2743 | 0 | tie |
| SLOC | Lower | **38 ✅** | 39 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 28 | 28 | 0 | tie |
| LLOC | Lower | 25 | 25 | 0 | tie |
| Comments (CLOC) | Higher | 5 | 5 | 0 | tie |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03206

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **87.2473 ✅** | 86.5579 | 0.6895 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **77.3196 ✅** | 75.8239 | 1.4957 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 4 | 4 | 0 | tie |
| Halstead Volume | Lower | 433.8179 | 433.8179 | 0 | tie |
| Halstead Effort | Lower | 4635.0017 | 4635.0017 | 0 | tie |
| SLOC | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 18 | 18 | 0 | tie |
| LLOC | Lower | 11 | 11 | 0 | tie |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03209

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **82.5555 ✅** | 81.4801 | 1.0755 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 59.2578 | **62.9237 ✅** | -3.6659 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 824.7922 | **798.0630 ✅** | 26.7293 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 20863.4947 | **18863.3068 ✅** | 2000.1879 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **25 ✅** | 27 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 21 | 21 | 0 | tie |
| LLOC | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 1 | **2 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03213

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **70.9173 ✅** | 66.5034 | 4.4139 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **56.1105 ✅** | 49.7749 | 6.3355 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **8 ✅** | 12 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **11 ✅** | 25 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1134.1230 ✅** | 1333.7400 | -199.6170 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **39255.2894 ✅** | 56017.0789 | -16761.7894 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **45 ✅** | 53 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **32 ✅** | 39 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **23 ✅** | 27 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03216

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **59.9790 ✅** | 59.5295 | 0.4495 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 34.3680 | **35.1370 ✅** | -0.7690 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 18 | **15 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1647.1401 | **1573.7932 ✅** | 73.3469 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 55343.9058 | **47861.8280 ✅** | 7482.0778 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **70 ✅** | 72 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **54 ✅** | 55 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 44 | **41 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **7 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03218

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 69.6991 | **70.4107 ✅** | -0.7116 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 49.6395 | **58.0535 ✅** | -8.4140 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1172.4109 | **1091.7827 ✅** | 80.6282 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 27285.1990 | **24565.1113 ✅** | 2720.0876 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 48 | **47 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 34 | **31 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 23 | **17 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | **9 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p03268

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **85.4355 ✅** | 84.4410 | 0.9945 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **77.4183 ✅** | 71.9905 | 5.4278 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | **614.7155 ✅** | 655.3899 | -40.6744 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **11647.2410 ✅** | 11797.0178 | -149.7768 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **24 ✅** | 25 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **16 ✅** | 18 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **4 ✅** | 3 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03277

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 49.7275 | **53.8133 ✅** | -4.0858 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 26.5910 | **28.8598 ✅** | -2.2688 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **15 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 16 | **11 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2724.7510 | **2325.4108 ✅** | 399.3402 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 132604.5491 | **105806.1916 ✅** | 26798.3575 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 109 | **93 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 76 | **66 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 53 | **44 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **17 ✅** | 11 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 18 | **17 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p03278

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 34.6211 | **37.4985 ✅** | -2.8775 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 1.5654 | **4.6740 ✅** | -3.1086 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 33 | **27 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 61 | **56 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6079.1284 | **5813.9203 ✅** | 265.2081 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 613801.9970 | **542553.1373 ✅** | 71248.8598 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 173 | **160 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 134 | **125 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 120 | 120 | 0 | tie |
| Comments (CLOC) | Higher | **18 ✅** | 16 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 21 | **19 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p03294

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **86.4623 ✅** | 82.8714 | 3.5908 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **75.8815 ✅** | 74.6070 | 1.2745 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 1 | 1 | 0 | tie |
| Halstead Volume | Lower | **576.1088 ✅** | 622.6656 | -46.5567 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **8394.7288 ✅** | 8915.4391 | -520.7102 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **23 ✅** | 28 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **15 ✅** | 16 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **12 ✅** | 13 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 3 | **5 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03296

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **80.3786 ✅** | 76.4233 | 3.9553 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **67.5698 ✅** | 66.8286 | 0.7412 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **742.4827 ✅** | 824.0030 | -81.5203 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **20047.0329 ✅** | 23699.0429 | -3652.0100 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **30 ✅** | 36 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **22 ✅** | 25 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 19 | 19 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **7 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03311

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 77.4065 | **79.0722 ✅** | -1.6656 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 72.8180 | **73.3298 ✅** | -0.5118 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | 778.8222 | **741.4002 ✅** | 37.4221 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 14392.6349 | **13623.2281 ✅** | 769.4068 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 36 | **33 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 20 | **19 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 8 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03317

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **81.9360 ✅** | 80.4807 | 1.4553 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **75.7765 ✅** | 70.8061 | 4.9704 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | **713.1384 ✅** | 845.7454 | -132.6070 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **15764.1112 ✅** | 19010.8852 | -3246.7740 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **28 ✅** | 29 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **17 ✅** | 19 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **13 ✅** | 15 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **6 ✅** | 5 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03318

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **71.5254 ✅** | 69.6092 | 1.9161 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **56.8860 ✅** | 53.5253 | 3.3607 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **7 ✅** | 9 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1054.5813 ✅** | 1141.2328 | -86.6514 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **37015.8053 ✅** | 40730.2034 | -3714.3981 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **45 ✅** | 48 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **32 ✅** | 35 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 29 | 29 | 0 | tie |
| Comments (CLOC) | Higher | 7 | 7 | 0 | tie |
| Blank Lines | Lower | 7 | 7 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03361

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.8171 ✅** | 65.0024 | 3.8147 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **53.2138 ✅** | 50.5236 | 2.6902 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 15 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **21 ✅** | 24 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1328.8775 ✅** | 1470.6608 | -141.7832 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **39137.5871 ✅** | 45884.6158 | -6747.0287 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **46 ✅** | 54 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **31 ✅** | 36 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **22 ✅** | 23 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **10 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03365

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **58.4820 ✅** | 58.1782 | 0.3038 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 30.2468 | **33.0831 ✅** | -2.8362 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | 13 | 0 | tie |
| Cognitive (sum) | Lower | 9 | 9 | 0 | tie |
| Halstead Volume | Lower | **2623.3331 ✅** | 2659.2377 | -35.9045 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 161334.9876 | **147958.7474 ✅** | 13376.2402 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **69 ✅** | 70 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 54 | **53 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 46 | **45 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | **7 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p03369

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **92.3964 ✅** | 90.2073 | 2.1891 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **96.8969 ✅** | 83.0801 | 13.8168 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **394.9548 ✅** | 414.5863 | -19.6315 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **4157.4189 ✅** | 4698.6444 | -541.2256 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **18 ✅** | 20 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **9 ✅** | 14 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **6 ✅** | 10 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **6 ✅** | 3 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 3 | 3 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03406

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **46.5368 ✅** | 46.4677 | 0.0691 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 11.0207 | **15.6101 ✅** | -4.5894 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 20 | 20 | 0 | tie |
| Cognitive (sum) | Lower | 32 | 32 | 0 | tie |
| Halstead Volume | Lower | 4233.0262 | **4172.4725 ✅** | 60.5536 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **306609.4830 ✅** | 318087.1670 | -11477.6840 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **112 ✅** | 113 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 93 | **91 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 79 | **77 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **10 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 13 | 13 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p03408

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 70.6990 | **71.0630 ✅** | -0.3641 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 46.7853 | **47.5185 ✅** | -0.7332 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | 10 | 0 | tie |
| Cognitive (sum) | Lower | 15 | 15 | 0 | tie |
| Halstead Volume | Lower | 1082.6181 | 1082.6181 | 0 | tie |
| Halstead Effort | Lower | 30464.8746 | 30464.8746 | 0 | tie |
| SLOC | Lower | 45 | **44 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 34 | 34 | 0 | tie |
| LLOC | Lower | 26 | 26 | 0 | tie |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03414

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 14.9604 | **23.3631 ✅** | -8.4026 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -27.4914 | **-8.9513 ✅** | -18.5401 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 55 | **49 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **88 ✅** | 116 | -28 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 11837.9902 | **7607.6119 ✅** | 4230.3783 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 1482116.3742 | **718496.6827 ✅** | 763619.6915 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 344 | **257 ✅** | 87 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 296 | **198 ✅** | 98 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 214 | **172 ✅** | 42 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 27 | **38 ✅** | -11 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 30 | **24 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 8 | **2 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 9 | **1 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |

## p03434

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.8759 | **76.7817 ✅** | -0.9058 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 65.1235 | **65.1372 ✅** | -0.0138 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | 845.1050 | **840.2535 ✅** | 4.8515 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **16682.3723 ✅** | 17277.7131 | -595.3408 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 38 | **36 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 24 | 24 | 0 | tie |
| LLOC | Lower | 19 | 19 | 0 | tie |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03435

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.0764 | **75.5328 ✅** | -0.4564 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **58.2154 ✅** | 56.0946 | 2.1208 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | 977.2383 | 977.2383 | 0 | tie |
| Halstead Effort | Lower | 34691.9582 | 34691.9582 | 0 | tie |
| SLOC | Lower | 36 | **35 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 26 | 26 | 0 | tie |
| LLOC | Lower | 20 | 20 | 0 | tie |
| Comments (CLOC) | Higher | **4 ✅** | 3 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03448

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 76.3076 | **79.2842 ✅** | -2.9766 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 57.5096 | **65.7033 ✅** | -8.1937 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | **6 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 921.5188 | **791.6176 ✅** | 129.9012 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 20273.4139 | **13235.8460 ✅** | 7037.5679 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 34 | **31 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 25 | **21 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 18 | **15 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 3 | **4 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03454

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **49.9935 ✅** | 49.5021 | 0.4914 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 14.5763 | **21.9052 ✅** | -7.3289 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 24 | 24 | 0 | tie |
| Cognitive (sum) | Lower | **40 ✅** | 42 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3148.9865 | **3039.6635 ✅** | 109.3230 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **240897.4681 ✅** | 244507.5681 | -3610.1000 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **94 ✅** | 98 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 75 | 75 | 0 | tie |
| LLOC | Lower | 60 | 60 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **10 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 15 | **13 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p03463

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 78.3582 | **87.5260 ✅** | -9.1678 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 62.0948 | **77.9469 ✅** | -15.8521 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | **3 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 5 | **2 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 803.4553 | **539.2711 ✅** | 264.1842 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 16434.3129 | **6173.2345 ✅** | 10261.0784 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 30 | **22 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 23 | **15 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p03473

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 106.0590 | 106.0590 | 0 | tie |
| MI (SEI) | Higher | 77.5136 | 77.5136 | 0 | tie |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | 258.5242 | 258.5242 | 0 | tie |
| Halstead Effort | Lower | 2197.4556 | 2197.4556 | 0 | tie |
| SLOC | Lower | 9 | 9 | 0 | tie |
| PLOC | Lower | 7 | 7 | 0 | tie |
| LLOC | Lower | 4 | 4 | 0 | tie |
| Comments (CLOC) | Higher | 0 | 0 | 0 | tie |
| Blank Lines | Lower | 2 | 2 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03478

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **78.7814 ✅** | 77.4384 | 1.3429 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **67.3081 ✅** | 59.7029 | 7.6052 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **789.8694 ✅** | 1179.9910 | -390.1217 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **16653.0790 ✅** | 34158.7056 | -17505.6266 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 32 | **31 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 22 | 22 | 0 | tie |
| LLOC | Lower | 19 | **18 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **5 ✅** | 3 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03495

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.1259 | **75.4343 ✅** | -0.3084 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 65.4658 | **68.2298 ✅** | -2.7640 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **900.3406 ✅** | 999.7083 | -99.3677 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **19285.2956 ✅** | 21570.6288 | -2285.3332 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 39 | **37 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 23 | **22 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 19 | 19 | 0 | tie |
| Comments (CLOC) | Higher | 8 | **9 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03544

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 87.8504 | **90.2254 ✅** | -2.3750 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **85.0068 ✅** | 78.8587 | 6.1481 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 1 | 1 | 0 | tie |
| Halstead Volume | Lower | 506.6527 | 506.6527 | 0 | tie |
| Halstead Effort | Lower | 8386.4347 | 8386.4347 | 0 | tie |
| SLOC | Lower | 22 | **19 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 13 | 13 | 0 | tie |
| LLOC | Lower | 12 | 12 | 0 | tie |
| Comments (CLOC) | Higher | **5 ✅** | 2 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03562

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 34.5925 | **34.8451 ✅** | -0.2525 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -1.9441 | **1.1768 ✅** | -3.1210 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **33 ✅** | 36 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **42 ✅** | 57 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 5897.5530 | **5484.7336 ✅** | 412.8193 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 529949.1262 | **454734.2792 ✅** | 75214.8470 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 175 | **169 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 139 | **132 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 110 | **105 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 13 | **16 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 25 | **22 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | **1 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p03568

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **81.8876 ✅** | 81.3384 | 0.5492 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 70.1537 | **71.7111 ✅** | -1.5573 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 4 | 4 | 0 | tie |
| Halstead Volume | Lower | 617.3437 | 617.3437 | 0 | tie |
| Halstead Effort | Lower | 10612.4315 | 10612.4315 | 0 | tie |
| SLOC | Lower | **29 ✅** | 30 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 19 | 19 | 0 | tie |
| LLOC | Lower | 15 | 15 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03592

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 82.4986 | **84.4067 ✅** | -1.9081 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **71.9073 ✅** | 62.1306 | 9.7766 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 6 | 6 | 0 | tie |
| Halstead Volume | Lower | 685.7667 | 685.7667 | 0 | tie |
| Halstead Effort | Lower | 12735.6665 | 12735.6665 | 0 | tie |
| SLOC | Lower | 27 | **24 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 19 | 19 | 0 | tie |
| LLOC | Lower | 13 | 13 | 0 | tie |
| Comments (CLOC) | Higher | **4 ✅** | 1 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03608

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **52.3887 ✅** | 52.2283 | 0.1604 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **23.9944 ✅** | 23.6491 | 0.3454 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 18 | 18 | 0 | tie |
| Cognitive (sum) | Lower | 30 | 30 | 0 | tie |
| Halstead Volume | Lower | 2966.3176 | **2955.7087 ✅** | 10.6089 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **144038.8646 ✅** | 155895.6126 | -11856.7480 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **90 ✅** | 91 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **74 ✅** | 75 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **51 ✅** | 52 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | 8 | 0 | tie |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03626

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 63.3167 | **68.7154 ✅** | -5.3987 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 53.9018 | **54.6417 ✅** | -0.7399 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1237.2308 | **1122.0269 ✅** | 115.2039 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 36993.2015 | **35545.8137 ✅** | 1447.3877 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 68 | **51 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 42 | **36 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 38 | **33 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **20 ✅** | 9 | 11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03632

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 83.9875 | **84.9727 ✅** | -0.9851 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 75.4312 | **79.6761 ✅** | -4.2449 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 776.9541 | **671.9361 ✅** | 105.0181 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 12607.8465 | **9071.1370 ✅** | 3536.7095 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 24 | 24 | 0 | tie |
| PLOC | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03646

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **70.9920 ✅** | 70.6784 | 0.3135 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **54.5368 ✅** | 51.9723 | 2.5645 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 10 | 10 | 0 | tie |
| Cognitive (sum) | Lower | 17 | 17 | 0 | tie |
| Halstead Volume | Lower | **1023.3007 ✅** | 1086.8956 | -63.5950 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **26935.9144 ✅** | 29872.0991 | -2936.1848 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 45 | 45 | 0 | tie |
| PLOC | Lower | **34 ✅** | 35 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **24 ✅** | 25 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **6 ✅** | 5 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03647

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 56.2461 | **61.4601 ✅** | -5.2140 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 44.1797 | **46.2925 ✅** | -2.1128 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **15 ✅** | 20 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 17 | **15 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1506.4117 | **1441.2813 ✅** | 65.1304 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 45694.4872 | **38914.5952 ✅** | 6779.8920 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 92 | **63 ✅** | 29 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 54 | **44 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 35 | **21 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **27 ✅** | 12 | 15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **8 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03701

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **78.5223 ✅** | 72.7454 | 5.7769 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **56.9059 ✅** | 55.1633 | 1.7425 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 7 | 7 | 0 | tie |
| Cognitive (sum) | Lower | **7 ✅** | 10 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **721.6932 ✅** | 960.9439 | -239.2507 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **17011.3400 ✅** | 24946.1029 | -7934.7629 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **33 ✅** | 43 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **24 ✅** | 29 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **15 ✅** | 26 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 2 | **5 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 9 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03709

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 35.2212 | **36.4753 ✅** | -1.2541 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 0.3628 | **10.7797 ✅** | -10.4169 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 32 | **26 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 51 | **48 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6947.2740 | **6238.7177 ✅** | 708.5564 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 742839.8698 | **511931.3491 ✅** | 230908.5207 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **162 ✅** | 169 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 128 | **120 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 109 | **102 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 14 | **32 ✅** | -18 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **22 ✅** | 23 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03719

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 90.9879 | 90.9879 | 0 | tie |
| MI (SEI) | Higher | 73.8286 | 73.8286 | 0 | tie |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 495.4196 | 495.4196 | 0 | tie |
| Halstead Effort | Lower | 6270.1547 | 6270.1547 | 0 | tie |
| SLOC | Lower | 18 | 18 | 0 | tie |
| PLOC | Lower | 14 | 14 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | 1 | 1 | 0 | tie |
| Blank Lines | Lower | 3 | 3 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03722

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **57.1020 ✅** | 56.7030 | 0.3990 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 33.6721 | **34.2574 ✅** | -0.5853 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17 | **14 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 26 | **24 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2036.3351 | **1987.3814 ✅** | 48.9538 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 86035.1596 | **73533.1110 ✅** | 12502.0486 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **77 ✅** | 83 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **58 ✅** | 63 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 39 | **38 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | **11 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03736

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **56.7495 ✅** | 54.1756 | 2.5739 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 28.0181 | **31.8253 ✅** | -3.8071 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **10 ✅** | 13 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **6 ✅** | 15 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2852.9540 | **2814.6654 ✅** | 38.2886 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 140908.0951 | **130680.8928 ✅** | 10227.2023 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **78 ✅** | 88 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **55 ✅** | 60 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **49 ✅** | 51 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **13 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03762

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 70.2546 | **70.4687 ✅** | -0.2141 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **58.2016 ✅** | 51.5158 | 6.6858 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 4 | 4 | 0 | tie |
| Halstead Volume | Lower | **1407.4106 ✅** | 1448.5951 | -41.1846 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **39189.1041 ✅** | 46173.9701 | -6984.8660 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 45 | **44 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 31 | 31 | 0 | tie |
| LLOC | Lower | 27 | 27 | 0 | tie |
| Comments (CLOC) | Higher | **9 ✅** | 5 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03766

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 60.9411 | **61.1366 ✅** | -0.1955 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 43.2372 | **44.0018 ✅** | -0.7646 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 8 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1951.2472 ✅** | 1970.4725 | -19.2253 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **102022.3521 ✅** | 107334.4511 | -5312.0990 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 69 | **67 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 45 | 45 | 0 | tie |
| LLOC | Lower | 39 | 39 | 0 | tie |
| Comments (CLOC) | Higher | 12 | 12 | 0 | tie |
| Blank Lines | Lower | 14 | **13 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p03797

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **83.3388 ✅** | 82.0424 | 1.2963 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 69.7282 | **70.9048 ✅** | -1.1765 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **583.4588 ✅** | 639.5337 | -56.0749 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **10225.8828 ✅** | 13463.8667 | -3237.9839 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **27 ✅** | 28 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 19 | 19 | 0 | tie |
| LLOC | Lower | 8 | 8 | 0 | tie |
| Comments (CLOC) | Higher | 3 | **4 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03808

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 57.4688 | **60.5704 ✅** | -3.1016 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **39.0817 ✅** | 33.7891 | 5.2926 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 15 | **13 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1914 ✅** | 1929.1596 | -15.1596 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 131236.6000 | **129519.7845 ✅** | 1716.8155 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 79 | **66 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 54 | **50 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 47 | **44 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **14 ✅** | 5 | 9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p03815

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **87.0577 ✅** | 82.5168 | 4.5409 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **76.8424 ✅** | 68.7461 | 8.0964 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **4 ✅** | 7 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **491.5424 ✅** | 625.5158 | -133.9734 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **7938.4099 ✅** | 12510.3160 | -4571.9061 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **23 ✅** | 27 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **16 ✅** | 19 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 7 | 7 | 0 | tie |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03817

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **90.0422 ✅** | 88.4759 | 1.5663 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 54.6099 | **68.9331 ✅** | -14.3232 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 502.1132 | **496.8278 ✅** | 5.2854 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 7632.1208 | **7551.7827 ✅** | 80.3381 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **19 ✅** | 21 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 16 | 16 | 0 | tie |
| LLOC | Lower | 7 | 7 | 0 | tie |
| Comments (CLOC) | Higher | 0 | **1 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03837

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 57.1445 | **57.7131 ✅** | -0.5687 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **32.3548 ✅** | 32.0213 | 0.3335 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 15 | 15 | 0 | tie |
| Cognitive (sum) | Lower | 25 | 25 | 0 | tie |
| Halstead Volume | Lower | 2298.3198 | **2238.7115 ✅** | 59.6082 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 88485.3114 | **86190.3941 ✅** | 2294.9172 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 76 | **74 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 59 | 59 | 0 | tie |
| LLOC | Lower | **49 ✅** | 52 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **8 ✅** | 7 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03838

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 74.1041 | **78.5877 ✅** | -4.4836 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 55.9268 | **80.6570 ✅** | -24.7302 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | **4 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 12 | **2 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 918.1417 | **569.7958 ✅** | 348.3458 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 27748.2813 | **9116.7334 ✅** | 18631.5479 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 39 | **37 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 31 | **16 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 26 | **13 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | **16 ✅** | -12 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03845

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 73.1297 | **73.9825 ✅** | -0.8528 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 47.4956 | **55.9239 ✅** | -8.4283 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 1321.6751 | 1321.6751 | 0 | tie |
| Halstead Effort | Lower | 27466.0608 | 27466.0608 | 0 | tie |
| SLOC | Lower | 39 | **37 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 29 | 29 | 0 | tie |
| LLOC | Lower | 23 | 23 | 0 | tie |
| Comments (CLOC) | Higher | 2 | **4 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | **6 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03865

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **82.8267 ✅** | 80.6542 | 2.1725 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **85.2701 ✅** | 77.3489 | 7.9212 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 5 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **437.5941 ✅** | 523.2548 | -85.6607 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **5951.2795 ✅** | 10465.0964 | -4513.8169 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **31 ✅** | 33 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **14 ✅** | 17 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **12 ✅** | 9 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03899

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.1995 | **57.3206 ✅** | -5.1211 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 32.2194 | **37.0382 ✅** | -4.8188 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 23 | **14 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2374.1105 | **2092.8008 ✅** | 281.3097 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 117396.7209 | **90238.2648 ✅** | 27158.4561 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 105 | **82 ✅** | 23 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 70 | **54 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 55 | **41 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **20 ✅** | 13 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03901

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 72.9525 | **83.5180 ✅** | -10.5654 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 64.6155 | **70.7607 ✅** | -6.1452 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 1 | 1 | 0 | tie |
| Halstead Volume | Lower | 915.1977 | **748.8245 ✅** | 166.3732 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 24161.2194 | **18435.3469 ✅** | 5725.8725 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 45 | **25 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 26 | **17 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **11 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 3 | 8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **5 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p03914

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 67.9429 | **73.3841 ✅** | -5.4412 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **58.3049 ✅** | 57.0135 | 1.2914 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 6 | 6 | 0 | tie |
| Halstead Volume | Lower | 1300.2379 | **1258.5799 ✅** | 41.6580 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 41867.6618 | **32723.0782 ✅** | 9144.5836 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 54 | **39 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 31 | **27 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 31 | **25 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **14 ✅** | 5 | 9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **7 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03929

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.6851 | **63.9219 ✅** | -4.2368 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 41.0513 | **46.1766 ✅** | -5.1253 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **14 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **19 ✅** | 24 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1595.3683 | **1362.7309 ✅** | 232.6375 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 67005.4704 | **39730.6529 ✅** | 27274.8175 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 70 | **60 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 49 | **43 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 37 | **34 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 9 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p03932

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **42.5379 ✅** | 41.0561 | 1.4819 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **14.6447 ✅** | 11.7956 | 2.8492 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **19 ✅** | 26 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **19 ✅** | 28 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4091.7968 ✅** | 4545.6497 | -453.8528 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **285147.0926 ✅** | 363651.9747 | -78504.8821 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 147 | **141 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 108 | **105 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 62 | **60 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **20 ✅** | 17 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 19 | 19 | 0 | tie |
| Functions | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03951

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 80.3039 | **84.0987 ✅** | -3.7949 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **74.1329 ✅** | 71.5985 | 2.5344 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 710.8407 | **669.6940 ✅** | 41.1467 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 13598.6913 | **12756.0762 ✅** | 842.6151 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 31 | **25 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 19 | **18 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 3 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03977

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 55.7347 | **59.2502 ✅** | -3.5156 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 36.7661 | **38.4764 ✅** | -1.7103 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | **12 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 17 | **13 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1842.9499 | **1614.4447 ✅** | 228.5052 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 103442.9925 | **81167.5975 ✅** | 22275.3950 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 89 | **78 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 63 | **57 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 59 | **56 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **16 ✅** | 11 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03996

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **53.1769 ✅** | 53.0818 | 0.0951 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **31.1832 ✅** | 27.5612 | 3.6220 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 15 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **17 ✅** | 20 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2488.3095 ✅** | 2504.9896 | -16.6801 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **99797.0927 ✅** | 102650.1153 | -2853.0226 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 96 | **95 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 71 | 71 | 0 | tie |
| LLOC | Lower | **55 ✅** | 56 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **15 ✅** | 11 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p04005

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 76.7227 | **78.6519 ✅** | -1.9292 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 66.0920 | **70.0845 ✅** | -3.9924 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 9 | 9 | 0 | tie |
| Halstead Volume | Lower | 889.2935 | **818.2861 ✅** | 71.0074 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 25678.3491 | **22698.5452 ✅** | 2979.8039 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 34 | **31 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 24 | **22 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 20 | **17 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | 6 | 0 | tie |
| Blank Lines | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p04013

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 57.6361 | **58.3916 ✅** | -0.7555 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **40.9462 ✅** | 39.4679 | 1.4784 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 19 | 19 | 0 | tie |
| Cognitive (sum) | Lower | 24 | 24 | 0 | tie |
| Halstead Volume | Lower | 2073.3186 | **2047.1598 ✅** | 26.1588 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 111111.0286 | **105645.8517 ✅** | 5465.1768 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 72 | **69 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 48 | 48 | 0 | tie |
| LLOC | Lower | 33 | 33 | 0 | tie |
| Comments (CLOC) | Higher | **14 ✅** | 11 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p04020

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **77.9655 ✅** | 74.1886 | 3.7769 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **68.0692 ✅** | 58.8890 | 9.1801 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 9 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 9 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **839.5814 ✅** | 979.4744 | -139.8930 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **30015.0357 ✅** | 36689.4785 | -6674.4428 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **33 ✅** | 38 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **21 ✅** | 28 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **20 ✅** | 22 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **6 ✅** | 5 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p04022

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **27.8449 ✅** | 22.5445 | 5.3004 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **-9.8755 ✅** | -17.5791 | 7.7036 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **40 ✅** | 58 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **90 ✅** | 120 | -30 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5845.0446 ✅** | 6599.5760 | -754.5314 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **488541.0412 ✅** | 667807.0978 | -179266.0566 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **241 ✅** | 249 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **193 ✅** | 204 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 154 | **152 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **20 ✅** | 17 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 30 | 30 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p04024

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **39.7986 ✅** | 38.1940 | 1.6046 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **9.4602 ✅** | 7.4484 | 2.0118 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **34 ✅** | 36 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **47 ✅** | 49 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4155.0063 ✅** | 4354.9231 | -199.9168 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **296621.2849 ✅** | 334156.6012 | -37535.3163 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **140 ✅** | 148 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **106 ✅** | 112 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **83 ✅** | 91 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 15 | **16 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **19 ✅** | 20 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p04030

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **71.5758 ✅** | 68.4764 | 3.0994 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 55.4448 | **57.1593 ✅** | -1.7146 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **8 ✅** | 11 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **14 ✅** | 19 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1071.6827 ✅** | 1075.3329 | -3.6502 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **26327.6709 ✅** | 29571.6549 | -3243.9841 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **44 ✅** | 51 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **33 ✅** | 35 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 27 | **25 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **11 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

