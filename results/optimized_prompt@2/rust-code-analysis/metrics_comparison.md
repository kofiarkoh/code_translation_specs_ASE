# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/codenet_statistics copy/r_code/metrics.json`

## p00004

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.6749 ✅** | 73.9636 | 67.1011 | +0.7112 | Higher | +7.5738 | Higher | +6.8625 | Higher |
| MI (SEI) | 61.5150 | **68.4487 ✅** | 44.1504 | -6.9337 | Lower | +17.3646 | Higher | +24.2983 | Higher |
| Cyclomatic (sum) | 6 | 6 | 16 | 0 | Same | -10 | Lower | -10 | Lower |
| Cognitive (sum) | 5 | 5 | 13 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | 1018.6151 | **855.0660 ✅** | 1272.0008 | +163.5491 | Higher | -253.3858 | Lower | -416.9349 | Lower |
| Halstead Effort | 23109.8298 | **18310.2055 ✅** | 37688.9140 | +4799.6243 | Higher | -14579.0842 | Lower | -19378.7085 | Lower |
| SLOC | **38 ✅** | 42 | 49 | -4 | Lower | -11 | Lower | -7 | Lower |
| PLOC | **26 ✅** | 27 | 41 | -1 | Lower | -15 | Lower | -14 | Lower |
| LLOC | **17 ✅** | 18 | 18 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 6 | **12 ✅** | 4 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 8 | 7 | **5 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00010

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.8945 | 63.4819 | **67.9108 ✅** | +3.4126 | Higher | -1.0164 | Lower | -4.4290 | Lower |
| MI (SEI) | 44.9452 | **50.1878 ✅** | 38.5928 | -5.2426 | Lower | +6.3524 | Higher | +11.5950 | Higher |
| Cyclomatic (sum) | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 5 | **1 ✅** | 5 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 1934.2061 | 2097.4048 | **1929.0164 ✅** | -163.1986 | Lower | +5.1898 | Higher | +168.3884 | Higher |
| Halstead Effort | 67420.8990 | **64495.1964 ✅** | 75231.6381 | +2925.7026 | Higher | -7810.7392 | Lower | -10736.4418 | Lower |
| SLOC | 50 | 61 | **47 ✅** | -11 | Lower | +3 | Higher | +14 | Higher |
| PLOC | 34 | 34 | 36 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 28 | **24 ✅** | 29 | +4 | Higher | -1 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 2 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.6294 ✅** | 80.0268 | 80.5028 | +7.6026 | Higher | +7.1266 | Higher | -0.4760 | Lower |
| MI (SEI) | **93.1576 ✅** | 78.2244 | 77.1225 | +14.9333 | Higher | +16.0351 | Higher | +1.1019 | Higher |
| Cyclomatic (sum) | **6 ✅** | 8 | 8 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **10 ✅** | 21 | 12 | -11 | Lower | -2 | Lower | +9 | Higher |
| Halstead Volume | **462.9591 ✅** | 516.9925 | 634.8730 | -54.0334 | Lower | -171.9139 | Lower | -117.8805 | Lower |
| Halstead Effort | **5967.0286 ✅** | 6824.3010 | 14632.3105 | -857.2724 | Lower | -8665.2819 | Lower | -7808.0095 | Lower |
| SLOC | **22 ✅** | 33 | 30 | -11 | Lower | -8 | Lower | +3 | Higher |
| PLOC | **18 ✅** | 21 | 25 | -3 | Lower | -7 | Lower | -4 | Lower |
| LLOC | **10 ✅** | 11 | 17 | -1 | Lower | -7 | Lower | -6 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 8 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00015

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.0510 | **66.3507 ✅** | 57.9460 | -7.2997 | Lower | +1.1050 | Higher | +8.4047 | Higher |
| MI (SEI) | 38.0410 | **54.1063 ✅** | 22.3097 | -16.0653 | Lower | +15.7313 | Higher | +31.7966 | Higher |
| Cyclomatic (sum) | 14 | **12 ✅** | 17 | +2 | Higher | -3 | Lower | -5 | Lower |
| Cognitive (sum) | 23 | **10 ✅** | 33 | +13 | Higher | -10 | Lower | -23 | Lower |
| Halstead Volume | 1887.4802 | **1156.9905 ✅** | 2044.2721 | +730.4897 | Higher | -156.7918 | Lower | -887.2815 | Lower |
| Halstead Effort | 77899.0055 | **26810.2631 ✅** | 110273.8755 | +51088.7424 | Higher | -32374.8700 | Lower | -83463.6125 | Lower |
| SLOC | 73 | **56 ✅** | 73 | +17 | Higher | 0 | Same | -17 | Lower |
| PLOC | 53 | **40 ✅** | 59 | +13 | Higher | -6 | Lower | -19 | Lower |
| LLOC | 40 | **20 ✅** | 48 | +20 | Higher | -8 | Lower | -28 | Lower |
| Comments (CLOC) | 10 | **12 ✅** | 2 | -2 | Lower | +8 | Higher | +10 | Higher |
| Blank Lines | 12 | **8 ✅** | 14 | +4 | Higher | -2 | Lower | -6 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00023

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.2786 | **72.4159 ✅** | 66.0652 | -6.1373 | Lower | +0.2134 | Higher | +6.3507 | Higher |
| MI (SEI) | 40.9509 | **66.6552 ✅** | 20.6293 | -25.7043 | Lower | +20.3216 | Higher | +46.0258 | Higher |
| Cyclomatic (sum) | 9 | **7 ✅** | 10 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 11 | **7 ✅** | 11 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 1339.6033 | **1187.5658 ✅** | 1412.4339 | +152.0375 | Higher | -72.8306 | Lower | -224.8681 | Lower |
| Halstead Effort | **33576.5094 ✅** | 34713.4624 | 41767.6885 | -1136.9531 | Lower | -8191.1791 | Lower | -7054.2260 | Lower |
| SLOC | 56 | **41 ✅** | 55 | +15 | Higher | +1 | Higher | -14 | Lower |
| PLOC | 45 | **29 ✅** | 46 | +16 | Higher | -1 | Lower | -17 | Lower |
| LLOC | 19 | **16 ✅** | 19 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 0 | -8 | Lower | +4 | Higher | +12 | Higher |
| Blank Lines | 7 | **4 ✅** | 9 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | **0 ✅** | 5 | 1 | -5 | Lower | -1 | Lower | +4 | Higher |

## p00024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 84.9575 | 85.4813 | **93.3639 ✅** | -0.5239 | Lower | -8.4064 | Lower | -7.8826 | Lower |
| MI (SEI) | 79.4133 | **92.1105 ✅** | 90.4835 | -12.6972 | Lower | -11.0702 | Lower | +1.6270 | Higher |
| Cyclomatic (sum) | 6 | 5 | **4 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 6 | 6 | **3 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 519.6825 | **386.4273 ✅** | 452.7842 | +133.2552 | Higher | +66.8984 | Higher | -66.3569 | Lower |
| Halstead Effort | 8413.9079 | **4890.7207 ✅** | 7018.1550 | +3523.1872 | Higher | +1395.7529 | Higher | -2127.4343 | Lower |
| SLOC | 25 | 27 | **16 ✅** | -2 | Lower | +9 | Higher | +11 | Higher |
| PLOC | 19 | 14 | **13 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| LLOC | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 5 | **13 ✅** | 3 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.7531 | 60.4710 | **69.8067 ✅** | +7.2821 | Higher | -2.0536 | Lower | -9.3357 | Lower |
| MI (SEI) | 34.4406 | **45.5944 ✅** | 26.4345 | -11.1538 | Lower | +8.0061 | Higher | +19.1599 | Higher |
| Cyclomatic (sum) | 15 | 17 | **14 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 21 | 24 | 21 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 1101.3004 | 1569.0965 | **1076.8490 ✅** | -467.7960 | Lower | +24.4514 | Higher | +492.2474 | Higher |
| Halstead Effort | **35461.8738 ✅** | 48118.9579 | 40816.7205 | -12657.0841 | Lower | -5354.8468 | Lower | +7302.2373 | Higher |
| SLOC | 50 | 68 | **45 ✅** | -18 | Lower | +5 | Higher | +23 | Higher |
| PLOC | 43 | 55 | **39 ✅** | -12 | Lower | +4 | Higher | +16 | Higher |
| LLOC | **21 ✅** | 32 | 25 | -11 | Lower | -4 | Lower | +7 | Higher |
| Comments (CLOC) | 1 | **14 ✅** | 0 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00071

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.2400 | 55.2430 | **66.7390 ✅** | +5.9970 | Higher | -5.4990 | Lower | -11.4959 | Lower |
| MI (SEI) | **41.3817 ✅** | 40.4967 | 21.9068 | +0.8850 | Higher | +19.4749 | Higher | +18.5899 | Higher |
| Cyclomatic (sum) | 17 | 15 | **13 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 18 | **14 ✅** | 15 | +4 | Higher | +3 | Higher | -1 | Lower |
| Halstead Volume | 2106.2621 | 2823.6687 | **2030.3809 ✅** | -717.4065 | Lower | +75.8812 | Higher | +793.2877 | Higher |
| Halstead Effort | 79755.4141 | 218021.4471 | **72452.5410 ✅** | -138266.0330 | Lower | +7302.8731 | Higher | +145568.9061 | Higher |
| SLOC | 59 | 80 | **45 ✅** | -21 | Lower | +14 | Higher | +35 | Higher |
| PLOC | 40 | 52 | **38 ✅** | -12 | Lower | +2 | Higher | +14 | Higher |
| LLOC | 27 | 27 | **26 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 8 | **20 ✅** | 0 | -12 | Lower | +8 | Higher | +20 | Higher |
| Blank Lines | 11 | 12 | **7 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.0225 | 59.5843 | **64.4391 ✅** | +2.4382 | Higher | -2.4166 | Lower | -4.8548 | Lower |
| MI (SEI) | 50.8884 | 48.9088 | **52.2638 ✅** | +1.9796 | Higher | -1.3754 | Lower | -3.3550 | Lower |
| Cyclomatic (sum) | 21 | 20 | **15 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| Cognitive (sum) | 10 | 8 | **7 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Halstead Volume | 1440.7238 | 1968.3642 | **1311.8294 ✅** | -527.6404 | Lower | +128.8944 | Higher | +656.5348 | Higher |
| Halstead Effort | 67894.1107 | 78963.4479 | **44274.2423 ✅** | -11069.3373 | Lower | +23619.8684 | Higher | +34689.2057 | Higher |
| SLOC | 60 | 64 | **58 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| PLOC | 45 | 45 | **40 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| LLOC | 23 | 26 | **22 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Comments (CLOC) | 15 | **18 ✅** | 13 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | **8 ✅** | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00077

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.6270 | 70.3397 | **76.6106 ✅** | +5.2873 | Higher | -0.9836 | Lower | -6.2709 | Lower |
| MI (SEI) | 52.1766 | 60.3907 | **67.5749 ✅** | -8.2141 | Lower | -15.3983 | Lower | -7.1842 | Lower |
| Cyclomatic (sum) | 9 | 11 | **8 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 29 | **19 ✅** | 22 | +10 | Higher | +7 | Higher | -3 | Lower |
| Halstead Volume | 879.0549 | 851.2372 | **830.2185 ✅** | +27.8177 | Higher | +48.8364 | Higher | +21.0187 | Higher |
| Halstead Effort | 16380.8498 | **16173.5074 ✅** | 17745.9206 | +207.3424 | Higher | -1365.0708 | Lower | -1572.4132 | Lower |
| SLOC | 36 | 49 | **35 ✅** | -13 | Lower | +1 | Higher | +14 | Higher |
| PLOC | 32 | 34 | **31 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| LLOC | 24 | **16 ✅** | 23 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 7 | -9 | Lower | -5 | Lower | +4 | Higher |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.7926 ✅** | 77.5094 | 77.4424 | +3.2832 | Higher | +3.3502 | Higher | +0.0670 | Higher |
| MI (SEI) | **77.3370 ✅** | 76.9630 | 59.0449 | +0.3740 | Higher | +18.2921 | Higher | +17.9181 | Higher |
| Cyclomatic (sum) | **6 ✅** | 8 | 8 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **5 ✅** | 10 | 7 | -5 | Lower | -2 | Lower | +3 | Higher |
| Halstead Volume | 655.9909 | **498.5531 ✅** | 774.3513 | +157.4378 | Higher | -118.3603 | Lower | -275.7982 | Lower |
| Halstead Effort | 14103.8052 | **8586.1925 ✅** | 18517.0960 | +5517.6127 | Higher | -4413.2908 | Lower | -9930.9035 | Lower |
| SLOC | **30 ✅** | 39 | 34 | -9 | Lower | -4 | Lower | +5 | Higher |
| PLOC | **22 ✅** | 24 | 26 | -2 | Lower | -4 | Lower | -2 | Lower |
| LLOC | 16 | **10 ✅** | 17 | +6 | Higher | -1 | Lower | -7 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 3 | -6 | Lower | +5 | Higher | +11 | Higher |
| Blank Lines | 5 | **3 ✅** | 7 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.6028 | **68.1576 ✅** | 55.7414 | -8.5549 | Lower | +3.8614 | Higher | +12.4163 | Higher |
| MI (SEI) | 24.1067 | **56.5976 ✅** | 6.1424 | -32.4909 | Lower | +17.9643 | Higher | +50.4552 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 14 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 23 | **10 ✅** | 25 | +13 | Higher | -2 | Lower | -15 | Lower |
| Halstead Volume | 1777.4921 | **1195.0281 ✅** | 2579.8707 | +582.4640 | Higher | -802.3787 | Lower | -1384.8426 | Lower |
| Halstead Effort | 42363.5611 | **24737.0821 ✅** | 108420.7217 | +17626.4790 | Higher | -66057.1606 | Lower | -83683.6395 | Lower |
| SLOC | 74 | **51 ✅** | 81 | +23 | Higher | -7 | Lower | -30 | Lower |
| PLOC | 63 | **35 ✅** | 70 | +28 | Higher | -7 | Lower | -35 | Lower |
| LLOC | 41 | **19 ✅** | 57 | +22 | Higher | -16 | Lower | -38 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 0 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 9 | 9 | 11 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.1348 ✅** | 62.7913 | 59.9361 | +0.3435 | Higher | +3.1987 | Higher | +2.8552 | Higher |
| MI (SEI) | 41.2183 | **46.7113 ✅** | 33.7800 | -5.4930 | Lower | +7.4383 | Higher | +12.9313 | Higher |
| Cyclomatic (sum) | **13 ✅** | 18 | 15 | -5 | Lower | -2 | Lower | +3 | Higher |
| Cognitive (sum) | 29 | **18 ✅** | 37 | +11 | Higher | -8 | Lower | -19 | Lower |
| Halstead Volume | **1355.3197 ✅** | 1577.1398 | 1523.1814 | -221.8201 | Lower | -167.8617 | Lower | +53.9584 | Higher |
| Halstead Effort | **49260.6576 ✅** | 70971.2905 | 67273.8461 | -21710.6329 | Lower | -18013.1885 | Lower | +3697.4444 | Higher |
| SLOC | 64 | **58 ✅** | 73 | +6 | Higher | -9 | Lower | -15 | Lower |
| PLOC | 48 | **44 ✅** | 55 | +4 | Higher | -7 | Lower | -11 | Lower |
| LLOC | 39 | **28 ✅** | 46 | +11 | Higher | -7 | Lower | -18 | Lower |
| Comments (CLOC) | 7 | **10 ✅** | 6 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 11 | **8 ✅** | 12 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00118

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.8136 ✅** | 53.1627 | 49.7296 | +2.6509 | Higher | +6.0840 | Higher | +3.4331 | Higher |
| MI (SEI) | 26.4525 | **39.0432 ✅** | -1.1052 | -12.5907 | Lower | +27.5577 | Higher | +40.1484 | Higher |
| Cyclomatic (sum) | **20 ✅** | 21 | 28 | -1 | Lower | -8 | Lower | -7 | Lower |
| Cognitive (sum) | 26 | 26 | 34 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | 2585.8015 | **2579.0287 ✅** | 3408.8630 | +6.7728 | Higher | -823.0615 | Lower | -829.8343 | Lower |
| Halstead Effort | 213954.2192 | **126972.1780 ✅** | 381918.9047 | +86982.0412 | Higher | -167964.6856 | Lower | -254946.7268 | Lower |
| SLOC | **74 ✅** | 86 | 88 | -12 | Lower | -14 | Lower | -2 | Lower |
| PLOC | **56 ✅** | 58 | 73 | -2 | Lower | -17 | Lower | -15 | Lower |
| LLOC | 40 | **37 ✅** | 55 | +3 | Higher | -15 | Lower | -18 | Lower |
| Comments (CLOC) | 5 | **23 ✅** | 0 | -18 | Lower | +5 | Higher | +23 | Higher |
| Blank Lines | 13 | 13 | 15 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00124

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.5291 | 57.9157 | **68.6642 ✅** | +10.6134 | Higher | -0.1352 | Lower | -10.7485 | Lower |
| MI (SEI) | 45.1948 | 39.2042 | **49.9723 ✅** | +5.9906 | Higher | -4.7775 | Lower | -10.7681 | Lower |
| Cyclomatic (sum) | 8 | 16 | 8 | -8 | Lower | 0 | Same | +8 | Higher |
| Cognitive (sum) | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **1292.9007 ✅** | 1820.1279 | 1341.5647 | -527.2271 | Lower | -48.6640 | Lower | +478.5632 | Higher |
| Halstead Effort | **26620.4947 ✅** | 58921.3489 | 30357.6927 | -32300.8542 | Lower | -3737.1980 | Lower | +28563.6562 | Higher |
| SLOC | 50 | 77 | **49 ✅** | -27 | Lower | +1 | Higher | +28 | Higher |
| PLOC | **37 ✅** | 58 | 39 | -21 | Lower | -2 | Lower | +19 | Higher |
| LLOC | **25 ✅** | 29 | 27 | -4 | Lower | -2 | Lower | +2 | Higher |
| Comments (CLOC) | 4 | **13 ✅** | 6 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 12 | 13 | **9 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00136

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.5188 ✅** | 68.5647 | 57.9666 | +2.9542 | Higher | +13.5522 | Higher | +10.5980 | Higher |
| MI (SEI) | **64.9997 ✅** | 64.6641 | 27.4841 | +0.3356 | Higher | +37.5156 | Higher | +37.1800 | Higher |
| Cyclomatic (sum) | 10 | **8 ✅** | 19 | +2 | Higher | -9 | Lower | -11 | Lower |
| Cognitive (sum) | 11 | **8 ✅** | 15 | +3 | Higher | -4 | Lower | -7 | Lower |
| Halstead Volume | **1065.3932 ✅** | 1458.2225 | 1786.4402 | -392.8293 | Lower | -721.0469 | Lower | -328.2176 | Lower |
| Halstead Effort | **20837.8377 ✅** | 30367.4836 | 56570.6048 | -9529.6460 | Lower | -35732.7671 | Lower | -26203.1212 | Lower |
| SLOC | **43 ✅** | 48 | 74 | -5 | Lower | -31 | Lower | -26 | Lower |
| PLOC | **32 ✅** | 36 | 60 | -4 | Lower | -28 | Lower | -24 | Lower |
| LLOC | 26 | 26 | 50 | 0 | Same | -24 | Lower | -24 | Lower |
| Comments (CLOC) | 12 | **18 ✅** | 4 | -6 | Lower | +8 | Higher | +14 | Higher |
| Blank Lines | 6 | 6 | 10 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00137

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.2231 ✅** | 80.7990 | 78.6030 | +0.4241 | Higher | +2.6201 | Higher | +2.1960 | Higher |
| MI (SEI) | 71.9791 | **80.6976 ✅** | 70.4296 | -8.7185 | Lower | +1.5495 | Higher | +10.2680 | Higher |
| Cyclomatic (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 4 | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 701.4924 | **585.4121 ✅** | 739.3422 | +116.0803 | Higher | -37.8498 | Lower | -153.9301 | Lower |
| Halstead Effort | 13552.8326 | **7805.4945 ✅** | 16899.2498 | +5747.3380 | Higher | -3346.4172 | Lower | -9093.7552 | Lower |
| SLOC | **29 ✅** | 32 | 34 | -3 | Lower | -5 | Lower | -2 | Lower |
| PLOC | 21 | **16 ✅** | 22 | +5 | Higher | -1 | Lower | -6 | Lower |
| LLOC | 16 | **11 ✅** | 21 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 7 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00146

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.2602 | **63.4114 ✅** | 53.0758 | -12.1512 | Lower | -1.8156 | Lower | +10.3356 | Higher |
| MI (SEI) | 0.3902 | **49.2914 ✅** | 11.2674 | -48.9012 | Lower | -10.8771 | Lower | +38.0240 | Higher |
| Cyclomatic (sum) | 21 | **11 ✅** | 19 | +10 | Higher | +2 | Higher | -8 | Lower |
| Cognitive (sum) | 31 | **15 ✅** | 28 | +16 | Higher | +3 | Higher | -13 | Lower |
| Halstead Volume | 3461.3279 | **1907.8487 ✅** | 3200.1393 | +1553.4792 | Higher | +261.1886 | Higher | -1292.2906 | Lower |
| Halstead Effort | 289189.7271 | **79540.4571 ✅** | 236200.7576 | +209649.2700 | Higher | +52988.9694 | Higher | -156660.3006 | Lower |
| SLOC | 88 | **58 ✅** | 83 | +30 | Higher | +5 | Higher | -25 | Lower |
| PLOC | 77 | **44 ✅** | 73 | +33 | Higher | +4 | Higher | -29 | Lower |
| LLOC | 63 | **38 ✅** | 58 | +25 | Higher | +5 | Higher | -20 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 1 | -12 | Lower | -1 | Lower | +11 | Higher |
| Blank Lines | 11 | **8 ✅** | 10 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p00156

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.2932 | **53.3706 ✅** | 49.7747 | -3.0774 | Lower | +0.5185 | Higher | +3.5959 | Higher |
| MI (SEI) | 22.1576 | **29.2480 ✅** | -1.3457 | -7.0904 | Lower | +23.5033 | Higher | +30.5937 | Higher |
| Cyclomatic (sum) | **22 ✅** | 23 | 25 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 57 | **37 ✅** | 56 | +20 | Higher | +1 | Higher | -19 | Lower |
| Halstead Volume | 3247.5441 | **2533.4570 ✅** | 3725.5273 | +714.0871 | Higher | -477.9831 | Lower | -1192.0703 | Lower |
| Halstead Effort | 203606.8960 | **104060.3976 ✅** | 329661.3992 | +99546.4984 | Higher | -126054.5033 | Lower | -225601.0016 | Lower |
| SLOC | 94 | **83 ✅** | 89 | +11 | Higher | +5 | Higher | -6 | Lower |
| PLOC | 77 | **61 ✅** | 78 | +16 | Higher | -1 | Lower | -17 | Lower |
| LLOC | 72 | **48 ✅** | 76 | +24 | Higher | -4 | Lower | -28 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 0 | -1 | Lower | +9 | Higher | +10 | Higher |
| Blank Lines | 15 | 14 | **11 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00167

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.3011 | 68.8283 | **71.2999 ✅** | +1.4727 | Higher | -0.9989 | Lower | -2.4716 | Lower |
| MI (SEI) | 45.7125 | **54.1134 ✅** | 48.8319 | -8.4009 | Lower | -3.1194 | Lower | +5.2815 | Higher |
| Cyclomatic (sum) | **9 ✅** | 12 | 13 | -3 | Lower | -4 | Lower | -1 | Lower |
| Cognitive (sum) | **11 ✅** | 13 | 20 | -2 | Lower | -9 | Lower | -7 | Lower |
| Halstead Volume | **1066.7949 ✅** | 1089.1057 | 1128.7712 | -22.3108 | Lower | -61.9764 | Lower | -39.6656 | Lower |
| Halstead Effort | **28684.9287 ✅** | 41045.6698 | 35426.0512 | -12360.7410 | Lower | -6741.1224 | Lower | +5619.6186 | Higher |
| SLOC | 47 | 49 | **41 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| PLOC | 40 | **34 ✅** | 37 | +6 | Higher | +3 | Higher | -3 | Lower |
| LLOC | 22 | **21 ✅** | 23 | +1 | Higher | -1 | Lower | -2 | Lower |
| Comments (CLOC) | 3 | **8 ✅** | 3 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 7 | 10 | **4 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p00187

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.4690 | **63.3757 ✅** | 51.4218 | -9.9067 | Lower | +2.0472 | Higher | +11.9539 | Higher |
| MI (SEI) | 12.1547 | **44.0556 ✅** | 12.8788 | -31.9008 | Lower | -0.7240 | Lower | +31.1768 | Higher |
| Cyclomatic (sum) | 19 | **13 ✅** | 23 | +6 | Higher | -4 | Lower | -10 | Lower |
| Cognitive (sum) | 24 | **11 ✅** | 46 | +13 | Higher | -22 | Lower | -35 | Lower |
| Halstead Volume | 3748.4897 | **1667.1650 ✅** | 3976.2625 | +2081.3247 | Higher | -227.7728 | Lower | -2309.0975 | Lower |
| Halstead Effort | 190235.8529 | **45665.8234 ✅** | 249609.8800 | +144570.0295 | Higher | -59374.0271 | Lower | -203944.0566 | Lower |
| SLOC | 77 | **59 ✅** | 81 | +18 | Higher | -4 | Lower | -22 | Lower |
| PLOC | 65 | **42 ✅** | 70 | +23 | Higher | -5 | Lower | -28 | Lower |
| LLOC | 44 | **18 ✅** | 57 | +26 | Higher | -13 | Lower | -39 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 2 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 12 | **9 ✅** | 11 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p00190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.5171 | **56.2111 ✅** | 32.0052 | -24.6940 | Lower | -0.4881 | Lower | +24.2058 | Higher |
| MI (SEI) | -16.8419 | **32.0546 ✅** | -18.4328 | -48.8966 | Lower | +1.5909 | Higher | +50.4874 | Higher |
| Cyclomatic (sum) | 37 | **18 ✅** | 33 | +19 | Higher | +4 | Higher | -15 | Lower |
| Cognitive (sum) | 72 | **15 ✅** | 60 | +57 | Higher | +12 | Higher | -45 | Lower |
| Halstead Volume | 6687.3885 | **2052.7627 ✅** | 6499.4265 | +4634.6258 | Higher | +187.9620 | Higher | -4446.6638 | Lower |
| Halstead Effort | 595177.5797 | **54434.8679 ✅** | 655035.4869 | +540742.7117 | Higher | -59857.9072 | Lower | -600600.6189 | Lower |
| SLOC | 192 | **80 ✅** | 199 | +112 | Higher | -7 | Lower | -119 | Lower |
| PLOC | 169 | **59 ✅** | 177 | +110 | Higher | -8 | Lower | -118 | Lower |
| LLOC | 99 | **25 ✅** | 135 | +74 | Higher | -36 | Lower | -110 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 2 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 21 | **15 ✅** | 20 | +6 | Higher | +1 | Higher | -5 | Lower |
| Functions | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |

## p00203

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.6999 | **66.1709 ✅** | 49.9079 | -16.4710 | Lower | -0.2080 | Lower | +16.2630 | Higher |
| MI (SEI) | 9.8510 | **52.3449 ✅** | -0.6444 | -42.4939 | Lower | +10.4953 | Higher | +52.9893 | Higher |
| Cyclomatic (sum) | 26 | **11 ✅** | 30 | +15 | Higher | -4 | Lower | -19 | Lower |
| Cognitive (sum) | 39 | **6 ✅** | 39 | +33 | Higher | 0 | Same | -33 | Lower |
| Halstead Volume | 3049.8189 | **1184.7038 ✅** | 3238.9857 | +1865.1151 | Higher | -189.1668 | Lower | -2054.2819 | Lower |
| Halstead Effort | 229972.8293 | **35825.4422 ✅** | 246856.9830 | +194147.3871 | Higher | -16884.1537 | Lower | -211031.5408 | Lower |
| SLOC | 94 | **57 ✅** | 86 | +37 | Higher | +8 | Higher | -29 | Lower |
| PLOC | 79 | **38 ✅** | 77 | +41 | Higher | +2 | Higher | -39 | Lower |
| LLOC | 49 | **23 ✅** | 50 | +26 | Higher | -1 | Lower | -27 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 0 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 15 | 11 | **9 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Functions | 5 | **1 ✅** | 6 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | **2 ✅** | 3 | 5 | -1 | Lower | -3 | Lower | -2 | Lower |
| Exits | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |

## p00208

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.4744 ✅** | 70.2128 | 72.4648 | +2.2616 | Higher | +0.0096 | Higher | -2.2520 | Lower |
| MI (SEI) | 60.4527 | 62.0266 | **71.4406 ✅** | -1.5739 | Lower | -10.9879 | Lower | -9.4140 | Lower |
| Cyclomatic (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 15 | 15 | **8 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Halstead Volume | **862.5902 ✅** | 894.8170 | 998.9625 | -32.2268 | Lower | -136.3723 | Lower | -104.1455 | Lower |
| Halstead Effort | 24021.2618 | **23676.8588 ✅** | 32466.2827 | +344.4030 | Higher | -8445.0210 | Lower | -8789.4239 | Lower |
| SLOC | 44 | 50 | **42 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| PLOC | 33 | 33 | 36 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 29 | 29 | 33 | 0 | Same | -4 | Lower | -4 | Lower |
| Comments (CLOC) | 8 | 13 | **17 ✅** | -5 | Lower | -9 | Lower | -4 | Lower |
| Blank Lines | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00244

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.2925 | **50.3972 ✅** | 33.9573 | -8.1047 | Lower | +8.3351 | Higher | +16.4398 | Higher |
| MI (SEI) | 2.4077 | **20.8498 ✅** | -14.9273 | -18.4421 | Lower | +17.3350 | Higher | +35.7771 | Higher |
| Cyclomatic (sum) | 33 | **19 ✅** | 35 | +14 | Higher | -2 | Lower | -16 | Lower |
| Cognitive (sum) | 37 | **17 ✅** | 42 | +20 | Higher | -5 | Lower | -25 | Lower |
| Halstead Volume | 3923.7526 | **2574.9670 ✅** | 5992.3159 | +1348.7856 | Higher | -2068.5633 | Lower | -3417.3489 | Lower |
| Halstead Effort | 197384.7065 | **98572.9561 ✅** | 470396.7962 | +98811.7504 | Higher | -273012.0897 | Lower | -371823.8401 | Lower |
| SLOC | 124 | **105 ✅** | 176 | +19 | Higher | -52 | Lower | -71 | Lower |
| PLOC | 102 | **84 ✅** | 148 | +18 | Higher | -46 | Lower | -64 | Lower |
| LLOC | 48 | **33 ✅** | 94 | +15 | Higher | -46 | Lower | -61 | Lower |
| Comments (CLOC) | 4 | **9 ✅** | 2 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 22 | **18 ✅** | 26 | +4 | Higher | -4 | Lower | -8 | Lower |
| Functions | **5 ✅** | 6 | 9 | -1 | Lower | -4 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **4 ✅** | 5 | 9 | -1 | Lower | -5 | Lower | -4 | Lower |

## p00252

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.2620 | **84.2665 ✅** | 79.3740 | -15.0045 | Lower | -10.1121 | Lower | +4.8924 | Higher |
| MI (SEI) | 56.1368 | **86.3064 ✅** | 65.6567 | -30.1697 | Lower | -9.5199 | Lower | +20.6498 | Higher |
| Cyclomatic (sum) | 15 | **9 ✅** | 17 | +6 | Higher | -2 | Lower | -8 | Lower |
| Cognitive (sum) | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 1144.0489 | **460 ✅** | 827.3508 | +684.0489 | Higher | +316.6981 | Higher | -367.3508 | Lower |
| Halstead Effort | 31318.3392 | **8117.6471 ✅** | 19759.0834 | +23200.6921 | Higher | +11559.2557 | Higher | -11641.4364 | Lower |
| SLOC | 45 | 26 | 26 | +19 | Higher | +19 | Higher | 0 | Same |
| PLOC | 30 | **14 ✅** | 20 | +16 | Higher | +10 | Higher | -6 | Lower |
| LLOC | 12 | **6 ✅** | 12 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 8 | **9 ✅** | 3 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 8 | 4 | **3 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p00256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **50.6936 ✅** | 44.2117 | 47.6488 | +6.4818 | Higher | +3.0447 | Higher | -3.4371 | Lower |
| MI (SEI) | 10.7927 | **21.2510 ✅** | 10.7308 | -10.4584 | Lower | +0.0619 | Higher | +10.5202 | Higher |
| Cyclomatic (sum) | **20 ✅** | 43 | 24 | -23 | Lower | -4 | Lower | +19 | Higher |
| Cognitive (sum) | 39 | **23 ✅** | 50 | +16 | Higher | -11 | Lower | -27 | Lower |
| Halstead Volume | 3512.6467 | **3203.0689 ✅** | 4076.3726 | +309.5778 | Higher | -563.7259 | Lower | -873.3037 | Lower |
| Halstead Effort | 253858.9789 | **120410.7524 ✅** | 333493.4292 | +133448.2265 | Higher | -79634.4503 | Lower | -213082.6768 | Lower |
| SLOC | **92 ✅** | 102 | 100 | -10 | Lower | -8 | Lower | +2 | Higher |
| PLOC | **79 ✅** | 84 | 90 | -5 | Lower | -11 | Lower | -6 | Lower |
| LLOC | 88 | **28 ✅** | 106 | +60 | Higher | -18 | Lower | -78 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 4 | -14 | Lower | -2 | Lower | +12 | Higher |
| Blank Lines | 11 | 10 | 10 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | 6 | **1 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Closures | 1 | 6 | **0 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Exits | 1 | 19 | **0 ✅** | -18 | Lower | +1 | Higher | +19 | Higher |

## p00261

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.9858 | 59.7396 | **70.3395 ✅** | +8.2462 | Higher | -2.3537 | Lower | -10.5999 | Lower |
| MI (SEI) | 23.2983 | **45.2420 ✅** | 26.4904 | -21.9437 | Lower | -3.1920 | Lower | +18.7517 | Higher |
| Cyclomatic (sum) | 9 | 10 | **7 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 17 | 20 | **11 ✅** | -3 | Lower | +6 | Higher | +9 | Higher |
| Halstead Volume | 1145.2226 | 2248.9489 | **1083.4332 ✅** | -1103.7263 | Lower | +61.7894 | Higher | +1165.5157 | Higher |
| Halstead Effort | **18082.4623 ✅** | 97922.9846 | 22836.7413 | -79840.5223 | Lower | -4754.2790 | Lower | +75086.2433 | Higher |
| SLOC | 53 | 70 | **48 ✅** | -17 | Lower | +5 | Higher | +22 | Higher |
| PLOC | 46 | 49 | **41 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| LLOC | 23 | 27 | **19 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| Comments (CLOC) | 0 | **16 ✅** | 0 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 7 | 10 | 7 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00270

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.7146 | **52.4531 ✅** | 23.7794 | -12.7385 | Lower | +15.9353 | Higher | +28.6738 | Higher |
| MI (SEI) | -5.7397 | **26.4178 ✅** | -29.7209 | -32.1575 | Lower | +23.9812 | Higher | +56.1387 | Higher |
| Cyclomatic (sum) | 32 | **15 ✅** | 46 | +17 | Higher | -14 | Lower | -31 | Lower |
| Cognitive (sum) | 53 | **18 ✅** | 59 | +35 | Higher | -6 | Lower | -41 | Lower |
| Halstead Volume | 5287.7249 | **2649.2449 ✅** | 9426.2398 | +2638.4800 | Higher | -4138.5149 | Lower | -6776.9949 | Lower |
| Halstead Effort | 366174.9485 | **92567.7340 ✅** | 935809.8308 | +273607.2145 | Higher | -569634.8823 | Lower | -843242.0968 | Lower |
| SLOC | 134 | **97 ✅** | 244 | +37 | Higher | -110 | Lower | -147 | Lower |
| PLOC | 114 | **78 ✅** | 215 | +36 | Higher | -101 | Lower | -137 | Lower |
| LLOC | 82 | **40 ✅** | 178 | +42 | Higher | -96 | Lower | -138 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 2 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 20 | **16 ✅** | 27 | +4 | Higher | -7 | Lower | -11 | Lower |
| Functions | **3 ✅** | 5 | 11 | -2 | Lower | -8 | Lower | -6 | Lower |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | **1 ✅** | 3 | 5 | -2 | Lower | -4 | Lower | -2 | Lower |

## p00271

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.6995 | 73.1047 | **89.2638 ✅** | +8.5948 | Higher | -7.5642 | Lower | -16.1591 | Lower |
| MI (SEI) | 72.6664 | 59.9405 | **85.3205 ✅** | +12.7259 | Higher | -12.6541 | Lower | -25.3800 | Lower |
| Cyclomatic (sum) | 5 | 9 | **3 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Cognitive (sum) | 2 | 8 | **1 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Halstead Volume | 640.0769 | 820.8633 | **519.5448 ✅** | -180.7864 | Lower | +120.5321 | Higher | +301.3185 | Higher |
| Halstead Effort | 11607.5488 | 16926.7678 | **5590.7541 ✅** | -5319.2190 | Lower | +6016.7947 | Higher | +11336.0137 | Higher |
| SLOC | 29 | 43 | **20 ✅** | -14 | Lower | +9 | Higher | +23 | Higher |
| PLOC | 19 | 31 | **13 ✅** | -12 | Lower | +6 | Higher | +18 | Higher |
| LLOC | 11 | 16 | **9 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Comments (CLOC) | 5 | **7 ✅** | 4 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00283

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.5188 ✅** | 59.0324 | 58.5944 | +0.4864 | Higher | +0.9244 | Higher | +0.4380 | Higher |
| MI (SEI) | 11.5921 | **38.3666 ✅** | 10.4621 | -26.7745 | Lower | +1.1301 | Higher | +27.9046 | Higher |
| Cyclomatic (sum) | **14 ✅** | 18 | 16 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | 21 | 22 | **20 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 2476.5927 | **2172.6284 ✅** | 2582.1849 | +303.9643 | Higher | -105.5922 | Lower | -409.5565 | Lower |
| Halstead Effort | 143417.2332 | **75718.4117 ✅** | 149766.7258 | +67698.8214 | Higher | -6349.4926 | Lower | -74048.3140 | Lower |
| SLOC | **65 ✅** | 66 | 66 | -1 | Lower | -1 | Lower | 0 | Same |
| PLOC | 55 | **48 ✅** | 57 | +7 | Higher | -2 | Lower | -9 | Lower |
| LLOC | 42 | **31 ✅** | 46 | +11 | Higher | -4 | Lower | -15 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 0 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p00288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.1902 | **46.4816 ✅** | 16.9871 | -22.2913 | Lower | +7.2031 | Higher | +29.4945 | Higher |
| MI (SEI) | -36.3217 | **28.5838 ✅** | -39.4596 | -64.9055 | Lower | +3.1380 | Higher | +68.0435 | Higher |
| Cyclomatic (sum) | 44 | **20 ✅** | 54 | +24 | Higher | -10 | Lower | -34 | Lower |
| Cognitive (sum) | 78 | **38 ✅** | 89 | +40 | Higher | -11 | Lower | -51 | Lower |
| Halstead Volume | 10149.7662 | **2964.1900 ✅** | 11945.0476 | +7185.5762 | Higher | -1795.2815 | Lower | -8980.8576 | Lower |
| Halstead Effort | 1217971.9427 | **144859.9669 ✅** | 1365547.6813 | +1073111.9757 | Higher | -147575.7387 | Lower | -1220687.7144 | Lower |
| SLOC | 239 | **126 ✅** | 307 | +113 | Higher | -68 | Lower | -181 | Lower |
| PLOC | 218 | **92 ✅** | 276 | +126 | Higher | -58 | Lower | -184 | Lower |
| LLOC | 166 | **58 ✅** | 201 | +108 | Higher | -35 | Lower | -143 | Lower |
| Comments (CLOC) | 0 | **32 ✅** | 2 | -32 | Lower | -2 | Lower | +30 | Higher |
| Blank Lines | 21 | **18 ✅** | 29 | +3 | Higher | -8 | Lower | -11 | Lower |
| Functions | 8 | **4 ✅** | 13 | +4 | Higher | -5 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **3 ✅** | 8 | +1 | Higher | -4 | Lower | -5 | Lower |

## p00292

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.4064 ✅** | 71.0474 | 78.5518 | +10.3591 | Higher | +2.8547 | Higher | -7.5044 | Lower |
| MI (SEI) | 74.5400 | 61.8177 | **74.8248 ✅** | +12.7223 | Higher | -0.2848 | Lower | -13.0072 | Lower |
| Cyclomatic (sum) | **4 ✅** | 6 | 10 | -2 | Lower | -6 | Lower | -4 | Lower |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **707.8164 ✅** | 870.2800 | 2222.5590 | -162.4636 | Lower | -1514.7426 | Lower | -1352.2790 | Lower |
| Halstead Effort | **12167.7016 ✅** | 15304.9246 | 32439.2594 | -3137.2230 | Lower | -20271.5578 | Lower | -17134.3349 | Lower |
| SLOC | 29 | 50 | **22 ✅** | -21 | Lower | +7 | Higher | +28 | Higher |
| PLOC | **18 ✅** | 29 | 20 | -11 | Lower | -2 | Lower | +9 | Higher |
| LLOC | 13 | 13 | **11 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 6 | **12 ✅** | 6 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 6 | 9 | **-4 ✅** | -3 | Lower | +10 | Higher | +13 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00315

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.8183 | **58.2607 ✅** | 53.4044 | -4.4423 | Lower | +0.4139 | Higher | +4.8562 | Higher |
| MI (SEI) | 33.0677 | **43.6759 ✅** | 21.8497 | -10.6083 | Lower | +11.2179 | Higher | +21.8262 | Higher |
| Cyclomatic (sum) | 17 | **16 ✅** | 17 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 16 | 19 | 16 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 3535.1299 | **1572.5298 ✅** | 3541.1523 | +1962.6001 | Higher | -6.0224 | Lower | -1968.6224 | Lower |
| Halstead Effort | 250994.2233 | **55393.6319 ✅** | 251421.8115 | +195600.5915 | Higher | -427.5881 | Lower | -196028.1796 | Lower |
| SLOC | 79 | 79 | 81 | 0 | Same | -2 | Lower | -2 | Lower |
| PLOC | 66 | **52 ✅** | 66 | +14 | Higher | 0 | Same | -14 | Lower |
| LLOC | 45 | **33 ✅** | 45 | +12 | Higher | 0 | Same | -12 | Lower |
| Comments (CLOC) | 13 | **18 ✅** | 5 | -5 | Lower | +8 | Higher | +13 | Higher |
| Blank Lines | 13 | 13 | 15 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p00317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.7858 | **56.1749 ✅** | 32.7384 | -11.3891 | Lower | +12.0474 | Higher | +23.4365 | Higher |
| MI (SEI) | -8.3395 | **38.1044 ✅** | -12.8249 | -46.4439 | Lower | +4.4854 | Higher | +50.9293 | Higher |
| Cyclomatic (sum) | 27 | **16 ✅** | 40 | +11 | Higher | -13 | Lower | -24 | Lower |
| Cognitive (sum) | 56 | **19 ✅** | 71 | +37 | Higher | -15 | Lower | -52 | Lower |
| Halstead Volume | 3899.0078 | **1869.6176 ✅** | 6072.3036 | +2029.3901 | Higher | -2173.2959 | Lower | -4202.6860 | Lower |
| Halstead Effort | 241363.5760 | **62112.8528 ✅** | 575182.0945 | +179250.7233 | Higher | -333818.5185 | Lower | -513069.2417 | Lower |
| SLOC | 116 | **85 ✅** | 176 | +31 | Higher | -60 | Lower | -91 | Lower |
| PLOC | 104 | **59 ✅** | 155 | +45 | Higher | -51 | Lower | -96 | Lower |
| LLOC | 67 | **35 ✅** | 117 | +32 | Higher | -50 | Lower | -82 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 4 | -16 | Lower | -4 | Lower | +12 | Higher |
| Blank Lines | **12 ✅** | 13 | 17 | -1 | Lower | -5 | Lower | -4 | Lower |
| Functions | **2 ✅** | 3 | 8 | -1 | Lower | -6 | Lower | -5 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **3 ✅** | 5 | 11 | -2 | Lower | -8 | Lower | -6 | Lower |

## p00320

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.2184 ✅** | 52.0474 | 52.4739 | +3.1710 | Higher | +2.7445 | Higher | -0.4265 | Lower |
| MI (SEI) | 5.5916 | **29.3952 ✅** | 1.7339 | -23.8036 | Lower | +3.8577 | Higher | +27.6613 | Higher |
| Cyclomatic (sum) | **16 ✅** | 22 | 17 | -6 | Lower | -1 | Lower | +5 | Higher |
| Cognitive (sum) | **17 ✅** | 24 | 20 | -7 | Lower | -3 | Lower | +4 | Higher |
| Halstead Volume | 2937.0714 | **2653.8917 ✅** | 3158.1071 | +283.1797 | Higher | -221.0357 | Lower | -504.2154 | Lower |
| Halstead Effort | 230927.2421 | **99903.7114 ✅** | 269036.5870 | +131023.5307 | Higher | -38109.3449 | Lower | -169132.8756 | Lower |
| SLOC | **78 ✅** | 90 | 89 | -12 | Lower | -11 | Lower | +1 | Higher |
| PLOC | 70 | 70 | 81 | 0 | Same | -11 | Lower | -11 | Lower |
| LLOC | 44 | **38 ✅** | 53 | +6 | Higher | -9 | Lower | -15 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00329

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.3599 | **60.2015 ✅** | 49.6191 | -13.8416 | Lower | -3.2592 | Lower | +10.5825 | Higher |
| MI (SEI) | 6.0598 | **51.4609 ✅** | 8.8365 | -45.4011 | Lower | -2.7767 | Lower | +42.6244 | Higher |
| Cyclomatic (sum) | 24 | **15 ✅** | 21 | +9 | Higher | +3 | Higher | -6 | Lower |
| Cognitive (sum) | 27 | **15 ✅** | 25 | +12 | Higher | +2 | Higher | -10 | Lower |
| Halstead Volume | 3472.4916 | **1890.6918 ✅** | 3089.4941 | +1581.7998 | Higher | +382.9975 | Higher | -1198.8022 | Lower |
| Halstead Effort | 218569.6687 | **90193.0025 ✅** | 190518.8010 | +128376.6662 | Higher | +28050.8677 | Higher | -100325.7985 | Lower |
| SLOC | 114 | **67 ✅** | 101 | +47 | Higher | +13 | Higher | -34 | Lower |
| PLOC | 98 | **49 ✅** | 90 | +49 | Higher | +8 | Higher | -41 | Lower |
| LLOC | 60 | **27 ✅** | 65 | +33 | Higher | -5 | Lower | -38 | Lower |
| Comments (CLOC) | 3 | **22 ✅** | 2 | -19 | Lower | +1 | Higher | +20 | Higher |
| Blank Lines | 14 | 9 | 9 | +5 | Higher | +5 | Higher | 0 | Same |
| Functions | 7 | **3 ✅** | 6 | +4 | Higher | +1 | Higher | -3 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |

## p00336

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.3126 | 68.5127 | 74.3126 | +5.7998 | Higher | 0 | Same | -5.7998 | Lower |
| MI (SEI) | 65.9899 | 59.9579 | 65.9899 | +6.0320 | Higher | 0 | Same | -6.0320 | Lower |
| Cyclomatic (sum) | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 7 | 10 | 7 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 1007.2198 | 1147.7973 | 1007.2198 | -140.5775 | Lower | 0 | Same | +140.5775 | Higher |
| Halstead Effort | 35492.5064 | 43366.7761 | 35492.5064 | -7874.2697 | Lower | 0 | Same | +7874.2697 | Higher |
| SLOC | 39 | 52 | 39 | -13 | Lower | 0 | Same | +13 | Higher |
| PLOC | 23 | 28 | 23 | -5 | Lower | 0 | Same | +5 | Higher |
| LLOC | 18 | 21 | 18 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 9 | **14 ✅** | 9 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00378

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.9408 | **90.6799 ✅** | 78.8867 | -12.7391 | Lower | -0.9459 | Lower | +11.7932 | Higher |
| MI (SEI) | 56.0671 | **98.6609 ✅** | 61.8941 | -42.5939 | Lower | -5.8271 | Lower | +36.7668 | Higher |
| Cyclomatic (sum) | 7 | **3 ✅** | 6 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 9 | **0 ✅** | 9 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 807.0714 | **294.0315 ✅** | 854.4993 | +513.0399 | Higher | -47.4278 | Lower | -560.4677 | Lower |
| Halstead Effort | 23606.8395 | **3849.1400 ✅** | 21789.7312 | +19757.6995 | Higher | +1817.1083 | Higher | -17940.5912 | Lower |
| SLOC | 33 | **22 ✅** | 31 | +11 | Higher | +2 | Higher | -9 | Lower |
| PLOC | 25 | **11 ✅** | 24 | +14 | Higher | +1 | Higher | -13 | Lower |
| LLOC | 14 | **6 ✅** | 20 | +8 | Higher | -6 | Lower | -14 | Lower |
| Comments (CLOC) | 2 | **10 ✅** | 3 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00428

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.8542 | **65.9641 ✅** | 57.2261 | -4.1099 | Lower | +4.6280 | Higher | +8.7380 | Higher |
| MI (SEI) | 38.5136 | **45.9029 ✅** | 9.2008 | -7.3893 | Lower | +29.3128 | Higher | +36.7021 | Higher |
| Cyclomatic (sum) | 16 | **13 ✅** | 23 | +3 | Higher | -7 | Lower | -10 | Lower |
| Cognitive (sum) | 29 | **11 ✅** | 29 | +18 | Higher | 0 | Same | -18 | Lower |
| Halstead Volume | 1763.2976 | **1502.0256 ✅** | 2052.0351 | +261.2719 | Higher | -288.7375 | Lower | -550.0095 | Lower |
| Halstead Effort | 57510.6287 | **46247.8539 ✅** | 104467.2424 | +11262.7748 | Higher | -46956.6137 | Lower | -58219.3885 | Lower |
| SLOC | 61 | **52 ✅** | 70 | +9 | Higher | -9 | Lower | -18 | Lower |
| PLOC | 46 | **38 ✅** | 63 | +8 | Higher | -17 | Lower | -25 | Lower |
| LLOC | 33 | **22 ✅** | 43 | +11 | Higher | -10 | Lower | -21 | Lower |
| Comments (CLOC) | 6 | 6 | 0 | 0 | Same | +6 | Higher | +6 | Higher |
| Blank Lines | 10 | 9 | **7 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.8288 | 69.0296 | **82.7071 ✅** | +10.7992 | Higher | -2.8783 | Lower | -13.6775 | Lower |
| MI (SEI) | **77.2147 ✅** | 62.1355 | 76.5183 | +15.0791 | Higher | +0.6964 | Higher | -14.3827 | Lower |
| Cyclomatic (sum) | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 5 | 13 | 5 | -8 | Lower | 0 | Same | +8 | Higher |
| Halstead Volume | 558.8024 | 780.6987 | **527.3324 ✅** | -221.8963 | Lower | +31.4701 | Higher | +253.3664 | Higher |
| Halstead Effort | 9606.0802 | 16264.5567 | **6403.3214 ✅** | -6658.4765 | Lower | +3202.7588 | Higher | +9861.2353 | Higher |
| SLOC | 34 | 57 | **29 ✅** | -23 | Lower | +5 | Higher | +28 | Higher |
| PLOC | 18 | 33 | 18 | -15 | Lower | 0 | Same | +15 | Higher |
| LLOC | 10 | 16 | **9 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Comments (CLOC) | 10 | **17 ✅** | 6 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00445

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.2997 | 78.0542 | **82.9515 ✅** | +1.2455 | Higher | -3.6518 | Lower | -4.8973 | Lower |
| MI (SEI) | 67.7739 | **75.1894 ✅** | 65.0465 | -7.4156 | Lower | +2.7274 | Higher | +10.1430 | Higher |
| Cyclomatic (sum) | 7 | **5 ✅** | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 11 | **2 ✅** | 9 | +9 | Higher | +2 | Higher | -7 | Lower |
| Halstead Volume | 621.4760 | 657.8627 | **601.3779 ✅** | -36.3867 | Lower | +20.0982 | Higher | +56.4849 | Higher |
| Halstead Effort | 13021.4026 | 12499.3918 | **11070.8200 ✅** | +522.0108 | Higher | +1950.5826 | Higher | +1428.5718 | Higher |
| SLOC | 33 | 36 | **27 ✅** | -3 | Lower | +6 | Higher | +9 | Higher |
| PLOC | 23 | 21 | 21 | +2 | Higher | +2 | Higher | 0 | Same |
| LLOC | 17 | **15 ✅** | 16 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 2 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 6 | 7 | **4 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.7748 | 73.2423 | **73.8866 ✅** | +0.5325 | Higher | -0.1118 | Lower | -0.6443 | Lower |
| MI (SEI) | 60.4004 | 62.2724 | **66.6079 ✅** | -1.8719 | Lower | -6.2075 | Lower | -4.3355 | Lower |
| Cyclomatic (sum) | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 19 | **13 ✅** | 20 | +6 | Higher | -1 | Lower | -7 | Lower |
| Halstead Volume | 895.3468 | **823.0107 ✅** | 1032.4684 | +72.3361 | Higher | -137.1216 | Lower | -209.4577 | Lower |
| Halstead Effort | 28472.0278 | **25474.1417 ✅** | 37168.8618 | +2997.8861 | Higher | -8696.8340 | Lower | -11694.7201 | Lower |
| SLOC | 39 | 42 | **37 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| PLOC | 29 | 31 | **28 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| LLOC | **18 ✅** | 19 | 23 | -1 | Lower | -5 | Lower | -4 | Lower |
| Comments (CLOC) | 6 | 8 | **9 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00453

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.8563 | **71.9089 ✅** | 53.0575 | -21.0525 | Lower | -2.2011 | Lower | +18.8514 | Higher |
| MI (SEI) | 21.4997 | **53.9565 ✅** | 3.0849 | -32.4568 | Lower | +18.4147 | Higher | +50.8715 | Higher |
| Cyclomatic (sum) | 24 | **7 ✅** | 22 | +17 | Higher | +2 | Higher | -15 | Lower |
| Cognitive (sum) | 73 | **9 ✅** | 64 | +64 | Higher | +9 | Higher | -55 | Lower |
| Halstead Volume | 3519.2571 | **1128.6631 ✅** | 3413.0268 | +2390.5940 | Higher | +106.2302 | Higher | -2284.3638 | Lower |
| Halstead Effort | 301696.3117 | **23160.1662 ✅** | 293093.6796 | +278536.1456 | Higher | +8602.6322 | Higher | -269933.5134 | Lower |
| SLOC | 86 | **43 ✅** | 78 | +43 | Higher | +8 | Higher | -35 | Lower |
| PLOC | 73 | **29 ✅** | 71 | +44 | Higher | +2 | Higher | -42 | Lower |
| LLOC | 57 | **24 ✅** | 56 | +33 | Higher | +1 | Higher | -32 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 0 | +2 | Higher | +7 | Higher | +5 | Higher |
| Blank Lines | 8 | 9 | **7 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00465

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.2105 | **52.3902 ✅** | 34.8717 | -3.1798 | Lower | +14.3388 | Higher | +17.5185 | Higher |
| MI (SEI) | 13.4684 | **31.9040 ✅** | -9.5070 | -18.4356 | Lower | +22.9754 | Higher | +41.4110 | Higher |
| Cyclomatic (sum) | 25 | **17 ✅** | 38 | +8 | Higher | -13 | Lower | -21 | Lower |
| Cognitive (sum) | 38 | **15 ✅** | 51 | +23 | Higher | -13 | Lower | -36 | Lower |
| Halstead Volume | **3388.7473 ✅** | 3703.6708 | 5590.1262 | -314.9235 | Lower | -2201.3789 | Lower | -1886.4553 | Lower |
| Halstead Effort | **137970.4264 ✅** | 154913.1556 | 485980.3241 | -16942.7291 | Lower | -348009.8977 | Lower | -331067.1686 | Lower |
| SLOC | 95 | **85 ✅** | 163 | +10 | Higher | -68 | Lower | -78 | Lower |
| PLOC | 77 | **59 ✅** | 138 | +18 | Higher | -61 | Lower | -79 | Lower |
| LLOC | 50 | **44 ✅** | 93 | +6 | Higher | -43 | Lower | -49 | Lower |
| Comments (CLOC) | 4 | **15 ✅** | 4 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 15 | **14 ✅** | 23 | +1 | Higher | -8 | Lower | -9 | Lower |
| Functions | 3 | 3 | 9 | 0 | Same | -6 | Lower | -6 | Lower |
| Closures | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | 2 | 2 | 6 | 0 | Same | -4 | Lower | -4 | Lower |

## p00468

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.4405 | **63.3221 ✅** | 57.5125 | -0.8816 | Lower | +4.9280 | Higher | +5.8095 | Higher |
| MI (SEI) | 40.7767 | **50.0900 ✅** | 9.2068 | -9.3133 | Lower | +31.5699 | Higher | +40.8833 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 19 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 30 | 27 | **20 ✅** | +3 | Higher | +10 | Higher | +7 | Higher |
| Halstead Volume | 1565.1781 | **1529.3548 ✅** | 2123.1720 | +35.8234 | Higher | -557.9938 | Lower | -593.8172 | Lower |
| Halstead Effort | 56617.3090 | **43658.2992 ✅** | 88081.3055 | +12959.0098 | Higher | -31463.9965 | Lower | -44423.0063 | Lower |
| SLOC | 62 | **60 ✅** | 72 | +2 | Higher | -10 | Lower | -12 | Lower |
| PLOC | 46 | **42 ✅** | 63 | +4 | Higher | -17 | Lower | -21 | Lower |
| LLOC | 38 | **33 ✅** | 49 | +5 | Higher | -11 | Lower | -16 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 0 | -6 | Lower | +7 | Higher | +13 | Higher |
| Blank Lines | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 5 | +1 | Higher | -4 | Lower | -5 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00479

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1819 | 68.4776 | **77.0343 ✅** | +5.7043 | Higher | -2.8524 | Lower | -8.5567 | Lower |
| MI (SEI) | 60.5805 | 66.6395 | **66.7258 ✅** | -6.0590 | Lower | -6.1453 | Lower | -0.0863 | Lower |
| Cyclomatic (sum) | 7 | 14 | **6 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Cognitive (sum) | **8 ✅** | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | 988.1571 | **744.1398 ✅** | 1004.2264 | +244.0173 | Higher | -16.0693 | Lower | -260.0866 | Lower |
| Halstead Effort | 27066.9125 | **23420.8212 ✅** | 27189.4304 | +3646.0913 | Higher | -122.5179 | Lower | -3768.6092 | Lower |
| SLOC | 39 | 55 | **33 ✅** | -16 | Lower | +6 | Higher | +22 | Higher |
| PLOC | 26 | 32 | **21 ✅** | -6 | Lower | +5 | Higher | +11 | Higher |
| LLOC | 15 | 15 | **13 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 6 | **23 ✅** | 6 | -17 | Lower | 0 | Same | +17 | Higher |
| Blank Lines | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

## p00492

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.6027 | **65.5856 ✅** | 58.4454 | -7.9829 | Lower | -0.8427 | Lower | +7.1402 | Higher |
| MI (SEI) | 27.4556 | **58.6240 ✅** | 29.1967 | -31.1684 | Lower | -1.7411 | Lower | +29.4273 | Higher |
| Cyclomatic (sum) | 18 | **12 ✅** | 18 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 26 | **23 ✅** | 26 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 2490.2904 | **1340.3910 ✅** | 2550.7343 | +1149.8994 | Higher | -60.4439 | Lower | -1210.3433 | Lower |
| Halstead Effort | 158756.0115 | **68908.2828 ✅** | 150941.4268 | +89847.7288 | Higher | +7814.5848 | Higher | -82033.1440 | Lower |
| SLOC | 69 | **56 ✅** | 65 | +13 | Higher | +4 | Higher | -9 | Lower |
| PLOC | 57 | **34 ✅** | 54 | +23 | Higher | +3 | Higher | -20 | Lower |
| LLOC | 45 | **22 ✅** | 47 | +23 | Higher | -2 | Lower | -25 | Lower |
| Comments (CLOC) | 4 | **18 ✅** | 4 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.9236 | **73.4584 ✅** | 64.0412 | -10.5348 | Lower | -1.1177 | Lower | +9.4172 | Higher |
| MI (SEI) | 16.4023 | **68.5665 ✅** | 18.0148 | -52.1642 | Lower | -1.6125 | Lower | +50.5517 | Higher |
| Cyclomatic (sum) | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 15 | **10 ✅** | 15 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1725.8446 | **814.2399 ✅** | 1725.8446 | +911.6046 | Higher | 0 | Same | -911.6046 | Lower |
| Halstead Effort | 69896.7049 | **25526.4218 ✅** | 69896.7049 | +44370.2831 | Higher | 0 | Same | -44370.2831 | Lower |
| SLOC | 60 | **41 ✅** | 56 | +19 | Higher | +4 | Higher | -15 | Lower |
| PLOC | 47 | **25 ✅** | 47 | +22 | Higher | 0 | Same | -22 | Lower |
| LLOC | 35 | **16 ✅** | 35 | +19 | Higher | 0 | Same | -19 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 0 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 13 | **7 ✅** | 9 | +6 | Higher | +4 | Higher | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p00507

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.8694 | 70.6379 | **72.3637 ✅** | -4.7685 | Lower | -6.4943 | Lower | -1.7258 | Lower |
| MI (SEI) | 50.5618 | **66.1171 ✅** | 41.8414 | -15.5553 | Lower | +8.7204 | Higher | +24.2756 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 10 | **9 ✅** | 20 | +1 | Higher | -10 | Lower | -11 | Lower |
| Halstead Volume | 1312.1877 | **775.2393 ✅** | 1134.4851 | +536.9485 | Higher | +177.7027 | Higher | -359.2458 | Lower |
| Halstead Effort | 38914.5678 | **16150.8185 ✅** | 36303.5222 | +22763.7492 | Higher | +2611.0456 | Higher | -20152.7037 | Lower |
| SLOC | 57 | 51 | **40 ✅** | +6 | Higher | +17 | Higher | +11 | Higher |
| PLOC | 39 | **30 ✅** | 32 | +9 | Higher | +7 | Higher | -2 | Lower |
| LLOC | 27 | **17 ✅** | 21 | +10 | Higher | +6 | Higher | -4 | Lower |
| Comments (CLOC) | 10 | **17 ✅** | 1 | -7 | Lower | +9 | Higher | +16 | Higher |
| Blank Lines | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00514

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.0879 | **71.0461 ✅** | 52.1762 | -19.9582 | Lower | -1.0882 | Lower | +18.8699 | Higher |
| MI (SEI) | 7.9741 | **65.9948 ✅** | 15.6660 | -58.0207 | Lower | -7.6919 | Lower | +50.3288 | Higher |
| Cyclomatic (sum) | 21 | **8 ✅** | 21 | +13 | Higher | 0 | Same | -13 | Lower |
| Cognitive (sum) | 19 | **5 ✅** | 18 | +14 | Higher | +1 | Higher | -13 | Lower |
| Halstead Volume | 2641.7210 | **966.1985 ✅** | 2706.0763 | +1675.5225 | Higher | -64.3554 | Lower | -1739.8778 | Lower |
| Halstead Effort | 147614.2138 | **30918.3524 ✅** | 175503.2928 | +116695.8614 | Higher | -27889.0790 | Lower | -144584.9404 | Lower |
| SLOC | 97 | **47 ✅** | 90 | +50 | Higher | +7 | Higher | -43 | Lower |
| PLOC | 82 | **29 ✅** | 79 | +53 | Higher | +3 | Higher | -50 | Lower |
| LLOC | 55 | **18 ✅** | 53 | +37 | Higher | +2 | Higher | -35 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 3 | -14 | Lower | -2 | Lower | +12 | Higher |
| Blank Lines | 15 | 9 | 9 | +6 | Higher | +6 | Higher | 0 | Same |
| Functions | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p00518

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.3463 | **72.5240 ✅** | 62.3942 | -13.1777 | Lower | -3.0479 | Lower | +10.1297 | Higher |
| MI (SEI) | 36.9844 | **68.9399 ✅** | 29.6848 | -31.9555 | Lower | +7.2996 | Higher | +39.2550 | Higher |
| Cyclomatic (sum) | 17 | **10 ✅** | 15 | +7 | Higher | +2 | Higher | -5 | Lower |
| Cognitive (sum) | 27 | **5 ✅** | 27 | +22 | Higher | 0 | Same | -22 | Lower |
| Halstead Volume | 1947.9881 | **944.9282 ✅** | 1661.2208 | +1003.0599 | Higher | +286.7673 | Higher | -716.2926 | Lower |
| Halstead Effort | 81133.7047 | **24730.5423 ✅** | 73722.9642 | +56403.1625 | Higher | +7410.7405 | Higher | -48992.4219 | Lower |
| SLOC | 68 | **42 ✅** | 61 | +26 | Higher | +7 | Higher | -19 | Lower |
| PLOC | 51 | **26 ✅** | 49 | +25 | Higher | +2 | Higher | -23 | Lower |
| LLOC | 42 | **17 ✅** | 41 | +25 | Higher | +1 | Higher | -24 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 2 | -6 | Lower | +6 | Higher | +12 | Higher |
| Blank Lines | 13 | **8 ✅** | 12 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00519

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **49.9314 ✅** | 46.2418 | 36.7314 | +3.6896 | Higher | +13.2000 | Higher | +9.5104 | Higher |
| MI (SEI) | 8.7257 | **13.9025 ✅** | -19.7559 | -5.1768 | Lower | +28.4816 | Higher | +33.6584 | Higher |
| Cyclomatic (sum) | **16 ✅** | 25 | 29 | -9 | Lower | -13 | Lower | -4 | Lower |
| Cognitive (sum) | 21 | **17 ✅** | 29 | +4 | Higher | -8 | Lower | -12 | Lower |
| Halstead Volume | 3519.8252 | **2682.1846 ✅** | 5499.3627 | +837.6406 | Higher | -1979.5375 | Lower | -2817.1781 | Lower |
| Halstead Effort | 148596.3959 | **111525.2373 ✅** | 435241.9367 | +37071.1585 | Higher | -286645.5408 | Lower | -323716.6994 | Lower |
| SLOC | **102 ✅** | 123 | 166 | -21 | Lower | -64 | Lower | -43 | Lower |
| PLOC | **89 ✅** | 95 | 145 | -6 | Lower | -56 | Lower | -50 | Lower |
| LLOC | 48 | **28 ✅** | 93 | +20 | Higher | -45 | Lower | -65 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 0 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | **13 ✅** | 19 | 21 | -6 | Lower | -8 | Lower | -2 | Lower |
| Functions | **2 ✅** | 5 | 8 | -3 | Lower | -6 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 5 | 3 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00528

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.3521 | **51.5342 ✅** | 24.0184 | -11.1821 | Lower | +16.3337 | Higher | +27.5158 | Higher |
| MI (SEI) | -7.3261 | **24.7568 ✅** | -25.5359 | -32.0830 | Lower | +18.2098 | Higher | +50.2928 | Higher |
| Cyclomatic (sum) | 33 | **21 ✅** | 45 | +12 | Higher | -12 | Lower | -24 | Lower |
| Cognitive (sum) | 23 | **15 ✅** | 42 | +8 | Higher | -19 | Lower | -27 | Lower |
| Halstead Volume | 5038.1206 | **2503.9851 ✅** | 9058.3041 | +2534.1355 | Higher | -4020.1835 | Lower | -6554.3190 | Lower |
| Halstead Effort | 305260.5536 | **88891.4715 ✅** | 1075857.2309 | +216369.0821 | Higher | -770596.6773 | Lower | -986965.7594 | Lower |
| SLOC | 129 | **96 ✅** | 247 | +33 | Higher | -118 | Lower | -151 | Lower |
| PLOC | 107 | **76 ✅** | 211 | +31 | Higher | -104 | Lower | -135 | Lower |
| LLOC | 66 | **29 ✅** | 194 | +37 | Higher | -128 | Lower | -165 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 5 | -9 | Lower | -4 | Lower | +5 | Higher |
| Blank Lines | 22 | **17 ✅** | 34 | +5 | Higher | -12 | Lower | -17 | Lower |
| Functions | **4 ✅** | 5 | 11 | -1 | Lower | -7 | Lower | -6 | Lower |
| Closures | 8 | **0 ✅** | 2 | +8 | Higher | +6 | Higher | -2 | Lower |
| Exits | **4 ✅** | 5 | 8 | -1 | Lower | -4 | Lower | -3 | Lower |

## p00534

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.3663 | **75.0482 ✅** | 68.0206 | -7.6820 | Lower | -0.6543 | Lower | +7.0276 | Higher |
| MI (SEI) | 53.2157 | **62.9418 ✅** | 49.8868 | -9.7261 | Lower | +3.3290 | Higher | +13.0550 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1647.4271 | **1026.9765 ✅** | 1768.6336 | +620.4505 | Higher | -121.2065 | Lower | -741.6571 | Lower |
| Halstead Effort | 64029.9991 | **37328.3643 ✅** | 67650.2352 | +26701.6348 | Higher | -3620.2361 | Lower | -30321.8710 | Lower |
| SLOC | 49 | **36 ✅** | 46 | +13 | Higher | +3 | Higher | -10 | Lower |
| PLOC | 32 | **24 ✅** | 32 | +8 | Higher | 0 | Same | -8 | Lower |
| LLOC | 28 | **17 ✅** | 28 | +11 | Higher | 0 | Same | -11 | Lower |
| Comments (CLOC) | **9 ✅** | 6 | 6 | +3 | Higher | +3 | Higher | 0 | Same |
| Blank Lines | 10 | **6 ✅** | 8 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00543

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.6412 | **83.0399 ✅** | 69.6564 | -13.3987 | Lower | -0.0152 | Lower | +13.3835 | Higher |
| MI (SEI) | 56.0942 | **85.9984 ✅** | 53.9543 | -29.9042 | Lower | +2.1398 | Higher | +32.0440 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 10 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 15 | **4 ✅** | 9 | +11 | Higher | +6 | Higher | -5 | Lower |
| Halstead Volume | 1134.1283 | **582.3703 ✅** | 1155.3545 | +551.7580 | Higher | -21.2262 | Lower | -572.9842 | Lower |
| Halstead Effort | 37687.9542 | **17128.5377 ✅** | 39181.5868 | +20559.4165 | Higher | -1493.6326 | Lower | -22053.0491 | Lower |
| SLOC | 46 | **26 ✅** | 47 | +20 | Higher | -1 | Lower | -21 | Lower |
| PLOC | 33 | **17 ✅** | 33 | +16 | Higher | 0 | Same | -16 | Lower |
| LLOC | 22 | **10 ✅** | 25 | +12 | Higher | -3 | Lower | -15 | Lower |
| Comments (CLOC) | 8 | **10 ✅** | 7 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00554

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.7487 | 65.2081 | **66.1077 ✅** | -2.4594 | Lower | -3.3589 | Lower | -0.8996 | Lower |
| MI (SEI) | 39.2228 | **53.8795 ✅** | 41.7227 | -14.6567 | Lower | -2.4998 | Lower | +12.1569 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 12 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 1684.4130 | **1411.5594 ✅** | 1527.2011 | +272.8536 | Higher | +157.2119 | Higher | -115.6417 | Lower |
| Halstead Effort | 72741.6853 | **37983.7798 ✅** | 63496.3217 | +34757.9055 | Higher | +9245.3636 | Higher | -25512.5420 | Lower |
| SLOC | 62 | 58 | **52 ✅** | +4 | Higher | +10 | Higher | +6 | Higher |
| PLOC | 44 | **33 ✅** | 40 | +11 | Higher | +4 | Higher | -7 | Lower |
| LLOC | 40 | **25 ✅** | 36 | +15 | Higher | +4 | Higher | -11 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 4 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 12 | 13 | **9 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00571

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3940 | **71.1134 ✅** | 57.5630 | -5.7194 | Lower | +7.8310 | Higher | +13.5504 | Higher |
| MI (SEI) | 45.4946 | **65.3241 ✅** | 23.6191 | -19.8295 | Lower | +21.8755 | Higher | +41.7050 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 12 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1659.8363 | **1065.9822 ✅** | 2324.8490 | +593.8540 | Higher | -665.0128 | Lower | -1258.8668 | Lower |
| Halstead Effort | 48494.1350 | **20653.4057 ✅** | 101712.1453 | +27840.7293 | Higher | -53218.0104 | Lower | -81058.7397 | Lower |
| SLOC | 56 | **46 ✅** | 77 | +10 | Higher | -21 | Lower | -31 | Lower |
| PLOC | 43 | **27 ✅** | 62 | +16 | Higher | -19 | Lower | -35 | Lower |
| LLOC | 28 | **21 ✅** | 42 | +7 | Higher | -14 | Lower | -21 | Lower |
| Comments (CLOC) | 7 | **14 ✅** | 3 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | 9 | 9 | 13 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |

## p00594

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.0125 ✅** | 58.6920 | 61.6566 | +4.3205 | Higher | +1.3559 | Higher | -2.9646 | Lower |
| MI (SEI) | 43.4390 | 38.7231 | **43.6204 ✅** | +4.7159 | Higher | -0.1815 | Lower | -4.8973 | Lower |
| Cyclomatic (sum) | 16 | 16 | 16 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 22 | 22 | **19 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | **1411.1872 ✅** | 1774.3505 | 1577.1398 | -363.1633 | Lower | -165.9525 | Lower | +197.2107 | Higher |
| Halstead Effort | **70830.7445 ✅** | 85760.2742 | 73031.7473 | -14929.5297 | Lower | -2201.0029 | Lower | +12728.5269 | Higher |
| SLOC | **61 ✅** | 74 | 64 | -13 | Lower | -3 | Lower | +10 | Higher |
| PLOC | **47 ✅** | 57 | 56 | -10 | Lower | -9 | Lower | +1 | Higher |
| LLOC | **33 ✅** | 43 | 39 | -10 | Lower | -6 | Lower | +4 | Higher |
| Comments (CLOC) | 8 | **11 ✅** | 10 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | 9 | 12 | **7 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00629

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.9729 | 56.8783 | **56.9970 ✅** | +0.0946 | Higher | -0.0242 | Lower | -0.1188 | Lower |
| MI (SEI) | 30.4734 | **34.8586 ✅** | 18.0735 | -4.3852 | Lower | +12.3999 | Higher | +16.7851 | Higher |
| Cyclomatic (sum) | 17 | **16 ✅** | 21 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 20 | **17 ✅** | 19 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 1927.2757 | **1826.5263 ✅** | 2685.3425 | +100.7494 | Higher | -758.0668 | Lower | -858.8162 | Lower |
| Halstead Effort | 71422.5718 | **53330.3214 ✅** | 122112.4185 | +18092.2505 | Higher | -50689.8467 | Lower | -68782.0971 | Lower |
| SLOC | 79 | 82 | **67 ✅** | -3 | Lower | +12 | Higher | +15 | Higher |
| PLOC | 65 | 59 | **58 ✅** | +6 | Higher | +7 | Higher | +1 | Higher |
| LLOC | 36 | **26 ✅** | 41 | +10 | Higher | -5 | Lower | -15 | Lower |
| Comments (CLOC) | 7 | **11 ✅** | 1 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 13 | 14 | **8 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 5 | **0 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00641

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.6547 | **52.9989 ✅** | 47.2174 | -5.3442 | Lower | +0.4373 | Higher | +5.7815 | Higher |
| MI (SEI) | 8.1745 | **37.0139 ✅** | 9.3184 | -28.8394 | Lower | -1.1439 | Lower | +27.6956 | Higher |
| Cyclomatic (sum) | 22 | **14 ✅** | 23 | +8 | Higher | -1 | Lower | -9 | Lower |
| Cognitive (sum) | 14 | **11 ✅** | 15 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | 3709.9376 | **2493.1627 ✅** | 3440.0372 | +1216.7750 | Higher | +269.9005 | Higher | -946.8745 | Lower |
| Halstead Effort | 218356.3292 | **107148.0137 ✅** | 264719.0504 | +111208.3155 | Higher | -46362.7212 | Lower | -157571.0366 | Lower |
| SLOC | 106 | **97 ✅** | 110 | +9 | Higher | -4 | Lower | -13 | Lower |
| PLOC | 87 | **71 ✅** | 92 | +16 | Higher | -5 | Lower | -21 | Lower |
| LLOC | 57 | **45 ✅** | 67 | +12 | Higher | -10 | Lower | -22 | Lower |
| Comments (CLOC) | 3 | **24 ✅** | 4 | -21 | Lower | -1 | Lower | +20 | Higher |
| Blank Lines | 17 | **14 ✅** | 16 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |

## p00649

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.1828 | **57.4761 ✅** | 46.2701 | -10.2933 | Lower | +0.9127 | Higher | +11.2060 | Higher |
| MI (SEI) | 7.5043 | **32.7318 ✅** | 7.9050 | -25.2275 | Lower | -0.4007 | Lower | +24.8268 | Higher |
| Cyclomatic (sum) | 26 | **21 ✅** | 25 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 31 | **16 ✅** | 29 | +15 | Higher | +2 | Higher | -13 | Lower |
| Halstead Volume | 2788.8061 | **2044.2297 ✅** | 3380.1288 | +744.5764 | Higher | -591.3227 | Lower | -1335.8991 | Lower |
| Halstead Effort | 111916.0013 | **68560.3203 ✅** | 228619.6196 | +43355.6810 | Higher | -116703.6183 | Lower | -160059.2993 | Lower |
| SLOC | 113 | **71 ✅** | 114 | +42 | Higher | -1 | Lower | -43 | Lower |
| PLOC | 95 | **56 ✅** | 93 | +39 | Higher | +2 | Higher | -37 | Lower |
| LLOC | 53 | **23 ✅** | 71 | +30 | Higher | -18 | Lower | -48 | Lower |
| Comments (CLOC) | 3 | **7 ✅** | 4 | -4 | Lower | -1 | Lower | +3 | Higher |
| Blank Lines | 16 | **10 ✅** | 19 | +6 | Higher | -3 | Lower | -9 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p00653

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.4899 | **62.6816 ✅** | 43.7423 | -18.1917 | Lower | +0.7476 | Higher | +18.9393 | Higher |
| MI (SEI) | 1.6025 | **53.5405 ✅** | 0.4892 | -51.9380 | Lower | +1.1134 | Higher | +53.0514 | Higher |
| Cyclomatic (sum) | 27 | **15 ✅** | 28 | +12 | Higher | -1 | Lower | -13 | Lower |
| Cognitive (sum) | 27 | **10 ✅** | 30 | +17 | Higher | -3 | Lower | -20 | Lower |
| Halstead Volume | 4869.8848 | **1571.9078 ✅** | 4947.0306 | +3297.9770 | Higher | -77.1458 | Lower | -3375.1228 | Lower |
| Halstead Effort | 431282.9642 | **56195.7048 ✅** | 432617.8259 | +375087.2594 | Higher | -1334.8617 | Lower | -376422.1211 | Lower |
| SLOC | 110 | **61 ✅** | 113 | +49 | Higher | -3 | Lower | -52 | Lower |
| PLOC | 89 | **40 ✅** | 91 | +49 | Higher | -2 | Lower | -51 | Lower |
| LLOC | 75 | **21 ✅** | 80 | +54 | Higher | -5 | Lower | -59 | Lower |
| Comments (CLOC) | 2 | **18 ✅** | 2 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 19 | **10 ✅** | 20 | +9 | Higher | -1 | Lower | -10 | Lower |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **3 ✅** | 7 | +4 | Higher | 0 | Same | -4 | Lower |

## p00655

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.8174 | **56.7020 ✅** | 29.7807 | -18.8846 | Lower | +8.0367 | Higher | +26.9213 | Higher |
| MI (SEI) | -7.9280 | **40.8222 ✅** | -16.9517 | -48.7502 | Lower | +9.0237 | Higher | +57.7739 | Higher |
| Cyclomatic (sum) | 40 | **21 ✅** | 45 | +19 | Higher | -5 | Lower | -24 | Lower |
| Cognitive (sum) | 30 | **15 ✅** | 41 | +15 | Higher | -11 | Lower | -26 | Lower |
| Halstead Volume | 4462.6356 | **1573.6168 ✅** | 6999.7933 | +2889.0188 | Higher | -2537.1577 | Lower | -5426.1765 | Lower |
| Halstead Effort | 255157.7517 | **48152.6734 ✅** | 933212.4422 | +207005.0783 | Higher | -678054.6904 | Lower | -885059.7687 | Lower |
| SLOC | 142 | **81 ✅** | 188 | +61 | Higher | -46 | Lower | -107 | Lower |
| PLOC | 128 | **63 ✅** | 171 | +65 | Higher | -43 | Lower | -108 | Lower |
| LLOC | 63 | **25 ✅** | 117 | +38 | Higher | -54 | Lower | -92 | Lower |
| Comments (CLOC) | 2 | **17 ✅** | 4 | -15 | Lower | -2 | Lower | +13 | Higher |
| Blank Lines | 14 | **11 ✅** | 15 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 8 | **6 ✅** | 10 | +2 | Higher | -2 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -3.0127 | **42.0249 ✅** | -7.6843 | -45.0376 | Lower | +4.6716 | Higher | +49.7092 | Higher |
| MI (SEI) | -66.8107 | **17.5490 ✅** | -64.3540 | -84.3598 | Lower | -2.4568 | Lower | +81.9030 | Higher |
| Cyclomatic (sum) | 130 | **31 ✅** | 143 | +99 | Higher | -13 | Lower | -112 | Lower |
| Cognitive (sum) | 176 | **18 ✅** | 187 | +158 | Higher | -11 | Lower | -169 | Lower |
| Halstead Volume | 13604.2840 | **3162.0196 ✅** | 13344.1126 | +10442.2645 | Higher | +260.1714 | Higher | -10182.0931 | Lower |
| Halstead Effort | 2017855.4270 | **145594.0613 ✅** | 1943639.0259 | +1872261.3657 | Higher | +74216.4011 | Higher | -1798044.9647 | Lower |
| SLOC | 344 | **139 ✅** | 384 | +205 | Higher | -40 | Lower | -245 | Lower |
| PLOC | 324 | **105 ✅** | 365 | +219 | Higher | -41 | Lower | -260 | Lower |
| LLOC | 223 | **28 ✅** | 262 | +195 | Higher | -39 | Lower | -234 | Lower |
| Comments (CLOC) | 0 | **23 ✅** | 4 | -23 | Lower | -4 | Lower | +19 | Higher |
| Blank Lines | 20 | 16 | **15 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Functions | 12 | **8 ✅** | 10 | +4 | Higher | +2 | Higher | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 23 | **4 ✅** | 27 | +19 | Higher | -4 | Lower | -23 | Lower |

## p00680

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 17.0466 | **47.4149 ✅** | 14.9010 | -30.3683 | Lower | +2.1456 | Higher | +32.5139 | Higher |
| MI (SEI) | -33.8732 | **20.2285 ✅** | -40.4185 | -54.1017 | Lower | +6.5453 | Higher | +60.6470 | Higher |
| Cyclomatic (sum) | 60 | **22 ✅** | 62 | +38 | Higher | -2 | Lower | -40 | Lower |
| Cognitive (sum) | 81 | **23 ✅** | 85 | +58 | Higher | -4 | Lower | -62 | Lower |
| Halstead Volume | 11293.6148 | **3365.3639 ✅** | 11005.9392 | +7928.2509 | Higher | +287.6756 | Higher | -7640.5753 | Lower |
| Halstead Effort | 1286702.0689 | **140765.9987 ✅** | 1236945.2745 | +1145936.0702 | Higher | +49756.7945 | Higher | -1096179.2757 | Lower |
| SLOC | 286 | **111 ✅** | 320 | +175 | Higher | -34 | Lower | -209 | Lower |
| PLOC | 254 | **93 ✅** | 280 | +161 | Higher | -26 | Lower | -187 | Lower |
| LLOC | 166 | **35 ✅** | 188 | +131 | Higher | -22 | Lower | -153 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 3 | -7 | Lower | +3 | Higher | +10 | Higher |
| Blank Lines | 30 | **13 ✅** | 38 | +17 | Higher | -8 | Lower | -25 | Lower |
| Functions | 14 | **6 ✅** | 15 | +8 | Higher | -1 | Lower | -9 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 12 | **7 ✅** | 13 | +5 | Higher | -1 | Lower | -6 | Lower |

## p00690

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.9855 | **50.5487 ✅** | 47.7650 | -1.5631 | Lower | +1.2205 | Higher | +2.7837 | Higher |
| MI (SEI) | -2.4842 | **27.7460 ✅** | 6.7202 | -30.2302 | Lower | -9.2044 | Lower | +21.0257 | Higher |
| Cyclomatic (sum) | 25 | **17 ✅** | 27 | +8 | Higher | -2 | Lower | -10 | Lower |
| Cognitive (sum) | 38 | **22 ✅** | 38 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 3424.9814 | **2990.5820 ✅** | 3282.1060 | +434.3994 | Higher | +142.8754 | Higher | -291.5240 | Lower |
| Halstead Effort | 221947.8076 | **120179.6662 ✅** | 206936.7837 | +101768.1414 | Higher | +15011.0239 | Higher | -86757.1176 | Lower |
| SLOC | **96 ✅** | 102 | 102 | -6 | Lower | -6 | Lower | 0 | Same |
| PLOC | 87 | **71 ✅** | 88 | +16 | Higher | -1 | Lower | -17 | Lower |
| LLOC | 63 | **49 ✅** | 68 | +14 | Higher | -5 | Lower | -19 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 2 | -16 | Lower | -2 | Lower | +14 | Higher |
| Blank Lines | **9 ✅** | 18 | 12 | -9 | Lower | -3 | Lower | +6 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00692

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8826 | **72.2200 ✅** | 49.0165 | -12.3374 | Lower | +10.8660 | Higher | +23.2034 | Higher |
| MI (SEI) | 13.1351 | **63.6630 ✅** | 27.3835 | -50.5279 | Lower | -14.2484 | Lower | +36.2795 | Higher |
| Cyclomatic (sum) | 24 | **8 ✅** | 29 | +16 | Higher | -5 | Lower | -21 | Lower |
| Cognitive (sum) | 18 | **11 ✅** | 28 | +7 | Higher | -10 | Lower | -17 | Lower |
| Halstead Volume | 2643.8195 | **824.3742 ✅** | 3369.7909 | +1819.4453 | Higher | -725.9714 | Lower | -2545.4167 | Lower |
| Halstead Effort | 156855.0262 | **16935.5129 ✅** | 210401.3200 | +139919.5133 | Higher | -53546.2939 | Lower | -193465.8071 | Lower |
| SLOC | 54 | **46 ✅** | 91 | +8 | Higher | -37 | Lower | -45 | Lower |
| PLOC | 47 | **30 ✅** | 66 | +17 | Higher | -19 | Lower | -36 | Lower |
| LLOC | 35 | **17 ✅** | 44 | +18 | Higher | -9 | Lower | -27 | Lower |
| Comments (CLOC) | 0 | 11 | **15 ✅** | -11 | Lower | -15 | Lower | -4 | Lower |
| Blank Lines | **7 ✅** | 8 | 10 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00723

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0859 | **66.7035 ✅** | 51.3818 | -13.6176 | Lower | +1.7040 | Higher | +15.3217 | Higher |
| MI (SEI) | 2.5150 | **47.2792 ✅** | 0.5657 | -44.7642 | Lower | +1.9493 | Higher | +46.7136 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 21 | +7 | Higher | -5 | Lower | -12 | Lower |
| Cognitive (sum) | 28 | **6 ✅** | 29 | +22 | Higher | -1 | Lower | -23 | Lower |
| Halstead Volume | 3149.7606 | **1168.2782 ✅** | 2753.2511 | +1981.4824 | Higher | +396.5095 | Higher | -1584.9728 | Lower |
| Halstead Effort | 151458.4886 | **26091.5472 ✅** | 137505.2248 | +125366.9415 | Higher | +13953.2639 | Higher | -111413.6776 | Lower |
| SLOC | 87 | **57 ✅** | 94 | +30 | Higher | -7 | Lower | -37 | Lower |
| PLOC | 70 | **37 ✅** | 83 | +33 | Higher | -13 | Lower | -46 | Lower |
| LLOC | 50 | **21 ✅** | 45 | +29 | Higher | +5 | Higher | -24 | Lower |
| Comments (CLOC) | 0 | **7 ✅** | 0 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 17 | 13 | **11 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Functions | 4 | **3 ✅** | 6 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 4 | 3 | -3 | Lower | -2 | Lower | +1 | Higher |

## p00740

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.4301 | **74.5943 ✅** | 60.8254 | -8.1642 | Lower | +5.6047 | Higher | +13.7689 | Higher |
| MI (SEI) | 40.1145 | **68.7193 ✅** | 22.9172 | -28.6048 | Lower | +17.1973 | Higher | +45.8021 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 15 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 25 | **6 ✅** | 16 | +19 | Higher | +9 | Higher | -10 | Lower |
| Halstead Volume | 1277.6181 | **829.2698 ✅** | 1601.2991 | +448.3483 | Higher | -323.6810 | Lower | -772.0293 | Lower |
| Halstead Effort | 49827.1052 | **20623.5798 ✅** | 53376.6360 | +29203.5254 | Higher | -3549.5308 | Lower | -32753.0562 | Lower |
| SLOC | 51 | **38 ✅** | 68 | +13 | Higher | -17 | Lower | -30 | Lower |
| PLOC | 44 | **28 ✅** | 58 | +16 | Higher | -14 | Lower | -30 | Lower |
| LLOC | 32 | **13 ✅** | 37 | +19 | Higher | -5 | Lower | -24 | Lower |
| Comments (CLOC) | 3 | **10 ✅** | 1 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 7 | **6 ✅** | 10 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | **1 ✅** | 2 | 5 | -1 | Lower | -4 | Lower | -3 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00749

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.7199 | **67.9592 ✅** | 22.9563 | -43.2393 | Lower | +1.7636 | Higher | +45.0029 | Higher |
| MI (SEI) | -34.6412 | **59.4637 ✅** | -28.2928 | -94.1049 | Lower | -6.3483 | Lower | +87.7566 | Higher |
| Cyclomatic (sum) | 53 | **13 ✅** | 55 | +40 | Higher | -2 | Lower | -42 | Lower |
| Cognitive (sum) | 82 | **11 ✅** | 78 | +71 | Higher | +4 | Higher | -67 | Lower |
| Halstead Volume | 7639.9270 | **1231.5459 ✅** | 8228.1400 | +6408.3811 | Higher | -588.2130 | Lower | -6996.5940 | Lower |
| Halstead Effort | 739910.3230 | **31034.9573 ✅** | 827280.7019 | +708875.3657 | Higher | -87370.3789 | Lower | -796245.7446 | Lower |
| SLOC | 223 | **49 ✅** | 236 | +174 | Higher | -13 | Lower | -187 | Lower |
| PLOC | 200 | **33 ✅** | 205 | +167 | Higher | -5 | Lower | -172 | Lower |
| LLOC | 136 | **17 ✅** | 138 | +119 | Higher | -2 | Lower | -121 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 3 | -13 | Lower | -3 | Lower | +10 | Higher |
| Blank Lines | 23 | **7 ✅** | 29 | +16 | Higher | -6 | Lower | -22 | Lower |
| Functions | 7 | **2 ✅** | 9 | +5 | Higher | -2 | Lower | -7 | Lower |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 13 | **1 ✅** | 13 | +12 | Higher | 0 | Same | -12 | Lower |

## p00762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.7717 | **69.8981 ✅** | 38.9680 | -30.1263 | Lower | +0.8037 | Higher | +30.9300 | Higher |
| MI (SEI) | 9.0937 | **59.7294 ✅** | 4.1333 | -50.6358 | Lower | +4.9603 | Higher | +55.5961 | Higher |
| Cyclomatic (sum) | 30 | **8 ✅** | 30 | +22 | Higher | 0 | Same | -22 | Lower |
| Cognitive (sum) | 20 | **7 ✅** | 16 | +13 | Higher | +4 | Higher | -9 | Lower |
| Halstead Volume | 5097.4612 | **1128.4003 ✅** | 4997.6393 | +3969.0609 | Higher | +99.8219 | Higher | -3869.2389 | Lower |
| Halstead Effort | 443928.9003 | **28482.3807 ✅** | 440661.4088 | +415446.5196 | Higher | +3267.4916 | Higher | -412179.0280 | Lower |
| SLOC | 139 | **48 ✅** | 147 | +91 | Higher | -8 | Lower | -99 | Lower |
| PLOC | 112 | **30 ✅** | 119 | +82 | Higher | -7 | Lower | -89 | Lower |
| LLOC | 94 | **19 ✅** | 93 | +75 | Higher | +1 | Higher | -74 | Lower |
| Comments (CLOC) | **15 ✅** | 11 | 11 | +4 | Higher | +4 | Higher | 0 | Same |
| Blank Lines | 23 | **9 ✅** | 24 | +14 | Higher | -1 | Lower | -15 | Lower |
| Functions | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 5 | **0 ✅** | 5 | +5 | Higher | 0 | Same | -5 | Lower |

## p00792

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1003 | **62.0374 ✅** | 51.6811 | -6.9370 | Lower | +3.4192 | Higher | +10.3563 | Higher |
| MI (SEI) | 14.5653 | **49.6897 ✅** | 17.5064 | -35.1244 | Lower | -2.9411 | Lower | +32.1833 | Higher |
| Cyclomatic (sum) | 19 | **11 ✅** | 23 | +8 | Higher | -4 | Lower | -12 | Lower |
| Cognitive (sum) | 31 | **12 ✅** | 45 | +19 | Higher | -14 | Lower | -33 | Lower |
| Halstead Volume | 2852.9540 | **1446.5567 ✅** | 3027.8658 | +1406.3973 | Higher | -174.9118 | Lower | -1581.3091 | Lower |
| Halstead Effort | 187093.7218 | **42379.5915 ✅** | 201316.1512 | +144714.1303 | Higher | -14222.4294 | Lower | -158936.5597 | Lower |
| SLOC | 76 | **69 ✅** | 87 | +7 | Higher | -11 | Lower | -18 | Lower |
| PLOC | 63 | **48 ✅** | 71 | +15 | Higher | -8 | Lower | -23 | Lower |
| LLOC | 58 | **35 ✅** | 62 | +23 | Higher | -4 | Lower | -27 | Lower |
| Comments (CLOC) | 1 | **17 ✅** | 4 | -16 | Lower | -3 | Lower | +13 | Higher |
| Blank Lines | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00799

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.8585 | **60.3456 ✅** | 49.9010 | -12.4871 | Lower | -2.0425 | Lower | +10.4446 | Higher |
| MI (SEI) | 13.1585 | **39.5349 ✅** | 13.5291 | -26.3764 | Lower | -0.3706 | Lower | +26.0058 | Higher |
| Cyclomatic (sum) | 23 | 21 | 21 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 65 | **23 ✅** | 55 | +42 | Higher | +10 | Higher | -32 | Lower |
| Halstead Volume | 2796.4671 | **1626.7294 ✅** | 2838 | +1169.7377 | Higher | -41.5329 | Lower | -1211.2706 | Lower |
| Halstead Effort | 134964.4942 | **61002.3528 ✅** | 168861 | +73962.1414 | Higher | -33896.5058 | Lower | -107858.6472 | Lower |
| SLOC | 113 | **64 ✅** | 102 | +49 | Higher | +11 | Higher | -38 | Lower |
| PLOC | 96 | **48 ✅** | 88 | +48 | Higher | +8 | Higher | -40 | Lower |
| LLOC | 70 | **31 ✅** | 76 | +39 | Higher | -6 | Lower | -45 | Lower |
| Comments (CLOC) | 6 | **8 ✅** | 4 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 14 | 13 | **12 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 6 | 2 | -6 | Lower | -2 | Lower | +4 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00811

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.2348 | **60.5224 ✅** | 57.4354 | -2.2876 | Lower | +0.7994 | Higher | +3.0871 | Higher |
| MI (SEI) | 19.5870 | **36.3255 ✅** | 18.0329 | -16.7385 | Lower | +1.5541 | Higher | +18.2927 | Higher |
| Cyclomatic (sum) | 19 | 20 | **17 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 25 | **22 ✅** | 25 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 2208 | **1565.9536 ✅** | 2459.0831 | +642.0464 | Higher | -251.0831 | Lower | -893.1294 | Lower |
| Halstead Effort | 120213.3333 | **62614.4186 ✅** | 157753.9052 | +57598.9147 | Higher | -37540.5719 | Lower | -95139.4866 | Lower |
| SLOC | 68 | **65 ✅** | 71 | +3 | Higher | -3 | Lower | -6 | Lower |
| PLOC | 57 | **55 ✅** | 61 | +2 | Higher | -4 | Lower | -6 | Lower |
| LLOC | 43 | **31 ✅** | 50 | +12 | Higher | -7 | Lower | -19 | Lower |
| Comments (CLOC) | 1 | **6 ✅** | 1 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 11 | 10 | 10 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p00816

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.0350 | **69.4207 ✅** | 58.2380 | -14.3856 | Lower | -3.2029 | Lower | +11.1827 | Higher |
| MI (SEI) | 27.4896 | **60.2354 ✅** | 29.7980 | -32.7458 | Lower | -2.3084 | Lower | +30.4374 | Higher |
| Cyclomatic (sum) | 24 | **14 ✅** | 16 | +10 | Higher | +8 | Higher | -2 | Lower |
| Cognitive (sum) | 22 | **13 ✅** | 36 | +9 | Higher | -14 | Lower | -23 | Lower |
| Halstead Volume | 2516.4428 | **1083.0835 ✅** | 2108.7599 | +1433.3592 | Higher | +407.6828 | Higher | -1025.6764 | Lower |
| Halstead Effort | 142358.7613 | **41066.9178 ✅** | 107834.3152 | +101291.8435 | Higher | +34524.4461 | Higher | -66767.3974 | Lower |
| SLOC | 74 | **46 ✅** | 72 | +28 | Higher | +2 | Higher | -26 | Lower |
| PLOC | 64 | **35 ✅** | 63 | +29 | Higher | +1 | Higher | -28 | Lower |
| LLOC | 48 | **21 ✅** | 49 | +27 | Higher | -1 | Lower | -28 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 5 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 9 | **6 ✅** | 8 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00820

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.1832 | 73.7084 | **77.1759 ✅** | -2.5252 | Lower | -5.9927 | Lower | -3.4674 | Lower |
| MI (SEI) | 58.9971 | 55.3865 | **68.3904 ✅** | +3.6106 | Higher | -9.3933 | Lower | -13.0039 | Lower |
| Cyclomatic (sum) | 13 | 12 | **8 ✅** | +1 | Higher | +5 | Higher | +4 | Higher |
| Cognitive (sum) | 12 | 25 | **7 ✅** | -13 | Lower | +5 | Higher | +18 | Higher |
| Halstead Volume | 926.4271 | 801.8106 | **744.6997 ✅** | +124.6166 | Higher | +181.7274 | Higher | +57.1108 | Higher |
| Halstead Effort | 33512.4942 | **19644.3585 ✅** | 22850.5233 | +13868.1357 | Higher | +10661.9710 | Higher | -3206.1647 | Lower |
| SLOC | 44 | 40 | **35 ✅** | +4 | Higher | +9 | Higher | +5 | Higher |
| PLOC | 32 | 32 | **26 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| LLOC | 18 | 18 | 18 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | **8 ✅** | 4 | 7 | +4 | Higher | +1 | Higher | -3 | Lower |
| Blank Lines | 7 | 5 | **4 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.6934 | **65.6925 ✅** | 45.8604 | -20.9991 | Lower | -1.1670 | Lower | +19.8321 | Higher |
| MI (SEI) | 8.1644 | **55.6349 ✅** | 2.5143 | -47.4706 | Lower | +5.6501 | Higher | +53.1206 | Higher |
| Cyclomatic (sum) | 23 | **11 ✅** | 20 | +12 | Higher | +3 | Higher | -9 | Lower |
| Cognitive (sum) | 42 | **21 ✅** | 40 | +21 | Higher | +2 | Higher | -19 | Lower |
| Halstead Volume | 4374.8238 | **1230.3606 ✅** | 4097.6639 | +3144.4632 | Higher | +277.1600 | Higher | -2867.3032 | Lower |
| Halstead Effort | 362931.8144 | **30451.4261 ✅** | 367360.3296 | +332480.3883 | Higher | -4428.5152 | Lower | -336908.9035 | Lower |
| SLOC | 119 | **58 ✅** | 118 | +61 | Higher | +1 | Higher | -60 | Lower |
| PLOC | 100 | **35 ✅** | 100 | +65 | Higher | 0 | Same | -65 | Lower |
| LLOC | 103 | **26 ✅** | 103 | +77 | Higher | 0 | Same | -77 | Lower |
| Comments (CLOC) | 6 | **15 ✅** | 2 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 16 | **10 ✅** | 16 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 3 | **0 ✅** | 1 | +3 | Higher | +2 | Higher | -1 | Lower |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.0765 | **72.8600 ✅** | 47.2619 | -24.7835 | Lower | +0.8146 | Higher | +25.5982 | Higher |
| MI (SEI) | 12.1510 | **64.3828 ✅** | 7.4041 | -52.2317 | Lower | +4.7469 | Higher | +56.9786 | Higher |
| Cyclomatic (sum) | 18 | **6 ✅** | 20 | +12 | Higher | -2 | Lower | -14 | Lower |
| Cognitive (sum) | 18 | **3 ✅** | 18 | +15 | Higher | 0 | Same | -15 | Lower |
| Halstead Volume | 4332.5832 | **796.3187 ✅** | 4371.1201 | +3536.2645 | Higher | -38.5368 | Lower | -3574.8014 | Lower |
| Halstead Effort | 229060.3427 | **13696.6816 ✅** | 260044.6005 | +215363.6610 | Higher | -30984.2579 | Lower | -246347.9189 | Lower |
| SLOC | 104 | **46 ✅** | 106 | +58 | Higher | -2 | Lower | -60 | Lower |
| PLOC | 86 | **28 ✅** | 87 | +58 | Higher | -1 | Lower | -59 | Lower |
| LLOC | 48 | **12 ✅** | 50 | +36 | Higher | -2 | Lower | -38 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 3 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 16 | **7 ✅** | 17 | +9 | Higher | -1 | Lower | -10 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 3 | **1 ✅** | 5 | +2 | Higher | -2 | Lower | -4 | Lower |

## p00855

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.4405 | 66.2455 | **75.1203 ✅** | +8.1951 | Higher | -0.6798 | Lower | -8.8748 | Lower |
| MI (SEI) | 57.0024 | 36.3213 | **71.0816 ✅** | +20.6811 | Higher | -14.0792 | Lower | -34.7603 | Lower |
| Cyclomatic (sum) | 12 | 16 | **10 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Cognitive (sum) | 12 | 14 | **11 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | **817.1940 ✅** | 1174.2906 | 851.2372 | -357.0966 | Lower | -34.0432 | Lower | +323.0534 | Higher |
| Halstead Effort | **22441.4052 ✅** | 39686.6724 | 33780.6774 | -17245.2673 | Lower | -11339.2723 | Lower | +5905.9950 | Higher |
| SLOC | 38 | 53 | **37 ✅** | -15 | Lower | +1 | Higher | +16 | Higher |
| PLOC | **31 ✅** | 46 | 33 | -15 | Lower | -2 | Lower | +13 | Higher |
| LLOC | **18 ✅** | 24 | 27 | -6 | Lower | -9 | Lower | -3 | Lower |
| Comments (CLOC) | 4 | 2 | **11 ✅** | +2 | Higher | -7 | Lower | -9 | Lower |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | 7 | **2 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |

## p00856

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.9521 ✅** | 52.0106 | 53.5312 | +3.9416 | Higher | +2.4210 | Higher | -1.5206 | Lower |
| MI (SEI) | 7.6683 | **35.5181 ✅** | 29.4847 | -27.8497 | Lower | -21.8163 | Lower | +6.0334 | Higher |
| Cyclomatic (sum) | 26 | 27 | **25 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | **17 ✅** | 29 | 21 | -12 | Lower | -4 | Lower | +8 | Higher |
| Halstead Volume | 2757.8348 | **2560.0445 ✅** | 2958.6623 | +197.7903 | Higher | -200.8276 | Lower | -398.6179 | Lower |
| Halstead Effort | 157607.3222 | **127148.8759 ✅** | 180414.0845 | +30458.4462 | Higher | -22806.7623 | Lower | -53265.2086 | Lower |
| SLOC | **66 ✅** | 85 | 76 | -19 | Lower | -10 | Lower | +9 | Higher |
| PLOC | **56 ✅** | 58 | 60 | -2 | Lower | -4 | Lower | -2 | Lower |
| LLOC | 37 | **36 ✅** | 43 | +1 | Higher | -6 | Lower | -7 | Lower |
| Comments (CLOC) | 0 | **19 ✅** | 9 | -19 | Lower | -9 | Lower | +10 | Higher |
| Blank Lines | **10 ✅** | 14 | 11 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **3 ✅** | 7 | +4 | Higher | 0 | Same | -4 | Lower |

## p00866

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.7080 | **60.8052 ✅** | 49.2060 | -8.0971 | Lower | +3.5021 | Higher | +11.5992 | Higher |
| MI (SEI) | 10.4874 | **46.7859 ✅** | 10.2496 | -36.2984 | Lower | +0.2378 | Higher | +36.5362 | Higher |
| Cyclomatic (sum) | 18 | 18 | 18 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 32 | **18 ✅** | 32 | +14 | Higher | 0 | Same | -14 | Lower |
| Halstead Volume | 3214.0704 | **1620.2520 ✅** | 3704.1195 | +1593.8184 | Higher | -490.0491 | Lower | -2083.8675 | Lower |
| Halstead Effort | 194521.1330 | **63372.7606 ✅** | 227705.8738 | +131148.3724 | Higher | -33184.7408 | Lower | -164333.1132 | Lower |
| SLOC | 86 | **65 ✅** | 102 | +21 | Higher | -16 | Lower | -37 | Lower |
| PLOC | 75 | **44 ✅** | 87 | +31 | Higher | -12 | Lower | -43 | Lower |
| LLOC | 51 | **31 ✅** | 55 | +20 | Higher | -4 | Lower | -24 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 3 | -13 | Lower | -2 | Lower | +11 | Higher |
| Blank Lines | **10 ✅** | 12 | 13 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | **0 ✅** | 5 | 1 | -5 | Lower | -1 | Lower | +4 | Higher |

## p00867

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 11.1131 | **73.3670 ✅** | 9.6234 | -62.2539 | Lower | +1.4897 | Higher | +63.7436 | Higher |
| MI (SEI) | -44.6445 | **70.5482 ✅** | -47.0286 | -115.1927 | Lower | +2.3841 | Higher | +117.5768 | Higher |
| Cyclomatic (sum) | 84 | **5 ✅** | 84 | +79 | Higher | 0 | Same | -79 | Lower |
| Cognitive (sum) | 112 | **3 ✅** | 112 | +109 | Higher | 0 | Same | -109 | Lower |
| Halstead Volume | 12362.4188 | **808.5134 ✅** | 13059.1570 | +11553.9053 | Higher | -696.7383 | Lower | -12250.6436 | Lower |
| Halstead Effort | 1782837.3909 | **17602.0115 ✅** | 2022718.3192 | +1765235.3793 | Higher | -239880.9284 | Lower | -2005116.3077 | Lower |
| SLOC | 285 | **45 ✅** | 307 | +240 | Higher | -22 | Lower | -262 | Lower |
| PLOC | 253 | **27 ✅** | 273 | +226 | Higher | -20 | Lower | -246 | Lower |
| LLOC | 170 | **14 ✅** | 174 | +156 | Higher | -4 | Lower | -160 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 2 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 30 | **6 ✅** | 32 | +24 | Higher | -2 | Lower | -26 | Lower |
| Functions | 15 | **2 ✅** | 16 | +13 | Higher | -1 | Lower | -14 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 17 | **1 ✅** | 18 | +16 | Higher | -1 | Lower | -17 | Lower |

## p00870

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.6265 | **63.7900 ✅** | 50.6629 | -11.1635 | Lower | +1.9635 | Higher | +13.1271 | Higher |
| MI (SEI) | 14.5616 | **54.1486 ✅** | 17.3089 | -39.5870 | Lower | -2.7473 | Lower | +36.8397 | Higher |
| Cyclomatic (sum) | 21 | **14 ✅** | 23 | +7 | Higher | -2 | Lower | -9 | Lower |
| Cognitive (sum) | 41 | **8 ✅** | 43 | +33 | Higher | -2 | Lower | -35 | Lower |
| Halstead Volume | 3724.8668 | **1089.2642 ✅** | 2991.8515 | +2635.6026 | Higher | +733.0154 | Higher | -1902.5872 | Lower |
| Halstead Effort | 130039.6261 | **44115.2019 ✅** | 153164.4771 | +85924.4242 | Higher | -23124.8510 | Lower | -109049.2752 | Lower |
| SLOC | 79 | **65 ✅** | 93 | +14 | Higher | -14 | Lower | -28 | Lower |
| PLOC | 68 | **39 ✅** | 75 | +29 | Higher | -7 | Lower | -36 | Lower |
| LLOC | 48 | **19 ✅** | 55 | +29 | Higher | -7 | Lower | -36 | Lower |
| Comments (CLOC) | 2 | **18 ✅** | 5 | -16 | Lower | -3 | Lower | +13 | Higher |
| Blank Lines | 11 | 11 | 13 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | **1 ✅** | 5 | 2 | -4 | Lower | -1 | Lower | +3 | Higher |
| Closures | **0 ✅** | 1 | 3 | -1 | Lower | -3 | Lower | -2 | Lower |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p00874

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.7925 ✅** | 62.1145 | 67.5887 | +8.6780 | Higher | +3.2038 | Higher | -5.4741 | Lower |
| MI (SEI) | **59.2830 ✅** | 45.5920 | 34.1082 | +13.6909 | Higher | +25.1748 | Higher | +11.4839 | Higher |
| Cyclomatic (sum) | **9 ✅** | 15 | 13 | -6 | Lower | -4 | Lower | +2 | Higher |
| Cognitive (sum) | **9 ✅** | 17 | 15 | -8 | Lower | -6 | Lower | +2 | Higher |
| Halstead Volume | **1111.4075 ✅** | 1585.4079 | 1322.5020 | -474.0004 | Lower | -211.0945 | Lower | +262.9059 | Higher |
| Halstead Effort | **26594.3947 ✅** | 59452.7960 | 39857.4745 | -32858.4014 | Lower | -13263.0799 | Lower | +19595.3215 | Higher |
| SLOC | **45 ✅** | 63 | 49 | -18 | Lower | -4 | Lower | +14 | Higher |
| PLOC | **29 ✅** | 46 | 39 | -17 | Lower | -10 | Lower | +7 | Higher |
| LLOC | **24 ✅** | 35 | 26 | -11 | Lower | -2 | Lower | +9 | Higher |
| Comments (CLOC) | 9 | **11 ✅** | 1 | -2 | Lower | +8 | Higher | +10 | Higher |
| Blank Lines | **9 ✅** | 10 | 10 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 4 | **0 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00889

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.7829 | **72.3697 ✅** | 44.2250 | -23.5868 | Lower | +4.5579 | Higher | +28.1447 | Higher |
| MI (SEI) | -3.1838 | **63.1819 ✅** | 4.3811 | -66.3656 | Lower | -7.5649 | Lower | +58.8007 | Higher |
| Cyclomatic (sum) | 21 | **8 ✅** | 25 | +13 | Higher | -4 | Lower | -17 | Lower |
| Cognitive (sum) | 40 | **9 ✅** | 44 | +31 | Higher | -4 | Lower | -35 | Lower |
| Halstead Volume | 3628.4694 | **919.9486 ✅** | 3854.5674 | +2708.5208 | Higher | -226.0980 | Lower | -2934.6188 | Lower |
| Halstead Effort | 325050.3844 | **23509.7965 ✅** | 301482.2364 | +301540.5879 | Higher | +23568.1480 | Higher | -277972.4399 | Lower |
| SLOC | 101 | **44 ✅** | 124 | +57 | Higher | -23 | Lower | -80 | Lower |
| PLOC | 91 | **28 ✅** | 108 | +63 | Higher | -17 | Lower | -80 | Lower |
| LLOC | 82 | **19 ✅** | 89 | +63 | Higher | -7 | Lower | -70 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 4 | -10 | Lower | -4 | Lower | +6 | Higher |
| Blank Lines | 10 | **9 ✅** | 14 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |

## p00897

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.4386 | **45.4407 ✅** | 21.7956 | -21.0022 | Lower | +2.6430 | Higher | +23.6451 | Higher |
| MI (SEI) | -31.1445 | **11.5446 ✅** | -31.1647 | -42.6891 | Lower | +0.0202 | Higher | +42.7093 | Higher |
| Cyclomatic (sum) | 43 | **24 ✅** | 48 | +19 | Higher | -5 | Lower | -24 | Lower |
| Cognitive (sum) | 58 | **30 ✅** | 53 | +28 | Higher | +5 | Higher | -23 | Lower |
| Halstead Volume | 9128 | **3270.3952 ✅** | 9769.9568 | +5857.6048 | Higher | -641.9568 | Lower | -6499.5615 | Lower |
| Halstead Effort | 789373.5652 | **127545.4139 ✅** | 1080580.7609 | +661828.1513 | Higher | -291207.1957 | Lower | -953035.3470 | Lower |
| SLOC | 247 | **123 ✅** | 265 | +124 | Higher | -18 | Lower | -142 | Lower |
| PLOC | 210 | **100 ✅** | 227 | +110 | Higher | -17 | Lower | -127 | Lower |
| LLOC | 151 | **41 ✅** | 183 | +110 | Higher | -32 | Lower | -142 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 3 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 37 | **19 ✅** | 36 | +18 | Higher | +1 | Higher | -17 | Lower |
| Functions | 10 | **7 ✅** | 12 | +3 | Higher | -2 | Lower | -5 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |

## p00907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.1304 | **64.4758 ✅** | 60.3975 | -3.3454 | Lower | +0.7329 | Higher | +4.0783 | Higher |
| MI (SEI) | 33.4523 | **55.0232 ✅** | 32.2165 | -21.5709 | Lower | +1.2358 | Higher | +22.8067 | Higher |
| Cyclomatic (sum) | 19 | **12 ✅** | 20 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 35 | **10 ✅** | 33 | +25 | Higher | +2 | Higher | -23 | Lower |
| Halstead Volume | 1605.0868 | **1271.1819 ✅** | 1604.0448 | +333.9049 | Higher | +1.0420 | Higher | -332.8629 | Lower |
| Halstead Effort | 74904.0503 | **41525.2746 ✅** | 81405.2713 | +33378.7757 | Higher | -6501.2209 | Lower | -39879.9966 | Lower |
| SLOC | 63 | **61 ✅** | 65 | +2 | Higher | -2 | Lower | -4 | Lower |
| PLOC | 51 | **36 ✅** | 53 | +15 | Higher | -2 | Lower | -17 | Lower |
| LLOC | 38 | **21 ✅** | 39 | +17 | Higher | -1 | Lower | -18 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 4 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 10 | 13 | 10 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p00924

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.0061 | **74.1742 ✅** | 50.4145 | -25.1681 | Lower | -1.4084 | Lower | +23.7597 | Higher |
| MI (SEI) | 16.9149 | **69.8435 ✅** | 18.5395 | -52.9286 | Lower | -1.6246 | Lower | +51.3041 | Higher |
| Cyclomatic (sum) | 25 | **10 ✅** | 21 | +15 | Higher | +4 | Higher | -11 | Lower |
| Cognitive (sum) | 25 | **8 ✅** | 25 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 4030.0457 | **800.9165 ✅** | 3668.7742 | +3229.1292 | Higher | +361.2715 | Higher | -2867.8577 | Lower |
| Halstead Effort | 247290.4634 | **19455.5962 ✅** | 222936.5774 | +227834.8671 | Higher | +24353.8859 | Higher | -203480.9812 | Lower |
| SLOC | 91 | **40 ✅** | 91 | +51 | Higher | 0 | Same | -51 | Lower |
| PLOC | 75 | **24 ✅** | 75 | +51 | Higher | 0 | Same | -51 | Lower |
| LLOC | 61 | **17 ✅** | 61 | +44 | Higher | 0 | Same | -44 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 6 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 11 | **6 ✅** | 11 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 6 | **0 ✅** | 2 | +6 | Higher | +4 | Higher | -2 | Lower |
| Exits | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |

## p00949

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.9889 | **55.3071 ✅** | 44.4042 | -8.3182 | Lower | +2.5847 | Higher | +10.9029 | Higher |
| MI (SEI) | 10.6172 | **36.2656 ✅** | 2.5879 | -25.6484 | Lower | +8.0293 | Higher | +33.6777 | Higher |
| Cyclomatic (sum) | 22 | **17 ✅** | 23 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 22 | **20 ✅** | 24 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | 3865.5224 | **2192.9603 ✅** | 3967.8407 | +1672.5621 | Higher | -102.3183 | Lower | -1774.8804 | Lower |
| Halstead Effort | 263527.7856 | **96983.6684 ✅** | 312892.5810 | +166544.1172 | Higher | -49364.7954 | Lower | -215908.9126 | Lower |
| SLOC | 109 | **84 ✅** | 125 | +25 | Higher | -16 | Lower | -41 | Lower |
| PLOC | 94 | **54 ✅** | 105 | +40 | Higher | -11 | Lower | -51 | Lower |
| LLOC | 75 | **40 ✅** | 89 | +35 | Higher | -14 | Lower | -49 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 3 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | **15 ✅** | 17 | 18 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | **0 ✅** | 2 | +3 | Higher | +1 | Higher | -2 | Lower |
| Exits | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |

## p00971

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.2895 | **64.1132 ✅** | 55.1424 | -14.8236 | Lower | -5.8529 | Lower | +8.9707 | Higher |
| MI (SEI) | 10.6289 | **52.2146 ✅** | 5.3801 | -41.5857 | Lower | +5.2488 | Higher | +46.8345 | Higher |
| Cyclomatic (sum) | 18 | **12 ✅** | 15 | +6 | Higher | +3 | Higher | -3 | Lower |
| Cognitive (sum) | 15 | 13 | 13 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 4128.9165 | **1881.8482 ✅** | 3242.8829 | +2247.0683 | Higher | +886.0336 | Higher | -1361.0347 | Lower |
| Halstead Effort | 370226.1819 | **94092.4107 ✅** | 259430.6350 | +276133.7712 | Higher | +110795.5469 | Higher | -165338.2243 | Lower |
| SLOC | 98 | **55 ✅** | 77 | +43 | Higher | +21 | Higher | -22 | Lower |
| PLOC | 86 | **38 ✅** | 68 | +48 | Higher | +18 | Higher | -30 | Lower |
| LLOC | 70 | **34 ✅** | 52 | +36 | Higher | +18 | Higher | -18 | Lower |
| Comments (CLOC) | 3 | **13 ✅** | 0 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 12 | 9 | 9 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 5 | **1 ✅** | 3 | +4 | Higher | +2 | Higher | -2 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 5 | **0 ✅** | 2 | +5 | Higher | +3 | Higher | -2 | Lower |

## p01050

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.6935 | **77.4597 ✅** | 72.5168 | -3.7661 | Lower | +1.1768 | Higher | +4.9429 | Higher |
| MI (SEI) | 63.9752 | **75.5515 ✅** | 70.7773 | -11.5762 | Lower | -6.8021 | Lower | +4.7742 | Higher |
| Cyclomatic (sum) | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 13 | **4 ✅** | 13 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 766.1299 | **574.7682 ✅** | 894.2888 | +191.3617 | Higher | -128.1589 | Lower | -319.5206 | Lower |
| Halstead Effort | 21317.5655 | **9360.5112 ✅** | 22752.7709 | +11957.0542 | Higher | -1435.2055 | Lower | -13392.2597 | Lower |
| SLOC | 43 | **39 ✅** | 44 | +4 | Higher | -1 | Lower | -5 | Lower |
| PLOC | 31 | **19 ✅** | 33 | +12 | Higher | -2 | Lower | -14 | Lower |
| LLOC | 28 | **13 ✅** | 31 | +15 | Higher | -3 | Lower | -18 | Lower |
| Comments (CLOC) | 9 | 13 | **17 ✅** | -4 | Lower | -8 | Lower | -4 | Lower |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p01065

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.5990 | **72.1254 ✅** | 58.5545 | -12.5263 | Lower | +1.0445 | Higher | +13.5709 | Higher |
| MI (SEI) | 30.5556 | **66.9905 ✅** | 24.1307 | -36.4350 | Lower | +6.4248 | Higher | +42.8598 | Higher |
| Cyclomatic (sum) | 15 | **6 ✅** | 18 | +9 | Higher | -3 | Lower | -12 | Lower |
| Cognitive (sum) | 30 | **6 ✅** | 32 | +24 | Higher | -2 | Lower | -26 | Lower |
| Halstead Volume | 2333.1560 | **982.1539 ✅** | 2621.3859 | +1351.0021 | Higher | -288.2299 | Lower | -1639.2320 | Lower |
| Halstead Effort | 146733.6394 | **29157.6945 ✅** | 163574.4804 | +117575.9449 | Higher | -16840.8409 | Lower | -134416.7858 | Lower |
| SLOC | 65 | **45 ✅** | 64 | +20 | Higher | +1 | Higher | -19 | Lower |
| PLOC | 52 | **28 ✅** | 53 | +24 | Higher | -1 | Lower | -25 | Lower |
| LLOC | 34 | **12 ✅** | 40 | +22 | Higher | -6 | Lower | -28 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 2 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.1832 | **76.2919 ✅** | 68.8888 | -7.1087 | Lower | +0.2944 | Higher | +7.4031 | Higher |
| MI (SEI) | 52.9106 | **71.6173 ✅** | 59.8642 | -18.7067 | Lower | -6.9536 | Lower | +11.7530 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 1238.6733 | **808.5182 ✅** | 1399.6763 | +430.1551 | Higher | -161.0030 | Lower | -591.1581 | Lower |
| Halstead Effort | 39786.1868 | **16140.4192 ✅** | 52972.3659 | +23645.7676 | Higher | -13186.1791 | Lower | -36831.9467 | Lower |
| SLOC | 48 | **36 ✅** | 47 | +12 | Higher | +1 | Higher | -11 | Lower |
| PLOC | 34 | **25 ✅** | 37 | +9 | Higher | -3 | Lower | -12 | Lower |
| LLOC | 24 | **13 ✅** | 31 | +11 | Higher | -7 | Lower | -18 | Lower |
| Comments (CLOC) | 7 | 10 | **12 ✅** | -3 | Lower | -5 | Lower | -2 | Lower |
| Blank Lines | 9 | **6 ✅** | 8 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01119

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.0689 | **60.8497 ✅** | 31.3751 | -8.7808 | Lower | +20.6938 | Higher | +29.4746 | Higher |
| MI (SEI) | 17.0614 | **50.1332 ✅** | -16.4358 | -33.0718 | Lower | +33.4972 | Higher | +66.5690 | Higher |
| Cyclomatic (sum) | 19 | **17 ✅** | 43 | +2 | Higher | -24 | Lower | -26 | Lower |
| Cognitive (sum) | 63 | **14 ✅** | 67 | +49 | Higher | -4 | Lower | -53 | Lower |
| Halstead Volume | 2635.5917 | **1394.0262 ✅** | 5270.5620 | +1241.5655 | Higher | -2634.9703 | Lower | -3876.5358 | Lower |
| Halstead Effort | 99311.3382 | **41356.1100 ✅** | 370194.2358 | +57955.2282 | Higher | -270882.8976 | Lower | -328838.1258 | Lower |
| SLOC | 94 | **69 ✅** | 192 | +25 | Higher | -98 | Lower | -123 | Lower |
| PLOC | 81 | **47 ✅** | 171 | +34 | Higher | -90 | Lower | -124 | Lower |
| LLOC | 69 | **23 ✅** | 120 | +46 | Higher | -51 | Lower | -97 | Lower |
| Comments (CLOC) | 4 | **19 ✅** | 3 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 11 | 11 | 19 | 0 | Same | -8 | Lower | -8 | Lower |
| Functions | **1 ✅** | 3 | 10 | -2 | Lower | -9 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 4 | 11 | -4 | Lower | -11 | Lower | -7 | Lower |

## p01125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.6714 ✅** | 58.2392 | 59.0016 | +1.4322 | Higher | +0.6698 | Higher | -0.7624 | Lower |
| MI (SEI) | 41.7693 | **47.5959 ✅** | 20.2860 | -5.8267 | Lower | +21.4833 | Higher | +27.3099 | Higher |
| Cyclomatic (sum) | 20 | 24 | **15 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| Cognitive (sum) | 25 | 35 | **22 ✅** | -10 | Lower | +3 | Higher | +13 | Higher |
| Halstead Volume | 2247.9370 | **1615.7105 ✅** | 2274 | +632.2265 | Higher | -26.0630 | Lower | -658.2895 | Lower |
| Halstead Effort | 87428.6926 | **71820.9392 ✅** | 79090.8293 | +15607.7534 | Higher | +8337.8633 | Higher | -7269.8901 | Lower |
| SLOC | **61 ✅** | 70 | 68 | -9 | Lower | -7 | Lower | +2 | Higher |
| PLOC | **47 ✅** | 54 | 57 | -7 | Lower | -10 | Lower | -3 | Lower |
| LLOC | 34 | **33 ✅** | 44 | +1 | Higher | -10 | Lower | -11 | Lower |
| Comments (CLOC) | 10 | **20 ✅** | 1 | -10 | Lower | +9 | Higher | +19 | Higher |
| Blank Lines | 11 | **8 ✅** | 11 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p01128

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.5863 | **54.5932 ✅** | 46.0662 | -9.0068 | Lower | -0.4798 | Lower | +8.5270 | Higher |
| MI (SEI) | 8.6527 | **38.7681 ✅** | -6.5941 | -30.1154 | Lower | +15.2468 | Higher | +45.3622 | Higher |
| Cyclomatic (sum) | 26 | **13 ✅** | 26 | +13 | Higher | 0 | Same | -13 | Lower |
| Cognitive (sum) | 41 | **10 ✅** | 37 | +31 | Higher | +4 | Higher | -27 | Lower |
| Halstead Volume | 3688.3355 | **1980.7439 ✅** | 3980.2318 | +1707.5916 | Higher | -291.8963 | Lower | -1999.4879 | Lower |
| Halstead Effort | 205608.5247 | **60135.3836 ✅** | 243920.6187 | +145473.1411 | Higher | -38312.0941 | Lower | -183785.2352 | Lower |
| SLOC | 114 | **96 ✅** | 108 | +18 | Higher | +6 | Higher | -12 | Lower |
| PLOC | 95 | **64 ✅** | 97 | +31 | Higher | -2 | Lower | -33 | Lower |
| LLOC | 46 | **23 ✅** | 62 | +23 | Higher | -16 | Lower | -39 | Lower |
| Comments (CLOC) | 5 | **23 ✅** | 0 | -18 | Lower | +5 | Higher | +23 | Higher |
| Blank Lines | 19 | 12 | **11 ✅** | +7 | Higher | +8 | Higher | +1 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |
| Exits | **1 ✅** | 3 | 5 | -2 | Lower | -4 | Lower | -2 | Lower |

## p01138

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.1237 ✅** | 64.9577 | 65.5570 | +3.1659 | Higher | +2.5666 | Higher | -0.5993 | Lower |
| MI (SEI) | 47.1413 | **50.7634 ✅** | 44.3800 | -3.6221 | Lower | +2.7613 | Higher | +6.3834 | Higher |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | **4 ✅** | 14 | 8 | -10 | Lower | -4 | Lower | +6 | Higher |
| Halstead Volume | 1488.5364 | **1455.9865 ✅** | 1801.6016 | +32.5499 | Higher | -313.0652 | Lower | -345.6151 | Lower |
| Halstead Effort | 44656.0929 | **38538.1437 ✅** | 55249.1167 | +6117.9492 | Higher | -10593.0238 | Lower | -16710.9730 | Lower |
| SLOC | **49 ✅** | 60 | 54 | -11 | Lower | -5 | Lower | +6 | Higher |
| PLOC | **36 ✅** | 39 | 41 | -3 | Lower | -5 | Lower | -2 | Lower |
| LLOC | **24 ✅** | 34 | 33 | -10 | Lower | -9 | Lower | +1 | Higher |
| Comments (CLOC) | 5 | **12 ✅** | 6 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p01150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.3907 | **63.1035 ✅** | 24.5762 | -38.7128 | Lower | -0.1855 | Lower | +38.5273 | Higher |
| MI (SEI) | -17.9702 | **51.5207 ✅** | -18.5829 | -69.4909 | Lower | +0.6128 | Higher | +70.1037 | Higher |
| Cyclomatic (sum) | 60 | **14 ✅** | 59 | +46 | Higher | +1 | Higher | -45 | Lower |
| Cognitive (sum) | 76 | **9 ✅** | 72 | +67 | Higher | +4 | Higher | -63 | Lower |
| Halstead Volume | 6789.7508 | **1242.9923 ✅** | 6948.7930 | +5546.7584 | Higher | -159.0422 | Lower | -5705.8006 | Lower |
| Halstead Effort | 1068254.1218 | **44348.1909 ✅** | 1035506.4049 | +1023905.9308 | Higher | +32747.7169 | Higher | -991158.2140 | Lower |
| SLOC | 214 | **65 ✅** | 213 | +149 | Higher | +1 | Higher | -148 | Lower |
| PLOC | 189 | **40 ✅** | 188 | +149 | Higher | +1 | Higher | -148 | Lower |
| LLOC | 124 | **16 ✅** | 154 | +108 | Higher | -30 | Lower | -138 | Lower |
| Comments (CLOC) | 10 | **16 ✅** | 9 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 18 | **10 ✅** | 20 | +8 | Higher | -2 | Lower | -10 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |

## p01153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.2552 | 67.4642 | **72.0519 ✅** | +0.7910 | Higher | -3.7966 | Lower | -4.5876 | Lower |
| MI (SEI) | 53.3452 | 57.6650 | **59.6751 ✅** | -4.3198 | Lower | -6.3299 | Lower | -2.0100 | Lower |
| Cyclomatic (sum) | 10 | **8 ✅** | 9 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 20 | **5 ✅** | 17 | +15 | Higher | +3 | Higher | -12 | Lower |
| Halstead Volume | 1416.6079 | **1179.0891 ✅** | 1362.4051 | +237.5188 | Higher | +54.2028 | Higher | -183.3160 | Lower |
| Halstead Effort | 56228.4368 | **39917.0778 ✅** | 41962.0769 | +16311.3590 | Higher | +14266.3599 | Higher | -2044.9991 | Lower |
| SLOC | 48 | 55 | **39 ✅** | -7 | Lower | +9 | Higher | +16 | Higher |
| PLOC | 34 | 34 | **30 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| LLOC | 25 | 23 | 23 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 8 | **14 ✅** | 7 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01184

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.5335 | **60.0965 ✅** | 34.1371 | -24.5630 | Lower | +1.3964 | Higher | +25.9594 | Higher |
| MI (SEI) | -10.7576 | **31.1069 ✅** | -14.2922 | -41.8644 | Lower | +3.5346 | Higher | +45.3991 | Higher |
| Cyclomatic (sum) | 34 | **16 ✅** | 38 | +18 | Higher | -4 | Lower | -22 | Lower |
| Cognitive (sum) | 55 | **26 ✅** | 63 | +29 | Higher | -8 | Lower | -37 | Lower |
| Halstead Volume | 5153.4557 | **1845.8009 ✅** | 5348.3042 | +3307.6548 | Higher | -194.8485 | Lower | -3502.5034 | Lower |
| Halstead Effort | 408169.7950 | **53297.4997 ✅** | 465039.4369 | +354872.2953 | Higher | -56869.6419 | Lower | -411741.9372 | Lower |
| SLOC | 170 | **67 ✅** | 173 | +103 | Higher | -3 | Lower | -106 | Lower |
| PLOC | 142 | **53 ✅** | 147 | +89 | Higher | -5 | Lower | -94 | Lower |
| LLOC | 102 | **33 ✅** | 106 | +69 | Higher | -4 | Lower | -73 | Lower |
| Comments (CLOC) | 3 | **4 ✅** | 2 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 26 | **10 ✅** | 24 | +16 | Higher | +2 | Higher | -14 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **1 ✅** | 9 | +6 | Higher | -2 | Lower | -8 | Lower |

## p01255

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.7207 | **63.3713 ✅** | 10.1970 | -25.6506 | Lower | +27.5237 | Higher | +53.1743 | Higher |
| MI (SEI) | -3.3489 | **40.2952 ✅** | -46.9484 | -43.6441 | Lower | +43.5994 | Higher | +87.2435 | Higher |
| Cyclomatic (sum) | 31 | **12 ✅** | 79 | +19 | Higher | -48 | Lower | -67 | Lower |
| Cognitive (sum) | 50 | **12 ✅** | 152 | +38 | Higher | -102 | Lower | -140 | Lower |
| Halstead Volume | 4667.4062 | **1572 ✅** | 11432.2869 | +3095.4062 | Higher | -6764.8806 | Lower | -9860.2869 | Lower |
| Halstead Effort | 231231.0846 | **37877.7143 ✅** | 1316999.4501 | +193353.3703 | Higher | -1085768.3655 | Lower | -1279121.7358 | Lower |
| SLOC | 160 | **61 ✅** | 332 | +99 | Higher | -172 | Lower | -271 | Lower |
| PLOC | 135 | **46 ✅** | 302 | +89 | Higher | -167 | Lower | -256 | Lower |
| LLOC | 66 | **25 ✅** | 187 | +41 | Higher | -121 | Lower | -162 | Lower |
| Comments (CLOC) | 6 | 6 | 2 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 22 | **12 ✅** | 28 | +10 | Higher | -6 | Lower | -16 | Lower |
| Functions | 6 | **3 ✅** | 17 | +3 | Higher | -11 | Lower | -14 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **2 ✅** | 26 | +5 | Higher | -19 | Lower | -24 | Lower |

## p01275

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.4647 | **64.8204 ✅** | 48.2961 | -8.3557 | Lower | +8.1686 | Higher | +16.5243 | Higher |
| MI (SEI) | 27.0642 | **51.6209 ✅** | 16.7937 | -24.5567 | Lower | +10.2705 | Higher | +34.8273 | Higher |
| Cyclomatic (sum) | 18 | **10 ✅** | 24 | +8 | Higher | -6 | Lower | -14 | Lower |
| Cognitive (sum) | 17 | **10 ✅** | 20 | +7 | Higher | -3 | Lower | -10 | Lower |
| Halstead Volume | 2390.4204 | **1299.7003 ✅** | 2751.7866 | +1090.7201 | Higher | -361.3662 | Lower | -1452.0863 | Lower |
| Halstead Effort | 108465.3246 | **43323.3419 ✅** | 144468.7939 | +65141.9827 | Higher | -36003.4693 | Lower | -101145.4521 | Lower |
| SLOC | 75 | **61 ✅** | 109 | +14 | Higher | -34 | Lower | -48 | Lower |
| PLOC | 58 | **37 ✅** | 87 | +21 | Higher | -29 | Lower | -50 | Lower |
| LLOC | 37 | **27 ✅** | 56 | +10 | Higher | -19 | Lower | -29 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 8 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | **12 ✅** | 13 | 17 | -1 | Lower | -5 | Lower | -4 | Lower |
| Functions | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.3229 | 60.6320 | **62.2394 ✅** | +0.6909 | Higher | -0.9165 | Lower | -1.6073 | Lower |
| MI (SEI) | 44.8682 | **46.0507 ✅** | 15.2116 | -1.1825 | Lower | +29.6566 | Higher | +30.8391 | Higher |
| Cyclomatic (sum) | 15 | 12 | **11 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Cognitive (sum) | 27 | **10 ✅** | 22 | +17 | Higher | +5 | Higher | -12 | Lower |
| Halstead Volume | 2041.2771 | **1813.4154 ✅** | 1941.7650 | +227.8617 | Higher | +99.5121 | Higher | -128.3496 | Lower |
| Halstead Effort | 99894.9999 | **63469.5394 ✅** | 92401.2327 | +36425.4605 | Higher | +7493.7673 | Higher | -28931.6932 | Lower |
| SLOC | **61 ✅** | 69 | 62 | -8 | Lower | -1 | Lower | +7 | Higher |
| PLOC | **50 ✅** | 51 | 53 | -1 | Lower | -3 | Lower | -2 | Lower |
| LLOC | 45 | **33 ✅** | 50 | +12 | Higher | -5 | Lower | -17 | Lower |
| Comments (CLOC) | 11 | **15 ✅** | 0 | -4 | Lower | +11 | Higher | +15 | Higher |
| Blank Lines | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p01316

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.4027 | **68.3108 ✅** | 57.9479 | -9.9081 | Lower | +0.4549 | Higher | +10.3630 | Higher |
| MI (SEI) | 23.2314 | **61.0062 ✅** | 18.7723 | -37.7748 | Lower | +4.4591 | Higher | +42.2339 | Higher |
| Cyclomatic (sum) | 17 | **13 ✅** | 17 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 35 | **12 ✅** | 35 | +23 | Higher | 0 | Same | -23 | Lower |
| Halstead Volume | 2133.8889 | **1151.0155 ✅** | 2228.2761 | +982.8734 | Higher | -94.3872 | Lower | -1077.2606 | Lower |
| Halstead Effort | 91251.8294 | **43646.5084 ✅** | 86739.7229 | +47605.3209 | Higher | +4512.1064 | Higher | -43093.2145 | Lower |
| SLOC | 70 | **49 ✅** | 71 | +21 | Higher | -1 | Lower | -22 | Lower |
| PLOC | 57 | **33 ✅** | 59 | +24 | Higher | -2 | Lower | -26 | Lower |
| LLOC | 35 | **23 ✅** | 37 | +12 | Higher | -2 | Lower | -14 | Lower |
| Comments (CLOC) | 2 | **14 ✅** | 1 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 13 | **6 ✅** | 12 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p01317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0982 | **55.8504 ✅** | 50.0279 | -2.7522 | Lower | +3.0703 | Higher | +5.8225 | Higher |
| MI (SEI) | 2.6345 | **30.4507 ✅** | 9.5533 | -27.8162 | Lower | -6.9188 | Lower | +20.8974 | Higher |
| Cyclomatic (sum) | 17 | **14 ✅** | 19 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 38 | **18 ✅** | 41 | +20 | Higher | -3 | Lower | -23 | Lower |
| Halstead Volume | 3756 | **2174.0748 ✅** | 3775.9447 | +1581.9252 | Higher | -19.9447 | Lower | -1601.8700 | Lower |
| Halstead Effort | 232156.5714 | **96435.7456 ✅** | 207165.6344 | +135720.8258 | Higher | +24990.9371 | Higher | -110729.8888 | Lower |
| SLOC | **81 ✅** | 85 | 95 | -4 | Lower | -14 | Lower | -10 | Lower |
| PLOC | 69 | **63 ✅** | 79 | +6 | Higher | -10 | Lower | -16 | Lower |
| LLOC | 61 | **26 ✅** | 68 | +35 | Higher | -7 | Lower | -42 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 2 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | **12 ✅** | 14 | 16 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01339

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.5773 | **69.1642 ✅** | 36.3662 | -32.5869 | Lower | +0.2110 | Higher | +32.7980 | Higher |
| MI (SEI) | -8.9783 | **61.8823 ✅** | -7.8237 | -70.8606 | Lower | -1.1547 | Lower | +69.7059 | Higher |
| Cyclomatic (sum) | 33 | **6 ✅** | 33 | +27 | Higher | 0 | Same | -27 | Lower |
| Cognitive (sum) | 34 | **0 ✅** | 31 | +34 | Higher | +3 | Higher | -31 | Lower |
| Halstead Volume | 5535.7469 | **787.2783 ✅** | 5333.0353 | +4748.4685 | Higher | +202.7115 | Higher | -4545.7570 | Lower |
| Halstead Effort | 362305.0894 | **12596.4535 ✅** | 411898.5535 | +349708.6359 | Higher | -49593.4641 | Lower | -399302.1000 | Lower |
| SLOC | 158 | **58 ✅** | 162 | +100 | Higher | -4 | Lower | -104 | Lower |
| PLOC | 137 | **33 ✅** | 144 | +104 | Higher | -7 | Lower | -111 | Lower |
| LLOC | 92 | **10 ✅** | 94 | +82 | Higher | -2 | Lower | -84 | Lower |
| Comments (CLOC) | 3 | **17 ✅** | 4 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | 19 | **11 ✅** | 16 | +8 | Higher | +3 | Higher | -5 | Lower |
| Functions | 8 | **4 ✅** | 9 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.9519 | **68.0473 ✅** | 56.2502 | -12.0954 | Lower | -0.2983 | Lower | +11.7971 | Higher |
| MI (SEI) | 33.8750 | **58.8311 ✅** | 29.9340 | -24.9561 | Lower | +3.9410 | Higher | +28.8971 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 12 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 12 | +11 | Higher | +2 | Higher | -9 | Lower |
| Halstead Volume | 2416.7227 | **1167.8347 ✅** | 2459.8988 | +1248.8881 | Higher | -43.1761 | Lower | -1292.0642 | Lower |
| Halstead Effort | 123494.5324 | **26860.1972 ✅** | 139168.7773 | +96634.3352 | Higher | -15674.2449 | Lower | -112308.5801 | Lower |
| SLOC | 84 | **51 ✅** | 82 | +33 | Higher | +2 | Higher | -31 | Lower |
| PLOC | 67 | **31 ✅** | 68 | +36 | Higher | -1 | Lower | -37 | Lower |
| LLOC | 50 | **17 ✅** | 54 | +33 | Higher | -4 | Lower | -37 | Lower |
| Comments (CLOC) | 12 | **13 ✅** | 8 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | 12 | **9 ✅** | 12 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 2 | 4 | **0 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Exits | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |

## p01368

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.7768 | **55.9774 ✅** | 46.4786 | -9.2006 | Lower | +0.2982 | Higher | +9.4988 | Higher |
| MI (SEI) | 15.0743 | **41.9693 ✅** | 4.4639 | -26.8950 | Lower | +10.6104 | Higher | +37.5054 | Higher |
| Cyclomatic (sum) | 26 | **13 ✅** | 26 | +13 | Higher | 0 | Same | -13 | Lower |
| Cognitive (sum) | 61 | **16 ✅** | 45 | +45 | Higher | +16 | Higher | -29 | Lower |
| Halstead Volume | 3680.0293 | **1990.4342 ✅** | 3676.7558 | +1689.5951 | Higher | +3.2735 | Higher | -1686.3216 | Lower |
| Halstead Effort | 267456.3521 | **93628.9779 ✅** | 339732.2348 | +173827.3742 | Higher | -72275.8827 | Lower | -246103.2570 | Lower |
| SLOC | 106 | **88 ✅** | 108 | +18 | Higher | -2 | Lower | -20 | Lower |
| PLOC | 86 | **58 ✅** | 96 | +28 | Higher | -10 | Lower | -38 | Lower |
| LLOC | 67 | **32 ✅** | 78 | +35 | Higher | -11 | Lower | -46 | Lower |
| Comments (CLOC) | 8 | **23 ✅** | 2 | -15 | Lower | +6 | Higher | +21 | Higher |
| Blank Lines | 14 | 13 | **10 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01452

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7947 | **65.4634 ✅** | 55.0300 | -13.6687 | Lower | -3.2353 | Lower | +10.4334 | Higher |
| MI (SEI) | 17.4559 | **55.8518 ✅** | 13.9996 | -38.3959 | Lower | +3.4563 | Higher | +41.8522 | Higher |
| Cyclomatic (sum) | 20 | **14 ✅** | 15 | +6 | Higher | +5 | Higher | -1 | Lower |
| Cognitive (sum) | 12 | 8 | **6 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Halstead Volume | 3506.8249 | **1188.6820 ✅** | 3313.7315 | +2318.1429 | Higher | +193.0934 | Higher | -2125.0495 | Lower |
| Halstead Effort | 204162.9635 | **52715.4632 ✅** | 183601.4383 | +151447.5002 | Higher | +20561.5251 | Higher | -130885.9751 | Lower |
| SLOC | 86 | **57 ✅** | 77 | +29 | Higher | +9 | Higher | -20 | Lower |
| PLOC | 73 | **40 ✅** | 62 | +33 | Higher | +11 | Higher | -22 | Lower |
| LLOC | 37 | **27 ✅** | 34 | +10 | Higher | +3 | Higher | -7 | Lower |
| Comments (CLOC) | 4 | **15 ✅** | 1 | -11 | Lower | +3 | Higher | +14 | Higher |
| Blank Lines | 13 | **8 ✅** | 15 | +5 | Higher | -2 | Lower | -7 | Lower |
| Functions | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **5 ✅** | 6 | +2 | Higher | +1 | Higher | -1 | Lower |

## p01459

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.5072 | **48.0239 ✅** | 39.6263 | -6.5168 | Lower | +1.8809 | Higher | +8.3976 | Higher |
| MI (SEI) | -3.3083 | **23.6006 ✅** | -6.0034 | -26.9089 | Lower | +2.6951 | Higher | +29.6040 | Higher |
| Cyclomatic (sum) | **28 ✅** | 31 | 31 | -3 | Lower | -3 | Lower | 0 | Same |
| Cognitive (sum) | 47 | **43 ✅** | 62 | +4 | Higher | -15 | Lower | -19 | Lower |
| Halstead Volume | 5284.3292 | **3059.8132 ✅** | 5492.8392 | +2224.5160 | Higher | -208.5099 | Lower | -2433.0260 | Lower |
| Halstead Effort | 331054.9706 | **198725.1018 ✅** | 398230.8396 | +132329.8688 | Higher | -67175.8690 | Lower | -199505.7378 | Lower |
| SLOC | 127 | **97 ✅** | 135 | +30 | Higher | -8 | Lower | -38 | Lower |
| PLOC | 106 | **75 ✅** | 114 | +31 | Higher | -8 | Lower | -39 | Lower |
| LLOC | 63 | **44 ✅** | 90 | +19 | Higher | -27 | Lower | -46 | Lower |
| Comments (CLOC) | 2 | **13 ✅** | 2 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 19 | **14 ✅** | 19 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 7 | 1 | -6 | Lower | 0 | Same | +6 | Higher |

## p01471

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.9740 | **61.0049 ✅** | 56.0829 | -1.0309 | Lower | +3.8910 | Higher | +4.9219 | Higher |
| MI (SEI) | 34.4542 | **44.2221 ✅** | 21.4551 | -9.7679 | Lower | +12.9991 | Higher | +22.7670 | Higher |
| Cyclomatic (sum) | 14 | **13 ✅** | 17 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 22 | **9 ✅** | 19 | +13 | Higher | +3 | Higher | -10 | Lower |
| Halstead Volume | 1971.4790 | **1414.3882 ✅** | 1960.8364 | +557.0908 | Higher | +10.6426 | Higher | -546.4481 | Lower |
| Halstead Effort | 67202.7900 | **43771.5935 ✅** | 83874.7749 | +23431.1965 | Higher | -16671.9849 | Lower | -40103.1814 | Lower |
| SLOC | **68 ✅** | 72 | 83 | -4 | Lower | -15 | Lower | -11 | Lower |
| PLOC | 58 | **54 ✅** | 70 | +4 | Higher | -12 | Lower | -16 | Lower |
| LLOC | 35 | **25 ✅** | 43 | +10 | Higher | -8 | Lower | -18 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 3 | -7 | Lower | +3 | Higher | +10 | Higher |
| Blank Lines | **9 ✅** | 11 | 11 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | **1 ✅** | 3 | 4 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 4 | 3 | -4 | Lower | -3 | Lower | +1 | Higher |

## p01521

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.4134 ✅** | 72.2366 | 80.6528 | +12.1768 | Higher | +3.7606 | Higher | -8.4162 | Lower |
| MI (SEI) | 74.7715 | 68.1346 | **75.2883 ✅** | +6.6368 | Higher | -0.5168 | Lower | -7.1536 | Lower |
| Cyclomatic (sum) | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **453.9955 ✅** | 645.7255 | 673.8746 | -191.7300 | Lower | -219.8791 | Lower | -28.1491 | Lower |
| Halstead Effort | **5864.1082 ✅** | 11510.7590 | 12736.2292 | -5646.6509 | Lower | -6872.1210 | Lower | -1225.4701 | Lower |
| SLOC | **27 ✅** | 49 | 30 | -22 | Lower | -3 | Lower | +19 | Higher |
| PLOC | **19 ✅** | 28 | 21 | -9 | Lower | -2 | Lower | +7 | Higher |
| LLOC | **13 ✅** | 17 | 14 | -4 | Lower | -1 | Lower | +3 | Higher |
| Comments (CLOC) | 4 | **16 ✅** | 7 | -12 | Lower | -3 | Lower | +9 | Higher |
| Blank Lines | **4 ✅** | 9 | 5 | -5 | Lower | -1 | Lower | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01529

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.4203 | **73.5688 ✅** | 61.9734 | -13.1485 | Lower | -1.5531 | Lower | +11.5954 | Higher |
| MI (SEI) | 33.6193 | **64.7747 ✅** | 34.1470 | -31.1554 | Lower | -0.5277 | Lower | +30.6276 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 11 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 21 | **12 ✅** | 17 | +9 | Higher | +4 | Higher | -5 | Lower |
| Halstead Volume | 2176.5806 | **983.0417 ✅** | 2149.8790 | +1193.5389 | Higher | +26.7016 | Higher | -1166.8373 | Lower |
| Halstead Effort | 123584.9663 | **37511.9778 ✅** | 123147.7579 | +86072.9885 | Higher | +437.2084 | Higher | -85635.7802 | Lower |
| SLOC | 65 | **40 ✅** | 61 | +25 | Higher | +4 | Higher | -21 | Lower |
| PLOC | 52 | **27 ✅** | 48 | +25 | Higher | +4 | Higher | -21 | Lower |
| LLOC | 42 | **19 ✅** | 43 | +23 | Higher | -1 | Lower | -24 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 4 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 11 | **5 ✅** | 11 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |

## p01545

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.5987 | **66.7466 ✅** | 55.0872 | -8.1479 | Lower | +3.5115 | Higher | +11.6593 | Higher |
| MI (SEI) | 26.1367 | **58.3585 ✅** | 22.2802 | -32.2218 | Lower | +3.8564 | Higher | +36.0782 | Higher |
| Cyclomatic (sum) | 14 | **8 ✅** | 17 | +6 | Higher | -3 | Lower | -9 | Lower |
| Cognitive (sum) | 9 | **2 ✅** | 12 | +7 | Higher | -3 | Lower | -10 | Lower |
| Halstead Volume | 2454.1546 | **1087.6737 ✅** | 2466.0486 | +1366.4808 | Higher | -11.8940 | Lower | -1378.3749 | Lower |
| Halstead Effort | 145513.4084 | **23554.9344 ✅** | 152051.3647 | +121958.4740 | Higher | -6537.9563 | Lower | -128496.4303 | Lower |
| SLOC | 69 | **59 ✅** | 82 | +10 | Higher | -13 | Lower | -23 | Lower |
| PLOC | 57 | **35 ✅** | 67 | +22 | Higher | -10 | Lower | -32 | Lower |
| LLOC | 39 | **15 ✅** | 43 | +24 | Higher | -4 | Lower | -28 | Lower |
| Comments (CLOC) | 3 | **17 ✅** | 4 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | **10 ✅** | 11 | 11 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01570

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.3028 | **59.8887 ✅** | 46.5469 | -9.5860 | Lower | +3.7559 | Higher | +13.3419 | Higher |
| MI (SEI) | 6.6377 | **46.5150 ✅** | 3.6764 | -39.8773 | Lower | +2.9613 | Higher | +42.8386 | Higher |
| Cyclomatic (sum) | 19 | **12 ✅** | 20 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 23 | **11 ✅** | 24 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 3356.4392 | **1682.3757 ✅** | 3998.2098 | +1674.0635 | Higher | -641.7706 | Lower | -2315.8341 | Lower |
| Halstead Effort | 198248.8111 | **59584.1385 ✅** | 275305.3043 | +138664.6725 | Higher | -77056.4932 | Lower | -215721.1657 | Lower |
| SLOC | 97 | **74 ✅** | 114 | +23 | Higher | -17 | Lower | -40 | Lower |
| PLOC | 84 | **45 ✅** | 98 | +39 | Higher | -14 | Lower | -53 | Lower |
| LLOC | 68 | **27 ✅** | 90 | +41 | Higher | -22 | Lower | -63 | Lower |
| Comments (CLOC) | 1 | **18 ✅** | 2 | -17 | Lower | -1 | Lower | +16 | Higher |
| Blank Lines | **13 ✅** | 14 | 14 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p01600

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.8066 ✅** | 65.6080 | 64.2283 | +0.1986 | Higher | +1.5783 | Higher | +1.3797 | Higher |
| MI (SEI) | 47.0448 | **55.4112 ✅** | 32.9720 | -8.3665 | Lower | +14.0728 | Higher | +22.4393 | Higher |
| Cyclomatic (sum) | 11 | **10 ✅** | 13 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 14 | **12 ✅** | 21 | +2 | Higher | -7 | Lower | -9 | Lower |
| Halstead Volume | 1593.9515 | **1307.0610 ✅** | 1864.5836 | +286.8905 | Higher | -270.6320 | Lower | -557.5225 | Lower |
| Halstead Effort | **65297.0485 ✅** | 89206.9158 | 95640.2770 | -23909.8673 | Lower | -30343.2284 | Lower | -6433.3612 | Lower |
| SLOC | **53 ✅** | 58 | 54 | -5 | Lower | -1 | Lower | +4 | Higher |
| PLOC | 37 | **33 ✅** | 43 | +4 | Higher | -6 | Lower | -10 | Lower |
| LLOC | 25 | **24 ✅** | 30 | +1 | Higher | -5 | Lower | -6 | Lower |
| Comments (CLOC) | 7 | **15 ✅** | 2 | -8 | Lower | +5 | Higher | +13 | Higher |
| Blank Lines | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p01601

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.2484 | 73.1924 | **76.1566 ✅** | -2.9440 | Lower | -5.9082 | Lower | -2.9643 | Lower |
| MI (SEI) | 37.8671 | **68.0093 ✅** | 47.7515 | -30.1422 | Lower | -9.8844 | Lower | +20.2578 | Higher |
| Cyclomatic (sum) | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 6 | **3 ✅** | 5 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 1031.0257 | **785.3350 ✅** | 829.8336 | +245.6907 | Higher | +201.1921 | Higher | -44.4986 | Lower |
| Halstead Effort | 35332.4570 | **16148.4508 ✅** | 30572.8171 | +19184.0062 | Higher | +4759.6400 | Higher | -14424.3663 | Lower |
| SLOC | 47 | 44 | **36 ✅** | +3 | Higher | +11 | Higher | +8 | Higher |
| PLOC | 38 | **29 ✅** | 30 | +9 | Higher | +8 | Higher | -1 | Lower |
| LLOC | 27 | **19 ✅** | 22 | +8 | Higher | +5 | Higher | -3 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 1 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 9 | 7 | **6 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 4 | **1 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |

## p01656

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.5655 ✅** | 73.3656 | 75.9643 | +3.1999 | Higher | +0.6012 | Higher | -2.5987 | Lower |
| MI (SEI) | 65.0841 | **68.8099 ✅** | 66.2353 | -3.7257 | Lower | -1.1511 | Lower | +2.5746 | Higher |
| Cyclomatic (sum) | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 3 | 7 | 3 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 999.5173 | **917.7708 ✅** | 1122.0269 | +81.7465 | Higher | -122.5097 | Lower | -204.2562 | Lower |
| Halstead Effort | **18625.6198 ✅** | 19517.9256 | 19074.4581 | -892.3058 | Lower | -448.8383 | Lower | +443.4674 | Higher |
| SLOC | 35 | 42 | 35 | -7 | Lower | 0 | Same | +7 | Higher |
| PLOC | 24 | 29 | 24 | -5 | Lower | 0 | Same | +5 | Higher |
| LLOC | 20 | **16 ✅** | 20 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 7 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.7437 | **74.0557 ✅** | 68.6326 | -9.3120 | Lower | -3.8889 | Lower | +5.4231 | Higher |
| MI (SEI) | 52.9789 | **62.1361 ✅** | 57.0986 | -9.1572 | Lower | -4.1197 | Lower | +5.0375 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 10 | +5 | Higher | +3 | Higher | -2 | Lower |
| Cognitive (sum) | 26 | **7 ✅** | 13 | +19 | Higher | +13 | Higher | -6 | Lower |
| Halstead Volume | 1507.7805 | **895.1721 ✅** | 1406.7615 | +612.6085 | Higher | +101.0190 | Higher | -511.5894 | Lower |
| Halstead Effort | 73881.2458 | **22262.5398 ✅** | 59984.3097 | +51618.7060 | Higher | +13896.9360 | Higher | -37721.7699 | Lower |
| SLOC | 56 | **40 ✅** | 47 | +16 | Higher | +9 | Higher | -7 | Lower |
| PLOC | 44 | **26 ✅** | 37 | +18 | Higher | +7 | Higher | -11 | Lower |
| LLOC | 40 | **15 ✅** | 37 | +25 | Higher | +3 | Higher | -22 | Lower |
| Comments (CLOC) | **13 ✅** | 7 | 10 | +6 | Higher | +3 | Higher | -3 | Lower |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.9597 | **59.4884 ✅** | 43.8594 | -18.5287 | Lower | -2.8997 | Lower | +15.6290 | Higher |
| MI (SEI) | -13.0448 | **43.6413 ✅** | -8.9632 | -56.6861 | Lower | -4.0816 | Lower | +52.6046 | Higher |
| Cyclomatic (sum) | 35 | **12 ✅** | 34 | +23 | Higher | +1 | Higher | -22 | Lower |
| Cognitive (sum) | 31 | **13 ✅** | 34 | +18 | Higher | -3 | Lower | -21 | Lower |
| Halstead Volume | 4759.3545 | **1895.6893 ✅** | 5103.5793 | +2863.6652 | Higher | -344.2248 | Lower | -3207.8900 | Lower |
| Halstead Effort | 365756.3906 | **82031.6441 ✅** | 454818.9745 | +283724.7465 | Higher | -89062.5839 | Lower | -372787.3304 | Lower |
| SLOC | 123 | **73 ✅** | 102 | +50 | Higher | +21 | Higher | -29 | Lower |
| PLOC | 103 | **47 ✅** | 84 | +56 | Higher | +19 | Higher | -37 | Lower |
| LLOC | 61 | **26 ✅** | 68 | +35 | Higher | -7 | Lower | -42 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 0 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 20 | **12 ✅** | 18 | +8 | Higher | +2 | Higher | -6 | Lower |
| Functions | 5 | **3 ✅** | 4 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **4 ✅** | 8 | +5 | Higher | +1 | Higher | -4 | Lower |

## p01725

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.8558 | **44.3446 ✅** | 41.0990 | -2.4888 | Lower | +0.7567 | Higher | +3.2456 | Higher |
| MI (SEI) | 5.1623 | **16.1971 ✅** | 6.0162 | -11.0348 | Lower | -0.8538 | Lower | +10.1810 | Higher |
| Cyclomatic (sum) | 29 | 30 | 29 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 49 | 49 | **46 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 4292.5038 | **3019.5258 ✅** | 4520.8284 | +1272.9779 | Higher | -228.3247 | Lower | -1501.3026 | Lower |
| Halstead Effort | 227230.1597 | **156569.2771 ✅** | 295224.9192 | +70660.8826 | Higher | -67994.7595 | Lower | -138655.6421 | Lower |
| SLOC | 131 | **124 ✅** | 135 | +7 | Higher | -4 | Lower | -11 | Lower |
| PLOC | 113 | **90 ✅** | 121 | +23 | Higher | -8 | Lower | -31 | Lower |
| LLOC | 59 | **46 ✅** | 79 | +13 | Higher | -20 | Lower | -33 | Lower |
| Comments (CLOC) | 7 | **14 ✅** | 9 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 16 | 20 | **12 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| Functions | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |

## p01767

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.1035 | **67.5743 ✅** | 60.5639 | -9.4709 | Lower | -2.4605 | Lower | +7.0104 | Higher |
| MI (SEI) | 33.6810 | **65.6015 ✅** | 22.5468 | -31.9205 | Lower | +11.1342 | Higher | +43.0547 | Higher |
| Cyclomatic (sum) | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 2175.9819 | **1010.9352 ✅** | 2117.2732 | +1165.0467 | Higher | +58.7087 | Higher | -1106.3381 | Lower |
| Halstead Effort | 91140.1643 | **35612.4890 ✅** | 98359.7961 | +55527.6753 | Higher | -7219.6318 | Lower | -62747.3071 | Lower |
| SLOC | 75 | **55 ✅** | 65 | +20 | Higher | +10 | Higher | -10 | Lower |
| PLOC | 52 | **34 ✅** | 52 | +18 | Higher | 0 | Same | -18 | Lower |
| LLOC | 36 | **21 ✅** | 36 | +15 | Higher | 0 | Same | -15 | Lower |
| Comments (CLOC) | 8 | **24 ✅** | 1 | -16 | Lower | +7 | Higher | +23 | Higher |
| Blank Lines | 15 | **6 ✅** | 12 | +9 | Higher | +3 | Higher | -6 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p01768

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.4663 ✅** | 52.4862 | 48.3600 | +0.9801 | Higher | +5.1063 | Higher | +4.1262 | Higher |
| MI (SEI) | 3.0638 | **22.6747 ✅** | 9.7083 | -19.6108 | Lower | -6.6444 | Lower | +12.9664 | Higher |
| Cyclomatic (sum) | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | **7 ✅** | 11 | 11 | -4 | Lower | -4 | Lower | 0 | Same |
| Halstead Volume | 2927.5448 | **1998.6585 ✅** | 2945.7289 | +928.8863 | Higher | -18.1841 | Lower | -947.0704 | Lower |
| Halstead Effort | 89317.7343 | **57186.6158 ✅** | 131277.0475 | +32131.1184 | Higher | -41959.3132 | Lower | -74090.4316 | Lower |
| SLOC | **87 ✅** | 103 | 119 | -16 | Lower | -32 | Lower | -16 | Lower |
| PLOC | **70 ✅** | 78 | 101 | -8 | Lower | -31 | Lower | -23 | Lower |
| LLOC | 35 | **28 ✅** | 57 | +7 | Higher | -22 | Lower | -29 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 4 | -8 | Lower | -4 | Lower | +4 | Higher |
| Blank Lines | 17 | 17 | 18 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p01782

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 26.6540 | **61.1428 ✅** | 24.7823 | -34.4888 | Lower | +1.8717 | Higher | +36.3606 | Higher |
| MI (SEI) | -25.4085 | **46.9842 ✅** | -28.6495 | -72.3927 | Lower | +3.2410 | Higher | +75.6337 | Higher |
| Cyclomatic (sum) | 44 | **11 ✅** | 43 | +33 | Higher | +1 | Higher | -32 | Lower |
| Cognitive (sum) | 51 | **6 ✅** | 51 | +45 | Higher | 0 | Same | -45 | Lower |
| Halstead Volume | 8180.1688 | **1504.7422 ✅** | 8332.2024 | +6675.4265 | Higher | -152.0337 | Lower | -6827.4602 | Lower |
| Halstead Effort | 757647.2310 | **34784.6247 ✅** | 754897.5397 | +722862.6063 | Higher | +2749.6914 | Higher | -720112.9149 | Lower |
| SLOC | 220 | **72 ✅** | 249 | +148 | Higher | -29 | Lower | -177 | Lower |
| PLOC | 190 | **45 ✅** | 218 | +145 | Higher | -28 | Lower | -173 | Lower |
| LLOC | 125 | **20 ✅** | 150 | +105 | Higher | -25 | Lower | -130 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 2 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 28 | **13 ✅** | 29 | +15 | Higher | -1 | Lower | -16 | Lower |
| Functions | 13 | **4 ✅** | 13 | +9 | Higher | 0 | Same | -9 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |

## p01841

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.5035 | **51.5648 ✅** | 46.2368 | -4.0613 | Lower | +1.2667 | Higher | +5.3279 | Higher |
| MI (SEI) | 2.3991 | **28.2720 ✅** | 8.9132 | -25.8729 | Lower | -6.5141 | Lower | +19.3588 | Higher |
| Cyclomatic (sum) | 20 | 23 | 20 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | **15 ✅** | 22 | 16 | -7 | Lower | -1 | Lower | +6 | Higher |
| Halstead Volume | 4297.7052 | **2433.0366 ✅** | 4129.9268 | +1864.6686 | Higher | +167.7784 | Higher | -1696.8902 | Lower |
| Halstead Effort | 281013.1550 | **80290.2077 ✅** | 225396.0040 | +200722.9473 | Higher | +55617.1510 | Higher | -145105.7963 | Lower |
| SLOC | 105 | **94 ✅** | 115 | +11 | Higher | -10 | Lower | -21 | Lower |
| PLOC | 89 | **77 ✅** | 94 | +12 | Higher | -5 | Lower | -17 | Lower |
| LLOC | 79 | **37 ✅** | 76 | +42 | Higher | +3 | Higher | -39 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 5 | -12 | Lower | -4 | Lower | +8 | Higher |
| Blank Lines | 16 | **12 ✅** | 16 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p01847

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.7712 | 51.8545 | **54.3860 ✅** | -1.0833 | Lower | -3.6148 | Lower | -2.5315 | Lower |
| MI (SEI) | 8.4136 | **29.9159 ✅** | 17.2542 | -21.5023 | Lower | -8.8406 | Lower | +12.6617 | Higher |
| Cyclomatic (sum) | 26 | **19 ✅** | 21 | +7 | Higher | +5 | Higher | -2 | Lower |
| Cognitive (sum) | 62 | **22 ✅** | 50 | +40 | Higher | +12 | Higher | -28 | Lower |
| Halstead Volume | 3048.1537 | **2657.4567 ✅** | 2876.4076 | +390.6970 | Higher | +171.7461 | Higher | -218.9509 | Lower |
| Halstead Effort | 192262.2949 | **104254.0720 ✅** | 170786.7018 | +88008.2228 | Higher | +21475.5931 | Higher | -66532.6297 | Lower |
| SLOC | 88 | 95 | **77 ✅** | -7 | Lower | +11 | Higher | +18 | Higher |
| PLOC | 71 | 73 | **63 ✅** | -2 | Lower | +8 | Higher | +10 | Higher |
| LLOC | 72 | **41 ✅** | 79 | +31 | Higher | -7 | Lower | -38 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 2 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | 17 | 12 | 12 | +5 | Higher | +5 | Higher | 0 | Same |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 6 | 2 | -5 | Lower | -1 | Lower | +4 | Higher |

## p01907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.6543 | **60.9391 ✅** | 39.4787 | -10.2848 | Lower | +11.1756 | Higher | +21.4604 | Higher |
| MI (SEI) | 9.7170 | **36.4125 ✅** | -4.1650 | -26.6955 | Lower | +13.8821 | Higher | +40.5776 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 23 | +5 | Higher | -7 | Lower | -12 | Lower |
| Cognitive (sum) | 13 | **5 ✅** | 25 | +8 | Higher | -12 | Lower | -20 | Lower |
| Halstead Volume | 2971.3062 | **1499.0527 ✅** | 5029.6663 | +1472.2535 | Higher | -2058.3601 | Lower | -3530.6135 | Lower |
| Halstead Effort | 118026.8853 | **38038.4631 ✅** | 333395.0212 | +79988.4222 | Higher | -215368.1359 | Lower | -295356.5581 | Lower |
| SLOC | 103 | **73 ✅** | 157 | +30 | Higher | -54 | Lower | -84 | Lower |
| PLOC | 84 | **56 ✅** | 130 | +28 | Higher | -46 | Lower | -74 | Lower |
| LLOC | 47 | **20 ✅** | 119 | +27 | Higher | -72 | Lower | -99 | Lower |
| Comments (CLOC) | 2 | **7 ✅** | 4 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 19 | **14 ✅** | 25 | +5 | Higher | -6 | Lower | -11 | Lower |
| Functions | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p01916

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.3254 ✅** | 83.4029 | 83.6991 | +0.9226 | Higher | +0.6263 | Higher | -0.2963 | Lower |
| MI (SEI) | 80.3336 | **81.3722 ✅** | 81.0363 | -1.0385 | Lower | -0.7027 | Lower | +0.3358 | Higher |
| Cyclomatic (sum) | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 542.8367 | **482.1526 ✅** | 544.3964 | +60.6841 | Higher | -1.5598 | Lower | -62.2438 | Lower |
| Halstead Effort | 7328.2950 | **5464.3962 ✅** | 7992.7294 | +1863.8988 | Higher | -664.4344 | Lower | -2528.3332 | Lower |
| SLOC | **26 ✅** | 29 | 27 | -3 | Lower | -1 | Lower | +2 | Higher |
| PLOC | **14 ✅** | 16 | 18 | -2 | Lower | -4 | Lower | -2 | Lower |
| LLOC | 10 | 10 | 14 | 0 | Same | -4 | Lower | -4 | Lower |
| Comments (CLOC) | 6 | **8 ✅** | 7 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01917

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.0944 | **70.0775 ✅** | 54.0751 | -15.9831 | Lower | +0.0193 | Higher | +16.0025 | Higher |
| MI (SEI) | 4.5809 | **56.9061 ✅** | 4.5530 | -52.3252 | Lower | +0.0279 | Higher | +52.3531 | Higher |
| Cyclomatic (sum) | 22 | **11 ✅** | 22 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 47 | **7 ✅** | 47 | +40 | Higher | 0 | Same | -40 | Lower |
| Halstead Volume | 2796 | **1167.2572 ✅** | 2806.4234 | +1628.7428 | Higher | -10.4234 | Lower | -1639.1662 | Lower |
| Halstead Effort | 173131.2632 | **53110.2009 ✅** | 169320.8785 | +120021.0623 | Higher | +3810.3846 | Higher | -116210.6776 | Lower |
| SLOC | 78 | **45 ✅** | 78 | +33 | Higher | 0 | Same | -33 | Lower |
| PLOC | 70 | **30 ✅** | 70 | +40 | Higher | 0 | Same | -40 | Lower |
| LLOC | 56 | **14 ✅** | 56 | +42 | Higher | 0 | Same | -42 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.0903 | **60.4460 ✅** | 57.5576 | -5.3558 | Lower | -2.4673 | Lower | +2.8885 | Higher |
| MI (SEI) | 5.5086 | **48.2563 ✅** | 29.7948 | -42.7477 | Lower | -24.2862 | Lower | +18.4615 | Higher |
| Cyclomatic (sum) | 17 | **12 ✅** | 16 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 3122.8684 | **1576.8392 ✅** | 3305.4923 | +1546.0293 | Higher | -182.6239 | Lower | -1728.6531 | Lower |
| Halstead Effort | 256412.8193 | **81263.5332 ✅** | 307043.5091 | +175149.2861 | Higher | -50630.6898 | Lower | -225779.9759 | Lower |
| SLOC | 76 | 73 | **65 ✅** | +3 | Higher | +11 | Higher | +8 | Higher |
| PLOC | 62 | **51 ✅** | 56 | +11 | Higher | +6 | Higher | -5 | Lower |
| LLOC | 42 | **41 ✅** | 44 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | 0 | **19 ✅** | 5 | -19 | Lower | -5 | Lower | +14 | Higher |
| Blank Lines | 14 | 17 | **9 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| Functions | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.5621 | **72.5745 ✅** | 72.0588 | -5.0124 | Lower | -4.4966 | Lower | +0.5158 | Higher |
| MI (SEI) | 49.8877 | **63.3099 ✅** | 42.6541 | -13.4222 | Lower | +7.2336 | Higher | +20.6558 | Higher |
| Cyclomatic (sum) | 18 | **11 ✅** | 16 | +7 | Higher | +2 | Higher | -5 | Lower |
| Cognitive (sum) | 10 | 8 | **7 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Halstead Volume | 1213.2411 | **965.0994 ✅** | 1281.0393 | +248.1417 | Higher | -67.7981 | Lower | -315.9399 | Lower |
| Halstead Effort | 38667.1687 | **23062.5479 ✅** | 29989.4577 | +15604.6208 | Higher | +8677.7110 | Higher | -6926.9097 | Lower |
| SLOC | 47 | 41 | **36 ✅** | +6 | Higher | +11 | Higher | +5 | Higher |
| PLOC | 32 | **27 ✅** | 29 | +5 | Higher | +3 | Higher | -2 | Lower |
| LLOC | 19 | 17 | 17 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 6 | **9 ✅** | 1 | -3 | Lower | +5 | Higher | +8 | Higher |
| Blank Lines | 9 | 7 | 7 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 8 | **0 ✅** | 7 | +8 | Higher | +1 | Higher | -7 | Lower |

## p02045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.9684 | 57.3860 | **59.2515 ✅** | -9.4176 | Lower | -11.2831 | Lower | -1.8655 | Lower |
| MI (SEI) | 11.3223 | **39.2293 ✅** | 11.5120 | -27.9070 | Lower | -0.1897 | Lower | +27.7173 | Higher |
| Cyclomatic (sum) | 26 | 17 | 17 | +9 | Higher | +9 | Higher | 0 | Same |
| Cognitive (sum) | 23 | **7 ✅** | 13 | +16 | Higher | +10 | Higher | -6 | Lower |
| Halstead Volume | 3401.7521 | **2276.7023 ✅** | 2516.7121 | +1125.0497 | Higher | +885.0400 | Higher | -240.0098 | Lower |
| Halstead Effort | 214666.3780 | **79926.7838 ✅** | 124607.9394 | +134739.5942 | Higher | +90058.4386 | Higher | -44681.1556 | Lower |
| SLOC | 101 | 73 | **63 ✅** | +28 | Higher | +38 | Higher | +10 | Higher |
| PLOC | 86 | **48 ✅** | 54 | +38 | Higher | +32 | Higher | -6 | Lower |
| LLOC | 68 | **31 ✅** | 49 | +37 | Higher | +19 | Higher | -18 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 0 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 12 | 13 | **9 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 6 | 2 | 2 | +4 | Higher | +4 | Higher | 0 | Same |
| Exits | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |

## p02046

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.8854 ✅** | 74.2880 | 75.7507 | +4.5974 | Higher | +3.1348 | Higher | -1.4627 | Lower |
| MI (SEI) | 53.7589 | **71.6182 ✅** | 48.1273 | -17.8593 | Lower | +5.6316 | Higher | +23.4910 | Higher |
| Cyclomatic (sum) | 12 | **8 ✅** | 12 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 11 | **9 ✅** | 12 | +2 | Higher | -1 | Lower | -3 | Lower |
| Halstead Volume | 900.0687 | **792.6707 ✅** | 985.7896 | +107.3981 | Higher | -85.7208 | Lower | -193.1189 | Lower |
| Halstead Effort | 32307.7304 | **23988.7174 ✅** | 38238.2585 | +8319.0131 | Higher | -5930.5281 | Lower | -14249.5412 | Lower |
| SLOC | **28 ✅** | 41 | 33 | -13 | Lower | -5 | Lower | +8 | Higher |
| PLOC | 25 | **23 ✅** | 28 | +2 | Higher | -3 | Lower | -5 | Lower |
| LLOC | **10 ✅** | 12 | 22 | -2 | Lower | -12 | Lower | -10 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 1 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | **3 ✅** | 7 | 5 | -4 | Lower | -2 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.2160 | **49.9700 ✅** | 21.8095 | -7.7540 | Lower | +20.4065 | Higher | +28.1605 | Higher |
| MI (SEI) | -2.6000 | **22.8829 ✅** | -39.9600 | -25.4830 | Lower | +37.3599 | Higher | +62.8429 | Higher |
| Cyclomatic (sum) | 26 | **21 ✅** | 42 | +5 | Higher | -16 | Lower | -21 | Lower |
| Cognitive (sum) | 14 | **11 ✅** | 40 | +3 | Higher | -26 | Lower | -29 | Lower |
| Halstead Volume | 4684.0199 | **2800.6141 ✅** | 11714.2585 | +1883.4058 | Higher | -7030.2385 | Lower | -8913.6443 | Lower |
| Halstead Effort | 271673.1554 | **108431.3251 ✅** | 1552432.1037 | +163241.8304 | Higher | -1280758.9483 | Lower | -1444000.7787 | Lower |
| SLOC | 130 | **102 ✅** | 272 | +28 | Higher | -142 | Lower | -170 | Lower |
| PLOC | 111 | **81 ✅** | 252 | +30 | Higher | -141 | Lower | -171 | Lower |
| LLOC | 62 | **33 ✅** | 206 | +29 | Higher | -144 | Lower | -173 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 0 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 19 | **18 ✅** | 20 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 6 | 6 | 11 | 0 | Same | -5 | Lower | -5 | Lower |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | **4 ✅** | 7 | 6 | -3 | Lower | -2 | Lower | +1 | Higher |

## p02061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.3984 ✅** | 54.5253 | 56.9327 | +3.8731 | Higher | +1.4657 | Higher | -2.4074 | Lower |
| MI (SEI) | 10.0776 | **37.9088 ✅** | 16.7445 | -27.8312 | Lower | -6.6669 | Lower | +21.1643 | Higher |
| Cyclomatic (sum) | 15 | 20 | 15 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 25 | 25 | 25 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 2232.3208 | **1625.4956 ✅** | 2298.3198 | +606.8252 | Higher | -65.9990 | Lower | -672.8242 | Lower |
| Halstead Effort | 133598.2003 | **62697.6871 ✅** | 156213.9223 | +70900.5132 | Higher | -22615.7220 | Lower | -93516.2351 | Lower |
| SLOC | **71 ✅** | 93 | 77 | -22 | Lower | -6 | Lower | +16 | Higher |
| PLOC | **60 ✅** | 66 | 68 | -6 | Lower | -8 | Lower | -2 | Lower |
| LLOC | 51 | **38 ✅** | 59 | +13 | Higher | -8 | Lower | -21 | Lower |
| Comments (CLOC) | 0 | **20 ✅** | 1 | -20 | Lower | -1 | Lower | +19 | Higher |
| Blank Lines | 11 | 11 | **9 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 4 | **0 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p02104

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.5261 | **84.6799 ✅** | 60.8099 | -22.1537 | Lower | +1.7162 | Higher | +23.8700 | Higher |
| MI (SEI) | 43.0661 | **82.8749 ✅** | 29.5688 | -39.8087 | Lower | +13.4974 | Higher | +53.3061 | Higher |
| Cyclomatic (sum) | 11 | **4 ✅** | 13 | +7 | Higher | -2 | Lower | -9 | Lower |
| Cognitive (sum) | 14 | **2 ✅** | 15 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 1664.5284 | **530 ✅** | 1925.6463 | +1134.5284 | Higher | -261.1179 | Lower | -1395.6463 | Lower |
| Halstead Effort | 79183.9938 | **7626.1111 ✅** | 109521.1353 | +71557.8827 | Higher | -30337.1415 | Lower | -101895.0241 | Lower |
| SLOC | 64 | **26 ✅** | 66 | +38 | Higher | -2 | Lower | -40 | Lower |
| PLOC | 44 | **16 ✅** | 52 | +28 | Higher | -8 | Lower | -36 | Lower |
| LLOC | 41 | **12 ✅** | 48 | +29 | Higher | -7 | Lower | -36 | Lower |
| Comments (CLOC) | **9 ✅** | 7 | 3 | +2 | Higher | +6 | Higher | +4 | Higher |
| Blank Lines | 11 | **4 ✅** | 12 | +7 | Higher | -1 | Lower | -8 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.3249 | **64.8485 ✅** | 64.0944 | -1.5236 | Lower | -0.7694 | Lower | +0.7542 | Higher |
| MI (SEI) | 51.5206 | **51.7123 ✅** | 49.4818 | -0.1918 | Lower | +2.0388 | Higher | +2.2305 | Higher |
| Cyclomatic (sum) | 11 | 14 | **10 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 22 | **14 ✅** | 21 | +8 | Higher | +1 | Higher | -7 | Lower |
| Halstead Volume | **1657.8086 ✅** | 1678.2807 | 1950.7077 | -20.4721 | Lower | -292.8990 | Lower | -272.4270 | Lower |
| Halstead Effort | **54470.8547 ✅** | 80167.8724 | 82682.1380 | -25697.0177 | Lower | -28211.2833 | Lower | -2514.2656 | Lower |
| SLOC | 61 | **53 ✅** | 56 | +8 | Higher | +5 | Higher | -3 | Lower |
| PLOC | 40 | **37 ✅** | 48 | +3 | Higher | -8 | Lower | -11 | Lower |
| LLOC | 31 | **21 ✅** | 46 | +10 | Higher | -15 | Lower | -25 | Lower |
| Comments (CLOC) | **15 ✅** | 11 | 11 | +4 | Higher | +4 | Higher | 0 | Same |
| Blank Lines | 12 | **7 ✅** | 8 | +5 | Higher | +4 | Higher | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.4929 | **37.5063 ✅** | 28.3915 | -0.0134 | Lower | +9.1014 | Higher | +9.1149 | Higher |
| MI (SEI) | -11.3379 | **-11.0131 ✅** | -20.6269 | -0.3248 | Lower | +9.2890 | Higher | +9.6138 | Higher |
| Cyclomatic (sum) | **37 ✅** | 40 | 46 | -3 | Lower | -9 | Lower | -6 | Lower |
| Cognitive (sum) | 42 | **32 ✅** | 50 | +10 | Higher | -8 | Lower | -18 | Lower |
| Halstead Volume | 5544.8055 | **4843.2543 ✅** | 7214.2199 | +701.5512 | Higher | -1669.4144 | Lower | -2370.9655 | Lower |
| Halstead Effort | 342444.3800 | **260790.6183 ✅** | 681397.8888 | +81653.7617 | Higher | -338953.5088 | Lower | -420607.2705 | Lower |
| SLOC | 141 | 141 | 200 | 0 | Same | -59 | Lower | -59 | Lower |
| PLOC | 121 | **118 ✅** | 172 | +3 | Higher | -51 | Lower | -54 | Lower |
| LLOC | 74 | **42 ✅** | 117 | +32 | Higher | -43 | Lower | -75 | Lower |
| Comments (CLOC) | 1 | 1 | **3 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | **20 ✅** | 23 | 26 | -3 | Lower | -6 | Lower | -3 | Lower |
| Functions | **4 ✅** | 6 | 11 | -2 | Lower | -7 | Lower | -5 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **4 ✅** | 7 | 7 | -3 | Lower | -3 | Lower | 0 | Same |

## p02162

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.2652 | **72.7845 ✅** | 70.6586 | -2.5193 | Lower | -0.3934 | Lower | +2.1260 | Higher |
| MI (SEI) | 56.3468 | **62.2587 ✅** | 56.6302 | -5.9119 | Lower | -0.2834 | Lower | +5.6285 | Higher |
| Cyclomatic (sum) | 11 | **10 ✅** | 14 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 920.8124 | **724.9246 ✅** | 980.4320 | +195.8878 | Higher | -59.6196 | Lower | -255.5074 | Lower |
| Halstead Effort | 18098.7268 | **14208.5223 ✅** | 24131.9971 | +3890.2044 | Higher | -6033.2704 | Lower | -9923.4748 | Lower |
| SLOC | 48 | 45 | **44 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| PLOC | 35 | **31 ✅** | 33 | +4 | Higher | +2 | Higher | -2 | Lower |
| LLOC | **11 ✅** | 12 | 14 | -1 | Lower | -3 | Lower | -2 | Lower |
| Comments (CLOC) | 8 | **9 ✅** | 7 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 7 | 5 | 5 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p02166

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.8070 | **67.7181 ✅** | 66.6053 | -8.9111 | Lower | -7.7983 | Lower | +1.1129 | Higher |
| MI (SEI) | 39.1141 | **57.3533 ✅** | 21.7139 | -18.2392 | Lower | +17.4002 | Higher | +35.6394 | Higher |
| Cyclomatic (sum) | 23 | **12 ✅** | 13 | +11 | Higher | +10 | Higher | -1 | Lower |
| Cognitive (sum) | 20 | 12 | **11 ✅** | +8 | Higher | +9 | Higher | +1 | Higher |
| Halstead Volume | 2102.3441 | **1190.3185 ✅** | 1819.3223 | +912.0256 | Higher | +283.0218 | Higher | -629.0038 | Lower |
| Halstead Effort | 107994.0956 | **32115.7084 ✅** | 82779.1648 | +75878.3872 | Higher | +25214.9308 | Higher | -50663.4564 | Lower |
| SLOC | 63 | 51 | **47 ✅** | +12 | Higher | +16 | Higher | +4 | Higher |
| PLOC | 54 | **35 ✅** | 41 | +19 | Higher | +13 | Higher | -6 | Lower |
| LLOC | 34 | **21 ✅** | 33 | +13 | Higher | +1 | Higher | -12 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 0 | -3 | Lower | +9 | Higher | +12 | Higher |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 5 | 1 | **0 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |

## p02186

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.3419 | **57.9583 ✅** | 37.7791 | -19.6164 | Lower | +0.5628 | Higher | +20.1792 | Higher |
| MI (SEI) | -18.0433 | **39.4714 ✅** | -18.6516 | -57.5147 | Lower | +0.6083 | Higher | +58.1230 | Higher |
| Cyclomatic (sum) | 23 | **13 ✅** | 25 | +10 | Higher | -2 | Lower | -12 | Lower |
| Cognitive (sum) | 12 | **9 ✅** | 14 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | 6646.3531 | **2333.1361 ✅** | 7207.4913 | +4313.2170 | Higher | -561.1382 | Lower | -4874.3552 | Lower |
| Halstead Effort | 570132.4765 | **141932.4479 ✅** | 716473.5004 | +428200.0286 | Higher | -146341.0239 | Lower | -574541.0525 | Lower |
| SLOC | 154 | **74 ✅** | 151 | +80 | Higher | +3 | Higher | -77 | Lower |
| PLOC | 126 | **59 ✅** | 130 | +67 | Higher | -4 | Lower | -71 | Lower |
| LLOC | 71 | **46 ✅** | 77 | +25 | Higher | -6 | Lower | -31 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 28 | **10 ✅** | 21 | +18 | Higher | +7 | Higher | -11 | Lower |
| Functions | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 11 | **2 ✅** | 13 | +9 | Higher | -2 | Lower | -11 | Lower |

## p02190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.1518 | **82.1176 ✅** | 74.4064 | -4.9658 | Lower | +2.7455 | Higher | +7.7112 | Higher |
| MI (SEI) | 73.0277 | **75.5660 ✅** | 62.1204 | -2.5382 | Lower | +10.9073 | Higher | +13.4455 | Higher |
| Cyclomatic (sum) | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 691.1242 | **617.3437 ✅** | 846.4973 | +73.7805 | Higher | -155.3731 | Lower | -229.1536 | Lower |
| Halstead Effort | 10817.5963 | **6814.5242 ✅** | 17758.8080 | +4003.0721 | Higher | -6941.2117 | Lower | -10944.2837 | Lower |
| SLOC | 38 | **29 ✅** | 41 | +9 | Higher | -3 | Lower | -12 | Lower |
| PLOC | 19 | **17 ✅** | 25 | +2 | Higher | -6 | Lower | -8 | Lower |
| LLOC | 13 | **11 ✅** | 18 | +2 | Higher | -5 | Lower | -7 | Lower |
| Comments (CLOC) | **11 ✅** | 6 | 7 | +5 | Higher | +4 | Higher | -1 | Lower |
| Blank Lines | 8 | **6 ✅** | 9 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02205

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.3592 | **81.6716 ✅** | 80.8524 | -3.3124 | Lower | -2.4932 | Lower | +0.8192 | Higher |
| MI (SEI) | 76.3292 | **88.6941 ✅** | 68.1516 | -12.3648 | Lower | +8.1777 | Higher | +20.5425 | Higher |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 774.8300 | **358.2030 ✅** | 708.4702 | +416.6269 | Higher | +66.3597 | Higher | -350.2672 | Lower |
| Halstead Effort | 13656.3780 | **5117.1859 ✅** | 14523.6394 | +8539.1921 | Higher | -867.2614 | Lower | -9406.4535 | Lower |
| SLOC | 34 | 35 | **30 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| PLOC | 19 | **17 ✅** | 21 | +2 | Higher | -2 | Lower | -4 | Lower |
| LLOC | 16 | **10 ✅** | 20 | +6 | Higher | -4 | Lower | -10 | Lower |
| Comments (CLOC) | 11 | **20 ✅** | 4 | -9 | Lower | +7 | Higher | +16 | Higher |
| Blank Lines | 7 | **5 ✅** | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02242

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.4814 ✅** | 48.1088 | 48.8637 | +4.3727 | Higher | +3.6178 | Higher | -0.7549 | Lower |
| MI (SEI) | 10.1346 | **19.4601 ✅** | 5.0550 | -9.3255 | Lower | +5.0796 | Higher | +14.4050 | Higher |
| Cyclomatic (sum) | **20 ✅** | 25 | 25 | -5 | Lower | -5 | Lower | 0 | Same |
| Cognitive (sum) | **17 ✅** | 30 | 23 | -13 | Lower | -6 | Lower | +7 | Higher |
| Halstead Volume | **2576.9231 ✅** | 3255.7930 | 3087.4893 | -678.8698 | Lower | -510.5662 | Lower | +168.3037 | Higher |
| Halstead Effort | **101197.9194 ✅** | 122092.2363 | 169009.1651 | -20894.3169 | Lower | -67811.2457 | Lower | -46916.9288 | Lower |
| SLOC | **91 ✅** | 103 | 100 | -12 | Lower | -9 | Lower | +3 | Higher |
| PLOC | **81 ✅** | 85 | 90 | -4 | Lower | -9 | Lower | -5 | Lower |
| LLOC | **51 ✅** | 53 | 67 | -2 | Lower | -16 | Lower | -14 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 1 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 10 | 16 | 10 | -6 | Lower | 0 | Same | +6 | Higher |
| Functions | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | **1 ✅** | 3 | 3 | -2 | Lower | -2 | Lower | 0 | Same |

## p02249

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.9157 | **62.7715 ✅** | 48.8957 | -13.8558 | Lower | +0.0200 | Higher | +13.8759 | Higher |
| MI (SEI) | 8.0306 | **46.4623 ✅** | 8.0016 | -38.4318 | Lower | +0.0289 | Higher | +38.4607 | Higher |
| Cyclomatic (sum) | 22 | **11 ✅** | 22 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 33 | **13 ✅** | 36 | +20 | Higher | -3 | Lower | -23 | Lower |
| Halstead Volume | 3601.4830 | **1376.6255 ✅** | 3615.3892 | +2224.8574 | Higher | -13.9062 | Lower | -2238.7636 | Lower |
| Halstead Effort | 234824.3512 | **38447.1846 ✅** | 234569.8918 | +196377.1667 | Higher | +254.4594 | Higher | -196122.7072 | Lower |
| SLOC | 99 | **67 ✅** | 99 | +32 | Higher | 0 | Same | -32 | Lower |
| PLOC | 82 | **49 ✅** | 82 | +33 | Higher | 0 | Same | -33 | Lower |
| LLOC | 59 | **14 ✅** | 65 | +45 | Higher | -6 | Lower | -51 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 15 | **8 ✅** | 15 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 4 | 3 | **2 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 3 | **0 ✅** | 0 | Same | +3 | Higher | +3 | Higher |

## p02256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.6418 | **65.4989 ✅** | 64.1428 | -1.8571 | Lower | -0.5010 | Lower | +1.3561 | Higher |
| MI (SEI) | **52.6136 ✅** | 52.4001 | 47.1995 | +0.2135 | Higher | +5.4140 | Higher | +5.2006 | Higher |
| Cyclomatic (sum) | 15 | 14 | **12 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 15 | **8 ✅** | 12 | +7 | Higher | +3 | Higher | -4 | Lower |
| Halstead Volume | 1375.9311 | 1397.1893 | **1288.2994 ✅** | -21.2583 | Lower | +87.6317 | Higher | +108.8900 | Higher |
| Halstead Effort | 45484.3497 | 42206.7611 | **39848.4324 ✅** | +3277.5886 | Higher | +5635.9173 | Higher | +2358.3287 | Higher |
| SLOC | 60 | **54 ✅** | 62 | +6 | Higher | -2 | Lower | -8 | Lower |
| PLOC | 47 | **40 ✅** | 48 | +7 | Higher | -1 | Lower | -8 | Lower |
| LLOC | 26 | **22 ✅** | 28 | +4 | Higher | -2 | Lower | -6 | Lower |
| Comments (CLOC) | **15 ✅** | 11 | 10 | +4 | Higher | +5 | Higher | +1 | Higher |
| Blank Lines | 10 | **8 ✅** | 13 | +2 | Higher | -3 | Lower | -5 | Lower |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 7 | 4 | -6 | Lower | -3 | Lower | +3 | Higher |

## p02266

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.6299 | **66.1945 ✅** | 53.1108 | -12.5646 | Lower | +0.5192 | Higher | +13.0837 | Higher |
| MI (SEI) | 11.1727 | **47.7149 ✅** | 10.5063 | -36.5422 | Lower | +0.6664 | Higher | +37.2086 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 16 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 41 | **8 ✅** | 28 | +33 | Higher | +13 | Higher | -20 | Lower |
| Halstead Volume | 2087.6195 | **1157.1748 ✅** | 2463.1734 | +930.4448 | Higher | -375.5538 | Lower | -1305.9986 | Lower |
| Halstead Effort | 69480.2607 | **25383.1885 ✅** | 93400.8716 | +44097.0722 | Higher | -23920.6109 | Lower | -68017.6831 | Lower |
| SLOC | 96 | **59 ✅** | 94 | +37 | Higher | +2 | Higher | -35 | Lower |
| PLOC | 87 | **45 ✅** | 87 | +42 | Higher | 0 | Same | -42 | Lower |
| LLOC | 53 | **28 ✅** | 58 | +25 | Higher | -5 | Lower | -30 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 1 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 9 | 8 | **7 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02276

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.1557 | 60.0870 | **69.0079 ✅** | +7.0687 | Higher | -1.8522 | Lower | -8.9209 | Lower |
| MI (SEI) | 45.3085 | 45.3751 | **52.6576 ✅** | -0.0666 | Lower | -7.3491 | Lower | -7.2825 | Lower |
| Cyclomatic (sum) | 10 | 15 | **9 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Cognitive (sum) | 10 | 17 | 10 | -7 | Lower | 0 | Same | +7 | Higher |
| Halstead Volume | 1363.9083 | 1686.2630 | **1281.1554 ✅** | -322.3546 | Lower | +82.7530 | Higher | +405.1076 | Higher |
| Halstead Effort | 49970.7796 | 65080.6364 | **44938.9878 ✅** | -15109.8568 | Lower | +5031.7918 | Higher | +20141.6485 | Higher |
| SLOC | 52 | 70 | **48 ✅** | -18 | Lower | +4 | Higher | +22 | Higher |
| PLOC | **37 ✅** | 49 | 41 | -12 | Lower | -4 | Lower | +8 | Higher |
| LLOC | **25 ✅** | 34 | 26 | -9 | Lower | -1 | Lower | +8 | Higher |
| Comments (CLOC) | 5 | **15 ✅** | 7 | -10 | Lower | -2 | Lower | +8 | Higher |
| Blank Lines | 10 | 11 | **7 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.0431 | 57.3307 | **58.3747 ✅** | -0.2876 | Lower | -1.3316 | Lower | -1.0440 | Lower |
| MI (SEI) | 28.9508 | **38.4259 ✅** | 10.0433 | -9.4752 | Lower | +18.9075 | Higher | +28.3826 | Higher |
| Cyclomatic (sum) | 15 | 15 | 15 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 17 | **10 ✅** | 17 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 2161.3733 | 2217.4595 | **2146.9269 ✅** | -56.0862 | Lower | +14.4463 | Higher | +70.5325 | Higher |
| Halstead Effort | 89813.8213 | **67066.8349 ✅** | 86341.2771 | +22746.9864 | Higher | +3472.5442 | Higher | -19274.4422 | Lower |
| SLOC | 78 | 76 | **72 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| PLOC | 61 | **54 ✅** | 61 | +7 | Higher | 0 | Same | -7 | Lower |
| LLOC | 38 | **26 ✅** | 38 | +12 | Higher | 0 | Same | -12 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 0 | -7 | Lower | +6 | Higher | +13 | Higher |
| Blank Lines | 11 | 12 | 11 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02279

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.9759 ✅** | 49.9251 | 53.7153 | +5.0508 | Higher | +1.2606 | Higher | -3.7902 | Lower |
| MI (SEI) | **25.6026 ✅** | 23.6799 | 11.7369 | +1.9227 | Higher | +13.8657 | Higher | +11.9430 | Higher |
| Cyclomatic (sum) | **15 ✅** | 22 | 16 | -7 | Lower | -1 | Lower | +6 | Higher |
| Cognitive (sum) | **14 ✅** | 34 | 19 | -20 | Lower | -5 | Lower | +15 | Higher |
| Halstead Volume | 2859.6989 | **2544.0179 ✅** | 2893.0216 | +315.6811 | Higher | -33.3226 | Lower | -349.0037 | Lower |
| Halstead Effort | 103649.4965 | **96725.6803 ✅** | 118488.1014 | +6923.8162 | Higher | -14838.6049 | Lower | -21762.4211 | Lower |
| SLOC | **81 ✅** | 104 | 86 | -23 | Lower | -5 | Lower | +18 | Higher |
| PLOC | **63 ✅** | 89 | 74 | -26 | Lower | -11 | Lower | +15 | Higher |
| LLOC | 42 | **30 ✅** | 54 | +12 | Higher | -12 | Lower | -24 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 1 | -6 | Lower | +5 | Higher | +11 | Higher |
| Blank Lines | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02297

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.9824 | 66.7900 | **78.4165 ✅** | +7.1923 | Higher | -4.4341 | Lower | -11.6265 | Lower |
| MI (SEI) | 60.1908 | 52.3898 | **69.4394 ✅** | +7.8009 | Higher | -9.2486 | Lower | -17.0495 | Lower |
| Cyclomatic (sum) | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 1073.2518 | 1421.2788 | **977.6705 ✅** | -348.0270 | Lower | +95.5813 | Higher | +443.6083 | Higher |
| Halstead Effort | **25992.8161 ✅** | 42484.7124 | 26885.9378 | -16491.8962 | Lower | -893.1217 | Lower | +15598.7745 | Higher |
| SLOC | 39 | 54 | **31 ✅** | -15 | Lower | +8 | Higher | +23 | Higher |
| PLOC | 26 | 36 | **21 ✅** | -10 | Lower | +5 | Higher | +15 | Higher |
| LLOC | 19 | 23 | **16 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |
| Comments (CLOC) | 6 | **10 ✅** | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 8 | 11 | **6 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02305

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.7423 | 67.8730 | **72.7997 ✅** | +2.8692 | Higher | -2.0574 | Lower | -4.9267 | Lower |
| MI (SEI) | 52.5789 | **59.1368 ✅** | 51.4286 | -6.5579 | Lower | +1.1503 | Higher | +7.7082 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 8 | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 1236.9954 | 1241.8920 | **1224.0067 ✅** | -4.8966 | Lower | +12.9887 | Higher | +17.8852 | Higher |
| Halstead Effort | 42185.8093 | **32254.6942 ✅** | 41888.2309 | +9931.1150 | Higher | +297.5784 | Higher | -9633.5366 | Lower |
| SLOC | 43 | 52 | **38 ✅** | -9 | Lower | +5 | Higher | +14 | Higher |
| PLOC | 34 | 32 | **31 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| LLOC | 18 | **15 ✅** | 22 | +3 | Higher | -4 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 3 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 5 | 6 | **4 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.7893 | **79.6785 ✅** | 66.7843 | -12.8893 | Lower | +0.0049 | Higher | +12.8942 | Higher |
| MI (SEI) | 39.8527 | **78.8544 ✅** | 32.4284 | -39.0017 | Lower | +7.4243 | Higher | +46.4260 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 10 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 11 | **4 ✅** | 11 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 1463.4857 | **603.9396 ✅** | 1556.2200 | +859.5462 | Higher | -92.7343 | Lower | -952.2805 | Lower |
| Halstead Effort | 76558.5977 | **12078.7910 ✅** | 82355.1647 | +64479.8067 | Higher | -5796.5670 | Lower | -70276.3737 | Lower |
| SLOC | 52 | **33 ✅** | 51 | +19 | Higher | +1 | Higher | -18 | Lower |
| PLOC | 42 | **18 ✅** | 43 | +24 | Higher | -1 | Lower | -25 | Lower |
| LLOC | 34 | **11 ✅** | 36 | +23 | Higher | -2 | Lower | -25 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 1 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p02328

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.9319 | **67.1749 ✅** | 62.9841 | -1.2430 | Lower | +2.9478 | Higher | +4.1908 | Higher |
| MI (SEI) | 40.1817 | **55.3356 ✅** | 16.2860 | -15.1538 | Lower | +23.8958 | Higher | +39.0496 | Higher |
| Cyclomatic (sum) | **8 ✅** | 10 | 11 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **10 ✅** | 14 | 14 | -4 | Lower | -4 | Lower | 0 | Same |
| Halstead Volume | 1416.4331 | **1141.0212 ✅** | 1863.6634 | +275.4119 | Higher | -447.2303 | Lower | -722.6422 | Lower |
| Halstead Effort | **39195.3605 ✅** | 46553.6653 | 88439.3017 | -7358.3048 | Lower | -49243.9412 | Lower | -41885.6364 | Lower |
| SLOC | 57 | **55 ✅** | 60 | +2 | Higher | -3 | Lower | -5 | Lower |
| PLOC | 43 | **37 ✅** | 52 | +6 | Higher | -9 | Lower | -15 | Lower |
| LLOC | 29 | **22 ✅** | 38 | +7 | Higher | -9 | Lower | -16 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 0 | -8 | Lower | +4 | Higher | +12 | Higher |
| Blank Lines | 11 | **6 ✅** | 8 | +5 | Higher | +3 | Higher | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02333

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.9726 | 54.8899 | **60.3219 ✅** | +1.0828 | Higher | -4.3493 | Lower | -5.4321 | Lower |
| MI (SEI) | 6.9851 | **34.4214 ✅** | 12.8526 | -27.4363 | Lower | -5.8674 | Lower | +21.5689 | Higher |
| Cyclomatic (sum) | 19 | 15 | 15 | +4 | Higher | +4 | Higher | 0 | Same |
| Cognitive (sum) | 13 | **10 ✅** | 11 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | 2514 | 2502 | **2474.5939 ✅** | +12 | Higher | +39.4061 | Higher | +27.4061 | Higher |
| Halstead Effort | 178567.9412 | **155481.4286 ✅** | 164633.2757 | +23086.5126 | Higher | +13934.6654 | Higher | -9151.8472 | Lower |
| SLOC | 75 | 85 | **61 ✅** | -10 | Lower | +14 | Higher | +24 | Higher |
| PLOC | 61 | 56 | **52 ✅** | +5 | Higher | +9 | Higher | +4 | Higher |
| LLOC | 39 | 39 | **38 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 0 | **14 ✅** | 0 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 14 | 16 | **9 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |

## p02340

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.3543 | 67.3699 | **69.1262 ✅** | -6.0156 | Lower | -7.7719 | Lower | -1.7564 | Lower |
| MI (SEI) | 28.3881 | **56.0189 ✅** | 36.1635 | -27.6307 | Lower | -7.7754 | Lower | +19.8554 | Higher |
| Cyclomatic (sum) | 17 | 9 | **8 ✅** | +8 | Higher | +9 | Higher | +1 | Higher |
| Cognitive (sum) | 11 | 6 | 6 | +5 | Higher | +5 | Higher | 0 | Same |
| Halstead Volume | 1857.2236 | **1289.2432 ✅** | 1494.5458 | +567.9804 | Higher | +362.6778 | Higher | -205.3026 | Lower |
| Halstead Effort | 74741.9266 | **36098.8101 ✅** | 47742.4356 | +38643.1165 | Higher | +26999.4910 | Higher | -11643.6255 | Lower |
| SLOC | 61 | 53 | **46 ✅** | +8 | Higher | +15 | Higher | +7 | Higher |
| PLOC | 50 | **31 ✅** | 37 | +19 | Higher | +13 | Higher | -6 | Lower |
| LLOC | 28 | **18 ✅** | 21 | +10 | Higher | +7 | Higher | -3 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 1 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | 10 | 12 | **8 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 6 | 2 | 2 | +4 | Higher | +4 | Higher | 0 | Same |

## p02347

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.3081 | **54.4795 ✅** | 40.7270 | -14.1714 | Lower | -0.4189 | Lower | +13.7525 | Higher |
| MI (SEI) | -8.0446 | **32.9763 ✅** | -14.3988 | -41.0210 | Lower | +6.3541 | Higher | +47.3751 | Higher |
| Cyclomatic (sum) | 29 | **20 ✅** | 25 | +9 | Higher | +4 | Higher | -5 | Lower |
| Cognitive (sum) | 33 | **12 ✅** | 33 | +21 | Higher | 0 | Same | -21 | Lower |
| Halstead Volume | 4805.8171 | **1947.9881 ✅** | 4951.4229 | +2857.8290 | Higher | -145.6057 | Lower | -3003.4347 | Lower |
| Halstead Effort | 331938.6319 | **65118.4597 ✅** | 365881.5829 | +266820.1722 | Higher | -33942.9510 | Lower | -300763.1232 | Lower |
| SLOC | 139 | **88 ✅** | 142 | +51 | Higher | -3 | Lower | -54 | Lower |
| PLOC | 117 | **68 ✅** | 125 | +49 | Higher | -8 | Lower | -57 | Lower |
| LLOC | 55 | **25 ✅** | 80 | +30 | Higher | -25 | Lower | -55 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 0 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 22 | **13 ✅** | 17 | +9 | Higher | +5 | Higher | -4 | Lower |
| Functions | **4 ✅** | 7 | 5 | -3 | Lower | -1 | Lower | +2 | Higher |
| Closures | 4 | 1 | **0 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Exits | **3 ✅** | 6 | 4 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.3756 | **45.0138 ✅** | 40.1562 | -1.6381 | Lower | +3.2194 | Higher | +4.8576 | Higher |
| MI (SEI) | 2.8455 | **12.7675 ✅** | -8.7851 | -9.9219 | Lower | +11.6307 | Higher | +21.5526 | Higher |
| Cyclomatic (sum) | **23 ✅** | 24 | 25 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **13 ✅** | 20 | 14 | -7 | Lower | -1 | Lower | +6 | Higher |
| Halstead Volume | 4717.1563 | **4498.5078 ✅** | 5290.2266 | +218.6485 | Higher | -573.0703 | Lower | -791.7188 | Lower |
| Halstead Effort | **287521.9094 ✅** | 398217.9080 | 360051.2421 | -110695.9986 | Lower | -72529.3326 | Lower | +38166.6659 | Higher |
| SLOC | 126 | **114 ✅** | 144 | +12 | Higher | -18 | Lower | -30 | Lower |
| PLOC | 109 | **88 ✅** | 129 | +21 | Higher | -20 | Lower | -41 | Lower |
| LLOC | **64 ✅** | 65 | 71 | -1 | Lower | -7 | Lower | -6 | Lower |
| Comments (CLOC) | 4 | **9 ✅** | 1 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 17 | 17 | **15 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **7 ✅** | 11 | +2 | Higher | -2 | Lower | -4 | Lower |

## p02355

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.3016 | **66.4160 ✅** | 53.9650 | -16.1143 | Lower | -3.6633 | Lower | +12.4510 | Higher |
| MI (SEI) | 18.7940 | **52.0177 ✅** | 18.6605 | -33.2236 | Lower | +0.1336 | Higher | +33.3572 | Higher |
| Cyclomatic (sum) | 28 | **12 ✅** | 22 | +16 | Higher | +6 | Higher | -10 | Lower |
| Cognitive (sum) | 31 | **19 ✅** | 28 | +12 | Higher | +3 | Higher | -9 | Lower |
| Halstead Volume | 2486.5484 | **1208.5264 ✅** | 2114.6668 | +1278.0221 | Higher | +371.8816 | Higher | -906.1405 | Lower |
| Halstead Effort | 178579.3886 | **58879.4050 ✅** | 150141.3452 | +119699.9835 | Higher | +28438.0434 | Higher | -91261.9401 | Lower |
| SLOC | 94 | **55 ✅** | 86 | +39 | Higher | +8 | Higher | -31 | Lower |
| PLOC | 78 | **41 ✅** | 68 | +37 | Higher | +10 | Higher | -27 | Lower |
| LLOC | 61 | **31 ✅** | 61 | +30 | Higher | 0 | Same | -30 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 3 | -4 | Lower | +3 | Higher | +7 | Higher |
| Blank Lines | 12 | **7 ✅** | 16 | +5 | Higher | -4 | Lower | -9 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 5 | **2 ✅** | 3 | +3 | Higher | +2 | Higher | -1 | Lower |

## p02369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.9539 | **60.9342 ✅** | 42.5542 | -13.9804 | Lower | +4.3997 | Higher | +18.3801 | Higher |
| MI (SEI) | 10.2513 | **48.2051 ✅** | 0.2484 | -37.9537 | Lower | +10.0029 | Higher | +47.9566 | Higher |
| Cyclomatic (sum) | 16 | **14 ✅** | 18 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 17 | **13 ✅** | 22 | +4 | Higher | -5 | Lower | -9 | Lower |
| Halstead Volume | 3230.6967 | **1497.5203 ✅** | 4449.4184 | +1733.1765 | Higher | -1218.7217 | Lower | -2951.8982 | Lower |
| Halstead Effort | 130035.5438 | **44442.5376 ✅** | 244082.3832 | +85593.0062 | Higher | -114046.8394 | Lower | -199639.8456 | Lower |
| SLOC | 126 | **70 ✅** | 145 | +56 | Higher | -19 | Lower | -75 | Lower |
| PLOC | 103 | **51 ✅** | 126 | +52 | Higher | -23 | Lower | -75 | Lower |
| LLOC | 57 | **27 ✅** | 74 | +30 | Higher | -17 | Lower | -47 | Lower |
| Comments (CLOC) | 6 | **17 ✅** | 4 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | 21 | **11 ✅** | 18 | +10 | Higher | +3 | Higher | -7 | Lower |
| Functions | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 6 | 3 | -4 | Lower | -1 | Lower | +3 | Higher |

## p02370

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.9338 | **62.9375 ✅** | 56.5301 | -6.0037 | Lower | +0.4037 | Higher | +6.4074 | Higher |
| MI (SEI) | 28.4876 | **47.4056 ✅** | 24.2564 | -18.9181 | Lower | +4.2311 | Higher | +23.1492 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 14 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | **14 ✅** | 15 | 18 | -1 | Lower | -4 | Lower | -3 | Lower |
| Halstead Volume | 2520.5123 | **1688.3186 ✅** | 2304.2740 | +832.1938 | Higher | +216.2384 | Higher | -615.9554 | Lower |
| Halstead Effort | 100100.3475 | **44671.9964 ✅** | 113656.7567 | +55428.3512 | Higher | -13556.4092 | Lower | -68984.7604 | Lower |
| SLOC | 78 | **63 ✅** | 80 | +15 | Higher | -2 | Lower | -17 | Lower |
| PLOC | 60 | **42 ✅** | 65 | +18 | Higher | -5 | Lower | -23 | Lower |
| LLOC | 51 | **32 ✅** | 56 | +19 | Higher | -5 | Lower | -24 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 4 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 12 | **11 ✅** | 13 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02371

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.4703 | **63.4930 ✅** | 42.6098 | -21.0228 | Lower | -0.1396 | Lower | +20.8832 | Higher |
| MI (SEI) | -2.7784 | **55.4968 ✅** | -2.7145 | -58.2751 | Lower | -0.0639 | Lower | +58.2113 | Higher |
| Cyclomatic (sum) | 21 | **8 ✅** | 20 | +13 | Higher | +1 | Higher | -12 | Lower |
| Cognitive (sum) | 24 | **10 ✅** | 22 | +14 | Higher | +2 | Higher | -12 | Lower |
| Halstead Volume | 5433.2717 | **1368.3158 ✅** | 5399.1605 | +4064.9559 | Higher | +34.1112 | Higher | -4030.8447 | Lower |
| Halstead Effort | 546224.9144 | **40973.4568 ✅** | 528842.2622 | +505251.4576 | Higher | +17382.6522 | Higher | -487868.8054 | Lower |
| SLOC | 131 | **67 ✅** | 132 | +64 | Higher | -1 | Lower | -65 | Lower |
| PLOC | 117 | **35 ✅** | 112 | +82 | Higher | +5 | Higher | -77 | Lower |
| LLOC | 103 | **18 ✅** | 113 | +85 | Higher | -10 | Lower | -95 | Lower |
| Comments (CLOC) | 2 | **22 ✅** | 2 | -20 | Lower | 0 | Same | +20 | Higher |
| Blank Lines | 14 | **10 ✅** | 20 | +4 | Higher | -6 | Lower | -10 | Lower |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |

## p02393

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **86.9728 ✅** | 86.6621 | 75.3405 | +0.3107 | Higher | +11.6323 | Higher | +11.3217 | Higher |
| MI (SEI) | 82.6635 | **89.3208 ✅** | 60.7264 | -6.6573 | Lower | +21.9371 | Higher | +28.5944 | Higher |
| Cyclomatic (sum) | 4 | 4 | 13 | 0 | Same | -9 | Lower | -9 | Lower |
| Cognitive (sum) | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |
| Halstead Volume | **437.5941 ✅** | 530.3976 | 1240.0092 | -92.8036 | Lower | -802.4152 | Lower | -709.6116 | Lower |
| Halstead Effort | **5803.8794 ✅** | 6641.5010 | 56656.6129 | -837.6216 | Lower | -50852.7335 | Lower | -50015.1119 | Lower |
| SLOC | 24 | **23 ✅** | 31 | +1 | Higher | -7 | Lower | -8 | Lower |
| PLOC | 14 | 14 | 23 | 0 | Same | -9 | Lower | -9 | Lower |
| LLOC | 7 | 7 | **5 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 5 | **8 ✅** | 4 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02396

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.1626 ✅** | 86.9994 | 83.6279 | +2.1632 | Higher | +5.5347 | Higher | +3.3715 | Higher |
| MI (SEI) | 81.1769 | **87.5441 ✅** | 76.0038 | -6.3672 | Lower | +5.1731 | Higher | +11.5402 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 10 | 10 | **2 ✅** | 0 | Same | +8 | Higher | +8 | Higher |
| Halstead Volume | 398.5071 | 398.5071 | 471.4646 | 0 | Same | -72.9574 | Lower | -72.9574 | Lower |
| Halstead Effort | 5927.7936 | 5927.7936 | 7425.5671 | 0 | Same | -1497.7734 | Lower | -1497.7734 | Lower |
| SLOC | **21 ✅** | 24 | 28 | -3 | Lower | -7 | Lower | -4 | Lower |
| PLOC | 17 | 17 | 18 | 0 | Same | -1 | Lower | -1 | Lower |
| LLOC | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 3 | **7 ✅** | 5 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | 2 | 2 | 6 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02412

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.8991 | 70.4323 | **75.5319 ✅** | -0.5331 | Lower | -5.6327 | Lower | -5.0996 | Lower |
| MI (SEI) | 49.1548 | **53.4174 ✅** | 47.5362 | -4.2626 | Lower | +1.6186 | Higher | +5.8812 | Higher |
| Cyclomatic (sum) | 17 | **12 ✅** | 13 | +5 | Higher | +4 | Higher | -1 | Lower |
| Cognitive (sum) | 29 | **16 ✅** | 25 | +13 | Higher | +4 | Higher | -9 | Lower |
| Halstead Volume | 926.4271 | 910.9526 | **818.9147 ✅** | +15.4746 | Higher | +107.5124 | Higher | +92.0378 | Higher |
| Halstead Effort | 29433.3618 | 31055.2013 | **22520.1554 ✅** | -1621.8395 | Lower | +6913.2063 | Higher | +8535.0459 | Higher |
| SLOC | 45 | 47 | **35 ✅** | -2 | Lower | +10 | Higher | +12 | Higher |
| PLOC | 34 | 34 | **28 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| LLOC | **18 ✅** | 20 | 19 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 4 | **6 ✅** | 1 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | 7 | 9 | **6 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02418

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.5811 | **89.9215 ✅** | 80.2024 | -9.3404 | Lower | +0.3787 | Higher | +9.7191 | Higher |
| MI (SEI) | 74.8382 | **88.0965 ✅** | 79.3048 | -13.2582 | Lower | -4.4665 | Lower | +8.7917 | Higher |
| Cyclomatic (sum) | 7 | 4 | **3 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Cognitive (sum) | 4 | 2 | 2 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 590.1941 | **325.4847 ✅** | 475.4888 | +264.7094 | Higher | +114.7054 | Higher | -150.0040 | Lower |
| Halstead Effort | 9836.5685 | **3173.4761 ✅** | 6847.0380 | +6663.0924 | Higher | +2989.5305 | Higher | -3673.5619 | Lower |
| SLOC | 31 | **22 ✅** | 36 | +9 | Higher | -5 | Lower | -14 | Lower |
| PLOC | 18 | **14 ✅** | 17 | +4 | Higher | +1 | Higher | -3 | Lower |
| LLOC | 12 | **6 ✅** | 12 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 7 | 5 | **12 ✅** | +2 | Higher | -5 | Lower | -7 | Lower |
| Blank Lines | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02430

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.4617 ✅** | 63.9053 | 65.1337 | +8.5564 | Higher | +7.3280 | Higher | -1.2284 | Lower |
| MI (SEI) | **51.5109 ✅** | 48.5038 | 43.9712 | +3.0071 | Higher | +7.5397 | Higher | +4.5326 | Higher |
| Cyclomatic (sum) | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | **6 ✅** | 10 | 9 | -4 | Lower | -3 | Lower | +1 | Higher |
| Halstead Volume | **1242.9923 ✅** | 1493.5472 | 1546.5070 | -250.5549 | Lower | -303.5146 | Lower | -52.9598 | Lower |
| Halstead Effort | **35124.5577 ✅** | 58888.4332 | 51518.0139 | -23763.8755 | Lower | -16393.4562 | Lower | +7370.4193 | Higher |
| SLOC | **37 ✅** | 60 | 55 | -23 | Lower | -18 | Lower | +5 | Higher |
| PLOC | **32 ✅** | 41 | 46 | -9 | Lower | -14 | Lower | -5 | Lower |
| LLOC | **18 ✅** | 34 | 25 | -16 | Lower | -7 | Lower | +9 | Higher |
| Comments (CLOC) | 3 | **11 ✅** | 6 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | **4 ✅** | 10 | 8 | -6 | Lower | -4 | Lower | +2 | Higher |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **1 ✅** | 5 | +6 | Higher | +2 | Higher | -4 | Lower |

## p02433

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.8187 | **44.0728 ✅** | 32.3738 | -9.2540 | Lower | +2.4449 | Higher | +11.6990 | Higher |
| MI (SEI) | -12.8321 | **6.0807 ✅** | -16.4309 | -18.9128 | Lower | +3.5989 | Higher | +22.5117 | Higher |
| Cyclomatic (sum) | 41 | **29 ✅** | 44 | +12 | Higher | -3 | Lower | -15 | Lower |
| Cognitive (sum) | 46 | **31 ✅** | 44 | +15 | Higher | +2 | Higher | -13 | Lower |
| Halstead Volume | 4672.4460 | **3243.2675 ✅** | 4916.0028 | +1429.1784 | Higher | -243.5569 | Lower | -1672.7353 | Lower |
| Halstead Effort | 254487.1878 | **142512.9915 ✅** | 276212.7024 | +111974.1962 | Higher | -21725.5146 | Lower | -133699.7109 | Lower |
| SLOC | 166 | **125 ✅** | 182 | +41 | Higher | -16 | Lower | -57 | Lower |
| PLOC | 140 | **110 ✅** | 153 | +30 | Higher | -13 | Lower | -43 | Lower |
| LLOC | 85 | **58 ✅** | 96 | +27 | Higher | -11 | Lower | -38 | Lower |
| Comments (CLOC) | 2 | **5 ✅** | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 24 | **11 ✅** | 27 | +13 | Higher | -3 | Lower | -16 | Lower |
| Functions | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 2 | 4 | **0 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Exits | 12 | **2 ✅** | 13 | +10 | Higher | -1 | Lower | -11 | Lower |

## p02444

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.6852 | **67.8715 ✅** | 67.7470 | -1.1863 | Lower | -1.0619 | Lower | +0.1244 | Higher |
| MI (SEI) | 54.6746 | 55.8207 | **57.3617 ✅** | -1.1461 | Lower | -2.6871 | Lower | -1.5410 | Lower |
| Cyclomatic (sum) | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 7 | **5 ✅** | 9 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | 1450.2317 | **1242.2611 ✅** | 1437.7080 | +207.9705 | Higher | +12.5236 | Higher | -195.4469 | Lower |
| Halstead Effort | 39273.2087 | **37626.1791 ✅** | 39626.8277 | +1647.0297 | Higher | -353.6190 | Lower | -2000.6486 | Lower |
| SLOC | 54 | 52 | **50 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| PLOC | 36 | **31 ✅** | 34 | +5 | Higher | +2 | Higher | -3 | Lower |
| LLOC | 23 | **19 ✅** | 25 | +4 | Higher | -2 | Lower | -6 | Lower |
| Comments (CLOC) | 12 | 11 | 12 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02449

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.8174 ✅** | 52.8685 | 50.1384 | +1.9489 | Higher | +4.6790 | Higher | +2.7301 | Higher |
| MI (SEI) | 24.1011 | **33.9617 ✅** | 21.4904 | -9.8606 | Lower | +2.6107 | Higher | +12.4713 | Higher |
| Cyclomatic (sum) | 18 | **17 ✅** | 23 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | 18 | **13 ✅** | 53 | +5 | Higher | -35 | Lower | -40 | Lower |
| Halstead Volume | 2683.7110 | **2238.7778 ✅** | 2723.6637 | +444.9332 | Higher | -39.9527 | Lower | -484.8859 | Lower |
| Halstead Effort | 163017.3094 | **117074.9103 ✅** | 212573.4428 | +45942.3991 | Higher | -49556.1334 | Lower | -95498.5325 | Lower |
| SLOC | **80 ✅** | 97 | 99 | -17 | Lower | -19 | Lower | -2 | Lower |
| PLOC | 59 | 59 | 86 | 0 | Same | -27 | Lower | -27 | Lower |
| LLOC | 46 | **44 ✅** | 77 | +2 | Higher | -31 | Lower | -33 | Lower |
| Comments (CLOC) | 5 | **19 ✅** | 9 | -14 | Lower | -4 | Lower | +10 | Higher |
| Blank Lines | 18 | 20 | **8 ✅** | -2 | Lower | +10 | Higher | +12 | Higher |
| Functions | 3 | 3 | **1 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | 5 | **0 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |

## p02450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.5835 | 71.5822 | **74.0074 ✅** | +0.0013 | Higher | -2.4239 | Lower | -2.4252 | Lower |
| MI (SEI) | 44.9698 | **65.2166 ✅** | 49.7382 | -20.2469 | Lower | -4.7684 | Lower | +15.4785 | Higher |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | **1236.9954 ✅** | 1237.3008 | 1254.5655 | -0.3054 | Lower | -17.5701 | Lower | -17.2647 | Lower |
| Halstead Effort | 40087.8145 | **35139.3431 ✅** | 38934.7906 | +4948.4714 | Higher | +1153.0239 | Higher | -3795.4475 | Lower |
| SLOC | 42 | 42 | **36 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| PLOC | 35 | **29 ✅** | 30 | +6 | Higher | +5 | Higher | -1 | Lower |
| LLOC | **18 ✅** | 19 | 19 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02457

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.0342 | **63.0359 ✅** | 42.1559 | -21.0017 | Lower | -0.1217 | Lower | +20.8800 | Higher |
| MI (SEI) | -11.9019 | **46.2225 ✅** | -1.4463 | -58.1244 | Lower | -10.4556 | Lower | +47.6688 | Higher |
| Cyclomatic (sum) | 31 | **14 ✅** | 34 | +17 | Higher | -3 | Lower | -20 | Lower |
| Cognitive (sum) | 32 | **8 ✅** | 37 | +24 | Higher | -5 | Lower | -29 | Lower |
| Halstead Volume | 4080.6781 | **1200.7669 ✅** | 4381.0962 | +2879.9112 | Higher | -300.4181 | Lower | -3180.3292 | Lower |
| Halstead Effort | 209863.4461 | **29590.3281 ✅** | 353750.2120 | +180273.1180 | Higher | -143886.7659 | Lower | -324159.8839 | Lower |
| SLOC | 128 | **66 ✅** | 119 | +62 | Higher | +9 | Higher | -53 | Lower |
| PLOC | 115 | **47 ✅** | 102 | +68 | Higher | +13 | Higher | -55 | Lower |
| LLOC | 58 | **17 ✅** | 59 | +41 | Higher | -1 | Lower | -42 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 2 | -11 | Lower | -2 | Lower | +9 | Higher |
| Blank Lines | 13 | **8 ✅** | 17 | +5 | Higher | -4 | Lower | -9 | Lower |
| Functions | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 9 | **4 ✅** | 14 | +5 | Higher | -5 | Lower | -10 | Lower |

## p02542

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 17.0876 | **57.2415 ✅** | 10.5614 | -40.1539 | Lower | +6.5262 | Higher | +46.6801 | Higher |
| MI (SEI) | -40.8277 | **45.1611 ✅** | -54.2530 | -85.9888 | Lower | +13.4253 | Higher | +99.4141 | Higher |
| Cyclomatic (sum) | 56 | **17 ✅** | 61 | +39 | Higher | -5 | Lower | -44 | Lower |
| Cognitive (sum) | 112 | **19 ✅** | 135 | +93 | Higher | -23 | Lower | -116 | Lower |
| Halstead Volume | 12403.0007 | **2151.8208 ✅** | 18203.7233 | +10251.1799 | Higher | -5800.7227 | Lower | -16051.9025 | Lower |
| Halstead Effort | 2085662.4807 | **121362.6916 ✅** | 4559813.3698 | +1964299.7892 | Higher | -2474150.8890 | Lower | -4438450.6782 | Lower |
| SLOC | 293 | **75 ✅** | 361 | +218 | Higher | -68 | Lower | -286 | Lower |
| PLOC | 265 | **46 ✅** | 337 | +219 | Higher | -72 | Lower | -291 | Lower |
| LLOC | 201 | **28 ✅** | 353 | +173 | Higher | -152 | Lower | -325 | Lower |
| Comments (CLOC) | 1 | **21 ✅** | 0 | -20 | Lower | +1 | Higher | +21 | Higher |
| Blank Lines | 28 | **13 ✅** | 24 | +15 | Higher | +4 | Higher | -11 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **3 ✅** | 5 | +3 | Higher | +1 | Higher | -2 | Lower |

## p02562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 13.1943 | **44.2784 ✅** | 10.8894 | -31.0841 | Lower | +2.3048 | Higher | +33.3889 | Higher |
| MI (SEI) | -38.0695 | **17.9602 ✅** | -53.6779 | -56.0296 | Lower | +15.6084 | Higher | +71.6380 | Higher |
| Cyclomatic (sum) | 63 | **26 ✅** | 62 | +37 | Higher | +1 | Higher | -36 | Lower |
| Cognitive (sum) | 113 | **24 ✅** | 119 | +89 | Higher | -6 | Lower | -95 | Lower |
| Halstead Volume | 15055.8350 | **3650.1184 ✅** | 16781.9864 | +11405.7166 | Higher | -1726.1514 | Lower | -13131.8681 | Lower |
| Halstead Effort | 2578523.2042 | **238635.0979 ✅** | 3681548.2744 | +2339888.1063 | Higher | -1103025.0702 | Lower | -3442913.1765 | Lower |
| SLOC | 317 | **124 ✅** | 358 | +193 | Higher | -41 | Lower | -234 | Lower |
| PLOC | 279 | **95 ✅** | 331 | +184 | Higher | -52 | Lower | -236 | Lower |
| LLOC | 281 | **53 ✅** | 339 | +228 | Higher | -58 | Lower | -286 | Lower |
| Comments (CLOC) | 8 | **17 ✅** | 0 | -9 | Lower | +8 | Higher | +17 | Higher |
| Blank Lines | 38 | **17 ✅** | 27 | +21 | Higher | +11 | Higher | -10 | Lower |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02584

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.5226 | **81.6455 ✅** | 78.6547 | -3.1229 | Lower | -0.1321 | Lower | +2.9909 | Higher |
| MI (SEI) | 64.1296 | **82.8040 ✅** | 66.5379 | -18.6744 | Lower | -2.4082 | Lower | +16.2662 | Higher |
| Cyclomatic (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 867.7128 | **432.4343 ✅** | 803.3811 | +435.2784 | Higher | +64.3317 | Higher | -370.9468 | Lower |
| Halstead Effort | 18395.5111 | **7135.1666 ✅** | 17935.4839 | +11260.3445 | Higher | +460.0272 | Higher | -10800.3173 | Lower |
| SLOC | **32 ✅** | 33 | 33 | -1 | Lower | -1 | Lower | 0 | Same |
| PLOC | 24 | **18 ✅** | 22 | +6 | Higher | +2 | Higher | -4 | Lower |
| LLOC | 21 | **12 ✅** | 20 | +9 | Higher | +1 | Higher | -8 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 5 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02585

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.3285 | **72.9409 ✅** | 56.0127 | -16.6124 | Lower | +0.3157 | Higher | +16.9282 | Higher |
| MI (SEI) | 28.1747 | **66.6423 ✅** | 15.4743 | -38.4675 | Lower | +12.7005 | Higher | +51.1680 | Higher |
| Cyclomatic (sum) | 15 | **8 ✅** | 15 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 24 | **8 ✅** | 24 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 2688.8133 | **1109.2048 ✅** | 2857.1398 | +1579.6085 | Higher | -168.3265 | Lower | -1747.9350 | Lower |
| Halstead Effort | 222095.9767 | **33544.4997 ✅** | 225419.9271 | +188551.4770 | Higher | -3323.9505 | Lower | -191875.4274 | Lower |
| SLOC | 76 | **40 ✅** | 76 | +36 | Higher | 0 | Same | -36 | Lower |
| PLOC | 60 | **28 ✅** | 62 | +32 | Higher | -2 | Lower | -34 | Lower |
| LLOC | 62 | **21 ✅** | 64 | +41 | Higher | -2 | Lower | -43 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 1 | -5 | Lower | +5 | Higher | +10 | Higher |
| Blank Lines | 14 | **6 ✅** | 14 | +8 | Higher | 0 | Same | -8 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02606

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.8519 | 84.1118 | **90.5053 ✅** | -1.2599 | Lower | -7.6534 | Lower | -6.3935 | Lower |
| MI (SEI) | 75.2567 | **89.9942 ✅** | 79.7618 | -14.7375 | Lower | -4.5051 | Lower | +10.2324 | Higher |
| Cyclomatic (sum) | 5 | 3 | **2 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 640.7332 | **357.2334 ✅** | 593.8777 | +283.4998 | Higher | +46.8555 | Higher | -236.6443 | Lower |
| Halstead Effort | 12334.1138 | **4593.0011 ✅** | 6326.0884 | +7741.1128 | Higher | +6008.0255 | Higher | -1733.0873 | Lower |
| SLOC | 27 | 31 | **18 ✅** | -4 | Lower | +9 | Higher | +13 | Higher |
| PLOC | 16 | 13 | **12 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| LLOC | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 2 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 6 | 7 | **4 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02617

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.1032 | **79.7261 ✅** | 74.5482 | -3.6230 | Lower | +1.5549 | Higher | +5.1779 | Higher |
| MI (SEI) | 60.0394 | **82.8673 ✅** | 56.5468 | -22.8279 | Lower | +3.4926 | Higher | +26.3205 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 1094.4844 | **456.3408 ✅** | 1043.7263 | +638.1436 | Higher | +50.7582 | Higher | -587.3854 | Lower |
| Halstead Effort | 32429.1682 | **8951.3008 ✅** | 32564.2594 | +23477.8674 | Higher | -135.0912 | Lower | -23612.9586 | Lower |
| SLOC | **34 ✅** | 36 | 38 | -2 | Lower | -4 | Lower | -2 | Lower |
| PLOC | 22 | **16 ✅** | 26 | +6 | Higher | -4 | Lower | -10 | Lower |
| LLOC | 18 | **8 ✅** | 18 | +10 | Higher | 0 | Same | -10 | Lower |
| Comments (CLOC) | 4 | **16 ✅** | 4 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02618

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.2001 | **62.7125 ✅** | 50.6420 | -12.5124 | Lower | -0.4419 | Lower | +12.0706 | Higher |
| MI (SEI) | 26.0642 | **56.6930 ✅** | 10.5564 | -30.6288 | Lower | +15.5077 | Higher | +46.1365 | Higher |
| Cyclomatic (sum) | 18 | **8 ✅** | 19 | +10 | Higher | -1 | Lower | -11 | Lower |
| Cognitive (sum) | 15 | **6 ✅** | 16 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 3357.7619 | **1589.9051 ✅** | 3585.3026 | +1767.8568 | Higher | -227.5406 | Lower | -1995.3975 | Lower |
| Halstead Effort | 188183.9018 | **33564.6633 ✅** | 221651.3723 | +154619.2385 | Higher | -33467.4705 | Lower | -188086.7090 | Lower |
| SLOC | 99 | **67 ✅** | 93 | +32 | Higher | +6 | Higher | -26 | Lower |
| PLOC | 72 | **39 ✅** | 74 | +33 | Higher | -2 | Lower | -35 | Lower |
| LLOC | 53 | **20 ✅** | 58 | +33 | Higher | -5 | Lower | -38 | Lower |
| Comments (CLOC) | 14 | **26 ✅** | 2 | -12 | Lower | +12 | Higher | +24 | Higher |
| Blank Lines | 18 | **13 ✅** | 19 | +5 | Higher | -1 | Lower | -6 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p02639

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.0915 ✅** | 83.4979 | 77.2349 | +5.5936 | Higher | +11.8566 | Higher | +6.2630 | Higher |
| MI (SEI) | **90.1718 ✅** | 78.8284 | 64.8968 | +11.3434 | Higher | +25.2750 | Higher | +13.9316 | Higher |
| Cyclomatic (sum) | **5 ✅** | 7 | 8 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **3 ✅** | 9 | 5 | -6 | Lower | -2 | Lower | +4 | Higher |
| Halstead Volume | **422.2594 ✅** | 517.9694 | 884.4279 | -95.7100 | Lower | -462.1685 | Lower | -366.4585 | Lower |
| Halstead Effort | **6150.9122 ✅** | 8119.1706 | 15313.7057 | -1968.2585 | Lower | -9162.7935 | Lower | -7194.5351 | Lower |
| SLOC | **21 ✅** | 27 | 33 | -6 | Lower | -12 | Lower | -6 | Lower |
| PLOC | **15 ✅** | 19 | 24 | -4 | Lower | -9 | Lower | -5 | Lower |
| LLOC | **7 ✅** | 11 | 15 | -4 | Lower | -8 | Lower | -4 | Lower |
| Comments (CLOC) | 6 | 6 | 5 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02644

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.9780 ✅** | 53.6480 | 55.9548 | +3.3299 | Higher | +1.0231 | Higher | -2.3068 | Lower |
| MI (SEI) | 23.8923 | **37.6554 ✅** | 6.7558 | -13.7632 | Lower | +17.1364 | Higher | +30.8996 | Higher |
| Cyclomatic (sum) | 16 | 16 | 17 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 28 | 28 | **21 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Halstead Volume | 2933.4294 | **2728.1326 ✅** | 3416.7832 | +205.2968 | Higher | -483.3538 | Lower | -688.6505 | Lower |
| Halstead Effort | **147080.7845 ✅** | 171549.2867 | 215063.2035 | -24468.5022 | Lower | -67982.4189 | Lower | -43513.9168 | Lower |
| SLOC | 70 | 88 | 70 | -18 | Lower | 0 | Same | +18 | Higher |
| PLOC | 59 | 62 | **57 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| LLOC | 43 | **31 ✅** | 53 | +12 | Higher | -10 | Lower | -22 | Lower |
| Comments (CLOC) | 3 | **21 ✅** | 0 | -18 | Lower | +3 | Higher | +21 | Higher |
| Blank Lines | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p02647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.4759 | **82.3918 ✅** | 66.0967 | -18.9159 | Lower | -2.6208 | Lower | +16.2951 | Higher |
| MI (SEI) | **52.8634 ✅** | 43.8781 | 20.7766 | +8.9852 | Higher | +32.0868 | Higher | +23.1015 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 11 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 18 | **11 ✅** | 20 | +7 | Higher | -2 | Lower | -9 | Lower |
| Halstead Volume | 1695.4645 | **814.3479 ✅** | 1599.6239 | +881.1165 | Higher | +95.8405 | Higher | -785.2760 | Lower |
| Halstead Effort | 75769.7218 | **23654.8677 ✅** | 66136.1763 | +52114.8541 | Higher | +9633.5456 | Higher | -42481.3086 | Lower |
| SLOC | 60 | **25 ✅** | 52 | +35 | Higher | +8 | Higher | -27 | Lower |
| PLOC | 48 | **20 ✅** | 47 | +28 | Higher | +1 | Higher | -27 | Lower |
| LLOC | 42 | **15 ✅** | 43 | +27 | Higher | -1 | Lower | -28 | Lower |
| Comments (CLOC) | **16 ✅** | 0 | 0 | +16 | Higher | +16 | Higher | 0 | Same |
| Blank Lines | 9 | 5 | 5 | +4 | Higher | +4 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.4331 | **72.4185 ✅** | 42.0192 | -22.9854 | Lower | +7.4139 | Higher | +30.3993 | Higher |
| MI (SEI) | 5.8538 | **67.3170 ✅** | -1.6760 | -61.4632 | Lower | +7.5298 | Higher | +68.9931 | Higher |
| Cyclomatic (sum) | 22 | **9 ✅** | 22 | +13 | Higher | 0 | Same | -13 | Lower |
| Cognitive (sum) | 19 | **10 ✅** | 22 | +9 | Higher | -3 | Lower | -12 | Lower |
| Halstead Volume | 3961.4877 | **936.6593 ✅** | 4409.9279 | +3024.8284 | Higher | -448.4402 | Lower | -3473.2685 | Lower |
| Halstead Effort | 366956.3801 | **43086.3299 ✅** | 272064.0992 | +323870.0503 | Higher | +94892.2809 | Higher | -228977.7693 | Lower |
| SLOC | 93 | **43 ✅** | 142 | +50 | Higher | -49 | Lower | -99 | Lower |
| PLOC | 77 | **26 ✅** | 115 | +51 | Higher | -38 | Lower | -89 | Lower |
| LLOC | 59 | **18 ✅** | 54 | +41 | Higher | +5 | Higher | -36 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 3 | -12 | Lower | -2 | Lower | +10 | Higher |
| Blank Lines | 15 | **6 ✅** | 26 | +9 | Higher | -11 | Lower | -20 | Lower |
| Functions | 6 | **2 ✅** | 9 | +4 | Higher | -3 | Lower | -7 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 6 | **2 ✅** | 8 | +4 | Higher | -2 | Lower | -6 | Lower |

## p02678

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.2019 | **63.8943 ✅** | 60.8637 | -5.6924 | Lower | -2.6619 | Lower | +3.0306 | Higher |
| MI (SEI) | 28.7360 | **44.9145 ✅** | 22.3283 | -16.1785 | Lower | +6.4076 | Higher | +22.5862 | Higher |
| Cyclomatic (sum) | 11 | 10 | **8 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 2238.3869 | **1918.4656 ✅** | 2268.7174 | +319.9213 | Higher | -30.3304 | Lower | -350.2518 | Lower |
| Halstead Effort | 73085.9354 | **54551.1518 ✅** | 85602.5793 | +18534.7836 | Higher | -12516.6439 | Lower | -31051.4275 | Lower |
| SLOC | 76 | **57 ✅** | 67 | +19 | Higher | +9 | Higher | -10 | Lower |
| PLOC | 64 | **42 ✅** | 56 | +22 | Higher | +8 | Higher | -14 | Lower |
| LLOC | 37 | **31 ✅** | 46 | +6 | Higher | -9 | Lower | -15 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 1 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 12 | **8 ✅** | 11 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 4 | **1 ✅** | 2 | +3 | Higher | +2 | Higher | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |

## p02703

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.2595 | **45.5052 ✅** | 36.1655 | -1.2457 | Lower | +8.0939 | Higher | +9.3396 | Higher |
| MI (SEI) | -9.6079 | **15.5913 ✅** | -20.8777 | -25.1991 | Lower | +11.2698 | Higher | +36.4690 | Higher |
| Cyclomatic (sum) | **22 ✅** | 29 | 26 | -7 | Lower | -4 | Lower | +3 | Higher |
| Cognitive (sum) | **21 ✅** | 24 | 26 | -3 | Lower | -5 | Lower | -2 | Lower |
| Halstead Volume | 5103 | **2870.1554 ✅** | 6871.8451 | +2232.8446 | Higher | -1768.8451 | Lower | -4001.6897 | Lower |
| Halstead Effort | 188347.0909 | **121609.5454 ✅** | 317129.0435 | +66737.5455 | Higher | -128781.9526 | Lower | -195519.4980 | Lower |
| SLOC | **118 ✅** | 119 | 167 | -1 | Lower | -49 | Lower | -48 | Lower |
| PLOC | 99 | **98 ✅** | 146 | +1 | Higher | -47 | Lower | -48 | Lower |
| LLOC | 68 | **31 ✅** | 125 | +37 | Higher | -57 | Lower | -94 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 0 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 19 | **18 ✅** | 21 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 6 | 4 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02727

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.6746 | **54.7298 ✅** | 52.8312 | -1.0552 | Lower | +0.8434 | Higher | +1.8986 | Higher |
| MI (SEI) | 28.6725 | **36.3135 ✅** | 17.3747 | -7.6410 | Lower | +11.2978 | Higher | +18.9388 | Higher |
| Cyclomatic (sum) | 28 | 26 | **23 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | **16 ✅** | 20 | 29 | -4 | Lower | -13 | Lower | -9 | Lower |
| Halstead Volume | **2520.5123 ✅** | 2548.3960 | 2810.3807 | -27.8837 | Lower | -289.8683 | Lower | -261.9846 | Lower |
| Halstead Effort | 108647.3480 | **102204.0930 ✅** | 151214.0927 | +6443.2550 | Higher | -42566.7446 | Lower | -49009.9996 | Lower |
| SLOC | 76 | **73 ✅** | 83 | +3 | Higher | -7 | Lower | -10 | Lower |
| PLOC | 57 | **49 ✅** | 73 | +8 | Higher | -16 | Lower | -24 | Lower |
| LLOC | **32 ✅** | 36 | 52 | -4 | Lower | -20 | Lower | -16 | Lower |
| Comments (CLOC) | 8 | **13 ✅** | 3 | -5 | Lower | +5 | Higher | +10 | Higher |
| Blank Lines | 13 | 13 | **10 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 4 | 10 | **1 ✅** | -6 | Lower | +3 | Higher | +9 | Higher |
| Exits | 15 | 7 | **1 ✅** | +8 | Higher | +14 | Higher | +6 | Higher |

## p02728

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.5179 | **70.7189 ✅** | 39.0242 | -32.2010 | Lower | -0.5063 | Lower | +31.6947 | Higher |
| MI (SEI) | -17.4839 | **66.7270 ✅** | -16.6517 | -84.2109 | Lower | -0.8322 | Lower | +83.3786 | Higher |
| Cyclomatic (sum) | 26 | **8 ✅** | 27 | +18 | Higher | -1 | Lower | -19 | Lower |
| Cognitive (sum) | 16 | **7 ✅** | 17 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 5860.5560 | **1100.2376 ✅** | 5766.8371 | +4760.3184 | Higher | +93.7189 | Higher | -4666.5995 | Lower |
| Halstead Effort | 755602.8490 | **30806.6532 ✅** | 692020.4514 | +724796.1958 | Higher | +63582.3976 | Higher | -661213.7982 | Lower |
| SLOC | 152 | **46 ✅** | 146 | +106 | Higher | +6 | Higher | -100 | Lower |
| PLOC | 133 | **27 ✅** | 127 | +106 | Higher | +6 | Higher | -100 | Lower |
| LLOC | 94 | **16 ✅** | 90 | +78 | Higher | +4 | Higher | -74 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 0 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 19 | **8 ✅** | 19 | +11 | Higher | 0 | Same | -11 | Lower |
| Functions | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p02731

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 91.1202 | **91.1891 ✅** | 90.9561 | -0.0689 | Lower | +0.1642 | Higher | +0.2330 | Higher |
| MI (SEI) | 93.4769 | **93.5763 ✅** | 92.5563 | -0.0994 | Lower | +0.9206 | Higher | +1.0200 | Higher |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 380 | 375 | **336.8853 ✅** | +5 | Higher | +43.1147 | Higher | +38.1147 | Higher |
| Halstead Effort | 3990 | **3335.5263 ✅** | 3467.9374 | +654.4737 | Higher | +522.0626 | Higher | -132.4111 | Lower |
| SLOC | 20 | 20 | 21 | 0 | Same | -1 | Lower | -1 | Lower |
| PLOC | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02733

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.1890 | **66.8120 ✅** | 49.2378 | -12.6230 | Lower | +4.9511 | Higher | +17.5741 | Higher |
| MI (SEI) | 22.8633 | **56.4333 ✅** | 17.9954 | -33.5700 | Lower | +4.8680 | Higher | +38.4380 | Higher |
| Cyclomatic (sum) | 19 | **10 ✅** | 25 | +9 | Higher | -6 | Lower | -15 | Lower |
| Cognitive (sum) | 44 | **16 ✅** | 66 | +28 | Higher | -22 | Lower | -50 | Lower |
| Halstead Volume | 2488.8541 | **1094.6527 ✅** | 3059.7693 | +1394.2014 | Higher | -570.9153 | Lower | -1965.1167 | Lower |
| Halstead Effort | 131169.3355 | **38269.0566 ✅** | 237492.0958 | +92900.2788 | Higher | -106322.7603 | Lower | -199223.0392 | Lower |
| SLOC | 84 | **57 ✅** | 98 | +27 | Higher | -14 | Lower | -41 | Lower |
| PLOC | 72 | **39 ✅** | 90 | +33 | Higher | -18 | Lower | -51 | Lower |
| LLOC | 66 | **28 ✅** | 87 | +38 | Higher | -21 | Lower | -59 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 7 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 10 | **7 ✅** | 8 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02734

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.0452 | **82.8516 ✅** | 65.5610 | -14.8064 | Lower | +2.4842 | Higher | +17.2906 | Higher |
| MI (SEI) | 48.3248 | **81.6791 ✅** | 45.4651 | -33.3543 | Lower | +2.8597 | Higher | +36.2140 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 10 | +4 | Higher | -1 | Lower | -5 | Lower |
| Cognitive (sum) | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 2021.7564 | **640.7628 ✅** | 2094.2868 | +1380.9937 | Higher | -72.5303 | Lower | -1453.5240 | Lower |
| Halstead Effort | 138571.1854 | **14872.4410 ✅** | 160343.8310 | +123698.7444 | Higher | -21772.6456 | Lower | -145471.3900 | Lower |
| SLOC | 44 | **27 ✅** | 50 | +17 | Higher | -6 | Lower | -23 | Lower |
| PLOC | 38 | **18 ✅** | 39 | +20 | Higher | -1 | Lower | -21 | Lower |
| LLOC | 36 | **13 ✅** | 41 | +23 | Higher | -5 | Lower | -28 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 6 | -3 | Lower | -1 | Lower | +2 | Higher |
| Blank Lines | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02743

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.5331 | **87.5308 ✅** | 84.3892 | -1.9977 | Lower | +1.1438 | Higher | +3.1416 | Higher |
| MI (SEI) | 78.2067 | **91.0532 ✅** | 83.0033 | -12.8464 | Lower | -4.7966 | Lower | +8.0499 | Higher |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 659.0224 | **346.1296 ✅** | 633.3094 | +312.8928 | Higher | +25.7129 | Higher | -287.1799 | Lower |
| Halstead Effort | 9872.6621 | **4499.6842 ✅** | 8893.8674 | +5372.9779 | Higher | +978.7947 | Higher | -4394.1832 | Lower |
| SLOC | **23 ✅** | 25 | 25 | -2 | Lower | -2 | Lower | 0 | Same |
| PLOC | 15 | 15 | 15 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 4 | **9 ✅** | 7 | -5 | Lower | -3 | Lower | +2 | Higher |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02761

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.8184 | **66.7025 ✅** | 61.5771 | -4.8841 | Lower | +0.2413 | Higher | +5.1254 | Higher |
| MI (SEI) | 25.6961 | **60.5897 ✅** | 25.2587 | -34.8937 | Lower | +0.4374 | Higher | +35.3311 | Higher |
| Cyclomatic (sum) | 19 | **13 ✅** | 19 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 25 | **13 ✅** | 25 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 2029.5119 | **1094.2540 ✅** | 2011.8639 | +935.2578 | Higher | +17.6479 | Higher | -917.6099 | Lower |
| Halstead Effort | 113652.6637 | **39393.1441 ✅** | 106094.3866 | +74259.5196 | Higher | +7558.2772 | Higher | -66701.2424 | Lower |
| SLOC | 56 | **55 ✅** | 57 | +1 | Higher | -1 | Lower | -2 | Lower |
| PLOC | 49 | **34 ✅** | 50 | +15 | Higher | -1 | Lower | -16 | Lower |
| LLOC | 39 | **23 ✅** | 38 | +16 | Higher | +1 | Higher | -15 | Lower |
| Comments (CLOC) | 1 | **18 ✅** | 1 | -17 | Lower | 0 | Same | +17 | Higher |
| Blank Lines | 7 | 10 | 7 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 7 | 0 | -7 | Lower | 0 | Same | +7 | Higher |

## p02762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.0153 | **52.9011 ✅** | 50.9340 | -0.8858 | Lower | +1.0813 | Higher | +1.9671 | Higher |
| MI (SEI) | 9.0993 | **39.9627 ✅** | 7.3243 | -30.8635 | Lower | +1.7749 | Higher | +32.6384 | Higher |
| Cyclomatic (sum) | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 12 | 12 | 15 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | 3481.9096 | **2297.7804 ✅** | 3622.2011 | +1184.1292 | Higher | -140.2914 | Lower | -1324.4206 | Lower |
| Halstead Effort | 169050.2664 | **96446.3106 ✅** | 233505.6121 | +72603.9557 | Higher | -64455.3457 | Lower | -137059.3014 | Lower |
| SLOC | **90 ✅** | 96 | 95 | -6 | Lower | -5 | Lower | +1 | Higher |
| PLOC | 73 | **68 ✅** | 82 | +5 | Higher | -9 | Lower | -14 | Lower |
| LLOC | 59 | **42 ✅** | 71 | +17 | Higher | -12 | Lower | -29 | Lower |
| Comments (CLOC) | 1 | **29 ✅** | 1 | -28 | Lower | 0 | Same | +28 | Higher |
| Blank Lines | 17 | 14 | **13 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p02783

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.2582 | 74.2838 | **88.7261 ✅** | +1.9743 | Higher | -12.4679 | Lower | -14.4423 | Lower |
| MI (SEI) | 69.8256 | 65.0423 | **86.9076 ✅** | +4.7834 | Higher | -17.0819 | Lower | -21.8653 | Lower |
| Cyclomatic (sum) | 9 | 7 | **3 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Cognitive (sum) | 4 | 4 | **1 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 714.8752 | 769.2152 | **494.8981 ✅** | -54.3400 | Lower | +219.9771 | Higher | +274.3172 | Higher |
| Halstead Effort | 15369.8168 | 18091.9426 | **6520.2821 ✅** | -2722.1258 | Lower | +8849.5347 | Higher | +11571.6606 | Higher |
| SLOC | 37 | 42 | **21 ✅** | -5 | Lower | +16 | Higher | +21 | Higher |
| PLOC | 24 | 26 | **13 ✅** | -2 | Lower | +11 | Higher | +13 | Higher |
| LLOC | 13 | 11 | **10 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Comments (CLOC) | 9 | 9 | 5 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 7 | 7 | **4 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p02812

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.6775 | **78.2527 ✅** | 75.3702 | -4.5752 | Lower | -1.6927 | Lower | +2.8825 | Higher |
| MI (SEI) | 63.7484 | **80.4791 ✅** | 64.9475 | -16.7307 | Lower | -1.1990 | Lower | +15.5317 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 839.5814 | **532.1928 ✅** | 926.4271 | +307.3886 | Higher | -86.8457 | Lower | -394.2343 | Lower |
| Halstead Effort | 22248.9076 | **15167.4951 ✅** | 22645.9964 | +7081.4125 | Higher | -397.0888 | Lower | -7478.5013 | Lower |
| SLOC | 43 | 37 | 37 | +6 | Higher | +6 | Higher | 0 | Same |
| PLOC | 29 | **24 ✅** | 25 | +5 | Higher | +4 | Higher | -1 | Lower |
| LLOC | 24 | **17 ✅** | 24 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 9 | **16 ✅** | 7 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 8 | **5 ✅** | 7 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | **0 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p02831

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.0182 | 67.1792 | **81.4888 ✅** | +13.8389 | Higher | -0.4707 | Lower | -14.3096 | Lower |
| MI (SEI) | 61.5800 | 54.6206 | **84.9978 ✅** | +6.9593 | Higher | -23.4178 | Lower | -30.3772 | Lower |
| Cyclomatic (sum) | 6 | 12 | **4 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Cognitive (sum) | 5 | 7 | **3 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Halstead Volume | 698.1203 | 1104.9384 | **512.6840 ✅** | -406.8181 | Lower | +185.4363 | Higher | +592.2544 | Higher |
| Halstead Effort | 12700.4195 | 32264.2013 | **7792.7970 ✅** | -19563.7818 | Lower | +4907.6225 | Higher | +24471.4043 | Higher |
| SLOC | **29 ✅** | 54 | 32 | -25 | Lower | -3 | Lower | +22 | Higher |
| PLOC | 23 | 39 | **14 ✅** | -16 | Lower | +9 | Higher | +25 | Higher |
| LLOC | 16 | 24 | **9 ✅** | -8 | Lower | +7 | Higher | +15 | Higher |
| Comments (CLOC) | 2 | 11 | **14 ✅** | -9 | Lower | -12 | Lower | -3 | Lower |
| Blank Lines | **4 ✅** | 7 | 5 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.5300 | **70.1832 ✅** | 61.8758 | -9.6532 | Lower | -1.3458 | Lower | +8.3074 | Higher |
| MI (SEI) | 33.6871 | **60.6727 ✅** | 24.9471 | -26.9856 | Lower | +8.7400 | Higher | +35.7256 | Higher |
| Cyclomatic (sum) | 15 | **7 ✅** | 15 | +8 | Higher | 0 | Same | -8 | Lower |
| Cognitive (sum) | 26 | **7 ✅** | 26 | +19 | Higher | 0 | Same | -19 | Lower |
| Halstead Volume | 1774.9682 | **983.1622 ✅** | 1835.3846 | +791.8060 | Higher | -60.4164 | Lower | -852.2224 | Lower |
| Halstead Effort | 88630.0809 | **25910.4212 ✅** | 90348.2886 | +62719.6597 | Higher | -1718.2078 | Lower | -64437.8674 | Lower |
| SLOC | 67 | **50 ✅** | 61 | +17 | Higher | +6 | Higher | -11 | Lower |
| PLOC | 53 | **28 ✅** | 53 | +25 | Higher | 0 | Same | -25 | Lower |
| LLOC | 36 | **21 ✅** | 37 | +15 | Higher | -1 | Lower | -16 | Lower |
| Comments (CLOC) | 5 | **12 ✅** | 1 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | 9 | 11 | **7 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02840

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.1259 | **75.0645 ✅** | 49.7194 | -25.9386 | Lower | -0.5936 | Lower | +25.3450 | Higher |
| MI (SEI) | 4.9628 | **75.0298 ✅** | 5.7962 | -70.0671 | Lower | -0.8334 | Lower | +69.2337 | Higher |
| Cyclomatic (sum) | 20 | **9 ✅** | 19 | +11 | Higher | +1 | Higher | -10 | Lower |
| Cognitive (sum) | 24 | **6 ✅** | 23 | +18 | Higher | +1 | Higher | -17 | Lower |
| Halstead Volume | 3778.7675 | **705.4144 ✅** | 3754.8982 | +3073.3531 | Higher | +23.8693 | Higher | -3049.4838 | Lower |
| Halstead Effort | 263266.0185 | **20686.2778 ✅** | 283377.4754 | +242579.7407 | Higher | -20111.4569 | Lower | -262691.1976 | Lower |
| SLOC | 99 | **40 ✅** | 97 | +59 | Higher | +2 | Higher | -57 | Lower |
| PLOC | 85 | **25 ✅** | 84 | +60 | Higher | +1 | Higher | -59 | Lower |
| LLOC | 72 | **14 ✅** | 73 | +58 | Higher | -1 | Lower | -59 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 1 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 14 | **6 ✅** | 13 | +8 | Higher | +1 | Higher | -7 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p02846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.1944 | **78.9648 ✅** | 61.7742 | -9.7704 | Lower | +7.4203 | Higher | +17.1906 | Higher |
| MI (SEI) | 36.2619 | **84.5539 ✅** | 33.0574 | -48.2920 | Lower | +3.2045 | Higher | +51.4965 | Higher |
| Cyclomatic (sum) | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 11 | **5 ✅** | 7 | +6 | Higher | +4 | Higher | -2 | Lower |
| Halstead Volume | 1475.0682 | **634.1371 ✅** | 2520.2850 | +840.9310 | Higher | -1045.2168 | Lower | -1886.1479 | Lower |
| Halstead Effort | 52549.3029 | **16555.5086 ✅** | 175519.8482 | +35993.7943 | Higher | -122970.5454 | Lower | -158964.3396 | Lower |
| SLOC | 46 | **33 ✅** | 63 | +13 | Higher | -17 | Lower | -30 | Lower |
| PLOC | 37 | **20 ✅** | 51 | +17 | Higher | -14 | Lower | -31 | Lower |
| LLOC | 24 | **10 ✅** | 54 | +14 | Higher | -30 | Lower | -44 | Lower |
| Comments (CLOC) | 1 | **18 ✅** | 4 | -17 | Lower | -3 | Lower | +14 | Higher |
| Blank Lines | 8 | **3 ✅** | 8 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02862

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.5529 | 58.3438 | **63.4262 ✅** | +4.2091 | Higher | -0.8733 | Lower | -5.0824 | Lower |
| MI (SEI) | 15.8676 | **35.7734 ✅** | 16.9239 | -19.9058 | Lower | -1.0563 | Lower | +18.8495 | Higher |
| Cyclomatic (sum) | 13 | 14 | **11 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **1952.9764 ✅** | 2162.4177 | 2008.3428 | -209.4413 | Lower | -55.3664 | Lower | +154.0749 | Higher |
| Halstead Effort | **94891.6764 ✅** | 115547.0804 | 103520.9426 | -20655.4040 | Lower | -8629.2663 | Lower | +12026.1377 | Higher |
| SLOC | 59 | 73 | **57 ✅** | -14 | Lower | +2 | Higher | +16 | Higher |
| PLOC | 48 | 55 | 48 | -7 | Lower | 0 | Same | +7 | Higher |
| LLOC | **28 ✅** | 37 | 37 | -9 | Lower | -9 | Lower | 0 | Same |
| Comments (CLOC) | 0 | **9 ✅** | 0 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 11 | 14 | **9 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02884

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.3548 | **63.7406 ✅** | 55.1516 | -12.3857 | Lower | -3.7968 | Lower | +8.5889 | Higher |
| MI (SEI) | **23.4751 ✅** | 17.6828 | 6.0043 | +5.7923 | Higher | +17.4708 | Higher | +11.6785 | Higher |
| Cyclomatic (sum) | 21 | **14 ✅** | 21 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 37 | **25 ✅** | 32 | +12 | Higher | +5 | Higher | -7 | Lower |
| Halstead Volume | 2677.8214 | **1959.3468 ✅** | 2585.8015 | +718.4746 | Higher | +92.0199 | Higher | -626.4547 | Lower |
| Halstead Effort | 155241.2680 | **69580.7062 ✅** | 175521.0701 | +85660.5618 | Higher | -20279.8021 | Lower | -105940.3639 | Lower |
| SLOC | 95 | **54 ✅** | 76 | +41 | Higher | +19 | Higher | -22 | Lower |
| PLOC | 71 | **45 ✅** | 65 | +26 | Higher | +6 | Higher | -20 | Lower |
| LLOC | 64 | **29 ✅** | 61 | +35 | Higher | +3 | Higher | -32 | Lower |
| Comments (CLOC) | **9 ✅** | 0 | 0 | +9 | Higher | +9 | Higher | 0 | Same |
| Blank Lines | 17 | **9 ✅** | 11 | +8 | Higher | +6 | Higher | -2 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p02885

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.5280 | 80.1881 | **88.9578 ✅** | +7.3399 | Higher | -1.4298 | Lower | -8.7697 | Lower |
| MI (SEI) | 84.6434 | 77.2434 | **84.8791 ✅** | +7.4001 | Higher | -0.2356 | Lower | -7.6357 | Lower |
| Cyclomatic (sum) | 4 | 9 | **3 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Cognitive (sum) | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **515.7359 ✅** | 717.1782 | 551.0324 | -201.4423 | Lower | -35.2965 | Lower | +166.1458 | Higher |
| Halstead Effort | 8306.0621 | 12737.7088 | **7013.1395 ✅** | -4431.6467 | Lower | +1292.9226 | Higher | +5724.5693 | Higher |
| SLOC | 22 | 29 | **20 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| PLOC | 11 | 18 | 11 | -7 | Lower | 0 | Same | +7 | Higher |
| LLOC | **7 ✅** | 9 | 8 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 5 | **8 ✅** | 4 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02923

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.7033 | **81.7766 ✅** | 54.0506 | -6.0733 | Lower | +21.6527 | Higher | +27.7260 | Higher |
| MI (SEI) | **66.6477 ✅** | 42.9905 | 4.3141 | +23.6572 | Higher | +62.3337 | Higher | +38.6765 | Higher |
| Cyclomatic (sum) | **5 ✅** | 7 | 20 | -2 | Lower | -15 | Lower | -13 | Lower |
| Cognitive (sum) | **5 ✅** | 7 | 15 | -2 | Lower | -10 | Lower | -8 | Lower |
| Halstead Volume | 873.6286 | **577.2720 ✅** | 2192.1419 | +296.3566 | Higher | -1318.5133 | Lower | -1614.8699 | Lower |
| Halstead Effort | 22790.3120 | **17462.4788 ✅** | 107000.2228 | +5327.8332 | Higher | -84209.9108 | Lower | -89537.7440 | Lower |
| SLOC | 38 | **29 ✅** | 87 | +9 | Higher | -49 | Lower | -58 | Lower |
| PLOC | 25 | **24 ✅** | 73 | +1 | Higher | -48 | Lower | -49 | Lower |
| LLOC | 22 | **17 ✅** | 46 | +5 | Higher | -24 | Lower | -29 | Lower |
| Comments (CLOC) | **8 ✅** | 0 | 0 | +8 | Higher | +8 | Higher | 0 | Same |
| Blank Lines | 7 | **5 ✅** | 14 | +2 | Higher | -7 | Lower | -9 | Lower |
| Functions | **1 ✅** | 2 | 9 | -1 | Lower | -8 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 5 | -2 | Lower | -5 | Lower | -3 | Lower |

## p02945

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **86.9933 ✅** | 72.2498 | 78.9507 | +14.7435 | Higher | +8.0426 | Higher | -6.7009 | Lower |
| MI (SEI) | **83.0637 ✅** | 60.1286 | 69.0009 | +22.9351 | Higher | +14.0629 | Higher | -8.8722 | Lower |
| Cyclomatic (sum) | **2 ✅** | 9 | 5 | -7 | Lower | -3 | Lower | +4 | Higher |
| Cognitive (sum) | **0 ✅** | 5 | 3 | -5 | Lower | -3 | Lower | +2 | Higher |
| Halstead Volume | **543.7040 ✅** | 900.6695 | 661.6006 | -356.9655 | Lower | -117.8966 | Lower | +239.0689 | Higher |
| Halstead Effort | **7340.0040 ✅** | 23350.6901 | 12371.9308 | -16010.6861 | Lower | -5031.9268 | Lower | +10978.7593 | Higher |
| SLOC | **23 ✅** | 44 | 34 | -21 | Lower | -11 | Lower | +10 | Higher |
| PLOC | **13 ✅** | 28 | 22 | -15 | Lower | -9 | Lower | +6 | Higher |
| LLOC | **10 ✅** | 14 | 19 | -4 | Lower | -9 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 6 | -3 | Lower | -1 | Lower | +2 | Higher |
| Blank Lines | **5 ✅** | 8 | 6 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.0320 | 78.6463 | **81.9433 ✅** | -3.6142 | Lower | -6.9113 | Lower | -3.2971 | Lower |
| MI (SEI) | 63.1249 | 70.5938 | **79.3683 ✅** | -7.4689 | Lower | -16.2435 | Lower | -8.7745 | Lower |
| Cyclomatic (sum) | 7 | 5 | 5 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 5 | **2 ✅** | 3 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | 718.0631 | 701.4924 | **610.7609 ✅** | +16.5707 | Higher | +107.3021 | Higher | +90.7314 | Higher |
| Halstead Effort | 14174.5647 | 11871.4093 | **11342.7030 ✅** | +2303.1554 | Higher | +2831.8617 | Higher | +528.7064 | Higher |
| SLOC | 41 | 34 | **29 ✅** | +7 | Higher | +12 | Higher | +5 | Higher |
| PLOC | 25 | 21 | **18 ✅** | +4 | Higher | +7 | Higher | +3 | Higher |
| LLOC | 18 | **13 ✅** | 14 | +5 | Higher | +4 | Higher | -1 | Lower |
| Comments (CLOC) | 7 | 7 | **8 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 9 | 7 | **6 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02953

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.0622 | **80.7139 ✅** | 80.2406 | -0.6518 | Lower | -0.1784 | Lower | +0.4733 | Higher |
| MI (SEI) | 69.9717 | **85.4234 ✅** | 74.6936 | -15.4516 | Lower | -4.7219 | Lower | +10.7297 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 754.9244 | **494.8981 ✅** | 729.4619 | +260.0264 | Higher | +25.4625 | Higher | -234.5639 | Lower |
| Halstead Effort | 16127.9312 | **11062.4274 ✅** | 17142.3552 | +5065.5038 | Higher | -1014.4240 | Lower | -6079.9278 | Lower |
| SLOC | 30 | 33 | 30 | -3 | Lower | 0 | Same | +3 | Higher |
| PLOC | 22 | **16 ✅** | 21 | +6 | Higher | +1 | Higher | -5 | Lower |
| LLOC | 16 | **8 ✅** | 16 | +8 | Higher | 0 | Same | -8 | Lower |
| Comments (CLOC) | 5 | **16 ✅** | 7 | -11 | Lower | -2 | Lower | +9 | Higher |
| Blank Lines | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02956

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.2754 | **70.0694 ✅** | 32.2878 | -37.7940 | Lower | -0.0124 | Lower | +37.7816 | Higher |
| MI (SEI) | -17.2856 | **66.3068 ✅** | -9.3538 | -83.5924 | Lower | -7.9319 | Lower | +75.6605 | Higher |
| Cyclomatic (sum) | 37 | **7 ✅** | 37 | +30 | Higher | 0 | Same | -30 | Lower |
| Cognitive (sum) | 48 | **2 ✅** | 48 | +46 | Higher | 0 | Same | -46 | Lower |
| Halstead Volume | 6828.0109 | **746.7564 ✅** | 6929.7317 | +6081.2544 | Higher | -101.7208 | Lower | -6182.9753 | Lower |
| Halstead Effort | 993902.3298 | **13994.7683 ✅** | 1030797.5881 | +979907.5615 | Higher | -36895.2584 | Lower | -1016802.8199 | Lower |
| SLOC | 182 | **55 ✅** | 181 | +127 | Higher | +1 | Higher | -126 | Lower |
| PLOC | 157 | **27 ✅** | 156 | +130 | Higher | +1 | Higher | -129 | Lower |
| LLOC | 150 | **11 ✅** | 149 | +139 | Higher | +1 | Higher | -138 | Lower |
| Comments (CLOC) | 2 | **20 ✅** | 8 | -18 | Lower | -6 | Lower | +12 | Higher |
| Blank Lines | 25 | **10 ✅** | 25 | +15 | Higher | 0 | Same | -15 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p02957

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1929 | 75.6752 | **85.7932 ✅** | -1.4823 | Lower | -11.6003 | Lower | -10.1181 | Lower |
| MI (SEI) | 57.9012 | 69.9733 | **84.9271 ✅** | -12.0721 | Lower | -27.0259 | Lower | -14.9538 | Lower |
| Cyclomatic (sum) | 8 | 8 | **3 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Cognitive (sum) | 5 | 5 | **1 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Halstead Volume | 807.3024 | 769.2152 | **505.3170 ✅** | +38.0872 | Higher | +301.9854 | Higher | +263.8983 | Higher |
| Halstead Effort | 18918.9564 | 16984.2727 | **6657.5512 ✅** | +1934.6837 | Higher | +12261.4052 | Higher | +10326.7215 | Higher |
| SLOC | 41 | 38 | **25 ✅** | +3 | Higher | +16 | Higher | +13 | Higher |
| PLOC | 29 | 24 | **13 ✅** | +5 | Higher | +16 | Higher | +11 | Higher |
| LLOC | 15 | 12 | **9 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| Comments (CLOC) | 5 | **10 ✅** | 7 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 3 | **1 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02959

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.7198 | **78.2983 ✅** | 72.0171 | -5.5785 | Lower | +0.7027 | Higher | +6.2811 | Higher |
| MI (SEI) | 62.4019 | **79.1862 ✅** | 62.9838 | -16.7843 | Lower | -0.5819 | Lower | +16.2024 | Higher |
| Cyclomatic (sum) | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 15 | **1 ✅** | 15 | +14 | Higher | 0 | Same | -14 | Lower |
| Halstead Volume | 1198.1743 | **716.2669 ✅** | 1371.5333 | +481.9073 | Higher | -173.3590 | Lower | -655.2664 | Lower |
| Halstead Effort | 62023.1389 | **15882.4410 ✅** | 58633.0491 | +46140.6979 | Higher | +3390.0898 | Higher | -42750.6081 | Lower |
| SLOC | 39 | **35 ✅** | 39 | +4 | Higher | 0 | Same | -4 | Lower |
| PLOC | 23 | **17 ✅** | 25 | +6 | Higher | -2 | Lower | -8 | Lower |
| LLOC | 24 | **15 ✅** | 27 | +9 | Higher | -3 | Lower | -12 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 9 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02988

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.5718 | 72.2256 | **75.8311 ✅** | +2.3462 | Higher | -1.2593 | Lower | -3.6055 | Lower |
| MI (SEI) | 54.1479 | **63.3903 ✅** | 61.8052 | -9.2424 | Lower | -7.6572 | Lower | +1.5852 | Higher |
| Cyclomatic (sum) | 9 | 9 | **8 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 7 | 7 | 11 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | 988.7168 | 972.0530 | **883.4401 ✅** | +16.6638 | Higher | +105.2768 | Higher | +88.6130 | Higher |
| Halstead Effort | 32689.4497 | 31206.9531 | **26944.9219 ✅** | +1482.4966 | Higher | +5744.5278 | Higher | +4262.0312 | Higher |
| SLOC | 37 | 43 | **36 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| PLOC | 26 | 26 | **24 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| LLOC | 20 | 20 | **18 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 3 | **10 ✅** | 5 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 8 | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03003

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.4486 | **72.8326 ✅** | 66.7198 | -4.3840 | Lower | +1.7288 | Higher | +6.1128 | Higher |
| MI (SEI) | 50.7657 | **67.2564 ✅** | 43.7308 | -16.4907 | Lower | +7.0349 | Higher | +23.5256 | Higher |
| Cyclomatic (sum) | 9 | **7 ✅** | 11 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 10 | **9 ✅** | 14 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | 1744.3346 | **1096.1055 ✅** | 2079.0331 | +648.2291 | Higher | -334.6985 | Lower | -982.9276 | Lower |
| Halstead Effort | 77747.4830 | **42897.5827 ✅** | 123933.4709 | +34849.9003 | Higher | -46185.9879 | Lower | -81035.8882 | Lower |
| SLOC | 45 | **41 ✅** | 46 | +4 | Higher | -1 | Lower | -5 | Lower |
| PLOC | 31 | **24 ✅** | 34 | +7 | Higher | -3 | Lower | -10 | Lower |
| LLOC | 28 | **18 ✅** | 32 | +10 | Higher | -4 | Lower | -14 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 4 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 9 | **6 ✅** | 8 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **0 ✅** | 3 | +2 | Higher | -1 | Lower | -3 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03007

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.3823 | **68.8919 ✅** | 47.7816 | -17.5096 | Lower | +3.6007 | Higher | +21.1103 | Higher |
| MI (SEI) | 8.5052 | **56.3715 ✅** | 2.8003 | -47.8663 | Lower | +5.7049 | Higher | +53.5712 | Higher |
| Cyclomatic (sum) | 20 | **13 ✅** | 20 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 36 | **15 ✅** | 36 | +21 | Higher | 0 | Same | -21 | Lower |
| Halstead Volume | 3076.9001 | **1172.0182 ✅** | 4073.9216 | +1904.8818 | Higher | -997.0216 | Lower | -2901.9034 | Lower |
| Halstead Effort | 247019.8496 | **29951.5774 ✅** | 510578.7788 | +217068.2721 | Higher | -263558.9292 | Lower | -480627.2014 | Lower |
| SLOC | 92 | **47 ✅** | 105 | +45 | Higher | -13 | Lower | -58 | Lower |
| PLOC | 84 | **32 ✅** | 98 | +52 | Higher | -14 | Lower | -66 | Lower |
| LLOC | 61 | **20 ✅** | 78 | +41 | Higher | -17 | Lower | -58 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 1 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 8 | **0 ✅** | 8 | +8 | Higher | 0 | Same | -8 | Lower |

## p03029

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 90.0508 | 90.3630 | **91.0561 ✅** | -0.3122 | Lower | -1.0053 | Lower | -0.6931 | Lower |
| MI (SEI) | 89.3901 | **90.5524 ✅** | 88.4917 | -1.1623 | Lower | +0.8984 | Higher | +2.0607 | Higher |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 466.7648 | 515.7359 | **451.3769 ✅** | -48.9711 | Lower | +15.3878 | Higher | +64.3590 | Higher |
| Halstead Effort | 5251.1035 | 5626.2096 | **4739.4575 ✅** | -375.1061 | Lower | +511.6460 | Higher | +886.7521 | Higher |
| SLOC | 20 | 19 | 19 | +1 | Higher | +1 | Higher | 0 | Same |
| PLOC | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 7 | 8 | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 5 | 5 | 4 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03034

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.5073 | **77.9742 ✅** | 71.9725 | -9.4669 | Lower | -3.4652 | Lower | +6.0017 | Higher |
| MI (SEI) | 52.8403 | **77.0922 ✅** | 59.9274 | -24.2519 | Lower | -7.0871 | Lower | +17.1648 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 8 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 5 | +4 | Higher | +1 | Higher | -3 | Lower |
| Halstead Volume | 1167.8347 | **798.2752 ✅** | 1066.6888 | +369.5594 | Higher | +101.1458 | Higher | -268.4136 | Lower |
| Halstead Effort | 35521.6376 | **24726.5758 ✅** | 30894.4688 | +10795.0618 | Higher | +4627.1689 | Higher | -6167.8930 | Lower |
| SLOC | 51 | **34 ✅** | 43 | +17 | Higher | +8 | Higher | -9 | Lower |
| PLOC | 33 | **19 ✅** | 29 | +14 | Higher | +4 | Higher | -10 | Lower |
| LLOC | 29 | **16 ✅** | 27 | +13 | Higher | +2 | Higher | -11 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 10 | **5 ✅** | 9 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03043

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.2949 | 70.6315 | **80.6559 ✅** | +6.6634 | Higher | -3.3610 | Lower | -10.0244 | Lower |
| MI (SEI) | 54.9315 | **63.5396 ✅** | 55.6004 | -8.6081 | Lower | -0.6689 | Lower | +7.9392 | Higher |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 998.3469 | 1049.3987 | **872.7228 ✅** | -51.0518 | Lower | +125.6241 | Higher | +176.6759 | Higher |
| Halstead Effort | 26652.2956 | 21665.0052 | **21599.8890 ✅** | +4987.2905 | Higher | +5052.4066 | Higher | +65.1161 | Higher |
| SLOC | 33 | 49 | **28 ✅** | -16 | Lower | +5 | Higher | +21 | Higher |
| PLOC | 24 | 25 | **22 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| LLOC | 22 | **15 ✅** | 20 | +7 | Higher | +2 | Higher | -5 | Lower |
| Comments (CLOC) | 2 | **14 ✅** | 1 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 7 | 11 | **5 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03051

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.1962 | **76.3518 ✅** | 60.1223 | -16.1555 | Lower | +0.0739 | Higher | +16.2295 | Higher |
| MI (SEI) | 12.3658 | **76.4798 ✅** | 12.2592 | -64.1140 | Lower | +0.1066 | Higher | +64.2206 | Higher |
| Cyclomatic (sum) | 12 | **5 ✅** | 12 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 13 | **3 ✅** | 13 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 2492.7078 | **657.3039 ✅** | 2528.3894 | +1835.4040 | Higher | -35.6816 | Lower | -1871.0855 | Lower |
| Halstead Effort | 176415.7315 | **15446.6410 ✅** | 175975.9028 | +160969.0905 | Higher | +439.8287 | Higher | -160529.2618 | Lower |
| SLOC | 64 | **40 ✅** | 64 | +24 | Higher | 0 | Same | -24 | Lower |
| PLOC | 54 | **20 ✅** | 54 | +34 | Higher | 0 | Same | -34 | Lower |
| LLOC | 43 | **14 ✅** | 43 | +29 | Higher | 0 | Same | -29 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 0 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 10 | **8 ✅** | 10 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03053

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.9579 ✅** | 61.3401 | 59.3558 | +0.6178 | Higher | +2.6021 | Higher | +1.9843 | Higher |
| MI (SEI) | 40.2492 | **48.3016 ✅** | 32.5944 | -8.0524 | Lower | +7.6548 | Higher | +15.7072 | Higher |
| Cyclomatic (sum) | **13 ✅** | 14 | 15 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 16 | 23 | 16 | -7 | Lower | 0 | Same | +7 | Higher |
| Halstead Volume | 2078.0145 | **1831.1361 ✅** | 2694.9028 | +246.8784 | Higher | -616.8882 | Lower | -863.7667 | Lower |
| Halstead Effort | 77818.9794 | **75686.9582 ✅** | 129269.3250 | +2132.0212 | Higher | -51450.3456 | Lower | -53582.3668 | Lower |
| SLOC | **60 ✅** | 64 | 63 | -4 | Lower | -3 | Lower | +1 | Higher |
| PLOC | **43 ✅** | 46 | 54 | -3 | Lower | -11 | Lower | -8 | Lower |
| LLOC | 34 | **29 ✅** | 49 | +5 | Higher | -15 | Lower | -20 | Lower |
| Comments (CLOC) | 7 | **15 ✅** | 5 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03055

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.4893 | **66.6166 ✅** | 31.3663 | -30.1273 | Lower | +5.1230 | Higher | +35.2503 | Higher |
| MI (SEI) | -2.3048 | **51.7156 ✅** | -15.6042 | -54.0204 | Lower | +13.2994 | Higher | +67.3198 | Higher |
| Cyclomatic (sum) | 30 | **10 ✅** | 38 | +20 | Higher | -8 | Lower | -28 | Lower |
| Cognitive (sum) | 53 | **11 ✅** | 56 | +42 | Higher | -3 | Lower | -45 | Lower |
| Halstead Volume | 5616.8868 | **1607.2298 ✅** | 6176.5237 | +4009.6570 | Higher | -559.6369 | Lower | -4569.2939 | Lower |
| Halstead Effort | 635562.9560 | **54324.3679 ✅** | 819637.8430 | +581238.5881 | Higher | -184074.8870 | Lower | -765313.4751 | Lower |
| SLOC | 165 | **51 ✅** | 196 | +114 | Higher | -31 | Lower | -145 | Lower |
| PLOC | 144 | **36 ✅** | 172 | +108 | Higher | -28 | Lower | -136 | Lower |
| LLOC | 134 | **23 ✅** | 145 | +111 | Higher | -11 | Lower | -122 | Lower |
| Comments (CLOC) | 9 | 9 | 4 | 0 | Same | +5 | Higher | +5 | Higher |
| Blank Lines | 13 | **8 ✅** | 24 | +5 | Higher | -11 | Lower | -16 | Lower |
| Functions | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |

## p03064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.9175 | **76.5294 ✅** | 69.1592 | -6.6119 | Lower | +0.7583 | Higher | +7.3702 | Higher |
| MI (SEI) | 47.8350 | **70.6965 ✅** | 46.8428 | -22.8615 | Lower | +0.9922 | Higher | +23.8537 | Higher |
| Cyclomatic (sum) | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 5 | **1 ✅** | 7 | +4 | Higher | -2 | Lower | -6 | Lower |
| Halstead Volume | 1402.3082 | **814.2399 ✅** | 1552.2664 | +588.0682 | Higher | -149.9583 | Lower | -738.0265 | Lower |
| Halstead Effort | 50047.8951 | **13528.9096 ✅** | 73991.3671 | +36518.9854 | Higher | -23943.4721 | Lower | -60462.4575 | Lower |
| SLOC | 46 | **38 ✅** | 46 | +8 | Higher | 0 | Same | -8 | Lower |
| PLOC | 33 | **26 ✅** | 36 | +7 | Higher | -3 | Lower | -10 | Lower |
| LLOC | 33 | **16 ✅** | 35 | +17 | Higher | -2 | Lower | -19 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 4 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 9 | 6 | 6 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8627 | **72.7486 ✅** | 59.4989 | -12.8859 | Lower | +0.3638 | Higher | +13.2497 | Higher |
| MI (SEI) | 30.2324 | **71.2136 ✅** | 27.1300 | -40.9812 | Lower | +3.1024 | Higher | +44.0836 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 11 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 2200.4126 | **818.2861 ✅** | 2356.9114 | +1382.1265 | Higher | -156.4987 | Lower | -1538.6252 | Lower |
| Halstead Effort | 88425.8832 | **29151.4427 ✅** | 96869.0565 | +59274.4405 | Higher | -8443.1733 | Lower | -67717.6138 | Lower |
| SLOC | 68 | **44 ✅** | 69 | +24 | Higher | -1 | Lower | -25 | Lower |
| PLOC | 58 | **31 ✅** | 57 | +27 | Higher | +1 | Higher | -26 | Lower |
| LLOC | 49 | **24 ✅** | 48 | +25 | Higher | +1 | Higher | -24 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 3 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p03080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.7353 | **74.3999 ✅** | 73.4944 | -3.6646 | Lower | -2.7591 | Lower | +0.9055 | Higher |
| MI (SEI) | 60.6266 | 58.9882 | **62.9546 ✅** | +1.6384 | Higher | -2.3280 | Lower | -3.9665 | Lower |
| Cyclomatic (sum) | 12 | 10 | 10 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 11 | **5 ✅** | 12 | +6 | Higher | -1 | Lower | -7 | Lower |
| Halstead Volume | 918.9329 | **829.8336 ✅** | 845.1914 | +89.0993 | Higher | +73.7415 | Higher | -15.3578 | Lower |
| Halstead Effort | 28274.8586 | 17522.2558 | **17356.6086 ✅** | +10752.6029 | Higher | +10918.2500 | Higher | +165.6471 | Higher |
| SLOC | 46 | **39 ✅** | 41 | +7 | Higher | +5 | Higher | -2 | Lower |
| PLOC | 32 | **30 ✅** | 31 | +2 | Higher | +1 | Higher | -1 | Lower |
| LLOC | 20 | **16 ✅** | 19 | +4 | Higher | +1 | Higher | -3 | Lower |
| Comments (CLOC) | **10 ✅** | 5 | 8 | +5 | Higher | +2 | Higher | -3 | Lower |
| Blank Lines | 8 | **4 ✅** | 6 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p03105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **83.9531 ✅** | 79.5865 | 81.6880 | +4.3666 | Higher | +2.2650 | Higher | -2.1016 | Lower |
| MI (SEI) | 79.6946 | 74.2754 | **82.3211 ✅** | +5.4192 | Higher | -2.6265 | Lower | -8.0457 | Lower |
| Cyclomatic (sum) | 4 | 6 | **3 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 3 | 4 | **2 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 609.5060 | 614.7242 | **569.3504 ✅** | -5.2181 | Lower | +40.1556 | Higher | +45.3738 | Higher |
| Halstead Effort | 9325.4425 | 11679.7595 | **7259.2178 ✅** | -2354.3171 | Lower | +2066.2247 | Higher | +4420.5417 | Higher |
| SLOC | **26 ✅** | 33 | 31 | -7 | Lower | -5 | Lower | +2 | Higher |
| PLOC | 15 | 22 | 15 | -7 | Lower | 0 | Same | +7 | Higher |
| LLOC | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Comments (CLOC) | 6 | 8 | **11 ✅** | -2 | Lower | -5 | Lower | -3 | Lower |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.6204 ✅** | 69.7189 | 65.7768 | +3.9015 | Higher | +7.8436 | Higher | +3.9421 | Higher |
| MI (SEI) | 55.5345 | **59.2203 ✅** | 31.4857 | -3.6858 | Lower | +24.0487 | Higher | +27.7346 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 17 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | 12 | **7 ✅** | 30 | +5 | Higher | -18 | Lower | -23 | Lower |
| Halstead Volume | **882.4241 ✅** | 900.6639 | 1229.4484 | -18.2398 | Lower | -347.0243 | Lower | -328.7845 | Lower |
| Halstead Effort | **28295.1198 ✅** | 30602.1032 | 55448.1216 | -2306.9834 | Lower | -27153.0018 | Lower | -24846.0184 | Lower |
| SLOC | **39 ✅** | 50 | 53 | -11 | Lower | -14 | Lower | -3 | Lower |
| PLOC | **28 ✅** | 37 | 45 | -9 | Lower | -17 | Lower | -8 | Lower |
| LLOC | **19 ✅** | 20 | 35 | -1 | Lower | -16 | Lower | -15 | Lower |
| Comments (CLOC) | 4 | **11 ✅** | 1 | -7 | Lower | +3 | Higher | +10 | Higher |
| Blank Lines | 7 | **6 ✅** | 8 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.2844 | **56.8120 ✅** | 51.9433 | -2.5276 | Lower | +2.3411 | Higher | +4.8686 | Higher |
| MI (SEI) | 30.6257 | **37.3480 ✅** | 14.6443 | -6.7223 | Lower | +15.9813 | Higher | +22.7037 | Higher |
| Cyclomatic (sum) | 14 | **12 ✅** | 15 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 8 | **7 ✅** | 10 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 3048.4201 | **2208 ✅** | 3565.2417 | +840.4201 | Higher | -516.8217 | Lower | -1357.2417 | Lower |
| Halstead Effort | 130723.4256 | **79969.2308 ✅** | 251943.7500 | +50754.1948 | Higher | -121220.3244 | Lower | -171974.5192 | Lower |
| SLOC | 84 | **82 ✅** | 91 | +2 | Higher | -7 | Lower | -9 | Lower |
| PLOC | 71 | **58 ✅** | 77 | +13 | Higher | -6 | Lower | -19 | Lower |
| LLOC | 45 | **37 ✅** | 62 | +8 | Higher | -17 | Lower | -25 | Lower |
| Comments (CLOC) | 11 | **14 ✅** | 3 | -3 | Lower | +8 | Higher | +11 | Higher |
| Blank Lines | 13 | **12 ✅** | 14 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |

## p03115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.3126 ✅** | 63.7200 | 16.7433 | +0.5926 | Higher | +47.5692 | Higher | +46.9767 | Higher |
| MI (SEI) | 18.6099 | **55.6884 ✅** | -34.5879 | -37.0785 | Lower | +53.1978 | Higher | +90.2763 | Higher |
| Cyclomatic (sum) | 15 | **11 ✅** | 67 | +4 | Higher | -52 | Lower | -56 | Lower |
| Cognitive (sum) | 10 | **7 ✅** | 37 | +3 | Higher | -27 | Lower | -30 | Lower |
| Halstead Volume | 1888.8205 | **1202.1225 ✅** | 8595.3782 | +686.6980 | Higher | -6706.5577 | Lower | -7393.2557 | Lower |
| Halstead Effort | 104298.3045 | **36985.3013 ✅** | 858309.9091 | +67313.0033 | Higher | -754011.6046 | Lower | -821324.6079 | Lower |
| SLOC | **52 ✅** | 66 | 288 | -14 | Lower | -236 | Lower | -222 | Lower |
| PLOC | 44 | **37 ✅** | 242 | +7 | Higher | -198 | Lower | -205 | Lower |
| LLOC | 30 | **22 ✅** | 117 | +8 | Higher | -87 | Lower | -95 | Lower |
| Comments (CLOC) | 0 | **21 ✅** | 5 | -21 | Lower | -5 | Lower | +16 | Higher |
| Blank Lines | **8 ✅** | 12 | 41 | -4 | Lower | -33 | Lower | -29 | Lower |
| Functions | 3 | **2 ✅** | 35 | +1 | Higher | -32 | Lower | -33 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 4 | 34 | -1 | Lower | -31 | Lower | -30 | Lower |

## p03123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.9281 | **76.5102 ✅** | 41.5397 | -33.5821 | Lower | +1.3884 | Higher | +34.9705 | Higher |
| MI (SEI) | -4.4244 | **77.0138 ✅** | -1.9842 | -81.4382 | Lower | -2.4402 | Lower | +78.9980 | Higher |
| Cyclomatic (sum) | 25 | **8 ✅** | 25 | +17 | Higher | 0 | Same | -17 | Lower |
| Cognitive (sum) | 23 | **7 ✅** | 24 | +16 | Higher | -1 | Lower | -17 | Lower |
| Halstead Volume | 4373.2380 | **558.3519 ✅** | 4426.3022 | +3814.8861 | Higher | -53.0642 | Lower | -3867.9503 | Lower |
| Halstead Effort | 268846.5983 | **16806.3922 ✅** | 272217.5842 | +252040.2061 | Higher | -3370.9859 | Lower | -255411.1920 | Lower |
| SLOC | 129 | **40 ✅** | 140 | +89 | Higher | -11 | Lower | -100 | Lower |
| PLOC | 113 | **22 ✅** | 120 | +91 | Higher | -7 | Lower | -98 | Lower |
| LLOC | 81 | **15 ✅** | 91 | +66 | Higher | -10 | Lower | -76 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 3 | -15 | Lower | -2 | Lower | +13 | Higher |
| Blank Lines | 16 | **6 ✅** | 18 | +10 | Higher | -2 | Lower | -12 | Lower |
| Functions | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 8 | **2 ✅** | 8 | +6 | Higher | 0 | Same | -6 | Lower |

## p03142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.8219 | **74.1723 ✅** | 55.9894 | -17.3504 | Lower | +0.8325 | Higher | +18.1829 | Higher |
| MI (SEI) | 20.0557 | **64.6971 ✅** | 6.5002 | -44.6414 | Lower | +13.5555 | Higher | +58.1970 | Higher |
| Cyclomatic (sum) | 14 | **4 ✅** | 14 | +10 | Higher | 0 | Same | -10 | Lower |
| Cognitive (sum) | 20 | **2 ✅** | 20 | +18 | Higher | 0 | Same | -18 | Lower |
| Halstead Volume | 2454 | **1331.9501 ✅** | 2556.7787 | +1122.0499 | Higher | -102.7787 | Lower | -1224.8287 | Lower |
| Halstead Effort | 115854.6316 | **31021.5466 ✅** | 123812.0092 | +84833.0850 | Higher | -7957.3776 | Lower | -92790.4626 | Lower |
| SLOC | 77 | **37 ✅** | 80 | +40 | Higher | -3 | Lower | -43 | Lower |
| PLOC | 66 | **26 ✅** | 69 | +40 | Higher | -3 | Lower | -43 | Lower |
| LLOC | 46 | **22 ✅** | 52 | +24 | Higher | -6 | Lower | -30 | Lower |
| Comments (CLOC) | 2 | **8 ✅** | 0 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 11 | **6 ✅** | 11 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03145

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.0426 | 78.4574 | **88.1397 ✅** | +8.5852 | Higher | -1.0971 | Lower | -9.6823 | Lower |
| MI (SEI) | **85.6405 ✅** | 72.2358 | 82.9502 | +13.4048 | Higher | +2.6904 | Higher | -10.7144 | Lower |
| Cyclomatic (sum) | 2 | 6 | 2 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | **538.5747 ✅** | 695.9688 | 579.0316 | -157.3941 | Lower | -40.4569 | Lower | +116.9372 | Higher |
| Halstead Effort | **4932.8548 ✅** | 14963.3293 | 5400.0991 | -10030.4745 | Lower | -467.2443 | Lower | +9563.2303 | Higher |
| SLOC | 23 | 34 | **21 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| PLOC | **11 ✅** | 23 | 12 | -12 | Lower | -1 | Lower | +11 | Higher |
| LLOC | **8 ✅** | 13 | 9 | -5 | Lower | -1 | Lower | +4 | Higher |
| Comments (CLOC) | 6 | **8 ✅** | 4 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03157

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.2626 | 52.7008 | **59.8630 ✅** | -3.4382 | Lower | -10.6003 | Lower | -7.1621 | Lower |
| MI (SEI) | 7.0924 | **28.1437 ✅** | 11.9868 | -21.0514 | Lower | -4.8944 | Lower | +16.1569 | Higher |
| Cyclomatic (sum) | 33 | 23 | **13 ✅** | +10 | Higher | +20 | Higher | +10 | Higher |
| Cognitive (sum) | 27 | 20 | **13 ✅** | +7 | Higher | +14 | Higher | +7 | Higher |
| Halstead Volume | 3330.3974 | 2776.1864 | **2422.8102 ✅** | +554.2109 | Higher | +907.5872 | Higher | +353.3762 | Higher |
| Halstead Effort | 175317.1450 | 150608.1142 | **113629.7989 ✅** | +24709.0307 | Higher | +61687.3461 | Higher | +36978.3153 | Higher |
| SLOC | 85 | 84 | **65 ✅** | +1 | Higher | +20 | Higher | +19 | Higher |
| PLOC | 67 | 60 | **52 ✅** | +7 | Higher | +15 | Higher | +8 | Higher |
| LLOC | 49 | **31 ✅** | 35 | +18 | Higher | +14 | Higher | -4 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 0 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 18 | 14 | **13 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 12 | 3 | **1 ✅** | +9 | Higher | +11 | Higher | +2 | Higher |

## p03164

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.1103 | 57.2871 | **69.8937 ✅** | +2.8232 | Higher | -9.7834 | Lower | -12.6067 | Lower |
| MI (SEI) | 12.8527 | 37.7473 | **38.0933 ✅** | -24.8946 | Lower | -25.2405 | Lower | -0.3460 | Lower |
| Cyclomatic (sum) | 18 | 24 | **11 ✅** | -6 | Lower | +7 | Higher | +13 | Higher |
| Cognitive (sum) | 12 | 10 | 10 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 2041.2771 | 2330.7542 | **1499.2156 ✅** | -289.4770 | Lower | +542.0615 | Higher | +831.5385 | Higher |
| Halstead Effort | 86783.4395 | 89544.3225 | **56479.0722 ✅** | -2760.8831 | Lower | +30304.3673 | Higher | +33065.2504 | Higher |
| SLOC | 63 | 66 | **42 ✅** | -3 | Lower | +21 | Higher | +24 | Higher |
| PLOC | 50 | 47 | **34 ✅** | +3 | Higher | +16 | Higher | +13 | Higher |
| LLOC | 39 | 31 | **27 ✅** | +8 | Higher | +12 | Higher | +4 | Higher |
| Comments (CLOC) | 0 | **10 ✅** | 1 | -10 | Lower | -1 | Lower | +9 | Higher |
| Blank Lines | 13 | 9 | **8 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 8 | 6 | **3 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Exits | 2 | 10 | **1 ✅** | -8 | Lower | +1 | Higher | +9 | Higher |

## p03165

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.8314 ✅** | 61.1361 | 62.2869 | +1.6953 | Higher | +0.5446 | Higher | -1.1508 | Lower |
| MI (SEI) | **44.7101 ✅** | 41.8922 | 43.4099 | +2.8180 | Higher | +1.3002 | Higher | -1.5178 | Lower |
| Cyclomatic (sum) | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 16 | **15 ✅** | 16 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1851.1220 | **1803.8313 ✅** | 1936.5195 | +47.2907 | Higher | -85.3975 | Lower | -132.6882 | Lower |
| Halstead Effort | 93250.2705 | 104934.4146 | **92248.7464 ✅** | -11684.1441 | Lower | +1001.5241 | Higher | +12685.6682 | Higher |
| SLOC | **59 ✅** | 67 | 61 | -8 | Lower | -2 | Lower | +6 | Higher |
| PLOC | **43 ✅** | 45 | 45 | -2 | Lower | -2 | Lower | 0 | Same |
| LLOC | 37 | **35 ✅** | 39 | +2 | Higher | -2 | Lower | -4 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | **9 ✅** | 12 | 10 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03173

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.0914 ✅** | 63.5857 | 69.8445 | +6.5057 | Higher | +0.2469 | Higher | -6.2588 | Lower |
| MI (SEI) | 26.3360 | **50.8467 ✅** | 37.8186 | -24.5106 | Lower | -11.4825 | Lower | +13.0281 | Higher |
| Cyclomatic (sum) | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1699.7198 | **1648.0225 ✅** | 1653.4689 | +51.6973 | Higher | +46.2509 | Higher | -5.4464 | Lower |
| Halstead Effort | 94243.3900 | **49137.0912 ✅** | 81533.1205 | +45106.2988 | Higher | +12710.2695 | Higher | -32396.0293 | Lower |
| SLOC | **41 ✅** | 61 | 42 | -20 | Lower | -1 | Lower | +19 | Higher |
| PLOC | 34 | 38 | 34 | -4 | Lower | 0 | Same | +4 | Higher |
| LLOC | 26 | 25 | 25 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 0 | **14 ✅** | 1 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | **7 ✅** | 10 | 8 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03183

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.4200 | **75.6006 ✅** | 66.0863 | -8.1806 | Lower | +1.3337 | Higher | +9.5143 | Higher |
| MI (SEI) | 47.2783 | **69.8294 ✅** | 31.0159 | -22.5511 | Lower | +16.2624 | Higher | +38.8135 | Higher |
| Cyclomatic (sum) | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 7 | **3 ✅** | 7 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 1334.6055 | **703.2223 ✅** | 1724.8121 | +631.3832 | Higher | -390.2066 | Lower | -1021.5898 | Lower |
| Halstead Effort | 33623.4474 | **17721.2016 ✅** | 44436.6053 | +15902.2458 | Higher | -10813.1579 | Lower | -26715.4037 | Lower |
| SLOC | 53 | **41 ✅** | 53 | +12 | Higher | 0 | Same | -12 | Lower |
| PLOC | 38 | **24 ✅** | 42 | +14 | Higher | -4 | Lower | -18 | Lower |
| LLOC | 19 | **9 ✅** | 23 | +10 | Higher | -4 | Lower | -14 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 1 | -5 | Lower | +5 | Higher | +10 | Higher |
| Blank Lines | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03194

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.2068 | 71.3854 | **79.5906 ✅** | +1.8214 | Higher | -6.3838 | Lower | -8.2052 | Lower |
| MI (SEI) | 58.7582 | **63.5678 ✅** | 59.3078 | -4.8097 | Lower | -0.5496 | Lower | +4.2601 | Higher |
| Cyclomatic (sum) | 9 | 10 | **7 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 9 | 8 | **7 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Halstead Volume | 933.6510 | 828.5343 | **790.8268 ✅** | +105.1167 | Higher | +142.8242 | Higher | +37.7074 | Higher |
| Halstead Effort | 30847.8302 | 34154.0248 | **22736.2718 ✅** | -3306.1946 | Lower | +8111.5583 | Higher | +11417.7529 | Higher |
| SLOC | 41 | 47 | **30 ✅** | -6 | Lower | +11 | Higher | +17 | Higher |
| PLOC | 28 | 32 | **23 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| LLOC | 25 | 23 | **20 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Comments (CLOC) | 6 | **12 ✅** | 2 | -6 | Lower | +4 | Higher | +10 | Higher |
| Blank Lines | 7 | 9 | **5 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03206

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.2015 ✅** | 86.8314 | 85.9321 | +1.3701 | Higher | +2.2693 | Higher | +0.8993 | Higher |
| MI (SEI) | **86.7616 ✅** | 86.0486 | 74.4472 | +0.7130 | Higher | +12.3144 | Higher | +11.6014 | Higher |
| Cyclomatic (sum) | 9 | 9 | **6 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 6 | **3 ✅** | 4 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | 419.8342 | **411.5468 ✅** | 430.8598 | +8.2874 | Higher | -11.0256 | Lower | -19.3130 | Lower |
| Halstead Effort | 4758.1209 | **4389.8327 ✅** | 4452.2177 | +368.2882 | Higher | +305.9033 | Higher | -62.3850 | Lower |
| SLOC | **21 ✅** | 23 | 25 | -2 | Lower | -4 | Lower | -2 | Lower |
| PLOC | **15 ✅** | 17 | 18 | -2 | Lower | -3 | Lower | -1 | Lower |
| LLOC | 4 | 4 | 11 | 0 | Same | -7 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 3 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 2 | **1 ✅** | 4 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03209

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.6841 | 80.2068 | **81.7629 ✅** | -2.5228 | Lower | -4.0788 | Lower | -1.5561 | Lower |
| MI (SEI) | 60.1591 | **78.0768 ✅** | 73.7875 | -17.9177 | Lower | -13.6284 | Lower | +4.2893 | Higher |
| Cyclomatic (sum) | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 6 | **1 ✅** | 6 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1076.8490 | **596.0559 ✅** | 755.8125 | +480.7931 | Higher | +321.0365 | Higher | -159.7566 | Lower |
| Halstead Effort | 27129.6485 | **12517.1749 ✅** | 21603.6411 | +14612.4737 | Higher | +5526.0074 | Higher | -9086.4662 | Lower |
| SLOC | 31 | 33 | **27 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| PLOC | 25 | **16 ✅** | 20 | +9 | Higher | +5 | Higher | -4 | Lower |
| LLOC | 15 | **9 ✅** | 12 | +6 | Higher | +3 | Higher | -3 | Lower |
| Comments (CLOC) | 3 | **10 ✅** | 5 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 5 | 7 | **3 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |

## p03213

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.6006 | **82.6775 ✅** | 72.7383 | -11.0769 | Lower | -1.1377 | Lower | +9.9392 | Higher |
| MI (SEI) | 40.3910 | **83.6550 ✅** | 30.0528 | -43.2639 | Lower | +10.3382 | Higher | +53.6021 | Higher |
| Cyclomatic (sum) | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 1329.0504 | **507.3986 ✅** | 1353.1137 | +821.6518 | Higher | -24.0633 | Lower | -845.7151 | Lower |
| Halstead Effort | 54705.4287 | **13128.9391 ✅** | 62919.7869 | +41576.4896 | Higher | -8214.3582 | Lower | -49790.8478 | Lower |
| SLOC | 41 | **29 ✅** | 38 | +12 | Higher | +3 | Higher | -9 | Lower |
| PLOC | 33 | **20 ✅** | 32 | +13 | Higher | +1 | Higher | -12 | Lower |
| LLOC | 23 | **17 ✅** | 22 | +6 | Higher | +1 | Higher | -5 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 0 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 7 | **5 ✅** | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03216

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.9556 | **65.1842 ✅** | 60.4963 | -8.2286 | Lower | -3.5407 | Lower | +4.6879 | Higher |
| MI (SEI) | 36.6895 | **53.1200 ✅** | 13.0023 | -16.4305 | Lower | +23.6872 | Higher | +40.1177 | Higher |
| Cyclomatic (sum) | 18 | 14 | 14 | +4 | Higher | +4 | Higher | 0 | Same |
| Cognitive (sum) | 15 | **5 ✅** | 30 | +10 | Higher | -15 | Lower | -25 | Lower |
| Halstead Volume | 1850.0351 | **1484.3714 ✅** | 1867.3021 | +365.6637 | Higher | -17.2670 | Lower | -382.9306 | Lower |
| Halstead Effort | 69067.9774 | **42632.5287 ✅** | 78185.7447 | +26435.4487 | Higher | -9117.7673 | Lower | -35553.2160 | Lower |
| SLOC | 79 | **54 ✅** | 67 | +25 | Higher | +12 | Higher | -13 | Lower |
| PLOC | 59 | **37 ✅** | 58 | +22 | Higher | +1 | Higher | -21 | Lower |
| LLOC | 45 | **24 ✅** | 55 | +21 | Higher | -10 | Lower | -31 | Lower |
| Comments (CLOC) | 12 | 12 | 0 | 0 | Same | +12 | Higher | +12 | Higher |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03218

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.7212 | **71.4839 ✅** | 68.1407 | -0.7627 | Lower | +2.5805 | Higher | +3.3432 | Higher |
| MI (SEI) | 51.5846 | **64.8201 ✅** | 44.9400 | -13.2355 | Lower | +6.6446 | Higher | +19.8801 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 11 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 4 | **3 ✅** | 7 | +1 | Higher | -3 | Lower | -4 | Lower |
| Halstead Volume | 1099.7607 | **992.6744 ✅** | 1220.0362 | +107.0863 | Higher | -120.2754 | Lower | -227.3617 | Lower |
| Halstead Effort | 27316.6374 | **21462.3059 ✅** | 35625.0556 | +5854.3315 | Higher | -8308.4182 | Lower | -14162.7497 | Lower |
| SLOC | 46 | 46 | 50 | 0 | Same | -4 | Lower | -4 | Lower |
| PLOC | 33 | **29 ✅** | 36 | +4 | Higher | -3 | Lower | -7 | Lower |
| LLOC | 19 | 15 | 15 | +4 | Higher | +4 | Higher | 0 | Same |
| Comments (CLOC) | 5 | **13 ✅** | 4 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | **8 ✅** | 9 | 10 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | **1 ✅** | 2 | 4 | -1 | Lower | -3 | Lower | -2 | Lower |

## p03268

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.2664 | 67.6080 | **82.7523 ✅** | +10.6583 | Higher | -4.4859 | Lower | -15.1442 | Lower |
| MI (SEI) | 65.9777 | 50.9337 | **74.9094 ✅** | +15.0439 | Higher | -8.9318 | Lower | -23.9757 | Lower |
| Cyclomatic (sum) | 4 | 10 | **3 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Cognitive (sum) | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 865.6692 | 1111.5857 | **713.5293 ✅** | -245.9166 | Lower | +152.1399 | Higher | +398.0564 | Higher |
| Halstead Effort | 15880.5520 | 27861.3588 | **14066.7205 ✅** | -11980.8068 | Lower | +1813.8315 | Higher | +13794.6384 | Higher |
| SLOC | 33 | 54 | **27 ✅** | -21 | Lower | +6 | Higher | +27 | Higher |
| PLOC | 23 | 38 | **18 ✅** | -15 | Lower | +5 | Higher | +20 | Higher |
| LLOC | 15 | 20 | **11 ✅** | -5 | Lower | +4 | Higher | +9 | Higher |
| Comments (CLOC) | 5 | **8 ✅** | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 5 | 8 | **4 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.0885 | **74.0535 ✅** | 55.2188 | -17.9650 | Lower | +0.8697 | Higher | +18.8347 | Higher |
| MI (SEI) | 23.8240 | **70.8411 ✅** | 24.3655 | -47.0171 | Lower | -0.5415 | Lower | +46.4755 | Higher |
| Cyclomatic (sum) | 15 | **7 ✅** | 16 | +8 | Higher | -1 | Lower | -9 | Lower |
| Cognitive (sum) | 12 | **4 ✅** | 13 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 2495.8598 | **1012.8602 ✅** | 2611.0746 | +1482.9996 | Higher | -115.2148 | Lower | -1598.2144 | Lower |
| Halstead Effort | 129912.7006 | **24423.0000 ✅** | 140998.0265 | +105489.7006 | Higher | -11085.3259 | Lower | -116575.0265 | Lower |
| SLOC | 79 | **39 ✅** | 81 | +40 | Higher | -2 | Lower | -42 | Lower |
| PLOC | 65 | **26 ✅** | 71 | +39 | Higher | -6 | Lower | -45 | Lower |
| LLOC | 51 | **17 ✅** | 55 | +34 | Higher | -4 | Lower | -38 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 5 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | 14 | **6 ✅** | 10 | +8 | Higher | +4 | Higher | -4 | Lower |
| Functions | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03278

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.9001 | **64.8358 ✅** | 44.9461 | -18.9357 | Lower | +0.9540 | Higher | +19.8897 | Higher |
| MI (SEI) | 0.0199 | **57.8796 ✅** | -8.8210 | -57.8597 | Lower | +8.8409 | Higher | +66.7006 | Higher |
| Cyclomatic (sum) | 19 | **8 ✅** | 20 | +11 | Higher | -1 | Lower | -12 | Lower |
| Cognitive (sum) | 19 | **6 ✅** | 21 | +13 | Higher | -2 | Lower | -15 | Lower |
| Halstead Volume | 6299.4934 | **1570.6657 ✅** | 6437.4175 | +4728.8277 | Higher | -137.9241 | Lower | -4866.7518 | Lower |
| Halstead Effort | 916883.5859 | **43426.7839 ✅** | 907522.6015 | +873456.8020 | Higher | +9360.9844 | Higher | -864095.8175 | Lower |
| SLOC | 104 | **59 ✅** | 108 | +45 | Higher | -4 | Lower | -49 | Lower |
| PLOC | 92 | **35 ✅** | 95 | +57 | Higher | -3 | Lower | -60 | Lower |
| LLOC | 86 | **21 ✅** | 90 | +65 | Higher | -4 | Lower | -69 | Lower |
| Comments (CLOC) | 1 | **20 ✅** | 0 | -19 | Lower | +1 | Higher | +20 | Higher |
| Blank Lines | **12 ✅** | 13 | 13 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03294

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.4795 | 80.4070 | **85.8402 ✅** | +1.0725 | Higher | -4.3607 | Lower | -5.4332 | Lower |
| MI (SEI) | 71.8128 | 73.9517 | **80.9276 ✅** | -2.1388 | Lower | -9.1148 | Lower | -6.9760 | Lower |
| Cyclomatic (sum) | 4 | 5 | **3 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 627.9875 | 603.9396 | **568.6918 ✅** | +24.0480 | Higher | +59.2958 | Higher | +35.2478 | Higher |
| Halstead Effort | 9051.2114 | 11474.8515 | **7108.6469 ✅** | -2423.6401 | Lower | +1942.5645 | Higher | +4366.2046 | Higher |
| SLOC | 30 | 32 | **24 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| PLOC | 18 | 20 | **15 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| LLOC | 14 | 13 | **12 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Comments (CLOC) | 5 | **7 ✅** | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 7 | 5 | 5 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03296

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.4958 | 76.9835 | **77.3876 ✅** | -2.4877 | Lower | -2.8918 | Lower | -0.4040 | Lower |
| MI (SEI) | 61.0333 | **70.9452 ✅** | 67.2354 | -9.9119 | Lower | -6.2021 | Lower | +3.7098 | Higher |
| Cyclomatic (sum) | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 6 | 4 | 4 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 930.2778 | **773.2987 ✅** | 938.2737 | +156.9791 | Higher | -7.9959 | Lower | -164.9750 | Lower |
| Halstead Effort | 21524.7026 | **15589.7013 ✅** | 25802.5268 | +5935.0013 | Higher | -4277.8242 | Lower | -10212.8255 | Lower |
| SLOC | 39 | 36 | **33 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| PLOC | 27 | **20 ✅** | 28 | +7 | Higher | -1 | Lower | -8 | Lower |
| LLOC | 23 | **15 ✅** | 22 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 8 | 8 | **5 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03311

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.7272 | **82.0395 ✅** | 60.1686 | -20.3123 | Lower | +1.5586 | Higher | +21.8709 | Higher |
| MI (SEI) | 33.4590 | **80.4780 ✅** | 32.5782 | -47.0191 | Lower | +0.8807 | Higher | +47.8998 | Higher |
| Cyclomatic (sum) | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 17 | **1 ✅** | 17 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 1856.9800 | **589.3715 ✅** | 1982.4507 | +1267.6085 | Higher | -125.4707 | Lower | -1393.0793 | Lower |
| Halstead Effort | 111418.7985 | **6563.4548 ✅** | 119773.0636 | +104855.3437 | Higher | -8354.2651 | Lower | -113209.6088 | Lower |
| SLOC | 64 | **30 ✅** | 69 | +34 | Higher | -5 | Lower | -39 | Lower |
| PLOC | 51 | **15 ✅** | 55 | +36 | Higher | -4 | Lower | -40 | Lower |
| LLOC | 45 | **11 ✅** | 53 | +34 | Higher | -8 | Lower | -42 | Lower |
| Comments (CLOC) | 4 | **9 ✅** | 5 | -5 | Lower | -1 | Lower | +4 | Higher |
| Blank Lines | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.1754 | **83.2157 ✅** | 80.1990 | -8.0403 | Lower | -5.0236 | Lower | +3.0167 | Higher |
| MI (SEI) | 60.3255 | **86.7340 ✅** | 68.2452 | -26.4085 | Lower | -7.9197 | Lower | +18.4888 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 1005.2933 | **571.5856 ✅** | 911.6476 | +433.7076 | Higher | +93.6456 | Higher | -340.0620 | Lower |
| Halstead Effort | 22284.0004 | **10912.0896 ✅** | 23827.1535 | +11371.9108 | Higher | -1543.1532 | Lower | -12915.0639 | Lower |
| SLOC | 37 | **27 ✅** | 28 | +10 | Higher | +9 | Higher | -1 | Lower |
| PLOC | 25 | **14 ✅** | 19 | +11 | Higher | +6 | Higher | -5 | Lower |
| LLOC | 19 | **8 ✅** | 15 | +11 | Higher | +4 | Higher | -7 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 4 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 7 | 5 | 5 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03318

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.6911 | 68.1218 | **75.2623 ✅** | +5.5693 | Higher | -1.5712 | Lower | -7.1405 | Lower |
| MI (SEI) | 43.6023 | **55.2150 ✅** | 51.4469 | -11.6127 | Lower | -7.8446 | Lower | +3.7682 | Higher |
| Cyclomatic (sum) | 7 | 12 | 7 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 1085.9754 | 1036.7536 | **1030.1249 ✅** | +49.2218 | Higher | +55.8505 | Higher | +6.6287 | Higher |
| Halstead Effort | 32941.2529 | **30888.1073 ✅** | 35539.3078 | +2053.1456 | Higher | -2598.0549 | Lower | -4651.2004 | Lower |
| SLOC | 39 | 52 | **36 ✅** | -13 | Lower | +3 | Higher | +16 | Higher |
| PLOC | 32 | 43 | 32 | -11 | Lower | 0 | Same | +11 | Higher |
| LLOC | **27 ✅** | 30 | 28 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 1 | **10 ✅** | 2 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | 7 | 7 | **4 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p03361

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.8597 ✅** | 61.3165 | 67.8957 | +9.5432 | Higher | +2.9640 | Higher | -6.5792 | Lower |
| MI (SEI) | 52.0400 | 44.5426 | **54.1657 ✅** | +7.4974 | Higher | -2.1258 | Lower | -9.6231 | Lower |
| Cyclomatic (sum) | **14 ✅** | 16 | 15 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 32 | **22 ✅** | 31 | +10 | Higher | +1 | Higher | -9 | Lower |
| Halstead Volume | 1489.2593 | 1768.3963 | **1487.8148 ✅** | -279.1370 | Lower | +1.4446 | Higher | +280.5815 | Higher |
| Halstead Effort | 57310.8069 | 57353.3936 | **55459.5780 ✅** | -42.5868 | Lower | +1851.2288 | Higher | +1893.8156 | Higher |
| SLOC | **38 ✅** | 63 | 45 | -25 | Lower | -7 | Lower | +18 | Higher |
| PLOC | **29 ✅** | 51 | 34 | -22 | Lower | -5 | Lower | +17 | Higher |
| LLOC | **21 ✅** | 31 | 25 | -10 | Lower | -4 | Lower | +6 | Higher |
| Comments (CLOC) | 4 | **11 ✅** | 8 | -7 | Lower | -4 | Lower | +3 | Higher |
| Blank Lines | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03365

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.7842 | **69.1580 ✅** | 62.2583 | -4.3738 | Lower | +2.5259 | Higher | +6.8997 | Higher |
| MI (SEI) | 18.8830 | **62.7882 ✅** | 15.4425 | -43.9052 | Lower | +3.4404 | Higher | +47.3456 | Higher |
| Cyclomatic (sum) | 11 | **9 ✅** | 13 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 8 | **5 ✅** | 9 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | 2058.9251 | **1167.2572 ✅** | 2301.2528 | +891.6679 | Higher | -242.3277 | Lower | -1133.9956 | Lower |
| Halstead Effort | 122473.8708 | **49024.8008 ✅** | 146231.0768 | +73449.0700 | Higher | -23757.2060 | Lower | -97206.2760 | Lower |
| SLOC | 52 | **49 ✅** | 57 | +3 | Higher | -5 | Lower | -8 | Lower |
| PLOC | 43 | **37 ✅** | 46 | +6 | Higher | -3 | Lower | -9 | Lower |
| LLOC | 36 | **25 ✅** | 38 | +11 | Higher | -2 | Lower | -13 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 0 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 9 | **6 ✅** | 11 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p03369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **85.7070 ✅** | 79.4072 | 85.4184 | +6.2998 | Higher | +0.2886 | Higher | -6.0112 | Lower |
| MI (SEI) | **84.9044 ✅** | 76.6064 | 79.3541 | +8.2980 | Higher | +5.5503 | Higher | -2.7477 | Lower |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 491.5424 | 605.9969 | **459.8300 ✅** | -114.4545 | Lower | +31.7124 | Higher | +146.1669 | Higher |
| Halstead Effort | **6077.2516 ✅** | 7971.1901 | 7207.8351 | -1893.9386 | Lower | -1130.5836 | Lower | +763.3550 | Higher |
| SLOC | **25 ✅** | 34 | 26 | -9 | Lower | -1 | Lower | +8 | Higher |
| PLOC | **13 ✅** | 18 | 14 | -5 | Lower | -1 | Lower | +4 | Higher |
| LLOC | **9 ✅** | 11 | 10 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 7 | **10 ✅** | 5 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | **6 ✅** | 8 | 7 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03406

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.7430 | **54.2138 ✅** | 48.5835 | -4.4708 | Lower | +1.1594 | Higher | +5.6303 | Higher |
| MI (SEI) | 5.8922 | **38.3212 ✅** | -3.4714 | -32.4289 | Lower | +9.3636 | Higher | +41.7926 | Higher |
| Cyclomatic (sum) | 20 | **18 ✅** | 21 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 32 | **16 ✅** | 33 | +16 | Higher | -1 | Lower | -17 | Lower |
| Halstead Volume | 3463.7369 | **2406 ✅** | 3656.3166 | +1057.7369 | Higher | -192.5796 | Lower | -1250.3166 | Lower |
| Halstead Effort | 340435.8580 | **176294.1818 ✅** | 308828.1675 | +164141.6762 | Higher | +31607.6905 | Higher | -132533.9857 | Lower |
| SLOC | 98 | **86 ✅** | 102 | +12 | Higher | -4 | Lower | -16 | Lower |
| PLOC | 87 | **57 ✅** | 89 | +30 | Higher | -2 | Lower | -32 | Lower |
| LLOC | 70 | **44 ✅** | 66 | +26 | Higher | +4 | Higher | -22 | Lower |
| Comments (CLOC) | 1 | **20 ✅** | 0 | -19 | Lower | +1 | Higher | +20 | Higher |
| Blank Lines | **10 ✅** | 15 | 13 | -5 | Lower | -3 | Lower | +2 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p03408

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.9405 | **67.2098 ✅** | 66.2204 | -4.2693 | Lower | -3.2799 | Lower | +0.9894 | Higher |
| MI (SEI) | 44.6680 | **56.5814 ✅** | 45.1351 | -11.9134 | Lower | -0.4671 | Lower | +11.4463 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 10 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 15 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 1530.8001 | 1518.1973 | **1296.0773 ✅** | +12.6028 | Higher | +234.7227 | Higher | +222.1200 | Higher |
| Halstead Effort | 47733.1296 | **37148.3906 ✅** | 42511.3364 | +10584.7391 | Higher | +5221.7932 | Higher | -5362.9459 | Lower |
| SLOC | 65 | **53 ✅** | 56 | +12 | Higher | +9 | Higher | -3 | Lower |
| PLOC | 45 | **38 ✅** | 43 | +7 | Higher | +2 | Higher | -5 | Lower |
| LLOC | 37 | **28 ✅** | 35 | +9 | Higher | +2 | Higher | -7 | Lower |
| Comments (CLOC) | 10 | **13 ✅** | 6 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 11 | **7 ✅** | 8 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03414

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.7864 | **66.9358 ✅** | 21.7210 | -44.1494 | Lower | +1.0654 | Higher | +45.2148 | Higher |
| MI (SEI) | -37.1252 | **56.8988 ✅** | -30.1647 | -94.0240 | Lower | -6.9605 | Lower | +87.0635 | Higher |
| Cyclomatic (sum) | 56 | **13 ✅** | 55 | +43 | Higher | +1 | Higher | -42 | Lower |
| Cognitive (sum) | 64 | **11 ✅** | 65 | +53 | Higher | -1 | Lower | -54 | Lower |
| Halstead Volume | 9056.1351 | **1174.2237 ✅** | 9774.7075 | +7881.9114 | Higher | -718.5725 | Lower | -8600.4839 | Lower |
| Halstead Effort | 1177681.2936 | **37218.6974 ✅** | 1302968.5164 | +1140462.5961 | Higher | -125287.2228 | Lower | -1265749.8189 | Lower |
| SLOC | 228 | **53 ✅** | 241 | +175 | Higher | -13 | Lower | -188 | Lower |
| PLOC | 208 | **31 ✅** | 221 | +177 | Higher | -13 | Lower | -190 | Lower |
| LLOC | 158 | **18 ✅** | 163 | +140 | Higher | -5 | Lower | -145 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 3 | -13 | Lower | -3 | Lower | +10 | Higher |
| Blank Lines | 20 | **9 ✅** | 20 | +11 | Higher | 0 | Same | -11 | Lower |
| Functions | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 5 | **1 ✅** | 4 | +4 | Higher | +1 | Higher | -3 | Lower |
| Exits | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |

## p03434

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.7596 | **77.7027 ✅** | 55.9892 | -19.9430 | Lower | +1.7705 | Higher | +21.7135 | Higher |
| MI (SEI) | 21.5494 | **78.8750 ✅** | 6.6018 | -57.3256 | Lower | +14.9477 | Higher | +72.2733 | Higher |
| Cyclomatic (sum) | 13 | **6 ✅** | 15 | +7 | Higher | -2 | Lower | -9 | Lower |
| Cognitive (sum) | 14 | **5 ✅** | 19 | +9 | Higher | -5 | Lower | -14 | Lower |
| Halstead Volume | 2424 | **524.7928 ✅** | 2544 | +1899.2072 | Higher | -120 | Lower | -2019.2072 | Lower |
| Halstead Effort | 123530.7692 | **12627.8256 ✅** | 136169.2308 | +110902.9437 | Higher | -12638.4615 | Lower | -123541.4052 | Lower |
| SLOC | 74 | **39 ✅** | 79 | +35 | Higher | -5 | Lower | -40 | Lower |
| PLOC | 60 | **22 ✅** | 65 | +38 | Higher | -5 | Lower | -43 | Lower |
| LLOC | 55 | **15 ✅** | 57 | +40 | Higher | -2 | Lower | -42 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 0 | -14 | Lower | +2 | Higher | +16 | Higher |
| Blank Lines | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.6514 ✅** | 65.7869 | 67.1500 | +6.8646 | Higher | +5.5014 | Higher | -1.3631 | Lower |
| MI (SEI) | **56.2702 ✅** | 49.7255 | 43.7142 | +6.5447 | Higher | +12.5560 | Higher | +6.0112 | Higher |
| Cyclomatic (sum) | **11 ✅** | 12 | 13 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 11 | **8 ✅** | 21 | +3 | Higher | -10 | Lower | -13 | Lower |
| Halstead Volume | **1026.9765 ✅** | 1954.7369 | 1351.1399 | -927.7603 | Lower | -324.1634 | Lower | +603.5970 | Higher |
| Halstead Effort | **37578.0048 ✅** | 111645.5485 | 67065.6726 | -74067.5436 | Lower | -29487.6678 | Lower | +44579.8759 | Higher |
| SLOC | **40 ✅** | 49 | 50 | -9 | Lower | -10 | Lower | -1 | Lower |
| PLOC | **28 ✅** | 34 | 40 | -6 | Lower | -12 | Lower | -6 | Lower |
| LLOC | 21 | **11 ✅** | 29 | +10 | Higher | -8 | Lower | -18 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 4 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03448

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.8008 | **83.0544 ✅** | 75.2874 | -8.2536 | Lower | -0.4866 | Lower | +7.7670 | Higher |
| MI (SEI) | 61.6967 | **86.1068 ✅** | 57.6132 | -24.4101 | Lower | +4.0835 | Higher | +28.4937 | Higher |
| Cyclomatic (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 994.2412 | **503.6588 ✅** | 905.4263 | +490.5823 | Higher | +88.8149 | Higher | -401.7675 | Lower |
| Halstead Effort | 20989.5354 | **10325.0058 ✅** | 20330.9358 | +10664.5296 | Higher | +658.5997 | Higher | -10005.9299 | Lower |
| SLOC | 38 | **28 ✅** | 38 | +10 | Higher | 0 | Same | -10 | Lower |
| PLOC | 26 | **21 ✅** | 26 | +5 | Higher | 0 | Same | -5 | Lower |
| LLOC | 20 | **13 ✅** | 20 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 4 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 7 | **4 ✅** | 8 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03454

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.8658 | **79.9413 ✅** | 51.3451 | -30.0755 | Lower | -1.4793 | Lower | +28.5962 | Higher |
| MI (SEI) | 6.8853 | **82.1544 ✅** | 0.9199 | -75.2691 | Lower | +5.9654 | Higher | +81.2344 | Higher |
| Cyclomatic (sum) | 26 | **6 ✅** | 25 | +20 | Higher | +1 | Higher | -19 | Lower |
| Cognitive (sum) | 41 | **4 ✅** | 40 | +37 | Higher | +1 | Higher | -36 | Lower |
| Halstead Volume | 3054.1129 | **523.1869 ✅** | 2956.5258 | +2530.9260 | Higher | +97.5870 | Higher | -2433.3390 | Lower |
| Halstead Effort | 255840.6863 | **13951.6498 ✅** | 260174.2737 | +241889.0365 | Higher | -4333.5874 | Lower | -246222.6239 | Lower |
| SLOC | 93 | **34 ✅** | 87 | +59 | Higher | +6 | Higher | -53 | Lower |
| PLOC | 75 | **19 ✅** | 72 | +56 | Higher | +3 | Higher | -53 | Lower |
| LLOC | 55 | **12 ✅** | 53 | +43 | Higher | +2 | Higher | -41 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 0 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | 17 | **5 ✅** | 15 | +12 | Higher | +2 | Higher | -10 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |

## p03463

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.3831 | 79.9298 | **86.1932 ✅** | -0.5467 | Lower | -6.8101 | Lower | -6.2634 | Lower |
| MI (SEI) | 70.0125 | 77.3604 | **79.0573 ✅** | -7.3478 | Lower | -9.0448 | Lower | -1.6969 | Lower |
| Cyclomatic (sum) | 5 | 5 | **3 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 668.1505 | **548.0527 ✅** | 606.6998 | +120.0977 | Higher | +61.4507 | Higher | -58.6471 | Lower |
| Halstead Effort | 11160.5875 | 8101.6492 | **6875.9311 ✅** | +3058.9383 | Higher | +4284.6564 | Higher | +1225.7181 | Higher |
| SLOC | 33 | 34 | **23 ✅** | -1 | Lower | +10 | Higher | +11 | Higher |
| PLOC | 22 | 20 | **15 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| LLOC | 13 | 11 | **10 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Comments (CLOC) | 6 | **10 ✅** | 4 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p03473

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 91.7133 | 90.5501 | **95.3084 ✅** | +1.1632 | Higher | -3.5951 | Lower | -4.7583 | Lower |
| MI (SEI) | 91.9922 | 92.1743 | **96.1394 ✅** | -0.1821 | Lower | -4.1472 | Lower | -3.9651 | Lower |
| Cyclomatic (sum) | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 310.3352 | 333.4034 | **281.7629 ✅** | -23.0682 | Lower | +28.5723 | Higher | +51.6405 | Higher |
| Halstead Effort | 3258.5199 | 3621.1313 | **2403.7897 ✅** | -362.6114 | Lower | +854.7302 | Higher | +1217.3416 | Higher |
| SLOC | 20 | 21 | **17 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| PLOC | 12 | 13 | **8 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| LLOC | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 5 | **6 ✅** | 4 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 3 | **2 ✅** | 5 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03478

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.9579 | **75.8644 ✅** | 75.5473 | -12.9064 | Lower | -12.5893 | Lower | +0.3171 | Higher |
| MI (SEI) | 40.4283 | **75.1953 ✅** | 61.0902 | -34.7669 | Lower | -20.6619 | Lower | +14.1051 | Higher |
| Cyclomatic (sum) | 10 | 8 | **5 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 2427.1964 | **632.1911 ✅** | 1065.3932 | +1795.0052 | Higher | +1361.8032 | Higher | -433.2021 | Lower |
| Halstead Effort | 55846.6229 | **16836.2482 ✅** | 22722.8394 | +39010.3747 | Higher | +33123.7834 | Higher | -5886.5913 | Lower |
| SLOC | 56 | 40 | **36 ✅** | +16 | Higher | +20 | Higher | +4 | Higher |
| PLOC | 36 | 25 | **24 ✅** | +11 | Higher | +12 | Higher | +1 | Higher |
| LLOC | 21 | **17 ✅** | 20 | +4 | Higher | +1 | Higher | -3 | Lower |
| Comments (CLOC) | 6 | **15 ✅** | 5 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 14 | **8 ✅** | 9 | +6 | Higher | +5 | Higher | -1 | Lower |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.2291 | **72.6191 ✅** | 56.2102 | -15.3900 | Lower | +1.0189 | Higher | +16.4089 | Higher |
| MI (SEI) | 20.4395 | **71.5019 ✅** | 19.0191 | -51.0623 | Lower | +1.4204 | Higher | +52.4828 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 14 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 11 | **6 ✅** | 14 | +5 | Higher | -3 | Lower | -8 | Lower |
| Halstead Volume | 2479.0482 | **782.2026 ✅** | 2548.3960 | +1696.8456 | Higher | -69.3478 | Lower | -1766.1934 | Lower |
| Halstead Effort | 125426.4390 | **20989.1029 ✅** | 140333.9701 | +104437.3361 | Higher | -14907.5310 | Lower | -119344.8672 | Lower |
| SLOC | 77 | **45 ✅** | 79 | +32 | Higher | -2 | Lower | -34 | Lower |
| PLOC | 61 | **24 ✅** | 63 | +37 | Higher | -2 | Lower | -39 | Lower |
| LLOC | 56 | **11 ✅** | 57 | +45 | Higher | -1 | Lower | -46 | Lower |
| Comments (CLOC) | 2 | **18 ✅** | 2 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 16 | **9 ✅** | 16 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03544

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.4240 ✅** | 82.2117 | 36.3955 | +5.2123 | Higher | +51.0285 | Higher | +45.8162 | Higher |
| MI (SEI) | 84.4934 | **86.6643 ✅** | 1.3191 | -2.1709 | Lower | +83.1743 | Higher | +85.3452 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 38 | -2 | Lower | -34 | Lower | -32 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 34 | -1 | Lower | -32 | Lower | -31 | Lower |
| Halstead Volume | 526.1548 | **450.8319 ✅** | 4505.8018 | +75.3229 | Higher | -3979.6470 | Lower | -4054.9700 | Lower |
| Halstead Effort | 11385.4050 | **9580.1771 ✅** | 428952.3340 | +1805.2279 | Higher | -417566.9290 | Lower | -419372.1569 | Lower |
| SLOC | **22 ✅** | 31 | 159 | -9 | Lower | -137 | Lower | -128 | Lower |
| PLOC | **15 ✅** | 21 | 139 | -6 | Lower | -124 | Lower | -118 | Lower |
| LLOC | **13 ✅** | 16 | 98 | -3 | Lower | -85 | Lower | -82 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 12 | -9 | Lower | -7 | Lower | +2 | Higher |
| Blank Lines | **4 ✅** | 5 | 18 | -1 | Lower | -14 | Lower | -13 | Lower |
| Functions | **1 ✅** | 2 | 11 | -1 | Lower | -10 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 12 | -3 | Lower | -12 | Lower | -9 | Lower |

## p03562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.4781 | **73.5260 ✅** | 41.2474 | -31.0479 | Lower | +1.2307 | Higher | +32.2786 | Higher |
| MI (SEI) | -4.0230 | **64.3771 ✅** | -0.9014 | -68.4002 | Lower | -3.1216 | Lower | +65.2785 | Higher |
| Cyclomatic (sum) | 32 | **8 ✅** | 33 | +24 | Higher | -1 | Lower | -25 | Lower |
| Cognitive (sum) | 52 | **4 ✅** | 50 | +48 | Higher | +2 | Higher | -46 | Lower |
| Halstead Volume | 4742.7338 | **917.7708 ✅** | 4797.1771 | +3824.9630 | Higher | -54.4434 | Lower | -3879.4064 | Lower |
| Halstead Effort | 477039.9743 | **17835.3458 ✅** | 530994.2085 | +459204.6285 | Higher | -53954.2342 | Lower | -513158.8627 | Lower |
| SLOC | 117 | **41 ✅** | 124 | +76 | Higher | -7 | Lower | -83 | Lower |
| PLOC | 104 | **24 ✅** | 107 | +80 | Higher | -3 | Lower | -83 | Lower |
| LLOC | 83 | **16 ✅** | 90 | +67 | Higher | -7 | Lower | -74 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 3 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 13 | **8 ✅** | 16 | +5 | Higher | -3 | Lower | -8 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03568

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.7669 | 74.6127 | **84.3202 ✅** | +8.1542 | Higher | -1.5533 | Lower | -9.7075 | Lower |
| MI (SEI) | **74.5581 ✅** | 72.4314 | 61.7018 | +2.1267 | Higher | +12.8563 | Higher | +10.7296 | Higher |
| Cyclomatic (sum) | **4 ✅** | 8 | 5 | -4 | Lower | -1 | Lower | +3 | Higher |
| Cognitive (sum) | **3 ✅** | 11 | 4 | -8 | Lower | -1 | Lower | +7 | Higher |
| Halstead Volume | **607.8213 ✅** | 804.2346 | 614.0075 | -196.4133 | Lower | -6.1863 | Lower | +190.2271 | Higher |
| Halstead Effort | **9377.8137 ✅** | 18127.4479 | 11359.1391 | -8749.6342 | Lower | -1981.3255 | Lower | +6768.3088 | Higher |
| SLOC | 28 | 40 | **25 ✅** | -12 | Lower | +3 | Higher | +15 | Higher |
| PLOC | **17 ✅** | 24 | 19 | -7 | Lower | -2 | Lower | +5 | Higher |
| LLOC | **13 ✅** | 15 | 15 | -2 | Lower | -2 | Lower | 0 | Same |
| Comments (CLOC) | 5 | **14 ✅** | 1 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03592

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.2478 ✅** | 71.0888 | 52.4839 | +8.1590 | Higher | +26.7639 | Higher | +18.6049 | Higher |
| MI (SEI) | 53.5002 | **54.5168 ✅** | 2.2574 | -1.0166 | Lower | +51.2428 | Higher | +52.2594 | Higher |
| Cyclomatic (sum) | **9 ✅** | 11 | 22 | -2 | Lower | -13 | Lower | -11 | Lower |
| Cognitive (sum) | **8 ✅** | 12 | 18 | -4 | Lower | -10 | Lower | -6 | Lower |
| Halstead Volume | **773.2083 ✅** | 897.3731 | 2278.7366 | -124.1648 | Lower | -1505.5283 | Lower | -1381.3634 | Lower |
| Halstead Effort | **20103.4157 ✅** | 24021.9885 | 100176.7659 | -3918.5728 | Lower | -80073.3502 | Lower | -76154.7774 | Lower |
| SLOC | **30 ✅** | 46 | 92 | -16 | Lower | -62 | Lower | -46 | Lower |
| PLOC | **23 ✅** | 35 | 77 | -12 | Lower | -54 | Lower | -42 | Lower |
| LLOC | **13 ✅** | 22 | 47 | -9 | Lower | -34 | Lower | -25 | Lower |
| Comments (CLOC) | 1 | **6 ✅** | 0 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 6 | **5 ✅** | 15 | +1 | Higher | -9 | Lower | -10 | Lower |
| Functions | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |

## p03608

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.9341 | 51.3780 | **53.8638 ✅** | -1.4439 | Lower | -3.9297 | Lower | -2.4858 | Lower |
| MI (SEI) | 9.1942 | **26.0815 ✅** | 20.7161 | -16.8873 | Lower | -11.5219 | Lower | +5.3654 | Higher |
| Cyclomatic (sum) | 19 | 21 | **18 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 25 | **20 ✅** | 30 | +5 | Higher | -5 | Lower | -10 | Lower |
| Halstead Volume | 3381.1189 | **2946.2312 ✅** | 3101.5194 | +434.8877 | Higher | +279.5995 | Higher | -155.2882 | Lower |
| Halstead Effort | **138073.8555 ✅** | 150810.2116 | 158858.3119 | -12736.3561 | Lower | -20784.4564 | Lower | -8048.1003 | Lower |
| SLOC | 99 | 92 | **81 ✅** | +7 | Higher | +18 | Higher | +11 | Higher |
| PLOC | 85 | 73 | **69 ✅** | +12 | Higher | +16 | Higher | +4 | Higher |
| LLOC | 52 | **44 ✅** | 54 | +8 | Higher | -2 | Lower | -10 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 4 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 14 | 14 | **12 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03626

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.2159 | **86.0752 ✅** | 71.1478 | -20.8593 | Lower | -5.9319 | Lower | +14.9274 | Higher |
| MI (SEI) | 45.1332 | **92.3300 ✅** | 27.8601 | -47.1968 | Lower | +17.2730 | Higher | +64.4699 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 15 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 1555.1858 | **497.5405 ✅** | 1195.9332 | +1057.6453 | Higher | +359.2526 | Higher | -698.3927 | Lower |
| Halstead Effort | 57792.7096 | **10339.5125 ✅** | 42479.5472 | +47453.1971 | Higher | +15313.1625 | Higher | -32140.0346 | Lower |
| SLOC | 57 | **24 ✅** | 43 | +33 | Higher | +14 | Higher | -19 | Lower |
| PLOC | 42 | **17 ✅** | 36 | +25 | Higher | +6 | Higher | -19 | Lower |
| LLOC | 40 | **10 ✅** | 34 | +30 | Higher | +6 | Higher | -24 | Lower |
| Comments (CLOC) | 7 | **11 ✅** | 0 | -4 | Lower | +7 | Higher | +11 | Higher |
| Blank Lines | 9 | **4 ✅** | 7 | +5 | Higher | +2 | Higher | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03632

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.4845 | **86.7976 ✅** | 71.3845 | -19.3131 | Lower | -3.9001 | Lower | +15.4131 | Higher |
| MI (SEI) | 49.7756 | **92.4119 ✅** | 53.0406 | -42.6363 | Lower | -3.2650 | Lower | +39.3712 | Higher |
| Cyclomatic (sum) | 18 | **6 ✅** | 8 | +12 | Higher | +10 | Higher | -2 | Lower |
| Cognitive (sum) | 17 | **5 ✅** | 7 | +12 | Higher | +10 | Higher | -2 | Lower |
| Halstead Volume | 1231.5029 | **322.8412 ✅** | 1111.8397 | +908.6617 | Higher | +119.6632 | Higher | -788.9984 | Lower |
| Halstead Effort | 50993.3422 | **5230.0278 ✅** | 32675.7325 | +45763.3144 | Higher | +18317.6097 | Higher | -27445.7047 | Lower |
| SLOC | 47 | **26 ✅** | 44 | +21 | Higher | +3 | Higher | -18 | Lower |
| PLOC | 32 | **13 ✅** | 32 | +19 | Higher | 0 | Same | -19 | Lower |
| LLOC | 22 | **10 ✅** | 26 | +12 | Higher | -4 | Lower | -16 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 5 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 9 | **5 ✅** | 8 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p03646

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.3565 ✅** | 71.1669 | 69.8517 | +1.1896 | Higher | +2.5048 | Higher | +1.3153 | Higher |
| MI (SEI) | 50.3306 | **66.7784 ✅** | 42.2350 | -16.4478 | Lower | +8.0956 | Higher | +24.5434 | Higher |
| Cyclomatic (sum) | 8 | 8 | 11 | 0 | Same | -3 | Lower | -3 | Lower |
| Cognitive (sum) | 13 | **3 ✅** | 19 | +10 | Higher | -6 | Lower | -16 | Lower |
| Halstead Volume | 1342.9880 | **1080.9610 ✅** | 1219.0740 | +262.0270 | Higher | +123.9140 | Higher | -138.1130 | Lower |
| Halstead Effort | 41986.0466 | **29344.9113 ✅** | 36350.5706 | +12641.1353 | Higher | +5635.4760 | Higher | -7005.6593 | Lower |
| SLOC | **39 ✅** | 45 | 45 | -6 | Lower | -6 | Lower | 0 | Same |
| PLOC | 31 | **27 ✅** | 41 | +4 | Higher | -10 | Lower | -14 | Lower |
| LLOC | 23 | **16 ✅** | 30 | +7 | Higher | -7 | Lower | -14 | Lower |
| Comments (CLOC) | 3 | **15 ✅** | 2 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 5 | 6 | **4 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.5492 | **73.0357 ✅** | 64.7581 | -4.4865 | Lower | +3.7912 | Higher | +8.2776 | Higher |
| MI (SEI) | 55.3769 | **65.0410 ✅** | 55.5752 | -9.6641 | Lower | -0.1983 | Lower | +9.4658 | Higher |
| Cyclomatic (sum) | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 15 | **7 ✅** | 16 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 983.1622 | **788.7516 ✅** | 1189.5970 | +194.4106 | Higher | -206.4348 | Lower | -400.8454 | Lower |
| Halstead Effort | 21483.9153 | **13145.8603 ✅** | 37472.3059 | +8338.0550 | Higher | -15988.3906 | Lower | -24326.4456 | Lower |
| SLOC | 53 | **45 ✅** | 63 | +8 | Higher | -10 | Lower | -18 | Lower |
| PLOC | 39 | **27 ✅** | 41 | +12 | Higher | -2 | Lower | -14 | Lower |
| LLOC | 29 | **17 ✅** | 30 | +12 | Higher | -1 | Lower | -13 | Lower |
| Comments (CLOC) | 10 | 11 | **18 ✅** | -1 | Lower | -8 | Lower | -7 | Lower |
| Blank Lines | 9 | **7 ✅** | 10 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03701

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.1042 | 71.8293 | **74.5080 ✅** | -1.7251 | Lower | -4.4038 | Lower | -2.6787 | Lower |
| MI (SEI) | 55.2913 | **64.5070 ✅** | 49.6877 | -9.2157 | Lower | +5.6036 | Higher | +14.8193 | Higher |
| Cyclomatic (sum) | 8 | 10 | **7 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 11 | 13 | **9 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Halstead Volume | 955.0226 | **813.4553 ✅** | 928.1034 | +141.5673 | Higher | +26.9192 | Higher | -114.6481 | Lower |
| Halstead Effort | 25980.9556 | **21556.5657 ✅** | 27116.7595 | +4424.3899 | Higher | -1135.8038 | Lower | -5560.1937 | Lower |
| SLOC | 50 | 46 | **39 ✅** | +4 | Higher | +11 | Higher | +7 | Higher |
| PLOC | 34 | **29 ✅** | 30 | +5 | Higher | +4 | Higher | -1 | Lower |
| LLOC | 27 | **20 ✅** | 27 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 2 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 10 | **7 ✅** | 8 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03709

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.6658 | **75.9722 ✅** | 19.8733 | -38.3064 | Lower | +17.7926 | Higher | +56.0990 | Higher |
| MI (SEI) | -18.1023 | **72.2835 ✅** | -35.5217 | -90.3859 | Lower | +17.4193 | Higher | +107.8052 | Higher |
| Cyclomatic (sum) | 32 | **5 ✅** | 64 | +27 | Higher | -32 | Lower | -59 | Lower |
| Cognitive (sum) | 50 | **3 ✅** | 82 | +47 | Higher | -32 | Lower | -79 | Lower |
| Halstead Volume | 6266.1869 | **765.1033 ✅** | 9487.8492 | +5501.0836 | Higher | -3221.6623 | Lower | -8722.7459 | Lower |
| Halstead Effort | 648028.1663 | **14969.4130 ✅** | 1169527.5394 | +633058.7533 | Higher | -521499.3731 | Lower | -1154558.1264 | Lower |
| SLOC | 144 | **39 ✅** | 240 | +105 | Higher | -96 | Lower | -201 | Lower |
| PLOC | 122 | **20 ✅** | 202 | +102 | Higher | -80 | Lower | -182 | Lower |
| LLOC | 102 | **15 ✅** | 126 | +87 | Higher | -24 | Lower | -111 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 1 | -12 | Lower | -1 | Lower | +11 | Higher |
| Blank Lines | 22 | **8 ✅** | 38 | +14 | Higher | -16 | Lower | -30 | Lower |
| Functions | 4 | **1 ✅** | 16 | +3 | Higher | -12 | Lower | -15 | Lower |
| Closures | 4 | **1 ✅** | 3 | +3 | Higher | +1 | Higher | -2 | Lower |
| Exits | 3 | **0 ✅** | 15 | +3 | Higher | -12 | Lower | -15 | Lower |

## p03719

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.1929 ✅** | 83.0379 | 34.4296 | +5.1549 | Higher | +53.7632 | Higher | +48.6083 | Higher |
| MI (SEI) | **79.5743 ✅** | 72.7871 | -5.1223 | +6.7872 | Higher | +84.6967 | Higher | +77.9094 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 42 | -2 | Lower | -38 | Lower | -36 | Lower |
| Cognitive (sum) | **3 ✅** | 4 | 38 | -1 | Lower | -35 | Lower | -34 | Lower |
| Halstead Volume | **524.6170 ✅** | 591.4629 | 4728.3039 | -66.8459 | Lower | -4203.6870 | Lower | -4136.8411 | Lower |
| Halstead Effort | **6776.3028 ✅** | 9880.1182 | 440277.8404 | -3103.8154 | Lower | -433501.5376 | Lower | -430397.7222 | Lower |
| SLOC | **21 ✅** | 27 | 167 | -6 | Lower | -146 | Lower | -140 | Lower |
| PLOC | **14 ✅** | 19 | 146 | -5 | Lower | -132 | Lower | -127 | Lower |
| LLOC | **9 ✅** | 11 | 94 | -2 | Lower | -85 | Lower | -83 | Lower |
| Comments (CLOC) | 3 | 4 | **8 ✅** | -1 | Lower | -5 | Lower | -4 | Lower |
| Blank Lines | 4 | 4 | 19 | 0 | Same | -15 | Lower | -15 | Lower |
| Functions | 1 | 1 | 12 | 0 | Same | -11 | Lower | -11 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 1 | 13 | -1 | Lower | -13 | Lower | -12 | Lower |

## p03722

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8728 | 58.9144 | **60.2673 ✅** | +0.9584 | Higher | -0.3946 | Lower | -1.3530 | Lower |
| MI (SEI) | **35.4259 ✅** | 34.9527 | 12.4684 | +0.4733 | Higher | +22.9576 | Higher | +22.4843 | Higher |
| Cyclomatic (sum) | 12 | 14 | 12 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 17 | 21 | 17 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 2006.5142 | **1781.2294 ✅** | 2035.6786 | +225.2847 | Higher | -29.1645 | Lower | -254.4492 | Lower |
| Halstead Effort | 82449.4911 | **69604.9649 ✅** | 84388.1317 | +12844.5262 | Higher | -1938.6405 | Lower | -14783.1668 | Lower |
| SLOC | 70 | 75 | **68 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| PLOC | **54 ✅** | 59 | 59 | -5 | Lower | -5 | Lower | 0 | Same |
| LLOC | 38 | **24 ✅** | 40 | +14 | Higher | -2 | Lower | -16 | Lower |
| Comments (CLOC) | 7 | **8 ✅** | 0 | -1 | Lower | +7 | Higher | +8 | Higher |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p03736

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.8183 | **67.7581 ✅** | 58.5999 | -9.9398 | Lower | -0.7816 | Lower | +9.1581 | Higher |
| MI (SEI) | 8.8333 | **57.4201 ✅** | 9.9610 | -48.5867 | Lower | -1.1277 | Lower | +47.4591 | Higher |
| Cyclomatic (sum) | 11 | **9 ✅** | 11 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 2978.8315 | **1128.8126 ✅** | 2932.0917 | +1850.0189 | Higher | +46.7398 | Higher | -1803.2791 | Lower |
| Halstead Effort | 170665.2513 | **35698.6998 ✅** | 164197.1374 | +134966.5514 | Higher | +6468.1139 | Higher | -128498.4376 | Lower |
| SLOC | 71 | **54 ✅** | 68 | +17 | Higher | +3 | Higher | -14 | Lower |
| PLOC | 58 | **35 ✅** | 56 | +23 | Higher | +2 | Higher | -21 | Lower |
| LLOC | 51 | **30 ✅** | 51 | +21 | Higher | 0 | Same | -21 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 13 | **8 ✅** | 12 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.1523 | **72.6557 ✅** | 69.6659 | -7.5033 | Lower | -4.5136 | Lower | +2.9898 | Higher |
| MI (SEI) | 45.4590 | **70.6722 ✅** | 57.4540 | -25.2133 | Lower | -11.9951 | Lower | +13.2182 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 7 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 4 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | 1759.6285 | **994.2714 ✅** | 1507.9324 | +765.3571 | Higher | +251.6961 | Higher | -513.6610 | Lower |
| Halstead Effort | 80886.1507 | **21589.8934 ✅** | 56366.5136 | +59296.2573 | Higher | +24519.6371 | Higher | -34776.6203 | Lower |
| SLOC | 55 | **44 ✅** | 45 | +11 | Higher | +10 | Higher | -1 | Lower |
| PLOC | 38 | **23 ✅** | 29 | +15 | Higher | +9 | Higher | -6 | Lower |
| LLOC | 30 | **17 ✅** | 25 | +13 | Higher | +5 | Higher | -8 | Lower |
| Comments (CLOC) | 7 | **17 ✅** | 9 | -10 | Lower | -2 | Lower | +8 | Higher |
| Blank Lines | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03766

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.0425 | **70.1631 ✅** | 67.1185 | -0.1206 | Lower | +2.9240 | Higher | +3.0446 | Higher |
| MI (SEI) | 42.7466 | **61.3090 ✅** | 37.3402 | -18.5624 | Lower | +5.4065 | Higher | +23.9689 | Higher |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 1663.6666 | **1072.3327 ✅** | 1805.9714 | +591.3339 | Higher | -142.3048 | Lower | -733.6387 | Lower |
| Halstead Effort | 84015.1634 | **39497.5869 ✅** | 86453.5994 | +44517.5765 | Higher | -2438.4360 | Lower | -46956.0125 | Lower |
| SLOC | **42 ✅** | 48 | 49 | -6 | Lower | -7 | Lower | -1 | Lower |
| PLOC | 35 | **33 ✅** | 40 | +2 | Higher | -5 | Lower | -7 | Lower |
| LLOC | 28 | **22 ✅** | 30 | +6 | Higher | -2 | Lower | -8 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 7 | 7 | 9 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03797

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.1808 | **84.9752 ✅** | 83.2057 | -7.7944 | Lower | -6.0249 | Lower | +1.7695 | Higher |
| MI (SEI) | 58.2685 | **87.6210 ✅** | 60.0939 | -29.3525 | Lower | -1.8254 | Lower | +27.5271 | Higher |
| Cyclomatic (sum) | 7 | **4 ✅** | 5 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 5 | **1 ✅** | 4 | +4 | Higher | +1 | Higher | -3 | Lower |
| Halstead Volume | 777.6424 | **320.6267 ✅** | 760.7724 | +457.0157 | Higher | +16.8701 | Higher | -440.1456 | Lower |
| Halstead Effort | 18476.7844 | **4465.8725 ✅** | 14884.6771 | +14010.9118 | Higher | +3592.1073 | Higher | -10418.8046 | Lower |
| SLOC | 35 | 30 | **25 ✅** | +5 | Higher | +10 | Higher | +5 | Higher |
| PLOC | 25 | **14 ✅** | 20 | +11 | Higher | +5 | Higher | -6 | Lower |
| LLOC | 10 | **8 ✅** | 17 | +2 | Higher | -7 | Lower | -9 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 1 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 7 | 6 | **4 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03808

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.8630 | **62.1075 ✅** | 62.0776 | -1.2446 | Lower | -1.2146 | Lower | +0.0300 | Higher |
| MI (SEI) | 43.0877 | **44.6059 ✅** | 15.1818 | -1.5182 | Lower | +27.9059 | Higher | +29.4240 | Higher |
| Cyclomatic (sum) | 14 | 15 | **13 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 22 | **13 ✅** | 22 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 1823.6193 | **1251.3856 ✅** | 1833.5481 | +572.2338 | Higher | -9.9287 | Lower | -582.1625 | Lower |
| Halstead Effort | 108414.1701 | **29627.3991 ✅** | 135400.4715 | +78786.7710 | Higher | -26986.3014 | Lower | -105773.0724 | Lower |
| SLOC | 66 | 68 | **62 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| PLOC | 52 | **49 ✅** | 54 | +3 | Higher | -2 | Lower | -5 | Lower |
| LLOC | 44 | **22 ✅** | 45 | +22 | Higher | -1 | Lower | -23 | Lower |
| Comments (CLOC) | 11 | 11 | 0 | 0 | Same | +11 | Higher | +11 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 0 | 6 | 0 | -6 | Lower | 0 | Same | +6 | Higher |

## p03815

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.1140 | **88.1578 ✅** | 87.1140 | -1.0438 | Lower | 0 | Same | +1.0438 | Higher |
| MI (SEI) | 80.4875 | **90.7819 ✅** | 80.4875 | -10.2944 | Lower | 0 | Same | +10.2944 | Higher |
| Cyclomatic (sum) | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 486.2570 | **364.1791 ✅** | 486.2570 | +122.0779 | Higher | 0 | Same | -122.0779 | Lower |
| Halstead Effort | 8314.9948 | **4643.2835 ✅** | 8314.9948 | +3671.7113 | Higher | 0 | Same | -3671.7113 | Lower |
| SLOC | 23 | 24 | 23 | -1 | Lower | 0 | Same | +1 | Higher |
| PLOC | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| LLOC | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 4 | **8 ✅** | 4 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03817

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 83.6018 | **91.4282 ✅** | 68.0867 | -7.8263 | Lower | +15.5151 | Higher | +23.3414 | Higher |
| MI (SEI) | 76.3387 | **96.0800 ✅** | 42.3728 | -19.7413 | Lower | +33.9659 | Higher | +53.7073 | Higher |
| Cyclomatic (sum) | 5 | **2 ✅** | 13 | +3 | Higher | -8 | Lower | -11 | Lower |
| Cognitive (sum) | 4 | **0 ✅** | 9 | +4 | Higher | -5 | Lower | -9 | Lower |
| Halstead Volume | 554.6772 | **358.1520 ✅** | 1128.4003 | +196.5252 | Higher | -573.7232 | Lower | -770.2483 | Lower |
| Halstead Effort | 9464.1791 | **4382.0948 ✅** | 33739.1699 | +5082.0842 | Higher | -24274.9908 | Lower | -29357.0750 | Lower |
| SLOC | 27 | **20 ✅** | 50 | +7 | Higher | -23 | Lower | -30 | Lower |
| PLOC | 17 | **10 ✅** | 39 | +7 | Higher | -22 | Lower | -29 | Lower |
| LLOC | 8 | **7 ✅** | 10 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | 5 | **7 ✅** | 3 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 5 | 5 | 10 | 0 | Same | -5 | Lower | -5 | Lower |
| Functions | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 6 | 0 | Same | -6 | Lower | -6 | Lower |

## p03837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.6572 | **61.2881 ✅** | 57.2874 | -3.6309 | Lower | +0.3698 | Higher | +4.0008 | Higher |
| MI (SEI) | 34.6248 | **45.4334 ✅** | 34.6137 | -10.8086 | Lower | +0.0111 | Higher | +10.8197 | Higher |
| Cyclomatic (sum) | 14 | 14 | 16 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 19 | **17 ✅** | 19 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 2365.2588 | **1942.5369 ✅** | 2531.7408 | +422.7219 | Higher | -166.4821 | Lower | -589.2040 | Lower |
| Halstead Effort | 121145.5973 | **115815.3875 ✅** | 147516.0997 | +5330.2098 | Higher | -26370.5024 | Lower | -31700.7122 | Lower |
| SLOC | 74 | **63 ✅** | 72 | +11 | Higher | +2 | Higher | -9 | Lower |
| PLOC | 56 | **46 ✅** | 54 | +10 | Higher | +2 | Higher | -8 | Lower |
| LLOC | 46 | **29 ✅** | 44 | +17 | Higher | +2 | Higher | -15 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 9 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03838

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.3352 | **80.7037 ✅** | 65.4651 | -14.3685 | Lower | +0.8701 | Higher | +15.2386 | Higher |
| MI (SEI) | 42.3564 | **79.7723 ✅** | 45.1707 | -37.4159 | Lower | -2.8143 | Lower | +34.6016 | Higher |
| Cyclomatic (sum) | 15 | **5 ✅** | 15 | +10 | Higher | 0 | Same | -10 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 14 | +11 | Higher | 0 | Same | -11 | Lower |
| Halstead Volume | 1280.1538 | **629.7513 ✅** | 1426.1363 | +650.4025 | Higher | -145.9825 | Lower | -796.3850 | Lower |
| Halstead Effort | 36964.4423 | **12595.0260 ✅** | 45992.8957 | +24369.4162 | Higher | -9028.4534 | Lower | -33397.8697 | Lower |
| SLOC | 52 | **31 ✅** | 53 | +21 | Higher | -1 | Lower | -22 | Lower |
| PLOC | 43 | **19 ✅** | 44 | +24 | Higher | -1 | Lower | -25 | Lower |
| LLOC | 14 | **8 ✅** | 31 | +6 | Higher | -17 | Lower | -23 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 6 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03845

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.3056 | **76.3443 ✅** | 72.7406 | -8.0387 | Lower | -4.4350 | Lower | +3.6037 | Higher |
| MI (SEI) | 56.5056 | **79.1622 ✅** | 60.0352 | -22.6566 | Lower | -3.5296 | Lower | +19.1270 | Higher |
| Cyclomatic (sum) | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 1326.3062 | **802.9035 ✅** | 1259.3721 | +523.4027 | Higher | +66.9341 | Higher | -456.4686 | Lower |
| Halstead Effort | 36994.4695 | **18318.0948 ✅** | 27526.2765 | +18676.3746 | Higher | +9468.1930 | Higher | -9208.1816 | Lower |
| SLOC | 51 | **37 ✅** | 40 | +14 | Higher | +11 | Higher | -3 | Lower |
| PLOC | 34 | **21 ✅** | 26 | +13 | Higher | +8 | Higher | -5 | Lower |
| LLOC | 21 | **16 ✅** | 21 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | 11 | **18 ✅** | 7 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03865

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.4020 | 81.9830 | **92.6308 ✅** | +6.4191 | Higher | -4.2288 | Lower | -10.6478 | Lower |
| MI (SEI) | 85.2998 | 80.1404 | **94.7820 ✅** | +5.1593 | Higher | -9.4822 | Lower | -14.6415 | Lower |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 379.5669 | 492.4112 | **361.2099 ✅** | -112.8442 | Lower | +18.3571 | Higher | +131.2013 | Higher |
| Halstead Effort | **5018.7185 ✅** | 5088.2488 | 5056.9381 | -69.5303 | Lower | -38.2196 | Lower | +31.3107 | Higher |
| SLOC | 23 | 31 | **18 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| PLOC | 14 | 19 | **12 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| LLOC | 8 | 12 | **5 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |
| Comments (CLOC) | 5 | **9 ✅** | 5 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03899

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.0385 | **71.7221 ✅** | 62.7174 | -9.6836 | Lower | -0.6789 | Lower | +9.0047 | Higher |
| MI (SEI) | 34.2410 | **69.5290 ✅** | 25.9174 | -35.2881 | Lower | +8.3236 | Higher | +43.6116 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 11 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 12 | **10 ✅** | 11 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 2123.1312 | **1089.0677 ✅** | 2067.1828 | +1034.0635 | Higher | +55.9484 | Higher | -978.1151 | Lower |
| Halstead Effort | 86885.0621 | **40295.5044 ✅** | 88302.2270 | +46589.5577 | Higher | -1417.1649 | Lower | -48006.7225 | Lower |
| SLOC | 61 | **44 ✅** | 59 | +17 | Higher | +2 | Higher | -15 | Lower |
| PLOC | 48 | **30 ✅** | 48 | +18 | Higher | 0 | Same | -18 | Lower |
| LLOC | 37 | **24 ✅** | 35 | +13 | Higher | +2 | Higher | -11 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 1 | -13 | Lower | +3 | Higher | +16 | Higher |
| Blank Lines | 9 | **6 ✅** | 11 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03901

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.0978 | **77.5472 ✅** | 45.3471 | -3.4493 | Lower | +28.7508 | Higher | +32.2001 | Higher |
| MI (SEI) | 65.9053 | **74.3302 ✅** | 19.3788 | -8.4250 | Lower | +46.5265 | Higher | +54.9515 | Higher |
| Cyclomatic (sum) | **4 ✅** | 9 | 14 | -5 | Lower | -10 | Lower | -5 | Lower |
| Cognitive (sum) | **1 ✅** | 4 | 11 | -3 | Lower | -10 | Lower | -7 | Lower |
| Halstead Volume | 910.3489 | **726.0704 ✅** | 3462.2553 | +184.2784 | Higher | -2551.9065 | Lower | -2736.1849 | Lower |
| Halstead Effort | 18763.3021 | **14661.0378 ✅** | 147030.4438 | +4102.2643 | Higher | -128267.1417 | Lower | -132369.4060 | Lower |
| SLOC | 42 | **34 ✅** | 140 | +8 | Higher | -98 | Lower | -106 | Lower |
| PLOC | 25 | **19 ✅** | 104 | +6 | Higher | -79 | Lower | -85 | Lower |
| LLOC | 15 | **9 ✅** | 70 | +6 | Higher | -55 | Lower | -61 | Lower |
| Comments (CLOC) | 10 | 10 | **21 ✅** | 0 | Same | -11 | Lower | -11 | Lower |
| Blank Lines | 9 | **7 ✅** | 18 | +2 | Higher | -9 | Lower | -11 | Lower |
| Functions | 2 | **1 ✅** | 5 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p03914

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.7671 | 72.6257 | **77.5930 ✅** | +4.1414 | Higher | -0.8259 | Lower | -4.9673 | Lower |
| MI (SEI) | 61.5970 | **71.3516 ✅** | 50.7057 | -9.7546 | Lower | +10.8913 | Higher | +20.6459 | Higher |
| Cyclomatic (sum) | 5 | 9 | 5 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 6 | 11 | 6 | -5 | Lower | 0 | Same | +5 | Higher |
| Halstead Volume | 1216.1643 | **900.0687 ✅** | 1268.6181 | +316.0956 | Higher | -52.4538 | Lower | -368.5493 | Lower |
| Halstead Effort | 39498.9016 | **31288.1037 ✅** | 39863.8831 | +8210.7979 | Higher | -364.9815 | Lower | -8575.7793 | Lower |
| SLOC | 32 | 43 | **30 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| PLOC | **23 ✅** | 26 | 24 | -3 | Lower | -1 | Lower | +2 | Higher |
| LLOC | 21 | **17 ✅** | 22 | +4 | Higher | -1 | Lower | -5 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 1 | -13 | Lower | +3 | Higher | +16 | Higher |
| Blank Lines | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03929

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.0623 | **74.6335 ✅** | 66.1993 | -7.5712 | Lower | +0.8630 | Higher | +8.4343 | Higher |
| MI (SEI) | 52.7659 | **65.2295 ✅** | 45.7122 | -12.4637 | Lower | +7.0537 | Higher | +19.5174 | Higher |
| Cyclomatic (sum) | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 16 | **5 ✅** | 24 | +11 | Higher | -8 | Lower | -19 | Lower |
| Halstead Volume | 1236.0045 | **668.3510 ✅** | 1153.2892 | +567.6535 | Higher | +82.7154 | Higher | -484.9382 | Lower |
| Halstead Effort | 49059.8711 | **14946.7587 ✅** | 47625.5997 | +34113.1125 | Higher | +1434.2715 | Higher | -32678.8410 | Lower |
| SLOC | 51 | **43 ✅** | 55 | +8 | Higher | -4 | Lower | -12 | Lower |
| PLOC | 34 | **27 ✅** | 41 | +7 | Higher | -7 | Lower | -14 | Lower |
| LLOC | 28 | **17 ✅** | 31 | +11 | Higher | -3 | Lower | -14 | Lower |
| Comments (CLOC) | 9 | 9 | 6 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03932

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.4290 | **59.5905 ✅** | 37.4127 | -17.1615 | Lower | +5.0163 | Higher | +22.1778 | Higher |
| MI (SEI) | 16.3438 | **42.8707 ✅** | 12.8570 | -26.5269 | Lower | +3.4868 | Higher | +30.0136 | Higher |
| Cyclomatic (sum) | 22 | **12 ✅** | 26 | +10 | Higher | -4 | Lower | -14 | Lower |
| Cognitive (sum) | 19 | **8 ✅** | 26 | +11 | Higher | -7 | Lower | -18 | Lower |
| Halstead Volume | 4662.4892 | **1858.8287 ✅** | 5613.2461 | +2803.6605 | Higher | -950.7569 | Lower | -3754.4174 | Lower |
| Halstead Effort | 419416.8050 | **92941.4332 ✅** | 477927.8088 | +326475.3719 | Higher | -58511.0038 | Lower | -384986.3756 | Lower |
| SLOC | 136 | **73 ✅** | 165 | +63 | Higher | -29 | Lower | -92 | Lower |
| PLOC | 100 | **45 ✅** | 112 | +55 | Higher | -12 | Lower | -67 | Lower |
| LLOC | 65 | **26 ✅** | 73 | +39 | Higher | -8 | Lower | -47 | Lower |
| Comments (CLOC) | 21 | 14 | **33 ✅** | +7 | Higher | -12 | Lower | -19 | Lower |
| Blank Lines | 20 | **15 ✅** | 27 | +5 | Higher | -7 | Lower | -12 | Lower |
| Functions | 5 | **2 ✅** | 7 | +3 | Higher | -2 | Lower | -5 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |

## p03951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.6439 | **74.5884 ✅** | 73.3889 | -8.9445 | Lower | -7.7450 | Lower | +1.1994 | Higher |
| MI (SEI) | 51.6154 | **76.7933 ✅** | 48.4921 | -25.1779 | Lower | +3.1234 | Higher | +28.3013 | Higher |
| Cyclomatic (sum) | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 14 | +11 | Higher | 0 | Same | -11 | Lower |
| Halstead Volume | 1513.5248 | **585.1497 ✅** | 1142.3054 | +928.3751 | Higher | +371.2194 | Higher | -557.1557 | Lower |
| Halstead Effort | 58270.7063 | **16676.7670 ✅** | 44386.7258 | +41593.9393 | Higher | +13883.9804 | Higher | -27709.9589 | Lower |
| SLOC | 56 | 45 | **38 ✅** | +11 | Higher | +18 | Higher | +7 | Higher |
| PLOC | 41 | **24 ✅** | 33 | +17 | Higher | +8 | Higher | -9 | Lower |
| LLOC | 39 | **12 ✅** | 31 | +27 | Higher | +8 | Higher | -19 | Lower |
| Comments (CLOC) | 11 | **22 ✅** | 2 | -11 | Lower | +9 | Higher | +20 | Higher |
| Blank Lines | 9 | 8 | **5 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p03977

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.1542 | **78.4969 ✅** | 59.3961 | -9.3427 | Lower | +9.7580 | Higher | +19.1007 | Higher |
| MI (SEI) | 56.2690 | **79.1594 ✅** | 35.7966 | -22.8904 | Lower | +20.4724 | Higher | +43.3628 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 14 | +4 | Higher | -3 | Lower | -7 | Lower |
| Cognitive (sum) | 13 | **6 ✅** | 19 | +7 | Higher | -6 | Lower | -13 | Lower |
| Halstead Volume | 1140.1575 | **660.8156 ✅** | 1692.9565 | +479.3419 | Higher | -552.7990 | Lower | -1032.1409 | Lower |
| Halstead Effort | 37930.5966 | **10462.9141 ✅** | 81098.0779 | +27467.6825 | Higher | -43167.4812 | Lower | -70635.1637 | Lower |
| SLOC | 48 | **34 ✅** | 74 | +14 | Higher | -26 | Lower | -40 | Lower |
| PLOC | 31 | **23 ✅** | 56 | +8 | Higher | -25 | Lower | -33 | Lower |
| LLOC | 25 | **15 ✅** | 49 | +10 | Higher | -24 | Lower | -34 | Lower |
| Comments (CLOC) | 9 | **13 ✅** | 8 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 9 | **5 ✅** | 10 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03996

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.7480 | **73.8198 ✅** | 55.3426 | -17.0717 | Lower | +1.4054 | Higher | +18.4772 | Higher |
| MI (SEI) | 26.5922 | **73.7224 ✅** | 27.5591 | -47.1302 | Lower | -0.9669 | Lower | +46.1633 | Higher |
| Cyclomatic (sum) | 14 | **9 ✅** | 14 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 19 | **7 ✅** | 16 | +12 | Higher | +3 | Higher | -9 | Lower |
| Halstead Volume | 2298 | **829.8336 ✅** | 2785.4872 | +1468.1664 | Higher | -487.4872 | Lower | -1955.6535 | Lower |
| Halstead Effort | 101414.3684 | **24065.1746 ✅** | 136964.4414 | +77349.1938 | Higher | -35550.0730 | Lower | -112899.2669 | Lower |
| SLOC | 79 | **41 ✅** | 81 | +38 | Higher | -2 | Lower | -40 | Lower |
| PLOC | 64 | **24 ✅** | 65 | +40 | Higher | -1 | Lower | -41 | Lower |
| LLOC | 48 | **16 ✅** | 55 | +32 | Higher | -7 | Lower | -39 | Lower |
| Comments (CLOC) | 5 | **17 ✅** | 7 | -12 | Lower | -2 | Lower | +10 | Higher |
| Blank Lines | 10 | **8 ✅** | 11 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |

## p04005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.0887 | **77.4090 ✅** | 74.9200 | -0.3203 | Lower | +2.1687 | Higher | +2.4891 | Higher |
| MI (SEI) | 49.7522 | **73.0253 ✅** | 62.4086 | -23.2731 | Lower | -12.6564 | Lower | +10.6166 | Higher |
| Cyclomatic (sum) | 9 | **6 ✅** | 8 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 870.2800 | **712.5395 ✅** | 966.4985 | +157.7405 | Higher | -96.2184 | Lower | -253.9590 | Lower |
| Halstead Effort | 21556.1668 | **11671.3969 ✅** | 29342.8933 | +9884.7699 | Higher | -7786.7265 | Lower | -17671.4964 | Lower |
| SLOC | **33 ✅** | 36 | 37 | -3 | Lower | -4 | Lower | -1 | Lower |
| PLOC | 26 | **22 ✅** | 26 | +4 | Higher | 0 | Same | -4 | Lower |
| LLOC | 17 | **12 ✅** | 22 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 6 | -9 | Lower | -5 | Lower | +4 | Higher |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p04013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.1349 | **70.8451 ✅** | 64.3215 | -5.7101 | Lower | +0.8135 | Higher | +6.5236 | Higher |
| MI (SEI) | 19.8981 | **61.8831 ✅** | 29.7001 | -41.9850 | Lower | -9.8020 | Lower | +32.1830 | Higher |
| Cyclomatic (sum) | 16 | **10 ✅** | 16 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 18 | **8 ✅** | 18 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 1979.6813 | **982.9534 ✅** | 2170.8819 | +996.7279 | Higher | -191.2006 | Lower | -1187.9285 | Lower |
| Halstead Effort | 118610.2159 | **41705.3085 ✅** | 120371.6558 | +76904.9073 | Higher | -1761.4399 | Lower | -78666.3473 | Lower |
| SLOC | 48 | **46 ✅** | 49 | +2 | Higher | -1 | Lower | -3 | Lower |
| PLOC | 39 | **29 ✅** | 39 | +10 | Higher | 0 | Same | -10 | Lower |
| LLOC | 34 | **22 ✅** | 33 | +12 | Higher | +1 | Higher | -11 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 1 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p04020

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.4370 | 69.8629 | **80.2667 ✅** | +2.5741 | Higher | -7.8297 | Lower | -10.4038 | Lower |
| MI (SEI) | 57.5458 | 60.9778 | **64.9924 ✅** | -3.4321 | Lower | -7.4466 | Lower | -4.0146 | Lower |
| Cyclomatic (sum) | 8 | 9 | **6 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 8 | 7 | **5 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Halstead Volume | 1131.5807 | 1086.8956 | **899.8434 ✅** | +44.6850 | Higher | +231.7373 | Higher | +187.0522 | Higher |
| Halstead Effort | 38239.6224 | 33618.8061 | **31699.0287 ✅** | +4620.8163 | Higher | +6540.5937 | Higher | +1919.7774 | Higher |
| SLOC | 41 | 48 | **28 ✅** | -7 | Lower | +13 | Higher | +20 | Higher |
| PLOC | 28 | 30 | **23 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| LLOC | 24 | 23 | **22 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Comments (CLOC) | 6 | **12 ✅** | 3 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 7 | 9 | **4 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p04022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.9277 | **59.9601 ✅** | 4.3175 | -28.0324 | Lower | +27.6103 | Higher | +55.6426 | Higher |
| MI (SEI) | -25.7697 | **41.9150 ✅** | -52.8443 | -67.6847 | Lower | +27.0746 | Higher | +94.7593 | Higher |
| Cyclomatic (sum) | 38 | **17 ✅** | 88 | +21 | Higher | -50 | Lower | -71 | Lower |
| Cognitive (sum) | 86 | **27 ✅** | 101 | +59 | Higher | -15 | Lower | -74 | Lower |
| Halstead Volume | 5457.1731 | **1387.7925 ✅** | 12805.1971 | +4069.3806 | Higher | -7348.0240 | Lower | -11417.4046 | Lower |
| Halstead Effort | 500075.5024 | **43531.7865 ✅** | 1461242.1133 | +456543.7159 | Higher | -961166.6110 | Lower | -1417710.3268 | Lower |
| SLOC | 197 | **73 ✅** | 405 | +124 | Higher | -208 | Lower | -332 | Lower |
| PLOC | 174 | **50 ✅** | 352 | +124 | Higher | -178 | Lower | -302 | Lower |
| LLOC | 153 | **31 ✅** | 226 | +122 | Higher | -73 | Lower | -195 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 4 | -12 | Lower | -4 | Lower | +8 | Higher |
| Blank Lines | 23 | **12 ✅** | 51 | +11 | Higher | -28 | Lower | -39 | Lower |
| Functions | 3 | 3 | 36 | 0 | Same | -33 | Lower | -33 | Lower |
| Closures | **0 ✅** | 1 | 4 | -1 | Lower | -4 | Lower | -3 | Lower |
| Exits | 4 | 4 | 34 | 0 | Same | -30 | Lower | -30 | Lower |

## p04024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.4047 | **61.6973 ✅** | 40.7636 | -20.2927 | Lower | +0.6411 | Higher | +20.9338 | Higher |
| MI (SEI) | -5.5986 | **49.0778 ✅** | -13.1241 | -54.6764 | Lower | +7.5256 | Higher | +62.2019 | Higher |
| Cyclomatic (sum) | 34 | **12 ✅** | 37 | +22 | Higher | -3 | Lower | -25 | Lower |
| Cognitive (sum) | 45 | **10 ✅** | 51 | +35 | Higher | -6 | Lower | -41 | Lower |
| Halstead Volume | 4342.7973 | **1696.9459 ✅** | 4196.7011 | +2645.8513 | Higher | +146.0962 | Higher | -2499.7551 | Lower |
| Halstead Effort | 314852.8008 | **56989.1014 ✅** | 351282.9539 | +257863.6993 | Higher | -36430.1531 | Lower | -294293.8524 | Lower |
| SLOC | 125 | **66 ✅** | 126 | +59 | Higher | -1 | Lower | -60 | Lower |
| PLOC | 105 | **40 ✅** | 108 | +65 | Higher | -3 | Lower | -68 | Lower |
| LLOC | 79 | **29 ✅** | 81 | +50 | Higher | -2 | Lower | -52 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 0 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 19 | **12 ✅** | 18 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 8 | **3 ✅** | 9 | +5 | Higher | -1 | Lower | -6 | Lower |

## p04030

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.5365 | 79.4957 | **82.0925 ✅** | +0.0409 | Higher | -2.5559 | Lower | -2.5968 | Lower |
| MI (SEI) | 73.7927 | **77.3708 ✅** | 76.8151 | -3.5782 | Lower | -3.0224 | Lower | +0.5557 | Higher |
| Cyclomatic (sum) | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 565.5380 | **477.0233 ✅** | 482.1526 | +88.5147 | Higher | +83.3854 | Higher | -5.1293 | Lower |
| Halstead Effort | 9452.5643 | 8333.8780 | **8196.5943 ✅** | +1118.6863 | Higher | +1255.9701 | Higher | +137.2838 | Higher |
| SLOC | 34 | 36 | **31 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| PLOC | 23 | 22 | **21 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| LLOC | 17 | 16 | 16 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 8 | **11 ✅** | 7 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.8942 | **67.8585 ✅** | 59.3248 | -6.9642 | Lower | 1.5695 | Higher | 8.5337 | Higher |
| MI (SEI) | 33.1368 | **56.8447 ✅** | 28.1862 | -23.7079 | Lower | 4.9506 | Higher | 28.6585 | Higher |
| Cyclomatic (sum) | 16.5267 | **11.4033 ✅** | 18.0467 | 5.1233 | Higher | -1.5200 | Lower | -6.6433 | Lower |
| Cognitive (sum) | 21.1233 | **10.6567 ✅** | 23.0233 | 10.4667 | Higher | -1.9000 | Lower | -12.3667 | Lower |
| Halstead Volume | 2499.3176 | **1336.8020 ✅** | 2843.5731 | 1162.5156 | Higher | -344.2555 | Lower | -1506.7711 | Lower |
| Halstead Effort | 173306.0017 | **48504.8748 ✅** | 233144.5412 | 124801.1269 | Higher | -59838.5395 | Lower | -184639.6664 | Lower |
| SLOC | 75.3933 | **56.2500 ✅** | 84.0200 | 19.1433 | Higher | -8.6267 | Lower | -27.7700 | Lower |
| PLOC | 61.1767 | **38.4400 ✅** | 70.4000 | 22.7367 | Higher | -9.2233 | Lower | -31.9600 | Lower |
| LLOC | 43.8967 | **22.2733 ✅** | 53.3067 | 21.6233 | Higher | -9.4100 | Lower | -31.0333 | Lower |
| Comments (CLOC) | 4.7367 | **12.4400 ✅** | 3.8700 | -7.7033 | Lower | 0.8667 | Higher | 8.5700 | Higher |
| Blank Lines | 11.1000 | **8.9500 ✅** | 11.3067 | 2.1500 | Higher | -0.2067 | Lower | -2.3567 | Lower |
| Functions | 2.5500 | **2.3767 ✅** | 3.3400 | 0.1733 | Higher | -0.7900 | Lower | -0.9633 | Lower |
| Closures | 0.7433 | 0.6633 | **0.4667 ✅** | 0.0800 | Higher | 0.2767 | Higher | 0.1967 | Higher |
| Exits | **2.0700 ✅** | 2.1167 | 2.5500 | -0.0467 | Lower | -0.4800 | Lower | -0.4333 | Lower |

