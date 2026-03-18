# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/codenet_statistics copy/r_code/full_translation_loop_all_metrics.json`

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

## p00071

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.7661 ✅** | 55.4142 | 48.5578 | +2.3519 | Higher | +9.2082 | Higher | +6.8564 | Higher |
| MI (SEI) | **30.0662 ✅** | 21.6284 | 22.3745 | +8.4378 | Higher | +7.6917 | Higher | -0.7461 | Lower |
| Cyclomatic (sum) | **20 ✅** | 21 | 23 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 21 | **18 ✅** | 29 | +3 | Higher | -8 | Lower | -11 | Lower |
| Halstead Volume | **2311.7332 ✅** | 2562.0403 | 3691.1681 | -250.3071 | Lower | -1379.4349 | Lower | -1129.1278 | Lower |
| Halstead Effort | **84708.5087 ✅** | 129570.5016 | 163959.3036 | -44861.9929 | Lower | -79250.7950 | Lower | -34388.8021 | Lower |
| SLOC | **68 ✅** | 75 | 99 | -7 | Lower | -31 | Lower | -24 | Lower |
| PLOC | **50 ✅** | 60 | 75 | -10 | Lower | -25 | Lower | -15 | Lower |
| LLOC | **34 ✅** | 40 | 54 | -6 | Lower | -20 | Lower | -14 | Lower |
| Comments (CLOC) | 5 | 3 | **12 ✅** | +2 | Higher | -7 | Lower | -9 | Lower |
| Blank Lines | 13 | **12 ✅** | 13 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | **2 ✅** | 5 | 4 | -3 | Lower | -2 | Lower | +1 | Higher |

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

## p00080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.3077 ✅** | 68.3916 | 71.0672 | +6.9161 | Higher | +4.2405 | Higher | -2.6755 | Lower |
| MI (SEI) | **66.7439 ✅** | 58.0817 | 57.4676 | +8.6622 | Higher | +9.2764 | Higher | +0.6142 | Higher |
| Cyclomatic (sum) | **6 ✅** | 9 | 8 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **5 ✅** | 9 | 7 | -4 | Lower | -2 | Lower | +2 | Higher |
| Halstead Volume | **711.7859 ✅** | 943.8016 | 962.2906 | -232.0157 | Lower | -250.5047 | Lower | -18.4891 | Lower |
| Halstead Effort | **15206.3351 ✅** | 21007.1958 | 22132.6846 | -5800.8607 | Lower | -6926.3495 | Lower | -1125.4888 | Lower |
| SLOC | **41 ✅** | 55 | 47 | -14 | Lower | -6 | Lower | +8 | Higher |
| PLOC | **24 ✅** | 35 | 30 | -11 | Lower | -6 | Lower | +5 | Higher |
| LLOC | **18 ✅** | 24 | 21 | -6 | Lower | -3 | Lower | +3 | Higher |
| Comments (CLOC) | 9 | **13 ✅** | 8 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 8 | 8 | 10 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.8898 | **69.4483 ✅** | 52.6284 | -14.5585 | Lower | +2.2614 | Higher | +16.8199 | Higher |
| MI (SEI) | 31.5940 | **55.3357 ✅** | 27.8731 | -23.7416 | Lower | +3.7209 | Higher | +27.4625 | Higher |
| Cyclomatic (sum) | 19 | **10 ✅** | 17 | +9 | Higher | +2 | Higher | -7 | Lower |
| Cognitive (sum) | 19 | **13 ✅** | 31 | +6 | Higher | -12 | Lower | -18 | Lower |
| Halstead Volume | 1949.7947 | **1202.5228 ✅** | 2344.5715 | +747.2719 | Higher | -394.7768 | Lower | -1142.0487 | Lower |
| Halstead Effort | 60655.5710 | **26856.3430 ✅** | 87462.7108 | +33799.2280 | Higher | -26807.1397 | Lower | -60606.3678 | Lower |
| SLOC | 87 | **47 ✅** | 97 | +40 | Higher | -10 | Lower | -50 | Lower |
| PLOC | 67 | **33 ✅** | 71 | +34 | Higher | -4 | Lower | -38 | Lower |
| LLOC | 39 | **22 ✅** | 46 | +17 | Higher | -7 | Lower | -24 | Lower |
| Comments (CLOC) | 11 | 8 | **12 ✅** | +3 | Higher | -1 | Lower | -4 | Lower |
| Blank Lines | 11 | **7 ✅** | 14 | +4 | Higher | -3 | Lower | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.6625 | **60.9130 ✅** | 57.2187 | -6.2505 | Lower | -2.5562 | Lower | +3.6943 | Higher |
| MI (SEI) | 32.4221 | **45.4372 ✅** | 34.9608 | -13.0151 | Lower | -2.5387 | Lower | +10.4764 | Higher |
| Cyclomatic (sum) | 16 | **13 ✅** | 15 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 24 | **8 ✅** | 23 | +16 | Higher | +1 | Higher | -15 | Lower |
| Halstead Volume | 2022 | **1571.6454 ✅** | 1658.8280 | +450.3546 | Higher | +363.1720 | Higher | -87.1826 | Lower |
| Halstead Effort | 71578.8000 | **42306.9949 ✅** | 72359.2195 | +29271.8051 | Higher | -780.4195 | Lower | -30052.2246 | Lower |
| SLOC | 91 | **70 ✅** | 84 | +21 | Higher | +7 | Higher | -14 | Lower |
| PLOC | 64 | **47 ✅** | 60 | +17 | Higher | +4 | Higher | -13 | Lower |
| LLOC | 51 | **36 ✅** | 50 | +15 | Higher | +1 | Higher | -14 | Lower |
| Comments (CLOC) | 13 | **14 ✅** | 11 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 16 | **9 ✅** | 13 | +7 | Higher | +3 | Higher | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p00137

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.1289 | **81.8403 ✅** | 75.0010 | -2.7114 | Lower | +4.1279 | Higher | +6.8394 | Higher |
| MI (SEI) | 64.5446 | **77.2639 ✅** | 61.3458 | -12.7192 | Lower | +3.1989 | Higher | +15.9181 | Higher |
| Cyclomatic (sum) | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 733.3552 | **651.1518 ✅** | 815.4045 | +82.2034 | Higher | -82.0494 | Lower | -164.2528 | Lower |
| Halstead Effort | 13630.8407 | **8302.1851 ✅** | 16903.9630 | +5328.6556 | Higher | -3273.1224 | Lower | -8601.7780 | Lower |
| SLOC | 33 | **29 ✅** | 40 | +4 | Higher | -7 | Lower | -11 | Lower |
| PLOC | 22 | **17 ✅** | 25 | +5 | Higher | -3 | Lower | -8 | Lower |
| LLOC | 22 | **13 ✅** | 22 | +9 | Higher | 0 | Same | -9 | Lower |
| Comments (CLOC) | 4 | **7 ✅** | 6 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | 7 | **5 ✅** | 9 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p00156

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **47.6340 ✅** | 43.7594 | 33.9235 | +3.8746 | Higher | +13.7105 | Higher | +9.8359 | Higher |
| MI (SEI) | **19.0872 ✅** | 10.5553 | -6.0020 | +8.5320 | Higher | +25.0892 | Higher | +16.5573 | Higher |
| Cyclomatic (sum) | **26 ✅** | 32 | 41 | -6 | Lower | -15 | Lower | -9 | Lower |
| Cognitive (sum) | **36 ✅** | 38 | 51 | -2 | Lower | -15 | Lower | -13 | Lower |
| Halstead Volume | **3627.7153 ✅** | 3911.4677 | 5346.9509 | -283.7525 | Lower | -1719.2356 | Lower | -1435.4832 | Lower |
| Halstead Effort | **204801.0166 ✅** | 208285.6564 | 388265.0200 | -3484.6398 | Lower | -183464.0034 | Lower | -179979.3636 | Lower |
| SLOC | **101 ✅** | 115 | 168 | -14 | Lower | -67 | Lower | -53 | Lower |
| PLOC | **79 ✅** | 92 | 137 | -13 | Lower | -58 | Lower | -45 | Lower |
| LLOC | **62 ✅** | 67 | 116 | -5 | Lower | -54 | Lower | -49 | Lower |
| Comments (CLOC) | **10 ✅** | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |
| Blank Lines | **14 ✅** | 15 | 24 | -1 | Lower | -10 | Lower | -9 | Lower |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 4 | -3 | Lower | -4 | Lower | -1 | Lower |

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

## p00190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 28.3691 | **36.9663 ✅** | 29.6366 | -8.5972 | Lower | -1.2675 | Lower | +7.3297 | Higher |
| MI (SEI) | -9.1019 | **9.4199 ✅** | -9.0064 | -18.5219 | Lower | -0.0956 | Lower | +18.4263 | Higher |
| Cyclomatic (sum) | 38 | **34 ✅** | 35 | +4 | Higher | +3 | Higher | -1 | Lower |
| Cognitive (sum) | 60 | **51 ✅** | 57 | +9 | Higher | +3 | Higher | -6 | Lower |
| Halstead Volume | 7250.9687 | **3772.3081 ✅** | 7061.5918 | +3478.6606 | Higher | +189.3769 | Higher | -3289.2837 | Lower |
| Halstead Effort | 624107.4723 | **205160.6141 ✅** | 596748.0945 | +418946.8582 | Higher | +27359.3777 | Higher | -391587.4804 | Lower |
| SLOC | 224 | **172 ✅** | 218 | +52 | Higher | +6 | Higher | -46 | Lower |
| PLOC | 193 | **128 ✅** | 191 | +65 | Higher | +2 | Higher | -63 | Lower |
| LLOC | 122 | **90 ✅** | 120 | +32 | Higher | +2 | Higher | -30 | Lower |
| Comments (CLOC) | 19 | **26 ✅** | 16 | -7 | Lower | +3 | Higher | +10 | Higher |
| Blank Lines | 26 | **18 ✅** | 24 | +8 | Higher | +2 | Higher | -6 | Lower |
| Functions | 5 | **1 ✅** | 5 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 10 | **0 ✅** | 11 | +10 | Higher | -1 | Lower | -11 | Lower |

## p00203

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.9831 ✅** | 32.8191 | 34.3441 | +7.1640 | Higher | +5.6390 | Higher | -1.5250 | Lower |
| MI (SEI) | **12.8328 ✅** | 5.1605 | 4.0915 | +7.6723 | Higher | +8.7413 | Higher | +1.0690 | Higher |
| Cyclomatic (sum) | **36 ✅** | 41 | 42 | -5 | Lower | -6 | Lower | -1 | Lower |
| Cognitive (sum) | **68 ✅** | 72 | 77 | -4 | Lower | -9 | Lower | -5 | Lower |
| Halstead Volume | **3839.0025 ✅** | 5620.0664 | 4631.7290 | -1781.0639 | Lower | -792.7265 | Lower | +988.3374 | Higher |
| Halstead Effort | **300193.3023 ✅** | 393179.8455 | 321297.7249 | -92986.5432 | Lower | -21104.4226 | Lower | +71882.1206 | Higher |
| SLOC | **138 ✅** | 177 | 169 | -39 | Lower | -31 | Lower | +8 | Higher |
| PLOC | **106 ✅** | 130 | 132 | -24 | Lower | -26 | Lower | -2 | Lower |
| LLOC | **62 ✅** | 96 | 84 | -34 | Lower | -22 | Lower | +12 | Higher |
| Comments (CLOC) | 19 | **29 ✅** | 21 | -10 | Lower | -2 | Lower | +8 | Higher |
| Blank Lines | **14 ✅** | 19 | 18 | -5 | Lower | -4 | Lower | +1 | Higher |
| Functions | 6 | **1 ✅** | 5 | +5 | Higher | +1 | Higher | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **0 ✅** | 6 | +7 | Higher | +1 | Higher | -6 | Lower |

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

## p00244

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.4942 | **49.8802 ✅** | 30.7660 | -1.3859 | Lower | +17.7282 | Higher | +19.1142 | Higher |
| MI (SEI) | 21.2978 | **24.4906 ✅** | -14.3875 | -3.1928 | Lower | +35.6854 | Higher | +38.8782 | Higher |
| Cyclomatic (sum) | 21 | **17 ✅** | 37 | +4 | Higher | -16 | Lower | -20 | Lower |
| Cognitive (sum) | 33 | **23 ✅** | 42 | +10 | Higher | -9 | Lower | -19 | Lower |
| Halstead Volume | **3299.5695 ✅** | 3617.2817 | 6495.4616 | -317.7122 | Lower | -3195.8921 | Lower | -2878.1799 | Lower |
| Halstead Effort | **171323.8005 ✅** | 190165.6669 | 494751.7156 | -18841.8664 | Lower | -323427.9151 | Lower | -304586.0487 | Lower |
| SLOC | 106 | **100 ✅** | 203 | +6 | Higher | -97 | Lower | -103 | Lower |
| PLOC | 79 | **72 ✅** | 173 | +7 | Higher | -94 | Lower | -101 | Lower |
| LLOC | 58 | **50 ✅** | 121 | +8 | Higher | -63 | Lower | -71 | Lower |
| Comments (CLOC) | 12 | **13 ✅** | 6 | -1 | Lower | +6 | Higher | +7 | Higher |
| Blank Lines | 15 | 15 | 24 | 0 | Same | -9 | Lower | -9 | Lower |
| Functions | 2 | **1 ✅** | 9 | +1 | Higher | -7 | Lower | -8 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | **1 ✅** | 2 | 7 | -1 | Lower | -6 | Lower | -5 | Lower |

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

## p00256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **44.6047 ✅** | 30.2318 | 42.2643 | +14.3729 | Higher | +2.3404 | Higher | -12.0324 | Lower |
| MI (SEI) | **19.2332 ✅** | -4.1375 | 13.0710 | +23.3707 | Higher | +6.1623 | Higher | -17.2085 | Lower |
| Cyclomatic (sum) | **20 ✅** | 50 | 25 | -30 | Lower | -5 | Lower | +25 | Higher |
| Cognitive (sum) | **42 ✅** | 61 | 59 | -19 | Lower | -17 | Lower | +2 | Higher |
| Halstead Volume | **3858.1037 ✅** | 5790.3319 | 4119.4340 | -1932.2282 | Lower | -261.3303 | Lower | +1670.8979 | Higher |
| Halstead Effort | **273559.5079 ✅** | 467936.0251 | 285986.4694 | -194376.5172 | Lower | -12426.9614 | Lower | +181949.5557 | Higher |
| SLOC | **130 ✅** | 181 | 137 | -51 | Lower | -7 | Lower | +44 | Higher |
| PLOC | **89 ✅** | 148 | 105 | -59 | Lower | -16 | Lower | +43 | Higher |
| LLOC | 96 | **86 ✅** | 108 | +10 | Higher | -12 | Lower | -22 | Lower |
| Comments (CLOC) | **20 ✅** | 17 | 16 | +3 | Higher | +4 | Higher | +1 | Higher |
| Blank Lines | 22 | 17 | 17 | +5 | Higher | +5 | Higher | 0 | Same |
| Functions | 2 | 7 | 2 | -5 | Lower | 0 | Same | +5 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 7 | 1 | -6 | Lower | 0 | Same | +6 | Higher |

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

## p00288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 12.9919 | **34.6532 ✅** | 11.8334 | -21.6613 | Lower | +1.1586 | Higher | +22.8199 | Higher |
| MI (SEI) | -36.8290 | **1.4841 ✅** | -43.0592 | -38.3131 | Lower | +6.2302 | Higher | +44.5433 | Higher |
| Cyclomatic (sum) | 63 | **27 ✅** | 64 | +36 | Higher | -1 | Lower | -37 | Lower |
| Cognitive (sum) | 105 | **37 ✅** | 88 | +68 | Higher | +17 | Higher | -51 | Lower |
| Halstead Volume | 12356.6235 | **6501.3660 ✅** | 12375.3375 | +5855.2576 | Higher | -18.7139 | Lower | -5873.9715 | Lower |
| Halstead Effort | 1370710.9240 | **434259.9158 ✅** | 1268898.8274 | +936451.0082 | Higher | +101812.0966 | Higher | -834638.9115 | Lower |
| SLOC | 342 | **184 ✅** | 362 | +158 | Higher | -20 | Lower | -178 | Lower |
| PLOC | 293 | **139 ✅** | 318 | +154 | Higher | -25 | Lower | -179 | Lower |
| LLOC | 219 | **103 ✅** | 222 | +116 | Higher | -3 | Lower | -119 | Lower |
| Comments (CLOC) | 11 | **20 ✅** | 5 | -9 | Lower | +6 | Higher | +15 | Higher |
| Blank Lines | 40 | **26 ✅** | 40 | +14 | Higher | 0 | Same | -14 | Lower |
| Functions | 8 | **4 ✅** | 13 | +4 | Higher | -5 | Lower | -9 | Lower |
| Closures | 4 | 0 | 0 | +4 | Higher | +4 | Higher | 0 | Same |
| Exits | 7 | **4 ✅** | 9 | +3 | Higher | -2 | Lower | -5 | Lower |

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

## p00329

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.4131 | **47.3327 ✅** | 41.2688 | -9.9195 | Lower | -3.8556 | Lower | +6.0639 | Higher |
| MI (SEI) | -0.6150 | **23.5284 ✅** | 4.2373 | -24.1434 | Lower | -4.8523 | Lower | +19.2912 | Higher |
| Cyclomatic (sum) | 33 | 27 | **25 ✅** | +6 | Higher | +8 | Higher | +2 | Higher |
| Cognitive (sum) | 40 | 48 | **31 ✅** | -8 | Lower | +9 | Higher | +17 | Higher |
| Halstead Volume | 4196.9807 | **3072.6227 ✅** | 4271.2764 | +1124.3581 | Higher | -74.2957 | Lower | -1198.6538 | Lower |
| Halstead Effort | 243900.0106 | **185464.9323 ✅** | 259848.9254 | +58435.0783 | Higher | -15948.9148 | Lower | -74383.9931 | Lower |
| SLOC | 164 | **107 ✅** | 144 | +57 | Higher | +20 | Higher | -37 | Lower |
| PLOC | 137 | **79 ✅** | 124 | +58 | Higher | +13 | Higher | -45 | Lower |
| LLOC | 85 | **63 ✅** | 84 | +22 | Higher | +1 | Higher | -21 | Lower |
| Comments (CLOC) | 9 | **16 ✅** | 8 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 18 | 15 | **12 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| Functions | 8 | **2 ✅** | 6 | +6 | Higher | +2 | Higher | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 8 | **4 ✅** | 7 | +4 | Higher | +1 | Higher | -3 | Lower |

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

## p00453

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.5208 | **62.6818 ✅** | 42.6531 | -16.1610 | Lower | +3.8677 | Higher | +20.0287 | Higher |
| MI (SEI) | 21.5666 | **53.7027 ✅** | 10.0019 | -32.1361 | Lower | +11.5647 | Higher | +43.7007 | Higher |
| Cyclomatic (sum) | 27 | **15 ✅** | 29 | +12 | Higher | -2 | Lower | -14 | Lower |
| Cognitive (sum) | 58 | **13 ✅** | 58 | +45 | Higher | 0 | Same | -45 | Lower |
| Halstead Volume | 3591.8124 | **1173.4744 ✅** | 4261.3558 | +2418.3380 | Higher | -669.5434 | Lower | -3087.8814 | Lower |
| Halstead Effort | 254099.8433 | **36612.4000 ✅** | 286535.2020 | +217487.4433 | Higher | -32435.3587 | Lower | -249922.8021 | Lower |
| SLOC | 107 | **67 ✅** | 125 | +40 | Higher | -18 | Lower | -58 | Lower |
| PLOC | 85 | **38 ✅** | 101 | +47 | Higher | -16 | Lower | -63 | Lower |
| LLOC | 65 | **20 ✅** | 80 | +45 | Higher | -15 | Lower | -60 | Lower |
| Comments (CLOC) | 15 | **20 ✅** | 10 | -5 | Lower | +5 | Higher | +10 | Higher |
| Blank Lines | 11 | **9 ✅** | 14 | +2 | Higher | -3 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00465

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **44.8281 ✅** | 36.3164 | 28.3066 | +8.5118 | Higher | +16.5215 | Higher | +8.0097 | Higher |
| MI (SEI) | **11.6700 ✅** | -7.2809 | -14.8684 | +18.9509 | Higher | +26.5385 | Higher | +7.5875 | Higher |
| Cyclomatic (sum) | **27 ✅** | 45 | 44 | -18 | Lower | -17 | Lower | +1 | Higher |
| Cognitive (sum) | **30 ✅** | 49 | 47 | -19 | Lower | -17 | Lower | +2 | Higher |
| Halstead Volume | **4082.6768 ✅** | 5588.6761 | 6213.1830 | -1505.9994 | Lower | -2130.5063 | Lower | -624.5069 | Lower |
| Halstead Effort | **215157.0647 ✅** | 343368.2615 | 473492.9617 | -128211.1968 | Lower | -258335.8969 | Lower | -130124.7001 | Lower |
| SLOC | **114 ✅** | 135 | 217 | -21 | Lower | -103 | Lower | -82 | Lower |
| PLOC | **92 ✅** | 115 | 182 | -23 | Lower | -90 | Lower | -67 | Lower |
| LLOC | 65 | **59 ✅** | 123 | +6 | Higher | -58 | Lower | -64 | Lower |
| Comments (CLOC) | 8 | 3 | **9 ✅** | +5 | Higher | -1 | Lower | -6 | Lower |
| Blank Lines | 17 | 17 | 27 | 0 | Same | -10 | Lower | -10 | Lower |
| Functions | 4 | 4 | 10 | 0 | Same | -6 | Lower | -6 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **3 ✅** | 6 | 7 | -3 | Lower | -4 | Lower | -1 | Lower |

## p00468

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.5653 ✅** | 60.3079 | 58.1449 | +2.2574 | Higher | +4.4204 | Higher | +2.1630 | Higher |
| MI (SEI) | 41.9227 | **43.3269 ✅** | 35.2246 | -1.4042 | Lower | +6.6982 | Higher | +8.1023 | Higher |
| Cyclomatic (sum) | **13 ✅** | 17 | 16 | -4 | Lower | -3 | Lower | +1 | Higher |
| Cognitive (sum) | 17 | **16 ✅** | 21 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | **1512.1710 ✅** | 1619.0913 | 1814.0288 | -106.9203 | Lower | -301.8578 | Lower | -194.9375 | Lower |
| Halstead Effort | **45152.4810 ✅** | 54531.8942 | 62713.5669 | -9379.4133 | Lower | -17561.0859 | Lower | -8181.6726 | Lower |
| SLOC | **64 ✅** | 68 | 76 | -4 | Lower | -12 | Lower | -8 | Lower |
| PLOC | **46 ✅** | 47 | 56 | -1 | Lower | -10 | Lower | -9 | Lower |
| LLOC | 37 | **34 ✅** | 47 | +3 | Higher | -10 | Lower | -13 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 9 | -4 | Lower | -1 | Lower | +3 | Higher |
| Blank Lines | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00479

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.1269 ✅** | 70.6157 | 74.0896 | +4.5111 | Higher | +1.0373 | Higher | -3.4739 | Lower |
| MI (SEI) | 54.6432 | **60.6630 ✅** | 56.0888 | -6.0198 | Lower | -1.4455 | Lower | +4.5743 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 8 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **1014.7115 ✅** | 1027.3825 | 1043.4521 | -12.6710 | Lower | -28.7406 | Lower | -16.0696 | Lower |
| Halstead Effort | 23697.7417 | **20604.7273 ✅** | 23790.7076 | +3093.0144 | Higher | -92.9659 | Lower | -3185.9803 | Lower |
| SLOC | **37 ✅** | 48 | 38 | -11 | Lower | -1 | Lower | +10 | Higher |
| PLOC | **26 ✅** | 30 | 28 | -4 | Lower | -2 | Lower | +2 | Higher |
| LLOC | 18 | 17 | 17 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 3 | **11 ✅** | 4 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 8 | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p00495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.9905 ✅** | 55.7335 | 61.3827 | +8.2569 | Higher | +2.6077 | Higher | -5.6492 | Lower |
| MI (SEI) | 32.2483 | **41.7193 ✅** | 30.3952 | -9.4710 | Lower | +1.8531 | Higher | +11.3241 | Higher |
| Cyclomatic (sum) | 13 | 14 | 13 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 15 | 26 | 15 | -11 | Lower | 0 | Same | +11 | Higher |
| Halstead Volume | **1649.2888 ✅** | 1995.7433 | 1724.7844 | -346.4545 | Lower | -75.4956 | Lower | +270.9589 | Higher |
| Halstead Effort | **71402.8305 ✅** | 75302.8037 | 78685.8544 | -3899.9732 | Lower | -7283.0238 | Lower | -3383.0507 | Lower |
| SLOC | **57 ✅** | 88 | 66 | -31 | Lower | -9 | Lower | +22 | Higher |
| PLOC | **45 ✅** | 53 | 49 | -8 | Lower | -4 | Lower | +4 | Higher |
| LLOC | **33 ✅** | 41 | 39 | -8 | Lower | -6 | Lower | +2 | Higher |
| Comments (CLOC) | 2 | **23 ✅** | 3 | -21 | Lower | -1 | Lower | +20 | Higher |
| Blank Lines | **10 ✅** | 13 | 14 | -3 | Lower | -4 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p00507

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.9131 ✅** | 67.4496 | 64.1118 | +1.4635 | Higher | +4.8012 | Higher | +3.3377 | Higher |
| MI (SEI) | **59.8766 ✅** | 59.5069 | 47.8878 | +0.3696 | Higher | +11.9887 | Higher | +11.6191 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 13 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 17 | **8 ✅** | 23 | +9 | Higher | -6 | Lower | -15 | Lower |
| Halstead Volume | **1080.1680 ✅** | 1182.4113 | 1447.0999 | -102.2432 | Lower | -366.9319 | Lower | -264.6887 | Lower |
| Halstead Effort | 30892.8052 | **26440.0294 ✅** | 43714.4765 | +4452.7758 | Higher | -12821.6712 | Lower | -17274.4471 | Lower |
| SLOC | **51 ✅** | 55 | 59 | -4 | Lower | -8 | Lower | -4 | Lower |
| PLOC | **30 ✅** | 33 | 41 | -3 | Lower | -11 | Lower | -8 | Lower |
| LLOC | 21 | 21 | 31 | 0 | Same | -10 | Lower | -10 | Lower |
| Comments (CLOC) | 13 | **16 ✅** | 10 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00514

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.1578 ✅** | 50.6977 | 49.3244 | +0.4601 | Higher | +1.8334 | Higher | +1.3733 | Higher |
| MI (SEI) | 23.9082 | **28.6035 ✅** | 16.6067 | -4.6953 | Lower | +7.3015 | Higher | +11.9968 | Higher |
| Cyclomatic (sum) | 18 | **17 ✅** | 19 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 16 | **13 ✅** | 17 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | 3073.9803 | **2819.0918 ✅** | 2899.0437 | +254.8885 | Higher | +174.9366 | Higher | -79.9519 | Lower |
| Halstead Effort | 179998.6264 | **138981.2271 ✅** | 167359.3777 | +41017.3993 | Higher | +12639.2486 | Higher | -28378.1506 | Lower |
| SLOC | **96 ✅** | 103 | 108 | -7 | Lower | -12 | Lower | -5 | Lower |
| PLOC | 75 | **73 ✅** | 88 | +2 | Higher | -13 | Lower | -15 | Lower |
| LLOC | 64 | **59 ✅** | 69 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | 10 | **17 ✅** | 7 | -7 | Lower | +3 | Higher | +10 | Higher |
| Blank Lines | **12 ✅** | 14 | 13 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

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

## p00519

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.2992 | **48.1579 ✅** | 27.3271 | -12.8587 | Lower | +7.9721 | Higher | +20.8308 | Higher |
| MI (SEI) | -2.3093 | **22.8898 ✅** | -12.2428 | -25.1992 | Lower | +9.9334 | Higher | +35.1326 | Higher |
| Cyclomatic (sum) | 41 | **19 ✅** | 44 | +22 | Higher | -3 | Lower | -25 | Lower |
| Cognitive (sum) | 35 | **33 ✅** | 62 | +2 | Higher | -27 | Lower | -29 | Lower |
| Halstead Volume | 5381.9462 | **3525.3968 ✅** | 6890.0116 | +1856.5494 | Higher | -1508.0655 | Lower | -3364.6148 | Lower |
| Halstead Effort | 353689.3493 | **169603.6354 ✅** | 526305.0227 | +184085.7139 | Higher | -172615.6733 | Lower | -356701.3873 | Lower |
| SLOC | 154 | **109 ✅** | 223 | +45 | Higher | -69 | Lower | -114 | Lower |
| PLOC | 128 | **81 ✅** | 188 | +47 | Higher | -60 | Lower | -107 | Lower |
| LLOC | **54 ✅** | 66 | 96 | -12 | Lower | -42 | Lower | -30 | Lower |
| Comments (CLOC) | 9 | 15 | 15 | -6 | Lower | -6 | Lower | 0 | Same |
| Blank Lines | 18 | **13 ✅** | 21 | +5 | Higher | -3 | Lower | -8 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 10 | **0 ✅** | 5 | +10 | Higher | +5 | Higher | -5 | Lower |
| Exits | **0 ✅** | 1 | 7 | -1 | Lower | -7 | Lower | -6 | Lower |

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

## p00534

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.0335 ✅** | 63.9768 | 67.3190 | +6.0567 | Higher | +2.7145 | Higher | -3.3422 | Lower |
| MI (SEI) | **56.5372 ✅** | 55.2266 | 49.8249 | +1.3106 | Higher | +6.7123 | Higher | +5.4017 | Higher |
| Cyclomatic (sum) | 8 | 8 | 10 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **1344.2105 ✅** | 1587.5128 | 1493.5152 | -243.3023 | Lower | -149.3047 | Lower | +93.9976 | Higher |
| Halstead Effort | **44309.1610 ✅** | 51117.9138 | 53119.3575 | -6808.7528 | Lower | -8810.1965 | Lower | -2001.4437 | Lower |
| SLOC | **45 ✅** | 62 | 50 | -17 | Lower | -5 | Lower | +12 | Higher |
| PLOC | **29 ✅** | 33 | 35 | -4 | Lower | -6 | Lower | -2 | Lower |
| LLOC | **25 ✅** | 27 | 28 | -2 | Lower | -3 | Lower | -1 | Lower |
| Comments (CLOC) | 8 | **19 ✅** | 7 | -11 | Lower | +1 | Higher | +12 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00543

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.5417 ✅** | 72.8729 | 62.8684 | +1.6687 | Higher | +11.6733 | Higher | +10.0046 | Higher |
| MI (SEI) | 61.4247 | **61.4340 ✅** | 47.6114 | -0.0093 | Lower | +13.8133 | Higher | +13.8226 | Higher |
| Cyclomatic (sum) | 7 | 7 | 13 | 0 | Same | -6 | Lower | -6 | Lower |
| Cognitive (sum) | 8 | **7 ✅** | 11 | +1 | Higher | -3 | Lower | -4 | Lower |
| Halstead Volume | **999.8260 ✅** | 1008.9778 | 1498.2976 | -9.1518 | Lower | -498.4716 | Lower | -489.3197 | Lower |
| Halstead Effort | 29703.1639 | **21457.5952 ✅** | 60681.0516 | +8245.5687 | Higher | -30977.8877 | Lower | -39223.4564 | Lower |
| SLOC | **38 ✅** | 42 | 63 | -4 | Lower | -25 | Lower | -21 | Lower |
| PLOC | **27 ✅** | 29 | 45 | -2 | Lower | -18 | Lower | -16 | Lower |
| LLOC | 21 | **19 ✅** | 35 | +2 | Higher | -14 | Lower | -16 | Lower |
| Comments (CLOC) | 6 | 8 | **12 ✅** | -2 | Lower | -6 | Lower | -4 | Lower |
| Blank Lines | 5 | 5 | 7 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

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

## p00571

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.9611 | **61.3707 ✅** | 50.5320 | -11.4097 | Lower | -0.5710 | Lower | +10.8387 | Higher |
| MI (SEI) | 11.8481 | **50.3960 ✅** | 19.9646 | -38.5479 | Lower | -8.1165 | Lower | +30.4314 | Higher |
| Cyclomatic (sum) | 23 | **11 ✅** | 19 | +12 | Higher | +4 | Higher | -8 | Lower |
| Cognitive (sum) | 19 | 13 | **11 ✅** | +6 | Higher | +8 | Higher | +2 | Higher |
| Halstead Volume | 2567.8982 | **1440.2240 ✅** | 2584.9598 | +1127.6742 | Higher | -17.0615 | Lower | -1144.7357 | Lower |
| Halstead Effort | 109326.9003 | **46447.2247 ✅** | 109338.2978 | +62879.6756 | Higher | -11.3975 | Lower | -62891.0731 | Lower |
| SLOC | 102 | **72 ✅** | 104 | +30 | Higher | -2 | Lower | -32 | Lower |
| PLOC | 84 | **43 ✅** | 81 | +41 | Higher | +3 | Higher | -38 | Lower |
| LLOC | 49 | **31 ✅** | 49 | +18 | Higher | 0 | Same | -18 | Lower |
| Comments (CLOC) | 3 | **20 ✅** | 8 | -17 | Lower | -5 | Lower | +12 | Higher |
| Blank Lines | 15 | **12 ✅** | 15 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 5 | **1 ✅** | 5 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 5 | +2 | Higher | -2 | Lower | -4 | Lower |

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

## p00629

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.7343 ✅** | 48.7571 | 51.6797 | +3.9772 | Higher | +1.0546 | Higher | -2.9226 | Lower |
| MI (SEI) | **33.3947 ✅** | 19.7294 | 22.2455 | +13.6653 | Higher | +11.1492 | Higher | -2.5161 | Lower |
| Cyclomatic (sum) | **19 ✅** | 33 | 23 | -14 | Lower | -4 | Lower | +10 | Higher |
| Cognitive (sum) | **15 ✅** | 28 | 23 | -13 | Lower | -8 | Lower | +5 | Higher |
| Halstead Volume | **2171.8136 ✅** | 2595.5515 | 2821.6549 | -423.7379 | Lower | -649.8413 | Lower | -226.1034 | Lower |
| Halstead Effort | **62604.8877 ✅** | 97664.5285 | 109339.1261 | -35059.6408 | Lower | -46734.2384 | Lower | -11674.5976 | Lower |
| SLOC | 96 | 95 | **89 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |
| PLOC | **67 ✅** | 78 | 74 | -11 | Lower | -7 | Lower | +4 | Higher |
| LLOC | 37 | **33 ✅** | 55 | +4 | Higher | -18 | Lower | -22 | Lower |
| Comments (CLOC) | **18 ✅** | 8 | 7 | +10 | Higher | +11 | Higher | +1 | Higher |
| Blank Lines | 13 | **9 ✅** | 10 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | 6 | **0 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

## p00641

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.9280 | 33.5020 | **40.6497 ✅** | +6.4260 | Higher | -0.7218 | Lower | -7.1477 | Lower |
| MI (SEI) | 1.7057 | **18.8233 ✅** | 5.3512 | -17.1175 | Lower | -3.6455 | Lower | +13.4720 | Higher |
| Cyclomatic (sum) | 30 | 33 | **27 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Cognitive (sum) | 19 | 31 | **17 ✅** | -12 | Lower | +2 | Higher | +14 | Higher |
| Halstead Volume | 4430.9021 | **3565.6063 ✅** | 4129.9268 | +865.2958 | Higher | +300.9753 | Higher | -564.3205 | Lower |
| Halstead Effort | 317649.8411 | **215521.0891 ✅** | 273926.7799 | +102128.7520 | Higher | +43723.0611 | Higher | -58405.6908 | Lower |
| SLOC | **144 ✅** | 220 | 147 | -76 | Lower | -3 | Lower | +73 | Higher |
| PLOC | 117 | **107 ✅** | 119 | +10 | Higher | -2 | Lower | -12 | Lower |
| LLOC | 76 | **63 ✅** | 80 | +13 | Higher | -4 | Lower | -17 | Lower |
| Comments (CLOC) | 7 | **97 ✅** | 10 | -90 | Lower | -3 | Lower | +87 | Higher |
| Blank Lines | 21 | **16 ✅** | 19 | +5 | Higher | +2 | Higher | -3 | Lower |
| Functions | 7 | **4 ✅** | 8 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |

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

## p00655

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 23.5938 | **29.2647 ✅** | 19.9478 | -5.6709 | Lower | +3.6460 | Higher | +9.3169 | Higher |
| MI (SEI) | -22.8708 | **1.5942 ✅** | -27.7158 | -24.4650 | Lower | +4.8450 | Higher | +29.3100 | Higher |
| Cyclomatic (sum) | 54 | **45 ✅** | 63 | +9 | Higher | -9 | Lower | -18 | Lower |
| Cognitive (sum) | **48 ✅** | 60 | 69 | -12 | Lower | -21 | Lower | -9 | Lower |
| Halstead Volume | 8024.1108 | **5316.5771 ✅** | 8501.9483 | +2707.5336 | Higher | -477.8375 | Lower | -3185.3711 | Lower |
| Halstead Effort | 977037.4903 | **349519.1148 ✅** | 1111326.0954 | +627518.3754 | Higher | -134288.6051 | Lower | -761806.9805 | Lower |
| SLOC | 232 | **212 ✅** | 251 | +20 | Higher | -19 | Lower | -39 | Lower |
| PLOC | 207 | **154 ✅** | 227 | +53 | Higher | -20 | Lower | -73 | Lower |
| LLOC | 131 | **84 ✅** | 153 | +47 | Higher | -22 | Lower | -69 | Lower |
| Comments (CLOC) | 7 | **38 ✅** | 7 | -31 | Lower | 0 | Same | +31 | Higher |
| Blank Lines | 20 | 20 | **19 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 12 | **9 ✅** | 12 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 12 | **7 ✅** | 19 | +5 | Higher | -7 | Lower | -12 | Lower |

## p00658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -11.9178 | **42.4616 ✅** | -7.7188 | -54.3795 | Lower | -4.1991 | Lower | +50.1804 | Higher |
| MI (SEI) | -63.5745 | **24.5594 ✅** | -64.1216 | -88.1339 | Lower | +0.5471 | Higher | +88.6810 | Higher |
| Cyclomatic (sum) | 147 | **32 ✅** | 133 | +115 | Higher | +14 | Higher | -101 | Lower |
| Cognitive (sum) | 181 | **31 ✅** | 176 | +150 | Higher | +5 | Higher | -145 | Lower |
| Halstead Volume | 15081.5816 | **2720.0864 ✅** | 15466.1326 | +12361.4951 | Higher | -384.5510 | Lower | -12746.0461 | Lower |
| Halstead Effort | 2183140.8951 | **81079.4999 ✅** | 2241285.9004 | +2102061.3952 | Higher | -58145.0053 | Lower | -2160206.4005 | Lower |
| SLOC | 453 | **140 ✅** | 423 | +313 | Higher | +30 | Higher | -283 | Lower |
| PLOC | 410 | **87 ✅** | 387 | +323 | Higher | +23 | Higher | -300 | Lower |
| LLOC | 262 | **43 ✅** | 265 | +219 | Higher | -3 | Lower | -222 | Lower |
| Comments (CLOC) | 16 | **37 ✅** | 6 | -21 | Lower | +10 | Higher | +31 | Higher |
| Blank Lines | 28 | **16 ✅** | 30 | +12 | Higher | -2 | Lower | -14 | Lower |
| Functions | 11 | **1 ✅** | 12 | +10 | Higher | -1 | Lower | -11 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 29 | **0 ✅** | 27 | +29 | Higher | +2 | Higher | -27 | Lower |

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

## p00692

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.2508 | **46.6694 ✅** | 40.6258 | -1.4187 | Lower | +4.6250 | Higher | +6.0437 | Higher |
| MI (SEI) | **20.7784 ✅** | 16.6469 | 9.8502 | +4.1315 | Higher | +10.9282 | Higher | +6.7967 | Higher |
| Cyclomatic (sum) | 35 | **22 ✅** | 40 | +13 | Higher | -5 | Lower | -18 | Lower |
| Cognitive (sum) | **30 ✅** | 38 | 44 | -8 | Lower | -14 | Lower | -6 | Lower |
| Halstead Volume | 3413.8412 | **3210.3375 ✅** | 4753.9685 | +203.5037 | Higher | -1340.1273 | Lower | -1543.6309 | Lower |
| Halstead Effort | 208927.0797 | **148616.4867 ✅** | 312403.6414 | +60310.5930 | Higher | -103476.5617 | Lower | -163787.1547 | Lower |
| SLOC | **105 ✅** | 118 | 117 | -13 | Lower | -12 | Lower | +1 | Higher |
| PLOC | **80 ✅** | 93 | 94 | -13 | Lower | -14 | Lower | -1 | Lower |
| LLOC | **50 ✅** | 59 | 72 | -9 | Lower | -22 | Lower | -13 | Lower |
| Comments (CLOC) | **15 ✅** | 11 | 11 | +4 | Higher | +4 | Higher | 0 | Same |
| Blank Lines | **10 ✅** | 14 | 12 | -4 | Lower | -2 | Lower | +2 | Higher |
| Functions | **3 ✅** | 6 | 6 | -3 | Lower | -3 | Lower | 0 | Same |
| Closures | 4 | 0 | 0 | +4 | Higher | +4 | Higher | 0 | Same |
| Exits | **3 ✅** | 9 | 6 | -6 | Lower | -3 | Lower | +3 | Higher |

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

## p00740

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.9355 | **66.9922 ✅** | 61.5398 | -4.0567 | Lower | +1.3957 | Higher | +5.4524 | Higher |
| MI (SEI) | 39.3590 | **54.7263 ✅** | 33.1891 | -15.3673 | Lower | +6.1699 | Higher | +21.5372 | Higher |
| Cyclomatic (sum) | 14 | **9 ✅** | 16 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 18 | **4 ✅** | 22 | +14 | Higher | -4 | Lower | -18 | Lower |
| Halstead Volume | 1347.3356 | **1167.8347 ✅** | 1398.4377 | +179.5010 | Higher | -51.1020 | Lower | -230.6030 | Lower |
| Halstead Effort | 39807.6436 | **32981.2617 ✅** | 46428.1304 | +6826.3819 | Higher | -6620.4868 | Lower | -13446.8687 | Lower |
| SLOC | 64 | **56 ✅** | 67 | +8 | Higher | -3 | Lower | -11 | Lower |
| PLOC | 50 | **33 ✅** | 55 | +17 | Higher | -5 | Lower | -22 | Lower |
| LLOC | 40 | **23 ✅** | 42 | +17 | Higher | -2 | Lower | -19 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 4 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00749

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 18.4006 | **34.9603 ✅** | 21.7884 | -16.5597 | Lower | -3.3878 | Lower | +13.1719 | Higher |
| MI (SEI) | -25.4242 | **7.3962 ✅** | -27.3136 | -32.8203 | Lower | +1.8894 | Higher | +34.7097 | Higher |
| Cyclomatic (sum) | 64 | **43 ✅** | 52 | +21 | Higher | +12 | Higher | -9 | Lower |
| Cognitive (sum) | 79 | 85 | **67 ✅** | -6 | Lower | +12 | Higher | +18 | Higher |
| Halstead Volume | 8527.4467 | **4162.0325 ✅** | 8698.2690 | +4365.4142 | Higher | -170.8223 | Lower | -4536.2365 | Lower |
| Halstead Effort | 867831.6949 | **264719.6206 ✅** | 738292.0995 | +603112.0743 | Higher | +129539.5954 | Higher | -473572.4789 | Lower |
| SLOC | 272 | **166 ✅** | 260 | +106 | Higher | +12 | Higher | -94 | Lower |
| PLOC | 234 | **122 ✅** | 225 | +112 | Higher | +9 | Higher | -103 | Lower |
| LLOC | 160 | **87 ✅** | 163 | +73 | Higher | -3 | Lower | -76 | Lower |
| Comments (CLOC) | 14 | **25 ✅** | 6 | -11 | Lower | +8 | Higher | +19 | Higher |
| Blank Lines | 28 | **19 ✅** | 31 | +9 | Higher | -3 | Lower | -12 | Lower |
| Functions | 6 | **1 ✅** | 7 | +5 | Higher | -1 | Lower | -6 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 15 | **2 ✅** | 12 | +13 | Higher | +3 | Higher | -10 | Lower |

## p00762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **35.7568 ✅** | 25.6663 | 34.9611 | +10.0905 | Higher | +0.7957 | Higher | -9.2948 | Lower |
| MI (SEI) | 0.5301 | -1.9213 | **1.7260 ✅** | +2.4514 | Higher | -1.1959 | Lower | -3.6473 | Lower |
| Cyclomatic (sum) | **32 ✅** | 39 | 34 | -7 | Lower | -2 | Lower | +5 | Higher |
| Cognitive (sum) | **19 ✅** | 50 | 20 | -31 | Lower | -1 | Lower | +30 | Higher |
| Halstead Volume | **5200.4775 ✅** | 6019.8525 | 5753.0857 | -819.3749 | Lower | -552.6081 | Lower | +266.7668 | Higher |
| Halstead Effort | **389942.9498 ✅** | 431092.7237 | 453898.6220 | -41149.7739 | Lower | -63955.6722 | Lower | -22805.8983 | Lower |
| SLOC | 172 | 277 | **170 ✅** | -105 | Lower | +2 | Higher | +107 | Higher |
| PLOC | 141 | 190 | **137 ✅** | -49 | Lower | +4 | Higher | +53 | Higher |
| LLOC | **98 ✅** | 139 | 103 | -41 | Lower | -5 | Lower | +36 | Higher |
| Comments (CLOC) | 14 | **59 ✅** | 17 | -45 | Lower | -3 | Lower | +42 | Higher |
| Blank Lines | 24 | 31 | 24 | -7 | Lower | 0 | Same | +7 | Higher |
| Functions | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | 9 | **5 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

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

## p00799

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.0231 | **46.8069 ✅** | 41.6695 | -1.7838 | Lower | +3.3535 | Higher | +5.1374 | Higher |
| MI (SEI) | 17.7045 | **26.6283 ✅** | 9.9565 | -8.9238 | Lower | +7.7480 | Higher | +16.6718 | Higher |
| Cyclomatic (sum) | **22 ✅** | 27 | 29 | -5 | Lower | -7 | Lower | -2 | Lower |
| Cognitive (sum) | 46 | **20 ✅** | 53 | +26 | Higher | -7 | Lower | -33 | Lower |
| Halstead Volume | 2964.9177 | **2790.5108 ✅** | 3460.7756 | +174.4069 | Higher | -495.8579 | Lower | -670.2649 | Lower |
| Halstead Effort | 133358.2130 | **123019.9643 ✅** | 194703.2370 | +10338.2487 | Higher | -61345.0240 | Lower | -71683.2727 | Lower |
| SLOC | 134 | **114 ✅** | 142 | +20 | Higher | -8 | Lower | -28 | Lower |
| PLOC | 105 | **73 ✅** | 119 | +32 | Higher | -14 | Lower | -46 | Lower |
| LLOC | 78 | **49 ✅** | 87 | +29 | Higher | -9 | Lower | -38 | Lower |
| Comments (CLOC) | 17 | **23 ✅** | 13 | -6 | Lower | +4 | Higher | +10 | Higher |
| Blank Lines | 20 | 18 | **14 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

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

## p00816

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.4949 | 37.2342 | **40.5406 ✅** | +3.2607 | Higher | -0.0457 | Lower | -3.3064 | Lower |
| MI (SEI) | **17.7323 ✅** | 6.6022 | 9.2690 | +11.1302 | Higher | +8.4633 | Higher | -2.6668 | Lower |
| Cyclomatic (sum) | 32 | 39 | 32 | -7 | Lower | 0 | Same | +7 | Higher |
| Cognitive (sum) | 44 | 51 | **42 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| Halstead Volume | **3410.5799 ✅** | 4052.7523 | 3605.0239 | -642.1724 | Lower | -194.4440 | Lower | +447.7284 | Higher |
| Halstead Effort | **193379.8800 ✅** | 237865.3836 | 202241.8390 | -44485.5035 | Lower | -8861.9590 | Lower | +35623.5446 | Higher |
| SLOC | 147 | 154 | **144 ✅** | -7 | Lower | +3 | Higher | +10 | Higher |
| PLOC | **106 ✅** | 123 | 114 | -17 | Lower | -8 | Lower | +9 | Higher |
| LLOC | 83 | **82 ✅** | 86 | +1 | Higher | -3 | Lower | -4 | Lower |
| Comments (CLOC) | **29 ✅** | 17 | 14 | +12 | Higher | +15 | Higher | +3 | Higher |
| Blank Lines | **12 ✅** | 14 | 16 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p00837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.8579 | **50.9665 ✅** | 40.0828 | -12.1085 | Lower | -1.2249 | Lower | +10.8837 | Higher |
| MI (SEI) | 11.2479 | **28.1986 ✅** | 7.2216 | -16.9507 | Lower | +4.0262 | Higher | +20.9769 | Higher |
| Cyclomatic (sum) | 23 | **21 ✅** | 24 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 34 | **21 ✅** | 31 | +13 | Higher | +3 | Higher | -10 | Lower |
| Halstead Volume | 4505.3460 | **2459.3350 ✅** | 4738.7374 | +2046.0111 | Higher | -233.3914 | Lower | -2279.4024 | Lower |
| Halstead Effort | 300636.3965 | **117925.1127 ✅** | 289628.8021 | +182711.2838 | Higher | +11007.5945 | Higher | -171703.6894 | Lower |
| SLOC | 169 | **100 ✅** | 152 | +69 | Higher | +17 | Higher | -52 | Lower |
| PLOC | 119 | **73 ✅** | 118 | +46 | Higher | +1 | Higher | -45 | Lower |
| LLOC | 111 | **44 ✅** | 110 | +67 | Higher | +1 | Higher | -66 | Lower |
| Comments (CLOC) | **26 ✅** | 15 | 14 | +11 | Higher | +12 | Higher | +1 | Higher |
| Blank Lines | 25 | **12 ✅** | 21 | +13 | Higher | +4 | Higher | -9 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

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

## p00855

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.0377 | 48.1882 | **66.5347 ✅** | +17.8495 | Higher | -0.4969 | Lower | -18.3465 | Lower |
| MI (SEI) | 38.5437 | 21.6496 | **45.6903 ✅** | +16.8940 | Higher | -7.1466 | Lower | -24.0406 | Lower |
| Cyclomatic (sum) | 14 | 27 | **11 ✅** | -13 | Lower | +3 | Higher | +16 | Higher |
| Cognitive (sum) | 15 | 35 | **12 ✅** | -20 | Lower | +3 | Higher | +23 | Higher |
| Halstead Volume | **1124.7324 ✅** | 2199.0688 | 1167.2783 | -1074.3365 | Lower | -42.5460 | Lower | +1031.7905 | Higher |
| Halstead Effort | 35691.5066 | 96130.7220 | **33888.7260 ✅** | -60439.2154 | Lower | +1802.7806 | Higher | +62241.9960 | Higher |
| SLOC | 56 | 113 | 56 | -57 | Lower | 0 | Same | +57 | Higher |
| PLOC | 45 | 88 | **41 ✅** | -43 | Lower | +4 | Higher | +47 | Higher |
| LLOC | 30 | 50 | **29 ✅** | -20 | Lower | +1 | Higher | +21 | Higher |
| Comments (CLOC) | 3 | **13 ✅** | 6 | -10 | Lower | -3 | Lower | +7 | Higher |
| Blank Lines | 9 | 12 | 9 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 12 | **1 ✅** | -7 | Lower | +4 | Higher | +11 | Higher |

## p00856

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.9207 | 39.3130 | **50.3337 ✅** | -0.3923 | Lower | -11.4130 | Lower | -11.0208 | Lower |
| MI (SEI) | 4.4760 | 7.4471 | **19.9762 ✅** | -2.9712 | Lower | -15.5002 | Lower | -12.5290 | Lower |
| Cyclomatic (sum) | 42 | 40 | **24 ✅** | +2 | Higher | +18 | Higher | +16 | Higher |
| Cognitive (sum) | 62 | 60 | **22 ✅** | +2 | Higher | +40 | Higher | +38 | Higher |
| Halstead Volume | 4247.3736 | 4009.9892 | **3049.4679 ✅** | +237.3844 | Higher | +1197.9057 | Higher | +960.5214 | Higher |
| Halstead Effort | 324265.0033 | 269652.8591 | **180299.7867 ✅** | +54612.1442 | Higher | +143965.2166 | Higher | +89353.0724 | Higher |
| SLOC | 131 | 134 | **93 ✅** | -3 | Lower | +38 | Higher | +41 | Higher |
| PLOC | 106 | 107 | **72 ✅** | -1 | Lower | +34 | Higher | +35 | Higher |
| LLOC | 64 | 74 | **44 ✅** | -10 | Lower | +20 | Higher | +30 | Higher |
| Comments (CLOC) | 9 | **12 ✅** | 7 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 16 | 15 | **14 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | **2 ✅** | 5 | 4 | -3 | Lower | -2 | Lower | +1 | Higher |

## p00866

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.8744 | 40.1579 | **46.1685 ✅** | +0.7165 | Higher | -5.2941 | Lower | -6.0106 | Lower |
| MI (SEI) | 13.4208 | **15.0774 ✅** | 10.0562 | -1.6566 | Lower | +3.3647 | Higher | +5.0212 | Higher |
| Cyclomatic (sum) | 30 | 36 | **20 ✅** | -6 | Lower | +10 | Higher | +16 | Higher |
| Cognitive (sum) | 33 | 37 | **29 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| Halstead Volume | 4123.4598 | 3884.8030 | **3861.9487 ✅** | +238.6568 | Higher | +261.5111 | Higher | +22.8543 | Higher |
| Halstead Effort | 220574.3282 | 220948.1695 | **204113.4842 ✅** | -373.8413 | Lower | +16460.8439 | Higher | +16834.6852 | Higher |
| SLOC | 139 | 136 | **118 ✅** | +3 | Higher | +21 | Higher | +18 | Higher |
| PLOC | 106 | 98 | **97 ✅** | +8 | Higher | +9 | Higher | +1 | Higher |
| LLOC | **59 ✅** | 71 | 62 | -12 | Lower | -3 | Lower | +9 | Higher |
| Comments (CLOC) | 19 | **22 ✅** | 6 | -3 | Lower | +13 | Higher | +16 | Higher |
| Blank Lines | 15 | 17 | 15 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | 5 | **1 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Exits | 3 | 8 | **0 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |

## p00867

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 10.1666 | **65.8458 ✅** | 6.2705 | -55.6793 | Lower | +3.8961 | Higher | +59.5754 | Higher |
| MI (SEI) | -42.1770 | **59.8853 ✅** | -44.4184 | -102.0623 | Lower | +2.2414 | Higher | +104.3037 | Higher |
| Cyclomatic (sum) | 81 | **9 ✅** | 90 | +72 | Higher | -9 | Lower | -81 | Lower |
| Cognitive (sum) | 92 | **6 ✅** | 95 | +86 | Higher | -3 | Lower | -89 | Lower |
| Halstead Volume | 12398.6264 | **1174.2906 ✅** | 13510.3625 | +11224.3358 | Higher | -1111.7361 | Lower | -12336.0720 | Lower |
| Halstead Effort | 1427641.8394 | **25403.8196 ✅** | 1538375.4881 | +1402238.0198 | Higher | -110733.6488 | Lower | -1512971.6685 | Lower |
| SLOC | 315 | **60 ✅** | 343 | +255 | Higher | -28 | Lower | -283 | Lower |
| PLOC | 279 | **31 ✅** | 305 | +248 | Higher | -26 | Lower | -274 | Lower |
| LLOC | 179 | **20 ✅** | 197 | +159 | Higher | -18 | Lower | -177 | Lower |
| Comments (CLOC) | 6 | **21 ✅** | 10 | -15 | Lower | -4 | Lower | +11 | Higher |
| Blank Lines | 33 | **8 ✅** | 31 | +25 | Higher | +2 | Higher | -23 | Lower |
| Functions | 13 | **1 ✅** | 15 | +12 | Higher | -2 | Lower | -14 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 14 | **0 ✅** | 16 | +14 | Higher | -2 | Lower | -16 | Lower |

## p00870

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.0750 | **58.8103 ✅** | 49.8399 | -3.7353 | Lower | +5.2351 | Higher | +8.9703 | Higher |
| MI (SEI) | 29.0830 | **39.7104 ✅** | 29.6696 | -10.6274 | Lower | -0.5866 | Lower | +10.0408 | Higher |
| Cyclomatic (sum) | 19 | **15 ✅** | 23 | +4 | Higher | -4 | Lower | -8 | Lower |
| Cognitive (sum) | 27 | **21 ✅** | 39 | +6 | Higher | -12 | Lower | -18 | Lower |
| Halstead Volume | 2350.7404 | **1738.6253 ✅** | 2710.3426 | +612.1151 | Higher | -359.6022 | Lower | -971.7173 | Lower |
| Halstead Effort | 114637.7750 | **63037.5867 ✅** | 133285.0825 | +51600.1883 | Higher | -18647.3076 | Lower | -70247.4958 | Lower |
| SLOC | 81 | **75 ✅** | 101 | +6 | Higher | -20 | Lower | -26 | Lower |
| PLOC | 61 | **51 ✅** | 73 | +10 | Higher | -12 | Lower | -22 | Lower |
| LLOC | 45 | **39 ✅** | 50 | +6 | Higher | -5 | Lower | -11 | Lower |
| Comments (CLOC) | 8 | 12 | **19 ✅** | -4 | Lower | -11 | Lower | -7 | Lower |
| Blank Lines | 12 | 12 | 12 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

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

## p00889

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.2291 | **48.5458 ✅** | 42.0173 | -9.3166 | Lower | -2.7881 | Lower | +6.5285 | Higher |
| MI (SEI) | 0.0634 | **27.5717 ✅** | 8.4992 | -27.5083 | Lower | -8.4357 | Lower | +19.0725 | Higher |
| Cyclomatic (sum) | 29 | **20 ✅** | 25 | +9 | Higher | +4 | Higher | -5 | Lower |
| Cognitive (sum) | 31 | **12 ✅** | 30 | +19 | Higher | +1 | Higher | -18 | Lower |
| Halstead Volume | 4297.6067 | **2958.0183 ✅** | 4037.9163 | +1339.5885 | Higher | +259.6905 | Higher | -1079.8980 | Lower |
| Halstead Effort | 306588.1958 | **144469.6136 ✅** | 260782.0937 | +162118.5822 | Higher | +45806.1021 | Higher | -116312.4801 | Lower |
| SLOC | 154 | **111 ✅** | 140 | +43 | Higher | +14 | Higher | -29 | Lower |
| PLOC | 127 | **77 ✅** | 111 | +50 | Higher | +16 | Higher | -34 | Lower |
| LLOC | 103 | **55 ✅** | 98 | +48 | Higher | +5 | Higher | -43 | Lower |
| Comments (CLOC) | 7 | **21 ✅** | 11 | -14 | Lower | -4 | Lower | +10 | Higher |
| Blank Lines | 20 | **14 ✅** | 18 | +6 | Higher | +2 | Higher | -4 | Lower |
| Functions | 5 | 4 | **3 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **2 ✅** | 5 | +4 | Higher | +1 | Higher | -3 | Lower |

## p00897

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.0379 | **43.7246 ✅** | 22.1784 | -7.6867 | Lower | +13.8596 | Higher | +21.5463 | Higher |
| MI (SEI) | -0.8221 | **22.0396 ✅** | -20.5387 | -22.8617 | Lower | +19.7166 | Higher | +42.5783 | Higher |
| Cyclomatic (sum) | 33 | **20 ✅** | 47 | +13 | Higher | -14 | Lower | -27 | Lower |
| Cognitive (sum) | **31 ✅** | 34 | 60 | -3 | Lower | -29 | Lower | -26 | Lower |
| Halstead Volume | 5467.5588 | **4256.0470 ✅** | 8848 | +1211.5119 | Higher | -3380.4412 | Lower | -4591.9530 | Lower |
| Halstead Effort | 335446.2842 | **222378.4531 ✅** | 683272.6809 | +113067.8310 | Higher | -347826.3967 | Lower | -460894.2277 | Lower |
| SLOC | 164 | **133 ✅** | 271 | +31 | Higher | -107 | Lower | -138 | Lower |
| PLOC | 137 | **90 ✅** | 228 | +47 | Higher | -91 | Lower | -138 | Lower |
| LLOC | 95 | **67 ✅** | 164 | +28 | Higher | -69 | Lower | -97 | Lower |
| Comments (CLOC) | 11 | **28 ✅** | 16 | -17 | Lower | -5 | Lower | +12 | Higher |
| Blank Lines | 17 | **16 ✅** | 27 | +1 | Higher | -10 | Lower | -11 | Lower |
| Functions | 6 | **1 ✅** | 6 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 7 | **2 ✅** | 6 | +5 | Higher | +1 | Higher | -4 | Lower |

## p00907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.4200 ✅** | 46.3983 | 56.5504 | +12.0217 | Higher | +1.8697 | Higher | -10.1521 | Lower |
| MI (SEI) | **35.4628 ✅** | 26.7080 | 33.0345 | +8.7548 | Higher | +2.4283 | Higher | -6.3265 | Lower |
| Cyclomatic (sum) | 20 | 26 | 20 | -6 | Lower | 0 | Same | +6 | Higher |
| Cognitive (sum) | 28 | 28 | **25 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | **1782 ✅** | 2909.8635 | 1904.7520 | -1127.8635 | Lower | -122.7520 | Lower | +1005.1115 | Higher |
| Halstead Effort | 87048 | 137916.5486 | **84941.6435 ✅** | -50868.5486 | Lower | +2106.3565 | Higher | +52974.9051 | Higher |
| SLOC | **71 ✅** | 117 | 78 | -46 | Lower | -7 | Lower | +39 | Higher |
| PLOC | **55 ✅** | 83 | 64 | -28 | Lower | -9 | Lower | +19 | Higher |
| LLOC | **40 ✅** | 47 | 47 | -7 | Lower | -7 | Lower | 0 | Same |
| Comments (CLOC) | 8 | **25 ✅** | 9 | -17 | Lower | -1 | Lower | +16 | Higher |
| Blank Lines | 9 | 13 | 9 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 8 | 1 | -7 | Lower | 0 | Same | +7 | Higher |

## p00924

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.6469 ✅** | 46.1846 | 51.1395 | +8.4622 | Higher | +3.5073 | Higher | -4.9549 | Lower |
| MI (SEI) | 21.9478 | **26.7206 ✅** | 17.9687 | -4.7728 | Lower | +3.9791 | Higher | +8.7519 | Higher |
| Cyclomatic (sum) | **17 ✅** | 33 | 20 | -16 | Lower | -3 | Lower | +13 | Higher |
| Cognitive (sum) | **21 ✅** | 37 | 22 | -16 | Lower | -1 | Lower | +15 | Higher |
| Halstead Volume | **3014.4446 ✅** | 3025.8148 | 3452.4561 | -11.3702 | Lower | -438.0115 | Lower | -426.6413 | Lower |
| Halstead Effort | **150394.5710 ✅** | 167900.5296 | 215816.8651 | -17505.9586 | Lower | -65422.2941 | Lower | -47916.3355 | Lower |
| SLOC | **79 ✅** | 106 | 90 | -27 | Lower | -11 | Lower | +16 | Higher |
| PLOC | **64 ✅** | 76 | 78 | -12 | Lower | -14 | Lower | -2 | Lower |
| LLOC | **52 ✅** | 53 | 61 | -1 | Lower | -9 | Lower | -8 | Lower |
| Comments (CLOC) | 4 | **22 ✅** | 5 | -18 | Lower | -1 | Lower | +17 | Higher |
| Blank Lines | 11 | 10 | **7 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | **1 ✅** | 5 | 2 | -4 | Lower | -1 | Lower | +3 | Higher |
| Exits | 5 | **3 ✅** | 9 | +2 | Higher | -4 | Lower | -6 | Lower |

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

## p01065

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.3973 | **64.8507 ✅** | 49.4754 | -10.4534 | Lower | +4.9220 | Higher | +15.3753 | Higher |
| MI (SEI) | 32.2431 | **41.8828 ✅** | 22.4920 | -9.6397 | Lower | +9.7512 | Higher | +19.3909 | Higher |
| Cyclomatic (sum) | 18 | **11 ✅** | 22 | +7 | Higher | -4 | Lower | -11 | Lower |
| Cognitive (sum) | 42 | **19 ✅** | 52 | +23 | Higher | -10 | Lower | -33 | Lower |
| Halstead Volume | 2499.2287 | **1915.6282 ✅** | 3134.3219 | +583.6006 | Higher | -635.0932 | Lower | -1218.6937 | Lower |
| Halstead Effort | 108597.4396 | **65234.9049 ✅** | 194470.4270 | +43362.5347 | Higher | -85872.9874 | Lower | -129235.5221 | Lower |
| SLOC | 84 | **53 ✅** | 100 | +31 | Higher | -16 | Lower | -47 | Lower |
| PLOC | 59 | **42 ✅** | 77 | +17 | Higher | -18 | Lower | -35 | Lower |
| LLOC | 46 | **33 ✅** | 61 | +13 | Higher | -15 | Lower | -28 | Lower |
| Comments (CLOC) | **12 ✅** | 5 | 11 | +7 | Higher | +1 | Higher | -6 | Lower |
| Blank Lines | 13 | **6 ✅** | 12 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p01106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.3520 | 60.7484 | **61.3771 ✅** | -1.3965 | Lower | -2.0251 | Lower | -0.6287 | Lower |
| MI (SEI) | 37.0944 | **47.9090 ✅** | 43.8294 | -10.8146 | Lower | -6.7350 | Lower | +4.0796 | Higher |
| Cyclomatic (sum) | 18 | 18 | **14 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Cognitive (sum) | 14 | 14 | **13 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 1861.6734 | **1423.2252 ✅** | 1651.9506 | +438.4482 | Higher | +209.7227 | Higher | -228.7255 | Lower |
| Halstead Effort | 64134.6482 | **37125.8450 ✅** | 53192.8106 | +27008.8033 | Higher | +10941.8376 | Higher | -16066.9656 | Lower |
| SLOC | 68 | 68 | **66 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| PLOC | 52 | **44 ✅** | 47 | +8 | Higher | +5 | Higher | -3 | Lower |
| LLOC | 36 | **20 ✅** | 37 | +16 | Higher | -1 | Lower | -17 | Lower |
| Comments (CLOC) | 8 | **16 ✅** | 11 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 10 | **8 ✅** | 9 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01119

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.7116 | **53.7328 ✅** | 27.0487 | -22.0212 | Lower | +4.6630 | Higher | +26.6842 | Higher |
| MI (SEI) | -6.6880 | **34.3007 ✅** | -16.2088 | -40.9887 | Lower | +9.5208 | Higher | +50.5095 | Higher |
| Cyclomatic (sum) | 43 | **23 ✅** | 50 | +20 | Higher | -7 | Lower | -27 | Lower |
| Cognitive (sum) | 62 | **29 ✅** | 63 | +33 | Higher | -1 | Lower | -34 | Lower |
| Halstead Volume | 4940.3002 | **2115.5379 ✅** | 5733.3671 | +2824.7623 | Higher | -793.0669 | Lower | -3617.8291 | Lower |
| Halstead Effort | 285831.6548 | **83482.3819 ✅** | 363495.4727 | +202349.2729 | Higher | -77663.8180 | Lower | -280013.0909 | Lower |
| SLOC | 192 | **86 ✅** | 221 | +106 | Higher | -29 | Lower | -135 | Lower |
| PLOC | 162 | **64 ✅** | 191 | +98 | Higher | -29 | Lower | -127 | Lower |
| LLOC | 129 | **41 ✅** | 148 | +88 | Higher | -19 | Lower | -107 | Lower |
| Comments (CLOC) | 12 | **15 ✅** | 9 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | 18 | **8 ✅** | 22 | +10 | Higher | -4 | Lower | -14 | Lower |
| Functions | 6 | **1 ✅** | 9 | +5 | Higher | -3 | Lower | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 12 | **0 ✅** | 15 | +12 | Higher | -3 | Lower | -15 | Lower |

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

## p01150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.4953 | **70.3774 ✅** | 31.4191 | -35.8821 | Lower | +3.0762 | Higher | +38.9583 | Higher |
| MI (SEI) | 2.1603 | **71.2185 ✅** | -8.7234 | -69.0582 | Lower | +10.8837 | Higher | +79.9419 | Higher |
| Cyclomatic (sum) | 40 | **7 ✅** | 45 | +33 | Higher | -5 | Lower | -38 | Lower |
| Cognitive (sum) | 44 | **8 ✅** | 49 | +36 | Higher | -5 | Lower | -41 | Lower |
| Halstead Volume | 4652.9276 | **629.6861 ✅** | 6056.2875 | +4023.2415 | Higher | -1403.3599 | Lower | -5426.6014 | Lower |
| Halstead Effort | 448805.2085 | **9445.2909 ✅** | 641052.3141 | +439359.9176 | Higher | -192247.1056 | Lower | -631607.0232 | Lower |
| SLOC | 172 | **57 ✅** | 178 | +115 | Higher | -6 | Lower | -121 | Lower |
| PLOC | 124 | **23 ✅** | 145 | +101 | Higher | -21 | Lower | -122 | Lower |
| LLOC | 85 | **12 ✅** | 126 | +73 | Higher | -41 | Lower | -114 | Lower |
| Comments (CLOC) | 18 | **29 ✅** | 9 | -11 | Lower | +9 | Higher | +20 | Higher |
| Blank Lines | 30 | **5 ✅** | 28 | +25 | Higher | +2 | Higher | -23 | Lower |
| Functions | 6 | **1 ✅** | 7 | +5 | Higher | -1 | Lower | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **0 ✅** | 7 | +7 | Higher | 0 | Same | -7 | Lower |

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

## p01184

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **30.3139 ✅** | 19.9827 | 29.8214 | +10.3312 | Higher | +0.4925 | Higher | -9.8387 | Lower |
| MI (SEI) | **-6.8695 ✅** | -10.3271 | -10.7997 | +3.4576 | Higher | +3.9302 | Higher | +0.4726 | Higher |
| Cyclomatic (sum) | **41 ✅** | 59 | 42 | -18 | Lower | -1 | Lower | +17 | Higher |
| Cognitive (sum) | 51 | 91 | 51 | -40 | Lower | 0 | Same | +40 | Higher |
| Halstead Volume | **5671.2979 ✅** | 7331.6924 | 6148.9764 | -1660.3945 | Lower | -477.6786 | Lower | +1182.7160 | Higher |
| Halstead Effort | **442398.0607 ✅** | 683509.2420 | 507889.4813 | -241111.1814 | Lower | -65491.4206 | Lower | +175619.7608 | Higher |
| SLOC | 206 | 278 | **204 ✅** | -72 | Lower | +2 | Higher | +74 | Higher |
| PLOC | **171 ✅** | 205 | 172 | -34 | Lower | -1 | Lower | +33 | Higher |
| LLOC | 124 | **122 ✅** | 132 | +2 | Higher | -8 | Lower | -10 | Lower |
| Comments (CLOC) | 16 | **50 ✅** | 11 | -34 | Lower | +5 | Higher | +39 | Higher |
| Blank Lines | 22 | 25 | 22 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |

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

## p01275

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.1667 | **62.9256 ✅** | 49.7210 | -9.7589 | Lower | +3.4457 | Higher | +13.2046 | Higher |
| MI (SEI) | 20.4013 | **44.5389 ✅** | 22.6563 | -24.1376 | Lower | -2.2550 | Lower | +21.8826 | Higher |
| Cyclomatic (sum) | 17 | **16 ✅** | 18 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 14 | **13 ✅** | 16 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 2492.9231 | **1364.0696 ✅** | 2890.1719 | +1128.8535 | Higher | -397.2488 | Lower | -1526.1023 | Lower |
| Halstead Effort | 117638.2701 | **45716.9989 ✅** | 142121.0603 | +71921.2712 | Higher | -24482.7902 | Lower | -96404.0614 | Lower |
| SLOC | 92 | **62 ✅** | 107 | +30 | Higher | -15 | Lower | -45 | Lower |
| PLOC | 75 | **45 ✅** | 80 | +30 | Higher | -5 | Lower | -35 | Lower |
| LLOC | 52 | **28 ✅** | 63 | +24 | Higher | -11 | Lower | -35 | Lower |
| Comments (CLOC) | 5 | 9 | **12 ✅** | -4 | Lower | -7 | Lower | -3 | Lower |
| Blank Lines | 14 | **9 ✅** | 15 | +5 | Higher | -1 | Lower | -6 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |

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

## p01316

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.7286 | **57.1155 ✅** | 56.9558 | -1.3870 | Lower | -1.2273 | Lower | +0.1597 | Higher |
| MI (SEI) | 31.3206 | **39.8951 ✅** | 33.0676 | -8.5745 | Lower | -1.7470 | Lower | +6.8275 | Higher |
| Cyclomatic (sum) | 19 | **14 ✅** | 16 | +5 | Higher | +3 | Higher | -2 | Lower |
| Cognitive (sum) | 30 | **15 ✅** | 29 | +15 | Higher | +1 | Higher | -14 | Lower |
| Halstead Volume | 2073.0948 | **1980.7439 ✅** | 2021.0377 | +92.3509 | Higher | +52.0571 | Higher | -40.2938 | Lower |
| Halstead Effort | 77175.6655 | **64088.0680 ✅** | 68738.7810 | +13087.5976 | Higher | +8436.8845 | Higher | -4650.7130 | Lower |
| SLOC | 81 | 81 | **79 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| PLOC | 62 | **52 ✅** | 58 | +10 | Higher | +4 | Higher | -6 | Lower |
| LLOC | 48 | **39 ✅** | 48 | +9 | Higher | 0 | Same | -9 | Lower |
| Comments (CLOC) | 9 | **16 ✅** | 9 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **46.8327 ✅** | 41.4343 | 46.6718 | +5.3985 | Higher | +0.1609 | Higher | -5.2376 | Lower |
| MI (SEI) | **19.8336 ✅** | 10.2753 | 12.9392 | +9.5584 | Higher | +6.8945 | Higher | -2.6639 | Lower |
| Cyclomatic (sum) | **21 ✅** | 27 | 23 | -6 | Lower | -2 | Lower | +4 | Higher |
| Cognitive (sum) | **42 ✅** | 45 | 47 | -3 | Lower | -5 | Lower | -2 | Lower |
| Halstead Volume | **3620.5921 ✅** | 4043.9075 | 3714.3455 | -423.3155 | Lower | -93.7534 | Lower | +329.5620 | Higher |
| Halstead Effort | 200037.7123 | **192525.1632 ✅** | 206304.2330 | +7512.5490 | Higher | -6266.5207 | Lower | -13779.0697 | Lower |
| SLOC | 114 | 141 | **111 ✅** | -27 | Lower | +3 | Higher | +30 | Higher |
| PLOC | **87 ✅** | 105 | 92 | -18 | Lower | -5 | Lower | +13 | Higher |
| LLOC | 74 | **72 ✅** | 76 | +2 | Higher | -2 | Lower | -4 | Lower |
| Comments (CLOC) | 14 | 14 | 7 | 0 | Same | +7 | Higher | +7 | Higher |
| Blank Lines | 14 | 22 | **12 ✅** | -8 | Lower | +2 | Higher | +10 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01339

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.3989 | **43.5309 ✅** | 42.0267 | -0.1320 | Lower | +1.3722 | Higher | +1.5042 | Higher |
| MI (SEI) | 11.0347 | **26.2068 ✅** | 11.7663 | -15.1721 | Lower | -0.7317 | Lower | +14.4405 | Higher |
| Cyclomatic (sum) | 27 | **26 ✅** | 27 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 42 | **40 ✅** | 42 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 4241.3597 | **3387.9282 ✅** | 4328.6518 | +853.4314 | Higher | -87.2921 | Lower | -940.7235 | Lower |
| Halstead Effort | 283844.8382 | **195104.8083 ✅** | 264441.2720 | +88740.0299 | Higher | +19403.5662 | Higher | -69336.4637 | Lower |
| SLOC | **123 ✅** | 133 | 133 | -10 | Lower | -10 | Lower | 0 | Same |
| PLOC | 96 | **82 ✅** | 102 | +14 | Higher | -6 | Lower | -20 | Lower |
| LLOC | 77 | **62 ✅** | 76 | +15 | Higher | +1 | Higher | -14 | Lower |
| Comments (CLOC) | 10 | **37 ✅** | 14 | -27 | Lower | -4 | Lower | +23 | Higher |
| Blank Lines | 17 | **15 ✅** | 17 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p01368

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.9717 ✅** | 37.6784 | 36.2913 | +2.2933 | Higher | +3.6803 | Higher | +1.3870 | Higher |
| MI (SEI) | **5.8198 ✅** | 5.6998 | 2.9004 | +0.1200 | Higher | +2.9194 | Higher | +2.7994 | Higher |
| Cyclomatic (sum) | **34 ✅** | 46 | 36 | -12 | Lower | -2 | Lower | +10 | Higher |
| Cognitive (sum) | 55 | **47 ✅** | 53 | +8 | Higher | +2 | Higher | -6 | Lower |
| Halstead Volume | 4398.8729 | **4211.2183 ✅** | 4560.3587 | +187.6547 | Higher | -161.4858 | Lower | -349.1404 | Lower |
| Halstead Effort | 361662.0529 | **204989.1480 ✅** | 345284.3033 | +156672.9049 | Higher | +16377.7496 | Higher | -140295.1553 | Lower |
| SLOC | 136 | **134 ✅** | 164 | +2 | Higher | -28 | Lower | -30 | Lower |
| PLOC | 113 | **109 ✅** | 137 | +4 | Higher | -24 | Lower | -28 | Lower |
| LLOC | 84 | **62 ✅** | 102 | +22 | Higher | -18 | Lower | -40 | Lower |
| Comments (CLOC) | 10 | 12 | **15 ✅** | -2 | Lower | -5 | Lower | -3 | Lower |
| Blank Lines | 14 | 13 | 13 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 8 | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Exits | 4 | **0 ✅** | 5 | +4 | Higher | -1 | Lower | -5 | Lower |

## p01452

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.1930 | 40.0899 | **54.1699 ✅** | +4.1032 | Higher | -9.9769 | Lower | -14.0800 | Lower |
| MI (SEI) | 8.0907 | 11.0948 | **20.3503 ✅** | -3.0041 | Lower | -12.2596 | Lower | -9.2555 | Lower |
| Cyclomatic (sum) | 28 | 32 | **12 ✅** | -4 | Lower | +16 | Higher | +20 | Higher |
| Cognitive (sum) | 16 | 17 | **6 ✅** | -1 | Lower | +10 | Higher | +11 | Higher |
| Halstead Volume | 4070.3919 | 4292.0785 | **3533.9578 ✅** | -221.6866 | Lower | +536.4341 | Higher | +758.1206 | Higher |
| Halstead Effort | 260919.0192 | 331305.5357 | **182732.2899 ✅** | -70386.5165 | Lower | +78186.7293 | Higher | +148573.2458 | Higher |
| SLOC | 117 | 140 | **83 ✅** | -23 | Lower | +34 | Higher | +57 | Higher |
| PLOC | 94 | 109 | **64 ✅** | -15 | Lower | +30 | Higher | +45 | Higher |
| LLOC | 58 | 72 | **52 ✅** | -14 | Lower | +6 | Higher | +20 | Higher |
| Comments (CLOC) | 6 | **17 ✅** | 4 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | 17 | **14 ✅** | 15 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 7 | 5 | 5 | +2 | Higher | +2 | Higher | 0 | Same |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 12 | 13 | **4 ✅** | -1 | Lower | +8 | Higher | +9 | Higher |

## p01459

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 29.9234 | 25.4206 | **32.3492 ✅** | +4.5028 | Higher | -2.4257 | Lower | -6.9286 | Lower |
| MI (SEI) | **-5.5552 ✅** | -8.2449 | -8.3197 | +2.6897 | Higher | +2.7645 | Higher | +0.0748 | Higher |
| Cyclomatic (sum) | 39 | 55 | **38 ✅** | -16 | Lower | +1 | Higher | +17 | Higher |
| Cognitive (sum) | **69 ✅** | 70 | 71 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 6290.6056 | **6026.8715 ✅** | 6331.5335 | +263.7341 | Higher | -40.9279 | Lower | -304.6620 | Lower |
| Halstead Effort | 463804.6526 | **417930.9599 ✅** | 482931.2145 | +45873.6927 | Higher | -19126.5619 | Lower | -65000.2546 | Lower |
| SLOC | 210 | 224 | **183 ✅** | -14 | Lower | +27 | Higher | +41 | Higher |
| PLOC | 168 | 178 | **151 ✅** | -10 | Lower | +17 | Higher | +27 | Higher |
| LLOC | 109 | **105 ✅** | 115 | +4 | Higher | -6 | Lower | -10 | Lower |
| Comments (CLOC) | 20 | **26 ✅** | 9 | -6 | Lower | +11 | Higher | +17 | Higher |
| Blank Lines | 23 | **20 ✅** | 23 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 20 | 2 | -18 | Lower | 0 | Same | +18 | Higher |

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

## p01529

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.9682 | **61.1588 ✅** | 48.1177 | -3.1906 | Lower | +9.8505 | Higher | +13.0411 | Higher |
| MI (SEI) | 36.1548 | **46.5807 ✅** | 21.7013 | -10.4259 | Lower | +14.4534 | Higher | +24.8794 | Higher |
| Cyclomatic (sum) | 14 | **13 ✅** | 26 | +1 | Higher | -12 | Lower | -13 | Lower |
| Cognitive (sum) | 28 | **17 ✅** | 35 | +11 | Higher | -7 | Lower | -18 | Lower |
| Halstead Volume | 2136.6995 | **1800.6880 ✅** | 3205.5551 | +336.0115 | Higher | -1068.8557 | Lower | -1404.8672 | Lower |
| Halstead Effort | 119767.6275 | **74207.2992 ✅** | 247701.9874 | +45560.3282 | Higher | -127934.3600 | Lower | -173494.6882 | Lower |
| SLOC | 75 | **66 ✅** | 102 | +9 | Higher | -27 | Lower | -36 | Lower |
| PLOC | 54 | **45 ✅** | 78 | +9 | Higher | -24 | Lower | -33 | Lower |
| LLOC | 49 | **35 ✅** | 74 | +14 | Higher | -25 | Lower | -39 | Lower |
| Comments (CLOC) | 10 | **14 ✅** | 12 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | 11 | **9 ✅** | 13 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p01545

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.3383 | **59.9869 ✅** | 52.5720 | -3.6486 | Lower | +3.7663 | Higher | +7.4149 | Higher |
| MI (SEI) | 32.4684 | **38.8774 ✅** | 31.6764 | -6.4089 | Lower | +0.7921 | Higher | +7.2010 | Higher |
| Cyclomatic (sum) | 16 | **13 ✅** | 18 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | **14 ✅** | 15 | 24 | -1 | Lower | -10 | Lower | -9 | Lower |
| Halstead Volume | 2465.1530 | **2055.5102 ✅** | 2964.9843 | +409.6428 | Higher | -499.8313 | Lower | -909.4741 | Lower |
| Halstead Effort | 144842.7694 | **91016.0781 ✅** | 197235.9106 | +53826.6912 | Higher | -52393.1412 | Lower | -106219.8324 | Lower |
| SLOC | 77 | **68 ✅** | 89 | +9 | Higher | -12 | Lower | -21 | Lower |
| PLOC | 59 | **51 ✅** | 64 | +8 | Higher | -5 | Lower | -13 | Lower |
| LLOC | 39 | 39 | 56 | 0 | Same | -17 | Lower | -17 | Lower |
| Comments (CLOC) | 9 | 9 | **15 ✅** | 0 | Same | -6 | Lower | -6 | Lower |
| Blank Lines | 11 | **8 ✅** | 10 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |

## p01570

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.8839 | **64.6207 ✅** | 38.6154 | -12.7368 | Lower | +13.2685 | Higher | +26.0053 | Higher |
| MI (SEI) | 22.2000 | **63.7390 ✅** | 6.6754 | -41.5390 | Lower | +15.5246 | Higher | +57.0635 | Higher |
| Cyclomatic (sum) | 14 | **6 ✅** | 25 | +8 | Higher | -11 | Lower | -19 | Lower |
| Cognitive (sum) | 18 | **7 ✅** | 43 | +11 | Higher | -25 | Lower | -36 | Lower |
| Halstead Volume | 3190.7629 | **1004.5354 ✅** | 5025.4959 | +2186.2275 | Higher | -1834.7330 | Lower | -4020.9605 | Lower |
| Halstead Effort | 158588.5139 | **22521.6841 ✅** | 367654.7020 | +136066.8298 | Higher | -209066.1881 | Lower | -345133.0179 | Lower |
| SLOC | 96 | **71 ✅** | 161 | +25 | Higher | -65 | Lower | -90 | Lower |
| PLOC | 71 | **27 ✅** | 124 | +44 | Higher | -53 | Lower | -97 | Lower |
| LLOC | 61 | **21 ✅** | 120 | +40 | Higher | -59 | Lower | -99 | Lower |
| Comments (CLOC) | 8 | **39 ✅** | 17 | -31 | Lower | -9 | Lower | +22 | Higher |
| Blank Lines | 17 | **5 ✅** | 21 | +12 | Higher | -4 | Lower | -16 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |
| Exits | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |

## p01600

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.1553 ✅** | 60.5601 | 57.2921 | +5.5951 | Higher | +8.8632 | Higher | +3.2680 | Higher |
| MI (SEI) | 43.6631 | **51.2605 ✅** | 33.8445 | -7.5974 | Lower | +9.8186 | Higher | +17.4160 | Higher |
| Cyclomatic (sum) | **10 ✅** | 12 | 16 | -2 | Lower | -6 | Lower | -4 | Lower |
| Cognitive (sum) | 14 | **13 ✅** | 22 | +1 | Higher | -8 | Lower | -9 | Lower |
| Halstead Volume | **1557.9944 ✅** | 1838.6659 | 2052.0351 | -280.6715 | Lower | -494.0408 | Lower | -213.3693 | Lower |
| Halstead Effort | **55283.6710 ✅** | 65009.9715 | 101370.5349 | -9726.3006 | Lower | -46086.8639 | Lower | -36360.5633 | Lower |
| SLOC | **53 ✅** | 69 | 77 | -16 | Lower | -24 | Lower | -8 | Lower |
| PLOC | 41 | **40 ✅** | 60 | +1 | Higher | -19 | Lower | -20 | Lower |
| LLOC | 31 | **30 ✅** | 46 | +1 | Higher | -15 | Lower | -16 | Lower |
| Comments (CLOC) | 5 | **22 ✅** | 9 | -17 | Lower | -4 | Lower | +13 | Higher |
| Blank Lines | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

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

## p01658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.0511 ✅** | 59.4278 | 64.2064 | +5.6233 | Higher | +0.8447 | Higher | -4.7787 | Lower |
| MI (SEI) | 43.1635 | **55.3680 ✅** | 43.1876 | -12.2045 | Lower | -0.0241 | Lower | +12.1804 | Higher |
| Cyclomatic (sum) | 11 | 10 | 10 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 25 | **17 ✅** | 18 | +8 | Higher | +7 | Higher | -1 | Lower |
| Halstead Volume | 1391.8847 | **1130.1023 ✅** | 1539.8830 | +261.7823 | Higher | -147.9983 | Lower | -409.7806 | Lower |
| Halstead Effort | 59262.1665 | **33432.1944 ✅** | 55339.5440 | +25829.9722 | Higher | +3922.6225 | Higher | -21907.3496 | Lower |
| SLOC | **58 ✅** | 89 | 60 | -31 | Lower | -2 | Lower | +29 | Higher |
| PLOC | 44 | **37 ✅** | 46 | +7 | Higher | -2 | Lower | -9 | Lower |
| LLOC | 41 | **29 ✅** | 42 | +12 | Higher | -1 | Lower | -13 | Lower |
| Comments (CLOC) | 6 | **44 ✅** | 7 | -38 | Lower | -1 | Lower | +37 | Higher |
| Blank Lines | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p01668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.6708 | **49.8776 ✅** | 29.5811 | -17.2069 | Lower | +3.0896 | Higher | +20.2965 | Higher |
| MI (SEI) | -1.5954 | **23.4149 ✅** | -4.5531 | -25.0103 | Lower | +2.9576 | Higher | +27.9679 | Higher |
| Cyclomatic (sum) | 45 | **22 ✅** | 48 | +23 | Higher | -3 | Lower | -26 | Lower |
| Cognitive (sum) | 47 | **18 ✅** | 48 | +29 | Higher | -1 | Lower | -30 | Lower |
| Halstead Volume | 5807.3164 | **3189.7664 ✅** | 6265.4074 | +2617.5500 | Higher | -458.0910 | Lower | -3075.6410 | Lower |
| Halstead Effort | 489065.3841 | **158585.5556 ✅** | 534125.9805 | +330479.8285 | Higher | -45060.5964 | Lower | -375540.4249 | Lower |
| SLOC | 167 | **97 ✅** | 189 | +70 | Higher | -22 | Lower | -92 | Lower |
| PLOC | 131 | **77 ✅** | 151 | +54 | Higher | -20 | Lower | -74 | Lower |
| LLOC | 69 | **36 ✅** | 75 | +33 | Higher | -6 | Lower | -39 | Lower |
| Comments (CLOC) | 15 | 11 | **19 ✅** | +4 | Higher | -4 | Lower | -8 | Lower |
| Blank Lines | 26 | **10 ✅** | 26 | +16 | Higher | 0 | Same | -16 | Lower |
| Functions | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 10 | **5 ✅** | 10 | +5 | Higher | 0 | Same | -5 | Lower |

## p01725

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **41.7286 ✅** | 34.9014 | 35.0039 | +6.8272 | Higher | +6.7247 | Higher | -0.1025 | Lower |
| MI (SEI) | **8.6024 ✅** | -2.2723 | -3.6921 | +10.8747 | Higher | +12.2945 | Higher | +1.4198 | Higher |
| Cyclomatic (sum) | **28 ✅** | 47 | 34 | -19 | Lower | -6 | Lower | +13 | Higher |
| Cognitive (sum) | 51 | **50 ✅** | 57 | +1 | Higher | -6 | Lower | -7 | Lower |
| Halstead Volume | **3998.9815 ✅** | 4455.5928 | 5121.5324 | -456.6113 | Lower | -1122.5509 | Lower | -665.9395 | Lower |
| Halstead Effort | **211028.2209 ✅** | 250487.8586 | 400079.7041 | -39459.6378 | Lower | -189051.4832 | Lower | -149591.8455 | Lower |
| SLOC | **137 ✅** | 154 | 176 | -17 | Lower | -39 | Lower | -22 | Lower |
| PLOC | **114 ✅** | 134 | 149 | -20 | Lower | -35 | Lower | -15 | Lower |
| LLOC | 59 | **58 ✅** | 114 | +1 | Higher | -55 | Lower | -56 | Lower |
| Comments (CLOC) | **11 ✅** | 9 | 10 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | 13 | **12 ✅** | 18 | +1 | Higher | -5 | Lower | -6 | Lower |
| Functions | **3 ✅** | 4 | 5 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **2 ✅** | 3 | 5 | -1 | Lower | -3 | Lower | -2 | Lower |

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

## p01782

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.2351 | **61.4528 ✅** | 21.3555 | -39.2177 | Lower | +0.8796 | Higher | +40.0973 | Higher |
| MI (SEI) | -25.1812 | **47.5140 ✅** | -26.8775 | -72.6952 | Lower | +1.6963 | Higher | +74.3915 | Higher |
| Cyclomatic (sum) | 46 | **18 ✅** | 45 | +28 | Higher | +1 | Higher | -27 | Lower |
| Cognitive (sum) | 50 | **18 ✅** | 54 | +32 | Higher | -4 | Lower | -36 | Lower |
| Halstead Volume | 8449.0402 | **1364.0696 ✅** | 8974.3709 | +7084.9706 | Higher | -525.3307 | Lower | -7610.3013 | Lower |
| Halstead Effort | 693947.8312 | **45495.7327 ✅** | 775669.0458 | +648452.0985 | Higher | -81721.2146 | Lower | -730173.3131 | Lower |
| SLOC | 278 | **66 ✅** | 292 | +212 | Higher | -14 | Lower | -226 | Lower |
| PLOC | 244 | **44 ✅** | 253 | +200 | Higher | -9 | Lower | -209 | Lower |
| LLOC | 146 | **25 ✅** | 166 | +121 | Higher | -20 | Lower | -141 | Lower |
| Comments (CLOC) | 9 | **14 ✅** | 9 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 29 | **8 ✅** | 33 | +21 | Higher | -4 | Lower | -25 | Lower |
| Functions | 13 | **2 ✅** | 12 | +11 | Higher | +1 | Higher | -10 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **3 ✅** | 4 | +4 | Higher | +3 | Higher | -1 | Lower |

## p01841

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **42.5374 ✅** | 40.5861 | 35.0920 | +1.9513 | Higher | +7.4454 | Higher | +5.4941 | Higher |
| MI (SEI) | **14.3522 ✅** | 9.7708 | 3.6174 | +4.5814 | Higher | +10.7348 | Higher | +6.1534 | Higher |
| Cyclomatic (sum) | **26 ✅** | 31 | 28 | -5 | Lower | -2 | Lower | +3 | Higher |
| Cognitive (sum) | 23 | **22 ✅** | 25 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | **3825.8632 ✅** | 4362.5253 | 6229.1155 | -536.6621 | Lower | -2403.2523 | Lower | -1866.5902 | Lower |
| Halstead Effort | **187815.1032 ✅** | 251935.8350 | 365252.6813 | -64120.7318 | Lower | -177437.5781 | Lower | -113316.8464 | Lower |
| SLOC | **136 ✅** | 137 | 179 | -1 | Lower | -43 | Lower | -42 | Lower |
| PLOC | **102 ✅** | 109 | 140 | -7 | Lower | -38 | Lower | -31 | Lower |
| LLOC | **56 ✅** | 74 | 114 | -18 | Lower | -58 | Lower | -40 | Lower |
| Comments (CLOC) | 17 | 14 | **22 ✅** | +3 | Higher | -5 | Lower | -8 | Lower |
| Blank Lines | **17 ✅** | 19 | 18 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | 8 | **2 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |

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

## p01907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.9465 ✅** | 56.3958 | 33.1937 | +1.5508 | Higher | +24.7529 | Higher | +23.2021 | Higher |
| MI (SEI) | **36.7088 ✅** | 32.3956 | -3.8889 | +4.3132 | Higher | +40.5977 | Higher | +36.2845 | Higher |
| Cyclomatic (sum) | **10 ✅** | 13 | 29 | -3 | Lower | -19 | Lower | -16 | Lower |
| Cognitive (sum) | **13 ✅** | 17 | 28 | -4 | Lower | -15 | Lower | -11 | Lower |
| Halstead Volume | **2457.3603 ✅** | 2899.7391 | 5626.8213 | -442.3788 | Lower | -3169.4609 | Lower | -2727.0821 | Lower |
| Halstead Effort | **85111.6996 ✅** | 118605.0168 | 343199.5608 | -33493.3171 | Lower | -258087.8612 | Lower | -224594.5441 | Lower |
| SLOC | 76 | 76 | 205 | 0 | Same | -129 | Lower | -129 | Lower |
| PLOC | 56 | **54 ✅** | 163 | +2 | Higher | -107 | Lower | -109 | Lower |
| LLOC | 48 | **44 ✅** | 136 | +4 | Higher | -88 | Lower | -92 | Lower |
| Comments (CLOC) | 11 | 9 | **16 ✅** | +2 | Higher | -5 | Lower | -7 | Lower |
| Blank Lines | **9 ✅** | 14 | 26 | -5 | Lower | -17 | Lower | -12 | Lower |
| Functions | 1 | 1 | 5 | 0 | Same | -4 | Lower | -4 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |

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

## p02045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.5103 ✅** | 45.5535 | 44.5228 | +2.9568 | Higher | +3.9875 | Higher | +1.0307 | Higher |
| MI (SEI) | 18.9047 | **29.2303 ✅** | 9.9735 | -10.3256 | Lower | +8.9312 | Higher | +19.2568 | Higher |
| Cyclomatic (sum) | 34 | **31 ✅** | 36 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 22 | **19 ✅** | 22 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 3305.1537 | **3233.0407 ✅** | 3343.8340 | +72.1130 | Higher | -38.6803 | Lower | -110.7933 | Lower |
| Halstead Effort | 203935.0127 | **170859.1727 ✅** | 191225.5058 | +33075.8400 | Higher | +12709.5070 | Higher | -20366.3331 | Lower |
| SLOC | **88 ✅** | 111 | 109 | -23 | Lower | -21 | Lower | +2 | Higher |
| PLOC | 75 | **72 ✅** | 92 | +3 | Higher | -17 | Lower | -20 | Lower |
| LLOC | 44 | **32 ✅** | 57 | +12 | Higher | -13 | Lower | -25 | Lower |
| Comments (CLOC) | 7 | **30 ✅** | 6 | -23 | Lower | +1 | Higher | +24 | Higher |
| Blank Lines | **7 ✅** | 9 | 11 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | 8 | 0 | Same | -7 | Lower | -7 | Lower |

## p02046

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.6926 | **69.9812 ✅** | 67.4244 | -0.2886 | Lower | +2.2682 | Higher | +2.5568 | Higher |
| MI (SEI) | 48.3478 | **58.7607 ✅** | 52.0361 | -10.4129 | Lower | -3.6883 | Lower | +6.7246 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 14 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1202.5542 | **1016.4898 ✅** | 1226.2308 | +186.0644 | Higher | -23.6766 | Lower | -209.7410 | Lower |
| Halstead Effort | 34666.7341 | **24219.8233 ✅** | 36315.2954 | +10446.9108 | Higher | -1648.5613 | Lower | -12095.4721 | Lower |
| SLOC | **45 ✅** | 48 | 50 | -3 | Lower | -5 | Lower | -2 | Lower |
| PLOC | 34 | **30 ✅** | 35 | +4 | Higher | -1 | Lower | -5 | Lower |
| LLOC | 26 | **20 ✅** | 27 | +6 | Higher | -1 | Lower | -7 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 8 | -6 | Lower | -4 | Lower | +2 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **36.7747 ✅** | 32.7430 | 26.6317 | +4.0317 | Higher | +10.1430 | Higher | +6.1113 | Higher |
| MI (SEI) | 0.7972 | **4.6910 ✅** | -18.6962 | -3.8938 | Lower | +19.4934 | Higher | +23.3872 | Higher |
| Cyclomatic (sum) | **31 ✅** | 55 | 42 | -24 | Lower | -11 | Lower | +13 | Higher |
| Cognitive (sum) | **22 ✅** | 42 | 42 | -20 | Lower | -20 | Lower | 0 | Same |
| Halstead Volume | 5086.8736 | **4642.3986 ✅** | 8029.9987 | +444.4750 | Higher | -2943.1251 | Lower | -3387.6001 | Lower |
| Halstead Effort | 274046.3580 | **261259.9068 ✅** | 582132.1934 | +12786.4511 | Higher | -308085.8354 | Lower | -320872.2865 | Lower |
| SLOC | 165 | **155 ✅** | 228 | +10 | Higher | -63 | Lower | -73 | Lower |
| PLOC | 131 | **121 ✅** | 197 | +10 | Higher | -66 | Lower | -76 | Lower |
| LLOC | 77 | **55 ✅** | 131 | +22 | Higher | -54 | Lower | -76 | Lower |
| Comments (CLOC) | 12 | **22 ✅** | 8 | -10 | Lower | +4 | Higher | +14 | Higher |
| Blank Lines | 22 | **13 ✅** | 25 | +9 | Higher | -3 | Lower | -12 | Lower |
| Functions | 6 | **5 ✅** | 11 | +1 | Higher | -5 | Lower | -6 | Lower |
| Closures | 0 | 8 | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Exits | **4 ✅** | 13 | 9 | -9 | Lower | -5 | Lower | +4 | Higher |

## p02061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.1389 | **50.5245 ✅** | 47.7854 | -3.3856 | Lower | -0.6465 | Lower | +2.7391 | Higher |
| MI (SEI) | 19.5392 | **27.7497 ✅** | 15.3067 | -8.2104 | Lower | +4.2325 | Higher | +12.4429 | Higher |
| Cyclomatic (sum) | 25 | 25 | **20 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Cognitive (sum) | 35 | 31 | 31 | +4 | Higher | +4 | Higher | 0 | Same |
| Halstead Volume | 3384.7422 | **2632.0503 ✅** | 3238.5200 | +752.6918 | Higher | +146.2221 | Higher | -606.4697 | Lower |
| Halstead Effort | 215697.4844 | **136922.6191 ✅** | 204123.9176 | +78774.8653 | Higher | +11573.5668 | Higher | -67201.2984 | Lower |
| SLOC | 108 | **95 ✅** | 113 | +13 | Higher | -5 | Lower | -18 | Lower |
| PLOC | 84 | **75 ✅** | 92 | +9 | Higher | -8 | Lower | -17 | Lower |
| LLOC | 65 | **59 ✅** | 77 | +6 | Higher | -12 | Lower | -18 | Lower |
| Comments (CLOC) | 12 | **14 ✅** | 8 | -2 | Lower | +4 | Higher | +6 | Higher |
| Blank Lines | 12 | **8 ✅** | 13 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p02114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.9479 | **55.9205 ✅** | 43.2059 | -2.9725 | Lower | +9.7420 | Higher | +12.7145 | Higher |
| MI (SEI) | 23.4563 | **24.3086 ✅** | 17.2862 | -0.8523 | Lower | +6.1700 | Higher | +7.0224 | Higher |
| Cyclomatic (sum) | **18 ✅** | 19 | 26 | -1 | Lower | -8 | Lower | -7 | Lower |
| Cognitive (sum) | **35 ✅** | 37 | 41 | -2 | Lower | -6 | Lower | -4 | Lower |
| Halstead Volume | 2663.8555 | **2436.6917 ✅** | 3966.3257 | +227.1638 | Higher | -1302.4702 | Lower | -1529.6339 | Lower |
| Halstead Effort | 133771.8737 | **99498.2457 ✅** | 248466.7741 | +34273.6281 | Higher | -114694.9004 | Lower | -148968.5285 | Lower |
| SLOC | 90 | **76 ✅** | 129 | +14 | Higher | -39 | Lower | -53 | Lower |
| PLOC | 70 | **62 ✅** | 92 | +8 | Higher | -22 | Lower | -30 | Lower |
| LLOC | 61 | **38 ✅** | 79 | +23 | Higher | -18 | Lower | -41 | Lower |
| Comments (CLOC) | 7 | 4 | **19 ✅** | +3 | Higher | -12 | Lower | -15 | Lower |
| Blank Lines | 13 | **10 ✅** | 18 | +3 | Higher | -5 | Lower | -8 | Lower |
| Functions | 1 | 1 | 5 | 0 | Same | -4 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 5 | -1 | Lower | -5 | Lower | -4 | Lower |

## p02153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.0499 | **39.6403 ✅** | 21.4425 | -0.5904 | Lower | +17.6074 | Higher | +18.1978 | Higher |
| MI (SEI) | **7.1594 ✅** | 5.5144 | -28.1164 | +1.6450 | Higher | +35.2759 | Higher | +33.6308 | Higher |
| Cyclomatic (sum) | **28 ✅** | 35 | 57 | -7 | Lower | -29 | Lower | -22 | Lower |
| Cognitive (sum) | **45 ✅** | 48 | 63 | -3 | Lower | -18 | Lower | -15 | Lower |
| Halstead Volume | 5262.2759 | **4589.8012 ✅** | 8316.4736 | +672.4747 | Higher | -3054.1977 | Lower | -3726.6724 | Lower |
| Halstead Effort | 317198.2982 | **257061.1901 ✅** | 715021.0488 | +60137.1081 | Higher | -397822.7506 | Lower | -457959.8587 | Lower |
| SLOC | 148 | **135 ✅** | 251 | +13 | Higher | -103 | Lower | -116 | Lower |
| PLOC | 121 | **109 ✅** | 230 | +12 | Higher | -109 | Lower | -121 | Lower |
| LLOC | 82 | **77 ✅** | 141 | +5 | Higher | -59 | Lower | -64 | Lower |
| Comments (CLOC) | **15 ✅** | 10 | 5 | +5 | Higher | +10 | Higher | +5 | Higher |
| Blank Lines | 16 | 16 | 17 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 10 | +1 | Higher | -8 | Lower | -9 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **2 ✅** | 5 | 11 | -3 | Lower | -9 | Lower | -6 | Lower |

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

## p02205

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.5609 | **83.7962 ✅** | 79.8068 | -1.2353 | Lower | +2.7541 | Higher | +3.9894 | Higher |
| MI (SEI) | 64.2794 | **74.0437 ✅** | 70.9247 | -9.7643 | Lower | -6.6453 | Lower | +3.1189 | Higher |
| Cyclomatic (sum) | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 708.2439 | **656.5805 ✅** | 708.4857 | +51.6634 | Higher | -0.2418 | Lower | -51.9052 | Lower |
| Halstead Effort | 15283.1578 | **8910.7350 ✅** | 13759.5372 | +6372.4228 | Higher | +1523.6205 | Higher | -4848.8022 | Lower |
| SLOC | 27 | **26 ✅** | 32 | +1 | Higher | -5 | Lower | -6 | Lower |
| PLOC | 21 | **17 ✅** | 21 | +4 | Higher | 0 | Same | -4 | Lower |
| LLOC | 21 | **15 ✅** | 21 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 2 | 4 | **6 ✅** | -2 | Lower | -4 | Lower | -2 | Lower |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

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

## p02340

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.2576 ✅** | 64.2958 | 69.0385 | +4.9617 | Higher | +0.2191 | Higher | -4.7427 | Lower |
| MI (SEI) | 48.2960 | 55.0679 | **56.0739 ✅** | -6.7719 | Lower | -7.7779 | Lower | -1.0060 | Lower |
| Cyclomatic (sum) | 13 | 14 | **8 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| Cognitive (sum) | 9 | 12 | **6 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Halstead Volume | 1441.2475 | 1487.9272 | **1421.4572 ✅** | -46.6796 | Lower | +19.7903 | Higher | +66.4700 | Higher |
| Halstead Effort | 53369.8334 | 58051.7041 | **43976.3325 ✅** | -4681.8707 | Lower | +9393.5009 | Higher | +14075.3716 | Higher |
| SLOC | **43 ✅** | 57 | 47 | -14 | Lower | -4 | Lower | +10 | Higher |
| PLOC | 34 | 34 | **30 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| LLOC | **18 ✅** | 22 | 21 | -4 | Lower | -3 | Lower | +1 | Higher |
| Comments (CLOC) | 4 | **16 ✅** | 9 | -12 | Lower | -5 | Lower | +7 | Higher |
| Blank Lines | **5 ✅** | 7 | 8 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

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

## p02353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **40.2470 ✅** | 37.8575 | 34.6866 | +2.3896 | Higher | +5.5605 | Higher | +3.1709 | Higher |
| MI (SEI) | 0.6169 | **2.6046 ✅** | -3.9163 | -1.9877 | Lower | +4.5332 | Higher | +6.5209 | Higher |
| Cyclomatic (sum) | **21 ✅** | 32 | 31 | -11 | Lower | -10 | Lower | +1 | Higher |
| Cognitive (sum) | **19 ✅** | 20 | 20 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **4835.3529 ✅** | 5430.1937 | 5599.8017 | -594.8408 | Lower | -764.4488 | Lower | -169.6080 | Lower |
| Halstead Effort | **290842.8682 ✅** | 449916.2322 | 370818.8712 | -159073.3640 | Lower | -79976.0030 | Lower | +79097.3610 | Higher |
| SLOC | 156 | **149 ✅** | 182 | +7 | Higher | -26 | Lower | -33 | Lower |
| PLOC | 124 | 124 | 154 | 0 | Same | -30 | Lower | -30 | Lower |
| LLOC | **71 ✅** | 75 | 86 | -4 | Lower | -15 | Lower | -11 | Lower |
| Comments (CLOC) | 7 | 11 | 11 | -4 | Lower | -4 | Lower | 0 | Same |
| Blank Lines | 26 | **16 ✅** | 20 | +10 | Higher | +6 | Higher | -4 | Lower |
| Functions | **6 ✅** | 7 | 8 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |

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

## p02371

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.5263 | **55.7331 ✅** | 38.5439 | -10.2067 | Lower | +6.9825 | Higher | +17.1892 | Higher |
| MI (SEI) | 17.7379 | **38.7913 ✅** | 4.5626 | -21.0534 | Lower | +13.1753 | Higher | +34.2288 | Higher |
| Cyclomatic (sum) | **16 ✅** | 18 | 23 | -2 | Lower | -7 | Lower | -5 | Lower |
| Cognitive (sum) | **11 ✅** | 15 | 23 | -4 | Lower | -12 | Lower | -8 | Lower |
| Halstead Volume | 4147.8877 | **2006.5642 ✅** | 6020.1903 | +2141.3236 | Higher | -1872.3025 | Lower | -4013.6261 | Lower |
| Halstead Effort | 257331.7017 | **75924.0492 ✅** | 502893.4802 | +181407.6525 | Higher | -245561.7785 | Lower | -426969.4310 | Lower |
| SLOC | 127 | **83 ✅** | 157 | +44 | Higher | -30 | Lower | -74 | Lower |
| PLOC | 98 | **55 ✅** | 133 | +43 | Higher | -35 | Lower | -78 | Lower |
| LLOC | 75 | **35 ✅** | 117 | +40 | Higher | -42 | Lower | -82 | Lower |
| Comments (CLOC) | 16 | **17 ✅** | 14 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 16 | **12 ✅** | 18 | +4 | Higher | -2 | Lower | -6 | Lower |
| Functions | 5 | **1 ✅** | 6 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |

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

## p02433

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.9405 | **36.1443 ✅** | 28.3642 | -0.2038 | Lower | +7.5763 | Higher | +7.7801 | Higher |
| MI (SEI) | **1.6902 ✅** | 1.2493 | -12.0142 | +0.4408 | Higher | +13.7044 | Higher | +13.2635 | Higher |
| Cyclomatic (sum) | **32 ✅** | 33 | 43 | -1 | Lower | -11 | Lower | -10 | Lower |
| Cognitive (sum) | **22 ✅** | 52 | 39 | -30 | Lower | -17 | Lower | +13 | Higher |
| Halstead Volume | **4357.0075 ✅** | 4702.8164 | 6153.7025 | -345.8089 | Lower | -1796.6950 | Lower | -1450.8861 | Lower |
| Halstead Effort | 237682.2714 | **215244.2890 ✅** | 388961.9498 | +22437.9824 | Higher | -151279.6784 | Lower | -173717.6609 | Lower |
| SLOC | 180 | **171 ✅** | 220 | +9 | Higher | -40 | Lower | -49 | Lower |
| PLOC | 141 | **139 ✅** | 180 | +2 | Higher | -39 | Lower | -41 | Lower |
| LLOC | 89 | 89 | 120 | 0 | Same | -31 | Lower | -31 | Lower |
| Comments (CLOC) | **16 ✅** | 14 | 13 | +2 | Higher | +3 | Higher | +1 | Higher |
| Blank Lines | 24 | **20 ✅** | 30 | +4 | Higher | -6 | Lower | -10 | Lower |
| Functions | 8 | **3 ✅** | 12 | +5 | Higher | -4 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **2 ✅** | 15 | +7 | Higher | -6 | Lower | -13 | Lower |

## p02444

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.7792 ✅** | 50.6336 | 62.3034 | +15.1456 | Higher | +3.4758 | Higher | -11.6698 | Lower |
| MI (SEI) | **50.6917 ✅** | 26.7770 | 46.9058 | +23.9147 | Higher | +3.7858 | Higher | -20.1288 | Lower |
| Cyclomatic (sum) | **8 ✅** | 24 | 12 | -16 | Lower | -4 | Lower | +12 | Higher |
| Cognitive (sum) | **10 ✅** | 22 | 19 | -12 | Lower | -9 | Lower | +3 | Higher |
| Halstead Volume | **1630.3434 ✅** | 2607.4928 | 1835.0249 | -977.1493 | Lower | -204.6815 | Lower | +772.4678 | Higher |
| Halstead Effort | **44306.9805 ✅** | 129738.6651 | 68813.4351 | -85431.6846 | Lower | -24506.4546 | Lower | +60925.2300 | Higher |
| SLOC | **55 ✅** | 96 | 62 | -41 | Lower | -7 | Lower | +34 | Higher |
| PLOC | **37 ✅** | 69 | 42 | -32 | Lower | -5 | Lower | +27 | Higher |
| LLOC | **33 ✅** | 38 | 36 | -5 | Lower | -3 | Lower | +2 | Higher |
| Comments (CLOC) | 10 | **13 ✅** | 12 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | 10 | 14 | **9 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p02457

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.6203 | **48.7900 ✅** | 36.6949 | -0.1696 | Lower | +11.9254 | Higher | +12.0950 | Higher |
| MI (SEI) | **21.0263 ✅** | 15.8571 | 7.0149 | +5.1693 | Higher | +14.0115 | Higher | +8.8422 | Higher |
| Cyclomatic (sum) | **24 ✅** | 26 | 36 | -2 | Lower | -12 | Lower | -10 | Lower |
| Cognitive (sum) | **31 ✅** | 32 | 41 | -1 | Lower | -10 | Lower | -9 | Lower |
| Halstead Volume | **2992.7573 ✅** | 2996.0618 | 4647.1101 | -3.3045 | Lower | -1654.3528 | Lower | -1651.0483 | Lower |
| Halstead Effort | **163241.3061 ✅** | 165702.6447 | 394530.1595 | -2461.3386 | Lower | -231288.8534 | Lower | -228827.5148 | Lower |
| SLOC | 104 | **100 ✅** | 159 | +4 | Higher | -55 | Lower | -59 | Lower |
| PLOC | 83 | **81 ✅** | 123 | +2 | Higher | -40 | Lower | -42 | Lower |
| LLOC | 51 | **49 ✅** | 70 | +2 | Higher | -19 | Lower | -21 | Lower |
| Comments (CLOC) | 11 | 6 | **20 ✅** | +5 | Higher | -9 | Lower | -14 | Lower |
| Blank Lines | **10 ✅** | 13 | 16 | -3 | Lower | -6 | Lower | -3 | Lower |
| Functions | 3 | 3 | 6 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **3 ✅** | 11 | +1 | Higher | -7 | Lower | -8 | Lower |

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

## p02584

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.8283 ✅** | 73.8573 | 77.4071 | +4.9709 | Higher | +1.4212 | Higher | -3.5498 | Lower |
| MI (SEI) | 69.1103 | **69.4175 ✅** | 66.2983 | -0.3072 | Lower | +2.8120 | Higher | +3.1192 | Higher |
| Cyclomatic (sum) | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 4 | 8 | 4 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | **777.0015 ✅** | 872.7228 | 850.1643 | -95.7213 | Lower | -73.1628 | Lower | +22.5585 | Higher |
| Halstead Effort | **16169.0313 ✅** | 18693.7221 | 19128.6958 | -2524.6909 | Lower | -2959.6645 | Lower | -434.9736 | Lower |
| SLOC | **33 ✅** | 42 | 35 | -9 | Lower | -2 | Lower | +7 | Higher |
| PLOC | **22 ✅** | 23 | 23 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | 20 | **16 ✅** | 22 | +4 | Higher | -2 | Lower | -6 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 6 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p02668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.9527 | 41.8292 | **64.8076 ✅** | +14.1235 | Higher | -8.8548 | Lower | -22.9784 | Lower |
| MI (SEI) | 37.6943 | 22.9954 | **48.8519 ✅** | +14.6989 | Higher | -11.1576 | Lower | -25.8565 | Lower |
| Cyclomatic (sum) | 14 | 26 | **6 ✅** | -12 | Lower | +8 | Higher | +20 | Higher |
| Cognitive (sum) | 13 | 21 | **8 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| Halstead Volume | 2677.7475 | 3368.7513 | **2029.8457 ✅** | -691.0038 | Lower | +647.9017 | Higher | +1338.9056 | Higher |
| Halstead Effort | 161621.1854 | 207307.7712 | **99646.9720 ✅** | -45686.5858 | Lower | +61974.2135 | Higher | +107660.7992 | Higher |
| SLOC | 79 | 148 | **56 ✅** | -69 | Lower | +23 | Higher | +92 | Higher |
| PLOC | 53 | 95 | **36 ✅** | -42 | Lower | +17 | Higher | +59 | Higher |
| LLOC | 44 | 63 | **34 ✅** | -19 | Lower | +10 | Higher | +29 | Higher |
| Comments (CLOC) | 15 | **39 ✅** | 10 | -24 | Lower | +5 | Higher | +29 | Higher |
| Blank Lines | 12 | 14 | **10 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 3 | 9 | **0 ✅** | -6 | Lower | +3 | Higher | +9 | Higher |

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

## p02703

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.9635 | **43.9858 ✅** | 42.3412 | -2.0223 | Lower | -0.3777 | Lower | +1.6446 | Higher |
| MI (SEI) | 10.6088 | **16.0457 ✅** | 11.4882 | -5.4370 | Lower | -0.8795 | Lower | +4.5575 | Higher |
| Cyclomatic (sum) | 26 | 29 | **23 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Cognitive (sum) | 25 | 31 | **23 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Halstead Volume | 4371.6574 | **3844.1488 ✅** | 4434.3688 | +527.5086 | Higher | -62.7115 | Lower | -590.2201 | Lower |
| Halstead Effort | 229383.4339 | **194723.6090 ✅** | 230330.1152 | +34659.8250 | Higher | -946.6813 | Lower | -35606.5062 | Lower |
| SLOC | 135 | **119 ✅** | 137 | +16 | Higher | -2 | Lower | -18 | Lower |
| PLOC | 101 | **91 ✅** | 100 | +10 | Higher | +1 | Higher | -9 | Lower |
| LLOC | 76 | **52 ✅** | 80 | +24 | Higher | -4 | Lower | -28 | Lower |
| Comments (CLOC) | 13 | 14 | 14 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | 21 | **15 ✅** | 23 | +6 | Higher | -2 | Lower | -8 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |

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

## p02728

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.8653 | **35.1546 ✅** | 32.8848 | -0.2893 | Lower | +1.9805 | Higher | +2.2698 | Higher |
| MI (SEI) | -1.6629 | **17.6610 ✅** | -0.1722 | -19.3239 | Lower | -1.4907 | Lower | +17.8332 | Higher |
| Cyclomatic (sum) | **24 ✅** | 31 | 29 | -7 | Lower | -5 | Lower | +2 | Higher |
| Cognitive (sum) | **14 ✅** | 37 | 17 | -23 | Lower | -3 | Lower | +20 | Higher |
| Halstead Volume | 6891.2489 | **4194.6503 ✅** | 6550.1460 | +2696.5985 | Higher | +341.1028 | Higher | -2355.4957 | Lower |
| Halstead Effort | 673182.4470 | **269808.4412 ✅** | 689675.7936 | +403374.0058 | Higher | -16493.3466 | Lower | -419867.3524 | Lower |
| SLOC | **186 ✅** | 194 | 199 | -8 | Lower | -13 | Lower | -5 | Lower |
| PLOC | 145 | **110 ✅** | 152 | +35 | Higher | -7 | Lower | -42 | Lower |
| LLOC | 108 | **79 ✅** | 118 | +29 | Higher | -10 | Lower | -39 | Lower |
| Comments (CLOC) | 15 | **67 ✅** | 23 | -52 | Lower | -8 | Lower | +44 | Higher |
| Blank Lines | 28 | **17 ✅** | 25 | +11 | Higher | +3 | Higher | -8 | Lower |
| Functions | 11 | **5 ✅** | 12 | +6 | Higher | -1 | Lower | -7 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |

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

## p02734

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.4675 | **74.5347 ✅** | 58.6576 | -14.0672 | Lower | +1.8099 | Higher | +15.8771 | Higher |
| MI (SEI) | 45.8063 | **64.7147 ✅** | 26.5503 | -18.9085 | Lower | +19.2560 | Higher | +38.1644 | Higher |
| Cyclomatic (sum) | 9 | **6 ✅** | 15 | +3 | Higher | -6 | Lower | -9 | Lower |
| Cognitive (sum) | 12 | **6 ✅** | 21 | +6 | Higher | -9 | Lower | -15 | Lower |
| Halstead Volume | 1871.9004 | **965.0994 ✅** | 2544.2863 | +906.8010 | Higher | -672.3860 | Lower | -1579.1869 | Lower |
| Halstead Effort | 92304.0524 | **23019.4081 ✅** | 201753.9541 | +69284.6443 | Higher | -109449.9017 | Lower | -178734.5460 | Lower |
| SLOC | 72 | **39 ✅** | 67 | +33 | Higher | +5 | Higher | -28 | Lower |
| PLOC | 42 | **24 ✅** | 52 | +18 | Higher | -10 | Lower | -28 | Lower |
| LLOC | 44 | **18 ✅** | 51 | +26 | Higher | -7 | Lower | -33 | Lower |
| Comments (CLOC) | **16 ✅** | 8 | 3 | +8 | Higher | +13 | Higher | +5 | Higher |
| Blank Lines | 14 | **7 ✅** | 12 | +7 | Higher | +2 | Higher | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p02762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.9889 ✅** | 46.0837 | 47.7100 | +2.9051 | Higher | +1.2788 | Higher | -1.6263 | Lower |
| MI (SEI) | 18.8180 | **29.2209 ✅** | 11.4245 | -10.4029 | Lower | +7.3935 | Higher | +17.7964 | Higher |
| Cyclomatic (sum) | 19 | 19 | 19 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 18 | **12 ✅** | 18 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 3375.9096 | **3108.1805 ✅** | 3954.3973 | +267.7291 | Higher | -578.4877 | Lower | -846.2168 | Lower |
| Halstead Effort | 184324.6643 | **150912.0843 ✅** | 246389.3732 | +33412.5800 | Higher | -62064.7088 | Lower | -95477.2889 | Lower |
| SLOC | **105 ✅** | 129 | 108 | -24 | Lower | -3 | Lower | +21 | Higher |
| PLOC | 83 | **80 ✅** | 88 | +3 | Higher | -5 | Lower | -8 | Lower |
| LLOC | 68 | **52 ✅** | 73 | +16 | Higher | -5 | Lower | -21 | Lower |
| Comments (CLOC) | 9 | **36 ✅** | 5 | -27 | Lower | +4 | Higher | +31 | Higher |
| Blank Lines | 13 | 13 | 15 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | **4 ✅** | 6 | 5 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 7 | **3 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

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

## p02840

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.6799 | **43.3840 ✅** | 42.3569 | -0.7041 | Lower | +0.3230 | Higher | +1.0271 | Higher |
| MI (SEI) | 11.9069 | **22.1853 ✅** | 11.2759 | -10.2784 | Lower | +0.6309 | Higher | +10.9093 | Higher |
| Cyclomatic (sum) | **21 ✅** | 22 | 23 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 31 | **26 ✅** | 32 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | 3803.4289 | **3707.7030 ✅** | 4132.4496 | +95.7258 | Higher | -329.0207 | Lower | -424.7466 | Lower |
| Halstead Effort | 298840.8395 | **220074.1706 ✅** | 324205.9762 | +78766.6689 | Higher | -25365.1367 | Lower | -104131.8056 | Lower |
| SLOC | 145 | **138 ✅** | 140 | +7 | Higher | +5 | Higher | -2 | Lower |
| PLOC | 113 | **95 ✅** | 108 | +18 | Higher | +5 | Higher | -13 | Lower |
| LLOC | 85 | **82 ✅** | 83 | +3 | Higher | +2 | Higher | -1 | Lower |
| Comments (CLOC) | 15 | **30 ✅** | 14 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 17 | **14 ✅** | 18 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **2 ✅** | 7 | 3 | -5 | Lower | -1 | Lower | +4 | Higher |

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

## p02884

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **49.7863 ✅** | 44.9534 | 46.9165 | +4.8330 | Higher | +2.8698 | Higher | -1.9631 | Lower |
| MI (SEI) | 25.9561 | 9.8563 | **32.5172 ✅** | +16.0998 | Higher | -6.5611 | Lower | -22.6609 | Lower |
| Cyclomatic (sum) | 24 | 31 | **22 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| Cognitive (sum) | 39 | 40 | **35 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| Halstead Volume | 2702.4272 | 3528.5592 | **2690.0873 ✅** | -826.1321 | Lower | +12.3399 | Higher | +838.4719 | Higher |
| Halstead Effort | 185220.1993 | 215761.0180 | **183236.3311 ✅** | -30540.8187 | Lower | +1983.8682 | Higher | +32524.6869 | Higher |
| SLOC | **100 ✅** | 112 | 123 | -12 | Lower | -23 | Lower | -11 | Lower |
| PLOC | **69 ✅** | 92 | 72 | -23 | Lower | -3 | Lower | +20 | Higher |
| LLOC | 63 | **56 ✅** | 67 | +7 | Higher | -4 | Lower | -11 | Lower |
| Comments (CLOC) | 14 | 6 | **39 ✅** | +8 | Higher | -25 | Lower | -33 | Lower |
| Blank Lines | 18 | 15 | **12 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

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

## p02956

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 27.2644 | **73.9086 ✅** | 27.1856 | -46.6442 | Lower | +0.0787 | Higher | +46.7230 | Higher |
| MI (SEI) | -10.0018 | **74.6686 ✅** | -9.2641 | -84.6704 | Lower | -0.7377 | Lower | +83.9328 | Higher |
| Cyclomatic (sum) | 41 | **5 ✅** | 40 | +36 | Higher | +1 | Higher | -35 | Lower |
| Cognitive (sum) | 43 | **2 ✅** | 47 | +41 | Higher | -4 | Lower | -45 | Lower |
| Halstead Volume | 7534.1545 | **781.3821 ✅** | 7886.2573 | +6752.7724 | Higher | -352.1028 | Lower | -7104.8752 | Lower |
| Halstead Effort | 885263.1552 | **12095.7953 ✅** | 1042112.5714 | +873167.3599 | Higher | -156849.4162 | Lower | -1030016.7761 | Lower |
| SLOC | 227 | **44 ✅** | 228 | +183 | Higher | -1 | Lower | -184 | Lower |
| PLOC | 188 | **19 ✅** | 184 | +169 | Higher | +4 | Higher | -165 | Lower |
| LLOC | 178 | **12 ✅** | 188 | +166 | Higher | -10 | Lower | -176 | Lower |
| Comments (CLOC) | 20 | 20 | **22 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | 23 | **5 ✅** | 25 | +18 | Higher | -2 | Lower | -20 | Lower |
| Functions | 6 | **1 ✅** | 4 | +5 | Higher | +2 | Higher | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

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

## p03007

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.9853 | **51.0245 ✅** | 42.8052 | -5.0392 | Lower | +3.1801 | Higher | +8.2193 | Higher |
| MI (SEI) | 13.4890 | **34.6143 ✅** | 10.3307 | -21.1253 | Lower | +3.1583 | Higher | +24.2836 | Higher |
| Cyclomatic (sum) | 19 | **17 ✅** | 23 | +2 | Higher | -4 | Lower | -6 | Lower |
| Cognitive (sum) | 36 | **23 ✅** | 44 | +13 | Higher | -8 | Lower | -21 | Lower |
| Halstead Volume | 4410.0468 | **2568.8675 ✅** | 5012.0439 | +1841.1793 | Higher | -601.9971 | Lower | -2443.1764 | Lower |
| Halstead Effort | 546845.8083 | **130715.8347 ✅** | 586775.8715 | +416129.9736 | Higher | -39930.0632 | Lower | -456060.0368 | Lower |
| SLOC | 116 | **104 ✅** | 128 | +12 | Higher | -12 | Lower | -24 | Lower |
| PLOC | 95 | **67 ✅** | 103 | +28 | Higher | -8 | Lower | -36 | Lower |
| LLOC | 78 | **50 ✅** | 82 | +28 | Higher | -4 | Lower | -32 | Lower |
| Comments (CLOC) | 9 | **26 ✅** | 11 | -17 | Lower | -2 | Lower | +15 | Higher |
| Blank Lines | **12 ✅** | 13 | 15 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |

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

## p03034

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.2812 | **71.4197 ✅** | 65.2078 | -3.1385 | Lower | +3.0734 | Higher | +6.2120 | Higher |
| MI (SEI) | 55.2413 | **69.0404 ✅** | 48.4678 | -13.7991 | Lower | +6.7735 | Higher | +20.5725 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 12 | +4 | Higher | -2 | Lower | -6 | Lower |
| Cognitive (sum) | 9 | **5 ✅** | 11 | +4 | Higher | -2 | Lower | -6 | Lower |
| Halstead Volume | 1098.4583 | **862.7708 ✅** | 1364.0696 | +235.6876 | Higher | -265.6112 | Lower | -501.2988 | Lower |
| Halstead Effort | 34954.5138 | **20312.6253 ✅** | 51482.6264 | +14641.8885 | Higher | -16528.1126 | Lower | -31170.0011 | Lower |
| SLOC | 52 | **49 ✅** | 57 | +3 | Higher | -5 | Lower | -8 | Lower |
| PLOC | 34 | **25 ✅** | 40 | +9 | Higher | -6 | Lower | -15 | Lower |
| LLOC | 29 | **22 ✅** | 33 | +7 | Higher | -4 | Lower | -11 | Lower |
| Comments (CLOC) | 10 | **19 ✅** | 9 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 8 | **5 ✅** | 9 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03043

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.8998 | **78.4598 ✅** | 61.0357 | -6.5600 | Lower | +10.8640 | Higher | +17.4241 | Higher |
| MI (SEI) | 58.9241 | **72.4463 ✅** | 48.4465 | -13.5222 | Lower | +10.4776 | Higher | +23.9998 | Higher |
| Cyclomatic (sum) | 5 | **4 ✅** | 11 | +1 | Higher | -6 | Lower | -7 | Lower |
| Cognitive (sum) | 4 | **3 ✅** | 17 | +1 | Higher | -13 | Lower | -14 | Lower |
| Halstead Volume | 1072.0884 | **834.0579 ✅** | 1536.0610 | +238.0305 | Higher | -463.9726 | Lower | -702.0031 | Lower |
| Halstead Effort | 30018.4749 | **13792.8456 ✅** | 56042.9518 | +16225.6293 | Higher | -26024.4769 | Lower | -42250.1061 | Lower |
| SLOC | 45 | **33 ✅** | 72 | +12 | Higher | -27 | Lower | -39 | Lower |
| PLOC | 27 | **21 ✅** | 44 | +6 | Higher | -17 | Lower | -23 | Lower |
| LLOC | 26 | **19 ✅** | 37 | +7 | Higher | -11 | Lower | -18 | Lower |
| Comments (CLOC) | 8 | 8 | **18 ✅** | 0 | Same | -10 | Lower | -10 | Lower |
| Blank Lines | 10 | **5 ✅** | 12 | +5 | Higher | -2 | Lower | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03051

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.0566 | **72.2769 ✅** | 55.4815 | -17.2202 | Lower | -0.4249 | Lower | +16.7954 | Higher |
| MI (SEI) | 28.4157 | **68.9745 ✅** | 27.8868 | -40.5588 | Lower | +0.5289 | Higher | +41.0877 | Higher |
| Cyclomatic (sum) | 14 | **8 ✅** | 14 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 14 | **8 ✅** | 15 | +6 | Higher | -1 | Lower | -7 | Lower |
| Halstead Volume | 2832.5875 | **815.4045 ✅** | 2819.0918 | +2017.1830 | Higher | +13.4957 | Higher | -2003.6873 | Lower |
| Halstead Effort | 183764.1151 | **20385.1131 ✅** | 174911.8339 | +163379.0020 | Higher | +8852.2813 | Higher | -154526.7207 | Lower |
| SLOC | 82 | **46 ✅** | 80 | +36 | Higher | +2 | Higher | -34 | Lower |
| PLOC | 63 | **25 ✅** | 60 | +38 | Higher | +3 | Higher | -35 | Lower |
| LLOC | 56 | **15 ✅** | 51 | +41 | Higher | +5 | Higher | -36 | Lower |
| Comments (CLOC) | 8 | **16 ✅** | 7 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 12 | **5 ✅** | 13 | +7 | Higher | -1 | Lower | -8 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

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

## p03064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.3332 | **63.3236 ✅** | 59.8013 | -4.9903 | Lower | -1.4681 | Lower | +3.5222 | Higher |
| MI (SEI) | 44.6949 | **57.8877 ✅** | 48.4588 | -13.1928 | Lower | -3.7640 | Lower | +9.4288 | Higher |
| Cyclomatic (sum) | 9 | 7 | **6 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | 14 | **5 ✅** | 6 | +9 | Higher | +8 | Higher | -1 | Lower |
| Halstead Volume | 2199.0688 | **1289.0948 ✅** | 1893.4344 | +909.9740 | Higher | +305.6344 | Higher | -604.3396 | Lower |
| Halstead Effort | 100442.4676 | **40648.0699 ✅** | 65323.4877 | +59794.3977 | Higher | +35118.9799 | Higher | -24675.4178 | Lower |
| SLOC | 78 | **70 ✅** | 78 | +8 | Higher | 0 | Same | -8 | Lower |
| PLOC | 48 | **32 ✅** | 44 | +16 | Higher | +4 | Higher | -12 | Lower |
| LLOC | 44 | **29 ✅** | 46 | +15 | Higher | -2 | Lower | -17 | Lower |
| Comments (CLOC) | 20 | **28 ✅** | 23 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 13 | **10 ✅** | 12 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p03123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.7298 | **64.2458 ✅** | 40.9174 | -25.5161 | Lower | -2.1877 | Lower | +23.3284 | Higher |
| MI (SEI) | 1.9210 | **58.4224 ✅** | 1.3219 | -56.5014 | Lower | +0.5991 | Higher | +57.1005 | Higher |
| Cyclomatic (sum) | 27 | **10 ✅** | 25 | +17 | Higher | +2 | Higher | -15 | Lower |
| Cognitive (sum) | 26 | **10 ✅** | 24 | +16 | Higher | +2 | Higher | -14 | Lower |
| Halstead Volume | 4771.5120 | **1190.9601 ✅** | 4377.5958 | +3580.5519 | Higher | +393.9162 | Higher | -3186.6357 | Lower |
| Halstead Effort | 289224.0281 | **35232.5697 ✅** | 270446.3846 | +253991.4585 | Higher | +18777.6436 | Higher | -235213.8149 | Lower |
| SLOC | 158 | **65 ✅** | 146 | +93 | Higher | +12 | Higher | -81 | Lower |
| PLOC | 126 | **35 ✅** | 119 | +91 | Higher | +7 | Higher | -84 | Lower |
| LLOC | 98 | **21 ✅** | 90 | +77 | Higher | +8 | Higher | -69 | Lower |
| Comments (CLOC) | 10 | **24 ✅** | 6 | -14 | Lower | +4 | Higher | +18 | Higher |
| Blank Lines | 22 | **6 ✅** | 21 | +16 | Higher | +1 | Higher | -15 | Lower |
| Functions | 9 | **1 ✅** | 9 | +8 | Higher | 0 | Same | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **2 ✅** | 8 | +7 | Higher | +1 | Higher | -6 | Lower |

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

## p03216

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.8013 | **66.1688 ✅** | 56.1677 | -5.3675 | Lower | +4.6336 | Higher | +10.0011 | Higher |
| MI (SEI) | 36.0045 | **57.4645 ✅** | 27.6275 | -21.4600 | Lower | +8.3771 | Higher | +29.8370 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 18 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | **18 ✅** | 19 | 42 | -1 | Lower | -24 | Lower | -23 | Lower |
| Halstead Volume | 1611.8442 | **1185.1787 ✅** | 1991.3790 | +426.6655 | Higher | -379.5348 | Lower | -806.2002 | Lower |
| Halstead Effort | 53052.7004 | **30279.4053 ✅** | 70439.2423 | +22773.2951 | Higher | -17386.5419 | Lower | -40159.8370 | Lower |
| SLOC | 67 | **57 ✅** | 81 | +10 | Higher | -14 | Lower | -24 | Lower |
| PLOC | 53 | **34 ✅** | 64 | +19 | Higher | -11 | Lower | -30 | Lower |
| LLOC | 43 | **25 ✅** | 56 | +18 | Higher | -13 | Lower | -31 | Lower |
| Comments (CLOC) | 6 | **16 ✅** | 6 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 10 | **7 ✅** | 12 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p03365

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.4324 | 56.9917 | **58.6753 ✅** | -1.5593 | Lower | -3.2429 | Lower | -1.6836 | Lower |
| MI (SEI) | 29.4615 | 29.0030 | **30.6652 ✅** | +0.4586 | Higher | -1.2037 | Lower | -1.6623 | Lower |
| Cyclomatic (sum) | 15 | 15 | **13 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 11 | 10 | **9 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Halstead Volume | 2831.5107 | **2272.3711 ✅** | 2645.2271 | +559.1396 | Higher | +186.2836 | Higher | -372.8560 | Lower |
| Halstead Effort | 167960.0686 | **123232.4326 ✅** | 146088.6803 | +44727.6360 | Higher | +21871.3882 | Higher | -22856.2477 | Lower |
| SLOC | 79 | 77 | **68 ✅** | +2 | Higher | +11 | Higher | +9 | Higher |
| PLOC | 60 | 60 | **53 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| LLOC | 50 | **44 ✅** | 46 | +6 | Higher | +4 | Higher | -2 | Lower |
| Comments (CLOC) | **8 ✅** | 6 | 5 | +2 | Higher | +3 | Higher | +1 | Higher |
| Blank Lines | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 7 | **3 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |

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

## p03406

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.9442 | **47.7858 ✅** | 46.9928 | -1.8416 | Lower | -1.0486 | Lower | +0.7930 | Higher |
| MI (SEI) | 12.8541 | **22.5566 ✅** | 14.2473 | -9.7026 | Lower | -1.3932 | Lower | +8.3093 | Higher |
| Cyclomatic (sum) | 22 | 21 | **20 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 33 | **21 ✅** | 32 | +12 | Higher | +1 | Higher | -11 | Lower |
| Halstead Volume | 4223.7886 | **3466.3036 ✅** | 3877.6815 | +757.4850 | Higher | +346.1071 | Higher | -411.3779 | Lower |
| Halstead Effort | 311125.8641 | **225143.0848 ✅** | 265730.9296 | +85982.7793 | Higher | +45394.9346 | Higher | -40587.8447 | Lower |
| SLOC | 113 | **109 ✅** | 112 | +4 | Higher | +1 | Higher | -3 | Lower |
| PLOC | 92 | **81 ✅** | 92 | +11 | Higher | 0 | Same | -11 | Lower |
| LLOC | 75 | **61 ✅** | 72 | +14 | Higher | +3 | Higher | -11 | Lower |
| Comments (CLOC) | 8 | **15 ✅** | 8 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 13 | 14 | 13 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

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

## p03414

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 17.2598 | **42.5268 ✅** | 12.6730 | -25.2669 | Lower | +4.5868 | Higher | +29.8537 | Higher |
| MI (SEI) | -28.3764 | **12.8830 ✅** | -36.7808 | -41.2594 | Lower | +8.4044 | Higher | +49.6638 | Higher |
| Cyclomatic (sum) | 53 | **19 ✅** | 60 | +34 | Higher | -7 | Lower | -41 | Lower |
| Cognitive (sum) | 73 | **30 ✅** | 81 | +43 | Higher | -8 | Lower | -51 | Lower |
| Halstead Volume | 10616.6585 | **4279.3988 ✅** | 12898.0731 | +6337.2597 | Higher | -2281.4146 | Lower | -8618.6743 | Lower |
| Halstead Effort | 1225804.9755 | **234297.0817 ✅** | 1484011.2486 | +991507.8938 | Higher | -258206.2731 | Lower | -1249714.1670 | Lower |
| SLOC | 318 | **145 ✅** | 359 | +173 | Higher | -41 | Lower | -214 | Lower |
| PLOC | 274 | **115 ✅** | 320 | +159 | Higher | -46 | Lower | -205 | Lower |
| LLOC | 195 | **61 ✅** | 217 | +134 | Higher | -22 | Lower | -156 | Lower |
| Comments (CLOC) | 16 | **17 ✅** | 13 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 30 | **14 ✅** | 27 | +16 | Higher | +3 | Higher | -13 | Lower |
| Functions | 8 | **2 ✅** | 9 | +6 | Higher | -1 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 8 | **2 ✅** | 9 | +6 | Higher | -1 | Lower | -7 | Lower |

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

## p03454

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.9347 | **71.3135 ✅** | 47.8158 | -21.3787 | Lower | +2.1190 | Higher | +23.4977 | Higher |
| MI (SEI) | 17.9544 | **59.3560 ✅** | 12.5651 | -41.4016 | Lower | +5.3892 | Higher | +46.7909 | Higher |
| Cyclomatic (sum) | 24 | **8 ✅** | 25 | +16 | Higher | -1 | Lower | -17 | Lower |
| Cognitive (sum) | 40 | **7 ✅** | 40 | +33 | Higher | 0 | Same | -33 | Lower |
| Halstead Volume | 3292.6578 | **917.7708 ✅** | 3444.5367 | +2374.8870 | Higher | -151.8789 | Lower | -2526.7659 | Lower |
| Halstead Effort | 259884.7763 | **24386.4811 ✅** | 271995.3830 | +235498.2952 | Higher | -12110.6066 | Lower | -247608.9018 | Lower |
| SLOC | 93 | **47 ✅** | 103 | +46 | Higher | -10 | Lower | -56 | Lower |
| PLOC | 74 | **31 ✅** | 84 | +43 | Higher | -10 | Lower | -53 | Lower |
| LLOC | 58 | **19 ✅** | 58 | +39 | Higher | 0 | Same | -39 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 5 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 13 | **7 ✅** | 14 | +6 | Higher | -1 | Lower | -7 | Lower |
| Functions | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |

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

## p03562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 33.8809 | **70.5524 ✅** | 28.8049 | -36.6715 | Lower | +5.0760 | Higher | +41.7474 | Higher |
| MI (SEI) | -8.1371 | **54.0044 ✅** | -11.6060 | -62.1415 | Lower | +3.4690 | Higher | +65.6105 | Higher |
| Cyclomatic (sum) | 35 | **11 ✅** | 46 | +24 | Higher | -11 | Lower | -35 | Lower |
| Cognitive (sum) | 58 | **7 ✅** | 91 | +51 | Higher | -33 | Lower | -84 | Lower |
| Halstead Volume | 5870.7475 | **1065.3932 ✅** | 6874.7404 | +4805.3543 | Higher | -1003.9929 | Lower | -5809.3472 | Lower |
| Halstead Effort | 528233.8456 | **26137.6466 ✅** | 632201.1266 | +502096.1990 | Higher | -103967.2811 | Lower | -606063.4800 | Lower |
| SLOC | 178 | **45 ✅** | 198 | +133 | Higher | -20 | Lower | -153 | Lower |
| PLOC | 147 | **33 ✅** | 155 | +114 | Higher | -8 | Lower | -122 | Lower |
| LLOC | 118 | **18 ✅** | 122 | +100 | Higher | -4 | Lower | -104 | Lower |
| Comments (CLOC) | 7 | 6 | **11 ✅** | +1 | Higher | -4 | Lower | -5 | Lower |
| Blank Lines | 24 | **6 ✅** | 32 | +18 | Higher | -8 | Lower | -26 | Lower |
| Functions | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

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

## p03626

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.8030 | 65.1311 | **67.3364 ✅** | -1.3281 | Lower | -3.5334 | Lower | -2.2053 | Lower |
| MI (SEI) | 49.9882 | **57.3869 ✅** | 47.6791 | -7.3987 | Lower | +2.3091 | Higher | +9.7078 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 11 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 22 | **14 ✅** | 16 | +8 | Higher | +6 | Higher | -2 | Lower |
| Halstead Volume | 1292.5135 | **1209.6601 ✅** | 1260.3204 | +82.8534 | Higher | +32.1931 | Higher | -50.6603 | Lower |
| Halstead Effort | 45599.8762 | **35824.5482 ✅** | 43459.3229 | +9775.3280 | Higher | +2140.5533 | Higher | -7634.7747 | Lower |
| SLOC | 66 | 63 | **52 ✅** | +3 | Higher | +14 | Higher | +11 | Higher |
| PLOC | 46 | **37 ✅** | 39 | +9 | Higher | +7 | Higher | -2 | Lower |
| LLOC | 45 | 34 | 34 | +11 | Higher | +11 | Higher | 0 | Same |
| Comments (CLOC) | 14 | **20 ✅** | 6 | -6 | Lower | +8 | Higher | +14 | Higher |
| Blank Lines | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p03647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.1978 | **82.6122 ✅** | 54.4676 | -17.4144 | Lower | +10.7302 | Higher | +28.1445 | Higher |
| MI (SEI) | 48.2354 | **71.6249 ✅** | 40.8404 | -23.3895 | Lower | +7.3950 | Higher | +30.7845 | Higher |
| Cyclomatic (sum) | 12 | **5 ✅** | 17 | +7 | Higher | -5 | Lower | -12 | Lower |
| Cognitive (sum) | 18 | **5 ✅** | 17 | +13 | Higher | +1 | Higher | -12 | Lower |
| Halstead Volume | 1294.6138 | **599.0871 ✅** | 1756.4879 | +695.5267 | Higher | -461.8741 | Lower | -1157.4008 | Lower |
| Halstead Effort | 47728.0954 | **7531.3812 ✅** | 59934.2159 | +40196.7143 | Higher | -12206.1205 | Lower | -52402.8347 | Lower |
| SLOC | 58 | **28 ✅** | 95 | +30 | Higher | -37 | Lower | -67 | Lower |
| PLOC | 43 | **20 ✅** | 57 | +23 | Higher | -14 | Lower | -37 | Lower |
| LLOC | 26 | **14 ✅** | 37 | +12 | Higher | -11 | Lower | -23 | Lower |
| Comments (CLOC) | 9 | 4 | **26 ✅** | +5 | Higher | -17 | Lower | -22 | Lower |
| Blank Lines | 8 | **4 ✅** | 12 | +4 | Higher | -4 | Lower | -8 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

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

## p03709

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.9894 | **67.2159 ✅** | 33.9524 | -32.2265 | Lower | +1.0371 | Higher | +33.2635 | Higher |
| MI (SEI) | 2.8564 | **65.9241 ✅** | -0.3054 | -63.0677 | Lower | +3.1617 | Higher | +66.2294 | Higher |
| Cyclomatic (sum) | 32 | **6 ✅** | 32 | +26 | Higher | 0 | Same | -26 | Lower |
| Cognitive (sum) | 52 | **5 ✅** | 49 | +47 | Higher | +3 | Higher | -44 | Lower |
| Halstead Volume | 6366.9965 | **930.2778 ✅** | 6849.1733 | +5436.7187 | Higher | -482.1768 | Lower | -5918.8956 | Lower |
| Halstead Effort | 707285.4871 | **22362.4461 ✅** | 685621.0133 | +684923.0410 | Higher | +21664.4738 | Higher | -663258.5672 | Lower |
| SLOC | 169 | **62 ✅** | 176 | +107 | Higher | -7 | Lower | -114 | Lower |
| PLOC | 127 | **26 ✅** | 131 | +101 | Higher | -4 | Lower | -105 | Lower |
| LLOC | 107 | **21 ✅** | 115 | +86 | Higher | -8 | Lower | -94 | Lower |
| Comments (CLOC) | 19 | **30 ✅** | 17 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | 23 | **6 ✅** | 29 | +17 | Higher | -6 | Lower | -23 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

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

## p03736

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.7606 ✅** | 50.6740 | 55.4407 | +6.0866 | Higher | +1.3199 | Higher | -4.7668 | Lower |
| MI (SEI) | 28.0340 | 27.0049 | **28.4330 ✅** | +1.0291 | Higher | -0.3990 | Lower | -1.4281 | Lower |
| Cyclomatic (sum) | 10 | 17 | 10 | -7 | Lower | 0 | Same | +7 | Higher |
| Cognitive (sum) | 6 | 25 | 6 | -19 | Lower | 0 | Same | +19 | Higher |
| Halstead Volume | 2846.9096 | **2831.9956 ✅** | 3023.7196 | +14.9141 | Higher | -176.8100 | Lower | -191.7240 | Lower |
| Halstead Effort | **140609.5610 ✅** | 147778.6782 | 154490.9762 | -7169.1172 | Lower | -13881.4151 | Lower | -6712.2979 | Lower |
| SLOC | **78 ✅** | 103 | 83 | -25 | Lower | -5 | Lower | +20 | Higher |
| PLOC | **55 ✅** | 70 | 57 | -15 | Lower | -2 | Lower | +13 | Higher |
| LLOC | **49 ✅** | 60 | 51 | -11 | Lower | -2 | Lower | +9 | Higher |
| Comments (CLOC) | 6 | **15 ✅** | 8 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | **17 ✅** | 18 | 18 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

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

## p03766

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.6085 ✅** | 49.7432 | 63.1240 | +14.8653 | Higher | +1.4845 | Higher | -13.3808 | Lower |
| MI (SEI) | **40.7536 ✅** | 40.0072 | 36.0123 | +0.7464 | Higher | +4.7413 | Higher | +3.9949 | Higher |
| Cyclomatic (sum) | **9 ✅** | 15 | 10 | -6 | Lower | -1 | Lower | +5 | Higher |
| Cognitive (sum) | 5 | 10 | 5 | -5 | Lower | 0 | Same | +5 | Higher |
| Halstead Volume | **1846.9795 ✅** | 2075.8248 | 1998.1463 | -228.8453 | Lower | -151.1668 | Lower | +77.6786 | Higher |
| Halstead Effort | 92348.9748 | 104064.3763 | **88417.9722 ✅** | -11715.4015 | Lower | +3931.0027 | Higher | +15646.4041 | Higher |
| SLOC | **56 ✅** | 124 | 59 | -68 | Lower | -3 | Lower | +65 | Higher |
| PLOC | **43 ✅** | 64 | 45 | -21 | Lower | -2 | Lower | +19 | Higher |
| LLOC | 37 | 49 | **34 ✅** | -12 | Lower | +3 | Higher | +15 | Higher |
| Comments (CLOC) | 5 | **53 ✅** | 4 | -48 | Lower | +1 | Higher | +49 | Higher |
| Blank Lines | 10 | **8 ✅** | 10 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |

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

## p03865

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.6401 | **87.0751 ✅** | 76.1440 | -4.4349 | Lower | +6.4962 | Higher | +10.9311 | Higher |
| MI (SEI) | 80.0024 | **82.2647 ✅** | 68.8357 | -2.2623 | Lower | +11.1667 | Higher | +13.4290 | Higher |
| Cyclomatic (sum) | 6 | **4 ✅** | 8 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 4 | **3 ✅** | 10 | +1 | Higher | -6 | Lower | -7 | Lower |
| Halstead Volume | 459.8300 | **377.8306 ✅** | 648.2638 | +81.9994 | Higher | -188.4338 | Lower | -270.4332 | Lower |
| Halstead Effort | 6700.3799 | **4463.1237 ✅** | 17264.2884 | +2237.2562 | Higher | -10563.9085 | Lower | -12801.1647 | Lower |
| SLOC | 30 | **25 ✅** | 39 | +5 | Higher | -9 | Lower | -14 | Lower |
| PLOC | 18 | **16 ✅** | 25 | +2 | Higher | -7 | Lower | -9 | Lower |
| LLOC | **9 ✅** | 11 | 12 | -2 | Lower | -3 | Lower | -1 | Lower |
| Comments (CLOC) | 8 | 5 | **9 ✅** | +3 | Higher | -1 | Lower | -4 | Lower |
| Blank Lines | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p03899

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.9489 ✅** | 53.0931 | 55.5016 | +3.8558 | Higher | +1.4473 | Higher | -2.4085 | Lower |
| MI (SEI) | **38.0232 ✅** | 37.6068 | 30.2803 | +0.4164 | Higher | +7.7429 | Higher | +7.3264 | Higher |
| Cyclomatic (sum) | **11 ✅** | 20 | 12 | -9 | Lower | -1 | Lower | +8 | Higher |
| Cognitive (sum) | **14 ✅** | 22 | 15 | -8 | Lower | -1 | Lower | +7 | Higher |
| Halstead Volume | 2085.3160 | **2070.7898 ✅** | 2125.6481 | +14.5262 | Higher | -40.3321 | Lower | -54.8583 | Lower |
| Halstead Effort | 92345.6816 | **81204.5425 ✅** | 94414.2023 | +11141.1391 | Higher | -2068.5208 | Lower | -13209.6598 | Lower |
| SLOC | **84 ✅** | 94 | 90 | -10 | Lower | -6 | Lower | +4 | Higher |
| PLOC | **53 ✅** | 59 | 63 | -6 | Lower | -10 | Lower | -4 | Lower |
| LLOC | 40 | **38 ✅** | 44 | +2 | Higher | -4 | Lower | -6 | Lower |
| Comments (CLOC) | 15 | **23 ✅** | 10 | -8 | Lower | +5 | Higher | +13 | Higher |
| Blank Lines | 16 | **12 ✅** | 17 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

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

## p03929

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.8599 | **66.8907 ✅** | 61.1558 | -2.0308 | Lower | +3.7041 | Higher | +5.7349 | Higher |
| MI (SEI) | 51.0296 | **55.6691 ✅** | 39.2895 | -4.6396 | Lower | +11.7400 | Higher | +16.3796 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 14 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 13 | **6 ✅** | 24 | +7 | Higher | -11 | Lower | -18 | Lower |
| Halstead Volume | 1243.0557 | **1190.8537 ✅** | 1570.6700 | +52.2020 | Higher | -327.6143 | Lower | -379.8163 | Lower |
| Halstead Effort | 50869.6640 | **43466.1584 ✅** | 60259.3581 | +7403.5056 | Higher | -9389.6940 | Lower | -16793.1997 | Lower |
| SLOC | 60 | **56 ✅** | 68 | +4 | Higher | -8 | Lower | -12 | Lower |
| PLOC | **37 ✅** | 38 | 51 | -1 | Lower | -14 | Lower | -13 | Lower |
| LLOC | 30 | **25 ✅** | 50 | +5 | Higher | -20 | Lower | -25 | Lower |
| Comments (CLOC) | 12 | **13 ✅** | 8 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | 11 | **5 ✅** | 9 | +6 | Higher | +2 | Higher | -4 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

## p03932

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.5406 | 45.0790 | **48.5716 ✅** | -2.5384 | Lower | -6.0310 | Lower | -3.4926 | Lower |
| MI (SEI) | 14.8971 | 22.2609 | **25.7623 ✅** | -7.3638 | Lower | -10.8652 | Lower | -3.5014 | Lower |
| Cyclomatic (sum) | 19 | 28 | **18 ✅** | -9 | Lower | +1 | Higher | +10 | Higher |
| Cognitive (sum) | **19 ✅** | 71 | 22 | -52 | Lower | -3 | Lower | +49 | Higher |
| Halstead Volume | 4361.0534 | **3091.4228 ✅** | 3502.1317 | +1269.6306 | Higher | +858.9217 | Higher | -410.7089 | Lower |
| Halstead Effort | 344143.9969 | **189607.2674 ✅** | 312190.0257 | +154536.7295 | Higher | +31953.9712 | Higher | -122582.7582 | Lower |
| SLOC | 144 | 121 | **108 ✅** | +23 | Higher | +36 | Higher | +13 | Higher |
| PLOC | 104 | 88 | **74 ✅** | +16 | Higher | +30 | Higher | +14 | Higher |
| LLOC | 67 | 60 | **52 ✅** | +7 | Higher | +15 | Higher | +8 | Higher |
| Comments (CLOC) | 20 | **21 ✅** | 18 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 20 | **15 ✅** | 17 | +5 | Higher | +3 | Higher | -2 | Lower |
| Functions | 4 | **1 ✅** | 2 | +3 | Higher | +2 | Higher | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 3 | 3 | +2 | Higher | +2 | Higher | 0 | Same |

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

## p03977

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.8793 | **58.1418 ✅** | 56.5911 | -4.2625 | Lower | -2.7118 | Lower | +1.5507 | Higher |
| MI (SEI) | 35.2220 | **52.5898 ✅** | 38.1440 | -17.3677 | Lower | -2.9220 | Lower | +14.4458 | Higher |
| Cyclomatic (sum) | 18 | **11 ✅** | 12 | +7 | Higher | +6 | Higher | -1 | Lower |
| Cognitive (sum) | 21 | 13 | 13 | +8 | Higher | +8 | Higher | 0 | Same |
| Halstead Volume | 1944.8497 | **1248.6867 ✅** | 1986.1154 | +696.1630 | Higher | -41.2657 | Lower | -737.4287 | Lower |
| Halstead Effort | 98246.2779 | **35421.0782 ✅** | 85824.2587 | +62825.1997 | Higher | +12422.0192 | Higher | -50403.1805 | Lower |
| SLOC | 94 | 92 | **86 ✅** | +2 | Higher | +8 | Higher | +6 | Higher |
| PLOC | 67 | **41 ✅** | 63 | +26 | Higher | +4 | Higher | -22 | Lower |
| LLOC | 57 | **33 ✅** | 62 | +24 | Higher | -5 | Lower | -29 | Lower |
| Comments (CLOC) | 18 | **42 ✅** | 16 | -24 | Lower | +2 | Higher | +26 | Higher |
| Blank Lines | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p04020

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.8767 ✅** | 77.6362 | 77.5239 | +1.2405 | Higher | +1.3528 | Higher | +0.1123 | Higher |
| MI (SEI) | 67.8451 | **75.4224 ✅** | 59.7947 | -7.5773 | Lower | +8.0504 | Higher | +15.6277 | Higher |
| Cyclomatic (sum) | 6 | **5 ✅** | 8 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 856.1521 | **654.5972 ✅** | 920.7820 | +201.5549 | Higher | -64.6299 | Lower | -266.1848 | Lower |
| Halstead Effort | 30607.4377 | **11845.0914 ✅** | 30975.1065 | +18762.3463 | Higher | -367.6688 | Lower | -19130.0151 | Lower |
| SLOC | **31 ✅** | 37 | 32 | -6 | Lower | -1 | Lower | +5 | Higher |
| PLOC | 21 | **18 ✅** | 24 | +3 | Higher | -3 | Lower | -6 | Lower |
| LLOC | 20 | **16 ✅** | 20 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 5 | **12 ✅** | 3 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p04024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.4248 | **53.4595 ✅** | 37.2233 | -14.0347 | Lower | +2.2015 | Higher | +16.2362 | Higher |
| MI (SEI) | 9.4840 | **26.6458 ✅** | -1.6542 | -17.1618 | Lower | +11.1382 | Higher | +28.3000 | Higher |
| Cyclomatic (sum) | 34 | **23 ✅** | 38 | +11 | Higher | -4 | Lower | -15 | Lower |
| Cognitive (sum) | 46 | **27 ✅** | 53 | +19 | Higher | -7 | Lower | -26 | Lower |
| Halstead Volume | 4271.6715 | **2312.4442 ✅** | 4704.5885 | +1959.2273 | Higher | -432.9169 | Lower | -2392.1442 | Lower |
| Halstead Effort | 288899.8908 | **124293.8776 ✅** | 385414.3642 | +164606.0133 | Higher | -96514.4734 | Lower | -261120.4867 | Lower |
| SLOC | 142 | **85 ✅** | 149 | +57 | Higher | -7 | Lower | -64 | Lower |
| PLOC | 110 | **67 ✅** | 121 | +43 | Higher | -11 | Lower | -54 | Lower |
| LLOC | 89 | **49 ✅** | 89 | +40 | Higher | 0 | Same | -40 | Lower |
| Comments (CLOC) | **16 ✅** | 8 | 7 | +8 | Higher | +9 | Higher | +1 | Higher |
| Blank Lines | 16 | **10 ✅** | 21 | +6 | Higher | -5 | Lower | -11 | Lower |
| Functions | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |

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
| MI (original) | 59.0486 | **60.6563 ✅** | 55.5518 | -1.6078 | Lower | 3.4968 | Higher | 5.1045 | Higher |
| MI (SEI) | 35.8034 | **43.3734 ✅** | 30.4708 | -7.5699 | Lower | 5.3326 | Higher | 12.9025 | Higher |
| Cyclomatic (sum) | 17.8567 | **17.2400 ✅** | 19.9367 | 0.6167 | Higher | -2.0800 | Lower | -2.6967 | Lower |
| Cognitive (sum) | 22.6567 | **19.9600 ✅** | 25.5433 | 2.6967 | Higher | -2.8867 | Lower | -5.5833 | Lower |
| Halstead Volume | 2611.8089 | **2124.9849 ✅** | 3043.7575 | 486.8240 | Higher | -431.9485 | Lower | -918.7726 | Lower |
| Halstead Effort | 170410.6447 | **107038.1267 ✅** | 217661.5822 | 63372.5180 | Higher | -47250.9374 | Lower | -110623.4555 | Lower |
| SLOC | 88.0100 | **80.9600 ✅** | 100.3333 | 7.0500 | Higher | -12.3233 | Lower | -19.3733 | Lower |
| PLOC | 68.2433 | **57.0500 ✅** | 79.0800 | 11.1933 | Higher | -10.8367 | Lower | -22.0300 | Lower |
| LLOC | 49.2433 | **37.9067 ✅** | 58.9433 | 11.3367 | Higher | -9.7000 | Lower | -21.0367 | Lower |
| Comments (CLOC) | 8.7267 | **14.2633 ✅** | 9.0100 | -5.5367 | Lower | -0.2833 | Lower | 5.2533 | Higher |
| Blank Lines | 11.8533 | **10.1300 ✅** | 12.9900 | 1.7233 | Higher | -1.1367 | Lower | -2.8600 | Lower |
| Functions | 2.3633 | **1.8067 ✅** | 2.8133 | 0.5567 | Higher | -0.4500 | Lower | -1.0067 | Lower |
| Closures | 0.4500 | 0.7667 | **0.3833 ✅** | -0.3167 | Lower | 0.0667 | Higher | 0.3833 | Higher |
| Exits | **2.0933 ✅** | 2.3700 | 2.5633 | -0.2767 | Lower | -0.4700 | Lower | -0.1933 | Lower |

