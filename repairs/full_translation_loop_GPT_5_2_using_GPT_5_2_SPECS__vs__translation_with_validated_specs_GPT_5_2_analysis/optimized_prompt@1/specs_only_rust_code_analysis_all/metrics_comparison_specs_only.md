# specs_only Rust Code Analysis Comparison

Pipeline A: `full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS`
Pipeline B: `translation_with_validated_specs_GPT_5_2`
Variant column: `specs_only`
Source A: `/Users/lithe/Documents/research/code_translation_specs/code_translation_specs_ASE/repairs/full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS_analysis/optimized_prompt@1/rust-code-analysis-all/metrics_comparison.csv`
Source B: `/Users/lithe/Documents/research/code_translation_specs/code_translation_specs_ASE/repairs/translation_with_validated_specs_GPT_5_2_analysis/optimized_prompt@1/rust-code-analysis-all/metrics_comparison.csv`

## Overall Averages

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **59.1129 ✅** | 58.6480 | 0.4648 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **37.8259 ✅** | 36.7288 | 1.0971 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **17.7533 ✅** | 17.8200 | -0.0667 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23.0533 ✅** | 23.5333 | -0.4800 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2433.0887 ✅** | 2499.3853 | -66.2965 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **130573.8437 ✅** | 139797.7260 | -9223.8823 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **86.7267 ✅** | 87.9500 | -1.2233 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **63.9733 ✅** | 65.3433 | -1.3700 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **43.9967 ✅** | 46.1667 | -2.1700 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **11.7033 ✅** | 11.1767 | 0.5267 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **11.8800 ✅** | 12.4767 | -0.5967 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2.1267 | **2.0533 ✅** | 0.0733 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0.4600 | **0.4433 ✅** | 0.0167 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2.1800 | **2.0900 ✅** | 0.0900 | translation_with_validated_specs_GPT_5_2 |

## p00004

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.3644 | **66.5912 ✅** | -1.2268 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 44.8114 | **55.7667 ✅** | -10.9553 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | **15 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 8 | **6 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1452.6968 | **1218.6637 ✅** | 234.0331 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 41912.9927 | **31532.9242 ✅** | 10380.0685 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **48 ✅** | 52 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 38 | **36 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | **12 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 5 | **0 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00010

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 60.7720 | **68.6058 ✅** | -7.8337 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 45.6155 | **50.3237 ✅** | -4.7082 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | **5 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 12 | **2 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2315.1308 | **1886.2699 ✅** | 428.8609 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 83528.8658 | **55670.4516 ✅** | 27858.4142 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 66 | **46 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 41 | **30 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 33 | **23 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **14 ✅** | 6 | 8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00013

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 77.3280 | **80.0694 ✅** | -2.7413 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 69.4536 | **72.9466 ✅** | -3.4929 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 664.5142 | **612.0217 ✅** | 52.4925 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 11695.4504 | **10265.2736 ✅** | 1430.1768 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 37 | **33 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 23 | **21 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **8 ✅** | 7 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00015

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **53.4062 ✅** | 52.9570 | 0.4491 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **29.2617 ✅** | 22.2086 | 7.0530 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 21 | **19 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 30 | **19 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **2136.0520 ✅** | 2634.2597 | -498.2077 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **96834.3554 ✅** | 131605.4629 | -34771.1075 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 90 | **89 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 64 | 64 | 0 | tie |
| LLOC | Lower | 41 | 41 | 0 | tie |
| Comments (CLOC) | Higher | **11 ✅** | 6 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **15 ✅** | 19 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00023

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.7553 ✅** | 67.3746 | 0.3807 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **46.3907 ✅** | 45.6283 | 0.7624 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | **1500.3502 ✅** | 1517.7219 | -17.3717 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **35440.7050 ✅** | 37331.7430 | -1891.0380 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **50 ✅** | 51 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **35 ✅** | 36 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | 5 | 0 | tie |
| Blank Lines | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00024

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.0697 | **82.1104 ✅** | -7.0407 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 69.0634 | **77.0857 ✅** | -8.0223 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | **3 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 778.8222 | **581.3942 ✅** | 197.4280 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 12016.1145 | **7482.2911 ✅** | 4533.8234 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 41 | **30 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 21 | **15 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 14 | **11 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 7 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00056

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 58.8127 | **59.8862 ✅** | -1.0735 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **37.7804 ✅** | 32.2778 | 5.5026 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 18 | **17 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 26 | **24 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1521.8657 | **1469.4577 ✅** | 52.4080 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 64932.9370 | **58568.3873 ✅** | 6364.5497 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 75 | **72 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 54 | 54 | 0 | tie |
| LLOC | Lower | 39 | 39 | 0 | tie |
| Comments (CLOC) | Higher | **10 ✅** | 5 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | 13 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00071

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 48.0575 | **50.7816 ✅** | -2.7241 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 19.8009 | **26.8109 ✅** | -7.0100 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 24 | **21 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 59 | **30 ✅** | 29 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **2964.9260 ✅** | 2989.8935 | -24.9675 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 156625.4368 | **129468.0296 ✅** | 27157.4072 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 108 | **95 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 87 | **66 ✅** | 21 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 59 | **46 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | **13 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 17 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00074

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 61.6269 | **63.9986 ✅** | -2.3717 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 38.4913 | **47.8871 ✅** | -9.3959 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 19 | 19 | 0 | tie |
| Cognitive (sum) | Lower | 11 | 11 | 0 | tie |
| Halstead Volume | Lower | 1613.1606 | **1411.5155 ✅** | 201.6451 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 57520.6964 | **40679.0921 ✅** | 16841.6042 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 61 | **55 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 47 | **42 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **17 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **9 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00077

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 68.1957 | **71.7269 ✅** | -3.5312 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 51.3522 | **58.6959 ✅** | -7.3437 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | **17 ✅** | 19 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1024.3700 | **906.3699 ✅** | 118.0001 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 24320.5259 | **22529.7665 ✅** | 1790.7594 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 55 | **46 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 35 | **32 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 25 | **24 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 8 | 8 | 0 | tie |
| Blank Lines | Lower | 12 | **6 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00080

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 71.4442 | **74.6490 ✅** | -3.2048 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **64.3717 ✅** | 62.9920 | 1.3797 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 11 | **7 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 801.9089 | **798.6496 ✅** | 3.2592 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **16923.0422 ✅** | 17232.4018 | -309.3596 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 48 | **40 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 28 | **25 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 18 | **16 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 7 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **7 ✅** | 9 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00105

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **62.7641 ✅** | 51.7930 | 10.9711 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.0083 ✅** | 28.6895 | 15.3189 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **15 ✅** | 20 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **15 ✅** | 25 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1399.2361 ✅** | 2335.2568 | -936.0207 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **33007.6216 ✅** | 81085.3066 | -48077.6850 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **63 ✅** | 98 | -35 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **45 ✅** | 77 | -32 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **28 ✅** | 46 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **14 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 12 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00108

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 57.4903 | **63.6294 ✅** | -6.1392 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 37.7214 | **43.5406 ✅** | -5.8193 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 19 | **12 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 18 | **9 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1661.6838 | **1352.8382 ✅** | 308.8456 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 50834.4063 | **34722.8464 ✅** | 16111.5599 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 78 | **63 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 60 | **47 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 39 | **37 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 8 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00118

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **54.4328 ✅** | 53.5205 | 0.9122 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 22.5696 | **22.9541 ✅** | -0.3845 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **18 ✅** | 24 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2717.7811 ✅** | 2746.7041 | -28.9230 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **154536.0525 ✅** | 164802.2476 | -10266.1952 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **76 ✅** | 79 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **62 ✅** | 64 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **40 ✅** | 41 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **5 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00124

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 66.1539 | **66.8657 ✅** | -0.7118 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 38.9753 | **42.4273 ✅** | -3.4520 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 7 | **5 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1426.4770 | **1359.0516 ✅** | 67.4254 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 33052.5161 | **29918.5500 ✅** | 3133.9661 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 53 | 53 | 0 | tie |
| PLOC | Lower | 43 | **40 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **24 ✅** | 26 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 3 | **4 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **8 ✅** | 11 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00136

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **70.6656 ✅** | 70.4823 | 0.1833 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **46.7372 ✅** | 46.4770 | 0.2602 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 12 | **8 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1089.5845 ✅** | 1322.5020 | -232.9175 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **18924.3618 ✅** | 26979.0411 | -8054.6793 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 45 | **44 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 35 | 35 | 0 | tie |
| LLOC | Lower | **18 ✅** | 31 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00137

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **72.7294 ✅** | 69.4726 | 3.2569 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **59.2973 ✅** | 50.2923 | 9.0050 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **8 ✅** | 13 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 12 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **922.1952 ✅** | 1120.8195 | -198.6243 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **18735.9323 ✅** | 25778.8485 | -7042.9162 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **43 ✅** | 46 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **27 ✅** | 34 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **7 ✅** | 5 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **3 ✅** | 5 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00146

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 41.3919 | **43.2882 ✅** | -1.8962 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 10.9669 | **13.7619 ✅** | -2.7950 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 23 | **22 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 36 | **32 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4459.8959 | **4139.1224 ✅** | 320.7734 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 240206.2216 | **239880.9576 ✅** | 325.2640 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 145 | **134 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 109 | **100 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 84 | **82 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **16 ✅** | 15 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 20 | **19 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p00156

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **47.8489 ✅** | 32.6989 | 15.1501 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **7.1107 ✅** | -2.8767 | 9.9874 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **27 ✅** | 43 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **36 ✅** | 45 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3776.9993 ✅** | 5552.5175 | -1775.5182 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **216128.2945 ✅** | 365244.6029 | -149116.3084 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **97 ✅** | 174 | -77 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **80 ✅** | 140 | -60 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **63 ✅** | 85 | -22 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 2 | **14 ✅** | -12 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **15 ✅** | 20 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 7 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **0 ✅** | 6 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00167

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 64.4179 | **67.1766 ✅** | -2.7587 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 46.0703 | **48.9013 ✅** | -2.8310 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 16 | **12 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 14 | **11 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1260.1960 | **1105.4886 ✅** | 154.7074 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 36388.1600 | **27968.8628 ✅** | 8419.2972 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 58 | **54 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 44 | **39 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **25 ✅** | 26 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **8 ✅** | 7 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **9 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00187

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **38.8748 ✅** | 38.5290 | 0.3458 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | -0.3472 | **3.5643 ✅** | -3.9115 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **24 ✅** | 28 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 22 | 22 | 0 | tie |
| Halstead Volume | Lower | **4808.6099 ✅** | 4936.8299 | -128.2199 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **187484.0826 ✅** | 224473.3879 | -36989.3053 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 163 | **156 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 134 | **119 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **56 ✅** | 57 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **12 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **21 ✅** | 25 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | 7 | 0 | tie |

## p00190

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **36.2507 ✅** | 31.0947 | 5.1560 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **9.2350 ✅** | -4.0425 | 13.2775 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **31 ✅** | 37 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **36 ✅** | 37 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5221.6470 ✅** | 6487.6500 | -1266.0030 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **279558.9460 ✅** | 378573.4601 | -99014.5141 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **169 ✅** | 199 | -30 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **131 ✅** | 161 | -30 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **72 ✅** | 77 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **28 ✅** | 19 | 9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **23 ✅** | 32 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **1 ✅** | 4 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00203

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **49.6938 ✅** | 47.6368 | 2.0570 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 23.5431 | **25.5695 ✅** | -2.0265 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 27 | **26 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **60 ✅** | 66 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2565.5803 ✅** | 2701.8442 | -136.2639 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **169530.8438 ✅** | 191380.6288 | -21849.7851 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **98 ✅** | 111 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **77 ✅** | 78 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **58 ✅** | 62 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 11 | **19 ✅** | -8 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 15 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00208

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 63.6960 | **64.2644 ✅** | -0.5684 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 48.9073 | **51.3756 ✅** | -2.4683 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 14 | **12 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 17 | 17 | 0 | tie |
| Halstead Volume | Lower | 1222.5407 | **1140 ✅** | 82.5407 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 34231.1386 | **31920 ✅** | 2311.1386 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **63 ✅** | 64 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 41 | **39 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 27 | **25 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 12 | **14 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00244

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 51.6320 | **51.9654 ✅** | -0.3334 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **24.8409 ✅** | 24.1749 | 0.6660 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 17 | 17 | 0 | tie |
| Cognitive (sum) | Lower | **21 ✅** | 32 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3237.8667 | **3036.7882 ✅** | 201.0786 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 140764.8835 | **122230.7231 ✅** | 18534.1604 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 93 | 93 | 0 | tie |
| PLOC | Lower | 71 | **69 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 53 | **49 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 9 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **12 ✅** | 15 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00252

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 79.1900 | **82.2685 ✅** | -3.0785 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **67.6041 ✅** | 43.9039 | 23.7002 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 14 | **9 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 5 | **2 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **777.0015 ✅** | 866.8060 | -89.8045 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 15817.5306 | **14898.2275 ✅** | 919.3031 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 28 | **24 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **4 ✅** | 0 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00256

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **37.0522 ✅** | 34.5180 | 2.5342 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **12.0076 ✅** | 4.7481 | 7.2595 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 36 | **29 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **24 ✅** | 38 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 5907.6579 | **5711.9985 ✅** | 195.6594 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **400604.0467 ✅** | 410288.6744 | -9684.6277 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **144 ✅** | 188 | -44 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **111 ✅** | 132 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **67 ✅** | 107 | -40 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 26 | **27 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **18 ✅** | 29 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 9 | **5 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 8 | **5 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00261

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 54.3496 | **66.1546 ✅** | -11.8050 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 18.2439 | **57.2009 ✅** | -38.9569 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 32 | **11 ✅** | 21 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 25 | **14 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1710.5305 | **1125.7478 ✅** | 584.7827 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 62619.2837 | **22210.6993 ✅** | 40408.5844 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 78 | **58 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 68 | **38 ✅** | 30 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 17 | **14 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 2 | **16 ✅** | -14 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00270

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 29.3473 | **33.8786 ✅** | -4.5313 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -10.6511 | **-1.1827 ✅** | -9.4684 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 42 | **33 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 71 | **62 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6945.9387 | **6197.0690 ✅** | 748.8698 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 429396.3170 | **415892.1847 ✅** | 13504.1324 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 202 | **180 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 162 | **140 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 121 | **107 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 12 | **16 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 28 | **25 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00271

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **91.1548 ✅** | 90.2015 | 0.9533 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **84.9851 ✅** | 82.9700 | 2.0151 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 1 | 1 | 0 | tie |
| Halstead Volume | Lower | **423.7291 ✅** | 433.8179 | -10.0888 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **4413.8449 ✅** | 4699.6939 | -285.8490 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **19 ✅** | 20 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 12 | 12 | 0 | tie |
| LLOC | Lower | 8 | 8 | 0 | tie |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00283

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 43.8870 | **48.5005 ✅** | -4.6135 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 16.8416 | **20.6463 ✅** | -3.8047 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 27 | **23 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 35 | **33 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3173.1094 | **3106.9866 ✅** | 66.1228 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 136496.5885 | **127518.1029 ✅** | 8978.4856 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 131 | **105 ✅** | 26 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 98 | **83 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 57 | **38 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **17 ✅** | 11 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 21 | **14 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00288

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 23.5538 | **23.9190 ✅** | -0.3652 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -15.1888 | **-13.8514 ✅** | -1.3374 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 43 | **42 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 81 | **75 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 9005.4605 | **8879.4333 ✅** | 126.0271 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **690884.9372 ✅** | 710396.1591 | -19511.2219 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 262 | **261 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 206 | 206 | 0 | tie |
| LLOC | Lower | **145 ✅** | 147 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 23 | **25 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 34 | 34 | 0 | tie |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p00292

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 69.1925 | **84.4745 ✅** | -15.2820 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 53.5905 | **72.0388 ✅** | -18.4483 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | **3 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 7 | **1 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1106.1958 | **651.1817 ✅** | 455.0141 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 32632.7765 | **8930.4915 ✅** | 23702.2850 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 47 | **25 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 30 | **16 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 17 | **12 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 3 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **6 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00315

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **49.3862 ✅** | 42.8613 | 6.5249 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **29.4184 ✅** | 16.0352 | 13.3832 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **19 ✅** | 26 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **27 ✅** | 51 | -24 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3127.5645 ✅** | 4137.4006 | -1009.8361 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **194366.6927 ✅** | 248544.9373 | -54178.2446 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **105 ✅** | 130 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **75 ✅** | 98 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **62 ✅** | 84 | -22 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **21 ✅** | 18 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **11 ✅** | 14 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00317

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **42.1512 ✅** | 40.9509 | 1.2002 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 2.1466 | **4.5137 ✅** | -2.3671 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 34 | **28 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 50 | **32 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **3580.5475 ✅** | 4644.0555 | -1063.5080 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **178886.9596 ✅** | 205886.4595 | -26999.4998 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **127 ✅** | 137 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 108 | 108 | 0 | tie |
| LLOC | Lower | 57 | **56 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | **8 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **15 ✅** | 21 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 13 | **8 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |

## p00320

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **58.3515 ✅** | 46.0892 | 12.2624 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **37.2307 ✅** | 21.2837 | 15.9471 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **20 ✅** | 34 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **18 ✅** | 22 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2065.5684 ✅** | 3224.4113 | -1158.8430 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **75674.9142 ✅** | 241185.9681 | -165511.0539 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **68 ✅** | 103 | -35 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **50 ✅** | 80 | -30 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **21 ✅** | 46 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **14 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 3 | **2 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **0 ✅** | 4 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00329

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **49.9255 ✅** | 45.9313 | 3.9941 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 17.3804 | **20.6401 ✅** | -3.2597 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **24 ✅** | 25 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **35 ✅** | 44 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2714.7670 ✅** | 3074.8736 | -360.1066 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **148342.6254 ✅** | 207323.3498 | -58980.7244 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **99 ✅** | 120 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **79 ✅** | 91 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **52 ✅** | 68 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **17 ✅** | -11 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 14 | **13 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00336

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.1752 ✅** | 67.8384 | 0.3368 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 47.0983 | **57.3323 ✅** | -10.2340 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | **7 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1324.0769 | **1214.3191 ✅** | 109.7578 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 47517.2766 | **42826.4338 ✅** | 4690.8428 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **50 ✅** | 54 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 38 | **30 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 31 | **25 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | **13 ✅** | -8 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00378

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.6318 | **77.0016 ✅** | -1.3698 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **60.9840 ✅** | 49.1284 | 11.8556 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 9 | 9 | 0 | tie |
| Halstead Volume | Lower | 920.8124 | 920.8124 | 0 | tie |
| Halstead Effort | Lower | 23867.4578 | 23867.4578 | 0 | tie |
| SLOC | Lower | 37 | **34 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 27 | 27 | 0 | tie |
| LLOC | Lower | 23 | 23 | 0 | tie |
| Comments (CLOC) | Higher | **5 ✅** | 1 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **6 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00428

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **62.2890 ✅** | 60.1949 | 2.0941 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **43.6205 ✅** | 33.2648 | 10.3556 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **12 ✅** | 23 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1541.7262 ✅** | 1654.6757 | -112.9496 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **43168.3325 ✅** | 62415.3414 | -19247.0089 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **62 ✅** | 68 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **45 ✅** | 52 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **26 ✅** | 40 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **9 ✅** | 5 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00435

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **80.0639 ✅** | 76.7727 | 3.2912 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **78.0742 ✅** | 72.4808 | 5.5934 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **560.8010 ✅** | 743.3983 | -182.5973 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **7570.8137 ✅** | 11453.8400 | -3883.0264 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **33 ✅** | 38 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **19 ✅** | 21 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **8 ✅** | 12 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | **11 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00445

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.1660 | **75.8737 ✅** | -0.7077 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 55.4835 | **62.5142 ✅** | -7.0307 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 11 | 11 | 0 | tie |
| Cognitive (sum) | Lower | 16 | 16 | 0 | tie |
| Halstead Volume | Lower | 879.2259 | **837.7444 ✅** | 41.4816 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **20009.9694 ✅** | 20911.3884 | -901.4190 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 36 | **35 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 26 | **23 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 20 | **17 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 3 | **5 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 7 | 7 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00450

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 55.4374 | **66.4516 ✅** | -11.0142 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 41.5975 | **50.6115 ✅** | -9.0140 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17 | **11 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 15 | **13 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1850.1572 | **1254.5551 ✅** | 595.6021 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 70400.8536 | **37636.6517 ✅** | 32764.2018 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 88 | **55 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 52 | **40 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 34 | **31 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **23 ✅** | 9 | 14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | **7 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00453

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 38.1796 | **42.7225 ✅** | -4.5428 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 7.6039 | **13.9876 ✅** | -6.3837 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 34 | **28 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 70 | **61 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4390.5473 | **3984.2841 ✅** | 406.2631 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 284641.4117 | **225007.2048 ✅** | 59634.2070 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 152 | **129 ✅** | 23 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 115 | **97 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 80 | **68 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **17 ✅** | 15 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 20 | **17 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00465

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **42.0746 ✅** | 35.0572 | 7.0174 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **7.5254 ✅** | 6.3335 | 1.1919 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **23 ✅** | 31 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **37 ✅** | 48 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4461.5570 ✅** | 5491.7235 | -1030.1665 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **255707.1478 ✅** | 371255.5571 | -115548.4093 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **139 ✅** | 179 | -40 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **107 ✅** | 130 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **79 ✅** | 96 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | **27 ✅** | -17 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00468

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **60.5219 ✅** | 59.4976 | 1.0243 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **38.6805 ✅** | 33.8772 | 4.8033 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1553.8196 ✅** | 1653.9294 | -100.1097 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **50741.9221 ✅** | 59659.5950 | -8917.6729 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **68 ✅** | 70 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **49 ✅** | 51 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 39 | **38 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **8 ✅** | 6 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **11 ✅** | 13 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00479

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 73.1570 | **74.2049 ✅** | -1.0479 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 56.2030 | **58.7088 ✅** | -2.5058 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 10 | **3 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1076.4032 | **1066.7278 ✅** | 9.6754 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 21886.8652 | **20617.2742 ✅** | 1269.5910 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 41 | **38 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 26 | 26 | 0 | tie |
| LLOC | Lower | 20 | **16 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | 5 | 0 | tie |
| Blank Lines | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00492

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 48.0345 | **48.7753 ✅** | -0.7408 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 17.9668 | **25.3429 ✅** | -7.3760 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 29 | **24 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 54 | **39 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **3231.0884 ✅** | 3282.4001 | -51.3117 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 206569.3574 | **193005.1259 ✅** | 13564.2315 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **98 ✅** | 100 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 82 | **72 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 55 | **48 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 8 | **15 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 13 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00495

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 57.8710 | **64.4275 ✅** | -6.5564 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 41.6237 | **53.2130 ✅** | -11.5893 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17 | **12 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 24 | **16 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1621.5434 | **1283.0542 ✅** | 338.4892 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 55107.1398 | **42386.6114 ✅** | 12720.5284 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 79 | **61 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 57 | **39 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 36 | **31 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **16 ✅** | 15 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00507

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **60.6627 ✅** | 60.0314 | 0.6313 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 35.2978 | **39.6219 ✅** | -4.3241 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 14 | 14 | 0 | tie |
| Cognitive (sum) | Lower | 15 | 15 | 0 | tie |
| Halstead Volume | Lower | 1650.1288 | **1631.7780 ✅** | 18.3508 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 67792.7907 | **62887.8650 ✅** | 4904.9258 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **69 ✅** | 72 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 55 | **54 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 45 | **44 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **10 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p00514

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **39.9191 ✅** | 23.5833 | 16.3359 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **10.6766 ✅** | -11.3676 | 22.0442 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **27 ✅** | 53 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23 ✅** | 53 | -30 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4282.5644 ✅** | 6913.7975 | -2631.2331 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **241964.8873 ✅** | 599195.7821 | -357230.8949 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **152 ✅** | 247 | -95 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **113 ✅** | 189 | -76 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **79 ✅** | 120 | -41 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 19 | **28 ✅** | -9 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **21 ✅** | 31 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **1 ✅** | 4 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **7 ✅** | 17 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00518

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 55.6037 | **58.4837 ✅** | -2.8800 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 30.8678 | **37.9349 ✅** | -7.0671 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 19 | **16 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **24 ✅** | 30 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1968.0972 | **1631.7780 ✅** | 336.3192 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 63983.2405 | **49152.3371 ✅** | 14830.9035 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 83 | **77 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 64 | **53 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 46 | **37 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | **11 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 13 | 13 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00519

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 38.5396 | **46.0889 ✅** | -7.5493 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 4.1433 | **18.0870 ✅** | -13.9437 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 42 | **31 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 42 | **32 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4912.3820 | **3572.9442 ✅** | 1339.4378 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 362327.7904 | **203320.7507 ✅** | 159007.0397 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 128 | **104 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 105 | **80 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 58 | **47 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | **11 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 14 | **13 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 9 | **6 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00528

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **40.7191 ✅** | 36.4236 | 4.2955 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **11.1292 ✅** | 4.7931 | 6.3361 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **27 ✅** | 32 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **30 ✅** | 35 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5192.5472 ✅** | 5512.9991 | -320.4519 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **292080.7816 ✅** | 361780.5791 | -69699.7976 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **136 ✅** | 162 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **104 ✅** | 130 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **70 ✅** | 86 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 16 | **18 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **16 ✅** | 20 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 4 | **1 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00534

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 56.5015 | **57.1639 ✅** | -0.6624 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 38.7877 | **42.8248 ✅** | -4.0371 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22 | **16 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1976.5169 | **1888.8205 ✅** | 87.6964 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **77726.5262 ✅** | 81455.3822 | -3728.8560 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 83 | **82 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 55 | **50 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 39 | **38 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 16 | **20 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00543

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 71.8117 | **74.0910 ✅** | -2.2792 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 56.0626 | **56.0907 ✅** | -0.0281 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 12 | 12 | 0 | tie |
| Halstead Volume | Lower | 1100.1849 | **1043.1805 ✅** | 57.0044 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 28358.2133 | **25647.8508 ✅** | 2710.3625 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 43 | **38 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 31 | **29 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 22 | **20 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **6 ✅** | 4 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00554

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 67.6346 | **69.1803 ✅** | -1.5457 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **52.5870 ✅** | 50.8187 | 1.7683 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 11 | 11 | 0 | tie |
| Halstead Volume | Lower | 1225.2439 | **1162.2449 ✅** | 62.9989 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 35225.7614 | **33928.6118 ✅** | 1297.1495 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 53 | **49 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 35 | **34 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 27 | **25 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **9 ✅** | 6 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00571

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 56.4003 | **58.0405 ✅** | -1.6401 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 44.6634 | **45.9308 ✅** | -1.2674 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | 13 | 0 | tie |
| Cognitive (sum) | Lower | 13 | 13 | 0 | tie |
| Halstead Volume | Lower | 1771.4721 | **1667.9648 ✅** | 103.5073 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **46803.0979 ✅** | 46981.0091 | -177.9112 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 89 | **82 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 51 | **47 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 30 | **29 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **27 ✅** | 23 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **12 ✅** | 14 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p00594

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.4760 ✅** | 58.3694 | 9.1067 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 33.3240 | **36.0595 ✅** | -2.7355 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **10 ✅** | 18 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **11 ✅** | 16 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1282.4162 ✅** | 1728.0789 | -445.6628 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **37889.5691 ✅** | 60256.4669 | -22366.8978 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **52 ✅** | 74 | -22 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **41 ✅** | 56 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **33 ✅** | 38 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 1 | **9 ✅** | -8 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00629

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **54.3085 ✅** | 52.5038 | 1.8047 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 26.6623 | **28.6176 ✅** | -1.9553 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 17 | 17 | 0 | tie |
| Halstead Volume | Lower | **2143.3222 ✅** | 2225.2874 | -81.9652 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **57333.8677 ✅** | 68256.4101 | -10922.5424 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **90 ✅** | 98 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **70 ✅** | 72 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **33 ✅** | 36 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **13 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **12 ✅** | 15 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p00641

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 44.7929 | **45.9432 ✅** | -1.1503 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 9.1529 | **13.0409 ✅** | -3.8879 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | **20 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 24 | **16 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3946.1494 | **3635.1087 ✅** | 311.0408 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 213092.0680 | **182407.0088 ✅** | 30685.0592 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 124 | **122 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 99 | **97 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 78 | **70 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | **9 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 18 | **17 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00649

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **55.3364 ✅** | 51.3428 | 3.9936 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **39.0153 ✅** | 26.7710 | 12.2442 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 22 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **6 ✅** | 11 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2353.3828 ✅** | 2567.8982 | -214.5154 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **64741.1001 ✅** | 83456.6918 | -18715.5917 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **88 ✅** | 95 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **59 ✅** | 72 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **36 ✅** | 37 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **20 ✅** | 12 | 8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **1 ✅** | 5 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00653

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 34.8086 | **44.9307 ✅** | -10.1221 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 4.9132 | **16.2005 ✅** | -11.2873 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 32 | **19 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 42 | **34 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 5913.3861 | **4736.1060 ✅** | 1177.2801 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 519650.1759 | **307083.0006 ✅** | 212567.1752 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 175 | **121 ✅** | 54 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 131 | **93 ✅** | 38 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 127 | **78 ✅** | 49 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **24 ✅** | 14 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 20 | **17 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00655

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 18.1531 | **21.9774 ✅** | -3.8243 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -29.6071 | **-26.7140 ✅** | -2.8932 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 72 | **69 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 85 | **68 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 7225.4325 | **6204.5546 ✅** | 1020.8779 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 691648.2969 | **457177.7064 ✅** | 234470.5905 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 260 | **225 ✅** | 35 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 231 | **199 ✅** | 32 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 109 | **80 ✅** | 29 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 4 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 24 | **23 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 13 | **8 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |

## p00658

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **-8.3995 ✅** | -19.1659 | 10.7664 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **-63.7828 ✅** | -63.9264 | 0.1437 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **134 ✅** | 151 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **111 ✅** | 178 | -67 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **15131.5041 ✅** | 20153.5433 | -5022.0392 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **1501966.2575 ✅** | 2671998.8887 | -1170032.6313 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **438 ✅** | 610 | -172 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **390 ✅** | 480 | -90 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **233 ✅** | 318 | -85 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **64 ✅** | -56 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **40 ✅** | 72 | -32 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 23 | **17 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **35 ✅** | 40 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00680

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 14.1084 | **18.9397 ✅** | -4.8313 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -29.7124 | **-20.8912 ✅** | -8.8212 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 71 | **59 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 107 | **89 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 10100.4997 | **8762.2874 ✅** | 1338.2123 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 923084.6634 | **759808.4565 ✅** | 163276.2068 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 304 | **280 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 253 | **227 ✅** | 26 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 161 | **146 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 18 | **23 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 34 | **31 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 8 | 8 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 11 | **8 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00690

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.4204 | **47.6882 ✅** | -1.2677 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **18.6657 ✅** | 17.5188 | 1.1469 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 18 | **17 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **24 ✅** | 25 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3358.9976 | **3206.9753 ✅** | 152.0222 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 133229.4716 | **129080.7574 ✅** | 4148.7141 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 125 | **119 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 100 | **94 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **61 ✅** | 64 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **15 ✅** | 11 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00692

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.1566 | **48.0111 ✅** | -1.8544 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 16.6797 | **22.2714 ✅** | -5.5917 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **26 ✅** | 32 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 46 | **41 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3305.2300 | **2587.5237 ✅** | 717.7063 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 173073.8625 | **133310.2754 ✅** | 39763.5871 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 114 | **101 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 85 | **79 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 64 | **53 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | **12 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 18 | **12 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00723

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 51.7823 | **57.3530 ✅** | -5.5707 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 34.8673 | **36.5792 ✅** | -1.7119 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **4 ✅** | 7 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2614.4844 | **2107.8287 ✅** | 506.6557 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 102876.9227 | **83610.5398 ✅** | 19266.3829 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 109 | **84 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 63 | **51 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 44 | **38 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **27 ✅** | 13 | 14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **19 ✅** | 20 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **2 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00740

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.9802 | **66.4897 ✅** | -6.5094 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 33.4723 | **45.5317 ✅** | -12.0594 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 14 | **12 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 18 | **10 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1451.1213 | **1066.0475 ✅** | 385.0738 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 40147.6890 | **31759.3305 ✅** | 8388.3585 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 75 | **57 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 57 | **44 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 49 | **38 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | 6 | 0 | tie |
| Blank Lines | Lower | 12 | **9 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00749

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 18.0139 | **26.1080 ✅** | -8.0941 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -23.2816 | **-10.5309 ✅** | -12.7507 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 67 | **49 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 124 | **105 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 8623.2034 | **7283.1803 ✅** | 1340.0231 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 763920.0101 | **625833.2814 ✅** | 138086.7287 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 266 | **220 ✅** | 46 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 223 | **182 ✅** | 41 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 156 | **132 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 18 | **20 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 29 | **28 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **0 ✅** | 5 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00762

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 27.0521 | **39.5919 ✅** | -12.5398 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -11.7330 | **15.7684 ✅** | -27.5014 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 42 | **29 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 59 | **37 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6479.3252 | **4261.3558 ✅** | 2217.9694 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 530286.4873 | **352214.1007 ✅** | 178072.3867 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 238 | **151 ✅** | 87 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 197 | **117 ✅** | 80 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 148 | **92 ✅** | 56 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 18 | **29 ✅** | -11 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 24 | **18 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 7 | **2 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p00792

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **79.6974 ✅** | 49.4166 | 30.2808 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **70.4660 ✅** | 25.4964 | 44.9695 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **5 ✅** | 20 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 22 | -19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **628.9629 ✅** | 3158.4604 | -2529.4976 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **8748.3017 ✅** | 144112.4978 | -135364.1962 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **33 ✅** | 103 | -70 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **21 ✅** | 74 | -53 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **15 ✅** | 68 | -53 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **15 ✅** | -9 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **6 ✅** | 16 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00799

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 40.0264 | **44.6648 ✅** | -4.6384 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 4.6723 | **16.8104 ✅** | -12.1381 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 31 | **20 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 46 | **29 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3662.9709 | **3313.6551 ✅** | 349.3158 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 194070.8588 | **172156.6560 ✅** | 21914.2028 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 150 | **136 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 121 | **99 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 76 | **68 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 10 | **17 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **20 ✅** | 22 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | **1 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p00811

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 45.6274 | **48.6741 ✅** | -3.0468 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 15.4391 | **19.3781 ✅** | -3.9389 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 32 | **28 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 37 | **33 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2731.0729 | **2481.6475 ✅** | 249.4255 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 118801.6728 | **107202.0006 ✅** | 11599.6721 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 115 | **104 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 90 | **82 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 58 | **53 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 9 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00816

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **46.5102 ✅** | 42.8348 | 3.6754 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **15.8556 ✅** | 11.6895 | 4.1661 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **27 ✅** | 31 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **43 ✅** | 51 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2584.5501 ✅** | 3104.7267 | -520.1766 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **133736.7204 ✅** | 183075.3834 | -49338.6630 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **119 ✅** | 133 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **95 ✅** | 111 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **62 ✅** | 78 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | **12 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 17 | **12 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00820

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.1162 ✅** | 62.8597 | 5.2565 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **51.8980 ✅** | 45.3561 | 6.5419 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 16 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **12 ✅** | 21 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1068.5533 ✅** | 1251.3856 | -182.8323 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **28868.1740 ✅** | 39749.8948 | -10881.7209 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **53 ✅** | 64 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **34 ✅** | 45 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **21 ✅** | 27 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **10 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **3 ✅** | 5 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00837

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 40.2610 | **46.9816 ✅** | -6.7206 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 9.3596 | **19.3644 ✅** | -10.0048 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 28 | **16 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 62 | **29 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4742.5834 | **3377.7490 ✅** | 1364.8344 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 330224.3249 | **141516.0367 ✅** | 188708.2882 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 142 | **124 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 109 | **89 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 80 | **73 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 15 | 15 | 0 | tie |
| Blank Lines | Lower | **20 ✅** | 21 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | **1 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 4 | **0 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p00846

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 30.2252 | **40.3061 ✅** | -10.0809 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -10.8611 | **8.9437 ✅** | -19.8048 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 51 | **27 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 42 | **29 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 5643.4064 | **4412.0115 ✅** | 1231.3950 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 377833.8996 | **229684.1263 ✅** | 148149.7732 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 180 | **147 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 149 | **115 ✅** | 34 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 77 | **59 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 8 | **15 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 23 | **17 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 8 | **6 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 18 | **7 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |

## p00855

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.1362 | **67.3538 ✅** | -20.2176 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 14.1998 | **50.5449 ✅** | -36.3451 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 28 | **12 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 29 | **13 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2800.9038 | **1009.0900 ✅** | 1791.8139 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 154049.7112 | **28607.7007 ✅** | 125442.0104 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 110 | **55 ✅** | 55 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 92 | **40 ✅** | 52 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 60 | **27 ✅** | 33 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 12 | **9 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | **2 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 11 | **3 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |

## p00856

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.0671 | **51.5220 ✅** | -4.4549 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 11.8923 | **28.4004 ✅** | -16.5081 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 29 | **21 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 43 | **23 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3332.8833 | **2964.9843 ✅** | 367.8990 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 179095.3132 | **163074.1352 ✅** | 16021.1780 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 103 | **91 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 81 | **62 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 47 | **44 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | **13 ✅** | -8 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 17 | **16 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00866

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **37.6381 ✅** | 35.7783 | 1.8597 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **13.5218 ✅** | 11.0826 | 2.4392 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **27 ✅** | 31 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 49 | **39 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 4953.1292 | **4780.5428 ✅** | 172.5864 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 325717.7740 | **298750.2597 ✅** | 26967.5143 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **167 ✅** | 179 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **113 ✅** | 123 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **90 ✅** | 96 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 34 | **36 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **21 ✅** | 23 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 5 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 5 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00867

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **23.2673 ✅** | 18.7165 | 4.5507 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | -18.9016 | **-17.3217 ✅** | -1.5799 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **58 ✅** | 67 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **91 ✅** | 135 | -44 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **7354.3475 ✅** | 8592.5619 | -1238.2144 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **456719.9862 ✅** | 591250.0909 | -134530.1047 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **230 ✅** | 255 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **191 ✅** | 199 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **133 ✅** | 148 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 12 | **28 ✅** | -16 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **27 ✅** | 30 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | **3 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 15 | **4 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |

## p00870

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **53.7851 ✅** | 42.2456 | 11.5396 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **20.6098 ✅** | 11.2566 | 9.3531 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **20 ✅** | 36 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **27 ✅** | 50 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2671.2959 ✅** | 3555.8798 | -884.5838 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **127739.0986 ✅** | 218535.9318 | -90796.8332 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **83 ✅** | 123 | -40 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **67 ✅** | 98 | -31 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **51 ✅** | 71 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **11 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **12 ✅** | 14 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00874

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.5054 ✅** | 67.0305 | 1.4749 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **51.7822 ✅** | 47.5612 | 4.2210 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1266.0753 ✅** | 1358.6195 | -92.5443 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **35956.5371 ✅** | 42456.8604 | -6500.3233 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **49 ✅** | 51 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **35 ✅** | 38 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 27 | 27 | 0 | tie |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00889

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **52.7031 ✅** | 44.3133 | 8.3898 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **37.9320 ✅** | 21.9582 | 15.9737 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **20 ✅** | 27 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **21 ✅** | 32 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2386.7083 ✅** | 3066.8411 | -680.1328 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **88407.6537 ✅** | 158376.3038 | -69968.6502 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **92 ✅** | 129 | -37 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **60 ✅** | 86 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **44 ✅** | 69 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 24 | 24 | 0 | tie |
| Blank Lines | Lower | **11 ✅** | 19 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 3 | 3 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00897

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **43.1731 ✅** | 40.8113 | 2.3618 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **10.9027 ✅** | 8.9197 | 1.9831 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **20 ✅** | 24 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **27 ✅** | 34 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4517.2843 ✅** | 4981.9441 | -464.6598 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **223810.9027 ✅** | 250824.2791 | -27013.3764 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **135 ✅** | 143 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **106 ✅** | 108 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **77 ✅** | 83 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 12 | **14 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **19 ✅** | 23 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p00907

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 51.6211 | **57.9441 ✅** | -6.3230 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 22.4294 | **27.2479 ✅** | -4.8185 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 21 | **17 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 33 | **27 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2384.2647 | **1960.1630 ✅** | 424.1017 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 99484.6153 | **76838.3910 ✅** | 22646.2244 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 97 | **74 ✅** | 23 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 74 | **61 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 53 | **46 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **8 ✅** | 4 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 15 | **9 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p00924

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **48.4195 ✅** | 48.0289 | 0.3906 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **23.9656 ✅** | 20.0103 | 3.9553 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 25 | **21 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **24 ✅** | 27 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3260.1294 | **3039.6635 ✅** | 220.4659 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **182997.8300 ✅** | 185246.7666 | -2248.9366 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **101 ✅** | 112 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **72 ✅** | 85 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **51 ✅** | 77 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **14 ✅** | 12 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **15 ✅** | 19 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **3 ✅** | 5 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p00949

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 44.3411 | **58.2692 ✅** | -13.9281 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 7.0695 | **35.3526 ✅** | -28.2831 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 23 | **14 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 29 | **11 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3730.2655 | **2016.5341 ✅** | 1713.7314 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 191486.9636 | **64977.2103 ✅** | 126509.7533 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 128 | **75 ✅** | 53 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 104 | **52 ✅** | 52 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 74 | **33 ✅** | 41 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **9 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 18 | **14 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 10 | **6 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p00971

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 22.1207 | **45.8657 ✅** | -23.7450 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -0.5065 | **19.5375 ✅** | -20.0440 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 49 | **19 ✅** | 30 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 85 | **27 ✅** | 58 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6849.0230 | **3666.0348 ✅** | 3182.9882 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 464649.8594 | **209349.8848 ✅** | 255299.9746 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 287 | **124 ✅** | 163 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 179 | **88 ✅** | 91 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 90 | **79 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **91 ✅** | 17 | 74 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 23 | **19 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01050

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **74.2800 ✅** | 65.2622 | 9.0178 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **63.4640 ✅** | 47.8047 | 15.6593 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **7 ✅** | 11 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **9 ✅** | 20 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **769.7799 ✅** | 1202.5542 | -432.7743 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **15210.8500 ✅** | 33250.6229 | -18039.7729 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **42 ✅** | 60 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **28 ✅** | 42 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **24 ✅** | 38 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **9 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **6 ✅** | 9 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01065

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 48.1015 | **56.7495 ✅** | -8.6480 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 22.1892 | **37.6776 ✅** | -15.4885 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 21 | **13 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 58 | **33 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3262.1238 | **2308.9586 ✅** | 953.1652 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 151976.5889 | **93625.4535 ✅** | 58351.1354 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 109 | **80 ✅** | 29 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 81 | **55 ✅** | 26 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 58 | **45 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 14 | 14 | 0 | tie |
| Blank Lines | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01106

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 54.3530 | **58.0330 ✅** | -3.6801 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 33.1386 | **35.5743 ✅** | -2.4357 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 22 | **18 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22 | **14 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1893.2009 | **1843.5402 ✅** | 49.6607 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **61639.1005 ✅** | 62311.6590 | -672.5585 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 87 | **74 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 60 | **55 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **34 ✅** | 37 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **13 ✅** | 9 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | **11 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01119

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **46.5877 ✅** | 44.9232 | 1.6645 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **21.5871 ✅** | 16.9197 | 4.6673 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **24 ✅** | 28 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **27 ✅** | 38 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2907.6576 ✅** | 3026.8579 | -119.2003 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **110103.3027 ✅** | 138983.2252 | -28879.9225 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **119 ✅** | 123 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **87 ✅** | 97 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **58 ✅** | 68 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **17 ✅** | 14 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 15 | **12 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01125

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.4496 | **60.1767 ✅** | -7.7271 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 27.0564 | **40.1737 ✅** | -13.1173 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 19 | **14 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 20 | **10 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2370.1075 | **1732.3683 ✅** | 637.7392 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 93867.2803 | **52735.3302 ✅** | 41131.9502 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 95 | **70 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 72 | **50 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 52 | **34 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 10 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 15 | **12 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01128

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **38.3218 ✅** | 30.7225 | 7.5993 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **4.9320 ✅** | -6.0416 | 10.9736 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **23 ✅** | 38 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **19 ✅** | 34 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5087.8535 ✅** | 7102.8710 | -2015.0175 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **227929.6324 ✅** | 405780.1443 | -177850.5120 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **168 ✅** | 195 | -27 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **125 ✅** | 158 | -33 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **51 ✅** | 79 | -28 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 16 | 16 | 0 | tie |
| Blank Lines | Lower | 27 | **23 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01138

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.6165 | **62.9235 ✅** | -3.3070 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 33.9385 | **40.7049 ✅** | -6.7664 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **14 ✅** | 17 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 13 | **8 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2111.7666 | **1607.0902 ✅** | 504.6764 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 59709.1640 | **42333.1074 ✅** | 17376.0566 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 68 | **58 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 51 | **42 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 36 | **29 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | 6 | 0 | tie |
| Blank Lines | Lower | 12 | 12 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **1 ✅** | 7 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01150

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **47.5528 ✅** | 39.2066 | 8.3463 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **21.5792 ✅** | 15.8797 | 5.6995 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **22 ✅** | 32 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **25 ✅** | 55 | -30 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2856.9218 ✅** | 3937.0469 | -1080.1250 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **138034.4345 ✅** | 255794.4767 | -117760.0422 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **116 ✅** | 152 | -36 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **86 ✅** | 102 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 61 | **56 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 15 | **30 ✅** | -15 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **17 ✅** | 21 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **6 ✅** | 13 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01153

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.0236 | **61.2232 ✅** | -2.1996 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **39.3376 ✅** | 35.1866 | 4.1510 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 14 | 14 | 0 | tie |
| Cognitive (sum) | Lower | **26 ✅** | 27 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1980.7439 | **1966.9449 ✅** | 13.7990 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **73331.5393 ✅** | 77764.5706 | -4433.0313 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 72 | **63 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 50 | **49 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 38 | 38 | 0 | tie |
| Comments (CLOC) | Higher | **11 ✅** | 5 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **9 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01184

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **34.4082 ✅** | 31.5430 | 2.8652 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **1.9079 ✅** | -4.1355 | 6.0434 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **40 ✅** | 42 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **49 ✅** | 73 | -24 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4564.1498 ✅** | 5602.1591 | -1038.0093 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **292866.2783 ✅** | 423443.1945 | -130576.9162 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **174 ✅** | 189 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **135 ✅** | 154 | -19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **93 ✅** | 124 | -31 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **18 ✅** | 16 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 11 | **2 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |

## p01255

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 21.4197 | **25.2816 ✅** | -3.8618 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -15.7498 | **-12.1304 ✅** | -3.6193 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 53 | **45 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 52 | **50 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 9262.1932 | **7572.1927 ✅** | 1690.0005 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 649173.1879 | **475723.0057 ✅** | 173450.1823 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 257 | **242 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 198 | **187 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 113 | **104 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **26 ✅** | 22 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 34 | 34 | 0 | tie |
| Functions | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 6 | **3 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 9 | 9 | 0 | tie |

## p01275

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **56.7339 ✅** | 45.8560 | 10.8779 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.2024 ✅** | 13.6446 | 30.5579 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 24 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **9 ✅** | 20 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2079.2716 ✅** | 3436.1156 | -1356.8440 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **77338.7599 ✅** | 179341.1197 | -102002.3598 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **84 ✅** | 118 | -34 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **49 ✅** | 90 | -41 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **39 ✅** | 56 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **24 ✅** | 9 | 15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **11 ✅** | 20 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 6 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **2 ✅** | 8 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01288

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **59.5440 ✅** | 35.2715 | 24.2726 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **37.0414 ✅** | 4.6485 | 32.3929 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **11 ✅** | 37 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **9 ✅** | 24 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1878.9788 ✅** | 4494.5406 | -2615.5619 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **51448.2279 ✅** | 289245.4383 | -237797.2104 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **74 ✅** | 173 | -99 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **53 ✅** | 138 | -85 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **39 ✅** | 84 | -45 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **21 ✅** | -12 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **13 ✅** | 15 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 10 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **1 ✅** | 11 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01316

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 54.7554 | **57.4171 ✅** | -2.6617 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 21.5235 | **31.0425 ✅** | -9.5189 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **15 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22 | **20 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2168.0308 | **2011.3956 ✅** | 156.6353 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 76513.4220 | **62528.1660 ✅** | 13985.2560 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 86 | **78 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 68 | **60 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 46 | **40 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | **7 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 14 | **13 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01317

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 26.3193 | **41.2417 ✅** | -14.9223 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -15.1297 | **-4.0463 ✅** | -11.0835 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 50 | **28 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 88 | **57 ✅** | 31 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6238.1094 | **4391.5221 ✅** | 1846.5873 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 456962.8753 | **226088.9540 ✅** | 230873.9213 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 225 | **137 ✅** | 88 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 183 | **115 ✅** | 68 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 136 | **94 ✅** | 42 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 2 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 30 | **20 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 5 | **2 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 6 | **0 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |

## p01339

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **42.8413 ✅** | 40.1828 | 2.6585 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **19.0774 ✅** | 8.1376 | 10.9398 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **26 ✅** | 30 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **42 ✅** | 51 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4153.2853 ✅** | 4710.0117 | -556.7264 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **258258.8330 ✅** | 350652.2528 | -92393.4197 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **130 ✅** | 139 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **95 ✅** | 111 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **77 ✅** | 88 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **23 ✅** | 13 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **12 ✅** | 17 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 4 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01353

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 49.2808 | **53.0001 ✅** | -3.7193 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **29.5464 ✅** | 28.3475 | 1.1989 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **15 ✅** | 16 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2948.3198 | **2659.7056 ✅** | 288.6142 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 127787.8253 | **118045.6562 ✅** | 9742.1690 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 114 | **95 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 74 | **71 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 51 | **49 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **24 ✅** | 12 | 12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p01368

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 31.0037 | **33.6077 ✅** | -2.6039 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -12.1175 | **-3.7690 ✅** | -8.3484 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 47 | **37 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 83 | **74 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6110.8010 | **5866.5250 ✅** | 244.2760 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 437261.7621 | **399306.3005 ✅** | 37955.4616 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 177 | **176 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 148 | **146 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 109 | **102 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **12 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 23 | **18 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01452

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 50.6321 | **52.8733 ✅** | -2.2411 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 21.1163 | **23.2645 ✅** | -2.1482 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 19 | **15 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 11 | **9 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3474.3965 | **3309.6386 ✅** | 164.7578 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **222361.3754 ✅** | 225657.1803 | -3295.8049 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 94 | **88 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 70 | **67 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **51 ✅** | 53 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **8 ✅** | 7 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | **5 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p01459

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 28.4765 | **30.6286 ✅** | -2.1521 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 0.3749 | **3.6932 ✅** | -3.3183 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 49 | **38 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 56 | **54 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 6514.9342 | **6283.8531 ✅** | 231.0811 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 397312.2771 | **393982.9760 ✅** | 3329.3011 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **197 ✅** | 204 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 148 | **139 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **87 ✅** | 89 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 34 | **39 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **24 ✅** | 27 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01471

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 54.3251 | **55.2607 ✅** | -0.9356 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 30.3100 | **36.7747 ✅** | -6.4647 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17 | **14 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 21 | **16 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2212.1756 | **2110.1144 ✅** | 102.0613 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 88093.7503 | **86952.1511 ✅** | 1141.5992 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 89 | 89 | 0 | tie |
| PLOC | Lower | 66 | **61 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **44 ✅** | 45 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 11 | **17 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 15 | **12 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01521

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 68.6236 | **75.0612 ✅** | -6.4376 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 58.5577 | **67.0700 ✅** | -8.5122 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 11 | **6 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 14 | **6 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1045.3239 | **872.1673 ✅** | 173.1566 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 23678.1698 | **14990.3751 ✅** | 8687.7948 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 51 | **39 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 32 | **22 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 25 | **19 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 9 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01529

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 54.5953 | **56.4251 ✅** | -1.8298 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 32.3251 | **35.3609 ✅** | -3.0358 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 16 | **14 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 24 | **23 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2628.3881 | **2351.2693 ✅** | 277.1187 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 138826.6779 | **111111.8120 ✅** | 27714.8659 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 84 | **80 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 60 | **56 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 54 | **53 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 12 | 12 | 0 | tie |
| Blank Lines | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01545

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 45.6033 | **65.1328 ✅** | -19.5295 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 20.5779 | **45.9526 ✅** | -25.3748 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 26 | **12 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 39 | **15 ✅** | 24 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3390.5678 | **1637.7747 ✅** | 1752.7931 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 253515.5803 | **69977.6462 ✅** | 183537.9340 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 117 | **54 ✅** | 63 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 91 | **40 ✅** | 51 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 74 | **28 ✅** | 46 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **17 ✅** | 7 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **8 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 6 | **2 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p01570

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 50.8874 | **50.9484 ✅** | -0.0611 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **29.4854 ✅** | 23.1687 | 6.3167 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 18 | **17 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **35 ✅** | 37 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2942.0558 ✅** | 3039.2039 | -97.1480 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **137340.5162 ✅** | 153175.8745 | -15835.3583 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 99 | 99 | 0 | tie |
| PLOC | Lower | **68 ✅** | 75 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **55 ✅** | 66 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **17 ✅** | 10 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | **14 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01600

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **65.8320 ✅** | 64.7162 | 1.1158 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **54.2871 ✅** | 46.1002 | 8.1869 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **8 ✅** | 13 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1613.8656 ✅** | 1638 | -24.1344 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **45470.6627 ✅** | 51450 | -5979.3373 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **55 ✅** | 57 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **34 ✅** | 38 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **25 ✅** | 28 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **13 ✅** | 8 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **9 ✅** | 12 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01601

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 63.0725 | **63.9346 ✅** | -0.8621 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 46.9984 | **52.5832 ✅** | -5.5848 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **12 ✅** | 15 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 17 | **14 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1242.9923 | **1174.2906 ✅** | 68.7018 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 44348.1909 | **41687.3157 ✅** | 2660.8753 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 67 | **62 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 49 | **39 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 30 | **27 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 12 | **15 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p01656

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **71.5347 ✅** | 68.8891 | 2.6456 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **59.7560 ✅** | 54.6886 | 5.0674 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1027.4919 ✅** | 1185.5369 | -158.0450 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **16406.7255 ✅** | 24475.6004 | -8068.8749 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **46 ✅** | 51 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **28 ✅** | 31 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **19 ✅** | 22 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | 9 | 0 | tie |
| Blank Lines | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01658

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **65.1591 ✅** | 59.9174 | 5.2416 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **49.9442 ✅** | 37.8084 | 12.1359 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 17 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23 ✅** | 32 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1260.3364 ✅** | 1745.3657 | -485.0293 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **53418.8743 ✅** | 95995.1156 | -42576.2413 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **57 ✅** | 68 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **39 ✅** | 50 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **30 ✅** | 43 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **10 ✅** | 8 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01668

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **33.6571 ✅** | 29.6433 | 4.0138 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **2.5661 ✅** | -7.7370 | 10.3031 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **44 ✅** | 47 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 113 | **91 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **5415.1494 ✅** | 6365.4177 | -950.2683 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **383772.3445 ✅** | 557691.2817 | -173918.9373 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **163 ✅** | 190 | -27 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **125 ✅** | 157 | -32 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **91 ✅** | 116 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **19 ✅** | 14 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 19 | 19 | 0 | tie |
| Functions | Lower | **4 ✅** | 7 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **2 ✅** | 7 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01725

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **28.6207 ✅** | 27.2507 | 1.3700 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **4.2288 ✅** | -2.3217 | 6.5505 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **45 ✅** | 51 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **50 ✅** | 51 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5516.3629 ✅** | 6278.6240 | -762.2611 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **343450.2677 ✅** | 412654.3007 | -69204.0331 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 218 | **209 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **150 ✅** | 168 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 84 | **68 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **51 ✅** | 33 | 18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **19 ✅** | 20 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01767

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **58.7913 ✅** | 58.3940 | 0.3972 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **35.9554 ✅** | 29.5888 | 6.3666 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **11 ✅** | 13 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **12 ✅** | 13 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2171.6599 ✅** | 2238.5080 | -66.8480 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **79874.7116 ✅** | 91965.3683 | -12090.6568 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 74 | **73 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **55 ✅** | 59 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **40 ✅** | 42 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **9 ✅** | 5 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01768

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 56.4713 | **59.8879 ✅** | -3.4167 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 33.6176 | **39.9041 ✅** | -6.2865 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 11 | **9 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **12 ✅** | 15 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2372.8129 | **2004.5892 ✅** | 368.2237 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 67493.3452 | **54759.5104 ✅** | 12733.8348 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 83 | **73 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 59 | **48 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 42 | 42 | 0 | tie |
| Comments (CLOC) | Higher | 11 | 11 | 0 | tie |
| Blank Lines | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01782

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 13.1313 | **25.0125 ✅** | -11.8812 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **-14.4877 ✅** | -17.2640 | 2.7763 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 64 | **49 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 146 | **78 ✅** | 68 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 9559.2428 | **7935.2941 ✅** | 1623.9487 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 1006669.6242 | **703806.4806 ✅** | 302863.1436 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 363 | **229 ✅** | 134 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 239 | **195 ✅** | 44 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 163 | **151 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **96 ✅** | 12 | 84 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 28 | **24 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 5 | **3 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p01841

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **41.5740 ✅** | 35.0562 | 6.5178 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 3.9715 | **7.3310 ✅** | -3.3594 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **22 ✅** | 41 | -19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **9 ✅** | 52 | -43 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3967.0954 ✅** | 4463.8526 | -496.7571 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **138171.0312 ✅** | 222550.5667 | -84379.5355 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **151 ✅** | 166 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **122 ✅** | 123 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **53 ✅** | 85 | -32 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **25 ✅** | -17 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 22 | **19 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 10 | **6 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p01847

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 51.5366 | **54.0639 ✅** | -2.5273 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **25.6156 ✅** | 20.3331 | 5.2824 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **19 ✅** | 21 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2584.9598 | **2496.7973 ✅** | 88.1624 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **81177.6785 ✅** | 97022.2876 | -15844.6091 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 102 | **87 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 77 | **69 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 55 | **53 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 4 | 8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | 14 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p01907

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **58.9028 ✅** | 49.1293 | 9.7736 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **31.9585 ✅** | 22.6516 | 9.3068 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **9 ✅** | 21 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **10 ✅** | 35 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2641.7210 ✅** | 3292.0435 | -650.3225 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **101993.4013 ✅** | 181315.6251 | -79322.2238 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **71 ✅** | 102 | -31 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **55 ✅** | 79 | -24 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **54 ✅** | 62 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **12 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p01916

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 82.1909 | **87.4953 ✅** | -5.3044 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **77.8714 ✅** | 77.3718 | 0.4996 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 5 | **3 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 4 | **1 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 582.3703 | **472.3135 ✅** | 110.0568 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 8298.7765 | **5926.4096 ✅** | 2372.3669 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 29 | **23 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 19 | **14 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 15 | **11 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 3 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p01917

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **51.1757 ✅** | 44.0988 | 7.0769 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **20.1465 ✅** | 15.5371 | 4.6094 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **25 ✅** | 27 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 51 | **37 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **2748.2821 ✅** | 3614.8965 | -866.6144 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **116658.8497 ✅** | 189071.9967 | -72413.1469 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **90 ✅** | 124 | -34 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **73 ✅** | 90 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **48 ✅** | 66 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **14 ✅** | -8 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **12 ✅** | 20 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 3 | **2 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | 3 | 0 | tie |

## p02035

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 44.5376 | **46.3471 ✅** | -1.8095 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **13.7750 ✅** | 11.2043 | 2.5707 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 24 | **23 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **17 ✅** | 21 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 4427.6964 | **4066.7606 ✅** | 360.9358 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 370798.6895 | **347238.7892 ✅** | 23559.9002 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 118 | **110 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 90 | **89 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **58 ✅** | 78 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **11 ✅** | 6 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 17 | **15 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 8 | **3 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p02038

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **69.6025 ✅** | 69.2422 | 0.3603 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **58.7015 ✅** | 52.0564 | 6.6451 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 13 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1068.5533 ✅** | 1171.5962 | -103.0429 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **26066.2246 ✅** | 28486.5247 | -2420.3001 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 47 | **46 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **29 ✅** | 33 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **17 ✅** | 19 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **10 ✅** | 6 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02045

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **43.4917 ✅** | 41.3234 | 2.1683 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **14.5375 ✅** | 3.5382 | 10.9993 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **37 ✅** | 40 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **21 ✅** | 28 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 4014.4243 | **3841.7828 ✅** | 172.6416 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 261370.5102 | **209377.1610 ✅** | 51993.3492 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **108 ✅** | 120 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **88 ✅** | 97 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **34 ✅** | 52 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **11 ✅** | 5 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **12 ✅** | 21 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 6 | 6 | 0 | tie |
| Exits | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02046

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 66.0808 | **66.1381 ✅** | -0.0573 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **47.3075 ✅** | 44.8626 | 2.4449 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 18 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1179.8645 ✅** | 1236.9954 | -57.1309 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **31774.9708 ✅** | 37771.0153 | -5996.0445 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 55 | **51 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 40 | 40 | 0 | tie |
| LLOC | Lower | 19 | **14 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 5 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02056

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.7977 | **48.1733 ✅** | -0.3756 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **25.2147 ✅** | 24.6358 | 0.5789 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 16 | 16 | 0 | tie |
| Cognitive (sum) | Lower | 18 | 18 | 0 | tie |
| Halstead Volume | Lower | 3856.1607 | **3792.7121 ✅** | 63.4486 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 179373.6705 | **172716.5538 ✅** | 6657.1167 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 113 | **111 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **78 ✅** | 80 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 63 | **59 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **20 ✅** | 18 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02061

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.8950 | **51.9575 ✅** | -4.0625 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 25.1980 | **31.3236 ✅** | -6.1256 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 28 | **22 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 47 | **39 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2638.3841 | **2358.0792 ✅** | 280.3049 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 162088.5525 | **122004.9662 ✅** | 40083.5863 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 107 | **94 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 77 | **67 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 51 | **49 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **17 ✅** | 16 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p02104

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 58.7015 | **67.1753 ✅** | -8.4738 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 43.0357 | **45.9613 ✅** | -2.9256 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 14 | **12 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 29 | **15 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1780.6588 | **1405.3799 ✅** | 375.2789 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 66552.1223 | **44796.4840 ✅** | 21755.6383 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 76 | **50 ✅** | 26 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 54 | **38 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 39 | **25 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **16 ✅** | 5 | 11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02114

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **63.4432 ✅** | 51.5028 | 11.9405 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **42.6796 ✅** | 23.7818 | 18.8978 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 25 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **25 ✅** | 44 | -19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1814.1754 ✅** | 2868.2202 | -1054.0448 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **64972.7934 ✅** | 149264.5227 | -84291.7293 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **58 ✅** | 87 | -29 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **43 ✅** | 67 | -24 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **31 ✅** | 56 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **8 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02153

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **41.3587 ✅** | 37.9832 | 3.3755 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 5.3881 | **10.3306 ✅** | -4.9424 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 33 | 33 | 0 | tie |
| Cognitive (sum) | Lower | **57 ✅** | 67 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4815.5088 ✅** | 4864.8018 | -49.2930 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 298853.3925 | **245064.3887 ✅** | 53789.0039 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **123 ✅** | 151 | -28 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **99 ✅** | 110 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **71 ✅** | 80 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **22 ✅** | -15 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **17 ✅** | 20 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02162

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **69.6884 ✅** | 66.0847 | 3.6037 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **26.7728 ✅** | 21.7774 | 4.9954 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **19 ✅** | 21 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **11 ✅** | 16 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1274.5391 ✅** | 1321.9921 | -47.4530 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **33095.5329 ✅** | 37228.2431 | -4132.7103 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **40 ✅** | 48 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **34 ✅** | 43 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 0 | 0 | 0 | tie |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 4 | 4 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02166

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 55.7294 | **62.6286 ✅** | -6.8992 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 39.4541 | **41.1303 ✅** | -1.6763 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17 | **14 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2098.7952 | **1659.8363 ✅** | 438.9590 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 89272.0114 | **68260.7661 ✅** | 21011.2453 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 83 | **61 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 60 | **50 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 46 | **36 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **18 ✅** | 7 | 11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **5 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02186

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 38.1559 | **44.5390 ✅** | -6.3831 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 2.9018 | **20.8764 ✅** | -17.9745 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 28 | **19 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 5097.7330 | **3630.8396 ✅** | 1466.8934 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 386500.8445 | **207717.0320 ✅** | 178783.8125 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 158 | **135 ✅** | 23 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 124 | **97 ✅** | 27 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 69 | **59 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 12 | **24 ✅** | -12 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 26 | **22 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 11 | **6 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | **4 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p02190

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 78.2106 | **80.7255 ✅** | -2.5149 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 63.4235 | **68.0703 ✅** | -4.6468 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 4 | 4 | 0 | tie |
| Halstead Volume | Lower | 800.9165 | **694.5619 ✅** | 106.3546 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 14170.0609 | **13922.8088 ✅** | 247.2521 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 33 | **30 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 22 | **20 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 18 | **15 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 4 | 4 | 0 | tie |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02205

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **82.3333 ✅** | 73.8239 | 8.5094 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **71.1208 ✅** | 62.0054 | 9.1154 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **4 ✅** | 10 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **4 ✅** | 7 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **660.6753 ✅** | 856.7291 | -196.0538 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **11925.1888 ✅** | 20486.9998 | -8561.8111 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **28 ✅** | 40 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **18 ✅** | 27 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 14 | 14 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **7 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02242

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 48.1001 | **52.3285 ✅** | -4.2284 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 20.9288 | **26.3732 ✅** | -5.4444 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 24 | **21 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 38 | **33 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3027.2059 | **2454.0240 ✅** | 573.1819 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 165081.4339 | **103248.5713 ✅** | 61832.8626 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 107 | **92 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 84 | **72 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 65 | **54 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 10 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02249

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **41.5598 ✅** | 40.6996 | 0.8602 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **15.0298 ✅** | 13.8003 | 1.2294 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **21 ✅** | 24 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **28 ✅** | 30 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 5035.1523 | **4208.6994 ✅** | 826.4529 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 244352.9785 | **237415.7409 ✅** | 6937.2376 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **142 ✅** | 152 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **102 ✅** | 109 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **74 ✅** | 77 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 22 | **23 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **18 ✅** | 20 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02256

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.4447 ✅** | 70.4609 | 5.9838 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **60.4055 ✅** | 55.2797 | 5.1258 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **857.1308 ✅** | 1106.1958 | -249.0650 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **18565.4524 ✅** | 26179.9676 | -7614.5152 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **35 ✅** | 46 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **25 ✅** | 31 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **16 ✅** | 21 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **7 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02266

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **65.0048 ✅** | 63.2928 | 1.7120 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 48.4455 | **51.3796 ✅** | -2.9340 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 17 | 17 | 0 | tie |
| Halstead Volume | Lower | 1254.4146 | **1248.5813 ✅** | 5.8333 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **30047.1492 ✅** | 30457.8168 | -410.6675 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **61 ✅** | 66 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **42 ✅** | 43 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 28 | 28 | 0 | tie |
| Comments (CLOC) | Higher | 10 | **16 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02276

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 67.6126 | **70.1337 ✅** | -2.5212 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 47.7721 | **50.7371 ✅** | -2.9650 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | 1364.7284 | **1231.2950 ✅** | 133.4334 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 39949.3225 | **34312.0868 ✅** | 5637.2357 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 52 | **46 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 40 | **37 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 30 | **28 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **6 ✅** | 5 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | **5 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02277

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **62.3099 ✅** | 56.1005 | 6.2094 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **40.0282 ✅** | 25.7486 | 14.2796 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **13 ✅** | 16 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **17 ✅** | 19 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1588.3136 ✅** | 2290.8254 | -702.5117 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **46171.3946 ✅** | 90096.4856 | -43925.0910 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **64 ✅** | 80 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **50 ✅** | 68 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **34 ✅** | 43 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **7 ✅** | 5 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02279

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **56.6128 ✅** | 46.3526 | 10.2602 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **33.9377 ✅** | 21.2362 | 12.7015 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **16 ✅** | 20 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **19 ✅** | 40 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2246.2738 ✅** | 3359.9151 | -1113.6414 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **75352.2753 ✅** | 151639.9436 | -76287.6684 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **78 ✅** | 122 | -44 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **61 ✅** | 87 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **37 ✅** | 72 | -35 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | **18 ✅** | -8 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 18 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02297

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **77.4634 ✅** | 71.3867 | 6.0767 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **64.9211 ✅** | 51.8334 | 13.0877 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **5 ✅** | 10 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 22 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **966.4985 ✅** | 1368.9730 | -402.4745 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **25090.3000 ✅** | 47583.6128 | -22493.3128 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **33 ✅** | 40 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **21 ✅** | 30 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **19 ✅** | 26 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **5 ✅** | 4 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02305

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **72.6831 ✅** | 72.6094 | 0.0738 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **59.6343 ✅** | 59.5279 | 0.1064 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | **1179.0891 ✅** | 1195.9332 | -16.8441 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **21908.2356 ✅** | 22221.2104 | -312.9748 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 41 | 41 | 0 | tie |
| PLOC | Lower | 31 | 31 | 0 | tie |
| LLOC | Lower | 18 | 18 | 0 | tie |
| Comments (CLOC) | Higher | 7 | 7 | 0 | tie |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02317

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **72.8886 ✅** | 69.7498 | 3.1388 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **63.7269 ✅** | 57.0264 | 6.7004 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 10 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **10 ✅** | 13 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **909.5830 ✅** | 1062.7460 | -153.1630 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **21725.0400 ✅** | 31662.5000 | -9937.4601 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **44 ✅** | 48 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **27 ✅** | 33 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 21 | 21 | 0 | tie |
| Comments (CLOC) | Higher | **10 ✅** | 9 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02328

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **66.3573 ✅** | 62.7249 | 3.6324 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 47.6174 | **48.3322 ✅** | -0.7148 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **14 ✅** | 19 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1352.6779 ✅** | 1451.8387 | -99.1608 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **43716.0918 ✅** | 48004.3451 | -4288.2533 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **54 ✅** | 67 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **39 ✅** | 43 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **26 ✅** | 34 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | **14 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **8 ✅** | 11 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02333

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 56.3964 | **63.5303 ✅** | -7.1338 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 30.9910 | **38.6631 ✅** | -7.6721 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | **9 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 12 | **9 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2447.5994 | **1931.5532 ✅** | 516.0461 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 142266.7124 | **90839.8120 ✅** | 51426.9005 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 78 | **59 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 59 | **44 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 48 | **39 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **8 ✅** | 5 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | **1 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p02340

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 69.9535 | **71.3570 ✅** | -1.4036 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 59.1932 | **64.1396 ✅** | -4.9463 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | **7 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 8 | **6 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1219.5714 | **1168.2782 ✅** | 51.2932 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 40572.5287 | **35799.3829 ✅** | 4773.1458 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 46 | **44 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 28 | **25 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 22 | **21 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 10 | **12 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02347

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **49.1879 ✅** | 43.3695 | 5.8184 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **16.4947 ✅** | 4.7020 | 11.7926 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **19 ✅** | 26 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23 ✅** | 37 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3063.6374 ✅** | 4135.8294 | -1072.1919 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **133268.2288 ✅** | 194669.2101 | -61400.9813 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **107 ✅** | 126 | -19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **84 ✅** | 105 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **50 ✅** | 54 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **7 ✅** | 5 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **16 ✅** | 17 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02353

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **37.1289 ✅** | 36.9165 | 0.2124 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **0.3375 ✅** | -5.9336 | 6.2711 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **33 ✅** | 38 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23 ✅** | 30 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5180.1266 ✅** | 5205.1704 | -25.0438 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 400026.0242 | **355686.6449 ✅** | 44339.3793 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 156 | **147 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 132 | **129 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 63 | **61 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 4 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | 16 | 0 | tie |
| Functions | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 10 | 10 | 0 | tie |

## p02355

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **53.3603 ✅** | 48.8135 | 4.5468 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **29.8966 ✅** | 28.8863 | 1.0102 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **24 ✅** | 27 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **31 ✅** | 39 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2097.4048 ✅** | 2525.2646 | -427.8598 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **107491.9939 ✅** | 142993.1081 | -35501.1142 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **87 ✅** | 104 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **66 ✅** | 74 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **45 ✅** | 55 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 11 | **20 ✅** | -9 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 14 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02369

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **64.2394 ✅** | 63.7229 | 0.5165 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 42.1962 | **46.9083 ✅** | -4.7121 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 16 | **9 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1489.2593 ✅** | 1616.8028 | -127.5434 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **49299.6188 ✅** | 50215.9914 | -916.3726 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **58 ✅** | 60 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 45 | **42 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **29 ✅** | 34 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **10 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 10 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p02370

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.2351 ✅** | 67.5503 | 0.6848 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 49.1194 | **51.1149 ✅** | -1.9955 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | **1368.1055 ✅** | 1381.1730 | -13.0675 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 38124.5408 | **36083.1446 ✅** | 2041.3962 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **50 ✅** | 52 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 35 | 35 | 0 | tie |
| LLOC | Lower | 27 | 27 | 0 | tie |
| Comments (CLOC) | Higher | 6 | **8 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02371

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **56.8784 ✅** | 55.6138 | 1.2646 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 33.2955 | **39.2909 ✅** | -5.9954 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | **13 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 19 | 19 | 0 | tie |
| Halstead Volume | Lower | 2418.9757 | **2378.1618 ✅** | 40.8139 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 96534.0064 | **84556.8644 ✅** | 11977.1420 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **76 ✅** | 85 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 58 | **55 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **37 ✅** | 45 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **19 ✅** | -10 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02393

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 89.9348 | 89.9348 | 0 | tie |
| MI (SEI) | Higher | **83.1232 ✅** | 78.3377 | 4.7855 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | 560 | 560 | 0 | tie |
| Halstead Effort | Lower | 6130.5263 | 6130.5263 | 0 | tie |
| SLOC | Lower | 19 | 19 | 0 | tie |
| PLOC | Lower | 12 | 12 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | **3 ✅** | 2 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02396

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **91.7723 ✅** | 87.2263 | 4.5461 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 57.1059 | **66.6230 ✅** | -9.5171 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 5 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **360 ✅** | 435.5780 | -75.5780 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **4500 ✅** | 5880.3028 | -1380.3028 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **19 ✅** | 23 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **14 ✅** | 17 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 10 | 10 | 0 | tie |
| Comments (CLOC) | Higher | 0 | **1 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02412

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.2019 ✅** | 67.1284 | 0.0736 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **45.8843 ✅** | 45.4742 | 0.4101 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **16 ✅** | 22 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1083.6204 | **1082.5770 ✅** | 1.0434 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **27671.0206 ✅** | 30447.4772 | -2776.4565 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **52 ✅** | 53 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 41 | **38 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **25 ✅** | 27 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | 5 | 0 | tie |
| Blank Lines | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02418

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **87.4592 ✅** | 68.9534 | 18.5057 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **77.3198 ✅** | 48.4402 | 28.8795 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **3 ✅** | 9 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **2 ✅** | 8 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **475.6028 ✅** | 1214.0855 | -738.4827 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **5495.8551 ✅** | 32897.8017 | -27401.9466 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **23 ✅** | 49 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **16 ✅** | 37 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **11 ✅** | 26 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 3 | **5 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 7 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 4 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02430

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.6348 ✅** | 67.3339 | 1.3008 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 35.8446 | **51.9056 ✅** | -16.0610 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 18 | **12 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1231.4296 ✅** | 1247.7500 | -16.3203 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 37455.9842 | **33962.1940 ✅** | 3493.7902 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **47 ✅** | 50 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 37 | **36 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 27 | **22 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 1 | **8 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02433

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **57.7152 ✅** | 34.9054 | 22.8098 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **44.2669 ✅** | 2.1559 | 42.1109 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **17 ✅** | 35 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **30 ✅** | 63 | -33 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1606.6742 ✅** | 4422.2902 | -2815.6160 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **66884.2938 ✅** | 235855.4781 | -168971.1842 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **80 ✅** | 183 | -103 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **57 ✅** | 143 | -86 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **36 ✅** | 98 | -62 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **20 ✅** | 19 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 23 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02444

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **70.5442 ✅** | 64.9959 | 5.5483 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **53.4836 ✅** | 45.4370 | 8.0466 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 11 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 12 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1331.1784 ✅** | 1659.8363 | -328.6578 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **33254.8093 ✅** | 56483.2515 | -23228.4421 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **45 ✅** | 55 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **31 ✅** | 43 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **24 ✅** | 31 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 6 | **7 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02449

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.2965 | **49.4549 ✅** | -3.1584 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 19.7716 | **20.9937 ✅** | -1.2221 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 25 | **23 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 20 | **18 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2942.0533 | **2586 ✅** | 356.0533 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 168979.4691 | **141799 ✅** | 27180.4691 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 119 | **105 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 87 | **81 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 57 | **53 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **15 ✅** | 10 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 19 | **16 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02450

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.1055 | **67.2628 ✅** | -2.1574 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **49.1177 ✅** | 40.7049 | 8.4128 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 11 | **10 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 17 | **8 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1305.9659 ✅** | 1419.4095 | -113.4436 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **34292.1359 ✅** | 42156.4618 | -7864.3259 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 59 | **51 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 41 | **40 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 30 | **29 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **10 ✅** | 3 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p02457

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **64.3527 ✅** | 54.1624 | 10.1903 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **48.3530 ✅** | 25.0399 | 23.3130 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 21 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **11 ✅** | 28 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1523.0751 ✅** | 2564.5042 | -1041.4292 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **32416.8002 ✅** | 121098.2758 | -88681.4756 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **58 ✅** | 81 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **41 ✅** | 67 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **21 ✅** | 42 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **10 ✅** | 6 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02542

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 19.1369 | **20.9102 ✅** | -1.7733 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -17.3382 | **-13.8092 ✅** | -3.5290 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 55 | **49 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 90 | **86 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 9528.8303 | **9440.1205 ✅** | 88.7098 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 817163.9914 | **787516.8467 ✅** | 29647.1447 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 285 | **279 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 219 | **208 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **157 ✅** | 160 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 33 | **37 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **34 ✅** | 36 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 4 | **0 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 9 | **6 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p02562

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **29.4930 ✅** | 28.2469 | 1.2461 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **-6.4281 ✅** | -8.1875 | 1.7594 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **38 ✅** | 39 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 73 | **71 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **6833.7070 ✅** | 7004.3238 | -170.6168 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **530645.2959 ✅** | 578787.6675 | -48142.3716 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **213 ✅** | 225 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **166 ✅** | 178 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 129 | 129 | 0 | tie |
| Comments (CLOC) | Higher | 20 | **21 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 29 | **27 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p02584

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **79.2642 ✅** | 75.5799 | 3.6843 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 62.2351 | **70.4981 ✅** | -8.2630 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | 868.1631 | **825.0630 ✅** | 43.1001 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **16853.6881 ✅** | 18072.8088 | -1219.1207 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **31 ✅** | 39 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 24 | **19 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 14 | **12 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 3 | **11 ✅** | -8 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 9 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02585

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **45.5419 ✅** | 45.3202 | 0.2216 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 26.9082 | **28.4472 ✅** | -1.5391 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 19 | **18 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 41 | **40 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3211.1049 | **2981.6919 ✅** | 229.4130 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 153257.2774 | **146217.5831 ✅** | 7039.6943 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **132 ✅** | 139 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 84 | **77 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 75 | **67 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 33 | **40 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **16 ✅** | 22 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02606

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **92.4261 ✅** | 88.7969 | 3.6292 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 83.1812 | **89.4413 ✅** | -6.2602 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | **490.4654 ✅** | 510.2822 | -19.8168 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **5452.8216 ✅** | 5883.2538 | -430.4322 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **17 ✅** | 21 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 11 | 11 | 0 | tie |
| LLOC | Lower | 8 | 8 | 0 | tie |
| Comments (CLOC) | Higher | 2 | **6 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02617

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 79.8645 | **80.8246 ✅** | -0.9601 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 63.9238 | **71.3023 ✅** | -7.3786 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 5 | **2 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 910.9526 | **791.6176 ✅** | 119.3350 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 21243.4140 | **16623.9693 ✅** | 4619.4447 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 29 | 29 | 0 | tie |
| PLOC | Lower | 20 | **19 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 17 | **16 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 3 | **5 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02618

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 44.6905 | **45.3134 ✅** | -0.6229 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 18.1568 | **18.2212 ✅** | -0.0644 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **20 ✅** | 21 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **21 ✅** | 25 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3887.3487 | **3731.6780 ✅** | 155.6707 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 202859.7949 | **194284.1872 ✅** | 8575.6077 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 129 | **124 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 99 | **95 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 74 | **71 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **18 ✅** | 16 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **14 ✅** | 17 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | 4 | 0 | tie |

## p02639

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **82.8804 ✅** | 79.9715 | 2.9089 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **69.7989 ✅** | 64.2818 | 5.5171 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **627.6685 ✅** | 816.8482 | -189.1796 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 12116.7318 | **11175.9679 ✅** | 940.7639 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **26 ✅** | 29 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **19 ✅** | 22 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **11 ✅** | 14 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p02644

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **54.8302 ✅** | 51.6650 | 3.1652 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **33.7508 ✅** | 27.0791 | 6.6717 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 18 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **26 ✅** | 37 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2958.6623 ✅** | 3078.1808 | -119.5185 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **123467.2557 ✅** | 127799.4714 | -4332.2157 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **82 ✅** | 93 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **56 ✅** | 67 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **45 ✅** | 53 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **13 ✅** | 12 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02647

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **55.6730 ✅** | 54.3930 | 1.2800 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **31.9588 ✅** | 31.9130 | 0.0458 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **17 ✅** | 19 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **25 ✅** | 29 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2121.6628 ✅** | 2145.2491 | -23.5863 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **75389.7499 ✅** | 86525.0453 | -11135.2953 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **83 ✅** | 87 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 64 | 64 | 0 | tie |
| LLOC | Lower | 52 | **49 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 10 | **12 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02668

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 53.6530 | **62.4137 ✅** | -8.7607 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 33.3123 | **48.7301 ✅** | -15.4178 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | **8 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3033.0446 | **2144.2013 ✅** | 888.8433 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 194262.8075 | **111038.9952 ✅** | 83223.8123 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 90 | **62 ✅** | 28 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 60 | **40 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 56 | **37 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **16 ✅** | 14 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | **9 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02678

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 62.7583 | **66.4414 ✅** | -3.6831 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 48.4268 | **49.3801 ✅** | -0.9533 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 13 | **10 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1836.8222 | **1541.2525 ✅** | 295.5697 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 66560.6354 | **44916.5008 ✅** | 21644.1346 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 62 | **53 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 43 | **38 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 34 | **29 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 8 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02703

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **42.5472 ✅** | 39.6297 | 2.9174 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **15.1066 ✅** | 10.2346 | 4.8720 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **22 ✅** | 26 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **43 ✅** | 54 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **4355.0798 ✅** | 4830.7320 | -475.6522 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **196869.4013 ✅** | 278521.8909 | -81652.4896 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **138 ✅** | 151 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **101 ✅** | 118 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **74 ✅** | 82 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 19 | 19 | 0 | tie |
| Blank Lines | Lower | 18 | **15 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02727

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 45.9335 | **46.1288 ✅** | -0.1953 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **19.1239 ✅** | 16.2366 | 2.8873 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 38 | 38 | 0 | tie |
| Cognitive (sum) | Lower | 25 | **24 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **3149.2215 ✅** | 3232.2518 | -83.0303 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **166989.4902 ✅** | 182332.1556 | -15342.6654 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 99 | **97 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 81 | **79 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 44 | **43 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 8 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 11 | 11 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02728

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 40.1151 | **48.3584 ✅** | -8.2433 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 14.7621 | **20.6862 ✅** | -5.9242 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 24 | **20 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 32 | **26 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 5011.1565 | **3245.2637 ✅** | 1765.8928 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 343264.2177 | **174042.2903 ✅** | 169221.9274 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 149 | **109 ✅** | 40 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 106 | **83 ✅** | 23 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 92 | **62 ✅** | 30 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **26 ✅** | 12 | 14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 17 | **14 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02731

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 95.2389 | **98.6026 ✅** | -3.3638 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **92.9849 ✅** | 66.7563 | 26.2286 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | 344.9167 | 344.9167 | 0 | tie |
| Halstead Effort | Lower | 3702.1054 | 3702.1054 | 0 | tie |
| SLOC | Lower | 16 | **13 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 9 | 9 | 0 | tie |
| LLOC | Lower | 6 | 6 | 0 | tie |
| Comments (CLOC) | Higher | **3 ✅** | 0 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02733

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.5793 | **51.2859 ✅** | -3.7066 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 19.3146 | **28.4558 ✅** | -9.1411 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 26 | **19 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 65 | **42 ✅** | 23 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2975.3343 | **2526.7317 ✅** | 448.6026 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 143265.1553 | **96313.0668 ✅** | 46952.0885 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 108 | **100 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 85 | **70 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 68 | **59 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | **15 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **12 ✅** | 15 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02734

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **63.3614 ✅** | 60.0203 | 3.3410 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **53.4622 ✅** | 38.3783 | 15.0839 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **8 ✅** | 11 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **9 ✅** | 15 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1470.7134 ✅** | 2038.5774 | -567.8639 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **51229.8517 ✅** | 105789.8113 | -54559.9596 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **66 ✅** | 70 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **35 ✅** | 48 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **30 ✅** | 43 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **19 ✅** | 9 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **12 ✅** | 13 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02743

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **87.1954 ✅** | 82.6787 | 4.5167 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 67.0857 | **72.0652 ✅** | -4.9795 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | **635.5533 ✅** | 692.3877 | -56.8344 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **9723.9657 ✅** | 11869.5033 | -2145.5376 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **21 ✅** | 27 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **16 ✅** | 18 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **11 ✅** | 13 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 1 | **4 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02761

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **49.2728 ✅** | 46.8276 | 2.4452 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **22.8107 ✅** | 22.5442 | 0.2665 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **28 ✅** | 30 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **26 ✅** | 37 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2499.2287 ✅** | 2572.0218 | -72.7931 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **113871.1098 ✅** | 148065.0397 | -34193.9300 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **100 ✅** | 112 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **78 ✅** | 83 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **47 ✅** | 55 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 11 | **16 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 13 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **7 ✅** | 10 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02762

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 45.9790 | **46.2345 ✅** | -0.2556 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 16.2978 | **18.7806 ✅** | -2.4828 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 21 | **19 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 15 | 15 | 0 | tie |
| Halstead Volume | Lower | 3636.5197 | **3592.4824 ✅** | 44.0374 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 193010.0009 | **153795.4100 ✅** | 39214.5909 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **120 ✅** | 122 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 94 | **92 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 69 | 69 | 0 | tie |
| Comments (CLOC) | Higher | 12 | **15 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **14 ✅** | 15 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02783

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **89.3417 ✅** | 84.3658 | 4.9759 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **85.4329 ✅** | 79.6732 | 5.7596 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | **511.8226 ✅** | 523.1869 | -11.3643 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 7098.1710 | **6828.9654 ✅** | 269.2055 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **20 ✅** | 27 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **12 ✅** | 16 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | 4 | **6 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02812

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **78.7598 ✅** | 78.5151 | 0.2447 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **73.0828 ✅** | 61.1229 | 11.9599 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **720.6450 ✅** | 795.3929 | -74.7479 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **13812.3620 ✅** | 16703.2514 | -2890.8894 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 33 | **32 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **20 ✅** | 24 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **17 ✅** | 20 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **8 ✅** | 3 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02831

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **84.3866 ✅** | 73.1786 | 11.2080 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **81.9263 ✅** | 60.9600 | 20.9662 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **4 ✅** | 12 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 7 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **498.5531 ✅** | 822.0792 | -323.5260 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **8032.2446 ✅** | 20587.7213 | -12555.4767 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **27 ✅** | 41 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **15 ✅** | 28 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **10 ✅** | 15 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 7 | 7 | 0 | tie |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **1 ✅** | 5 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02837

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 60.6863 | **61.8124 ✅** | -1.1261 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 35.7369 | **39.6905 ✅** | -3.9536 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | 15 | 0 | tie |
| Cognitive (sum) | Lower | **26 ✅** | 30 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1722.3972 | **1680.2406 ✅** | 42.1566 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 55683.2882 | **47256.7674 ✅** | 8426.5208 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 67 | **63 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 52 | **47 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 38 | **34 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **7 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02840

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 47.8713 | **50.4735 ✅** | -2.6021 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **30.4036 ✅** | 25.6602 | 4.7434 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 16 | 16 | 0 | tie |
| Cognitive (sum) | Lower | 21 | **19 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **2846.5701 ✅** | 2985.2122 | -138.6421 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **136514.2362 ✅** | 139685.4028 | -3171.1666 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 124 | **104 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **72 ✅** | 75 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **58 ✅** | 64 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **32 ✅** | 14 | 18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 20 | **15 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02846

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 66.1734 | **67.8096 ✅** | -1.6362 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **53.2396 ✅** | 51.5282 | 1.7114 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | 1560.9671 | **1525.0452 ✅** | 35.9219 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 40493.3236 | **39789.8154 ✅** | 703.5082 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 56 | **51 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 34 | **33 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 28 | **27 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 8 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 10 | 10 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p02862

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **59.0232 ✅** | 55.1770 | 3.8462 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 31.1294 | **37.8725 ✅** | -6.7430 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 14 | 14 | 0 | tie |
| Cognitive (sum) | Lower | 10 | 10 | 0 | tie |
| Halstead Volume | Lower | **2261.7498 ✅** | 2386.1019 | -124.3521 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **118031.7782 ✅** | 135918.3285 | -17886.5503 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **69 ✅** | 86 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **56 ✅** | 58 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **44 ✅** | 50 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | **18 ✅** | -13 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | 5 | 0 | tie |

## p02884

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.4119 | **54.1880 ✅** | -1.7761 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **32.8616 ✅** | 32.2896 | 0.5719 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 16 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **21 ✅** | 25 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2703.3073 | **2548.1790 ✅** | 155.1283 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **102560.1692 ✅** | 107787.9728 | -5227.8035 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 98 | **87 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 62 | **61 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 47 | 47 | 0 | tie |
| Comments (CLOC) | Higher | **19 ✅** | 13 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 17 | **13 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | **2 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02885

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 90.6159 | **92.3276 ✅** | -1.7117 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **87.9584 ✅** | 83.1408 | 4.8176 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | **470 ✅** | 478.2215 | -8.2215 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **5117.7778 ✅** | 6163.7439 | -1045.9661 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 19 | **17 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 11 | 11 | 0 | tie |
| LLOC | Lower | 8 | 8 | 0 | tie |
| Comments (CLOC) | Higher | **4 ✅** | 2 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02923

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **73.1192 ✅** | 72.6228 | 0.4964 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **59.7578 ✅** | 52.9923 | 6.7655 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **7 ✅** | 9 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **894.2888 ✅** | 969.0748 | -74.7859 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **23139.7228 ✅** | 26021.4517 | -2881.7288 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 43 | **42 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **29 ✅** | 32 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 23 | 23 | 0 | tie |
| Comments (CLOC) | Higher | **7 ✅** | 4 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p02945

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **89.4935 ✅** | 83.9895 | 5.5040 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **81.8467 ✅** | 66.7061 | 15.1407 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **2 ✅** | 4 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **519.5726 ✅** | 605.2554 | -85.6828 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **6927.6346 ✅** | 11146.7869 | -4219.1523 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **20 ✅** | 26 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **13 ✅** | 19 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **10 ✅** | 16 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **3 ✅** | 2 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02951

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 80.0546 | **82.1359 ✅** | -2.0813 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **75.2740 ✅** | 70.9378 | 4.3362 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 6 | 6 | 0 | tie |
| Halstead Volume | Lower | **646.2897 ✅** | 656.5482 | -10.2586 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **10612.7565 ✅** | 11057.6543 | -444.8978 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 32 | **28 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 19 | 19 | 0 | tie |
| LLOC | Lower | 11 | 11 | 0 | tie |
| Comments (CLOC) | Higher | **8 ✅** | 4 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02953

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 76.9917 | **78.1818 ✅** | -1.1901 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 66.0044 | **66.1611 ✅** | -0.1567 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 7 | 7 | 0 | tie |
| Cognitive (sum) | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 806.4400 | **770.5296 ✅** | 35.9105 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 19757.7811 | **19373.3155 ✅** | 384.4656 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 35 | **33 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 24 | **23 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 17 | 17 | 0 | tie |
| Comments (CLOC) | Higher | **6 ✅** | 5 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p02956

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **35.3139 ✅** | 26.0285 | 9.2854 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **0.7787 ✅** | -10.4761 | 11.2547 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **36 ✅** | 43 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **34 ✅** | 39 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **5503.4386 ✅** | 7851.7771 | -2348.3385 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **493780.7402 ✅** | 785882.3555 | -292101.6153 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **164 ✅** | 235 | -71 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **136 ✅** | 184 | -48 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **105 ✅** | 159 | -54 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 14 | **23 ✅** | -9 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **14 ✅** | 29 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 5 | **4 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p02957

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 86.9223 | **91.4016 ✅** | -4.4793 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **76.5452 ✅** | 56.4692 | 20.0760 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | 527.3324 | **478.2215 ✅** | 49.1108 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 6538.9211 | **6163.7439 ✅** | 375.1773 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 23 | **18 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 18 | **14 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | **3 ✅** | 0 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02959

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 68.7053 | **73.7702 ✅** | -5.0649 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 59.1041 | **64.8617 ✅** | -5.7576 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1156.0872 | **1033.1535 ✅** | 122.9337 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 42775.2278 | **29548.1909 ✅** | 13227.0369 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 52 | **40 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 31 | **22 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 29 | **20 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **13 ✅** | 9 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p02988

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.3165 ✅** | 74.8239 | 1.4926 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **60.6527 ✅** | 55.2755 | 5.3772 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **10 ✅** | 11 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **919.9486 ✅** | 1025.1467 | -105.1981 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **24201.7236 ✅** | 29660.9104 | -5459.1868 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **34 ✅** | 35 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **23 ✅** | 27 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 18 | 18 | 0 | tie |
| Comments (CLOC) | Higher | **4 ✅** | 3 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03003

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **63.8352 ✅** | 59.0791 | 4.7562 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **50.0047 ✅** | 39.2140 | 10.7907 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | **14 ✅** | 15 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1682.4359 ✅** | 2140.9962 | -458.5603 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **75946.2086 ✅** | 110536.5741 | -34590.3656 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **58 ✅** | 72 | -14 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **40 ✅** | 49 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **30 ✅** | 40 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **12 ✅** | 11 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **7 ✅** | 12 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03007

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **55.8678 ✅** | 41.4597 | 14.4080 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **43.4607 ✅** | 12.6558 | 30.8049 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **13 ✅** | 23 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **16 ✅** | 57 | -41 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2264.8176 ✅** | 4698.4068 | -2433.5892 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **119069.4533 ✅** | 535265.9946 | -416196.5413 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **85 ✅** | 142 | -57 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **53 ✅** | 110 | -57 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **49 ✅** | 81 | -32 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **25 ✅** | 18 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **9 ✅** | 14 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **0 ✅** | 5 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03029

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 94.2770 | **95.1987 ✅** | -0.9217 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **86.5530 ✅** | 81.3164 | 5.2366 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | **415 ✅** | 425 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **4394.1176 ✅** | 4687.5000 | -293.3824 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 10 | 10 | 0 | tie |
| LLOC | Lower | 7 | 7 | 0 | tie |
| Comments (CLOC) | Higher | **2 ✅** | 1 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 4 | 4 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03034

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.1751 | **64.2443 ✅** | -5.0692 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **44.7437 ✅** | 43.2577 | 1.4860 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 16 | **12 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 23 | **15 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1488 | **1329.0504 ✅** | 158.9496 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 60805.0909 | **47867.2501 ✅** | 12937.8408 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 76 | **61 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 50 | **43 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 36 | 36 | 0 | tie |
| Comments (CLOC) | Higher | **17 ✅** | 7 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 11 | 11 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03043

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.7989 ✅** | 73.0663 | 3.7326 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 63.2381 | **64.1958 ✅** | -0.9577 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 4 | 4 | 0 | tie |
| Halstead Volume | Lower | **914.3047 ✅** | 987.0020 | -72.6973 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **19471.3039 ✅** | 22701.0462 | -3229.7424 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **35 ✅** | 43 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **23 ✅** | 25 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **20 ✅** | 22 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 5 | **10 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | 8 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03051

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **61.3120 ✅** | 55.3493 | 5.9627 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **46.6333 ✅** | 35.4656 | 11.1677 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 13 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1662.1084 ✅** | 2327.3740 | -665.2656 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **67984.8521 ✅** | 126245.8500 | -58260.9980 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **70 ✅** | 87 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **47 ✅** | 57 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **37 ✅** | 44 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 15 | 15 | 0 | tie |
| Blank Lines | Lower | **9 ✅** | 15 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 5 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 4 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03053

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 55.1722 | **55.6113 ✅** | -0.4390 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 35.5156 | **35.7147 ✅** | -0.1991 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 16 | 16 | 0 | tie |
| Cognitive (sum) | Lower | 30 | 30 | 0 | tie |
| Halstead Volume | Lower | **2108.7599 ✅** | 2161.9220 | -53.1620 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **103931.7400 ✅** | 105007.6380 | -1075.8979 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 87 | **84 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 64 | **62 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 51 | **49 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **15 ✅** | 14 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **8 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03055

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **59.4978 ✅** | 56.4536 | 3.0442 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **35.9439 ✅** | 27.8542 | 8.0897 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **11 ✅** | 15 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **12 ✅** | 18 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2064.6813 ✅** | 2237.2428 | -172.5615 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **68627.1898 ✅** | 82458.3773 | -13831.1875 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **72 ✅** | 80 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **52 ✅** | 62 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 40 | 40 | 0 | tie |
| Comments (CLOC) | Higher | **8 ✅** | 6 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | 12 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03064

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.5958 | **59.9980 ✅** | -7.4022 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 15.8911 | **41.5344 ✅** | -25.6433 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 18 | **11 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 17 | **8 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2754.0170 | **1875.3170 ✅** | 878.7000 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 158608.8994 | **80896.4874 ✅** | 77712.4120 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 91 | **72 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 72 | **50 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 59 | **45 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 3 | **12 ✅** | -9 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 16 | **11 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | **2 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 3 | **2 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03074

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 52.0431 | **61.5385 ✅** | -9.4955 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 31.6500 | **44.9810 ✅** | -13.3310 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | **12 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 19 | **12 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2307.2301 | **1749.5675 ✅** | 557.6625 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 107771.9300 | **60714.7213 ✅** | 47057.2087 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 104 | **66 ✅** | 38 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 69 | **44 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 69 | **39 ✅** | 30 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **19 ✅** | 12 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 16 | **11 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03080

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 69.3800 | **72.9139 ✅** | -3.5338 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 53.2477 | **58.2441 ✅** | -4.9964 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 979.4744 | **838.7429 ✅** | 140.7315 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 24780.7023 | **20297.5786 ✅** | 4483.1237 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 49 | **42 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 35 | **30 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 18 | **16 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03105

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 76.4472 | **89.9911 ✅** | -13.5439 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 66.5915 | **83.3063 ✅** | -16.7147 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | **3 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 6 | **2 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 824.8989 | **530 ✅** | 294.8989 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 17538.9209 | **7014.7059 ✅** | 10524.2150 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 33 | **19 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 23 | **12 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 10 | **9 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **6 ✅** | 3 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 4 | **0 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p03106

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 76.8730 | **79.5071 ✅** | -2.6341 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **67.8516 ✅** | 59.1872 | 8.6644 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 7 | 7 | 0 | tie |
| Cognitive (sum) | Lower | 9 | 9 | 0 | tie |
| Halstead Volume | Lower | 825.0630 | **803.6328 ✅** | 21.4302 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **19251.4702 ✅** | 19829.6394 | -578.1692 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 35 | **30 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 24 | **23 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **7 ✅** | 2 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03108

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 49.2322 | **55.0019 ✅** | -5.7698 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 21.8492 | **27.8528 ✅** | -6.0035 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 17 | **13 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 10 | **7 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3318.6421 | **2675.1400 ✅** | 643.5021 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 144994.4923 | **112831.4603 ✅** | 32163.0320 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 107 | **85 ✅** | 22 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 81 | **66 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 53 | **45 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 8 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | **11 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 7 | **5 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p03115

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 63.4206 | **64.8537 ✅** | -1.4331 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **45.4583 ✅** | 39.5478 | 5.9105 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1727.5790 ✅** | 1728.0789 | -0.4999 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 61392.2586 | **58796.8322 ✅** | 2595.4264 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 59 | **54 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **42 ✅** | 44 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 29 | 29 | 0 | tie |
| Comments (CLOC) | Higher | **9 ✅** | 4 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p03123

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **62.2425 ✅** | 44.7989 | 17.4436 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **57.8152 ✅** | 13.4591 | 44.3561 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 19 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **12 ✅** | 22 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1329.7127 ✅** | 3793.1485 | -2463.4357 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **45431.8517 ✅** | 200257.9183 | -154826.0666 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **71 ✅** | 131 | -60 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **37 ✅** | 97 | -60 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **29 ✅** | 87 | -58 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **31 ✅** | 12 | 19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 23 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 4 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 5 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03142

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **52.5380 ✅** | 45.1536 | 7.3844 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **36.3560 ✅** | 22.7006 | 13.6554 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **24 ✅** | 31 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23 ✅** | 29 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2456.7661 ✅** | 3126.8670 | -670.1009 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **121886.3092 ✅** | 166212.5230 | -44326.2138 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **87 ✅** | 115 | -28 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **57 ✅** | 78 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **34 ✅** | 47 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 20 | 20 | 0 | tie |
| Blank Lines | Lower | **11 ✅** | 17 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 4 | 4 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03145

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **87.7973 ✅** | 81.1384 | 6.6589 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **87.3488 ✅** | 75.7852 | 11.5636 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **2 ✅** | 4 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **535 ✅** | 670.5629 | -135.5629 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **5825.5556 ✅** | 12740.6959 | -6915.1403 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **22 ✅** | 30 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **12 ✅** | 18 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | 6 | **7 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03157

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 46.4053 | **51.5379 ✅** | -5.1325 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 10.7082 | **22.2543 ✅** | -11.5461 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 29 | **24 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 60 | **40 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3461.3518 | **2873.6135 ✅** | 587.7383 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 228522.8674 | **133650.1376 ✅** | 94872.7298 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 106 | **88 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 88 | **68 ✅** | 20 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 66 | **48 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | **7 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 13 | 13 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 2 | 2 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03164

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 50.7094 | **51.9780 ✅** | -1.2686 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 29.1097 | **31.3894 ✅** | -2.2797 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 26 | **25 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 26 | **23 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2511.6669 | **2355.3531 ✅** | 156.3137 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 109257.5084 | **103525.9871 ✅** | 5731.5213 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 94 | **90 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 70 | **66 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 38 | **36 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 15 | 15 | 0 | tie |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 4 | 4 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03165

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **62.5437 ✅** | 58.9563 | 3.5874 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **43.1813 ✅** | 40.4388 | 2.7426 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 15 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **21 ✅** | 24 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1821.1283 ✅** | 1919.7394 | -98.6111 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **94091.6293 ✅** | 106665.5228 | -12573.8935 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **63 ✅** | 72 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **46 ✅** | 51 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 39 | **37 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | **12 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03173

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 58.1773 | **63.5211 ✅** | -5.3438 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 37.3472 | **45.8165 ✅** | -8.4692 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2058.5727 | **1787.2067 ✅** | 271.3660 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 82710.5120 | **75732.8838 ✅** | 6977.6282 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 76 | **58 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 54 | **39 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 33 | **30 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 9 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 3 | **1 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 6 | **2 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p03183

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **63.1708 ✅** | 57.4468 | 5.7240 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **45.5936 ✅** | 38.9991 | 6.5945 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **11 ✅** | 20 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1696.7294 ✅** | 1881.8226 | -185.0932 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **49128.0278 ✅** | 51647.8490 | -2519.8212 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **62 ✅** | 83 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **44 ✅** | 56 | -12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **26 ✅** | 37 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | **15 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 12 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | 1 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03194

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **74.5404 ✅** | 73.4012 | 1.1392 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 56.4545 | **58.6418 ✅** | -2.1873 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | 8 | 0 | tie |
| Cognitive (sum) | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 882.4241 | **872.0704 ✅** | 10.3536 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 28678.7824 | **26756.7070 ✅** | 1922.0755 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **39 ✅** | 42 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **28 ✅** | 30 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **25 ✅** | 26 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **6 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 7 | **6 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03206

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **93.2283 ✅** | 85.0521 | 8.1761 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 59.5119 | **70.1788 ✅** | -10.6670 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | **7 ✅** | 11 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **406.9970 ✅** | 530.3976 | -123.4006 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **3846.1219 ✅** | 6199.0225 | -2352.9006 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **16 ✅** | 23 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **13 ✅** | 18 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 0 | **2 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 3 | 3 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03209

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.0100 | **80.6180 ✅** | -5.6079 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 57.2131 | **61.3327 ✅** | -4.1197 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 6 | 6 | 0 | tie |
| Cognitive (sum) | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 955.0286 | **841.0710 ✅** | 113.9576 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 25480.1627 | **19746.8852 ✅** | 5733.2775 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 38 | **28 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 26 | **22 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 22 | **11 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **4 ✅** | 2 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 8 | **4 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03213

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **58.7690 ✅** | 46.4927 | 12.2763 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **47.7498 ✅** | 24.2071 | 23.5427 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **13 ✅** | 29 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **20 ✅** | 43 | -23 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1506.4117 ✅** | 2502.4097 | -995.9980 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **56361.3165 ✅** | 150053.0308 | -93691.7143 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **81 ✅** | 117 | -36 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **51 ✅** | 84 | -33 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **50 ✅** | 71 | -21 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **24 ✅** | 20 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **11 ✅** | 13 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 6 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03216

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **59.7086 ✅** | 58.0047 | 1.7039 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **46.2414 ✅** | 28.7240 | 17.5175 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **14 ✅** | 16 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 25 | **14 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1813.6186 | **1718.7449 ✅** | 94.8737 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 66429.0532 | **48749.8547 ✅** | 17679.1985 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **71 ✅** | 78 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **42 ✅** | 62 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **34 ✅** | 45 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **17 ✅** | 5 | 12 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03218

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.8963 | **69.4399 ✅** | -3.5436 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 44.4720 | **56.1055 ✅** | -11.6335 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 10 | **8 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1305.4228 | **1155.6636 ✅** | 149.7592 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 35937.5224 | **25774.8009 ✅** | 10162.7214 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 57 | **49 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 40 | **32 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 28 | **22 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 6 | **9 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 12 | **8 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p03268

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **86.8511 ✅** | 82.9485 | 3.9025 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **72.4191 ✅** | 68.9616 | 3.4576 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 3 | 3 | 0 | tie |
| Cognitive (sum) | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **614.0075 ✅** | 687.1023 | -73.0948 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 11973.1466 | **11149.7962 ✅** | 823.3504 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **22 ✅** | 27 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **15 ✅** | 18 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 2 | **3 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03277

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **48.8320 ✅** | 37.9669 | 10.8651 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **26.1379 ✅** | 14.8950 | 11.2430 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **18 ✅** | 27 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **15 ✅** | 57 | -42 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2814.7040 ✅** | 4398.7550 | -1584.0510 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **138948.9437 ✅** | 283435.9037 | -144486.9600 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **114 ✅** | 170 | -56 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **80 ✅** | 113 | -33 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **54 ✅** | 78 | -24 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 19 | **37 ✅** | -18 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **16 ✅** | 21 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 5 | 5 | 0 | tie |
| Closures | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03278

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 38.3216 | **60.8321 ✅** | -22.5105 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 11.0520 | **49.4261 ✅** | -38.3741 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 26 | **11 ✅** | 15 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 45 | **8 ✅** | 37 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 5289.5532 | **1530.2285 ✅** | 3759.3248 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 481084.8638 | **54188.0899 ✅** | 426896.7740 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 159 | **73 ✅** | 86 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 113 | **44 ✅** | 69 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 104 | **29 ✅** | 75 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **25 ✅** | 20 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 21 | **9 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **2 ✅** | 4 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 4 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03294

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 82.1984 | **84.5220 ✅** | -2.3236 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **75.6826 ✅** | 67.8604 | 7.8222 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | **607.8213 ✅** | 617.3437 | -9.5224 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **9377.8137 ✅** | 10053.8824 | -676.0688 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 29 | **25 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 17 | 17 | 0 | tie |
| LLOC | Lower | 13 | 13 | 0 | tie |
| Comments (CLOC) | Higher | **6 ✅** | 2 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03296

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **79.1145 ✅** | 72.0166 | 7.0979 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **64.9836 ✅** | 57.1430 | 7.8407 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **5 ✅** | 10 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 13 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **774.3513 ✅** | 1122.9866 | -348.6353 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **17406.0702 ✅** | 34313.4802 | -16907.4100 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **32 ✅** | 41 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **23 ✅** | 30 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **19 ✅** | 23 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **6 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03311

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 76.7815 | **79.6401 ✅** | -2.8587 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 68.4614 | **70.6843 ✅** | -2.2228 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | 806.4400 | **731.5638 ✅** | 74.8762 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 15580.4217 | **12802.3671 ✅** | 2778.0545 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 37 | **32 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 20 | **19 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 16 | **15 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **8 ✅** | 6 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **7 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03317

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 75.4560 | **76.7791 ✅** | -1.3232 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 70.2174 | **71.9128 ✅** | -1.6954 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 883.1796 | **878.6965 ✅** | 4.4831 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **17476.7661 ✅** | 19221.4858 | -1744.7197 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 39 | **36 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 23 | **22 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **11 ✅** | 10 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03318

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **67.4526 ✅** | 64.6557 | 2.7969 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **51.8520 ✅** | 47.6340 | 4.2180 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 12 | **8 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1130.5903 ✅** | 1332.9334 | -202.3431 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **41791.4641 ✅** | 49076.1851 | -7284.7211 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **55 ✅** | 62 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **36 ✅** | 41 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **30 ✅** | 32 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **10 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **10 ✅** | 12 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p03361

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **60.7760 ✅** | 60.7308 | 0.0452 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **41.2540 ✅** | 40.2752 | 0.9788 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **18 ✅** | 21 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **34 ✅** | 39 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1710 | **1586.5461 ✅** | 123.4539 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **63180 ✅** | 68528.5550 | -5348.5550 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 64 | **63 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **46 ✅** | 49 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **32 ✅** | 33 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **9 ✅** | 8 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 9 | **6 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03365

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 55.5902 | **56.3289 ✅** | -0.7387 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 25.3579 | **35.5572 ✅** | -10.1993 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 16 | **12 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **12 ✅** | 20 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2846.5701 | **2333.1361 ✅** | 513.4340 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 186417.2446 | **127217.3174 ✅** | 59199.9272 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **77 ✅** | 83 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 63 | **59 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 51 | **44 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 5 | **13 ✅** | -8 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | **1 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p03369

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **90.2073 ✅** | 83.1607 | 7.0466 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **78.3748 ✅** | 72.4162 | 5.9586 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | **414.5863 ✅** | 539.1110 | -124.5248 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **4698.6444 ✅** | 8779.8081 | -4081.1637 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **20 ✅** | 28 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **14 ✅** | 19 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **10 ✅** | 16 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 2 | **4 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03406

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **66.6474 ✅** | 42.4354 | 24.2121 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **42.5014 ✅** | 9.1736 | 33.3278 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 27 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **9 ✅** | 40 | -31 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1376.6255 ✅** | 4400.8539 | -3024.2283 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **39355.2947 ✅** | 384634.6291 | -345279.3344 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **52 ✅** | 129 | -77 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **41 ✅** | 106 | -65 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **24 ✅** | 89 | -65 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 4 | **10 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 14 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 3 | 3 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03408

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 60.4120 | **64.1758 ✅** | -3.7638 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 39.3506 | **47.2471 ✅** | -7.8965 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 15 | **12 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 19 | **18 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 1584.0926 | **1280.1538 ✅** | 303.9387 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 51235.4938 | **36964.4423 ✅** | 14271.0515 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 70 | **62 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 54 | **43 ✅** | 11 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 36 | **35 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 9 | **10 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 10 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |

## p03414

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **26.6384 ✅** | 24.3022 | 2.3362 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **-8.7068 ✅** | -17.1179 | 8.4110 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 48 | **47 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **104 ✅** | 107 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **7174.6786 ✅** | 8155.6621 | -980.9835 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **541596.2495 ✅** | 637999.8968 | -96403.6473 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **217 ✅** | 244 | -27 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **176 ✅** | 202 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **139 ✅** | 143 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **22 ✅** | 15 | 7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **20 ✅** | 29 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03434

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **74.7548 ✅** | 74.3130 | 0.4418 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 62.8393 | **65.2071 ✅** | -2.3678 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | **893.5940 ✅** | 900.8062 | -7.2122 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 20793.2450 | **17457.6245 ✅** | 3335.6205 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **40 ✅** | 41 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 25 | 25 | 0 | tie |
| LLOC | Lower | 20 | 20 | 0 | tie |
| Comments (CLOC) | Higher | 7 | **9 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03435

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.5592 ✅** | 75.8317 | 0.7275 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 49.1896 | **64.5345 ✅** | -15.3449 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1060.5239 | **922.6439 ✅** | 137.8800 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 36455.5089 | **30447.2501 ✅** | 6008.2588 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | **32 ✅** | 35 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 25 | 25 | 0 | tie |
| LLOC | Lower | 18 | 18 | 0 | tie |
| Comments (CLOC) | Higher | 1 | **6 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03448

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 76.3076 | 76.3076 | 0 | tie |
| MI (SEI) | Higher | 57.5096 | 57.5096 | 0 | tie |
| Cyclomatic (sum) | Lower | 9 | 9 | 0 | tie |
| Cognitive (sum) | Lower | 12 | 12 | 0 | tie |
| Halstead Volume | Lower | 921.5188 | 921.5188 | 0 | tie |
| Halstead Effort | Lower | 20273.4139 | 20273.4139 | 0 | tie |
| SLOC | Lower | 34 | 34 | 0 | tie |
| PLOC | Lower | 25 | 25 | 0 | tie |
| LLOC | Lower | 18 | 18 | 0 | tie |
| Comments (CLOC) | Higher | 3 | 3 | 0 | tie |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03454

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 42.9032 | **47.3040 ✅** | -4.4008 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 17.8255 | **22.8410 ✅** | -5.0155 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 25 | **24 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **21 ✅** | 49 | -28 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 3643.1194 | **3236.8272 ✅** | 406.2922 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 242631.7542 | **222262.1333 ✅** | 20369.6209 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 137 | **110 ✅** | 27 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 97 | **80 ✅** | 17 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 64 | 64 | 0 | tie |
| Comments (CLOC) | Higher | **22 ✅** | 16 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 18 | **14 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 10 | **2 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 13 | **1 ✅** | 12 | translation_with_validated_specs_GPT_5_2 |

## p03463

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 78.1878 | **85.4551 ✅** | -7.2673 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 57.7931 | **79.8257 ✅** | -22.0326 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | **3 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 5 | **2 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 830.2185 | **539.2711 ✅** | 290.9474 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 16965.3347 | **6173.2345 ✅** | 10792.1002 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 30 | **25 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 24 | **15 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 10 | 10 | 0 | tie |
| Comments (CLOC) | Higher | 2 | **5 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 3 | **0 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p03473

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **102.2958 ✅** | 96.1398 | 6.1560 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 72.0844 | **98.1746 ✅** | -26.0902 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 2 | 2 | 0 | tie |
| Cognitive (sum) | Lower | 0 | 0 | 0 | tie |
| Halstead Volume | Lower | **285.2933 ✅** | 290.0481 | -4.7549 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **2516.6940 ✅** | 2558.6389 | -41.9449 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **11 ✅** | 16 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 8 | 8 | 0 | tie |
| LLOC | Lower | 5 | 5 | 0 | tie |
| Comments (CLOC) | Higher | 0 | **4 ✅** | -4 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03478

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 74.7248 | **78.1489 ✅** | -3.4241 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 54.0022 | **56.6413 ✅** | -2.6390 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 8 | **7 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 923.4712 | **853.4359 ✅** | 70.0353 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 20580.2161 | **17037.1092 ✅** | 3543.1069 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 38 | **32 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 29 | **25 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 22 | **19 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **3 ✅** | 2 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03495

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 69.6692 | **69.7921 ✅** | -0.1229 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **59.8294 ✅** | 56.5119 | 3.3175 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 5 | 5 | 0 | tie |
| Halstead Volume | Lower | 1134.3875 | **1128.8126 ✅** | 5.5749 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **25788.4093 ✅** | 29599.9760 | -3811.5667 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 50 | **49 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **29 ✅** | 30 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 24 | **22 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 9 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **9 ✅** | 10 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03544

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **88.6040 ✅** | 87.2838 | 1.3203 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **83.7219 ✅** | 73.1452 | 10.5767 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **506.6527 ✅** | 540.5365 | -33.8839 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **8386.4347 ✅** | 10540.4624 | -2154.0278 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **21 ✅** | 22 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **13 ✅** | 15 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **12 ✅** | 13 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **4 ✅** | 2 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03562

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 19.6919 | **40.6452 ✅** | -20.9533 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | -8.8919 | **7.5528 ✅** | -16.4447 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 63 | **28 ✅** | 35 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 67 | **36 ✅** | 31 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 7197.5932 | **4404.8767 ✅** | 2792.7165 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 584529.2439 | **257093.5844 ✅** | 327435.6595 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 269 | **142 ✅** | 127 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 191 | **109 ✅** | 82 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 113 | **78 ✅** | 35 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **54 ✅** | 12 | 42 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 26 | **22 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 10 | **6 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 2 | **0 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 14 | **6 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |

## p03568

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 81.1474 | **82.7483 ✅** | -1.6009 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 61.6645 | **72.2674 ✅** | -10.6029 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 4 | 4 | 0 | tie |
| Halstead Volume | Lower | 711.7859 | **653.6213 ✅** | 58.1646 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 12347.9380 | **11289.8232 ✅** | 1058.1148 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 29 | **27 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 21 | **20 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 17 | **16 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 2 | **4 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03592

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 83.8273 | **84.4067 ✅** | -0.5794 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **74.7841 ✅** | 62.1306 | 12.6535 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 5 | 5 | 0 | tie |
| Cognitive (sum) | Lower | 6 | 6 | 0 | tie |
| Halstead Volume | Lower | **675.0516 ✅** | 685.7667 | -10.7151 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **12536.6717 ✅** | 12735.6665 | -198.9948 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 25 | **24 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
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
| MI (original) | Higher | **46.7873 ✅** | 46.2512 | 0.5361 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **20.4753 ✅** | 17.4595 | 3.0157 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **23 ✅** | 24 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **30 ✅** | 43 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **3083.2590 ✅** | 3450.6858 | -367.4268 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 181059.4634 | **179029.6995 ✅** | 2029.7639 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 117 | **115 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **88 ✅** | 93 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 66 | **62 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **15 ✅** | 12 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | **11 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 5 | **2 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |

## p03626

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 59.1445 | **67.9903 ✅** | -8.8458 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 41.1938 | **53.1001 ✅** | -11.9063 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | **9 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **11 ✅** | 15 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 1583.5069 | **1144.2453 ✅** | 439.2616 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 54951.9681 | **37256.6271 ✅** | 17695.3410 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 79 | **53 ✅** | 26 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 52 | **36 ✅** | 16 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 37 | **33 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **14 ✅** | 9 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | **9 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03632

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 82.6909 | **84.9727 ✅** | -2.2818 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 75.4191 | **76.7506 ✅** | -1.3315 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 4 | **2 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 776.9541 | **671.9361 ✅** | 105.0181 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 12607.8465 | **9071.1370 ✅** | 3536.7095 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 26 | **24 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 15 | **14 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 12 | **11 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **5 ✅** | 4 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | 6 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03646

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **65.1733 ✅** | 64.0304 | 1.1429 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **49.5368 ✅** | 47.6573 | 1.8795 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **12 ✅** | 14 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **23 ✅** | 29 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1300.7345 ✅** | 1334.6055 | -33.8710 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **42680.3499 ✅** | 45532.2898 | -2851.9399 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **58 ✅** | 60 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **44 ✅** | 45 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **29 ✅** | 31 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | 10 | 0 | tie |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03647

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 45.2754 | **60.2060 ✅** | -14.9306 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 21.8100 | **43.3834 ✅** | -21.5734 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 23 | **19 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 22 | **14 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 3042.1347 | **1511.3591 ✅** | 1530.7756 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 142250.2174 | **39230.5640 ✅** | 103019.6534 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 129 | **68 ✅** | 61 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 89 | **50 ✅** | 39 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 50 | **25 ✅** | 25 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **22 ✅** | 12 | 10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 19 | **10 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 4 | **1 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 6 | **2 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |

## p03701

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **74.0144 ✅** | 69.2486 | 4.7658 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 44.6331 | **52.9563 ✅** | -8.3232 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 11 | 11 | 0 | tie |
| Cognitive (sum) | Lower | **15 ✅** | 16 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **927.1038 ✅** | 1049.9730 | -122.8692 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **26886.0095 ✅** | 31394.1912 | -4508.1817 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **38 ✅** | 49 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **30 ✅** | 33 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **15 ✅** | 20 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 1 | **7 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **7 ✅** | 9 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03709

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **49.7482 ✅** | 31.7387 | 18.0095 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **37.7331 ✅** | -0.4582 | 38.1914 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **19 ✅** | 38 | -19 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **14 ✅** | 49 | -35 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **2453.5220 ✅** | 7120.2668 | -4666.7449 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **110558.7034 ✅** | 752184.9886 | -641626.2852 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **111 ✅** | 183 | -72 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **58 ✅** | 147 | -89 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **37 ✅** | 126 | -89 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **39 ✅** | 22 | 17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **14 ✅** | 23 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **4 ✅** | 6 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | **2 ✅** | 4 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03719

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 88.4428 | **90.9879 ✅** | -2.5451 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **83.5911 ✅** | 55.9742 | 27.6169 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 3 | 3 | 0 | tie |
| Halstead Volume | Lower | 500 | **495.4196 ✅** | 4.5804 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 6750 | **6270.1547 ✅** | 479.8453 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 21 | **18 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 14 | 14 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | **4 ✅** | 0 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03722

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 49.8906 | **50.2860 ✅** | -0.3954 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **29.2344 ✅** | 25.0333 | 4.2011 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 23 | **20 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 50 | **32 ✅** | 18 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **2378.1618 ✅** | 2672.1632 | -294.0014 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **102260.9579 ✅** | 114618.7457 | -12357.7878 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 105 | **103 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | **78 ✅** | 80 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **53 ✅** | 57 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **19 ✅** | 13 | 6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **9 ✅** | 11 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03736

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **62.3797 ✅** | 41.5798 | 20.7999 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **48.4529 ✅** | 11.8456 | 36.6073 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **10 ✅** | 23 | -13 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **12 ✅** | 32 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1789.4963 ✅** | 4210.5431 | -2421.0468 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **61668.7960 ✅** | 277895.8467 | -216227.0508 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **64 ✅** | 146 | -82 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **41 ✅** | 107 | -66 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **32 ✅** | 82 | -50 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 14 | **17 ✅** | -3 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **11 ✅** | 22 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 5 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03762

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **68.1476 ✅** | 65.4853 | 2.6623 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **57.3619 ✅** | 48.6839 | 8.6781 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 8 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1429.0581 ✅** | 1630.9528 | -201.8947 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **34801.7667 ✅** | 46019.3159 | -11217.5492 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **51 ✅** | 56 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **31 ✅** | 38 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **22 ✅** | 28 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **12 ✅** | 9 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 3 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **1 ✅** | 2 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03766

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 65.2356 | **66.7511 ✅** | -1.5155 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **47.4658 ✅** | 42.1418 | 5.3240 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 6 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1588.9536 ✅** | 1683.4638 | -94.5102 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **61855.6926 ✅** | 75755.8719 | -13900.1793 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 59 | **52 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 37 | **36 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 34 | **33 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **9 ✅** | 4 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | **12 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 2 | 2 | 0 | tie |

## p03797

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **85.1094 ✅** | 81.4382 | 3.6713 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **69.2179 ✅** | 62.1859 | 7.0321 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **5 ✅** | 6 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **4 ✅** | 7 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **599.0871 ✅** | 643.9533 | -44.8662 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **10783.5685 ✅** | 12879.0660 | -2095.4975 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **24 ✅** | 29 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **19 ✅** | 22 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **8 ✅** | 10 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 2 | 2 | 0 | tie |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03808

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **58.7665 ✅** | 56.6985 | 2.0681 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **40.3955 ✅** | 37.3879 | 3.0077 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 14 | 14 | 0 | tie |
| Cognitive (sum) | Lower | 15 | **12 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1758.5314 ✅** | 2065.6722 | -307.1408 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **76872.9441 ✅** | 100672.8279 | -23799.8838 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **76 ✅** | 82 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **50 ✅** | 56 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **38 ✅** | 46 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 13 | **14 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 13 | 13 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 6 | **5 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03815

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **87.0577 ✅** | 82.5168 | 4.5409 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **72.3596 ✅** | 68.7461 | 3.6135 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **4 ✅** | 7 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **491.5424 ✅** | 625.5158 | -133.9734 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **7938.4099 ✅** | 12510.3160 | -4571.9061 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **23 ✅** | 27 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **16 ✅** | 19 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 7 | 7 | 0 | tie |
| Comments (CLOC) | Higher | 2 | **3 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03817

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **86.7174 ✅** | 75.2905 | 11.4269 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **71.8685 ✅** | 66.5612 | 5.3073 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **4 ✅** | 10 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **3 ✅** | 8 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **524.7928 ✅** | 758.1585 | -233.3657 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **7222.1479 ✅** | 13065.5978 | -5843.4500 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **23 ✅** | 38 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **17 ✅** | 27 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **8 ✅** | 9 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 2 | **8 ✅** | -6 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 3 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03837

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 51.5864 | **54.0352 ✅** | -2.4488 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 27.0431 | **29.4302 ✅** | -2.3871 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 20 | **15 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 33 | **30 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2766.7671 | **2384.4301 ✅** | 382.3370 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 116639.7284 | **79544.5879 ✅** | 37095.1405 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 94 | **91 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 68 | **67 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 55 | **50 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | **12 ✅** | 11 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 14 | **13 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03838

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **85.0313 ✅** | 69.0063 | 16.0250 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **85.6995 ✅** | 69.1387 | 16.5608 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **3 ✅** | 6 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **2 ✅** | 4 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **585.0582 ✅** | 856.7291 | -271.6709 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **8320.8281 ✅** | 17819.9649 | -9499.1368 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **25 ✅** | 57 | -32 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **12 ✅** | 21 | -9 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **9 ✅** | 16 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **29 ✅** | -21 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03845

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.6696 ✅** | 76.2000 | 0.4696 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **60.6529 ✅** | 59.6451 | 1.0078 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | 1072.3128 | 1072.3128 | 0 | tie |
| Halstead Effort | Lower | 21446.2560 | 21446.2560 | 0 | tie |
| SLOC | Lower | **34 ✅** | 35 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 25 | 25 | 0 | tie |
| LLOC | Lower | 21 | 21 | 0 | tie |
| Comments (CLOC) | Higher | 4 | 4 | 0 | tie |
| Blank Lines | Lower | 7 | 7 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03865

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 84.5164 | **84.7982 ✅** | -0.2818 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | **82.1136 ✅** | 81.4522 | 0.6614 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | **3 ✅** | 4 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **486.2570 ✅** | 585.4121 | -99.1551 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **7502.2509 ✅** | 10058.4441 | -2556.1932 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 27 | **25 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 15 | 15 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | **7 ✅** | 6 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03899

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **54.0055 ✅** | 53.6118 | 0.3937 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **38.6526 ✅** | 36.7027 | 1.9499 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **31 ✅** | 33 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | 2291.2883 | **2215.9572 ✅** | 75.3311 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | **74466.8695 ✅** | 76002.6169 | -1535.7474 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **95 ✅** | 97 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **58 ✅** | 60 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | 43 | 43 | 0 | tie |
| Comments (CLOC) | Higher | **24 ✅** | 22 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **13 ✅** | 16 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03901

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **82.8451 ✅** | 82.2337 | 0.6114 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | 69.3407 | **71.4232 ✅** | -2.0825 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 4 | 4 | 0 | tie |
| Cognitive (sum) | Lower | 2 | 2 | 0 | tie |
| Halstead Volume | Lower | 754.2508 | 754.2508 | 0 | tie |
| Halstead Effort | Lower | 15740.8863 | 15740.8863 | 0 | tie |
| SLOC | Lower | **26 ✅** | 27 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | 18 | 18 | 0 | tie |
| LLOC | Lower | 9 | 9 | 0 | tie |
| Comments (CLOC) | Higher | 3 | **4 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 2 | 2 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p03914

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.7975 ✅** | 57.7916 | 19.0059 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **81.6540 ✅** | 44.3415 | 37.3125 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **3 ✅** | 10 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **2 ✅** | 22 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **526.1548 ✅** | 2157.7981 | -1631.6433 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **6213.6375 ✅** | 94279.1764 | -88065.5389 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **43 ✅** | 80 | -37 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **12 ✅** | 50 | -38 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **8 ✅** | 40 | -32 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **25 ✅** | 21 | 4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 14 | -8 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03929

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 61.0651 | **61.2717 ✅** | -0.2067 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 38.6246 | **39.4232 ✅** | -0.7986 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 12 | 12 | 0 | tie |
| Cognitive (sum) | Lower | **13 ✅** | 14 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1595.3815 ✅** | 1757.3943 | -162.0128 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | 55397.5219 | **52864.3204 ✅** | 2533.2015 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 70 | **67 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 50 | **49 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 41 | **39 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 8 | 8 | 0 | tie |
| Blank Lines | Lower | 12 | **10 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | **1 ✅** | 3 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 4 | **3 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03932

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 48.5155 | **53.4810 ✅** | -4.9655 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 26.8849 | **36.4050 ✅** | -9.5200 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 23 | **21 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 63 | **42 ✅** | 21 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2837.7202 | **2180.0971 ✅** | 657.6230 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 132475.2027 | **121177.0665 ✅** | 11298.1362 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 108 | **89 ✅** | 19 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 71 | **58 ✅** | 13 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 56 | **42 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 19 | 19 | 0 | tie |
| Blank Lines | Lower | 18 | **13 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p03951

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 78.0052 | **80.0774 ✅** | -2.0722 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 56.0581 | **73.8062 ✅** | -17.7480 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 6 | **4 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 8 | **3 ✅** | 5 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 833.1883 | **742.4827 ✅** | 90.7056 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 17160.4750 | **16334.6194 ✅** | 825.8556 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 33 | **31 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 26 | **19 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 21 | **15 ✅** | 6 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 2 | **7 ✅** | -5 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 5 | 5 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 0 | 0 | 0 | tie |

## p03977

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 53.1977 | **56.4263 ✅** | -3.2287 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 33.4364 | **36.9320 ✅** | -3.4956 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 25 | **18 ✅** | 7 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | **24 ✅** | 27 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **1998 ✅** | 2048.2435 | -50.2435 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **107606.5714 ✅** | 118798.1248 | -11191.5534 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | 88 | **79 ✅** | 9 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 62 | **58 ✅** | 4 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | **42 ✅** | 53 | -11 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **15 ✅** | 13 | 2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Functions | Lower | 2 | **1 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | **0 ✅** | 1 | translation_with_validated_specs_GPT_5_2 |

## p03996

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **57.6647 ✅** | 49.8250 | 7.8397 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **45.6265 ✅** | 28.8121 | 16.8144 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **17 ✅** | 18 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 23 | **21 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **1446.5567 ✅** | 2674.1130 | -1227.5563 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **50956.1272 ✅** | 131714.9202 | -80758.7931 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **83 ✅** | 109 | -26 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **49 ✅** | 74 | -25 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **36 ✅** | 56 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **23 ✅** | 20 | 3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **11 ✅** | 16 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Exits | Lower | 2 | 2 | 0 | tie |

## p04005

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **77.3497 ✅** | 75.3931 | 1.9566 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **64.5917 ✅** | 58.5705 | 6.0211 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **7 ✅** | 8 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **823.9246 ✅** | 961.0743 | -137.1497 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **18909.0700 ✅** | 21528.0641 | -2618.9941 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **34 ✅** | 36 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **23 ✅** | 25 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **19 ✅** | 21 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **5 ✅** | 4 | 1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **6 ✅** | 7 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | 1 | 1 | 0 | tie |

## p04013

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | 60.0714 | **62.5132 ✅** | -2.4418 | translation_with_validated_specs_GPT_5_2 |
| MI (SEI) | Higher | 41.4647 | **52.0011 ✅** | -10.5364 | translation_with_validated_specs_GPT_5_2 |
| Cyclomatic (sum) | Lower | 13 | **11 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Cognitive (sum) | Lower | 27 | **13 ✅** | 14 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | 2022.3814 | **1590 ✅** | 432.3814 | translation_with_validated_specs_GPT_5_2 |
| Halstead Effort | Lower | 89883.6192 | **53795.0000 ✅** | 36088.6192 | translation_with_validated_specs_GPT_5_2 |
| SLOC | Lower | 68 | **65 ✅** | 3 | translation_with_validated_specs_GPT_5_2 |
| PLOC | Lower | 48 | **40 ✅** | 8 | translation_with_validated_specs_GPT_5_2 |
| LLOC | Lower | 40 | **30 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Comments (CLOC) | Higher | 11 | **18 ✅** | -7 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 9 | 9 | 0 | tie |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p04020

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **76.2288 ✅** | 68.5652 | 7.6636 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **69.9582 ✅** | 57.0837 | 12.8745 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **7 ✅** | 9 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Volume | Lower | **855.4067 ✅** | 1154.9122 | -299.5055 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **27703.5119 ✅** | 43659.8067 | -15956.2948 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **36 ✅** | 51 | -15 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **22 ✅** | 32 | -10 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **21 ✅** | 28 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 9 | **11 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **6 ✅** | 9 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 2 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p04022

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **33.3490 ✅** | 20.8610 | 12.4881 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **1.5967 ✅** | -20.5121 | 22.1088 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **39 ✅** | 59 | -20 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 88 | **86 ✅** | 2 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **4520.0619 ✅** | 7104.8683 | -2584.8064 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **342291.9629 ✅** | 683728.2362 | -341436.2733 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **189 ✅** | 266 | -77 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **147 ✅** | 224 | -77 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **110 ✅** | 174 | -64 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | **22 ✅** | 17 | 5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Blank Lines | Lower | **20 ✅** | 26 | -6 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | **2 ✅** | 6 | -4 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **2 ✅** | 18 | -16 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p04024

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **38.8107 ✅** | 36.4123 | 2.3984 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **2.5081 ✅** | -0.9071 | 3.4152 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **38 ✅** | 39 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 66 | **56 ✅** | 10 | translation_with_validated_specs_GPT_5_2 |
| Halstead Volume | Lower | **4607.5186 ✅** | 4746.7126 | -139.1940 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **318371.9823 ✅** | 365333.1908 | -46961.2085 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **136 ✅** | 154 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **108 ✅** | 125 | -17 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **83 ✅** | 101 | -18 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 8 | **9 ✅** | -1 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | 20 | 20 | 0 | tie |
| Functions | Lower | 4 | 4 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **3 ✅** | 6 | -3 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

## p04030

| Metric | Better Direction | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS | translation_with_validated_specs_GPT_5_2 | Delta (A-B) | Better Pipeline |
|---|---|---:|---:|---:|---|
| MI (original) | Higher | **81.4424 ✅** | 76.6317 | 4.8108 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| MI (SEI) | Higher | **80.3929 ✅** | 73.4386 | 6.9543 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cyclomatic (sum) | Lower | **5 ✅** | 7 | -2 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Cognitive (sum) | Lower | 7 | 7 | 0 | tie |
| Halstead Volume | Lower | **494.8981 ✅** | 616.9158 | -122.0177 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Halstead Effort | Lower | **6309.9504 ✅** | 10138.8766 | -3828.9262 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| SLOC | Lower | **32 ✅** | 39 | -7 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| PLOC | Lower | **18 ✅** | 23 | -5 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| LLOC | Lower | **11 ✅** | 12 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Comments (CLOC) | Higher | 10 | **12 ✅** | -2 | translation_with_validated_specs_GPT_5_2 |
| Blank Lines | Lower | **4 ✅** | 5 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |
| Functions | Lower | 1 | 1 | 0 | tie |
| Closures | Lower | 0 | 0 | 0 | tie |
| Exits | Lower | **0 ✅** | 1 | -1 | full_translation_loop_GPT_5_2_using_GPT_5_2_SPECS |

