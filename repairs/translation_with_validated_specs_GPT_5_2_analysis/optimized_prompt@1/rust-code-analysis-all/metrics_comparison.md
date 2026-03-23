# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/code_translation_specs_ASE/r_code/full_translation_loop_all_metrics.json`

## p00004

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.0036 ✅** | 66.5912 | 64.2468 | +0.4124 | Higher | +2.7568 | Higher | +2.3444 | Higher |
| MI (SEI) | 48.0555 | **55.7667 ✅** | 40.4423 | -7.7112 | Lower | +7.6132 | Higher | +15.3244 | Higher |
| Cyclomatic (sum) | 15 | 15 | 22 | 0 | Same | -7 | Lower | -7 | Lower |
| Cognitive (sum) | 6 | 6 | 8 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 1272.0476 | **1218.6637 ✅** | 1585.9236 | +53.3838 | Higher | -313.8760 | Lower | -367.2599 | Lower |
| Halstead Effort | 31703.3394 | **31532.9242 ✅** | 43740.7958 | +170.4152 | Higher | -12037.4564 | Lower | -12207.8716 | Lower |
| SLOC | 50 | 52 | 50 | -2 | Lower | 0 | Same | +2 | Higher |
| PLOC | 38 | **36 ✅** | 42 | +2 | Higher | -4 | Lower | -6 | Lower |
| LLOC | 16 | **14 ✅** | 17 | +2 | Higher | -1 | Lower | -3 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 4 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 6 | 5 | **4 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 5 | 0 | Same | -5 | Lower | -5 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00010

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.1400 | **68.6058 ✅** | 65.9225 | -4.4658 | Lower | -1.7825 | Lower | +2.6833 | Higher |
| MI (SEI) | 37.6619 | **50.3237 ✅** | 38.4657 | -12.6618 | Lower | -0.8038 | Lower | +11.8580 | Higher |
| Cyclomatic (sum) | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 2208.0565 | **1886.2699 ✅** | 2097.4048 | +321.7866 | Higher | +110.6517 | Higher | -211.1349 | Lower |
| Halstead Effort | 80830.6385 | **55670.4516 ✅** | 79753.8160 | +25160.1869 | Higher | +1076.8225 | Higher | -24083.3644 | Lower |
| SLOC | 56 | **46 ✅** | 51 | +10 | Higher | +5 | Higher | -5 | Lower |
| PLOC | 40 | **30 ✅** | 38 | +10 | Higher | +2 | Higher | -8 | Lower |
| LLOC | 34 | **23 ✅** | 32 | +11 | Higher | +2 | Higher | -9 | Lower |
| Comments (CLOC) | 4 | **6 ✅** | 3 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 12 | 10 | 10 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.3959 ✅** | 80.0694 | 75.9835 | +0.3266 | Higher | +4.4124 | Higher | +4.0859 | Higher |
| MI (SEI) | **76.8745 ✅** | 72.9466 | 68.4006 | +3.9279 | Higher | +8.4739 | Higher | +4.5460 | Higher |
| Cyclomatic (sum) | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | **574.7682 ✅** | 612.0217 | 730.4165 | -37.2535 | Lower | -155.6482 | Lower | -118.3947 | Lower |
| Halstead Effort | **9170.1659 ✅** | 10265.2736 | 13439.6629 | -1095.1077 | Lower | -4269.4970 | Lower | -3174.3893 | Lower |
| SLOC | 33 | 33 | 39 | 0 | Same | -6 | Lower | -6 | Lower |
| PLOC | **19 ✅** | 21 | 25 | -2 | Lower | -6 | Lower | -4 | Lower |
| LLOC | **16 ✅** | 18 | 18 | -2 | Lower | -2 | Lower | 0 | Same |
| Comments (CLOC) | 9 | 7 | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00015

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.7846 ✅** | 52.9570 | 50.3788 | +0.8276 | Higher | +3.4058 | Higher | +2.5782 | Higher |
| MI (SEI) | **31.1370 ✅** | 22.2086 | 22.8491 | +8.9284 | Higher | +8.2879 | Higher | -0.6405 | Lower |
| Cyclomatic (sum) | 20 | **19 ✅** | 23 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 26 | **19 ✅** | 36 | +7 | Higher | -10 | Lower | -17 | Lower |
| Halstead Volume | **2307.1838 ✅** | 2634.2597 | 2520.4463 | -327.0759 | Lower | -213.2625 | Lower | +113.8134 | Higher |
| Halstead Effort | **110895.2887 ✅** | 131605.4629 | 139254.6575 | -20710.1742 | Lower | -28359.3688 | Lower | -7649.1946 | Lower |
| SLOC | **87 ✅** | 89 | 100 | -2 | Lower | -13 | Lower | -11 | Lower |
| PLOC | **61 ✅** | 64 | 76 | -3 | Lower | -15 | Lower | -12 | Lower |
| LLOC | 41 | 41 | 56 | 0 | Same | -15 | Lower | -15 | Lower |
| Comments (CLOC) | **12 ✅** | 6 | 10 | +6 | Higher | +2 | Higher | -4 | Lower |
| Blank Lines | 14 | 19 | 14 | -5 | Lower | 0 | Same | +5 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00023

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.1493 | **67.3746 ✅** | 47.5256 | -1.2253 | Lower | +18.6237 | Higher | +19.8490 | Higher |
| MI (SEI) | **47.4373 ✅** | 45.6283 | 14.6191 | +1.8091 | Higher | +32.8183 | Higher | +31.0092 | Higher |
| Cyclomatic (sum) | 10 | **8 ✅** | 30 | +2 | Higher | -20 | Lower | -22 | Lower |
| Cognitive (sum) | 12 | **10 ✅** | 39 | +2 | Higher | -27 | Lower | -29 | Lower |
| Halstead Volume | 1559.7839 | **1517.7219 ✅** | 3409.1421 | +42.0620 | Higher | -1849.3582 | Lower | -1891.4202 | Lower |
| Halstead Effort | 40975.9451 | **37331.7430 ✅** | 196750.1149 | +3644.2021 | Higher | -155774.1698 | Lower | -159418.3719 | Lower |
| SLOC | 53 | **51 ✅** | 98 | +2 | Higher | -45 | Lower | -47 | Lower |
| PLOC | 37 | **36 ✅** | 79 | +1 | Higher | -42 | Lower | -43 | Lower |
| LLOC | **23 ✅** | 24 | 45 | -1 | Lower | -22 | Lower | -21 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | **9 ✅** | 10 | 13 | -1 | Lower | -4 | Lower | -3 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p00024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.1737 ✅** | 82.1104 | 74.5587 | +2.0634 | Higher | +9.6150 | Higher | +7.5516 | Higher |
| MI (SEI) | **77.6603 ✅** | 77.0857 | 65.8671 | +0.5746 | Higher | +11.7932 | Higher | +11.2186 | Higher |
| Cyclomatic (sum) | 5 | **3 ✅** | 8 | +2 | Higher | -3 | Lower | -5 | Lower |
| Cognitive (sum) | 3 | **1 ✅** | 11 | +2 | Higher | -8 | Lower | -10 | Lower |
| Halstead Volume | **558.9053 ✅** | 581.3942 | 752.4580 | -22.4890 | Lower | -193.5528 | Lower | -171.0638 | Lower |
| Halstead Effort | **7444.6182 ✅** | 7482.2911 | 14919.4267 | -37.6729 | Lower | -7474.8085 | Lower | -7437.1356 | Lower |
| SLOC | **26 ✅** | 30 | 41 | -4 | Lower | -15 | Lower | -11 | Lower |
| PLOC | 16 | **15 ✅** | 27 | +1 | Higher | -11 | Lower | -12 | Lower |
| LLOC | **9 ✅** | 11 | 13 | -2 | Lower | -4 | Lower | -2 | Lower |
| Comments (CLOC) | 5 | 7 | **9 ✅** | -2 | Lower | -4 | Lower | -2 | Lower |
| Blank Lines | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.9813 ✅** | 59.8862 | 61.7969 | +7.0950 | Higher | +5.1844 | Higher | -1.9106 | Lower |
| MI (SEI) | **47.1544 ✅** | 32.2778 | 39.7972 | +14.8766 | Higher | +7.3572 | Higher | -7.5194 | Lower |
| Cyclomatic (sum) | **13 ✅** | 17 | 18 | -4 | Lower | -5 | Lower | -1 | Lower |
| Cognitive (sum) | **17 ✅** | 24 | 23 | -7 | Lower | -6 | Lower | +1 | Higher |
| Halstead Volume | **1164 ✅** | 1469.4577 | 1405.1903 | -305.4577 | Lower | -241.1903 | Lower | +64.2674 | Higher |
| Halstead Effort | **35684.9143 ✅** | 58568.3873 | 56803.7545 | -22883.4730 | Lower | -21118.8402 | Lower | +1764.6328 | Higher |
| SLOC | **53 ✅** | 72 | 64 | -19 | Lower | -11 | Lower | +8 | Higher |
| PLOC | **39 ✅** | 54 | 47 | -15 | Lower | -8 | Lower | +7 | Higher |
| LLOC | **25 ✅** | 39 | 33 | -14 | Lower | -8 | Lower | +6 | Higher |
| Comments (CLOC) | 6 | 5 | **7 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 10 | 13 | 10 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00071

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.8871 ✅** | 50.7816 | 52.8486 | +6.1055 | Higher | +4.0385 | Higher | -2.0670 | Lower |
| MI (SEI) | **36.1193 ✅** | 26.8109 | 32.7690 | +9.3084 | Higher | +3.3503 | Higher | -5.9581 | Lower |
| Cyclomatic (sum) | **16 ✅** | 21 | 19 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | 22 | 30 | **20 ✅** | -8 | Lower | +2 | Higher | +10 | Higher |
| Halstead Volume | **2510.5806 ✅** | 2989.8935 | 3104.0667 | -479.3129 | Lower | -593.4861 | Lower | -114.1732 | Lower |
| Halstead Effort | **95496.8017 ✅** | 129468.0296 | 129278.6296 | -33971.2278 | Lower | -33781.8278 | Lower | +189.4000 | Higher |
| SLOC | **74 ✅** | 95 | 85 | -21 | Lower | -11 | Lower | +10 | Higher |
| PLOC | **51 ✅** | 66 | 58 | -15 | Lower | -7 | Lower | +8 | Higher |
| LLOC | 40 | 46 | 40 | -6 | Lower | 0 | Same | +6 | Higher |
| Comments (CLOC) | 11 | 13 | **15 ✅** | -2 | Lower | -4 | Lower | -2 | Lower |
| Blank Lines | 13 | 17 | **12 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.4028 | 63.9986 | **66.1355 ✅** | -1.5958 | Lower | -3.7326 | Lower | -2.1368 | Lower |
| MI (SEI) | 42.7631 | **47.8871 ✅** | 38.6329 | -5.1241 | Lower | +4.1302 | Higher | +9.2543 | Higher |
| Cyclomatic (sum) | 18 | 19 | **12 ✅** | -1 | Lower | +6 | Higher | +7 | Higher |
| Cognitive (sum) | 11 | 11 | **5 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| Halstead Volume | 1452.3896 | 1411.5155 | **1275.5115 ✅** | +40.8741 | Higher | +176.8781 | Higher | +136.0040 | Higher |
| Halstead Effort | 49925.8930 | 40679.0921 | **37582.0352 ✅** | +9246.8008 | Higher | +12343.8577 | Higher | +3097.0569 | Higher |
| SLOC | 61 | 55 | 55 | +6 | Higher | +6 | Higher | 0 | Same |
| PLOC | 49 | **42 ✅** | 43 | +7 | Higher | +6 | Higher | -1 | Lower |
| LLOC | 17 | 17 | 17 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 8 | **9 ✅** | 3 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00077

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.2968 ✅** | 71.7269 | 66.8525 | +4.5699 | Higher | +9.4443 | Higher | +4.8744 | Higher |
| MI (SEI) | **67.6057 ✅** | 58.6959 | 57.1991 | +8.9098 | Higher | +10.4067 | Higher | +1.4968 | Higher |
| Cyclomatic (sum) | **6 ✅** | 8 | 8 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **11 ✅** | 19 | 19 | -8 | Lower | -8 | Lower | 0 | Same |
| Halstead Volume | **745.6809 ✅** | 906.3699 | 960.6136 | -160.6891 | Lower | -214.9327 | Lower | -54.2436 | Lower |
| Halstead Effort | **17430.2902 ✅** | 22529.7665 | 27838.5807 | -5099.4763 | Lower | -10408.2905 | Lower | -5308.8142 | Lower |
| SLOC | **38 ✅** | 46 | 61 | -8 | Lower | -23 | Lower | -15 | Lower |
| PLOC | **26 ✅** | 32 | 36 | -6 | Lower | -10 | Lower | -4 | Lower |
| LLOC | **21 ✅** | 24 | 30 | -3 | Lower | -9 | Lower | -6 | Lower |
| Comments (CLOC) | 8 | 8 | **16 ✅** | 0 | Same | -8 | Lower | -8 | Lower |
| Blank Lines | 6 | 6 | 11 | 0 | Same | -5 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.2350 ✅** | 74.6490 | 71.5965 | +0.5860 | Higher | +3.6385 | Higher | +3.0525 | Higher |
| MI (SEI) | 62.2016 | **62.9920 ✅** | 62.4681 | -0.7904 | Lower | -0.2665 | Lower | +0.5239 | Higher |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **772.0879 ✅** | 798.6496 | 869.1539 | -26.5618 | Lower | -97.0660 | Lower | -70.5042 | Lower |
| Halstead Effort | 17325.6514 | **17232.4018 ✅** | 18779.9315 | +93.2496 | Higher | -1454.2801 | Lower | -1547.5297 | Lower |
| SLOC | **39 ✅** | 40 | 47 | -1 | Lower | -8 | Lower | -7 | Lower |
| PLOC | 26 | **25 ✅** | 28 | +1 | Higher | -2 | Lower | -3 | Lower |
| LLOC | 17 | **16 ✅** | 19 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | 6 | 7 | **11 ✅** | -1 | Lower | -5 | Lower | -4 | Lower |
| Blank Lines | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.0388 ✅** | 51.7930 | 48.6920 | +2.2459 | Higher | +5.3468 | Higher | +3.1009 | Higher |
| MI (SEI) | 24.9219 | **28.6895 ✅** | 20.8652 | -3.7675 | Lower | +4.0568 | Higher | +7.8243 | Higher |
| Cyclomatic (sum) | **18 ✅** | 20 | 20 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 33 | **25 ✅** | 39 | +8 | Higher | -6 | Lower | -14 | Lower |
| Halstead Volume | **2086.6543 ✅** | 2335.2568 | 2796.7396 | -248.6026 | Lower | -710.0853 | Lower | -461.4828 | Lower |
| Halstead Effort | **65729.6092 ✅** | 81085.3066 | 123493.5326 | -15355.6973 | Lower | -57763.9234 | Lower | -42408.2260 | Lower |
| SLOC | **91 ✅** | 98 | 112 | -7 | Lower | -21 | Lower | -14 | Lower |
| PLOC | **72 ✅** | 77 | 86 | -5 | Lower | -14 | Lower | -9 | Lower |
| LLOC | **44 ✅** | 46 | 55 | -2 | Lower | -11 | Lower | -9 | Lower |
| Comments (CLOC) | 7 | **14 ✅** | 12 | -7 | Lower | -5 | Lower | +2 | Higher |
| Blank Lines | 12 | 12 | 14 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.5874 | **63.6294 ✅** | 53.6684 | -6.0420 | Lower | +3.9190 | Higher | +9.9611 | Higher |
| MI (SEI) | 36.8351 | **43.5406 ✅** | 25.5038 | -6.7056 | Lower | +11.3313 | Higher | +18.0369 | Higher |
| Cyclomatic (sum) | 15 | **12 ✅** | 18 | +3 | Higher | -3 | Lower | -6 | Lower |
| Cognitive (sum) | 22 | **9 ✅** | 43 | +13 | Higher | -21 | Lower | -34 | Lower |
| Halstead Volume | 1665.7431 | **1352.8382 ✅** | 2094 | +312.9050 | Higher | -428.2569 | Lower | -741.1618 | Lower |
| Halstead Effort | 61275.5515 | **34722.8464 ✅** | 104234.6667 | +26552.7051 | Higher | -42959.1152 | Lower | -69511.8202 | Lower |
| SLOC | 82 | **63 ✅** | 93 | +19 | Higher | -11 | Lower | -30 | Lower |
| PLOC | 58 | **47 ✅** | 70 | +11 | Higher | -12 | Lower | -23 | Lower |
| LLOC | 46 | **37 ✅** | 56 | +9 | Higher | -10 | Lower | -19 | Lower |
| Comments (CLOC) | **12 ✅** | 8 | 8 | +4 | Higher | +4 | Higher | 0 | Same |
| Blank Lines | 13 | **11 ✅** | 15 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00118

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.2628 | **53.5205 ✅** | 43.7720 | -0.2577 | Lower | +9.4908 | Higher | +9.7486 | Higher |
| MI (SEI) | **31.6634 ✅** | 22.9541 | 15.6875 | +8.7093 | Higher | +15.9759 | Higher | +7.2667 | Higher |
| Cyclomatic (sum) | 24 | 24 | 32 | 0 | Same | -8 | Lower | -8 | Lower |
| Cognitive (sum) | 35 | **24 ✅** | 44 | +11 | Higher | -9 | Lower | -20 | Lower |
| Halstead Volume | **2670 ✅** | 2746.7041 | 4009.6850 | -76.7041 | Lower | -1339.6850 | Lower | -1262.9808 | Lower |
| Halstead Effort | **158021.8421 ✅** | 164802.2476 | 326740.4252 | -6780.4055 | Lower | -168718.5831 | Lower | -161938.1776 | Lower |
| SLOC | 81 | **79 ✅** | 114 | +2 | Higher | -33 | Lower | -35 | Lower |
| PLOC | **60 ✅** | 64 | 89 | -4 | Lower | -29 | Lower | -25 | Lower |
| LLOC | 51 | **41 ✅** | 69 | +10 | Higher | -18 | Lower | -28 | Lower |
| Comments (CLOC) | 12 | 5 | **13 ✅** | +7 | Higher | -1 | Lower | -8 | Lower |
| Blank Lines | **10 ✅** | 11 | 13 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00124

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.8657 | 66.8657 | 63.5760 | 0 | Same | +3.2896 | Higher | +3.2896 | Higher |
| MI (SEI) | **46.6822 ✅** | 42.4273 | 41.8275 | +4.2549 | Higher | +4.8546 | Higher | +0.5997 | Higher |
| Cyclomatic (sum) | 10 | 10 | 11 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 5 | 5 | 11 | 0 | Same | -6 | Lower | -6 | Lower |
| Halstead Volume | 1359.0516 | 1359.0516 | 1428.6093 | 0 | Same | -69.5577 | Lower | -69.5577 | Lower |
| Halstead Effort | 29918.5500 | 29918.5500 | 35126.9819 | 0 | Same | -5208.4319 | Lower | -5208.4319 | Lower |
| SLOC | 53 | 53 | 63 | 0 | Same | -10 | Lower | -10 | Lower |
| PLOC | 40 | 40 | 45 | 0 | Same | -5 | Lower | -5 | Lower |
| LLOC | 26 | 26 | 29 | 0 | Same | -3 | Lower | -3 | Lower |
| Comments (CLOC) | 6 | 4 | **7 ✅** | +2 | Higher | -1 | Lower | -3 | Lower |
| Blank Lines | **10 ✅** | 11 | 12 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00136

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.9053 | **70.4823 ✅** | 59.4659 | -8.5770 | Lower | +2.4393 | Higher | +11.0163 | Higher |
| MI (SEI) | 33.7158 | **46.4770 ✅** | 27.7164 | -12.7612 | Lower | +5.9994 | Higher | +18.7606 | Higher |
| Cyclomatic (sum) | 12 | **8 ✅** | 15 | +4 | Higher | -3 | Lower | -7 | Lower |
| Cognitive (sum) | 9 | **8 ✅** | 10 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 1794.4727 | **1322.5020 ✅** | 2178 | +471.9707 | Higher | -383.5273 | Lower | -855.4980 | Lower |
| Halstead Effort | 50220.9852 | **26979.0411 ✅** | 79289.5714 | +23241.9441 | Higher | -29068.5863 | Lower | -52310.5303 | Lower |
| SLOC | 64 | **44 ✅** | 67 | +20 | Higher | -3 | Lower | -23 | Lower |
| PLOC | 49 | **35 ✅** | 52 | +14 | Higher | -3 | Lower | -17 | Lower |
| LLOC | 45 | **31 ✅** | 45 | +14 | Higher | 0 | Same | -14 | Lower |
| Comments (CLOC) | **4 ✅** | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 11 | **6 ✅** | 13 | +5 | Higher | -2 | Lower | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00137

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.3018 | 69.4726 | **75.2017 ✅** | +1.8292 | Higher | -3.8999 | Lower | -5.7291 | Lower |
| MI (SEI) | 59.2652 | 50.2923 | **65.6770 ✅** | +8.9729 | Higher | -6.4118 | Lower | -15.3847 | Lower |
| Cyclomatic (sum) | 11 | 13 | **6 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Cognitive (sum) | 7 | 12 | **4 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Halstead Volume | 1062.7460 | 1120.8195 | **848.9143 ✅** | -58.0735 | Lower | +213.8317 | Higher | +271.9052 | Higher |
| Halstead Effort | 26568.6488 | 25778.8485 | **17598.6465 ✅** | +789.8003 | Higher | +8970.0023 | Higher | +8180.2020 | Higher |
| SLOC | 43 | 46 | **39 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| PLOC | 30 | 34 | **25 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| LLOC | 21 | **17 ✅** | 22 | +4 | Higher | -1 | Lower | -5 | Lower |
| Comments (CLOC) | 8 | 5 | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 2 | 5 | **1 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p00146

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **47.5777 ✅** | 43.2882 | 47.1248 | +4.2895 | Higher | +0.4529 | Higher | -3.8366 | Lower |
| MI (SEI) | 18.0831 | 13.7619 | **19.1101 ✅** | +4.3212 | Higher | -1.0270 | Lower | -5.3482 | Lower |
| Cyclomatic (sum) | 22 | 22 | 22 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 38 | **32 ✅** | 39 | +6 | Higher | -1 | Lower | -7 | Lower |
| Halstead Volume | **3765.2748 ✅** | 4139.1224 | 3765.8486 | -373.8476 | Lower | -0.5738 | Lower | +373.2738 | Higher |
| Halstead Effort | 248037.4772 | **239880.9576 ✅** | 245156.7461 | +8156.5195 | Higher | +2880.7311 | Higher | -5275.7884 | Lower |
| SLOC | **106 ✅** | 134 | 109 | -28 | Lower | -3 | Lower | +25 | Higher |
| PLOC | 85 | 100 | **82 ✅** | -15 | Lower | +3 | Higher | +18 | Higher |
| LLOC | 70 | 82 | **68 ✅** | -12 | Lower | +2 | Higher | +14 | Higher |
| Comments (CLOC) | 10 | **15 ✅** | 12 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | **11 ✅** | 19 | 15 | -8 | Lower | -4 | Lower | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p00156

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **37.1248 ✅** | 32.6989 | 33.7727 | +4.4259 | Higher | +3.3521 | Higher | -1.0738 | Lower |
| MI (SEI) | **1.9924 ✅** | -2.8767 | -3.2298 | +4.8691 | Higher | +5.2222 | Higher | +0.3531 | Higher |
| Cyclomatic (sum) | **37 ✅** | 43 | 42 | -6 | Lower | -5 | Lower | +1 | Higher |
| Cognitive (sum) | **39 ✅** | 45 | 63 | -6 | Lower | -24 | Lower | -18 | Lower |
| Halstead Volume | **4908.1135 ✅** | 5552.5175 | 5570.2157 | -644.4041 | Lower | -662.1022 | Lower | -17.6982 | Lower |
| Halstead Effort | **307897.7100 ✅** | 365244.6029 | 439512.9100 | -57346.8929 | Lower | -131615.2000 | Lower | -74268.3071 | Lower |
| SLOC | **150 ✅** | 174 | 165 | -24 | Lower | -15 | Lower | +9 | Higher |
| PLOC | **121 ✅** | 140 | 140 | -19 | Lower | -19 | Lower | 0 | Same |
| LLOC | 88 | **85 ✅** | 118 | +3 | Higher | -30 | Lower | -33 | Lower |
| Comments (CLOC) | 11 | **14 ✅** | 11 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 21 | 20 | **15 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| Functions | 5 | 7 | **4 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00167

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.1291 ✅** | 67.1766 | 69.7097 | +3.9525 | Higher | +1.4195 | Higher | -2.5330 | Lower |
| MI (SEI) | 52.6226 | 48.9013 | **53.4178 ✅** | +3.7213 | Higher | -0.7953 | Lower | -4.5165 | Lower |
| Cyclomatic (sum) | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **996.6582 ✅** | 1105.4886 | 1049.7573 | -108.8305 | Lower | -53.0991 | Lower | +55.7314 | Higher |
| Halstead Effort | **24332.2061 ✅** | 27968.8628 | 25569.0873 | -3636.6567 | Lower | -1236.8812 | Lower | +2399.7755 | Higher |
| SLOC | **45 ✅** | 54 | 49 | -9 | Lower | -4 | Lower | +5 | Higher |
| PLOC | **32 ✅** | 39 | 34 | -7 | Lower | -2 | Lower | +5 | Higher |
| LLOC | **21 ✅** | 26 | 25 | -5 | Lower | -4 | Lower | +1 | Higher |
| Comments (CLOC) | 5 | 7 | 7 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00187

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **45.8590 ✅** | 38.5290 | 38.2748 | +7.3299 | Higher | +7.5842 | Higher | +0.2543 | Higher |
| MI (SEI) | **13.4777 ✅** | 3.5643 | -4.3115 | +9.9134 | Higher | +17.7891 | Higher | +7.8757 | Higher |
| Cyclomatic (sum) | **19 ✅** | 28 | 33 | -9 | Lower | -14 | Lower | -5 | Lower |
| Cognitive (sum) | **19 ✅** | 22 | 28 | -3 | Lower | -9 | Lower | -6 | Lower |
| Halstead Volume | **4770.0956 ✅** | 4936.8299 | 5449.6156 | -166.7343 | Lower | -679.5200 | Lower | -512.7857 | Lower |
| Halstead Effort | 274344.9579 | **224473.3879 ✅** | 393734.7248 | +49871.5700 | Higher | -119389.7669 | Lower | -169261.3369 | Lower |
| SLOC | **114 ✅** | 156 | 143 | -42 | Lower | -29 | Lower | +13 | Higher |
| PLOC | **89 ✅** | 119 | 122 | -30 | Lower | -33 | Lower | -3 | Lower |
| LLOC | **55 ✅** | 57 | 81 | -2 | Lower | -26 | Lower | -24 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 4 | -3 | Lower | +5 | Higher | +8 | Higher |
| Blank Lines | **16 ✅** | 25 | 17 | -9 | Lower | -1 | Lower | +8 | Higher |
| Functions | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 7 | **2 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |

## p00190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 29.9597 | **31.0947 ✅** | 28.3747 | -1.1349 | Lower | +1.5851 | Higher | +2.7200 | Higher |
| MI (SEI) | **-2.3477 ✅** | -4.0425 | -11.6080 | +1.6948 | Higher | +9.2603 | Higher | +7.5655 | Higher |
| Cyclomatic (sum) | **36 ✅** | 37 | 37 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 62 | **37 ✅** | 50 | +25 | Higher | +12 | Higher | -13 | Lower |
| Halstead Volume | 6533.9185 | **6487.6500 ✅** | 7263.4127 | +46.2685 | Higher | -729.4942 | Lower | -775.7627 | Lower |
| Halstead Effort | 443839.7498 | **378573.4601 ✅** | 648456.4859 | +65266.2897 | Higher | -204616.7361 | Lower | -269883.0258 | Lower |
| SLOC | 216 | **199 ✅** | 227 | +17 | Higher | -11 | Lower | -28 | Lower |
| PLOC | 176 | **161 ✅** | 199 | +15 | Higher | -23 | Lower | -38 | Lower |
| LLOC | 84 | **77 ✅** | 130 | +7 | Higher | -46 | Lower | -53 | Lower |
| Comments (CLOC) | **28 ✅** | 19 | 15 | +9 | Higher | +13 | Higher | +4 | Higher |
| Blank Lines | 28 | 32 | **27 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 7 | 7 | 11 | 0 | Same | -4 | Lower | -4 | Lower |

## p00203

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.8166 ✅** | 47.6368 | 36.0980 | +1.1798 | Higher | +12.7186 | Higher | +11.5389 | Higher |
| MI (SEI) | **28.1940 ✅** | 25.5695 | 9.2146 | +2.6245 | Higher | +18.9794 | Higher | +16.3549 | Higher |
| Cyclomatic (sum) | 28 | **26 ✅** | 41 | +2 | Higher | -13 | Lower | -15 | Lower |
| Cognitive (sum) | 67 | **66 ✅** | 84 | +1 | Higher | -17 | Lower | -18 | Lower |
| Halstead Volume | **2343.6732 ✅** | 2701.8442 | 4018.6881 | -358.1710 | Lower | -1675.0149 | Lower | -1316.8439 | Lower |
| Halstead Effort | **143344.1199 ✅** | 191380.6288 | 291165.3277 | -48036.5089 | Lower | -147821.2078 | Lower | -99784.6989 | Lower |
| SLOC | **105 ✅** | 111 | 161 | -6 | Lower | -56 | Lower | -50 | Lower |
| PLOC | **77 ✅** | 78 | 121 | -1 | Lower | -44 | Lower | -43 | Lower |
| LLOC | **54 ✅** | 62 | 78 | -8 | Lower | -24 | Lower | -16 | Lower |
| Comments (CLOC) | 19 | 19 | **25 ✅** | 0 | Same | -6 | Lower | -6 | Lower |
| Blank Lines | **9 ✅** | 15 | 16 | -6 | Lower | -7 | Lower | -1 | Lower |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 5 | 0 | Same | -5 | Lower | -5 | Lower |

## p00208

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.3048 ✅** | 64.2644 | 64.2755 | +9.0404 | Higher | +9.0294 | Higher | -0.0111 | Lower |
| MI (SEI) | **58.3185 ✅** | 51.3756 | 45.7816 | +6.9429 | Higher | +12.5369 | Higher | +5.5940 | Higher |
| Cyclomatic (sum) | **9 ✅** | 12 | 11 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | 15 | 17 | **11 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| Halstead Volume | **789.8694 ✅** | 1140 | 1248.8106 | -350.1306 | Lower | -458.9412 | Lower | -108.8106 | Lower |
| Halstead Effort | **18956.8646 ✅** | 31920 | 41447.5928 | -12963.1354 | Lower | -22490.7282 | Lower | -9527.5928 | Lower |
| SLOC | **43 ✅** | 64 | 63 | -21 | Lower | -20 | Lower | +1 | Higher |
| PLOC | **30 ✅** | 39 | 44 | -9 | Lower | -14 | Lower | -5 | Lower |
| LLOC | **23 ✅** | 25 | 39 | -2 | Lower | -16 | Lower | -14 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 9 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | **7 ✅** | 11 | 11 | -4 | Lower | -4 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00244

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.4067 | **51.9654 ✅** | 29.6905 | -5.5587 | Lower | +16.7162 | Higher | +22.2749 | Higher |
| MI (SEI) | 17.4802 | **24.1749 ✅** | -19.3987 | -6.6947 | Lower | +36.8789 | Higher | +43.5736 | Higher |
| Cyclomatic (sum) | 22 | **17 ✅** | 41 | +5 | Higher | -19 | Lower | -24 | Lower |
| Cognitive (sum) | **28 ✅** | 32 | 47 | -4 | Lower | -19 | Lower | -15 | Lower |
| Halstead Volume | 3658.7894 | **3036.7882 ✅** | 6393.6877 | +622.0012 | Higher | -2734.8983 | Lower | -3356.8996 | Lower |
| Halstead Effort | 192622.6448 | **122230.7231 ✅** | 514077.0828 | +70391.9217 | Higher | -321454.4380 | Lower | -391846.3597 | Lower |
| SLOC | 115 | **93 ✅** | 206 | +22 | Higher | -91 | Lower | -113 | Lower |
| PLOC | 86 | **69 ✅** | 179 | +17 | Higher | -93 | Lower | -110 | Lower |
| LLOC | 60 | **49 ✅** | 122 | +11 | Higher | -62 | Lower | -73 | Lower |
| Comments (CLOC) | **12 ✅** | 9 | 3 | +3 | Higher | +9 | Higher | +6 | Higher |
| Blank Lines | 17 | **15 ✅** | 24 | +2 | Higher | -7 | Lower | -9 | Lower |
| Functions | 2 | **1 ✅** | 9 | +1 | Higher | -7 | Lower | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 7 | +1 | Higher | -6 | Lower | -7 | Lower |

## p00252

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.1653 | **82.2685 ✅** | 77.8052 | -11.1031 | Lower | -6.6398 | Lower | +4.4633 | Higher |
| MI (SEI) | 47.9642 | 43.9039 | **63.6988 ✅** | +4.0603 | Higher | -15.7346 | Lower | -19.7949 | Lower |
| Cyclomatic (sum) | 26 | **9 ✅** | 20 | +17 | Higher | +6 | Higher | -11 | Lower |
| Cognitive (sum) | 9 | **2 ✅** | 6 | +7 | Higher | +3 | Higher | -4 | Lower |
| Halstead Volume | 1167.9463 | **866.8060 ✅** | 979.6931 | +301.1404 | Higher | +188.2533 | Higher | -112.8871 | Lower |
| Halstead Effort | 38834.2157 | **14898.2275 ✅** | 28050.1591 | +23935.9882 | Higher | +10784.0566 | Higher | -13151.9316 | Lower |
| SLOC | 34 | **24 ✅** | 26 | +10 | Higher | +8 | Higher | -2 | Lower |
| PLOC | 29 | 20 | 20 | +9 | Higher | +9 | Higher | 0 | Same |
| LLOC | 12 | **9 ✅** | 12 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 2 | 0 | **3 ✅** | +2 | Higher | -1 | Lower | -3 | Lower |
| Blank Lines | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |
| Exits | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |

## p00256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **40.8373 ✅** | 34.5180 | 40.3832 | +6.3193 | Higher | +0.4540 | Higher | -5.8653 | Lower |
| MI (SEI) | **14.1629 ✅** | 4.7481 | 8.1096 | +9.4148 | Higher | +6.0533 | Higher | -3.3616 | Lower |
| Cyclomatic (sum) | **22 ✅** | 29 | 25 | -7 | Lower | -3 | Lower | +4 | Higher |
| Cognitive (sum) | 52 | **38 ✅** | 59 | +14 | Higher | -7 | Lower | -21 | Lower |
| Halstead Volume | **4299.1412 ✅** | 5711.9985 | 4749.1531 | -1412.8574 | Lower | -450.0119 | Lower | +962.8454 | Higher |
| Halstead Effort | **239044.0275 ✅** | 410288.6744 | 366001.3992 | -171244.6468 | Lower | -126957.3717 | Lower | +44287.2751 | Higher |
| SLOC | 154 | 188 | **147 ✅** | -34 | Lower | +7 | Higher | +41 | Higher |
| PLOC | **105 ✅** | 132 | 112 | -27 | Lower | -7 | Lower | +20 | Higher |
| LLOC | 110 | **107 ✅** | 130 | +3 | Higher | -20 | Lower | -23 | Lower |
| Comments (CLOC) | 24 | **27 ✅** | 14 | -3 | Lower | +10 | Higher | +13 | Higher |
| Blank Lines | 27 | 29 | **22 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p00261

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.9724 ✅** | 66.1546 | 65.1845 | +3.8178 | Higher | +4.7879 | Higher | +0.9701 | Higher |
| MI (SEI) | **60.7489 ✅** | 57.2009 | 42.7073 | +3.5480 | Higher | +18.0415 | Higher | +14.4935 | Higher |
| Cyclomatic (sum) | **8 ✅** | 11 | 10 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **11 ✅** | 14 | 14 | -3 | Lower | -3 | Lower | 0 | Same |
| Halstead Volume | **1043.1805 ✅** | 1125.7478 | 1211.8245 | -82.5673 | Lower | -168.6440 | Lower | -86.0767 | Lower |
| Halstead Effort | 23673.4664 | 22210.6993 | **19950.7687 ✅** | +1462.7671 | Higher | +3722.6978 | Higher | +2259.9306 | Higher |
| SLOC | **49 ✅** | 58 | 61 | -9 | Lower | -12 | Lower | -3 | Lower |
| PLOC | **33 ✅** | 38 | 45 | -5 | Lower | -12 | Lower | -7 | Lower |
| LLOC | 16 | **14 ✅** | 19 | +2 | Higher | -3 | Lower | -5 | Lower |
| Comments (CLOC) | 12 | **16 ✅** | 6 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | **7 ✅** | 8 | 10 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00270

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **34.1510 ✅** | 33.8786 | 21.6376 | +0.2724 | Higher | +12.5134 | Higher | +12.2410 | Higher |
| MI (SEI) | -3.9833 | **-1.1827 ✅** | -24.8217 | -2.8005 | Lower | +20.8384 | Higher | +23.6389 | Higher |
| Cyclomatic (sum) | 35 | **33 ✅** | 53 | +2 | Higher | -18 | Lower | -20 | Lower |
| Cognitive (sum) | **55 ✅** | 62 | 66 | -7 | Lower | -11 | Lower | -4 | Lower |
| Halstead Volume | **5773.2612 ✅** | 6197.0690 | 8775.3383 | -423.8077 | Lower | -3002.0771 | Lower | -2578.2694 | Lower |
| Halstead Effort | **368289.0800 ✅** | 415892.1847 | 802489.5608 | -47603.1047 | Lower | -434200.4809 | Lower | -386597.3761 | Lower |
| SLOC | **176 ✅** | 180 | 258 | -4 | Lower | -82 | Lower | -78 | Lower |
| PLOC | 144 | **140 ✅** | 226 | +4 | Higher | -82 | Lower | -86 | Lower |
| LLOC | 107 | 107 | 167 | 0 | Same | -60 | Lower | -60 | Lower |
| Comments (CLOC) | 11 | **16 ✅** | 9 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | **22 ✅** | 25 | 25 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 5 | **2 ✅** | 10 | +3 | Higher | -5 | Lower | -8 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p00271

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **92.2658 ✅** | 90.2015 | 89.3969 | +2.0643 | Higher | +2.8688 | Higher | +0.8046 | Higher |
| MI (SEI) | 87.2723 | 82.9700 | **87.8754 ✅** | +4.3024 | Higher | -0.6030 | Lower | -4.9054 | Lower |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **405 ✅** | 433.8179 | 435 | -28.8179 | Lower | -30 | Lower | -1.1821 | Lower |
| Halstead Effort | **4288.2353 ✅** | 4699.6939 | 4398.3333 | -411.4586 | Lower | -110.0980 | Lower | +301.3605 | Higher |
| SLOC | **18 ✅** | 20 | 21 | -2 | Lower | -3 | Lower | -1 | Lower |
| PLOC | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **7 ✅** | 8 | 8 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 3 | 3 | **5 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00283

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.9853 ✅** | 48.5005 | 47.5701 | +0.4848 | Higher | +1.4152 | Higher | +0.9304 | Higher |
| MI (SEI) | **22.0022 ✅** | 20.6463 | 13.2498 | +1.3559 | Higher | +8.7524 | Higher | +7.3965 | Higher |
| Cyclomatic (sum) | **22 ✅** | 23 | 25 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | **24 ✅** | 33 | 30 | -9 | Lower | -6 | Lower | +3 | Higher |
| Halstead Volume | **2789.5340 ✅** | 3106.9866 | 3027.2059 | -317.4526 | Lower | -237.6719 | Lower | +79.7807 | Higher |
| Halstead Effort | **123067.6765 ✅** | 127518.1029 | 168514.4609 | -4450.4264 | Lower | -45446.7843 | Lower | -40996.3579 | Lower |
| SLOC | 107 | **105 ✅** | 109 | +2 | Higher | -2 | Lower | -4 | Lower |
| PLOC | 85 | **83 ✅** | 88 | +2 | Higher | -3 | Lower | -5 | Lower |
| LLOC | 45 | **38 ✅** | 53 | +7 | Higher | -8 | Lower | -15 | Lower |
| Comments (CLOC) | **12 ✅** | 11 | 6 | +1 | Higher | +6 | Higher | +5 | Higher |
| Blank Lines | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **0 ✅** | 6 | +3 | Higher | -3 | Lower | -6 | Lower |

## p00288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.3611 | **23.9190 ✅** | 5.3500 | -1.5579 | Lower | +17.0112 | Higher | +18.5690 | Higher |
| MI (SEI) | -21.6448 | **-13.8514 ✅** | -45.7032 | -7.7934 | Lower | +24.0583 | Higher | +31.8517 | Higher |
| Cyclomatic (sum) | 45 | **42 ✅** | 76 | +3 | Higher | -31 | Lower | -34 | Lower |
| Cognitive (sum) | 84 | **75 ✅** | 110 | +9 | Higher | -26 | Lower | -35 | Lower |
| Halstead Volume | 9226 | **8879.4333 ✅** | 13691.3826 | +346.5667 | Higher | -4465.3826 | Lower | -4811.9493 | Lower |
| Halstead Effort | 839365.4348 | **710396.1591 ✅** | 1472183.9290 | +128969.2757 | Higher | -632818.4942 | Lower | -761787.7699 | Lower |
| SLOC | 272 | **261 ✅** | 441 | +11 | Higher | -169 | Lower | -180 | Lower |
| PLOC | 222 | **206 ✅** | 375 | +16 | Higher | -153 | Lower | -169 | Lower |
| LLOC | 162 | **147 ✅** | 252 | +15 | Higher | -90 | Lower | -105 | Lower |
| Comments (CLOC) | 14 | **25 ✅** | 16 | -11 | Lower | -2 | Lower | +9 | Higher |
| Blank Lines | 37 | **34 ✅** | 51 | +3 | Higher | -14 | Lower | -17 | Lower |
| Functions | 6 | **5 ✅** | 19 | +1 | Higher | -13 | Lower | -14 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 4 | 4 | 14 | 0 | Same | -10 | Lower | -10 | Lower |

## p00292

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.6054 ✅** | 84.4745 | 66.7534 | +0.1309 | Higher | +17.8519 | Higher | +17.7211 | Higher |
| MI (SEI) | **78.5998 ✅** | 72.0388 | 60.1316 | +6.5610 | Higher | +18.4682 | Higher | +11.9072 | Higher |
| Cyclomatic (sum) | 3 | 3 | 10 | 0 | Same | -7 | Lower | -7 | Lower |
| Cognitive (sum) | 1 | 1 | 8 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | **634.9992 ✅** | 651.1817 | 994.2714 | -16.1824 | Lower | -359.2722 | Lower | -343.0897 | Lower |
| Halstead Effort | **8081.8084 ✅** | 8930.4915 | 23690.4283 | -848.6831 | Lower | -15608.6199 | Lower | -14759.9368 | Lower |
| SLOC | 25 | 25 | 59 | 0 | Same | -34 | Lower | -34 | Lower |
| PLOC | **15 ✅** | 16 | 33 | -1 | Lower | -18 | Lower | -17 | Lower |
| LLOC | **11 ✅** | 12 | 22 | -1 | Lower | -11 | Lower | -10 | Lower |
| Comments (CLOC) | 5 | 3 | **19 ✅** | +2 | Higher | -14 | Lower | -16 | Lower |
| Blank Lines | 6 | 6 | 9 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00315

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.2778 | 42.8613 | **53.0103 ✅** | +9.4165 | Higher | -0.7325 | Lower | -10.1490 | Lower |
| MI (SEI) | 24.8760 | 16.0352 | **27.0245 ✅** | +8.8407 | Higher | -2.1485 | Lower | -10.9893 | Lower |
| Cyclomatic (sum) | **16 ✅** | 26 | 18 | -10 | Lower | -2 | Lower | +8 | Higher |
| Cognitive (sum) | **19 ✅** | 51 | 20 | -32 | Lower | -1 | Lower | +31 | Higher |
| Halstead Volume | **3310.5331 ✅** | 4137.4006 | 3387.2888 | -826.8674 | Lower | -76.7557 | Lower | +750.1118 | Higher |
| Halstead Effort | 221162.0062 | 248544.9373 | **209027.2302 ✅** | -27382.9311 | Lower | +12134.7760 | Higher | +39517.7071 | Higher |
| SLOC | 90 | 130 | **83 ✅** | -40 | Lower | +7 | Higher | +47 | Higher |
| PLOC | 68 | 98 | **62 ✅** | -30 | Lower | +6 | Higher | +36 | Higher |
| LLOC | 53 | 84 | **52 ✅** | -31 | Lower | +1 | Higher | +32 | Higher |
| Comments (CLOC) | 9 | **18 ✅** | 9 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 13 | 14 | **12 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.0681 | **40.9509 ✅** | 22.3874 | -8.8828 | Lower | +9.6807 | Higher | +18.5635 | Higher |
| MI (SEI) | **5.4645 ✅** | 4.5137 | -13.8120 | +0.9508 | Higher | +19.2765 | Higher | +18.3257 | Higher |
| Cyclomatic (sum) | 35 | **28 ✅** | 52 | +7 | Higher | -17 | Lower | -24 | Lower |
| Cognitive (sum) | 77 | **32 ✅** | 89 | +45 | Higher | -12 | Lower | -57 | Lower |
| Halstead Volume | 5440.3639 | **4644.0555 ✅** | 7659.6697 | +796.3084 | Higher | -2219.3058 | Lower | -3015.6142 | Lower |
| Halstead Effort | 340868.2031 | **205886.4595 ✅** | 694673.1218 | +134981.7436 | Higher | -353804.9187 | Lower | -488786.6623 | Lower |
| SLOC | 204 | **137 ✅** | 261 | +67 | Higher | -57 | Lower | -124 | Lower |
| PLOC | 143 | **108 ✅** | 206 | +35 | Higher | -63 | Lower | -98 | Lower |
| LLOC | 125 | **56 ✅** | 165 | +69 | Higher | -40 | Lower | -109 | Lower |
| Comments (CLOC) | **39 ✅** | 8 | 28 | +31 | Higher | +11 | Higher | -20 | Lower |
| Blank Lines | 23 | **21 ✅** | 29 | +2 | Higher | -6 | Lower | -8 | Lower |
| Functions | **3 ✅** | 7 | 8 | -4 | Lower | -5 | Lower | -1 | Lower |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **6 ✅** | 8 | 13 | -2 | Lower | -7 | Lower | -5 | Lower |

## p00320

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.2278 | 46.0892 | **46.9036 ✅** | +0.1387 | Higher | -0.6758 | Lower | -0.8144 | Lower |
| MI (SEI) | 13.7380 | **21.2837 ✅** | 12.7295 | -7.5457 | Lower | +1.0085 | Higher | +8.5542 | Higher |
| Cyclomatic (sum) | 25 | 34 | **20 ✅** | -9 | Lower | +5 | Higher | +14 | Higher |
| Cognitive (sum) | 23 | 22 | 22 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 3702.9352 | **3224.4113 ✅** | 3733.1134 | +478.5238 | Higher | -30.1782 | Lower | -508.7020 | Lower |
| Halstead Effort | 265399.4611 | **241185.9681 ✅** | 286852.4317 | +24213.4930 | Higher | -21452.9706 | Lower | -45666.4636 | Lower |
| SLOC | 111 | **103 ✅** | 114 | +8 | Higher | -3 | Lower | -11 | Lower |
| PLOC | 92 | **80 ✅** | 92 | +12 | Higher | 0 | Same | -12 | Lower |
| LLOC | 56 | **46 ✅** | 63 | +10 | Higher | -7 | Lower | -17 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 7 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 12 | **9 ✅** | 15 | +3 | Higher | -3 | Lower | -6 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00329

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.6947 | **45.9313 ✅** | 37.1387 | -6.2367 | Lower | +2.5559 | Higher | +8.7926 | Higher |
| MI (SEI) | 4.3911 | **20.6401 ✅** | -2.2903 | -16.2490 | Lower | +6.6814 | Higher | +22.9303 | Higher |
| Cyclomatic (sum) | 26 | **25 ✅** | 30 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | **28 ✅** | 44 | 31 | -16 | Lower | -3 | Lower | +13 | Higher |
| Halstead Volume | 4579.1894 | **3074.8736 ✅** | 5052.2190 | +1504.3158 | Higher | -473.0296 | Lower | -1977.3454 | Lower |
| Halstead Effort | 248490.4044 | **207323.3498 ✅** | 387198.7484 | +41167.0546 | Higher | -138708.3440 | Lower | -179875.3986 | Lower |
| SLOC | 153 | **120 ✅** | 164 | +33 | Higher | -11 | Lower | -44 | Lower |
| PLOC | 126 | **91 ✅** | 141 | +35 | Higher | -15 | Lower | -50 | Lower |
| LLOC | 90 | **68 ✅** | 98 | +22 | Higher | -8 | Lower | -30 | Lower |
| Comments (CLOC) | 11 | **17 ✅** | 8 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 16 | **13 ✅** | 16 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 6 | **2 ✅** | 8 | +4 | Higher | -2 | Lower | -6 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 8 | **4 ✅** | 7 | +4 | Higher | +1 | Higher | -3 | Lower |

## p00336

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.8108 | 67.8384 | **75.9739 ✅** | +4.9724 | Higher | -3.1631 | Lower | -8.1355 | Lower |
| MI (SEI) | **61.5611 ✅** | 57.3323 | 56.7227 | +4.2288 | Higher | +4.8384 | Higher | +0.6096 | Higher |
| Cyclomatic (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 1150.5023 | 1214.3191 | **1122.0269 ✅** | -63.8169 | Lower | +28.4753 | Higher | +92.2922 | Higher |
| Halstead Effort | 36688.2392 | 42826.4338 | **36250.1014 ✅** | -6138.1946 | Lower | +438.1378 | Higher | +6576.3324 | Higher |
| SLOC | 41 | 54 | **34 ✅** | -13 | Lower | +7 | Higher | +20 | Higher |
| PLOC | 26 | 30 | **24 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| LLOC | 21 | 25 | **19 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Comments (CLOC) | 8 | **13 ✅** | 3 | -5 | Lower | +5 | Higher | +10 | Higher |
| Blank Lines | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00378

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.1398 ✅** | 77.0016 | 76.4909 | +1.1381 | Higher | +1.6489 | Higher | +0.5107 | Higher |
| MI (SEI) | 56.5263 | 49.1284 | **62.8956 ✅** | +7.3979 | Higher | -6.3693 | Lower | -13.7672 | Lower |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **893.5940 ✅** | 920.8124 | 928.1232 | -27.2184 | Lower | -34.5292 | Lower | -7.3108 | Lower |
| Halstead Effort | 23233.4440 | 23867.4578 | **21926.9100 ✅** | -634.0138 | Lower | +1306.5340 | Higher | +1940.5478 | Higher |
| SLOC | **32 ✅** | 34 | 35 | -2 | Lower | -3 | Lower | -1 | Lower |
| PLOC | 26 | 27 | 26 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 22 | 23 | 22 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 2 | 1 | **5 ✅** | +1 | Higher | -3 | Lower | -4 | Lower |
| Blank Lines | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00428

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.1709 ✅** | 60.1949 | 59.4951 | +7.9760 | Higher | +8.6759 | Higher | +0.6998 | Higher |
| MI (SEI) | **49.9081 ✅** | 33.2648 | 33.9187 | +16.6432 | Higher | +15.9894 | Higher | -0.6539 | Lower |
| Cyclomatic (sum) | **12 ✅** | 17 | 17 | -5 | Lower | -5 | Lower | 0 | Same |
| Cognitive (sum) | **9 ✅** | 23 | 26 | -14 | Lower | -17 | Lower | -3 | Lower |
| Halstead Volume | **1317.8692 ✅** | 1654.6757 | 1808.8749 | -336.8065 | Lower | -491.0056 | Lower | -154.1992 | Lower |
| Halstead Effort | **35846.0433 ✅** | 62415.3414 | 67832.8083 | -26569.2981 | Lower | -31986.7650 | Lower | -5417.4669 | Lower |
| SLOC | **48 ✅** | 68 | 69 | -20 | Lower | -21 | Lower | -1 | Lower |
| PLOC | **35 ✅** | 52 | 56 | -17 | Lower | -21 | Lower | -4 | Lower |
| LLOC | **26 ✅** | 40 | 42 | -14 | Lower | -16 | Lower | -2 | Lower |
| Comments (CLOC) | 6 | 5 | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **86.9210 ✅** | 76.7727 | 86.7236 | +10.1483 | Higher | +0.1973 | Higher | -9.9509 | Lower |
| MI (SEI) | 76.7469 | 72.4808 | **80.0262 ✅** | +4.2662 | Higher | -3.2793 | Lower | -7.5454 | Lower |
| Cyclomatic (sum) | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | **482.8093 ✅** | 743.3983 | 501.4827 | -260.5889 | Lower | -18.6734 | Lower | +241.9155 | Higher |
| Halstead Effort | **6448.9532 ✅** | 11453.8400 | 6619.5720 | -5004.8868 | Lower | -170.6188 | Lower | +4834.2681 | Higher |
| SLOC | 23 | 38 | 23 | -15 | Lower | 0 | Same | +15 | Higher |
| PLOC | 16 | 21 | **15 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| LLOC | **8 ✅** | 12 | 11 | -4 | Lower | -3 | Lower | +1 | Higher |
| Comments (CLOC) | 3 | **11 ✅** | 4 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00445

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.5661 ✅** | 75.8737 | 77.4766 | +1.6925 | Higher | +0.0895 | Higher | -1.6030 | Lower |
| MI (SEI) | 61.9215 | 62.5142 | **66.7040 ✅** | -0.5926 | Lower | -4.7825 | Lower | -4.1898 | Lower |
| Cyclomatic (sum) | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **722.0966 ✅** | 837.7444 | 734.6337 | -115.6478 | Lower | -12.5372 | Lower | +103.1106 | Higher |
| Halstead Effort | **14289.1798 ✅** | 20911.3884 | 16498.6494 | -6622.2086 | Lower | -2209.4696 | Lower | +4412.7390 | Higher |
| SLOC | 35 | 35 | 35 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 24 | 23 | 23 | +1 | Higher | +1 | Higher | 0 | Same |
| LLOC | 18 | 17 | 17 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 4 | 5 | **6 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.2683 ✅** | 66.4516 | 66.1146 | +0.8166 | Higher | +1.1537 | Higher | +0.3371 | Higher |
| MI (SEI) | **50.8784 ✅** | 50.6115 | 49.7760 | +0.2669 | Higher | +1.1023 | Higher | +0.8355 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 12 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 13 | 13 | 16 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | 1151.3120 | 1254.5551 | **1145.7884 ✅** | -103.2430 | Lower | +5.5236 | Higher | +108.7667 | Higher |
| Halstead Effort | 41247.9669 | 37636.6517 | **37132.0309 ✅** | +3611.3151 | Higher | +4115.9359 | Higher | +504.6208 | Higher |
| SLOC | **53 ✅** | 55 | 57 | -2 | Lower | -4 | Lower | -2 | Lower |
| PLOC | **39 ✅** | 40 | 42 | -1 | Lower | -3 | Lower | -2 | Lower |
| LLOC | 32 | **31 ✅** | 34 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | 8 | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00453

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.2014 | 42.7225 | **44.4294 ✅** | +1.4790 | Higher | -0.2279 | Lower | -1.7069 | Lower |
| MI (SEI) | **17.2554 ✅** | 13.9876 | 14.3915 | +3.2678 | Higher | +2.8639 | Higher | -0.4039 | Lower |
| Cyclomatic (sum) | 29 | 28 | 28 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 58 | 61 | **55 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Halstead Volume | **3786.2441 ✅** | 3984.2841 | 4217.3511 | -198.0401 | Lower | -431.1070 | Lower | -233.0669 | Lower |
| Halstead Effort | 252442.0283 | **225007.2048 ✅** | 290480.8136 | +27434.8235 | Higher | -38038.7853 | Lower | -65473.6088 | Lower |
| SLOC | 118 | 129 | **114 ✅** | -11 | Lower | +4 | Higher | +15 | Higher |
| PLOC | 93 | 97 | **91 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| LLOC | 69 | **68 ✅** | 71 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | 15 | 15 | 11 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 12 | 17 | 12 | -5 | Lower | 0 | Same | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00465

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **43.6759 ✅** | 35.0572 | 28.8734 | +8.6187 | Higher | +14.8024 | Higher | +6.1837 | Higher |
| MI (SEI) | **15.1110 ✅** | 6.3335 | -15.7449 | +8.7775 | Higher | +30.8559 | Higher | +22.0784 | Higher |
| Cyclomatic (sum) | **27 ✅** | 31 | 43 | -4 | Lower | -16 | Lower | -12 | Lower |
| Cognitive (sum) | **30 ✅** | 48 | 44 | -18 | Lower | -14 | Lower | +4 | Higher |
| Halstead Volume | **4124.9455 ✅** | 5491.7235 | 6355.0874 | -1366.7780 | Lower | -2230.1418 | Lower | -863.3638 | Lower |
| Halstead Effort | **216028.3971 ✅** | 371255.5571 | 468864.2234 | -155227.1599 | Lower | -252835.8263 | Lower | -97608.6663 | Lower |
| SLOC | **122 ✅** | 179 | 211 | -57 | Lower | -89 | Lower | -32 | Lower |
| PLOC | **94 ✅** | 130 | 181 | -36 | Lower | -87 | Lower | -51 | Lower |
| LLOC | **68 ✅** | 96 | 118 | -28 | Lower | -50 | Lower | -22 | Lower |
| Comments (CLOC) | 14 | **27 ✅** | 7 | -13 | Lower | +7 | Higher | +20 | Higher |
| Blank Lines | **17 ✅** | 24 | 24 | -7 | Lower | -7 | Lower | 0 | Same |
| Functions | 4 | **3 ✅** | 10 | +1 | Higher | -6 | Lower | -7 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **2 ✅** | 7 | +1 | Higher | -4 | Lower | -5 | Lower |

## p00468

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.8204 ✅** | 59.4976 | 61.2383 | +3.3228 | Higher | +1.5821 | Higher | -1.7407 | Lower |
| MI (SEI) | **40.9411 ✅** | 33.8772 | 33.0224 | +7.0639 | Higher | +7.9187 | Higher | +0.8548 | Higher |
| Cyclomatic (sum) | **13 ✅** | 18 | 16 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | **17 ✅** | 24 | 21 | -7 | Lower | -4 | Lower | +3 | Higher |
| Halstead Volume | **1512.1710 ✅** | 1653.9294 | 1628.6332 | -141.7584 | Lower | -116.4622 | Lower | +25.2962 | Higher |
| Halstead Effort | **45152.4810 ✅** | 59659.5950 | 56544.1076 | -14507.1141 | Lower | -11391.6267 | Lower | +3115.4874 | Higher |
| SLOC | **63 ✅** | 70 | 65 | -7 | Lower | -2 | Lower | +5 | Higher |
| PLOC | **46 ✅** | 51 | 52 | -5 | Lower | -6 | Lower | -1 | Lower |
| LLOC | **37 ✅** | 38 | 43 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | **7 ✅** | 6 | 4 | +1 | Higher | +3 | Higher | +2 | Higher |
| Blank Lines | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00479

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.1355 ✅** | 74.2049 | 70.1362 | +0.9306 | Higher | +4.9993 | Higher | +4.0687 | Higher |
| MI (SEI) | 58.0970 | **58.7088 ✅** | 45.9734 | -0.6118 | Lower | +12.1236 | Higher | +12.7354 | Higher |
| Cyclomatic (sum) | 7 | 7 | 10 | 0 | Same | -3 | Lower | -3 | Lower |
| Cognitive (sum) | 3 | 3 | 10 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | **1055.5655 ✅** | 1066.7278 | 1206.3519 | -11.1624 | Lower | -150.7864 | Lower | -139.6241 | Lower |
| Halstead Effort | **19387.2188 ✅** | 20617.2742 | 28866.2776 | -1230.0555 | Lower | -9479.0588 | Lower | -8249.0034 | Lower |
| SLOC | **36 ✅** | 38 | 45 | -2 | Lower | -9 | Lower | -7 | Lower |
| PLOC | **25 ✅** | 26 | 33 | -1 | Lower | -8 | Lower | -7 | Lower |
| LLOC | **15 ✅** | 16 | 19 | -1 | Lower | -4 | Lower | -3 | Lower |
| Comments (CLOC) | 4 | **5 ✅** | 3 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p00492

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.6747 ✅** | 48.7753 | 51.6255 | +3.8993 | Higher | +1.0492 | Higher | -2.8501 | Lower |
| MI (SEI) | **32.6704 ✅** | 25.3429 | 30.1681 | +7.3275 | Higher | +2.5023 | Higher | -4.8252 | Lower |
| Cyclomatic (sum) | **16 ✅** | 24 | 18 | -8 | Lower | -2 | Lower | +6 | Higher |
| Cognitive (sum) | 30 | 39 | **27 ✅** | -9 | Lower | +3 | Higher | +12 | Higher |
| Halstead Volume | **2769.4162 ✅** | 3282.4001 | 2809.5748 | -512.9839 | Lower | -40.1586 | Lower | +472.8253 | Higher |
| Halstead Effort | **147240.6280 ✅** | 193005.1259 | 153809.2753 | -45764.4979 | Lower | -6568.6473 | Lower | +39195.8506 | Higher |
| SLOC | **93 ✅** | 100 | 96 | -7 | Lower | -3 | Lower | +4 | Higher |
| PLOC | 66 | 72 | **64 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| LLOC | 56 | **48 ✅** | 55 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | **17 ✅** | 15 | 16 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | 13 | 13 | 16 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.8284 | **64.4275 ✅** | 58.8512 | -3.5991 | Lower | +1.9772 | Higher | +5.5762 | Higher |
| MI (SEI) | 41.1880 | **53.2130 ✅** | 30.5809 | -12.0250 | Lower | +10.6071 | Higher | +22.6321 | Higher |
| Cyclomatic (sum) | 13 | **12 ✅** | 15 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | **15 ✅** | 16 | 16 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | 1670.6706 | **1283.0542 ✅** | 1958.9306 | +387.6164 | Higher | -288.2600 | Lower | -675.8764 | Lower |
| Halstead Effort | 69071.7887 | **42386.6114 ✅** | 92960.1605 | +26685.1773 | Higher | -23888.3718 | Lower | -50573.5492 | Lower |
| SLOC | 69 | **61 ✅** | 72 | +8 | Higher | -3 | Lower | -11 | Lower |
| PLOC | 47 | **39 ✅** | 54 | +8 | Higher | -7 | Lower | -15 | Lower |
| LLOC | 36 | **31 ✅** | 41 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | 10 | **15 ✅** | 5 | -5 | Lower | +5 | Higher | +10 | Higher |
| Blank Lines | 14 | **10 ✅** | 14 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |

## p00507

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.6180 ✅** | 60.0314 | 64.9959 | +8.5866 | Higher | +3.6221 | Higher | -4.9645 | Lower |
| MI (SEI) | **58.3460 ✅** | 39.6219 | 55.9029 | +18.7241 | Higher | +2.4430 | Higher | -16.2810 | Lower |
| Cyclomatic (sum) | **9 ✅** | 14 | 11 | -5 | Lower | -2 | Lower | +3 | Higher |
| Cognitive (sum) | 17 | **15 ✅** | 21 | +2 | Higher | -4 | Lower | -6 | Lower |
| Halstead Volume | **1143.2464 ✅** | 1631.7780 | 1265.7371 | -488.5315 | Lower | -122.4907 | Lower | +366.0409 | Higher |
| Halstead Effort | **33559.8149 ✅** | 62887.8650 | 35440.6397 | -29328.0501 | Lower | -1880.8248 | Lower | +27447.2253 | Higher |
| SLOC | **51 ✅** | 72 | 60 | -21 | Lower | -9 | Lower | +12 | Higher |
| PLOC | **31 ✅** | 54 | 35 | -23 | Lower | -4 | Lower | +19 | Higher |
| LLOC | **22 ✅** | 44 | 25 | -22 | Lower | -3 | Lower | +19 | Higher |
| Comments (CLOC) | 12 | 10 | **17 ✅** | +2 | Higher | -5 | Lower | -7 | Lower |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00514

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.8718 ✅** | 23.5833 | 48.1381 | +25.2885 | Higher | +0.7337 | Higher | -24.5548 | Lower |
| MI (SEI) | **22.4415 ✅** | -11.3676 | 18.2048 | +33.8091 | Higher | +4.2366 | Higher | -29.5724 | Lower |
| Cyclomatic (sum) | 20 | 53 | 20 | -33 | Lower | 0 | Same | +33 | Higher |
| Cognitive (sum) | **16 ✅** | 53 | 18 | -37 | Lower | -2 | Lower | +35 | Higher |
| Halstead Volume | **3025.8148 ✅** | 6913.7975 | 3199.2697 | -3887.9827 | Lower | -173.4550 | Lower | +3714.5278 | Higher |
| Halstead Effort | **166104.6227 ✅** | 599195.7821 | 207632.6054 | -433091.1594 | Lower | -41527.9827 | Lower | +391563.1767 | Higher |
| SLOC | **108 ✅** | 247 | 111 | -139 | Lower | -3 | Lower | +136 | Higher |
| PLOC | **83 ✅** | 189 | 86 | -106 | Lower | -3 | Lower | +103 | Higher |
| LLOC | **60 ✅** | 120 | 73 | -60 | Lower | -13 | Lower | +47 | Higher |
| Comments (CLOC) | 13 | **28 ✅** | 10 | -15 | Lower | +3 | Higher | +18 | Higher |
| Blank Lines | 15 | 31 | 15 | -16 | Lower | 0 | Same | +16 | Higher |
| Functions | 6 | 9 | **5 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 3 | 17 | **2 ✅** | -14 | Lower | +1 | Higher | +15 | Higher |

## p00518

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.8380 | **58.4837 ✅** | 57.4449 | -3.6457 | Lower | -2.6069 | Lower | +1.0388 | Higher |
| MI (SEI) | 37.8047 | **37.9349 ✅** | 28.4783 | -0.1302 | Lower | +9.3264 | Higher | +9.4567 | Higher |
| Cyclomatic (sum) | 17 | **16 ✅** | 18 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 32 | 30 | **29 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Halstead Volume | 1935.8132 | **1631.7780 ✅** | 1979.7584 | +304.0352 | Higher | -43.9452 | Lower | -347.9804 | Lower |
| Halstead Effort | 69504.9114 | **49152.3371 ✅** | 79850.2561 | +20352.5744 | Higher | -10345.3446 | Lower | -30697.9190 | Lower |
| SLOC | 90 | 77 | **75 ✅** | +13 | Higher | +15 | Higher | +2 | Higher |
| PLOC | 61 | **53 ✅** | 59 | +8 | Higher | +2 | Higher | -6 | Lower |
| LLOC | 54 | **37 ✅** | 51 | +17 | Higher | +3 | Higher | -14 | Lower |
| Comments (CLOC) | **19 ✅** | 11 | 5 | +8 | Higher | +14 | Higher | +6 | Higher |
| Blank Lines | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00519

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.9543 | **46.0889 ✅** | 31.0085 | -3.1346 | Lower | +11.9458 | Higher | +15.0804 | Higher |
| MI (SEI) | 14.8691 | **18.0870 ✅** | -8.3042 | -3.2179 | Lower | +23.1734 | Higher | +26.3912 | Higher |
| Cyclomatic (sum) | 33 | **31 ✅** | 38 | +2 | Higher | -5 | Lower | -7 | Lower |
| Cognitive (sum) | 32 | 32 | 42 | 0 | Same | -10 | Lower | -10 | Lower |
| Halstead Volume | 3826.4064 | **3572.9442 ✅** | 6831.3250 | +253.4622 | Higher | -3004.9186 | Lower | -3258.3808 | Lower |
| Halstead Effort | 233758.6448 | **203320.7507 ✅** | 493124.0733 | +30437.8941 | Higher | -259365.4284 | Lower | -289803.3226 | Lower |
| SLOC | 120 | **104 ✅** | 194 | +16 | Higher | -74 | Lower | -90 | Lower |
| PLOC | 92 | **80 ✅** | 167 | +12 | Higher | -75 | Lower | -87 | Lower |
| LLOC | 49 | **47 ✅** | 88 | +2 | Higher | -39 | Lower | -41 | Lower |
| Comments (CLOC) | **14 ✅** | 11 | 12 | +3 | Higher | +2 | Higher | -1 | Lower |
| Blank Lines | 14 | **13 ✅** | 18 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 8 | 6 | **4 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Exits | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |

## p00528

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.8739 | **36.4236 ✅** | 17.0371 | -0.5498 | Lower | +18.8367 | Higher | +19.3865 | Higher |
| MI (SEI) | 3.6366 | **4.7931 ✅** | -28.9579 | -1.1566 | Lower | +32.5944 | Higher | +33.7510 | Higher |
| Cyclomatic (sum) | 34 | **32 ✅** | 60 | +2 | Higher | -26 | Lower | -28 | Lower |
| Cognitive (sum) | 40 | **35 ✅** | 62 | +5 | Higher | -22 | Lower | -27 | Lower |
| Halstead Volume | 5718.2685 | **5512.9991 ✅** | 10605.4655 | +205.2694 | Higher | -4887.1970 | Lower | -5092.4664 | Lower |
| Halstead Effort | 389495.7756 | **361780.5791 ✅** | 1085676.8905 | +27715.1965 | Higher | -696181.1149 | Lower | -723896.3114 | Lower |
| SLOC | **161 ✅** | 162 | 292 | -1 | Lower | -131 | Lower | -130 | Lower |
| PLOC | 131 | **130 ✅** | 253 | +1 | Higher | -122 | Lower | -123 | Lower |
| LLOC | 108 | **86 ✅** | 198 | +22 | Higher | -90 | Lower | -112 | Lower |
| Comments (CLOC) | 17 | **18 ✅** | 13 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | **19 ✅** | 20 | 27 | -1 | Lower | -8 | Lower | -7 | Lower |
| Functions | 2 | **1 ✅** | 13 | +1 | Higher | -11 | Lower | -12 | Lower |
| Closures | 3 | 6 | **0 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Exits | 1 | 1 | 13 | 0 | Same | -12 | Lower | -12 | Lower |

## p00534

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.0199 ✅** | 57.1639 | 68.5465 | +12.8560 | Higher | +1.4734 | Higher | -11.3826 | Lower |
| MI (SEI) | **56.5175 ✅** | 42.8248 | 50.7473 | +13.6926 | Higher | +5.7702 | Higher | -7.9224 | Lower |
| Cyclomatic (sum) | **8 ✅** | 14 | 10 | -6 | Lower | -2 | Lower | +4 | Higher |
| Cognitive (sum) | **10 ✅** | 16 | 11 | -6 | Lower | -1 | Lower | +5 | Higher |
| Halstead Volume | **1347.7491 ✅** | 1888.8205 | 1529.3548 | -541.0713 | Lower | -181.6056 | Lower | +359.4657 | Higher |
| Halstead Effort | **42204.5140 ✅** | 81455.3822 | 54521.4972 | -39250.8681 | Lower | -12316.9832 | Lower | +26933.8849 | Higher |
| SLOC | **45 ✅** | 82 | 46 | -37 | Lower | -1 | Lower | +36 | Higher |
| PLOC | **29 ✅** | 50 | 32 | -21 | Lower | -3 | Lower | +18 | Higher |
| LLOC | 25 | 38 | 25 | -13 | Lower | 0 | Same | +13 | Higher |
| Comments (CLOC) | 8 | **20 ✅** | 6 | -12 | Lower | +2 | Higher | +14 | Higher |
| Blank Lines | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00543

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.1393 ✅** | 74.0910 | 63.5340 | +2.0483 | Higher | +12.6052 | Higher | +10.5569 | Higher |
| MI (SEI) | **62.4901 ✅** | 56.0907 | 46.6291 | +6.3994 | Higher | +15.8610 | Higher | +9.4616 | Higher |
| Cyclomatic (sum) | **7 ✅** | 8 | 13 | -1 | Lower | -6 | Lower | -5 | Lower |
| Cognitive (sum) | **8 ✅** | 12 | 11 | -4 | Lower | -3 | Lower | +1 | Higher |
| Halstead Volume | **950.0906 ✅** | 1043.1805 | 1457.6461 | -93.0899 | Lower | -507.5555 | Lower | -414.4656 | Lower |
| Halstead Effort | **24524.2132 ✅** | 25647.8508 | 50547.4046 | -1123.6376 | Lower | -26023.1913 | Lower | -24899.5537 | Lower |
| SLOC | **35 ✅** | 38 | 61 | -3 | Lower | -26 | Lower | -23 | Lower |
| PLOC | **25 ✅** | 29 | 45 | -4 | Lower | -20 | Lower | -16 | Lower |
| LLOC | **18 ✅** | 20 | 35 | -2 | Lower | -17 | Lower | -15 | Lower |
| Comments (CLOC) | 5 | 4 | **10 ✅** | +1 | Higher | -5 | Lower | -6 | Lower |
| Blank Lines | 5 | 5 | 7 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p00554

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.4551 ✅** | 69.1803 | 59.0272 | +9.2747 | Higher | +19.4278 | Higher | +10.1531 | Higher |
| MI (SEI) | **64.0322 ✅** | 50.8187 | 40.2637 | +13.2135 | Higher | +23.7685 | Higher | +10.5550 | Higher |
| Cyclomatic (sum) | **5 ✅** | 9 | 14 | -4 | Lower | -9 | Lower | -5 | Lower |
| Cognitive (sum) | **5 ✅** | 11 | 19 | -6 | Lower | -14 | Lower | -8 | Lower |
| Halstead Volume | **879.0549 ✅** | 1162.2449 | 1896.1119 | -283.1900 | Lower | -1017.0570 | Lower | -733.8669 | Lower |
| Halstead Effort | **16816.7022 ✅** | 33928.6118 | 79230.3893 | -17111.9096 | Lower | -62413.6871 | Lower | -45301.7775 | Lower |
| SLOC | **32 ✅** | 49 | 73 | -17 | Lower | -41 | Lower | -24 | Lower |
| PLOC | **24 ✅** | 34 | 53 | -10 | Lower | -29 | Lower | -19 | Lower |
| LLOC | **15 ✅** | 25 | 45 | -10 | Lower | -30 | Lower | -20 | Lower |
| Comments (CLOC) | 4 | 6 | **12 ✅** | -2 | Lower | -8 | Lower | -6 | Lower |
| Blank Lines | **5 ✅** | 9 | 10 | -4 | Lower | -5 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00571

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.1370 | **58.0405 ✅** | 49.5012 | -5.9035 | Lower | +2.6358 | Higher | +8.5393 | Higher |
| MI (SEI) | 25.6265 | **45.9308 ✅** | 18.9969 | -20.3044 | Lower | +6.6296 | Higher | +26.9340 | Higher |
| Cyclomatic (sum) | 21 | **13 ✅** | 19 | +8 | Higher | +2 | Higher | -6 | Lower |
| Cognitive (sum) | 15 | **13 ✅** | 14 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 2229.9171 | **1667.9648 ✅** | 2572.8587 | +561.9523 | Higher | -342.9416 | Lower | -904.8939 | Lower |
| Halstead Effort | 83646.1291 | **46981.0091 ✅** | 105431.2760 | +36665.1201 | Higher | -21785.1469 | Lower | -58450.2670 | Lower |
| SLOC | 96 | **82 ✅** | 111 | +14 | Higher | -15 | Lower | -29 | Lower |
| PLOC | 72 | **47 ✅** | 87 | +25 | Higher | -15 | Lower | -40 | Lower |
| LLOC | 38 | **29 ✅** | 49 | +9 | Higher | -11 | Lower | -20 | Lower |
| Comments (CLOC) | 10 | **23 ✅** | 9 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | 16 | **14 ✅** | 15 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 6 | **1 ✅** | 5 | +5 | Higher | +1 | Higher | -4 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |

## p00594

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.2445 ✅** | 58.3694 | 60.4160 | +5.8751 | Higher | +3.8285 | Higher | -2.0466 | Lower |
| MI (SEI) | **45.0059 ✅** | 36.0595 | 41.2446 | +8.9464 | Higher | +3.7613 | Higher | -5.1851 | Lower |
| Cyclomatic (sum) | **12 ✅** | 18 | 13 | -6 | Lower | -1 | Lower | +5 | Higher |
| Cognitive (sum) | **11 ✅** | 16 | 19 | -5 | Lower | -8 | Lower | -3 | Lower |
| Halstead Volume | **1399.2361 ✅** | 1728.0789 | 1583.9792 | -328.8428 | Lower | -184.7430 | Lower | +144.0998 | Higher |
| Halstead Effort | **43716.1346 ✅** | 60256.4669 | 54053.2895 | -16540.3323 | Lower | -10337.1549 | Lower | +6203.1774 | Higher |
| SLOC | **60 ✅** | 74 | 72 | -14 | Lower | -12 | Lower | +2 | Higher |
| PLOC | **45 ✅** | 56 | 54 | -11 | Lower | -9 | Lower | +2 | Higher |
| LLOC | **37 ✅** | 38 | 42 | -1 | Lower | -5 | Lower | -4 | Lower |
| Comments (CLOC) | 8 | 9 | **11 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | **9 ✅** | 11 | 10 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00629

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.8357 ✅** | 52.5038 | 51.5067 | +1.3319 | Higher | +2.3291 | Higher | +0.9971 | Higher |
| MI (SEI) | **36.3118 ✅** | 28.6176 | 23.1170 | +7.6942 | Higher | +13.1948 | Higher | +5.5006 | Higher |
| Cyclomatic (sum) | 18 | 18 | 23 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 22 | **17 ✅** | 23 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | **2169.7662 ✅** | 2225.2874 | 2721.9938 | -55.5212 | Lower | -552.2276 | Lower | -496.7064 | Lower |
| Halstead Effort | **54151.8242 ✅** | 68256.4101 | 104116.2616 | -14104.5859 | Lower | -49964.4374 | Lower | -35859.8515 | Lower |
| SLOC | 91 | 98 | 91 | -7 | Lower | 0 | Same | +7 | Higher |
| PLOC | **71 ✅** | 72 | 74 | -1 | Lower | -3 | Lower | -2 | Lower |
| LLOC | 38 | **36 ✅** | 55 | +2 | Higher | -17 | Lower | -19 | Lower |
| Comments (CLOC) | **19 ✅** | 13 | 8 | +6 | Higher | +11 | Higher | +5 | Higher |
| Blank Lines | 11 | 15 | **10 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00641

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.8830 | **45.9432 ✅** | 40.3180 | -6.0602 | Lower | -0.4350 | Lower | +5.6252 | Higher |
| MI (SEI) | 0.6981 | **13.0409 ✅** | 3.9569 | -12.3428 | Lower | -3.2588 | Lower | +9.0840 | Higher |
| Cyclomatic (sum) | 32 | **20 ✅** | 28 | +12 | Higher | +4 | Higher | -8 | Lower |
| Cognitive (sum) | 25 | **16 ✅** | 18 | +9 | Higher | +7 | Higher | -2 | Lower |
| Halstead Volume | 4180.7534 | **3635.1087 ✅** | 4123.4838 | +545.6447 | Higher | +57.2696 | Higher | -488.3752 | Lower |
| Halstead Effort | 307285.3745 | **182407.0088 ✅** | 273499.4371 | +124878.3657 | Higher | +33785.9374 | Higher | -91092.4283 | Lower |
| SLOC | 143 | **122 ✅** | 148 | +21 | Higher | -5 | Lower | -26 | Lower |
| PLOC | 119 | **97 ✅** | 120 | +22 | Higher | -1 | Lower | -23 | Lower |
| LLOC | 84 | **70 ✅** | 81 | +14 | Higher | +3 | Higher | -11 | Lower |
| Comments (CLOC) | 6 | 9 | 9 | -3 | Lower | -3 | Lower | 0 | Same |
| Blank Lines | 19 | **17 ✅** | 20 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 6 | **4 ✅** | 8 | +2 | Higher | -2 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |

## p00649

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.7344 | **51.3428 ✅** | 37.9408 | -10.6085 | Lower | +2.7936 | Higher | +13.4020 | Higher |
| MI (SEI) | 6.2365 | **26.7710 ✅** | 0.7549 | -20.5345 | Lower | +5.4816 | Higher | +26.0161 | Higher |
| Cyclomatic (sum) | 30 | **22 ✅** | 30 | +8 | Higher | 0 | Same | -8 | Lower |
| Cognitive (sum) | 41 | **11 ✅** | 33 | +30 | Higher | +8 | Higher | -22 | Lower |
| Halstead Volume | 4142.4618 | **2567.8982 ✅** | 4330.0660 | +1574.5636 | Higher | -187.6042 | Lower | -1762.1678 | Lower |
| Halstead Effort | 226454.5807 | **83456.6918 ✅** | 251400.6972 | +142997.8889 | Higher | -24946.1165 | Lower | -167944.0054 | Lower |
| SLOC | 140 | **95 ✅** | 164 | +45 | Higher | -24 | Lower | -69 | Lower |
| PLOC | 113 | **72 ✅** | 139 | +41 | Higher | -26 | Lower | -67 | Lower |
| LLOC | 80 | **37 ✅** | 103 | +43 | Higher | -23 | Lower | -66 | Lower |
| Comments (CLOC) | 10 | **12 ✅** | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 17 | **11 ✅** | 16 | +6 | Higher | +1 | Higher | -5 | Lower |
| Functions | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **2 ✅** | 5 | 3 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00653

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.0572 | **44.9307 ✅** | 33.2570 | -10.8735 | Lower | +0.8002 | Higher | +11.6737 | Higher |
| MI (SEI) | -1.0362 | **16.2005 ✅** | -7.7358 | -17.2367 | Lower | +6.6996 | Higher | +23.9362 | Higher |
| Cyclomatic (sum) | 35 | **19 ✅** | 33 | +16 | Higher | +2 | Higher | -14 | Lower |
| Cognitive (sum) | **30 ✅** | 34 | 37 | -4 | Lower | -7 | Lower | -3 | Lower |
| Halstead Volume | 6091.4020 | **4736.1060 ✅** | 6099.0600 | +1355.2960 | Higher | -7.6580 | Lower | -1362.9540 | Lower |
| Halstead Effort | 533497.7137 | **307083.0006 ✅** | 580281.9926 | +226414.7131 | Higher | -46784.2789 | Lower | -273198.9919 | Lower |
| SLOC | 174 | **121 ✅** | 188 | +53 | Higher | -14 | Lower | -67 | Lower |
| PLOC | 134 | **93 ✅** | 151 | +41 | Higher | -17 | Lower | -58 | Lower |
| LLOC | 97 | **78 ✅** | 116 | +19 | Higher | -19 | Lower | -38 | Lower |
| Comments (CLOC) | **15 ✅** | 14 | 9 | +1 | Higher | +6 | Higher | +5 | Higher |
| Blank Lines | 25 | **17 ✅** | 28 | +8 | Higher | -3 | Lower | -11 | Lower |
| Functions | 8 | **1 ✅** | 6 | +7 | Higher | +2 | Higher | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **0 ✅** | 8 | +9 | Higher | +1 | Higher | -8 | Lower |

## p00655

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **23.3360 ✅** | 21.9774 | 18.5663 | +1.3586 | Higher | +4.7696 | Higher | +3.4110 | Higher |
| MI (SEI) | **-22.1161 ✅** | -26.7140 | -25.2869 | +4.5978 | Higher | +3.1707 | Higher | -1.4271 | Lower |
| Cyclomatic (sum) | **56 ✅** | 69 | 63 | -13 | Lower | -7 | Lower | +6 | Higher |
| Cognitive (sum) | **49 ✅** | 68 | 91 | -19 | Lower | -42 | Lower | -23 | Lower |
| Halstead Volume | 7822.6860 | **6204.5546 ✅** | 8633.4496 | +1618.1314 | Higher | -810.7636 | Lower | -2428.8950 | Lower |
| Halstead Effort | 981541.2309 | **457177.7064 ✅** | 1236581.7752 | +524363.5245 | Higher | -255040.5443 | Lower | -779404.0688 | Lower |
| SLOC | 231 | **225 ✅** | 272 | +6 | Higher | -41 | Lower | -47 | Lower |
| PLOC | 203 | **199 ✅** | 236 | +4 | Higher | -33 | Lower | -37 | Lower |
| LLOC | 122 | **80 ✅** | 163 | +42 | Higher | -41 | Lower | -83 | Lower |
| Comments (CLOC) | 8 | 4 | **14 ✅** | +4 | Higher | -6 | Lower | -10 | Lower |
| Blank Lines | **20 ✅** | 23 | 24 | -3 | Lower | -4 | Lower | -1 | Lower |
| Functions | 11 | 12 | 11 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 11 | **8 ✅** | 11 | +3 | Higher | 0 | Same | -3 | Lower |

## p00658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **-10.2447 ✅** | -19.1659 | -12.1236 | +8.9213 | Higher | +1.8789 | Higher | -7.0424 | Lower |
| MI (SEI) | **-59.6579 ✅** | -63.9264 | -62.0250 | +4.2686 | Higher | +2.3671 | Higher | -1.9014 | Lower |
| Cyclomatic (sum) | **139 ✅** | 151 | 147 | -12 | Lower | -8 | Lower | +4 | Higher |
| Cognitive (sum) | 181 | **178 ✅** | 184 | +3 | Higher | -3 | Lower | -6 | Lower |
| Halstead Volume | 15152.4918 | 20153.5433 | **14658.3826 ✅** | -5001.0516 | Lower | +494.1092 | Higher | +5495.1608 | Higher |
| Halstead Effort | **1989769.5545 ✅** | 2671998.8887 | 2219279.1203 | -682229.3342 | Lower | -229509.5658 | Lower | +452719.7684 | Higher |
| SLOC | **457 ✅** | 610 | 463 | -153 | Lower | -6 | Lower | +147 | Higher |
| PLOC | **407 ✅** | 480 | 412 | -73 | Lower | -5 | Lower | +68 | Higher |
| LLOC | **265 ✅** | 318 | 266 | -53 | Lower | -1 | Lower | +52 | Higher |
| Comments (CLOC) | 22 | **64 ✅** | 21 | -42 | Lower | +1 | Higher | +43 | Higher |
| Blank Lines | **29 ✅** | 72 | 31 | -43 | Lower | -2 | Lower | +41 | Higher |
| Functions | 12 | 17 | **11 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 29 | 40 | **28 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |

## p00680

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **19.2441 ✅** | 18.9397 | 17.9938 | +0.3044 | Higher | +1.2503 | Higher | +0.9459 | Higher |
| MI (SEI) | -26.4471 | **-20.8912 ✅** | -34.6946 | -5.5559 | Lower | +8.2475 | Higher | +13.8034 | Higher |
| Cyclomatic (sum) | 60 | 59 | **58 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | **76 ✅** | 89 | 89 | -13 | Lower | -13 | Lower | 0 | Same |
| Halstead Volume | 8855 | **8762.2874 ✅** | 10510.6911 | +92.7126 | Higher | -1655.6911 | Lower | -1748.4038 | Lower |
| Halstead Effort | 793485 | **759808.4565 ✅** | 965411.8911 | +33676.5435 | Higher | -171926.8911 | Lower | -205603.4346 | Lower |
| SLOC | **270 ✅** | 280 | 284 | -10 | Lower | -14 | Lower | -4 | Lower |
| PLOC | 229 | **227 ✅** | 244 | +2 | Higher | -15 | Lower | -17 | Lower |
| LLOC | **145 ✅** | 146 | 194 | -1 | Lower | -49 | Lower | -48 | Lower |
| Comments (CLOC) | 11 | **23 ✅** | 4 | -12 | Lower | +7 | Higher | +19 | Higher |
| Blank Lines | 32 | **31 ✅** | 36 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 10 | **8 ✅** | 9 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 10 | 8 | **6 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |

## p00690

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.4699 | **47.6882 ✅** | 43.8635 | -2.2183 | Lower | +1.6064 | Higher | +3.8247 | Higher |
| MI (SEI) | 12.2509 | **17.5188 ✅** | 7.4774 | -5.2679 | Lower | +4.7735 | Higher | +10.0414 | Higher |
| Cyclomatic (sum) | 26 | **17 ✅** | 30 | +9 | Higher | -4 | Lower | -13 | Lower |
| Cognitive (sum) | 34 | **25 ✅** | 37 | +9 | Higher | -3 | Lower | -12 | Lower |
| Halstead Volume | 3478.6814 | **3206.9753 ✅** | 3484.3333 | +271.7061 | Higher | -5.6519 | Lower | -277.3580 | Lower |
| Halstead Effort | 179929.2444 | **129080.7574 ✅** | 173101.6792 | +50848.4870 | Higher | +6827.5653 | Higher | -44020.9217 | Lower |
| SLOC | **117 ✅** | 119 | 122 | -2 | Lower | -5 | Lower | -3 | Lower |
| PLOC | 96 | **94 ✅** | 105 | +2 | Higher | -9 | Lower | -11 | Lower |
| LLOC | **63 ✅** | 64 | 65 | -1 | Lower | -2 | Lower | -1 | Lower |
| Comments (CLOC) | 8 | **11 ✅** | 6 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 13 | 14 | **11 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00692

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.2490 | **48.0111 ✅** | 42.1161 | -1.7620 | Lower | +4.1330 | Higher | +5.8950 | Higher |
| MI (SEI) | 20.0618 | **22.2714 ✅** | 13.9592 | -2.2096 | Lower | +6.1026 | Higher | +8.3123 | Higher |
| Cyclomatic (sum) | 33 | **32 ✅** | 35 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | **31 ✅** | 41 | 32 | -10 | Lower | -1 | Lower | +9 | Higher |
| Halstead Volume | 3697.4041 | **2587.5237 ✅** | 4223.7886 | +1109.8804 | Higher | -526.3846 | Lower | -1636.2650 | Lower |
| Halstead Effort | 269045.1464 | **133310.2754 ✅** | 307945.4782 | +135734.8710 | Higher | -38900.3318 | Lower | -174635.2028 | Lower |
| SLOC | **99 ✅** | 101 | 119 | -2 | Lower | -20 | Lower | -18 | Lower |
| PLOC | **78 ✅** | 79 | 90 | -1 | Lower | -12 | Lower | -11 | Lower |
| LLOC | 60 | **53 ✅** | 69 | +7 | Higher | -9 | Lower | -16 | Lower |
| Comments (CLOC) | 12 | 12 | **14 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | **9 ✅** | 12 | 15 | -3 | Lower | -6 | Lower | -3 | Lower |
| Functions | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 3 | -1 | Lower | -3 | Lower | -2 | Lower |

## p00723

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.7881 ✅** | 57.3530 | 52.0392 | +1.4351 | Higher | +6.7489 | Higher | +5.3138 | Higher |
| MI (SEI) | 35.0411 | **36.5792 ✅** | 22.2470 | -1.5382 | Lower | +12.7940 | Higher | +14.3322 | Higher |
| Cyclomatic (sum) | **8 ✅** | 9 | 19 | -1 | Lower | -11 | Lower | -10 | Lower |
| Cognitive (sum) | **5 ✅** | 7 | 29 | -2 | Lower | -24 | Lower | -22 | Lower |
| Halstead Volume | **2105.9862 ✅** | 2107.8287 | 3035.2734 | -1.8425 | Lower | -929.2872 | Lower | -927.4446 | Lower |
| Halstead Effort | **71375.8570 ✅** | 83610.5398 | 177922.9329 | -12234.6828 | Lower | -106547.0759 | Lower | -94312.3931 | Lower |
| SLOC | **78 ✅** | 84 | 90 | -6 | Lower | -12 | Lower | -6 | Lower |
| PLOC | 52 | **51 ✅** | 72 | +1 | Higher | -20 | Lower | -21 | Lower |
| LLOC | 38 | 38 | 50 | 0 | Same | -12 | Lower | -12 | Lower |
| Comments (CLOC) | 9 | **13 ✅** | 7 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 18 | 20 | **12 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p00740

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.7368 | **66.4897 ✅** | 62.3715 | -3.7529 | Lower | +0.3652 | Higher | +4.1181 | Higher |
| MI (SEI) | 40.3024 | **45.5317 ✅** | 36.3876 | -5.2292 | Lower | +3.9148 | Higher | +9.1441 | Higher |
| Cyclomatic (sum) | 13 | **12 ✅** | 14 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 18 | **10 ✅** | 21 | +8 | Higher | -3 | Lower | -11 | Lower |
| Halstead Volume | 1329.3790 | **1066.0475 ✅** | 1364.4009 | +263.3316 | Higher | -35.0219 | Lower | -298.3534 | Lower |
| Halstead Effort | 37973.0683 | **31759.3305 ✅** | 44911.5294 | +6213.7378 | Higher | -6938.4611 | Lower | -13152.1989 | Lower |
| SLOC | 66 | **57 ✅** | 66 | +9 | Higher | 0 | Same | -9 | Lower |
| PLOC | 51 | **44 ✅** | 53 | +7 | Higher | -2 | Lower | -9 | Lower |
| LLOC | 43 | **38 ✅** | 43 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | **7 ✅** | 6 | 5 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 12 | **9 ✅** | 10 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00749

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.4827 | **26.1080 ✅** | 20.7398 | -3.6253 | Lower | +1.7429 | Higher | +5.3681 | Higher |
| MI (SEI) | -20.0481 | **-10.5309 ✅** | -28.0844 | -9.5171 | Lower | +8.0364 | Higher | +17.5535 | Higher |
| Cyclomatic (sum) | 53 | **49 ✅** | 53 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | **70 ✅** | 105 | 71 | -35 | Lower | -1 | Lower | +34 | Higher |
| Halstead Volume | 7927.9652 | **7283.1803 ✅** | 8846.0512 | +644.7848 | Higher | -918.0860 | Lower | -1562.8709 | Lower |
| Halstead Effort | 760650.2478 | **625833.2814 ✅** | 735340.7150 | +134816.9664 | Higher | +25309.5328 | Higher | -109507.4336 | Lower |
| SLOC | 253 | **220 ✅** | 272 | +33 | Higher | -19 | Lower | -52 | Lower |
| PLOC | 215 | **182 ✅** | 233 | +33 | Higher | -18 | Lower | -51 | Lower |
| LLOC | 153 | **132 ✅** | 162 | +21 | Higher | -9 | Lower | -30 | Lower |
| Comments (CLOC) | 14 | **20 ✅** | 7 | -6 | Lower | +7 | Higher | +13 | Higher |
| Blank Lines | **26 ✅** | 28 | 33 | -2 | Lower | -7 | Lower | -5 | Lower |
| Functions | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 13 | **5 ✅** | 12 | +8 | Higher | +1 | Higher | -7 | Lower |

## p00762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.6087 | **39.5919 ✅** | 36.4969 | -1.9832 | Lower | +1.1118 | Higher | +3.0950 | Higher |
| MI (SEI) | 5.6562 | **15.7684 ✅** | 4.4699 | -10.1123 | Lower | +1.1862 | Higher | +11.2985 | Higher |
| Cyclomatic (sum) | 31 | **29 ✅** | 32 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | **19 ✅** | 37 | 20 | -18 | Lower | -1 | Lower | +17 | Higher |
| Halstead Volume | 5372.1314 | **4261.3558 ✅** | 5761.8130 | +1110.7757 | Higher | -389.6816 | Lower | -1500.4572 | Lower |
| Halstead Effort | 419026.2526 | **352214.1007 ✅** | 422842.7285 | +66812.1520 | Higher | -3816.4759 | Lower | -70628.6278 | Lower |
| SLOC | 154 | **151 ✅** | 159 | +3 | Higher | -5 | Lower | -8 | Lower |
| PLOC | 126 | **117 ✅** | 130 | +9 | Higher | -4 | Lower | -13 | Lower |
| LLOC | 100 | **92 ✅** | 105 | +8 | Higher | -5 | Lower | -13 | Lower |
| Comments (CLOC) | 16 | **29 ✅** | 17 | -13 | Lower | -1 | Lower | +12 | Higher |
| Blank Lines | 21 | **18 ✅** | 22 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 8 | **2 ✅** | 8 | +6 | Higher | 0 | Same | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **1 ✅** | 5 | +4 | Higher | 0 | Same | -4 | Lower |

## p00792

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **50.5392 ✅** | 49.4166 | 50.1707 | +1.1225 | Higher | +0.3684 | Higher | -0.7541 | Lower |
| MI (SEI) | 22.0858 | **25.4964 ✅** | 17.7969 | -3.4107 | Lower | +4.2888 | Higher | +7.6995 | Higher |
| Cyclomatic (sum) | 20 | 20 | 21 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 22 | 22 | 24 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 3169.1902 | **3158.4604 ✅** | 3362.6091 | +10.7298 | Higher | -193.4190 | Lower | -204.1487 | Lower |
| Halstead Effort | 168346.0041 | **144112.4978 ✅** | 196712.6339 | +24233.5063 | Higher | -28366.6297 | Lower | -52600.1360 | Lower |
| SLOC | 96 | 103 | **95 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| PLOC | 74 | 74 | 75 | 0 | Same | -1 | Lower | -1 | Lower |
| LLOC | 70 | **68 ✅** | 72 | +2 | Higher | -2 | Lower | -4 | Lower |
| Comments (CLOC) | 9 | **15 ✅** | 6 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00799

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.7823 | 44.6648 | **44.8556 ✅** | -2.8825 | Lower | -3.0733 | Lower | -0.1908 | Lower |
| MI (SEI) | 13.0592 | **16.8104 ✅** | 11.3494 | -3.7513 | Lower | +1.7097 | Higher | +5.4610 | Higher |
| Cyclomatic (sum) | 24 | **20 ✅** | 24 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 62 | **29 ✅** | 47 | +33 | Higher | +15 | Higher | -18 | Lower |
| Halstead Volume | 3417.6163 | 3313.6551 | **3155.2034 ✅** | +103.9612 | Higher | +262.4130 | Higher | +158.4517 | Higher |
| Halstead Effort | 179843.3420 | 172156.6560 | **170347.4173 ✅** | +7686.6860 | Higher | +9495.9247 | Higher | +1809.2387 | Higher |
| SLOC | 152 | 136 | **129 ✅** | +16 | Higher | +23 | Higher | +7 | Higher |
| PLOC | 117 | **99 ✅** | 108 | +18 | Higher | +9 | Higher | -9 | Lower |
| LLOC | 91 | **68 ✅** | 83 | +23 | Higher | +8 | Higher | -15 | Lower |
| Comments (CLOC) | **19 ✅** | 17 | 9 | +2 | Higher | +10 | Higher | +8 | Higher |
| Blank Lines | 18 | 22 | **14 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00811

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.3897 | 48.6741 | **52.0352 ✅** | +2.7156 | Higher | -0.6455 | Lower | -3.3611 | Lower |
| MI (SEI) | **19.6510 ✅** | 19.3781 | 12.6264 | +0.2730 | Higher | +7.0246 | Higher | +6.7516 | Higher |
| Cyclomatic (sum) | 21 | 28 | **19 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| Cognitive (sum) | 25 | 33 | **24 ✅** | -8 | Lower | +1 | Higher | +9 | Higher |
| Halstead Volume | 2749.0881 | **2481.6475 ✅** | 2836.5591 | +267.4406 | Higher | -87.4710 | Lower | -354.9116 | Lower |
| Halstead Effort | 142036.2188 | **107202.0006 ✅** | 155752.8810 | +34834.2182 | Higher | -13716.6622 | Lower | -48550.8804 | Lower |
| SLOC | 94 | 104 | **92 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| PLOC | 74 | 82 | 74 | -8 | Lower | 0 | Same | +8 | Higher |
| LLOC | 62 | **53 ✅** | 59 | +9 | Higher | +3 | Higher | -6 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 2 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 16 | **15 ✅** | 17 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00816

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.5304 ✅** | 42.8348 | 41.8584 | +9.6956 | Higher | +10.6720 | Higher | +0.9764 | Higher |
| MI (SEI) | **24.9674 ✅** | 11.6895 | 10.3777 | +13.2779 | Higher | +14.5897 | Higher | +1.3117 | Higher |
| Cyclomatic (sum) | **19 ✅** | 31 | 29 | -12 | Lower | -10 | Lower | +2 | Higher |
| Cognitive (sum) | **28 ✅** | 51 | 40 | -23 | Lower | -12 | Lower | +11 | Higher |
| Halstead Volume | **2333.5639 ✅** | 3104.7267 | 3488.0931 | -771.1628 | Lower | -1154.5292 | Lower | -383.3664 | Lower |
| Halstead Effort | **108266.5093 ✅** | 183075.3834 | 169593.4908 | -74808.8741 | Lower | -61326.9816 | Lower | +13481.8926 | Higher |
| SLOC | **95 ✅** | 133 | 140 | -38 | Lower | -45 | Lower | -7 | Lower |
| PLOC | **74 ✅** | 111 | 110 | -37 | Lower | -36 | Lower | +1 | Higher |
| LLOC | **52 ✅** | 78 | 85 | -26 | Lower | -33 | Lower | -7 | Lower |
| Comments (CLOC) | 9 | 12 | **13 ✅** | -3 | Lower | -4 | Lower | -1 | Lower |
| Blank Lines | 14 | **12 ✅** | 17 | +2 | Higher | -3 | Lower | -5 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |

## p00820

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.3656 ✅** | 62.8597 | 71.7452 | +9.5059 | Higher | +0.6203 | Higher | -8.8856 | Lower |
| MI (SEI) | **53.0818 ✅** | 45.3561 | 48.9393 | +7.7257 | Higher | +4.1424 | Higher | -3.5832 | Lower |
| Cyclomatic (sum) | 11 | 16 | **10 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Cognitive (sum) | 10 | 21 | **9 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| Halstead Volume | **1004.6724 ✅** | 1251.3856 | 1097.5901 | -246.7132 | Lower | -92.9177 | Lower | +153.7955 | Higher |
| Halstead Effort | **28298.2726 ✅** | 39749.8948 | 30095.2119 | -11451.6222 | Lower | -1796.9392 | Lower | +9654.6830 | Higher |
| SLOC | **41 ✅** | 64 | 42 | -23 | Lower | -1 | Lower | +22 | Higher |
| PLOC | 34 | 45 | **33 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| LLOC | 22 | 27 | 22 | -5 | Lower | 0 | Same | +5 | Higher |
| Comments (CLOC) | 4 | **10 ✅** | 3 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | **5 ✅** | 11 | 6 | -6 | Lower | -1 | Lower | +5 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 5 | **1 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |

## p00837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.3209 | **46.9816 ✅** | 41.2047 | -1.6607 | Lower | +4.1162 | Higher | +5.7769 | Higher |
| MI (SEI) | 19.1390 | **19.3644 ✅** | 8.5022 | -0.2254 | Lower | +10.6368 | Higher | +10.8622 | Higher |
| Cyclomatic (sum) | 17 | **16 ✅** | 20 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | **27 ✅** | 29 | 29 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | 3660.3964 | **3377.7490 ✅** | 4652.9386 | +282.6473 | Higher | -992.5422 | Lower | -1275.1895 | Lower |
| Halstead Effort | 184608.2922 | **141516.0367 ✅** | 279176.3132 | +43092.2556 | Higher | -94568.0210 | Lower | -137660.2766 | Lower |
| SLOC | 132 | **124 ✅** | 151 | +8 | Higher | -19 | Lower | -27 | Lower |
| PLOC | 95 | **89 ✅** | 115 | +6 | Higher | -20 | Lower | -26 | Lower |
| LLOC | 81 | **73 ✅** | 113 | +8 | Higher | -32 | Lower | -40 | Lower |
| Comments (CLOC) | **19 ✅** | 15 | 14 | +4 | Higher | +5 | Higher | +1 | Higher |
| Blank Lines | 21 | 21 | 23 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **41.3133 ✅** | 40.3061 | 39.7818 | +1.0072 | Higher | +1.5315 | Higher | +0.5244 | Higher |
| MI (SEI) | 5.0507 | **8.9437 ✅** | 0.7056 | -3.8930 | Lower | +4.3451 | Higher | +8.2381 | Higher |
| Cyclomatic (sum) | **20 ✅** | 27 | 21 | -7 | Lower | -1 | Lower | +6 | Higher |
| Cognitive (sum) | **18 ✅** | 29 | 21 | -11 | Lower | -3 | Lower | +8 | Higher |
| Halstead Volume | **4117.0409 ✅** | 4412.0115 | 4701.3825 | -294.9706 | Lower | -584.3416 | Lower | -289.3710 | Lower |
| Halstead Effort | **208085.0160 ✅** | 229684.1263 | 270830.4591 | -21599.1103 | Lower | -62745.4430 | Lower | -41146.3327 | Lower |
| SLOC | 156 | **147 ✅** | 162 | +9 | Higher | -6 | Lower | -15 | Lower |
| PLOC | 126 | **115 ✅** | 133 | +11 | Higher | -7 | Lower | -18 | Lower |
| LLOC | **48 ✅** | 59 | 67 | -11 | Lower | -19 | Lower | -8 | Lower |
| Comments (CLOC) | 10 | **15 ✅** | 8 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 20 | **17 ✅** | 21 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | 7 | **4 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p00855

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.6483 ✅** | 67.3538 | 65.9662 | +0.2945 | Higher | +1.6821 | Higher | +1.3876 | Higher |
| MI (SEI) | **54.9505 ✅** | 50.5449 | 47.7949 | +4.4056 | Higher | +7.1556 | Higher | +2.7500 | Higher |
| Cyclomatic (sum) | 11 | 12 | 11 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 12 | 13 | 12 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **996.6582 ✅** | 1009.0900 | 1167.2783 | -12.4318 | Lower | -170.6202 | Lower | -158.1884 | Lower |
| Halstead Effort | **28031.0107 ✅** | 28607.7007 | 33888.7260 | -576.6900 | Lower | -5857.7153 | Lower | -5281.0253 | Lower |
| SLOC | 55 | 55 | 58 | 0 | Same | -3 | Lower | -3 | Lower |
| PLOC | **37 ✅** | 40 | 41 | -3 | Lower | -4 | Lower | -1 | Lower |
| LLOC | **26 ✅** | 27 | 29 | -1 | Lower | -3 | Lower | -2 | Lower |
| Comments (CLOC) | **11 ✅** | 8 | 8 | +3 | Higher | +3 | Higher | 0 | Same |
| Blank Lines | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00856

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.8127 ✅** | 51.5220 | 51.8179 | +1.2907 | Higher | +0.9948 | Higher | -0.2959 | Lower |
| MI (SEI) | 24.8022 | **28.4004 ✅** | 18.1850 | -3.5982 | Lower | +6.6172 | Higher | +10.2154 | Higher |
| Cyclomatic (sum) | 24 | **21 ✅** | 25 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 23 | 23 | **21 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | **2802.2137 ✅** | 2964.9843 | 3011.4288 | -162.7706 | Lower | -209.2151 | Lower | -46.4445 | Lower |
| Halstead Effort | **152195.2317 ✅** | 163074.1352 | 175980.3677 | -10878.9036 | Lower | -23785.1360 | Lower | -12906.2325 | Lower |
| SLOC | **82 ✅** | 91 | 84 | -9 | Lower | -2 | Lower | +7 | Higher |
| PLOC | 66 | **62 ✅** | 69 | +4 | Higher | -3 | Lower | -7 | Lower |
| LLOC | **38 ✅** | 44 | 40 | -6 | Lower | -2 | Lower | +4 | Higher |
| Comments (CLOC) | 7 | **13 ✅** | 4 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 12 | 16 | 12 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **2 ✅** | 6 | 4 | -4 | Lower | -2 | Lower | +2 | Higher |

## p00866

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **46.3486 ✅** | 35.7783 | 46.1721 | +10.5703 | Higher | +0.1766 | Higher | -10.3937 | Lower |
| MI (SEI) | **17.3112 ✅** | 11.0826 | 8.7536 | +6.2286 | Higher | +8.5576 | Higher | +2.3290 | Higher |
| Cyclomatic (sum) | **18 ✅** | 31 | 20 | -13 | Lower | -2 | Lower | +11 | Higher |
| Cognitive (sum) | **25 ✅** | 39 | 29 | -14 | Lower | -4 | Lower | +10 | Higher |
| Halstead Volume | **3768.8886 ✅** | 4780.5428 | 4070.3997 | -1011.6542 | Lower | -301.5111 | Lower | +710.1431 | Higher |
| Halstead Effort | **181083.3190 ✅** | 298750.2597 | 210833.9833 | -117666.9408 | Lower | -29750.6643 | Lower | +87916.2765 | Higher |
| SLOC | 121 | 179 | **116 ✅** | -58 | Lower | +5 | Higher | +63 | Higher |
| PLOC | **94 ✅** | 123 | 96 | -29 | Lower | -2 | Lower | +27 | Higher |
| LLOC | **60 ✅** | 96 | 61 | -36 | Lower | -1 | Lower | +35 | Higher |
| Comments (CLOC) | 13 | **36 ✅** | 5 | -23 | Lower | +8 | Higher | +31 | Higher |
| Blank Lines | 15 | 23 | 15 | -8 | Lower | 0 | Same | +8 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p00867

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 6.8085 | **18.7165 ✅** | 5.8014 | -11.9081 | Lower | +1.0071 | Higher | +12.9151 | Higher |
| MI (SEI) | -39.2737 | **-17.3217 ✅** | -47.7758 | -21.9520 | Lower | +8.5021 | Higher | +30.4541 | Higher |
| Cyclomatic (sum) | 85 | **67 ✅** | 92 | +18 | Higher | -7 | Lower | -25 | Lower |
| Cognitive (sum) | 103 | 135 | **95 ✅** | -32 | Lower | +8 | Higher | +40 | Higher |
| Halstead Volume | 13534.6306 | **8592.5619 ✅** | 13657.5409 | +4942.0687 | Higher | -122.9103 | Lower | -5064.9790 | Lower |
| Halstead Effort | 1557523.6405 | **591250.0909 ✅** | 1518373.7941 | +966273.5496 | Higher | +39149.8464 | Higher | -927123.7032 | Lower |
| SLOC | 356 | **255 ✅** | 342 | +101 | Higher | +14 | Higher | -87 | Lower |
| PLOC | 305 | **199 ✅** | 306 | +106 | Higher | -1 | Lower | -107 | Lower |
| LLOC | 207 | **148 ✅** | 198 | +59 | Higher | +9 | Higher | -50 | Lower |
| Comments (CLOC) | 20 | **28 ✅** | 6 | -8 | Lower | +14 | Higher | +22 | Higher |
| Blank Lines | 37 | **30 ✅** | 33 | +7 | Higher | +4 | Higher | -3 | Lower |
| Functions | 12 | **3 ✅** | 16 | +9 | Higher | -4 | Lower | -13 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 16 | **4 ✅** | 17 | +12 | Higher | -1 | Lower | -13 | Lower |

## p00870

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.1554 ✅** | 42.2456 | 48.7882 | +10.9098 | Higher | +4.3672 | Higher | -6.5426 | Lower |
| MI (SEI) | 22.5996 | 11.2566 | **23.2796 ✅** | +11.3430 | Higher | -0.6800 | Lower | -12.0230 | Lower |
| Cyclomatic (sum) | **19 ✅** | 36 | 24 | -17 | Lower | -5 | Lower | +12 | Higher |
| Cognitive (sum) | **32 ✅** | 50 | 41 | -18 | Lower | -9 | Lower | +9 | Higher |
| Halstead Volume | **2626.5162 ✅** | 3555.8798 | 2986.2524 | -929.3635 | Lower | -359.7362 | Lower | +569.6273 | Higher |
| Halstead Effort | **117008.7237 ✅** | 218535.9318 | 157728.4235 | -101527.2080 | Lower | -40719.6998 | Lower | +60807.5083 | Higher |
| SLOC | **88 ✅** | 123 | 103 | -35 | Lower | -15 | Lower | +20 | Higher |
| PLOC | **69 ✅** | 98 | 78 | -29 | Lower | -9 | Lower | +20 | Higher |
| LLOC | **51 ✅** | 71 | 54 | -20 | Lower | -3 | Lower | +17 | Higher |
| Comments (CLOC) | 6 | 11 | **13 ✅** | -5 | Lower | -7 | Lower | -2 | Lower |
| Blank Lines | 13 | 14 | **12 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00874

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.8323 | 67.0305 | **70.0553 ✅** | +1.8019 | Higher | -1.2230 | Lower | -3.0248 | Lower |
| MI (SEI) | 45.9353 | 47.5612 | **52.8222 ✅** | -1.6259 | Lower | -6.8869 | Lower | -5.2610 | Lower |
| Cyclomatic (sum) | 9 | 12 | 9 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 1242.6959 | 1358.6195 | **1195.9332 ✅** | -115.9237 | Lower | +46.7627 | Higher | +162.6863 | Higher |
| Halstead Effort | 31440.2052 | 42456.8604 | **31182.8508 ✅** | -11016.6552 | Lower | +257.3545 | Higher | +11274.0096 | Higher |
| SLOC | 49 | 51 | **46 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| PLOC | 35 | 38 | **33 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| LLOC | 29 | 27 | 27 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 4 | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 10 | 10 | **8 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00889

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.1946 | **44.3133 ✅** | 41.3335 | -4.1187 | Lower | -1.1389 | Lower | +2.9798 | Higher |
| MI (SEI) | 7.4379 | **21.9582 ✅** | 7.4752 | -14.5203 | Lower | -0.0373 | Lower | +14.4830 | Higher |
| Cyclomatic (sum) | 28 | 27 | **26 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 44 | 32 | 32 | +12 | Higher | +12 | Higher | 0 | Same |
| Halstead Volume | 4405.3399 | **3066.8411 ✅** | 4215.6297 | +1338.4988 | Higher | +189.7102 | Higher | -1148.7886 | Lower |
| Halstead Effort | 285068.1247 | **158376.3038 ✅** | 281565.7387 | +126691.8209 | Higher | +3502.3860 | Higher | -123189.4349 | Lower |
| SLOC | 146 | **129 ✅** | 142 | +17 | Higher | +4 | Higher | -13 | Lower |
| PLOC | 119 | **86 ✅** | 115 | +33 | Higher | +4 | Higher | -29 | Lower |
| LLOC | 107 | **69 ✅** | 102 | +38 | Higher | +5 | Higher | -33 | Lower |
| Comments (CLOC) | 13 | **24 ✅** | 11 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | 19 | 19 | **17 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 2 | 3 | **0 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Exits | 4 | **0 ✅** | 5 | +4 | Higher | -1 | Lower | -5 | Lower |

## p00897

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **45.3504 ✅** | 40.8113 | 23.7332 | +4.5391 | Higher | +21.6173 | Higher | +17.0782 | Higher |
| MI (SEI) | **14.0051 ✅** | 8.9197 | -14.2047 | +5.0854 | Higher | +28.2098 | Higher | +23.1243 | Higher |
| Cyclomatic (sum) | **19 ✅** | 24 | 45 | -5 | Lower | -26 | Lower | -21 | Lower |
| Cognitive (sum) | **25 ✅** | 34 | 44 | -9 | Lower | -19 | Lower | -10 | Lower |
| Halstead Volume | **4151.3452 ✅** | 4981.9441 | 8155.9065 | -830.5989 | Lower | -4004.5613 | Lower | -3173.9624 | Lower |
| Halstead Effort | **189598.7499 ✅** | 250824.2791 | 564405.2061 | -61225.5292 | Lower | -374806.4562 | Lower | -313580.9270 | Lower |
| SLOC | **123 ✅** | 143 | 260 | -20 | Lower | -137 | Lower | -117 | Lower |
| PLOC | **97 ✅** | 108 | 214 | -11 | Lower | -117 | Lower | -106 | Lower |
| LLOC | **71 ✅** | 83 | 143 | -12 | Lower | -72 | Lower | -60 | Lower |
| Comments (CLOC) | 11 | 14 | **24 ✅** | -3 | Lower | -13 | Lower | -10 | Lower |
| Blank Lines | **17 ✅** | 23 | 24 | -6 | Lower | -7 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 8 | +1 | Higher | -6 | Lower | -7 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 2 | **0 ✅** | 12 | +2 | Higher | -10 | Lower | -12 | Lower |

## p00907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.0051 ✅** | 57.9441 | 57.3406 | +2.0610 | Higher | +2.6645 | Higher | +0.6035 | Higher |
| MI (SEI) | **35.4774 ✅** | 27.2479 | 28.5314 | +8.2295 | Higher | +6.9459 | Higher | -1.2836 | Lower |
| Cyclomatic (sum) | 19 | **17 ✅** | 20 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 28 | 27 | **25 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Halstead Volume | **1807.9765 ✅** | 1960.1630 | 1848.8669 | -152.1866 | Lower | -40.8905 | Lower | +111.2961 | Higher |
| Halstead Effort | 80283.9278 | **76838.3910 ✅** | 83991.3829 | +3445.5369 | Higher | -3707.4550 | Lower | -7152.9919 | Lower |
| SLOC | **65 ✅** | 74 | 75 | -9 | Lower | -10 | Lower | -1 | Lower |
| PLOC | **53 ✅** | 61 | 63 | -8 | Lower | -10 | Lower | -2 | Lower |
| LLOC | **39 ✅** | 46 | 46 | -7 | Lower | -7 | Lower | 0 | Same |
| Comments (CLOC) | **6 ✅** | 4 | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00924

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.9377 ✅** | 48.0289 | 52.0654 | +5.9088 | Higher | +1.8723 | Higher | -4.0365 | Lower |
| MI (SEI) | **28.4976 ✅** | 20.0103 | 19.4938 | +8.4873 | Higher | +9.0039 | Higher | +0.5165 | Higher |
| Cyclomatic (sum) | **18 ✅** | 21 | 19 | -3 | Lower | -1 | Lower | +2 | Higher |
| Cognitive (sum) | **23 ✅** | 27 | 24 | -4 | Lower | -1 | Lower | +3 | Higher |
| Halstead Volume | **2943.0542 ✅** | 3039.6635 | 3356.4392 | -96.6093 | Lower | -413.3850 | Lower | -316.7757 | Lower |
| Halstead Effort | **164742.5935 ✅** | 185246.7666 | 183071.8686 | -20504.1731 | Lower | -18329.2751 | Lower | +2174.8980 | Higher |
| SLOC | **82 ✅** | 112 | 87 | -30 | Lower | -5 | Lower | +25 | Higher |
| PLOC | **64 ✅** | 85 | 75 | -21 | Lower | -11 | Lower | +10 | Higher |
| LLOC | **57 ✅** | 77 | 61 | -20 | Lower | -4 | Lower | +16 | Higher |
| Comments (CLOC) | 9 | **12 ✅** | 5 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 9 | 19 | **7 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |

## p00949

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.7902 | **58.2692 ✅** | 34.8269 | -13.4790 | Lower | +9.9632 | Higher | +23.4423 | Higher |
| MI (SEI) | 10.7824 | **35.3526 ✅** | -2.8798 | -24.5701 | Lower | +13.6623 | Higher | +38.2324 | Higher |
| Cyclomatic (sum) | 25 | **14 ✅** | 31 | +11 | Higher | -6 | Lower | -17 | Lower |
| Cognitive (sum) | 26 | **11 ✅** | 34 | +15 | Higher | -8 | Lower | -23 | Lower |
| Halstead Volume | 3637.2286 | **2016.5341 ✅** | 5545.0277 | +1620.6945 | Higher | -1907.7990 | Lower | -3528.4936 | Lower |
| Halstead Effort | 221253.3028 | **64977.2103 ✅** | 488479.3437 | +156276.0925 | Higher | -267226.0408 | Lower | -423502.1334 | Lower |
| SLOC | 122 | **75 ✅** | 181 | +47 | Higher | -59 | Lower | -106 | Lower |
| PLOC | 96 | **52 ✅** | 142 | +44 | Higher | -46 | Lower | -90 | Lower |
| LLOC | 77 | **33 ✅** | 144 | +44 | Higher | -67 | Lower | -111 | Lower |
| Comments (CLOC) | 8 | 9 | **12 ✅** | -1 | Lower | -4 | Lower | -3 | Lower |
| Blank Lines | 18 | **14 ✅** | 27 | +4 | Higher | -9 | Lower | -13 | Lower |
| Functions | 6 | 4 | 4 | +2 | Higher | +2 | Higher | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 8 | 6 | **5 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p00971

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.1760 | **45.8657 ✅** | 44.6650 | -7.6897 | Lower | -6.4890 | Lower | +1.2007 | Higher |
| MI (SEI) | 5.3465 | **19.5375 ✅** | 11.1685 | -14.1910 | Lower | -5.8220 | Lower | +8.3691 | Higher |
| Cyclomatic (sum) | 21 | 19 | **15 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Cognitive (sum) | 28 | 27 | **19 ✅** | +1 | Higher | +9 | Higher | +8 | Higher |
| Halstead Volume | 5033.8409 | **3666.0348 ✅** | 4873.3835 | +1367.8061 | Higher | +160.4574 | Higher | -1207.3486 | Lower |
| Halstead Effort | 416194.1667 | **209349.8848 ✅** | 373864.9567 | +206844.2819 | Higher | +42329.2100 | Higher | -164515.0718 | Lower |
| SLOC | 175 | **124 ✅** | 129 | +51 | Higher | +46 | Higher | -5 | Lower |
| PLOC | 142 | **88 ✅** | 101 | +54 | Higher | +41 | Higher | -13 | Lower |
| LLOC | 106 | **79 ✅** | 86 | +27 | Higher | +20 | Higher | -7 | Lower |
| Comments (CLOC) | **18 ✅** | 17 | 10 | +1 | Higher | +8 | Higher | +7 | Higher |
| Blank Lines | **15 ✅** | 19 | 18 | -4 | Lower | -3 | Lower | +1 | Higher |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01050

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.6513 ✅** | 65.2622 | 70.2874 | +8.3892 | Higher | +3.3640 | Higher | -5.0252 | Lower |
| MI (SEI) | **65.2435 ✅** | 47.8047 | 55.9129 | +17.4388 | Higher | +9.3306 | Higher | -8.1082 | Lower |
| Cyclomatic (sum) | **7 ✅** | 11 | 9 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | **12 ✅** | 20 | 13 | -8 | Lower | -1 | Lower | +7 | Higher |
| Halstead Volume | **807.3024 ✅** | 1202.5542 | 939.3790 | -395.2518 | Lower | -132.0765 | Lower | +263.1752 | Higher |
| Halstead Effort | **18719.3245 ✅** | 33250.6229 | 26000.6675 | -14531.2983 | Lower | -7281.3430 | Lower | +7249.9554 | Higher |
| SLOC | **43 ✅** | 60 | 49 | -17 | Lower | -6 | Lower | +11 | Higher |
| PLOC | **29 ✅** | 42 | 36 | -13 | Lower | -7 | Lower | +6 | Higher |
| LLOC | **27 ✅** | 38 | 31 | -11 | Lower | -4 | Lower | +7 | Higher |
| Comments (CLOC) | **10 ✅** | 9 | 8 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 6 | 9 | **5 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p01065

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.7351 | **56.7495 ✅** | 46.8228 | -0.0144 | Lower | +9.9123 | Higher | +9.9267 | Higher |
| MI (SEI) | 34.4217 | **37.6776 ✅** | 19.9901 | -3.2559 | Lower | +14.4316 | Higher | +17.6875 | Higher |
| Cyclomatic (sum) | 16 | **13 ✅** | 26 | +3 | Higher | -10 | Lower | -13 | Lower |
| Cognitive (sum) | 43 | **33 ✅** | 55 | +10 | Higher | -12 | Lower | -22 | Lower |
| Halstead Volume | 2378.9574 | **2308.9586 ✅** | 3343.8340 | +69.9989 | Higher | -964.8766 | Lower | -1034.8754 | Lower |
| Halstead Effort | 104674.1260 | **93625.4535 ✅** | 191936.0705 | +11048.6725 | Higher | -87261.9445 | Lower | -98310.6169 | Lower |
| SLOC | **76 ✅** | 80 | 109 | -4 | Lower | -33 | Lower | -29 | Lower |
| PLOC | 56 | **55 ✅** | 81 | +1 | Higher | -25 | Lower | -26 | Lower |
| LLOC | **43 ✅** | 45 | 65 | -2 | Lower | -22 | Lower | -20 | Lower |
| Comments (CLOC) | 10 | **14 ✅** | 13 | -4 | Lower | -3 | Lower | +1 | Higher |
| Blank Lines | **11 ✅** | 14 | 15 | -3 | Lower | -4 | Lower | -1 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p01106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.5003 | 58.0330 | **60.0034 ✅** | +0.4673 | Higher | -1.5030 | Lower | -1.9703 | Lower |
| MI (SEI) | **40.4470 ✅** | 35.5743 | 39.9237 | +4.8728 | Higher | +0.5233 | Higher | -4.3494 | Lower |
| Cyclomatic (sum) | 18 | 18 | **14 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Cognitive (sum) | 14 | 14 | **13 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 2003.5960 | 1843.5402 | **1791.0827 ✅** | +160.0558 | Higher | +212.5133 | Higher | +52.4575 | Higher |
| Halstead Effort | 72773.4698 | 62311.6590 | **61574.5179 ✅** | +10461.8108 | Higher | +11198.9519 | Higher | +737.1410 | Higher |
| SLOC | 70 | 74 | 70 | -4 | Lower | 0 | Same | +4 | Higher |
| PLOC | 53 | 55 | **50 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| LLOC | 38 | **37 ✅** | 41 | +1 | Higher | -3 | Lower | -4 | Lower |
| Comments (CLOC) | **12 ✅** | 9 | 10 | +3 | Higher | +2 | Higher | -1 | Lower |
| Blank Lines | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01119

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.0116 | **44.9232 ✅** | 26.4867 | -13.9116 | Lower | +4.5249 | Higher | +18.4364 | Higher |
| MI (SEI) | -1.2355 | **16.9197 ✅** | -14.8235 | -18.1552 | Lower | +13.5881 | Higher | +31.7433 | Higher |
| Cyclomatic (sum) | 42 | **28 ✅** | 51 | +14 | Higher | -9 | Lower | -23 | Lower |
| Cognitive (sum) | 70 | **38 ✅** | 68 | +32 | Higher | +2 | Higher | -30 | Lower |
| Halstead Volume | 5043.5408 | **3026.8579 ✅** | 5699.8470 | +2016.6829 | Higher | -656.3062 | Lower | -2672.9891 | Lower |
| Halstead Effort | 320834.2758 | **138983.2252 ✅** | 384739.6738 | +181851.0506 | Higher | -63905.3980 | Lower | -245756.4486 | Lower |
| SLOC | 202 | **123 ✅** | 226 | +79 | Higher | -24 | Lower | -103 | Lower |
| PLOC | 159 | **97 ✅** | 193 | +62 | Higher | -34 | Lower | -96 | Lower |
| LLOC | 126 | **68 ✅** | 152 | +58 | Higher | -26 | Lower | -84 | Lower |
| Comments (CLOC) | **24 ✅** | 14 | 12 | +10 | Higher | +12 | Higher | +2 | Higher |
| Blank Lines | 19 | **12 ✅** | 22 | +7 | Higher | -3 | Lower | -10 | Lower |
| Functions | 6 | **1 ✅** | 8 | +5 | Higher | -2 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 12 | **0 ✅** | 15 | +12 | Higher | -3 | Lower | -15 | Lower |

## p01125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.4215 | **60.1767 ✅** | 59.2484 | -3.7552 | Lower | -2.8269 | Lower | +0.9283 | Higher |
| MI (SEI) | 31.9130 | **40.1737 ✅** | 29.3998 | -8.2608 | Lower | +2.5131 | Higher | +10.7739 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 14 | **10 ✅** | 14 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 2165.6437 | **1732.3683 ✅** | 1981.3590 | +433.2753 | Higher | +184.2847 | Higher | -248.9907 | Lower |
| Halstead Effort | 74065.0137 | **52735.3302 ✅** | 77219.4512 | +21329.6835 | Higher | -3154.4375 | Lower | -24484.1210 | Lower |
| SLOC | 81 | 70 | 70 | +11 | Higher | +11 | Higher | 0 | Same |
| PLOC | 60 | **50 ✅** | 54 | +10 | Higher | +6 | Higher | -4 | Lower |
| LLOC | 50 | **34 ✅** | 44 | +16 | Higher | +6 | Higher | -10 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 4 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 15 | **12 ✅** | 13 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01128

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.8493 ✅** | 30.7225 | 42.9526 | +18.1268 | Higher | +5.8967 | Higher | -12.2301 | Lower |
| MI (SEI) | **20.1238 ✅** | -6.0416 | 8.2173 | +26.1654 | Higher | +11.9065 | Higher | -14.2589 | Lower |
| Cyclomatic (sum) | **20 ✅** | 38 | 23 | -18 | Lower | -3 | Lower | +15 | Higher |
| Cognitive (sum) | 37 | **34 ✅** | 36 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | **3631.2422 ✅** | 7102.8710 | 4323.7316 | -3471.6287 | Lower | -692.4894 | Lower | +2779.1393 | Higher |
| Halstead Effort | **165013.1712 ✅** | 405780.1443 | 201617.4854 | -240766.9731 | Lower | -36604.3142 | Lower | +204162.6589 | Higher |
| SLOC | **102 ✅** | 195 | 133 | -93 | Lower | -31 | Lower | +62 | Higher |
| PLOC | **78 ✅** | 158 | 103 | -80 | Lower | -25 | Lower | +55 | Higher |
| LLOC | **53 ✅** | 79 | 59 | -26 | Lower | -6 | Lower | +20 | Higher |
| Comments (CLOC) | 10 | **16 ✅** | 9 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | **15 ✅** | 23 | 21 | -8 | Lower | -6 | Lower | +2 | Higher |
| Functions | **1 ✅** | 8 | 2 | -7 | Lower | -1 | Lower | +6 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **1 ✅** | 10 | 4 | -9 | Lower | -3 | Lower | +6 | Higher |

## p01138

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.1531 ✅** | 62.9235 | 45.3559 | +2.2296 | Higher | +19.7972 | Higher | +17.5676 | Higher |
| MI (SEI) | **41.4584 ✅** | 40.7049 | 10.6907 | +0.7534 | Higher | +30.7677 | Higher | +30.0143 | Higher |
| Cyclomatic (sum) | **10 ✅** | 17 | 32 | -7 | Lower | -22 | Lower | -15 | Lower |
| Cognitive (sum) | 8 | 8 | 36 | 0 | Same | -28 | Lower | -28 | Lower |
| Halstead Volume | **1506 ✅** | 1607.0902 | 3304.8281 | -101.0902 | Lower | -1798.8281 | Lower | -1697.7379 | Lower |
| Halstead Effort | **32948.3415 ✅** | 42333.1074 | 170105.1147 | -9384.7660 | Lower | -137156.7732 | Lower | -127772.0072 | Lower |
| SLOC | **57 ✅** | 58 | 110 | -1 | Lower | -53 | Lower | -52 | Lower |
| PLOC | 43 | **42 ✅** | 94 | +1 | Higher | -51 | Lower | -52 | Lower |
| LLOC | 31 | **29 ✅** | 60 | +2 | Higher | -29 | Lower | -31 | Lower |
| Comments (CLOC) | 5 | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | 11 | 12 | 11 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | 2 | 5 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 7 | 11 | -6 | Lower | -10 | Lower | -4 | Lower |

## p01150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.2719 | **39.2066 ✅** | 30.5709 | -4.9347 | Lower | +3.7010 | Higher | +8.6357 | Higher |
| MI (SEI) | 4.9629 | **15.8797 ✅** | -11.1042 | -10.9169 | Lower | +16.0670 | Higher | +26.9839 | Higher |
| Cyclomatic (sum) | 41 | **32 ✅** | 42 | +9 | Higher | -1 | Lower | -10 | Lower |
| Cognitive (sum) | **43 ✅** | 55 | 44 | -12 | Lower | -1 | Lower | +11 | Higher |
| Halstead Volume | 4908.8991 | **3937.0469 ✅** | 5842.5781 | +971.8523 | Higher | -933.6790 | Lower | -1905.5313 | Lower |
| Halstead Effort | 438462.8706 | **255794.4767 ✅** | 597241.3194 | +182668.3938 | Higher | -158778.4488 | Lower | -341446.8427 | Lower |
| SLOC | 169 | **152 ✅** | 198 | +17 | Higher | -29 | Lower | -46 | Lower |
| PLOC | 122 | **102 ✅** | 162 | +20 | Higher | -40 | Lower | -60 | Lower |
| LLOC | 84 | **56 ✅** | 143 | +28 | Higher | -59 | Lower | -87 | Lower |
| Comments (CLOC) | 23 | **30 ✅** | 9 | -7 | Lower | +14 | Higher | +21 | Higher |
| Blank Lines | 24 | **21 ✅** | 30 | +3 | Higher | -6 | Lower | -9 | Lower |
| Functions | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | 13 | 6 | -7 | Lower | 0 | Same | +7 | Higher |

## p01153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.6993 | 61.2232 | **61.4717 ✅** | -0.5239 | Lower | -0.7725 | Lower | -0.2485 | Lower |
| MI (SEI) | **38.6309 ✅** | 35.1866 | 31.1962 | +3.4443 | Higher | +7.4347 | Higher | +3.9904 | Higher |
| Cyclomatic (sum) | 14 | 14 | 16 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 32 | **27 ✅** | 38 | +5 | Higher | -6 | Lower | -11 | Lower |
| Halstead Volume | **1714.8085 ✅** | 1966.9449 | 1716.3884 | -252.1363 | Lower | -1.5799 | Lower | +250.5564 | Higher |
| Halstead Effort | **76258.5444 ✅** | 77764.5706 | 78792.9564 | -1506.0262 | Lower | -2534.4121 | Lower | -1028.3858 | Lower |
| SLOC | 68 | 63 | 63 | +5 | Higher | +5 | Higher | 0 | Same |
| PLOC | 49 | 49 | 50 | 0 | Same | -1 | Lower | -1 | Lower |
| LLOC | 40 | **38 ✅** | 39 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | **8 ✅** | 5 | 3 | +3 | Higher | +5 | Higher | +2 | Higher |
| Blank Lines | 11 | **9 ✅** | 10 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01184

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **38.4773 ✅** | 31.5430 | 30.8755 | +6.9343 | Higher | +7.6018 | Higher | +0.6675 | Higher |
| MI (SEI) | **8.0378 ✅** | -4.1355 | -12.5161 | +12.1733 | Higher | +20.5540 | Higher | +8.3806 | Higher |
| Cyclomatic (sum) | **32 ✅** | 42 | 41 | -10 | Lower | -9 | Lower | +1 | Higher |
| Cognitive (sum) | **47 ✅** | 73 | 49 | -26 | Lower | -2 | Lower | +24 | Higher |
| Halstead Volume | **4015.1289 ✅** | 5602.1591 | 6039.8604 | -1587.0302 | Lower | -2024.7315 | Lower | -437.7013 | Lower |
| Halstead Effort | **249629.1190 ✅** | 423443.1945 | 495268.5496 | -173814.0755 | Lower | -245639.4306 | Lower | -71825.3551 | Lower |
| SLOC | **158 ✅** | 189 | 195 | -31 | Lower | -37 | Lower | -6 | Lower |
| PLOC | **124 ✅** | 154 | 168 | -30 | Lower | -44 | Lower | -14 | Lower |
| LLOC | **97 ✅** | 124 | 129 | -27 | Lower | -32 | Lower | -5 | Lower |
| Comments (CLOC) | **18 ✅** | 16 | 7 | +2 | Higher | +11 | Higher | +9 | Higher |
| Blank Lines | **16 ✅** | 20 | 21 | -4 | Lower | -5 | Lower | -1 | Lower |
| Functions | 3 | **2 ✅** | 5 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 4 | **2 ✅** | 9 | +2 | Higher | -5 | Lower | -7 | Lower |

## p01255

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 0.2065 | **25.2816 ✅** | -5.6422 | -25.0750 | Lower | +5.8487 | Higher | +30.9238 | Higher |
| MI (SEI) | -50.8028 | **-12.1304 ✅** | -60.6885 | -38.6724 | Lower | +9.8857 | Higher | +48.5580 | Higher |
| Cyclomatic (sum) | 98 | **45 ✅** | 111 | +53 | Higher | -13 | Lower | -66 | Lower |
| Cognitive (sum) | 168 | **50 ✅** | 178 | +118 | Higher | -10 | Lower | -128 | Lower |
| Halstead Volume | 14415.8926 | **7572.1927 ✅** | 16002.5836 | +6843.6999 | Higher | -1586.6910 | Lower | -8430.3909 | Lower |
| Halstead Effort | 1458482.8814 | **475723.0057 ✅** | 1905960.6102 | +982759.8758 | Higher | -447477.7287 | Lower | -1430237.6045 | Lower |
| SLOC | 436 | **242 ✅** | 503 | +194 | Higher | -67 | Lower | -261 | Lower |
| PLOC | 381 | **187 ✅** | 439 | +194 | Higher | -58 | Lower | -252 | Lower |
| LLOC | 211 | **104 ✅** | 268 | +107 | Higher | -57 | Lower | -164 | Lower |
| Comments (CLOC) | 16 | **22 ✅** | 12 | -6 | Lower | +4 | Higher | +10 | Higher |
| Blank Lines | 42 | **34 ✅** | 55 | +8 | Higher | -13 | Lower | -21 | Lower |
| Functions | 17 | **8 ✅** | 21 | +9 | Higher | -4 | Lower | -13 | Lower |
| Closures | 3 | 3 | **0 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Exits | 20 | **9 ✅** | 25 | +11 | Higher | -5 | Lower | -16 | Lower |

## p01275

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.0489 ✅** | 45.8560 | 49.8336 | +11.1929 | Higher | +7.2153 | Higher | -3.9776 | Lower |
| MI (SEI) | **27.4798 ✅** | 13.6446 | 20.9304 | +13.8352 | Higher | +6.5494 | Higher | -7.2858 | Lower |
| Cyclomatic (sum) | **15 ✅** | 24 | 18 | -9 | Lower | -3 | Lower | +6 | Higher |
| Cognitive (sum) | **12 ✅** | 20 | 16 | -8 | Lower | -4 | Lower | +4 | Higher |
| Halstead Volume | **2340.9639 ✅** | 3436.1156 | 2912.2336 | -1095.1517 | Lower | -571.2697 | Lower | +523.8819 | Higher |
| Halstead Effort | **99568.9987 ✅** | 179341.1197 | 138267.7879 | -79772.1210 | Lower | -38698.7892 | Lower | +41073.3317 | Higher |
| SLOC | **76 ✅** | 118 | 106 | -42 | Lower | -30 | Lower | +12 | Higher |
| PLOC | **61 ✅** | 90 | 80 | -29 | Lower | -19 | Lower | +10 | Higher |
| LLOC | **40 ✅** | 56 | 63 | -16 | Lower | -23 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | 9 | **10 ✅** | -4 | Lower | -5 | Lower | -1 | Lower |
| Blank Lines | **12 ✅** | 20 | 16 | -8 | Lower | -4 | Lower | +4 | Higher |
| Functions | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 4 | 8 | **3 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |

## p01288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.1955 ✅** | 35.2715 | 41.0528 | +24.9240 | Higher | +19.1427 | Higher | -5.7813 | Lower |
| MI (SEI) | **40.9990 ✅** | 4.6485 | 8.5905 | +36.3505 | Higher | +32.4086 | Higher | -3.9420 | Lower |
| Cyclomatic (sum) | **12 ✅** | 37 | 31 | -25 | Lower | -19 | Lower | +6 | Higher |
| Cognitive (sum) | **14 ✅** | 24 | 34 | -10 | Lower | -20 | Lower | -10 | Lower |
| Halstead Volume | **1804.2581 ✅** | 4494.5406 | 3727.8318 | -2690.2825 | Lower | -1923.5737 | Lower | +766.7088 | Higher |
| Halstead Effort | **61654.0784 ✅** | 289245.4383 | 236903.7110 | -227591.3599 | Lower | -175249.6326 | Lower | +52341.7273 | Higher |
| SLOC | **71 ✅** | 173 | 140 | -102 | Lower | -69 | Lower | +33 | Higher |
| PLOC | **53 ✅** | 138 | 117 | -85 | Lower | -64 | Lower | +21 | Higher |
| LLOC | **48 ✅** | 84 | 84 | -36 | Lower | -36 | Lower | 0 | Same |
| Comments (CLOC) | 11 | **21 ✅** | 12 | -10 | Lower | -1 | Lower | +9 | Higher |
| Blank Lines | **8 ✅** | 15 | 11 | -7 | Lower | -3 | Lower | +4 | Higher |
| Functions | **1 ✅** | 10 | 5 | -9 | Lower | -4 | Lower | +5 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 11 | 4 | -11 | Lower | -4 | Lower | +7 | Higher |

## p01316

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.1052 ✅** | 57.4171 | 56.5016 | +1.6881 | Higher | +2.6036 | Higher | +0.9155 | Higher |
| MI (SEI) | **35.9422 ✅** | 31.0425 | 35.5215 | +4.8997 | Higher | +0.4207 | Higher | -4.4790 | Lower |
| Cyclomatic (sum) | 15 | 15 | 16 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 25 | **20 ✅** | 29 | +5 | Higher | -4 | Lower | -9 | Lower |
| Halstead Volume | **1948.6193 ✅** | 2011.3956 | 2040.2472 | -62.7762 | Lower | -91.6279 | Lower | -28.8516 | Lower |
| Halstead Effort | 64213.8035 | **62528.1660 ✅** | 62567.5798 | +1685.6375 | Higher | +1646.2237 | Higher | -39.4138 | Lower |
| SLOC | **71 ✅** | 78 | 81 | -7 | Lower | -10 | Lower | -3 | Lower |
| PLOC | **56 ✅** | 60 | 58 | -4 | Lower | -2 | Lower | +2 | Higher |
| LLOC | 47 | **40 ✅** | 48 | +7 | Higher | -1 | Lower | -8 | Lower |
| Comments (CLOC) | 8 | 7 | **12 ✅** | +1 | Higher | -4 | Lower | -5 | Lower |
| Blank Lines | **10 ✅** | 13 | 13 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p01317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.3302 | 41.2417 | **45.3328 ✅** | +4.0885 | Higher | -0.0026 | Lower | -4.0911 | Lower |
| MI (SEI) | **12.7842 ✅** | -4.0463 | 11.5905 | +16.8304 | Higher | +1.1936 | Higher | -15.6368 | Lower |
| Cyclomatic (sum) | 23 | 28 | 23 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 48 | 57 | **47 ✅** | -9 | Lower | +1 | Higher | +10 | Higher |
| Halstead Volume | 3973.6819 | 4391.5221 | **3868.6403 ✅** | -417.8402 | Lower | +105.0415 | Higher | +522.8817 | Higher |
| Halstead Effort | **185101.3277 ✅** | 226088.9540 | 207320.4363 | -40987.6263 | Lower | -22219.1086 | Lower | +18768.5177 | Higher |
| SLOC | **118 ✅** | 137 | 119 | -19 | Lower | -1 | Lower | +18 | Higher |
| PLOC | 97 | 115 | **94 ✅** | -18 | Lower | +3 | Higher | +21 | Higher |
| LLOC | 79 | 94 | **78 ✅** | -15 | Lower | +1 | Higher | +16 | Higher |
| Comments (CLOC) | **9 ✅** | 2 | 8 | +7 | Higher | +1 | Higher | -6 | Lower |
| Blank Lines | **13 ✅** | 20 | 17 | -7 | Lower | -4 | Lower | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01339

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **43.0927 ✅** | 40.1828 | 41.1106 | +2.9099 | Higher | +1.9822 | Higher | -0.9277 | Lower |
| MI (SEI) | **11.1311 ✅** | 8.1376 | 1.0949 | +2.9935 | Higher | +10.0362 | Higher | +7.0428 | Higher |
| Cyclomatic (sum) | **26 ✅** | 30 | 28 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | 48 | 51 | **31 ✅** | -3 | Lower | +17 | Higher | +20 | Higher |
| Halstead Volume | **4255.8356 ✅** | 4710.0117 | 4607.6318 | -454.1762 | Lower | -351.7962 | Lower | +102.3799 | Higher |
| Halstead Effort | **296844.5318 ✅** | 350652.2528 | 320733.0629 | -53807.7209 | Lower | -23888.5311 | Lower | +29919.1899 | Higher |
| SLOC | **127 ✅** | 139 | 136 | -12 | Lower | -9 | Lower | +3 | Higher |
| PLOC | **101 ✅** | 111 | 115 | -10 | Lower | -14 | Lower | -4 | Lower |
| LLOC | 87 | 88 | **84 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Comments (CLOC) | 11 | **13 ✅** | 5 | -2 | Lower | +6 | Higher | +8 | Higher |
| Blank Lines | **15 ✅** | 17 | 17 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 2 | 2 | 6 | 0 | Same | -4 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 4 | 4 | -3 | Lower | -3 | Lower | 0 | Same |

## p01353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.4071 ✅** | 53.0001 | 47.9656 | +1.4069 | Higher | +6.4415 | Higher | +5.0345 | Higher |
| MI (SEI) | **32.2610 ✅** | 28.3475 | 16.5169 | +3.9135 | Higher | +15.7441 | Higher | +11.8306 | Higher |
| Cyclomatic (sum) | 14 | 14 | 19 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 16 | 16 | 18 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | **2575.6493 ✅** | 2659.7056 | 3181.1447 | -84.0563 | Lower | -605.4954 | Lower | -521.4391 | Lower |
| Halstead Effort | **117548.2485 ✅** | 118045.6562 | 174651.0803 | -497.4077 | Lower | -57102.8317 | Lower | -56605.4241 | Lower |
| SLOC | **88 ✅** | 95 | 114 | -7 | Lower | -26 | Lower | -19 | Lower |
| PLOC | **66 ✅** | 71 | 93 | -5 | Lower | -27 | Lower | -22 | Lower |
| LLOC | **48 ✅** | 49 | 66 | -1 | Lower | -18 | Lower | -17 | Lower |
| Comments (CLOC) | **13 ✅** | 12 | 9 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | **13 ✅** | 15 | 14 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 3 | 5 | 0 | Same | -2 | Lower | -2 | Lower |

## p01368

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.4986 ✅** | 33.6077 | 36.0778 | +5.8909 | Higher | +3.4208 | Higher | -2.4702 | Lower |
| MI (SEI) | **7.8128 ✅** | -3.7690 | 4.1681 | +11.5818 | Higher | +3.6446 | Higher | -7.9372 | Lower |
| Cyclomatic (sum) | **35 ✅** | 37 | 37 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **55 ✅** | 74 | 57 | -19 | Lower | -2 | Lower | +17 | Higher |
| Halstead Volume | **4505.3460 ✅** | 5866.5250 | 4723.0589 | -1361.1790 | Lower | -217.7128 | Lower | +1143.4662 | Higher |
| Halstead Effort | 401018.3016 | 399306.3005 | **360593.5359 ✅** | +1712.0011 | Higher | +40424.7657 | Higher | +38712.7646 | Higher |
| SLOC | **137 ✅** | 176 | 162 | -39 | Lower | -25 | Lower | +14 | Higher |
| PLOC | **112 ✅** | 146 | 131 | -34 | Lower | -19 | Lower | +15 | Higher |
| LLOC | **83 ✅** | 102 | 95 | -19 | Lower | -12 | Lower | +7 | Higher |
| Comments (CLOC) | 13 | 12 | **17 ✅** | +1 | Higher | -4 | Lower | -5 | Lower |
| Blank Lines | 14 | 18 | 14 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |

## p01452

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.3397 | **52.8733 ✅** | 52.4239 | -0.5336 | Lower | -0.0842 | Lower | +0.4494 | Higher |
| MI (SEI) | **23.7201 ✅** | 23.2645 | 22.0893 | +0.4556 | Higher | +1.6308 | Higher | +1.1752 | Higher |
| Cyclomatic (sum) | 16 | 15 | **12 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Cognitive (sum) | 10 | 9 | **6 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Halstead Volume | **3271.3626 ✅** | 3309.6386 | 3841.7828 | -38.2760 | Lower | -570.4202 | Lower | -532.1441 | Lower |
| Halstead Effort | **177880.3415 ✅** | 225657.1803 | 185995.2762 | -47776.8388 | Lower | -8114.9348 | Lower | +39661.9040 | Higher |
| SLOC | 90 | **88 ✅** | 90 | +2 | Higher | 0 | Same | -2 | Lower |
| PLOC | 68 | **67 ✅** | 69 | +1 | Higher | -1 | Lower | -2 | Lower |
| LLOC | **52 ✅** | 53 | 58 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | **8 ✅** | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 14 | 15 | 14 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p01459

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.6685 | 30.6286 | **31.8789 ✅** | +1.0399 | Higher | -0.2103 | Lower | -1.2503 | Lower |
| MI (SEI) | -4.4732 | **3.6932 ✅** | -7.2488 | -8.1663 | Lower | +2.7756 | Higher | +10.9419 | Higher |
| Cyclomatic (sum) | **36 ✅** | 38 | 39 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | 69 | **54 ✅** | 73 | +15 | Higher | -4 | Lower | -19 | Lower |
| Halstead Volume | **6072.4846 ✅** | 6283.8531 | 6519.3549 | -211.3685 | Lower | -446.8704 | Lower | -235.5018 | Lower |
| Halstead Effort | 435917.6414 | **393982.9760 ✅** | 500404.5398 | +41934.6654 | Higher | -64486.8984 | Lower | -106421.5638 | Lower |
| SLOC | 199 | 204 | **184 ✅** | -5 | Lower | +15 | Higher | +20 | Higher |
| PLOC | 161 | **139 ✅** | 152 | +22 | Higher | +9 | Higher | -13 | Lower |
| LLOC | 103 | **89 ✅** | 116 | +14 | Higher | -13 | Lower | -27 | Lower |
| Comments (CLOC) | 17 | **39 ✅** | 11 | -22 | Lower | +6 | Higher | +28 | Higher |
| Blank Lines | 22 | 27 | 22 | -5 | Lower | 0 | Same | +5 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 7 | 2 | -6 | Lower | -1 | Lower | +5 | Higher |

## p01471

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.8889 ✅** | 55.2607 | 44.1095 | +1.6282 | Higher | +12.7794 | Higher | +11.1512 | Higher |
| MI (SEI) | 34.6704 | **36.7747 ✅** | 13.6823 | -2.1043 | Lower | +20.9880 | Higher | +23.0923 | Higher |
| Cyclomatic (sum) | 14 | 14 | 25 | 0 | Same | -11 | Lower | -11 | Lower |
| Cognitive (sum) | 18 | **16 ✅** | 31 | +2 | Higher | -13 | Lower | -15 | Lower |
| Halstead Volume | **1991.3790 ✅** | 2110.1144 | 3095.1563 | -118.7354 | Lower | -1103.7774 | Lower | -985.0420 | Lower |
| Halstead Effort | **70718.2386 ✅** | 86952.1511 | 155389.4819 | -16233.9125 | Lower | -84671.2433 | Lower | -68437.3308 | Lower |
| SLOC | **82 ✅** | 89 | 134 | -7 | Lower | -52 | Lower | -45 | Lower |
| PLOC | 61 | 61 | 106 | 0 | Same | -45 | Lower | -45 | Lower |
| LLOC | **44 ✅** | 45 | 77 | -1 | Lower | -33 | Lower | -32 | Lower |
| Comments (CLOC) | 11 | **17 ✅** | 13 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 13 | **12 ✅** | 16 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 1 | 1 | 5 | 0 | Same | -4 | Lower | -4 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 7 | -1 | Lower | -7 | Lower | -6 | Lower |

## p01521

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.0221 ✅** | 75.0612 | 73.9418 | +5.9609 | Higher | +7.0804 | Higher | +1.1194 | Higher |
| MI (SEI) | 71.3567 | 67.0700 | **71.7142 ✅** | +4.2867 | Higher | -0.3575 | Lower | -4.6442 | Lower |
| Cyclomatic (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **627.6685 ✅** | 872.1673 | 1034.8634 | -244.4987 | Lower | -407.1948 | Lower | -162.6961 | Lower |
| Halstead Effort | **10139.2610 ✅** | 14990.3751 | 24917.5700 | -4851.1140 | Lower | -14778.3090 | Lower | -9927.1950 | Lower |
| SLOC | **30 ✅** | 39 | 39 | -9 | Lower | -9 | Lower | 0 | Same |
| PLOC | **19 ✅** | 22 | 22 | -3 | Lower | -3 | Lower | 0 | Same |
| LLOC | **11 ✅** | 19 | 20 | -8 | Lower | -9 | Lower | -1 | Lower |
| Comments (CLOC) | 5 | 9 | **14 ✅** | -4 | Lower | -9 | Lower | -5 | Lower |
| Blank Lines | 6 | 8 | **4 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01529

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.0072 | **56.4251 ✅** | 44.0314 | -0.4179 | Lower | +11.9758 | Higher | +12.3936 | Higher |
| MI (SEI) | 28.6198 | **35.3609 ✅** | 16.7091 | -6.7412 | Lower | +11.9107 | Higher | +18.6518 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 31 | +1 | Higher | -16 | Lower | -17 | Lower |
| Cognitive (sum) | 28 | **23 ✅** | 47 | +5 | Higher | -19 | Lower | -24 | Lower |
| Halstead Volume | **2345.2249 ✅** | 2351.2693 | 3879.9882 | -6.0444 | Lower | -1534.7633 | Lower | -1528.7189 | Lower |
| Halstead Effort | 119665.1015 | **111111.8120 ✅** | 293928.9021 | +8553.2894 | Higher | -174263.8007 | Lower | -182817.0901 | Lower |
| SLOC | 81 | **80 ✅** | 115 | +1 | Higher | -34 | Lower | -35 | Lower |
| PLOC | 61 | **56 ✅** | 89 | +5 | Higher | -28 | Lower | -33 | Lower |
| LLOC | **52 ✅** | 53 | 79 | -1 | Lower | -27 | Lower | -26 | Lower |
| Comments (CLOC) | 7 | 12 | **14 ✅** | -5 | Lower | -7 | Lower | -2 | Lower |
| Blank Lines | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01545

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.7576 | **65.1328 ✅** | 52.1979 | -12.3751 | Lower | +0.5598 | Higher | +12.9349 | Higher |
| MI (SEI) | 32.8930 | **45.9526 ✅** | 22.6877 | -13.0597 | Lower | +10.2053 | Higher | +23.2650 | Higher |
| Cyclomatic (sum) | 20 | **12 ✅** | 20 | +8 | Higher | 0 | Same | -8 | Lower |
| Cognitive (sum) | 26 | **15 ✅** | 16 | +11 | Higher | +10 | Higher | -1 | Lower |
| Halstead Volume | 3135.6763 | **1637.7747 ✅** | 2916.4266 | +1497.9016 | Higher | +219.2497 | Higher | -1278.6519 | Lower |
| Halstead Effort | 221337.8491 | **69977.6462 ✅** | 196063.4084 | +151360.2029 | Higher | +25274.4407 | Higher | -126085.7622 | Lower |
| SLOC | 84 | **54 ✅** | 89 | +30 | Higher | -5 | Lower | -35 | Lower |
| PLOC | 62 | **40 ✅** | 70 | +22 | Higher | -8 | Lower | -30 | Lower |
| LLOC | 49 | **28 ✅** | 53 | +21 | Higher | -4 | Lower | -25 | Lower |
| Comments (CLOC) | **15 ✅** | 7 | 7 | +8 | Higher | +8 | Higher | 0 | Same |
| Blank Lines | 9 | **8 ✅** | 12 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 3 | 3 | 5 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01570

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.3319 ✅** | 50.9484 | 42.6471 | +1.3834 | Higher | +9.6848 | Higher | +8.3013 | Higher |
| MI (SEI) | **26.4793 ✅** | 23.1687 | 5.0951 | +3.3105 | Higher | +21.3842 | Higher | +18.0737 | Higher |
| Cyclomatic (sum) | **15 ✅** | 17 | 27 | -2 | Lower | -12 | Lower | -10 | Lower |
| Cognitive (sum) | **23 ✅** | 37 | 31 | -14 | Lower | -8 | Lower | +6 | Higher |
| Halstead Volume | **2893.6053 ✅** | 3039.2039 | 4436.0221 | -145.5986 | Lower | -1542.4169 | Lower | -1396.8183 | Lower |
| Halstead Effort | **139144.6707 ✅** | 153175.8745 | 265088.0966 | -14031.2037 | Lower | -125943.4259 | Lower | -111912.2222 | Lower |
| SLOC | **95 ✅** | 99 | 127 | -4 | Lower | -32 | Lower | -28 | Lower |
| PLOC | **72 ✅** | 75 | 110 | -3 | Lower | -38 | Lower | -35 | Lower |
| LLOC | **62 ✅** | 66 | 96 | -4 | Lower | -34 | Lower | -30 | Lower |
| Comments (CLOC) | **11 ✅** | 10 | 6 | +1 | Higher | +5 | Higher | +4 | Higher |
| Blank Lines | 16 | 14 | **12 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p01600

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.3251 | **64.7162 ✅** | 56.7795 | -6.3911 | Lower | +1.5455 | Higher | +7.9366 | Higher |
| MI (SEI) | 32.0837 | **46.1002 ✅** | 37.0861 | -14.0165 | Lower | -5.0024 | Lower | +9.0141 | Higher |
| Cyclomatic (sum) | 17 | **10 ✅** | 18 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 24 | **13 ✅** | 26 | +11 | Higher | -2 | Lower | -13 | Lower |
| Halstead Volume | 2166 | **1638 ✅** | 2250 | +528 | Higher | -84 | Lower | -612 | Lower |
| Halstead Effort | 117084.3333 | **51450 ✅** | 126875 | +65634.3333 | Higher | -9790.6667 | Lower | -75425 | Lower |
| SLOC | 70 | **57 ✅** | 75 | +13 | Higher | -5 | Lower | -18 | Lower |
| PLOC | 57 | **38 ✅** | 55 | +19 | Higher | +2 | Higher | -17 | Lower |
| LLOC | 44 | **28 ✅** | 40 | +16 | Higher | +4 | Higher | -12 | Lower |
| Comments (CLOC) | 6 | 8 | **12 ✅** | -2 | Lower | -6 | Lower | -4 | Lower |
| Blank Lines | **7 ✅** | 12 | 8 | -5 | Lower | -1 | Lower | +4 | Higher |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |

## p01601

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.0301 ✅** | 63.9346 | 71.3234 | +12.0955 | Higher | +4.7068 | Higher | -7.3887 | Lower |
| MI (SEI) | 52.4242 | 52.5832 | **55.0150 ✅** | -0.1590 | Lower | -2.5908 | Lower | -2.4318 | Lower |
| Cyclomatic (sum) | **8 ✅** | 15 | 10 | -7 | Lower | -2 | Lower | +5 | Higher |
| Cognitive (sum) | **5 ✅** | 14 | 6 | -9 | Lower | -1 | Lower | +8 | Higher |
| Halstead Volume | **780.6987 ✅** | 1174.2906 | 960.1154 | -393.5919 | Lower | -179.4167 | Lower | +214.1752 | Higher |
| Halstead Effort | **25680.8790 ✅** | 41687.3157 | 29503.5456 | -16006.4367 | Lower | -3822.6666 | Lower | +12183.7700 | Higher |
| SLOC | **37 ✅** | 62 | 45 | -25 | Lower | -8 | Lower | +17 | Higher |
| PLOC | **30 ✅** | 39 | 34 | -9 | Lower | -4 | Lower | +5 | Higher |
| LLOC | **22 ✅** | 27 | 23 | -5 | Lower | -1 | Lower | +4 | Higher |
| Comments (CLOC) | 2 | **15 ✅** | 6 | -13 | Lower | -4 | Lower | +9 | Higher |
| Blank Lines | **5 ✅** | 8 | 6 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |

## p01656

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **82.4925 ✅** | 68.8891 | 72.0746 | +13.6034 | Higher | +10.4179 | Higher | -3.1855 | Lower |
| MI (SEI) | **76.5794 ✅** | 54.6886 | 65.9799 | +21.8907 | Higher | +10.5995 | Higher | -11.2913 | Lower |
| Cyclomatic (sum) | **4 ✅** | 7 | 7 | -3 | Lower | -3 | Lower | 0 | Same |
| Cognitive (sum) | **3 ✅** | 8 | 8 | -5 | Lower | -5 | Lower | 0 | Same |
| Halstead Volume | **640.7628 ✅** | 1185.5369 | 948.8860 | -544.7741 | Lower | -308.1232 | Lower | +236.6509 | Higher |
| Halstead Effort | **8543.5035 ✅** | 24475.6004 | 19715.7417 | -15932.0969 | Lower | -11172.2382 | Lower | +4759.8588 | Higher |
| SLOC | **28 ✅** | 51 | 45 | -23 | Lower | -17 | Lower | +6 | Higher |
| PLOC | **18 ✅** | 31 | 26 | -13 | Lower | -8 | Lower | +5 | Higher |
| LLOC | **13 ✅** | 22 | 19 | -9 | Lower | -6 | Lower | +3 | Higher |
| Comments (CLOC) | 6 | 9 | **13 ✅** | -3 | Lower | -7 | Lower | -4 | Lower |
| Blank Lines | **5 ✅** | 11 | 7 | -6 | Lower | -2 | Lower | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p01658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.2731 | 59.9174 | **66.0892 ✅** | +3.3557 | Higher | -2.8161 | Lower | -6.1717 | Lower |
| MI (SEI) | 43.6113 | 37.8084 | **48.5168 ✅** | +5.8029 | Higher | -4.9055 | Lower | -10.7084 | Lower |
| Cyclomatic (sum) | 14 | 17 | **10 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| Cognitive (sum) | 28 | 32 | **14 ✅** | -4 | Lower | +14 | Higher | +18 | Higher |
| Halstead Volume | 1466.3476 | 1745.3657 | **1405.9517 ✅** | -279.0181 | Lower | +60.3959 | Higher | +339.4140 | Higher |
| Halstead Effort | 67000.8074 | 95995.1156 | **49823.4130 ✅** | -28994.3082 | Lower | +17177.3944 | Higher | +46171.7026 | Higher |
| SLOC | 61 | 68 | **55 ✅** | -7 | Lower | +6 | Higher | +13 | Higher |
| PLOC | 44 | 50 | **42 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| LLOC | 41 | 43 | **39 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Comments (CLOC) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 9 | 10 | **8 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p01668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **33.1152 ✅** | 29.6433 | 32.8408 | +3.4719 | Higher | +0.2745 | Higher | -3.1975 | Lower |
| MI (SEI) | -0.6377 | -7.7370 | **1.4941 ✅** | +7.0994 | Higher | -2.1318 | Lower | -9.2312 | Lower |
| Cyclomatic (sum) | **40 ✅** | 47 | 44 | -7 | Lower | -4 | Lower | +3 | Higher |
| Cognitive (sum) | **34 ✅** | 91 | 47 | -57 | Lower | -13 | Lower | +44 | Higher |
| Halstead Volume | 5748.9773 | 6365.4177 | **5557.7084 ✅** | -616.4405 | Lower | +191.2689 | Higher | +807.7094 | Higher |
| Halstead Effort | **428865.6078 ✅** | 557691.2817 | 442499.4458 | -128825.6739 | Lower | -13633.8380 | Lower | +115191.8359 | Higher |
| SLOC | 175 | 190 | **170 ✅** | -15 | Lower | +5 | Higher | +20 | Higher |
| PLOC | 138 | 157 | **132 ✅** | -19 | Lower | +6 | Higher | +25 | Higher |
| LLOC | 77 | 116 | **72 ✅** | -39 | Lower | +5 | Higher | +44 | Higher |
| Comments (CLOC) | 17 | 14 | **20 ✅** | +3 | Higher | -3 | Lower | -6 | Lower |
| Blank Lines | 25 | **19 ✅** | 25 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 10 | **7 ✅** | 13 | +3 | Higher | -3 | Lower | -6 | Lower |

## p01725

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.1181 ✅** | 27.2507 | 35.9544 | +11.8675 | Higher | +3.1637 | Higher | -8.7037 | Lower |
| MI (SEI) | **5.1441 ✅** | -2.3217 | -3.0219 | +7.4658 | Higher | +8.1660 | Higher | +0.7002 | Higher |
| Cyclomatic (sum) | **28 ✅** | 51 | 33 | -23 | Lower | -5 | Lower | +18 | Higher |
| Cognitive (sum) | 51 | 51 | 55 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | **4321.1756 ✅** | 6278.6240 | 4967.6471 | -1957.4484 | Lower | -646.4715 | Lower | +1310.9769 | Higher |
| Halstead Effort | **312882.4125 ✅** | 412654.3007 | 356466.3126 | -99771.8882 | Lower | -43583.9001 | Lower | +56187.9881 | Higher |
| SLOC | **157 ✅** | 209 | 170 | -52 | Lower | -13 | Lower | +39 | Higher |
| PLOC | **131 ✅** | 168 | 145 | -37 | Lower | -14 | Lower | +23 | Higher |
| LLOC | 101 | **68 ✅** | 110 | +33 | Higher | -9 | Lower | -42 | Lower |
| Comments (CLOC) | 13 | **33 ✅** | 9 | -20 | Lower | +4 | Higher | +24 | Higher |
| Blank Lines | **15 ✅** | 20 | 17 | -5 | Lower | -2 | Lower | +3 | Higher |
| Functions | **3 ✅** | 6 | 5 | -3 | Lower | -2 | Lower | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 5 | 5 | -3 | Lower | -3 | Lower | 0 | Same |

## p01767

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.4119 ✅** | 58.3940 | 56.8308 | +4.0179 | Higher | +5.5811 | Higher | +1.5632 | Higher |
| MI (SEI) | **36.7549 ✅** | 29.5888 | 32.6836 | +7.1660 | Higher | +4.0713 | Higher | -3.0948 | Lower |
| Cyclomatic (sum) | **11 ✅** | 13 | 14 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **12 ✅** | 13 | 14 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | **1878.3912 ✅** | 2238.5080 | 2261.7054 | -360.1167 | Lower | -383.3142 | Lower | -23.1975 | Lower |
| Halstead Effort | **70439.6704 ✅** | 91965.3683 | 104321.1631 | -21525.6980 | Lower | -33881.4928 | Lower | -12355.7948 | Lower |
| SLOC | **62 ✅** | 73 | 79 | -11 | Lower | -17 | Lower | -6 | Lower |
| PLOC | **48 ✅** | 59 | 59 | -11 | Lower | -11 | Lower | 0 | Same |
| LLOC | **35 ✅** | 42 | 42 | -7 | Lower | -7 | Lower | 0 | Same |
| Comments (CLOC) | 5 | 5 | **9 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | **9 ✅** | 11 | 12 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |

## p01768

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.8292 | **59.8879 ✅** | 54.6122 | -4.0587 | Lower | +1.2170 | Higher | +5.2758 | Higher |
| MI (SEI) | 33.5539 | **39.9041 ✅** | 24.7307 | -6.3503 | Lower | +8.8232 | Higher | +15.1735 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 15 | +3 | Higher | -3 | Lower | -6 | Lower |
| Cognitive (sum) | 15 | 15 | **12 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 2384.8577 | **2004.5892 ✅** | 2738.3689 | +380.2684 | Higher | -353.5113 | Lower | -733.7797 | Lower |
| Halstead Effort | 87948.8969 | **54759.5104 ✅** | 108544.2835 | +33189.3865 | Higher | -20595.3866 | Lower | -53784.7731 | Lower |
| SLOC | 85 | **73 ✅** | 84 | +12 | Higher | +1 | Higher | -11 | Lower |
| PLOC | 61 | **48 ✅** | 63 | +13 | Higher | -2 | Lower | -15 | Lower |
| LLOC | 45 | 42 | 42 | +3 | Higher | +3 | Higher | 0 | Same |
| Comments (CLOC) | **12 ✅** | 11 | 6 | +1 | Higher | +6 | Higher | +5 | Higher |
| Blank Lines | 14 | 14 | 15 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p01782

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **28.5226 ✅** | 25.0125 | 24.0303 | +3.5101 | Higher | +4.4923 | Higher | +0.9822 | Higher |
| MI (SEI) | **-13.7797 ✅** | -17.2640 | -25.5769 | +3.4843 | Higher | +11.7972 | Higher | +8.3130 | Higher |
| Cyclomatic (sum) | **38 ✅** | 49 | 40 | -11 | Lower | -2 | Lower | +9 | Higher |
| Cognitive (sum) | **47 ✅** | 78 | 51 | -31 | Lower | -4 | Lower | +27 | Higher |
| Halstead Volume | **7239.5587 ✅** | 7935.2941 | 8254.4544 | -695.7354 | Lower | -1014.8957 | Lower | -319.1603 | Lower |
| Halstead Effort | **502408.9169 ✅** | 703806.4806 | 712634.5612 | -201397.5637 | Lower | -210225.6444 | Lower | -8828.0807 | Lower |
| SLOC | **222 ✅** | 229 | 273 | -7 | Lower | -51 | Lower | -44 | Lower |
| PLOC | **190 ✅** | 195 | 237 | -5 | Lower | -47 | Lower | -42 | Lower |
| LLOC | **127 ✅** | 151 | 158 | -24 | Lower | -31 | Lower | -7 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 6 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 24 | 24 | 33 | 0 | Same | -9 | Lower | -9 | Lower |
| Functions | 10 | **5 ✅** | 10 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **3 ✅** | 5 | +6 | Higher | +4 | Higher | -2 | Lower |

## p01841

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.7135 | 35.0562 | **39.3357 ✅** | +2.6573 | Higher | -1.6222 | Lower | -4.2795 | Lower |
| MI (SEI) | 4.8041 | **7.3310 ✅** | 4.5489 | -2.5269 | Lower | +0.2552 | Higher | +2.7821 | Higher |
| Cyclomatic (sum) | 32 | 41 | **25 ✅** | -9 | Lower | +7 | Higher | +16 | Higher |
| Cognitive (sum) | **29 ✅** | 52 | 38 | -23 | Lower | -9 | Lower | +14 | Higher |
| Halstead Volume | 4471.6875 | **4463.8526 ✅** | 5025.3298 | +7.8350 | Higher | -553.6423 | Lower | -561.4772 | Lower |
| Halstead Effort | 230412.7634 | **222550.5667 ✅** | 244389.7224 | +7862.1967 | Higher | -13976.9589 | Lower | -21839.1556 | Lower |
| SLOC | 160 | 166 | **154 ✅** | -6 | Lower | +6 | Higher | +12 | Higher |
| PLOC | 125 | 123 | **122 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| LLOC | **75 ✅** | 85 | 85 | -10 | Lower | -10 | Lower | 0 | Same |
| Comments (CLOC) | 15 | **25 ✅** | 12 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 21 | **19 ✅** | 20 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | 7 | **3 ✅** | 0 | Same | +4 | Higher | +4 | Higher |

## p01847

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.8393 | **54.0639 ✅** | 40.7933 | -0.2246 | Lower | +13.0459 | Higher | +13.2706 | Higher |
| MI (SEI) | **24.6484 ✅** | 20.3331 | 0.2519 | +4.3153 | Higher | +24.3966 | Higher | +20.0813 | Higher |
| Cyclomatic (sum) | **16 ✅** | 17 | 30 | -1 | Lower | -14 | Lower | -13 | Lower |
| Cognitive (sum) | **18 ✅** | 21 | 42 | -3 | Lower | -24 | Lower | -21 | Lower |
| Halstead Volume | 2538.6455 | **2496.7973 ✅** | 3444.6571 | +41.8482 | Higher | -906.0115 | Lower | -947.8597 | Lower |
| Halstead Effort | 109904.7760 | **97022.2876 ✅** | 215210.9577 | +12882.4884 | Higher | -105306.1817 | Lower | -118188.6701 | Lower |
| SLOC | 89 | **87 ✅** | 148 | +2 | Higher | -59 | Lower | -61 | Lower |
| PLOC | 70 | **69 ✅** | 123 | +1 | Higher | -53 | Lower | -54 | Lower |
| LLOC | 54 | **53 ✅** | 107 | +1 | Higher | -53 | Lower | -54 | Lower |
| Comments (CLOC) | **7 ✅** | 4 | 5 | +3 | Higher | +2 | Higher | -1 | Lower |
| Blank Lines | 14 | 14 | 20 | 0 | Same | -6 | Lower | -6 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 5 | 0 | Same | -4 | Lower | -4 | Lower |

## p01907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.3834 | **49.1293 ✅** | 34.3597 | -14.7459 | Lower | +0.0236 | Higher | +14.7695 | Higher |
| MI (SEI) | -1.1369 | **22.6516 ✅** | -4.9119 | -23.7885 | Lower | +3.7751 | Higher | +27.5636 | Higher |
| Cyclomatic (sum) | 40 | **21 ✅** | 30 | +19 | Higher | +10 | Higher | -9 | Lower |
| Cognitive (sum) | 55 | 35 | **32 ✅** | +20 | Higher | +23 | Higher | +3 | Higher |
| Halstead Volume | 5115.7693 | **3292.0435 ✅** | 5362.5734 | +1823.7258 | Higher | -246.8042 | Lower | -2070.5300 | Lower |
| Halstead Effort | 315372.1289 | **181315.6251 ✅** | 342869.5382 | +134056.5038 | Higher | -27497.4093 | Lower | -161553.9131 | Lower |
| SLOC | 168 | **102 ✅** | 191 | +66 | Higher | -23 | Lower | -89 | Lower |
| PLOC | 138 | **79 ✅** | 157 | +59 | Higher | -19 | Lower | -78 | Lower |
| LLOC | 107 | **62 ✅** | 131 | +45 | Higher | -24 | Lower | -69 | Lower |
| Comments (CLOC) | **13 ✅** | 12 | 11 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 17 | **12 ✅** | 23 | +5 | Higher | -6 | Lower | -11 | Lower |
| Functions | 6 | **1 ✅** | 5 | +5 | Higher | +1 | Higher | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |

## p01916

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.0496 ✅** | 87.4953 | 85.7932 | +1.5543 | Higher | +3.2563 | Higher | +1.7020 | Higher |
| MI (SEI) | **88.1492 ✅** | 77.3718 | 86.7987 | +10.7774 | Higher | +1.3505 | Higher | -9.4269 | Lower |
| Cyclomatic (sum) | 4 | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 517.9694 | **472.3135 ✅** | 505.3170 | +45.6559 | Higher | +12.6524 | Higher | -33.0035 | Lower |
| Halstead Effort | 6403.9855 | 5926.4096 | **5684.8160 ✅** | +477.5760 | Higher | +719.1695 | Higher | +241.5936 | Higher |
| SLOC | **20 ✅** | 23 | 25 | -3 | Lower | -5 | Lower | -2 | Lower |
| PLOC | 13 | 14 | 13 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 5 | 3 | **8 ✅** | +2 | Higher | -3 | Lower | -5 | Lower |
| Blank Lines | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01917

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.5147 ✅** | 44.0988 | 51.2382 | +7.4159 | Higher | +0.2764 | Higher | -7.1395 | Lower |
| MI (SEI) | **21.5811 ✅** | 15.5371 | 14.5165 | +6.0441 | Higher | +7.0647 | Higher | +1.0206 | Higher |
| Cyclomatic (sum) | **22 ✅** | 27 | 23 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | 51 | **37 ✅** | 57 | +14 | Higher | -6 | Lower | -20 | Lower |
| Halstead Volume | **2745.6166 ✅** | 3614.8965 | 2966.5730 | -869.2799 | Lower | -220.9564 | Lower | +648.3235 | Higher |
| Halstead Effort | **154616.5270 ✅** | 189071.9967 | 178983.2365 | -34455.4697 | Lower | -24366.7095 | Lower | +10088.7602 | Higher |
| SLOC | 92 | 124 | **90 ✅** | -32 | Lower | +2 | Higher | +34 | Higher |
| PLOC | 76 | 90 | 76 | -14 | Lower | 0 | Same | +14 | Higher |
| LLOC | **60 ✅** | 66 | 62 | -6 | Lower | -2 | Lower | +4 | Higher |
| Comments (CLOC) | 7 | **14 ✅** | 3 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | **10 ✅** | 20 | 11 | -10 | Lower | -1 | Lower | +9 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.5220 | 46.3471 | **53.0845 ✅** | +6.1749 | Higher | -0.5625 | Lower | -6.7374 | Lower |
| MI (SEI) | **21.5951 ✅** | 11.2043 | 20.9645 | +10.3909 | Higher | +0.6306 | Higher | -9.7602 | Lower |
| Cyclomatic (sum) | 16 | 23 | 16 | -7 | Lower | 0 | Same | +7 | Higher |
| Cognitive (sum) | 15 | 21 | **14 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Halstead Volume | 3639.2479 | 4066.7606 | **3514.5506 ✅** | -427.5127 | Lower | +124.6973 | Higher | +552.2100 | Higher |
| Halstead Effort | 290897.2143 | 347238.7892 | **255204.2996 ✅** | -56341.5750 | Lower | +35692.9146 | Higher | +92034.4896 | Higher |
| SLOC | 86 | 110 | **84 ✅** | -24 | Lower | +2 | Higher | +26 | Higher |
| PLOC | 71 | 89 | **69 ✅** | -18 | Lower | +2 | Higher | +20 | Higher |
| LLOC | 61 | 78 | **54 ✅** | -17 | Lower | +7 | Higher | +24 | Higher |
| Comments (CLOC) | 6 | 6 | 5 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | **9 ✅** | 15 | 10 | -6 | Lower | -1 | Lower | +5 | Higher |
| Functions | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 8 | 4 | -4 | Lower | 0 | Same | +4 | Higher |

## p02038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.1885 | 69.2422 | **72.0402 ✅** | +0.9463 | Higher | -1.8517 | Lower | -2.7980 | Lower |
| MI (SEI) | 57.8627 | 52.0564 | **58.1016 ✅** | +5.8063 | Higher | -0.2388 | Lower | -6.0452 | Lower |
| Cyclomatic (sum) | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 9 | 12 | 9 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 1205.0057 | 1171.5962 | **1144.0489 ✅** | +33.4095 | Higher | +60.9568 | Higher | +27.5473 | Higher |
| Halstead Effort | 30928.4792 | **28486.5247 ✅** | 32605.3942 | +2441.9544 | Higher | -1676.9151 | Lower | -4118.8695 | Lower |
| SLOC | 43 | 46 | **39 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| PLOC | 31 | 33 | **28 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| LLOC | 17 | 19 | **16 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Comments (CLOC) | **8 ✅** | 6 | 6 | +2 | Higher | +2 | Higher | 0 | Same |
| Blank Lines | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **46.8941 ✅** | 41.3234 | 45.6505 | +5.5706 | Higher | +1.2435 | Higher | -4.3271 | Lower |
| MI (SEI) | **17.4543 ✅** | 3.5382 | 13.3714 | +13.9161 | Higher | +4.0829 | Higher | -9.8332 | Lower |
| Cyclomatic (sum) | 36 | 40 | **35 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Cognitive (sum) | 22 | 28 | 22 | -6 | Lower | 0 | Same | +6 | Higher |
| Halstead Volume | 3361.3850 | 3841.7828 | **3356.4048 ✅** | -480.3978 | Lower | +4.9802 | Higher | +485.3780 | Higher |
| Halstead Effort | 218124.6572 | 209377.1610 | **207677.5461 ✅** | +8747.4962 | Higher | +10447.1111 | Higher | +1699.6150 | Higher |
| SLOC | **94 ✅** | 120 | 103 | -26 | Lower | -9 | Lower | +17 | Higher |
| PLOC | **79 ✅** | 97 | 87 | -18 | Lower | -8 | Lower | +10 | Higher |
| LLOC | **44 ✅** | 52 | 56 | -8 | Lower | -12 | Lower | -4 | Lower |
| Comments (CLOC) | **8 ✅** | 5 | 7 | +3 | Higher | +1 | Higher | -2 | Lower |
| Blank Lines | 9 | 21 | 9 | -12 | Lower | 0 | Same | +12 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 3 | **1 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |

## p02046

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.2981 | 66.1381 | **68.0857 ✅** | -6.8400 | Lower | -8.7876 | Lower | -1.9476 | Lower |
| MI (SEI) | 38.6265 | 44.8626 | **49.9887 ✅** | -6.2361 | Lower | -11.3622 | Lower | -5.1261 | Lower |
| Cyclomatic (sum) | 22 | 18 | **14 ✅** | +4 | Higher | +8 | Higher | +4 | Higher |
| Cognitive (sum) | 14 | 12 | **8 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Halstead Volume | 1505.9566 | 1236.9954 | **1226.2308 ✅** | +268.9612 | Higher | +279.7259 | Higher | +10.7647 | Higher |
| Halstead Effort | 41189.9487 | 37771.0153 | **36315.2954 ✅** | +3418.9334 | Higher | +4874.6532 | Higher | +1455.7199 | Higher |
| SLOC | 69 | 51 | **48 ✅** | +18 | Higher | +21 | Higher | +3 | Higher |
| PLOC | 53 | 40 | **35 ✅** | +13 | Higher | +18 | Higher | +5 | Higher |
| LLOC | 20 | **14 ✅** | 27 | +6 | Higher | -7 | Lower | -13 | Lower |
| Comments (CLOC) | **9 ✅** | 5 | 6 | +4 | Higher | +3 | Higher | -1 | Lower |
| Blank Lines | 11 | 9 | **8 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.0295 | **48.1733 ✅** | 22.8988 | -4.1437 | Lower | +21.1307 | Higher | +25.2745 | Higher |
| MI (SEI) | 15.0726 | **24.6358 ✅** | -21.2225 | -9.5632 | Lower | +36.2951 | Higher | +45.8583 | Higher |
| Cyclomatic (sum) | 21 | **16 ✅** | 43 | +5 | Higher | -22 | Lower | -27 | Lower |
| Cognitive (sum) | **17 ✅** | 18 | 40 | -1 | Lower | -23 | Lower | -22 | Lower |
| Halstead Volume | 4122.8912 | **3792.7121 ✅** | 8587.8095 | +330.1790 | Higher | -4464.9183 | Lower | -4795.0974 | Lower |
| Halstead Effort | 178965.1907 | **172716.5538 ✅** | 660730.1256 | +6248.6369 | Higher | -481764.9349 | Lower | -488013.5718 | Lower |
| SLOC | 130 | **111 ✅** | 277 | +19 | Higher | -147 | Lower | -166 | Lower |
| PLOC | 99 | **80 ✅** | 238 | +19 | Higher | -139 | Lower | -158 | Lower |
| LLOC | 64 | **59 ✅** | 142 | +5 | Higher | -78 | Lower | -83 | Lower |
| Comments (CLOC) | 15 | **18 ✅** | 14 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 18 | **17 ✅** | 28 | +1 | Higher | -10 | Lower | -11 | Lower |
| Functions | 3 | **1 ✅** | 11 | +2 | Higher | -8 | Lower | -10 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | **1 ✅** | 9 | +1 | Higher | -7 | Lower | -8 | Lower |

## p02061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.9343 | **51.9575 ✅** | 47.1188 | -6.0232 | Lower | -1.1845 | Lower | +4.8387 | Higher |
| MI (SEI) | 18.4446 | **31.3236 ✅** | 8.2807 | -12.8791 | Lower | +10.1638 | Higher | +23.0429 | Higher |
| Cyclomatic (sum) | 24 | 22 | **19 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | 36 | 39 | **28 ✅** | -3 | Lower | +8 | Higher | +11 | Higher |
| Halstead Volume | 3384.7422 | **2358.0792 ✅** | 3362.3590 | +1026.6630 | Higher | +22.3832 | Higher | -1004.2798 | Lower |
| Halstead Effort | 223520.7092 | **122004.9662 ✅** | 211037.4715 | +101515.7430 | Higher | +12483.2377 | Higher | -89032.5052 | Lower |
| SLOC | 118 | **94 ✅** | 118 | +24 | Higher | 0 | Same | -24 | Lower |
| PLOC | 93 | **67 ✅** | 102 | +26 | Higher | -9 | Lower | -35 | Lower |
| LLOC | 75 | **49 ✅** | 82 | +26 | Higher | -7 | Lower | -33 | Lower |
| Comments (CLOC) | 14 | **16 ✅** | 4 | -2 | Lower | +10 | Higher | +12 | Higher |
| Blank Lines | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02104

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.4591 | **67.1753 ✅** | 50.4745 | -6.7162 | Lower | +9.9846 | Higher | +16.7008 | Higher |
| MI (SEI) | 43.7010 | **45.9613 ✅** | 30.5959 | -2.2603 | Lower | +13.1051 | Higher | +15.3654 | Higher |
| Cyclomatic (sum) | 12 | 12 | 19 | 0 | Same | -7 | Lower | -7 | Lower |
| Cognitive (sum) | 18 | **15 ✅** | 31 | +3 | Higher | -13 | Lower | -16 | Lower |
| Halstead Volume | 1792.5422 | **1405.3799 ✅** | 2693.5892 | +387.1623 | Higher | -901.0471 | Lower | -1288.2093 | Lower |
| Halstead Effort | 75286.7717 | **44796.4840 ✅** | 166542.6517 | +30490.2877 | Higher | -91255.8799 | Lower | -121746.1677 | Lower |
| SLOC | 70 | **50 ✅** | 103 | +20 | Higher | -33 | Lower | -53 | Lower |
| PLOC | 47 | **38 ✅** | 72 | +9 | Higher | -25 | Lower | -34 | Lower |
| LLOC | 41 | **25 ✅** | 63 | +16 | Higher | -22 | Lower | -38 | Lower |
| Comments (CLOC) | 13 | 5 | **20 ✅** | +8 | Higher | -7 | Lower | -15 | Lower |
| Blank Lines | 10 | **7 ✅** | 11 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.7356 | 51.5028 | **57.8504 ✅** | +3.2329 | Higher | -3.1148 | Lower | -6.3476 | Lower |
| MI (SEI) | 30.8690 | 23.7818 | **33.0678 ✅** | +7.0872 | Higher | -2.1988 | Lower | -9.2861 | Lower |
| Cyclomatic (sum) | 23 | 25 | **12 ✅** | -2 | Lower | +11 | Higher | +13 | Higher |
| Cognitive (sum) | 40 | 44 | **22 ✅** | -4 | Lower | +18 | Higher | +22 | Higher |
| Halstead Volume | 2461.6834 | 2868.2202 | **2291.2883 ✅** | -406.5368 | Lower | +170.3951 | Higher | +576.9319 | Higher |
| Halstead Effort | 122013.8749 | 149264.5227 | **99023.5027 ✅** | -27250.6478 | Lower | +22990.3722 | Higher | +50241.0200 | Higher |
| SLOC | 77 | 87 | **76 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| PLOC | 60 | 67 | **54 ✅** | -7 | Lower | +6 | Higher | +13 | Higher |
| LLOC | 52 | 56 | **50 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Comments (CLOC) | **9 ✅** | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | **8 ✅** | 12 | 14 | -4 | Lower | -6 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **42.4812 ✅** | 37.9832 | 25.5448 | +4.4980 | Higher | +16.9363 | Higher | +12.4384 | Higher |
| MI (SEI) | **13.1250 ✅** | 10.3306 | -19.6867 | +2.7944 | Higher | +32.8117 | Higher | +30.0173 | Higher |
| Cyclomatic (sum) | **28 ✅** | 33 | 53 | -5 | Lower | -25 | Lower | -20 | Lower |
| Cognitive (sum) | **44 ✅** | 67 | 47 | -23 | Lower | -3 | Lower | +20 | Higher |
| Halstead Volume | **4490.9444 ✅** | 4864.8018 | 7200.1422 | -373.8573 | Lower | -2709.1978 | Lower | -2335.3405 | Lower |
| Halstead Effort | **243952.1229 ✅** | 245064.3887 | 579211.4413 | -1112.2657 | Lower | -335259.3184 | Lower | -334147.0526 | Lower |
| SLOC | **126 ✅** | 151 | 216 | -25 | Lower | -90 | Lower | -65 | Lower |
| PLOC | **101 ✅** | 110 | 192 | -9 | Lower | -91 | Lower | -82 | Lower |
| LLOC | 80 | 80 | 105 | 0 | Same | -25 | Lower | -25 | Lower |
| Comments (CLOC) | 14 | **22 ✅** | 7 | -8 | Lower | +7 | Higher | +15 | Higher |
| Blank Lines | **14 ✅** | 20 | 18 | -6 | Lower | -4 | Lower | +2 | Higher |
| Functions | 1 | 1 | 8 | 0 | Same | -7 | Lower | -7 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | **0 ✅** | 2 | 9 | -2 | Lower | -9 | Lower | -7 | Lower |

## p02162

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.6948 ✅** | 66.0847 | 64.3500 | +0.6101 | Higher | +2.3448 | Higher | +1.7347 | Higher |
| MI (SEI) | 22.6576 | 21.7774 | **39.8161 ✅** | +0.8802 | Higher | -17.1585 | Lower | -18.0387 | Lower |
| Cyclomatic (sum) | 21 | 21 | **20 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 16 | 16 | **15 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **1255.3371 ✅** | 1321.9921 | 1416.6079 | -66.6551 | Lower | -161.2708 | Lower | -94.6158 | Lower |
| Halstead Effort | **35351.1889 ✅** | 37228.2431 | 38202.7164 | -1877.0543 | Lower | -2851.5275 | Lower | -974.4732 | Lower |
| SLOC | **47 ✅** | 48 | 53 | -1 | Lower | -6 | Lower | -5 | Lower |
| PLOC | 43 | 43 | 43 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 12 | 12 | **11 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 0 | 0 | **4 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02166

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.9582 | **62.6286 ✅** | 60.2729 | -2.6703 | Lower | -0.3147 | Lower | +2.3556 | Higher |
| MI (SEI) | 34.5333 | **41.1303 ✅** | 38.2877 | -6.5970 | Lower | -3.7544 | Lower | +2.8426 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 23 | 24 | 23 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 1891.8954 | **1659.8363 ✅** | 1864.9113 | +232.0591 | Higher | +26.9840 | Higher | -205.0751 | Lower |
| Halstead Effort | 79331.7743 | **68260.7661 ✅** | 88159.4450 | +11071.0082 | Higher | -8827.6707 | Lower | -19898.6789 | Lower |
| SLOC | 68 | **61 ✅** | 67 | +7 | Higher | +1 | Higher | -6 | Lower |
| PLOC | 56 | **50 ✅** | 54 | +6 | Higher | +2 | Higher | -4 | Lower |
| LLOC | 41 | **36 ✅** | 39 | +5 | Higher | +2 | Higher | -3 | Lower |
| Comments (CLOC) | 6 | 7 | **8 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p02186

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 33.3048 | **44.5390 ✅** | 29.8200 | -11.2342 | Lower | +3.4848 | Higher | +14.7190 | Higher |
| MI (SEI) | -1.0580 | **20.8764 ✅** | -6.0454 | -21.9344 | Lower | +4.9874 | Higher | +26.9218 | Higher |
| Cyclomatic (sum) | 29 | **19 ✅** | 31 | +10 | Higher | -2 | Lower | -12 | Lower |
| Cognitive (sum) | 14 | 14 | 22 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | 7212.8918 | **3630.8396 ✅** | 7796.9483 | +3582.0522 | Higher | -584.0565 | Lower | -4166.1088 | Lower |
| Halstead Effort | 625134.2205 | **207717.0320 ✅** | 789177.6092 | +417417.1886 | Higher | -164043.3887 | Lower | -581460.5772 | Lower |
| SLOC | 188 | **135 ✅** | 221 | +53 | Higher | -33 | Lower | -86 | Lower |
| PLOC | 150 | **97 ✅** | 171 | +53 | Higher | -21 | Lower | -74 | Lower |
| LLOC | 83 | **59 ✅** | 99 | +24 | Higher | -16 | Lower | -40 | Lower |
| Comments (CLOC) | 19 | **24 ✅** | 22 | -5 | Lower | -3 | Lower | +2 | Higher |
| Blank Lines | 25 | **22 ✅** | 32 | +3 | Higher | -7 | Lower | -10 | Lower |
| Functions | 12 | **6 ✅** | 10 | +6 | Higher | +2 | Higher | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 12 | **4 ✅** | 14 | +8 | Higher | -2 | Lower | -10 | Lower |

## p02190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.3826 | **80.7255 ✅** | 77.6158 | -2.3429 | Lower | +0.7668 | Higher | +3.1097 | Higher |
| MI (SEI) | 65.8782 | **68.0703 ✅** | 66.7012 | -2.1921 | Lower | -0.8230 | Lower | +1.3691 | Higher |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 737.9720 | **694.5619 ✅** | 781.3821 | +43.4101 | Higher | -43.4101 | Lower | -86.8202 | Lower |
| Halstead Effort | **13796.8680 ✅** | 13922.8088 | 15627.6425 | -125.9408 | Lower | -1830.7745 | Lower | -1704.8337 | Lower |
| SLOC | 34 | **30 ✅** | 35 | +4 | Higher | -1 | Lower | -5 | Lower |
| PLOC | 22 | **20 ✅** | 22 | +2 | Higher | 0 | Same | -2 | Lower |
| LLOC | 18 | **15 ✅** | 18 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 5 | 4 | **6 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02205

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.1701 ✅** | 73.8239 | 79.8068 | +6.3462 | Higher | +0.3633 | Higher | -5.9829 | Lower |
| MI (SEI) | **75.3388 ✅** | 62.0054 | 70.9247 | +13.3335 | Higher | +4.4141 | Higher | -8.9194 | Lower |
| Cyclomatic (sum) | 4 | 10 | 4 | -6 | Lower | 0 | Same | +6 | Higher |
| Cognitive (sum) | 4 | 7 | **3 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Halstead Volume | **660.6753 ✅** | 856.7291 | 708.4857 | -196.0538 | Lower | -47.8104 | Lower | +148.2434 | Higher |
| Halstead Effort | **11925.1888 ✅** | 20486.9998 | 13759.5372 | -8561.8111 | Lower | -1834.3485 | Lower | +6727.4626 | Higher |
| SLOC | 32 | 40 | 32 | -8 | Lower | 0 | Same | +8 | Higher |
| PLOC | **18 ✅** | 27 | 21 | -9 | Lower | -3 | Lower | +6 | Higher |
| LLOC | 14 | 14 | 21 | 0 | Same | -7 | Lower | -7 | Lower |
| Comments (CLOC) | **8 ✅** | 7 | 6 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02242

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.8807 | **52.3285 ✅** | 38.3342 | -10.4478 | Lower | +3.5465 | Higher | +13.9943 | Higher |
| MI (SEI) | 1.1488 | **26.3732 ✅** | 3.0180 | -25.2244 | Lower | -1.8693 | Lower | +23.3552 | Higher |
| Cyclomatic (sum) | 31 | **21 ✅** | 33 | +10 | Higher | -2 | Lower | -12 | Lower |
| Cognitive (sum) | 28 | 33 | 28 | -5 | Lower | 0 | Same | +5 | Higher |
| Halstead Volume | 3730.0225 | **2454.0240 ✅** | 4191.5679 | +1275.9984 | Higher | -461.5454 | Lower | -1737.5439 | Lower |
| Halstead Effort | 201356.9019 | **103248.5713 ✅** | 281332.3525 | +98108.3306 | Higher | -79975.4506 | Lower | -178083.7812 | Lower |
| SLOC | 133 | **92 ✅** | 155 | +41 | Higher | -22 | Lower | -63 | Lower |
| PLOC | 114 | **72 ✅** | 127 | +42 | Higher | -13 | Lower | -55 | Lower |
| LLOC | 69 | **54 ✅** | 81 | +15 | Higher | -12 | Lower | -27 | Lower |
| Comments (CLOC) | 4 | 10 | **11 ✅** | -6 | Lower | -7 | Lower | -1 | Lower |
| Blank Lines | 16 | **12 ✅** | 18 | +4 | Higher | -2 | Lower | -6 | Lower |
| Functions | 5 | **1 ✅** | 7 | +4 | Higher | -2 | Lower | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 6 | +2 | Higher | -3 | Lower | -5 | Lower |

## p02249

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **40.7779 ✅** | 40.6996 | 40.6672 | +0.0783 | Higher | +0.1107 | Higher | +0.0324 | Higher |
| MI (SEI) | 6.8200 | **13.8003 ✅** | 4.7714 | -6.9803 | Lower | +2.0486 | Higher | +9.0289 | Higher |
| Cyclomatic (sum) | 31 | **24 ✅** | 32 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 39 | **30 ✅** | 41 | +9 | Higher | -2 | Lower | -11 | Lower |
| Halstead Volume | 4504.8116 | **4208.6994 ✅** | 4506.5838 | +296.1122 | Higher | -1.7722 | Lower | -297.8844 | Lower |
| Halstead Effort | 279453.6571 | **237415.7409 ✅** | 307717.7345 | +42037.9162 | Higher | -28264.0774 | Lower | -70301.9936 | Lower |
| SLOC | 134 | 152 | **133 ✅** | -18 | Lower | +1 | Higher | +19 | Higher |
| PLOC | 112 | **109 ✅** | 111 | +3 | Higher | +1 | Higher | -2 | Lower |
| LLOC | 79 | **77 ✅** | 80 | +2 | Higher | -1 | Lower | -3 | Lower |
| Comments (CLOC) | 10 | **23 ✅** | 8 | -13 | Lower | +2 | Higher | +15 | Higher |
| Blank Lines | **12 ✅** | 20 | 14 | -8 | Lower | -2 | Lower | +6 | Higher |
| Functions | 6 | **2 ✅** | 5 | +4 | Higher | +1 | Higher | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |

## p02256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.0425 | **70.4609 ✅** | 63.8434 | -0.4183 | Lower | +6.1991 | Higher | +6.6174 | Higher |
| MI (SEI) | 52.5064 | **55.2797 ✅** | 40.0407 | -2.7733 | Lower | +12.4657 | Higher | +15.2390 | Higher |
| Cyclomatic (sum) | 11 | **9 ✅** | 11 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 6 | **4 ✅** | 11 | +2 | Higher | -5 | Lower | -7 | Lower |
| Halstead Volume | **961.0998 ✅** | 1106.1958 | 1173.9221 | -145.0960 | Lower | -212.8223 | Lower | -67.7263 | Lower |
| Halstead Effort | **23288.1870 ✅** | 26179.9676 | 28956.7448 | -2891.7806 | Lower | -5668.5578 | Lower | -2776.7772 | Lower |
| SLOC | 48 | **46 ✅** | 66 | +2 | Higher | -18 | Lower | -20 | Lower |
| PLOC | 35 | **31 ✅** | 47 | +4 | Higher | -12 | Lower | -16 | Lower |
| LLOC | **20 ✅** | 21 | 26 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | **7 ✅** | 8 | 13 | -1 | Lower | -6 | Lower | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02266

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.7873 | **63.2928 ✅** | 46.9427 | -0.5055 | Lower | +15.8446 | Higher | +16.3501 | Higher |
| MI (SEI) | **52.2185 ✅** | 51.3796 | 19.6106 | +0.8390 | Higher | +32.6080 | Higher | +31.7690 | Higher |
| Cyclomatic (sum) | 12 | 12 | 17 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 18 | **17 ✅** | 32 | +1 | Higher | -14 | Lower | -15 | Lower |
| Halstead Volume | **1145.5841 ✅** | 1248.5813 | 2810.1803 | -102.9972 | Lower | -1664.5963 | Lower | -1561.5990 | Lower |
| Halstead Effort | **26015.8452 ✅** | 30457.8168 | 91780.4902 | -4441.9716 | Lower | -65764.6451 | Lower | -61322.6735 | Lower |
| SLOC | 70 | **66 ✅** | 130 | +4 | Higher | -60 | Lower | -64 | Lower |
| PLOC | **42 ✅** | 43 | 101 | -1 | Lower | -59 | Lower | -58 | Lower |
| LLOC | **25 ✅** | 28 | 63 | -3 | Lower | -38 | Lower | -35 | Lower |
| Comments (CLOC) | **19 ✅** | 16 | 16 | +3 | Higher | +3 | Higher | 0 | Same |
| Blank Lines | 9 | **8 ✅** | 13 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02276

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.1469 | **70.1337 ✅** | 62.3238 | -2.9868 | Lower | +4.8231 | Higher | +7.8100 | Higher |
| MI (SEI) | 47.2021 | **50.7371 ✅** | 36.4156 | -3.5350 | Lower | +10.7865 | Higher | +14.3215 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 12 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 10 | 10 | 14 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | 1427.9958 | **1231.2950 ✅** | 1655.7065 | +196.7009 | Higher | -227.7107 | Lower | -424.4116 | Lower |
| Halstead Effort | 50574.8530 | **34312.0868 ✅** | 62916.8485 | +16262.7662 | Higher | -12341.9955 | Lower | -28604.7617 | Lower |
| SLOC | 52 | **46 ✅** | 64 | +6 | Higher | -12 | Lower | -18 | Lower |
| PLOC | 40 | **37 ✅** | 49 | +3 | Higher | -9 | Lower | -12 | Lower |
| LLOC | 29 | **28 ✅** | 32 | +1 | Higher | -3 | Lower | -4 | Lower |
| Comments (CLOC) | **6 ✅** | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 7 | **5 ✅** | 11 | +2 | Higher | -4 | Lower | -6 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |

## p02277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.4445 | **56.1005 ✅** | 54.8666 | -0.6560 | Lower | +0.5779 | Higher | +1.2339 | Higher |
| MI (SEI) | **26.1470 ✅** | 25.7486 | 19.7003 | +0.3985 | Higher | +6.4468 | Higher | +6.0483 | Higher |
| Cyclomatic (sum) | 16 | 16 | 17 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 18 | 19 | 18 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 2317.2461 | **2290.8254 ✅** | 2477.5719 | +26.4207 | Higher | -160.3258 | Lower | -186.7465 | Lower |
| Halstead Effort | 96250.4892 | **90096.4856 ✅** | 106320.1504 | +6154.0036 | Higher | -10069.6611 | Lower | -16223.6647 | Lower |
| SLOC | 83 | **80 ✅** | 83 | +3 | Higher | 0 | Same | -3 | Lower |
| PLOC | **67 ✅** | 68 | 70 | -1 | Lower | -3 | Lower | -2 | Lower |
| LLOC | 43 | 43 | **40 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Comments (CLOC) | **6 ✅** | 5 | 3 | +1 | Higher | +3 | Higher | +2 | Higher |
| Blank Lines | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p02279

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.8340 | 46.3526 | **50.4637 ✅** | +3.4815 | Higher | -0.6296 | Lower | -4.1111 | Lower |
| MI (SEI) | 19.5481 | **21.2362 ✅** | 12.1274 | -1.6880 | Lower | +7.4208 | Higher | +9.1088 | Higher |
| Cyclomatic (sum) | **17 ✅** | 20 | 18 | -3 | Lower | -1 | Lower | +2 | Higher |
| Cognitive (sum) | 25 | 40 | **20 ✅** | -15 | Lower | +5 | Higher | +20 | Higher |
| Halstead Volume | **2871.4854 ✅** | 3359.9151 | 2999.0052 | -488.4297 | Lower | -127.5198 | Lower | +360.9099 | Higher |
| Halstead Effort | **115577.2880 ✅** | 151639.9436 | 122753.3994 | -36062.6556 | Lower | -7176.1114 | Lower | +28886.5442 | Higher |
| SLOC | 108 | 122 | **101 ✅** | -14 | Lower | +7 | Higher | +21 | Higher |
| PLOC | **85 ✅** | 87 | 87 | -2 | Lower | -2 | Lower | 0 | Same |
| LLOC | 55 | 72 | 55 | -17 | Lower | 0 | Same | +17 | Higher |
| Comments (CLOC) | 9 | **18 ✅** | 3 | -9 | Lower | +6 | Higher | +15 | Higher |
| Blank Lines | 14 | 18 | **12 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02297

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.9974 ✅** | 71.3867 | 69.2397 | +8.6106 | Higher | +10.7576 | Higher | +2.1470 | Higher |
| MI (SEI) | **70.1089 ✅** | 51.8334 | 54.7293 | +18.2755 | Higher | +15.3795 | Higher | -2.8959 | Lower |
| Cyclomatic (sum) | **4 ✅** | 10 | 7 | -6 | Lower | -3 | Lower | +3 | Higher |
| Cognitive (sum) | **2 ✅** | 22 | 4 | -20 | Lower | -2 | Lower | +18 | Higher |
| Halstead Volume | **928.1232 ✅** | 1368.9730 | 1429.3474 | -440.8498 | Lower | -501.2242 | Lower | -60.3744 | Lower |
| Halstead Effort | **21903.7069 ✅** | 47583.6128 | 32855.6738 | -25679.9060 | Lower | -10951.9670 | Lower | +14727.9390 | Higher |
| SLOC | **29 ✅** | 40 | 47 | -11 | Lower | -18 | Lower | -7 | Lower |
| PLOC | **19 ✅** | 30 | 31 | -11 | Lower | -12 | Lower | -1 | Lower |
| LLOC | **17 ✅** | 26 | 25 | -9 | Lower | -8 | Lower | +1 | Higher |
| Comments (CLOC) | 5 | 4 | **8 ✅** | +1 | Higher | -3 | Lower | -4 | Lower |
| Blank Lines | **5 ✅** | 6 | 8 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02305

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.7349 | **72.6094 ✅** | 71.3508 | -1.8745 | Lower | -0.6160 | Lower | +1.2586 | Higher |
| MI (SEI) | 46.8414 | **59.5279 ✅** | 45.1292 | -12.6865 | Lower | +1.7123 | Higher | +14.3988 | Higher |
| Cyclomatic (sum) | 8 | **6 ✅** | 9 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 1259.7972 | **1195.9332 ✅** | 1440.7960 | +63.8640 | Higher | -180.9989 | Lower | -244.8628 | Lower |
| Halstead Effort | 38339.8274 | **22221.2104 ✅** | 40435.2436 | +16118.6171 | Higher | -2095.4162 | Lower | -18214.0332 | Lower |
| SLOC | 44 | 41 | **40 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| PLOC | 33 | **31 ✅** | 32 | +2 | Higher | +1 | Higher | -1 | Lower |
| LLOC | 22 | **18 ✅** | 26 | +4 | Higher | -4 | Lower | -8 | Lower |
| Comments (CLOC) | 3 | **7 ✅** | 2 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.4417 | **69.7498 ✅** | 59.6476 | -4.3081 | Lower | +5.7941 | Higher | +10.1022 | Higher |
| MI (SEI) | 42.2447 | **57.0264 ✅** | 35.1518 | -14.7818 | Lower | +7.0929 | Higher | +21.8747 | Higher |
| Cyclomatic (sum) | 10 | 10 | 14 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | **11 ✅** | 13 | 16 | -2 | Lower | -5 | Lower | -3 | Lower |
| Halstead Volume | 1592.3790 | **1062.7460 ✅** | 1835.0249 | +529.6330 | Higher | -242.6460 | Lower | -772.2790 | Lower |
| Halstead Effort | 64106.0955 | **31662.5000 ✅** | 80741.0972 | +32443.5954 | Higher | -16635.0017 | Lower | -49078.5972 | Lower |
| SLOC | 55 | **48 ✅** | 71 | +7 | Higher | -16 | Lower | -23 | Lower |
| PLOC | 44 | **33 ✅** | 56 | +11 | Higher | -12 | Lower | -23 | Lower |
| LLOC | 36 | **21 ✅** | 44 | +15 | Higher | -8 | Lower | -23 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 7 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | **6 ✅** | 8 | 9 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 5 | +2 | Higher | -2 | Lower | -4 | Lower |

## p02328

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.5824 | **62.7249 ✅** | 57.3448 | -1.1425 | Lower | +4.2376 | Higher | +5.3801 | Higher |
| MI (SEI) | 36.7788 | **48.3322 ✅** | 25.5346 | -11.5534 | Lower | +11.2441 | Higher | +22.7976 | Higher |
| Cyclomatic (sum) | 11 | **10 ✅** | 14 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | **14 ✅** | 19 | 16 | -5 | Lower | -2 | Lower | +3 | Higher |
| Halstead Volume | 1813.3182 | **1451.8387 ✅** | 2048.8319 | +361.4795 | Higher | -235.5137 | Lower | -596.9932 | Lower |
| Halstead Effort | 80843.7713 | **48004.3451 ✅** | 98881.7510 | +32839.4262 | Higher | -18037.9796 | Lower | -50877.4059 | Lower |
| SLOC | **66 ✅** | 67 | 79 | -1 | Lower | -13 | Lower | -12 | Lower |
| PLOC | 50 | **43 ✅** | 63 | +7 | Higher | -13 | Lower | -20 | Lower |
| LLOC | 40 | **34 ✅** | 46 | +6 | Higher | -6 | Lower | -12 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 4 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |

## p02333

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.8791 | **63.5303 ✅** | 55.4059 | -6.6512 | Lower | +1.4731 | Higher | +8.1243 | Higher |
| MI (SEI) | 20.6526 | **38.6631 ✅** | 28.4789 | -18.0105 | Lower | -7.8263 | Lower | +10.1842 | Higher |
| Cyclomatic (sum) | 15 | **9 ✅** | 17 | +6 | Higher | -2 | Lower | -8 | Lower |
| Cognitive (sum) | 11 | **9 ✅** | 13 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | 2862.3752 | **1931.5532 ✅** | 2821.6549 | +930.8219 | Higher | +40.7203 | Higher | -890.1016 | Lower |
| Halstead Effort | 182476.4170 | **90839.8120 ✅** | 180585.9115 | +91636.6051 | Higher | +1890.5055 | Higher | -89746.0995 | Lower |
| SLOC | 72 | **59 ✅** | 77 | +13 | Higher | -5 | Lower | -18 | Lower |
| PLOC | 57 | **44 ✅** | 60 | +13 | Higher | -3 | Lower | -16 | Lower |
| LLOC | 43 | **39 ✅** | 42 | +4 | Higher | +1 | Higher | -3 | Lower |
| Comments (CLOC) | 2 | 5 | **7 ✅** | -3 | Lower | -5 | Lower | -2 | Lower |
| Blank Lines | 13 | 10 | 10 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **1 ✅** | 4 | +4 | Higher | +1 | Higher | -3 | Lower |

## p02340

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.4249 ✅** | 71.3570 | 69.1085 | +1.0679 | Higher | +3.3164 | Higher | +2.2485 | Higher |
| MI (SEI) | 53.1276 | **64.1396 ✅** | 54.9184 | -11.0120 | Lower | -1.7908 | Lower | +9.2212 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 1224.9069 | **1168.2782 ✅** | 1499.6431 | +56.6287 | Higher | -274.7362 | Lower | -331.3649 | Lower |
| Halstead Effort | 40066.3097 | **35799.3829 ✅** | 48694.2947 | +4266.9267 | Higher | -8627.9851 | Lower | -12894.9118 | Lower |
| SLOC | **40 ✅** | 44 | 46 | -4 | Lower | -6 | Lower | -2 | Lower |
| PLOC | 29 | **25 ✅** | 30 | +4 | Higher | -1 | Lower | -5 | Lower |
| LLOC | 21 | 21 | 21 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 4 | **12 ✅** | 8 | -8 | Lower | -4 | Lower | +4 | Higher |
| Blank Lines | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02347

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.6915 | **43.3695 ✅** | 39.1584 | -3.6780 | Lower | +0.5331 | Higher | +4.2111 | Higher |
| MI (SEI) | 1.5347 | **4.7020 ✅** | -0.6528 | -3.1673 | Lower | +2.1875 | Higher | +5.3548 | Higher |
| Cyclomatic (sum) | 30 | **26 ✅** | 29 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 29 | 37 | **28 ✅** | -8 | Lower | +1 | Higher | +9 | Higher |
| Halstead Volume | 4951.3835 | **4135.8294 ✅** | 5487.8252 | +815.5542 | Higher | -536.4417 | Lower | -1351.9958 | Lower |
| Halstead Effort | 345936.6623 | **194669.2101 ✅** | 408510.3829 | +151267.4521 | Higher | -62573.7206 | Lower | -213841.1728 | Lower |
| SLOC | 141 | **126 ✅** | 143 | +15 | Higher | -2 | Lower | -17 | Lower |
| PLOC | 117 | **105 ✅** | 117 | +12 | Higher | 0 | Same | -12 | Lower |
| LLOC | 60 | **54 ✅** | 69 | +6 | Higher | -9 | Lower | -15 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | 21 | **17 ✅** | 21 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |

## p02353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **40.7009 ✅** | 36.9165 | 39.9818 | +3.7844 | Higher | +0.7191 | Higher | -3.0653 | Lower |
| MI (SEI) | **3.5615 ✅** | -5.9336 | -0.6314 | +9.4951 | Higher | +4.1929 | Higher | -5.3021 | Lower |
| Cyclomatic (sum) | **21 ✅** | 38 | 22 | -17 | Lower | -1 | Lower | +16 | Higher |
| Cognitive (sum) | **19 ✅** | 30 | 20 | -11 | Lower | -1 | Lower | +10 | Higher |
| Halstead Volume | **4804.7365 ✅** | 5205.1704 | 5278.5687 | -400.4339 | Lower | -473.8321 | Lower | -73.3982 | Lower |
| Halstead Effort | **291123.3555 ✅** | 355686.6449 | 319810.8793 | -64563.2895 | Lower | -28687.5238 | Lower | +35875.7656 | Higher |
| SLOC | 152 | **147 ✅** | 152 | +5 | Higher | 0 | Same | -5 | Lower |
| PLOC | **123 ✅** | 129 | 124 | -6 | Lower | -1 | Lower | +5 | Higher |
| LLOC | 71 | **61 ✅** | 75 | +10 | Higher | -4 | Lower | -14 | Lower |
| Comments (CLOC) | **9 ✅** | 4 | 6 | +5 | Higher | +3 | Higher | -2 | Lower |
| Blank Lines | 24 | **16 ✅** | 22 | +8 | Higher | +2 | Higher | -6 | Lower |
| Functions | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |

## p02355

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.0001 ✅** | 48.8135 | 45.1372 | +2.1865 | Higher | +5.8628 | Higher | +3.6763 | Higher |
| MI (SEI) | 23.8471 | **28.8863 ✅** | 5.3846 | -5.0392 | Lower | +18.4625 | Higher | +23.5017 | Higher |
| Cyclomatic (sum) | **24 ✅** | 27 | 25 | -3 | Lower | -1 | Lower | +2 | Higher |
| Cognitive (sum) | 36 | 39 | **31 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| Halstead Volume | **2139.8662 ✅** | 2525.2646 | 2791.5957 | -385.3984 | Lower | -651.7295 | Lower | -266.3311 | Lower |
| Halstead Effort | **129364.6392 ✅** | 142993.1081 | 175011.5751 | -13628.4689 | Lower | -45646.9359 | Lower | -32018.4670 | Lower |
| SLOC | **100 ✅** | 104 | 130 | -4 | Lower | -30 | Lower | -26 | Lower |
| PLOC | 76 | **74 ✅** | 107 | +2 | Higher | -31 | Lower | -33 | Lower |
| LLOC | 60 | **55 ✅** | 89 | +5 | Higher | -29 | Lower | -34 | Lower |
| Comments (CLOC) | 10 | **20 ✅** | 4 | -10 | Lower | +6 | Higher | +16 | Higher |
| Blank Lines | 15 | **14 ✅** | 19 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |

## p02369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.0496 | **63.7229 ✅** | 53.3967 | -5.6733 | Lower | +4.6530 | Higher | +10.3263 | Higher |
| MI (SEI) | 34.1981 | **46.9083 ✅** | 29.8071 | -12.7102 | Lower | +4.3910 | Higher | +17.1012 | Higher |
| Cyclomatic (sum) | 13 | **11 ✅** | 19 | +2 | Higher | -6 | Lower | -8 | Lower |
| Cognitive (sum) | 16 | **9 ✅** | 22 | +7 | Higher | -6 | Lower | -13 | Lower |
| Halstead Volume | 1798.1673 | **1616.8028 ✅** | 2183.1602 | +181.3645 | Higher | -384.9929 | Lower | -566.3575 | Lower |
| Halstead Effort | 55648.5455 | **50215.9914 ✅** | 79040.3237 | +5432.5540 | Higher | -23391.7782 | Lower | -28824.3322 | Lower |
| SLOC | 80 | **60 ✅** | 92 | +20 | Higher | -12 | Lower | -32 | Lower |
| PLOC | 61 | **42 ✅** | 69 | +19 | Higher | -8 | Lower | -27 | Lower |
| LLOC | 34 | 34 | 36 | 0 | Same | -2 | Lower | -2 | Lower |
| Comments (CLOC) | 9 | 10 | **12 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | 12 | **10 ✅** | 13 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **0 ✅** | 5 | +3 | Higher | -2 | Lower | -5 | Lower |

## p02370

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.6000 ✅** | 67.5503 | 50.1701 | +2.0497 | Higher | +19.4298 | Higher | +17.3801 | Higher |
| MI (SEI) | 49.9670 | **51.1149 ✅** | 24.3274 | -1.1479 | Lower | +25.6396 | Higher | +26.7875 | Higher |
| Cyclomatic (sum) | 8 | 8 | 18 | 0 | Same | -10 | Lower | -10 | Lower |
| Cognitive (sum) | 10 | 10 | 17 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | **1364.4009 ✅** | 1381.1730 | 2729.7126 | -16.7721 | Lower | -1365.3117 | Lower | -1348.5396 | Lower |
| Halstead Effort | **34110.0223 ✅** | 36083.1446 | 139432.4804 | -1973.1223 | Lower | -105322.4581 | Lower | -103349.3357 | Lower |
| SLOC | **46 ✅** | 52 | 106 | -6 | Lower | -60 | Lower | -54 | Lower |
| PLOC | **34 ✅** | 35 | 81 | -1 | Lower | -47 | Lower | -46 | Lower |
| LLOC | **26 ✅** | 27 | 67 | -1 | Lower | -41 | Lower | -40 | Lower |
| Comments (CLOC) | 5 | 8 | **13 ✅** | -3 | Lower | -8 | Lower | -5 | Lower |
| Blank Lines | **7 ✅** | 9 | 14 | -2 | Lower | -7 | Lower | -5 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p02371

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.3462 ✅** | 55.6138 | 38.3890 | +0.7324 | Higher | +17.9572 | Higher | +17.2248 | Higher |
| MI (SEI) | 27.5380 | **39.2909 ✅** | 4.9953 | -11.7529 | Lower | +22.5428 | Higher | +34.2957 | Higher |
| Cyclomatic (sum) | **11 ✅** | 13 | 23 | -2 | Lower | -12 | Lower | -10 | Lower |
| Cognitive (sum) | **10 ✅** | 19 | 23 | -9 | Lower | -13 | Lower | -4 | Lower |
| Halstead Volume | 2949.6643 | **2378.1618 ✅** | 6080.7137 | +571.5025 | Higher | -3131.0494 | Lower | -3702.5519 | Lower |
| Halstead Effort | 129373.6495 | **84556.8644 ✅** | 505523.7400 | +44816.7851 | Higher | -376150.0905 | Lower | -420966.8756 | Lower |
| SLOC | **78 ✅** | 85 | 158 | -7 | Lower | -80 | Lower | -73 | Lower |
| PLOC | 64 | **55 ✅** | 134 | +9 | Higher | -70 | Lower | -79 | Lower |
| LLOC | 50 | **45 ✅** | 118 | +5 | Higher | -68 | Lower | -73 | Lower |
| Comments (CLOC) | 6 | **19 ✅** | 15 | -13 | Lower | -9 | Lower | +4 | Higher |
| Blank Lines | **9 ✅** | 12 | 17 | -3 | Lower | -8 | Lower | -5 | Lower |
| Functions | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 6 | 0 | Same | -4 | Lower | -4 | Lower |

## p02393

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.8107 ✅** | 89.9348 | 76.1207 | +0.8759 | Higher | +14.6900 | Higher | +13.8141 | Higher |
| MI (SEI) | **80.2025 ✅** | 78.3377 | 66.0186 | +1.8648 | Higher | +14.1839 | Higher | +12.3190 | Higher |
| Cyclomatic (sum) | 2 | 2 | 12 | 0 | Same | -10 | Lower | -10 | Lower |
| Cognitive (sum) | 0 | 0 | 7 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | 560 | 560 | 918.0797 | 0 | Same | -358.0797 | Lower | -358.0797 | Lower |
| Halstead Effort | 6130.5263 | 6130.5263 | 24864.6593 | 0 | Same | -18734.1330 | Lower | -18734.1330 | Lower |
| SLOC | **18 ✅** | 19 | 33 | -1 | Lower | -15 | Lower | -14 | Lower |
| PLOC | 12 | 12 | 23 | 0 | Same | -11 | Lower | -11 | Lower |
| LLOC | 9 | 9 | **8 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 2 | 2 | **6 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02396

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.8755 | **87.2263 ✅** | 81.6184 | -1.3508 | Lower | +4.2571 | Higher | +5.6078 | Higher |
| MI (SEI) | **77.8409 ✅** | 66.6230 | 76.2330 | +11.2178 | Higher | +1.6078 | Higher | -9.6100 | Lower |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 5 | 5 | **2 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 435.5780 | 435.5780 | 505.3170 | 0 | Same | -69.7390 | Lower | -69.7390 | Lower |
| Halstead Effort | 5880.3028 | 5880.3028 | 6872.3109 | 0 | Same | -992.0081 | Lower | -992.0081 | Lower |
| SLOC | 25 | **23 ✅** | 31 | +2 | Higher | -6 | Lower | -8 | Lower |
| PLOC | 17 | 17 | 19 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 10 | 10 | 12 | 0 | Same | -2 | Lower | -2 | Lower |
| Comments (CLOC) | 4 | 1 | **7 ✅** | +3 | Higher | -3 | Lower | -6 | Lower |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02412

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.0699 | 67.1284 | **71.5041 ✅** | +3.9415 | Higher | -0.4342 | Lower | -4.3757 | Lower |
| MI (SEI) | 47.9357 | 45.4742 | **51.6389 ✅** | +2.4615 | Higher | -3.7032 | Lower | -6.1646 | Lower |
| Cyclomatic (sum) | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | **19 ✅** | 22 | 21 | -3 | Lower | -2 | Lower | +1 | Higher |
| Halstead Volume | 905.8642 | 1082.5770 | **895.1721 ✅** | -176.7128 | Lower | +10.6921 | Higher | +187.4049 | Higher |
| Halstead Effort | 23250.5135 | 30447.4772 | **21931.7153 ✅** | -7196.9637 | Lower | +1318.7981 | Higher | +8515.7618 | Higher |
| SLOC | 44 | 53 | **43 ✅** | -9 | Lower | +1 | Higher | +10 | Higher |
| PLOC | 33 | 38 | 33 | -5 | Lower | 0 | Same | +5 | Higher |
| LLOC | 23 | 27 | **22 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Comments (CLOC) | 3 | **5 ✅** | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | 8 | 10 | **7 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02418

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.1126 ✅** | 68.9534 | 80.4583 | +20.1592 | Higher | +8.6544 | Higher | -11.5048 | Lower |
| MI (SEI) | **83.8457 ✅** | 48.4402 | 72.5887 | +35.4054 | Higher | +11.2570 | Higher | -24.1484 | Lower |
| Cyclomatic (sum) | **3 ✅** | 9 | 7 | -6 | Lower | -4 | Lower | +2 | Higher |
| Cognitive (sum) | **2 ✅** | 8 | 4 | -6 | Lower | -2 | Lower | +4 | Higher |
| Halstead Volume | **397.4581 ✅** | 1214.0855 | 604.2966 | -816.6274 | Lower | -206.8385 | Lower | +609.7889 | Higher |
| Halstead Effort | **3444.6372 ✅** | 32897.8017 | 9208.3290 | -29453.1645 | Lower | -5763.6918 | Lower | +23689.4727 | Higher |
| SLOC | **22 ✅** | 49 | 31 | -27 | Lower | -9 | Lower | +18 | Higher |
| PLOC | **14 ✅** | 37 | 22 | -23 | Lower | -8 | Lower | +15 | Higher |
| LLOC | **9 ✅** | 26 | 11 | -17 | Lower | -2 | Lower | +15 | Higher |
| Comments (CLOC) | 4 | 5 | **6 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 4 | 2 | -4 | Lower | -2 | Lower | +2 | Higher |

## p02430

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.2405 ✅** | 67.3339 | 66.9752 | +9.9066 | Higher | +10.2653 | Higher | +0.3587 | Higher |
| MI (SEI) | **59.9264 ✅** | 51.9056 | 56.1796 | +8.0208 | Higher | +3.7468 | Higher | -4.2740 | Lower |
| Cyclomatic (sum) | **10 ✅** | 14 | 11 | -4 | Lower | -1 | Lower | +3 | Higher |
| Cognitive (sum) | **11 ✅** | 12 | 15 | -1 | Lower | -4 | Lower | -3 | Lower |
| Halstead Volume | **982.5742 ✅** | 1247.7500 | 1435.2384 | -265.1757 | Lower | -452.6642 | Lower | -187.4885 | Lower |
| Halstead Effort | **23217.8650 ✅** | 33962.1940 | 41580.9078 | -10744.3290 | Lower | -18363.0429 | Lower | -7618.7138 | Lower |
| SLOC | **31 ✅** | 50 | 51 | -19 | Lower | -20 | Lower | -1 | Lower |
| PLOC | **23 ✅** | 36 | 33 | -13 | Lower | -10 | Lower | +3 | Higher |
| LLOC | **16 ✅** | 22 | 24 | -6 | Lower | -8 | Lower | -2 | Lower |
| Comments (CLOC) | 3 | 8 | **12 ✅** | -5 | Lower | -9 | Lower | -4 | Lower |
| Blank Lines | **5 ✅** | 8 | 7 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02433

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 28.4072 | **34.9054 ✅** | 25.1163 | -6.4982 | Lower | +3.2909 | Higher | +9.7891 | Higher |
| MI (SEI) | -8.4433 | **2.1559 ✅** | -16.7734 | -10.5992 | Lower | +8.3301 | Higher | +18.9294 | Higher |
| Cyclomatic (sum) | 41 | **35 ✅** | 44 | +6 | Higher | -3 | Lower | -9 | Lower |
| Cognitive (sum) | **36 ✅** | 63 | 41 | -27 | Lower | -5 | Lower | +22 | Higher |
| Halstead Volume | 5884.6708 | **4422.2902 ✅** | 6612.7434 | +1462.3806 | Higher | -728.0726 | Lower | -2190.4532 | Lower |
| Halstead Effort | 346648.1652 | **235855.4781 ✅** | 431118.6117 | +110792.6871 | Higher | -84470.4465 | Lower | -195263.1336 | Lower |
| SLOC | 229 | **183 ✅** | 259 | +46 | Higher | -30 | Lower | -76 | Lower |
| PLOC | 180 | **143 ✅** | 210 | +37 | Higher | -30 | Lower | -67 | Lower |
| LLOC | 122 | **98 ✅** | 122 | +24 | Higher | 0 | Same | -24 | Lower |
| Comments (CLOC) | **20 ✅** | 19 | 15 | +1 | Higher | +5 | Higher | +4 | Higher |
| Blank Lines | 31 | **23 ✅** | 34 | +8 | Higher | -3 | Lower | -11 | Lower |
| Functions | 9 | **2 ✅** | 10 | +7 | Higher | -1 | Lower | -8 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 10 | **2 ✅** | 13 | +8 | Higher | -3 | Lower | -11 | Lower |

## p02444

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.7026 ✅** | 64.9959 | 63.2608 | +3.7067 | Higher | +5.4418 | Higher | +1.7351 | Higher |
| MI (SEI) | **50.5148 ✅** | 45.4370 | 43.3832 | +5.0778 | Higher | +7.1316 | Higher | +2.0538 | Higher |
| Cyclomatic (sum) | **8 ✅** | 11 | 10 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **10 ✅** | 12 | 11 | -2 | Lower | -1 | Lower | +1 | Higher |
| Halstead Volume | **1516.3170 ✅** | 1659.8363 | 1846.9795 | -143.5193 | Lower | -330.6625 | Lower | -187.1432 | Lower |
| Halstead Effort | **47258.5454 ✅** | 56483.2515 | 61000.2417 | -9224.7061 | Lower | -13741.6963 | Lower | -4516.9903 | Lower |
| SLOC | **47 ✅** | 55 | 60 | -8 | Lower | -13 | Lower | -5 | Lower |
| PLOC | **35 ✅** | 43 | 41 | -8 | Lower | -6 | Lower | +2 | Higher |
| LLOC | **28 ✅** | 31 | 36 | -3 | Lower | -8 | Lower | -5 | Lower |
| Comments (CLOC) | 6 | 7 | **8 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | **8 ✅** | 9 | 11 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02449

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.3467 | **49.4549 ✅** | 42.1679 | -2.1082 | Lower | +5.1788 | Higher | +7.2870 | Higher |
| MI (SEI) | 19.2379 | **20.9937 ✅** | 8.8258 | -1.7558 | Lower | +10.4120 | Higher | +12.1678 | Higher |
| Cyclomatic (sum) | 25 | 23 | **21 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 22 | **18 ✅** | 46 | +4 | Higher | -24 | Lower | -28 | Lower |
| Halstead Volume | 2748 | **2586 ✅** | 4196.9807 | +162 | Higher | -1448.9807 | Lower | -1610.9807 | Lower |
| Halstead Effort | 148392 | **141799 ✅** | 333072.3885 | +6593 | Higher | -184680.3885 | Lower | -191273.3885 | Lower |
| SLOC | 114 | **105 ✅** | 145 | +9 | Higher | -31 | Lower | -40 | Lower |
| PLOC | 85 | **81 ✅** | 116 | +4 | Higher | -31 | Lower | -35 | Lower |
| LLOC | 54 | **53 ✅** | 114 | +1 | Higher | -60 | Lower | -61 | Lower |
| Comments (CLOC) | 12 | 10 | 12 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | 17 | **16 ✅** | 17 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 4 | 4 | **1 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 8 | 7 | **1 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |

## p02450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.8629 ✅** | 67.2628 | 65.1093 | +0.6000 | Higher | +2.7536 | Higher | +2.1535 | Higher |
| MI (SEI) | 46.8669 | 40.7049 | **50.8134 ✅** | +6.1620 | Higher | -3.9465 | Lower | -10.1084 | Lower |
| Cyclomatic (sum) | **9 ✅** | 10 | 11 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 13 | **8 ✅** | 14 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | 1497.3722 | **1419.4095 ✅** | 1453.0128 | +77.9627 | Higher | +44.3593 | Higher | -33.6033 | Lower |
| Halstead Effort | 54279.7410 | **42156.4618 ✅** | 52104.1321 | +12123.2792 | Higher | +2175.6089 | Higher | -9947.6702 | Lower |
| SLOC | **49 ✅** | 51 | 57 | -2 | Lower | -8 | Lower | -6 | Lower |
| PLOC | **35 ✅** | 40 | 38 | -5 | Lower | -3 | Lower | +2 | Higher |
| LLOC | 24 | 29 | 24 | -5 | Lower | 0 | Same | +5 | Higher |
| Comments (CLOC) | 5 | 3 | **11 ✅** | +2 | Higher | -6 | Lower | -8 | Lower |
| Blank Lines | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p02457

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.0993 | **54.1624 ✅** | 36.9226 | -5.0631 | Lower | +12.1767 | Higher | +17.2398 | Higher |
| MI (SEI) | 23.6267 | **25.0399 ✅** | 8.0704 | -1.4132 | Lower | +15.5563 | Higher | +16.9695 | Higher |
| Cyclomatic (sum) | 23 | **21 ✅** | 36 | +2 | Higher | -13 | Lower | -15 | Lower |
| Cognitive (sum) | 31 | **28 ✅** | 42 | +3 | Higher | -11 | Lower | -14 | Lower |
| Halstead Volume | 2940.0024 | **2564.5042 ✅** | 4626.9751 | +375.4982 | Higher | -1686.9726 | Lower | -2062.4708 | Lower |
| Halstead Effort | 144060.1187 | **121098.2758 ✅** | 369685.8652 | +22961.8429 | Higher | -225625.7465 | Lower | -248587.5894 | Lower |
| SLOC | 103 | **81 ✅** | 157 | +22 | Higher | -54 | Lower | -76 | Lower |
| PLOC | 82 | **67 ✅** | 121 | +15 | Higher | -39 | Lower | -54 | Lower |
| LLOC | 50 | **42 ✅** | 70 | +8 | Higher | -20 | Lower | -28 | Lower |
| Comments (CLOC) | 13 | 6 | **21 ✅** | +7 | Higher | -8 | Lower | -15 | Lower |
| Blank Lines | 8 | 8 | 15 | 0 | Same | -7 | Lower | -7 | Lower |
| Functions | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 9 | +2 | Higher | -5 | Lower | -7 | Lower |

## p02542

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 14.2478 | **20.9102 ✅** | 8.9268 | -6.6624 | Lower | +5.3210 | Higher | +11.9834 | Higher |
| MI (SEI) | -26.6593 | **-13.8092 ✅** | -43.2690 | -12.8501 | Lower | +16.6097 | Higher | +29.4598 | Higher |
| Cyclomatic (sum) | 55 | **49 ✅** | 60 | +6 | Higher | -5 | Lower | -11 | Lower |
| Cognitive (sum) | 144 | **86 ✅** | 131 | +58 | Higher | +13 | Higher | -45 | Lower |
| Halstead Volume | 11682.5156 | **9440.1205 ✅** | 16138.3500 | +2242.3951 | Higher | -4455.8344 | Lower | -6698.2295 | Lower |
| Halstead Effort | 1464602.1825 | **787516.8467 ✅** | 2799133.5187 | +677085.3358 | Higher | -1334531.3362 | Lower | -2011616.6719 | Lower |
| SLOC | 361 | **279 ✅** | 421 | +82 | Higher | -60 | Lower | -142 | Lower |
| PLOC | 290 | **208 ✅** | 355 | +82 | Higher | -65 | Lower | -147 | Lower |
| LLOC | 224 | **160 ✅** | 354 | +64 | Higher | -130 | Lower | -194 | Lower |
| Comments (CLOC) | 34 | **37 ✅** | 13 | -3 | Lower | +21 | Higher | +24 | Higher |
| Blank Lines | 43 | **36 ✅** | 58 | +7 | Higher | -15 | Lower | -22 | Lower |
| Functions | **2 ✅** | 4 | 5 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 6 | 5 | -3 | Lower | -2 | Lower | +1 | Higher |

## p02562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 18.5659 | **28.2469 ✅** | 7.0325 | -9.6810 | Lower | +11.5334 | Higher | +21.2144 | Higher |
| MI (SEI) | -19.3500 | **-8.1875 ✅** | -45.8660 | -11.1624 | Lower | +26.5160 | Higher | +37.6785 | Higher |
| Cyclomatic (sum) | 54 | **39 ✅** | 65 | +15 | Higher | -11 | Lower | -26 | Lower |
| Cognitive (sum) | 114 | **71 ✅** | 132 | +43 | Higher | -18 | Lower | -61 | Lower |
| Halstead Volume | 9983.3359 | **7004.3238 ✅** | 15558.7373 | +2979.0121 | Higher | -5575.4014 | Lower | -8554.4135 | Lower |
| Halstead Effort | 1121576.1458 | **578787.6675 ✅** | 2438720.9306 | +542788.4783 | Higher | -1317144.7848 | Lower | -1859933.2631 | Lower |
| SLOC | 295 | **225 ✅** | 446 | +70 | Higher | -151 | Lower | -221 | Lower |
| PLOC | 233 | **178 ✅** | 381 | +55 | Higher | -148 | Lower | -203 | Lower |
| LLOC | 186 | **129 ✅** | 348 | +57 | Higher | -162 | Lower | -219 | Lower |
| Comments (CLOC) | **31 ✅** | 21 | 13 | +10 | Higher | +18 | Higher | +8 | Higher |
| Blank Lines | 32 | **27 ✅** | 57 | +5 | Higher | -25 | Lower | -30 | Lower |
| Functions | **3 ✅** | 5 | 5 | -2 | Lower | -2 | Lower | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **3 ✅** | 4 | 6 | -1 | Lower | -3 | Lower | -2 | Lower |

## p02584

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.6739 ✅** | 75.5799 | 78.0397 | +6.0940 | Higher | +3.6342 | Higher | -2.4598 | Lower |
| MI (SEI) | 67.3263 | **70.4981 ✅** | 67.5848 | -3.1718 | Lower | -0.2585 | Lower | +2.9133 | Higher |
| Cyclomatic (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **803.6328 ✅** | 825.0630 | 823.9246 | -21.4302 | Lower | -20.2918 | Lower | +1.1384 | Higher |
| Halstead Effort | **17603.3852 ✅** | 18072.8088 | 18167.5379 | -469.4236 | Lower | -564.1527 | Lower | -94.7291 | Lower |
| SLOC | **27 ✅** | 39 | 34 | -12 | Lower | -7 | Lower | +5 | Higher |
| PLOC | 19 | 19 | 22 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 12 | 12 | 21 | 0 | Same | -9 | Lower | -9 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 6 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02585

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.5270 | 45.3202 | **51.4279 ✅** | +4.2068 | Higher | -1.9009 | Lower | -6.1077 | Lower |
| MI (SEI) | **34.2123 ✅** | 28.4472 | 25.3615 | +5.7651 | Higher | +8.8507 | Higher | +3.0857 | Higher |
| Cyclomatic (sum) | **16 ✅** | 18 | 18 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **26 ✅** | 40 | 27 | -14 | Lower | -1 | Lower | +13 | Higher |
| Halstead Volume | **2842.8451 ✅** | 2981.6919 | 2918.3745 | -138.8467 | Lower | -75.5294 | Lower | +63.3174 | Higher |
| Halstead Effort | 207602.5074 | **146217.5831 ✅** | 240228.3031 | +61384.9244 | Higher | -32625.7956 | Lower | -94010.7200 | Lower |
| SLOC | 112 | 139 | **96 ✅** | -27 | Lower | +16 | Higher | +43 | Higher |
| PLOC | 67 | 77 | 67 | -10 | Lower | 0 | Same | +10 | Higher |
| LLOC | 70 | 67 | 67 | +3 | Higher | +3 | Higher | 0 | Same |
| Comments (CLOC) | 32 | **40 ✅** | 11 | -8 | Lower | +21 | Higher | +29 | Higher |
| Blank Lines | **17 ✅** | 22 | 18 | -5 | Lower | -1 | Lower | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02606

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.5720 ✅** | 88.7969 | 86.0123 | +1.7751 | Higher | +4.5597 | Higher | +2.7847 | Higher |
| MI (SEI) | 87.7933 | **89.4413 ✅** | 83.5636 | -1.6481 | Lower | +4.2296 | Higher | +5.8777 | Higher |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **495.4196 ✅** | 510.2822 | 575.0609 | -14.8626 | Lower | -79.6413 | Lower | -64.7787 | Lower |
| Halstead Effort | **5507.9006 ✅** | 5883.2538 | 6991.5302 | -375.3532 | Lower | -1483.6296 | Lower | -1108.2764 | Lower |
| SLOC | **19 ✅** | 21 | 24 | -2 | Lower | -5 | Lower | -3 | Lower |
| PLOC | 11 | 11 | 13 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 4 | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02617

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.8383 | **80.8246 ✅** | 73.0083 | -0.9863 | Lower | +6.8300 | Higher | +7.8163 | Higher |
| MI (SEI) | **74.0114 ✅** | 71.3023 | 65.4139 | +2.7091 | Higher | +8.5976 | Higher | +5.8885 | Higher |
| Cyclomatic (sum) | 5 | **4 ✅** | 8 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 823.7780 | **791.6176 ✅** | 1094.9081 | +32.1604 | Higher | -271.1301 | Lower | -303.2905 | Lower |
| Halstead Effort | 18911.0767 | **16623.9693 ✅** | 31836.5586 | +2287.1075 | Higher | -12925.4819 | Lower | -15212.5893 | Lower |
| SLOC | 30 | **29 ✅** | 40 | +1 | Higher | -10 | Lower | -11 | Lower |
| PLOC | **18 ✅** | 19 | 22 | -1 | Lower | -4 | Lower | -3 | Lower |
| LLOC | **15 ✅** | 16 | 18 | -1 | Lower | -3 | Lower | -2 | Lower |
| Comments (CLOC) | 7 | 5 | **10 ✅** | +2 | Higher | -3 | Lower | -5 | Lower |
| Blank Lines | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02618

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.5369 | **45.3134 ✅** | 42.0512 | -2.7765 | Lower | +0.4857 | Higher | +3.2623 | Higher |
| MI (SEI) | 11.6688 | **18.2212 ✅** | 10.0449 | -6.5523 | Lower | +1.6239 | Higher | +8.1762 | Higher |
| Cyclomatic (sum) | 22 | **21 ✅** | 23 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 18 | 25 | **17 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Halstead Volume | 4463.6750 | **3731.6780 ✅** | 4382.7126 | +731.9970 | Higher | +80.9624 | Higher | -651.0346 | Lower |
| Halstead Effort | 270158.6160 | **194284.1872 ✅** | 223153.1147 | +75874.4288 | Higher | +47005.5013 | Higher | -28868.9275 | Lower |
| SLOC | 137 | **124 ✅** | 140 | +13 | Higher | -3 | Lower | -16 | Lower |
| PLOC | 105 | **95 ✅** | 109 | +10 | Higher | -4 | Lower | -14 | Lower |
| LLOC | 75 | 71 | **67 ✅** | +4 | Higher | +8 | Higher | +4 | Higher |
| Comments (CLOC) | 14 | **16 ✅** | 13 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 18 | **17 ✅** | 18 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 6 | **5 ✅** | 7 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |

## p02639

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.1291 ✅** | 79.9715 | 67.3541 | +1.1576 | Higher | +13.7750 | Higher | +12.6173 | Higher |
| MI (SEI) | 62.5191 | **64.2818 ✅** | 43.3895 | -1.7626 | Lower | +19.1296 | Higher | +20.8922 | Higher |
| Cyclomatic (sum) | 7 | 7 | 22 | 0 | Same | -15 | Lower | -15 | Lower |
| Cognitive (sum) | 5 | 5 | 10 | 0 | Same | -5 | Lower | -5 | Lower |
| Halstead Volume | 816.8482 | 816.8482 | 1299.3447 | 0 | Same | -482.4966 | Lower | -482.4966 | Lower |
| Halstead Effort | 11175.9679 | 11175.9679 | 32833.4417 | 0 | Same | -21657.4738 | Lower | -21657.4738 | Lower |
| SLOC | **27 ✅** | 29 | 44 | -2 | Lower | -17 | Lower | -15 | Lower |
| PLOC | 22 | 22 | 37 | 0 | Same | -15 | Lower | -15 | Lower |
| LLOC | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 2 | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 5 | 0 | Same | -5 | Lower | -5 | Lower |
| Exits | 0 | 0 | 5 | 0 | Same | -5 | Lower | -5 | Lower |

## p02644

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7903 | 51.6650 | **53.7352 ✅** | +0.1253 | Higher | -1.9449 | Lower | -2.0702 | Lower |
| MI (SEI) | 20.7540 | **27.0791 ✅** | 17.5957 | -6.3251 | Lower | +3.1583 | Higher | +9.4834 | Higher |
| Cyclomatic (sum) | 14 | 18 | 14 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 28 | 37 | **20 ✅** | -9 | Lower | +8 | Higher | +17 | Higher |
| Halstead Volume | 3248.7091 | **3078.1808 ✅** | 3265.3584 | +170.5283 | Higher | -16.6494 | Lower | -187.1776 | Lower |
| Halstead Effort | 161185.9502 | **127799.4714 ✅** | 179254.5720 | +33386.4788 | Higher | -18068.6218 | Lower | -51455.1006 | Lower |
| SLOC | 96 | 93 | **85 ✅** | +3 | Higher | +11 | Higher | +8 | Higher |
| PLOC | 72 | 67 | 67 | +5 | Higher | +5 | Higher | 0 | Same |
| LLOC | 67 | **53 ✅** | 56 | +14 | Higher | +11 | Higher | -3 | Lower |
| Comments (CLOC) | 7 | **12 ✅** | 3 | -5 | Lower | +4 | Higher | +9 | Higher |
| Blank Lines | 17 | **14 ✅** | 15 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.4010 ✅** | 54.3930 | 58.0097 | +7.0080 | Higher | +3.3913 | Higher | -3.6167 | Lower |
| MI (SEI) | **44.1911 ✅** | 31.9130 | 33.7493 | +12.2781 | Higher | +10.4418 | Higher | -1.8364 | Lower |
| Cyclomatic (sum) | **10 ✅** | 19 | 15 | -9 | Lower | -5 | Lower | +4 | Higher |
| Cognitive (sum) | **14 ✅** | 29 | 24 | -15 | Lower | -10 | Lower | +5 | Higher |
| Halstead Volume | **1788.3235 ✅** | 2145.2491 | 2028 | -356.9256 | Lower | -239.6765 | Lower | +117.2491 | Higher |
| Halstead Effort | **66832.2035 ✅** | 86525.0453 | 105021.4286 | -19692.8418 | Lower | -38189.2251 | Lower | -18496.3833 | Lower |
| SLOC | **68 ✅** | 87 | 75 | -19 | Lower | -7 | Lower | +12 | Higher |
| PLOC | **46 ✅** | 64 | 56 | -18 | Lower | -10 | Lower | +8 | Higher |
| LLOC | **43 ✅** | 49 | 49 | -6 | Lower | -6 | Lower | 0 | Same |
| Comments (CLOC) | 12 | 12 | 8 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 12 | 12 | 12 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.0605 | 62.4137 | **65.3967 ✅** | +1.6468 | Higher | -1.3363 | Lower | -2.9830 | Lower |
| MI (SEI) | 44.7469 | 48.7301 | **48.8140 ✅** | -3.9832 | Lower | -4.0671 | Lower | -0.0839 | Lower |
| Cyclomatic (sum) | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 2217.7564 | 2144.2013 | **2029.8457 ✅** | +73.5551 | Higher | +187.9107 | Higher | +114.3556 | Higher |
| Halstead Effort | **95603.2839 ✅** | 111038.9952 | 99646.9720 | -15435.7113 | Lower | -4043.6880 | Lower | +11392.0232 | Higher |
| SLOC | 57 | 62 | **54 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| PLOC | 40 | 40 | **36 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| LLOC | 38 | 37 | **34 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Comments (CLOC) | 8 | **14 ✅** | 9 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02678

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.8500 ✅** | 66.4414 | 55.8128 | +1.4086 | Higher | +12.0373 | Higher | +10.6287 | Higher |
| MI (SEI) | 48.4620 | **49.3801 ✅** | 29.0716 | -0.9180 | Lower | +19.3904 | Higher | +20.3085 | Higher |
| Cyclomatic (sum) | **7 ✅** | 9 | 11 | -2 | Lower | -4 | Lower | -2 | Lower |
| Cognitive (sum) | **8 ✅** | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | **1539.8830 ✅** | 1541.2525 | 2693.1414 | -1.3695 | Lower | -1153.2584 | Lower | -1151.8889 | Lower |
| Halstead Effort | 47596.3825 | **44916.5008 ✅** | 115535.7647 | +2679.8817 | Higher | -67939.3822 | Lower | -70619.2639 | Lower |
| SLOC | **50 ✅** | 53 | 83 | -3 | Lower | -33 | Lower | -30 | Lower |
| PLOC | **36 ✅** | 38 | 62 | -2 | Lower | -26 | Lower | -24 | Lower |
| LLOC | 31 | **29 ✅** | 54 | +2 | Higher | -23 | Lower | -25 | Lower |
| Comments (CLOC) | 6 | 8 | 8 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 8 | **7 ✅** | 14 | +1 | Higher | -6 | Lower | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02703

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.0497 | 39.6297 | **41.4047 ✅** | -3.5801 | Lower | -5.3551 | Lower | -1.7750 | Lower |
| MI (SEI) | 3.8726 | 10.2346 | **12.0134 ✅** | -6.3620 | Lower | -8.1408 | Lower | -1.7788 | Lower |
| Cyclomatic (sum) | 26 | 26 | **24 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 28 | 54 | **25 ✅** | -26 | Lower | +3 | Higher | +29 | Higher |
| Halstead Volume | **4405.3968 ✅** | 4830.7320 | 4542.8981 | -425.3352 | Lower | -137.5014 | Lower | +287.8339 | Higher |
| Halstead Effort | **230329.9229 ✅** | 278521.8909 | 232192.5710 | -48191.9680 | Lower | -1862.6481 | Lower | +46329.3199 | Higher |
| SLOC | 194 | 151 | **142 ✅** | +43 | Higher | +52 | Higher | +9 | Higher |
| PLOC | 106 | 118 | **101 ✅** | -12 | Lower | +5 | Higher | +17 | Higher |
| LLOC | **76 ✅** | 82 | 80 | -6 | Lower | -4 | Lower | +2 | Higher |
| Comments (CLOC) | **22 ✅** | 19 | 17 | +3 | Higher | +5 | Higher | +2 | Higher |
| Blank Lines | 70 | **15 ✅** | 25 | +55 | Higher | +45 | Higher | -10 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |

## p02727

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.7312 | **46.1288 ✅** | 44.0884 | -2.3976 | Lower | -0.3572 | Lower | +2.0404 | Higher |
| MI (SEI) | 12.6912 | **16.2366 ✅** | 9.0770 | -3.5454 | Lower | +3.6142 | Higher | +7.1596 | Higher |
| Cyclomatic (sum) | 43 | **38 ✅** | 42 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 32 | **24 ✅** | 31 | +8 | Higher | +1 | Higher | -7 | Lower |
| Halstead Volume | 3408 | **3232.2518 ✅** | 3535.1299 | +175.7482 | Higher | -127.1299 | Lower | -302.8781 | Lower |
| Halstead Effort | 262369.9459 | **182332.1556 ✅** | 267507.0012 | +80037.7904 | Higher | -5137.0552 | Lower | -85174.8456 | Lower |
| SLOC | 103 | **97 ✅** | 101 | +6 | Higher | +2 | Higher | -4 | Lower |
| PLOC | 86 | **79 ✅** | 87 | +7 | Higher | -1 | Lower | -8 | Lower |
| LLOC | 51 | **43 ✅** | 50 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 8 | 8 | 5 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02728

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.5754 | **48.3584 ✅** | 32.4499 | -1.7831 | Lower | +14.1255 | Higher | +15.9085 | Higher |
| MI (SEI) | 17.5349 | **20.6862 ✅** | 0.4918 | -3.1513 | Lower | +17.0431 | Higher | +20.1944 | Higher |
| Cyclomatic (sum) | 22 | **20 ✅** | 30 | +2 | Higher | -8 | Lower | -10 | Lower |
| Cognitive (sum) | 28 | 26 | **18 ✅** | +2 | Higher | +10 | Higher | +8 | Higher |
| Halstead Volume | 3356.7736 | **3245.2637 ✅** | 6403.8333 | +111.5099 | Higher | -3047.0597 | Lower | -3158.5696 | Lower |
| Halstead Effort | 203496.4802 | **174042.2903 ✅** | 672402.4973 | +29454.1899 | Higher | -468906.0171 | Lower | -498360.2070 | Lower |
| SLOC | 117 | **109 ✅** | 203 | +8 | Higher | -86 | Lower | -94 | Lower |
| PLOC | 88 | **83 ✅** | 153 | +5 | Higher | -65 | Lower | -70 | Lower |
| LLOC | 71 | **62 ✅** | 119 | +9 | Higher | -48 | Lower | -57 | Lower |
| Comments (CLOC) | 12 | 12 | **26 ✅** | 0 | Same | -14 | Lower | -14 | Lower |
| Blank Lines | 17 | **14 ✅** | 26 | +3 | Higher | -9 | Lower | -12 | Lower |
| Functions | 3 | 3 | 12 | 0 | Same | -9 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |

## p02731

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 96.1400 | **98.6026 ✅** | 96.1400 | -2.4627 | Lower | 0 | Same | +2.4627 | Higher |
| MI (SEI) | 90.0032 | 66.7563 | 90.0032 | +23.2468 | Higher | 0 | Same | -23.2468 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 354.6326 | **344.9167 ✅** | 354.6326 | +9.7160 | Higher | 0 | Same | -9.7160 | Lower |
| Halstead Effort | 3806.3900 | **3702.1054 ✅** | 3806.3900 | +104.2847 | Higher | 0 | Same | -104.2847 | Lower |
| SLOC | 15 | **13 ✅** | 15 | +2 | Higher | 0 | Same | -2 | Lower |
| PLOC | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 2 | 0 | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02733

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.1582 | **51.2859 ✅** | 49.4435 | -0.1277 | Lower | +1.7147 | Higher | +1.8424 | Higher |
| MI (SEI) | 25.0588 | **28.4558 ✅** | 24.9167 | -3.3970 | Lower | +0.1420 | Higher | +3.5390 | Higher |
| Cyclomatic (sum) | 20 | **19 ✅** | 21 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 51 | **42 ✅** | 53 | +9 | Higher | -2 | Lower | -11 | Lower |
| Halstead Volume | 2724.0964 | **2526.7317 ✅** | 3098.9720 | +197.3647 | Higher | -374.8757 | Lower | -572.2404 | Lower |
| Halstead Effort | 114030.6751 | **96313.0668 ✅** | 151477.7536 | +17717.6083 | Higher | -37447.0785 | Lower | -55164.6868 | Lower |
| SLOC | **97 ✅** | 100 | 102 | -3 | Lower | -5 | Lower | -2 | Lower |
| PLOC | 74 | **70 ✅** | 76 | +4 | Higher | -2 | Lower | -6 | Lower |
| LLOC | 65 | **59 ✅** | 67 | +6 | Higher | -2 | Lower | -8 | Lower |
| Comments (CLOC) | 11 | **15 ✅** | 14 | -4 | Lower | -3 | Lower | +1 | Higher |
| Blank Lines | **12 ✅** | 15 | 13 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02734

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.5029 ✅** | 60.0203 | 59.8148 | +3.4825 | Higher | +3.6880 | Higher | +0.2055 | Higher |
| MI (SEI) | **45.2715 ✅** | 38.3783 | 30.9036 | +6.8932 | Higher | +14.3679 | Higher | +7.4747 | Higher |
| Cyclomatic (sum) | **9 ✅** | 11 | 14 | -2 | Lower | -5 | Lower | -3 | Lower |
| Cognitive (sum) | **12 ✅** | 15 | 19 | -3 | Lower | -7 | Lower | -4 | Lower |
| Halstead Volume | **1941.7650 ✅** | 2038.5774 | 2455.3253 | -96.8124 | Lower | -513.5602 | Lower | -416.7479 | Lower |
| Halstead Effort | **101975.0270 ✅** | 105789.8113 | 183573.9275 | -3814.7843 | Lower | -81598.9004 | Lower | -77784.1161 | Lower |
| SLOC | **59 ✅** | 70 | 64 | -11 | Lower | -5 | Lower | +6 | Higher |
| PLOC | **39 ✅** | 48 | 50 | -9 | Lower | -11 | Lower | -2 | Lower |
| LLOC | **37 ✅** | 43 | 49 | -6 | Lower | -12 | Lower | -6 | Lower |
| Comments (CLOC) | 9 | 9 | 4 | 0 | Same | +5 | Higher | +5 | Higher |
| Blank Lines | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02743

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.5422 ✅** | 82.6787 | 86.0685 | +4.8635 | Higher | +1.4737 | Higher | -3.3898 | Lower |
| MI (SEI) | 74.0074 | 72.0652 | **78.9792 ✅** | +1.9423 | Higher | -4.9718 | Lower | -6.9140 | Lower |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 594.5414 | 692.3877 | 594.5414 | -97.8463 | Lower | 0 | Same | +97.8463 | Higher |
| Halstead Effort | 9043.2872 | 11869.5033 | 9043.2872 | -2826.2160 | Lower | 0 | Same | +2826.2160 | Higher |
| SLOC | **21 ✅** | 27 | 23 | -6 | Lower | -2 | Lower | +4 | Higher |
| PLOC | 15 | 18 | 15 | -3 | Lower | 0 | Same | +3 | Higher |
| LLOC | 10 | 13 | 10 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 2 | 4 | 4 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02761

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1040 | 46.8276 | **55.9454 ✅** | +8.2764 | Higher | -0.8414 | Lower | -9.1178 | Lower |
| MI (SEI) | **36.9559 ✅** | 22.5442 | 30.5771 | +14.4118 | Higher | +6.3788 | Higher | -8.0329 | Lower |
| Cyclomatic (sum) | **22 ✅** | 30 | 23 | -8 | Lower | -1 | Lower | +7 | Higher |
| Cognitive (sum) | **28 ✅** | 37 | 30 | -9 | Lower | -2 | Lower | +7 | Higher |
| Halstead Volume | **2047.1805 ✅** | 2572.0218 | 2404.5478 | -524.8413 | Lower | -357.3673 | Lower | +167.4740 | Higher |
| Halstead Effort | **85432.3386 ✅** | 148065.0397 | 127841.7926 | -62632.7011 | Lower | -42409.4539 | Lower | +20223.2472 | Higher |
| SLOC | 81 | 112 | **72 ✅** | -31 | Lower | +9 | Higher | +40 | Higher |
| PLOC | 56 | 83 | **54 ✅** | -27 | Lower | +2 | Higher | +29 | Higher |
| LLOC | **40 ✅** | 55 | 41 | -15 | Lower | -1 | Lower | +14 | Higher |
| Comments (CLOC) | 15 | **16 ✅** | 7 | -1 | Lower | +8 | Higher | +9 | Higher |
| Blank Lines | 12 | 13 | **11 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 10 | 1 | -9 | Lower | 0 | Same | +9 | Higher |

## p02762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.9064 | 46.2345 | **48.1233 ✅** | +0.6718 | Higher | -1.2169 | Lower | -1.8887 | Lower |
| MI (SEI) | **20.7051 ✅** | 18.7806 | 13.4135 | +1.9245 | Higher | +7.2916 | Higher | +5.3671 | Higher |
| Cyclomatic (sum) | 19 | 19 | **18 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 17 | **15 ✅** | 19 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | **3323.8974 ✅** | 3592.4824 | 3817.4837 | -268.5850 | Lower | -493.5863 | Lower | -225.0013 | Lower |
| Halstead Effort | 170626.7351 | **153795.4100 ✅** | 224913.4153 | +16831.3251 | Higher | -54286.6802 | Lower | -71118.0053 | Lower |
| SLOC | 120 | 122 | **108 ✅** | -2 | Lower | +12 | Higher | +14 | Higher |
| PLOC | 89 | 92 | **86 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| LLOC | 71 | **69 ✅** | 73 | +2 | Higher | -2 | Lower | -4 | Lower |
| Comments (CLOC) | **16 ✅** | 15 | 6 | +1 | Higher | +10 | Higher | +9 | Higher |
| Blank Lines | 15 | 15 | 16 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02783

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.9209 ✅** | 84.3658 | 89.0162 | +5.5551 | Higher | +0.9047 | Higher | -4.6504 | Lower |
| MI (SEI) | 82.5651 | 79.6732 | **84.3165 ✅** | +2.8919 | Higher | -1.7514 | Lower | -4.6433 | Lower |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **457.8717 ✅** | 523.1869 | 468.0466 | -65.3152 | Lower | -10.1749 | Lower | +55.1403 | Higher |
| Halstead Effort | **6181.2674 ✅** | 6828.9654 | 6552.6521 | -647.6981 | Lower | -371.3848 | Lower | +276.3133 | Higher |
| SLOC | **20 ✅** | 27 | 21 | -7 | Lower | -1 | Lower | +6 | Higher |
| PLOC | 13 | 16 | 13 | -3 | Lower | 0 | Same | +3 | Higher |
| LLOC | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Comments (CLOC) | 3 | **6 ✅** | 4 | -3 | Lower | -1 | Lower | +2 | Higher |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02812

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.1038 ✅** | 78.5151 | 72.7690 | +1.5887 | Higher | +7.3348 | Higher | +5.7461 | Higher |
| MI (SEI) | **67.7839 ✅** | 61.1229 | 64.0725 | +6.6610 | Higher | +3.7115 | Higher | -2.9495 | Lower |
| Cyclomatic (sum) | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Halstead Volume | 796.3187 | **795.3929 ✅** | 915.1977 | +0.9258 | Higher | -118.8790 | Lower | -119.8048 | Lower |
| Halstead Effort | **16244.9015 ✅** | 16703.2514 | 21625.7828 | -458.3499 | Lower | -5380.8813 | Lower | -4922.5314 | Lower |
| SLOC | **29 ✅** | 32 | 43 | -3 | Lower | -14 | Lower | -11 | Lower |
| PLOC | **21 ✅** | 24 | 27 | -3 | Lower | -6 | Lower | -3 | Lower |
| LLOC | **18 ✅** | 20 | 20 | -2 | Lower | -2 | Lower | 0 | Same |
| Comments (CLOC) | 4 | 3 | **10 ✅** | +1 | Higher | -6 | Lower | -7 | Lower |
| Blank Lines | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02831

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.4417 ✅** | 73.1786 | 79.0799 | +15.2631 | Higher | +9.3617 | Higher | -5.9013 | Lower |
| MI (SEI) | **79.9332 ✅** | 60.9600 | 75.7114 | +18.9732 | Higher | +4.2219 | Higher | -14.7513 | Lower |
| Cyclomatic (sum) | **4 ✅** | 12 | 7 | -8 | Lower | -3 | Lower | +5 | Higher |
| Cognitive (sum) | **3 ✅** | 7 | 6 | -4 | Lower | -3 | Lower | +1 | Higher |
| Halstead Volume | **500.1075 ✅** | 822.0792 | 713.5293 | -321.9716 | Lower | -213.4218 | Lower | +108.5499 | Higher |
| Halstead Effort | **7918.3691 ✅** | 20587.7213 | 14912.7624 | -12669.3522 | Lower | -6994.3932 | Lower | +5674.9590 | Higher |
| SLOC | **21 ✅** | 41 | 32 | -20 | Lower | -11 | Lower | +9 | Higher |
| PLOC | **15 ✅** | 28 | 20 | -13 | Lower | -5 | Lower | +8 | Higher |
| LLOC | **10 ✅** | 15 | 15 | -5 | Lower | -5 | Lower | 0 | Same |
| Comments (CLOC) | 3 | 7 | **9 ✅** | -4 | Lower | -6 | Lower | -2 | Lower |
| Blank Lines | 3 | 6 | 3 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p02837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.8139 | **61.8124 ✅** | 59.7105 | -2.9985 | Lower | -0.8966 | Lower | +2.1020 | Higher |
| MI (SEI) | 35.4663 | **39.6905 ✅** | 35.6531 | -4.2241 | Lower | -0.1868 | Lower | +4.0373 | Higher |
| Cyclomatic (sum) | 16 | 15 | 15 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 26 | 30 | **24 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Halstead Volume | 1887.6738 | **1680.2406 ✅** | 1896 | +207.4332 | Higher | -8.3262 | Lower | -215.7594 | Lower |
| Halstead Effort | 72630.4976 | **47256.7674 ✅** | 85617.9429 | +25373.7301 | Higher | -12987.4453 | Lower | -38361.1754 | Lower |
| SLOC | 72 | **63 ✅** | 69 | +9 | Higher | +3 | Higher | -6 | Lower |
| PLOC | 56 | **47 ✅** | 53 | +9 | Higher | +3 | Higher | -6 | Lower |
| LLOC | 41 | **34 ✅** | 38 | +7 | Higher | +3 | Higher | -4 | Lower |
| Comments (CLOC) | **8 ✅** | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p02840

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.0590 | **50.4735 ✅** | 41.0654 | -2.4145 | Lower | +6.9936 | Higher | +9.4080 | Higher |
| MI (SEI) | 21.9140 | **25.6602 ✅** | 9.4127 | -3.7462 | Lower | +12.5014 | Higher | +16.2475 | Higher |
| Cyclomatic (sum) | 21 | **16 ✅** | 23 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 31 | **19 ✅** | 34 | +12 | Higher | -3 | Lower | -15 | Lower |
| Halstead Volume | 3107.7639 | **2985.2122 ✅** | 4272.9331 | +122.5517 | Higher | -1165.1692 | Lower | -1287.7208 | Lower |
| Halstead Effort | 205457.7251 | **139685.4028 ✅** | 309600.2390 | +65772.3223 | Higher | -104142.5140 | Lower | -169914.8363 | Lower |
| SLOC | 111 | **104 ✅** | 150 | +7 | Higher | -39 | Lower | -46 | Lower |
| PLOC | 82 | **75 ✅** | 116 | +7 | Higher | -34 | Lower | -41 | Lower |
| LLOC | 65 | **64 ✅** | 79 | +1 | Higher | -14 | Lower | -15 | Lower |
| Comments (CLOC) | 14 | 14 | **15 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 15 | 15 | 19 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.7016 ✅** | 67.8096 | 62.2056 | +1.8920 | Higher | +7.4960 | Higher | +5.6040 | Higher |
| MI (SEI) | **52.0064 ✅** | 51.5282 | 39.3413 | +0.4782 | Higher | +12.6651 | Higher | +12.1869 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | **1461.7588 ✅** | 1525.0452 | 2319.6249 | -63.2864 | Lower | -857.8661 | Lower | -794.5797 | Lower |
| Halstead Effort | **38980.2334 ✅** | 39789.8154 | 111341.9949 | -809.5819 | Lower | -72361.7615 | Lower | -71552.1796 | Lower |
| SLOC | **46 ✅** | 51 | 63 | -5 | Lower | -17 | Lower | -12 | Lower |
| PLOC | 34 | **33 ✅** | 47 | +1 | Higher | -13 | Lower | -14 | Lower |
| LLOC | **25 ✅** | 27 | 46 | -2 | Lower | -21 | Lower | -19 | Lower |
| Comments (CLOC) | 6 | **8 ✅** | 7 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | **6 ✅** | 10 | 9 | -4 | Lower | -3 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02862

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.2855 | 55.1770 | **57.4382 ✅** | +2.1085 | Higher | -0.1527 | Lower | -2.2612 | Lower |
| MI (SEI) | 23.9178 | **37.8725 ✅** | 21.4592 | -13.9546 | Lower | +2.4586 | Higher | +16.4133 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Halstead Volume | 2535.8392 | **2386.1019 ✅** | 2570.5943 | +149.7373 | Higher | -34.7551 | Lower | -184.4924 | Lower |
| Halstead Effort | 130831.6096 | 135918.3285 | **128888.4024 ✅** | -5086.7189 | Lower | +1943.2072 | Higher | +7029.9261 | Higher |
| SLOC | 73 | 86 | **72 ✅** | -13 | Lower | +1 | Higher | +14 | Higher |
| PLOC | **57 ✅** | 58 | 58 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **44 ✅** | 50 | 45 | -6 | Lower | -1 | Lower | +5 | Higher |
| Comments (CLOC) | 3 | **18 ✅** | 2 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |

## p02884

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.9643 | **54.1880 ✅** | 46.6728 | -0.2237 | Lower | +7.2915 | Higher | +7.5152 | Higher |
| MI (SEI) | 31.1926 | **32.2896 ✅** | 32.0520 | -1.0971 | Lower | -0.8594 | Lower | +0.2376 | Higher |
| Cyclomatic (sum) | 18 | **16 ✅** | 22 | +2 | Higher | -4 | Lower | -6 | Lower |
| Cognitive (sum) | 35 | **25 ✅** | 35 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | **2434.9851 ✅** | 2548.1790 | 2748.9370 | -113.1939 | Lower | -313.9518 | Lower | -200.7579 | Lower |
| Halstead Effort | 126132.2301 | **107787.9728 ✅** | 189785.1611 | +18344.2573 | Higher | -63652.9310 | Lower | -81997.1883 | Lower |
| SLOC | 87 | 87 | 124 | 0 | Same | -37 | Lower | -37 | Lower |
| PLOC | 65 | **61 ✅** | 73 | +4 | Higher | -8 | Lower | -12 | Lower |
| LLOC | 61 | **47 ✅** | 66 | +14 | Higher | -5 | Lower | -19 | Lower |
| Comments (CLOC) | 12 | 13 | **39 ✅** | -1 | Lower | -27 | Lower | -26 | Lower |
| Blank Lines | 12 | 13 | 12 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02885

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 91.4146 | **92.3276 ✅** | 90.2482 | -0.9129 | Lower | +1.1665 | Higher | +2.0794 | Higher |
| MI (SEI) | 86.0444 | 83.1408 | **87.4279 ✅** | +2.9036 | Higher | -1.3835 | Lower | -4.2871 | Lower |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **477.0233 ✅** | 478.2215 | 504.4394 | -1.1982 | Lower | -27.4161 | Lower | -26.2179 | Lower |
| Halstead Effort | **5824.7058 ✅** | 6163.7439 | 6132.9213 | -339.0381 | Lower | -308.2155 | Lower | +30.8226 | Higher |
| SLOC | 18 | **17 ✅** | 19 | +1 | Higher | -1 | Lower | -2 | Lower |
| PLOC | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 3 | 2 | **4 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02923

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.7520 ✅** | 72.6228 | 50.0483 | +5.1292 | Higher | +27.7037 | Higher | +22.5745 | Higher |
| MI (SEI) | **59.2879 ✅** | 52.9923 | 20.3635 | +6.2956 | Higher | +38.9244 | Higher | +32.6288 | Higher |
| Cyclomatic (sum) | **6 ✅** | 9 | 25 | -3 | Lower | -19 | Lower | -16 | Lower |
| Cognitive (sum) | **7 ✅** | 10 | 24 | -3 | Lower | -17 | Lower | -14 | Lower |
| Halstead Volume | **797.0770 ✅** | 969.0748 | 2536.6586 | -171.9977 | Lower | -1739.5816 | Lower | -1567.5838 | Lower |
| Halstead Effort | **17830.2663 ✅** | 26021.4517 | 132828.6670 | -8191.1854 | Lower | -114998.4007 | Lower | -106807.2153 | Lower |
| SLOC | **34 ✅** | 42 | 99 | -8 | Lower | -65 | Lower | -57 | Lower |
| PLOC | **25 ✅** | 32 | 76 | -7 | Lower | -51 | Lower | -44 | Lower |
| LLOC | **20 ✅** | 23 | 52 | -3 | Lower | -32 | Lower | -29 | Lower |
| Comments (CLOC) | 3 | 4 | **8 ✅** | -1 | Lower | -5 | Lower | -4 | Lower |
| Blank Lines | **6 ✅** | 7 | 15 | -1 | Lower | -9 | Lower | -8 | Lower |
| Functions | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 6 | -2 | Lower | -6 | Lower | -4 | Lower |

## p02945

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.2294 ✅** | 83.9895 | 77.7657 | +3.2399 | Higher | +9.4636 | Higher | +6.2237 | Higher |
| MI (SEI) | **83.4043 ✅** | 66.7061 | 67.4949 | +16.6982 | Higher | +15.9094 | Higher | -0.7889 | Lower |
| Cyclomatic (sum) | **2 ✅** | 4 | 7 | -2 | Lower | -5 | Lower | -3 | Lower |
| Cognitive (sum) | **0 ✅** | 2 | 3 | -2 | Lower | -3 | Lower | -1 | Lower |
| Halstead Volume | **519.5726 ✅** | 605.2554 | 760.5802 | -85.6828 | Lower | -241.0076 | Lower | -155.3248 | Lower |
| Halstead Effort | **6927.6346 ✅** | 11146.7869 | 14197.4969 | -4219.1523 | Lower | -7269.8623 | Lower | -3050.7100 | Lower |
| SLOC | **23 ✅** | 26 | 34 | -3 | Lower | -11 | Lower | -8 | Lower |
| PLOC | **13 ✅** | 19 | 22 | -6 | Lower | -9 | Lower | -3 | Lower |
| LLOC | **10 ✅** | 16 | 19 | -6 | Lower | -9 | Lower | -3 | Lower |
| Comments (CLOC) | 5 | 2 | **6 ✅** | +3 | Higher | -1 | Lower | -4 | Lower |
| Blank Lines | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.6686 | 82.1359 | **84.5811 ✅** | -1.4673 | Lower | -3.9125 | Lower | -2.4452 | Lower |
| MI (SEI) | **79.1746 ✅** | 70.9378 | 67.9456 | +8.2368 | Higher | +11.2290 | Higher | +2.9922 | Higher |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 4 | 6 | **3 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | **600.2829 ✅** | 656.5482 | 610.3717 | -56.2653 | Lower | -10.0888 | Lower | +46.1765 | Higher |
| Halstead Effort | **9604.5264 ✅** | 11057.6543 | 10053.1808 | -1453.1279 | Lower | -448.6543 | Lower | +1004.4736 | Higher |
| SLOC | 32 | 28 | **25 ✅** | +4 | Higher | +7 | Higher | +3 | Higher |
| PLOC | 18 | 19 | 18 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 14 | **11 ✅** | 16 | +3 | Higher | -2 | Lower | -5 | Lower |
| Comments (CLOC) | **10 ✅** | 4 | 2 | +6 | Higher | +8 | Higher | +2 | Higher |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02953

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.4667 ✅** | 78.1818 | 70.6721 | +1.2849 | Higher | +8.7945 | Higher | +7.5096 | Higher |
| MI (SEI) | 65.9666 | **66.1611 ✅** | 56.9995 | -0.1945 | Lower | +8.9671 | Higher | +9.1616 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 9 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | **6 ✅** | 11 | 10 | -5 | Lower | -4 | Lower | +1 | Higher |
| Halstead Volume | **764.3204 ✅** | 770.5296 | 993.3187 | -6.2092 | Lower | -228.9983 | Lower | -222.7891 | Lower |
| Halstead Effort | **16399.6578 ✅** | 19373.3155 | 27965.7416 | -2973.6577 | Lower | -11566.0837 | Lower | -8592.4260 | Lower |
| SLOC | **31 ✅** | 33 | 47 | -2 | Lower | -16 | Lower | -14 | Lower |
| PLOC | **22 ✅** | 23 | 32 | -1 | Lower | -10 | Lower | -9 | Lower |
| LLOC | **16 ✅** | 17 | 23 | -1 | Lower | -7 | Lower | -6 | Lower |
| Comments (CLOC) | 4 | 5 | **8 ✅** | -1 | Lower | -4 | Lower | -3 | Lower |
| Blank Lines | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p02956

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **30.5833 ✅** | 26.0285 | 27.4575 | +4.5547 | Higher | +3.1258 | Higher | -1.4290 | Lower |
| MI (SEI) | **-3.0783 ✅** | -10.4761 | -7.7333 | +7.3977 | Higher | +4.6550 | Higher | -2.7427 | Lower |
| Cyclomatic (sum) | **38 ✅** | 43 | 40 | -5 | Lower | -2 | Lower | +3 | Higher |
| Cognitive (sum) | 40 | **39 ✅** | 47 | +1 | Higher | -7 | Lower | -8 | Lower |
| Halstead Volume | **6848.3610 ✅** | 7851.7771 | 7908.8224 | -1003.4161 | Lower | -1060.4614 | Lower | -57.0454 | Lower |
| Halstead Effort | **748548.3110 ✅** | 785882.3555 | 1005550.2826 | -37334.0445 | Lower | -257001.9716 | Lower | -219667.9271 | Lower |
| SLOC | **199 ✅** | 235 | 224 | -36 | Lower | -25 | Lower | +11 | Higher |
| PLOC | **164 ✅** | 184 | 177 | -20 | Lower | -13 | Lower | +7 | Higher |
| LLOC | **158 ✅** | 159 | 179 | -1 | Lower | -21 | Lower | -20 | Lower |
| Comments (CLOC) | 22 | 23 | **24 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | **19 ✅** | 29 | 26 | -10 | Lower | -7 | Lower | +3 | Higher |
| Functions | 5 | 8 | **4 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Closures | 3 | 3 | **0 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Exits | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02957

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.0531 | **91.4016 ✅** | 81.0461 | -3.3485 | Lower | +7.0070 | Higher | +10.3555 | Higher |
| MI (SEI) | **78.7074 ✅** | 56.4692 | 69.1434 | +22.2382 | Higher | +9.5640 | Higher | -12.6741 | Lower |
| Cyclomatic (sum) | 3 | 3 | 7 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 487.2819 | **478.2215 ✅** | 664.3364 | +9.0604 | Higher | -177.0546 | Lower | -186.1149 | Lower |
| Halstead Effort | 6673.0547 | **6163.7439 ✅** | 11188.0297 | +509.3109 | Higher | -4514.9750 | Lower | -5024.2859 | Lower |
| SLOC | 22 | **18 ✅** | 29 | +4 | Higher | -7 | Lower | -11 | Lower |
| PLOC | 14 | 14 | 21 | 0 | Same | -7 | Lower | -7 | Lower |
| LLOC | 10 | **9 ✅** | 11 | +1 | Higher | -1 | Lower | -2 | Lower |
| Comments (CLOC) | 3 | 0 | **4 ✅** | +3 | Higher | -1 | Lower | -4 | Lower |
| Blank Lines | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 3 | +1 | Higher | -2 | Lower | -3 | Lower |

## p02959

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.8965 | **73.7702 ✅** | 66.1230 | -4.8737 | Lower | +2.7735 | Higher | +7.6472 | Higher |
| MI (SEI) | 59.8753 | **64.8617 ✅** | 41.6430 | -4.9864 | Lower | +18.2323 | Higher | +23.2187 | Higher |
| Cyclomatic (sum) | 9 | **6 ✅** | 11 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 15 | **8 ✅** | 16 | +7 | Higher | -1 | Lower | -8 | Lower |
| Halstead Volume | 1397.6145 | **1033.1535 ✅** | 1591.5674 | +364.4610 | Higher | -193.9530 | Lower | -558.4139 | Lower |
| Halstead Effort | 58816.2766 | **29548.1909 ✅** | 73212.1025 | +29268.0858 | Higher | -14395.8259 | Lower | -43663.9116 | Lower |
| SLOC | 47 | **40 ✅** | 52 | +7 | Higher | -5 | Lower | -12 | Lower |
| PLOC | 27 | **22 ✅** | 38 | +5 | Higher | -11 | Lower | -16 | Lower |
| LLOC | 29 | **20 ✅** | 29 | +9 | Higher | 0 | Same | -9 | Lower |
| Comments (CLOC) | **12 ✅** | 9 | 4 | +3 | Higher | +8 | Higher | +5 | Higher |
| Blank Lines | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02988

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.8901 | **74.8239 ✅** | 60.4418 | -0.9338 | Lower | +13.4483 | Higher | +14.3821 | Higher |
| MI (SEI) | 53.3681 | **55.2755 ✅** | 42.3045 | -1.9074 | Lower | +11.0636 | Higher | +12.9710 | Higher |
| Cyclomatic (sum) | 11 | 11 | 16 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 17 | **11 ✅** | 27 | +6 | Higher | -10 | Lower | -16 | Lower |
| Halstead Volume | 1031.7796 | **1025.1467 ✅** | 1649.3124 | +6.6329 | Higher | -617.5328 | Lower | -624.1657 | Lower |
| Halstead Effort | 31526.5986 | **29660.9104 ✅** | 57254.7011 | +1865.6881 | Higher | -25728.1026 | Lower | -27593.7907 | Lower |
| SLOC | 37 | **35 ✅** | 68 | +2 | Higher | -31 | Lower | -33 | Lower |
| PLOC | 30 | **27 ✅** | 47 | +3 | Higher | -17 | Lower | -20 | Lower |
| LLOC | 19 | **18 ✅** | 32 | +1 | Higher | -13 | Lower | -14 | Lower |
| Comments (CLOC) | 3 | 3 | **11 ✅** | 0 | Same | -8 | Lower | -8 | Lower |
| Blank Lines | **5 ✅** | 6 | 10 | -1 | Lower | -5 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03003

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.8675 ✅** | 59.0791 | 66.0994 | +8.7884 | Higher | +1.7681 | Higher | -7.0203 | Lower |
| MI (SEI) | **44.8522 ✅** | 39.2140 | 41.9011 | +5.6382 | Higher | +2.9511 | Higher | -2.6871 | Lower |
| Cyclomatic (sum) | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 16 | **15 ✅** | 17 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | **1526.2790 ✅** | 2140.9962 | 1625.0220 | -614.7172 | Lower | -98.7431 | Lower | +515.9741 | Higher |
| Halstead Effort | **77557.5829 ✅** | 110536.5741 | 85638.6610 | -32978.9912 | Lower | -8081.0781 | Lower | +24897.9131 | Higher |
| SLOC | **48 ✅** | 72 | 51 | -24 | Lower | -3 | Lower | +21 | Higher |
| PLOC | **36 ✅** | 49 | 39 | -13 | Lower | -3 | Lower | +10 | Higher |
| LLOC | 32 | 40 | 32 | -8 | Lower | 0 | Same | +8 | Higher |
| Comments (CLOC) | 4 | **11 ✅** | 4 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03007

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **45.8280 ✅** | 41.4597 | 44.3259 | +4.3682 | Higher | +1.5021 | Higher | -2.8662 | Lower |
| MI (SEI) | **14.2287 ✅** | 12.6558 | 4.4992 | +1.5729 | Higher | +9.7295 | Higher | +8.1566 | Higher |
| Cyclomatic (sum) | **19 ✅** | 23 | 22 | -4 | Lower | -3 | Lower | +1 | Higher |
| Cognitive (sum) | **36 ✅** | 57 | 40 | -21 | Lower | -4 | Lower | +17 | Higher |
| Halstead Volume | **4425.5855 ✅** | 4698.4068 | 4907.4982 | -272.8213 | Lower | -481.9127 | Lower | -209.0914 | Lower |
| Halstead Effort | **533940.9083 ✅** | 535265.9946 | 567354.6701 | -1325.0863 | Lower | -33413.7618 | Lower | -32088.6755 | Lower |
| SLOC | **117 ✅** | 142 | 119 | -25 | Lower | -2 | Lower | +23 | Higher |
| PLOC | **95 ✅** | 110 | 106 | -15 | Lower | -11 | Lower | +4 | Higher |
| LLOC | **78 ✅** | 81 | 79 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 10 | **18 ✅** | 4 | -8 | Lower | +6 | Higher | +14 | Higher |
| Blank Lines | 12 | 14 | **10 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |

## p03029

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 94.2770 | **95.1987 ✅** | 93.2080 | -0.9217 | Lower | +1.0690 | Higher | +1.9907 | Higher |
| MI (SEI) | 86.5530 | 81.3164 | **89.2643 ✅** | +5.2366 | Higher | -2.7113 | Lower | -7.9479 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **415 ✅** | 425 | 421.9926 | -10 | Lower | -6.9926 | Lower | +3.0074 | Higher |
| Halstead Effort | 4394.1176 | 4687.5000 | **4033.7527 ✅** | -293.3824 | Lower | +360.3649 | Higher | +653.7473 | Higher |
| SLOC | 16 | **15 ✅** | 17 | +1 | Higher | -1 | Lower | -2 | Lower |
| PLOC | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 2 | 1 | **3 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03034

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.7233 ✅** | 64.2443 | 64.2224 | +1.4790 | Higher | +1.5009 | Higher | +0.0219 | Higher |
| MI (SEI) | **46.6900 ✅** | 43.2577 | 46.4064 | +3.4323 | Higher | +0.2836 | Higher | -3.1487 | Lower |
| Cyclomatic (sum) | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 12 | 15 | **11 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Halstead Volume | **1320.9789 ✅** | 1329.0504 | 1405.1903 | -8.0715 | Lower | -84.2115 | Lower | -76.1399 | Lower |
| Halstead Effort | **46234.2602 ✅** | 47867.2501 | 54670.6862 | -1632.9899 | Lower | -8436.4260 | Lower | -6803.4361 | Lower |
| SLOC | **55 ✅** | 61 | 60 | -6 | Lower | -5 | Lower | +1 | Higher |
| PLOC | **39 ✅** | 43 | 41 | -4 | Lower | -2 | Lower | +2 | Higher |
| LLOC | **31 ✅** | 36 | 35 | -5 | Lower | -4 | Lower | +1 | Higher |
| Comments (CLOC) | 7 | 7 | **9 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03043

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.9799 ✅** | 73.0663 | 62.0416 | +1.9137 | Higher | +12.9383 | Higher | +11.0247 | Higher |
| MI (SEI) | 63.8308 | **64.1958 ✅** | 49.1638 | -0.3650 | Lower | +14.6671 | Higher | +15.0321 | Higher |
| Cyclomatic (sum) | 5 | 5 | 12 | 0 | Same | -7 | Lower | -7 | Lower |
| Cognitive (sum) | 4 | 4 | 10 | 0 | Same | -6 | Lower | -6 | Lower |
| Halstead Volume | 1004.0193 | **987.0020 ✅** | 1447.1882 | +17.0173 | Higher | -443.1689 | Lower | -460.1862 | Lower |
| Halstead Effort | 26327.6168 | **22701.0462 ✅** | 56183.5804 | +3626.5706 | Higher | -29855.9636 | Lower | -33482.5342 | Lower |
| SLOC | **38 ✅** | 43 | 68 | -5 | Lower | -30 | Lower | -25 | Lower |
| PLOC | 26 | **25 ✅** | 43 | +1 | Higher | -17 | Lower | -18 | Lower |
| LLOC | 24 | **22 ✅** | 33 | +2 | Higher | -9 | Lower | -11 | Lower |
| Comments (CLOC) | 7 | 10 | **16 ✅** | -3 | Lower | -9 | Lower | -6 | Lower |
| Blank Lines | **6 ✅** | 8 | 11 | -2 | Lower | -5 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |

## p03051

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.1887 ✅** | 55.3493 | 55.1345 | +1.8394 | Higher | +2.0542 | Higher | +0.2149 | Higher |
| MI (SEI) | 31.9696 | **35.4656 ✅** | 29.8169 | -3.4961 | Lower | +2.1526 | Higher | +5.6487 | Higher |
| Cyclomatic (sum) | **12 ✅** | 13 | 14 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 13 | **12 ✅** | 15 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 2498.3865 | **2327.3740 ✅** | 2790.5108 | +171.0125 | Higher | -292.1243 | Lower | -463.1367 | Lower |
| Halstead Effort | 148529.0776 | **126245.8500 ✅** | 179892.8138 | +22283.2275 | Higher | -31363.7362 | Lower | -53646.9638 | Lower |
| SLOC | **77 ✅** | 87 | 82 | -10 | Lower | -5 | Lower | +5 | Higher |
| PLOC | **56 ✅** | 57 | 60 | -1 | Lower | -4 | Lower | -3 | Lower |
| LLOC | 46 | **44 ✅** | 51 | +2 | Higher | -5 | Lower | -7 | Lower |
| Comments (CLOC) | 8 | **15 ✅** | 9 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 14 | 15 | **13 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |

## p03053

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.1363 ✅** | 55.6113 | 54.5289 | +4.5250 | Higher | +5.6073 | Higher | +1.0823 | Higher |
| MI (SEI) | 33.7203 | **35.7147 ✅** | 21.8794 | -1.9944 | Lower | +11.8409 | Higher | +13.8353 | Higher |
| Cyclomatic (sum) | **15 ✅** | 16 | 18 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 21 | 30 | **20 ✅** | -9 | Lower | +1 | Higher | +10 | Higher |
| Halstead Volume | 2319.3233 | **2161.9220 ✅** | 2950.1694 | +157.4014 | Higher | -630.8460 | Lower | -788.2474 | Lower |
| Halstead Effort | **88555.9822 ✅** | 105007.6380 | 164809.9837 | -16451.6558 | Lower | -76254.0016 | Lower | -59802.3458 | Lower |
| SLOC | **63 ✅** | 84 | 79 | -21 | Lower | -16 | Lower | +5 | Higher |
| PLOC | **48 ✅** | 62 | 64 | -14 | Lower | -16 | Lower | -2 | Lower |
| LLOC | **37 ✅** | 49 | 51 | -12 | Lower | -14 | Lower | -2 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 4 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 10 | **8 ✅** | 11 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03055

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.9134 | **56.4536 ✅** | 27.9934 | -7.5402 | Lower | +20.9200 | Higher | +28.4602 | Higher |
| MI (SEI) | 22.9603 | **27.8542 ✅** | -12.1983 | -4.8940 | Lower | +35.1586 | Higher | +40.0526 | Higher |
| Cyclomatic (sum) | 16 | **15 ✅** | 37 | +1 | Higher | -21 | Lower | -22 | Lower |
| Cognitive (sum) | 23 | **18 ✅** | 64 | +5 | Higher | -41 | Lower | -46 | Lower |
| Halstead Volume | 3582.6793 | **2237.2428 ✅** | 7709.6682 | +1345.4365 | Higher | -4126.9890 | Lower | -5472.4254 | Lower |
| Halstead Effort | 193464.6803 | **82458.3773 ✅** | 995610.6053 | +111006.3030 | Higher | -802145.9250 | Lower | -913152.2280 | Lower |
| SLOC | 108 | **80 ✅** | 228 | +28 | Higher | -120 | Lower | -148 | Lower |
| PLOC | 79 | **62 ✅** | 187 | +17 | Higher | -108 | Lower | -125 | Lower |
| LLOC | 71 | **40 ✅** | 168 | +31 | Higher | -97 | Lower | -128 | Lower |
| Comments (CLOC) | 14 | 6 | **15 ✅** | +8 | Higher | -1 | Lower | -9 | Lower |
| Blank Lines | 16 | **12 ✅** | 27 | +4 | Higher | -11 | Lower | -15 | Lower |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | **0 ✅** | 1 | 4 | -1 | Lower | -4 | Lower | -3 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.7868 ✅** | 59.9980 | 57.0314 | +5.7888 | Higher | +8.7554 | Higher | +2.9666 | Higher |
| MI (SEI) | **55.6055 ✅** | 41.5344 | 42.6695 | +14.0711 | Higher | +12.9360 | Higher | -1.1351 | Lower |
| Cyclomatic (sum) | **7 ✅** | 11 | 11 | -4 | Lower | -4 | Lower | 0 | Same |
| Cognitive (sum) | **6 ✅** | 8 | 9 | -2 | Lower | -3 | Lower | -1 | Lower |
| Halstead Volume | **1522.3843 ✅** | 1875.3170 | 2052.4642 | -352.9327 | Lower | -530.0799 | Lower | -177.1472 | Lower |
| Halstead Effort | **63841.9220 ✅** | 80896.4874 | 81145.6389 | -17054.5654 | Lower | -17303.7169 | Lower | -249.1514 | Lower |
| SLOC | **57 ✅** | 72 | 84 | -15 | Lower | -27 | Lower | -12 | Lower |
| PLOC | **36 ✅** | 50 | 49 | -14 | Lower | -13 | Lower | +1 | Higher |
| LLOC | **36 ✅** | 45 | 39 | -9 | Lower | -3 | Lower | +6 | Higher |
| Comments (CLOC) | 15 | 12 | **21 ✅** | +3 | Higher | -6 | Lower | -9 | Lower |
| Blank Lines | **10 ✅** | 11 | 14 | -1 | Lower | -4 | Lower | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.2110 ✅** | 61.5385 | 53.5734 | +0.6724 | Higher | +8.6376 | Higher | +7.9652 | Higher |
| MI (SEI) | **47.6914 ✅** | 44.9810 | 31.9031 | +2.7104 | Higher | +15.7883 | Higher | +13.0779 | Higher |
| Cyclomatic (sum) | **9 ✅** | 12 | 16 | -3 | Lower | -7 | Lower | -4 | Lower |
| Cognitive (sum) | **11 ✅** | 12 | 24 | -1 | Lower | -13 | Lower | -12 | Lower |
| Halstead Volume | 1755.4983 | **1749.5675 ✅** | 2580.4282 | +5.9307 | Higher | -824.9299 | Lower | -830.8606 | Lower |
| Halstead Effort | **53658.1900 ✅** | 60714.7213 | 125570.8350 | -7056.5313 | Lower | -71912.6450 | Lower | -64856.1137 | Lower |
| SLOC | 66 | 66 | 90 | 0 | Same | -24 | Lower | -24 | Lower |
| PLOC | 46 | **44 ✅** | 64 | +2 | Higher | -18 | Lower | -20 | Lower |
| LLOC | 39 | 39 | 58 | 0 | Same | -19 | Lower | -19 | Lower |
| Comments (CLOC) | 14 | 12 | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | **8 ✅** | 11 | 12 | -3 | Lower | -4 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.3758 | **72.9139 ✅** | 68.9091 | -1.5380 | Lower | +2.4668 | Higher | +4.0048 | Higher |
| MI (SEI) | 51.2502 | **58.2441 ✅** | 50.6309 | -6.9939 | Lower | +0.6193 | Higher | +7.6131 | Higher |
| Cyclomatic (sum) | 12 | 11 | 11 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 11 | **9 ✅** | 10 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 1002.3861 | **838.7429 ✅** | 1120.8195 | +163.6432 | Higher | -118.4334 | Lower | -282.0766 | Lower |
| Halstead Effort | 26545.9486 | **20297.5786 ✅** | 30732.1476 | +6248.3700 | Higher | -4186.1990 | Lower | -10434.5690 | Lower |
| SLOC | 43 | **42 ✅** | 49 | +1 | Higher | -6 | Lower | -7 | Lower |
| PLOC | 32 | **30 ✅** | 36 | +2 | Higher | -4 | Lower | -6 | Lower |
| LLOC | 18 | **16 ✅** | 24 | +2 | Higher | -6 | Lower | -8 | Lower |
| Comments (CLOC) | 4 | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **91.8920 ✅** | 89.9911 | 87.6152 | +1.9009 | Higher | +4.2769 | Higher | +2.3759 | Higher |
| MI (SEI) | 75.5246 | **83.3063 ✅** | 78.0756 | -7.7817 | Lower | -2.5510 | Lower | +5.2307 | Higher |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **520 ✅** | 530 | 530.0990 | -10 | Lower | -10.0990 | Lower | -0.0990 | Lower |
| Halstead Effort | **6652.9412 ✅** | 7014.7059 | 7454.5173 | -361.7647 | Lower | -801.5761 | Lower | -439.8114 | Lower |
| SLOC | **17 ✅** | 19 | 22 | -2 | Lower | -5 | Lower | -3 | Lower |
| PLOC | 12 | 12 | 15 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 1 | 3 | 3 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.9511 | **79.5071 ✅** | 56.4654 | -10.5560 | Lower | +12.4856 | Higher | +23.0416 | Higher |
| MI (SEI) | 55.8919 | **59.1872 ✅** | 27.7780 | -3.2954 | Lower | +28.1139 | Higher | +31.4093 | Higher |
| Cyclomatic (sum) | 17 | **7 ✅** | 25 | +10 | Higher | -8 | Lower | -18 | Lower |
| Cognitive (sum) | 12 | **9 ✅** | 39 | +3 | Higher | -27 | Lower | -30 | Lower |
| Halstead Volume | 1111.7125 | **803.6328 ✅** | 1753.6782 | +308.0797 | Higher | -641.9657 | Lower | -950.0454 | Lower |
| Halstead Effort | 33618.1875 | **19829.6394 ✅** | 99959.6588 | +13788.5481 | Higher | -66341.4714 | Lower | -80130.0195 | Lower |
| SLOC | 45 | **30 ✅** | 75 | +15 | Higher | -30 | Lower | -45 | Lower |
| PLOC | 33 | **23 ✅** | 66 | +10 | Higher | -33 | Lower | -43 | Lower |
| LLOC | **19 ✅** | 20 | 48 | -1 | Lower | -29 | Lower | -28 | Lower |
| Comments (CLOC) | **8 ✅** | 2 | 5 | +6 | Higher | +3 | Higher | -3 | Lower |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 4 | **0 ✅** | 3 | +4 | Higher | +1 | Higher | -3 | Lower |

## p03108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.1254 | **55.0019 ✅** | 48.7263 | -3.8765 | Lower | +2.3992 | Higher | +6.2756 | Higher |
| MI (SEI) | **30.3506 ✅** | 27.8528 | 19.2297 | +2.4978 | Higher | +11.1209 | Higher | +8.6231 | Higher |
| Cyclomatic (sum) | 14 | **13 ✅** | 16 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 7 | 7 | 15 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | 2964.9260 | **2675.1400 ✅** | 4054.6109 | +289.7860 | Higher | -1089.6849 | Lower | -1379.4709 | Lower |
| Halstead Effort | 136954.3461 | **112831.4603 ✅** | 292734.4593 | +24122.8858 | Higher | -155780.1132 | Lower | -179902.9990 | Lower |
| SLOC | 103 | **85 ✅** | 105 | +18 | Higher | -2 | Lower | -20 | Lower |
| PLOC | 72 | **66 ✅** | 80 | +6 | Higher | -8 | Lower | -14 | Lower |
| LLOC | 53 | **45 ✅** | 68 | +8 | Higher | -15 | Lower | -23 | Lower |
| Comments (CLOC) | **19 ✅** | 8 | 10 | +11 | Higher | +9 | Higher | -2 | Lower |
| Blank Lines | 14 | **11 ✅** | 17 | +3 | Higher | -3 | Lower | -6 | Lower |
| Functions | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |

## p03115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.4233 | **64.8537 ✅** | 61.0682 | -4.4304 | Lower | -0.6449 | Lower | +3.7855 | Higher |
| MI (SEI) | 33.5769 | **39.5478 ✅** | 27.7644 | -5.9709 | Lower | +5.8125 | Higher | +11.7834 | Higher |
| Cyclomatic (sum) | 14 | **12 ✅** | 16 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 12 | **11 ✅** | 13 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 1984.5129 | **1728.0789 ✅** | 1949.6963 | +256.4339 | Higher | +34.8166 | Higher | -221.6174 | Lower |
| Halstead Effort | 82637.1520 | **58796.8322 ✅** | 86566.5157 | +23840.3197 | Higher | -3929.3638 | Lower | -27769.6835 | Lower |
| SLOC | 66 | **54 ✅** | 62 | +12 | Higher | +4 | Higher | -8 | Lower |
| PLOC | 54 | **44 ✅** | 52 | +10 | Higher | +2 | Higher | -8 | Lower |
| LLOC | 38 | **29 ✅** | 35 | +9 | Higher | +3 | Higher | -6 | Lower |
| Comments (CLOC) | **5 ✅** | 4 | 2 | +1 | Higher | +3 | Higher | +2 | Higher |
| Blank Lines | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |

## p03123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.1409 | **44.7989 ✅** | 40.2036 | -3.6580 | Lower | +0.9373 | Higher | +4.5952 | Higher |
| MI (SEI) | 0.4325 | **13.4591 ✅** | -1.0976 | -13.0266 | Lower | +1.5302 | Higher | +14.5568 | Higher |
| Cyclomatic (sum) | 25 | **19 ✅** | 26 | +6 | Higher | -1 | Lower | -7 | Lower |
| Cognitive (sum) | 24 | **22 ✅** | 25 | +2 | Higher | -1 | Lower | -3 | Lower |
| Halstead Volume | 4377.5958 | **3793.1485 ✅** | 4416.4502 | +584.4473 | Higher | -38.8544 | Lower | -623.3017 | Lower |
| Halstead Effort | 256016.3943 | **200257.9183 ✅** | 275092.4489 | +55758.4760 | Higher | -19076.0546 | Lower | -74834.5306 | Lower |
| SLOC | 144 | **131 ✅** | 150 | +13 | Higher | -6 | Lower | -19 | Lower |
| PLOC | 119 | **97 ✅** | 122 | +22 | Higher | -3 | Lower | -25 | Lower |
| LLOC | 90 | **87 ✅** | 92 | +3 | Higher | -2 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | **12 ✅** | 5 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | **20 ✅** | 23 | 23 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 8 | **5 ✅** | 9 | +3 | Higher | -1 | Lower | -4 | Lower |

## p03142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.3248 | 45.1536 | **53.0329 ✅** | +4.1712 | Higher | -3.7080 | Lower | -7.8793 | Lower |
| MI (SEI) | 25.2961 | 22.7006 | **30.5467 ✅** | +2.5954 | Higher | -5.2506 | Lower | -7.8460 | Lower |
| Cyclomatic (sum) | 28 | 31 | **18 ✅** | -3 | Lower | +10 | Higher | +13 | Higher |
| Cognitive (sum) | 43 | 29 | **21 ✅** | +14 | Higher | +22 | Higher | +8 | Higher |
| Halstead Volume | 2809.9213 | 3126.8670 | **2713.5311 ✅** | -316.9457 | Lower | +96.3901 | Higher | +413.3359 | Higher |
| Halstead Effort | 169087.0112 | 166212.5230 | **127824.6362 ✅** | +2874.4882 | Higher | +41262.3750 | Higher | +38387.8868 | Higher |
| SLOC | 96 | 115 | **89 ✅** | -19 | Lower | +7 | Higher | +26 | Higher |
| PLOC | 74 | 78 | **64 ✅** | -4 | Lower | +10 | Higher | +14 | Higher |
| LLOC | 49 | 47 | 47 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 13 | **20 ✅** | 13 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | **10 ✅** | 17 | 12 | -7 | Lower | -2 | Lower | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | 4 | **0 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03145

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.8432 ✅** | 81.1384 | 89.4380 | +8.7048 | Higher | +0.4052 | Higher | -8.2996 | Lower |
| MI (SEI) | **91.6345 ✅** | 75.7852 | 88.5060 | +15.8493 | Higher | +3.1285 | Higher | -12.7208 | Lower |
| Cyclomatic (sum) | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | **485.7822 ✅** | 670.5629 | 525.1448 | -184.7808 | Lower | -39.3626 | Lower | +145.4181 | Higher |
| Halstead Effort | **4829.2463 ✅** | 12740.6959 | 5309.7975 | -7911.4496 | Lower | -480.5512 | Lower | +7430.8983 | Higher |
| SLOC | 20 | 30 | 20 | -10 | Lower | 0 | Same | +10 | Higher |
| PLOC | **11 ✅** | 18 | 12 | -7 | Lower | -1 | Lower | +6 | Higher |
| LLOC | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 6 | **7 ✅** | 5 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03157

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.0022 | **51.5379 ✅** | 47.2202 | -0.5357 | Lower | +3.7820 | Higher | +4.3177 | Higher |
| MI (SEI) | **24.5493 ✅** | 22.2543 | 11.7302 | +2.2950 | Higher | +12.8191 | Higher | +10.5241 | Higher |
| Cyclomatic (sum) | 26 | **24 ✅** | 26 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 38 | 40 | **35 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Halstead Volume | 3132.2355 | **2873.6135 ✅** | 3585.8557 | +258.6220 | Higher | -453.6202 | Lower | -712.2422 | Lower |
| Halstead Effort | 175405.1901 | **133650.1376 ✅** | 213557.6303 | +41755.0525 | Higher | -38152.4402 | Lower | -79907.4927 | Lower |
| SLOC | **86 ✅** | 88 | 104 | -2 | Lower | -18 | Lower | -16 | Lower |
| PLOC | **67 ✅** | 68 | 87 | -1 | Lower | -20 | Lower | -19 | Lower |
| LLOC | **47 ✅** | 48 | 53 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | **9 ✅** | 7 | 5 | +2 | Higher | +4 | Higher | +2 | Higher |
| Blank Lines | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03164

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.2300 ✅** | 51.9780 | 55.9289 | +7.2520 | Higher | +3.3011 | Higher | -3.9509 | Lower |
| MI (SEI) | **36.3994 ✅** | 31.3894 | 32.7402 | +5.0100 | Higher | +3.6591 | Higher | -1.3509 | Lower |
| Cyclomatic (sum) | **21 ✅** | 25 | 23 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | **17 ✅** | 23 | 22 | -6 | Lower | -5 | Lower | +1 | Higher |
| Halstead Volume | **2016 ✅** | 2355.3531 | 2038.2525 | -339.3531 | Lower | -22.2525 | Lower | +317.1007 | Higher |
| Halstead Effort | **80769.2308 ✅** | 103525.9871 | 85821.1560 | -22756.7564 | Lower | -5051.9252 | Lower | +17704.8312 | Higher |
| SLOC | **64 ✅** | 90 | 76 | -26 | Lower | -12 | Lower | +14 | Higher |
| PLOC | **50 ✅** | 66 | 56 | -16 | Lower | -6 | Lower | +10 | Higher |
| LLOC | **31 ✅** | 36 | 35 | -5 | Lower | -4 | Lower | +1 | Higher |
| Comments (CLOC) | 7 | **15 ✅** | 9 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | **8 ✅** | 9 | 11 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03165

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.9190 | 58.9563 | **62.4126 ✅** | +2.9627 | Higher | -0.4936 | Lower | -3.4563 | Lower |
| MI (SEI) | **44.6998 ✅** | 40.4388 | 41.7701 | +4.2610 | Higher | +2.9297 | Higher | -1.3313 | Lower |
| Cyclomatic (sum) | 11 | 15 | **10 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Cognitive (sum) | 19 | 24 | **16 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Halstead Volume | **1870.6805 ✅** | 1919.7394 | 1963.0741 | -49.0590 | Lower | -92.3936 | Lower | -43.3346 | Lower |
| Halstead Effort | 81454.7733 | 106665.5228 | **77825.5545 ✅** | -25210.7495 | Lower | +3629.2189 | Higher | +28839.9684 | Higher |
| SLOC | 64 | 72 | **62 ✅** | -8 | Lower | +2 | Higher | +10 | Higher |
| PLOC | 45 | 51 | 45 | -6 | Lower | 0 | Same | +6 | Higher |
| LLOC | 41 | **37 ✅** | 40 | +4 | Higher | +1 | Higher | -3 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 8 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03173

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.5463 ✅** | 63.5211 | 63.7893 | +1.0252 | Higher | +0.7570 | Higher | -0.2682 | Lower |
| MI (SEI) | **46.0705 ✅** | 45.8165 | 41.8314 | +0.2540 | Higher | +4.2391 | Higher | +3.9850 | Higher |
| Cyclomatic (sum) | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **9 ✅** | 14 | 11 | -5 | Lower | -2 | Lower | +3 | Higher |
| Halstead Volume | 1788.3235 | **1787.2067 ✅** | 1893.4344 | +1.1168 | Higher | -105.1109 | Lower | -106.2277 | Lower |
| Halstead Effort | 80311.9821 | **75732.8838 ✅** | 102331.5242 | +4579.0983 | Higher | -22019.5422 | Lower | -26598.6405 | Lower |
| SLOC | 56 | 58 | 56 | -2 | Lower | 0 | Same | +2 | Higher |
| PLOC | **38 ✅** | 39 | 40 | -1 | Lower | -2 | Lower | -1 | Lower |
| LLOC | 29 | 30 | 29 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 8 | **9 ✅** | 6 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p03183

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.1396 ✅** | 57.4468 | 56.7810 | +3.6928 | Higher | +4.3585 | Higher | +0.6658 | Higher |
| MI (SEI) | **43.9157 ✅** | 38.9991 | 26.9586 | +4.9166 | Higher | +16.9572 | Higher | +12.0405 | Higher |
| Cyclomatic (sum) | **11 ✅** | 12 | 16 | -1 | Lower | -5 | Lower | -4 | Lower |
| Cognitive (sum) | **14 ✅** | 20 | 22 | -6 | Lower | -8 | Lower | -2 | Lower |
| Halstead Volume | **1799.1613 ✅** | 1881.8226 | 2174.7492 | -82.6614 | Lower | -375.5879 | Lower | -292.9265 | Lower |
| Halstead Effort | **49656.8507 ✅** | 51647.8490 | 74008.0457 | -1990.9983 | Lower | -24351.1950 | Lower | -22360.1966 | Lower |
| SLOC | **68 ✅** | 83 | 78 | -15 | Lower | -10 | Lower | +5 | Higher |
| PLOC | **47 ✅** | 56 | 64 | -9 | Lower | -17 | Lower | -8 | Lower |
| LLOC | **32 ✅** | 37 | 38 | -5 | Lower | -6 | Lower | -1 | Lower |
| Comments (CLOC) | 12 | **15 ✅** | 5 | -3 | Lower | +7 | Higher | +10 | Higher |
| Blank Lines | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03194

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.6461 ✅** | 73.4012 | 67.0950 | +1.2449 | Higher | +7.5511 | Higher | +6.3063 | Higher |
| MI (SEI) | **59.1397 ✅** | 58.6418 | 56.7689 | +0.4979 | Higher | +2.3708 | Higher | +1.8729 | Higher |
| Cyclomatic (sum) | 8 | 8 | 12 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | **864.6653 ✅** | 872.0704 | 1122.9866 | -7.4051 | Lower | -258.3213 | Lower | -250.9162 | Lower |
| Halstead Effort | 28121.2743 | **26756.7070 ✅** | 41236.0689 | +1364.5673 | Higher | -13114.7946 | Lower | -14479.3619 | Lower |
| SLOC | **39 ✅** | 42 | 54 | -3 | Lower | -15 | Lower | -12 | Lower |
| PLOC | **28 ✅** | 30 | 36 | -2 | Lower | -8 | Lower | -6 | Lower |
| LLOC | **25 ✅** | 26 | 28 | -1 | Lower | -3 | Lower | -2 | Lower |
| Comments (CLOC) | 5 | 6 | **13 ✅** | -1 | Lower | -8 | Lower | -7 | Lower |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p03206

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **86.5579 ✅** | 85.0521 | 85.7931 | +1.5057 | Higher | +0.7647 | Higher | -0.7410 | Lower |
| MI (SEI) | **75.8239 ✅** | 70.1788 | 74.8225 | +5.6451 | Higher | +1.0015 | Higher | -4.6436 | Lower |
| Cyclomatic (sum) | **6 ✅** | 11 | 7 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | **433.8179 ✅** | 530.3976 | 480.8030 | -96.5798 | Lower | -46.9851 | Lower | +49.5946 | Higher |
| Halstead Effort | **4635.0017 ✅** | 6199.0225 | 5494.8917 | -1564.0208 | Lower | -859.8900 | Lower | +704.1308 | Higher |
| SLOC | 24 | **23 ✅** | 24 | +1 | Higher | 0 | Same | -1 | Lower |
| PLOC | 18 | 18 | 18 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 11 | **5 ✅** | 11 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 3 | 2 | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03209

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.4801 ✅** | 80.6180 | 80.8211 | +0.8621 | Higher | +0.6590 | Higher | -0.2031 | Lower |
| MI (SEI) | **62.9237 ✅** | 61.3327 | 61.6258 | +1.5909 | Higher | +1.2979 | Higher | -0.2931 | Lower |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | **798.0630 ✅** | 841.0710 | 808.8476 | -43.0081 | Lower | -10.7846 | Lower | +32.2234 | Higher |
| Halstead Effort | **18863.3068 ✅** | 19746.8852 | 19118.2163 | -883.5784 | Lower | -254.9096 | Lower | +628.6689 | Higher |
| SLOC | **27 ✅** | 28 | 28 | -1 | Lower | -1 | Lower | 0 | Same |
| PLOC | 21 | 22 | 21 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p03213

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.5034 ✅** | 46.4927 | 66.1190 | +20.0107 | Higher | +0.3844 | Higher | -19.6262 | Lower |
| MI (SEI) | **49.7749 ✅** | 24.2071 | 44.8869 | +25.5679 | Higher | +4.8880 | Higher | -20.6798 | Lower |
| Cyclomatic (sum) | 12 | 29 | **9 ✅** | -17 | Lower | +3 | Higher | +20 | Higher |
| Cognitive (sum) | 25 | 43 | **13 ✅** | -18 | Lower | +12 | Higher | +30 | Higher |
| Halstead Volume | **1333.7400 ✅** | 2502.4097 | 1381.3735 | -1168.6697 | Lower | -47.6336 | Lower | +1121.0362 | Higher |
| Halstead Effort | 56017.0789 | 150053.0308 | **55686.6210 ✅** | -94035.9519 | Lower | +330.4579 | Higher | +94366.4098 | Higher |
| SLOC | **53 ✅** | 117 | 56 | -64 | Lower | -3 | Lower | +61 | Higher |
| PLOC | **39 ✅** | 84 | 43 | -45 | Lower | -4 | Lower | +41 | Higher |
| LLOC | **27 ✅** | 71 | 29 | -44 | Lower | -2 | Lower | +42 | Higher |
| Comments (CLOC) | 8 | **20 ✅** | 6 | -12 | Lower | +2 | Higher | +14 | Higher |
| Blank Lines | **6 ✅** | 13 | 7 | -7 | Lower | -1 | Lower | +6 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **0 ✅** | 6 | 1 | -6 | Lower | -1 | Lower | +5 | Higher |

## p03216

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.5295 ✅** | 58.0047 | 57.8581 | +1.5248 | Higher | +1.6714 | Higher | +0.1466 | Higher |
| MI (SEI) | **35.1370 ✅** | 28.7240 | 26.7986 | +6.4130 | Higher | +8.3384 | Higher | +1.9253 | Higher |
| Cyclomatic (sum) | 17 | 16 | 16 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 15 | **14 ✅** | 31 | +1 | Higher | -16 | Lower | -17 | Lower |
| Halstead Volume | **1573.7932 ✅** | 1718.7449 | 1916.8842 | -144.9517 | Lower | -343.0910 | Lower | -198.1393 | Lower |
| Halstead Effort | **47861.8280 ✅** | 48749.8547 | 62184.6357 | -888.0267 | Lower | -14322.8077 | Lower | -13434.7810 | Lower |
| SLOC | **72 ✅** | 78 | 76 | -6 | Lower | -4 | Lower | +2 | Higher |
| PLOC | **55 ✅** | 62 | 61 | -7 | Lower | -6 | Lower | +1 | Higher |
| LLOC | **41 ✅** | 45 | 54 | -4 | Lower | -13 | Lower | -9 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 4 | +2 | Higher | +3 | Higher | +1 | Higher |
| Blank Lines | **10 ✅** | 11 | 12 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03218

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.4107 ✅** | 69.4399 | 62.5171 | +0.9708 | Higher | +7.8935 | Higher | +6.9227 | Higher |
| MI (SEI) | **58.0535 ✅** | 56.1055 | 47.5483 | +1.9480 | Higher | +10.5052 | Higher | +8.5572 | Higher |
| Cyclomatic (sum) | 8 | 8 | 13 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 5 | **4 ✅** | 14 | +1 | Higher | -9 | Lower | -10 | Lower |
| Halstead Volume | **1091.7827 ✅** | 1155.6636 | 1386.7289 | -63.8809 | Lower | -294.9462 | Lower | -231.0653 | Lower |
| Halstead Effort | **24565.1113 ✅** | 25774.8009 | 35841.6096 | -1209.6896 | Lower | -11276.4983 | Lower | -10066.8086 | Lower |
| SLOC | **47 ✅** | 49 | 66 | -2 | Lower | -19 | Lower | -17 | Lower |
| PLOC | **31 ✅** | 32 | 44 | -1 | Lower | -13 | Lower | -12 | Lower |
| LLOC | **17 ✅** | 22 | 20 | -5 | Lower | -3 | Lower | +2 | Higher |
| Comments (CLOC) | 9 | 9 | **13 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03268

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.4410 ✅** | 82.9485 | 81.7122 | +1.4925 | Higher | +2.7288 | Higher | +1.2364 | Higher |
| MI (SEI) | 71.9905 | 68.9616 | **72.4810 ✅** | +3.0289 | Higher | -0.4905 | Lower | -3.5194 | Lower |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **655.3899 ✅** | 687.1023 | 697.5825 | -31.7124 | Lower | -42.1926 | Lower | -10.4802 | Lower |
| Halstead Effort | 11797.0178 | **11149.7962 ✅** | 12923.6335 | +647.2215 | Higher | -1126.6158 | Lower | -1773.8373 | Lower |
| SLOC | **25 ✅** | 27 | 29 | -2 | Lower | -4 | Lower | -2 | Lower |
| PLOC | 18 | 18 | 19 | 0 | Same | -1 | Lower | -1 | Lower |
| LLOC | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 3 | 3 | **5 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | **4 ✅** | 6 | 5 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.8133 ✅** | 37.9669 | 51.5092 | +15.8464 | Higher | +2.3041 | Higher | -13.5423 | Lower |
| MI (SEI) | **28.8598 ✅** | 14.8950 | 20.6569 | +13.9648 | Higher | +8.2029 | Higher | -5.7619 | Lower |
| Cyclomatic (sum) | **15 ✅** | 27 | 18 | -12 | Lower | -3 | Lower | +9 | Higher |
| Cognitive (sum) | **11 ✅** | 57 | 16 | -46 | Lower | -5 | Lower | +41 | Higher |
| Halstead Volume | **2325.4108 ✅** | 4398.7550 | 2781.8351 | -2073.3441 | Lower | -456.4243 | Lower | +1616.9198 | Higher |
| Halstead Effort | **105806.1916 ✅** | 283435.9037 | 167439.9798 | -177629.7120 | Lower | -61633.7882 | Lower | +115995.9238 | Higher |
| SLOC | **93 ✅** | 170 | 97 | -77 | Lower | -4 | Lower | +73 | Higher |
| PLOC | **66 ✅** | 113 | 74 | -47 | Lower | -8 | Lower | +39 | Higher |
| LLOC | **44 ✅** | 78 | 57 | -34 | Lower | -13 | Lower | +21 | Higher |
| Comments (CLOC) | 11 | **37 ✅** | 7 | -26 | Lower | +4 | Higher | +30 | Higher |
| Blank Lines | 17 | 21 | 17 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |

## p03278

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.4985 | **60.8321 ✅** | 36.0993 | -23.3335 | Lower | +1.3992 | Higher | +24.7328 | Higher |
| MI (SEI) | 4.6740 | **49.4261 ✅** | 0.5061 | -44.7521 | Lower | +4.1679 | Higher | +48.9200 | Higher |
| Cyclomatic (sum) | 27 | **11 ✅** | 24 | +16 | Higher | +3 | Higher | -13 | Lower |
| Cognitive (sum) | 56 | **8 ✅** | 31 | +48 | Higher | +25 | Higher | -23 | Lower |
| Halstead Volume | 5813.9203 | **1530.2285 ✅** | 7603.6775 | +4283.6919 | Higher | -1789.7572 | Lower | -6073.4491 | Lower |
| Halstead Effort | 542553.1373 | **54188.0899 ✅** | 903316.8886 | +488365.0474 | Higher | -360763.7513 | Lower | -849128.7987 | Lower |
| SLOC | 160 | **73 ✅** | 167 | +87 | Higher | -7 | Lower | -94 | Lower |
| PLOC | 125 | **44 ✅** | 129 | +81 | Higher | -4 | Lower | -85 | Lower |
| LLOC | 120 | **29 ✅** | 112 | +91 | Higher | +8 | Higher | -83 | Lower |
| Comments (CLOC) | 16 | **20 ✅** | 14 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 19 | **9 ✅** | 24 | +10 | Higher | -5 | Lower | -15 | Lower |
| Functions | **1 ✅** | 4 | 4 | -3 | Lower | -3 | Lower | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | **0 ✅** | 4 | 2 | -4 | Lower | -2 | Lower | +2 | Higher |

## p03294

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.8714 | 84.5220 | **85.2326 ✅** | -1.6506 | Lower | -2.3612 | Lower | -0.7106 | Lower |
| MI (SEI) | 74.6070 | 67.8604 | **82.7443 ✅** | +6.7466 | Higher | -8.1373 | Lower | -14.8839 | Lower |
| Cyclomatic (sum) | **3 ✅** | 4 | 5 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 622.6656 | 617.3437 | **585.0582 ✅** | +5.3219 | Higher | +37.6074 | Higher | +32.2854 | Higher |
| Halstead Effort | 8915.4391 | 10053.8824 | **8313.9853 ✅** | -1138.4434 | Lower | +601.4538 | Higher | +1739.8971 | Higher |
| SLOC | 28 | 25 | **24 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| PLOC | 16 | 17 | **13 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| LLOC | 13 | 13 | **10 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Comments (CLOC) | 5 | 2 | **6 ✅** | +3 | Higher | -1 | Lower | -4 | Lower |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03296

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.4233 ✅** | 72.0166 | 75.4892 | +4.4066 | Higher | +0.9341 | Higher | -3.4726 | Lower |
| MI (SEI) | 66.8286 | 57.1430 | **70.3672 ✅** | +9.6856 | Higher | -3.5386 | Lower | -13.2243 | Lower |
| Cyclomatic (sum) | 7 | 10 | **5 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Cognitive (sum) | 6 | 13 | **4 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| Halstead Volume | **824.0030 ✅** | 1122.9866 | 839.5814 | -298.9836 | Lower | -15.5784 | Lower | +283.4052 | Higher |
| Halstead Effort | 23699.0429 | 34313.4802 | **17631.2098 ✅** | -10614.4373 | Lower | +6067.8331 | Higher | +16682.2704 | Higher |
| SLOC | **36 ✅** | 41 | 39 | -5 | Lower | -3 | Lower | +2 | Higher |
| PLOC | 25 | 30 | **23 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| LLOC | **19 ✅** | 23 | 21 | -4 | Lower | -2 | Lower | +2 | Higher |
| Comments (CLOC) | 7 | 6 | **11 ✅** | +1 | Higher | -4 | Lower | -5 | Lower |
| Blank Lines | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03311

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.0722 | **79.6401 ✅** | 54.6898 | -0.5680 | Lower | +24.3823 | Higher | +24.9503 | Higher |
| MI (SEI) | **73.3298 ✅** | 70.6843 | 33.7958 | +2.6455 | Higher | +39.5340 | Higher | +36.8884 | Higher |
| Cyclomatic (sum) | 4 | 4 | 16 | 0 | Same | -12 | Lower | -12 | Lower |
| Cognitive (sum) | 2 | 2 | 25 | 0 | Same | -23 | Lower | -23 | Lower |
| Halstead Volume | 741.4002 | **731.5638 ✅** | 2232.8236 | +9.8363 | Higher | -1491.4234 | Lower | -1501.2598 | Lower |
| Halstead Effort | 13623.2281 | **12802.3671 ✅** | 116266.3155 | +820.8609 | Higher | -102643.0874 | Lower | -103463.9483 | Lower |
| SLOC | 33 | **32 ✅** | 88 | +1 | Higher | -55 | Lower | -56 | Lower |
| PLOC | 19 | 19 | 63 | 0 | Same | -44 | Lower | -44 | Lower |
| LLOC | 15 | 15 | 56 | 0 | Same | -41 | Lower | -41 | Lower |
| Comments (CLOC) | 8 | 6 | **14 ✅** | +2 | Higher | -6 | Lower | -8 | Lower |
| Blank Lines | 7 | 7 | 11 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.4807 ✅** | 76.7791 | 75.4018 | +3.7016 | Higher | +5.0789 | Higher | +1.3773 | Higher |
| MI (SEI) | 70.8061 | **71.9128 ✅** | 66.7765 | -1.1067 | Lower | +4.0297 | Higher | +5.1364 | Higher |
| Cyclomatic (sum) | 4 | 4 | 7 | 0 | Same | -3 | Lower | -3 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 6 | -1 | Lower | -4 | Lower | -3 | Lower |
| Halstead Volume | **845.7454 ✅** | 878.6965 | 920.8124 | -32.9511 | Lower | -75.0670 | Lower | -42.1159 | Lower |
| Halstead Effort | **19010.8852 ✅** | 19221.4858 | 24459.0798 | -210.6006 | Lower | -5448.1945 | Lower | -5237.5939 | Lower |
| SLOC | **29 ✅** | 36 | 37 | -7 | Lower | -8 | Lower | -1 | Lower |
| PLOC | **19 ✅** | 22 | 24 | -3 | Lower | -5 | Lower | -2 | Lower |
| LLOC | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 8 | -5 | Lower | -3 | Lower | +2 | Higher |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03318

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.6092 ✅** | 64.6557 | 65.8711 | +4.9535 | Higher | +3.7382 | Higher | -1.2153 | Lower |
| MI (SEI) | **53.5253 ✅** | 47.6340 | 51.2544 | +5.8912 | Higher | +2.2708 | Higher | -3.6204 | Lower |
| Cyclomatic (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **1141.2328 ✅** | 1332.9334 | 1231.4296 | -191.7007 | Lower | -90.1969 | Lower | +101.5038 | Higher |
| Halstead Effort | **40730.2034 ✅** | 49076.1851 | 40849.4929 | -8345.9817 | Lower | -119.2895 | Lower | +8226.6922 | Higher |
| SLOC | **48 ✅** | 62 | 59 | -14 | Lower | -11 | Lower | +3 | Higher |
| PLOC | **35 ✅** | 41 | 38 | -6 | Lower | -3 | Lower | +3 | Higher |
| LLOC | **29 ✅** | 32 | 30 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 7 | 10 | **11 ✅** | -3 | Lower | -4 | Lower | -1 | Lower |
| Blank Lines | **7 ✅** | 12 | 11 | -5 | Lower | -4 | Lower | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03361

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.0024 | 60.7308 | **65.8432 ✅** | +4.2716 | Higher | -0.8408 | Lower | -5.1125 | Lower |
| MI (SEI) | 50.5236 | 40.2752 | **52.1314 ✅** | +10.2485 | Higher | -1.6077 | Lower | -11.8562 | Lower |
| Cyclomatic (sum) | 15 | 21 | **14 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Cognitive (sum) | 24 | 39 | **22 ✅** | -15 | Lower | +2 | Higher | +17 | Higher |
| Halstead Volume | **1470.6608 ✅** | 1586.5461 | 1470.8229 | -115.8853 | Lower | -0.1621 | Lower | +115.7232 | Higher |
| Halstead Effort | 45884.6158 | 68528.5550 | **42932.1267 ✅** | -22643.9393 | Lower | +2952.4891 | Higher | +25596.4283 | Higher |
| SLOC | 54 | 63 | **52 ✅** | -9 | Lower | +2 | Higher | +11 | Higher |
| PLOC | 36 | 49 | **35 ✅** | -13 | Lower | +1 | Higher | +14 | Higher |
| LLOC | 23 | 33 | 23 | -10 | Lower | 0 | Same | +10 | Higher |
| Comments (CLOC) | 10 | 8 | 10 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p03365

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.1782 ✅** | 56.3289 | 57.2372 | +1.8493 | Higher | +0.9411 | Higher | -0.9082 | Lower |
| MI (SEI) | 33.0831 | **35.5572 ✅** | 35.1590 | -2.4741 | Lower | -2.0759 | Lower | +0.3982 | Higher |
| Cyclomatic (sum) | 13 | **12 ✅** | 13 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 9 | 20 | 9 | -11 | Lower | 0 | Same | +11 | Higher |
| Halstead Volume | 2659.2377 | **2333.1361 ✅** | 2680.1940 | +326.1015 | Higher | -20.9564 | Lower | -347.0579 | Lower |
| Halstead Effort | 147958.7474 | **127217.3174 ✅** | 154266.9822 | +20741.4300 | Higher | -6308.2348 | Lower | -27049.6648 | Lower |
| SLOC | **70 ✅** | 83 | 74 | -13 | Lower | -4 | Lower | +9 | Higher |
| PLOC | **53 ✅** | 59 | 54 | -6 | Lower | -1 | Lower | +5 | Higher |
| LLOC | 45 | **44 ✅** | 47 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 10 | -6 | Lower | -3 | Lower | +3 | Higher |
| Blank Lines | **10 ✅** | 11 | 11 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p03369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.2073 ✅** | 83.1607 | 86.9210 | +7.0466 | Higher | +3.2863 | Higher | -3.7603 | Lower |
| MI (SEI) | **83.0801 ✅** | 72.4162 | 80.2091 | +10.6639 | Higher | +2.8711 | Higher | -7.7928 | Lower |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **414.5863 ✅** | 539.1110 | 504.6426 | -124.5248 | Lower | -90.0563 | Lower | +34.4684 | Higher |
| Halstead Effort | **4698.6444 ✅** | 8779.8081 | 6791.6486 | -4081.1637 | Lower | -2093.0042 | Lower | +1988.1595 | Higher |
| SLOC | **20 ✅** | 28 | 23 | -8 | Lower | -3 | Lower | +5 | Higher |
| PLOC | **14 ✅** | 19 | 15 | -5 | Lower | -1 | Lower | +4 | Higher |
| LLOC | **10 ✅** | 16 | 11 | -6 | Lower | -1 | Lower | +5 | Higher |
| Comments (CLOC) | 3 | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03406

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.4677 | 42.4354 | **46.7943 ✅** | +4.0324 | Higher | -0.3266 | Lower | -4.3590 | Lower |
| MI (SEI) | **15.6101 ✅** | 9.1736 | 12.6503 | +6.4365 | Higher | +2.9598 | Higher | -3.4768 | Lower |
| Cyclomatic (sum) | 20 | 27 | 20 | -7 | Lower | 0 | Same | +7 | Higher |
| Cognitive (sum) | 32 | 40 | 32 | -8 | Lower | 0 | Same | +8 | Higher |
| Halstead Volume | 4172.4725 | 4400.8539 | **3918.4906 ✅** | -228.3813 | Lower | +253.9820 | Higher | +482.3633 | Higher |
| Halstead Effort | 318087.1670 | 384634.6291 | **277308.5641 ✅** | -66547.4621 | Lower | +40778.6029 | Higher | +107326.0650 | Higher |
| SLOC | 113 | 129 | 113 | -16 | Lower | 0 | Same | +16 | Higher |
| PLOC | **91 ✅** | 106 | 93 | -15 | Lower | -2 | Lower | +13 | Higher |
| LLOC | 77 | 89 | **73 ✅** | -12 | Lower | +4 | Higher | +16 | Higher |
| Comments (CLOC) | 10 | 10 | 7 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | 13 | 14 | 13 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03408

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.0630 ✅** | 64.1758 | 69.0366 | +6.8872 | Higher | +2.0264 | Higher | -4.8608 | Lower |
| MI (SEI) | **47.5185 ✅** | 47.2471 | 44.1937 | +0.2713 | Higher | +3.3248 | Higher | +3.0534 | Higher |
| Cyclomatic (sum) | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **15 ✅** | 18 | 16 | -3 | Lower | -1 | Lower | +2 | Higher |
| Halstead Volume | **1082.6181 ✅** | 1280.1538 | 1191.4075 | -197.5357 | Lower | -108.7893 | Lower | +88.7464 | Higher |
| Halstead Effort | **30464.8746 ✅** | 36964.4423 | 35988.7217 | -6499.5677 | Lower | -5523.8472 | Lower | +975.7205 | Higher |
| SLOC | **44 ✅** | 62 | 47 | -18 | Lower | -3 | Lower | +15 | Higher |
| PLOC | **34 ✅** | 43 | 37 | -9 | Lower | -3 | Lower | +6 | Higher |
| LLOC | 26 | 35 | 26 | -9 | Lower | 0 | Same | +9 | Higher |
| Comments (CLOC) | 3 | **10 ✅** | 3 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 7 | 10 | 7 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03414

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 23.3631 | **24.3022 ✅** | 18.9600 | -0.9392 | Lower | +4.4031 | Higher | +5.3423 | Higher |
| MI (SEI) | **-8.9513 ✅** | -17.1179 | -24.8404 | +8.1666 | Higher | +15.8891 | Higher | +7.7225 | Higher |
| Cyclomatic (sum) | 49 | **47 ✅** | 53 | +2 | Higher | -4 | Lower | -6 | Lower |
| Cognitive (sum) | 116 | 107 | **76 ✅** | +9 | Higher | +40 | Higher | +31 | Higher |
| Halstead Volume | **7607.6119 ✅** | 8155.6621 | 9471.7611 | -548.0502 | Lower | -1864.1492 | Lower | -1316.0990 | Lower |
| Halstead Effort | 718496.6827 | **637999.8968 ✅** | 1132003.4501 | +80496.7859 | Higher | -413506.7674 | Lower | -494003.5533 | Lower |
| SLOC | 257 | **244 ✅** | 297 | +13 | Higher | -40 | Lower | -53 | Lower |
| PLOC | **198 ✅** | 202 | 249 | -4 | Lower | -51 | Lower | -47 | Lower |
| LLOC | 172 | **143 ✅** | 210 | +29 | Higher | -38 | Lower | -67 | Lower |
| Comments (CLOC) | **38 ✅** | 15 | 17 | +23 | Higher | +21 | Higher | -2 | Lower |
| Blank Lines | **24 ✅** | 29 | 31 | -5 | Lower | -7 | Lower | -2 | Lower |
| Functions | **2 ✅** | 3 | 7 | -1 | Lower | -5 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 3 | 8 | -2 | Lower | -7 | Lower | -5 | Lower |

## p03434

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.7817 ✅** | 74.3130 | 53.5419 | +2.4687 | Higher | +23.2399 | Higher | +20.7712 | Higher |
| MI (SEI) | 65.1372 | **65.2071 ✅** | 17.0253 | -0.0699 | Lower | +48.1119 | Higher | +48.1818 | Higher |
| Cyclomatic (sum) | 5 | 5 | 15 | 0 | Same | -10 | Lower | -10 | Lower |
| Cognitive (sum) | 5 | 5 | 16 | 0 | Same | -11 | Lower | -11 | Lower |
| Halstead Volume | **840.2535 ✅** | 900.8062 | 2713.5311 | -60.5527 | Lower | -1873.2776 | Lower | -1812.7249 | Lower |
| Halstead Effort | **17277.7131 ✅** | 17457.6245 | 172632.2656 | -179.9114 | Lower | -155354.5525 | Lower | -155174.6411 | Lower |
| SLOC | **36 ✅** | 41 | 90 | -5 | Lower | -54 | Lower | -49 | Lower |
| PLOC | **24 ✅** | 25 | 72 | -1 | Lower | -48 | Lower | -47 | Lower |
| LLOC | **19 ✅** | 20 | 61 | -1 | Lower | -42 | Lower | -41 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 3 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | **6 ✅** | 7 | 15 | -1 | Lower | -9 | Lower | -8 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.5328 | **75.8317 ✅** | 66.5723 | -0.2989 | Lower | +8.9605 | Higher | +9.2594 | Higher |
| MI (SEI) | 56.0946 | **64.5345 ✅** | 44.9799 | -8.4399 | Lower | +11.1147 | Higher | +19.5546 | Higher |
| Cyclomatic (sum) | 9 | 9 | 13 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 10 | 10 | 21 | 0 | Same | -11 | Lower | -11 | Lower |
| Halstead Volume | 977.2383 | **922.6439 ✅** | 1419.5554 | +54.5943 | Higher | -442.3172 | Lower | -496.9115 | Lower |
| Halstead Effort | 34691.9582 | **30447.2501 ✅** | 64987.2473 | +4244.7081 | Higher | -30295.2890 | Lower | -34539.9971 | Lower |
| SLOC | 35 | 35 | 51 | 0 | Same | -16 | Lower | -16 | Lower |
| PLOC | 26 | **25 ✅** | 39 | +1 | Higher | -13 | Lower | -14 | Lower |
| LLOC | 20 | **18 ✅** | 29 | +2 | Higher | -9 | Lower | -11 | Lower |
| Comments (CLOC) | 3 | **6 ✅** | 5 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | 6 | **5 ✅** | 7 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03448

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.2842 ✅** | 76.3076 | 77.8892 | +2.9766 | Higher | +1.3950 | Higher | -1.5816 | Lower |
| MI (SEI) | **65.7033 ✅** | 57.5096 | 59.7954 | +8.1937 | Higher | +5.9079 | Higher | -2.2858 | Lower |
| Cyclomatic (sum) | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **791.6176 ✅** | 921.5188 | 851.9862 | -129.9012 | Lower | -60.3686 | Lower | +69.5327 | Higher |
| Halstead Effort | 13235.8460 | 20273.4139 | **12321.0305 ✅** | -7037.5679 | Lower | +914.8155 | Higher | +7952.3834 | Higher |
| SLOC | **31 ✅** | 34 | 33 | -3 | Lower | -2 | Lower | +1 | Higher |
| PLOC | **21 ✅** | 25 | 25 | -4 | Lower | -4 | Lower | 0 | Same |
| LLOC | **15 ✅** | 18 | 16 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | **4 ✅** | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03454

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **49.5021 ✅** | 47.3040 | 48.9807 | +2.1981 | Higher | +0.5214 | Higher | -1.6767 | Lower |
| MI (SEI) | 21.9052 | **22.8410 ✅** | 12.9039 | -0.9358 | Lower | +9.0014 | Higher | +9.9371 | Higher |
| Cyclomatic (sum) | 24 | 24 | 25 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 42 | 49 | **40 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| Halstead Volume | **3039.6635 ✅** | 3236.8272 | 3214.8615 | -197.1637 | Lower | -175.1980 | Lower | +21.9657 | Higher |
| Halstead Effort | 244507.5681 | **222262.1333 ✅** | 247142.4775 | +22245.4348 | Higher | -2634.9094 | Lower | -24880.3441 | Lower |
| SLOC | 98 | 110 | 98 | -12 | Lower | 0 | Same | +12 | Higher |
| PLOC | **75 ✅** | 80 | 79 | -5 | Lower | -4 | Lower | +1 | Higher |
| LLOC | 60 | 64 | 60 | -4 | Lower | 0 | Same | +4 | Higher |
| Comments (CLOC) | 10 | **16 ✅** | 4 | -6 | Lower | +6 | Higher | +12 | Higher |
| Blank Lines | **13 ✅** | 14 | 15 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |

## p03463

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.5260 ✅** | 85.4551 | 86.8012 | +2.0709 | Higher | +0.7247 | Higher | -1.3462 | Lower |
| MI (SEI) | 77.9469 | 79.8257 | **79.9345 ✅** | -1.8788 | Lower | -1.9876 | Lower | -0.1088 | Lower |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 539.2711 | 539.2711 | 539.7502 | 0 | Same | -0.4791 | Lower | -0.4791 | Lower |
| Halstead Effort | 6173.2345 | 6173.2345 | **5766.8045 ✅** | 0 | Same | +406.4301 | Higher | +406.4301 | Higher |
| SLOC | **22 ✅** | 25 | 23 | -3 | Lower | -1 | Lower | +2 | Higher |
| PLOC | 15 | 15 | 15 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 3 | **5 ✅** | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03473

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **106.0590 ✅** | 96.1398 | 97.2452 | +9.9192 | Higher | +8.8138 | Higher | -1.1054 | Lower |
| MI (SEI) | 77.5136 | **98.1746 ✅** | 96.7335 | -20.6610 | Lower | -19.2199 | Lower | +1.4411 | Higher |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **258.5242 ✅** | 290.0481 | 286.7268 | -31.5240 | Lower | -28.2026 | Lower | +3.3213 | Higher |
| Halstead Effort | **2197.4556 ✅** | 2558.6389 | 2334.7756 | -361.1834 | Lower | -137.3200 | Lower | +223.8634 | Higher |
| SLOC | **9 ✅** | 16 | 15 | -7 | Lower | -6 | Lower | +1 | Higher |
| PLOC | **7 ✅** | 8 | 8 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 0 | **4 ✅** | 3 | -4 | Lower | -3 | Lower | +1 | Higher |
| Blank Lines | **2 ✅** | 4 | 4 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03478

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.4384 | **78.1489 ✅** | 69.3049 | -0.7104 | Lower | +8.1336 | Higher | +8.8440 | Higher |
| MI (SEI) | **59.7029 ✅** | 56.6413 | 49.2008 | +3.0616 | Higher | +10.5021 | Higher | +7.4405 | Higher |
| Cyclomatic (sum) | **5 ✅** | 7 | 7 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | 1179.9910 | **853.4359 ✅** | 1411.5594 | +326.5551 | Higher | -231.5684 | Lower | -558.1235 | Lower |
| Halstead Effort | 34158.7056 | **17037.1092 ✅** | 35047.0030 | +17121.5964 | Higher | -888.2974 | Lower | -18009.8938 | Lower |
| SLOC | **31 ✅** | 32 | 47 | -1 | Lower | -16 | Lower | -15 | Lower |
| PLOC | **22 ✅** | 25 | 36 | -3 | Lower | -14 | Lower | -11 | Lower |
| LLOC | 18 | 19 | 18 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 3 | 2 | **5 ✅** | +1 | Higher | -2 | Lower | -3 | Lower |
| Blank Lines | 6 | **5 ✅** | 7 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.4343 ✅** | 69.7921 | 54.3204 | +5.6422 | Higher | +21.1139 | Higher | +15.4717 | Higher |
| MI (SEI) | **68.2298 ✅** | 56.5119 | 30.0996 | +11.7179 | Higher | +38.1303 | Higher | +26.4123 | Higher |
| Cyclomatic (sum) | **5 ✅** | 7 | 15 | -2 | Lower | -10 | Lower | -8 | Lower |
| Cognitive (sum) | **2 ✅** | 5 | 15 | -3 | Lower | -13 | Lower | -10 | Lower |
| Halstead Volume | **999.7083 ✅** | 1128.8126 | 2418.9757 | -129.1044 | Lower | -1419.2674 | Lower | -1290.1630 | Lower |
| Halstead Effort | **21570.6288 ✅** | 29599.9760 | 118184.2406 | -8029.3472 | Lower | -96613.6118 | Lower | -88584.2646 | Lower |
| SLOC | **37 ✅** | 49 | 89 | -12 | Lower | -52 | Lower | -40 | Lower |
| PLOC | **22 ✅** | 30 | 62 | -8 | Lower | -40 | Lower | -32 | Lower |
| LLOC | **19 ✅** | 22 | 52 | -3 | Lower | -33 | Lower | -30 | Lower |
| Comments (CLOC) | 9 | 9 | **11 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | **7 ✅** | 10 | 16 | -3 | Lower | -9 | Lower | -6 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03544

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.2254 ✅** | 87.2838 | 31.7900 | +2.9416 | Higher | +58.4354 | Higher | +55.4938 | Higher |
| MI (SEI) | **78.8587 ✅** | 73.1452 | -6.6209 | +5.7135 | Higher | +85.4796 | Higher | +79.7662 | Higher |
| Cyclomatic (sum) | **3 ✅** | 4 | 45 | -1 | Lower | -42 | Lower | -41 | Lower |
| Cognitive (sum) | **1 ✅** | 3 | 39 | -2 | Lower | -38 | Lower | -36 | Lower |
| Halstead Volume | **506.6527 ✅** | 540.5365 | 5353.2020 | -33.8839 | Lower | -4846.5494 | Lower | -4812.6655 | Lower |
| Halstead Effort | **8386.4347 ✅** | 10540.4624 | 528299.6104 | -2154.0278 | Lower | -519913.1757 | Lower | -517759.1480 | Lower |
| SLOC | **19 ✅** | 22 | 181 | -3 | Lower | -162 | Lower | -159 | Lower |
| PLOC | **13 ✅** | 15 | 155 | -2 | Lower | -142 | Lower | -140 | Lower |
| LLOC | **12 ✅** | 13 | 112 | -1 | Lower | -100 | Lower | -99 | Lower |
| Comments (CLOC) | 2 | 2 | **11 ✅** | 0 | Same | -9 | Lower | -9 | Lower |
| Blank Lines | **4 ✅** | 5 | 18 | -1 | Lower | -14 | Lower | -13 | Lower |
| Functions | 1 | 1 | 12 | 0 | Same | -11 | Lower | -11 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 12 | 0 | Same | -12 | Lower | -12 | Lower |

## p03562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.8451 | **40.6452 ✅** | 30.1318 | -5.8001 | Lower | +4.7132 | Higher | +10.5133 | Higher |
| MI (SEI) | 1.1768 | **7.5528 ✅** | -6.9222 | -6.3760 | Lower | +8.0990 | Higher | +14.4750 | Higher |
| Cyclomatic (sum) | 36 | **28 ✅** | 43 | +8 | Higher | -7 | Lower | -15 | Lower |
| Cognitive (sum) | 57 | **36 ✅** | 85 | +21 | Higher | -28 | Lower | -49 | Lower |
| Halstead Volume | 5484.7336 | **4404.8767 ✅** | 6586.6749 | +1079.8570 | Higher | -1101.9412 | Lower | -2181.7982 | Lower |
| Halstead Effort | 454734.2792 | **257093.5844 ✅** | 601392.0518 | +197640.6948 | Higher | -146657.7725 | Lower | -344298.4674 | Lower |
| SLOC | 169 | **142 ✅** | 193 | +27 | Higher | -24 | Lower | -51 | Lower |
| PLOC | 132 | **109 ✅** | 151 | +23 | Higher | -19 | Lower | -42 | Lower |
| LLOC | 105 | **78 ✅** | 117 | +27 | Higher | -12 | Lower | -39 | Lower |
| Comments (CLOC) | **16 ✅** | 12 | 15 | +4 | Higher | +1 | Higher | -3 | Lower |
| Blank Lines | 22 | 22 | 29 | 0 | Same | -7 | Lower | -7 | Lower |
| Functions | 5 | 6 | **4 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 6 | 2 | -5 | Lower | -1 | Lower | +4 | Higher |

## p03568

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.3384 | **82.7483 ✅** | 80.8265 | -1.4099 | Lower | +0.5119 | Higher | +1.9218 | Higher |
| MI (SEI) | 71.7111 | **72.2674 ✅** | 68.2160 | -0.5563 | Lower | +3.4951 | Higher | +4.0515 | Higher |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **617.3437 ✅** | 653.6213 | 681.2068 | -36.2777 | Lower | -63.8631 | Lower | -27.5855 | Lower |
| Halstead Effort | **10612.4315 ✅** | 11289.8232 | 11704.3713 | -677.3918 | Lower | -1091.9398 | Lower | -414.5481 | Lower |
| SLOC | 30 | **27 ✅** | 30 | +3 | Higher | 0 | Same | -3 | Lower |
| PLOC | **19 ✅** | 20 | 20 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **15 ✅** | 16 | 16 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | **5 ✅** | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03592

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 84.4067 | 84.4067 | 77.6982 | 0 | Same | +6.7086 | Higher | +6.7086 | Higher |
| MI (SEI) | 62.1306 | 62.1306 | **65.0943 ✅** | 0 | Same | -2.9637 | Lower | -2.9637 | Lower |
| Cyclomatic (sum) | 5 | 5 | 7 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 685.7667 | 685.7667 | 770.5296 | 0 | Same | -84.7629 | Lower | -84.7629 | Lower |
| Halstead Effort | 12735.6665 | 12735.6665 | 13115.0558 | 0 | Same | -379.3893 | Lower | -379.3893 | Lower |
| SLOC | 24 | 24 | 34 | 0 | Same | -10 | Lower | -10 | Lower |
| PLOC | 19 | 19 | 25 | 0 | Same | -6 | Lower | -6 | Lower |
| LLOC | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 1 | 1 | **5 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p03608

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.2283 | 46.2512 | **52.6203 ✅** | +5.9771 | Higher | -0.3920 | Lower | -6.3691 | Lower |
| MI (SEI) | **23.6491 ✅** | 17.4595 | 18.1748 | +6.1895 | Higher | +5.4743 | Higher | -0.7152 | Lower |
| Cyclomatic (sum) | 18 | 24 | 18 | -6 | Lower | 0 | Same | +6 | Higher |
| Cognitive (sum) | 30 | 43 | 30 | -13 | Lower | 0 | Same | +13 | Higher |
| Halstead Volume | 2955.7087 | 3450.6858 | **2937.5755 ✅** | -494.9771 | Lower | +18.1332 | Higher | +513.1103 | Higher |
| Halstead Effort | 155895.6126 | 179029.6995 | **142682.2406 ✅** | -23134.0869 | Lower | +13213.3720 | Higher | +36347.4589 | Higher |
| SLOC | 91 | 115 | **89 ✅** | -24 | Lower | +2 | Higher | +26 | Higher |
| PLOC | 75 | 93 | 75 | -18 | Lower | 0 | Same | +18 | Higher |
| LLOC | 52 | 62 | 52 | -10 | Lower | 0 | Same | +10 | Higher |
| Comments (CLOC) | 8 | **12 ✅** | 4 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03626

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.7154 ✅** | 67.9903 | 67.3364 | +0.7251 | Higher | +1.3790 | Higher | +0.6538 | Higher |
| MI (SEI) | **54.6417 ✅** | 53.1001 | 47.6791 | +1.5416 | Higher | +6.9625 | Higher | +5.4209 | Higher |
| Cyclomatic (sum) | 9 | 9 | 11 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 15 | 15 | 16 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **1122.0269 ✅** | 1144.2453 | 1260.3204 | -22.2184 | Lower | -138.2934 | Lower | -116.0751 | Lower |
| Halstead Effort | **35545.8137 ✅** | 37256.6271 | 43459.3229 | -1710.8134 | Lower | -7913.5091 | Lower | -6202.6958 | Lower |
| SLOC | **51 ✅** | 53 | 52 | -2 | Lower | -1 | Lower | +1 | Higher |
| PLOC | 36 | 36 | 39 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 33 | 33 | 34 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 9 | 9 | 6 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | 8 | 9 | **7 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03632

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 84.9727 | 84.9727 | 65.4063 | 0 | Same | +19.5663 | Higher | +19.5663 | Higher |
| MI (SEI) | **79.6761 ✅** | 76.7506 | 45.7160 | +2.9255 | Higher | +33.9601 | Higher | +31.0347 | Higher |
| Cyclomatic (sum) | 3 | 3 | 10 | 0 | Same | -7 | Lower | -7 | Lower |
| Cognitive (sum) | 2 | 2 | 11 | 0 | Same | -9 | Lower | -9 | Lower |
| Halstead Volume | 671.9361 | 671.9361 | 1515.7196 | 0 | Same | -843.7836 | Lower | -843.7836 | Lower |
| Halstead Effort | 9071.1370 | 9071.1370 | 46482.0688 | 0 | Same | -37410.9318 | Lower | -37410.9318 | Lower |
| SLOC | 24 | 24 | 56 | 0 | Same | -32 | Lower | -32 | Lower |
| PLOC | 14 | 14 | 40 | 0 | Same | -26 | Lower | -26 | Lower |
| LLOC | 11 | 11 | 33 | 0 | Same | -22 | Lower | -22 | Lower |
| Comments (CLOC) | 5 | 4 | **7 ✅** | +1 | Higher | -2 | Lower | -3 | Lower |
| Blank Lines | **5 ✅** | 6 | 9 | -1 | Lower | -4 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03646

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.6784 ✅** | 64.0304 | 70.4300 | +6.6481 | Higher | +0.2485 | Higher | -6.3996 | Lower |
| MI (SEI) | **51.9723 ✅** | 47.6573 | 51.6139 | +4.3150 | Higher | +0.3584 | Higher | -3.9566 | Lower |
| Cyclomatic (sum) | 10 | 14 | 10 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 17 | 29 | 17 | -12 | Lower | 0 | Same | +12 | Higher |
| Halstead Volume | **1086.8956 ✅** | 1334.6055 | 1140.0879 | -247.7098 | Lower | -53.1923 | Lower | +194.5175 | Higher |
| Halstead Effort | **29872.0991 ✅** | 45532.2898 | 31770.4507 | -15660.1907 | Lower | -1898.3516 | Lower | +13761.8391 | Higher |
| SLOC | 45 | 60 | 45 | -15 | Lower | 0 | Same | +15 | Higher |
| PLOC | 35 | 45 | 35 | -10 | Lower | 0 | Same | +10 | Higher |
| LLOC | 25 | 31 | 25 | -6 | Lower | 0 | Same | +6 | Higher |
| Comments (CLOC) | 5 | **10 ✅** | 5 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.4601 ✅** | 60.2060 | 60.7762 | +1.2541 | Higher | +0.6839 | Higher | -0.5702 | Lower |
| MI (SEI) | 46.2925 | 43.3834 | **49.9456 ✅** | +2.9090 | Higher | -3.6531 | Lower | -6.5621 | Lower |
| Cyclomatic (sum) | 20 | 19 | **15 ✅** | +1 | Higher | +5 | Higher | +4 | Higher |
| Cognitive (sum) | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1441.2813 | 1511.3591 | **1352.8382 ✅** | -70.0778 | Lower | +88.4431 | Higher | +158.5209 | Higher |
| Halstead Effort | 38914.5952 | 39230.5640 | **36075.6846 ✅** | -315.9687 | Lower | +2838.9106 | Higher | +3154.8794 | Higher |
| SLOC | **63 ✅** | 68 | 72 | -5 | Lower | -9 | Lower | -4 | Lower |
| PLOC | **44 ✅** | 50 | 45 | -6 | Lower | -1 | Lower | +5 | Higher |
| LLOC | **21 ✅** | 25 | 28 | -4 | Lower | -7 | Lower | -3 | Lower |
| Comments (CLOC) | 12 | 12 | **20 ✅** | 0 | Same | -8 | Lower | -8 | Lower |
| Blank Lines | **8 ✅** | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03701

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.7454 ✅** | 69.2486 | 70.9782 | +3.4968 | Higher | +1.7672 | Higher | -1.7296 | Lower |
| MI (SEI) | **55.1633 ✅** | 52.9563 | 49.8971 | +2.2071 | Higher | +5.2663 | Higher | +3.0592 | Higher |
| Cyclomatic (sum) | **7 ✅** | 11 | 9 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | 10 | 16 | 10 | -6 | Lower | 0 | Same | +6 | Higher |
| Halstead Volume | **960.9439 ✅** | 1049.9730 | 1072.4096 | -89.0291 | Lower | -111.4657 | Lower | -22.4366 | Lower |
| Halstead Effort | **24946.1029 ✅** | 31394.1912 | 31780.4454 | -6448.0883 | Lower | -6834.3425 | Lower | -386.2542 | Lower |
| SLOC | **43 ✅** | 49 | 45 | -6 | Lower | -2 | Lower | +4 | Higher |
| PLOC | **29 ✅** | 33 | 33 | -4 | Lower | -4 | Lower | 0 | Same |
| LLOC | 26 | **20 ✅** | 27 | +6 | Higher | -1 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | **7 ✅** | 4 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 9 | 9 | **8 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03709

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **36.4753 ✅** | 31.7387 | 31.9394 | +4.7366 | Higher | +4.5359 | Higher | -0.2007 | Lower |
| MI (SEI) | **10.7797 ✅** | -0.4582 | 0.7860 | +11.2379 | Higher | +9.9937 | Higher | -1.2442 | Lower |
| Cyclomatic (sum) | **26 ✅** | 38 | 33 | -12 | Lower | -7 | Lower | +5 | Higher |
| Cognitive (sum) | **48 ✅** | 49 | 54 | -1 | Lower | -6 | Lower | -5 | Lower |
| Halstead Volume | **6238.7177 ✅** | 7120.2668 | 7479.7888 | -881.5491 | Lower | -1241.0711 | Lower | -359.5220 | Lower |
| Halstead Effort | **511931.3491 ✅** | 752184.9886 | 805947.2441 | -240253.6395 | Lower | -294015.8950 | Lower | -53762.2555 | Lower |
| SLOC | **169 ✅** | 183 | 191 | -14 | Lower | -22 | Lower | -8 | Lower |
| PLOC | **120 ✅** | 147 | 140 | -27 | Lower | -20 | Lower | +7 | Higher |
| LLOC | **102 ✅** | 126 | 121 | -24 | Lower | -19 | Lower | +5 | Higher |
| Comments (CLOC) | **32 ✅** | 22 | 26 | +10 | Higher | +6 | Higher | -4 | Lower |
| Blank Lines | 23 | 23 | 25 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | **2 ✅** | 6 | 3 | -4 | Lower | -1 | Lower | +3 | Higher |
| Closures | **0 ✅** | 4 | 3 | -4 | Lower | -3 | Lower | +1 | Higher |
| Exits | **1 ✅** | 6 | 2 | -5 | Lower | -1 | Lower | +4 | Higher |

## p03719

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 90.9879 | 90.9879 | 90.9879 | 0 | Same | 0 | Same | 0 | Same |
| MI (SEI) | 73.8286 | 55.9742 | 73.8286 | +17.8544 | Higher | 0 | Same | -17.8544 | Lower |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 495.4196 | 495.4196 | 495.4196 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Effort | 6270.1547 | 6270.1547 | 6270.1547 | 0 | Same | 0 | Same | 0 | Same |
| SLOC | 18 | 18 | 18 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 1 | 0 | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03722

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.7030 | 50.2860 | **58.2477 ✅** | +6.4170 | Higher | -1.5447 | Lower | -7.9616 | Lower |
| MI (SEI) | **34.2574 ✅** | 25.0333 | 32.7925 | +9.2241 | Higher | +1.4649 | Higher | -7.7592 | Lower |
| Cyclomatic (sum) | **14 ✅** | 20 | 15 | -6 | Lower | -1 | Lower | +5 | Higher |
| Cognitive (sum) | 24 | 32 | **19 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| Halstead Volume | **1987.3814 ✅** | 2672.1632 | 2020.0077 | -684.7818 | Lower | -32.6263 | Lower | +652.1555 | Higher |
| Halstead Effort | **73533.1110 ✅** | 114618.7457 | 81939.7995 | -41085.6347 | Lower | -8406.6885 | Lower | +32678.9462 | Higher |
| SLOC | 83 | 103 | **74 ✅** | -20 | Lower | +9 | Higher | +29 | Higher |
| PLOC | 63 | 80 | **59 ✅** | -17 | Lower | +4 | Higher | +21 | Higher |
| LLOC | 38 | 57 | **35 ✅** | -19 | Lower | +3 | Higher | +22 | Higher |
| Comments (CLOC) | 11 | **13 ✅** | 7 | -2 | Lower | +4 | Higher | +6 | Higher |
| Blank Lines | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03736

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.1756 ✅** | 41.5798 | 52.0924 | +12.5959 | Higher | +2.0833 | Higher | -10.5126 | Lower |
| MI (SEI) | **31.8253 ✅** | 11.8456 | 20.9861 | +19.9797 | Higher | +10.8391 | Higher | -9.1406 | Lower |
| Cyclomatic (sum) | 13 | 23 | **12 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| Cognitive (sum) | 15 | 32 | **7 ✅** | -17 | Lower | +8 | Higher | +25 | Higher |
| Halstead Volume | **2814.6654 ✅** | 4210.5431 | 3348.8891 | -1395.8777 | Lower | -534.2238 | Lower | +861.6540 | Higher |
| Halstead Effort | **130680.8928 ✅** | 277895.8467 | 180700.4769 | -147214.9539 | Lower | -50019.5841 | Lower | +97195.3698 | Higher |
| SLOC | **88 ✅** | 146 | 96 | -58 | Lower | -8 | Lower | +50 | Higher |
| PLOC | **60 ✅** | 107 | 72 | -47 | Lower | -12 | Lower | +35 | Higher |
| LLOC | **51 ✅** | 82 | 55 | -31 | Lower | -4 | Lower | +27 | Higher |
| Comments (CLOC) | 13 | **17 ✅** | 7 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 18 | 22 | **17 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 3 | 3 | -2 | Lower | -2 | Lower | 0 | Same |

## p03762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.4687 ✅** | 65.4853 | 65.8860 | +4.9834 | Higher | +4.5827 | Higher | -0.4007 | Lower |
| MI (SEI) | **51.5158 ✅** | 48.6839 | 43.0710 | +2.8319 | Higher | +8.4447 | Higher | +5.6129 | Higher |
| Cyclomatic (sum) | **6 ✅** | 8 | 8 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **1448.5951 ✅** | 1630.9528 | 1792.5422 | -182.3576 | Lower | -343.9470 | Lower | -161.5894 | Lower |
| Halstead Effort | 46173.9701 | **46019.3159 ✅** | 59486.7928 | +154.6541 | Higher | -13312.8227 | Lower | -13467.4768 | Lower |
| SLOC | **44 ✅** | 56 | 53 | -12 | Lower | -9 | Lower | +3 | Higher |
| PLOC | **31 ✅** | 38 | 40 | -7 | Lower | -9 | Lower | -2 | Lower |
| LLOC | **27 ✅** | 28 | 33 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 5 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p03766

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.1366 | **66.7511 ✅** | 62.3192 | -5.6145 | Lower | -1.1826 | Lower | +4.4319 | Higher |
| MI (SEI) | **44.0018 ✅** | 42.1418 | 38.3995 | +1.8600 | Higher | +5.6023 | Higher | +3.7423 | Higher |
| Cyclomatic (sum) | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 8 | 6 | **5 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Halstead Volume | 1970.4725 | **1683.4638 ✅** | 1998.6678 | +287.0087 | Higher | -28.1953 | Lower | -315.2040 | Lower |
| Halstead Effort | 107334.4511 | **75755.8719 ✅** | 87015.1719 | +31578.5792 | Higher | +20319.2792 | Higher | -11259.3000 | Lower |
| SLOC | 67 | **52 ✅** | 62 | +15 | Higher | +5 | Higher | -10 | Lower |
| PLOC | 45 | **36 ✅** | 46 | +9 | Higher | -1 | Lower | -10 | Lower |
| LLOC | 39 | **33 ✅** | 35 | +6 | Higher | +4 | Higher | -2 | Lower |
| Comments (CLOC) | **12 ✅** | 4 | 6 | +8 | Higher | +6 | Higher | -2 | Lower |
| Blank Lines | 13 | 12 | **10 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p03797

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.0424 | 81.4382 | **82.6034 ✅** | +0.6043 | Higher | -0.5610 | Lower | -1.1652 | Lower |
| MI (SEI) | 70.9048 | 62.1859 | **72.0584 ✅** | +8.7189 | Higher | -1.1536 | Lower | -9.8725 | Lower |
| Cyclomatic (sum) | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | 7 | **4 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | **639.5337 ✅** | 643.9533 | 672.0903 | -4.4196 | Lower | -32.5566 | Lower | -28.1370 | Lower |
| Halstead Effort | 13463.8667 | **12879.0660 ✅** | 14785.9855 | +584.8007 | Higher | -1322.1188 | Lower | -1906.9195 | Lower |
| SLOC | 28 | 29 | **27 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| PLOC | 19 | 22 | 19 | -3 | Lower | 0 | Same | +3 | Higher |
| LLOC | **8 ✅** | 10 | 16 | -2 | Lower | -8 | Lower | -6 | Lower |
| Comments (CLOC) | 4 | 2 | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03808

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.5704 ✅** | 56.6985 | 55.6791 | +3.8720 | Higher | +4.8913 | Higher | +1.0193 | Higher |
| MI (SEI) | 33.7891 | **37.3879 ✅** | 30.4364 | -3.5987 | Lower | +3.3528 | Higher | +6.9515 | Higher |
| Cyclomatic (sum) | 14 | 14 | 15 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 13 | **12 ✅** | 15 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | **1929.1596 ✅** | 2065.6722 | 2230.3814 | -136.5126 | Lower | -301.2218 | Lower | -164.7093 | Lower |
| Halstead Effort | 129519.7845 | **100672.8279 ✅** | 128246.9322 | +28846.9566 | Higher | +1272.8523 | Higher | -27574.1043 | Lower |
| SLOC | **66 ✅** | 82 | 84 | -16 | Lower | -18 | Lower | -2 | Lower |
| PLOC | **50 ✅** | 56 | 60 | -6 | Lower | -10 | Lower | -4 | Lower |
| LLOC | **44 ✅** | 46 | 51 | -2 | Lower | -7 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 9 | -9 | Lower | -4 | Lower | +5 | Higher |
| Blank Lines | **11 ✅** | 13 | 15 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |

## p03815

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.5168 | 82.5168 | **87.4758 ✅** | 0 | Same | -4.9589 | Lower | -4.9589 | Lower |
| MI (SEI) | 68.7461 | 68.7461 | **73.4222 ✅** | 0 | Same | -4.6761 | Lower | -4.6761 | Lower |
| Cyclomatic (sum) | 7 | 7 | **4 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 625.5158 | 625.5158 | **520.9453 ✅** | 0 | Same | +104.5705 | Higher | +104.5705 | Higher |
| Halstead Effort | 12510.3160 | 12510.3160 | **7749.0619 ✅** | 0 | Same | +4761.2541 | Higher | +4761.2541 | Higher |
| SLOC | 27 | 27 | **22 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| PLOC | 19 | 19 | **16 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| LLOC | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 3 | 3 | 2 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03817

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.4759 ✅** | 75.2905 | 80.8704 | +13.1854 | Higher | +7.6055 | Higher | -5.5799 | Lower |
| MI (SEI) | 68.9331 | 66.5612 | **71.1377 ✅** | +2.3718 | Higher | -2.2046 | Lower | -4.5765 | Lower |
| Cyclomatic (sum) | **4 ✅** | 10 | 6 | -6 | Lower | -2 | Lower | +4 | Higher |
| Cognitive (sum) | **3 ✅** | 8 | 4 | -5 | Lower | -1 | Lower | +4 | Higher |
| Halstead Volume | **496.8278 ✅** | 758.1585 | 646.2567 | -261.3307 | Lower | -149.4289 | Lower | +111.9017 | Higher |
| Halstead Effort | **7551.7827 ✅** | 13065.5978 | 11551.8394 | -5513.8152 | Lower | -4000.0567 | Lower | +1513.7585 | Higher |
| SLOC | **21 ✅** | 38 | 30 | -17 | Lower | -9 | Lower | +8 | Higher |
| PLOC | **16 ✅** | 27 | 20 | -11 | Lower | -4 | Lower | +7 | Higher |
| LLOC | **7 ✅** | 9 | 8 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 1 | **8 ✅** | 5 | -7 | Lower | -4 | Lower | +3 | Higher |
| Blank Lines | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p03837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.7131 ✅** | 54.0352 | 56.2930 | +3.6780 | Higher | +1.4201 | Higher | -2.2579 | Lower |
| MI (SEI) | **32.0213 ✅** | 29.4302 | 28.2254 | +2.5911 | Higher | +3.7959 | Higher | +1.2048 | Higher |
| Cyclomatic (sum) | 15 | 15 | 16 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 25 | 30 | **20 ✅** | -5 | Lower | +5 | Higher | +10 | Higher |
| Halstead Volume | **2238.7115 ✅** | 2384.4301 | 2590.0754 | -145.7186 | Lower | -351.3639 | Lower | -205.6453 | Lower |
| Halstead Effort | 86190.3941 | **79544.5879 ✅** | 138254.0245 | +6645.8062 | Higher | -52063.6303 | Lower | -58709.4366 | Lower |
| SLOC | **74 ✅** | 91 | 76 | -17 | Lower | -2 | Lower | +15 | Higher |
| PLOC | 59 | 67 | 59 | -8 | Lower | 0 | Same | +8 | Higher |
| LLOC | 52 | 50 | 50 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 7 | **11 ✅** | 6 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | **9 ✅** | 13 | 11 | -4 | Lower | -2 | Lower | +2 | Higher |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p03838

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.5877 ✅** | 69.0063 | 72.0501 | +9.5813 | Higher | +6.5376 | Higher | -3.0438 | Lower |
| MI (SEI) | **80.6570 ✅** | 69.1387 | 54.3639 | +11.5183 | Higher | +26.2931 | Higher | +14.7748 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 9 | -2 | Lower | -5 | Lower | -3 | Lower |
| Cognitive (sum) | **2 ✅** | 4 | 12 | -2 | Lower | -10 | Lower | -8 | Lower |
| Halstead Volume | **569.7958 ✅** | 856.7291 | 1005.4245 | -286.9332 | Lower | -435.6287 | Lower | -148.6954 | Lower |
| Halstead Effort | **9116.7334 ✅** | 17819.9649 | 29484.0734 | -8703.2315 | Lower | -20367.3400 | Lower | -11664.1085 | Lower |
| SLOC | **37 ✅** | 57 | 43 | -20 | Lower | -6 | Lower | +14 | Higher |
| PLOC | **16 ✅** | 21 | 33 | -5 | Lower | -17 | Lower | -12 | Lower |
| LLOC | **13 ✅** | 16 | 28 | -3 | Lower | -15 | Lower | -12 | Lower |
| Comments (CLOC) | 16 | **29 ✅** | 5 | -13 | Lower | +11 | Higher | +24 | Higher |
| Blank Lines | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03845

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.9825 | **76.2000 ✅** | 69.2503 | -2.2175 | Lower | +4.7323 | Higher | +6.9497 | Higher |
| MI (SEI) | 55.9239 | **59.6451 ✅** | 47.0761 | -3.7212 | Lower | +8.8478 | Higher | +12.5691 | Higher |
| Cyclomatic (sum) | 5 | **4 ✅** | 8 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 3 | **2 ✅** | 5 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 1321.6751 | **1072.3128 ✅** | 1459.3126 | +249.3623 | Higher | -137.6375 | Lower | -386.9998 | Lower |
| Halstead Effort | 27466.0608 | **21446.2560 ✅** | 35570.7438 | +6019.8048 | Higher | -8104.6831 | Lower | -14124.4878 | Lower |
| SLOC | 37 | **35 ✅** | 46 | +2 | Higher | -9 | Lower | -11 | Lower |
| PLOC | 29 | **25 ✅** | 34 | +4 | Higher | -5 | Lower | -9 | Lower |
| LLOC | 23 | **21 ✅** | 26 | +2 | Higher | -3 | Lower | -5 | Lower |
| Comments (CLOC) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | **6 ✅** | 7 | 8 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03865

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.6542 | **84.7982 ✅** | 77.4988 | -4.1439 | Lower | +3.1554 | Higher | +7.2993 | Higher |
| MI (SEI) | 77.3489 | **81.4522 ✅** | 78.8539 | -4.1033 | Lower | -1.5049 | Lower | +2.5983 | Higher |
| Cyclomatic (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | **523.2548 ✅** | 585.4121 | 551.0324 | -62.1573 | Lower | -27.7776 | Lower | +34.3797 | Higher |
| Halstead Effort | 10465.0964 | 10058.4441 | **9422.6539 ✅** | +406.6523 | Higher | +1042.4425 | Higher | +635.7902 | Higher |
| SLOC | 33 | **25 ✅** | 40 | +8 | Higher | -7 | Lower | -15 | Lower |
| PLOC | 17 | **15 ✅** | 19 | +2 | Higher | -2 | Lower | -4 | Lower |
| LLOC | 11 | **9 ✅** | 10 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | 9 | 6 | **17 ✅** | +3 | Higher | -8 | Lower | -11 | Lower |
| Blank Lines | 7 | 4 | 4 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p03899

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.3206 ✅** | 53.6118 | 54.0896 | +3.7088 | Higher | +3.2310 | Higher | -0.4778 | Lower |
| MI (SEI) | **37.0382 ✅** | 36.7027 | 27.5270 | +0.3355 | Higher | +9.5112 | Higher | +9.1757 | Higher |
| Cyclomatic (sum) | **11 ✅** | 14 | 12 | -3 | Lower | -1 | Lower | +2 | Higher |
| Cognitive (sum) | **14 ✅** | 33 | 19 | -19 | Lower | -5 | Lower | +14 | Higher |
| Halstead Volume | **2092.8008 ✅** | 2215.9572 | 2280.8495 | -123.1564 | Lower | -188.0488 | Lower | -64.8923 | Lower |
| Halstead Effort | 90238.2648 | **76002.6169 ✅** | 100072.2734 | +14235.6479 | Higher | -9834.0086 | Lower | -24069.6565 | Lower |
| SLOC | **82 ✅** | 97 | 96 | -15 | Lower | -14 | Lower | +1 | Higher |
| PLOC | **54 ✅** | 60 | 69 | -6 | Lower | -15 | Lower | -9 | Lower |
| LLOC | **41 ✅** | 43 | 50 | -2 | Lower | -9 | Lower | -7 | Lower |
| Comments (CLOC) | 13 | **22 ✅** | 10 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | **15 ✅** | 16 | 17 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03901

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **83.5180 ✅** | 82.2337 | 58.3867 | +1.2843 | Higher | +25.1313 | Higher | +23.8470 | Higher |
| MI (SEI) | 70.7607 | **71.4232 ✅** | 45.7770 | -0.6625 | Lower | +24.9836 | Higher | +25.6461 | Higher |
| Cyclomatic (sum) | 4 | 4 | 11 | 0 | Same | -7 | Lower | -7 | Lower |
| Cognitive (sum) | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **748.8245 ✅** | 754.2508 | 1771.2980 | -5.4263 | Lower | -1022.4735 | Lower | -1017.0472 | Lower |
| Halstead Effort | 18435.3469 | **15740.8863 ✅** | 51135.6876 | +2694.4606 | Higher | -32700.3407 | Lower | -35394.8013 | Lower |
| SLOC | **25 ✅** | 27 | 81 | -2 | Lower | -56 | Lower | -54 | Lower |
| PLOC | **17 ✅** | 18 | 50 | -1 | Lower | -33 | Lower | -32 | Lower |
| LLOC | 11 | **9 ✅** | 24 | +2 | Higher | -13 | Lower | -15 | Lower |
| Comments (CLOC) | 3 | 4 | **22 ✅** | -1 | Lower | -19 | Lower | -18 | Lower |
| Blank Lines | 5 | 5 | 9 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 6 | 0 | Same | -6 | Lower | -6 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03914

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.3841 ✅** | 57.7916 | 61.8099 | +15.5925 | Higher | +11.5742 | Higher | -4.0183 | Lower |
| MI (SEI) | **57.0135 ✅** | 44.3415 | 46.8363 | +12.6720 | Higher | +10.1772 | Higher | -2.4948 | Lower |
| Cyclomatic (sum) | **5 ✅** | 10 | 12 | -5 | Lower | -7 | Lower | -2 | Lower |
| Cognitive (sum) | **6 ✅** | 22 | 23 | -16 | Lower | -17 | Lower | -1 | Lower |
| Halstead Volume | **1258.5799 ✅** | 2157.7981 | 1827.6901 | -899.2181 | Lower | -569.1101 | Lower | +330.1080 | Higher |
| Halstead Effort | **32723.0782 ✅** | 94279.1764 | 74827.7817 | -61556.0983 | Lower | -42104.7035 | Lower | +19451.3947 | Higher |
| SLOC | **39 ✅** | 80 | 64 | -41 | Lower | -25 | Lower | +16 | Higher |
| PLOC | **27 ✅** | 50 | 41 | -23 | Lower | -14 | Lower | +9 | Higher |
| LLOC | **25 ✅** | 40 | 38 | -15 | Lower | -13 | Lower | +2 | Higher |
| Comments (CLOC) | 5 | **21 ✅** | 13 | -16 | Lower | -8 | Lower | +8 | Higher |
| Blank Lines | **7 ✅** | 14 | 10 | -7 | Lower | -3 | Lower | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03929

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.9219 ✅** | 61.2717 | 60.4377 | +2.6502 | Higher | +3.4842 | Higher | +0.8340 | Higher |
| MI (SEI) | **46.1766 ✅** | 39.4232 | 36.4446 | +6.7534 | Higher | +9.7320 | Higher | +2.9786 | Higher |
| Cyclomatic (sum) | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 24 | **14 ✅** | 24 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | **1362.7309 ✅** | 1757.3943 | 1647.5559 | -394.6634 | Lower | -284.8251 | Lower | +109.8384 | Higher |
| Halstead Effort | **39730.6529 ✅** | 52864.3204 | 67423.0580 | -13133.6675 | Lower | -27692.4051 | Lower | -14558.7376 | Lower |
| SLOC | **60 ✅** | 67 | 70 | -7 | Lower | -10 | Lower | -3 | Lower |
| PLOC | **43 ✅** | 49 | 54 | -6 | Lower | -11 | Lower | -5 | Lower |
| LLOC | **34 ✅** | 39 | 52 | -5 | Lower | -18 | Lower | -13 | Lower |
| Comments (CLOC) | **9 ✅** | 8 | 7 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p03932

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.0561 | **53.4810 ✅** | 44.1640 | -12.4250 | Lower | -3.1079 | Lower | +9.3171 | Higher |
| MI (SEI) | 11.7956 | **36.4050 ✅** | 17.7666 | -24.6094 | Lower | -5.9710 | Lower | +18.6384 | Higher |
| Cyclomatic (sum) | 26 | 21 | **19 ✅** | +5 | Higher | +7 | Higher | +2 | Higher |
| Cognitive (sum) | 28 | 42 | **18 ✅** | -14 | Lower | +10 | Higher | +24 | Higher |
| Halstead Volume | 4545.6497 | **2180.0971 ✅** | 4285.7386 | +2365.5525 | Higher | +259.9111 | Higher | -2105.6415 | Lower |
| Halstead Effort | 363651.9747 | **121177.0665 ✅** | 368048.7354 | +242474.9082 | Higher | -4396.7608 | Lower | -246871.6690 | Lower |
| SLOC | 141 | **89 ✅** | 131 | +52 | Higher | +10 | Higher | -42 | Lower |
| PLOC | 105 | **58 ✅** | 93 | +47 | Higher | +12 | Higher | -35 | Lower |
| LLOC | 60 | **42 ✅** | 65 | +18 | Higher | -5 | Lower | -23 | Lower |
| Comments (CLOC) | 17 | 19 | 19 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 19 | **13 ✅** | 20 | +6 | Higher | -1 | Lower | -7 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **1 ✅** | 6 | +3 | Higher | -2 | Lower | -5 | Lower |

## p03951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.0987 ✅** | 80.0774 | 61.9168 | +4.0213 | Higher | +22.1820 | Higher | +18.1606 | Higher |
| MI (SEI) | 71.5985 | **73.8062 ✅** | 44.8484 | -2.2076 | Lower | +26.7502 | Higher | +28.9578 | Higher |
| Cyclomatic (sum) | 4 | 4 | 11 | 0 | Same | -7 | Lower | -7 | Lower |
| Cognitive (sum) | 3 | 3 | 13 | 0 | Same | -10 | Lower | -10 | Lower |
| Halstead Volume | **669.6940 ✅** | 742.4827 | 1480.5030 | -72.7887 | Lower | -810.8090 | Lower | -738.0203 | Lower |
| Halstead Effort | **12756.0762 ✅** | 16334.6194 | 53956.1099 | -3578.5432 | Lower | -41200.0337 | Lower | -37621.4904 | Lower |
| SLOC | **25 ✅** | 31 | 69 | -6 | Lower | -44 | Lower | -38 | Lower |
| PLOC | **18 ✅** | 19 | 48 | -1 | Lower | -30 | Lower | -29 | Lower |
| LLOC | **14 ✅** | 15 | 35 | -1 | Lower | -21 | Lower | -20 | Lower |
| Comments (CLOC) | 3 | 7 | **12 ✅** | -4 | Lower | -9 | Lower | -5 | Lower |
| Blank Lines | **4 ✅** | 5 | 9 | -1 | Lower | -5 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03977

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.2502 ✅** | 56.4263 | 53.4969 | +2.8239 | Higher | +5.7534 | Higher | +2.9294 | Higher |
| MI (SEI) | **38.4764 ✅** | 36.9320 | 37.2476 | +1.5443 | Higher | +1.2288 | Higher | -0.3155 | Lower |
| Cyclomatic (sum) | **12 ✅** | 18 | 15 | -6 | Lower | -3 | Lower | +3 | Higher |
| Cognitive (sum) | **13 ✅** | 27 | 19 | -14 | Lower | -6 | Lower | +8 | Higher |
| Halstead Volume | **1614.4447 ✅** | 2048.2435 | 2167.4530 | -433.7989 | Lower | -553.0083 | Lower | -119.2094 | Lower |
| Halstead Effort | **81167.5975 ✅** | 118798.1248 | 101742.7923 | -37630.5273 | Lower | -20575.1948 | Lower | +17055.3325 | Higher |
| SLOC | **78 ✅** | 79 | 97 | -1 | Lower | -19 | Lower | -18 | Lower |
| PLOC | **57 ✅** | 58 | 68 | -1 | Lower | -11 | Lower | -10 | Lower |
| LLOC | 56 | **53 ✅** | 65 | +3 | Higher | -9 | Lower | -12 | Lower |
| Comments (CLOC) | 11 | 13 | **23 ✅** | -2 | Lower | -12 | Lower | -10 | Lower |
| Blank Lines | 11 | 11 | **9 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03996

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0818 | 49.8250 | **53.9050 ✅** | +3.2568 | Higher | -0.8232 | Lower | -4.0799 | Lower |
| MI (SEI) | 27.5612 | 28.8121 | **29.9696 ✅** | -1.2509 | Lower | -2.4084 | Lower | -1.1574 | Lower |
| Cyclomatic (sum) | **15 ✅** | 18 | 17 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **20 ✅** | 21 | 21 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **2504.9896 ✅** | 2674.1130 | 2574.2989 | -169.1234 | Lower | -69.3093 | Lower | +99.8141 | Higher |
| Halstead Effort | **102650.1153 ✅** | 131714.9202 | 120992.0468 | -29064.8050 | Lower | -18341.9315 | Lower | +10722.8735 | Higher |
| SLOC | 95 | 109 | **87 ✅** | -14 | Lower | +8 | Higher | +22 | Higher |
| PLOC | 71 | 74 | **66 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| LLOC | 56 | 56 | 56 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 11 | **20 ✅** | 11 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 14 | 16 | **12 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p04005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.6519 ✅** | 75.3931 | 68.6051 | +3.2588 | Higher | +10.0468 | Higher | +6.7880 | Higher |
| MI (SEI) | **70.0845 ✅** | 58.5705 | 59.0614 | +11.5139 | Higher | +11.0231 | Higher | -0.4909 | Lower |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 9 | **7 ✅** | 10 | +2 | Higher | -1 | Lower | -3 | Lower |
| Halstead Volume | **818.2861 ✅** | 961.0743 | 1127.5815 | -142.7882 | Lower | -309.2954 | Lower | -166.5072 | Lower |
| Halstead Effort | 22698.5452 | **21528.0641 ✅** | 30509.7543 | +1170.4810 | Higher | -7811.2091 | Lower | -8981.6901 | Lower |
| SLOC | **31 ✅** | 36 | 52 | -5 | Lower | -21 | Lower | -16 | Lower |
| PLOC | **22 ✅** | 25 | 31 | -3 | Lower | -9 | Lower | -6 | Lower |
| LLOC | **17 ✅** | 21 | 24 | -4 | Lower | -7 | Lower | -3 | Lower |
| Comments (CLOC) | 6 | 4 | **13 ✅** | +2 | Higher | -7 | Lower | -9 | Lower |
| Blank Lines | **3 ✅** | 7 | 8 | -4 | Lower | -5 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p04013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.3916 | **62.5132 ✅** | 49.9405 | -4.1216 | Lower | +8.4511 | Higher | +12.5727 | Higher |
| MI (SEI) | 39.4679 | **52.0011 ✅** | 28.3481 | -12.5333 | Lower | +11.1198 | Higher | +23.6530 | Higher |
| Cyclomatic (sum) | 19 | **11 ✅** | 24 | +8 | Higher | -5 | Lower | -13 | Lower |
| Cognitive (sum) | 24 | **13 ✅** | 28 | +11 | Higher | -4 | Lower | -15 | Lower |
| Halstead Volume | 2047.1598 | **1590 ✅** | 2979.2807 | +457.1598 | Higher | -932.1209 | Lower | -1389.2807 | Lower |
| Halstead Effort | 105645.8517 | **53795.0000 ✅** | 199909.7317 | +51850.8517 | Higher | -94263.8800 | Lower | -146114.7317 | Lower |
| SLOC | 69 | **65 ✅** | 96 | +4 | Higher | -27 | Lower | -31 | Lower |
| PLOC | 48 | **40 ✅** | 68 | +8 | Higher | -20 | Lower | -28 | Lower |
| LLOC | 33 | **30 ✅** | 45 | +3 | Higher | -12 | Lower | -15 | Lower |
| Comments (CLOC) | 11 | **18 ✅** | 16 | -7 | Lower | -5 | Lower | +2 | Higher |
| Blank Lines | 10 | **9 ✅** | 12 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p04020

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1886 | 68.5652 | **75.6345 ✅** | +5.6234 | Higher | -1.4458 | Lower | -7.0693 | Lower |
| MI (SEI) | **58.8890 ✅** | 57.0837 | 56.5384 | +1.8053 | Higher | +2.3506 | Higher | +0.5453 | Higher |
| Cyclomatic (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 9 | **7 ✅** | 8 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | **979.4744 ✅** | 1154.9122 | 1048.8809 | -175.4378 | Lower | -69.4065 | Lower | +106.0313 | Higher |
| Halstead Effort | **36689.4785 ✅** | 43659.8067 | 38514.9069 | -6970.3282 | Lower | -1825.4285 | Lower | +5144.8998 | Higher |
| SLOC | 38 | 51 | **34 ✅** | -13 | Lower | +4 | Higher | +17 | Higher |
| PLOC | 28 | 32 | **26 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| LLOC | 22 | 28 | **21 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Comments (CLOC) | 5 | **11 ✅** | 3 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 6 | 9 | **5 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p04022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.5445 | 20.8610 | **29.2696 ✅** | +1.6835 | Higher | -6.7251 | Lower | -8.4086 | Lower |
| MI (SEI) | -17.5791 | -20.5121 | **-16.9609 ✅** | +2.9330 | Higher | -0.6183 | Lower | -3.5513 | Lower |
| Cyclomatic (sum) | 58 | 59 | **39 ✅** | -1 | Lower | +19 | Higher | +20 | Higher |
| Cognitive (sum) | 120 | 86 | 86 | +34 | Higher | +34 | Higher | 0 | Same |
| Halstead Volume | 6599.5760 | 7104.8683 | **5834.1741 ✅** | -505.2923 | Lower | +765.4019 | Higher | +1270.6942 | Higher |
| Halstead Effort | 667807.0978 | 683728.2362 | **491255.6895 ✅** | -15921.1384 | Lower | +176551.4082 | Higher | +192472.5466 | Higher |
| SLOC | 249 | 266 | **224 ✅** | -17 | Lower | +25 | Higher | +42 | Higher |
| PLOC | 204 | 224 | **189 ✅** | -20 | Lower | +15 | Higher | +35 | Higher |
| LLOC | **152 ✅** | 174 | 155 | -22 | Lower | -3 | Lower | +19 | Higher |
| Comments (CLOC) | 17 | 17 | 6 | 0 | Same | +11 | Higher | +11 | Higher |
| Blank Lines | 30 | **26 ✅** | 30 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 3 | 6 | 3 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 18 | 4 | -14 | Lower | 0 | Same | +14 | Higher |

## p04024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **38.1940 ✅** | 36.4123 | 35.9617 | +1.7817 | Higher | +2.2324 | Higher | +0.4506 | Higher |
| MI (SEI) | **7.4484 ✅** | -0.9071 | -2.4562 | +8.3555 | Higher | +9.9047 | Higher | +1.5491 | Higher |
| Cyclomatic (sum) | **36 ✅** | 39 | 40 | -3 | Lower | -4 | Lower | -1 | Lower |
| Cognitive (sum) | **49 ✅** | 56 | 54 | -7 | Lower | -5 | Lower | +2 | Higher |
| Halstead Volume | **4354.9231 ✅** | 4746.7126 | 4952.4317 | -391.7895 | Lower | -597.5086 | Lower | -205.7191 | Lower |
| Halstead Effort | **334156.6012 ✅** | 365333.1908 | 393508.4730 | -31176.5897 | Lower | -59351.8719 | Lower | -28175.2822 | Lower |
| SLOC | **148 ✅** | 154 | 154 | -6 | Lower | -6 | Lower | 0 | Same |
| PLOC | **112 ✅** | 125 | 127 | -13 | Lower | -15 | Lower | -2 | Lower |
| LLOC | **91 ✅** | 101 | 92 | -10 | Lower | -1 | Lower | +9 | Higher |
| Comments (CLOC) | **16 ✅** | 9 | 8 | +7 | Higher | +8 | Higher | +1 | Higher |
| Blank Lines | 20 | 20 | **19 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **5 ✅** | 6 | 7 | -1 | Lower | -2 | Lower | -1 | Lower |

## p04030

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.4764 | **76.6317 ✅** | 70.7892 | -8.1553 | Lower | -2.3127 | Lower | +5.8425 | Higher |
| MI (SEI) | 57.1593 | **73.4386 ✅** | 59.9044 | -16.2792 | Lower | -2.7451 | Lower | +13.5341 | Higher |
| Cyclomatic (sum) | 11 | 7 | 7 | +4 | Higher | +4 | Higher | 0 | Same |
| Cognitive (sum) | 19 | **7 ✅** | 11 | +12 | Higher | +8 | Higher | -4 | Lower |
| Halstead Volume | 1075.3329 | **616.9158 ✅** | 1061.0450 | +458.4171 | Higher | +14.2879 | Higher | -444.1292 | Lower |
| Halstead Effort | 29571.6549 | **10138.8766 ✅** | 26123.6588 | +19432.7783 | Higher | +3447.9962 | Higher | -15984.7821 | Lower |
| SLOC | 51 | **39 ✅** | 47 | +12 | Higher | +4 | Higher | -8 | Lower |
| PLOC | 35 | **23 ✅** | 31 | +12 | Higher | +4 | Higher | -8 | Lower |
| LLOC | 25 | **12 ✅** | 27 | +13 | Higher | -2 | Lower | -15 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 10 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.9721 ✅** | 58.6480 | 55.3677 | 1.3240 | Higher | 4.6044 | Higher | 3.2803 | Higher |
| MI (SEI) | **37.8016 ✅** | 36.7288 | 29.8724 | 1.0727 | Higher | 7.9292 | Higher | 6.8564 | Higher |
| Cyclomatic (sum) | **17.0900 ✅** | 17.8200 | 20.1200 | -0.7300 | Lower | -3.0300 | Lower | -2.3000 | Lower |
| Cognitive (sum) | **22.2767 ✅** | 23.5333 | 25.2567 | -1.2567 | Lower | -2.9800 | Lower | -1.7233 | Lower |
| Halstead Volume | 2517.0589 | **2499.3853 ✅** | 3054.2866 | 17.6736 | Higher | -537.2278 | Lower | -554.9013 | Lower |
| Halstead Effort | 154876.3752 | **139797.7260 ✅** | 219963.9815 | 15078.6492 | Higher | -65087.6062 | Lower | -80166.2555 | Lower |
| SLOC | **84.9833 ✅** | 87.9500 | 100.9100 | -2.9667 | Lower | -15.9267 | Lower | -12.9600 | Lower |
| PLOC | **65.1333 ✅** | 65.3433 | 79.5833 | -0.2100 | Lower | -14.4500 | Lower | -14.2400 | Lower |
| LLOC | 47.5633 | **46.1667 ✅** | 59.2767 | 1.3967 | Higher | -11.7133 | Lower | -13.1100 | Lower |
| Comments (CLOC) | 9.3467 | **11.1767 ✅** | 8.9033 | -1.8300 | Lower | 0.4433 | Higher | 2.2733 | Higher |
| Blank Lines | **11.6400 ✅** | 12.4767 | 13.1333 | -0.8367 | Lower | -1.4933 | Lower | -0.6567 | Lower |
| Functions | 2.2033 | **2.0533 ✅** | 2.8033 | 0.1500 | Higher | -0.6000 | Lower | -0.7500 | Lower |
| Closures | 0.4133 | 0.4433 | **0.3833 ✅** | -0.0300 | Lower | 0.0300 | Higher | 0.0600 | Higher |
| Exits | **1.8300 ✅** | 2.0900 | 2.6467 | -0.2600 | Lower | -0.8167 | Lower | -0.5567 | Lower |

