# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/code_translation_specs_ASE/r_code_copy/full_translation_loop_GPT_5_2_metrics.json`

## p00004

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.6377 ✅** | 69.5450 | 66.3610 | +5.0927 | Higher | +8.2767 | Higher | +3.1840 | Higher |
| MI (SEI) | **63.2352 ✅** | 49.3154 | 46.9008 | +13.9198 | Higher | +16.3345 | Higher | +2.4146 | Higher |
| Cyclomatic (sum) | **4 ✅** | 7 | 15 | -3 | Lower | -11 | Lower | -8 | Lower |
| Cognitive (sum) | **1 ✅** | 5 | 6 | -4 | Lower | -5 | Lower | -1 | Lower |
| Halstead Volume | **1120.8195 ✅** | 1262.2896 | 1353.2727 | -141.4701 | Lower | -232.4532 | Lower | -90.9831 | Lower |
| Halstead Effort | **26471.1194 ✅** | 29032.6612 | 33493.4988 | -2561.5418 | Lower | -7022.3794 | Lower | -4460.8376 | Lower |
| SLOC | **38 ✅** | 48 | 51 | -10 | Lower | -13 | Lower | -3 | Lower |
| PLOC | **27 ✅** | 36 | 40 | -9 | Lower | -13 | Lower | -4 | Lower |
| LLOC | 21 | 22 | **18 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Comments (CLOC) | **7 ✅** | 5 | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00010

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.4513 ✅** | 65.2164 | 62.9869 | +0.2349 | Higher | +2.4644 | Higher | +2.2295 | Higher |
| MI (SEI) | 43.9221 | **49.6761 ✅** | 39.6132 | -5.7540 | Lower | +4.3089 | Higher | +10.0629 | Higher |
| Cyclomatic (sum) | **5 ✅** | 6 | 9 | -1 | Lower | -4 | Lower | -3 | Lower |
| Cognitive (sum) | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 2099.4810 | **1984.7152 ✅** | 2034.9220 | +114.7658 | Higher | +64.5591 | Higher | -50.2067 | Lower |
| Halstead Effort | 70467.1962 | **62622.9888 ✅** | 84110.1077 | +7844.2074 | Higher | -13642.9115 | Lower | -21487.1190 | Lower |
| SLOC | **54 ✅** | 55 | 60 | -1 | Lower | -6 | Lower | -5 | Lower |
| PLOC | 37 | **34 ✅** | 43 | +3 | Higher | -6 | Lower | -9 | Lower |
| LLOC | 34 | **26 ✅** | 33 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.3134 | **84.2105 ✅** | 80.0207 | -2.8971 | Lower | +1.2927 | Higher | +4.1899 | Higher |
| MI (SEI) | **73.5169 ✅** | 67.1268 | 71.4369 | +6.3900 | Higher | +2.0799 | Higher | -4.3101 | Lower |
| Cyclomatic (sum) | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **4 ✅** | 7 | 6 | -3 | Lower | -2 | Lower | +1 | Higher |
| Halstead Volume | 585.4121 | **554.9672 ✅** | 622.3752 | +30.4449 | Higher | -36.9631 | Lower | -67.4080 | Lower |
| Halstead Effort | 9339.9838 | **7504.6705 ✅** | 10891.5661 | +1835.3133 | Higher | -1551.5823 | Lower | -3386.8955 | Lower |
| SLOC | 31 | **26 ✅** | 32 | +5 | Higher | -1 | Lower | -6 | Lower |
| PLOC | 19 | 19 | 22 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 16 | **12 ✅** | 16 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 6 | 2 | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Blank Lines | 6 | 5 | **4 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00015

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.0186 ✅** | 49.8737 | 50.1815 | +2.1448 | Higher | +1.8371 | Higher | -0.3077 | Lower |
| MI (SEI) | **29.8625 ✅** | 24.3980 | 23.5014 | +5.4645 | Higher | +6.3612 | Higher | +0.8967 | Higher |
| Cyclomatic (sum) | **22 ✅** | 27 | 23 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | 32 | **26 ✅** | 36 | +6 | Higher | -4 | Lower | -10 | Lower |
| Halstead Volume | **2409.5122 ✅** | 2917.5242 | 2538.0485 | -508.0120 | Lower | -128.5363 | Lower | +379.4756 | Higher |
| Halstead Effort | **114402.6562 ✅** | 153170.0204 | 145495.1087 | -38767.3642 | Lower | -31092.4524 | Lower | +7674.9117 | Higher |
| SLOC | 93 | 93 | 101 | 0 | Same | -8 | Lower | -8 | Lower |
| PLOC | 66 | **65 ✅** | 76 | +1 | Higher | -10 | Lower | -11 | Lower |
| LLOC | 45 | **43 ✅** | 56 | +2 | Higher | -11 | Lower | -13 | Lower |
| Comments (CLOC) | **14 ✅** | 11 | 11 | +3 | Higher | +3 | Higher | 0 | Same |
| Blank Lines | 14 | 19 | 14 | -5 | Lower | 0 | Same | +5 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p00023

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.6179 ✅** | 63.1883 | 62.4670 | +0.4296 | Higher | +1.1509 | Higher | +0.7213 | Higher |
| MI (SEI) | 30.7300 | **48.9995 ✅** | 38.8887 | -18.2695 | Lower | -8.1587 | Lower | +10.1108 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 11 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 1627.3763 | **1520.4976 ✅** | 1955.1808 | +106.8787 | Higher | -327.8045 | Lower | -434.6832 | Lower |
| Halstead Effort | 44585.3246 | **39532.9377 ✅** | 76252.0507 | +5052.3869 | Higher | -31666.7261 | Lower | -36719.1130 | Lower |
| SLOC | 62 | 66 | **61 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| PLOC | 50 | **42 ✅** | 46 | +8 | Higher | +4 | Higher | -4 | Lower |
| LLOC | **23 ✅** | 26 | 30 | -3 | Lower | -7 | Lower | -4 | Lower |
| Comments (CLOC) | 2 | **14 ✅** | 6 | -12 | Lower | -4 | Lower | +8 | Higher |
| Blank Lines | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p00024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **91.4123 ✅** | 79.1542 | 82.4918 | +12.2581 | Higher | +8.9205 | Higher | -3.3376 | Lower |
| MI (SEI) | **89.1075 ✅** | 69.9877 | 74.2630 | +19.1197 | Higher | +14.8445 | Higher | -4.2753 | Lower |
| Cyclomatic (sum) | **3 ✅** | 8 | 5 | -5 | Lower | -2 | Lower | +3 | Higher |
| Cognitive (sum) | **1 ✅** | 9 | 3 | -8 | Lower | -2 | Lower | +6 | Higher |
| Halstead Volume | **403.2542 ✅** | 611.4563 | 613.1154 | -208.2022 | Lower | -209.8612 | Lower | -1.6590 | Lower |
| Halstead Effort | **4870.8857 ✅** | 9936.1655 | 8629.0312 | -5065.2799 | Lower | -3758.1456 | Lower | +1307.1343 | Higher |
| SLOC | **19 ✅** | 33 | 28 | -14 | Lower | -9 | Lower | +5 | Higher |
| PLOC | **11 ✅** | 22 | 17 | -11 | Lower | -6 | Lower | +5 | Higher |
| LLOC | **8 ✅** | 11 | 11 | -3 | Lower | -3 | Lower | 0 | Same |
| Comments (CLOC) | 4 | **6 ✅** | 5 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | **4 ✅** | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.9970 ✅** | 56.9715 | 58.8631 | +6.0255 | Higher | +4.1339 | Higher | -1.8916 | Lower |
| MI (SEI) | 37.6923 | 32.0893 | **38.0138 ✅** | +5.6030 | Higher | -0.3215 | Lower | -5.9244 | Lower |
| Cyclomatic (sum) | **15 ✅** | 19 | 18 | -4 | Lower | -3 | Lower | +1 | Higher |
| Cognitive (sum) | **21 ✅** | 28 | 23 | -7 | Lower | -2 | Lower | +5 | Higher |
| Halstead Volume | **1273.8787 ✅** | 1764.5538 | 1571.5425 | -490.6750 | Lower | -297.6637 | Lower | +193.0113 | Higher |
| Halstead Effort | **42781.0940 ✅** | 70772.9133 | 62203.3500 | -27991.8193 | Lower | -19422.2560 | Lower | +8569.5634 | Higher |
| SLOC | **64 ✅** | 79 | 74 | -15 | Lower | -10 | Lower | +5 | Higher |
| PLOC | **45 ✅** | 62 | 50 | -17 | Lower | -5 | Lower | +12 | Higher |
| LLOC | **29 ✅** | 48 | 36 | -19 | Lower | -7 | Lower | +12 | Higher |
| Comments (CLOC) | 5 | 8 | **10 ✅** | -3 | Lower | -5 | Lower | -2 | Lower |
| Blank Lines | 14 | **9 ✅** | 14 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.7009 | 53.2448 | **67.5391 ✅** | +9.4561 | Higher | -4.8382 | Lower | -14.2943 | Lower |
| MI (SEI) | **44.1023 ✅** | 24.8742 | 43.6025 | +19.2281 | Higher | +0.4998 | Higher | -18.7283 | Lower |
| Cyclomatic (sum) | 21 | 30 | **12 ✅** | -9 | Lower | +9 | Higher | +18 | Higher |
| Cognitive (sum) | 7 | 18 | **5 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| Halstead Volume | 1405.3799 | 2221.1730 | **1092.8788 ✅** | -815.7931 | Lower | +312.5011 | Higher | +1128.2942 | Higher |
| Halstead Effort | 59728.6454 | 106298.9936 | **29726.3024 ✅** | -46570.3482 | Lower | +30002.3429 | Higher | +76572.6912 | Higher |
| SLOC | 58 | 79 | **53 ✅** | -21 | Lower | +5 | Higher | +26 | Higher |
| PLOC | 41 | 61 | 41 | -20 | Lower | 0 | Same | +20 | Higher |
| LLOC | 17 | 28 | **15 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| Comments (CLOC) | **8 ✅** | 6 | 4 | +2 | Higher | +4 | Higher | +2 | Higher |
| Blank Lines | 9 | 13 | **8 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00077

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.6290 ✅** | 73.6536 | 68.1672 | +3.9754 | Higher | +9.4618 | Higher | +5.4864 | Higher |
| MI (SEI) | **66.8220 ✅** | 60.7262 | 56.0810 | +6.0959 | Higher | +10.7411 | Higher | +4.6452 | Higher |
| Cyclomatic (sum) | 6 | 6 | 8 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 11 | 11 | 19 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | **745.6809 ✅** | 907.5881 | 921.5188 | -161.9072 | Lower | -175.8379 | Lower | -13.9308 | Lower |
| Halstead Effort | **17430.2902 ✅** | 20311.1947 | 26281.7165 | -2880.9045 | Lower | -8851.4263 | Lower | -5970.5218 | Lower |
| SLOC | **35 ✅** | 42 | 57 | -7 | Lower | -22 | Lower | -15 | Lower |
| PLOC | **26 ✅** | 29 | 36 | -3 | Lower | -10 | Lower | -7 | Lower |
| LLOC | **21 ✅** | 23 | 30 | -2 | Lower | -9 | Lower | -7 | Lower |
| Comments (CLOC) | 6 | 7 | **12 ✅** | -1 | Lower | -6 | Lower | -5 | Lower |
| Blank Lines | **5 ✅** | 6 | 11 | -1 | Lower | -6 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00118

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.3462 | **46.2825 ✅** | 45.1606 | -1.9362 | Lower | -0.8144 | Lower | +1.1219 | Higher |
| MI (SEI) | 17.2650 | 19.6646 | **21.3260 ✅** | -2.3995 | Lower | -4.0610 | Lower | -1.6615 | Lower |
| Cyclomatic (sum) | 32 | 32 | **31 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 48 | 44 | **39 ✅** | +4 | Higher | +9 | Higher | +5 | Higher |
| Halstead Volume | **3494.0674 ✅** | 3498.8394 | 3690 | -4.7720 | Lower | -195.9326 | Lower | -191.1606 | Lower |
| Halstead Effort | 285975.9758 | **241876.2895 ✅** | 316854.4737 | +44099.6863 | Higher | -30878.4979 | Lower | -74978.1842 | Lower |
| SLOC | 115 | **102 ✅** | 109 | +13 | Higher | +6 | Higher | -7 | Lower |
| PLOC | 88 | **79 ✅** | 80 | +9 | Higher | +8 | Higher | -1 | Lower |
| LLOC | 61 | 61 | **60 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 14 | 12 | **17 ✅** | +2 | Higher | -3 | Lower | -5 | Lower |
| Blank Lines | 13 | **12 ✅** | 13 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00124

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.9108 ✅** | 66.2211 | 63.2370 | +0.6897 | Higher | +3.6737 | Higher | +2.9840 | Higher |
| MI (SEI) | **44.7531 ✅** | 43.3691 | 42.6882 | +1.3840 | Higher | +2.0649 | Higher | +0.6809 | Higher |
| Cyclomatic (sum) | 10 | 10 | 11 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 5 | 5 | 11 | 0 | Same | -6 | Lower | -6 | Lower |
| Halstead Volume | **1347.3063 ✅** | 1370.7292 | 1451.8387 | -23.4228 | Lower | -104.5324 | Lower | -81.1096 | Lower |
| Halstead Effort | **30988.0459 ✅** | 31063.1415 | 37107.2899 | -75.0957 | Lower | -6119.2440 | Lower | -6044.1484 | Lower |
| SLOC | **53 ✅** | 55 | 64 | -2 | Lower | -11 | Lower | -9 | Lower |
| PLOC | 40 | 40 | 45 | 0 | Same | -5 | Lower | -5 | Lower |
| LLOC | 26 | 26 | 29 | 0 | Same | -3 | Lower | -3 | Lower |
| Comments (CLOC) | 5 | 5 | **8 ✅** | 0 | Same | -3 | Lower | -3 | Lower |
| Blank Lines | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00136

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.6851 ✅** | 68.3852 | 58.1079 | +1.2999 | Higher | +11.5773 | Higher | +10.2774 | Higher |
| MI (SEI) | 45.9490 | **53.1168 ✅** | 27.9581 | -7.1678 | Lower | +17.9909 | Higher | +25.1587 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 17 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | **9 ✅** | 10 | 12 | -1 | Lower | -3 | Lower | -2 | Lower |
| Halstead Volume | 1387.5263 | **1164 ✅** | 2258.3879 | +223.5263 | Higher | -870.8616 | Lower | -1094.3879 | Lower |
| Halstead Effort | 33686.0559 | **22201.1707 ✅** | 84348.1631 | +11484.8852 | Higher | -50662.1072 | Lower | -62146.9924 | Lower |
| SLOC | **43 ✅** | 50 | 70 | -7 | Lower | -27 | Lower | -20 | Lower |
| PLOC | **35 ✅** | 36 | 55 | -1 | Lower | -20 | Lower | -19 | Lower |
| LLOC | 31 | **17 ✅** | 45 | +14 | Higher | -14 | Lower | -28 | Lower |
| Comments (CLOC) | 3 | **8 ✅** | 4 | -5 | Lower | -1 | Lower | +4 | Higher |
| Blank Lines | **5 ✅** | 6 | 13 | -1 | Lower | -8 | Lower | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00146

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.6955 | **70.7367 ✅** | 47.6114 | -24.0413 | Lower | -0.9159 | Lower | +23.1253 | Higher |
| MI (SEI) | 21.2218 | **55.8500 ✅** | 20.2333 | -34.6282 | Lower | +0.9885 | Higher | +35.6167 | Higher |
| Cyclomatic (sum) | 21 | **8 ✅** | 22 | +13 | Higher | -1 | Lower | -14 | Lower |
| Cognitive (sum) | 38 | **6 ✅** | 36 | +32 | Higher | +2 | Higher | -30 | Lower |
| Halstead Volume | 3717.4467 | **1174.1920 ✅** | 3853.0614 | +2543.2547 | Higher | -135.6146 | Lower | -2678.8693 | Lower |
| Halstead Effort | 241283.3358 | **28059.1410 ✅** | 242269.6847 | +213224.1948 | Higher | -986.3489 | Lower | -214210.5437 | Lower |
| SLOC | 114 | **45 ✅** | 105 | +69 | Higher | +9 | Higher | -60 | Lower |
| PLOC | 85 | **32 ✅** | 80 | +53 | Higher | +5 | Higher | -48 | Lower |
| LLOC | 72 | **22 ✅** | 66 | +50 | Higher | +6 | Higher | -44 | Lower |
| Comments (CLOC) | **16 ✅** | 7 | 12 | +9 | Higher | +4 | Higher | -5 | Lower |
| Blank Lines | 13 | **6 ✅** | 13 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p00167

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.9747 ✅** | 67.8951 | 67.8945 | +3.0796 | Higher | +3.0802 | Higher | +0.0006 | Higher |
| MI (SEI) | 52.3997 | 50.6204 | **53.5121 ✅** | +1.7793 | Higher | -1.1124 | Lower | -2.8918 | Lower |
| Cyclomatic (sum) | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 13 | 16 | 13 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | **1026.7090 ✅** | 1150.4906 | 1083.0835 | -123.7816 | Lower | -56.3746 | Lower | +67.4070 | Higher |
| Halstead Effort | **25301.0429 ✅** | 32117.8621 | 28346.9108 | -6816.8192 | Lower | -3045.8679 | Lower | +3770.9514 | Higher |
| SLOC | **45 ✅** | 51 | 52 | -6 | Lower | -7 | Lower | -1 | Lower |
| PLOC | **35 ✅** | 39 | 38 | -4 | Lower | -3 | Lower | +1 | Higher |
| LLOC | **25 ✅** | 27 | 26 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 5 | 7 | **9 ✅** | -2 | Lower | -4 | Lower | -2 | Lower |
| Blank Lines | **7 ✅** | 8 | 8 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00187

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.0028 ✅** | 39.4090 | 38.3676 | +8.5938 | Higher | +9.6352 | Higher | +1.0413 | Higher |
| MI (SEI) | **13.5641 ✅** | 1.0032 | -5.8241 | +12.5609 | Higher | +19.3882 | Higher | +6.8273 | Higher |
| Cyclomatic (sum) | **18 ✅** | 31 | 33 | -13 | Lower | -15 | Lower | -2 | Lower |
| Cognitive (sum) | 21 | 21 | 28 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | **4394.4196 ✅** | 4584.1976 | 5471.4589 | -189.7780 | Lower | -1077.0393 | Lower | -887.2613 | Lower |
| Halstead Effort | 259843.9403 | **211343.2614 ✅** | 384628.7713 | +48500.6789 | Higher | -124784.8310 | Lower | -173285.5099 | Lower |
| SLOC | **104 ✅** | 145 | 142 | -41 | Lower | -38 | Lower | +3 | Higher |
| PLOC | **84 ✅** | 116 | 122 | -32 | Lower | -38 | Lower | -6 | Lower |
| LLOC | 60 | **42 ✅** | 81 | +18 | Higher | -21 | Lower | -39 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 3 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | **14 ✅** | 22 | 17 | -8 | Lower | -3 | Lower | +5 | Higher |
| Functions | 2 | 6 | 2 | -4 | Lower | 0 | Same | +4 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 9 | 2 | -7 | Lower | 0 | Same | +7 | Higher |

## p00208

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.1227 ✅** | 67.7476 | 66.4621 | +0.3751 | Higher | +1.6606 | Higher | +1.2855 | Higher |
| MI (SEI) | 54.8633 | **57.9682 ✅** | 45.4838 | -3.1049 | Lower | +9.3795 | Higher | +12.4844 | Higher |
| Cyclomatic (sum) | 11 | 12 | **10 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 20 | 18 | **9 ✅** | +2 | Higher | +11 | Higher | +9 | Higher |
| Halstead Volume | **1021.0366 ✅** | 1049.9202 | 1237.2110 | -28.8836 | Lower | -216.1744 | Lower | -187.2908 | Lower |
| Halstead Effort | 35572.9138 | **34973.2040 ✅** | 42705.1100 | +599.7098 | Higher | -7132.1962 | Lower | -7731.9060 | Lower |
| SLOC | 53 | 53 | 56 | 0 | Same | -3 | Lower | -3 | Lower |
| PLOC | 38 | **34 ✅** | 43 | +4 | Higher | -5 | Lower | -9 | Lower |
| LLOC | 32 | **26 ✅** | 40 | +6 | Higher | -8 | Lower | -14 | Lower |
| Comments (CLOC) | 10 | **13 ✅** | 6 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00252

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.4480 | **86.1076 ✅** | 77.8052 | -14.6596 | Lower | -6.3572 | Lower | +8.3024 | Higher |
| MI (SEI) | 57.0090 | **86.1724 ✅** | 63.6988 | -29.1634 | Lower | -6.6898 | Lower | +22.4736 | Higher |
| Cyclomatic (sum) | 23 | **8 ✅** | 20 | +15 | Higher | +3 | Higher | -12 | Lower |
| Cognitive (sum) | 9 | **3 ✅** | 6 | +6 | Higher | +3 | Higher | -3 | Lower |
| Halstead Volume | 1057.0804 | **656.3911 ✅** | 979.6931 | +400.6893 | Higher | +77.3874 | Higher | -323.3019 | Lower |
| Halstead Effort | 32165.4478 | **9752.0967 ✅** | 28050.1591 | +22413.3511 | Higher | +4115.2887 | Higher | -18298.0624 | Lower |
| SLOC | 36 | **21 ✅** | 26 | +15 | Higher | +10 | Higher | -5 | Lower |
| PLOC | 29 | **11 ✅** | 20 | +18 | Higher | +9 | Higher | -9 | Lower |
| LLOC | 12 | **8 ✅** | 12 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 3 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 3 | 3 | -3 | Lower | -3 | Lower | 0 | Same |
| Exits | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |

## p00261

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.5300 | **82.5139 ✅** | 65.8094 | -17.9840 | Lower | -1.2794 | Lower | +16.7046 | Higher |
| MI (SEI) | 46.1487 | **80.9065 ✅** | 46.6595 | -34.7578 | Lower | -0.5108 | Lower | +34.2470 | Higher |
| Cyclomatic (sum) | 11 | **5 ✅** | 8 | +6 | Higher | +3 | Higher | -3 | Lower |
| Cognitive (sum) | 15 | **6 ✅** | 14 | +9 | Higher | +1 | Higher | -8 | Lower |
| Halstead Volume | 1189.1701 | **444.6136 ✅** | 1173.9971 | +744.5566 | Higher | +15.1731 | Higher | -729.3835 | Lower |
| Halstead Effort | 21725.2237 | **5335.3626 ✅** | 19688.2478 | +16389.8611 | Higher | +2036.9759 | Higher | -14352.8852 | Lower |
| SLOC | 63 | **31 ✅** | 61 | +32 | Higher | +2 | Higher | -30 | Lower |
| PLOC | 47 | **18 ✅** | 44 | +29 | Higher | +3 | Higher | -26 | Lower |
| LLOC | 21 | **11 ✅** | 21 | +10 | Higher | 0 | Same | -10 | Lower |
| Comments (CLOC) | 9 | 9 | 8 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 8 | **4 ✅** | 9 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00270

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **36.1235 ✅** | 30.1056 | 17.9789 | +6.0179 | Higher | +18.1446 | Higher | +12.1267 | Higher |
| MI (SEI) | **-1.8669 ✅** | -6.2222 | -33.2237 | +4.3553 | Higher | +31.3568 | Higher | +27.0015 | Higher |
| Cyclomatic (sum) | **30 ✅** | 48 | 55 | -18 | Lower | -25 | Lower | -7 | Lower |
| Cognitive (sum) | **52 ✅** | 59 | 67 | -7 | Lower | -15 | Lower | -8 | Lower |
| Halstead Volume | **6141.4764 ✅** | 7072.3672 | 10580.6800 | -930.8908 | Lower | -4439.2035 | Lower | -3508.3128 | Lower |
| Halstead Effort | **402598.6775 ✅** | 610354.9788 | 1071293.8467 | -207756.3013 | Lower | -668695.1692 | Lower | -460938.8679 | Lower |
| SLOC | **164 ✅** | 176 | 296 | -12 | Lower | -132 | Lower | -120 | Lower |
| PLOC | **133 ✅** | 144 | 266 | -11 | Lower | -133 | Lower | -122 | Lower |
| LLOC | **99 ✅** | 111 | 206 | -12 | Lower | -107 | Lower | -95 | Lower |
| Comments (CLOC) | 10 | **14 ✅** | 6 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 21 | **18 ✅** | 24 | +3 | Higher | -3 | Lower | -6 | Lower |
| Functions | 3 | **1 ✅** | 10 | +2 | Higher | -7 | Lower | -9 | Lower |
| Closures | 0 | 7 | 0 | -7 | Lower | 0 | Same | +7 | Higher |
| Exits | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |

## p00271

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 90.1522 | 86.9419 | **90.3083 ✅** | +3.2103 | Higher | -0.1561 | Lower | -3.3664 | Lower |
| MI (SEI) | 72.0655 | 66.3529 | **89.8633 ✅** | +5.7126 | Higher | -17.7978 | Lower | -23.5104 | Lower |
| Cyclomatic (sum) | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 513.8337 | 577.2720 | **425 ✅** | -63.4383 | Lower | +88.8337 | Higher | +152.2720 | Higher |
| Halstead Effort | 6084.8733 | 7944.3627 | **4131.9444 ✅** | -1859.4893 | Lower | +1952.9289 | Higher | +3812.4182 | Higher |
| SLOC | **19 ✅** | 22 | 20 | -3 | Lower | -1 | Lower | +2 | Higher |
| PLOC | 14 | 17 | **12 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| LLOC | 10 | 12 | **8 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Comments (CLOC) | 1 | 1 | **5 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00283

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.0055 | 45.7213 | **47.7837 ✅** | -5.7158 | Lower | -7.7782 | Lower | -2.0624 | Lower |
| MI (SEI) | 7.1058 | **21.7659 ✅** | 15.9525 | -14.6601 | Lower | -8.8467 | Lower | +5.8134 | Higher |
| Cyclomatic (sum) | 37 | 28 | **23 ✅** | +9 | Higher | +14 | Higher | +5 | Higher |
| Cognitive (sum) | 29 | **26 ✅** | 27 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | 3827.2463 | **3116.0895 ✅** | 3174.1281 | +711.1568 | Higher | +653.1182 | Higher | -58.0386 | Lower |
| Halstead Effort | 259640.3895 | **139445.0045 ✅** | 178544.7068 | +120195.3849 | Higher | +81095.6827 | Higher | -39099.7023 | Lower |
| SLOC | 136 | 116 | **109 ✅** | +20 | Higher | +27 | Higher | +7 | Higher |
| PLOC | 110 | 87 | **85 ✅** | +23 | Higher | +25 | Higher | +2 | Higher |
| LLOC | 55 | **46 ✅** | 56 | +9 | Higher | -1 | Lower | -10 | Lower |
| Comments (CLOC) | 11 | **18 ✅** | 8 | -7 | Lower | +3 | Higher | +10 | Higher |
| Blank Lines | 18 | **12 ✅** | 17 | +6 | Higher | +1 | Higher | -5 | Lower |
| Functions | 3 | **1 ✅** | 2 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 8 | 3 | **0 ✅** | +5 | Higher | +8 | Higher | +3 | Higher |
| Exits | 4 | **0 ✅** | 3 | +4 | Higher | +1 | Higher | -3 | Lower |

## p00292

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.7001 ✅** | 77.3037 | 76.6058 | +4.3964 | Higher | +5.0943 | Higher | +0.6979 | Higher |
| MI (SEI) | **75.5381 ✅** | 71.4072 | 72.3419 | +4.1309 | Higher | +3.1962 | Higher | -0.9347 | Lower |
| Cyclomatic (sum) | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **713.9409 ✅** | 727.1195 | 734.4261 | -13.1786 | Lower | -20.4852 | Lower | -7.3066 | Lower |
| Halstead Effort | 11137.4781 | 12645.5561 | **11128.1516 ✅** | -1508.0780 | Lower | +9.3264 | Higher | +1517.4045 | Higher |
| SLOC | **28 ✅** | 36 | 38 | -8 | Lower | -10 | Lower | -2 | Lower |
| PLOC | **18 ✅** | 21 | 20 | -3 | Lower | -2 | Lower | +1 | Higher |
| LLOC | **11 ✅** | 13 | 15 | -2 | Lower | -4 | Lower | -2 | Lower |
| Comments (CLOC) | 6 | 9 | **11 ✅** | -3 | Lower | -5 | Lower | -2 | Lower |
| Blank Lines | **5 ✅** | 6 | 7 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00315

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.2081 | **55.1762 ✅** | 54.3032 | -0.9680 | Lower | -0.0951 | Lower | +0.8729 | Higher |
| MI (SEI) | 28.4472 | **40.7621 ✅** | 26.5202 | -12.3150 | Lower | +1.9270 | Higher | +14.2420 | Higher |
| Cyclomatic (sum) | **15 ✅** | 17 | 16 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 20 | 31 | **19 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| Halstead Volume | 3072.1219 | **2016 ✅** | 3366 | +1056.1219 | Higher | -293.8781 | Lower | -1350 | Lower |
| Halstead Effort | 197405.7784 | **94377.6000 ✅** | 215335.4211 | +103028.1784 | Higher | -17929.6426 | Lower | -120957.8211 | Lower |
| SLOC | 83 | 87 | **79 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| PLOC | 61 | **52 ✅** | 59 | +9 | Higher | +2 | Higher | -7 | Lower |
| LLOC | 51 | **41 ✅** | 52 | +10 | Higher | -1 | Lower | -11 | Lower |
| Comments (CLOC) | 9 | **22 ✅** | 7 | -13 | Lower | +2 | Higher | +15 | Higher |
| Blank Lines | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **26.9491 ✅** | 17.0835 | 25.4224 | +9.8656 | Higher | +1.5267 | Higher | -8.3389 | Lower |
| MI (SEI) | -13.0303 | -18.8790 | **-12.4707 ✅** | +5.8488 | Higher | -0.5596 | Lower | -6.4083 | Lower |
| Cyclomatic (sum) | **47 ✅** | 66 | 52 | -19 | Lower | -5 | Lower | +14 | Higher |
| Cognitive (sum) | **81 ✅** | 131 | 97 | -50 | Lower | -16 | Lower | +34 | Higher |
| Halstead Volume | **6673.4297 ✅** | 8599.9082 | 6879.7175 | -1926.4784 | Lower | -206.2878 | Lower | +1720.1907 | Higher |
| Halstead Effort | **532450.7145 ✅** | 804091.4137 | 575283.7192 | -271640.6992 | Lower | -42833.0046 | Lower | +228807.6945 | Higher |
| SLOC | **221 ✅** | 286 | 224 | -65 | Lower | -3 | Lower | +62 | Higher |
| PLOC | 189 | 234 | **188 ✅** | -45 | Lower | +1 | Higher | +46 | Higher |
| LLOC | 150 | 167 | **144 ✅** | -17 | Lower | +6 | Higher | +23 | Higher |
| Comments (CLOC) | 14 | **34 ✅** | 18 | -20 | Lower | -4 | Lower | +16 | Higher |
| Blank Lines | 19 | 19 | 21 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |

## p00320

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.0994 | **51.6687 ✅** | 46.9304 | -5.5692 | Lower | -0.8309 | Lower | +4.7383 | Higher |
| MI (SEI) | 13.1987 | **31.7907 ✅** | 12.7681 | -18.5921 | Lower | +0.4306 | Higher | +19.0226 | Higher |
| Cyclomatic (sum) | 24 | 21 | **20 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Cognitive (sum) | 25 | 23 | **22 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Halstead Volume | 3650.8728 | **2288.2724 ✅** | 3713.9364 | +1362.6004 | Higher | -63.0636 | Lower | -1425.6640 | Lower |
| Halstead Effort | 240729.4248 | **106328.3914 ✅** | 260740.1836 | +134401.0334 | Higher | -20010.7588 | Lower | -154411.7922 | Lower |
| SLOC | 114 | **98 ✅** | 114 | +16 | Higher | 0 | Same | -16 | Lower |
| PLOC | 92 | **69 ✅** | 94 | +23 | Higher | -2 | Lower | -25 | Lower |
| LLOC | 55 | **47 ✅** | 64 | +8 | Higher | -9 | Lower | -17 | Lower |
| Comments (CLOC) | 8 | **18 ✅** | 7 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |

## p00336

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.7329 | 70.1761 | **74.3959 ✅** | +2.5568 | Higher | -1.6630 | Lower | -4.2198 | Lower |
| MI (SEI) | **59.7061 ✅** | 53.3598 | 49.8628 | +6.3464 | Higher | +9.8433 | Higher | +3.4970 | Higher |
| Cyclomatic (sum) | 6 | 10 | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 1167.8596 | 1197.1354 | 1167.8596 | -29.2757 | Lower | 0 | Same | +29.2757 | Higher |
| Halstead Effort | 36787.5789 | 39629.3090 | 36787.5789 | -2841.7301 | Lower | 0 | Same | +2841.7301 | Higher |
| SLOC | 41 | 45 | **37 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| PLOC | 26 | 32 | 26 | -6 | Lower | 0 | Same | +6 | Higher |
| LLOC | 21 | 22 | 21 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | **7 ✅** | 6 | 2 | +1 | Higher | +5 | Higher | +4 | Higher |
| Blank Lines | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00378

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.8013 | **79.6809 ✅** | 78.7479 | -1.8797 | Lower | -0.9466 | Lower | +0.9331 | Higher |
| MI (SEI) | 62.8329 | **83.6503 ✅** | 57.6902 | -20.8174 | Lower | +5.1428 | Higher | +25.9601 | Higher |
| Cyclomatic (sum) | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 9 | **1 ✅** | 9 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 866.5159 | **573.8617 ✅** | 877.6250 | +292.6542 | Higher | -11.1092 | Lower | -303.7634 | Lower |
| Halstead Effort | 22604.7616 | **8215.2829 ✅** | 23352.4575 | +14389.4787 | Higher | -747.6960 | Lower | -15137.1746 | Lower |
| SLOC | 33 | 35 | **31 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| PLOC | 25 | **14 ✅** | 25 | +11 | Higher | 0 | Same | -11 | Lower |
| LLOC | 21 | **11 ✅** | 21 | +10 | Higher | 0 | Same | -10 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 2 | -13 | Lower | +2 | Higher | +15 | Higher |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00428

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.4022 | **66.0843 ✅** | 59.3886 | -3.6821 | Lower | +3.0136 | Higher | +6.6957 | Higher |
| MI (SEI) | 40.5414 | **48.9391 ✅** | 37.8090 | -8.3977 | Lower | +2.7324 | Higher | +11.1301 | Higher |
| Cyclomatic (sum) | 15 | **12 ✅** | 16 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 24 | **9 ✅** | 27 | +15 | Higher | -3 | Lower | -18 | Lower |
| Halstead Volume | 1500.0740 | **1363.9083 ✅** | 1765.4359 | +136.1656 | Higher | -265.3619 | Lower | -401.5276 | Lower |
| Halstead Effort | 47980.3074 | **38189.4335 ✅** | 65829.3597 | +9790.8739 | Higher | -17849.0523 | Lower | -27639.9262 | Lower |
| SLOC | 63 | **54 ✅** | 71 | +9 | Higher | -8 | Lower | -17 | Lower |
| PLOC | 51 | **39 ✅** | 56 | +12 | Higher | -5 | Lower | -17 | Lower |
| LLOC | 34 | **26 ✅** | 44 | +8 | Higher | -10 | Lower | -18 | Lower |
| Comments (CLOC) | 7 | 8 | **9 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | **7 ✅** | 9 | 8 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.2410 | 79.6525 | **83.8251 ✅** | +2.5885 | Higher | -1.5841 | Lower | -4.1726 | Lower |
| MI (SEI) | 75.9476 | 69.1680 | **81.2181 ✅** | +6.7796 | Higher | -5.2705 | Lower | -12.0501 | Lower |
| Cyclomatic (sum) | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 551.8279 | 675.0516 | **531.3642 ✅** | -123.2237 | Lower | +20.4636 | Higher | +143.6873 | Higher |
| Halstead Effort | 7557.6424 | 10758.6341 | **7084.8566 ✅** | -3200.9917 | Lower | +472.7858 | Higher | +3673.7775 | Higher |
| SLOC | 29 | 31 | **27 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| PLOC | 19 | 22 | **16 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| LLOC | **8 ✅** | 13 | 12 | -5 | Lower | -4 | Lower | +1 | Higher |
| Comments (CLOC) | 6 | 5 | **7 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00445

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.6419 ✅** | 75.5223 | 77.0203 | +4.1196 | Higher | +2.6217 | Higher | -1.4979 | Lower |
| MI (SEI) | 59.0819 | 63.9312 | **67.6898 ✅** | -4.8494 | Lower | -8.6080 | Lower | -3.7586 | Lower |
| Cyclomatic (sum) | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **707.0159 ✅** | 820.9895 | 734.6337 | -113.9736 | Lower | -27.6178 | Lower | +86.3557 | Higher |
| Halstead Effort | **15230.3015 ✅** | 20145.8190 | 16498.6494 | -4915.5175 | Lower | -1268.3479 | Lower | +3647.1696 | Higher |
| SLOC | **31 ✅** | 36 | 36 | -5 | Lower | -5 | Lower | 0 | Same |
| PLOC | 23 | 25 | 23 | -2 | Lower | 0 | Same | +2 | Higher |
| LLOC | 17 | 19 | 17 | -2 | Lower | 0 | Same | +2 | Higher |
| Comments (CLOC) | 2 | 6 | **7 ✅** | -4 | Lower | -5 | Lower | -1 | Lower |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.8359 | **75.9528 ✅** | 64.0852 | -10.1169 | Lower | +1.7506 | Higher | +11.8675 | Higher |
| MI (SEI) | 46.5393 | **66.8623 ✅** | 48.2316 | -20.3231 | Lower | -1.6923 | Lower | +18.6308 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 13 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 13 | **5 ✅** | 15 | +8 | Higher | -2 | Lower | -10 | Lower |
| Halstead Volume | 1277.4029 | **702.9585 ✅** | 1185.6795 | +574.4444 | Higher | +91.7234 | Higher | -482.7210 | Lower |
| Halstead Effort | 42537.5170 | **12050.7171 ✅** | 37581.8067 | +30486.7999 | Higher | +4955.7103 | Higher | -25531.0896 | Lower |
| SLOC | 56 | **39 ✅** | 63 | +17 | Higher | -7 | Lower | -24 | Lower |
| PLOC | 42 | **27 ✅** | 46 | +15 | Higher | -4 | Lower | -19 | Lower |
| LLOC | 34 | **20 ✅** | 37 | +14 | Higher | -3 | Lower | -17 | Lower |
| Comments (CLOC) | 7 | 8 | **11 ✅** | -1 | Lower | -4 | Lower | -3 | Lower |
| Blank Lines | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00492

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7164 | 47.4928 | **52.9511 ✅** | +4.2236 | Higher | -1.2347 | Lower | -5.4583 | Lower |
| MI (SEI) | 28.8870 | 28.6173 | **29.5932 ✅** | +0.2697 | Higher | -0.7062 | Lower | -0.9759 | Lower |
| Cyclomatic (sum) | 18 | 28 | 18 | -10 | Lower | 0 | Same | +10 | Higher |
| Cognitive (sum) | 27 | 45 | 27 | -18 | Lower | 0 | Same | +18 | Higher |
| Halstead Volume | 2948.0303 | 3308.8633 | **2855.3029 ✅** | -360.8330 | Lower | +92.7274 | Higher | +453.5604 | Higher |
| Halstead Effort | 164608.3830 | 207379.4103 | **161076.3252 ✅** | -42771.0273 | Lower | +3532.0579 | Higher | +46303.0852 | Higher |
| SLOC | 94 | 102 | **88 ✅** | -8 | Lower | +6 | Higher | +14 | Higher |
| PLOC | 67 | 70 | **66 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| LLOC | 55 | **51 ✅** | 60 | +4 | Higher | -5 | Lower | -9 | Lower |
| Comments (CLOC) | 14 | **22 ✅** | 12 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 13 | 11 | 11 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00518

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.6167 | 54.1796 | **56.9881 ✅** | +2.4371 | Higher | -0.3714 | Lower | -2.8085 | Lower |
| MI (SEI) | 32.7820 | **38.9315 ✅** | 30.8633 | -6.1495 | Lower | +1.9187 | Higher | +8.0681 | Higher |
| Cyclomatic (sum) | 18 | 21 | 18 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 29 | 33 | 29 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 1974.6062 | **1974.3451 ✅** | 1991.3790 | +0.2611 | Higher | -16.7728 | Lower | -17.0339 | Lower |
| Halstead Effort | **68628.0676 ✅** | 78544.5983 | 82957.9338 | -9916.5307 | Lower | -14329.8662 | Lower | -4413.3355 | Lower |
| SLOC | 79 | 88 | **77 ✅** | -9 | Lower | +2 | Higher | +11 | Higher |
| PLOC | 60 | 60 | **58 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| LLOC | 51 | **42 ✅** | 50 | +9 | Higher | +1 | Higher | -8 | Lower |
| Comments (CLOC) | 9 | **21 ✅** | 7 | -12 | Lower | +2 | Higher | +14 | Higher |
| Blank Lines | 13 | **12 ✅** | 14 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00528

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.4445 | **47.3663 ✅** | 18.5887 | -11.9218 | Lower | +16.8558 | Higher | +28.7776 | Higher |
| MI (SEI) | -0.5931 | **18.8320 ✅** | -26.8558 | -19.4252 | Lower | +26.2626 | Higher | +45.6878 | Higher |
| Cyclomatic (sum) | **32 ✅** | 33 | 54 | -1 | Lower | -22 | Lower | -21 | Lower |
| Cognitive (sum) | 41 | **29 ✅** | 55 | +12 | Higher | -14 | Lower | -26 | Lower |
| Halstead Volume | 5623.5580 | **3282.5874 ✅** | 9835.4122 | +2340.9705 | Higher | -4211.8542 | Lower | -6552.8248 | Lower |
| Halstead Effort | 409005.6974 | **157424.5130 ✅** | 1009044.6734 | +251581.1844 | Higher | -600038.9760 | Lower | -851620.1604 | Lower |
| SLOC | 171 | **96 ✅** | 296 | +75 | Higher | -125 | Lower | -200 | Lower |
| PLOC | 137 | **75 ✅** | 254 | +62 | Higher | -117 | Lower | -179 | Lower |
| LLOC | 99 | **40 ✅** | 189 | +59 | Higher | -90 | Lower | -149 | Lower |
| Comments (CLOC) | 13 | 9 | **14 ✅** | +4 | Higher | -1 | Lower | -5 | Lower |
| Blank Lines | 24 | **12 ✅** | 29 | +12 | Higher | -5 | Lower | -17 | Lower |
| Functions | 2 | **1 ✅** | 11 | +1 | Higher | -9 | Lower | -10 | Lower |
| Closures | 3 | 7 | **2 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Exits | **2 ✅** | 6 | 9 | -4 | Lower | -7 | Lower | -3 | Lower |

## p00554

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.5574 | **64.1110 ✅** | 60.3316 | -2.5537 | Lower | +1.2258 | Higher | +3.7794 | Higher |
| MI (SEI) | 44.0895 | **61.2850 ✅** | 40.1936 | -17.1955 | Lower | +3.8959 | Higher | +21.0914 | Higher |
| Cyclomatic (sum) | 14 | 12 | 12 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 19 | **12 ✅** | 18 | +7 | Higher | +1 | Higher | -6 | Lower |
| Halstead Volume | 1595.6553 | **972.0530 ✅** | 1837.0430 | +623.6022 | Higher | -241.3877 | Lower | -864.9899 | Lower |
| Halstead Effort | 66485.6367 | **19801.0806 ✅** | 74137.8058 | +46684.5561 | Higher | -7652.1691 | Lower | -54336.7252 | Lower |
| SLOC | **66 ✅** | 68 | 70 | -2 | Lower | -4 | Lower | -2 | Lower |
| PLOC | 46 | **30 ✅** | 50 | +16 | Higher | -4 | Lower | -20 | Lower |
| LLOC | 36 | **16 ✅** | 45 | +20 | Higher | -9 | Lower | -29 | Lower |
| Comments (CLOC) | 11 | **31 ✅** | 10 | -20 | Lower | +1 | Higher | +21 | Higher |
| Blank Lines | 10 | **7 ✅** | 11 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |

## p00594

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.2445 ✅** | 59.5149 | 62.6282 | +4.7295 | Higher | +1.6163 | Higher | -3.1133 | Lower |
| MI (SEI) | **43.4462 ✅** | 40.1805 | 43.4170 | +3.2656 | Higher | +0.0292 | Higher | -3.2365 | Lower |
| Cyclomatic (sum) | **12 ✅** | 17 | 13 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | **11 ✅** | 18 | 19 | -7 | Lower | -8 | Lower | -1 | Lower |
| Halstead Volume | **1399.2361 ✅** | 1511.8393 | 1494 | -112.6031 | Lower | -94.7639 | Lower | +17.8393 | Higher |
| Halstead Effort | **43716.1346 ✅** | 49468.2700 | 49385 | -5752.1354 | Lower | -5668.8654 | Lower | +83.2700 | Higher |
| SLOC | **60 ✅** | 73 | 64 | -13 | Lower | -4 | Lower | +9 | Higher |
| PLOC | **45 ✅** | 57 | 48 | -12 | Lower | -3 | Lower | +9 | Higher |
| LLOC | **37 ✅** | 41 | 38 | -4 | Lower | -1 | Lower | +3 | Higher |
| Comments (CLOC) | 7 | **11 ✅** | 9 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00649

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **46.8288 ✅** | 39.1980 | 37.2207 | +7.6308 | Higher | +9.6082 | Higher | +1.9774 | Higher |
| MI (SEI) | **12.8495 ✅** | 9.3499 | 2.0402 | +3.4996 | Higher | +10.8093 | Higher | +7.3097 | Higher |
| Cyclomatic (sum) | **23 ✅** | 33 | 31 | -10 | Lower | -8 | Lower | +2 | Higher |
| Cognitive (sum) | **27 ✅** | 46 | 32 | -19 | Lower | -5 | Lower | +14 | Higher |
| Halstead Volume | **3227.5357 ✅** | 4187.2452 | 4413.9322 | -959.7095 | Lower | -1186.3965 | Lower | -226.6870 | Lower |
| Halstead Effort | **122707.2551 ✅** | 197398.7045 | 229007.2166 | -74691.4494 | Lower | -106299.9615 | Lower | -31608.5121 | Lower |
| SLOC | **115 ✅** | 147 | 168 | -32 | Lower | -53 | Lower | -21 | Lower |
| PLOC | **92 ✅** | 115 | 136 | -23 | Lower | -44 | Lower | -21 | Lower |
| LLOC | **62 ✅** | 71 | 82 | -9 | Lower | -20 | Lower | -11 | Lower |
| Comments (CLOC) | 7 | **17 ✅** | 13 | -10 | Lower | -6 | Lower | +4 | Higher |
| Blank Lines | 16 | **15 ✅** | 22 | +1 | Higher | -6 | Lower | -7 | Lower |
| Functions | **2 ✅** | 4 | 6 | -2 | Lower | -4 | Lower | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **0 ✅** | 3 | 6 | -3 | Lower | -6 | Lower | -3 | Lower |

## p00653

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 26.4805 | 25.0493 | **32.8773 ✅** | +1.4311 | Higher | -6.3968 | Lower | -7.8279 | Lower |
| MI (SEI) | -13.1937 | -15.4361 | **-9.3205 ✅** | +2.2425 | Higher | -3.8732 | Lower | -6.1156 | Lower |
| Cyclomatic (sum) | 50 | 58 | **33 ✅** | -8 | Lower | +17 | Higher | +25 | Higher |
| Cognitive (sum) | 54 | 56 | **30 ✅** | -2 | Lower | +24 | Higher | +26 | Higher |
| Halstead Volume | 6867.9507 | 7248.7254 | **6245.3901 ✅** | -380.7747 | Lower | +622.5606 | Higher | +1003.3353 | Higher |
| Halstead Effort | 737617.9064 | 654834.1808 | **567194.9755 ✅** | +82783.7256 | Higher | +170422.9309 | Higher | +87639.2053 | Higher |
| SLOC | 216 | 207 | **191 ✅** | +9 | Higher | +25 | Higher | +16 | Higher |
| PLOC | 172 | 173 | **156 ✅** | -1 | Lower | +16 | Higher | +17 | Higher |
| LLOC | 118 | **113 ✅** | 119 | +5 | Higher | -1 | Lower | -6 | Lower |
| Comments (CLOC) | **14 ✅** | 12 | 8 | +2 | Higher | +6 | Higher | +4 | Higher |
| Blank Lines | 33 | **22 ✅** | 29 | +11 | Higher | +4 | Higher | -7 | Lower |
| Functions | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 8 | 11 | 8 | -3 | Lower | 0 | Same | +3 | Higher |

## p00680

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **17.8970 ✅** | 12.9627 | 16.3821 | +4.9343 | Higher | +1.5149 | Higher | -3.4194 | Lower |
| MI (SEI) | -28.5072 | **-26.1072 ✅** | -30.8574 | -2.4000 | Lower | +2.3502 | Higher | +4.7502 | Higher |
| Cyclomatic (sum) | 62 | 73 | **60 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| Cognitive (sum) | **87 ✅** | 109 | 88 | -22 | Lower | -1 | Lower | +21 | Higher |
| Halstead Volume | **9171.5836 ✅** | 9633.4262 | 10396.1332 | -461.8427 | Lower | -1224.5496 | Lower | -762.7069 | Lower |
| Halstead Effort | **820229.2023 ✅** | 898840.5535 | 925158.6930 | -78611.3512 | Lower | -104929.4907 | Lower | -26318.1395 | Lower |
| SLOC | **282 ✅** | 322 | 306 | -40 | Lower | -24 | Lower | +16 | Higher |
| PLOC | **241 ✅** | 254 | 262 | -13 | Lower | -21 | Lower | -8 | Lower |
| LLOC | **152 ✅** | 155 | 183 | -3 | Lower | -31 | Lower | -28 | Lower |
| Comments (CLOC) | 11 | **32 ✅** | 12 | -21 | Lower | -1 | Lower | +20 | Higher |
| Blank Lines | **32 ✅** | 36 | 33 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 10 | **8 ✅** | 11 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 10 | 13 | **9 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p00690

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.0835 | 39.3316 | **42.3533 ✅** | +2.7518 | Higher | -0.2699 | Lower | -3.0217 | Lower |
| MI (SEI) | **9.9463 ✅** | 2.8663 | 3.1488 | +7.0800 | Higher | +6.7975 | Higher | -0.2825 | Lower |
| Cyclomatic (sum) | 31 | 36 | **29 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Cognitive (sum) | 40 | 33 | 33 | +7 | Higher | +7 | Higher | 0 | Same |
| Halstead Volume | 3851.6100 | 4254.5569 | **3720.9581 ✅** | -402.9468 | Lower | +130.6519 | Higher | +533.5988 | Higher |
| Halstead Effort | 214632.8574 | 223116.0529 | **201916.6976 ✅** | -8483.1955 | Lower | +12716.1597 | Higher | +21199.3553 | Higher |
| SLOC | **130 ✅** | 139 | 133 | -9 | Lower | -3 | Lower | +6 | Higher |
| PLOC | **106 ✅** | 113 | 117 | -7 | Lower | -11 | Lower | -4 | Lower |
| LLOC | 68 | **66 ✅** | 67 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | **11 ✅** | 8 | 5 | +3 | Higher | +6 | Higher | +3 | Higher |
| Blank Lines | 13 | 18 | **11 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00723

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.4812 ✅** | 50.5882 | 50.2825 | +0.8930 | Higher | +1.1987 | Higher | +0.3057 | Higher |
| MI (SEI) | 21.0192 | **26.7888 ✅** | 19.9906 | -5.7696 | Lower | +1.0287 | Higher | +6.7982 | Higher |
| Cyclomatic (sum) | 19 | 22 | 19 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 29 | **25 ✅** | 29 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 2950.9602 | **2453.2475 ✅** | 3064.5078 | +497.7127 | Higher | -113.5476 | Lower | -611.2603 | Lower |
| Halstead Effort | 199822.1640 | **106470.9426 ✅** | 175030.5427 | +93351.2215 | Higher | +24791.6213 | Higher | -68559.6001 | Lower |
| SLOC | **94 ✅** | 101 | 100 | -7 | Lower | -6 | Lower | +1 | Higher |
| PLOC | **70 ✅** | 71 | 72 | -1 | Lower | -2 | Lower | -1 | Lower |
| LLOC | 50 | **39 ✅** | 48 | +11 | Higher | +2 | Higher | -9 | Lower |
| Comments (CLOC) | 7 | **14 ✅** | 8 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 18 | **16 ✅** | 20 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p00792

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.3295 | **61.5211 ✅** | 48.0566 | -11.1916 | Lower | +2.2729 | Higher | +13.4644 | Higher |
| MI (SEI) | 23.8631 | **55.5264 ✅** | 18.1775 | -31.6633 | Lower | +5.6857 | Higher | +37.3490 | Higher |
| Cyclomatic (sum) | 20 | **15 ✅** | 23 | +5 | Higher | -3 | Lower | -8 | Lower |
| Cognitive (sum) | 22 | **16 ✅** | 25 | +6 | Higher | -3 | Lower | -9 | Lower |
| Halstead Volume | 3194.7583 | **1122.9866 ✅** | 3703.6708 | +2071.7717 | Higher | -508.9126 | Lower | -2580.6842 | Lower |
| Halstead Effort | 192027.7930 | **27176.2763 ✅** | 249406.7708 | +164851.5167 | Higher | -57378.9778 | Lower | -222230.4945 | Lower |
| SLOC | 97 | **73 ✅** | 102 | +24 | Higher | -5 | Lower | -29 | Lower |
| PLOC | 73 | **38 ✅** | 80 | +35 | Higher | -7 | Lower | -42 | Lower |
| LLOC | 70 | **21 ✅** | 74 | +49 | Higher | -4 | Lower | -53 | Lower |
| Comments (CLOC) | 11 | **28 ✅** | 9 | -17 | Lower | +2 | Higher | +19 | Higher |
| Blank Lines | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p00811

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.7891 | 46.7148 | **52.6258 ✅** | +3.0743 | Higher | -2.8367 | Lower | -5.9110 | Lower |
| MI (SEI) | 17.0949 | **20.3750 ✅** | 16.1871 | -3.2801 | Lower | +0.9078 | Higher | +4.1879 | Higher |
| Cyclomatic (sum) | 24 | 25 | **19 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| Cognitive (sum) | 30 | 33 | **23 ✅** | -3 | Lower | +7 | Higher | +10 | Higher |
| Halstead Volume | **2700.9850 ✅** | 2714.6747 | 2807.5204 | -13.6897 | Lower | -106.5354 | Lower | -92.8457 | Lower |
| Halstead Effort | 147893.9342 | **141332.7519 ✅** | 150732.6492 | +6561.1823 | Higher | -2838.7150 | Lower | -9399.8973 | Lower |
| SLOC | 100 | 119 | **89 ✅** | -19 | Lower | +11 | Higher | +30 | Higher |
| PLOC | 78 | 88 | **71 ✅** | -10 | Lower | +7 | Higher | +17 | Higher |
| LLOC | 58 | 68 | **57 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| Comments (CLOC) | 6 | **15 ✅** | 3 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00820

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.8095 ✅** | 58.5159 | 71.3055 | +17.2936 | Higher | +4.5040 | Higher | -12.7895 | Lower |
| MI (SEI) | 52.3380 | 36.4879 | **55.5358 ✅** | +15.8501 | Higher | -3.1978 | Lower | -19.0479 | Lower |
| Cyclomatic (sum) | **8 ✅** | 17 | 10 | -9 | Lower | -2 | Lower | +7 | Higher |
| Cognitive (sum) | **7 ✅** | 31 | 9 | -24 | Lower | -2 | Lower | +22 | Higher |
| Halstead Volume | **887.1242 ✅** | 1912.5116 | 1110.0211 | -1025.3875 | Lower | -222.8969 | Lower | +802.4906 | Higher |
| Halstead Effort | **23542.9103 ✅** | 75631.1420 | 30077.9901 | -52088.2317 | Lower | -6535.0799 | Lower | +45553.1518 | Higher |
| SLOC | **36 ✅** | 72 | 43 | -36 | Lower | -7 | Lower | +29 | Higher |
| PLOC | **29 ✅** | 57 | 33 | -28 | Lower | -4 | Lower | +24 | Higher |
| LLOC | **21 ✅** | 41 | 22 | -20 | Lower | -1 | Lower | +19 | Higher |
| Comments (CLOC) | 2 | **9 ✅** | 6 | -7 | Lower | -4 | Lower | +3 | Higher |
| Blank Lines | **5 ✅** | 8 | 6 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **40.2791 ✅** | 25.9316 | 39.9343 | +14.3474 | Higher | +0.3448 | Higher | -14.0027 | Lower |
| MI (SEI) | **7.5184 ✅** | -9.0662 | -0.1316 | +16.5847 | Higher | +7.6501 | Higher | -8.9346 | Lower |
| Cyclomatic (sum) | 21 | 57 | **20 ✅** | -36 | Lower | +1 | Higher | +37 | Higher |
| Cognitive (sum) | 21 | 55 | 21 | -34 | Lower | 0 | Same | +34 | Higher |
| Halstead Volume | **4618.6819 ✅** | 6900.2540 | 4960.2410 | -2281.5721 | Lower | -341.5591 | Lower | +1940.0130 | Higher |
| Halstead Effort | **242480.7979 ✅** | 523451.5833 | 278809.9634 | -280970.7854 | Lower | -36329.1655 | Lower | +244641.6199 | Higher |
| SLOC | **158 ✅** | 202 | 160 | -44 | Lower | -2 | Lower | +42 | Higher |
| PLOC | **124 ✅** | 160 | 131 | -36 | Lower | -7 | Lower | +29 | Higher |
| LLOC | **59 ✅** | 98 | 65 | -39 | Lower | -6 | Lower | +33 | Higher |
| Comments (CLOC) | 15 | **20 ✅** | 7 | -5 | Lower | +8 | Higher | +13 | Higher |
| Blank Lines | **21 ✅** | 23 | 22 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | 21 | **4 ✅** | -15 | Lower | +2 | Higher | +17 | Higher |

## p00874

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.2992 | **69.8632 ✅** | 64.6961 | -0.5640 | Lower | +4.6030 | Higher | +5.1671 | Higher |
| MI (SEI) | 47.4521 | **48.2884 ✅** | 40.6505 | -0.8363 | Lower | +6.8016 | Higher | +7.6379 | Higher |
| Cyclomatic (sum) | 11 | **9 ✅** | 12 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | **6 ✅** | 8 | 15 | -2 | Lower | -9 | Lower | -7 | Lower |
| Halstead Volume | **1266.0034 ✅** | 1328.8775 | 1351.7892 | -62.8742 | Lower | -85.7859 | Lower | -22.9117 | Lower |
| Halstead Effort | **34712.9957 ✅** | 36167.6172 | 37309.3827 | -1454.6215 | Lower | -2596.3870 | Lower | -1141.7655 | Lower |
| SLOC | 46 | **45 ✅** | 59 | +1 | Higher | -13 | Lower | -14 | Lower |
| PLOC | 36 | **35 ✅** | 45 | +1 | Higher | -9 | Lower | -10 | Lower |
| LLOC | **25 ✅** | 29 | 34 | -4 | Lower | -9 | Lower | -5 | Lower |
| Comments (CLOC) | 4 | 4 | **5 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00949

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.7822 | **52.7678 ✅** | 38.5686 | -14.9856 | Lower | -0.7864 | Lower | +14.1992 | Higher |
| MI (SEI) | 6.2440 | **26.2759 ✅** | 1.4178 | -20.0319 | Lower | +4.8262 | Higher | +24.8581 | Higher |
| Cyclomatic (sum) | 27 | **15 ✅** | 26 | +12 | Higher | +1 | Higher | -11 | Lower |
| Cognitive (sum) | 29 | **20 ✅** | 32 | +9 | Higher | -3 | Lower | -12 | Lower |
| Halstead Volume | 4475.2777 | **2843.2676 ✅** | 4851.5365 | +1632.0101 | Higher | -376.2588 | Lower | -2008.2689 | Lower |
| Halstead Effort | 317913.5937 | **117246.0161 ✅** | 396591.0525 | +200667.5776 | Higher | -78677.4588 | Lower | -279345.0364 | Lower |
| SLOC | 171 | **93 ✅** | 161 | +78 | Higher | +10 | Higher | -68 | Lower |
| PLOC | 135 | **72 ✅** | 129 | +63 | Higher | +6 | Higher | -57 | Lower |
| LLOC | 131 | **46 ✅** | 129 | +85 | Higher | +2 | Higher | -83 | Lower |
| Comments (CLOC) | **19 ✅** | 10 | 10 | +9 | Higher | +9 | Higher | 0 | Same |
| Blank Lines | 19 | **11 ✅** | 22 | +8 | Higher | -3 | Lower | -11 | Lower |
| Functions | 5 | 4 | **3 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 6 | **3 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p00971

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.5492 | **83.8847 ✅** | 44.6650 | -38.3355 | Lower | +0.8842 | Higher | +39.2197 | Higher |
| MI (SEI) | 18.5503 | **81.6259 ✅** | 10.1581 | -63.0756 | Lower | +8.3922 | Higher | +71.4679 | Higher |
| Cyclomatic (sum) | 19 | **3 ✅** | 15 | +16 | Higher | +4 | Higher | -12 | Lower |
| Cognitive (sum) | 28 | **1 ✅** | 19 | +27 | Higher | +9 | Higher | -18 | Lower |
| Halstead Volume | 4098.5627 | **645.4951 ✅** | 4873.3835 | +3453.0676 | Higher | -774.8208 | Lower | -4227.8884 | Lower |
| Halstead Effort | 295765.6674 | **7530.7760 ✅** | 373864.9567 | +288234.8914 | Higher | -78099.2893 | Lower | -366334.1807 | Lower |
| SLOC | 122 | **26 ✅** | 129 | +96 | Higher | -7 | Lower | -103 | Lower |
| PLOC | 93 | **16 ✅** | 101 | +77 | Higher | -8 | Lower | -85 | Lower |
| LLOC | 96 | **12 ✅** | 86 | +84 | Higher | +10 | Higher | -74 | Lower |
| Comments (CLOC) | **16 ✅** | 7 | 9 | +9 | Higher | +7 | Higher | -2 | Lower |
| Blank Lines | 13 | **3 ✅** | 19 | +10 | Higher | -6 | Lower | -16 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p01050

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.2031 | **94.4650 ✅** | 70.7174 | -19.2618 | Lower | +4.4857 | Higher | +23.7476 | Higher |
| MI (SEI) | 57.3088 | **99.7795 ✅** | 58.3206 | -42.4707 | Lower | -1.0119 | Lower | +41.4588 | Higher |
| Cyclomatic (sum) | 7 | **2 ✅** | 9 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 12 | **0 ✅** | 13 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 811.9636 | **277.3259 ✅** | 922.1952 | +534.6377 | Higher | -110.2316 | Lower | -644.8692 | Lower |
| Halstead Effort | 20840.3993 | **2258.2255 ✅** | 25113.3512 | +18582.1737 | Higher | -4272.9520 | Lower | -22855.1257 | Lower |
| SLOC | 39 | **18 ✅** | 48 | +21 | Higher | -9 | Lower | -30 | Lower |
| PLOC | 30 | **8 ✅** | 34 | +22 | Higher | -4 | Lower | -26 | Lower |
| LLOC | 29 | **5 ✅** | 30 | +24 | Higher | -1 | Lower | -25 | Lower |
| Comments (CLOC) | 4 | 6 | **9 ✅** | -2 | Lower | -5 | Lower | -3 | Lower |
| Blank Lines | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p01125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.8580 | 45.9818 | **57.4896 ✅** | +10.8762 | Higher | -0.6317 | Lower | -11.5079 | Lower |
| MI (SEI) | **34.1415 ✅** | 18.4380 | 27.7772 | +15.7035 | Higher | +6.3643 | Higher | -9.3392 | Lower |
| Cyclomatic (sum) | 16 | 37 | **14 ✅** | -21 | Lower | +2 | Higher | +23 | Higher |
| Cognitive (sum) | 26 | 36 | **17 ✅** | -10 | Lower | +9 | Higher | +19 | Higher |
| Halstead Volume | **2059.4391 ✅** | 2715.0084 | 2073.2272 | -655.5693 | Lower | -13.7881 | Lower | +641.7812 | Higher |
| Halstead Effort | 78660.5273 | 125213.6029 | **74117.8718 ✅** | -46553.0756 | Lower | +4542.6555 | Higher | +51095.7311 | Higher |
| SLOC | 79 | 105 | **78 ✅** | -26 | Lower | +1 | Higher | +27 | Higher |
| PLOC | **57 ✅** | 83 | 60 | -26 | Lower | -3 | Lower | +23 | Higher |
| LLOC | **45 ✅** | 51 | 47 | -6 | Lower | -2 | Lower | +4 | Higher |
| Comments (CLOC) | 10 | **11 ✅** | 5 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 2 | 9 | **0 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |

## p01128

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **46.2543 ✅** | 28.8259 | 41.4050 | +17.4285 | Higher | +4.8493 | Higher | -12.5791 | Lower |
| MI (SEI) | **15.6077 ✅** | -3.3383 | 14.6643 | +18.9460 | Higher | +0.9434 | Higher | -18.0026 | Lower |
| Cyclomatic (sum) | 23 | 38 | 23 | -15 | Lower | 0 | Same | +15 | Higher |
| Cognitive (sum) | 37 | 51 | **36 ✅** | -14 | Lower | +1 | Higher | +15 | Higher |
| Halstead Volume | **3807.0599 ✅** | 6734.4507 | 4354.4703 | -2927.3909 | Lower | -547.4104 | Lower | +2379.9804 | Higher |
| Halstead Effort | **192781.6349 ✅** | 423329.0213 | 209206.6839 | -230547.3865 | Lower | -16425.0490 | Lower | +214122.3375 | Higher |
| SLOC | **113 ✅** | 223 | 146 | -110 | Lower | -33 | Lower | +77 | Higher |
| PLOC | **87 ✅** | 160 | 102 | -73 | Lower | -15 | Lower | +58 | Higher |
| LLOC | **50 ✅** | 100 | 58 | -50 | Lower | -8 | Lower | +42 | Higher |
| Comments (CLOC) | 10 | **30 ✅** | 22 | -20 | Lower | -12 | Lower | +8 | Higher |
| Blank Lines | **16 ✅** | 33 | 22 | -17 | Lower | -6 | Lower | +11 | Higher |
| Functions | **1 ✅** | 5 | 2 | -4 | Lower | -1 | Lower | +3 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **1 ✅** | 5 | 4 | -4 | Lower | -3 | Lower | +1 | Higher |

## p01138

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3173 | 60.3932 | **65.8416 ✅** | +4.9241 | Higher | -0.5242 | Lower | -5.4483 | Lower |
| MI (SEI) | **45.1797 ✅** | 34.8554 | 44.5885 | +10.3243 | Higher | +0.5912 | Higher | -9.7331 | Lower |
| Cyclomatic (sum) | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **1382.2124 ✅** | 1986.9932 | 1394.0262 | -604.7808 | Lower | -11.8138 | Lower | +592.9670 | Higher |
| Halstead Effort | **30931.6721 ✅** | 58003.2898 | 31196.0453 | -27071.6177 | Lower | -264.3733 | Lower | +26807.2444 | Higher |
| SLOC | 58 | 68 | **56 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| PLOC | 41 | 48 | 41 | -7 | Lower | 0 | Same | +7 | Higher |
| LLOC | **29 ✅** | 34 | 30 | -5 | Lower | -1 | Lower | +4 | Higher |
| Comments (CLOC) | **7 ✅** | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 11 | 14 | **10 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.5446 ✅** | 59.8226 | 60.8302 | +3.7220 | Higher | +2.7144 | Higher | -1.0076 | Lower |
| MI (SEI) | 39.3669 | **51.7782 ✅** | 34.4144 | -12.4113 | Lower | +4.9525 | Higher | +17.3638 | Higher |
| Cyclomatic (sum) | 14 | **13 ✅** | 15 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 33 | **19 ✅** | 37 | +14 | Higher | -4 | Lower | -18 | Lower |
| Halstead Volume | 1628.5187 | **1329.7617 ✅** | 1841.2673 | +298.7570 | Higher | -212.7486 | Lower | -511.5056 | Lower |
| Halstead Effort | 70128.0872 | **38484.8676 ✅** | 77847.6973 | +31643.2196 | Higher | -7719.6100 | Lower | -39362.8297 | Lower |
| SLOC | **58 ✅** | 79 | 65 | -21 | Lower | -7 | Lower | +14 | Higher |
| PLOC | 45 | **42 ✅** | 49 | +3 | Higher | -4 | Lower | -7 | Lower |
| LLOC | 35 | **23 ✅** | 36 | +12 | Higher | -1 | Lower | -13 | Lower |
| Comments (CLOC) | 5 | **28 ✅** | 5 | -23 | Lower | 0 | Same | +23 | Higher |
| Blank Lines | 9 | 9 | 11 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01255

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -0.8656 | **21.8094 ✅** | -1.9040 | -22.6750 | Lower | +1.0384 | Higher | +23.7134 | Higher |
| MI (SEI) | -53.7526 | **-17.8700 ✅** | -57.4224 | -35.8826 | Lower | +3.6698 | Higher | +39.5524 | Higher |
| Cyclomatic (sum) | 103 | **61 ✅** | 105 | +42 | Higher | -2 | Lower | -44 | Lower |
| Cognitive (sum) | 164 | **63 ✅** | 171 | +101 | Higher | -7 | Lower | -108 | Lower |
| Halstead Volume | 14201.6187 | **7764.4287 ✅** | 15104.0808 | +6437.1900 | Higher | -902.4621 | Lower | -7339.6521 | Lower |
| Halstead Effort | 1488085.1856 | **647984.7096 ✅** | 1674782.9544 | +840100.4759 | Higher | -186697.7688 | Lower | -1026798.2447 | Lower |
| SLOC | 436 | **237 ✅** | 443 | +199 | Higher | -7 | Lower | -206 | Lower |
| PLOC | 380 | **192 ✅** | 395 | +188 | Higher | -15 | Lower | -203 | Lower |
| LLOC | 201 | **113 ✅** | 242 | +88 | Higher | -41 | Lower | -129 | Lower |
| Comments (CLOC) | 12 | **17 ✅** | 8 | -5 | Lower | +4 | Higher | +9 | Higher |
| Blank Lines | 46 | **28 ✅** | 40 | +18 | Higher | +6 | Higher | -12 | Lower |
| Functions | 20 | **7 ✅** | 20 | +13 | Higher | 0 | Same | -13 | Lower |
| Closures | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 24 | **19 ✅** | 24 | +5 | Higher | 0 | Same | -5 | Lower |

## p01288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.3477 ✅** | 53.6829 | 42.8705 | +4.6648 | Higher | +15.4772 | Higher | +10.8124 | Higher |
| MI (SEI) | **40.3114 ✅** | 35.5634 | 11.6737 | +4.7480 | Higher | +28.6377 | Higher | +23.8896 | Higher |
| Cyclomatic (sum) | **13 ✅** | 20 | 28 | -7 | Lower | -15 | Lower | -8 | Lower |
| Cognitive (sum) | **15 ✅** | 21 | 42 | -6 | Lower | -27 | Lower | -21 | Lower |
| Halstead Volume | **1837.3604 ✅** | 2439.0590 | 3780.3984 | -601.6986 | Lower | -1943.0380 | Lower | -1341.3394 | Lower |
| Halstead Effort | **72826.2831 ✅** | 144252.9159 | 225753.9789 | -71426.6327 | Lower | -152927.6958 | Lower | -81501.0630 | Lower |
| SLOC | **78 ✅** | 86 | 130 | -8 | Lower | -52 | Lower | -44 | Lower |
| PLOC | **55 ✅** | 58 | 104 | -3 | Lower | -49 | Lower | -46 | Lower |
| LLOC | 50 | **49 ✅** | 88 | +1 | Higher | -38 | Lower | -39 | Lower |
| Comments (CLOC) | 14 | **17 ✅** | 12 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | **10 ✅** | 11 | 14 | -1 | Lower | -4 | Lower | -3 | Lower |
| Functions | **1 ✅** | 4 | 4 | -3 | Lower | -3 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 3 | -3 | Lower | -3 | Lower | 0 | Same |

## p01353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.6267 | **83.1629 ✅** | 48.8222 | -33.5362 | Lower | +0.8045 | Higher | +34.3408 | Higher |
| MI (SEI) | 21.0390 | **88.3191 ✅** | 16.7681 | -67.2801 | Lower | +4.2709 | Higher | +71.5510 | Higher |
| Cyclomatic (sum) | 19 | **4 ✅** | 18 | +15 | Higher | +1 | Higher | -14 | Lower |
| Cognitive (sum) | 25 | **1 ✅** | 17 | +24 | Higher | +8 | Higher | -16 | Lower |
| Halstead Volume | 3170.5894 | **371.5647 ✅** | 3064.3401 | +2799.0247 | Higher | +106.2494 | Higher | -2692.7753 | Lower |
| Halstead Effort | 179504.1407 | **4963.0431 ✅** | 166700.0996 | +174541.0976 | Higher | +12804.0411 | Higher | -161737.0565 | Lower |
| SLOC | 103 | **32 ✅** | 111 | +71 | Higher | -8 | Lower | -79 | Lower |
| PLOC | 80 | **12 ✅** | 89 | +68 | Higher | -9 | Lower | -77 | Lower |
| LLOC | 62 | **8 ✅** | 63 | +54 | Higher | -1 | Lower | -55 | Lower |
| Comments (CLOC) | 10 | **15 ✅** | 8 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 14 | **5 ✅** | 14 | +9 | Higher | 0 | Same | -9 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01471

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.2445 ✅** | 44.5396 | 47.9874 | +9.7049 | Higher | +6.2571 | Higher | -3.4478 | Lower |
| MI (SEI) | **25.4367 ✅** | 11.8282 | 17.6353 | +13.6085 | Higher | +7.8014 | Higher | -5.8072 | Lower |
| Cyclomatic (sum) | **18 ✅** | 31 | 21 | -13 | Lower | -3 | Lower | +10 | Higher |
| Cognitive (sum) | **24 ✅** | 38 | 26 | -14 | Lower | -2 | Lower | +12 | Higher |
| Halstead Volume | **2149.5078 ✅** | 2647.2479 | 2746.7208 | -497.7402 | Lower | -597.2130 | Lower | -99.4728 | Lower |
| Halstead Effort | **89473.2615 ✅** | 135220.2217 | 136780.8502 | -45746.9602 | Lower | -47307.5888 | Lower | -1560.6286 | Lower |
| SLOC | **89 ✅** | 126 | 116 | -37 | Lower | -27 | Lower | +10 | Higher |
| PLOC | **72 ✅** | 101 | 94 | -29 | Lower | -22 | Lower | +7 | Higher |
| LLOC | **45 ✅** | 48 | 68 | -3 | Lower | -23 | Lower | -20 | Lower |
| Comments (CLOC) | 7 | 9 | **10 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | **11 ✅** | 17 | 13 | -6 | Lower | -2 | Lower | +4 | Higher |
| Functions | 2 | **1 ✅** | 4 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |

## p01521

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.4781 | **83.2291 ✅** | 73.4122 | -3.7509 | Lower | +6.0659 | Higher | +9.8168 | Higher |
| MI (SEI) | 74.1191 | **77.8457 ✅** | 70.5976 | -3.7266 | Lower | +3.5215 | Higher | +7.2481 | Higher |
| Cyclomatic (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 627.6685 | **509.0490 ✅** | 1058.8993 | +118.6196 | Higher | -431.2308 | Lower | -549.8503 | Lower |
| Halstead Effort | 10139.2610 | **7211.5271 ✅** | 26711.5888 | +2927.7340 | Higher | -16572.3278 | Lower | -19500.0617 | Lower |
| SLOC | 33 | **28 ✅** | 40 | +5 | Higher | -7 | Lower | -12 | Lower |
| PLOC | 19 | **17 ✅** | 22 | +2 | Higher | -3 | Lower | -5 | Lower |
| LLOC | **11 ✅** | 13 | 20 | -2 | Lower | -9 | Lower | -7 | Lower |
| Comments (CLOC) | 8 | 6 | **14 ✅** | +2 | Higher | -6 | Lower | -8 | Lower |
| Blank Lines | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01601

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.4547 ✅** | 70.2054 | 70.6494 | +5.2493 | Higher | +4.8052 | Higher | -0.4441 | Lower |
| MI (SEI) | 51.3705 | 48.8613 | **53.5271 ✅** | +2.5092 | Higher | -2.1566 | Lower | -4.6658 | Lower |
| Cyclomatic (sum) | **8 ✅** | 12 | 10 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | **5 ✅** | 9 | 6 | -4 | Lower | -1 | Lower | +3 | Higher |
| Halstead Volume | **802.5364 ✅** | 1017.5193 | 954.5007 | -214.9829 | Lower | -151.9642 | Lower | +63.0186 | Higher |
| Halstead Effort | **27455.1943 ✅** | 32711.3615 | 28833.8745 | -5256.1672 | Lower | -1378.6802 | Lower | +3877.4870 | Higher |
| SLOC | **38 ✅** | 46 | 47 | -8 | Lower | -9 | Lower | -1 | Lower |
| PLOC | **30 ✅** | 39 | 34 | -9 | Lower | -4 | Lower | +5 | Higher |
| LLOC | **22 ✅** | 26 | 23 | -4 | Lower | -1 | Lower | +3 | Higher |
| Comments (CLOC) | 2 | 4 | **6 ✅** | -2 | Lower | -4 | Lower | -2 | Lower |
| Blank Lines | 6 | **3 ✅** | 8 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 5 | 2 | -4 | Lower | -1 | Lower | +3 | Higher |

## p01656

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.4317 ✅** | 57.3417 | 70.4442 | +20.0900 | Higher | +6.9874 | Higher | -13.1026 | Lower |
| MI (SEI) | **69.1470 ✅** | 37.6561 | 62.8272 | +31.4909 | Higher | +6.3198 | Higher | -25.1711 | Lower |
| Cyclomatic (sum) | 8 | 23 | 8 | -15 | Lower | 0 | Same | +15 | Higher |
| Cognitive (sum) | **5 ✅** | 19 | 11 | -14 | Lower | -6 | Lower | +8 | Higher |
| Halstead Volume | **775.9559 ✅** | 1760.9956 | 1015.8941 | -985.0397 | Lower | -239.9383 | Lower | +745.1015 | Higher |
| Halstead Effort | **13207.4154 ✅** | 67622.2301 | 21950.5693 | -54414.8147 | Lower | -8743.1539 | Lower | +45671.6609 | Higher |
| SLOC | **34 ✅** | 73 | 48 | -39 | Lower | -14 | Lower | +25 | Higher |
| PLOC | **23 ✅** | 54 | 29 | -31 | Lower | -6 | Lower | +25 | Higher |
| LLOC | **13 ✅** | 32 | 21 | -19 | Lower | -8 | Lower | +11 | Higher |
| Comments (CLOC) | 7 | 11 | **13 ✅** | -4 | Lower | -6 | Lower | -2 | Lower |
| Blank Lines | **5 ✅** | 9 | 7 | -4 | Lower | -2 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p01767

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.5733 ✅** | 60.5324 | 59.4692 | +2.0409 | Higher | +3.1040 | Higher | +1.0631 | Higher |
| MI (SEI) | 32.5353 | **34.9544 ✅** | 34.9456 | -2.4190 | Lower | -2.4102 | Lower | +0.0088 | Higher |
| Cyclomatic (sum) | 11 | 11 | 13 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 12 | 12 | 13 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **1915.6282 ✅** | 2022 | 2074.6025 | -106.3718 | Lower | -158.9743 | Lower | -52.6025 | Lower |
| Halstead Effort | **75421.0173 ✅** | 76091.0526 | 87078.7103 | -670.0354 | Lower | -11657.6931 | Lower | -10987.6577 | Lower |
| SLOC | **61 ✅** | 68 | 70 | -7 | Lower | -9 | Lower | -2 | Lower |
| PLOC | **48 ✅** | 54 | 51 | -6 | Lower | -3 | Lower | +3 | Higher |
| LLOC | 35 | 41 | 35 | -6 | Lower | 0 | Same | +6 | Higher |
| Comments (CLOC) | 3 | 6 | **7 ✅** | -3 | Lower | -4 | Lower | -1 | Lower |
| Blank Lines | 10 | **8 ✅** | 12 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p01768

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.7038 ✅** | 43.7667 | 52.1215 | +10.9371 | Higher | +2.5823 | Higher | -8.3549 | Lower |
| MI (SEI) | **27.8790 ✅** | 18.6269 | 22.9541 | +9.2521 | Higher | +4.9249 | Higher | -4.3272 | Lower |
| Cyclomatic (sum) | **15 ✅** | 32 | 17 | -17 | Lower | -2 | Lower | +15 | Higher |
| Cognitive (sum) | **9 ✅** | 38 | 14 | -29 | Lower | -5 | Lower | +24 | Higher |
| Halstead Volume | 2792.8236 | 3167.6992 | **2757.9565 ✅** | -374.8757 | Lower | +34.8671 | Higher | +409.7428 | Higher |
| Halstead Effort | **108492.6473 ✅** | 161684.6492 | 113016.2599 | -53192.0018 | Lower | -4523.6126 | Lower | +48668.3893 | Higher |
| SLOC | **83 ✅** | 123 | 95 | -40 | Lower | -12 | Lower | +28 | Higher |
| PLOC | **59 ✅** | 88 | 72 | -29 | Lower | -13 | Lower | +16 | Higher |
| LLOC | **36 ✅** | 52 | 39 | -16 | Lower | -3 | Lower | +13 | Higher |
| Comments (CLOC) | 8 | **18 ✅** | 8 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01847

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **45.5385 ✅** | 35.5198 | 40.3857 | +10.0186 | Higher | +5.1528 | Higher | -4.8658 | Lower |
| MI (SEI) | 2.7029 | **6.2499 ✅** | 2.0405 | -3.5470 | Lower | +0.6624 | Higher | +4.2094 | Higher |
| Cyclomatic (sum) | **26 ✅** | 33 | 30 | -7 | Lower | -4 | Lower | +3 | Higher |
| Cognitive (sum) | **25 ✅** | 45 | 47 | -20 | Lower | -22 | Lower | -2 | Lower |
| Halstead Volume | **3433.0781 ✅** | 5207.4104 | 3573.0031 | -1774.3323 | Lower | -139.9250 | Lower | +1634.4074 | Higher |
| Halstead Effort | **209829.7324 ✅** | 350830.6800 | 254108.5759 | -141000.9475 | Lower | -44278.8435 | Lower | +96722.1040 | Higher |
| SLOC | **117 ✅** | 172 | 150 | -55 | Lower | -33 | Lower | +22 | Higher |
| PLOC | **101 ✅** | 132 | 121 | -31 | Lower | -20 | Lower | +11 | Higher |
| LLOC | **74 ✅** | 94 | 106 | -20 | Lower | -32 | Lower | -12 | Lower |
| Comments (CLOC) | 2 | **24 ✅** | 7 | -22 | Lower | -5 | Lower | +17 | Higher |
| Blank Lines | **15 ✅** | 16 | 22 | -1 | Lower | -7 | Lower | -6 | Lower |
| Functions | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01916

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 86.6968 | **87.1934 ✅** | 84.6425 | -0.4966 | Lower | +2.0543 | Higher | +2.5509 | Higher |
| MI (SEI) | **89.3709 ✅** | 81.1786 | 86.3398 | +8.1923 | Higher | +3.0311 | Higher | -5.1613 | Lower |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | **526.8709 ✅** | 550.0164 | 533.8256 | -23.1455 | Lower | -6.9547 | Lower | +16.1908 | Higher |
| Halstead Effort | 6814.9603 | 8178.5044 | **6498.7467 ✅** | -1363.5441 | Lower | +316.2136 | Higher | +1679.7577 | Higher |
| SLOC | 23 | **22 ✅** | 26 | +1 | Higher | -3 | Lower | -4 | Lower |
| PLOC | 13 | 14 | 13 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 8 | 4 | **9 ✅** | +4 | Higher | -1 | Lower | -5 | Lower |
| Blank Lines | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01917

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.9171 | **67.0198 ✅** | 52.0835 | -14.1028 | Lower | +0.8336 | Higher | +14.9364 | Higher |
| MI (SEI) | 20.7304 | **58.6477 ✅** | 15.4574 | -37.9173 | Lower | +5.2730 | Higher | +43.1903 | Higher |
| Cyclomatic (sum) | 21 | **12 ✅** | 21 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 55 | **15 ✅** | 53 | +40 | Higher | +2 | Higher | -38 | Lower |
| Halstead Volume | 2429.8464 | **1139.3324 ✅** | 2661.5258 | +1290.5141 | Higher | -231.6793 | Lower | -1522.1934 | Lower |
| Halstead Effort | 114084.2534 | **29704.0220 ✅** | 147462.9132 | +84380.2314 | Higher | -33378.6598 | Lower | -117758.8912 | Lower |
| SLOC | 89 | **54 ✅** | 91 | +35 | Higher | -2 | Lower | -37 | Lower |
| PLOC | 71 | **33 ✅** | 77 | +38 | Higher | -6 | Lower | -44 | Lower |
| LLOC | 57 | **21 ✅** | 59 | +36 | Higher | -2 | Lower | -38 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 3 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 13 | **6 ✅** | 11 | +7 | Higher | +2 | Higher | -5 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.1581 | 51.1664 | **52.5587 ✅** | +0.9917 | Higher | -0.4006 | Lower | -1.3922 | Lower |
| MI (SEI) | 23.5896 | **29.0053 ✅** | 24.1518 | -5.4157 | Lower | -0.5621 | Lower | +4.8535 | Higher |
| Cyclomatic (sum) | **15 ✅** | 23 | 16 | -8 | Lower | -1 | Lower | +7 | Higher |
| Cognitive (sum) | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 3797.5702 | **2906.0347 ✅** | 3247.5441 | +891.5355 | Higher | +550.0260 | Higher | -341.5094 | Lower |
| Halstead Effort | 289802.0722 | **210240.4317 ✅** | 261387.6967 | +79561.6405 | Higher | +28414.3755 | Higher | -51147.2649 | Lower |
| SLOC | **88 ✅** | 91 | 89 | -3 | Lower | -1 | Lower | +2 | Higher |
| PLOC | 70 | **66 ✅** | 71 | +4 | Higher | -1 | Lower | -5 | Lower |
| LLOC | 61 | **43 ✅** | 54 | +18 | Higher | +7 | Higher | -11 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 8 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | **10 ✅** | 11 | 11 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 9 | 4 | -6 | Lower | -1 | Lower | +5 | Higher |

## p02038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.8608 | 53.7014 | **70.7270 ✅** | +12.1594 | Higher | -4.8662 | Lower | -17.0256 | Lower |
| MI (SEI) | 44.6757 | 33.0420 | **55.4860 ✅** | +11.6337 | Higher | -10.8103 | Lower | -22.4440 | Lower |
| Cyclomatic (sum) | 18 | 26 | **12 ✅** | -8 | Lower | +6 | Higher | +14 | Higher |
| Cognitive (sum) | 15 | 28 | **8 ✅** | -13 | Lower | +7 | Higher | +20 | Higher |
| Halstead Volume | 1387.7925 | 2005.6534 | **1317.2366 ✅** | -617.8609 | Lower | +70.5559 | Higher | +688.4168 | Higher |
| Halstead Effort | 44814.1340 | 78743.6980 | **35565.3883 ✅** | -33929.5640 | Lower | +9248.7457 | Higher | +43178.3097 | Higher |
| SLOC | 50 | 84 | **41 ✅** | -34 | Lower | +9 | Higher | +43 | Higher |
| PLOC | 39 | 59 | **29 ✅** | -20 | Lower | +10 | Higher | +30 | Higher |
| LLOC | 20 | 41 | **18 ✅** | -21 | Lower | +2 | Higher | +23 | Higher |
| Comments (CLOC) | 5 | **13 ✅** | 6 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 7 | 12 | 7 | -5 | Lower | 0 | Same | +5 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02104

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.3323 | **57.4523 ✅** | 51.2520 | -0.1200 | Lower | +6.0803 | Higher | +6.2004 | Higher |
| MI (SEI) | 38.1976 | **51.0894 ✅** | 25.7522 | -12.8918 | Lower | +12.4454 | Higher | +25.3372 | Higher |
| Cyclomatic (sum) | **16 ✅** | 17 | 18 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 24 | **12 ✅** | 23 | +12 | Higher | +1 | Higher | -11 | Lower |
| Halstead Volume | 1956 | **1625.8201 ✅** | 2742.8402 | +330.1799 | Higher | -786.8402 | Lower | -1117.0200 | Lower |
| Halstead Effort | 101292.8571 | **61781.1653 ✅** | 167562.6000 | +39511.6919 | Higher | -66269.7428 | Lower | -105781.4347 | Lower |
| SLOC | **78 ✅** | 81 | 99 | -3 | Lower | -21 | Lower | -18 | Lower |
| PLOC | 55 | **41 ✅** | 76 | +14 | Higher | -21 | Lower | -35 | Lower |
| LLOC | 43 | **25 ✅** | 67 | +18 | Higher | -24 | Lower | -42 | Lower |
| Comments (CLOC) | 13 | **34 ✅** | 12 | -21 | Lower | +1 | Higher | +22 | Higher |
| Blank Lines | 10 | **7 ✅** | 11 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p02162

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.3141 ✅** | 65.5177 | 66.2977 | +0.7963 | Higher | +0.0163 | Higher | -0.7800 | Lower |
| MI (SEI) | 45.8543 | **46.0735 ✅** | 33.0704 | -0.2192 | Lower | +12.7839 | Higher | +13.0032 | Higher |
| Cyclomatic (sum) | 21 | 21 | **20 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 16 | 17 | **13 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Halstead Volume | 1186.2403 | **1148.9009 ✅** | 1326.3062 | +37.3394 | Higher | -140.0659 | Lower | -177.4053 | Lower |
| Halstead Effort | **31316.7431 ✅** | 31847.5326 | 35767.4835 | -530.7895 | Lower | -4450.7404 | Lower | -3919.9509 | Lower |
| SLOC | 49 | 52 | **48 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| PLOC | 42 | 42 | **41 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | 12 | 12 | 13 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 1 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 5 | **0 ✅** | 0 | Same | +5 | Higher | +5 | Higher |

## p02166

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.6176 ✅** | 57.1632 | 62.1108 | +5.4544 | Higher | +0.5068 | Higher | -4.9476 | Lower |
| MI (SEI) | 40.7267 | **43.9233 ✅** | 32.4096 | -3.1965 | Lower | +8.3171 | Higher | +11.5137 | Higher |
| Cyclomatic (sum) | **12 ✅** | 18 | 15 | -6 | Lower | -3 | Lower | +3 | Higher |
| Cognitive (sum) | 18 | **15 ✅** | 23 | +3 | Higher | -5 | Lower | -8 | Lower |
| Halstead Volume | **1727.4339 ✅** | 1849.5846 | 1846.9795 | -122.1507 | Lower | -119.5456 | Lower | +2.6051 | Higher |
| Halstead Effort | **57391.7183 ✅** | 77066.0250 | 79050.7224 | -19674.3067 | Lower | -21659.0041 | Lower | -1984.6974 | Lower |
| SLOC | 62 | 78 | **60 ✅** | -16 | Lower | +2 | Higher | +18 | Higher |
| PLOC | 50 | 50 | 52 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 37 | **36 ✅** | 38 | +1 | Higher | -1 | Lower | -2 | Lower |
| Comments (CLOC) | 7 | **20 ✅** | 3 | -13 | Lower | +4 | Higher | +17 | Higher |
| Blank Lines | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02186

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.0496 | **40.2038 ✅** | 32.5150 | -9.1543 | Lower | -1.4655 | Lower | +7.6888 | Higher |
| MI (SEI) | -7.2838 | **3.4358 ✅** | -3.4782 | -10.7197 | Lower | -3.8056 | Lower | +6.9140 | Higher |
| Cyclomatic (sum) | 27 | 28 | **26 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 7450.8890 | **5361.5919 ✅** | 7210.8430 | +2089.2970 | Higher | +240.0459 | Higher | -1849.2511 | Lower |
| Halstead Effort | 667268.5008 | **408780.7677 ✅** | 667159.7383 | +258487.7331 | Higher | +108.7625 | Higher | -258378.9706 | Lower |
| SLOC | 220 | **137 ✅** | 206 | +83 | Higher | +14 | Higher | -69 | Lower |
| PLOC | 172 | **107 ✅** | 157 | +65 | Higher | +15 | Higher | -50 | Lower |
| LLOC | 94 | **70 ✅** | 91 | +24 | Higher | +3 | Higher | -21 | Lower |
| Comments (CLOC) | 17 | 8 | **19 ✅** | +9 | Higher | -2 | Lower | -11 | Lower |
| Blank Lines | 34 | **22 ✅** | 32 | +12 | Higher | +2 | Higher | -10 | Lower |
| Functions | 11 | **9 ✅** | 10 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 13 | **8 ✅** | 13 | +5 | Higher | 0 | Same | -5 | Lower |

## p02190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.6040 | 69.9644 | **72.7146 ✅** | +0.6397 | Higher | -2.1106 | Lower | -2.7502 | Lower |
| MI (SEI) | 58.1570 | 55.9128 | **58.8837 ✅** | +2.2443 | Higher | -0.7266 | Lower | -2.9709 | Lower |
| Cyclomatic (sum) | 9 | 11 | **7 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 13 | 11 | **6 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| Halstead Volume | 942.5240 | 975.6572 | **899.8434 ✅** | -33.1332 | Lower | +42.6806 | Higher | +75.8138 | Higher |
| Halstead Effort | 20875.1608 | 21094.3806 | **20192.4858 ✅** | -219.2198 | Lower | +682.6751 | Higher | +901.8949 | Higher |
| SLOC | 48 | 48 | **44 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| PLOC | 34 | 33 | **30 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| LLOC | 20 | 20 | 24 | 0 | Same | -4 | Lower | -4 | Lower |
| Comments (CLOC) | **9 ✅** | 8 | 7 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |

## p02242

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.0622 | **53.9075 ✅** | 31.4435 | -9.8453 | Lower | +12.6187 | Higher | +22.4640 | Higher |
| MI (SEI) | 10.2343 | **33.3586 ✅** | -3.3513 | -23.1243 | Lower | +13.5856 | Higher | +36.7099 | Higher |
| Cyclomatic (sum) | 25 | **17 ✅** | 45 | +8 | Higher | -20 | Lower | -28 | Lower |
| Cognitive (sum) | 23 | 23 | 64 | 0 | Same | -41 | Lower | -41 | Lower |
| Halstead Volume | 3432.6745 | **2315.1308 ✅** | 5000.7206 | +1117.5437 | Higher | -1568.0462 | Lower | -2685.5899 | Lower |
| Halstead Effort | 173540.7642 | **77788.3938 ✅** | 387636.5066 | +95752.3703 | Higher | -214095.7424 | Lower | -309848.1127 | Lower |
| SLOC | 130 | **90 ✅** | 189 | +40 | Higher | -59 | Lower | -99 | Lower |
| PLOC | 108 | **63 ✅** | 155 | +45 | Higher | -47 | Lower | -92 | Lower |
| LLOC | 64 | **45 ✅** | 99 | +19 | Higher | -35 | Lower | -54 | Lower |
| Comments (CLOC) | 9 | 15 | **17 ✅** | -6 | Lower | -8 | Lower | -2 | Lower |
| Blank Lines | 14 | 14 | 17 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 5 | **1 ✅** | 7 | +4 | Higher | -2 | Lower | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |

## p02249

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.0771 | **58.7460 ✅** | 39.1554 | -21.6689 | Lower | -2.0783 | Lower | +19.5906 | Higher |
| MI (SEI) | 0.9672 | **39.1568 ✅** | 1.7404 | -38.1895 | Lower | -0.7732 | Lower | +37.4164 | Higher |
| Cyclomatic (sum) | 37 | **19 ✅** | 31 | +18 | Higher | +6 | Higher | -12 | Lower |
| Cognitive (sum) | 45 | **20 ✅** | 38 | +25 | Higher | +7 | Higher | -18 | Lower |
| Halstead Volume | 4851.8299 | **2095.7840 ✅** | 4813.2723 | +2756.0459 | Higher | +38.5576 | Higher | -2717.4883 | Lower |
| Halstead Effort | 354790.0607 | **90401.9260 ✅** | 313692.5741 | +264388.1347 | Higher | +41097.4866 | Higher | -223290.6481 | Lower |
| SLOC | 151 | **67 ✅** | 145 | +84 | Higher | +6 | Higher | -78 | Lower |
| PLOC | 121 | **48 ✅** | 124 | +73 | Higher | -3 | Lower | -76 | Lower |
| LLOC | 86 | **29 ✅** | 86 | +57 | Higher | 0 | Same | -57 | Lower |
| Comments (CLOC) | 10 | 10 | 8 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 20 | **9 ✅** | 14 | +11 | Higher | +6 | Higher | -5 | Lower |
| Functions | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | 5 | **4 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p02256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.7758 | **70.2566 ✅** | 61.0570 | -1.4808 | Lower | +7.7188 | Higher | +9.1996 | Higher |
| MI (SEI) | 49.7558 | **55.8467 ✅** | 33.5325 | -6.0910 | Lower | +16.2232 | Higher | +22.3142 | Higher |
| Cyclomatic (sum) | **11 ✅** | 12 | 14 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 9 | **5 ✅** | 18 | +4 | Higher | -9 | Lower | -13 | Lower |
| Halstead Volume | **955.5762 ✅** | 1157.1893 | 1283.3399 | -201.6130 | Lower | -327.7637 | Lower | -126.1507 | Lower |
| Halstead Effort | **22051.7589 ✅** | 36699.4309 | 32258.4987 | -14647.6720 | Lower | -10206.7398 | Lower | +4440.9322 | Higher |
| SLOC | 52 | **44 ✅** | 73 | +8 | Higher | -21 | Lower | -29 | Lower |
| PLOC | 36 | **30 ✅** | 56 | +6 | Higher | -20 | Lower | -26 | Lower |
| LLOC | 21 | **16 ✅** | 27 | +5 | Higher | -6 | Lower | -11 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 5 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 10 | **7 ✅** | 12 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02266

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.7478 | **63.9796 ✅** | 55.5931 | -5.2318 | Lower | +3.1547 | Higher | +8.3865 | Higher |
| MI (SEI) | 43.8343 | **46.9641 ✅** | 39.9910 | -3.1298 | Lower | +3.8433 | Higher | +6.9731 | Higher |
| Cyclomatic (sum) | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 26 | **20 ✅** | 26 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 1447.1302 | **1329.3790 ✅** | 1669.5246 | +117.7512 | Higher | -222.3944 | Lower | -340.1456 | Lower |
| Halstead Effort | 42669.6673 | **36000.4413 ✅** | 54855.8077 | +6669.2260 | Higher | -12186.1404 | Lower | -18855.3664 | Lower |
| SLOC | 81 | **62 ✅** | 94 | +19 | Higher | -13 | Lower | -32 | Lower |
| PLOC | 55 | **44 ✅** | 65 | +11 | Higher | -10 | Lower | -21 | Lower |
| LLOC | 34 | **27 ✅** | 36 | +7 | Higher | -2 | Lower | -9 | Lower |
| Comments (CLOC) | 18 | 10 | **22 ✅** | +8 | Higher | -4 | Lower | -12 | Lower |
| Blank Lines | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02276

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.1875 | 62.1804 | **63.3334 ✅** | +0.0070 | Higher | -1.1459 | Lower | -1.1529 | Lower |
| MI (SEI) | 37.9131 | **43.5007 ✅** | 41.9435 | -5.5876 | Lower | -4.0304 | Lower | +1.5571 | Higher |
| Cyclomatic (sum) | 12 | 13 | 12 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 1619.5414 | **1411.7734 ✅** | 1583.5069 | +207.7680 | Higher | +36.0345 | Higher | -171.7334 | Lower |
| Halstead Effort | 59969.3043 | **42781.0127 ✅** | 57844.5745 | +17188.2916 | Higher | +2124.7298 | Higher | -15063.5617 | Lower |
| SLOC | 65 | 67 | **61 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| PLOC | 49 | 51 | **47 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| LLOC | 32 | 33 | **30 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Comments (CLOC) | 6 | **10 ✅** | 7 | -4 | Lower | -1 | Lower | +3 | Higher |
| Blank Lines | 10 | **7 ✅** | 8 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p02277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.6424 | **58.9268 ✅** | 45.7833 | -0.2844 | Lower | +12.8591 | Higher | +13.1435 | Higher |
| MI (SEI) | 26.0842 | **28.7121 ✅** | 5.4316 | -2.6279 | Lower | +20.6526 | Higher | +23.2805 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 26 | +1 | Higher | -11 | Lower | -12 | Lower |
| Cognitive (sum) | 17 | **13 ✅** | 33 | +4 | Higher | -16 | Lower | -20 | Lower |
| Halstead Volume | 2130 | **2107.8287 ✅** | 3551.2523 | +22.1713 | Higher | -1421.2523 | Lower | -1443.4235 | Lower |
| Halstead Effort | 85336.5385 | **79702.2740 ✅** | 188007.4723 | +5634.2644 | Higher | -102670.9338 | Lower | -108305.1982 | Lower |
| SLOC | 71 | 71 | 114 | 0 | Same | -43 | Lower | -43 | Lower |
| PLOC | 59 | **58 ✅** | 98 | +1 | Higher | -39 | Lower | -40 | Lower |
| LLOC | 36 | 36 | 64 | 0 | Same | -28 | Lower | -28 | Lower |
| Comments (CLOC) | 3 | **4 ✅** | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 10 | 10 | 14 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 3 | 3 | 6 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |

## p02279

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.0443 | **52.5909 ✅** | 50.3884 | -4.5467 | Lower | -2.3441 | Lower | +2.2026 | Higher |
| MI (SEI) | 16.6304 | **23.1149 ✅** | 12.0187 | -6.4845 | Lower | +4.6117 | Higher | +11.0961 | Higher |
| Cyclomatic (sum) | 19 | **15 ✅** | 18 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 21 | 24 | **20 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Halstead Volume | 3133.3933 | **2498.8196 ✅** | 3042.7407 | +634.5736 | Higher | +90.6526 | Higher | -543.9211 | Lower |
| Halstead Effort | 147206.8158 | **76239.4967 ✅** | 125289.3229 | +70967.3191 | Higher | +21917.4929 | Higher | -49049.8262 | Lower |
| SLOC | 114 | **98 ✅** | 101 | +16 | Higher | +13 | Higher | -3 | Lower |
| PLOC | 91 | **78 ✅** | 88 | +13 | Higher | +3 | Higher | -10 | Lower |
| LLOC | 57 | **44 ✅** | 56 | +13 | Higher | +1 | Higher | -12 | Lower |
| Comments (CLOC) | **9 ✅** | 8 | 3 | +1 | Higher | +6 | Higher | +5 | Higher |
| Blank Lines | 14 | 12 | 12 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |

## p02297

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.8787 ✅** | 74.9504 | 73.2331 | +0.9283 | Higher | +2.6456 | Higher | +1.7173 | Higher |
| MI (SEI) | **66.3154 ✅** | 65.4597 | 56.6003 | +0.8557 | Higher | +9.7151 | Higher | +8.8594 | Higher |
| Cyclomatic (sum) | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | **1044.0873 ✅** | 1055.4011 | 1145.5841 | -11.3138 | Lower | -101.4968 | Lower | -90.1830 | Lower |
| Halstead Effort | **23247.2566 ✅** | 26072.3162 | 26312.6346 | -2825.0597 | Lower | -3065.3780 | Lower | -240.3183 | Lower |
| SLOC | **35 ✅** | 38 | 40 | -3 | Lower | -5 | Lower | -2 | Lower |
| PLOC | 23 | **22 ✅** | 26 | +1 | Higher | -3 | Lower | -4 | Lower |
| LLOC | **16 ✅** | 18 | 21 | -2 | Lower | -5 | Lower | -3 | Lower |
| Comments (CLOC) | 7 | **8 ✅** | 5 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | **5 ✅** | 8 | 9 | -3 | Lower | -4 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02305

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.3019 ✅** | 69.0740 | 70.9508 | +3.2278 | Higher | +1.3511 | Higher | -1.8767 | Lower |
| MI (SEI) | 49.6664 | **58.6425 ✅** | 47.9209 | -8.9760 | Lower | +1.7455 | Higher | +10.7216 | Higher |
| Cyclomatic (sum) | **7 ✅** | 9 | 9 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **1213.8990 ✅** | 1264.9509 | 1440.7960 | -51.0518 | Lower | -226.8970 | Lower | -175.8452 | Lower |
| Halstead Effort | **25844.3018 ✅** | 30548.5631 | 40435.2436 | -4704.2613 | Lower | -14590.9418 | Lower | -9886.6806 | Lower |
| SLOC | 41 | 48 | 41 | -7 | Lower | 0 | Same | +7 | Higher |
| PLOC | 31 | **29 ✅** | 32 | +2 | Higher | -1 | Lower | -3 | Lower |
| LLOC | 18 | 18 | 26 | 0 | Same | -8 | Lower | -8 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 3 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.5936 | **65.7062 ✅** | 61.2704 | -7.1126 | Lower | -2.6769 | Lower | +4.4357 | Higher |
| MI (SEI) | 40.1460 | **50.8610 ✅** | 38.1869 | -10.7149 | Lower | +1.9591 | Higher | +12.6740 | Higher |
| Cyclomatic (sum) | 14 | 13 | 13 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 16 | 16 | **15 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 1818 | **1253.0030 ✅** | 1762.4421 | +564.9970 | Higher | +55.5579 | Higher | -509.4391 | Lower |
| Halstead Effort | 80589.3429 | **36033.3278 ✅** | 76925.4144 | +44556.0150 | Higher | +3663.9285 | Higher | -40892.0865 | Lower |
| SLOC | 76 | **56 ✅** | 66 | +20 | Higher | +10 | Higher | -10 | Lower |
| PLOC | 55 | **41 ✅** | 52 | +14 | Higher | +3 | Higher | -11 | Lower |
| LLOC | 41 | **25 ✅** | 41 | +16 | Higher | 0 | Same | -16 | Lower |
| Comments (CLOC) | **13 ✅** | 10 | 7 | +3 | Higher | +6 | Higher | +3 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |

## p02328

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.6680 | **65.9290 ✅** | 57.4274 | -3.2610 | Lower | +5.2406 | Higher | +8.5016 | Higher |
| MI (SEI) | 34.8561 | **46.4701 ✅** | 25.6538 | -11.6140 | Lower | +9.2022 | Higher | +20.8162 | Higher |
| Cyclomatic (sum) | 11 | **10 ✅** | 14 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 17 | 17 | **16 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 1701.1845 | **1370.7676 ✅** | 2016.5341 | +330.4169 | Higher | -315.3496 | Lower | -645.7666 | Lower |
| Halstead Effort | 53465.7983 | **41019.1806 ✅** | 104314.7646 | +12446.6177 | Higher | -50848.9663 | Lower | -63295.5841 | Lower |
| SLOC | 63 | **56 ✅** | 79 | +7 | Higher | -16 | Lower | -23 | Lower |
| PLOC | 50 | **37 ✅** | 63 | +13 | Higher | -13 | Lower | -26 | Lower |
| LLOC | 32 | **26 ✅** | 46 | +6 | Higher | -14 | Lower | -20 | Lower |
| Comments (CLOC) | 4 | **7 ✅** | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |

## p02333

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.7646 | 48.1792 | **57.2719 ✅** | +4.5854 | Higher | -4.5073 | Lower | -9.0927 | Lower |
| MI (SEI) | 22.2505 | 24.7645 | **28.3024 ✅** | -2.5140 | Lower | -6.0519 | Lower | -3.5379 | Lower |
| Cyclomatic (sum) | 19 | 22 | **15 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| Cognitive (sum) | 15 | 17 | **12 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Halstead Volume | 3041.7730 | 3074.7047 | **2654.1338 ✅** | -32.9317 | Lower | +387.6392 | Higher | +420.5709 | Higher |
| Halstead Effort | 195542.5517 | 194365.2602 | **162820.9003 ✅** | +1177.2915 | Higher | +32721.6513 | Higher | +31544.3598 | Higher |
| SLOC | 86 | 109 | **72 ✅** | -23 | Lower | +14 | Higher | +37 | Higher |
| PLOC | 65 | 77 | **57 ✅** | -12 | Lower | +8 | Higher | +20 | Higher |
| LLOC | 50 | 59 | **42 ✅** | -9 | Lower | +8 | Higher | +17 | Higher |
| Comments (CLOC) | 6 | **17 ✅** | 5 | -11 | Lower | +1 | Higher | +12 | Higher |
| Blank Lines | 15 | 15 | **10 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 7 | 6 | **3 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |

## p02347

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **41.3695 ✅** | 38.0277 | 38.7390 | +3.3417 | Higher | +2.6305 | Higher | -0.7113 | Lower |
| MI (SEI) | **4.0682 ✅** | -3.9634 | -1.3623 | +8.0316 | Higher | +5.4305 | Higher | -2.6011 | Lower |
| Cyclomatic (sum) | **27 ✅** | 30 | 29 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **27 ✅** | 38 | 28 | -11 | Lower | -1 | Lower | +10 | Higher |
| Halstead Volume | **4798.5185 ✅** | 5286.5302 | 5696.8984 | -488.0117 | Lower | -898.3800 | Lower | -410.3683 | Lower |
| Halstead Effort | 313530.3171 | **285319.3973 ✅** | 423882.2701 | +28210.9198 | Higher | -110351.9530 | Lower | -138562.8728 | Lower |
| SLOC | **134 ✅** | 153 | 145 | -19 | Lower | -11 | Lower | +8 | Higher |
| PLOC | **110 ✅** | 129 | 118 | -19 | Lower | -8 | Lower | +11 | Higher |
| LLOC | **57 ✅** | 66 | 76 | -9 | Lower | -19 | Lower | -10 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | 20 | 20 | 22 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02355

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.0689 | **55.8164 ✅** | 42.7479 | -4.7476 | Lower | +8.3209 | Higher | +13.0685 | Higher |
| MI (SEI) | 22.6250 | **32.8988 ✅** | 6.7931 | -10.2739 | Lower | +15.8319 | Higher | +26.1057 | Higher |
| Cyclomatic (sum) | 23 | **20 ✅** | 33 | +3 | Higher | -10 | Lower | -13 | Lower |
| Cognitive (sum) | 32 | **26 ✅** | 35 | +6 | Higher | -3 | Lower | -9 | Lower |
| Halstead Volume | 2139.8662 | **2027.1251 ✅** | 2958.5069 | +112.7411 | Higher | -818.6407 | Lower | -931.3818 | Lower |
| Halstead Effort | 121374.4703 | **101904.1280 ✅** | 189192.7259 | +19470.3424 | Higher | -67818.2556 | Lower | -87288.5979 | Lower |
| SLOC | 101 | **80 ✅** | 132 | +21 | Higher | -31 | Lower | -52 | Lower |
| PLOC | 79 | **61 ✅** | 108 | +18 | Higher | -29 | Lower | -47 | Lower |
| LLOC | 63 | **46 ✅** | 80 | +17 | Higher | -17 | Lower | -34 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 7 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 14 | **11 ✅** | 18 | +3 | Higher | -4 | Lower | -7 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |

## p02369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.0123 ✅** | 55.7528 | 54.1957 | +2.2595 | Higher | +3.8166 | Higher | +1.5571 | Higher |
| MI (SEI) | 32.8437 | **32.9089 ✅** | 29.9934 | -0.0652 | Lower | +2.8503 | Higher | +2.9154 | Higher |
| Cyclomatic (sum) | **13 ✅** | 21 | 17 | -8 | Lower | -4 | Lower | +4 | Higher |
| Cognitive (sum) | **16 ✅** | 18 | 21 | -2 | Lower | -5 | Lower | -3 | Lower |
| Halstead Volume | **1811.1158 ✅** | 1963.2919 | 2190.3234 | -152.1761 | Lower | -379.2076 | Lower | -227.0315 | Lower |
| Halstead Effort | **54612.1077 ✅** | 60909.9343 | 82236.6867 | -6297.8266 | Lower | -27624.5790 | Lower | -21326.7524 | Lower |
| SLOC | 80 | 80 | 90 | 0 | Same | -10 | Lower | -10 | Lower |
| PLOC | **61 ✅** | 62 | 68 | -1 | Lower | -7 | Lower | -6 | Lower |
| LLOC | 34 | **26 ✅** | 38 | +8 | Higher | -4 | Lower | -12 | Lower |
| Comments (CLOC) | 8 | 10 | **11 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | 14 | **11 ✅** | 13 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 8 | 4 | -5 | Lower | -1 | Lower | +4 | Higher |

## p02370

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.3634 ✅** | 68.0188 | 46.0194 | +0.3447 | Higher | +22.3440 | Higher | +21.9994 | Higher |
| MI (SEI) | 47.4872 | **52.0336 ✅** | 17.9295 | -4.5464 | Lower | +29.5577 | Higher | +34.1041 | Higher |
| Cyclomatic (sum) | 8 | 8 | 20 | 0 | Same | -12 | Lower | -12 | Lower |
| Cognitive (sum) | 10 | 10 | 20 | 0 | Same | -10 | Lower | -10 | Lower |
| Halstead Volume | 1421.4572 | **1340.8888 ✅** | 3010.7077 | +80.5684 | Higher | -1589.2504 | Lower | -1669.8189 | Lower |
| Halstead Effort | 38601.4474 | **34569.7891 ✅** | 166529.7676 | +4031.6584 | Higher | -127928.3202 | Lower | -131959.9785 | Lower |
| SLOC | **49 ✅** | 51 | 129 | -2 | Lower | -80 | Lower | -78 | Lower |
| PLOC | 36 | **34 ✅** | 95 | +2 | Higher | -59 | Lower | -61 | Lower |
| LLOC | 30 | **26 ✅** | 79 | +4 | Higher | -49 | Lower | -53 | Lower |
| Comments (CLOC) | 5 | 8 | **15 ✅** | -3 | Lower | -10 | Lower | -7 | Lower |
| Blank Lines | **8 ✅** | 9 | 19 | -1 | Lower | -11 | Lower | -10 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p02393

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.8107 ✅** | 89.9348 | 76.6192 | +0.8759 | Higher | +14.1915 | Higher | +13.3156 | Higher |
| MI (SEI) | 80.2025 | **83.1232 ✅** | 64.7996 | -2.9207 | Lower | +15.4028 | Higher | +18.3235 | Higher |
| Cyclomatic (sum) | 2 | 2 | 12 | 0 | Same | -10 | Lower | -10 | Lower |
| Cognitive (sum) | 0 | 0 | 7 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | 560 | 560 | 918.0797 | 0 | Same | -358.0797 | Lower | -358.0797 | Lower |
| Halstead Effort | 6130.5263 | 6130.5263 | 24864.6593 | 0 | Same | -18734.1330 | Lower | -18734.1330 | Lower |
| SLOC | **18 ✅** | 19 | 32 | -1 | Lower | -14 | Lower | -13 | Lower |
| PLOC | 12 | 12 | 23 | 0 | Same | -11 | Lower | -11 | Lower |
| LLOC | 9 | 9 | **8 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 2 | 3 | **5 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02396

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 92.6482 | **93.5742 ✅** | 83.9266 | -0.9260 | Lower | +8.7217 | Higher | +9.6477 | Higher |
| MI (SEI) | 76.2240 | **78.0533 ✅** | 76.9090 | -1.8293 | Lower | -0.6850 | Lower | +1.1443 | Higher |
| Cyclomatic (sum) | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 360 | 360 | 498.5531 | 0 | Same | -138.5531 | Lower | -138.5531 | Lower |
| Halstead Effort | 4500 | 4500 | 7478.2967 | 0 | Same | -2978.2967 | Lower | -2978.2967 | Lower |
| SLOC | 18 | **17 ✅** | 27 | +1 | Higher | -9 | Lower | -10 | Lower |
| PLOC | 14 | 14 | 18 | 0 | Same | -4 | Lower | -4 | Lower |
| LLOC | 10 | 10 | 11 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 1 | 1 | **5 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 3 | 3 | 5 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02412

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.5283 ✅** | 67.9205 | 67.3173 | +2.6078 | Higher | +3.2111 | Higher | +0.6033 | Higher |
| MI (SEI) | 47.1544 | 39.0603 | **51.1527 ✅** | +8.0941 | Higher | -3.9983 | Lower | -12.0924 | Lower |
| Cyclomatic (sum) | 14 | 15 | 14 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | **19 ✅** | 29 | 21 | -10 | Lower | -2 | Lower | +8 | Higher |
| Halstead Volume | **1005.2933 ✅** | 1066.4160 | 1043.9532 | -61.1227 | Lower | -38.6600 | Lower | +22.4628 | Higher |
| Halstead Effort | 27746.0937 | 29859.6470 | **25295.7893 ✅** | -2113.5533 | Lower | +2450.3044 | Higher | +4563.8577 | Higher |
| SLOC | **44 ✅** | 50 | 53 | -6 | Lower | -9 | Lower | -3 | Lower |
| PLOC | **35 ✅** | 39 | 36 | -4 | Lower | -1 | Lower | +3 | Higher |
| LLOC | 25 | 27 | 25 | -2 | Lower | 0 | Same | +2 | Higher |
| Comments (CLOC) | 3 | 2 | **8 ✅** | +1 | Higher | -5 | Lower | -6 | Lower |
| Blank Lines | **7 ✅** | 9 | 9 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02418

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.1071 | 81.7491 | **86.8650 ✅** | +0.3580 | Higher | -4.7579 | Lower | -5.1159 | Lower |
| MI (SEI) | 71.0999 | 74.8864 | **86.4733 ✅** | -3.7865 | Lower | -15.3734 | Lower | -11.5869 | Lower |
| Cyclomatic (sum) | 7 | 7 | **3 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Cognitive (sum) | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 604.2966 | 522.1624 | **411.1983 ✅** | +82.1342 | Higher | +193.0983 | Higher | +110.9641 | Higher |
| Halstead Effort | 9208.3290 | 6340.5437 | **3635.8586 ✅** | +2867.7853 | Higher | +5572.4704 | Higher | +2704.6851 | Higher |
| SLOC | 28 | 30 | **25 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| PLOC | 22 | 20 | **14 ✅** | +2 | Higher | +8 | Higher | +6 | Higher |
| LLOC | 11 | 9 | 9 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 4 | 6 | **7 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02430

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.7474 ✅** | 64.1260 | 66.5538 | +3.6214 | Higher | +1.1936 | Higher | -2.4279 | Lower |
| MI (SEI) | **53.6524 ✅** | 50.2761 | 53.2697 | +3.3763 | Higher | +0.3827 | Higher | -2.9936 | Lower |
| Cyclomatic (sum) | 13 | 15 | **10 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Cognitive (sum) | **12 ✅** | 13 | 14 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | **1132.4342 ✅** | 1253.6079 | 1285.7686 | -121.1737 | Lower | -153.3343 | Lower | -32.1606 | Lower |
| Halstead Effort | **32387.6184 ✅** | 36088.7132 | 34715.7510 | -3701.0948 | Lower | -2328.1326 | Lower | +1372.9623 | Higher |
| SLOC | **51 ✅** | 60 | 55 | -9 | Lower | -4 | Lower | +5 | Higher |
| PLOC | 36 | 39 | **35 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| LLOC | **22 ✅** | 23 | 26 | -1 | Lower | -4 | Lower | -3 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 11 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | **6 ✅** | 9 | 9 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02449

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.7017 ✅** | 50.3485 | 43.5689 | +5.3532 | Higher | +12.1328 | Higher | +6.7796 | Higher |
| MI (SEI) | **32.9759 ✅** | 21.2803 | 8.7649 | +11.6956 | Higher | +24.2110 | Higher | +12.5154 | Higher |
| Cyclomatic (sum) | **17 ✅** | 21 | 21 | -4 | Lower | -4 | Lower | 0 | Same |
| Cognitive (sum) | 22 | **16 ✅** | 46 | +6 | Higher | -24 | Lower | -30 | Lower |
| Halstead Volume | **2032.7194 ✅** | 2604 | 4005.0974 | -571.2806 | Lower | -1972.3780 | Lower | -1401.0974 | Lower |
| Halstead Effort | **78811.4352 ✅** | 150887.3333 | 358037.0811 | -72075.8981 | Lower | -279225.6458 | Lower | -207149.7477 | Lower |
| SLOC | **84 ✅** | 102 | 135 | -18 | Lower | -51 | Lower | -33 | Lower |
| PLOC | **64 ✅** | 79 | 112 | -15 | Lower | -48 | Lower | -33 | Lower |
| LLOC | **47 ✅** | 56 | 109 | -9 | Lower | -62 | Lower | -53 | Lower |
| Comments (CLOC) | **11 ✅** | 9 | 9 | +2 | Higher | +2 | Higher | 0 | Same |
| Blank Lines | **9 ✅** | 16 | 15 | -7 | Lower | -6 | Lower | +1 | Higher |
| Functions | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 8 | **1 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |

## p02450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.3616 ✅** | 67.0369 | 66.8495 | +2.3247 | Higher | +2.5121 | Higher | +0.1874 | Higher |
| MI (SEI) | 44.7541 | 48.7179 | **49.0100 ✅** | -3.9639 | Lower | -4.2559 | Lower | -0.2921 | Lower |
| Cyclomatic (sum) | **9 ✅** | 10 | 11 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **13 ✅** | 16 | 14 | -3 | Lower | -1 | Lower | +2 | Higher |
| Halstead Volume | 1463.4534 | **1315.0185 ✅** | 1470.3532 | +148.4349 | Higher | -6.8998 | Lower | -155.3347 | Lower |
| Halstead Effort | 50748.7887 | **42362.3814 ✅** | 59696.3411 | +8386.4072 | Higher | -8947.5525 | Lower | -17333.9597 | Lower |
| SLOC | **45 ✅** | 53 | 51 | -8 | Lower | -6 | Lower | +2 | Higher |
| PLOC | **35 ✅** | 39 | 38 | -4 | Lower | -3 | Lower | +1 | Higher |
| LLOC | **24 ✅** | 30 | 26 | -6 | Lower | -2 | Lower | +4 | Higher |
| Comments (CLOC) | 3 | 7 | 7 | -4 | Lower | -4 | Lower | 0 | Same |
| Blank Lines | 7 | 8 | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02542

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 13.8958 | **33.1729 ✅** | 8.3539 | -19.2772 | Lower | +5.5418 | Higher | +24.8190 | Higher |
| MI (SEI) | -26.4924 | **-6.5183 ✅** | -43.6579 | -19.9741 | Lower | +17.1655 | Higher | +37.1396 | Higher |
| Cyclomatic (sum) | 52 | **35 ✅** | 61 | +17 | Higher | -9 | Lower | -26 | Lower |
| Cognitive (sum) | 139 | **59 ✅** | 133 | +80 | Higher | +6 | Higher | -74 | Lower |
| Halstead Volume | 12368.1206 | **6276.9705 ✅** | 16022.6141 | +6091.1502 | Higher | -3654.4935 | Lower | -9745.6437 | Lower |
| Halstead Effort | 1437013.7910 | **449975.5933 ✅** | 2932468.7482 | +987038.1977 | Higher | -1495454.9571 | Lower | -2482493.1549 | Lower |
| SLOC | 378 | **182 ✅** | 431 | +196 | Higher | -53 | Lower | -249 | Lower |
| PLOC | 295 | **147 ✅** | 363 | +148 | Higher | -68 | Lower | -216 | Lower |
| LLOC | 234 | **104 ✅** | 350 | +130 | Higher | -116 | Lower | -246 | Lower |
| Comments (CLOC) | **39 ✅** | 10 | 14 | +29 | Higher | +25 | Higher | -4 | Lower |
| Blank Lines | 51 | **26 ✅** | 59 | +25 | Higher | -8 | Lower | -33 | Lower |
| Functions | **2 ✅** | 4 | 5 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 4 | 5 | -1 | Lower | -2 | Lower | -1 | Lower |

## p02562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 19.1391 | **25.2795 ✅** | 11.0275 | -6.1403 | Lower | +8.1117 | Higher | +14.2520 | Higher |
| MI (SEI) | -18.6516 | **-11.3684 ✅** | -38.5573 | -7.2833 | Lower | +19.9057 | Higher | +27.1889 | Higher |
| Cyclomatic (sum) | 52 | **43 ✅** | 62 | +9 | Higher | -10 | Lower | -19 | Lower |
| Cognitive (sum) | 112 | **73 ✅** | 125 | +39 | Higher | -13 | Lower | -52 | Lower |
| Halstead Volume | 9977.5103 | **7386.0455 ✅** | 12318.5231 | +2591.4648 | Higher | -2341.0128 | Lower | -4932.4776 | Lower |
| Halstead Effort | 1079566.6134 | **607907.5768 ✅** | 1798769.2918 | +471659.0367 | Higher | -719202.6784 | Lower | -1190861.7151 | Lower |
| SLOC | 293 | **251 ✅** | 392 | +42 | Higher | -99 | Lower | -141 | Lower |
| PLOC | 227 | **196 ✅** | 332 | +31 | Higher | -105 | Lower | -136 | Lower |
| LLOC | 181 | **136 ✅** | 240 | +45 | Higher | -59 | Lower | -104 | Lower |
| Comments (CLOC) | **31 ✅** | 25 | 15 | +6 | Higher | +16 | Higher | +10 | Higher |
| Blank Lines | 35 | **30 ✅** | 50 | +5 | Higher | -15 | Lower | -20 | Lower |
| Functions | **2 ✅** | 4 | 5 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **3 ✅** | 6 | 6 | -3 | Lower | -3 | Lower | 0 | Same |

## p02585

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.2734 | **56.2021 ✅** | 50.5502 | -3.9287 | Lower | +1.7232 | Higher | +5.6519 | Higher |
| MI (SEI) | 28.5816 | **35.8163 ✅** | 27.1696 | -7.2347 | Lower | +1.4120 | Higher | +8.6467 | Higher |
| Cyclomatic (sum) | 16 | **13 ✅** | 18 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 26 | **18 ✅** | 27 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 2893.5245 | **1975.3366 ✅** | 2949.4903 | +918.1879 | Higher | -55.9658 | Lower | -974.1537 | Lower |
| Halstead Effort | 232968.9136 | **83540.2788 ✅** | 229396.6119 | +149428.6348 | Higher | +3572.3018 | Higher | -145856.3330 | Lower |
| SLOC | 94 | **87 ✅** | 101 | +7 | Higher | -7 | Lower | -14 | Lower |
| PLOC | 66 | **58 ✅** | 70 | +8 | Higher | -4 | Lower | -12 | Lower |
| LLOC | 69 | **48 ✅** | 70 | +21 | Higher | -1 | Lower | -22 | Lower |
| Comments (CLOC) | 13 | 14 | **15 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 16 | **15 ✅** | 16 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02606

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **92.3738 ✅** | 86.6836 | 86.0123 | +5.6902 | Higher | +6.3616 | Higher | +0.6714 | Higher |
| MI (SEI) | 83.1058 | **88.5533 ✅** | 83.5636 | -5.4476 | Lower | -0.4579 | Lower | +4.9897 | Higher |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **495.4196 ✅** | 505.4097 | 575.0609 | -9.9901 | Lower | -79.6413 | Lower | -69.6512 | Lower |
| Halstead Effort | 5507.9006 | **5410.8571 ✅** | 6991.5302 | +97.0435 | Higher | -1483.6296 | Lower | -1580.6731 | Lower |
| SLOC | **17 ✅** | 24 | 24 | -7 | Lower | -7 | Lower | 0 | Same |
| PLOC | 11 | 11 | 13 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 2 | **8 ✅** | 6 | -6 | Lower | -4 | Lower | +2 | Higher |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02617

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.8479 ✅** | 70.1762 | 73.1759 | +8.6717 | Higher | +5.6719 | Higher | -2.9997 | Lower |
| MI (SEI) | 61.7362 | **70.2145 ✅** | 60.4471 | -8.4783 | Lower | +1.2891 | Higher | +9.7674 | Higher |
| Cyclomatic (sum) | **5 ✅** | 7 | 7 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **4 ✅** | 7 | 5 | -3 | Lower | -1 | Lower | +2 | Higher |
| Halstead Volume | 899.8434 | **871.2542 ✅** | 1026.0791 | +28.5892 | Higher | -126.2358 | Lower | -154.8250 | Lower |
| Halstead Effort | 21380.2790 | **18836.5147 ✅** | 30269.3349 | +2543.7643 | Higher | -8889.0559 | Lower | -11432.8202 | Lower |
| SLOC | **31 ✅** | 52 | 41 | -21 | Lower | -10 | Lower | +11 | Higher |
| PLOC | 22 | 22 | 27 | 0 | Same | -5 | Lower | -5 | Lower |
| LLOC | 18 | **13 ✅** | 23 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | 3 | **25 ✅** | 7 | -22 | Lower | -4 | Lower | +18 | Higher |
| Blank Lines | 6 | **5 ✅** | 8 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02618

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.5837 | **70.2354 ✅** | 41.6574 | -28.6517 | Lower | -0.0737 | Lower | +28.5779 | Higher |
| MI (SEI) | 10.2622 | **63.9573 ✅** | 10.3686 | -53.6951 | Lower | -0.1063 | Lower | +53.5888 | Higher |
| Cyclomatic (sum) | 24 | **8 ✅** | 24 | +16 | Higher | 0 | Same | -16 | Lower |
| Cognitive (sum) | 19 | **8 ✅** | 18 | +11 | Higher | +1 | Higher | -10 | Lower |
| Halstead Volume | 4587.5148 | **931.2363 ✅** | 4522.9463 | +3656.2785 | Higher | +64.5684 | Higher | -3591.7101 | Lower |
| Halstead Effort | 286719.6738 | **17072.6650 ✅** | 235193.2094 | +269647.0088 | Higher | +51526.4644 | Higher | -218120.5444 | Lower |
| SLOC | 140 | **50 ✅** | 140 | +90 | Higher | 0 | Same | -90 | Lower |
| PLOC | 110 | **29 ✅** | 110 | +81 | Higher | 0 | Same | -81 | Lower |
| LLOC | 74 | **20 ✅** | 70 | +54 | Higher | +4 | Higher | -50 | Lower |
| Comments (CLOC) | 14 | **15 ✅** | 14 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 17 | **6 ✅** | 16 | +11 | Higher | +1 | Higher | -10 | Lower |
| Functions | 7 | **1 ✅** | 7 | +6 | Higher | 0 | Same | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **1 ✅** | 6 | +5 | Higher | 0 | Same | -5 | Lower |

## p02639

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.1291 | **81.7405 ✅** | 77.7031 | -0.6114 | Lower | +3.4260 | Higher | +4.0374 | Higher |
| MI (SEI) | 62.5191 | 57.8969 | **64.8295 ✅** | +4.6222 | Higher | -2.3104 | Lower | -6.9326 | Lower |
| Cyclomatic (sum) | 7 | 7 | 12 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 816.8482 | 816.8482 | 1012.8500 | 0 | Same | -196.0019 | Lower | -196.0019 | Lower |
| Halstead Effort | 11175.9679 | 11175.9679 | 19156.0763 | 0 | Same | -7980.1084 | Lower | -7980.1084 | Lower |
| SLOC | 27 | **26 ✅** | 29 | +1 | Higher | -2 | Lower | -3 | Lower |
| PLOC | 22 | 22 | 22 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 2 | 1 | **4 ✅** | +1 | Higher | -2 | Lower | -3 | Lower |
| Blank Lines | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 5 | 0 | Same | -5 | Lower | -5 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02644

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.6777 ✅** | 54.5795 | 53.6556 | +1.0982 | Higher | +2.0221 | Higher | +0.9239 | Higher |
| MI (SEI) | 25.2804 | **33.7700 ✅** | 17.4808 | -8.4896 | Lower | +7.7996 | Higher | +16.2892 | Higher |
| Cyclomatic (sum) | 14 | **13 ✅** | 14 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | **26 ✅** | 27 | 28 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 3057.9825 | **2901.5222 ✅** | 3315.7416 | +156.4603 | Higher | -257.7590 | Lower | -414.2194 | Lower |
| Halstead Effort | 119714.3525 | **114378.0039 ✅** | 169687.9500 | +5336.3486 | Higher | -49973.5975 | Lower | -55309.9461 | Lower |
| SLOC | **77 ✅** | 85 | 85 | -8 | Lower | -8 | Lower | 0 | Same |
| PLOC | 58 | **57 ✅** | 66 | +1 | Higher | -8 | Lower | -9 | Lower |
| LLOC | **45 ✅** | 46 | 62 | -1 | Lower | -17 | Lower | -16 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 3 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 14 | 14 | 16 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.3390 | **62.5723 ✅** | 56.8166 | -3.2333 | Lower | +2.5224 | Higher | +5.7557 | Higher |
| MI (SEI) | 39.2493 | **43.6219 ✅** | 36.1837 | -4.3726 | Lower | +3.0657 | Higher | +7.4383 | Higher |
| Cyclomatic (sum) | 12 | 12 | 15 | 0 | Same | -3 | Lower | -3 | Lower |
| Cognitive (sum) | 22 | **18 ✅** | 24 | +4 | Higher | -2 | Lower | -6 | Lower |
| Halstead Volume | 1869.9683 | **1742.5327 ✅** | 2169.7526 | +127.4356 | Higher | -299.7843 | Lower | -427.2199 | Lower |
| Halstead Effort | 78198.6737 | **65959.9888 ✅** | 117166.6411 | +12238.6850 | Higher | -38967.9673 | Lower | -51206.6523 | Lower |
| SLOC | 74 | **62 ✅** | 79 | +12 | Higher | -5 | Lower | -17 | Lower |
| PLOC | 54 | **45 ✅** | 56 | +9 | Higher | -2 | Lower | -11 | Lower |
| LLOC | 49 | **40 ✅** | 49 | +9 | Higher | 0 | Same | -9 | Lower |
| Comments (CLOC) | 11 | 9 | **12 ✅** | +2 | Higher | -1 | Lower | -3 | Lower |
| Blank Lines | 10 | **8 ✅** | 12 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02678

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.4427 ✅** | 66.4608 | 56.0855 | +2.9819 | Higher | +13.3572 | Higher | +10.3753 | Higher |
| MI (SEI) | **49.6383 ✅** | 49.2043 | 29.5946 | +0.4340 | Higher | +20.0437 | Higher | +19.6098 | Higher |
| Cyclomatic (sum) | 7 | 7 | 11 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 8 | 8 | 10 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | **1469.8883 ✅** | 1677.5569 | 2653.8917 | -207.6686 | Lower | -1184.0034 | Lower | -976.3348 | Lower |
| Halstead Effort | **40270.6158 ✅** | 52004.2648 | 115363.0471 | -11733.6490 | Lower | -75092.4314 | Lower | -63358.7823 | Lower |
| SLOC | **46 ✅** | 53 | 82 | -7 | Lower | -36 | Lower | -29 | Lower |
| PLOC | **34 ✅** | 38 | 62 | -4 | Lower | -28 | Lower | -24 | Lower |
| LLOC | **27 ✅** | 32 | 54 | -5 | Lower | -27 | Lower | -22 | Lower |
| Comments (CLOC) | 5 | 8 | 8 | -3 | Lower | -3 | Lower | 0 | Same |
| Blank Lines | 8 | **7 ✅** | 14 | +1 | Higher | -6 | Lower | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02727

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.6800 ✅** | 44.3958 | 42.5670 | +11.2842 | Higher | +13.1130 | Higher | +1.8288 | Higher |
| MI (SEI) | **32.5335 ✅** | 21.2698 | 7.1536 | +11.2637 | Higher | +25.3799 | Higher | +14.1162 | Higher |
| Cyclomatic (sum) | **23 ✅** | 39 | 39 | -16 | Lower | -16 | Lower | 0 | Same |
| Cognitive (sum) | **20 ✅** | 24 | 41 | -4 | Lower | -21 | Lower | -17 | Lower |
| Halstead Volume | **2228.2761 ✅** | 3178.9251 | 3812.4140 | -950.6490 | Lower | -1584.1379 | Lower | -633.4890 | Lower |
| Halstead Effort | **105391.4367 ✅** | 192824.5114 | 270504.0750 | -87433.0746 | Lower | -165112.6383 | Lower | -77679.5637 | Lower |
| SLOC | **75 ✅** | 107 | 113 | -32 | Lower | -38 | Lower | -6 | Lower |
| PLOC | **57 ✅** | 78 | 96 | -21 | Lower | -39 | Lower | -18 | Lower |
| LLOC | 44 | **42 ✅** | 57 | +2 | Higher | -13 | Lower | -15 | Lower |
| Comments (CLOC) | 9 | **17 ✅** | 6 | -8 | Lower | +3 | Higher | +11 | Higher |
| Blank Lines | **9 ✅** | 12 | 11 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 8 | 12 | **3 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02731

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 96.1400 | 94.2568 | 96.1400 | +1.8832 | Higher | 0 | Same | -1.8832 | Lower |
| MI (SEI) | 95.1389 | **97.7243 ✅** | 90.0032 | -2.5854 | Lower | +5.1358 | Higher | +7.7212 | Higher |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 354.6326 | **344.9167 ✅** | 354.6326 | +9.7160 | Higher | 0 | Same | -9.7160 | Lower |
| Halstead Effort | 3806.3900 | **3702.1054 ✅** | 3806.3900 | +104.2847 | Higher | 0 | Same | -104.2847 | Lower |
| SLOC | 15 | 17 | 15 | -2 | Lower | 0 | Same | +2 | Higher |
| PLOC | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 3 | **5 ✅** | 2 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02733

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.4762 | **50.4160 ✅** | 46.3429 | -4.9398 | Lower | -0.8667 | Lower | +4.0731 | Higher |
| MI (SEI) | 21.5930 | **32.9399 ✅** | 20.6025 | -11.3469 | Lower | +0.9905 | Higher | +12.3375 | Higher |
| Cyclomatic (sum) | 25 | **20 ✅** | 24 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 70 | **44 ✅** | 66 | +26 | Higher | +4 | Higher | -22 | Lower |
| Halstead Volume | 3030.2132 | **2528.9291 ✅** | 3128.9559 | +501.2841 | Higher | -98.7427 | Lower | -600.0268 | Lower |
| Halstead Effort | 182587.1824 | **111359.0950 ✅** | 215741.5093 | +71228.0874 | Higher | -33154.3270 | Lower | -104382.4143 | Lower |
| SLOC | 124 | **104 ✅** | 118 | +20 | Higher | +6 | Higher | -14 | Lower |
| PLOC | 91 | **69 ✅** | 88 | +22 | Higher | +3 | Higher | -19 | Lower |
| LLOC | 82 | **60 ✅** | 84 | +22 | Higher | -2 | Lower | -24 | Lower |
| Comments (CLOC) | 20 | **24 ✅** | 16 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 14 | **13 ✅** | 15 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02743

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **86.7886 ✅** | 81.5287 | 85.0749 | +5.2599 | Higher | +1.7137 | Higher | -3.5463 | Lower |
| MI (SEI) | 76.9849 | 67.5241 | **79.9255 ✅** | +9.4608 | Higher | -2.9406 | Lower | -12.4014 | Lower |
| Cyclomatic (sum) | 4 | 9 | 4 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **594.5414 ✅** | 692.3877 | 630.3439 | -97.8463 | Lower | -35.8025 | Lower | +62.0438 | Higher |
| Halstead Effort | **9043.2872 ✅** | 11572.7657 | 10715.8456 | -2529.4785 | Lower | -1672.5583 | Lower | +856.9201 | Higher |
| SLOC | **22 ✅** | 27 | 24 | -5 | Lower | -2 | Lower | +3 | Higher |
| PLOC | 15 | 20 | 15 | -5 | Lower | 0 | Same | +5 | Higher |
| LLOC | 10 | **8 ✅** | 10 | +2 | Higher | 0 | Same | -2 | Lower |
| Comments (CLOC) | 3 | 3 | **5 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02761

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.3009 | **56.1480 ✅** | 53.6445 | -2.8471 | Lower | -0.3436 | Lower | +2.5035 | Higher |
| MI (SEI) | **36.8409 ✅** | 35.5204 | 29.2068 | +1.3205 | Higher | +7.6342 | Higher | +6.3137 | Higher |
| Cyclomatic (sum) | 21 | 21 | 25 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 20 | **17 ✅** | 36 | +3 | Higher | -16 | Lower | -19 | Lower |
| Halstead Volume | 2256.9572 | **1750.5337 ✅** | 2565.9557 | +506.4235 | Higher | -308.9985 | Lower | -815.4220 | Lower |
| Halstead Effort | 96261.8486 | **61949.4431 ✅** | 130767.5188 | +34312.4055 | Higher | -34505.6701 | Lower | -68818.0756 | Lower |
| SLOC | 89 | 81 | **79 ✅** | +8 | Higher | +10 | Higher | +2 | Higher |
| PLOC | **56 ✅** | 60 | 59 | -4 | Lower | -3 | Lower | +1 | Higher |
| LLOC | 44 | **37 ✅** | 44 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | **20 ✅** | 12 | 9 | +8 | Higher | +11 | Higher | +3 | Higher |
| Blank Lines | 14 | **9 ✅** | 11 | +5 | Higher | +3 | Higher | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 6 | **1 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |

## p02783

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.9209 ✅** | 85.4796 | 88.1399 | +4.4412 | Higher | +1.7810 | Higher | -2.6603 | Lower |
| MI (SEI) | 82.5651 | 79.0116 | **85.4244 ✅** | +3.5535 | Higher | -2.8593 | Lower | -6.4128 | Lower |
| Cyclomatic (sum) | 3 | 7 | 3 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | **457.8717 ✅** | 669.6940 | 479.2174 | -211.8223 | Lower | -21.3458 | Lower | +190.4766 | Higher |
| Halstead Effort | **6181.2674 ✅** | 14766.7527 | 6539.9086 | -8585.4854 | Lower | -358.6413 | Lower | +8226.8441 | Higher |
| SLOC | **20 ✅** | 22 | 22 | -2 | Lower | -2 | Lower | 0 | Same |
| PLOC | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 3 | 4 | **5 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02812

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.8043 ✅** | 69.5379 | 71.7065 | +6.2665 | Higher | +4.0978 | Higher | -2.1687 | Lower |
| MI (SEI) | 63.5826 | 53.9870 | **63.8544 ✅** | +9.5956 | Higher | -0.2717 | Lower | -9.8674 | Lower |
| Cyclomatic (sum) | **7 ✅** | 12 | 8 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | 6 | 10 | **5 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Halstead Volume | **852.2223 ✅** | 1081.9189 | 1122.6724 | -229.6966 | Lower | -270.4501 | Lower | -40.7536 | Lower |
| Halstead Effort | **16970.9783 ✅** | 27292.2755 | 27973.2542 | -10321.2973 | Lower | -11002.2759 | Lower | -680.9787 | Lower |
| SLOC | **37 ✅** | 47 | 43 | -10 | Lower | -6 | Lower | +4 | Higher |
| PLOC | **25 ✅** | 32 | 26 | -7 | Lower | -1 | Lower | +6 | Higher |
| LLOC | **21 ✅** | 22 | 23 | -1 | Lower | -2 | Lower | -1 | Lower |
| Comments (CLOC) | 6 | 7 | **11 ✅** | -1 | Lower | -5 | Lower | -4 | Lower |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02831

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.1067 ✅** | 63.6001 | 77.1077 | +16.5066 | Higher | +2.9989 | Higher | -13.5077 | Lower |
| MI (SEI) | 72.1832 | 52.4515 | **72.7942 ✅** | +19.7317 | Higher | -0.6110 | Lower | -20.3427 | Lower |
| Cyclomatic (sum) | 8 | 14 | 8 | -6 | Lower | 0 | Same | +6 | Higher |
| Cognitive (sum) | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **618.5942 ✅** | 1185.6915 | 691.1242 | -567.0973 | Lower | -72.5300 | Lower | +494.5673 | Higher |
| Halstead Effort | **11988.9444 ✅** | 36778.3926 | 13493.3774 | -24789.4482 | Lower | -1504.4330 | Lower | +23285.0151 | Higher |
| SLOC | **31 ✅** | 64 | 36 | -33 | Lower | -5 | Lower | +28 | Higher |
| PLOC | **21 ✅** | 41 | 23 | -20 | Lower | -2 | Lower | +18 | Higher |
| LLOC | **10 ✅** | 25 | 13 | -15 | Lower | -3 | Lower | +12 | Higher |
| Comments (CLOC) | 6 | **16 ✅** | 10 | -10 | Lower | -4 | Lower | +6 | Higher |
| Blank Lines | 4 | 7 | **3 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 2 | 6 | 2 | -4 | Lower | 0 | Same | +4 | Higher |

## p02837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.2213 | 48.4605 | **58.2300 ✅** | +8.7608 | Higher | -1.0087 | Lower | -9.7694 | Lower |
| MI (SEI) | **37.1891 ✅** | 30.1345 | 34.3178 | +7.0546 | Higher | +2.8713 | Higher | -4.1833 | Lower |
| Cyclomatic (sum) | 16 | 19 | 16 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | **26 ✅** | 33 | 27 | -7 | Lower | -1 | Lower | +6 | Higher |
| Halstead Volume | 2080.1669 | 2739.7895 | **1939.2024 ✅** | -659.6226 | Lower | +140.9644 | Higher | +800.5871 | Higher |
| Halstead Effort | 90338.6758 | 110899.2072 | **86698.5089 ✅** | -20560.5313 | Lower | +3640.1669 | Higher | +24200.6983 | Higher |
| SLOC | 77 | 116 | **74 ✅** | -39 | Lower | +3 | Higher | +42 | Higher |
| PLOC | 56 | 75 | 56 | -19 | Lower | 0 | Same | +19 | Higher |
| LLOC | 43 | 59 | **39 ✅** | -16 | Lower | +4 | Higher | +20 | Higher |
| Comments (CLOC) | 12 | **27 ✅** | 8 | -15 | Lower | +4 | Higher | +19 | Higher |
| Blank Lines | 10 | 14 | 10 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p02846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.1103 ✅** | 56.3397 | 65.7594 | +14.7706 | Higher | +5.3509 | Higher | -9.4197 | Lower |
| MI (SEI) | **52.8044 ✅** | 51.0840 | 44.8949 | +1.7204 | Higher | +7.9096 | Higher | +6.1892 | Higher |
| Cyclomatic (sum) | 7 | 9 | **6 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **1316.0027 ✅** | 1745.6783 | 2129.3084 | -429.6757 | Lower | -813.3057 | Lower | -383.6300 | Lower |
| Halstead Effort | **38935.5275 ✅** | 52986.4719 | 93208.7570 | -14050.9443 | Lower | -54273.2294 | Lower | -40222.2851 | Lower |
| SLOC | **43 ✅** | 95 | 52 | -52 | Lower | -9 | Lower | +43 | Higher |
| PLOC | **32 ✅** | 36 | 39 | -4 | Lower | -7 | Lower | -3 | Lower |
| LLOC | **23 ✅** | 27 | 36 | -4 | Lower | -13 | Lower | -9 | Lower |
| Comments (CLOC) | 5 | **48 ✅** | 6 | -43 | Lower | -1 | Lower | +42 | Higher |
| Blank Lines | **6 ✅** | 11 | 7 | -5 | Lower | -1 | Lower | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02862

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.9718 ✅** | 55.1770 | 60.1771 | +5.7948 | Higher | +0.7947 | Higher | -5.0001 | Lower |
| MI (SEI) | 27.0096 | **32.6747 ✅** | 25.5575 | -5.6651 | Lower | +1.4521 | Higher | +7.1172 | Higher |
| Cyclomatic (sum) | 11 | 14 | 11 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | **6 ✅** | 12 | 7 | -6 | Lower | -1 | Lower | +5 | Higher |
| Halstead Volume | **2357.3137 ✅** | 2386.1019 | 2376 | -28.7882 | Lower | -18.6863 | Lower | +10.1019 | Higher |
| Halstead Effort | **117503.0217 ✅** | 134069.0995 | 117862.1053 | -16566.0778 | Lower | -359.0836 | Lower | +16206.9943 | Higher |
| SLOC | **63 ✅** | 86 | 66 | -23 | Lower | -3 | Lower | +20 | Higher |
| PLOC | **50 ✅** | 61 | 52 | -11 | Lower | -2 | Lower | +9 | Higher |
| LLOC | **43 ✅** | 51 | 45 | -8 | Lower | -2 | Lower | +6 | Higher |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | **11 ✅** | 13 | 12 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |

## p02885

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **92.3276 ✅** | 90.6375 | 88.4174 | +1.6900 | Higher | +3.9102 | Higher | +2.2201 | Higher |
| MI (SEI) | 83.1408 | **87.9897 ✅** | 87.3390 | -4.8488 | Lower | -4.1981 | Lower | +0.6507 | Higher |
| Cyclomatic (sum) | 3 | 3 | 5 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 478.2215 | **468.0466 ✅** | 559.6209 | +10.1749 | Higher | -81.3994 | Lower | -91.5743 | Lower |
| Halstead Effort | 6163.7439 | **5824.5797 ✅** | 7731.6047 | +339.1642 | Higher | -1567.8608 | Lower | -1907.0250 | Lower |
| SLOC | **17 ✅** | 19 | 20 | -2 | Lower | -3 | Lower | -1 | Lower |
| PLOC | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 2 | 4 | **5 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02923

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.1386 | **75.9428 ✅** | 51.4007 | -5.8042 | Lower | +18.7379 | Higher | +24.5421 | Higher |
| MI (SEI) | 56.0623 | **58.9556 ✅** | 18.4751 | -2.8932 | Lower | +37.5872 | Higher | +40.4804 | Higher |
| Cyclomatic (sum) | 10 | **7 ✅** | 24 | +3 | Higher | -14 | Lower | -17 | Lower |
| Cognitive (sum) | 15 | **8 ✅** | 21 | +7 | Higher | -6 | Lower | -13 | Lower |
| Halstead Volume | 986.1761 | **829.8336 ✅** | 2483.7813 | +156.3425 | Higher | -1497.6053 | Lower | -1653.9477 | Lower |
| Halstead Effort | 27850.3428 | **19699.5282 ✅** | 131522.1360 | +8150.8145 | Higher | -103671.7933 | Lower | -111822.6078 | Lower |
| SLOC | 48 | **37 ✅** | 93 | +11 | Higher | -45 | Lower | -56 | Lower |
| PLOC | 34 | **27 ✅** | 73 | +7 | Higher | -39 | Lower | -46 | Lower |
| LLOC | 24 | **22 ✅** | 50 | +2 | Higher | -26 | Lower | -28 | Lower |
| Comments (CLOC) | **8 ✅** | 4 | 5 | +4 | Higher | +3 | Higher | -1 | Lower |
| Blank Lines | 7 | **6 ✅** | 15 | +1 | Higher | -8 | Lower | -9 | Lower |
| Functions | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 6 | +1 | Higher | -5 | Lower | -6 | Lower |

## p02945

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.8912 | **83.9916 ✅** | 78.2494 | -4.1005 | Lower | +1.6418 | Higher | +5.7423 | Higher |
| MI (SEI) | 66.1041 | 66.7093 | **68.5805 ✅** | -0.6052 | Lower | -2.4764 | Lower | -1.8713 | Lower |
| Cyclomatic (sum) | 5 | **4 ✅** | 7 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 666.9203 | **605 ✅** | 760.5802 | +61.9203 | Higher | -93.6599 | Lower | -155.5802 | Lower |
| Halstead Effort | 12829.4410 | **9175.8333 ✅** | 14197.4969 | +3653.6077 | Higher | -1368.0559 | Lower | -5021.6636 | Lower |
| SLOC | 32 | **26 ✅** | 33 | +6 | Higher | -1 | Lower | -7 | Lower |
| PLOC | 22 | **19 ✅** | 22 | +3 | Higher | 0 | Same | -3 | Lower |
| LLOC | 19 | **16 ✅** | 19 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 4 | 2 | **6 ✅** | +2 | Higher | -2 | Lower | -4 | Lower |
| Blank Lines | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 83.1965 | 76.1377 | **83.9457 ✅** | +7.0588 | Higher | -0.7491 | Lower | -7.8080 | Lower |
| MI (SEI) | **75.1779 ✅** | 72.8866 | 70.9285 | +2.2913 | Higher | +4.2494 | Higher | +1.9581 | Higher |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **559.6209 ✅** | 803.6111 | 610.3717 | -243.9902 | Lower | -50.7508 | Lower | +193.2395 | Higher |
| Halstead Effort | **8674.1241 ✅** | 15451.2506 | 10053.1808 | -6777.1265 | Lower | -1379.0566 | Lower | +5398.0698 | Higher |
| SLOC | 28 | 38 | **26 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| PLOC | 18 | 19 | 18 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 5 | **12 ✅** | 3 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02953

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.0296 ✅** | 73.5266 | 70.1854 | +6.5030 | Higher | +9.8441 | Higher | +3.3412 | Higher |
| MI (SEI) | **67.1680 ✅** | 64.8157 | 58.9535 | +2.3524 | Higher | +8.2145 | Higher | +5.8621 | Higher |
| Cyclomatic (sum) | **6 ✅** | 9 | 9 | -3 | Lower | -3 | Lower | 0 | Same |
| Cognitive (sum) | **6 ✅** | 7 | 10 | -1 | Lower | -4 | Lower | -3 | Lower |
| Halstead Volume | **759.6771 ✅** | 948.1678 | 1021.5380 | -188.4908 | Lower | -261.8609 | Lower | -73.3701 | Lower |
| Halstead Effort | **17040.9378 ✅** | 25819.3396 | 32178.4461 | -8778.4018 | Lower | -15137.5083 | Lower | -6359.1064 | Lower |
| SLOC | **30 ✅** | 40 | 48 | -10 | Lower | -18 | Lower | -8 | Lower |
| PLOC | **22 ✅** | 26 | 31 | -4 | Lower | -9 | Lower | -5 | Lower |
| LLOC | 16 | 16 | 22 | 0 | Same | -6 | Lower | -6 | Lower |
| Comments (CLOC) | 4 | 9 | **10 ✅** | -5 | Lower | -6 | Lower | -1 | Lower |
| Blank Lines | **4 ✅** | 6 | 8 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p02957

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.8068 ✅** | 88.5400 | 83.3987 | +0.2667 | Higher | +5.4080 | Higher | +5.1413 | Higher |
| MI (SEI) | 75.7299 | 75.3451 | **79.8949 ✅** | +0.3848 | Higher | -4.1650 | Lower | -4.5498 | Lower |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **487.2819 ✅** | 512.9283 | 562.6210 | -25.6464 | Lower | -75.3391 | Lower | -49.6927 | Lower |
| Halstead Effort | 6673.0547 | **5770.4434 ✅** | 7890.7590 | +902.6113 | Higher | -1217.7043 | Lower | -2120.3156 | Lower |
| SLOC | 21 | 21 | 28 | 0 | Same | -7 | Lower | -7 | Lower |
| PLOC | **14 ✅** | 15 | 16 | -1 | Lower | -2 | Lower | -1 | Lower |
| LLOC | 10 | 10 | 12 | 0 | Same | -2 | Lower | -2 | Lower |
| Comments (CLOC) | 2 | 2 | **7 ✅** | 0 | Same | -5 | Lower | -5 | Lower |
| Blank Lines | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02959

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.9650 | **70.7074 ✅** | 64.7682 | -0.7423 | Lower | +5.1969 | Higher | +5.9392 | Higher |
| MI (SEI) | 58.3894 | **61.7888 ✅** | 44.8971 | -3.3994 | Lower | +13.4923 | Higher | +16.8917 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 11 | +4 | Higher | -2 | Lower | -6 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 16 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 1397.6145 | **1102.8052 ✅** | 1639.4953 | +294.8093 | Higher | -241.8808 | Lower | -536.6901 | Lower |
| Halstead Effort | 58816.2766 | **33084.1556 ✅** | 79767.7509 | +25732.1210 | Higher | -20951.4742 | Lower | -46683.5952 | Lower |
| SLOC | **44 ✅** | 48 | 56 | -4 | Lower | -12 | Lower | -8 | Lower |
| PLOC | 27 | 27 | 38 | 0 | Same | -11 | Lower | -11 | Lower |
| LLOC | 29 | **27 ✅** | 29 | +2 | Higher | 0 | Same | -2 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 7 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 10 | **9 ✅** | 11 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02988

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.2418 | 66.1826 | **72.8659 ✅** | +1.0593 | Higher | -5.6241 | Lower | -6.6834 | Lower |
| MI (SEI) | 50.8403 | 48.8981 | **63.0985 ✅** | +1.9421 | Higher | -12.2583 | Lower | -14.2004 | Lower |
| Cyclomatic (sum) | 12 | 17 | **8 ✅** | -5 | Lower | +4 | Higher | +9 | Higher |
| Cognitive (sum) | 16 | 12 | **11 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Halstead Volume | 1157.1748 | 1363.5110 | **966.6233 ✅** | -206.3362 | Lower | +190.5514 | Higher | +396.8876 | Higher |
| Halstead Effort | 34694.5793 | 47330.3575 | **28354.2847 ✅** | -12635.7783 | Lower | +6340.2945 | Higher | +18976.0728 | Higher |
| SLOC | 53 | 50 | **42 ✅** | +3 | Higher | +11 | Higher | +8 | Higher |
| PLOC | 39 | 35 | **26 ✅** | +4 | Higher | +13 | Higher | +9 | Higher |
| LLOC | 25 | 22 | **20 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| Comments (CLOC) | 8 | 7 | **9 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | **6 ✅** | 9 | 7 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p03003

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.4411 | 61.1533 | **66.7475 ✅** | +5.2878 | Higher | -0.3063 | Lower | -5.5941 | Lower |
| MI (SEI) | 39.8969 | **51.4206 ✅** | 43.2330 | -11.5238 | Lower | -3.3361 | Lower | +8.1877 | Higher |
| Cyclomatic (sum) | 12 | 15 | 12 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 17 | **11 ✅** | 17 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | **1618.4880 ✅** | 1815.9741 | 1625.0220 | -197.4861 | Lower | -6.5340 | Lower | +190.9521 | Higher |
| Halstead Effort | 88235.5022 | **79104.9328 ✅** | 85638.6610 | +9130.5695 | Higher | +2596.8412 | Higher | -6533.7283 | Lower |
| SLOC | 50 | 64 | **49 ✅** | -14 | Lower | +1 | Higher | +15 | Higher |
| PLOC | 39 | **38 ✅** | 39 | +1 | Higher | 0 | Same | -1 | Lower |
| LLOC | 32 | **27 ✅** | 32 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | 3 | **19 ✅** | 4 | -16 | Lower | -1 | Lower | +15 | Higher |
| Blank Lines | 8 | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03029

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **94.2147 ✅** | 93.0501 | 93.3304 | +1.1646 | Higher | +0.8844 | Higher | -0.2802 | Lower |
| MI (SEI) | 86.4631 | 89.0366 | **89.4409 ✅** | -2.5734 | Lower | -2.9777 | Lower | -0.4043 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 420 | 435 | **412.1788 ✅** | -15 | Lower | +7.8212 | Higher | +22.8212 | Higher |
| Halstead Effort | 4447.0588 | 4797.7941 | **3782.3467 ✅** | -350.7353 | Lower | +664.7121 | Higher | +1015.4474 | Higher |
| SLOC | **16 ✅** | 17 | 17 | -1 | Lower | -1 | Lower | 0 | Same |
| PLOC | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 2 | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03053

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.8406 ✅** | 46.8817 | 53.4280 | +13.9589 | Higher | +7.4126 | Higher | -6.5463 | Lower |
| MI (SEI) | **32.9200 ✅** | 25.3300 | 21.7691 | +7.5900 | Higher | +11.1509 | Higher | +3.5609 | Higher |
| Cyclomatic (sum) | **15 ✅** | 25 | 18 | -10 | Lower | -3 | Lower | +7 | Higher |
| Cognitive (sum) | **21 ✅** | 41 | 29 | -20 | Lower | -8 | Lower | +12 | Higher |
| Halstead Volume | **2239.6828 ✅** | 3005.0889 | 3125.8432 | -765.4061 | Lower | -886.1604 | Lower | -120.7543 | Lower |
| Halstead Effort | **74469.4523 ✅** | 147480.5169 | 165463.2670 | -73011.0646 | Lower | -90993.8148 | Lower | -17982.7502 | Lower |
| SLOC | **61 ✅** | 114 | 83 | -53 | Lower | -22 | Lower | +31 | Higher |
| PLOC | **48 ✅** | 83 | 67 | -35 | Lower | -19 | Lower | +16 | Higher |
| LLOC | **36 ✅** | 57 | 58 | -21 | Lower | -22 | Lower | -1 | Lower |
| Comments (CLOC) | 4 | **21 ✅** | 5 | -17 | Lower | -1 | Lower | +16 | Higher |
| Blank Lines | **9 ✅** | 10 | 11 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p03055

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.9453 | **51.3505 ✅** | 29.5189 | -2.4052 | Lower | +19.4264 | Higher | +21.8317 | Higher |
| MI (SEI) | **24.1480 ✅** | 18.9747 | -7.9670 | +5.1733 | Higher | +32.1150 | Higher | +26.9417 | Higher |
| Cyclomatic (sum) | **18 ✅** | 25 | 37 | -7 | Lower | -19 | Lower | -12 | Lower |
| Cognitive (sum) | **29 ✅** | 33 | 64 | -4 | Lower | -35 | Lower | -31 | Lower |
| Halstead Volume | 3355.1371 | **2850.1391 ✅** | 7539.7821 | +504.9980 | Higher | -4184.6450 | Lower | -4689.6430 | Lower |
| Halstead Effort | 216108.1069 | **151575.5791 ✅** | 793831.3401 | +64532.5278 | Higher | -577723.2332 | Lower | -642255.7610 | Lower |
| SLOC | 107 | **88 ✅** | 209 | +19 | Higher | -102 | Lower | -121 | Lower |
| PLOC | 81 | **73 ✅** | 177 | +8 | Higher | -96 | Lower | -104 | Lower |
| LLOC | 78 | **44 ✅** | 166 | +34 | Higher | -88 | Lower | -122 | Lower |
| Comments (CLOC) | 15 | 5 | **17 ✅** | +10 | Higher | -2 | Lower | -12 | Lower |
| Blank Lines | 13 | **10 ✅** | 18 | +3 | Higher | -5 | Lower | -8 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **1 ✅** | 4 | +1 | Higher | -2 | Lower | -3 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.4394 ✅** | 54.9309 | 56.3747 | +2.5085 | Higher | +1.0647 | Higher | -1.4438 | Lower |
| MI (SEI) | 38.2504 | 36.2084 | **40.4481 ✅** | +2.0420 | Higher | -2.1977 | Lower | -4.2397 | Lower |
| Cyclomatic (sum) | 15 | 18 | **12 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Cognitive (sum) | 17 | **15 ✅** | 18 | +2 | Higher | -1 | Lower | -3 | Lower |
| Halstead Volume | **2002.7753 ✅** | 2021.9216 | 2228.0114 | -19.1463 | Lower | -225.2361 | Lower | -206.0898 | Lower |
| Halstead Effort | **74388.7959 ✅** | 87220.6428 | 91719.8026 | -12831.8469 | Lower | -17331.0067 | Lower | -4499.1598 | Lower |
| SLOC | **78 ✅** | 87 | 84 | -9 | Lower | -6 | Lower | +3 | Higher |
| PLOC | 55 | 58 | **54 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| LLOC | 42 | **41 ✅** | 51 | +1 | Higher | -9 | Lower | -10 | Lower |
| Comments (CLOC) | 13 | 16 | **19 ✅** | -3 | Lower | -6 | Lower | -3 | Lower |
| Blank Lines | **10 ✅** | 13 | 11 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 4 | **0 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |

## p03080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.8431 | **73.3526 ✅** | 71.7767 | -1.5095 | Lower | +0.0664 | Higher | +1.5759 | Higher |
| MI (SEI) | 58.3241 | **61.1093 ✅** | 56.2157 | -2.7852 | Lower | +2.1084 | Higher | +4.8935 | Higher |
| Cyclomatic (sum) | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 957.6739 | **830.9771 ✅** | 1013.8419 | +126.6968 | Higher | -56.1680 | Lower | -182.8649 | Lower |
| Halstead Effort | 24625.8996 | **18281.4953 ✅** | 25171.2477 | +6344.4043 | Higher | -545.3481 | Lower | -6889.7524 | Lower |
| SLOC | 43 | **41 ✅** | 43 | +2 | Higher | 0 | Same | -2 | Lower |
| PLOC | 32 | **28 ✅** | 29 | +4 | Higher | +3 | Higher | -1 | Lower |
| LLOC | 18 | **14 ✅** | 17 | +4 | Higher | +1 | Higher | -3 | Lower |
| Comments (CLOC) | 7 | 7 | 6 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 6 | 6 | 8 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.2464 | 76.7449 | **87.6152 ✅** | -0.4985 | Lower | -11.3688 | Lower | -10.8703 | Lower |
| MI (SEI) | 63.9798 | 55.2267 | **78.0756 ✅** | +8.7531 | Higher | -14.0957 | Lower | -22.8489 | Lower |
| Cyclomatic (sum) | 13 | 13 | **3 ✅** | 0 | Same | +10 | Higher | +10 | Higher |
| Cognitive (sum) | 6 | 6 | **2 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Halstead Volume | 857.3785 | 857.3785 | **530.0990 ✅** | 0 | Same | +327.2795 | Higher | +327.2795 | Higher |
| Halstead Effort | 19096.1568 | 19096.1568 | **7454.5173 ✅** | 0 | Same | +11641.6396 | Higher | +11641.6396 | Higher |
| SLOC | 33 | 32 | **22 ✅** | +1 | Higher | +11 | Higher | +10 | Higher |
| PLOC | 24 | 24 | **15 ✅** | 0 | Same | +9 | Higher | +9 | Higher |
| LLOC | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | **5 ✅** | 2 | 3 | +3 | Higher | +2 | Higher | -1 | Lower |
| Blank Lines | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | 3 | **0 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Exits | 4 | 4 | **0 ✅** | 0 | Same | +4 | Higher | +4 | Higher |

## p03106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.7826 ✅** | 71.9921 | 59.0384 | +3.7905 | Higher | +16.7442 | Higher | +12.9537 | Higher |
| MI (SEI) | **68.5688 ✅** | 61.9397 | 36.0957 | +6.6291 | Higher | +32.4731 | Higher | +25.8439 | Higher |
| Cyclomatic (sum) | **7 ✅** | 9 | 19 | -2 | Lower | -12 | Lower | -10 | Lower |
| Cognitive (sum) | 8 | **6 ✅** | 42 | +2 | Higher | -34 | Lower | -36 | Lower |
| Halstead Volume | **787.5601 ✅** | 1094.0328 | 1583.2352 | -306.4727 | Lower | -795.6750 | Lower | -489.2024 | Lower |
| Halstead Effort | **16876.2888 ✅** | 37372.1607 | 75892.0337 | -20495.8719 | Lower | -59015.7448 | Lower | -38519.8730 | Lower |
| SLOC | **38 ✅** | 42 | 72 | -4 | Lower | -34 | Lower | -30 | Lower |
| PLOC | **22 ✅** | 28 | 60 | -6 | Lower | -38 | Lower | -32 | Lower |
| LLOC | **18 ✅** | 20 | 48 | -2 | Lower | -30 | Lower | -28 | Lower |
| Comments (CLOC) | 9 | 9 | 8 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 7 | 6 | **4 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.3072 ✅** | 49.9902 | 50.3913 | +1.3170 | Higher | +0.9158 | Higher | -0.4011 | Lower |
| MI (SEI) | **25.5264 ✅** | 24.1879 | 21.0407 | +1.3385 | Higher | +4.4857 | Higher | +3.1472 | Higher |
| Cyclomatic (sum) | 15 | 17 | 15 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | **8 ✅** | 10 | 15 | -2 | Lower | -7 | Lower | -5 | Lower |
| Halstead Volume | **3098.9720 ✅** | 3134.1922 | 3695.7851 | -35.2201 | Lower | -596.8130 | Lower | -561.5929 | Lower |
| Halstead Effort | 134557.3663 | **133902.7636 ✅** | 275421.9366 | +654.6027 | Higher | -140864.5704 | Lower | -141519.1730 | Lower |
| SLOC | 99 | 104 | 99 | -5 | Lower | 0 | Same | +5 | Higher |
| PLOC | **74 ✅** | 79 | 77 | -5 | Lower | -3 | Lower | +2 | Higher |
| LLOC | 51 | **50 ✅** | 69 | +1 | Higher | -18 | Lower | -19 | Lower |
| Comments (CLOC) | 12 | **13 ✅** | 9 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 14 | 13 | 13 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 6 | 5 | **4 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | 7 | **4 ✅** | 0 | Same | +3 | Higher | +3 | Higher |

## p03115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.7444 | **94.5884 ✅** | 61.1855 | -33.8440 | Lower | -0.4411 | Lower | +33.4029 | Higher |
| MI (SEI) | 32.5900 | **105.4216 ✅** | 30.9113 | -72.8317 | Lower | +1.6787 | Higher | +74.5104 | Higher |
| Cyclomatic (sum) | 16 | **2 ✅** | 16 | +14 | Higher | 0 | Same | -14 | Lower |
| Cognitive (sum) | 13 | **0 ✅** | 13 | +13 | Higher | 0 | Same | -13 | Lower |
| Halstead Volume | 1974.0688 | **195.0420 ✅** | 1906.2070 | +1779.0269 | Higher | +67.8618 | Higher | -1711.1651 | Lower |
| Halstead Effort | 93430.5995 | **1440.3099 ✅** | 87484.8695 | +91990.2897 | Higher | +5945.7301 | Higher | -86044.5596 | Lower |
| SLOC | 63 | **20 ✅** | 62 | +43 | Higher | +1 | Higher | -42 | Lower |
| PLOC | 51 | **7 ✅** | 51 | +44 | Higher | 0 | Same | -44 | Lower |
| LLOC | 34 | **3 ✅** | 34 | +31 | Higher | 0 | Same | -31 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 3 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 8 | **3 ✅** | 8 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **0 ✅** | 6 | +6 | Higher | 0 | Same | -6 | Lower |

## p03142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **50.6126 ✅** | 49.8515 | 49.6271 | +0.7611 | Higher | +0.9855 | Higher | +0.2244 | Higher |
| MI (SEI) | 22.7854 | **25.7282 ✅** | 24.6239 | -2.9428 | Lower | -1.8384 | Lower | +1.1043 | Higher |
| Cyclomatic (sum) | 29 | 26 | **24 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| Cognitive (sum) | 42 | 30 | **21 ✅** | +12 | Higher | +21 | Higher | +9 | Higher |
| Halstead Volume | 2752.0481 | **2685.9760 ✅** | 3378.8265 | +66.0720 | Higher | -626.7784 | Lower | -692.8504 | Lower |
| Halstead Effort | 167186.9205 | **158401.9031 ✅** | 183649.1570 | +8785.0175 | Higher | -16462.2364 | Lower | -25247.2539 | Lower |
| SLOC | **88 ✅** | 97 | 94 | -9 | Lower | -6 | Lower | +3 | Higher |
| PLOC | 70 | 74 | **68 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| LLOC | 47 | 47 | 51 | 0 | Same | -4 | Lower | -4 | Lower |
| Comments (CLOC) | 8 | **13 ✅** | 12 | -5 | Lower | -4 | Lower | +1 | Higher |
| Blank Lines | 10 | 10 | 15 | 0 | Same | -5 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | **2 ✅** | 6 | +2 | Higher | -2 | Lower | -4 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03145

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **91.5500 ✅** | 80.2036 | 87.1739 | +11.3464 | Higher | +4.3761 | Higher | -6.9703 | Lower |
| MI (SEI) | **89.9361 ✅** | 76.4872 | 87.9853 | +13.4489 | Higher | +1.9507 | Higher | -11.4981 | Lower |
| Cyclomatic (sum) | 2 | 6 | 2 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **485.7822 ✅** | 734.6709 | 525.1448 | -248.8887 | Lower | -39.3626 | Lower | +209.5261 | Higher |
| Halstead Effort | **4829.2463 ✅** | 13538.9353 | 5309.7975 | -8709.6890 | Lower | -480.5512 | Lower | +8229.1378 | Higher |
| SLOC | **18 ✅** | 30 | 23 | -12 | Lower | -5 | Lower | +7 | Higher |
| PLOC | **11 ✅** | 16 | 12 | -5 | Lower | -1 | Lower | +4 | Higher |
| LLOC | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 4 | **8 ✅** | 7 | -4 | Lower | -3 | Lower | +1 | Higher |
| Blank Lines | **3 ✅** | 6 | 5 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03157

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.2313 | 47.1207 | **52.5755 ✅** | +1.1106 | Higher | -4.3443 | Lower | -5.4549 | Lower |
| MI (SEI) | 17.3488 | **24.7042 ✅** | 23.2776 | -7.3554 | Lower | -5.9289 | Lower | +1.4266 | Higher |
| Cyclomatic (sum) | 30 | 27 | **25 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| Cognitive (sum) | 51 | **42 ✅** | 48 | +9 | Higher | +3 | Higher | -6 | Lower |
| Halstead Volume | 3279.2588 | 3200.4836 | **3030.4483 ✅** | +78.7753 | Higher | +248.8105 | Higher | +170.0353 | Higher |
| Halstead Effort | 196755.5303 | **154708.1214 ✅** | 171189.4061 | +42047.4089 | Higher | +25566.1242 | Higher | -16481.2847 | Lower |
| SLOC | 95 | 107 | **80 ✅** | -12 | Lower | +15 | Higher | +27 | Higher |
| PLOC | 75 | 76 | **66 ✅** | -1 | Lower | +9 | Higher | +10 | Higher |
| LLOC | 52 | 55 | **46 ✅** | -3 | Lower | +6 | Higher | +9 | Higher |
| Comments (CLOC) | 7 | **18 ✅** | 6 | -11 | Lower | +1 | Higher | +12 | Higher |
| Blank Lines | 13 | 13 | **10 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p03164

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.7464 | 44.5736 | **56.1273 ✅** | +9.1728 | Higher | -2.3809 | Lower | -11.5537 | Lower |
| MI (SEI) | 31.2567 | 23.3523 | **34.5759 ✅** | +7.9044 | Higher | -3.3192 | Lower | -11.2236 | Lower |
| Cyclomatic (sum) | 25 | 36 | **23 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| Cognitive (sum) | 22 | 33 | **21 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| Halstead Volume | 2240.3208 | 3042.7407 | **2131.9182 ✅** | -802.4199 | Lower | +108.4026 | Higher | +910.8225 | Higher |
| Halstead Effort | 116997.8074 | 184548.8381 | **97703.5672 ✅** | -67551.0307 | Lower | +19294.2401 | Higher | +86845.2708 | Higher |
| SLOC | 82 | 112 | **74 ✅** | -30 | Lower | +8 | Higher | +38 | Higher |
| PLOC | 59 | 80 | **53 ✅** | -21 | Lower | +6 | Higher | +27 | Higher |
| LLOC | 38 | 44 | **33 ✅** | -6 | Lower | +5 | Higher | +11 | Higher |
| Comments (CLOC) | 11 | **21 ✅** | 10 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | 12 | 11 | 11 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03165

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.1443 ✅** | 60.8847 | 56.3443 | +0.2597 | Higher | +4.8000 | Higher | +4.5403 | Higher |
| MI (SEI) | **41.3661 ✅** | 40.6316 | 30.0711 | +0.7344 | Higher | +11.2949 | Higher | +10.5605 | Higher |
| Cyclomatic (sum) | **12 ✅** | 14 | 18 | -2 | Lower | -6 | Lower | -4 | Lower |
| Cognitive (sum) | **21 ✅** | 23 | 23 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | 2181.7072 | **1816.0299 ✅** | 2347.5154 | +365.6773 | Higher | -165.8082 | Lower | -531.4855 | Lower |
| Halstead Effort | 112682.2272 | **89733.2409 ✅** | 138082.0594 | +22948.9863 | Higher | -25399.8322 | Lower | -48348.8186 | Lower |
| SLOC | **63 ✅** | 66 | 76 | -3 | Lower | -13 | Lower | -10 | Lower |
| PLOC | **46 ✅** | 48 | 60 | -2 | Lower | -14 | Lower | -12 | Lower |
| LLOC | 40 | **34 ✅** | 43 | +6 | Higher | -3 | Lower | -9 | Lower |
| Comments (CLOC) | 9 | 9 | 7 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | **8 ✅** | 9 | 10 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **0 ✅** | 2 | 3 | -2 | Lower | -3 | Lower | -1 | Lower |

## p03173

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.1855 | 57.8932 | **64.1936 ✅** | +6.2922 | Higher | -0.0081 | Lower | -6.3004 | Lower |
| MI (SEI) | **44.0565 ✅** | 31.2147 | 40.6477 | +12.8418 | Higher | +3.4088 | Higher | -9.4330 | Lower |
| Cyclomatic (sum) | **11 ✅** | 16 | 12 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | **8 ✅** | 22 | 11 | -14 | Lower | -3 | Lower | +11 | Higher |
| Halstead Volume | **1833.8925 ✅** | 2353.6426 | 1852.9568 | -519.7501 | Lower | -19.0643 | Lower | +500.6858 | Higher |
| Halstead Effort | **79224.1544 ✅** | 108032.1956 | 94037.5564 | -28808.0412 | Lower | -14813.4020 | Lower | +13994.6392 | Higher |
| SLOC | 56 | 71 | **55 ✅** | -15 | Lower | +1 | Higher | +16 | Higher |
| PLOC | 40 | 55 | 40 | -15 | Lower | 0 | Same | +15 | Higher |
| LLOC | 29 | 32 | 29 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | **7 ✅** | 6 | 5 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | 5 | **3 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03183

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.1988 | **65.9913 ✅** | 58.3458 | -1.7925 | Lower | +5.8530 | Higher | +7.6454 | Higher |
| MI (SEI) | 48.7237 | **55.2493 ✅** | 34.1324 | -6.5257 | Lower | +14.5912 | Higher | +21.1169 | Higher |
| Cyclomatic (sum) | 10 | 10 | 14 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | **9 ✅** | 11 | 22 | -2 | Lower | -13 | Lower | -11 | Lower |
| Halstead Volume | 1542.1382 | **1281.7953 ✅** | 1987.0251 | +260.3430 | Higher | -444.8869 | Lower | -705.2298 | Lower |
| Halstead Effort | 37242.6380 | **29054.0259 ✅** | 62893.6633 | +8188.6121 | Higher | -25651.0253 | Lower | -33839.6374 | Lower |
| SLOC | 60 | **57 ✅** | 75 | +3 | Higher | -15 | Lower | -18 | Lower |
| PLOC | 41 | **35 ✅** | 58 | +6 | Higher | -17 | Lower | -23 | Lower |
| LLOC | 24 | **21 ✅** | 37 | +3 | Higher | -13 | Lower | -16 | Lower |
| Comments (CLOC) | 11 | **14 ✅** | 8 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03194

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.8457 ✅** | 54.7441 | 71.5194 | +19.1017 | Higher | +2.3263 | Higher | -16.7753 | Lower |
| MI (SEI) | 59.9845 | 32.6748 | **63.8562 ✅** | +27.3097 | Higher | -3.8717 | Lower | -31.1814 | Lower |
| Cyclomatic (sum) | 9 | 16 | **8 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Cognitive (sum) | 11 | 30 | **10 ✅** | -19 | Lower | +1 | Higher | +20 | Higher |
| Halstead Volume | **891.7293 ✅** | 2060.2497 | 943.2713 | -1168.5204 | Lower | -51.5420 | Lower | +1116.9785 | Higher |
| Halstead Effort | **31210.5247 ✅** | 100121.4907 | 31835.4048 | -68910.9660 | Lower | -624.8801 | Lower | +68286.0859 | Higher |
| SLOC | **40 ✅** | 90 | 46 | -50 | Lower | -6 | Lower | +44 | Higher |
| PLOC | 28 | 71 | 28 | -43 | Lower | 0 | Same | +43 | Higher |
| LLOC | 25 | 67 | 25 | -42 | Lower | 0 | Same | +42 | Higher |
| Comments (CLOC) | 6 | **13 ✅** | 12 | -7 | Lower | -6 | Lower | +1 | Higher |
| Blank Lines | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03206

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.9675 ✅** | 84.6799 | 85.1318 | +3.2876 | Higher | +2.8356 | Higher | -0.4519 | Lower |
| MI (SEI) | 74.3352 | 63.4585 | **76.8698 ✅** | +10.8767 | Higher | -2.5346 | Lower | -13.4113 | Lower |
| Cyclomatic (sum) | **6 ✅** | 11 | 7 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **433.8179 ✅** | 569.7578 | 480.8030 | -135.9399 | Lower | -46.9851 | Lower | +88.9548 | Higher |
| Halstead Effort | **4635.0017 ✅** | 6705.6110 | 5494.8917 | -2070.6093 | Lower | -859.8900 | Lower | +1210.7193 | Higher |
| SLOC | **22 ✅** | 23 | 25 | -1 | Lower | -3 | Lower | -2 | Lower |
| PLOC | 18 | 19 | 18 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 11 | **4 ✅** | 11 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 2 | 1 | **4 ✅** | +1 | Higher | -2 | Lower | -3 | Lower |
| Blank Lines | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03209

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.6054 | 62.0660 | **80.7859 ✅** | +14.5394 | Higher | -4.1804 | Lower | -18.7199 | Lower |
| MI (SEI) | 60.9677 | 55.6309 | **61.5750 ✅** | +5.3368 | Higher | -0.6073 | Lower | -5.9441 | Lower |
| Cyclomatic (sum) | 8 | 10 | **6 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 7 | 11 | **6 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Halstead Volume | 909.5830 | 1727.0200 | **814.3479 ✅** | -817.4370 | Lower | +95.2351 | Higher | +912.6721 | Higher |
| Halstead Effort | 21863.6801 | 104484.7096 | **18285.8120 ✅** | -82621.0295 | Lower | +3577.8681 | Higher | +86198.8976 | Higher |
| SLOC | 34 | 66 | **28 ✅** | -32 | Lower | +6 | Higher | +38 | Higher |
| PLOC | 25 | 37 | **21 ✅** | -12 | Lower | +4 | Higher | +16 | Higher |
| LLOC | 11 | 23 | **10 ✅** | -12 | Lower | +1 | Higher | +13 | Higher |
| Comments (CLOC) | 4 | **25 ✅** | 2 | -21 | Lower | +2 | Higher | +23 | Higher |
| Blank Lines | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p03213

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.5945 ✅** | 60.3163 | 65.0852 | +8.2782 | Higher | +3.5093 | Higher | -4.7689 | Lower |
| MI (SEI) | **49.6379 ✅** | 43.5129 | 39.7800 | +6.1250 | Higher | +9.8579 | Higher | +3.7329 | Higher |
| Cyclomatic (sum) | **8 ✅** | 14 | 11 | -6 | Lower | -3 | Lower | +3 | Higher |
| Cognitive (sum) | **11 ✅** | 24 | 14 | -13 | Lower | -3 | Lower | +10 | Higher |
| Halstead Volume | **1276.7420 ✅** | 1613.5872 | 1727.5790 | -336.8452 | Lower | -450.8370 | Lower | -113.9918 | Lower |
| Halstead Effort | **41653.7070 ✅** | 65112.9894 | 71867.2864 | -23459.2824 | Lower | -30213.5794 | Lower | -6754.2970 | Lower |
| SLOC | **50 ✅** | 71 | 54 | -21 | Lower | -4 | Lower | +17 | Higher |
| PLOC | **37 ✅** | 51 | 44 | -14 | Lower | -7 | Lower | +7 | Higher |
| LLOC | **28 ✅** | 43 | 30 | -15 | Lower | -2 | Lower | +13 | Higher |
| Comments (CLOC) | 6 | **13 ✅** | 4 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p03218

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.0307 | **73.0153 ✅** | 66.5480 | -1.9847 | Lower | +4.4827 | Higher | +6.4673 | Higher |
| MI (SEI) | 55.8981 | **58.1765 ✅** | 47.8925 | -2.2784 | Lower | +8.0056 | Higher | +10.2839 | Higher |
| Cyclomatic (sum) | 7 | **6 ✅** | 11 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 5 | **3 ✅** | 9 | +2 | Higher | -4 | Lower | -6 | Lower |
| Halstead Volume | **1083.0835 ✅** | 1106.1229 | 1303.9690 | -23.0394 | Lower | -220.8854 | Lower | -197.8461 | Lower |
| Halstead Effort | 26199.4175 | **24390.0109 ✅** | 31295.2558 | +1809.4066 | Higher | -5095.8383 | Lower | -6905.2449 | Lower |
| SLOC | 46 | **41 ✅** | 54 | +5 | Higher | -8 | Lower | -13 | Lower |
| PLOC | 31 | **29 ✅** | 40 | +2 | Higher | -9 | Lower | -11 | Lower |
| LLOC | 24 | 22 | **18 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Comments (CLOC) | 7 | 6 | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p03268

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 83.8056 | **83.8847 ✅** | 82.3552 | -0.0791 | Lower | +1.4504 | Higher | +1.5295 | Higher |
| MI (SEI) | 70.6246 | **77.0397 ✅** | 73.8623 | -6.4151 | Lower | -3.2377 | Lower | +3.1774 | Higher |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 655.3899 | **645.4951 ✅** | 687.6478 | +9.8948 | Higher | -32.2580 | Lower | -42.1528 | Lower |
| Halstead Effort | 11797.0178 | 12587.1541 | **11264.3266 ✅** | -790.1364 | Lower | +532.6912 | Higher | +1322.8275 | Higher |
| SLOC | 26 | 26 | 28 | 0 | Same | -2 | Lower | -2 | Lower |
| PLOC | 18 | **15 ✅** | 18 | +3 | Higher | 0 | Same | -3 | Lower |
| LLOC | 11 | 12 | 11 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 3 | 5 | 5 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0873 | **81.7347 ✅** | 52.7589 | -28.6475 | Lower | +0.3283 | Higher | +28.9758 | Higher |
| MI (SEI) | 27.5519 | **84.6047 ✅** | 24.4267 | -57.0528 | Lower | +3.1252 | Higher | +60.1780 | Higher |
| Cyclomatic (sum) | 16 | **4 ✅** | 17 | +12 | Higher | -1 | Lower | -13 | Lower |
| Cognitive (sum) | 12 | **1 ✅** | 15 | +11 | Higher | -3 | Lower | -14 | Lower |
| Halstead Volume | 2317.2461 | **664.5142 ✅** | 2887.3958 | +1652.7318 | Higher | -570.1497 | Lower | -2222.8816 | Lower |
| Halstead Effort | 107306.3180 | **11338.2740 ✅** | 178193.5703 | +95968.0441 | Higher | -70887.2523 | Lower | -166855.2963 | Lower |
| SLOC | 96 | **29 ✅** | 90 | +67 | Higher | +6 | Higher | -61 | Lower |
| PLOC | 71 | **15 ✅** | 71 | +56 | Higher | 0 | Same | -56 | Lower |
| LLOC | 46 | **11 ✅** | 56 | +35 | Higher | -10 | Lower | -45 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 8 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 15 | **4 ✅** | 14 | +11 | Higher | +1 | Higher | -10 | Lower |
| Functions | 6 | **1 ✅** | 5 | +5 | Higher | +1 | Higher | -4 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |

## p03278

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.3578 | **78.8314 ✅** | 38.4325 | -36.4736 | Lower | +3.9253 | Higher | +40.3989 | Higher |
| MI (SEI) | 5.9226 | **81.0086 ✅** | 0.3334 | -75.0860 | Lower | +5.5892 | Higher | +80.6753 | Higher |
| Cyclomatic (sum) | 20 | **4 ✅** | 22 | +16 | Higher | -2 | Lower | -18 | Lower |
| Cognitive (sum) | 26 | **2 ✅** | 25 | +24 | Higher | +1 | Higher | -23 | Lower |
| Halstead Volume | 5407.8921 | **543.7040 ✅** | 6966.8903 | +4864.1881 | Higher | -1558.9983 | Lower | -6423.1863 | Lower |
| Halstead Effort | 532836.4228 | **6409.9840 ✅** | 758379.7227 | +526426.4388 | Higher | -225543.2999 | Lower | -751969.7387 | Lower |
| SLOC | 134 | **37 ✅** | 153 | +97 | Higher | -19 | Lower | -116 | Lower |
| PLOC | 107 | **16 ✅** | 121 | +91 | Higher | -14 | Lower | -105 | Lower |
| LLOC | 92 | **11 ✅** | 106 | +81 | Higher | -14 | Lower | -95 | Lower |
| Comments (CLOC) | 8 | **16 ✅** | 9 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 19 | **5 ✅** | 23 | +14 | Higher | -4 | Lower | -18 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p03294

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.1305 ✅** | 83.9159 | 87.1089 | +5.2146 | Higher | +2.0216 | Higher | -3.1930 | Lower |
| MI (SEI) | 80.8252 | 76.6899 | **83.3323 ✅** | +4.1352 | Higher | -2.5071 | Lower | -6.6424 | Lower |
| Cyclomatic (sum) | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **457.8717 ✅** | 545.7845 | 508.7463 | -87.9128 | Lower | -50.8746 | Lower | +37.0382 | Higher |
| Halstead Effort | **5494.4599 ✅** | 7859.2962 | 6557.1743 | -2364.8364 | Lower | -1062.7145 | Lower | +1302.1219 | Higher |
| SLOC | **21 ✅** | 27 | 23 | -6 | Lower | -2 | Lower | +4 | Higher |
| PLOC | 13 | 16 | 13 | -3 | Lower | 0 | Same | +3 | Higher |
| LLOC | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Comments (CLOC) | 3 | 5 | 5 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03296

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.8346 | 73.8997 | **79.9888 ✅** | +4.9349 | Higher | -1.1542 | Lower | -6.0891 | Lower |
| MI (SEI) | 71.2669 | 52.3109 | **73.7801 ✅** | +18.9560 | Higher | -2.5132 | Lower | -21.4692 | Lower |
| Cyclomatic (sum) | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 742.4827 | 922.4348 | **722.5705 ✅** | -179.9521 | Lower | +19.9122 | Higher | +199.8643 | Higher |
| Halstead Effort | 20047.0329 | 23675.8277 | **15896.5518 ✅** | -3628.7948 | Lower | +4150.4811 | Higher | +7779.2760 | Higher |
| SLOC | 33 | 40 | **31 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| PLOC | 22 | 31 | **21 ✅** | -9 | Lower | +1 | Higher | +10 | Higher |
| LLOC | 19 | 26 | 19 | -7 | Lower | 0 | Same | +7 | Higher |
| Comments (CLOC) | 7 | 3 | 7 | +4 | Higher | 0 | Same | -4 | Lower |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03311

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.9552 | **76.9259 ✅** | 52.3361 | -15.9707 | Lower | +8.6191 | Higher | +24.5898 | Higher |
| MI (SEI) | 35.0212 | **71.7420 ✅** | 29.0440 | -36.7208 | Lower | +5.9773 | Higher | +42.6980 | Higher |
| Cyclomatic (sum) | 10 | **4 ✅** | 17 | +6 | Higher | -7 | Lower | -13 | Lower |
| Cognitive (sum) | 16 | **2 ✅** | 24 | +14 | Higher | -8 | Lower | -22 | Lower |
| Halstead Volume | 1703.2039 | **784.3458 ✅** | 2327.3478 | +918.8581 | Higher | -624.1439 | Lower | -1543.0020 | Lower |
| Halstead Effort | 81242.8253 | **14824.1356 ✅** | 110463.8745 | +66418.6897 | Higher | -29221.0492 | Lower | -95639.7389 | Lower |
| SLOC | 71 | **37 ✅** | 99 | +34 | Higher | -28 | Lower | -62 | Lower |
| PLOC | 52 | **20 ✅** | 69 | +32 | Higher | -17 | Lower | -49 | Lower |
| LLOC | 51 | **16 ✅** | 58 | +35 | Higher | -7 | Lower | -42 | Lower |
| Comments (CLOC) | 6 | 10 | **14 ✅** | -4 | Lower | -8 | Lower | -4 | Lower |
| Blank Lines | 13 | **7 ✅** | 16 | +6 | Higher | -3 | Lower | -9 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p03317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **83.9696 ✅** | 79.1383 | 69.1384 | +4.8313 | Higher | +14.8312 | Higher | +9.9999 | Higher |
| MI (SEI) | 74.8876 | **82.5064 ✅** | 59.3690 | -7.6187 | Lower | +15.5186 | Higher | +23.1373 | Higher |
| Cyclomatic (sum) | 4 | **3 ✅** | 9 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | 2 | **1 ✅** | 10 | +1 | Higher | -8 | Lower | -9 | Lower |
| Halstead Volume | 686.5287 | **583.4588 ✅** | 1100.1849 | +103.0699 | Higher | -413.6561 | Lower | -516.7261 | Lower |
| Halstead Effort | 14417.1032 | **8183.0093 ✅** | 29704.9914 | +6234.0939 | Higher | -15287.8882 | Lower | -21521.9820 | Lower |
| SLOC | **25 ✅** | 36 | 50 | -11 | Lower | -25 | Lower | -14 | Lower |
| PLOC | 16 | **14 ✅** | 32 | +2 | Higher | -16 | Lower | -18 | Lower |
| LLOC | 12 | **10 ✅** | 18 | +2 | Higher | -6 | Lower | -8 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 12 | -13 | Lower | -8 | Lower | +5 | Higher |
| Blank Lines | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03318

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.5973 ✅** | 58.1195 | 65.4548 | +12.4778 | Higher | +5.1425 | Higher | -7.3352 | Lower |
| MI (SEI) | **56.4187 ✅** | 44.7190 | 50.4338 | +11.6996 | Higher | +5.9848 | Higher | -5.7148 | Lower |
| Cyclomatic (sum) | **7 ✅** | 14 | 9 | -7 | Lower | -2 | Lower | +5 | Higher |
| Cognitive (sum) | **7 ✅** | 19 | 8 | -12 | Lower | -1 | Lower | +11 | Higher |
| Halstead Volume | **1031.0257 ✅** | 1765.2335 | 1266.0034 | -734.2078 | Lower | -234.9777 | Lower | +499.2301 | Higher |
| Halstead Effort | **34797.1168 ✅** | 71559.8507 | 41693.7111 | -36762.7339 | Lower | -6896.5943 | Lower | +29866.1396 | Higher |
| SLOC | **48 ✅** | 79 | 60 | -31 | Lower | -12 | Lower | +19 | Higher |
| PLOC | **32 ✅** | 50 | 39 | -18 | Lower | -7 | Lower | +11 | Higher |
| LLOC | **29 ✅** | 41 | 31 | -12 | Lower | -2 | Lower | +10 | Higher |
| Comments (CLOC) | 8 | **20 ✅** | 11 | -12 | Lower | -3 | Lower | +9 | Higher |
| Blank Lines | **8 ✅** | 10 | 11 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p03361

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.5232 ✅** | 64.3276 | 63.3056 | +2.1956 | Higher | +3.2177 | Higher | +1.0221 | Higher |
| MI (SEI) | 47.1349 | 41.4403 | **50.2222 ✅** | +5.6946 | Higher | -3.0873 | Lower | -8.7819 | Lower |
| Cyclomatic (sum) | **15 ✅** | 16 | 17 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 26 | **25 ✅** | 32 | +1 | Higher | -6 | Lower | -7 | Lower |
| Halstead Volume | **1311.6938 ✅** | 1602 | 1665.7431 | -290.3062 | Lower | -354.0494 | Lower | -63.7431 | Lower |
| Halstead Effort | **48602.9393 ✅** | 52606.2162 | 59490.8267 | -4003.2769 | Lower | -10887.8874 | Lower | -6884.6105 | Lower |
| SLOC | **51 ✅** | 54 | 56 | -3 | Lower | -5 | Lower | -2 | Lower |
| PLOC | 38 | 43 | **36 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| LLOC | 30 | 30 | **23 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Comments (CLOC) | 6 | 5 | **12 ✅** | +1 | Higher | -6 | Lower | -7 | Lower |
| Blank Lines | 7 | **6 ✅** | 8 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p03369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.0806 | **95.3925 ✅** | 86.5179 | -7.3119 | Lower | +1.5628 | Higher | +8.8747 | Higher |
| MI (SEI) | 78.8489 | **89.0267 ✅** | 79.7293 | -10.1778 | Lower | -0.8804 | Lower | +9.2974 | Higher |
| Cyclomatic (sum) | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 3 | **0 ✅** | 4 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | 463.7393 | **391.7346 ✅** | 521.7260 | +72.0047 | Higher | -57.9867 | Lower | -129.9914 | Lower |
| Halstead Effort | 6129.4238 | **4352.6071 ✅** | 8733.2403 | +1776.8167 | Higher | -2603.8165 | Lower | -4380.6332 | Lower |
| SLOC | 22 | **15 ✅** | 23 | +7 | Higher | -1 | Lower | -8 | Lower |
| PLOC | 15 | **9 ✅** | 15 | +6 | Higher | 0 | Same | -6 | Lower |
| LLOC | 11 | **6 ✅** | 11 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | 3 | 2 | **4 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03408

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.7664 | **70.4811 ✅** | 69.1118 | -2.7147 | Lower | -1.3455 | Lower | +1.3693 | Higher |
| MI (SEI) | 43.9121 | **54.9405 ✅** | 44.3023 | -11.0285 | Lower | -0.3902 | Lower | +10.6383 | Higher |
| Cyclomatic (sum) | 10 | **8 ✅** | 12 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 15 | **8 ✅** | 16 | +7 | Higher | -1 | Lower | -8 | Lower |
| Halstead Volume | 1212.7773 | **1077.0946 ✅** | 1174.2906 | +135.6827 | Higher | +38.4867 | Higher | -97.1960 | Lower |
| Halstead Effort | 33654.5708 | **25684.5631 ✅** | 35731.9849 | +7970.0077 | Higher | -2077.4140 | Lower | -10047.4218 | Lower |
| SLOC | 52 | 47 | 47 | +5 | Higher | +5 | Higher | 0 | Same |
| PLOC | 38 | **32 ✅** | 37 | +6 | Higher | +1 | Higher | -5 | Lower |
| LLOC | 30 | 26 | 26 | +4 | Higher | +4 | Higher | 0 | Same |
| Comments (CLOC) | 4 | **7 ✅** | 3 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 10 | 8 | **7 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03434

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.2254 ✅** | 76.8158 | 54.7573 | +1.4096 | Higher | +23.4681 | Higher | +22.0585 | Higher |
| MI (SEI) | **63.3430 ✅** | 62.9203 | 18.9684 | +0.4227 | Higher | +44.3746 | Higher | +43.9519 | Higher |
| Cyclomatic (sum) | 5 | 5 | 13 | 0 | Same | -8 | Lower | -8 | Lower |
| Cognitive (sum) | 5 | 5 | 13 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | 834.7617 | 834.7617 | 2803.9905 | 0 | Same | -1969.2289 | Lower | -1969.2289 | Lower |
| Halstead Effort | 17164.7869 | 17164.7869 | 176951.8306 | 0 | Same | -159787.0438 | Lower | -159787.0438 | Lower |
| SLOC | **33 ✅** | 36 | 85 | -3 | Lower | -52 | Lower | -49 | Lower |
| PLOC | 24 | 24 | 68 | 0 | Same | -44 | Lower | -44 | Lower |
| LLOC | 19 | 19 | 59 | 0 | Same | -40 | Lower | -40 | Lower |
| Comments (CLOC) | 4 | **5 ✅** | 3 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | **5 ✅** | 7 | 14 | -2 | Lower | -9 | Lower | -7 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.0764 ✅** | 72.2245 | 67.6547 | +2.8519 | Higher | +7.4217 | Higher | +4.5698 | Higher |
| MI (SEI) | **58.2154 ✅** | 53.1439 | 41.8265 | +5.0715 | Higher | +16.3889 | Higher | +11.3174 | Higher |
| Cyclomatic (sum) | **9 ✅** | 11 | 12 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **10 ✅** | 14 | 21 | -4 | Lower | -11 | Lower | -7 | Lower |
| Halstead Volume | **977.2383 ✅** | 1114.8462 | 1364.8579 | -137.6079 | Lower | -387.6196 | Lower | -250.0117 | Lower |
| Halstead Effort | **34691.9582 ✅** | 39019.6162 | 60281.2241 | -4327.6580 | Lower | -25589.2659 | Lower | -21261.6079 | Lower |
| SLOC | **36 ✅** | 40 | 49 | -4 | Lower | -13 | Lower | -9 | Lower |
| PLOC | **26 ✅** | 30 | 39 | -4 | Lower | -13 | Lower | -9 | Lower |
| LLOC | **20 ✅** | 22 | 29 | -2 | Lower | -9 | Lower | -7 | Lower |
| Comments (CLOC) | 4 | 4 | 3 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p03448

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.9331 ✅** | 78.7699 | 77.8892 | +2.1632 | Higher | +3.0439 | Higher | +0.8807 | Higher |
| MI (SEI) | 61.7874 | **69.6324 ✅** | 62.9598 | -7.8451 | Lower | -1.1724 | Lower | +6.6726 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 791.6176 | 791.6176 | 851.9862 | 0 | Same | -60.3686 | Lower | -60.3686 | Lower |
| Halstead Effort | 13235.8460 | 13235.8460 | **12321.0305 ✅** | 0 | Same | +914.8155 | Higher | +914.8155 | Higher |
| SLOC | **28 ✅** | 32 | 33 | -4 | Lower | -5 | Lower | -1 | Lower |
| PLOC | 21 | 21 | 24 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 15 | 15 | 16 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 2 | **6 ✅** | 4 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03463

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.9563 ✅** | 87.0198 | 82.7225 | +0.9365 | Higher | +5.2339 | Higher | +4.2974 | Higher |
| MI (SEI) | 74.5030 | **77.7802 ✅** | 74.3921 | -3.2771 | Lower | +0.1109 | Higher | +3.3880 | Higher |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **573.8617 ✅** | 687.1023 | 640.7628 | -113.2406 | Lower | -66.9011 | Lower | +46.3395 | Higher |
| Halstead Effort | **6886.3401 ✅** | 8842.7077 | 7863.9066 | -1956.3676 | Lower | -977.5665 | Lower | +978.8011 | Higher |
| SLOC | 21 | 21 | 28 | 0 | Same | -7 | Lower | -7 | Lower |
| PLOC | 15 | **14 ✅** | 18 | +1 | Higher | -3 | Lower | -4 | Lower |
| LLOC | **10 ✅** | 11 | 12 | -1 | Lower | -2 | Lower | -1 | Lower |
| Comments (CLOC) | 2 | 3 | **5 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03473

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 100.1019 | **100.8003 ✅** | 98.3629 | -0.6984 | Lower | +1.7390 | Higher | +2.4374 | Higher |
| MI (SEI) | **102.7888 ✅** | 91.5496 | 99.2720 | +11.2392 | Higher | +3.5168 | Higher | -7.7224 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **258.5242 ✅** | 290.0481 | 286.7268 | -31.5240 | Lower | -28.2026 | Lower | +3.3213 | Higher |
| Halstead Effort | **2197.4556 ✅** | 2558.6389 | 2334.7756 | -361.1834 | Lower | -137.3200 | Lower | +223.8634 | Higher |
| SLOC | 13 | **12 ✅** | 14 | +1 | Higher | -1 | Lower | -2 | Lower |
| PLOC | **7 ✅** | 8 | 8 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 3 | 1 | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03478

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.2862 ✅** | 77.2227 | 74.0728 | +0.0635 | Higher | +3.2134 | Higher | +3.1499 | Higher |
| MI (SEI) | **61.8480 ✅** | 58.6262 | 61.4327 | +3.2218 | Higher | +0.4153 | Higher | -2.8065 | Lower |
| Cyclomatic (sum) | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **834.0579 ✅** | 844.2975 | 1294.9016 | -10.2397 | Lower | -460.8437 | Lower | -450.6041 | Lower |
| Halstead Effort | 20017.3885 | **19418.8426 ✅** | 39959.8535 | +598.5459 | Higher | -19942.4650 | Lower | -20541.0109 | Lower |
| SLOC | 34 | 34 | 36 | 0 | Same | -2 | Lower | -2 | Lower |
| PLOC | 25 | 25 | 25 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 19 | 19 | **18 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 4 | 3 | **6 ✅** | +1 | Higher | -2 | Lower | -3 | Lower |
| Blank Lines | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.5858 ✅** | 71.8577 | 55.5891 | +2.7282 | Higher | +18.9968 | Higher | +16.2686 | Higher |
| MI (SEI) | **64.8903 ✅** | 59.3593 | 33.1673 | +5.5310 | Higher | +31.7230 | Higher | +26.1919 | Higher |
| Cyclomatic (sum) | 7 | 7 | 13 | 0 | Same | -6 | Lower | -6 | Lower |
| Cognitive (sum) | **5 ✅** | 7 | 14 | -2 | Lower | -9 | Lower | -7 | Lower |
| Halstead Volume | **914.3047 ✅** | 1061.0450 | 2304 | -146.7403 | Lower | -1389.6953 | Lower | -1242.9550 | Lower |
| Halstead Effort | **20639.5821 ✅** | 22509.3110 | 108553.8462 | -1869.7289 | Lower | -87914.2641 | Lower | -86044.5351 | Lower |
| SLOC | **39 ✅** | 44 | 86 | -5 | Lower | -47 | Lower | -42 | Lower |
| PLOC | **25 ✅** | 30 | 59 | -5 | Lower | -34 | Lower | -29 | Lower |
| LLOC | 18 | **17 ✅** | 52 | +1 | Higher | -34 | Lower | -35 | Lower |
| Comments (CLOC) | 8 | 8 | **12 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | **6 ✅** | 7 | 16 | -1 | Lower | -10 | Lower | -9 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03544

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.3645 ✅** | 82.5167 | 31.5194 | +4.8478 | Higher | +55.8451 | Higher | +50.9974 | Higher |
| MI (SEI) | **81.3236 ✅** | 78.3415 | -0.7175 | +2.9821 | Higher | +82.0412 | Higher | +79.0591 | Higher |
| Cyclomatic (sum) | **3 ✅** | 5 | 45 | -2 | Lower | -42 | Lower | -40 | Lower |
| Cognitive (sum) | **1 ✅** | 5 | 40 | -4 | Lower | -39 | Lower | -35 | Lower |
| Halstead Volume | 556.2803 | **546.9926 ✅** | 5267.9700 | +9.2877 | Higher | -4711.6897 | Lower | -4720.9774 | Lower |
| Halstead Effort | 9006.4432 | **8601.4587 ✅** | 560662.5215 | +404.9845 | Higher | -551656.0783 | Lower | -552061.0628 | Lower |
| SLOC | **22 ✅** | 29 | 185 | -7 | Lower | -163 | Lower | -156 | Lower |
| PLOC | **14 ✅** | 21 | 152 | -7 | Lower | -138 | Lower | -131 | Lower |
| LLOC | 13 | 13 | 112 | 0 | Same | -99 | Lower | -99 | Lower |
| Comments (CLOC) | 4 | 7 | **21 ✅** | -3 | Lower | -17 | Lower | -14 | Lower |
| Blank Lines | 4 | **3 ✅** | 16 | +1 | Higher | -12 | Lower | -13 | Lower |
| Functions | 1 | 1 | 11 | 0 | Same | -10 | Lower | -10 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 11 | 0 | Same | -11 | Lower | -11 | Lower |

## p03568

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.4115 ✅** | 78.4415 | 78.7364 | +2.9700 | Higher | +2.6751 | Higher | -0.2949 | Lower |
| MI (SEI) | **69.4669 ✅** | 60.5922 | 61.4422 | +8.8747 | Higher | +8.0247 | Higher | -0.8500 | Lower |
| Cyclomatic (sum) | **5 ✅** | 6 | 7 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **676.5315 ✅** | 766.1299 | 762.2515 | -89.5985 | Lower | -85.7201 | Lower | +3.8784 | Higher |
| Halstead Effort | **10978.2609 ✅** | 14389.9188 | 14726.6999 | -3411.6579 | Lower | -3748.4390 | Lower | -336.7811 | Lower |
| SLOC | **29 ✅** | 33 | 32 | -4 | Lower | -3 | Lower | +1 | Higher |
| PLOC | **20 ✅** | 24 | 23 | -4 | Lower | -3 | Lower | +1 | Higher |
| LLOC | 16 | 20 | 16 | -4 | Lower | 0 | Same | +4 | Higher |
| Comments (CLOC) | **4 ✅** | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03592

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **83.7454 ✅** | 78.3139 | 77.6982 | +5.4316 | Higher | +6.0473 | Higher | +0.6157 | Higher |
| MI (SEI) | 66.8419 | **70.7193 ✅** | 62.4423 | -3.8775 | Lower | +4.3995 | Higher | +8.2770 | Higher |
| Cyclomatic (sum) | **5 ✅** | 7 | 7 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **685.7667 ✅** | 751.2044 | 770.5296 | -65.4378 | Lower | -84.7629 | Lower | -19.3252 | Lower |
| Halstead Effort | **12735.6665 ✅** | 14804.9872 | 13115.0558 | -2069.3208 | Lower | -379.3893 | Lower | +1689.9314 | Higher |
| SLOC | **25 ✅** | 33 | 34 | -8 | Lower | -9 | Lower | -1 | Lower |
| PLOC | **19 ✅** | 22 | 25 | -3 | Lower | -6 | Lower | -3 | Lower |
| LLOC | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 2 | **7 ✅** | 4 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p03608

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7558 | 40.1706 | **53.8019 ✅** | +11.5851 | Higher | -2.0462 | Lower | -13.6313 | Lower |
| MI (SEI) | **22.9569 ✅** | 11.5001 | 20.5278 | +11.4568 | Higher | +2.4291 | Higher | -9.0277 | Lower |
| Cyclomatic (sum) | 19 | 30 | **18 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| Cognitive (sum) | **29 ✅** | 58 | 30 | -29 | Lower | -1 | Lower | +28 | Higher |
| Halstead Volume | **2993.1770 ✅** | 3723.8561 | 3020.9124 | -730.6792 | Lower | -27.7354 | Lower | +702.9437 | Higher |
| Halstead Effort | 158359.9451 | 208197.4111 | **157339.1884 ✅** | -49837.4660 | Lower | +1020.7567 | Higher | +50858.2227 | Higher |
| SLOC | 92 | 150 | **82 ✅** | -58 | Lower | +10 | Higher | +68 | Higher |
| PLOC | 75 | 116 | **67 ✅** | -41 | Lower | +8 | Higher | +49 | Higher |
| LLOC | **49 ✅** | 63 | 53 | -14 | Lower | -4 | Lower | +10 | Higher |
| Comments (CLOC) | 8 | **19 ✅** | 4 | -11 | Lower | +4 | Higher | +15 | Higher |
| Blank Lines | 12 | 16 | **11 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p03632

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.2950 | **73.0524 ✅** | 66.5999 | -0.7573 | Lower | +5.6951 | Higher | +6.4524 | Higher |
| MI (SEI) | 54.7831 | **58.4336 ✅** | 46.1970 | -3.6505 | Lower | +8.5861 | Higher | +12.2366 | Higher |
| Cyclomatic (sum) | **7 ✅** | 8 | 9 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 7 | 7 | 11 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | 1127.5815 | **1005.2883 ✅** | 1494.9872 | +122.2933 | Higher | -367.4057 | Lower | -489.6990 | Lower |
| Halstead Effort | 27980.7270 | **24911.0435 ✅** | 42677.2138 | +3069.6835 | Higher | -14696.4868 | Lower | -17766.1703 | Lower |
| SLOC | 42 | **41 ✅** | 53 | +1 | Higher | -11 | Lower | -12 | Lower |
| PLOC | 31 | **28 ✅** | 39 | +3 | Higher | -8 | Lower | -11 | Lower |
| LLOC | 26 | **24 ✅** | 32 | +2 | Higher | -6 | Lower | -8 | Lower |
| Comments (CLOC) | 5 | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | **6 ✅** | 7 | 8 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03646

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.6818 | **75.5265 ✅** | 70.6473 | -8.8447 | Lower | -3.9655 | Lower | +4.8792 | Higher |
| MI (SEI) | 46.6206 | **71.3872 ✅** | 51.9274 | -24.7666 | Lower | -5.3068 | Lower | +19.4599 | Higher |
| Cyclomatic (sum) | 12 | **6 ✅** | 10 | +6 | Higher | +2 | Higher | -4 | Lower |
| Cognitive (sum) | 23 | **7 ✅** | 17 | +16 | Higher | +6 | Higher | -10 | Lower |
| Halstead Volume | 1288.7558 | **737.0233 ✅** | 1093.4286 | +551.7326 | Higher | +195.3272 | Higher | -356.4054 | Lower |
| Halstead Effort | 40517.7019 | **16486.8901 ✅** | 31490.7444 | +24030.8118 | Higher | +9026.9575 | Higher | -15003.8543 | Lower |
| SLOC | 53 | **40 ✅** | 45 | +13 | Higher | +8 | Higher | -5 | Lower |
| PLOC | 43 | **24 ✅** | 35 | +19 | Higher | +8 | Higher | -11 | Lower |
| LLOC | 31 | **18 ✅** | 25 | +13 | Higher | +6 | Higher | -7 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 5 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03701

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.4330 ✅** | 72.8717 | 69.9966 | +2.5614 | Higher | +5.4365 | Higher | +2.8751 | Higher |
| MI (SEI) | 60.1732 | **62.4810 ✅** | 48.3564 | -2.3078 | Lower | +11.8168 | Higher | +14.1246 | Higher |
| Cyclomatic (sum) | **7 ✅** | 8 | 10 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 9 | 9 | 12 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | **776.8450 ✅** | 835.2930 | 1157.1748 | -58.4480 | Lower | -380.3297 | Lower | -321.8817 | Lower |
| Halstead Effort | **18126.3843 ✅** | 20973.1187 | 34219.3110 | -2846.7344 | Lower | -16092.9268 | Lower | -13246.1924 | Lower |
| SLOC | **39 ✅** | 44 | 46 | -5 | Lower | -7 | Lower | -2 | Lower |
| PLOC | 28 | 28 | 34 | 0 | Same | -6 | Lower | -6 | Lower |
| LLOC | 24 | **19 ✅** | 28 | +5 | Higher | -4 | Lower | -9 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 4 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 7 | 7 | 9 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03719

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 90.9879 | 90.9879 | 90.1120 | 0 | Same | +0.8759 | Higher | +0.8759 | Higher |
| MI (SEI) | 73.8286 | 73.8286 | **78.7969 ✅** | 0 | Same | -4.9683 | Lower | -4.9683 | Lower |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 495.4196 | 495.4196 | 495.4196 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Effort | 6270.1547 | 6270.1547 | 6270.1547 | 0 | Same | 0 | Same | 0 | Same |
| SLOC | 18 | 18 | 19 | 0 | Same | -1 | Lower | -1 | Lower |
| PLOC | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 1 | 1 | **2 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03722

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.9633 ✅** | 48.5901 | 56.5536 | +12.3732 | Higher | +4.4097 | Higher | -7.9635 | Lower |
| MI (SEI) | **37.4764 ✅** | 25.9831 | 31.9661 | +11.4932 | Higher | +5.5103 | Higher | -5.9829 | Lower |
| Cyclomatic (sum) | **12 ✅** | 21 | 15 | -9 | Lower | -3 | Lower | +6 | Higher |
| Cognitive (sum) | **17 ✅** | 32 | 25 | -15 | Lower | -8 | Lower | +7 | Higher |
| Halstead Volume | **1864.7978 ✅** | 2513.0060 | 2032.1399 | -648.2082 | Lower | -167.3420 | Lower | +480.8662 | Higher |
| Halstead Effort | **68411.4410 ✅** | 89812.6505 | 77500.7346 | -21401.2095 | Lower | -9089.2936 | Lower | +12311.9159 | Higher |
| SLOC | **67 ✅** | 115 | 82 | -48 | Lower | -15 | Lower | +33 | Higher |
| PLOC | **50 ✅** | 85 | 65 | -35 | Lower | -15 | Lower | +20 | Higher |
| LLOC | **34 ✅** | 57 | 39 | -23 | Lower | -5 | Lower | +18 | Higher |
| Comments (CLOC) | 7 | **19 ✅** | 9 | -12 | Lower | -2 | Lower | +10 | Higher |
| Blank Lines | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 4 | 1 | -4 | Lower | -1 | Lower | +3 | Higher |

## p03762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.8566 | 52.9547 | **68.9838 ✅** | +13.9020 | Higher | -2.1271 | Lower | -16.0291 | Lower |
| MI (SEI) | **46.5797 ✅** | 46.2026 | 46.2685 | +0.3771 | Higher | +0.3112 | Higher | -0.0658 | Lower |
| Cyclomatic (sum) | 7 | 15 | **6 ✅** | -8 | Lower | +1 | Higher | +9 | Higher |
| Cognitive (sum) | 5 | 17 | **4 ✅** | -12 | Lower | +1 | Higher | +13 | Higher |
| Halstead Volume | 1649.6411 | 1721.4566 | **1569.3745 ✅** | -71.8155 | Lower | +80.2666 | Higher | +152.0822 | Higher |
| Halstead Effort | 60706.7921 | 53795.5193 | **40019.0488 ✅** | +6911.2728 | Higher | +20687.7433 | Higher | +13776.4705 | Higher |
| SLOC | 52 | 108 | **47 ✅** | -56 | Lower | +5 | Higher | +61 | Higher |
| PLOC | 36 | 47 | **35 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| LLOC | 32 | **29 ✅** | 31 | +3 | Higher | +1 | Higher | -2 | Lower |
| Comments (CLOC) | 6 | **52 ✅** | 4 | -46 | Lower | +2 | Higher | +48 | Higher |
| Blank Lines | 10 | 9 | **8 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p03797

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.9812 | 81.1570 | **82.0148 ✅** | -0.1758 | Lower | -1.0336 | Lower | -0.8578 | Lower |
| MI (SEI) | 68.4392 | **81.9623 ✅** | 67.4125 | -13.5231 | Lower | +1.0267 | Higher | +14.5498 | Higher |
| Cyclomatic (sum) | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 6 | 4 | 4 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 661.2389 | **528.3162 ✅** | 672.0195 | +132.9227 | Higher | -10.7806 | Lower | -143.7033 | Lower |
| Halstead Effort | 12497.4146 | **7983.4441 ✅** | 13687.9758 | +4513.9705 | Higher | -1190.5612 | Lower | -5704.5318 | Lower |
| SLOC | 30 | 31 | **28 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| PLOC | 23 | **16 ✅** | 20 | +7 | Higher | +3 | Higher | -4 | Lower |
| LLOC | 19 | **7 ✅** | 17 | +12 | Higher | +2 | Higher | -10 | Lower |
| Comments (CLOC) | 4 | **11 ✅** | 3 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03808

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8530 | **76.4136 ✅** | 55.0686 | -16.5606 | Lower | +4.7844 | Higher | +21.3451 | Higher |
| MI (SEI) | 37.2432 | **75.8859 ✅** | 31.4447 | -38.6427 | Lower | +5.7985 | Higher | +44.4413 | Higher |
| Cyclomatic (sum) | 14 | **7 ✅** | 15 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 14 | **4 ✅** | 15 | +10 | Higher | -1 | Lower | -11 | Lower |
| Halstead Volume | 1928.1617 | **594.5414 ✅** | 2248.5146 | +1333.6203 | Higher | -320.3529 | Lower | -1653.9732 | Lower |
| Halstead Effort | 119847.3022 | **10631.7987 ✅** | 127415.8280 | +109215.5035 | Higher | -7568.5259 | Lower | -116784.0293 | Lower |
| SLOC | 69 | **40 ✅** | 87 | +29 | Higher | -18 | Lower | -47 | Lower |
| PLOC | 52 | **19 ✅** | 61 | +33 | Higher | -9 | Lower | -42 | Lower |
| LLOC | 44 | **13 ✅** | 52 | +31 | Higher | -8 | Lower | -39 | Lower |
| Comments (CLOC) | 8 | **15 ✅** | 11 | -7 | Lower | -3 | Lower | +4 | Higher |
| Blank Lines | 11 | **6 ✅** | 15 | +5 | Higher | -4 | Lower | -9 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p03815

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 86.3683 | **90.6184 ✅** | 83.5437 | -4.2501 | Lower | +2.8246 | Higher | +7.0747 | Higher |
| MI (SEI) | 78.8659 | **94.2300 ✅** | 73.8830 | -15.3641 | Lower | +4.9829 | Higher | +20.3470 | Higher |
| Cyclomatic (sum) | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 491.5424 | **359.4906 ✅** | 630.9011 | +132.0518 | Higher | -139.3587 | Lower | -271.4105 | Lower |
| Halstead Effort | 7938.4099 | **3505.0333 ✅** | 10162.9944 | +4433.3766 | Higher | -2224.5846 | Lower | -6657.9611 | Lower |
| SLOC | 24 | **21 ✅** | 26 | +3 | Higher | -2 | Lower | -5 | Lower |
| PLOC | 16 | **10 ✅** | 17 | +6 | Higher | -1 | Lower | -7 | Lower |
| LLOC | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 4 | **7 ✅** | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03817

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.0021 | **87.1186 ✅** | 85.5367 | -0.1165 | Lower | +1.4654 | Higher | +1.5819 | Higher |
| MI (SEI) | 76.7622 | **81.1725 ✅** | 74.2488 | -4.4103 | Lower | +2.5134 | Higher | +6.9237 | Higher |
| Cyclomatic (sum) | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **496.8278 ✅** | 533.8394 | 551.8279 | -37.0116 | Lower | -55.0000 | Lower | -17.9884 | Lower |
| Halstead Effort | 7551.7827 | **7496.9624 ✅** | 10719.2561 | +54.8203 | Higher | -3167.4734 | Lower | -3222.2938 | Lower |
| SLOC | 23 | **22 ✅** | 24 | +1 | Higher | -1 | Lower | -2 | Lower |
| PLOC | 16 | **14 ✅** | 17 | +2 | Higher | -1 | Lower | -3 | Lower |
| LLOC | **7 ✅** | 8 | 14 | -1 | Lower | -7 | Lower | -6 | Lower |
| Comments (CLOC) | 3 | **4 ✅** | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.9821 ✅** | 53.0562 | 56.3737 | +3.9259 | Higher | +0.6084 | Higher | -3.3174 | Lower |
| MI (SEI) | **33.2954 ✅** | 25.9856 | 29.9098 | +7.3098 | Higher | +3.3856 | Higher | -3.9242 | Lower |
| Cyclomatic (sum) | **15 ✅** | 21 | 16 | -6 | Lower | -1 | Lower | +5 | Higher |
| Cognitive (sum) | 25 | 42 | **20 ✅** | -17 | Lower | +5 | Higher | +22 | Higher |
| Halstead Volume | **2276.5829 ✅** | 2832.5875 | 2550.2171 | -556.0047 | Lower | -273.6342 | Lower | +282.3705 | Higher |
| Halstead Effort | **94755.0704 ✅** | 124662.1766 | 138136.7572 | -29907.1062 | Lower | -43381.6868 | Lower | -13474.5806 | Lower |
| SLOC | 77 | 84 | **76 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| PLOC | 59 | 69 | **58 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| LLOC | 49 | 58 | 49 | -9 | Lower | 0 | Same | +9 | Higher |
| Comments (CLOC) | **9 ✅** | 8 | 7 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 9 | **8 ✅** | 11 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p03838

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.4855 | **83.2972 ✅** | 72.3410 | -10.8117 | Lower | +0.1445 | Higher | +10.9562 | Higher |
| MI (SEI) | 52.7942 | **89.5260 ✅** | 57.5091 | -36.7318 | Lower | -4.7148 | Lower | +32.0170 | Higher |
| Cyclomatic (sum) | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 12 | **0 ✅** | 12 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 995.0056 | **436.7133 ✅** | 1102.7958 | +558.2923 | Higher | -107.7903 | Lower | -666.0826 | Lower |
| Halstead Effort | 28755.6616 | **4174.4650 ✅** | 33849.7059 | +24581.1966 | Higher | -5094.0443 | Lower | -29675.2409 | Lower |
| SLOC | 42 | **31 ✅** | 41 | +11 | Higher | +1 | Higher | -10 | Lower |
| PLOC | 33 | **11 ✅** | 31 | +22 | Higher | +2 | Higher | -20 | Lower |
| LLOC | 28 | **8 ✅** | 26 | +20 | Higher | +2 | Higher | -18 | Lower |
| Comments (CLOC) | 4 | **16 ✅** | 6 | -12 | Lower | -2 | Lower | +10 | Higher |
| Blank Lines | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03845

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.3816 ✅** | 75.0697 | 69.9454 | +0.3119 | Higher | +5.4363 | Higher | +5.1244 | Higher |
| MI (SEI) | **58.5663 ✅** | 54.4590 | 45.7024 | +4.1073 | Higher | +12.8639 | Higher | +8.7566 | Higher |
| Cyclomatic (sum) | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |
| Cognitive (sum) | **3 ✅** | 4 | 5 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 1200.7669 | **1072.3128 ✅** | 1466.3476 | +128.4541 | Higher | -265.5807 | Lower | -394.0348 | Lower |
| Halstead Effort | 25174.6999 | **20680.3183 ✅** | 37529.3348 | +4494.3816 | Higher | -12354.6349 | Lower | -16849.0165 | Lower |
| SLOC | **35 ✅** | 37 | 44 | -2 | Lower | -9 | Lower | -7 | Lower |
| PLOC | 26 | **25 ✅** | 32 | +1 | Higher | -6 | Lower | -7 | Lower |
| LLOC | 20 | **19 ✅** | 26 | +1 | Higher | -6 | Lower | -7 | Lower |
| Comments (CLOC) | **4 ✅** | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | **6 ✅** | 9 | 9 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03901

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.7982 | **88.8751 ✅** | 49.8761 | -18.0769 | Lower | +20.9221 | Higher | +38.9990 | Higher |
| MI (SEI) | 63.3264 | **89.5541 ✅** | 27.1397 | -26.2277 | Lower | +36.1867 | Higher | +62.4144 | Higher |
| Cyclomatic (sum) | 5 | **2 ✅** | 19 | +3 | Higher | -14 | Lower | -17 | Lower |
| Cognitive (sum) | 3 | **0 ✅** | 6 | +3 | Higher | -3 | Lower | -6 | Lower |
| Halstead Volume | 1157.1748 | **502.6697 ✅** | 2683.8916 | +654.5050 | Higher | -1526.7168 | Lower | -2181.2219 | Lower |
| Halstead Effort | 40056.0497 | **6646.4109 ✅** | 109381.2426 | +33409.6388 | Higher | -69325.1929 | Lower | -102734.8316 | Lower |
| SLOC | 47 | **21 ✅** | 107 | +26 | Higher | -60 | Lower | -86 | Lower |
| PLOC | 27 | **11 ✅** | 76 | +16 | Higher | -49 | Lower | -65 | Lower |
| LLOC | 19 | **8 ✅** | 40 | +11 | Higher | -21 | Lower | -32 | Lower |
| Comments (CLOC) | 13 | 6 | **17 ✅** | +7 | Higher | -4 | Lower | -11 | Lower |
| Blank Lines | 7 | **4 ✅** | 14 | +3 | Higher | -7 | Lower | -10 | Lower |
| Functions | 2 | **1 ✅** | 7 | +1 | Higher | -5 | Lower | -6 | Lower |
| Closures | 0 | 0 | 6 | 0 | Same | -6 | Lower | -6 | Lower |
| Exits | 2 | **0 ✅** | 7 | +2 | Higher | -5 | Lower | -7 | Lower |

## p03914

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.2001 | **75.0773 ✅** | 63.5756 | -0.8772 | Lower | +10.6245 | Higher | +11.5017 | Higher |
| MI (SEI) | 62.7058 | **76.3939 ✅** | 54.1006 | -13.6881 | Lower | +8.6051 | Higher | +22.2933 | Higher |
| Cyclomatic (sum) | **5 ✅** | 6 | 7 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 7 | +4 | Higher | -1 | Lower | -5 | Lower |
| Halstead Volume | 1166.4637 | **690.2166 ✅** | 1623.6095 | +476.2470 | Higher | -457.1458 | Lower | -933.3929 | Lower |
| Halstead Effort | 31537.7211 | **11129.7432 ✅** | 56779.9430 | +20407.9780 | Higher | -25242.2219 | Lower | -45650.1999 | Lower |
| SLOC | **38 ✅** | 42 | 64 | -4 | Lower | -26 | Lower | -22 | Lower |
| PLOC | 24 | **18 ✅** | 37 | +6 | Higher | -13 | Lower | -19 | Lower |
| LLOC | 22 | **8 ✅** | 33 | +14 | Higher | -11 | Lower | -25 | Lower |
| Comments (CLOC) | 7 | 19 | 19 | -12 | Lower | -12 | Lower | 0 | Same |
| Blank Lines | 7 | **5 ✅** | 9 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p03951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.8011 ✅** | 59.9651 | 60.6164 | +16.8361 | Higher | +16.1847 | Higher | -0.6513 | Lower |
| MI (SEI) | **73.7419 ✅** | 38.2508 | 43.2044 | +35.4911 | Higher | +30.5376 | Higher | -4.9535 | Lower |
| Cyclomatic (sum) | **4 ✅** | 19 | 12 | -15 | Lower | -8 | Lower | +7 | Higher |
| Cognitive (sum) | **3 ✅** | 20 | 27 | -17 | Lower | -24 | Lower | -7 | Lower |
| Halstead Volume | **739.3422 ✅** | 1657.8086 | 1462.6840 | -918.4664 | Lower | -723.3418 | Lower | +195.1246 | Higher |
| Halstead Effort | **15138.9112 ✅** | 74360.7383 | 53631.7469 | -59221.8271 | Lower | -38492.8357 | Lower | +20728.9914 | Higher |
| SLOC | **38 ✅** | 67 | 74 | -29 | Lower | -36 | Lower | -7 | Lower |
| PLOC | **19 ✅** | 52 | 52 | -33 | Lower | -33 | Lower | 0 | Same |
| LLOC | **16 ✅** | 37 | 40 | -21 | Lower | -24 | Lower | -3 | Lower |
| Comments (CLOC) | 12 | 8 | **13 ✅** | +4 | Higher | -1 | Lower | -5 | Lower |
| Blank Lines | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 5 | 1 | -5 | Lower | -1 | Lower | +4 | Higher |

## p03996

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.8525 | **63.8579 ✅** | 49.8024 | -11.0054 | Lower | +3.0500 | Higher | +14.0555 | Higher |
| MI (SEI) | 28.6904 | **38.6481 ✅** | 23.8987 | -9.9577 | Lower | +4.7917 | Higher | +14.7494 | Higher |
| Cyclomatic (sum) | **17 ✅** | 19 | 19 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 21 | **15 ✅** | 24 | +6 | Higher | -3 | Lower | -9 | Lower |
| Halstead Volume | 2560.5189 | **1450.2317 ✅** | 2802.9684 | +1110.2872 | Higher | -242.4495 | Lower | -1352.7368 | Lower |
| Halstead Effort | 116503.6088 | **46407.4128 ✅** | 137832.9249 | +70096.1960 | Higher | -21329.3161 | Lower | -91425.5121 | Lower |
| SLOC | 93 | **55 ✅** | 106 | +38 | Higher | -13 | Lower | -51 | Lower |
| PLOC | 67 | **43 ✅** | 79 | +24 | Higher | -12 | Lower | -36 | Lower |
| LLOC | 57 | **25 ✅** | 60 | +32 | Higher | -3 | Lower | -35 | Lower |
| Comments (CLOC) | 12 | 4 | **13 ✅** | +8 | Higher | -1 | Lower | -9 | Lower |
| Blank Lines | 15 | **8 ✅** | 14 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | **2 ✅** | 6 | 3 | -4 | Lower | -1 | Lower | +3 | Higher |

## p04005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.7348 ✅** | 76.1786 | 66.1709 | +1.5562 | Higher | +11.5639 | Higher | +10.0077 | Higher |
| MI (SEI) | **66.0018 ✅** | 56.8227 | 56.5669 | +9.1791 | Higher | +9.4349 | Higher | +0.2558 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 11 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 9 | **5 ✅** | 18 | +4 | Higher | -9 | Lower | -13 | Lower |
| Halstead Volume | **884.1883 ✅** | 942.9292 | 1184.7038 | -58.7408 | Lower | -300.5154 | Lower | -241.7746 | Lower |
| Halstead Effort | 26218.1067 | **19035.3823 ✅** | 36873.9051 | +7182.7244 | Higher | -10655.7984 | Lower | -17838.5228 | Lower |
| SLOC | **32 ✅** | 35 | 57 | -3 | Lower | -25 | Lower | -22 | Lower |
| PLOC | **24 ✅** | 26 | 35 | -2 | Lower | -11 | Lower | -9 | Lower |
| LLOC | **20 ✅** | 22 | 24 | -2 | Lower | -4 | Lower | -2 | Lower |
| Comments (CLOC) | 5 | 3 | **15 ✅** | +2 | Higher | -10 | Lower | -12 | Lower |
| Blank Lines | **4 ✅** | 6 | 7 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p04013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.4750 ✅** | 60.6688 | 50.1139 | +3.8062 | Higher | +14.3611 | Higher | +10.5549 | Higher |
| MI (SEI) | 42.0520 | **47.8282 ✅** | 26.7449 | -5.7762 | Lower | +15.3071 | Higher | +21.0833 | Higher |
| Cyclomatic (sum) | 16 | **12 ✅** | 26 | +4 | Higher | -10 | Lower | -14 | Lower |
| Cognitive (sum) | 18 | **14 ✅** | 40 | +4 | Higher | -22 | Lower | -26 | Lower |
| Halstead Volume | 1751.5363 | **1510.7185 ✅** | 3011.5849 | +240.8178 | Higher | -1260.0486 | Lower | -1500.8664 | Lower |
| Halstead Effort | 85387.3955 | **51949.2237 ✅** | 191988.5362 | +33438.1718 | Higher | -106601.1408 | Lower | -140039.3125 | Lower |
| SLOC | **52 ✅** | 73 | 92 | -21 | Lower | -40 | Lower | -19 | Lower |
| PLOC | **40 ✅** | 47 | 67 | -7 | Lower | -27 | Lower | -20 | Lower |
| LLOC | **27 ✅** | 38 | 41 | -11 | Lower | -14 | Lower | -3 | Lower |
| Comments (CLOC) | 5 | **18 ✅** | 13 | -13 | Lower | -8 | Lower | +5 | Higher |
| Blank Lines | **7 ✅** | 9 | 13 | -2 | Lower | -6 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p04022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 23.3438 | **36.8335 ✅** | 22.9738 | -13.4897 | Lower | +0.3700 | Higher | +13.8597 | Higher |
| MI (SEI) | -22.7430 | **14.6313 ✅** | -23.3562 | -37.3743 | Lower | +0.6131 | Higher | +37.9875 | Higher |
| Cyclomatic (sum) | 55 | **35 ✅** | 55 | +20 | Higher | 0 | Same | -20 | Lower |
| Cognitive (sum) | 131 | **79 ✅** | 130 | +52 | Higher | +1 | Higher | -51 | Lower |
| Halstead Volume | 6382.1157 | **3702.4724 ✅** | 6602.8392 | +2679.6433 | Higher | -220.7235 | Lower | -2900.3668 | Lower |
| Halstead Effort | 604477.5316 | **257090.4274 ✅** | 621682.7056 | +347387.1042 | Higher | -17205.1740 | Lower | -364592.2782 | Lower |
| SLOC | 250 | **172 ✅** | 253 | +78 | Higher | -3 | Lower | -81 | Lower |
| PLOC | 215 | **119 ✅** | 213 | +96 | Higher | +2 | Higher | -94 | Lower |
| LLOC | 160 | **91 ✅** | 160 | +69 | Higher | 0 | Same | -69 | Lower |
| Comments (CLOC) | 8 | **39 ✅** | 8 | -31 | Lower | 0 | Same | +31 | Higher |
| Blank Lines | 27 | **15 ✅** | 33 | +12 | Higher | -6 | Lower | -18 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |

## p04030

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.3439 ✅** | 69.3464 | 68.9665 | +5.9975 | Higher | +6.3774 | Higher | +0.3799 | Higher |
| MI (SEI) | 63.7910 | **64.2538 ✅** | 57.5609 | -0.4628 | Lower | +6.2301 | Higher | +6.6929 | Higher |
| Cyclomatic (sum) | **6 ✅** | 9 | 8 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | 12 | **11 ✅** | 14 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | **763.3676 ✅** | 993.8036 | 1117.4835 | -230.4361 | Lower | -354.1159 | Lower | -123.6799 | Lower |
| Halstead Effort | **15267.3516 ✅** | 23478.6111 | 29131.6395 | -8211.2594 | Lower | -13864.2879 | Lower | -5653.0284 | Lower |
| SLOC | **40 ✅** | 51 | 51 | -11 | Lower | -11 | Lower | 0 | Same |
| PLOC | **28 ✅** | 30 | 33 | -2 | Lower | -5 | Lower | -3 | Lower |
| LLOC | **22 ✅** | 27 | 28 | -5 | Lower | -6 | Lower | -1 | Lower |
| Comments (CLOC) | 7 | **17 ✅** | 11 | -10 | Lower | -4 | Lower | +6 | Higher |
| Blank Lines | **5 ✅** | 6 | 7 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.8307 ✅** | 64.7916 | 61.0460 | 0.0391 | Higher | 3.7848 | Higher | 3.7457 | Higher |
| MI (SEI) | 44.2689 | **48.5725 ✅** | 38.8545 | -4.3037 | Lower | 5.4144 | Higher | 9.7181 | Higher |
| Cyclomatic (sum) | **14.1058 ✅** | 15.0423 | 16.1058 | -0.9365 | Lower | -2 | Lower | -1.0635 | Lower |
| Cognitive (sum) | 18.1958 | **17.1852 ✅** | 20.7672 | 1.0106 | Higher | -2.5714 | Lower | -3.5820 | Lower |
| Halstead Volume | 2023.1876 | **1831.3214 ✅** | 2411.8713 | 191.8661 | Higher | -388.6837 | Lower | -580.5499 | Lower |
| Halstead Effort | 114766.3535 | **89030.3952 ✅** | 160816.7065 | 25735.9583 | Higher | -46050.3529 | Lower | -71786.3112 | Lower |
| SLOC | 70.2540 | **69.5026 ✅** | 81.6984 | 0.7513 | Higher | -11.4444 | Lower | -12.1958 | Lower |
| PLOC | 53.5556 | **49.3651 ✅** | 63.1905 | 4.1905 | Higher | -9.6349 | Lower | -13.8254 | Lower |
| LLOC | 38.5926 | **32.9048 ✅** | 46.9048 | 5.6878 | Higher | -8.3122 | Lower | -14 | Lower |
| Comments (CLOC) | 7.3915 | **11.3862 ✅** | 7.9153 | -3.9947 | Lower | -0.5238 | Lower | 3.4709 | Higher |
| Blank Lines | 9.9259 | **9.1852 ✅** | 11.1693 | 0.7407 | Higher | -1.2434 | Lower | -1.9841 | Lower |
| Functions | 1.8201 | **1.6296 ✅** | 2.1746 | 0.1905 | Higher | -0.3545 | Lower | -0.5450 | Lower |
| Closures | **0.4021 ✅** | 0.6984 | 0.4074 | -0.2963 | Lower | -0.0053 | Lower | 0.2910 | Higher |
| Exits | **1.4180 ✅** | 2.0317 | 1.7513 | -0.6138 | Lower | -0.3333 | Lower | 0.2804 | Higher |

