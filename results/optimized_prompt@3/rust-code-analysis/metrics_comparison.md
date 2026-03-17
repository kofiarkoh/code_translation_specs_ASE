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
| MI (original) | **83.8997 ✅** | 80.0268 | 80.5028 | +3.8729 | Higher | +3.3969 | Higher | -0.4760 | Lower |
| MI (SEI) | **85.5737 ✅** | 78.2244 | 77.1225 | +7.3493 | Higher | +8.4512 | Higher | +1.1019 | Higher |
| Cyclomatic (sum) | **7 ✅** | 8 | 8 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | **11 ✅** | 21 | 12 | -10 | Lower | -1 | Lower | +9 | Higher |
| Halstead Volume | 539.2711 | **516.9925 ✅** | 634.8730 | +22.2786 | Higher | -95.6019 | Lower | -117.8805 | Lower |
| Halstead Effort | 8148.9849 | **6824.3010 ✅** | 14632.3105 | +1324.6839 | Higher | -6483.3256 | Lower | -7808.0095 | Lower |
| SLOC | **26 ✅** | 33 | 30 | -7 | Lower | -4 | Lower | +3 | Higher |
| PLOC | 21 | 21 | 25 | 0 | Same | -4 | Lower | -4 | Lower |
| LLOC | 12 | **11 ✅** | 17 | +1 | Higher | -5 | Lower | -6 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 8 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
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
| MI (original) | 86.6181 | 85.4813 | **93.3639 ✅** | +1.1368 | Higher | -6.7458 | Lower | -7.8826 | Lower |
| MI (SEI) | 79.9758 | **92.1105 ✅** | 90.4835 | -12.1347 | Lower | -10.5077 | Lower | +1.6270 | Higher |
| Cyclomatic (sum) | 6 | 5 | **4 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 6 | 6 | **3 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 489.6174 | **386.4273 ✅** | 452.7842 | +103.1901 | Higher | +36.8332 | Higher | -66.3569 | Lower |
| Halstead Effort | 7833.8782 | **4890.7207 ✅** | 7018.1550 | +2943.1575 | Higher | +815.7232 | Higher | -2127.4343 | Lower |
| SLOC | 23 | 27 | **16 ✅** | -4 | Lower | +7 | Higher | +11 | Higher |
| PLOC | 19 | 14 | **13 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| LLOC | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 4 | **13 ✅** | 3 | -9 | Lower | +1 | Higher | +10 | Higher |
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
| MI (original) | 61.2400 | 55.2872 | **66.7390 ✅** | +5.9528 | Higher | -5.4990 | Lower | -11.4518 | Lower |
| MI (SEI) | **41.3817 ✅** | 40.5604 | 21.9068 | +0.8212 | Higher | +19.4749 | Higher | +18.6536 | Higher |
| Cyclomatic (sum) | 17 | 15 | **13 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 18 | **14 ✅** | 15 | +4 | Higher | +3 | Higher | -1 | Lower |
| Halstead Volume | 2106.2621 | 2799.7872 | **2030.3809 ✅** | -693.5251 | Lower | +75.8812 | Higher | +769.4063 | Higher |
| Halstead Effort | 79755.4141 | 230124.4453 | **72452.5410 ✅** | -150369.0312 | Lower | +7302.8731 | Higher | +157671.9043 | Higher |
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
| MI (SEI) | 37.6874 | **46.7113 ✅** | 33.7800 | -9.0239 | Lower | +3.9074 | Higher | +12.9313 | Higher |
| Cyclomatic (sum) | **13 ✅** | 18 | 15 | -5 | Lower | -2 | Lower | +3 | Higher |
| Cognitive (sum) | 29 | **18 ✅** | 37 | +11 | Higher | -8 | Lower | -19 | Lower |
| Halstead Volume | **1355.3197 ✅** | 1577.1398 | 1523.1814 | -221.8201 | Lower | -167.8617 | Lower | +53.9584 | Higher |
| Halstead Effort | **49260.6576 ✅** | 70971.2905 | 67273.8461 | -21710.6329 | Lower | -18013.1885 | Lower | +3697.4444 | Higher |
| SLOC | 64 | **58 ✅** | 73 | +6 | Higher | -9 | Lower | -15 | Lower |
| PLOC | 48 | **44 ✅** | 55 | +4 | Higher | -7 | Lower | -11 | Lower |
| LLOC | 39 | **28 ✅** | 46 | +11 | Higher | -7 | Lower | -18 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 6 | -5 | Lower | -1 | Lower | +4 | Higher |
| Blank Lines | 11 | **8 ✅** | 12 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00118

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.3242 ✅** | 53.2632 | 49.7296 | +3.0610 | Higher | +6.5946 | Higher | +3.5335 | Higher |
| MI (SEI) | 27.3155 | **39.1882 ✅** | -1.1052 | -11.8727 | Lower | +28.4207 | Higher | +40.2933 | Higher |
| Cyclomatic (sum) | **20 ✅** | 21 | 28 | -1 | Lower | -8 | Lower | -7 | Lower |
| Cognitive (sum) | 26 | 26 | 34 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | **2445.4527 ✅** | 2529.6693 | 3408.8630 | -84.2165 | Lower | -963.4102 | Lower | -879.1937 | Lower |
| Halstead Effort | 198081.6692 | **120276.9484 ✅** | 381918.9047 | +77804.7208 | Higher | -183837.2355 | Lower | -261641.9563 | Lower |
| SLOC | **73 ✅** | 86 | 88 | -13 | Lower | -15 | Lower | -2 | Lower |
| PLOC | **55 ✅** | 58 | 73 | -3 | Lower | -18 | Lower | -15 | Lower |
| LLOC | 39 | **37 ✅** | 55 | +2 | Higher | -16 | Lower | -18 | Lower |
| Comments (CLOC) | 5 | **23 ✅** | 0 | -18 | Lower | +5 | Higher | +23 | Higher |
| Blank Lines | 13 | 13 | 15 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00124

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.0439 | 57.9157 | **68.1810 ✅** | +10.1282 | Higher | -0.1371 | Lower | -10.2653 | Lower |
| MI (SEI) | 44.2992 | 39.2042 | **50.8649 ✅** | +5.0950 | Higher | -6.5657 | Lower | -11.6608 | Lower |
| Cyclomatic (sum) | 8 | 16 | 8 | -8 | Lower | 0 | Same | +8 | Higher |
| Cognitive (sum) | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **1334.4159 ✅** | 1820.1279 | 1382.3949 | -485.7120 | Lower | -47.9790 | Lower | +437.7329 | Higher |
| Halstead Effort | **28228.0287 ✅** | 58921.3489 | 32150.5565 | -30693.3202 | Lower | -3922.5278 | Lower | +26770.7924 | Higher |
| SLOC | 51 | 77 | **50 ✅** | -26 | Lower | +1 | Higher | +27 | Higher |
| PLOC | **38 ✅** | 58 | 40 | -20 | Lower | -2 | Lower | +18 | Higher |
| LLOC | **26 ✅** | 29 | 28 | -3 | Lower | -2 | Lower | +1 | Higher |
| Comments (CLOC) | 4 | **13 ✅** | 7 | -9 | Lower | -3 | Lower | +6 | Higher |
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
| MI (original) | 51.2510 | **64.4238 ✅** | 53.0393 | -13.1728 | Lower | -1.7883 | Lower | +11.3845 | Higher |
| MI (SEI) | 8.5966 | **53.3344 ✅** | 14.6618 | -44.7378 | Lower | -6.0652 | Lower | +38.6726 | Higher |
| Cyclomatic (sum) | 21 | **11 ✅** | 19 | +10 | Higher | +2 | Higher | -8 | Lower |
| Cognitive (sum) | 31 | **15 ✅** | 28 | +16 | Higher | +3 | Higher | -13 | Lower |
| Halstead Volume | 3467.4979 | **1751.7520 ✅** | 3222.7063 | +1715.7459 | Higher | +244.7916 | Higher | -1470.9543 | Lower |
| Halstead Effort | 289705.2168 | **65999.8324 ✅** | 233496.3132 | +223705.3843 | Higher | +56208.9036 | Higher | -167496.4807 | Lower |
| SLOC | 88 | **56 ✅** | 83 | +32 | Higher | +5 | Higher | -27 | Lower |
| PLOC | 77 | **42 ✅** | 73 | +35 | Higher | +4 | Higher | -31 | Lower |
| LLOC | 63 | **35 ✅** | 58 | +28 | Higher | +5 | Higher | -23 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 2 | -13 | Lower | -1 | Lower | +12 | Higher |
| Blank Lines | 11 | **8 ✅** | 10 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p00156

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.2932 | **52.6409 ✅** | 49.5598 | -2.3477 | Lower | +0.7334 | Higher | +3.0811 | Higher |
| MI (SEI) | 22.1576 | **29.9322 ✅** | 6.4730 | -7.7746 | Lower | +15.6846 | Higher | +23.4591 | Higher |
| Cyclomatic (sum) | **22 ✅** | 23 | 25 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 57 | **38 ✅** | 56 | +19 | Higher | +1 | Higher | -18 | Lower |
| Halstead Volume | 3247.5441 | **2609.8750 ✅** | 3749.8771 | +637.6691 | Higher | -502.3330 | Lower | -1140.0021 | Lower |
| Halstead Effort | 203606.8960 | **107902.0208 ✅** | 333210.2340 | +95704.8751 | Higher | -129603.3380 | Lower | -225308.2132 | Lower |
| SLOC | 94 | **86 ✅** | 90 | +8 | Higher | +4 | Higher | -4 | Lower |
| PLOC | 77 | **64 ✅** | 79 | +13 | Higher | -2 | Lower | -15 | Lower |
| LLOC | 72 | **50 ✅** | 77 | +22 | Higher | -5 | Lower | -27 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 1 | -3 | Lower | +8 | Higher | +11 | Higher |
| Blank Lines | 15 | 14 | **11 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00167

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.2197 | 68.8283 | **71.2999 ✅** | +1.3914 | Higher | -1.0802 | Lower | -2.4716 | Lower |
| MI (SEI) | 45.5951 | **54.1134 ✅** | 48.8319 | -8.5183 | Lower | -3.2368 | Lower | +5.2815 | Higher |
| Cyclomatic (sum) | **9 ✅** | 12 | 13 | -3 | Lower | -4 | Lower | -1 | Lower |
| Cognitive (sum) | **11 ✅** | 13 | 20 | -2 | Lower | -9 | Lower | -7 | Lower |
| Halstead Volume | **1083.6204 ✅** | 1089.1057 | 1128.7712 | -5.4853 | Lower | -45.1508 | Lower | -39.6656 | Lower |
| Halstead Effort | **28522.4367 ✅** | 41045.6698 | 35426.0512 | -12523.2331 | Lower | -6903.6145 | Lower | +5619.6186 | Higher |
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
| MI (original) | 31.5171 | **56.1771 ✅** | 31.3947 | -24.6600 | Lower | +0.1224 | Higher | +24.7824 | Higher |
| MI (SEI) | -16.8419 | **32.0057 ✅** | -19.3896 | -48.8476 | Lower | +2.5476 | Higher | +51.3952 | Higher |
| Cyclomatic (sum) | 37 | **18 ✅** | 33 | +19 | Higher | +4 | Higher | -15 | Lower |
| Cognitive (sum) | 72 | **15 ✅** | 60 | +57 | Higher | +12 | Higher | -45 | Lower |
| Halstead Volume | 6687.3885 | **2066.2044 ✅** | 6869.7071 | +4621.1841 | Higher | -182.3186 | Lower | -4803.5027 | Lower |
| Halstead Effort | 595177.5797 | **56294.6781 ✅** | 677811.0995 | +538882.9016 | Higher | -82633.5198 | Lower | -621516.4214 | Lower |
| SLOC | 192 | **80 ✅** | 203 | +112 | Higher | -11 | Lower | -123 | Lower |
| PLOC | 169 | **60 ✅** | 179 | +109 | Higher | -10 | Lower | -119 | Lower |
| LLOC | 99 | **26 ✅** | 136 | +73 | Higher | -37 | Lower | -110 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 2 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 21 | **15 ✅** | 22 | +6 | Higher | -1 | Lower | -7 | Lower |
| Functions | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |

## p00203

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.0362 | **66.0949 ✅** | 49.8934 | -17.0587 | Lower | -0.8572 | Lower | +16.2015 | Higher |
| MI (SEI) | -2.2074 | **53.3967 ✅** | -0.6652 | -55.6041 | Lower | -1.5422 | Lower | +54.0619 | Higher |
| Cyclomatic (sum) | 27 | **11 ✅** | 30 | +16 | Higher | -3 | Lower | -19 | Lower |
| Cognitive (sum) | 45 | **6 ✅** | 45 | +39 | Higher | 0 | Same | -39 | Lower |
| Halstead Volume | 3104.6060 | **1202.1414 ✅** | 3133.0958 | +1902.4647 | Higher | -28.4898 | Lower | -1930.9544 | Lower |
| Halstead Effort | 239306.3906 | **38468.5238 ✅** | 231401.5042 | +200837.8668 | Higher | +7904.8864 | Higher | -192932.9804 | Lower |
| SLOC | 96 | **57 ✅** | 87 | +39 | Higher | +9 | Higher | -30 | Lower |
| PLOC | 81 | **38 ✅** | 78 | +43 | Higher | +3 | Higher | -40 | Lower |
| LLOC | 50 | **23 ✅** | 49 | +27 | Higher | +1 | Higher | -26 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 0 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 15 | 11 | **9 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Functions | 5 | **1 ✅** | 6 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
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
| MI (original) | 44.9388 | **50.3972 ✅** | 33.9573 | -5.4584 | Lower | +10.9815 | Higher | +16.4398 | Higher |
| MI (SEI) | 4.4656 | **20.8498 ✅** | -14.9273 | -16.3841 | Lower | +19.3929 | Higher | +35.7771 | Higher |
| Cyclomatic (sum) | 29 | **19 ✅** | 35 | +10 | Higher | -6 | Lower | -16 | Lower |
| Cognitive (sum) | 30 | **17 ✅** | 42 | +13 | Higher | -12 | Lower | -25 | Lower |
| Halstead Volume | 3560.1507 | **2574.9670 ✅** | 5992.3159 | +985.1837 | Higher | -2432.1652 | Lower | -3417.3489 | Lower |
| Halstead Effort | 141159.9742 | **98572.9561 ✅** | 470396.7962 | +42587.0181 | Higher | -329236.8220 | Lower | -371823.8401 | Lower |
| SLOC | 115 | **105 ✅** | 176 | +10 | Higher | -61 | Lower | -71 | Lower |
| PLOC | 92 | **84 ✅** | 148 | +8 | Higher | -56 | Lower | -64 | Lower |
| LLOC | 44 | **33 ✅** | 94 | +11 | Higher | -50 | Lower | -61 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 2 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 22 | **18 ✅** | 26 | +4 | Higher | -4 | Lower | -8 | Lower |
| Functions | **5 ✅** | 6 | 9 | -1 | Lower | -4 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 5 | 9 | 0 | Same | -4 | Lower | -4 | Lower |

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
| MI (original) | **50.6653 ✅** | 44.2117 | 47.6407 | +6.4536 | Higher | +3.0247 | Higher | -3.4289 | Lower |
| MI (SEI) | 10.7519 | **21.2510 ✅** | 10.7190 | -10.4991 | Lower | +0.0329 | Higher | +10.5320 | Higher |
| Cyclomatic (sum) | **20 ✅** | 43 | 24 | -23 | Lower | -4 | Lower | +19 | Higher |
| Cognitive (sum) | 39 | **23 ✅** | 50 | +16 | Higher | -11 | Lower | -27 | Lower |
| Halstead Volume | 3531.7718 | **3203.0689 ✅** | 4082.7820 | +328.7029 | Higher | -551.0102 | Lower | -879.7131 | Lower |
| Halstead Effort | 255241.1512 | **120410.7524 ✅** | 334017.7899 | +134830.3988 | Higher | -78776.6387 | Lower | -213607.0375 | Lower |
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
| MI (original) | 39.6953 | **52.4531 ✅** | 23.7794 | -12.7579 | Lower | +15.9159 | Higher | +28.6738 | Higher |
| MI (SEI) | -5.7677 | **26.4178 ✅** | -29.7209 | -32.1855 | Lower | +23.9532 | Higher | +56.1387 | Higher |
| Cyclomatic (sum) | 32 | **15 ✅** | 46 | +17 | Higher | -14 | Lower | -31 | Lower |
| Cognitive (sum) | 53 | **18 ✅** | 59 | +35 | Higher | -6 | Lower | -41 | Lower |
| Halstead Volume | 5307.4798 | **2649.2449 ✅** | 9426.2398 | +2658.2349 | Higher | -4118.7600 | Lower | -6776.9949 | Lower |
| Halstead Effort | 367542.9745 | **92567.7340 ✅** | 935809.8308 | +274975.2405 | Higher | -568266.8563 | Lower | -843242.0968 | Lower |
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
| MI (original) | 81.6541 | 73.4930 | **90.9001 ✅** | +8.1612 | Higher | -9.2460 | Lower | -17.4071 | Lower |
| MI (SEI) | 74.9992 | 60.7998 | **84.6176 ✅** | +14.1993 | Higher | -9.6185 | Lower | -23.8178 | Lower |
| Cyclomatic (sum) | 5 | 9 | **3 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Cognitive (sum) | 2 | 7 | **1 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Halstead Volume | 645.6916 | 819.7476 | **445 ✅** | -174.0560 | Lower | +200.6916 | Higher | +374.7476 | Higher |
| Halstead Effort | 11709.3694 | 17766.4536 | **4414.8684 ✅** | -6057.0842 | Lower | +7294.5010 | Higher | +13351.5852 | Higher |
| SLOC | 29 | 42 | **19 ✅** | -13 | Lower | +10 | Higher | +23 | Higher |
| PLOC | 19 | 30 | **12 ✅** | -11 | Lower | +7 | Higher | +18 | Higher |
| LLOC | 11 | 16 | **8 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Comments (CLOC) | 6 | **7 ✅** | 3 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00283

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.1857 | 59.0324 | **59.6432 ✅** | +0.1533 | Higher | -0.4575 | Lower | -0.6108 | Lower |
| MI (SEI) | 20.5885 | **38.3666 ✅** | 11.8734 | -17.7781 | Lower | +8.7151 | Higher | +26.4932 | Higher |
| Cyclomatic (sum) | **14 ✅** | 18 | 15 | -4 | Lower | -1 | Lower | +3 | Higher |
| Cognitive (sum) | 21 | 22 | **17 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| Halstead Volume | 2517.7712 | **2172.6284 ✅** | 2550 | +345.1428 | Higher | -32.2288 | Lower | -377.3716 | Lower |
| Halstead Effort | 147785.5413 | **75718.4117 ✅** | 147900 | +72067.1296 | Higher | -114.4587 | Lower | -72181.5883 | Lower |
| SLOC | 66 | 66 | **63 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| PLOC | 56 | **48 ✅** | 54 | +8 | Higher | +2 | Higher | -6 | Lower |
| LLOC | 43 | **31 ✅** | 43 | +12 | Higher | 0 | Same | -12 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 0 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p00288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 23.9859 | **45.4221 ✅** | 16.9871 | -21.4362 | Lower | +6.9988 | Higher | +28.4350 | Higher |
| MI (SEI) | -25.5291 | **27.9416 ✅** | -39.4596 | -53.4707 | Lower | +13.9305 | Higher | +67.4013 | Higher |
| Cyclomatic (sum) | 44 | **20 ✅** | 54 | +24 | Higher | -10 | Lower | -34 | Lower |
| Cognitive (sum) | 78 | **38 ✅** | 89 | +40 | Higher | -11 | Lower | -51 | Lower |
| Halstead Volume | 10420.1573 | **3070.7254 ✅** | 11945.0476 | +7349.4319 | Higher | -1524.8904 | Lower | -8874.3222 | Lower |
| Halstead Effort | 1227995.7493 | **155040.9257 ✅** | 1365547.6813 | +1072954.8237 | Higher | -137551.9320 | Lower | -1210506.7557 | Lower |
| SLOC | 240 | **133 ✅** | 307 | +107 | Higher | -67 | Lower | -174 | Lower |
| PLOC | 219 | **98 ✅** | 276 | +121 | Higher | -57 | Lower | -178 | Lower |
| LLOC | 166 | **59 ✅** | 201 | +107 | Higher | -35 | Lower | -142 | Lower |
| Comments (CLOC) | 5 | **36 ✅** | 2 | -31 | Lower | +3 | Higher | +34 | Higher |
| Blank Lines | 21 | **18 ✅** | 29 | +3 | Higher | -8 | Lower | -11 | Lower |
| Functions | 8 | **4 ✅** | 13 | +4 | Higher | -5 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **3 ✅** | 8 | +1 | Higher | -4 | Lower | -5 | Lower |

## p00292

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.4064 ✅** | 71.0474 | 80.6399 | +10.3591 | Higher | +0.7665 | Higher | -9.5925 | Lower |
| MI (SEI) | 74.5400 | 61.8177 | **81.5758 ✅** | +12.7223 | Higher | -7.0358 | Lower | -19.7582 | Lower |
| Cyclomatic (sum) | 4 | 6 | **3 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 707.8164 | 870.2800 | **630.9018 ✅** | -162.4636 | Lower | +76.9146 | Higher | +239.3782 | Higher |
| Halstead Effort | 12167.7016 | 15304.9246 | **8766.2146 ✅** | -3137.2230 | Lower | +3401.4870 | Higher | +6538.7100 | Higher |
| SLOC | **29 ✅** | 50 | 32 | -21 | Lower | -3 | Lower | +18 | Higher |
| PLOC | 18 | 29 | **15 ✅** | -11 | Lower | +3 | Higher | +14 | Higher |
| LLOC | 13 | 13 | **11 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 6 | 12 | 12 | -6 | Lower | -6 | Lower | 0 | Same |
| Blank Lines | 6 | 9 | **5 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00315

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.8361 | **58.1546 ✅** | 52.2240 | -4.3186 | Lower | +1.6121 | Higher | +5.9307 | Higher |
| MI (SEI) | 33.0933 | **44.8219 ✅** | 19.8229 | -11.7286 | Lower | +13.2703 | Higher | +24.9990 | Higher |
| Cyclomatic (sum) | **17 ✅** | 18 | 18 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | **16 ✅** | 24 | 20 | -8 | Lower | -4 | Lower | +4 | Higher |
| Halstead Volume | 3523.0852 | **1591.2152 ✅** | 3658.5652 | +1931.8699 | Higher | -135.4800 | Lower | -2067.3499 | Lower |
| Halstead Effort | 248964.6854 | **59028.9528 ✅** | 245733.6271 | +189935.7326 | Higher | +3231.0583 | Higher | -186704.6743 | Lower |
| SLOC | 79 | **77 ✅** | 85 | +2 | Higher | -6 | Lower | -8 | Lower |
| PLOC | 66 | **50 ✅** | 70 | +16 | Higher | -4 | Lower | -20 | Lower |
| LLOC | 45 | **31 ✅** | 47 | +14 | Higher | -2 | Lower | -16 | Lower |
| Comments (CLOC) | 13 | **19 ✅** | 5 | -6 | Lower | +8 | Higher | +14 | Higher |
| Blank Lines | 13 | 13 | 15 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p00317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.7773 | **57.0152 ✅** | 32.7384 | -12.2379 | Lower | +12.0389 | Higher | +24.2768 | Higher |
| MI (SEI) | -8.3517 | **38.3081 ✅** | -12.8249 | -46.6598 | Lower | +4.4732 | Higher | +51.1330 | Higher |
| Cyclomatic (sum) | 27 | **17 ✅** | 40 | +10 | Higher | -13 | Lower | -23 | Lower |
| Cognitive (sum) | 56 | **21 ✅** | 71 | +35 | Higher | -15 | Lower | -50 | Lower |
| Halstead Volume | 3905.3476 | **1768.3963 ✅** | 6072.3036 | +2136.9513 | Higher | -2166.9560 | Lower | -4303.9073 | Lower |
| Halstead Effort | 241756.0371 | **56195.7048 ✅** | 575182.0945 | +185560.3323 | Higher | -333426.0574 | Lower | -518986.3897 | Lower |
| SLOC | 116 | **81 ✅** | 176 | +35 | Higher | -60 | Lower | -95 | Lower |
| PLOC | 104 | **59 ✅** | 155 | +45 | Higher | -51 | Lower | -96 | Lower |
| LLOC | 67 | **33 ✅** | 117 | +34 | Higher | -50 | Lower | -84 | Lower |
| Comments (CLOC) | 0 | **14 ✅** | 4 | -14 | Lower | -4 | Lower | +10 | Higher |
| Blank Lines | 12 | **11 ✅** | 17 | +1 | Higher | -5 | Lower | -6 | Lower |
| Functions | **2 ✅** | 3 | 8 | -1 | Lower | -6 | Lower | -5 | Lower |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | **3 ✅** | 6 | 11 | -3 | Lower | -8 | Lower | -5 | Lower |

## p00320

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.2075 ✅** | 52.0971 | 53.0339 | +3.1104 | Higher | +2.1735 | Higher | -0.9369 | Lower |
| MI (SEI) | 14.3015 | **29.4669 ✅** | 2.5419 | -15.1654 | Lower | +11.7596 | Higher | +26.9250 | Higher |
| Cyclomatic (sum) | **16 ✅** | 22 | 17 | -6 | Lower | -1 | Lower | +5 | Higher |
| Cognitive (sum) | **17 ✅** | 24 | 20 | -7 | Lower | -3 | Lower | +4 | Higher |
| Halstead Volume | 2943.2809 | **2628.6766 ✅** | 3043.7314 | +314.6043 | Higher | -100.4505 | Lower | -415.0548 | Lower |
| Halstead Effort | 231415.4604 | **97589.6177 ✅** | 259731.7479 | +133825.8427 | Higher | -28316.2875 | Lower | -162142.1302 | Lower |
| SLOC | **78 ✅** | 90 | 87 | -12 | Lower | -9 | Lower | +3 | Higher |
| PLOC | 70 | 70 | 79 | 0 | Same | -9 | Lower | -9 | Lower |
| LLOC | 44 | **38 ✅** | 51 | +6 | Higher | -7 | Lower | -13 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 0 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00329

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.3507 | **60.2015 ✅** | 49.8474 | -13.8509 | Lower | -3.4968 | Lower | +10.3541 | Higher |
| MI (SEI) | 7.9194 | **51.4609 ✅** | 9.5312 | -43.5415 | Lower | -1.6118 | Lower | +41.9297 | Higher |
| Cyclomatic (sum) | 24 | **15 ✅** | 23 | +9 | Higher | +1 | Higher | -8 | Lower |
| Cognitive (sum) | 27 | **15 ✅** | 28 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 3478.6814 | **1890.6918 ✅** | 2972.9952 | +1587.9896 | Higher | +505.6862 | Higher | -1082.3034 | Lower |
| Halstead Effort | 218959.2760 | **90193.0025 ✅** | 173726.3311 | +128766.2734 | Higher | +45232.9449 | Higher | -83533.3286 | Lower |
| SLOC | 114 | **67 ✅** | 98 | +47 | Higher | +16 | Higher | -31 | Lower |
| PLOC | 98 | **49 ✅** | 87 | +49 | Higher | +11 | Higher | -38 | Lower |
| LLOC | 60 | **27 ✅** | 55 | +33 | Higher | +5 | Higher | -28 | Lower |
| Comments (CLOC) | 4 | **22 ✅** | 2 | -18 | Lower | +2 | Higher | +20 | Higher |
| Blank Lines | 14 | 9 | 9 | +5 | Higher | +5 | Higher | 0 | Same |
| Functions | 7 | **3 ✅** | 6 | +4 | Higher | +1 | Higher | -3 | Lower |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
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
| MI (original) | 77.9408 | **90.6799 ✅** | 78.8192 | -12.7391 | Lower | -0.8784 | Lower | +11.8607 | Higher |
| MI (SEI) | 56.0671 | **98.6609 ✅** | 61.7967 | -42.5939 | Lower | -5.7296 | Lower | +36.8642 | Higher |
| Cyclomatic (sum) | 7 | **3 ✅** | 6 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 9 | **0 ✅** | 9 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 807.0714 | **294.0315 ✅** | 865.6692 | +513.0399 | Higher | -58.5978 | Lower | -571.6377 | Lower |
| Halstead Effort | 23606.8395 | **3849.1400 ✅** | 22507.3989 | +19757.6995 | Higher | +1099.4406 | Higher | -18658.2589 | Lower |
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
| MI (original) | 61.8542 | **65.9641 ✅** | 58.8741 | -4.1099 | Lower | +2.9800 | Higher | +7.0900 | Higher |
| MI (SEI) | 38.5136 | **45.9029 ✅** | 11.4766 | -7.3893 | Lower | +27.0371 | Higher | +34.4264 | Higher |
| Cyclomatic (sum) | 16 | **13 ✅** | 22 | +3 | Higher | -6 | Lower | -9 | Lower |
| Cognitive (sum) | 29 | **11 ✅** | 26 | +18 | Higher | +3 | Higher | -15 | Lower |
| Halstead Volume | 1763.2976 | **1502.0256 ✅** | 1968 | +261.2719 | Higher | -204.7024 | Lower | -465.9744 | Lower |
| Halstead Effort | 57510.6287 | **46247.8539 ✅** | 91315.2000 | +11262.7748 | Higher | -33804.5713 | Lower | -45067.3461 | Lower |
| SLOC | 61 | **52 ✅** | 65 | +9 | Higher | -4 | Lower | -13 | Lower |
| PLOC | 46 | **38 ✅** | 58 | +8 | Higher | -12 | Lower | -20 | Lower |
| LLOC | 33 | **22 ✅** | 38 | +11 | Higher | -5 | Lower | -16 | Lower |
| Comments (CLOC) | 6 | 6 | 0 | 0 | Same | +6 | Higher | +6 | Higher |
| Blank Lines | 10 | 9 | **7 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.6426 | 69.0296 | **82.7071 ✅** | +8.6130 | Higher | -5.0645 | Lower | -13.6775 | Lower |
| MI (SEI) | 74.3132 | 62.1355 | **76.5183 ✅** | +12.1776 | Higher | -2.2051 | Lower | -14.3827 | Lower |
| Cyclomatic (sum) | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 6 | 13 | **5 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Halstead Volume | 625.5088 | 780.6987 | **527.3324 ✅** | -155.1899 | Lower | +98.1765 | Higher | +253.3664 | Higher |
| Halstead Effort | 10851.2182 | 16264.5567 | **6403.3214 ✅** | -5413.3385 | Lower | +4447.8968 | Higher | +9861.2353 | Higher |
| SLOC | 37 | 57 | **29 ✅** | -20 | Lower | +8 | Higher | +28 | Higher |
| PLOC | 20 | 33 | **18 ✅** | -13 | Lower | +2 | Higher | +15 | Higher |
| LLOC | 10 | 16 | **9 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Comments (CLOC) | 11 | **17 ✅** | 6 | -6 | Lower | +5 | Higher | +11 | Higher |
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
| MI (original) | 73.8708 | 73.2423 | **73.8866 ✅** | +0.6285 | Higher | -0.0158 | Lower | -0.6443 | Lower |
| MI (SEI) | 62.5029 | 62.2724 | **66.6079 ✅** | +0.2305 | Higher | -4.1050 | Lower | -4.3355 | Lower |
| Cyclomatic (sum) | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 19 | **13 ✅** | 20 | +6 | Higher | -1 | Lower | -7 | Lower |
| Halstead Volume | 878.9685 | **823.0107 ✅** | 1032.4684 | +55.9578 | Higher | -153.4999 | Lower | -209.4577 | Lower |
| Halstead Effort | 27423.8169 | **25474.1417 ✅** | 37168.8618 | +1949.6752 | Higher | -9745.0448 | Lower | -11694.7201 | Lower |
| SLOC | 39 | 42 | **37 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| PLOC | 29 | 31 | **28 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| LLOC | **18 ✅** | 19 | 23 | -1 | Lower | -5 | Lower | -4 | Lower |
| Comments (CLOC) | 7 | 8 | **9 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
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
| MI (original) | 49.2105 | **52.3902 ✅** | 34.7744 | -3.1798 | Lower | +14.4361 | Higher | +17.6158 | Higher |
| MI (SEI) | 13.4684 | **31.9040 ✅** | -9.6474 | -18.4356 | Lower | +23.1158 | Higher | +41.5514 | Higher |
| Cyclomatic (sum) | 25 | **17 ✅** | 38 | +8 | Higher | -13 | Lower | -21 | Lower |
| Cognitive (sum) | 38 | **15 ✅** | 51 | +23 | Higher | -13 | Lower | -36 | Lower |
| Halstead Volume | **3388.7473 ✅** | 3703.6708 | 5695.7248 | -314.9235 | Lower | -2306.9775 | Lower | -1992.0539 | Lower |
| Halstead Effort | **137970.4264 ✅** | 154913.1556 | 495160.5900 | -16942.7291 | Lower | -357190.1635 | Lower | -340247.4344 | Lower |
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
| MI (original) | 62.4405 | **63.3221 ✅** | 54.9698 | -0.8816 | Lower | +7.4707 | Higher | +8.3523 | Higher |
| MI (SEI) | 40.7767 | **50.0900 ✅** | 6.0475 | -9.3133 | Lower | +34.7292 | Higher | +44.0425 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 24 | +1 | Higher | -9 | Lower | -10 | Lower |
| Cognitive (sum) | 30 | 27 | **24 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| Halstead Volume | 1565.1781 | **1529.3548 ✅** | 2345.0356 | +35.8234 | Higher | -779.8574 | Lower | -815.6808 | Lower |
| Halstead Effort | 56617.3090 | **43658.2992 ✅** | 116470.0998 | +12959.0098 | Higher | -59852.7908 | Lower | -72811.8006 | Lower |
| SLOC | 62 | **60 ✅** | 76 | +2 | Higher | -14 | Lower | -16 | Lower |
| PLOC | 46 | **42 ✅** | 67 | +4 | Higher | -21 | Lower | -25 | Lower |
| LLOC | 38 | **33 ✅** | 52 | +5 | Higher | -14 | Lower | -19 | Lower |
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
| MI (original) | 57.5649 | **65.5856 ✅** | 58.4454 | -8.0206 | Lower | -0.8805 | Lower | +7.1402 | Higher |
| MI (SEI) | 27.4012 | **58.6240 ✅** | 29.1967 | -31.2228 | Lower | -1.7955 | Lower | +29.4273 | Higher |
| Cyclomatic (sum) | 18 | **12 ✅** | 18 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 26 | **23 ✅** | 26 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 2508.4236 | **1340.3910 ✅** | 2550.7343 | +1168.0326 | Higher | -42.3108 | Lower | -1210.3433 | Lower |
| Halstead Effort | 159912.0019 | **68908.2828 ✅** | 150941.4268 | +91003.7191 | Higher | +8970.5752 | Higher | -82033.1440 | Lower |
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
| MI (original) | 62.8526 | **79.0950 ✅** | 63.9351 | -16.2424 | Lower | -1.0825 | Lower | +15.1599 | Higher |
| MI (SEI) | 16.2999 | **39.4273 ✅** | 17.8617 | -23.1274 | Lower | -1.5618 | Lower | +21.5656 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 15 | **8 ✅** | 15 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 1749.5675 | **944.4817 ✅** | 1761.4290 | +805.0858 | Higher | -11.8615 | Lower | -816.9473 | Lower |
| Halstead Effort | 70857.4843 | **32584.6176 ✅** | 71337.8741 | +38272.8667 | Higher | -480.3897 | Lower | -38753.2564 | Lower |
| SLOC | 60 | **28 ✅** | 56 | +32 | Higher | +4 | Higher | -28 | Lower |
| PLOC | 47 | **23 ✅** | 47 | +24 | Higher | 0 | Same | -24 | Lower |
| LLOC | 35 | **15 ✅** | 35 | +20 | Higher | 0 | Same | -20 | Lower |
| Comments (CLOC) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 13 | **5 ✅** | 9 | +8 | Higher | +4 | Higher | -4 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p00507

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.8694 | 67.8964 | **71.7597 ✅** | -2.0270 | Lower | -5.8903 | Lower | -3.8633 | Lower |
| MI (SEI) | 50.5618 | **58.6451 ✅** | 49.1898 | -8.0833 | Lower | +1.3720 | Higher | +9.4553 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 10 | **9 ✅** | 20 | +1 | Higher | -10 | Lower | -11 | Lower |
| Halstead Volume | 1312.1877 | **1099.1835 ✅** | 1179.8645 | +213.0042 | Higher | +132.3233 | Higher | -80.6810 | Lower |
| Halstead Effort | 38914.5678 | **23082.8538 ✅** | 39853.1999 | +15831.7140 | Higher | -938.6322 | Lower | -16770.3461 | Lower |
| SLOC | 57 | 54 | **41 ✅** | +3 | Higher | +16 | Higher | +13 | Higher |
| PLOC | 39 | 35 | **33 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| LLOC | 27 | **21 ✅** | 23 | +6 | Higher | +4 | Higher | -2 | Lower |
| Comments (CLOC) | 10 | **14 ✅** | 3 | -4 | Lower | +7 | Higher | +11 | Higher |
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
| MI (original) | **50.9394 ✅** | 46.2418 | 36.7314 | +4.6976 | Higher | +14.2080 | Higher | +9.5104 | Higher |
| MI (SEI) | 10.5057 | **13.9025 ✅** | -11.2945 | -3.3968 | Lower | +21.8002 | Higher | +25.1970 | Higher |
| Cyclomatic (sum) | **16 ✅** | 25 | 29 | -9 | Lower | -13 | Lower | -4 | Lower |
| Cognitive (sum) | 21 | **17 ✅** | 29 | +4 | Higher | -8 | Lower | -12 | Lower |
| Halstead Volume | 3502.3482 | **2682.1846 ✅** | 5499.3627 | +820.1635 | Higher | -1997.0146 | Lower | -2817.1781 | Lower |
| Halstead Effort | 142481.8912 | **111525.2373 ✅** | 435241.9367 | +30956.6539 | Higher | -292760.0454 | Lower | -323716.6994 | Lower |
| SLOC | **96 ✅** | 123 | 166 | -27 | Lower | -70 | Lower | -43 | Lower |
| PLOC | **84 ✅** | 95 | 145 | -11 | Lower | -61 | Lower | -50 | Lower |
| LLOC | 48 | **28 ✅** | 93 | +20 | Higher | -45 | Lower | -65 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 2 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | **12 ✅** | 19 | 21 | -7 | Lower | -9 | Lower | -2 | Lower |
| Functions | **2 ✅** | 5 | 8 | -3 | Lower | -6 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 5 | 3 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00528

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.3047 | **51.5342 ✅** | 24.0184 | -12.2295 | Lower | +15.2863 | Higher | +27.5158 | Higher |
| MI (SEI) | -8.7360 | **24.7568 ✅** | -29.5201 | -33.4929 | Lower | +20.7840 | Higher | +54.2769 | Higher |
| Cyclomatic (sum) | 35 | **21 ✅** | 45 | +14 | Higher | -10 | Lower | -24 | Lower |
| Cognitive (sum) | 28 | **15 ✅** | 42 | +13 | Higher | -14 | Lower | -27 | Lower |
| Halstead Volume | 5128.7809 | **2503.9851 ✅** | 9058.3041 | +2624.7958 | Higher | -3929.5233 | Lower | -6554.3190 | Lower |
| Halstead Effort | 315462.0631 | **88891.4715 ✅** | 1075857.2309 | +226570.5916 | Higher | -760395.1678 | Lower | -986965.7594 | Lower |
| SLOC | 133 | **96 ✅** | 247 | +37 | Higher | -114 | Lower | -151 | Lower |
| PLOC | 111 | **76 ✅** | 211 | +35 | Higher | -100 | Lower | -135 | Lower |
| LLOC | 68 | **29 ✅** | 194 | +39 | Higher | -126 | Lower | -165 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 2 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | 22 | **17 ✅** | 34 | +5 | Higher | -12 | Lower | -17 | Lower |
| Functions | **4 ✅** | 5 | 11 | -1 | Lower | -7 | Lower | -6 | Lower |
| Closures | 8 | **0 ✅** | 2 | +8 | Higher | +6 | Higher | -2 | Lower |
| Exits | **4 ✅** | 5 | 8 | -1 | Lower | -4 | Lower | -3 | Lower |

## p00534

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.2948 | **75.0482 ✅** | 65.8365 | -7.7535 | Lower | +1.4582 | Higher | +9.2117 | Higher |
| MI (SEI) | 53.1126 | **62.9418 ✅** | 45.9645 | -9.8292 | Lower | +7.1481 | Higher | +16.9773 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 11 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1670.2288 | **1026.9765 ✅** | 1900.2625 | +643.2523 | Higher | -230.0337 | Lower | -873.2860 | Lower |
| Halstead Effort | 64916.2275 | **37328.3643 ✅** | 71829.9221 | +27587.8632 | Higher | -6913.6946 | Lower | -34501.5578 | Lower |
| SLOC | 49 | **36 ✅** | 50 | +13 | Higher | -1 | Lower | -14 | Lower |
| PLOC | 32 | **24 ✅** | 36 | +8 | Higher | -4 | Lower | -12 | Lower |
| LLOC | 28 | **17 ✅** | 30 | +11 | Higher | -2 | Lower | -13 | Lower |
| Comments (CLOC) | **9 ✅** | 6 | 6 | +3 | Higher | +3 | Higher | 0 | Same |
| Blank Lines | 10 | **6 ✅** | 8 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | **0 ✅** | 3 | +1 | Higher | -2 | Lower | -3 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00543

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.7427 | **82.8508 ✅** | 69.5328 | -16.1081 | Lower | -2.7901 | Lower | +13.3179 | Higher |
| MI (SEI) | 52.6253 | **85.7255 ✅** | 58.3974 | -33.1003 | Lower | -5.7721 | Lower | +27.3281 | Higher |
| Cyclomatic (sum) | 19 | **9 ✅** | 10 | +10 | Higher | +9 | Higher | -1 | Lower |
| Cognitive (sum) | 15 | **4 ✅** | 9 | +11 | Higher | +6 | Higher | -5 | Lower |
| Halstead Volume | 1453.0306 | **603.9396 ✅** | 1183.1274 | +849.0911 | Higher | +269.9032 | Higher | -579.1879 | Lower |
| Halstead Effort | 47885.9071 | **17762.9280 ✅** | 43209.8712 | +30122.9791 | Higher | +4676.0359 | Higher | -25446.9432 | Lower |
| SLOC | 46 | **26 ✅** | 47 | +20 | Higher | -1 | Lower | -21 | Lower |
| PLOC | 33 | **17 ✅** | 33 | +16 | Higher | 0 | Same | -16 | Lower |
| LLOC | 22 | **10 ✅** | 25 | +12 | Higher | -3 | Lower | -15 | Lower |
| Comments (CLOC) | 8 | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 8 | 1 | 1 | +7 | Higher | +7 | Higher | 0 | Same |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00554

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.3354 | **65.2296 ✅** | 65.1901 | -3.8942 | Lower | -3.8547 | Lower | +0.0395 | Higher |
| MI (SEI) | 36.6260 | **53.9106 ✅** | 49.2286 | -17.2846 | Lower | -12.6026 | Lower | +4.6820 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 21 | **9 ✅** | 20 | +12 | Higher | +1 | Higher | -11 | Lower |
| Halstead Volume | 1740.5601 | **1405.7265 ✅** | 1549.7303 | +334.8336 | Higher | +190.8298 | Higher | -144.0038 | Lower |
| Halstead Effort | 78002.8764 | **37315.6487 ✅** | 66142.4888 | +40687.2277 | Higher | +11860.3876 | Higher | -28826.8400 | Lower |
| SLOC | 66 | 58 | **54 ✅** | +8 | Higher | +12 | Higher | +4 | Higher |
| PLOC | 48 | **33 ✅** | 42 | +15 | Higher | +6 | Higher | -9 | Lower |
| LLOC | 43 | **25 ✅** | 37 | +18 | Higher | +6 | Higher | -12 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 9 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 12 | 13 | **9 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00571

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3940 | **71.1134 ✅** | 57.5896 | -5.7194 | Lower | +7.8044 | Higher | +13.5238 | Higher |
| MI (SEI) | 39.5729 | **65.3241 ✅** | 23.6575 | -25.7512 | Lower | +15.9154 | Higher | +41.6666 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 12 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1659.8363 | **1065.9822 ✅** | 2312.9876 | +593.8540 | Higher | -653.1513 | Lower | -1247.0053 | Lower |
| Halstead Effort | 48494.1350 | **20653.4057 ✅** | 107388.7082 | +27840.7293 | Higher | -58894.5732 | Lower | -86735.3026 | Lower |
| SLOC | 56 | **46 ✅** | 77 | +10 | Higher | -21 | Lower | -31 | Lower |
| PLOC | 43 | **27 ✅** | 62 | +16 | Higher | -19 | Lower | -35 | Lower |
| LLOC | 28 | **21 ✅** | 42 | +7 | Higher | -14 | Lower | -21 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 3 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | 9 | 9 | 13 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |

## p00594

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.0125 | 58.6920 | **66.8232 ✅** | +4.3205 | Higher | -3.8108 | Lower | -8.1313 | Lower |
| MI (SEI) | 43.4390 | 38.7231 | **50.8180 ✅** | +4.7159 | Higher | -7.3790 | Lower | -12.0949 | Lower |
| Cyclomatic (sum) | 16 | 16 | **13 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 22 | 22 | **10 ✅** | 0 | Same | +12 | Higher | +12 | Higher |
| Halstead Volume | 1411.1872 | 1774.3505 | **1352.6779 ✅** | -363.1633 | Lower | +58.5093 | Higher | +421.6726 | Higher |
| Halstead Effort | 70830.7445 | 85760.2742 | **41893.2317 ✅** | -14929.5297 | Lower | +28937.5128 | Higher | +43867.0425 | Higher |
| SLOC | 61 | 74 | **51 ✅** | -13 | Lower | +10 | Higher | +23 | Higher |
| PLOC | 47 | 57 | **44 ✅** | -10 | Lower | +3 | Higher | +13 | Higher |
| LLOC | 33 | 43 | **27 ✅** | -10 | Lower | +6 | Higher | +16 | Higher |
| Comments (CLOC) | 8 | **11 ✅** | 8 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 9 | 12 | **7 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00629

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.9729 | 56.6819 | **56.9970 ✅** | +0.2910 | Higher | -0.0242 | Lower | -0.3152 | Lower |
| MI (SEI) | 30.4734 | **35.4818 ✅** | 18.0735 | -5.0084 | Lower | +12.3999 | Higher | +17.4083 | Higher |
| Cyclomatic (sum) | 17 | **16 ✅** | 21 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 20 | **17 ✅** | 19 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 1927.2757 | **1826.5263 ✅** | 2685.3425 | +100.7494 | Higher | -758.0668 | Lower | -858.8162 | Lower |
| Halstead Effort | 71422.5718 | **53330.3214 ✅** | 122112.4185 | +18092.2505 | Higher | -50689.8467 | Lower | -68782.0971 | Lower |
| SLOC | 79 | 83 | **67 ✅** | -4 | Lower | +12 | Higher | +16 | Higher |
| PLOC | 65 | 59 | **58 ✅** | +6 | Higher | +7 | Higher | +1 | Higher |
| LLOC | 36 | **26 ✅** | 41 | +10 | Higher | -5 | Lower | -15 | Lower |
| Comments (CLOC) | 7 | **12 ✅** | 1 | -5 | Lower | +6 | Higher | +11 | Higher |
| Blank Lines | 13 | 14 | **8 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 5 | **0 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00641

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.6547 | **52.9989 ✅** | 46.6205 | -5.3442 | Lower | +1.0342 | Higher | +6.3784 | Higher |
| MI (SEI) | 8.1745 | **37.0139 ✅** | 8.3700 | -28.8394 | Lower | -0.1955 | Lower | +28.6439 | Higher |
| Cyclomatic (sum) | 22 | **14 ✅** | 24 | +8 | Higher | -2 | Lower | -10 | Lower |
| Cognitive (sum) | 14 | **11 ✅** | 18 | +3 | Higher | -4 | Lower | -7 | Lower |
| Halstead Volume | 3709.9376 | **2493.1627 ✅** | 3394.7062 | +1216.7750 | Higher | +315.2314 | Higher | -901.5435 | Lower |
| Halstead Effort | 218356.3292 | **107148.0137 ✅** | 255155.5906 | +111208.3155 | Higher | -36799.2614 | Lower | -148007.5768 | Lower |
| SLOC | 106 | **97 ✅** | 113 | +9 | Higher | -7 | Lower | -16 | Lower |
| PLOC | 87 | **71 ✅** | 95 | +16 | Higher | -8 | Lower | -24 | Lower |
| LLOC | 57 | **45 ✅** | 67 | +12 | Higher | -10 | Lower | -22 | Lower |
| Comments (CLOC) | 3 | **24 ✅** | 4 | -21 | Lower | -1 | Lower | +20 | Higher |
| Blank Lines | 17 | **14 ✅** | 16 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |

## p00649

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.1828 | **57.4761 ✅** | 44.5135 | -10.2933 | Lower | +2.6693 | Higher | +12.9626 | Higher |
| MI (SEI) | 7.5043 | **32.7318 ✅** | 5.1103 | -25.2275 | Lower | +2.3941 | Higher | +27.6216 | Higher |
| Cyclomatic (sum) | 26 | **21 ✅** | 27 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 31 | **16 ✅** | 35 | +15 | Higher | -4 | Lower | -19 | Lower |
| Halstead Volume | 2788.8061 | **2044.2297 ✅** | 3511.2059 | +744.5764 | Higher | -722.3998 | Lower | -1466.9761 | Lower |
| Halstead Effort | 111916.0013 | **68560.3203 ✅** | 233084.3050 | +43355.6810 | Higher | -121168.3037 | Lower | -164523.9847 | Lower |
| SLOC | 113 | **71 ✅** | 122 | +42 | Higher | -9 | Lower | -51 | Lower |
| PLOC | 95 | **56 ✅** | 100 | +39 | Higher | -5 | Lower | -44 | Lower |
| LLOC | 53 | **23 ✅** | 75 | +30 | Higher | -22 | Lower | -52 | Lower |
| Comments (CLOC) | 3 | **7 ✅** | 4 | -4 | Lower | -1 | Lower | +3 | Higher |
| Blank Lines | 16 | **10 ✅** | 20 | +6 | Higher | -4 | Lower | -10 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p00653

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.4899 | **60.4673 ✅** | 43.7356 | -15.9774 | Lower | +0.7543 | Higher | +16.7317 | Higher |
| MI (SEI) | 1.6025 | **50.9328 ✅** | 0.4795 | -49.3303 | Lower | +1.1230 | Higher | +50.4533 | Higher |
| Cyclomatic (sum) | 27 | **15 ✅** | 28 | +12 | Higher | -1 | Lower | -13 | Lower |
| Cognitive (sum) | 27 | **10 ✅** | 30 | +17 | Higher | -3 | Lower | -20 | Lower |
| Halstead Volume | 4869.8848 | **1974.3760 ✅** | 4953.4056 | +2895.5088 | Higher | -83.5208 | Lower | -2979.0296 | Lower |
| Halstead Effort | 431282.9642 | **69189.0025 ✅** | 433175.3231 | +362093.9617 | Higher | -1892.3589 | Lower | -363986.3206 | Lower |
| SLOC | 110 | **65 ✅** | 113 | +45 | Higher | -3 | Lower | -48 | Lower |
| PLOC | 89 | **44 ✅** | 91 | +45 | Higher | -2 | Lower | -47 | Lower |
| LLOC | 75 | **25 ✅** | 80 | +50 | Higher | -5 | Lower | -55 | Lower |
| Comments (CLOC) | 2 | **20 ✅** | 2 | -18 | Lower | 0 | Same | +18 | Higher |
| Blank Lines | 19 | **10 ✅** | 20 | +9 | Higher | -1 | Lower | -10 | Lower |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **3 ✅** | 7 | +4 | Higher | 0 | Same | -4 | Lower |

## p00655

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.8267 | **56.7020 ✅** | 29.6571 | -19.8754 | Lower | +7.1696 | Higher | +27.0449 | Higher |
| MI (SEI) | -9.6017 | **40.8222 ✅** | -17.1593 | -50.4240 | Lower | +7.5575 | Higher | +57.9815 | Higher |
| Cyclomatic (sum) | 40 | **21 ✅** | 45 | +19 | Higher | -5 | Lower | -24 | Lower |
| Cognitive (sum) | 32 | **15 ✅** | 41 | +17 | Higher | -9 | Lower | -26 | Lower |
| Halstead Volume | 4551.7882 | **1573.6168 ✅** | 7050.7261 | +2978.1714 | Higher | -2498.9379 | Lower | -5477.1093 | Lower |
| Halstead Effort | 274507.8439 | **48152.6734 ✅** | 926133.6129 | +226355.1705 | Higher | -651625.7690 | Lower | -877980.9395 | Lower |
| SLOC | 150 | **81 ✅** | 189 | +69 | Higher | -39 | Lower | -108 | Lower |
| PLOC | 136 | **63 ✅** | 172 | +73 | Higher | -36 | Lower | -109 | Lower |
| LLOC | 69 | **25 ✅** | 118 | +44 | Higher | -49 | Lower | -93 | Lower |
| Comments (CLOC) | 2 | **17 ✅** | 4 | -15 | Lower | -2 | Lower | +13 | Higher |
| Blank Lines | 14 | **11 ✅** | 15 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 8 | **6 ✅** | 10 | +2 | Higher | -2 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -3.0127 | **42.0249 ✅** | -7.6951 | -45.0376 | Lower | +4.6824 | Higher | +49.7200 | Higher |
| MI (SEI) | -66.8107 | **17.5490 ✅** | -64.3695 | -84.3598 | Lower | -2.4412 | Lower | +81.9185 | Higher |
| Cyclomatic (sum) | 130 | **31 ✅** | 143 | +99 | Higher | -13 | Lower | -112 | Lower |
| Cognitive (sum) | 176 | **18 ✅** | 187 | +158 | Higher | -11 | Lower | -169 | Lower |
| Halstead Volume | 13604.2840 | **3162.0196 ✅** | 13371.7402 | +10442.2645 | Higher | +232.5438 | Higher | -10209.7206 | Lower |
| Halstead Effort | 2017855.4270 | **145594.0613 ✅** | 1947663.1233 | +1872261.3657 | Higher | +70192.3037 | Higher | -1802069.0620 | Lower |
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
| MI (original) | 17.0274 | **47.4149 ✅** | 14.9868 | -30.3875 | Lower | +2.0406 | Higher | +32.4281 | Higher |
| MI (SEI) | -33.9009 | **20.2285 ✅** | -34.2235 | -54.1293 | Lower | +0.3226 | Higher | +54.4520 | Higher |
| Cyclomatic (sum) | 60 | **22 ✅** | 62 | +38 | Higher | -2 | Lower | -40 | Lower |
| Cognitive (sum) | 81 | **23 ✅** | 85 | +58 | Higher | -4 | Lower | -62 | Lower |
| Halstead Volume | 11335.3400 | **3365.3639 ✅** | 10931.9966 | +7969.9761 | Higher | +403.3434 | Higher | -7566.6327 | Lower |
| Halstead Effort | 1300730.2634 | **140765.9987 ✅** | 1233841.6387 | +1159964.2647 | Higher | +66888.6247 | Higher | -1093075.6400 | Lower |
| SLOC | 286 | **111 ✅** | 319 | +175 | Higher | -33 | Lower | -208 | Lower |
| PLOC | 254 | **93 ✅** | 279 | +161 | Higher | -25 | Lower | -186 | Lower |
| LLOC | 166 | **35 ✅** | 187 | +131 | Higher | -21 | Lower | -152 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 10 | -7 | Lower | -4 | Lower | +3 | Higher |
| Blank Lines | 30 | **13 ✅** | 38 | +17 | Higher | -8 | Lower | -25 | Lower |
| Functions | 14 | **6 ✅** | 15 | +8 | Higher | -1 | Lower | -9 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 12 | **7 ✅** | 13 | +5 | Higher | -1 | Lower | -6 | Lower |

## p00690

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.9855 | **50.5592 ✅** | 47.7650 | -1.5737 | Lower | +1.2205 | Higher | +2.7942 | Higher |
| MI (SEI) | -2.4842 | **27.7612 ✅** | 6.7202 | -30.2454 | Lower | -9.2044 | Lower | +21.0410 | Higher |
| Cyclomatic (sum) | 25 | **17 ✅** | 27 | +8 | Higher | -2 | Lower | -10 | Lower |
| Cognitive (sum) | 38 | **22 ✅** | 38 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 3424.9814 | **2984.5159 ✅** | 3282.1060 | +440.4655 | Higher | +142.8754 | Higher | -297.5901 | Lower |
| Halstead Effort | 221947.8076 | **119935.8940 ✅** | 206936.7837 | +102011.9135 | Higher | +15011.0239 | Higher | -87000.8897 | Lower |
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
| MI (original) | 59.8826 | **72.2200 ✅** | 50.7036 | -12.3374 | Lower | +9.1790 | Higher | +21.5164 | Higher |
| MI (SEI) | 13.1351 | **63.6630 ✅** | 22.2686 | -50.5279 | Lower | -9.1335 | Lower | +41.3944 | Higher |
| Cyclomatic (sum) | 24 | **8 ✅** | 29 | +16 | Higher | -5 | Lower | -21 | Lower |
| Cognitive (sum) | 18 | **11 ✅** | 28 | +7 | Higher | -10 | Lower | -17 | Lower |
| Halstead Volume | 2643.8195 | **824.3742 ✅** | 3369.7909 | +1819.4453 | Higher | -725.9714 | Lower | -2545.4167 | Lower |
| Halstead Effort | 156855.0262 | **16935.5129 ✅** | 210401.3200 | +139919.5133 | Higher | -53546.2939 | Lower | -193465.8071 | Lower |
| SLOC | 54 | **46 ✅** | 82 | +8 | Higher | -28 | Lower | -36 | Lower |
| PLOC | 47 | **30 ✅** | 66 | +17 | Higher | -19 | Lower | -36 | Lower |
| LLOC | 35 | **17 ✅** | 44 | +18 | Higher | -9 | Lower | -27 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 7 | -11 | Lower | -7 | Lower | +4 | Higher |
| Blank Lines | **7 ✅** | 8 | 9 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00723

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0859 | **66.7035 ✅** | 51.6444 | -13.6176 | Lower | +1.4415 | Higher | +15.0591 | Higher |
| MI (SEI) | 2.5150 | **47.2792 ✅** | 0.9444 | -44.7642 | Lower | +1.5705 | Higher | +46.3348 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 21 | +7 | Higher | -5 | Lower | -12 | Lower |
| Cognitive (sum) | 28 | **6 ✅** | 29 | +22 | Higher | -1 | Lower | -23 | Lower |
| Halstead Volume | 3149.7606 | **1168.2782 ✅** | 2706.3872 | +1981.4824 | Higher | +443.3734 | Higher | -1538.1090 | Lower |
| Halstead Effort | 151458.4886 | **26091.5472 ✅** | 133386.2273 | +125366.9415 | Higher | +18072.2614 | Higher | -107294.6801 | Lower |
| SLOC | 87 | **57 ✅** | 93 | +30 | Higher | -6 | Lower | -36 | Lower |
| PLOC | 70 | **37 ✅** | 82 | +33 | Higher | -12 | Lower | -45 | Lower |
| LLOC | 50 | **21 ✅** | 44 | +29 | Higher | +6 | Higher | -23 | Lower |
| Comments (CLOC) | 0 | **7 ✅** | 0 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 17 | 13 | **11 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Functions | 4 | **3 ✅** | 6 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 4 | 3 | -3 | Lower | -2 | Lower | +1 | Higher |

## p00740

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3689 | **74.5943 ✅** | 60.7870 | -9.2254 | Lower | +4.5818 | Higher | +13.8072 | Higher |
| MI (SEI) | 40.8002 | **68.7193 ✅** | 22.8618 | -27.9191 | Lower | +17.9384 | Higher | +45.8575 | Higher |
| Cyclomatic (sum) | 17 | **11 ✅** | 15 | +6 | Higher | +2 | Higher | -4 | Lower |
| Cognitive (sum) | 27 | **6 ✅** | 16 | +21 | Higher | +11 | Higher | -10 | Lower |
| Halstead Volume | 1254.5551 | **829.2698 ✅** | 1613.1606 | +425.2852 | Higher | -358.6055 | Lower | -783.8907 | Lower |
| Halstead Effort | 50089.2723 | **20623.5798 ✅** | 53772.0185 | +29465.6925 | Higher | -3682.7462 | Lower | -33148.4387 | Lower |
| SLOC | 54 | **38 ✅** | 68 | +16 | Higher | -14 | Lower | -30 | Lower |
| PLOC | 47 | **28 ✅** | 58 | +19 | Higher | -11 | Lower | -30 | Lower |
| LLOC | 35 | **13 ✅** | 37 | +22 | Higher | -2 | Lower | -24 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 1 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 7 | **6 ✅** | 10 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | **1 ✅** | 2 | 5 | -1 | Lower | -4 | Lower | -3 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00749

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.0565 | **67.0327 ✅** | 22.7393 | -42.9762 | Lower | +1.3172 | Higher | +44.2934 | Higher |
| MI (SEI) | -35.3945 | **57.6728 ✅** | -28.6420 | -93.0674 | Lower | -6.7525 | Lower | +86.3149 | Higher |
| Cyclomatic (sum) | 55 | **14 ✅** | 55 | +41 | Higher | 0 | Same | -41 | Lower |
| Cognitive (sum) | 84 | **14 ✅** | 78 | +70 | Higher | +6 | Higher | -64 | Lower |
| Halstead Volume | 7726.6685 | **1243.0557 ✅** | 8356.0772 | +6483.6128 | Higher | -629.4087 | Lower | -7113.0215 | Lower |
| Halstead Effort | 755701.7780 | **31325.0037 ✅** | 824698.3974 | +724376.7744 | Higher | -68996.6194 | Lower | -793373.3938 | Lower |
| SLOC | 225 | **51 ✅** | 238 | +174 | Higher | -13 | Lower | -187 | Lower |
| PLOC | 202 | **35 ✅** | 207 | +167 | Higher | -5 | Lower | -172 | Lower |
| LLOC | 137 | **18 ✅** | 140 | +119 | Higher | -3 | Lower | -122 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 3 | -13 | Lower | -3 | Lower | +10 | Higher |
| Blank Lines | 23 | **7 ✅** | 29 | +16 | Higher | -6 | Lower | -22 | Lower |
| Functions | 7 | **2 ✅** | 9 | +5 | Higher | -2 | Lower | -7 | Lower |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 13 | **1 ✅** | 13 | +12 | Higher | 0 | Same | -12 | Lower |

## p00762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.5982 | **69.8981 ✅** | 38.9680 | -30.2998 | Lower | +0.6302 | Higher | +30.9300 | Higher |
| MI (SEI) | 9.4873 | **59.7294 ✅** | 4.1333 | -50.2422 | Lower | +5.3539 | Higher | +55.5961 | Higher |
| Cyclomatic (sum) | 30 | **8 ✅** | 30 | +22 | Higher | 0 | Same | -22 | Lower |
| Cognitive (sum) | 20 | **7 ✅** | 16 | +13 | Higher | +4 | Higher | -9 | Lower |
| Halstead Volume | 5154.0298 | **1128.4003 ✅** | 4997.6393 | +4025.6295 | Higher | +156.3906 | Higher | -3869.2389 | Lower |
| Halstead Effort | 451583.9657 | **28482.3807 ✅** | 440661.4088 | +423101.5849 | Higher | +10922.5569 | Higher | -412179.0280 | Lower |
| SLOC | 140 | **48 ✅** | 147 | +92 | Higher | -7 | Lower | -99 | Lower |
| PLOC | 113 | **30 ✅** | 119 | +83 | Higher | -6 | Lower | -89 | Lower |
| LLOC | 95 | **19 ✅** | 93 | +76 | Higher | +2 | Higher | -74 | Lower |
| Comments (CLOC) | **16 ✅** | 11 | 11 | +5 | Higher | +5 | Higher | 0 | Same |
| Blank Lines | 23 | **9 ✅** | 24 | +14 | Higher | -1 | Lower | -15 | Lower |
| Functions | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 5 | **0 ✅** | 5 | +5 | Higher | 0 | Same | -5 | Lower |

## p00792

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1003 | **59.8711 ✅** | 51.6638 | -4.7708 | Lower | +3.4366 | Higher | +8.2073 | Higher |
| MI (SEI) | 14.5653 | **46.1224 ✅** | 20.9614 | -31.5572 | Lower | -6.3962 | Lower | +25.1610 | Higher |
| Cyclomatic (sum) | 19 | **12 ✅** | 23 | +7 | Higher | -4 | Lower | -11 | Lower |
| Cognitive (sum) | 31 | **13 ✅** | 45 | +18 | Higher | -14 | Lower | -32 | Lower |
| Halstead Volume | 2852.9540 | **1553.5122 ✅** | 3037.9751 | +1299.4418 | Higher | -185.0211 | Lower | -1484.4629 | Lower |
| Halstead Effort | 187093.7218 | **44386.0636 ✅** | 197179.0490 | +142707.6582 | Higher | -10085.3272 | Lower | -152792.9854 | Lower |
| SLOC | 76 | 76 | 87 | 0 | Same | -11 | Lower | -11 | Lower |
| PLOC | 63 | **53 ✅** | 71 | +10 | Higher | -8 | Lower | -18 | Lower |
| LLOC | 58 | **40 ✅** | 62 | +18 | Higher | -4 | Lower | -22 | Lower |
| Comments (CLOC) | 1 | **18 ✅** | 6 | -17 | Lower | -5 | Lower | +12 | Higher |
| Blank Lines | 13 | **12 ✅** | 13 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00799

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.3119 | **60.3456 ✅** | 46.6912 | -11.0337 | Lower | +2.6207 | Higher | +13.6544 | Higher |
| MI (SEI) | 15.6151 | **39.5349 ✅** | 8.2903 | -23.9198 | Lower | +7.3248 | Higher | +31.2446 | Higher |
| Cyclomatic (sum) | 22 | **21 ✅** | 24 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 66 | **23 ✅** | 55 | +43 | Higher | +11 | Higher | -32 | Lower |
| Halstead Volume | 2619.7300 | **1626.7294 ✅** | 3086.3437 | +993.0006 | Higher | -466.6137 | Lower | -1459.6143 | Lower |
| Halstead Effort | 116141.3633 | **61002.3528 ✅** | 201018.4358 | +55139.0104 | Higher | -84877.0725 | Lower | -140016.0829 | Lower |
| SLOC | 107 | **64 ✅** | 116 | +43 | Higher | -9 | Lower | -52 | Lower |
| PLOC | 91 | **48 ✅** | 101 | +43 | Higher | -10 | Lower | -53 | Lower |
| LLOC | 65 | **31 ✅** | 76 | +34 | Higher | -11 | Lower | -45 | Lower |
| Comments (CLOC) | 6 | **8 ✅** | 4 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | **0 ✅** | 6 | 3 | -6 | Lower | -3 | Lower | +3 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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
| MI (original) | 55.1128 | **69.4207 ✅** | 58.2089 | -14.3078 | Lower | -3.0961 | Lower | +11.2117 | Higher |
| MI (SEI) | 29.1862 | **60.2354 ✅** | 31.5287 | -31.0491 | Lower | -2.3425 | Lower | +28.7067 | Higher |
| Cyclomatic (sum) | 24 | **14 ✅** | 16 | +10 | Higher | +8 | Higher | -2 | Lower |
| Cognitive (sum) | 22 | **13 ✅** | 36 | +9 | Higher | -14 | Lower | -23 | Lower |
| Halstead Volume | 2479.0698 | **1083.0835 ✅** | 2120.5737 | +1395.9863 | Higher | +358.4961 | Higher | -1037.4902 | Lower |
| Halstead Effort | 138296.6817 | **41066.9178 ✅** | 109305.9365 | +97229.7639 | Higher | +28990.7452 | Higher | -68239.0187 | Lower |
| SLOC | 74 | **46 ✅** | 72 | +28 | Higher | +2 | Higher | -26 | Lower |
| PLOC | 64 | **35 ✅** | 63 | +29 | Higher | +1 | Higher | -28 | Lower |
| LLOC | 48 | **21 ✅** | 49 | +27 | Higher | -1 | Lower | -28 | Lower |
| Comments (CLOC) | 7 | **11 ✅** | 6 | -4 | Lower | +1 | Higher | +5 | Higher |
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
| MI (original) | 41.8251 | **64.7202 ✅** | 45.8604 | -22.8951 | Lower | -4.0353 | Lower | +18.8598 | Higher |
| MI (SEI) | 4.0506 | **53.8639 ✅** | 2.5143 | -49.8133 | Lower | +1.5363 | Higher | +51.3496 | Higher |
| Cyclomatic (sum) | 29 | **12 ✅** | 20 | +17 | Higher | +9 | Higher | -8 | Lower |
| Cognitive (sum) | 48 | **25 ✅** | 40 | +23 | Higher | +8 | Higher | -15 | Lower |
| Halstead Volume | 4641.0287 | **1276.8985 ✅** | 4097.6639 | +3364.1302 | Higher | +543.3648 | Higher | -2820.7654 | Lower |
| Halstead Effort | 395647.6972 | **30900.9436 ✅** | 367360.3296 | +364746.7536 | Higher | +28287.3676 | Higher | -336459.3860 | Lower |
| SLOC | 128 | **60 ✅** | 118 | +68 | Higher | +10 | Higher | -58 | Lower |
| PLOC | 109 | **37 ✅** | 100 | +72 | Higher | +9 | Higher | -63 | Lower |
| LLOC | 103 | **27 ✅** | 103 | +76 | Higher | 0 | Same | -76 | Lower |
| Comments (CLOC) | 6 | **15 ✅** | 2 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 16 | **10 ✅** | 16 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 3 | **0 ✅** | 1 | +3 | Higher | +2 | Higher | -1 | Lower |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.0062 | **72.8600 ✅** | 46.9849 | -24.8538 | Lower | +1.0213 | Higher | +25.8751 | Higher |
| MI (SEI) | 12.0496 | **64.3828 ✅** | 6.9455 | -52.3332 | Lower | +5.1040 | Higher | +57.4372 | Higher |
| Cyclomatic (sum) | 18 | **6 ✅** | 20 | +12 | Higher | -2 | Lower | -14 | Lower |
| Cognitive (sum) | 18 | **3 ✅** | 18 | +15 | Higher | 0 | Same | -15 | Lower |
| Halstead Volume | 4391.5745 | **796.3187 ✅** | 4477.3627 | +3595.2558 | Higher | -85.7882 | Lower | -3681.0440 | Lower |
| Halstead Effort | 235118.1441 | **13696.6816 ✅** | 263136.8086 | +221421.4625 | Higher | -28018.6645 | Lower | -249440.1270 | Lower |
| SLOC | 104 | **46 ✅** | 107 | +58 | Higher | -3 | Lower | -61 | Lower |
| PLOC | 86 | **28 ✅** | 88 | +58 | Higher | -2 | Lower | -60 | Lower |
| LLOC | 48 | **12 ✅** | 51 | +36 | Higher | -3 | Lower | -39 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 3 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 16 | **7 ✅** | 17 | +9 | Higher | -1 | Lower | -10 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 3 | **1 ✅** | 5 | +2 | Higher | -2 | Lower | -4 | Lower |

## p00855

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.1317 | 66.2455 | **75.2264 ✅** | +6.8862 | Higher | -2.0946 | Lower | -8.9809 | Lower |
| MI (SEI) | 54.9313 | 36.3213 | **71.2345 ✅** | +18.6100 | Higher | -16.3033 | Lower | -34.9132 | Lower |
| Cyclomatic (sum) | 13 | 16 | **10 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Cognitive (sum) | 12 | 14 | **10 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Halstead Volume | 927.4295 | 1174.2906 | **834.0579 ✅** | -246.8611 | Lower | +93.3717 | Higher | +340.2327 | Higher |
| Halstead Effort | **26895.4559 ✅** | 39686.6724 | 33779.3431 | -12791.2166 | Lower | -6883.8873 | Lower | +5907.3293 | Higher |
| SLOC | 39 | 53 | **37 ✅** | -14 | Lower | +2 | Higher | +16 | Higher |
| PLOC | **32 ✅** | 46 | 33 | -14 | Lower | -1 | Lower | +13 | Higher |
| LLOC | **19 ✅** | 24 | 27 | -5 | Lower | -8 | Lower | -3 | Lower |
| Comments (CLOC) | 4 | 2 | **11 ✅** | +2 | Higher | -7 | Lower | -9 | Lower |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 3 | 7 | **2 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |

## p00856

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.8691 ✅** | 51.9947 | 53.4430 | +3.8745 | Higher | +2.4261 | Higher | -1.4484 | Lower |
| MI (SEI) | 7.5486 | **35.4951 ✅** | 29.3575 | -27.9465 | Lower | -21.8089 | Lower | +6.1376 | Higher |
| Cyclomatic (sum) | 26 | 27 | **25 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | **17 ✅** | 29 | 21 | -12 | Lower | -4 | Lower | +8 | Higher |
| Halstead Volume | 2802.2137 | **2567.8982 ✅** | 3009.2378 | +234.3155 | Higher | -207.0241 | Lower | -441.3396 | Lower |
| Halstead Effort | 160143.5319 | **124766.3586 ✅** | 183498.0860 | +35377.1734 | Higher | -23354.5540 | Lower | -58731.7274 | Lower |
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
| MI (original) | 52.7080 | **60.8052 ✅** | 49.1788 | -8.0971 | Lower | +3.5293 | Higher | +11.6264 | Higher |
| MI (SEI) | 10.4874 | **46.7859 ✅** | 10.2104 | -36.2984 | Lower | +0.2770 | Higher | +36.5755 | Higher |
| Cyclomatic (sum) | 18 | 18 | 18 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 32 | **18 ✅** | 32 | +14 | Higher | 0 | Same | -14 | Lower |
| Halstead Volume | 3214.0704 | **1620.2520 ✅** | 3723.5467 | +1593.8184 | Higher | -509.4763 | Lower | -2103.2947 | Lower |
| Halstead Effort | 194521.1330 | **63372.7606 ✅** | 228900.1354 | +131148.3724 | Higher | -34379.0024 | Lower | -165527.3748 | Lower |
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
| MI (original) | 11.1074 | **73.3670 ✅** | 9.6207 | -62.2596 | Lower | +1.4867 | Higher | +63.7463 | Higher |
| MI (SEI) | -44.6527 | **70.5482 ✅** | -45.6410 | -115.2009 | Lower | +0.9882 | Higher | +116.1892 | Higher |
| Cyclomatic (sum) | 84 | **5 ✅** | 84 | +79 | Higher | 0 | Same | -79 | Lower |
| Cognitive (sum) | 112 | **3 ✅** | 112 | +109 | Higher | 0 | Same | -109 | Lower |
| Halstead Volume | 12375.9815 | **808.5134 ✅** | 13065.8575 | +11567.4680 | Higher | -689.8760 | Lower | -12257.3440 | Lower |
| Halstead Effort | 1787445.3248 | **17602.0115 ✅** | 2026659.6667 | +1769843.3132 | Higher | -239214.3420 | Lower | -2009057.6552 | Lower |
| SLOC | 285 | **45 ✅** | 307 | +240 | Higher | -22 | Lower | -262 | Lower |
| PLOC | 253 | **27 ✅** | 273 | +226 | Higher | -20 | Lower | -246 | Lower |
| LLOC | 170 | **14 ✅** | 174 | +156 | Higher | -4 | Lower | -160 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 3 | -14 | Lower | -1 | Lower | +13 | Higher |
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
| MI (original) | **71.6334 ✅** | 60.0879 | 67.5887 | +11.5455 | Higher | +4.0447 | Higher | -7.5007 | Lower |
| MI (SEI) | **60.2925 ✅** | 41.5079 | 34.1082 | +18.7846 | Higher | +26.1843 | Higher | +7.3997 | Higher |
| Cyclomatic (sum) | **7 ✅** | 15 | 13 | -8 | Lower | -6 | Lower | +2 | Higher |
| Cognitive (sum) | **9 ✅** | 19 | 15 | -10 | Lower | -6 | Lower | +4 | Higher |
| Halstead Volume | **1032.9061 ✅** | 1763.2523 | 1322.5020 | -730.3462 | Lower | -289.5959 | Lower | +440.7503 | Higher |
| Halstead Effort | **22896.0849 ✅** | 70944.9741 | 39857.4745 | -48048.8892 | Lower | -16961.3896 | Lower | +31087.4996 | Higher |
| SLOC | **45 ✅** | 69 | 49 | -24 | Lower | -4 | Lower | +20 | Higher |
| PLOC | **29 ✅** | 52 | 39 | -23 | Lower | -10 | Lower | +13 | Higher |
| LLOC | **24 ✅** | 39 | 26 | -15 | Lower | -2 | Lower | +13 | Higher |
| Comments (CLOC) | 9 | **11 ✅** | 1 | -2 | Lower | +8 | Higher | +10 | Higher |
| Blank Lines | **9 ✅** | 10 | 10 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00889

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.8687 | **71.5881 ✅** | 44.3491 | -22.7195 | Lower | +4.5196 | Higher | +27.2391 | Higher |
| MI (SEI) | 4.6170 | **62.0543 ✅** | 6.1314 | -57.4373 | Lower | -1.5144 | Lower | +55.9229 | Higher |
| Cyclomatic (sum) | 21 | **8 ✅** | 25 | +13 | Higher | -4 | Lower | -17 | Lower |
| Cognitive (sum) | 40 | **9 ✅** | 44 | +31 | Higher | -4 | Lower | -35 | Lower |
| Halstead Volume | 3569.1131 | **1069.1472 ✅** | 3763.6708 | +2499.9659 | Higher | -194.5577 | Lower | -2694.5236 | Lower |
| Halstead Effort | 315213.6237 | **24234.0032 ✅** | 286587.8484 | +290979.6205 | Higher | +28625.7752 | Higher | -262353.8453 | Lower |
| SLOC | 101 | **44 ✅** | 124 | +57 | Higher | -23 | Lower | -80 | Lower |
| PLOC | 91 | **28 ✅** | 108 | +63 | Higher | -17 | Lower | -80 | Lower |
| LLOC | 82 | **19 ✅** | 89 | +63 | Higher | -7 | Lower | -70 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 5 | -9 | Lower | -4 | Lower | +5 | Higher |
| Blank Lines | 10 | **9 ✅** | 14 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |

## p00897

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.4147 | **45.2994 ✅** | 21.7372 | -20.8847 | Lower | +2.6775 | Higher | +23.5622 | Higher |
| MI (SEI) | -31.1790 | **11.2671 ✅** | -31.2489 | -42.4460 | Lower | +0.0700 | Higher | +42.5160 | Higher |
| Cyclomatic (sum) | 43 | **24 ✅** | 48 | +19 | Higher | -5 | Lower | -24 | Lower |
| Cognitive (sum) | 58 | **28 ✅** | 53 | +30 | Higher | +5 | Higher | -25 | Lower |
| Halstead Volume | 9170 | **3276.7703 ✅** | 9880.2739 | +5893.2297 | Higher | -710.2739 | Lower | -6603.5036 | Lower |
| Halstead Effort | 796593.9130 | **127794.0404 ✅** | 1092782.0962 | +668799.8726 | Higher | -296188.1832 | Lower | -964988.0558 | Lower |
| SLOC | 247 | **124 ✅** | 265 | +123 | Higher | -18 | Lower | -141 | Lower |
| PLOC | 210 | **100 ✅** | 227 | +110 | Higher | -17 | Lower | -127 | Lower |
| LLOC | 151 | **41 ✅** | 183 | +110 | Higher | -32 | Lower | -142 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 3 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 37 | **20 ✅** | 36 | +17 | Higher | +1 | Higher | -16 | Lower |
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
| MI (original) | 50.4131 | **74.1742 ✅** | 50.5215 | -23.7612 | Lower | -0.1085 | Lower | +23.6527 | Higher |
| MI (SEI) | 18.5374 | **69.8435 ✅** | 21.4921 | -51.3061 | Lower | -2.9547 | Lower | +48.3514 | Higher |
| Cyclomatic (sum) | 21 | **10 ✅** | 21 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 25 | **8 ✅** | 25 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 3669.7869 | **800.9165 ✅** | 3594.0284 | +2868.8705 | Higher | +75.7586 | Higher | -2793.1119 | Lower |
| Halstead Effort | 221144.5521 | **19455.5962 ✅** | 213041.7676 | +201688.9558 | Higher | +8102.7844 | Higher | -193586.1714 | Lower |
| SLOC | 91 | **40 ✅** | 91 | +51 | Higher | 0 | Same | -51 | Lower |
| PLOC | 75 | **24 ✅** | 75 | +51 | Higher | 0 | Same | -51 | Lower |
| LLOC | 61 | **17 ✅** | 62 | +44 | Higher | -1 | Lower | -45 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 8 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 11 | **6 ✅** | 11 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |

## p00949

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.3714 | **55.3071 ✅** | 44.3637 | -8.9358 | Lower | +2.0077 | Higher | +10.9434 | Higher |
| MI (SEI) | 9.4458 | **36.2656 ✅** | 5.8895 | -26.8198 | Lower | +3.5563 | Higher | +30.3761 | Higher |
| Cyclomatic (sum) | 22 | **17 ✅** | 23 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 22 | **20 ✅** | 24 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | 3890.6640 | **2192.9603 ✅** | 3998.9016 | +1697.7037 | Higher | -108.2376 | Lower | -1805.9413 | Lower |
| Halstead Effort | 273361.4340 | **96983.6684 ✅** | 308008.4209 | +176377.7656 | Higher | -34646.9869 | Lower | -211024.7525 | Lower |
| SLOC | 113 | **84 ✅** | 125 | +29 | Higher | -12 | Lower | -41 | Lower |
| PLOC | 98 | **54 ✅** | 105 | +44 | Higher | -7 | Lower | -51 | Lower |
| LLOC | 81 | **40 ✅** | 89 | +41 | Higher | -8 | Lower | -49 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 5 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | **15 ✅** | 17 | 18 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | **0 ✅** | 2 | +3 | Higher | +1 | Higher | -2 | Lower |
| Exits | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |

## p00971

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.2984 | **64.1132 ✅** | 56.1024 | -14.8147 | Lower | -6.8040 | Lower | +8.0108 | Higher |
| MI (SEI) | 10.6418 | **52.2146 ✅** | 6.5614 | -41.5729 | Lower | +4.0803 | Higher | +45.6532 | Higher |
| Cyclomatic (sum) | 18 | **12 ✅** | 13 | +6 | Higher | +5 | Higher | -1 | Lower |
| Cognitive (sum) | 15 | 13 | 13 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 4121.8624 | **1881.8482 ✅** | 2945.6095 | +2240.0142 | Higher | +1176.2530 | Higher | -1063.7612 | Lower |
| Halstead Effort | 356931.0078 | **94092.4107 ✅** | 218510.6652 | +262838.5971 | Higher | +138420.3426 | Higher | -124418.2545 | Lower |
| SLOC | 98 | **55 ✅** | 77 | +43 | Higher | +21 | Higher | -22 | Lower |
| PLOC | 86 | **38 ✅** | 68 | +48 | Higher | +18 | Higher | -30 | Lower |
| LLOC | 70 | **34 ✅** | 52 | +36 | Higher | +18 | Higher | -18 | Lower |
| Comments (CLOC) | 3 | **13 ✅** | 0 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 12 | 9 | 9 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 5 | **1 ✅** | 3 | +4 | Higher | +2 | Higher | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
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
| MI (original) | 59.2475 | **72.1254 ✅** | 58.5990 | -12.8779 | Lower | +0.6484 | Higher | +13.5263 | Higher |
| MI (SEI) | 30.1501 | **66.9905 ✅** | 24.1950 | -36.8404 | Lower | +5.9551 | Higher | +42.7955 | Higher |
| Cyclomatic (sum) | 16 | **6 ✅** | 18 | +10 | Higher | -2 | Lower | -12 | Lower |
| Cognitive (sum) | 31 | **6 ✅** | 32 | +25 | Higher | -1 | Lower | -26 | Lower |
| Halstead Volume | 2388.3531 | **982.1539 ✅** | 2599.0319 | +1406.1992 | Higher | -210.6788 | Lower | -1616.8779 | Lower |
| Halstead Effort | 154302.0837 | **29157.6945 ✅** | 165955.8286 | +125144.3892 | Higher | -11653.7449 | Lower | -136798.1341 | Lower |
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
| MI (original) | 69.4218 | **76.2919 ✅** | 68.9098 | -6.8702 | Lower | +0.5119 | Higher | +7.3821 | Higher |
| MI (SEI) | 53.2548 | **71.6173 ✅** | 59.8945 | -18.3625 | Lower | -6.6397 | Lower | +11.7227 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 1183.1274 | **808.5182 ✅** | 1394.0325 | +374.6092 | Higher | -210.9051 | Lower | -585.5143 | Lower |
| Halstead Effort | 35919.7486 | **16140.4192 ✅** | 52758.7677 | +19779.3294 | Higher | -16839.0190 | Lower | -36618.3484 | Lower |
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
| MI (original) | 52.0932 | **60.8497 ✅** | 31.4199 | -8.7565 | Lower | +20.6733 | Higher | +29.4298 | Higher |
| MI (SEI) | 17.0964 | **50.1332 ✅** | -14.9058 | -33.0368 | Lower | +32.0023 | Higher | +65.0390 | Higher |
| Cyclomatic (sum) | 19 | **17 ✅** | 43 | +2 | Higher | -24 | Lower | -26 | Lower |
| Cognitive (sum) | 63 | **14 ✅** | 67 | +49 | Higher | -4 | Lower | -53 | Lower |
| Halstead Volume | 2623.3331 | **1394.0262 ✅** | 5225.3731 | +1229.3070 | Higher | -2602.0400 | Lower | -3831.3469 | Lower |
| Halstead Effort | 98849.4250 | **41356.1100 ✅** | 369147.3273 | +57493.3150 | Higher | -270297.9023 | Lower | -327791.2173 | Lower |
| SLOC | 94 | **69 ✅** | 192 | +25 | Higher | -98 | Lower | -123 | Lower |
| PLOC | 81 | **47 ✅** | 171 | +34 | Higher | -90 | Lower | -124 | Lower |
| LLOC | 69 | **23 ✅** | 120 | +46 | Higher | -51 | Lower | -97 | Lower |
| Comments (CLOC) | 4 | **19 ✅** | 4 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 11 | 11 | 19 | 0 | Same | -8 | Lower | -8 | Lower |
| Functions | **1 ✅** | 3 | 10 | -2 | Lower | -9 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 4 | 11 | -4 | Lower | -11 | Lower | -7 | Lower |

## p01125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.6714 ✅** | 58.2392 | 58.9879 | +1.4322 | Higher | +0.6835 | Higher | -0.7487 | Lower |
| MI (SEI) | 41.7693 | **47.5959 ✅** | 24.0565 | -5.8267 | Lower | +17.7127 | Higher | +23.5394 | Higher |
| Cyclomatic (sum) | 20 | 24 | **15 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| Cognitive (sum) | 25 | 35 | **22 ✅** | -10 | Lower | +3 | Higher | +13 | Higher |
| Halstead Volume | 2247.9370 | **1615.7105 ✅** | 2280 | +632.2265 | Higher | -32.0630 | Lower | -664.2895 | Lower |
| Halstead Effort | 87428.6926 | **71820.9392 ✅** | 79299.5122 | +15607.7534 | Higher | +8129.1804 | Higher | -7478.5730 | Lower |
| SLOC | **61 ✅** | 70 | 68 | -9 | Lower | -7 | Lower | +2 | Higher |
| PLOC | **47 ✅** | 54 | 57 | -7 | Lower | -10 | Lower | -3 | Lower |
| LLOC | 34 | **33 ✅** | 44 | +1 | Higher | -10 | Lower | -11 | Lower |
| Comments (CLOC) | 10 | **20 ✅** | 2 | -10 | Lower | +8 | Higher | +18 | Higher |
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
| MI (original) | **65.9786 ✅** | 65.9590 | 63.6832 | +0.0195 | Higher | +2.2953 | Higher | +2.2758 | Higher |
| MI (SEI) | 46.8674 | **51.7338 ✅** | 42.5235 | -4.8664 | Lower | +4.3439 | Higher | +9.2103 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | **6 ✅** | 13 | 10 | -7 | Lower | -4 | Lower | +3 | Higher |
| Halstead Volume | 1589.4376 | **1409.0684 ✅** | 1875.5718 | +180.3692 | Higher | -286.1342 | Lower | -466.5034 | Lower |
| Halstead Effort | 50221.8133 | **36999.4098 ✅** | 60829.3569 | +13222.4035 | Higher | -10607.5436 | Lower | -23829.9471 | Lower |
| SLOC | **54 ✅** | 57 | 59 | -3 | Lower | -5 | Lower | -2 | Lower |
| PLOC | 40 | **37 ✅** | 44 | +3 | Higher | -4 | Lower | -7 | Lower |
| LLOC | **25 ✅** | 32 | 35 | -7 | Lower | -10 | Lower | -3 | Lower |
| Comments (CLOC) | 7 | **11 ✅** | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p01150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.4688 | **63.0802 ✅** | 24.6954 | -38.6114 | Lower | -0.2267 | Lower | +38.3847 | Higher |
| MI (SEI) | -17.8576 | **52.3275 ✅** | -18.3753 | -70.1851 | Lower | +0.5178 | Higher | +70.7028 | Higher |
| Cyclomatic (sum) | 60 | **14 ✅** | 59 | +46 | Higher | +1 | Higher | -45 | Lower |
| Cognitive (sum) | 76 | **9 ✅** | 72 | +67 | Higher | +4 | Higher | -63 | Lower |
| Halstead Volume | 6688.5999 | **1248.5813 ✅** | 6891.5864 | +5440.0186 | Higher | -202.9864 | Lower | -5643.0051 | Lower |
| Halstead Effort | 1034503.4550 | **46197.5084 ✅** | 1040905.2058 | +988305.9466 | Higher | -6401.7508 | Lower | -994707.6974 | Lower |
| SLOC | 214 | **65 ✅** | 212 | +149 | Higher | +2 | Higher | -147 | Lower |
| PLOC | 189 | **40 ✅** | 187 | +149 | Higher | +2 | Higher | -147 | Lower |
| LLOC | 124 | **16 ✅** | 154 | +108 | Higher | -30 | Lower | -138 | Lower |
| Comments (CLOC) | 10 | **17 ✅** | 9 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 18 | **10 ✅** | 20 | +8 | Higher | -2 | Lower | -10 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |

## p01153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.2552 | 66.8687 | **71.9870 ✅** | +1.3865 | Higher | -3.7318 | Lower | -5.1183 | Lower |
| MI (SEI) | 53.3452 | 55.8846 | **61.3446 ✅** | -2.5394 | Lower | -7.9994 | Lower | -5.4600 | Lower |
| Cyclomatic (sum) | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 20 | **8 ✅** | 17 | +12 | Higher | +3 | Higher | -9 | Lower |
| Halstead Volume | 1416.6079 | **1264.9509 ✅** | 1379.5064 | +151.6571 | Higher | +37.1015 | Higher | -114.5556 | Lower |
| Halstead Effort | 56228.4368 | 46701.9854 | **39716.4346 ✅** | +9526.4514 | Higher | +16512.0022 | Higher | +6985.5508 | Higher |
| SLOC | 48 | 55 | **39 ✅** | -7 | Lower | +9 | Higher | +16 | Higher |
| PLOC | 34 | 34 | **30 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| LLOC | 25 | 23 | **22 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Comments (CLOC) | 8 | **13 ✅** | 8 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01184

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.0805 | **60.0965 ✅** | 34.1242 | -25.0160 | Lower | +0.9563 | Higher | +25.9723 | Higher |
| MI (SEI) | -11.3680 | **31.1069 ✅** | -14.3108 | -42.4748 | Lower | +2.9428 | Higher | +45.4177 | Higher |
| Cyclomatic (sum) | 35 | **16 ✅** | 38 | +19 | Higher | -3 | Lower | -22 | Lower |
| Cognitive (sum) | 59 | **26 ✅** | 63 | +33 | Higher | -4 | Lower | -37 | Lower |
| Halstead Volume | 5186.7467 | **1845.8009 ✅** | 5361.5919 | +3340.9459 | Higher | -174.8452 | Lower | -3515.7911 | Lower |
| Halstead Effort | 413805.1388 | **53297.4997 ✅** | 466194.8144 | +360507.6390 | Higher | -52389.6756 | Lower | -412897.3147 | Lower |
| SLOC | 172 | **67 ✅** | 173 | +105 | Higher | -1 | Lower | -106 | Lower |
| PLOC | 144 | **53 ✅** | 147 | +91 | Higher | -3 | Lower | -94 | Lower |
| LLOC | 103 | **33 ✅** | 106 | +70 | Higher | -3 | Lower | -73 | Lower |
| Comments (CLOC) | 3 | **4 ✅** | 2 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 26 | **10 ✅** | 24 | +16 | Higher | +2 | Higher | -14 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **1 ✅** | 9 | +6 | Higher | -2 | Lower | -8 | Lower |

## p01255

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.2201 | **63.3713 ✅** | 10.4452 | -26.1512 | Lower | +26.7750 | Higher | +52.9261 | Higher |
| MI (SEI) | -10.2566 | **40.2952 ✅** | -46.6831 | -50.5518 | Lower | +36.4265 | Higher | +86.9783 | Higher |
| Cyclomatic (sum) | 31 | **12 ✅** | 78 | +19 | Higher | -47 | Lower | -66 | Lower |
| Cognitive (sum) | 50 | **12 ✅** | 152 | +38 | Higher | -102 | Lower | -140 | Lower |
| Halstead Volume | 5040.2661 | **1572 ✅** | 11499.9728 | +3468.2661 | Higher | -6459.7067 | Lower | -9927.9728 | Lower |
| Halstead Effort | 272034.3616 | **37877.7143 ✅** | 1249749.8576 | +234156.6473 | Higher | -977715.4960 | Lower | -1211872.1433 | Lower |
| SLOC | 161 | **61 ✅** | 331 | +100 | Higher | -170 | Lower | -270 | Lower |
| PLOC | 138 | **46 ✅** | 301 | +92 | Higher | -163 | Lower | -255 | Lower |
| LLOC | 72 | **25 ✅** | 186 | +47 | Higher | -114 | Lower | -161 | Lower |
| Comments (CLOC) | 2 | **6 ✅** | 2 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 22 | **12 ✅** | 28 | +10 | Higher | -6 | Lower | -16 | Lower |
| Functions | 6 | **3 ✅** | 17 | +3 | Higher | -11 | Lower | -14 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **2 ✅** | 26 | +5 | Higher | -19 | Lower | -24 | Lower |

## p01275

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.4647 | **64.7050 ✅** | 48.2330 | -8.2403 | Lower | +8.2318 | Higher | +16.4720 | Higher |
| MI (SEI) | 27.0642 | **51.4544 ✅** | 18.9386 | -24.3902 | Lower | +8.1256 | Higher | +32.5158 | Higher |
| Cyclomatic (sum) | 18 | **10 ✅** | 24 | +8 | Higher | -6 | Lower | -14 | Lower |
| Cognitive (sum) | 17 | **10 ✅** | 20 | +7 | Higher | -3 | Lower | -10 | Lower |
| Halstead Volume | 2390.4204 | **1328.8775 ✅** | 2785.4211 | +1061.5428 | Higher | -395.0007 | Lower | -1456.5435 | Lower |
| Halstead Effort | 108465.3246 | **40691.1469 ✅** | 144777.1174 | +67774.1777 | Higher | -36311.7929 | Lower | -104085.9705 | Lower |
| SLOC | 75 | **61 ✅** | 109 | +14 | Higher | -34 | Lower | -48 | Lower |
| PLOC | 58 | **37 ✅** | 87 | +21 | Higher | -29 | Lower | -50 | Lower |
| LLOC | 37 | **27 ✅** | 56 | +10 | Higher | -19 | Lower | -29 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 10 | -8 | Lower | -5 | Lower | +3 | Higher |
| Blank Lines | **12 ✅** | 13 | 17 | -1 | Lower | -5 | Lower | -4 | Lower |
| Functions | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8594 | 60.4818 | **62.2394 ✅** | -0.6224 | Lower | -2.3800 | Lower | -1.7576 | Lower |
| MI (SEI) | 42.0386 | **45.8339 ✅** | 15.2116 | -3.7954 | Lower | +26.8269 | Higher | +30.6223 | Higher |
| Cyclomatic (sum) | 16 | 12 | **11 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Cognitive (sum) | 31 | **10 ✅** | 22 | +21 | Higher | +9 | Higher | -12 | Lower |
| Halstead Volume | 2123.2040 | **1866.5774 ✅** | 1941.7650 | +256.6265 | Higher | +181.4389 | Higher | -75.1876 | Lower |
| Halstead Effort | 101164.4243 | **65330.2100 ✅** | 92401.2327 | +35834.2143 | Higher | +8763.1916 | Higher | -27071.0227 | Lower |
| SLOC | 65 | 69 | **62 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |
| PLOC | 54 | **51 ✅** | 53 | +3 | Higher | +1 | Higher | -2 | Lower |
| LLOC | 47 | **33 ✅** | 50 | +14 | Higher | -3 | Lower | -17 | Lower |
| Comments (CLOC) | 11 | **15 ✅** | 0 | -4 | Lower | +11 | Higher | +15 | Higher |
| Blank Lines | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p01316

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.4027 | **68.4121 ✅** | 57.9479 | -10.0094 | Lower | +0.4549 | Higher | +10.4642 | Higher |
| MI (SEI) | 23.2314 | **63.8724 ✅** | 22.4857 | -40.6410 | Lower | +0.7456 | Higher | +41.3866 | Higher |
| Cyclomatic (sum) | 17 | **13 ✅** | 17 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 35 | **12 ✅** | 35 | +23 | Higher | 0 | Same | -23 | Lower |
| Halstead Volume | 2133.8889 | **1128.8126 ✅** | 2228.2761 | +1005.0763 | Higher | -94.3872 | Lower | -1099.4634 | Lower |
| Halstead Effort | 91251.8294 | **41245.0773 ✅** | 86739.7229 | +50006.7520 | Higher | +4512.1064 | Higher | -45494.6456 | Lower |
| SLOC | 70 | **49 ✅** | 71 | +21 | Higher | -1 | Lower | -22 | Lower |
| PLOC | 57 | **33 ✅** | 59 | +24 | Higher | -2 | Lower | -26 | Lower |
| LLOC | 35 | **20 ✅** | 37 | +15 | Higher | -2 | Lower | -17 | Lower |
| Comments (CLOC) | 2 | **17 ✅** | 2 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 13 | **6 ✅** | 12 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p01317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.5328 | **55.8504 ✅** | 50.0279 | -7.3176 | Lower | -1.4951 | Lower | +5.8225 | Higher |
| MI (SEI) | -3.1373 | **30.4507 ✅** | 9.5533 | -33.5880 | Lower | -12.6906 | Lower | +20.8974 | Higher |
| Cyclomatic (sum) | 25 | **14 ✅** | 19 | +11 | Higher | +6 | Higher | -5 | Lower |
| Cognitive (sum) | 47 | **18 ✅** | 41 | +29 | Higher | +6 | Higher | -23 | Lower |
| Halstead Volume | 4125.0075 | **2174.0748 ✅** | 3775.9447 | +1950.9327 | Higher | +349.0627 | Higher | -1601.8700 | Lower |
| Halstead Effort | 296771.3708 | **96435.7456 ✅** | 207165.6344 | +200335.6252 | Higher | +89605.7364 | Higher | -110729.8888 | Lower |
| SLOC | 93 | **85 ✅** | 95 | +8 | Higher | -2 | Lower | -10 | Lower |
| PLOC | 81 | **63 ✅** | 79 | +18 | Higher | +2 | Higher | -16 | Lower |
| LLOC | 61 | **26 ✅** | 68 | +35 | Higher | -7 | Lower | -42 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 2 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | **12 ✅** | 14 | 16 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01339

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.9315 | **69.1642 ✅** | 36.0074 | -32.2327 | Lower | +0.9241 | Higher | +33.1568 | Higher |
| MI (SEI) | -8.4006 | **61.8823 ✅** | -8.4136 | -70.2829 | Lower | +0.0130 | Higher | +70.2959 | Higher |
| Cyclomatic (sum) | 33 | **6 ✅** | 33 | +27 | Higher | 0 | Same | -27 | Lower |
| Cognitive (sum) | 34 | **0 ✅** | 31 | +34 | Higher | +3 | Higher | -31 | Lower |
| Halstead Volume | 5380.5333 | **787.2783 ✅** | 5499.7269 | +4593.2549 | Higher | -119.1937 | Lower | -4712.4486 | Lower |
| Halstead Effort | 349734.6632 | **12596.4535 ✅** | 422104.0428 | +337138.2097 | Higher | -72369.3796 | Lower | -409507.5893 | Lower |
| SLOC | 156 | **58 ✅** | 164 | +98 | Higher | -8 | Lower | -106 | Lower |
| PLOC | 135 | **33 ✅** | 146 | +102 | Higher | -11 | Lower | -113 | Lower |
| LLOC | 89 | **10 ✅** | 96 | +79 | Higher | -7 | Lower | -86 | Lower |
| Comments (CLOC) | 3 | **17 ✅** | 4 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | 19 | **11 ✅** | 16 | +8 | Higher | +3 | Higher | -5 | Lower |
| Functions | 8 | **4 ✅** | 9 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.9388 | **68.8258 ✅** | 56.2805 | -12.8870 | Lower | -0.3417 | Lower | +12.5452 | Higher |
| MI (SEI) | 33.8561 | **59.7506 ✅** | 31.2667 | -25.8945 | Lower | +2.5894 | Higher | +28.4838 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 12 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 12 | +11 | Higher | +2 | Higher | -9 | Lower |
| Halstead Volume | 2422.8102 | **1098.4583 ✅** | 2445.5839 | +1324.3519 | Higher | -22.7737 | Lower | -1347.1256 | Lower |
| Halstead Effort | 123805.6018 | **23685.5081 ✅** | 140934.6120 | +100120.0937 | Higher | -17129.0102 | Lower | -117249.1039 | Lower |
| SLOC | 84 | **51 ✅** | 82 | +33 | Higher | +2 | Higher | -31 | Lower |
| PLOC | 67 | **31 ✅** | 68 | +36 | Higher | -1 | Lower | -37 | Lower |
| LLOC | 50 | **17 ✅** | 54 | +33 | Higher | -4 | Lower | -37 | Lower |
| Comments (CLOC) | 12 | **13 ✅** | 9 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 12 | **9 ✅** | 12 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |

## p01368

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.5652 | **55.5673 ✅** | 46.4446 | -9.0020 | Lower | +0.1206 | Higher | +9.1226 | Higher |
| MI (SEI) | 14.7691 | **42.3832 ✅** | 4.4149 | -27.6141 | Lower | +10.3542 | Higher | +37.9683 | Higher |
| Cyclomatic (sum) | 26 | **13 ✅** | 26 | +13 | Higher | 0 | Same | -13 | Lower |
| Cognitive (sum) | 61 | **16 ✅** | 45 | +45 | Higher | +16 | Higher | -29 | Lower |
| Halstead Volume | 3832.8158 | **2079.2716 ✅** | 3700.8342 | +1753.5442 | Higher | +131.9816 | Higher | -1621.5626 | Lower |
| Halstead Effort | 242035.2187 | **95088.6392 ✅** | 335106.0241 | +146946.5795 | Higher | -93070.8054 | Lower | -240017.3849 | Lower |
| SLOC | 106 | **89 ✅** | 108 | +17 | Higher | -2 | Lower | -19 | Lower |
| PLOC | 86 | **59 ✅** | 96 | +27 | Higher | -10 | Lower | -37 | Lower |
| LLOC | 67 | **32 ✅** | 78 | +35 | Higher | -11 | Lower | -46 | Lower |
| Comments (CLOC) | 8 | **25 ✅** | 2 | -17 | Lower | +6 | Higher | +23 | Higher |
| Blank Lines | 14 | 13 | **10 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01452

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.5523 | **65.4634 ✅** | 55.5591 | -13.9111 | Lower | -4.0067 | Lower | +9.9044 | Higher |
| MI (SEI) | 18.8554 | **55.8518 ✅** | 21.2265 | -36.9963 | Lower | -2.3711 | Lower | +34.6253 | Higher |
| Cyclomatic (sum) | 20 | **14 ✅** | 15 | +6 | Higher | +5 | Higher | -1 | Lower |
| Cognitive (sum) | 12 | 8 | **7 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Halstead Volume | 3544.1978 | **1188.6820 ✅** | 3248.9223 | +2355.5158 | Higher | +295.2755 | Higher | -2060.2403 | Lower |
| Halstead Effort | 209329.1846 | **52715.4632 ✅** | 183834.8539 | +156613.7214 | Higher | +25494.3308 | Higher | -131119.3906 | Lower |
| SLOC | 87 | **57 ✅** | 75 | +30 | Higher | +12 | Higher | -18 | Lower |
| PLOC | 74 | **40 ✅** | 61 | +34 | Higher | +13 | Higher | -21 | Lower |
| LLOC | 39 | **27 ✅** | 33 | +12 | Higher | +6 | Higher | -6 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 3 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 13 | **8 ✅** | 14 | +5 | Higher | -1 | Lower | -6 | Lower |
| Functions | 7 | **4 ✅** | 6 | +3 | Higher | +1 | Higher | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **5 ✅** | 6 | +2 | Higher | +1 | Higher | -1 | Lower |

## p01459

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.5072 | **51.7166 ✅** | 40.9605 | -10.2094 | Lower | +0.5467 | Higher | +10.7561 | Higher |
| MI (SEI) | -3.3083 | **28.2874 ✅** | -4.0695 | -31.5957 | Lower | +0.7611 | Higher | +32.3569 | Higher |
| Cyclomatic (sum) | 28 | **24 ✅** | 29 | +4 | Higher | -1 | Lower | -5 | Lower |
| Cognitive (sum) | 47 | **38 ✅** | 55 | +9 | Higher | -8 | Lower | -17 | Lower |
| Halstead Volume | 5284.3292 | **2680.4975 ✅** | 5349.3208 | +2603.8317 | Higher | -64.9916 | Lower | -2668.8233 | Lower |
| Halstead Effort | 331054.9706 | **148682.0641 ✅** | 377976.2152 | +182372.9065 | Higher | -46921.2446 | Lower | -229294.1511 | Lower |
| SLOC | 127 | **89 ✅** | 129 | +38 | Higher | -2 | Lower | -40 | Lower |
| PLOC | 106 | **69 ✅** | 108 | +37 | Higher | -2 | Lower | -39 | Lower |
| LLOC | 63 | **42 ✅** | 86 | +21 | Higher | -23 | Lower | -44 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 19 | **13 ✅** | 19 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 6 | 1 | -5 | Lower | 0 | Same | +5 | Higher |

## p01471

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.1950 | **61.0049 ✅** | 56.0668 | -1.8099 | Lower | +3.1282 | Higher | +4.9381 | Higher |
| MI (SEI) | 34.7535 | **44.2221 ✅** | 23.5960 | -9.4685 | Lower | +11.1575 | Higher | +20.6261 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 17 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 25 | **9 ✅** | 19 | +16 | Higher | +6 | Higher | -10 | Lower |
| Halstead Volume | 2001.8094 | **1414.3882 ✅** | 1966.9449 | +587.4212 | Higher | +34.8646 | Higher | -552.5567 | Lower |
| Halstead Effort | 69587.9004 | **43771.5935 ✅** | 84136.0670 | +25816.3070 | Higher | -14548.1666 | Lower | -40364.4736 | Lower |
| SLOC | **70 ✅** | 72 | 83 | -2 | Lower | -13 | Lower | -11 | Lower |
| PLOC | 60 | **54 ✅** | 70 | +6 | Higher | -10 | Lower | -16 | Lower |
| LLOC | 36 | **25 ✅** | 43 | +11 | Higher | -7 | Lower | -18 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 4 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | **9 ✅** | 11 | 11 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | **1 ✅** | 3 | 4 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 4 | 3 | -4 | Lower | -3 | Lower | +1 | Higher |

## p01521

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.4134 ✅** | 72.6032 | 80.6528 | +11.8102 | Higher | +3.7606 | Higher | -8.0495 | Lower |
| MI (SEI) | 74.7715 | 68.0478 | **75.2883 ✅** | +6.7236 | Higher | -0.5168 | Lower | -7.2404 | Lower |
| Cyclomatic (sum) | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **453.9955 ✅** | 641.6858 | 673.8746 | -187.6903 | Lower | -219.8791 | Lower | -32.1888 | Lower |
| Halstead Effort | **5864.1082 ✅** | 10866.8091 | 12736.2292 | -5002.7009 | Lower | -6872.1210 | Lower | -1869.4201 | Lower |
| SLOC | **27 ✅** | 48 | 30 | -21 | Lower | -3 | Lower | +18 | Higher |
| PLOC | **19 ✅** | 28 | 21 | -9 | Lower | -2 | Lower | +7 | Higher |
| LLOC | **13 ✅** | 17 | 14 | -4 | Lower | -1 | Lower | +3 | Higher |
| Comments (CLOC) | 4 | **15 ✅** | 7 | -11 | Lower | -3 | Lower | +8 | Higher |
| Blank Lines | **4 ✅** | 9 | 5 | -5 | Lower | -1 | Lower | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01529

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.3499 | **73.5688 ✅** | 61.8336 | -13.2189 | Lower | -1.4837 | Lower | +11.7352 | Higher |
| MI (SEI) | 35.3639 | **64.7747 ✅** | 43.9722 | -29.4108 | Lower | -8.6083 | Lower | +20.8025 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 11 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 21 | **12 ✅** | 17 | +9 | Higher | +4 | Higher | -5 | Lower |
| Halstead Volume | 2206.2343 | **983.0417 ✅** | 2208.4588 | +1223.1926 | Higher | -2.2245 | Lower | -1225.4171 | Lower |
| Halstead Effort | 127020.6948 | **37511.9778 ✅** | 130989.2147 | +89508.7170 | Higher | -3968.5199 | Lower | -93477.2369 | Lower |
| SLOC | 65 | **40 ✅** | 61 | +25 | Higher | +4 | Higher | -21 | Lower |
| PLOC | 52 | **27 ✅** | 48 | +25 | Higher | +4 | Higher | -21 | Lower |
| LLOC | 42 | **19 ✅** | 43 | +23 | Higher | -1 | Lower | -24 | Lower |
| Comments (CLOC) | 6 | 9 | **10 ✅** | -3 | Lower | -4 | Lower | -1 | Lower |
| Blank Lines | 11 | **5 ✅** | 11 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |

## p01545

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.5019 | **66.7466 ✅** | 55.0872 | -9.2447 | Lower | +2.4146 | Higher | +11.6593 | Higher |
| MI (SEI) | 24.3333 | **58.3585 ✅** | 22.2802 | -34.0252 | Lower | +2.0530 | Higher | +36.0782 | Higher |
| Cyclomatic (sum) | 15 | **8 ✅** | 17 | +7 | Higher | -2 | Lower | -9 | Lower |
| Cognitive (sum) | 11 | **2 ✅** | 12 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 2539.3025 | **1087.6737 ✅** | 2466.0486 | +1451.6288 | Higher | +73.2539 | Higher | -1378.3749 | Lower |
| Halstead Effort | 151915.2482 | **23554.9344 ✅** | 152051.3647 | +128360.3138 | Higher | -136.1165 | Lower | -128496.4303 | Lower |
| SLOC | 72 | **59 ✅** | 82 | +13 | Higher | -10 | Lower | -23 | Lower |
| PLOC | 60 | **35 ✅** | 67 | +25 | Higher | -7 | Lower | -32 | Lower |
| LLOC | 41 | **15 ✅** | 43 | +26 | Higher | -2 | Lower | -28 | Lower |
| Comments (CLOC) | 3 | **17 ✅** | 4 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | **10 ✅** | 11 | 11 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01570

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.3028 | **57.2705 ✅** | 46.4702 | -6.9677 | Lower | +3.8326 | Higher | +10.8003 | Higher |
| MI (SEI) | 6.6377 | **42.5435 ✅** | 3.5220 | -35.9058 | Lower | +3.1157 | Higher | +39.0215 | Higher |
| Cyclomatic (sum) | 19 | **15 ✅** | 20 | +4 | Higher | -1 | Lower | -5 | Lower |
| Cognitive (sum) | 23 | **15 ✅** | 24 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 3356.4392 | **1839.4060 ✅** | 3948.6902 | +1517.0332 | Higher | -592.2510 | Lower | -2109.2842 | Lower |
| Halstead Effort | 198248.8111 | **74495.9443 ✅** | 276408.3132 | +123752.8667 | Higher | -78159.5021 | Lower | -201912.3689 | Lower |
| SLOC | 97 | **81 ✅** | 115 | +16 | Higher | -18 | Lower | -34 | Lower |
| PLOC | 84 | **50 ✅** | 98 | +34 | Higher | -14 | Lower | -48 | Lower |
| LLOC | 68 | **30 ✅** | 91 | +38 | Higher | -23 | Lower | -61 | Lower |
| Comments (CLOC) | 1 | **19 ✅** | 2 | -18 | Lower | -1 | Lower | +17 | Higher |
| Blank Lines | **13 ✅** | 15 | 15 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p01600

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.5340 | **65.6080 ✅** | 64.2283 | -0.0740 | Lower | +1.3057 | Higher | +1.3797 | Higher |
| MI (SEI) | 52.1453 | **55.4112 ✅** | 32.9720 | -3.2660 | Lower | +19.1733 | Higher | +22.4393 | Higher |
| Cyclomatic (sum) | 11 | **10 ✅** | 13 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 14 | **12 ✅** | 21 | +2 | Higher | -7 | Lower | -9 | Lower |
| Halstead Volume | 1584.7222 | **1307.0610 ✅** | 1864.5836 | +277.6612 | Higher | -279.8613 | Lower | -557.5225 | Lower |
| Halstead Effort | **61592.8711 ✅** | 89206.9158 | 95640.2770 | -27614.0446 | Lower | -34047.4058 | Lower | -6433.3612 | Lower |
| SLOC | 54 | 58 | 54 | -4 | Lower | 0 | Same | +4 | Higher |
| PLOC | 38 | **33 ✅** | 43 | +5 | Higher | -5 | Lower | -10 | Lower |
| LLOC | 26 | **24 ✅** | 30 | +2 | Higher | -4 | Lower | -6 | Lower |
| Comments (CLOC) | 11 | **15 ✅** | 2 | -4 | Lower | +9 | Higher | +13 | Higher |
| Blank Lines | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p01601

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.9920 | 73.1924 | **76.1566 ✅** | -2.2004 | Lower | -5.1647 | Lower | -2.9643 | Lower |
| MI (SEI) | 39.1817 | **68.0093 ✅** | 47.7515 | -28.8275 | Lower | -8.5697 | Lower | +20.2578 | Higher |
| Cyclomatic (sum) | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 1023.3007 | **785.3350 ✅** | 829.8336 | +237.9657 | Higher | +193.4671 | Higher | -44.4986 | Lower |
| Halstead Effort | 34489.0226 | **16148.4508 ✅** | 30572.8171 | +18340.5718 | Higher | +3916.2055 | Higher | -14424.3663 | Lower |
| SLOC | 45 | 44 | **36 ✅** | +1 | Higher | +9 | Higher | +8 | Higher |
| PLOC | 36 | **29 ✅** | 30 | +7 | Higher | +6 | Higher | -1 | Lower |
| LLOC | 26 | **19 ✅** | 22 | +7 | Higher | +4 | Higher | -3 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 1 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 9 | 7 | **6 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 4 | **1 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |

## p01656

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.4575 ✅** | 73.3656 | 75.9643 | +5.0919 | Higher | +2.4932 | Higher | -2.5987 | Lower |
| MI (SEI) | 66.2535 | **68.8099 ✅** | 66.2353 | -2.5564 | Lower | +0.0182 | Higher | +2.5746 | Higher |
| Cyclomatic (sum) | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 3 | 7 | 3 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | **834.4205 ✅** | 917.7708 | 1122.0269 | -83.3503 | Lower | -287.6065 | Lower | -204.2562 | Lower |
| Halstead Effort | **13654.1532 ✅** | 19517.9256 | 19074.4581 | -5863.7723 | Lower | -5420.3049 | Lower | +443.4674 | Higher |
| SLOC | **33 ✅** | 42 | 35 | -9 | Lower | -2 | Lower | +7 | Higher |
| PLOC | **22 ✅** | 29 | 24 | -7 | Lower | -2 | Lower | +5 | Higher |
| LLOC | 18 | **16 ✅** | 20 | +2 | Higher | -2 | Lower | -4 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 7 | -8 | Lower | -2 | Lower | +6 | Higher |
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
| MI (original) | 39.4229 | **59.4884 ✅** | 43.9065 | -20.0655 | Lower | -4.4836 | Lower | +15.5819 | Higher |
| MI (SEI) | -14.9565 | **43.6413 ✅** | -8.8952 | -58.5978 | Lower | -6.0612 | Lower | +52.5366 | Higher |
| Cyclomatic (sum) | 38 | **12 ✅** | 34 | +26 | Higher | +4 | Higher | -22 | Lower |
| Cognitive (sum) | 37 | **13 ✅** | 34 | +24 | Higher | +3 | Higher | -21 | Lower |
| Halstead Volume | 4828.6961 | **1895.6893 ✅** | 5057.5425 | +2933.0068 | Higher | -228.8464 | Lower | -3161.8532 | Lower |
| Halstead Effort | 373885.9354 | **82031.6441 ✅** | 429940.6942 | +291854.2913 | Higher | -56054.7588 | Lower | -347909.0501 | Lower |
| SLOC | 129 | **73 ✅** | 102 | +56 | Higher | +27 | Higher | -29 | Lower |
| PLOC | 109 | **47 ✅** | 84 | +62 | Higher | +25 | Higher | -37 | Lower |
| LLOC | 64 | **26 ✅** | 68 | +38 | Higher | -4 | Lower | -42 | Lower |
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
| MI (original) | 58.0175 | **67.5743 ✅** | 60.5639 | -9.5568 | Lower | -2.5464 | Lower | +7.0104 | Higher |
| MI (SEI) | 33.5570 | **65.6015 ✅** | 22.5468 | -32.0445 | Lower | +11.0102 | Higher | +43.0547 | Higher |
| Cyclomatic (sum) | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 2212.2482 | **1010.9352 ✅** | 2117.2732 | +1201.3131 | Higher | +94.9750 | Higher | -1106.3381 | Lower |
| Halstead Effort | 92659.1670 | **35612.4890 ✅** | 98359.7961 | +57046.6780 | Higher | -5700.6291 | Lower | -62747.3071 | Lower |
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
| MI (original) | **53.4283 ✅** | 52.4862 | 45.3856 | +0.9421 | Higher | +8.0426 | Higher | +7.1005 | Higher |
| MI (SEI) | 11.2754 | **22.6747 ✅** | 5.2845 | -11.3992 | Lower | +5.9909 | Higher | +17.3901 | Higher |
| Cyclomatic (sum) | **16 ✅** | 17 | 20 | -1 | Lower | -4 | Lower | -3 | Lower |
| Cognitive (sum) | **7 ✅** | 11 | 22 | -4 | Lower | -15 | Lower | -11 | Lower |
| Halstead Volume | 2949.0218 | **1998.6585 ✅** | 3400.9350 | +950.3633 | Higher | -451.9132 | Lower | -1402.2765 | Lower |
| Halstead Effort | 88907.5457 | **57186.6158 ✅** | 154742.5427 | +31720.9299 | Higher | -65834.9970 | Lower | -97555.9269 | Lower |
| SLOC | **87 ✅** | 103 | 129 | -16 | Lower | -42 | Lower | -26 | Lower |
| PLOC | **70 ✅** | 78 | 111 | -8 | Lower | -41 | Lower | -33 | Lower |
| LLOC | 35 | **28 ✅** | 65 | +7 | Higher | -30 | Lower | -37 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 4 | -7 | Lower | -3 | Lower | +4 | Higher |
| Blank Lines | 17 | 17 | 18 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p01782

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 26.5660 | **61.1428 ✅** | 24.6136 | -34.5768 | Lower | +1.9524 | Higher | +36.5293 | Higher |
| MI (SEI) | -25.5520 | **46.9842 ✅** | -28.9066 | -72.5362 | Lower | +3.3546 | Higher | +75.8908 | Higher |
| Cyclomatic (sum) | 44 | **11 ✅** | 43 | +33 | Higher | +1 | Higher | -32 | Lower |
| Cognitive (sum) | 51 | **6 ✅** | 51 | +45 | Higher | 0 | Same | -45 | Lower |
| Halstead Volume | 8203.0394 | **1504.7422 ✅** | 8500.1152 | +6698.2972 | Higher | -297.0758 | Lower | -6995.3729 | Lower |
| Halstead Effort | 753330.4408 | **34784.6247 ✅** | 756302.9309 | +718545.8161 | Higher | -2972.4901 | Lower | -721518.3062 | Lower |
| SLOC | 221 | **72 ✅** | 250 | +149 | Higher | -29 | Lower | -178 | Lower |
| PLOC | 191 | **45 ✅** | 219 | +146 | Higher | -28 | Lower | -174 | Lower |
| LLOC | 125 | **20 ✅** | 150 | +105 | Higher | -25 | Lower | -130 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 2 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 28 | **13 ✅** | 29 | +15 | Higher | -1 | Lower | -16 | Lower |
| Functions | 13 | **4 ✅** | 13 | +9 | Higher | 0 | Same | -9 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |

## p01841

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.5035 | **50.6944 ✅** | 46.2073 | -3.1909 | Lower | +1.2962 | Higher | +4.4870 | Higher |
| MI (SEI) | 2.3991 | -0.0189 | **8.8707 ✅** | +2.4180 | Higher | -6.4715 | Lower | -8.8895 | Lower |
| Cyclomatic (sum) | 20 | 25 | 20 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | **15 ✅** | 30 | 16 | -15 | Lower | -1 | Lower | +14 | Higher |
| Halstead Volume | 4297.7052 | **2547.4616 ✅** | 4153.4145 | +1750.2436 | Higher | +144.2906 | Higher | -1605.9529 | Lower |
| Halstead Effort | 281013.1550 | **87324.6134 ✅** | 223869.0432 | +193688.5416 | Higher | +57144.1118 | Higher | -136544.4298 | Lower |
| SLOC | 105 | **95 ✅** | 115 | +10 | Higher | -10 | Lower | -20 | Lower |
| PLOC | 89 | **83 ✅** | 94 | +6 | Higher | -5 | Lower | -11 | Lower |
| LLOC | 79 | **41 ✅** | 76 | +38 | Higher | +3 | Higher | -35 | Lower |
| Comments (CLOC) | 1 | 0 | **5 ✅** | +1 | Higher | -4 | Lower | -5 | Lower |
| Blank Lines | 16 | **12 ✅** | 16 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p01847

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.7712 | 51.8545 | **56.7897 ✅** | -1.0833 | Lower | -6.0185 | Lower | -4.9352 | Lower |
| MI (SEI) | 8.4136 | **29.9159 ✅** | 20.5397 | -21.5023 | Lower | -12.1260 | Lower | +9.3763 | Higher |
| Cyclomatic (sum) | 26 | 19 | **16 ✅** | +7 | Higher | +10 | Higher | +3 | Higher |
| Cognitive (sum) | 62 | **22 ✅** | 36 | +40 | Higher | +26 | Higher | -14 | Lower |
| Halstead Volume | 3048.1537 | **2657.4567 ✅** | 2668.8318 | +390.6970 | Higher | +379.3219 | Higher | -11.3751 | Lower |
| Halstead Effort | 192262.2949 | **104254.0720 ✅** | 143635.0449 | +88008.2228 | Higher | +48627.2500 | Higher | -39380.9729 | Lower |
| SLOC | 88 | 95 | **73 ✅** | -7 | Lower | +15 | Higher | +22 | Higher |
| PLOC | 71 | 73 | **59 ✅** | -2 | Lower | +12 | Higher | +14 | Higher |
| LLOC | 72 | **41 ✅** | 62 | +31 | Higher | +10 | Higher | -21 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 2 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | 17 | 12 | 12 | +5 | Higher | +5 | Higher | 0 | Same |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 6 | 2 | -5 | Lower | -1 | Lower | +4 | Higher |

## p01907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.5830 | **60.9797 ✅** | 39.5047 | -11.3968 | Lower | +10.0782 | Higher | +21.4750 | Higher |
| MI (SEI) | 10.3883 | **41.6829 ✅** | -1.4535 | -31.2945 | Lower | +11.8418 | Higher | +43.1364 | Higher |
| Cyclomatic (sum) | 17 | **11 ✅** | 23 | +6 | Higher | -6 | Lower | -12 | Lower |
| Cognitive (sum) | 15 | **5 ✅** | 25 | +10 | Higher | -10 | Lower | -20 | Lower |
| Halstead Volume | 3102.1452 | **1487.3870 ✅** | 5004.4941 | +1614.7583 | Higher | -1902.3489 | Lower | -3517.1071 | Lower |
| Halstead Effort | 124914.8301 | **37742.4439 ✅** | 335906.4856 | +87172.3861 | Higher | -210991.6555 | Lower | -298164.0416 | Lower |
| SLOC | 107 | **73 ✅** | 157 | +34 | Higher | -50 | Lower | -84 | Lower |
| PLOC | 88 | **56 ✅** | 130 | +32 | Higher | -42 | Lower | -74 | Lower |
| LLOC | 49 | **20 ✅** | 119 | +29 | Higher | -70 | Lower | -99 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 6 | -8 | Lower | -3 | Lower | +5 | Higher |
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
| MI (original) | 54.0279 | **69.5614 ✅** | 53.9865 | -15.5335 | Lower | +0.0413 | Higher | +15.5748 | Higher |
| MI (SEI) | 4.4849 | **56.1614 ✅** | 13.1510 | -51.6765 | Lower | -8.6660 | Lower | +43.0104 | Higher |
| Cyclomatic (sum) | 22 | **11 ✅** | 22 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 47 | **7 ✅** | 47 | +40 | Higher | 0 | Same | -40 | Lower |
| Halstead Volume | 2832 | **1289.0687 ✅** | 2854.6023 | +1542.9313 | Higher | -22.6023 | Lower | -1565.5337 | Lower |
| Halstead Effort | 175360.4211 | **44867.9071 ✅** | 174130.7429 | +130492.5140 | Higher | +1229.6781 | Higher | -129262.8359 | Lower |
| SLOC | 78 | **45 ✅** | 78 | +33 | Higher | 0 | Same | -33 | Lower |
| PLOC | 70 | **30 ✅** | 70 | +40 | Higher | 0 | Same | -40 | Lower |
| LLOC | 56 | **14 ✅** | 56 | +42 | Higher | 0 | Same | -42 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 1 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.0903 | **61.7287 ✅** | 57.5384 | -6.6384 | Lower | -2.4482 | Lower | +4.1903 | Higher |
| MI (SEI) | 5.5086 | 33.0543 | **33.2773 ✅** | -27.5458 | Lower | -27.7687 | Lower | -0.2230 | Lower |
| Cyclomatic (sum) | 17 | **12 ✅** | 16 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 3122.8684 | **1609.5456 ✅** | 3317.6672 | +1513.3228 | Higher | -194.7988 | Lower | -1708.1216 | Lower |
| Halstead Effort | 256412.8193 | **95272.7212 ✅** | 309648.9432 | +161140.0981 | Higher | -53236.1239 | Lower | -214376.2220 | Lower |
| SLOC | 76 | 67 | **65 ✅** | +9 | Higher | +11 | Higher | +2 | Higher |
| PLOC | 62 | **51 ✅** | 56 | +11 | Higher | +6 | Higher | -5 | Lower |
| LLOC | 42 | **41 ✅** | 44 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | 0 | 4 | **7 ✅** | -4 | Lower | -7 | Lower | -3 | Lower |
| Blank Lines | 14 | 16 | **9 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Functions | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.5621 | 72.1276 | **72.2262 ✅** | -4.5654 | Lower | -4.6641 | Lower | -0.0986 | Lower |
| MI (SEI) | 49.8877 | **59.0336 ✅** | 51.7514 | -9.1459 | Lower | -1.8637 | Lower | +7.2822 | Higher |
| Cyclomatic (sum) | 18 | **11 ✅** | 16 | +7 | Higher | +2 | Higher | -5 | Lower |
| Cognitive (sum) | 10 | 8 | **7 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Halstead Volume | 1213.2411 | **975.6572 ✅** | 1240.4470 | +237.5840 | Higher | -27.2059 | Lower | -264.7899 | Lower |
| Halstead Effort | 38667.1687 | **23220.6404 ✅** | 28726.1416 | +15446.5283 | Higher | +9941.0271 | Higher | -5505.5013 | Lower |
| SLOC | 47 | 42 | **36 ✅** | +5 | Higher | +11 | Higher | +6 | Higher |
| PLOC | 32 | **28 ✅** | 29 | +4 | Higher | +3 | Higher | -1 | Lower |
| LLOC | 19 | 18 | **17 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Comments (CLOC) | 6 | **7 ✅** | 3 | -1 | Lower | +3 | Higher | +4 | Higher |
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
| MI (original) | 42.2160 | **50.2920 ✅** | 21.8095 | -8.0760 | Lower | +20.4065 | Higher | +28.4825 | Higher |
| MI (SEI) | -2.6000 | **21.0969 ✅** | -39.9600 | -23.6970 | Lower | +37.3599 | Higher | +61.0569 | Higher |
| Cyclomatic (sum) | 26 | **20 ✅** | 42 | +6 | Higher | -16 | Lower | -22 | Lower |
| Cognitive (sum) | 14 | **11 ✅** | 40 | +3 | Higher | -26 | Lower | -29 | Lower |
| Halstead Volume | 4684.0199 | **2751.4949 ✅** | 11714.2585 | +1932.5250 | Higher | -7030.2385 | Lower | -8962.7636 | Lower |
| Halstead Effort | 271673.1554 | **96252.2944 ✅** | 1552432.1037 | +175420.8611 | Higher | -1280758.9483 | Lower | -1456179.8094 | Lower |
| SLOC | 130 | **102 ✅** | 272 | +28 | Higher | -142 | Lower | -170 | Lower |
| PLOC | 111 | **81 ✅** | 252 | +30 | Higher | -141 | Lower | -171 | Lower |
| LLOC | 62 | **33 ✅** | 206 | +29 | Higher | -144 | Lower | -173 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 0 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 19 | **18 ✅** | 20 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 6 | 6 | 11 | 0 | Same | -5 | Lower | -5 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | **4 ✅** | 7 | 6 | -3 | Lower | -2 | Lower | +1 | Higher |

## p02061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.3802 ✅** | 54.5253 | 56.9126 | +3.8548 | Higher | +1.4676 | Higher | -2.3872 | Lower |
| MI (SEI) | 10.0512 | **37.9088 ✅** | 16.7155 | -27.8576 | Lower | -6.6643 | Lower | +21.1933 | Higher |
| Cyclomatic (sum) | 15 | 20 | 15 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 25 | 25 | 25 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 2240.1863 | **1625.4956 ✅** | 2307.2301 | +614.6907 | Higher | -67.0437 | Lower | -681.7345 | Lower |
| Halstead Effort | 130445.4443 | **62697.6871 ✅** | 152067.4351 | +67747.7571 | Higher | -21621.9909 | Lower | -89369.7480 | Lower |
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
| MI (original) | 62.3606 | **84.6799 ✅** | 60.1882 | -22.3193 | Lower | +2.1724 | Higher | +24.4917 | Higher |
| MI (SEI) | 47.5289 | **82.8749 ✅** | 28.5420 | -35.3459 | Lower | +18.9869 | Higher | +54.3328 | Higher |
| Cyclomatic (sum) | 11 | **4 ✅** | 14 | +7 | Higher | -3 | Lower | -10 | Lower |
| Cognitive (sum) | 14 | **2 ✅** | 17 | +12 | Higher | -3 | Lower | -15 | Lower |
| Halstead Volume | 1718.3761 | **530 ✅** | 1891.9052 | +1188.3761 | Higher | -173.5291 | Lower | -1361.9052 | Lower |
| Halstead Effort | 88220.2034 | **7626.1111 ✅** | 98406.8931 | +80594.0923 | Higher | -10186.6897 | Lower | -90780.7820 | Lower |
| SLOC | 64 | **26 ✅** | 68 | +38 | Higher | -4 | Lower | -42 | Lower |
| PLOC | 44 | **16 ✅** | 54 | +28 | Higher | -10 | Lower | -38 | Lower |
| LLOC | 41 | **12 ✅** | 48 | +29 | Higher | -7 | Lower | -36 | Lower |
| Comments (CLOC) | **13 ✅** | 7 | 3 | +6 | Higher | +10 | Higher | +4 | Higher |
| Blank Lines | 11 | **4 ✅** | 12 | +7 | Higher | -1 | Lower | -8 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.3249 | 63.5674 | **64.0569 ✅** | -0.2425 | Lower | -0.7320 | Lower | -0.4895 | Lower |
| MI (SEI) | 51.5206 | 49.3685 | **51.6827 ✅** | +2.1521 | Higher | -0.1621 | Lower | -2.3142 | Lower |
| Cyclomatic (sum) | 11 | 14 | **10 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 22 | **14 ✅** | 21 | +8 | Higher | +1 | Higher | -7 | Lower |
| Halstead Volume | **1657.8086 ✅** | 1913.1254 | 1964.8028 | -255.3168 | Lower | -306.9942 | Lower | -51.6773 | Lower |
| Halstead Effort | **54470.8547 ✅** | 107613.3055 | 87574.0667 | -53142.4508 | Lower | -33103.2120 | Lower | +20039.2388 | Higher |
| SLOC | 61 | **55 ✅** | 56 | +6 | Higher | +5 | Higher | -1 | Lower |
| PLOC | 40 | **39 ✅** | 48 | +1 | Higher | -8 | Lower | -9 | Lower |
| LLOC | 31 | **21 ✅** | 46 | +10 | Higher | -15 | Lower | -25 | Lower |
| Comments (CLOC) | **15 ✅** | 11 | 13 | +4 | Higher | +2 | Higher | -2 | Lower |
| Blank Lines | 12 | **7 ✅** | 8 | +5 | Higher | +4 | Higher | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.4674 | **37.5063 ✅** | 28.3623 | -0.0389 | Lower | +9.1050 | Higher | +9.1440 | Higher |
| MI (SEI) | -11.3748 | **-11.0131 ✅** | -20.6689 | -0.3617 | Lower | +9.2942 | Higher | +9.6558 | Higher |
| Cyclomatic (sum) | **37 ✅** | 40 | 46 | -3 | Lower | -9 | Lower | -6 | Lower |
| Cognitive (sum) | 42 | **32 ✅** | 50 | +10 | Higher | -8 | Lower | -18 | Lower |
| Halstead Volume | 5572.0862 | **4843.2543 ✅** | 7254.7492 | +728.8319 | Higher | -1682.6630 | Lower | -2411.4948 | Lower |
| Halstead Effort | 344129.2232 | **260790.6183 ✅** | 685225.9668 | +83338.6049 | Higher | -341096.7437 | Lower | -424435.3486 | Lower |
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
| MI (original) | 58.8070 | **67.4368 ✅** | 66.6222 | -8.6297 | Lower | -7.8152 | Lower | +0.8146 | Higher |
| MI (SEI) | 39.1141 | **57.0491 ✅** | 21.7383 | -17.9351 | Lower | +17.3758 | Higher | +35.3108 | Higher |
| Cyclomatic (sum) | 23 | 13 | 13 | +10 | Higher | +10 | Higher | 0 | Same |
| Cognitive (sum) | 20 | 12 | **11 ✅** | +8 | Higher | +9 | Higher | +1 | Higher |
| Halstead Volume | 2102.3441 | **1202.1414 ✅** | 1813.4154 | +900.2027 | Higher | +288.9287 | Higher | -611.2740 | Lower |
| Halstead Effort | 107994.0956 | **33844.9031 ✅** | 81717.0320 | +74149.1925 | Higher | +26277.0636 | Higher | -47872.1289 | Lower |
| SLOC | 63 | 51 | **47 ✅** | +12 | Higher | +16 | Higher | +4 | Higher |
| PLOC | 54 | **35 ✅** | 41 | +19 | Higher | +13 | Higher | -6 | Lower |
| LLOC | 34 | **21 ✅** | 33 | +13 | Higher | +1 | Higher | -12 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 0 | -3 | Lower | +9 | Higher | +12 | Higher |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 5 | 2 | **0 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |

## p02186

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.3419 | **57.9583 ✅** | 37.7791 | -19.6164 | Lower | +0.5628 | Higher | +20.1792 | Higher |
| MI (SEI) | -18.0433 | **39.4714 ✅** | -18.6516 | -57.5147 | Lower | +0.6083 | Higher | +58.1230 | Higher |
| Cyclomatic (sum) | 23 | **13 ✅** | 25 | +10 | Higher | -2 | Lower | -12 | Lower |
| Cognitive (sum) | 12 | **9 ✅** | 14 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | 6646.3531 | **2333.1361 ✅** | 7207.4913 | +4313.2170 | Higher | -561.1382 | Lower | -4874.3552 | Lower |
| Halstead Effort | 577921.1715 | **141932.4479 ✅** | 725635.5657 | +435988.7236 | Higher | -147714.3941 | Lower | -583703.1177 | Lower |
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
| MI (original) | 80.4501 | **82.1176 ✅** | 74.4064 | -1.6674 | Lower | +6.0438 | Higher | +7.7112 | Higher |
| MI (SEI) | 69.9115 | **75.5660 ✅** | 62.1204 | -5.6545 | Lower | +7.7911 | Higher | +13.4455 | Higher |
| Cyclomatic (sum) | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 691.1242 | **617.3437 ✅** | 846.4973 | +73.7805 | Higher | -155.3731 | Lower | -229.1536 | Lower |
| Halstead Effort | 10817.5963 | **6814.5242 ✅** | 17758.8080 | +4003.0721 | Higher | -6941.2117 | Lower | -10944.2837 | Lower |
| SLOC | 31 | **29 ✅** | 41 | +2 | Higher | -10 | Lower | -12 | Lower |
| PLOC | 19 | **17 ✅** | 25 | +2 | Higher | -6 | Lower | -8 | Lower |
| LLOC | 13 | **11 ✅** | 18 | +2 | Higher | -5 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | 6 | **7 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02205

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.3592 | **82.6249 ✅** | 80.8524 | -4.2656 | Lower | -2.4932 | Lower | +1.7724 | Higher |
| MI (SEI) | 76.3292 | **89.5288 ✅** | 68.1516 | -13.1995 | Lower | +8.1777 | Higher | +21.3772 | Higher |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 774.8300 | **358.2030 ✅** | 708.4702 | +416.6269 | Higher | +66.3597 | Higher | -350.2672 | Lower |
| Halstead Effort | 13656.3780 | **5117.1859 ✅** | 14523.6394 | +8539.1921 | Higher | -867.2614 | Lower | -9406.4535 | Lower |
| SLOC | 34 | 33 | **30 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| PLOC | 19 | **17 ✅** | 21 | +2 | Higher | -2 | Lower | -4 | Lower |
| LLOC | 16 | **10 ✅** | 20 | +6 | Higher | -4 | Lower | -10 | Lower |
| Comments (CLOC) | 11 | **18 ✅** | 4 | -7 | Lower | +7 | Higher | +14 | Higher |
| Blank Lines | 7 | **5 ✅** | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02242

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.4814 ✅** | 47.5170 | 48.9461 | +4.9644 | Higher | +3.5353 | Higher | -1.4291 | Lower |
| MI (SEI) | 2.0503 | **18.5033 ✅** | 5.1740 | -16.4530 | Lower | -3.1237 | Lower | +13.3293 | Higher |
| Cyclomatic (sum) | **20 ✅** | 26 | 25 | -6 | Lower | -5 | Lower | +1 | Higher |
| Cognitive (sum) | **17 ✅** | 31 | 23 | -14 | Lower | -6 | Lower | +8 | Higher |
| Halstead Volume | **2576.9231 ✅** | 3287.4026 | 3038.9099 | -710.4795 | Lower | -461.9867 | Lower | +248.4928 | Higher |
| Halstead Effort | **101197.9194 ✅** | 124771.8718 | 171128.6114 | -23573.9524 | Lower | -69930.6920 | Lower | -46356.7396 | Lower |
| SLOC | **91 ✅** | 105 | 100 | -14 | Lower | -9 | Lower | +5 | Higher |
| PLOC | **81 ✅** | 87 | 90 | -6 | Lower | -9 | Lower | -3 | Lower |
| LLOC | **51 ✅** | 54 | 67 | -3 | Lower | -16 | Lower | -13 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 1 | -10 | Lower | -1 | Lower | +9 | Higher |
| Blank Lines | 10 | 16 | 10 | -6 | Lower | 0 | Same | +6 | Higher |
| Functions | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | **1 ✅** | 3 | 3 | -2 | Lower | -2 | Lower | 0 | Same |

## p02249

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.8978 | **62.7715 ✅** | 48.8957 | -13.8737 | Lower | +0.0021 | Higher | +13.8759 | Higher |
| MI (SEI) | 8.0047 | **46.4623 ✅** | 8.0016 | -38.4576 | Lower | +0.0031 | Higher | +38.4607 | Higher |
| Cyclomatic (sum) | 22 | **11 ✅** | 22 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 33 | **13 ✅** | 38 | +20 | Higher | -5 | Lower | -25 | Lower |
| Halstead Volume | 3613.9019 | **1376.6255 ✅** | 3615.3892 | +2237.2763 | Higher | -1.4873 | Lower | -2238.7636 | Lower |
| Halstead Effort | 235634.0903 | **38447.1846 ✅** | 234569.8918 | +197186.9058 | Higher | +1064.1986 | Higher | -196122.7072 | Lower |
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
| MI (original) | 63.6418 | 65.4989 | **69.0523 ✅** | -1.8571 | Lower | -5.4105 | Lower | -3.5534 | Lower |
| MI (SEI) | 52.6136 | 52.4001 | **55.6482 ✅** | +0.2135 | Higher | -3.0347 | Lower | -3.2481 | Lower |
| Cyclomatic (sum) | 15 | 14 | **9 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| Cognitive (sum) | 15 | 8 | **5 ✅** | +7 | Higher | +10 | Higher | +3 | Higher |
| Halstead Volume | 1375.9311 | 1397.1893 | **1191.2093 ✅** | -21.2583 | Lower | +184.7217 | Higher | +205.9800 | Higher |
| Halstead Effort | 45484.3497 | 42206.7611 | **32984.1754 ✅** | +3277.5886 | Higher | +12500.1743 | Higher | +9222.5857 | Higher |
| SLOC | 60 | 54 | **49 ✅** | +6 | Higher | +11 | Higher | +5 | Higher |
| PLOC | 47 | 40 | **36 ✅** | +7 | Higher | +11 | Higher | +4 | Higher |
| LLOC | 26 | **22 ✅** | 24 | +4 | Higher | +2 | Higher | -2 | Lower |
| Comments (CLOC) | **15 ✅** | 11 | 9 | +4 | Higher | +6 | Higher | +2 | Higher |
| Blank Lines | 10 | **8 ✅** | 12 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 7 | 4 | -6 | Lower | -3 | Lower | +3 | Higher |

## p02266

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.6299 | **66.1945 ✅** | 52.7891 | -12.5646 | Lower | +0.8408 | Higher | +13.4054 | Higher |
| MI (SEI) | 11.1727 | **47.7149 ✅** | 10.1024 | -36.5422 | Lower | +1.0702 | Higher | +37.6124 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 17 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 41 | **8 ✅** | 33 | +33 | Higher | +8 | Higher | -25 | Lower |
| Halstead Volume | 2087.6195 | **1157.1748 ✅** | 2425.6716 | +930.4448 | Higher | -338.0520 | Lower | -1268.4968 | Lower |
| Halstead Effort | 69480.2607 | **25383.1885 ✅** | 90292.4326 | +44097.0722 | Higher | -20812.1719 | Lower | -64909.2441 | Lower |
| SLOC | 96 | **59 ✅** | 95 | +37 | Higher | +1 | Higher | -36 | Lower |
| PLOC | 87 | **45 ✅** | 88 | +42 | Higher | -1 | Lower | -43 | Lower |
| LLOC | 53 | **28 ✅** | 58 | +25 | Higher | -5 | Lower | -30 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 1 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 9 | 8 | **7 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02276

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.1116 | 60.0870 | **68.8459 ✅** | +4.0246 | Higher | -4.7344 | Lower | -8.7590 | Lower |
| MI (SEI) | 41.9099 | 45.3751 | **57.0211 ✅** | -3.4652 | Lower | -15.1112 | Lower | -11.6460 | Lower |
| Cyclomatic (sum) | 12 | 15 | **9 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Cognitive (sum) | 10 | 17 | 10 | -7 | Lower | 0 | Same | +7 | Higher |
| Halstead Volume | 1595.3683 | 1686.2630 | **1321.6751 ✅** | -90.8946 | Lower | +273.6932 | Higher | +364.5879 | Higher |
| Halstead Effort | 59256.5385 | 65080.6364 | **43426.4677 ✅** | -5824.0979 | Lower | +15830.0708 | Higher | +21654.1687 | Higher |
| SLOC | 58 | 70 | **48 ✅** | -12 | Lower | +10 | Higher | +22 | Higher |
| PLOC | 43 | 49 | **41 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| LLOC | 29 | 34 | **26 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Comments (CLOC) | 6 | **15 ✅** | 10 | -9 | Lower | -4 | Lower | +5 | Higher |
| Blank Lines | 10 | 11 | **7 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.0431 | 56.2170 | **58.3747 ✅** | +0.8261 | Higher | -1.3316 | Lower | -2.1576 | Lower |
| MI (SEI) | 28.9508 | **37.3777 ✅** | 10.0433 | -8.4269 | Lower | +18.9075 | Higher | +27.3343 | Higher |
| Cyclomatic (sum) | 15 | 16 | 15 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 17 | **10 ✅** | 17 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 2161.3733 | 2329.5782 | **2146.9269 ✅** | -168.2049 | Lower | +14.4463 | Higher | +182.6513 | Higher |
| Halstead Effort | 89813.8213 | **74784.2141 ✅** | 86341.2771 | +15029.6072 | Higher | +3472.5442 | Higher | -11557.0629 | Lower |
| SLOC | 78 | 79 | **72 ✅** | -1 | Lower | +6 | Higher | +7 | Higher |
| PLOC | 61 | **55 ✅** | 61 | +6 | Higher | 0 | Same | -6 | Lower |
| LLOC | 38 | **27 ✅** | 38 | +11 | Higher | 0 | Same | -11 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 0 | -8 | Lower | +6 | Higher | +14 | Higher |
| Blank Lines | 11 | 13 | 11 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02279

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.9258 ✅** | 49.8834 | 53.7153 | +5.0424 | Higher | +1.2105 | Higher | -3.8319 | Lower |
| MI (SEI) | 23.8409 | **24.5429 ✅** | 11.7369 | -0.7020 | Lower | +12.1040 | Higher | +12.8060 | Higher |
| Cyclomatic (sum) | **15 ✅** | 22 | 16 | -7 | Lower | -1 | Lower | +6 | Higher |
| Cognitive (sum) | **14 ✅** | 34 | 19 | -20 | Lower | -5 | Lower | +15 | Higher |
| Halstead Volume | 2887.3958 | **2564.5042 ✅** | 2893.0216 | +322.8916 | Higher | -5.6258 | Lower | -328.5173 | Lower |
| Halstead Effort | 103253.2743 | **96168.9090 ✅** | 118488.1014 | +7084.3653 | Higher | -15234.8271 | Lower | -22319.1924 | Lower |
| SLOC | **81 ✅** | 104 | 86 | -23 | Lower | -5 | Lower | +18 | Higher |
| PLOC | **63 ✅** | 89 | 74 | -26 | Lower | -11 | Lower | +15 | Higher |
| LLOC | 42 | **30 ✅** | 54 | +12 | Higher | -12 | Lower | -24 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 1 | -8 | Lower | +4 | Higher | +12 | Higher |
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
| MI (original) | 70.7423 | 67.5240 | **72.7997 ✅** | +3.2183 | Higher | -2.0574 | Lower | -5.2757 | Lower |
| MI (SEI) | 52.5789 | **58.7350 ✅** | 51.4286 | -6.1562 | Lower | +1.1503 | Higher | +7.3065 | Higher |
| Cyclomatic (sum) | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 8 | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 1236.9954 | 1270.6509 | **1224.0067 ✅** | -33.6555 | Lower | +12.9887 | Higher | +46.6442 | Higher |
| Halstead Effort | 42185.8093 | **35313.5062 ✅** | 41888.2309 | +6872.3031 | Higher | +297.5784 | Higher | -6574.7246 | Lower |
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
| MI (original) | 66.6985 | **79.6785 ✅** | 66.6498 | -12.9800 | Lower | +0.0487 | Higher | +13.0287 | Higher |
| MI (SEI) | 39.7217 | **78.8544 ✅** | 32.2343 | -39.1327 | Lower | +7.4874 | Higher | +46.6201 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 10 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 11 | **4 ✅** | 11 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 1489.2593 | **603.9396 ✅** | 1597.0226 | +885.3198 | Higher | -107.7633 | Lower | -993.0831 | Lower |
| Halstead Effort | 73460.5798 | **12078.7910 ✅** | 75087.5972 | +61381.7888 | Higher | -1627.0174 | Lower | -63008.8062 | Lower |
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
| MI (original) | 66.7405 | **71.4939 ✅** | 62.9509 | -4.7534 | Lower | +3.7896 | Higher | +8.5430 | Higher |
| MI (SEI) | 41.5850 | **45.0442 ✅** | 16.2382 | -3.4592 | Lower | +25.3469 | Higher | +28.8060 | Higher |
| Cyclomatic (sum) | **7 ✅** | 10 | 11 | -3 | Lower | -4 | Lower | -1 | Lower |
| Cognitive (sum) | **7 ✅** | 14 | 14 | -7 | Lower | -7 | Lower | 0 | Same |
| Halstead Volume | 1416.4331 | **1151.9401 ✅** | 1875.5718 | +264.4931 | Higher | -459.1387 | Lower | -723.6318 | Lower |
| Halstead Effort | **39704.3912 ✅** | 47690.3190 | 90652.6388 | -7985.9278 | Lower | -50948.2477 | Lower | -42962.3199 | Lower |
| SLOC | 55 | **42 ✅** | 60 | +13 | Higher | -5 | Lower | -18 | Lower |
| PLOC | 41 | **37 ✅** | 52 | +4 | Higher | -11 | Lower | -15 | Lower |
| LLOC | 28 | **22 ✅** | 38 | +6 | Higher | -10 | Lower | -16 | Lower |
| Comments (CLOC) | **4 ✅** | 2 | 0 | +2 | Higher | +4 | Higher | +2 | Higher |
| Blank Lines | 11 | **5 ✅** | 8 | +6 | Higher | +3 | Higher | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02333

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.8500 | 53.9774 | **60.6682 ✅** | +1.8725 | Higher | -4.8182 | Lower | -6.6907 | Lower |
| MI (SEI) | 6.8082 | **32.2309 ✅** | 13.3521 | -25.4227 | Lower | -6.5439 | Lower | +18.8788 | Higher |
| Cyclomatic (sum) | 19 | 17 | **15 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 13 | 13 | **11 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 2574 | 2631.7747 | **2437.5330 ✅** | -57.7747 | Lower | +136.4670 | Higher | +194.2417 | Higher |
| Halstead Effort | 185100.8824 | 174825.0359 | **168989.5957 ✅** | +10275.8464 | Higher | +16111.2867 | Higher | +5835.4402 | Higher |
| SLOC | 75 | 86 | **60 ✅** | -11 | Lower | +15 | Higher | +26 | Higher |
| PLOC | 61 | 59 | **51 ✅** | +2 | Higher | +10 | Higher | +8 | Higher |
| LLOC | 39 | 41 | **38 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 14 | 16 | **9 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |

## p02340

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.3543 | 67.3699 | **69.5869 ✅** | -6.0156 | Lower | -8.2326 | Lower | -2.2170 | Lower |
| MI (SEI) | 24.3984 | **56.0189 ✅** | 36.9510 | -31.6205 | Lower | -12.5526 | Lower | +19.0679 | Higher |
| Cyclomatic (sum) | 17 | 9 | **8 ✅** | +8 | Higher | +9 | Higher | +1 | Higher |
| Cognitive (sum) | 11 | 6 | 6 | +5 | Higher | +5 | Higher | 0 | Same |
| Halstead Volume | 1857.2236 | **1289.2432 ✅** | 1464.7781 | +567.9804 | Higher | +392.4455 | Higher | -175.5349 | Lower |
| Halstead Effort | 74741.9266 | **36098.8101 ✅** | 47928.4017 | +38643.1165 | Higher | +26813.5249 | Higher | -11829.5916 | Lower |
| SLOC | 61 | 53 | **45 ✅** | +8 | Higher | +16 | Higher | +8 | Higher |
| PLOC | 50 | **31 ✅** | 36 | +19 | Higher | +14 | Higher | -5 | Lower |
| LLOC | 28 | **18 ✅** | 21 | +10 | Higher | +7 | Higher | -3 | Lower |
| Comments (CLOC) | 1 | **12 ✅** | 1 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 10 | 12 | **8 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 6 | 2 | 2 | +4 | Higher | +4 | Higher | 0 | Same |

## p02347

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.3081 | **53.7888 ✅** | 40.7938 | -13.4807 | Lower | -0.4857 | Lower | +12.9950 | Higher |
| MI (SEI) | -8.0446 | **31.8057 ✅** | -14.3024 | -39.8504 | Lower | +6.2578 | Higher | +46.1081 | Higher |
| Cyclomatic (sum) | 29 | **21 ✅** | 25 | +8 | Higher | +4 | Higher | -4 | Lower |
| Cognitive (sum) | 33 | **14 ✅** | 33 | +19 | Higher | 0 | Same | -19 | Lower |
| Halstead Volume | 4805.8171 | **1984.5129 ✅** | 4997.0359 | +2821.3042 | Higher | -191.2188 | Lower | -3012.5230 | Lower |
| Halstead Effort | 331938.6319 | **66953.6848 ✅** | 365019.3282 | +264984.9471 | Higher | -33080.6963 | Lower | -298065.6434 | Lower |
| SLOC | 139 | **90 ✅** | 141 | +49 | Higher | -2 | Lower | -51 | Lower |
| PLOC | 117 | **70 ✅** | 124 | +47 | Higher | -7 | Lower | -54 | Lower |
| LLOC | 55 | **26 ✅** | 80 | +29 | Higher | -25 | Lower | -54 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 0 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 22 | **13 ✅** | 17 | +9 | Higher | +5 | Higher | -4 | Lower |
| Functions | **4 ✅** | 7 | 5 | -3 | Lower | -1 | Lower | +2 | Higher |
| Closures | 4 | 1 | **0 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Exits | **3 ✅** | 6 | 4 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.3756 | **45.0138 ✅** | 40.1562 | -1.6381 | Lower | +3.2194 | Higher | +4.8576 | Higher |
| MI (SEI) | 1.0576 | **12.7675 ✅** | -6.1441 | -11.7099 | Lower | +7.2017 | Higher | +18.9116 | Higher |
| Cyclomatic (sum) | **23 ✅** | 24 | 25 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **13 ✅** | 20 | 14 | -7 | Lower | -1 | Lower | +6 | Higher |
| Halstead Volume | 4717.1563 | **4498.5078 ✅** | 5290.2266 | +218.6485 | Higher | -573.0703 | Lower | -791.7188 | Lower |
| Halstead Effort | **287521.9094 ✅** | 398217.9080 | 360051.2421 | -110695.9986 | Lower | -72529.3326 | Lower | +38166.6659 | Higher |
| SLOC | 126 | **114 ✅** | 144 | +12 | Higher | -18 | Lower | -30 | Lower |
| PLOC | 109 | **88 ✅** | 129 | +21 | Higher | -20 | Lower | -41 | Lower |
| LLOC | **64 ✅** | 65 | 71 | -1 | Lower | -7 | Lower | -6 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 2 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 17 | 17 | **15 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **7 ✅** | 11 | +2 | Higher | -2 | Lower | -4 | Lower |

## p02355

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.2774 | **66.1741 ✅** | 53.4443 | -7.8966 | Lower | +4.8332 | Higher | +12.7298 | Higher |
| MI (SEI) | 28.0629 | **52.9255 ✅** | 17.8525 | -24.8626 | Lower | +10.2104 | Higher | +35.0730 | Higher |
| Cyclomatic (sum) | 18 | **12 ✅** | 23 | +6 | Higher | -5 | Lower | -11 | Lower |
| Cognitive (sum) | 21 | **19 ✅** | 30 | +2 | Higher | -9 | Lower | -11 | Lower |
| Halstead Volume | 1915.6282 | **1266.0753 ✅** | 2081.6888 | +649.5529 | Higher | -166.0606 | Lower | -815.6136 | Lower |
| Halstead Effort | 94272.1252 | **65354.8044 ✅** | 134886.9293 | +28917.3208 | Higher | -40614.8041 | Lower | -69532.1249 | Lower |
| SLOC | 72 | **55 ✅** | 88 | +17 | Higher | -16 | Lower | -33 | Lower |
| PLOC | 62 | **41 ✅** | 70 | +21 | Higher | -8 | Lower | -29 | Lower |
| LLOC | 46 | **31 ✅** | 61 | +15 | Higher | -15 | Lower | -30 | Lower |
| Comments (CLOC) | 4 | **11 ✅** | 3 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 8 | **7 ✅** | 16 | +1 | Higher | -8 | Lower | -9 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |

## p02369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.8736 | **60.9342 ✅** | 42.5542 | -14.0607 | Lower | +4.3194 | Higher | +18.3801 | Higher |
| MI (SEI) | 12.5786 | **48.2051 ✅** | 0.2484 | -35.6265 | Lower | +12.3302 | Higher | +47.9566 | Higher |
| Cyclomatic (sum) | 16 | **14 ✅** | 18 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 17 | **13 ✅** | 22 | +4 | Higher | -5 | Lower | -9 | Lower |
| Halstead Volume | 3280.9800 | **1497.5203 ✅** | 4449.4184 | +1783.4597 | Higher | -1168.4385 | Lower | -2951.8982 | Lower |
| Halstead Effort | 133699.9333 | **44442.5376 ✅** | 244082.3832 | +89257.3957 | Higher | -110382.4499 | Lower | -199639.8456 | Lower |
| SLOC | 126 | **70 ✅** | 145 | +56 | Higher | -19 | Lower | -75 | Lower |
| PLOC | 103 | **51 ✅** | 126 | +52 | Higher | -23 | Lower | -75 | Lower |
| LLOC | 57 | **27 ✅** | 74 | +30 | Higher | -17 | Lower | -47 | Lower |
| Comments (CLOC) | 8 | **17 ✅** | 4 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 21 | **11 ✅** | 18 | +10 | Higher | +3 | Higher | -7 | Lower |
| Functions | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 6 | 3 | -4 | Lower | -1 | Lower | +3 | Higher |

## p02370

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.6531 | **61.9807 ✅** | 56.4965 | -5.3276 | Lower | +0.1566 | Higher | +5.4842 | Higher |
| MI (SEI) | 29.5030 | **42.0915 ✅** | 24.2080 | -12.5885 | Lower | +5.2950 | Higher | +17.8835 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 14 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | **14 ✅** | 16 | 18 | -2 | Lower | -4 | Lower | -2 | Lower |
| Halstead Volume | 2556.7787 | **1761.4290 ✅** | 2319.1846 | +795.3497 | Higher | +237.5941 | Higher | -557.7556 | Lower |
| Halstead Effort | 103732.1649 | **49041.8913 ✅** | 112144.7821 | +54690.2736 | Higher | -8412.6172 | Lower | -63102.8908 | Lower |
| SLOC | 79 | **65 ✅** | 80 | +14 | Higher | -1 | Lower | -15 | Lower |
| PLOC | 61 | **45 ✅** | 65 | +16 | Higher | -4 | Lower | -20 | Lower |
| LLOC | 53 | **34 ✅** | 56 | +19 | Higher | -3 | Lower | -22 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 4 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | 12 | **11 ✅** | 13 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02371

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.5706 | **63.4930 ✅** | 42.6128 | -20.9224 | Lower | -0.0422 | Lower | +20.8802 | Higher |
| MI (SEI) | -2.6336 | **55.4968 ✅** | -5.4656 | -58.1304 | Lower | +2.8319 | Higher | +60.9623 | Higher |
| Cyclomatic (sum) | 21 | **8 ✅** | 20 | +13 | Higher | +1 | Higher | -12 | Lower |
| Cognitive (sum) | 24 | **10 ✅** | 22 | +14 | Higher | +2 | Higher | -12 | Lower |
| Halstead Volume | 5329.4389 | **1368.3158 ✅** | 5396.0363 | +3961.1231 | Higher | -66.5974 | Lower | -4027.7205 | Lower |
| Halstead Effort | 537425.4682 | **40973.4568 ✅** | 516400.6759 | +496452.0113 | Higher | +21024.7923 | Higher | -475427.2190 | Lower |
| SLOC | 131 | **67 ✅** | 132 | +64 | Higher | -1 | Lower | -65 | Lower |
| PLOC | 117 | **35 ✅** | 112 | +82 | Higher | +5 | Higher | -77 | Lower |
| LLOC | 103 | **18 ✅** | 113 | +85 | Higher | -10 | Lower | -95 | Lower |
| Comments (CLOC) | 2 | **22 ✅** | 1 | -20 | Lower | +1 | Higher | +21 | Higher |
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
| MI (original) | 80.5811 | **89.9215 ✅** | 83.7052 | -9.3404 | Lower | -3.1242 | Lower | +6.2162 | Higher |
| MI (SEI) | 74.8382 | **88.0965 ✅** | 77.7546 | -13.2582 | Lower | -2.9164 | Lower | +10.3418 | Higher |
| Cyclomatic (sum) | 7 | 4 | **3 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Cognitive (sum) | 4 | 2 | 2 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 590.1941 | **325.4847 ✅** | 475.4888 | +264.7094 | Higher | +114.7054 | Higher | -150.0040 | Lower |
| Halstead Effort | 9836.5685 | **3173.4761 ✅** | 6847.0380 | +6663.0924 | Higher | +2989.5305 | Higher | -3673.5619 | Lower |
| SLOC | 31 | **22 ✅** | 29 | +9 | Higher | +2 | Higher | -7 | Lower |
| PLOC | 18 | **14 ✅** | 17 | +4 | Higher | +1 | Higher | -3 | Lower |
| LLOC | 12 | **6 ✅** | 12 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | **7 ✅** | 5 | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02430

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.4376 ✅** | 63.9053 | 65.1337 | +8.5323 | Higher | +7.3039 | Higher | -1.2284 | Lower |
| MI (SEI) | **54.5096 ✅** | 48.5038 | 43.9712 | +6.0057 | Higher | +10.5383 | Higher | +4.5326 | Higher |
| Cyclomatic (sum) | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | **6 ✅** | 10 | 9 | -4 | Lower | -3 | Lower | +1 | Higher |
| Halstead Volume | **1248.7737 ✅** | 1493.5472 | 1546.5070 | -244.7735 | Lower | -297.7333 | Lower | -52.9598 | Lower |
| Halstead Effort | **35287.9277 ✅** | 58888.4332 | 51518.0139 | -23600.5054 | Lower | -16230.0861 | Lower | +7370.4193 | Higher |
| SLOC | **37 ✅** | 60 | 55 | -23 | Lower | -18 | Lower | +5 | Higher |
| PLOC | **32 ✅** | 41 | 46 | -9 | Lower | -14 | Lower | -5 | Lower |
| LLOC | **18 ✅** | 34 | 25 | -16 | Lower | -7 | Lower | +9 | Higher |
| Comments (CLOC) | 4 | **11 ✅** | 6 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | **4 ✅** | 10 | 8 | -6 | Lower | -4 | Lower | +2 | Higher |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **1 ✅** | 5 | +6 | Higher | +2 | Higher | -4 | Lower |

## p02433

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.8402 | **44.0930 ✅** | 32.4834 | -9.2528 | Lower | +2.3567 | Higher | +11.6095 | Higher |
| MI (SEI) | -12.8011 | **6.1099 ✅** | -16.2507 | -18.9110 | Lower | +3.4495 | Higher | +22.3605 | Higher |
| Cyclomatic (sum) | 41 | **29 ✅** | 44 | +12 | Higher | -3 | Lower | -15 | Lower |
| Cognitive (sum) | 46 | **31 ✅** | 44 | +15 | Higher | +2 | Higher | -13 | Lower |
| Halstead Volume | 4653.2178 | **3230.6967 ✅** | 4896.7747 | +1422.5211 | Higher | -243.5569 | Lower | -1666.0779 | Lower |
| Halstead Effort | 251273.7623 | **138539.8779 ✅** | 272974.4391 | +112733.8844 | Higher | -21700.6768 | Lower | -134434.5612 | Lower |
| SLOC | 166 | **125 ✅** | 181 | +41 | Higher | -15 | Lower | -56 | Lower |
| PLOC | 140 | **110 ✅** | 152 | +30 | Higher | -12 | Lower | -42 | Lower |
| LLOC | 85 | **58 ✅** | 95 | +27 | Higher | -10 | Lower | -37 | Lower |
| Comments (CLOC) | 2 | **5 ✅** | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 24 | **11 ✅** | 27 | +13 | Higher | -3 | Lower | -16 | Lower |
| Functions | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 2 | 4 | **0 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Exits | 12 | **2 ✅** | 13 | +10 | Higher | -1 | Lower | -11 | Lower |

## p02444

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.6852 | **67.8715 ✅** | 67.6648 | -1.1863 | Lower | -0.9797 | Lower | +0.2066 | Higher |
| MI (SEI) | 54.6746 | 55.8207 | **57.2431 ✅** | -1.1461 | Lower | -2.5685 | Lower | -1.4224 | Lower |
| Cyclomatic (sum) | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 7 | **5 ✅** | 9 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | 1450.2317 | **1242.2611 ✅** | 1460.6197 | +207.9705 | Higher | -10.3881 | Lower | -218.3586 | Lower |
| Halstead Effort | 39273.2087 | **37626.1791 ✅** | 40258.3309 | +1647.0297 | Higher | -985.1222 | Lower | -2632.1519 | Lower |
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
| MI (original) | **54.8174 ✅** | 52.8685 | 49.3214 | +1.9489 | Higher | +5.4960 | Higher | +3.5471 | Higher |
| MI (SEI) | 24.1011 | **33.9617 ✅** | 19.9018 | -9.8606 | Lower | +4.1993 | Higher | +14.0599 | Higher |
| Cyclomatic (sum) | 18 | **17 ✅** | 23 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | 18 | **13 ✅** | 55 | +5 | Higher | -37 | Lower | -42 | Lower |
| Halstead Volume | 2683.7110 | **2238.7778 ✅** | 2817.1002 | +444.9332 | Higher | -133.3892 | Lower | -578.3224 | Lower |
| Halstead Effort | 163017.3094 | **117074.9103 ✅** | 211406.8015 | +45942.3991 | Higher | -48389.4921 | Lower | -94331.8912 | Lower |
| SLOC | **80 ✅** | 97 | 103 | -17 | Lower | -23 | Lower | -6 | Lower |
| PLOC | 59 | 59 | 90 | 0 | Same | -31 | Lower | -31 | Lower |
| LLOC | 46 | **44 ✅** | 79 | +2 | Higher | -33 | Lower | -35 | Lower |
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
| MI (original) | 42.7643 | **63.0359 ✅** | 41.8960 | -20.2715 | Lower | +0.8684 | Higher | +21.1399 | Higher |
| MI (SEI) | -10.9503 | **46.2225 ✅** | -1.7195 | -57.1729 | Lower | -9.2309 | Lower | +47.9420 | Higher |
| Cyclomatic (sum) | 30 | **14 ✅** | 35 | +16 | Higher | -5 | Lower | -21 | Lower |
| Cognitive (sum) | 28 | **8 ✅** | 37 | +20 | Higher | -9 | Lower | -29 | Lower |
| Halstead Volume | 3990.7104 | **1200.7669 ✅** | 4406.3839 | +2789.9435 | Higher | -415.6735 | Lower | -3205.6169 | Lower |
| Halstead Effort | 199891.8341 | **29590.3281 ✅** | 357338.9823 | +170301.5060 | Higher | -157447.1482 | Lower | -327748.6542 | Lower |
| SLOC | 125 | **66 ✅** | 119 | +59 | Higher | +6 | Higher | -53 | Lower |
| PLOC | 112 | **47 ✅** | 102 | +65 | Higher | +10 | Higher | -55 | Lower |
| LLOC | 57 | **17 ✅** | 59 | +40 | Higher | -2 | Lower | -42 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 2 | -11 | Lower | -2 | Lower | +9 | Higher |
| Blank Lines | 13 | **8 ✅** | 17 | +5 | Higher | -4 | Lower | -9 | Lower |
| Functions | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 9 | **4 ✅** | 14 | +5 | Higher | -5 | Lower | -10 | Lower |

## p02542

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 17.0733 | **57.2415 ✅** | 10.5574 | -40.1682 | Lower | +6.5159 | Higher | +46.6841 | Higher |
| MI (SEI) | -40.8482 | **45.1611 ✅** | -54.2587 | -86.0094 | Lower | +13.4105 | Higher | +99.4198 | Higher |
| Cyclomatic (sum) | 56 | **17 ✅** | 61 | +39 | Higher | -5 | Lower | -44 | Lower |
| Cognitive (sum) | 112 | **19 ✅** | 135 | +93 | Higher | -23 | Lower | -116 | Lower |
| Halstead Volume | 12437.0374 | **2151.8208 ✅** | 18217.6779 | +10285.2167 | Higher | -5780.6404 | Lower | -16065.8571 | Lower |
| Halstead Effort | 2091386.0331 | **121362.6916 ✅** | 4568027.8560 | +1970023.3415 | Higher | -2476641.8228 | Lower | -4446665.1644 | Lower |
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
| MI (original) | 13.1895 | **42.5728 ✅** | 10.8894 | -29.3833 | Lower | +2.3000 | Higher | +31.6833 | Higher |
| MI (SEI) | -36.6730 | **15.6387 ✅** | -53.6779 | -52.3117 | Lower | +17.0048 | Higher | +69.3165 | Higher |
| Cyclomatic (sum) | 63 | **28 ✅** | 62 | +35 | Higher | +1 | Higher | -34 | Lower |
| Cognitive (sum) | 113 | **35 ✅** | 119 | +78 | Higher | -6 | Lower | -84 | Lower |
| Halstead Volume | 15069.7434 | **3817.2013 ✅** | 16781.9864 | +11252.5421 | Higher | -1712.2430 | Lower | -12964.7852 | Lower |
| Halstead Effort | 2580905.2117 | **260253.9013 ✅** | 3681548.2744 | +2320651.3104 | Higher | -1100643.0626 | Lower | -3421294.3731 | Lower |
| SLOC | 317 | **132 ✅** | 358 | +185 | Higher | -41 | Lower | -226 | Lower |
| PLOC | 279 | **102 ✅** | 331 | +177 | Higher | -52 | Lower | -229 | Lower |
| LLOC | 281 | **59 ✅** | 339 | +222 | Higher | -58 | Lower | -280 | Lower |
| Comments (CLOC) | 10 | **18 ✅** | 0 | -8 | Lower | +10 | Higher | +18 | Higher |
| Blank Lines | 38 | **17 ✅** | 27 | +21 | Higher | +11 | Higher | -10 | Lower |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02584

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.6196 | **81.6455 ✅** | 78.7953 | -3.0259 | Lower | -0.1757 | Lower | +2.8502 | Higher |
| MI (SEI) | 64.2696 | **82.8040 ✅** | 66.7408 | -18.5344 | Lower | -2.4712 | Lower | +16.0632 | Higher |
| Cyclomatic (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 851.6713 | **432.4343 ✅** | 781.9412 | +419.2370 | Higher | +69.7301 | Higher | -349.5069 | Lower |
| Halstead Effort | 18807.7419 | **7135.1666 ✅** | 17984.6476 | +11672.5753 | Higher | +823.0943 | Higher | -10849.4810 | Lower |
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
| MI (original) | 56.1986 | **72.9409 ✅** | 55.9910 | -16.7422 | Lower | +0.2076 | Higher | +16.9499 | Higher |
| MI (SEI) | 27.9874 | **66.6423 ✅** | 15.4429 | -38.6549 | Lower | +12.5445 | Higher | +51.1993 | Higher |
| Cyclomatic (sum) | 15 | **8 ✅** | 15 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 24 | **8 ✅** | 24 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 2756.7929 | **1109.2048 ✅** | 2869.0944 | +1647.5881 | Higher | -112.3015 | Lower | -1759.8896 | Lower |
| Halstead Effort | 205462.1526 | **33544.4997 ✅** | 226363.1067 | +171917.6529 | Higher | -20900.9542 | Lower | -192818.6070 | Lower |
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
| MI (original) | **76.1032 ✅** | 72.2216 | 74.5482 | +3.8816 | Higher | +1.5549 | Higher | -2.3267 | Lower |
| MI (SEI) | 60.0394 | **69.0993 ✅** | 56.5468 | -9.0599 | Lower | +3.4926 | Higher | +12.5525 | Higher |
| Cyclomatic (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 1094.4844 | **922.4348 ✅** | 1043.7263 | +172.0496 | Higher | +50.7582 | Higher | -121.2914 | Lower |
| Halstead Effort | 32429.1682 | **24905.7409 ✅** | 32564.2594 | +7523.4273 | Higher | -135.0912 | Lower | -7658.5185 | Lower |
| SLOC | **34 ✅** | 45 | 38 | -11 | Lower | -4 | Lower | +7 | Higher |
| PLOC | **22 ✅** | 25 | 26 | -3 | Lower | -4 | Lower | -1 | Lower |
| LLOC | 18 | **13 ✅** | 18 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | 4 | **16 ✅** | 4 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02618

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.2001 | **61.7732 ✅** | 50.6420 | -11.5730 | Lower | -0.4419 | Lower | +11.1312 | Higher |
| MI (SEI) | 26.0642 | **56.0373 ✅** | 13.0280 | -29.9732 | Lower | +13.0362 | Higher | +43.0093 | Higher |
| Cyclomatic (sum) | 18 | **8 ✅** | 19 | +10 | Higher | -1 | Lower | -11 | Lower |
| Cognitive (sum) | 15 | **6 ✅** | 16 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 3357.7619 | **1589.9051 ✅** | 3585.3026 | +1767.8568 | Higher | -227.5406 | Lower | -1995.3975 | Lower |
| Halstead Effort | 188183.9018 | **33564.6633 ✅** | 221651.3723 | +154619.2385 | Higher | -33467.4705 | Lower | -188086.7090 | Lower |
| SLOC | 99 | **71 ✅** | 93 | +28 | Higher | +6 | Higher | -22 | Lower |
| PLOC | 72 | **39 ✅** | 74 | +33 | Higher | -2 | Lower | -35 | Lower |
| LLOC | 53 | **20 ✅** | 58 | +33 | Higher | -5 | Lower | -38 | Lower |
| Comments (CLOC) | 14 | **29 ✅** | 3 | -15 | Lower | +11 | Higher | +26 | Higher |
| Blank Lines | 18 | **14 ✅** | 19 | +4 | Higher | -1 | Lower | -5 | Lower |
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
| MI (original) | **57.9176 ✅** | 53.6480 | 55.9548 | +4.2696 | Higher | +1.9628 | Higher | -2.3068 | Lower |
| MI (SEI) | 22.5124 | **37.6554 ✅** | 6.7558 | -15.1431 | Lower | +15.7566 | Higher | +30.8996 | Higher |
| Cyclomatic (sum) | **15 ✅** | 16 | 17 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 27 | 28 | **21 ✅** | -1 | Lower | +6 | Higher | +7 | Higher |
| Halstead Volume | 2801.0823 | **2728.1326 ✅** | 3416.7832 | +72.9497 | Higher | -615.7008 | Lower | -688.6505 | Lower |
| Halstead Effort | **129273.2070 ✅** | 171549.2867 | 215063.2035 | -42276.0797 | Lower | -85789.9964 | Lower | -43513.9168 | Lower |
| SLOC | **68 ✅** | 88 | 70 | -20 | Lower | -2 | Lower | +18 | Higher |
| PLOC | 57 | 62 | 57 | -5 | Lower | 0 | Same | +5 | Higher |
| LLOC | 42 | **31 ✅** | 53 | +11 | Higher | -11 | Lower | -22 | Lower |
| Comments (CLOC) | 2 | **21 ✅** | 0 | -19 | Lower | +2 | Higher | +21 | Higher |
| Blank Lines | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p02647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.4759 | **82.6004 ✅** | 66.0161 | -19.1245 | Lower | -2.5402 | Lower | +16.5843 | Higher |
| MI (SEI) | 52.8634 | **73.2177 ✅** | 20.6602 | -20.3544 | Lower | +32.2031 | Higher | +52.5575 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 11 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 18 | **11 ✅** | 20 | +7 | Higher | -2 | Lower | -9 | Lower |
| Halstead Volume | 1695.4645 | **782.3234 ✅** | 1624.6253 | +913.1410 | Higher | +70.8391 | Higher | -842.3019 | Lower |
| Halstead Effort | 75769.7218 | **25507.8613 ✅** | 70223.0289 | +50261.8605 | Higher | +5546.6929 | Higher | -44715.1676 | Lower |
| SLOC | 60 | **25 ✅** | 52 | +35 | Higher | +8 | Higher | -27 | Lower |
| PLOC | 48 | **20 ✅** | 47 | +28 | Higher | +1 | Higher | -27 | Lower |
| LLOC | 42 | **15 ✅** | 43 | +27 | Higher | -1 | Lower | -28 | Lower |
| Comments (CLOC) | **16 ✅** | 4 | 0 | +12 | Higher | +16 | Higher | +4 | Higher |
| Blank Lines | 9 | 5 | 5 | +4 | Higher | +4 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.8183 | **72.4185 ✅** | 42.0192 | -22.6001 | Lower | +7.7991 | Higher | +30.3993 | Higher |
| MI (SEI) | 12.2914 | **68.3563 ✅** | 4.0211 | -56.0650 | Lower | +8.2703 | Higher | +64.3352 | Higher |
| Cyclomatic (sum) | 22 | **9 ✅** | 22 | +13 | Higher | 0 | Same | -13 | Lower |
| Cognitive (sum) | 19 | **10 ✅** | 22 | +9 | Higher | -3 | Lower | -12 | Lower |
| Halstead Volume | 3936.4122 | **936.6593 ✅** | 4409.9279 | +2999.7528 | Higher | -473.5157 | Lower | -3473.2685 | Lower |
| Halstead Effort | 372038.9533 | **43086.3299 ✅** | 272064.0992 | +328952.6234 | Higher | +99974.8541 | Higher | -228977.7693 | Lower |
| SLOC | 91 | **43 ✅** | 142 | +48 | Higher | -51 | Lower | -99 | Lower |
| PLOC | 77 | **26 ✅** | 115 | +51 | Higher | -38 | Lower | -89 | Lower |
| LLOC | 59 | **18 ✅** | 54 | +41 | Higher | +5 | Higher | -36 | Lower |
| Comments (CLOC) | 3 | **14 ✅** | 7 | -11 | Lower | -4 | Lower | +7 | Higher |
| Blank Lines | 14 | **6 ✅** | 26 | +8 | Higher | -12 | Lower | -20 | Lower |
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
| MI (original) | 44.2310 | **45.5052 ✅** | 36.1655 | -1.2741 | Lower | +8.0655 | Higher | +9.3396 | Higher |
| MI (SEI) | -9.6489 | **15.5913 ✅** | -20.8777 | -25.2402 | Lower | +11.2288 | Higher | +36.4690 | Higher |
| Cyclomatic (sum) | **22 ✅** | 29 | 26 | -7 | Lower | -4 | Lower | +3 | Higher |
| Cognitive (sum) | **21 ✅** | 24 | 26 | -3 | Lower | -5 | Lower | -2 | Lower |
| Halstead Volume | 5131 | **2870.1554 ✅** | 6871.8451 | +2260.8446 | Higher | -1740.8451 | Lower | -4001.6897 | Lower |
| Halstead Effort | 189380.5455 | **121609.5454 ✅** | 317129.0435 | +67771.0000 | Higher | -127748.4980 | Lower | -195519.4980 | Lower |
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
| MI (original) | 53.6746 | **54.7298 ✅** | 52.8092 | -1.0552 | Lower | +0.8654 | Higher | +1.9206 | Higher |
| MI (SEI) | 28.6725 | **36.3135 ✅** | 17.3430 | -7.6410 | Lower | +11.3295 | Higher | +18.9705 | Higher |
| Cyclomatic (sum) | 28 | 26 | **23 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | **16 ✅** | 20 | 29 | -4 | Lower | -13 | Lower | -9 | Lower |
| Halstead Volume | **2520.5123 ✅** | 2548.3960 | 2822.2891 | -27.8837 | Lower | -301.7767 | Lower | -273.8930 | Lower |
| Halstead Effort | 108647.3480 | **102204.0930 ✅** | 151854.8303 | +6443.2550 | Higher | -43207.4823 | Lower | -49650.7373 | Lower |
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
| MI (original) | 68.0452 | **82.6099 ✅** | 64.5595 | -14.5648 | Lower | +3.4857 | Higher | +18.0505 | Higher |
| MI (SEI) | 48.3248 | **85.7583 ✅** | 44.3257 | -37.4335 | Lower | +3.9991 | Higher | +41.4326 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 13 | +4 | Higher | -4 | Lower | -8 | Lower |
| Cognitive (sum) | 12 | **3 ✅** | 15 | +9 | Higher | -3 | Lower | -12 | Lower |
| Halstead Volume | 2021.7564 | **671.2461 ✅** | 2223.5907 | +1350.5103 | Higher | -201.8343 | Lower | -1552.3447 | Lower |
| Halstead Effort | 138571.1854 | **14671.5215 ✅** | 196370.8567 | +123899.6639 | Higher | -57799.6713 | Lower | -181699.3352 | Lower |
| SLOC | 44 | **27 ✅** | 50 | +17 | Higher | -6 | Lower | -23 | Lower |
| PLOC | 38 | **18 ✅** | 39 | +20 | Higher | -1 | Lower | -21 | Lower |
| LLOC | 36 | **13 ✅** | 41 | +23 | Higher | -5 | Lower | -28 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 6 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02743

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.5031 | **87.5308 ✅** | 84.3892 | -2.0277 | Lower | +1.1139 | Higher | +3.1416 | Higher |
| MI (SEI) | 78.1635 | **91.0532 ✅** | 83.0033 | -12.8896 | Lower | -4.8398 | Lower | +8.0499 | Higher |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 662.8274 | **346.1296 ✅** | 633.3094 | +316.6979 | Higher | +29.5180 | Higher | -287.1799 | Lower |
| Halstead Effort | 10452.2788 | **4499.6842 ✅** | 8893.8674 | +5952.5946 | Higher | +1558.4114 | Higher | -4394.1832 | Lower |
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
| MI (original) | 61.8184 | **69.9213 ✅** | 61.5771 | -8.1029 | Lower | +0.2413 | Higher | +8.3442 | Higher |
| MI (SEI) | 25.6961 | **64.7794 ✅** | 25.2587 | -39.0833 | Lower | +0.4374 | Higher | +39.5207 | Higher |
| Cyclomatic (sum) | 19 | **12 ✅** | 19 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 25 | **10 ✅** | 25 | +15 | Higher | 0 | Same | -15 | Lower |
| Halstead Volume | 2029.5119 | **1005.0091 ✅** | 2011.8639 | +1024.5027 | Higher | +17.6479 | Higher | -1006.8548 | Lower |
| Halstead Effort | 113652.6637 | **39572.2340 ✅** | 106094.3866 | +74080.4297 | Higher | +7558.2772 | Higher | -66522.1526 | Lower |
| SLOC | 56 | **47 ✅** | 57 | +9 | Higher | -1 | Lower | -10 | Lower |
| PLOC | 49 | **28 ✅** | 50 | +21 | Higher | -1 | Lower | -22 | Lower |
| LLOC | 39 | **19 ✅** | 38 | +20 | Higher | +1 | Higher | -19 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 1 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

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
| MI (original) | 73.6775 | **76.9043 ✅** | 75.3702 | -3.2268 | Lower | -1.6927 | Lower | +1.5341 | Higher |
| MI (SEI) | 63.7484 | **79.3937 ✅** | 64.9475 | -15.6453 | Lower | -1.1990 | Lower | +14.4463 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 839.5814 | **585.4121 ✅** | 926.4271 | +254.1693 | Higher | -86.8457 | Lower | -341.0150 | Lower |
| Halstead Effort | 22248.9076 | **18440.4808 ✅** | 22645.9964 | +3808.4267 | Higher | -397.0888 | Lower | -4205.5155 | Lower |
| SLOC | 43 | 39 | **37 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| PLOC | 29 | 26 | **25 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| LLOC | 24 | **19 ✅** | 24 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | 9 | **18 ✅** | 7 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 8 | **5 ✅** | 7 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | **0 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p02831

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.0182 | 67.2887 | **86.2946 ✅** | +13.7295 | Higher | -5.2765 | Lower | -19.0059 | Lower |
| MI (SEI) | 61.5800 | 54.7786 | **86.3238 ✅** | +6.8014 | Higher | -24.7438 | Lower | -31.5452 | Lower |
| Cyclomatic (sum) | 6 | 12 | **4 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Cognitive (sum) | 5 | 7 | **3 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Halstead Volume | 698.1203 | 1081.9189 | **498.5531 ✅** | -383.7985 | Lower | +199.5672 | Higher | +583.3657 | Higher |
| Halstead Effort | 12700.4195 | 31592.0304 | **6954.8159 ✅** | -18891.6109 | Lower | +5745.6036 | Higher | +24637.2145 | Higher |
| SLOC | 29 | 54 | **24 ✅** | -25 | Lower | +5 | Higher | +30 | Higher |
| PLOC | 23 | 39 | **14 ✅** | -16 | Lower | +9 | Higher | +25 | Higher |
| LLOC | 16 | 24 | **9 ✅** | -8 | Lower | +7 | Higher | +15 | Higher |
| Comments (CLOC) | 2 | **11 ✅** | 7 | -9 | Lower | -5 | Lower | +4 | Higher |
| Blank Lines | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.0845 | **70.1832 ✅** | 61.8758 | -8.0987 | Lower | +0.2087 | Higher | +8.3074 | Higher |
| MI (SEI) | 36.4292 | **60.6727 ✅** | 24.9471 | -24.2436 | Lower | +11.4820 | Higher | +35.7256 | Higher |
| Cyclomatic (sum) | 14 | **7 ✅** | 15 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 24 | **7 ✅** | 26 | +17 | Higher | -2 | Lower | -19 | Lower |
| Halstead Volume | 1666.7109 | **983.1622 ✅** | 1835.3846 | +683.5486 | Higher | -168.6738 | Lower | -852.2224 | Lower |
| Halstead Effort | 71501.8960 | **25910.4212 ✅** | 90348.2886 | +45591.4748 | Higher | -18846.3926 | Lower | -64437.8674 | Lower |
| SLOC | 63 | **50 ✅** | 61 | +13 | Higher | +2 | Higher | -11 | Lower |
| PLOC | 49 | **28 ✅** | 53 | +21 | Higher | -4 | Lower | -25 | Lower |
| LLOC | 35 | **21 ✅** | 37 | +14 | Higher | -2 | Lower | -16 | Lower |
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
| MI (original) | 69.1132 | **78.9648 ✅** | 61.7742 | -9.8516 | Lower | +7.3390 | Higher | +17.1906 | Higher |
| MI (SEI) | 36.1447 | **84.5539 ✅** | 33.0574 | -48.4092 | Lower | +3.0873 | Higher | +51.4965 | Higher |
| Cyclomatic (sum) | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 11 | **5 ✅** | 7 | +6 | Higher | +4 | Higher | -2 | Lower |
| Halstead Volume | 1498.2976 | **634.1371 ✅** | 2520.2850 | +864.1604 | Higher | -1021.9874 | Lower | -1886.1479 | Lower |
| Halstead Effort | 53376.8509 | **16555.5086 ✅** | 175519.8482 | +36821.3423 | Higher | -122142.9973 | Lower | -158964.3396 | Lower |
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
| MI (original) | 62.5529 | 58.3438 | **63.4569 ✅** | +4.2091 | Higher | -0.9040 | Lower | -5.1131 | Lower |
| MI (SEI) | 15.8676 | **35.7734 ✅** | 16.9681 | -19.9058 | Lower | -1.1005 | Lower | +18.8052 | Higher |
| Cyclomatic (sum) | 13 | 14 | **11 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **1952.9764 ✅** | 2162.4177 | 1996.5290 | -209.4413 | Lower | -43.5526 | Lower | +165.8886 | Higher |
| Halstead Effort | **94891.6764 ✅** | 115547.0804 | 102095.2340 | -20655.4040 | Lower | -7203.5577 | Lower | +13451.8463 | Higher |
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
| MI (original) | 51.3086 | **63.7431 ✅** | 54.3975 | -12.4345 | Lower | -3.0888 | Lower | +9.3457 | Higher |
| MI (SEI) | 22.1886 | **40.3934 ✅** | 5.0181 | -18.2048 | Lower | +17.1705 | Higher | +35.3753 | Higher |
| Cyclomatic (sum) | 21 | **14 ✅** | 22 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 37 | **19 ✅** | 36 | +18 | Higher | +1 | Higher | -17 | Lower |
| Halstead Volume | 2701.7305 | **1958.3837 ✅** | 2637.7090 | +743.3468 | Higher | +64.0216 | Higher | -679.3253 | Lower |
| Halstead Effort | 154728.8374 | **77280.8335 ✅** | 177036.8194 | +77448.0039 | Higher | -22307.9820 | Lower | -99755.9859 | Lower |
| SLOC | 95 | **54 ✅** | 78 | +41 | Higher | +17 | Higher | -24 | Lower |
| PLOC | 71 | **44 ✅** | 67 | +27 | Higher | +4 | Higher | -23 | Lower |
| LLOC | 64 | **29 ✅** | 62 | +35 | Higher | +2 | Higher | -33 | Lower |
| Comments (CLOC) | **8 ✅** | 5 | 0 | +3 | Higher | +8 | Higher | +5 | Higher |
| Blank Lines | 17 | **9 ✅** | 11 | +8 | Higher | +6 | Higher | -2 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
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
| MI (original) | 74.7436 | **81.1830 ✅** | 54.0079 | -6.4394 | Lower | +20.7357 | Higher | +27.1751 | Higher |
| MI (SEI) | 63.2529 | **66.0296 ✅** | 4.2525 | -2.7767 | Lower | +59.0004 | Higher | +61.7771 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 20 | -1 | Lower | -14 | Lower | -13 | Lower |
| Cognitive (sum) | **6 ✅** | 7 | 15 | -1 | Lower | -9 | Lower | -8 | Lower |
| Halstead Volume | 927.1038 | **647.0781 ✅** | 2210.2090 | +280.0257 | Higher | -1283.1052 | Lower | -1563.1309 | Lower |
| Halstead Effort | 24318.6452 | **17438.7545 ✅** | 107882.0927 | +6879.8906 | Higher | -83563.4476 | Lower | -90443.3382 | Lower |
| SLOC | 39 | **29 ✅** | 87 | +10 | Higher | -48 | Lower | -58 | Lower |
| PLOC | 26 | **24 ✅** | 73 | +2 | Higher | -47 | Lower | -49 | Lower |
| LLOC | 22 | **17 ✅** | 46 | +5 | Higher | -24 | Lower | -29 | Lower |
| Comments (CLOC) | **7 ✅** | 3 | 0 | +4 | Higher | +7 | Higher | +3 | Higher |
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
| MI (original) | 79.9884 | **80.7139 ✅** | 76.1217 | -0.7255 | Lower | +3.8667 | Higher | +4.5922 | Higher |
| MI (SEI) | 69.8653 | **85.4234 ✅** | 69.8037 | -15.5580 | Lower | +0.0616 | Higher | +15.6197 | Higher |
| Cyclomatic (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 6 | 6 | 9 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | 765.7091 | **494.8981 ✅** | 873.2046 | +270.8110 | Higher | -107.4956 | Lower | -378.3066 | Lower |
| Halstead Effort | 16358.3302 | **11062.4274 ✅** | 18511.9384 | +5295.9028 | Higher | -2153.6082 | Lower | -7449.5110 | Lower |
| SLOC | **30 ✅** | 33 | 36 | -3 | Lower | -6 | Lower | -3 | Lower |
| PLOC | 22 | **16 ✅** | 27 | +6 | Higher | -5 | Lower | -11 | Lower |
| LLOC | 16 | **8 ✅** | 19 | +8 | Higher | -3 | Lower | -11 | Lower |
| Comments (CLOC) | 5 | **16 ✅** | 9 | -11 | Lower | -4 | Lower | +7 | Higher |
| Blank Lines | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p02956

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.2080 | **70.0694 ✅** | 32.2878 | -37.8613 | Lower | -0.0797 | Lower | +37.7816 | Higher |
| MI (SEI) | -7.7067 | **66.3068 ✅** | -9.3538 | -74.0135 | Lower | +1.6470 | Higher | +75.6605 | Higher |
| Cyclomatic (sum) | 37 | **7 ✅** | 37 | +30 | Higher | 0 | Same | -30 | Lower |
| Cognitive (sum) | 48 | **2 ✅** | 48 | +46 | Higher | 0 | Same | -46 | Lower |
| Halstead Volume | 6917.0166 | **746.7564 ✅** | 6929.7317 | +6170.2602 | Higher | -12.7151 | Lower | -6182.9753 | Lower |
| Halstead Effort | 1011757.7795 | **13994.7683 ✅** | 1030797.5881 | +997763.0112 | Higher | -19039.8087 | Lower | -1016802.8199 | Lower |
| SLOC | 182 | **55 ✅** | 181 | +127 | Higher | +1 | Higher | -126 | Lower |
| PLOC | 157 | **27 ✅** | 156 | +130 | Higher | +1 | Higher | -129 | Lower |
| LLOC | 150 | **11 ✅** | 149 | +139 | Higher | +1 | Higher | -138 | Lower |
| Comments (CLOC) | 10 | **20 ✅** | 8 | -10 | Lower | +2 | Higher | +12 | Higher |
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
| MI (original) | 72.4969 | **78.2983 ✅** | 71.8630 | -5.8014 | Lower | +0.6339 | Higher | +6.4352 | Higher |
| MI (SEI) | 63.6759 | **79.1862 ✅** | 62.7615 | -15.5102 | Lower | +0.9145 | Higher | +16.4247 | Higher |
| Cyclomatic (sum) | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 15 | **1 ✅** | 15 | +14 | Higher | 0 | Same | -14 | Lower |
| Halstead Volume | 1250.6531 | **716.2669 ✅** | 1412.7872 | +534.3862 | Higher | -162.1341 | Lower | -696.5202 | Lower |
| Halstead Effort | 57530.0427 | **15882.4410 ✅** | 63575.4224 | +41647.6017 | Higher | -6045.3797 | Lower | -47692.9814 | Lower |
| SLOC | 39 | **35 ✅** | 39 | +4 | Higher | 0 | Same | -4 | Lower |
| PLOC | 23 | **17 ✅** | 25 | +6 | Higher | -2 | Lower | -8 | Lower |
| LLOC | 24 | **15 ✅** | 27 | +9 | Higher | -3 | Lower | -12 | Lower |
| Comments (CLOC) | 9 | **14 ✅** | 9 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02988

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.5718 ✅** | 72.2256 | 72.2898 | +2.3462 | Higher | +2.2820 | Higher | -0.0642 | Lower |
| MI (SEI) | 54.1479 | **63.3903 ✅** | 54.9792 | -9.2424 | Lower | -0.8313 | Lower | +8.4111 | Higher |
| Cyclomatic (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 7 | 7 | 14 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | 988.7168 | **972.0530 ✅** | 1033.1535 | +16.6638 | Higher | -44.4367 | Lower | -61.1005 | Lower |
| Halstead Effort | 32689.4497 | 31206.9531 | **27954.7502 ✅** | +1482.4966 | Higher | +4734.6995 | Higher | +3252.2029 | Higher |
| SLOC | **37 ✅** | 43 | 42 | -6 | Lower | -5 | Lower | +1 | Higher |
| PLOC | 26 | 26 | 30 | 0 | Same | -4 | Lower | -4 | Lower |
| LLOC | 20 | 20 | 21 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 3 | **10 ✅** | 5 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 8 | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

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
| MI (original) | 53.7704 | **76.9810 ✅** | 47.7816 | -23.2106 | Lower | +5.9888 | Higher | +29.1994 | Higher |
| MI (SEI) | 15.5450 | **64.1614 ✅** | 2.8003 | -48.6165 | Lower | +12.7447 | Higher | +61.3611 | Higher |
| Cyclomatic (sum) | 18 | **8 ✅** | 20 | +10 | Higher | -2 | Lower | -12 | Lower |
| Cognitive (sum) | 32 | **10 ✅** | 36 | +22 | Higher | -4 | Lower | -26 | Lower |
| Halstead Volume | 2819.4702 | **846.2119 ✅** | 4073.9216 | +1973.2583 | Higher | -1254.4514 | Lower | -3227.7097 | Lower |
| Halstead Effort | 205062.2356 | **17629.4146 ✅** | 510578.7788 | +187432.8210 | Higher | -305516.5432 | Lower | -492949.3642 | Lower |
| SLOC | 84 | **34 ✅** | 105 | +50 | Higher | -21 | Lower | -71 | Lower |
| PLOC | 76 | **24 ✅** | 98 | +52 | Higher | -22 | Lower | -74 | Lower |
| LLOC | 53 | **15 ✅** | 78 | +38 | Higher | -25 | Lower | -63 | Lower |
| Comments (CLOC) | 2 | **5 ✅** | 1 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 8 | **5 ✅** | 7 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
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
| MI (original) | 68.5073 | **77.9742 ✅** | 72.3138 | -9.4669 | Lower | -3.8064 | Lower | +5.6604 | Higher |
| MI (SEI) | 52.8403 | **77.0922 ✅** | 60.3179 | -24.2519 | Lower | -7.4776 | Lower | +16.7743 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 7 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 5 | +4 | Higher | +1 | Higher | -3 | Lower |
| Halstead Volume | 1167.8347 | **798.2752 ✅** | 1044.1134 | +369.5594 | Higher | +123.7213 | Higher | -245.8381 | Lower |
| Halstead Effort | 35521.6376 | **24726.5758 ✅** | 29351.1877 | +10795.0618 | Higher | +6170.4500 | Higher | -4624.6118 | Lower |
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
| MI (original) | 60.1385 | **76.3518 ✅** | 60.0978 | -16.2133 | Lower | +0.0406 | Higher | +16.2540 | Higher |
| MI (SEI) | 12.2824 | **76.4798 ✅** | 12.2238 | -64.1973 | Lower | +0.0586 | Higher | +64.2560 | Higher |
| Cyclomatic (sum) | 12 | **5 ✅** | 12 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 13 | **3 ✅** | 13 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 2520.5634 | **657.3039 ✅** | 2540.3440 | +1863.2595 | Higher | -19.7806 | Lower | -1883.0401 | Lower |
| Halstead Effort | 184994.0751 | **15446.6410 ✅** | 176807.9401 | +169547.4341 | Higher | +8186.1350 | Higher | -161361.2991 | Lower |
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
| MI (original) | **61.9579 ✅** | 60.2914 | 58.5058 | +1.6665 | Higher | +3.4521 | Higher | +1.7856 | Higher |
| MI (SEI) | 40.2492 | **46.6781 ✅** | 10.3343 | -6.4289 | Lower | +29.9149 | Higher | +36.3437 | Higher |
| Cyclomatic (sum) | **13 ✅** | 15 | 16 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **16 ✅** | 24 | 19 | -8 | Lower | -3 | Lower | +5 | Higher |
| Halstead Volume | 2078.0145 | **2042.2893 ✅** | 2754.4719 | +35.7252 | Higher | -676.4574 | Lower | -712.1826 | Lower |
| Halstead Effort | **77818.9794 ✅** | 96594.7658 | 138232.5749 | -18775.7865 | Lower | -60413.5956 | Lower | -41637.8091 | Lower |
| SLOC | **60 ✅** | 65 | 65 | -5 | Lower | -5 | Lower | 0 | Same |
| PLOC | **43 ✅** | 47 | 56 | -4 | Lower | -13 | Lower | -9 | Lower |
| LLOC | 34 | **29 ✅** | 50 | +5 | Higher | -16 | Lower | -21 | Lower |
| Comments (CLOC) | 7 | **15 ✅** | 0 | -8 | Lower | +7 | Higher | +15 | Higher |
| Blank Lines | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03055

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.4893 | **66.6359 ✅** | 31.3663 | -30.1465 | Lower | +5.1230 | Higher | +35.2696 | Higher |
| MI (SEI) | 0.2836 | **51.7434 ✅** | -15.6042 | -51.4598 | Lower | +15.8878 | Higher | +67.3475 | Higher |
| Cyclomatic (sum) | 30 | **10 ✅** | 38 | +20 | Higher | -8 | Lower | -28 | Lower |
| Cognitive (sum) | 53 | **11 ✅** | 56 | +42 | Higher | -3 | Lower | -45 | Lower |
| Halstead Volume | 5616.8868 | **1601.2991 ✅** | 6176.5237 | +4015.5878 | Higher | -559.6369 | Lower | -4575.2247 | Lower |
| Halstead Effort | 635562.9560 | **54123.9089 ✅** | 819637.8430 | +581439.0470 | Higher | -184074.8870 | Lower | -765513.9341 | Lower |
| SLOC | 165 | **51 ✅** | 196 | +114 | Higher | -31 | Lower | -145 | Lower |
| PLOC | 144 | **36 ✅** | 172 | +108 | Higher | -28 | Lower | -136 | Lower |
| LLOC | 134 | **23 ✅** | 145 | +111 | Higher | -11 | Lower | -122 | Lower |
| Comments (CLOC) | **12 ✅** | 9 | 4 | +3 | Higher | +8 | Higher | +5 | Higher |
| Blank Lines | 13 | **8 ✅** | 24 | +5 | Higher | -11 | Lower | -16 | Lower |
| Functions | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |

## p03064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.8294 | **76.5294 ✅** | 69.1592 | -6.7000 | Lower | +0.6703 | Higher | +7.3702 | Higher |
| MI (SEI) | 52.1909 | **70.6965 ✅** | 46.8428 | -18.5056 | Lower | +5.3480 | Higher | +23.8537 | Higher |
| Cyclomatic (sum) | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 5 | **1 ✅** | 7 | +4 | Higher | -2 | Lower | -6 | Lower |
| Halstead Volume | 1426.2522 | **814.2399 ✅** | 1552.2664 | +612.0123 | Higher | -126.0143 | Lower | -738.0265 | Lower |
| Halstead Effort | 53705.7722 | **13528.9096 ✅** | 73991.3671 | +40176.8626 | Higher | -20285.5949 | Lower | -60462.4575 | Lower |
| SLOC | 46 | **38 ✅** | 46 | +8 | Higher | 0 | Same | -8 | Lower |
| PLOC | 33 | **26 ✅** | 36 | +7 | Higher | -3 | Lower | -10 | Lower |
| LLOC | 33 | **16 ✅** | 35 | +17 | Higher | -2 | Lower | -19 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 4 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 9 | 6 | 6 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8627 | **72.7486 ✅** | 59.9995 | -12.8859 | Lower | -0.1368 | Lower | +12.7491 | Higher |
| MI (SEI) | 30.2324 | **71.2136 ✅** | 27.9644 | -40.9812 | Lower | +2.2680 | Higher | +43.2492 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 11 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 2200.4126 | **818.2861 ✅** | 2240.1863 | +1382.1265 | Higher | -39.7737 | Lower | -1421.9002 | Lower |
| Halstead Effort | 88425.8832 | **29151.4427 ✅** | 96628.5248 | +59274.4405 | Higher | -8202.6416 | Lower | -67477.0821 | Lower |
| SLOC | 68 | **44 ✅** | 68 | +24 | Higher | 0 | Same | -24 | Lower |
| PLOC | 58 | **31 ✅** | 56 | +27 | Higher | +2 | Higher | -25 | Lower |
| LLOC | 49 | **24 ✅** | 47 | +25 | Higher | +2 | Higher | -23 | Lower |
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
| MI (original) | 54.3048 | **56.0803 ✅** | 51.9433 | -1.7755 | Lower | +2.3614 | Higher | +4.1370 | Higher |
| MI (SEI) | 30.6551 | **36.2389 ✅** | 14.6443 | -5.5838 | Lower | +16.0108 | Higher | +21.5946 | Higher |
| Cyclomatic (sum) | 14 | **13 ✅** | 15 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 8 | **7 ✅** | 10 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 3036.4928 | **2341.5104 ✅** | 3565.2417 | +694.9823 | Higher | -528.7490 | Lower | -1223.7313 | Lower |
| Halstead Effort | 134690.1437 | **86864.3259 ✅** | 251943.7500 | +47825.8178 | Higher | -117253.6064 | Lower | -165079.4241 | Lower |
| SLOC | 84 | **83 ✅** | 91 | +1 | Higher | -7 | Lower | -8 | Lower |
| PLOC | 71 | **59 ✅** | 77 | +12 | Higher | -6 | Lower | -18 | Lower |
| LLOC | 46 | **38 ✅** | 62 | +8 | Higher | -16 | Lower | -24 | Lower |
| Comments (CLOC) | 11 | **14 ✅** | 3 | -3 | Lower | +8 | Higher | +11 | Higher |
| Blank Lines | 13 | **12 ✅** | 14 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |

## p03115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.3126 ✅** | 63.7200 | 63.8481 | +0.5926 | Higher | +0.4645 | Higher | -0.1281 | Lower |
| MI (SEI) | 18.6099 | **55.6884 ✅** | 17.9398 | -37.0785 | Lower | +0.6701 | Higher | +37.7486 | Higher |
| Cyclomatic (sum) | 15 | **11 ✅** | 15 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1888.8205 | **1202.1225 ✅** | 1946.2949 | +686.6980 | Higher | -57.4745 | Lower | -744.1724 | Lower |
| Halstead Effort | 104298.3045 | **36985.3013 ✅** | 101707.8110 | +67313.0033 | Higher | +2590.4936 | Higher | -64722.5097 | Lower |
| SLOC | **52 ✅** | 66 | 53 | -14 | Lower | -1 | Lower | +13 | Higher |
| PLOC | 44 | **37 ✅** | 45 | +7 | Higher | -1 | Lower | -8 | Lower |
| LLOC | 30 | **22 ✅** | 30 | +8 | Higher | 0 | Same | -8 | Lower |
| Comments (CLOC) | 0 | **21 ✅** | 0 | -21 | Lower | 0 | Same | +21 | Higher |
| Blank Lines | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p03123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.8294 | **89.8491 ✅** | 41.8877 | -48.0197 | Lower | -0.0583 | Lower | +47.9613 | Higher |
| MI (SEI) | -6.0596 | **92.7737 ✅** | -1.5443 | -98.8333 | Lower | -4.5153 | Lower | +94.3180 | Higher |
| Cyclomatic (sum) | 26 | **5 ✅** | 24 | +21 | Higher | +2 | Higher | -19 | Lower |
| Cognitive (sum) | 24 | **2 ✅** | 23 | +22 | Higher | +1 | Higher | -21 | Lower |
| Halstead Volume | 4485.8629 | **315.7688 ✅** | 4424.7107 | +4170.0942 | Higher | +61.1523 | Higher | -4108.9419 | Lower |
| Halstead Effort | 271287.9019 | **5591.7385 ✅** | 266420.0782 | +265696.1633 | Higher | +4867.8237 | Higher | -260828.3397 | Lower |
| SLOC | 135 | **22 ✅** | 139 | +113 | Higher | -4 | Lower | -117 | Lower |
| PLOC | 117 | **13 ✅** | 119 | +104 | Higher | -2 | Lower | -106 | Lower |
| LLOC | 84 | **7 ✅** | 90 | +77 | Higher | -6 | Lower | -83 | Lower |
| Comments (CLOC) | 1 | **7 ✅** | 3 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 18 | **3 ✅** | 18 | +15 | Higher | 0 | Same | -15 | Lower |
| Functions | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 9 | **2 ✅** | 8 | +7 | Higher | +1 | Higher | -6 | Lower |

## p03142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.5238 | **74.1723 ✅** | 55.9894 | -18.6484 | Lower | -0.4655 | Lower | +18.1829 | Higher |
| MI (SEI) | 20.8082 | **64.6971 ✅** | 6.5002 | -43.8889 | Lower | +14.3080 | Higher | +58.1970 | Higher |
| Cyclomatic (sum) | 16 | **4 ✅** | 14 | +12 | Higher | +2 | Higher | -10 | Lower |
| Cognitive (sum) | 23 | **2 ✅** | 20 | +21 | Higher | +3 | Higher | -18 | Lower |
| Halstead Volume | 2559.5063 | **1331.9501 ✅** | 2556.7787 | +1227.5563 | Higher | +2.7276 | Higher | -1224.8287 | Lower |
| Halstead Effort | 131848.2532 | **31021.5466 ✅** | 123812.0092 | +100826.7066 | Higher | +8036.2441 | Higher | -92790.4626 | Lower |
| SLOC | 80 | **37 ✅** | 80 | +43 | Higher | 0 | Same | -43 | Lower |
| PLOC | 69 | **26 ✅** | 69 | +43 | Higher | 0 | Same | -43 | Lower |
| LLOC | 49 | **22 ✅** | 52 | +27 | Higher | -3 | Lower | -30 | Lower |
| Comments (CLOC) | 3 | **8 ✅** | 0 | -5 | Lower | +3 | Higher | +8 | Higher |
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
| MI (original) | 49.2626 | 52.7008 | **59.4994 ✅** | -3.4382 | Lower | -10.2367 | Lower | -6.7985 | Lower |
| MI (SEI) | 7.0924 | **28.1437 ✅** | 20.9392 | -21.0514 | Lower | -13.8468 | Lower | +7.2046 | Higher |
| Cyclomatic (sum) | 33 | 23 | **13 ✅** | +10 | Higher | +20 | Higher | +10 | Higher |
| Cognitive (sum) | 27 | 20 | **13 ✅** | +7 | Higher | +14 | Higher | +7 | Higher |
| Halstead Volume | 3330.3974 | 2776.1864 | **2477.5974 ✅** | +554.2109 | Higher | +852.8000 | Higher | +298.5891 | Higher |
| Halstead Effort | 175317.1450 | 150608.1142 | **119667.9533 ✅** | +24709.0307 | Higher | +55649.1917 | Higher | +30940.1610 | Higher |
| SLOC | 85 | 84 | **66 ✅** | +1 | Higher | +19 | Higher | +18 | Higher |
| PLOC | 67 | 60 | **53 ✅** | +7 | Higher | +14 | Higher | +7 | Higher |
| LLOC | 49 | **31 ✅** | 37 | +18 | Higher | +12 | Higher | -6 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 1 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 18 | 14 | **13 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 12 | 3 | **1 ✅** | +9 | Higher | +11 | Higher | +2 | Higher |

## p03164

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.1103 | 57.2871 | **69.8409 ✅** | +2.8232 | Higher | -9.7306 | Lower | -12.5538 | Lower |
| MI (SEI) | 12.8527 | 37.7473 | **38.0170 ✅** | -24.8946 | Lower | -25.1643 | Lower | -0.2697 | Lower |
| Cyclomatic (sum) | 18 | 24 | **11 ✅** | -6 | Lower | +7 | Higher | +13 | Higher |
| Cognitive (sum) | 12 | 10 | 10 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 2041.2771 | 2330.7542 | **1514.5376 ✅** | -289.4770 | Lower | +526.7396 | Higher | +816.2166 | Higher |
| Halstead Effort | 86783.4395 | 89544.3225 | **60338.0948 ✅** | -2760.8831 | Lower | +26445.3446 | Higher | +29206.2277 | Higher |
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
| MI (original) | **62.7228 ✅** | 61.1361 | 62.4045 | +1.5867 | Higher | +0.3183 | Higher | -1.2684 | Lower |
| MI (SEI) | **44.5534 ✅** | 41.8922 | 43.5797 | +2.6612 | Higher | +0.9737 | Higher | -1.6875 | Lower |
| Cyclomatic (sum) | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 16 | **15 ✅** | 16 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1890.2050 | **1803.8313 ✅** | 1893.1977 | +86.3737 | Higher | -2.9927 | Lower | -89.3664 | Lower |
| Halstead Effort | 92508.8560 | 104934.4146 | **91583.4390 ✅** | -12425.5586 | Lower | +925.4171 | Higher | +13350.9756 | Higher |
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
| MI (original) | **70.0561 ✅** | 63.5857 | 69.7669 | +6.4704 | Higher | +0.2893 | Higher | -6.1812 | Lower |
| MI (SEI) | 26.2852 | **50.8467 ✅** | 37.7066 | -24.5615 | Lower | -11.4215 | Lower | +13.1400 | Higher |
| Cyclomatic (sum) | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1711.2825 | **1648.0225 ✅** | 1678.3256 | +63.2600 | Higher | +32.9569 | Higher | -30.3031 | Lower |
| Halstead Effort | 94884.5015 | **49137.0912 ✅** | 85941.8440 | +45747.4103 | Higher | +8942.6576 | Higher | -36804.7528 | Lower |
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
| MI (original) | 77.9034 | 80.2068 | **81.7629 ✅** | -2.3035 | Lower | -3.8596 | Lower | -1.5561 | Lower |
| MI (SEI) | 60.4754 | **78.0768 ✅** | 73.7875 | -17.6014 | Lower | -13.3121 | Lower | +4.2893 | Higher |
| Cyclomatic (sum) | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 6 | **1 ✅** | 6 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1032.3814 | **596.0559 ✅** | 755.8125 | +436.3255 | Higher | +276.5689 | Higher | -159.7566 | Lower |
| Halstead Effort | 24570.6776 | **12517.1749 ✅** | 21603.6411 | +12053.5027 | Higher | +2967.0365 | Higher | -9086.4662 | Lower |
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
| MI (original) | 56.5343 | **65.1842 ✅** | 60.4963 | -8.6499 | Lower | -3.9620 | Lower | +4.6879 | Higher |
| MI (SEI) | 34.7131 | **53.1200 ✅** | 13.0023 | -18.4069 | Lower | +21.7108 | Higher | +40.1177 | Higher |
| Cyclomatic (sum) | 18 | 14 | 14 | +4 | Higher | +4 | Higher | 0 | Same |
| Cognitive (sum) | 15 | **5 ✅** | 30 | +10 | Higher | -15 | Lower | -25 | Lower |
| Halstead Volume | 1855.8165 | **1484.3714 ✅** | 1867.3021 | +371.4450 | Higher | -11.4856 | Lower | -382.9306 | Lower |
| Halstead Effort | 69283.8148 | **42632.5287 ✅** | 78185.7447 | +26651.2862 | Higher | -8901.9299 | Lower | -35553.2160 | Lower |
| SLOC | 81 | **54 ✅** | 67 | +27 | Higher | +14 | Higher | -13 | Lower |
| PLOC | 61 | **37 ✅** | 58 | +24 | Higher | +3 | Higher | -21 | Lower |
| LLOC | 46 | **24 ✅** | 55 | +22 | Higher | -9 | Lower | -31 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 0 | -1 | Lower | +11 | Higher | +12 | Higher |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03218

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.6162 | **71.4839 ✅** | 68.2398 | -0.8678 | Lower | +2.3764 | Higher | +3.2441 | Higher |
| MI (SEI) | 53.5295 | **64.8201 ✅** | 45.0829 | -11.2905 | Lower | +8.4467 | Higher | +19.7372 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 11 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 4 | **3 ✅** | 7 | +1 | Higher | -3 | Lower | -4 | Lower |
| Halstead Volume | 1122.2031 | **992.6744 ✅** | 1197.0166 | +129.5286 | Higher | -74.8135 | Lower | -204.3422 | Lower |
| Halstead Effort | 29557.3807 | **21462.3059 ✅** | 34952.8847 | +8095.0748 | Higher | -5395.5041 | Lower | -13490.5788 | Lower |
| SLOC | 46 | 46 | 50 | 0 | Same | -4 | Lower | -4 | Lower |
| PLOC | 33 | **29 ✅** | 36 | +4 | Higher | -3 | Lower | -7 | Lower |
| LLOC | 19 | 15 | 15 | +4 | Higher | +4 | Higher | 0 | Same |
| Comments (CLOC) | 6 | **13 ✅** | 4 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | **8 ✅** | 9 | 10 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | **1 ✅** | 2 | 4 | -1 | Lower | -3 | Lower | -2 | Lower |

## p03268

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.2664 | 67.7158 | **82.7523 ✅** | +10.5506 | Higher | -4.4859 | Lower | -15.0365 | Lower |
| MI (SEI) | 65.9777 | 51.0892 | **74.9094 ✅** | +14.8884 | Higher | -8.9318 | Lower | -23.8202 | Lower |
| Cyclomatic (sum) | 4 | 10 | **3 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Cognitive (sum) | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 865.6692 | 1088.7840 | **713.5293 ✅** | -223.1148 | Lower | +152.1399 | Higher | +375.2547 | Higher |
| Halstead Effort | 15880.5520 | 27289.8438 | **14066.7205 ✅** | -11409.2918 | Lower | +1813.8315 | Higher | +13223.1233 | Higher |
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
| MI (original) | 56.0885 | **74.0535 ✅** | 57.0567 | -17.9650 | Lower | -0.9682 | Lower | +16.9968 | Higher |
| MI (SEI) | 23.8240 | **70.8411 ✅** | 27.6128 | -47.0171 | Lower | -3.7888 | Lower | +43.2282 | Higher |
| Cyclomatic (sum) | 15 | **7 ✅** | 15 | +8 | Higher | 0 | Same | -8 | Lower |
| Cognitive (sum) | 12 | **4 ✅** | 12 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 2495.8598 | **1012.8602 ✅** | 2435.8908 | +1482.9996 | Higher | +59.9689 | Higher | -1423.0306 | Lower |
| Halstead Effort | 129912.7006 | **24423.0000 ✅** | 131735.6095 | +105489.7006 | Higher | -1822.9089 | Lower | -107312.6095 | Lower |
| SLOC | 79 | **39 ✅** | 75 | +40 | Higher | +4 | Higher | -36 | Lower |
| PLOC | 65 | **26 ✅** | 65 | +39 | Higher | 0 | Same | -39 | Lower |
| LLOC | 51 | **17 ✅** | 51 | +34 | Higher | 0 | Same | -34 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 5 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | 14 | **6 ✅** | 10 | +8 | Higher | +4 | Higher | -4 | Lower |
| Functions | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03278

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.8950 | **65.0158 ✅** | 44.9461 | -19.1208 | Lower | +0.9489 | Higher | +20.0697 | Higher |
| MI (SEI) | 3.1055 | **58.8201 ✅** | -8.8210 | -55.7146 | Lower | +11.9265 | Higher | +67.6411 | Higher |
| Cyclomatic (sum) | 19 | **8 ✅** | 20 | +11 | Higher | -1 | Lower | -12 | Lower |
| Cognitive (sum) | 19 | **6 ✅** | 21 | +13 | Higher | -2 | Lower | -15 | Lower |
| Halstead Volume | 6305.6634 | **1517.2171 ✅** | 6437.4175 | +4788.4463 | Higher | -131.7542 | Lower | -4920.2005 | Lower |
| Halstead Effort | 917781.6110 | **44866.2764 ✅** | 907522.6015 | +872915.3345 | Higher | +10259.0095 | Higher | -862656.3250 | Lower |
| SLOC | 104 | **59 ✅** | 108 | +45 | Higher | -4 | Lower | -49 | Lower |
| PLOC | 92 | **35 ✅** | 95 | +57 | Higher | -3 | Lower | -60 | Lower |
| LLOC | 86 | **21 ✅** | 90 | +65 | Higher | -4 | Lower | -69 | Lower |
| Comments (CLOC) | 2 | **21 ✅** | 0 | -19 | Lower | +2 | Higher | +21 | Higher |
| Blank Lines | **12 ✅** | 13 | 13 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03294

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.5344 | 80.1524 | **81.5455 ✅** | -2.6180 | Lower | -4.0111 | Lower | -1.3931 | Lower |
| MI (SEI) | 64.9599 | **73.5843 ✅** | 72.3424 | -8.6244 | Lower | -7.3825 | Lower | +1.2419 | Higher |
| Cyclomatic (sum) | 8 | 5 | **4 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Cognitive (sum) | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 760.7724 | **634.2483 ✅** | 689.1356 | +126.5241 | Higher | +71.6368 | Higher | -54.8874 | Lower |
| Halstead Effort | 13991.5965 | 12684.9653 | **10911.3140 ✅** | +1306.6311 | Higher | +3080.2824 | Higher | +1773.6513 | Higher |
| SLOC | 34 | 32 | **29 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| PLOC | 22 | **19 ✅** | 20 | +3 | Higher | +2 | Higher | -1 | Lower |
| LLOC | 16 | **14 ✅** | 15 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | 5 | **7 ✅** | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | 1 | **0 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03296

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.4958 | 72.4672 | **77.3876 ✅** | +2.0286 | Higher | -2.8918 | Lower | -4.9204 | Lower |
| MI (SEI) | 61.0333 | 62.7268 | **67.2354 ✅** | -1.6935 | Lower | -6.2021 | Lower | -4.5086 | Lower |
| Cyclomatic (sum) | 7 | 10 | **6 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 6 | 7 | **4 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Halstead Volume | **930.2778 ✅** | 955.3113 | 938.2737 | -25.0336 | Lower | -7.9959 | Lower | +17.0376 | Higher |
| Halstead Effort | **21524.7026 ✅** | 21972.1605 | 25802.5268 | -447.4579 | Lower | -4277.8242 | Lower | -3830.3663 | Lower |
| SLOC | 39 | 42 | **33 ✅** | -3 | Lower | +6 | Higher | +9 | Higher |
| PLOC | 27 | **26 ✅** | 28 | +1 | Higher | -1 | Lower | -2 | Lower |
| LLOC | 23 | **15 ✅** | 22 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 8 | 8 | **5 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03311

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.6945 | **81.9491 ✅** | 60.1378 | -20.2546 | Lower | +1.5567 | Higher | +21.8112 | Higher |
| MI (SEI) | 33.4118 | **80.3475 ✅** | 32.5338 | -46.9358 | Lower | +0.8779 | Higher | +47.8137 | Higher |
| Cyclomatic (sum) | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 17 | **1 ✅** | 17 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 1868.6959 | **599.7113 ✅** | 1994.2160 | +1268.9846 | Higher | -125.5201 | Lower | -1394.5047 | Lower |
| Halstead Effort | 112121.7562 | **6678.6031 ✅** | 120483.8830 | +105443.1531 | Higher | -8362.1267 | Lower | -113805.2798 | Lower |
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
| MI (original) | 75.2627 | **83.2157 ✅** | 68.4518 | -7.9529 | Lower | +6.8110 | Higher | +14.7639 | Higher |
| MI (SEI) | 62.6995 | **86.7340 ✅** | 45.8988 | -24.0345 | Lower | +16.8007 | Higher | +40.8352 | Higher |
| Cyclomatic (sum) | 6 | 6 | 12 | 0 | Same | -6 | Lower | -6 | Lower |
| Cognitive (sum) | 4 | **2 ✅** | 7 | +2 | Higher | -3 | Lower | -5 | Lower |
| Halstead Volume | 988.5384 | **571.5856 ✅** | 1248.5813 | +416.9527 | Higher | -260.0429 | Lower | -676.9957 | Lower |
| Halstead Effort | 21528.1688 | **10912.0896 ✅** | 38414.6849 | +10616.0792 | Higher | -16886.5161 | Lower | -27502.5953 | Lower |
| SLOC | 37 | **27 ✅** | 48 | +10 | Higher | -11 | Lower | -21 | Lower |
| PLOC | 25 | **14 ✅** | 37 | +11 | Higher | -12 | Lower | -23 | Lower |
| LLOC | 19 | **8 ✅** | 21 | +11 | Higher | -2 | Lower | -13 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 4 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **0 ✅** | 1 | 3 | -1 | Lower | -3 | Lower | -2 | Lower |

## p03318

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.6911 | 68.1218 | **74.0132 ✅** | +5.5693 | Higher | -0.3221 | Lower | -5.8913 | Lower |
| MI (SEI) | 43.6023 | **55.2150 ✅** | 52.9756 | -11.6127 | Lower | -9.3733 | Lower | +2.2394 | Higher |
| Cyclomatic (sum) | **7 ✅** | 12 | 8 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | **7 ✅** | 11 | 11 | -4 | Lower | -4 | Lower | 0 | Same |
| Halstead Volume | 1085.9754 | **1036.7536 ✅** | 1058.8993 | +49.2218 | Higher | +27.0761 | Higher | -22.1457 | Lower |
| Halstead Effort | 32941.2529 | **30888.1073 ✅** | 37590.9252 | +2053.1456 | Higher | -4649.6723 | Lower | -6702.8178 | Lower |
| SLOC | 39 | 52 | **38 ✅** | -13 | Lower | +1 | Higher | +14 | Higher |
| PLOC | **32 ✅** | 43 | 34 | -11 | Lower | -2 | Lower | +9 | Higher |
| LLOC | **27 ✅** | 30 | 29 | -3 | Lower | -2 | Lower | +1 | Higher |
| Comments (CLOC) | 1 | **10 ✅** | 3 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 7 | 7 | **4 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p03361

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.8597 ✅** | 61.0904 | 67.8957 | +9.7693 | Higher | +2.9640 | Higher | -6.8053 | Lower |
| MI (SEI) | 49.0371 | 44.2164 | **54.1657 ✅** | +4.8207 | Higher | -5.1286 | Lower | -9.9493 | Lower |
| Cyclomatic (sum) | **14 ✅** | 16 | 15 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 32 | **30 ✅** | 31 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 1489.2593 | 1846.9795 | **1487.8148 ✅** | -357.7202 | Lower | +1.4446 | Higher | +359.1647 | Higher |
| Halstead Effort | 57310.8069 | 59103.3439 | **55459.5780 ✅** | -1792.5370 | Lower | +1851.2288 | Higher | +3643.7659 | Higher |
| SLOC | **38 ✅** | 63 | 45 | -25 | Lower | -7 | Lower | +18 | Higher |
| PLOC | **29 ✅** | 51 | 34 | -22 | Lower | -5 | Lower | +17 | Higher |
| LLOC | **21 ✅** | 31 | 25 | -10 | Lower | -4 | Lower | +6 | Higher |
| Comments (CLOC) | 3 | **11 ✅** | 8 | -8 | Lower | -5 | Lower | +3 | Higher |
| Blank Lines | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03365

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.8140 | **69.1580 ✅** | 62.2583 | -4.3440 | Lower | +2.5557 | Higher | +6.8997 | Higher |
| MI (SEI) | 18.9260 | **62.7882 ✅** | 15.4425 | -43.8622 | Lower | +3.4834 | Higher | +47.3456 | Higher |
| Cyclomatic (sum) | 11 | **9 ✅** | 13 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 8 | **5 ✅** | 9 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | 2047.1598 | **1167.2572 ✅** | 2301.2528 | +879.9026 | Higher | -254.0930 | Lower | -1133.9956 | Lower |
| Halstead Effort | 120910.3746 | **49024.8008 ✅** | 146231.0768 | +71885.5737 | Higher | -25320.7022 | Lower | -97206.2760 | Lower |
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
| MI (original) | **85.7070 ✅** | 79.4072 | 85.6009 | +6.2998 | Higher | +0.1061 | Higher | -6.1936 | Lower |
| MI (SEI) | **84.9044 ✅** | 76.6064 | 79.6174 | +8.2980 | Higher | +5.2870 | Higher | -3.0110 | Lower |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 491.5424 | 605.9969 | **443.9738 ✅** | -114.4545 | Lower | +47.5686 | Higher | +162.0231 | Higher |
| Halstead Effort | **6077.2516 ✅** | 7971.1901 | 6748.4016 | -1893.9386 | Lower | -671.1500 | Lower | +1222.7886 | Higher |
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
| MI (original) | 49.7339 | **54.2138 ✅** | 47.7802 | -4.4799 | Lower | +1.9537 | Higher | +6.4337 | Higher |
| MI (SEI) | 5.8791 | **38.3212 ✅** | -4.5286 | -32.4421 | Lower | +10.4077 | Higher | +42.8498 | Higher |
| Cyclomatic (sum) | 20 | **18 ✅** | 22 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 32 | **16 ✅** | 38 | +16 | Higher | -6 | Lower | -22 | Lower |
| Halstead Volume | 3469.8030 | **2406 ✅** | 3730.0127 | +1063.8030 | Higher | -260.2097 | Lower | -1324.0127 | Lower |
| Halstead Effort | 341032.0680 | **176294.1818 ✅** | 313451.1857 | +164737.8861 | Higher | +27580.8823 | Higher | -137157.0038 | Lower |
| SLOC | 98 | **86 ✅** | 105 | +12 | Higher | -7 | Lower | -19 | Lower |
| PLOC | 87 | **57 ✅** | 92 | +30 | Higher | -5 | Lower | -35 | Lower |
| LLOC | 70 | **44 ✅** | 68 | +26 | Higher | +2 | Higher | -24 | Lower |
| Comments (CLOC) | 1 | **20 ✅** | 0 | -19 | Lower | +1 | Higher | +20 | Higher |
| Blank Lines | **10 ✅** | 15 | 13 | -5 | Lower | -3 | Lower | +2 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p03408

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.9405 | **67.1905 ✅** | 66.1905 | -4.2500 | Lower | -3.2500 | Lower | +1.0001 | Higher |
| MI (SEI) | 43.3743 | **56.5536 ✅** | 43.1377 | -13.1793 | Lower | +0.2366 | Higher | +13.4159 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 10 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 15 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 1530.8001 | 1523.8412 | **1303.5606 ✅** | +6.9589 | Higher | +227.2395 | Higher | +220.2805 | Higher |
| Halstead Effort | 47733.1296 | **37286.4887 ✅** | 41112.2965 | +10446.6410 | Higher | +6620.8331 | Higher | -3825.8079 | Lower |
| SLOC | 65 | **53 ✅** | 56 | +12 | Higher | +9 | Higher | -3 | Lower |
| PLOC | 45 | **38 ✅** | 43 | +7 | Higher | +2 | Higher | -5 | Lower |
| LLOC | 37 | **28 ✅** | 35 | +9 | Higher | +2 | Higher | -7 | Lower |
| Comments (CLOC) | 9 | **13 ✅** | 5 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 11 | **7 ✅** | 8 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03414

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.5102 | **66.9358 ✅** | 21.6968 | -44.4256 | Lower | +0.8133 | Higher | +45.2390 | Higher |
| MI (SEI) | -28.5833 | **56.8988 ✅** | -23.2809 | -85.4821 | Lower | -5.3024 | Lower | +80.1797 | Higher |
| Cyclomatic (sum) | 57 | **13 ✅** | 55 | +44 | Higher | +2 | Higher | -42 | Lower |
| Cognitive (sum) | 68 | **11 ✅** | 65 | +57 | Higher | +3 | Higher | -54 | Lower |
| Halstead Volume | 9136.9437 | **1174.2237 ✅** | 9820.2621 | +7962.7200 | Higher | -683.3184 | Lower | -8646.0384 | Lower |
| Halstead Effort | 1173107.4267 | **37218.6974 ✅** | 1309040.9395 | +1135888.7292 | Higher | -135933.5129 | Lower | -1271822.2421 | Lower |
| SLOC | 228 | **53 ✅** | 241 | +175 | Higher | -13 | Lower | -188 | Lower |
| PLOC | 208 | **31 ✅** | 221 | +177 | Higher | -13 | Lower | -190 | Lower |
| LLOC | 158 | **18 ✅** | 163 | +140 | Higher | -5 | Lower | -145 | Lower |
| Comments (CLOC) | 3 | **13 ✅** | 10 | -10 | Lower | -7 | Lower | +3 | Higher |
| Blank Lines | 20 | **9 ✅** | 20 | +11 | Higher | 0 | Same | -11 | Lower |
| Functions | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 5 | **1 ✅** | 4 | +4 | Higher | +1 | Higher | -3 | Lower |
| Exits | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |

## p03434

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.7340 | **76.3848 ✅** | 54.8065 | -18.6509 | Lower | +2.9275 | Higher | +21.5783 | Higher |
| MI (SEI) | 21.5124 | **76.2913 ✅** | 4.9973 | -54.7789 | Lower | +16.5151 | Higher | +71.2940 | Higher |
| Cyclomatic (sum) | 13 | **6 ✅** | 16 | +7 | Higher | -3 | Lower | -10 | Lower |
| Cognitive (sum) | 14 | **5 ✅** | 24 | +9 | Higher | -10 | Lower | -19 | Lower |
| Halstead Volume | 2436 | **578.6156 ✅** | 2619.7300 | +1857.3844 | Higher | -183.7300 | Lower | -2041.1144 | Lower |
| Halstead Effort | 124142.3077 | **12729.5436 ✅** | 139991.8218 | +111412.7641 | Higher | -15849.5141 | Lower | -127262.2782 | Lower |
| SLOC | 74 | **41 ✅** | 83 | +33 | Higher | -9 | Lower | -42 | Lower |
| PLOC | 60 | **23 ✅** | 69 | +37 | Higher | -9 | Lower | -46 | Lower |
| LLOC | 55 | **15 ✅** | 59 | +40 | Higher | -4 | Lower | -44 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 0 | -14 | Lower | +2 | Higher | +16 | Higher |
| Blank Lines | 14 | **8 ✅** | 14 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.6174 ✅** | 65.7869 | 67.1500 | +3.8305 | Higher | +2.4674 | Higher | -1.3631 | Lower |
| MI (SEI) | **50.3994 ✅** | 49.7255 | 43.7142 | +0.6739 | Higher | +6.6851 | Higher | +6.0112 | Higher |
| Cyclomatic (sum) | 12 | 12 | 13 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 12 | **8 ✅** | 21 | +4 | Higher | -9 | Lower | -13 | Lower |
| Halstead Volume | **1139.3324 ✅** | 1954.7369 | 1351.1399 | -815.4045 | Lower | -211.8076 | Lower | +603.5970 | Higher |
| Halstead Effort | **43294.6293 ✅** | 111645.5485 | 67065.6726 | -68350.9191 | Lower | -23771.0433 | Lower | +44579.8759 | Higher |
| SLOC | **46 ✅** | 49 | 50 | -3 | Lower | -4 | Lower | -1 | Lower |
| PLOC | **33 ✅** | 34 | 40 | -1 | Lower | -7 | Lower | -6 | Lower |
| LLOC | 23 | **11 ✅** | 29 | +12 | Higher | -6 | Lower | -18 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 4 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 8 | 7 | **6 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
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
| MI (original) | 49.8553 | **79.9413 ✅** | 51.3344 | -30.0860 | Lower | -1.4790 | Lower | +28.6069 | Higher |
| MI (SEI) | 6.8702 | **82.1544 ✅** | 0.9045 | -75.2842 | Lower | +5.9657 | Higher | +81.2499 | Higher |
| Cyclomatic (sum) | 26 | **6 ✅** | 25 | +20 | Higher | +1 | Higher | -19 | Lower |
| Cognitive (sum) | 41 | **4 ✅** | 40 | +37 | Higher | +1 | Higher | -36 | Lower |
| Halstead Volume | 3060.2828 | **523.1869 ✅** | 2962.6344 | +2537.0959 | Higher | +97.6484 | Higher | -2439.4475 | Lower |
| Halstead Effort | 256357.5362 | **13951.6498 ✅** | 260711.8238 | +242405.8863 | Higher | -4354.2877 | Lower | -246760.1740 | Lower |
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
| MI (original) | 62.9579 | **75.8750 ✅** | 75.5473 | -12.9170 | Lower | -12.5893 | Lower | +0.3277 | Higher |
| MI (SEI) | 40.4283 | **75.2106 ✅** | 61.0902 | -34.7823 | Lower | -20.6619 | Lower | +14.1204 | Higher |
| Cyclomatic (sum) | 10 | 8 | **5 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 2427.1964 | **630.9011 ✅** | 1065.3932 | +1796.2952 | Higher | +1361.8032 | Higher | -434.4921 | Lower |
| Halstead Effort | 55846.6229 | **15961.7988 ✅** | 22722.8394 | +39884.8241 | Higher | +33123.7834 | Higher | -6761.0406 | Lower |
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
| MI (original) | 54.2013 | **71.3405 ✅** | 56.2102 | -17.1392 | Lower | -2.0089 | Lower | +15.1303 | Higher |
| MI (SEI) | 12.3846 | **71.7536 ✅** | 19.0191 | -59.3691 | Lower | -6.6345 | Lower | +52.7345 | Higher |
| Cyclomatic (sum) | 15 | **9 ✅** | 14 | +6 | Higher | +1 | Higher | -5 | Lower |
| Cognitive (sum) | 15 | **6 ✅** | 14 | +9 | Higher | +1 | Higher | -8 | Lower |
| Halstead Volume | 2856.2459 | **873.5091 ✅** | 2548.3960 | +1982.7368 | Higher | +307.8499 | Higher | -1674.8870 | Lower |
| Halstead Effort | 158720.9196 | **22960.8096 ✅** | 140333.9701 | +135760.1101 | Higher | +18386.9496 | Higher | -117373.1605 | Lower |
| SLOC | 85 | **47 ✅** | 79 | +38 | Higher | +6 | Higher | -32 | Lower |
| PLOC | 69 | **24 ✅** | 63 | +45 | Higher | +6 | Higher | -39 | Lower |
| LLOC | 61 | **11 ✅** | 57 | +50 | Higher | +4 | Higher | -46 | Lower |
| Comments (CLOC) | 1 | **22 ✅** | 2 | -21 | Lower | -1 | Lower | +20 | Higher |
| Blank Lines | 16 | **9 ✅** | 16 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03544

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.4240 ✅** | 82.2117 | 36.3881 | +5.2123 | Higher | +51.0359 | Higher | +45.8236 | Higher |
| MI (SEI) | 84.4934 | **86.6643 ✅** | 1.3085 | -2.1709 | Lower | +83.1849 | Higher | +85.3558 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 38 | -2 | Lower | -34 | Lower | -32 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 34 | -1 | Lower | -32 | Lower | -31 | Lower |
| Halstead Volume | 526.1548 | **450.8319 ✅** | 4512.2112 | +75.3229 | Higher | -3986.0564 | Lower | -4061.3794 | Lower |
| Halstead Effort | 11385.4050 | **9580.1771 ✅** | 429562.5081 | +1805.2279 | Higher | -418177.1030 | Lower | -419982.3309 | Lower |
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
| MI (original) | 42.4641 | **73.5260 ✅** | 41.1795 | -31.0619 | Lower | +1.2847 | Higher | +32.3465 | Higher |
| MI (SEI) | -1.1215 | **64.3771 ✅** | 5.0599 | -65.4987 | Lower | -6.1815 | Lower | +59.3172 | Higher |
| Cyclomatic (sum) | 32 | **8 ✅** | 33 | +24 | Higher | -1 | Lower | -25 | Lower |
| Cognitive (sum) | 52 | **4 ✅** | 50 | +48 | Higher | +2 | Higher | -46 | Lower |
| Halstead Volume | 4755.4489 | **917.7708 ✅** | 4860.2150 | +3837.6781 | Higher | -104.7661 | Lower | -3942.4442 | Lower |
| Halstead Effort | 478318.9018 | **17835.3458 ✅** | 537971.7933 | +460483.5560 | Higher | -59652.8915 | Lower | -520136.4476 | Lower |
| SLOC | 117 | **41 ✅** | 124 | +76 | Higher | -7 | Lower | -83 | Lower |
| PLOC | 104 | **24 ✅** | 107 | +80 | Higher | -3 | Lower | -83 | Lower |
| LLOC | 83 | **16 ✅** | 90 | +67 | Higher | -7 | Lower | -74 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 7 | -7 | Lower | -5 | Lower | +2 | Higher |
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
| MI (original) | 48.9330 | 51.3780 | **53.7250 ✅** | -2.4450 | Lower | -4.7920 | Lower | -2.3470 | Lower |
| MI (SEI) | 9.9970 | **26.0815 ✅** | 20.5159 | -16.0845 | Lower | -10.5189 | Lower | +5.5656 | Higher |
| Cyclomatic (sum) | 20 | 21 | **18 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 27 | **20 ✅** | 30 | +7 | Higher | -3 | Lower | -10 | Lower |
| Halstead Volume | 3466.3018 | **2946.2312 ✅** | 3185.3957 | +520.0705 | Higher | +280.9061 | Higher | -239.1645 | Lower |
| Halstead Effort | **141910.3938 ✅** | 150810.2116 | 169952.5145 | -8899.8179 | Lower | -28042.1208 | Lower | -19142.3029 | Lower |
| SLOC | 103 | 92 | **81 ✅** | +11 | Higher | +22 | Higher | +11 | Higher |
| PLOC | 89 | 73 | **69 ✅** | +16 | Higher | +20 | Higher | +4 | Higher |
| LLOC | 54 | **44 ✅** | 54 | +10 | Higher | 0 | Same | -10 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 4 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 14 | 14 | **12 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03626

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.0915 | **86.0752 ✅** | 71.1478 | -22.9837 | Lower | -8.0563 | Lower | +14.9274 | Higher |
| MI (SEI) | 42.5609 | **92.3300 ✅** | 27.8601 | -49.7691 | Lower | +14.7008 | Higher | +64.4699 | Higher |
| Cyclomatic (sum) | 10 | **5 ✅** | 9 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 16 | **3 ✅** | 15 | +13 | Higher | +1 | Higher | -12 | Lower |
| Halstead Volume | 1639.0421 | **497.5405 ✅** | 1195.9332 | +1141.5016 | Higher | +443.1089 | Higher | -698.3927 | Lower |
| Halstead Effort | 59601.5307 | **10339.5125 ✅** | 42479.5472 | +49262.0182 | Higher | +17121.9835 | Higher | -32140.0346 | Lower |
| SLOC | 63 | **24 ✅** | 43 | +39 | Higher | +20 | Higher | -19 | Lower |
| PLOC | 45 | **17 ✅** | 36 | +28 | Higher | +9 | Higher | -19 | Lower |
| LLOC | 42 | **10 ✅** | 34 | +32 | Higher | +8 | Higher | -24 | Lower |
| Comments (CLOC) | 8 | **11 ✅** | 0 | -3 | Lower | +8 | Higher | +11 | Higher |
| Blank Lines | 11 | **4 ✅** | 7 | +7 | Higher | +4 | Higher | -3 | Lower |
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
| MI (original) | 68.5492 | **73.0357 ✅** | 66.1468 | -4.4865 | Lower | +2.4025 | Higher | +6.8890 | Higher |
| MI (SEI) | 55.3769 | **65.0410 ✅** | 53.1358 | -9.6641 | Lower | +2.2410 | Higher | +11.9051 | Higher |
| Cyclomatic (sum) | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 15 | **7 ✅** | 16 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 983.1622 | **788.7516 ✅** | 1178.4271 | +194.4106 | Higher | -195.2649 | Lower | -389.6755 | Lower |
| Halstead Effort | 21483.9153 | **13145.8603 ✅** | 37120.4533 | +8338.0550 | Higher | -15636.5379 | Lower | -23974.5930 | Lower |
| SLOC | 53 | **45 ✅** | 58 | +8 | Higher | -5 | Lower | -13 | Lower |
| PLOC | 39 | **27 ✅** | 43 | +12 | Higher | -4 | Lower | -16 | Lower |
| LLOC | 29 | **17 ✅** | 31 | +12 | Higher | -2 | Lower | -14 | Lower |
| Comments (CLOC) | 10 | 11 | **12 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03701

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.9830 | **71.8293 ✅** | 69.7747 | -1.8463 | Lower | +0.2082 | Higher | +2.0546 | Higher |
| MI (SEI) | 56.6256 | **64.5070 ✅** | 47.5019 | -7.8814 | Lower | +9.1237 | Higher | +17.0051 | Higher |
| Cyclomatic (sum) | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | 977.5499 | **813.4553 ✅** | 1105.4886 | +164.0945 | Higher | -127.9388 | Lower | -292.0333 | Lower |
| Halstead Effort | 25233.0055 | **21556.5657 ✅** | 35040.0420 | +3676.4398 | Higher | -9807.0364 | Lower | -13483.4762 | Lower |
| SLOC | 50 | **46 ✅** | 48 | +4 | Higher | +2 | Higher | -2 | Lower |
| PLOC | 34 | **29 ✅** | 38 | +5 | Higher | -4 | Lower | -9 | Lower |
| LLOC | 27 | **20 ✅** | 30 | +7 | Higher | -3 | Lower | -10 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 4 | -3 | Lower | +5 | Higher | +8 | Higher |
| Blank Lines | 10 | **7 ✅** | 8 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03709

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.6483 | **75.9722 ✅** | 19.8733 | -38.3240 | Lower | +17.7750 | Higher | +56.0990 | Higher |
| MI (SEI) | -18.1276 | **72.2835 ✅** | -35.5217 | -90.4112 | Lower | +17.3940 | Higher | +107.8052 | Higher |
| Cyclomatic (sum) | 32 | **5 ✅** | 64 | +27 | Higher | -32 | Lower | -59 | Lower |
| Cognitive (sum) | 50 | **3 ✅** | 82 | +47 | Higher | -32 | Lower | -79 | Lower |
| Halstead Volume | 6287.3518 | **765.1033 ✅** | 9487.8492 | +5522.2485 | Higher | -3200.4974 | Lower | -8722.7459 | Lower |
| Halstead Effort | 637805.4597 | **14969.4130 ✅** | 1169527.5394 | +622836.0468 | Higher | -531722.0797 | Lower | -1154558.1264 | Lower |
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
| MI (original) | 57.8183 | **67.5024 ✅** | 58.5999 | -9.6841 | Lower | -0.7816 | Lower | +8.9025 | Higher |
| MI (SEI) | 8.8333 | **57.0513 ✅** | 9.9610 | -48.2179 | Lower | -1.1277 | Lower | +47.0903 | Higher |
| Cyclomatic (sum) | 11 | **9 ✅** | 11 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 2978.8315 | **1185.6915 ✅** | 2932.0917 | +1793.1401 | Higher | +46.7398 | Higher | -1746.4003 | Lower |
| Halstead Effort | 170665.2513 | **39635.9717 ✅** | 164197.1374 | +131029.2796 | Higher | +6468.1139 | Higher | -124561.1657 | Lower |
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
| MI (original) | 65.1181 | **72.6557 ✅** | 69.6659 | -7.5375 | Lower | -4.5478 | Lower | +2.9898 | Higher |
| MI (SEI) | 45.4096 | **70.6722 ✅** | 57.4540 | -25.2626 | Lower | -12.0444 | Lower | +13.2182 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 7 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 4 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | 1771.2433 | **994.2714 ✅** | 1507.9324 | +776.9718 | Higher | +263.3108 | Higher | -513.6610 | Lower |
| Halstead Effort | 81420.0527 | **21589.8934 ✅** | 56366.5136 | +59830.1593 | Higher | +25053.5390 | Higher | -34776.6203 | Lower |
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
| MI (original) | 60.4869 | 60.7344 | **62.0610 ✅** | -0.2475 | Lower | -1.5741 | Lower | -1.3266 | Lower |
| MI (SEI) | **43.4717 ✅** | 42.0093 | 24.9319 | +1.4623 | Higher | +18.5398 | Higher | +17.0774 | Higher |
| Cyclomatic (sum) | 14 | 16 | **13 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 22 | **17 ✅** | 22 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1870.6805 | **1304.7622 ✅** | 1839.4060 | +565.9183 | Higher | +31.2745 | Higher | -534.6438 | Lower |
| Halstead Effort | 114828.6041 | **31737.4593 ✅** | 135833.0609 | +83091.1448 | Higher | -21004.4568 | Lower | -104095.6016 | Lower |
| SLOC | 67 | 72 | **62 ✅** | -5 | Lower | +5 | Higher | +10 | Higher |
| PLOC | 53 | 53 | 54 | 0 | Same | -1 | Lower | -1 | Lower |
| LLOC | 46 | **24 ✅** | 45 | +22 | Higher | +1 | Higher | -21 | Lower |
| Comments (CLOC) | **12 ✅** | 11 | 1 | +1 | Higher | +11 | Higher | +10 | Higher |
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
| MI (original) | 66.3810 | **80.7037 ✅** | 65.5062 | -14.3227 | Lower | +0.8748 | Higher | +15.1975 | Higher |
| MI (SEI) | 42.4225 | **79.7723 ✅** | 45.2300 | -37.3498 | Lower | -2.8075 | Lower | +34.5423 | Higher |
| Cyclomatic (sum) | 15 | **5 ✅** | 15 | +10 | Higher | 0 | Same | -10 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 14 | +11 | Higher | 0 | Same | -11 | Lower |
| Halstead Volume | 1268.9244 | **629.7513 ✅** | 1414.9069 | +639.1731 | Higher | -145.9825 | Lower | -785.1556 | Lower |
| Halstead Effort | 36640.1928 | **12595.0260 ✅** | 45630.7469 | +24045.1667 | Higher | -8990.5541 | Lower | -33035.7209 | Lower |
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
| MI (original) | 68.9537 | **76.3443 ✅** | 71.7588 | -7.3906 | Lower | -2.8051 | Lower | +4.5855 | Higher |
| MI (SEI) | 55.3023 | **79.1622 ✅** | 60.1452 | -23.8599 | Lower | -4.8429 | Lower | +19.0170 | Higher |
| Cyclomatic (sum) | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 1326.3062 | **802.9035 ✅** | 1347.5304 | +523.4027 | Higher | -21.2242 | Lower | -544.6269 | Lower |
| Halstead Effort | 36994.4695 | **18318.0948 ✅** | 36888.6437 | +18676.3746 | Higher | +105.8258 | Higher | -18570.5488 | Lower |
| SLOC | 49 | **37 ✅** | 41 | +12 | Higher | +8 | Higher | -4 | Lower |
| PLOC | 34 | **21 ✅** | 27 | +13 | Higher | +7 | Higher | -6 | Lower |
| LLOC | 21 | **16 ✅** | 23 | +5 | Higher | -2 | Lower | -7 | Lower |
| Comments (CLOC) | 9 | **18 ✅** | 8 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03865

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.4020 ✅** | 81.9830 | 87.6610 | +6.4191 | Higher | +0.7410 | Higher | -5.6780 | Lower |
| MI (SEI) | 85.2998 | 80.1404 | **86.8382 ✅** | +5.1593 | Higher | -1.5384 | Lower | -6.6977 | Lower |
| Cyclomatic (sum) | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **379.5669 ✅** | 492.4112 | 418.7639 | -112.8442 | Lower | -39.1970 | Lower | +73.6472 | Higher |
| Halstead Effort | **5018.7185 ✅** | 5088.2488 | 5620.2527 | -69.5303 | Lower | -601.5342 | Lower | -532.0039 | Lower |
| SLOC | 23 | 31 | 23 | -8 | Lower | 0 | Same | +8 | Higher |
| PLOC | **14 ✅** | 19 | 15 | -5 | Lower | -1 | Lower | +4 | Higher |
| LLOC | 8 | 12 | **7 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Comments (CLOC) | 5 | **9 ✅** | 6 | -4 | Lower | -1 | Lower | +3 | Higher |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03899

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.7201 | **71.7221 ✅** | 62.0597 | -10.0020 | Lower | -0.3396 | Lower | +9.6624 | Higher |
| MI (SEI) | 33.8834 | **69.5290 ✅** | 28.9068 | -35.6456 | Lower | +4.9767 | Higher | +40.6223 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 11 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 13 | **10 ✅** | 11 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | 2159.5278 | **1089.0677 ✅** | 2226.2085 | +1070.4601 | Higher | -66.6807 | Lower | -1137.1408 | Lower |
| Halstead Effort | 89924.9505 | **40295.5044 ✅** | 89922.9202 | +49629.4460 | Higher | +2.0303 | Higher | -49627.4157 | Lower |
| SLOC | 61 | **44 ✅** | 60 | +17 | Higher | +1 | Higher | -16 | Lower |
| PLOC | 48 | **30 ✅** | 49 | +18 | Higher | -1 | Lower | -19 | Lower |
| LLOC | 37 | **24 ✅** | 36 | +13 | Higher | +1 | Higher | -12 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 2 | -13 | Lower | +2 | Higher | +15 | Higher |
| Blank Lines | 9 | **6 ✅** | 11 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03901

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.7689 | **77.5472 ✅** | 57.6165 | -5.7783 | Lower | +14.1524 | Higher | +19.9306 | Higher |
| MI (SEI) | 63.6121 | **74.3302 ✅** | 41.0404 | -10.7182 | Lower | +22.5717 | Higher | +33.2899 | Higher |
| Cyclomatic (sum) | **5 ✅** | 9 | 9 | -4 | Lower | -4 | Lower | 0 | Same |
| Cognitive (sum) | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |
| Halstead Volume | 960.1154 | **726.0704 ✅** | 1931.1400 | +234.0449 | Higher | -971.0246 | Lower | -1205.0696 | Lower |
| Halstead Effort | 21904.8547 | **14661.0378 ✅** | 59906.4280 | +7243.8169 | Higher | -38001.5733 | Lower | -45245.3902 | Lower |
| SLOC | 47 | **34 ✅** | 85 | +13 | Higher | -38 | Lower | -51 | Lower |
| PLOC | 28 | **19 ✅** | 58 | +9 | Higher | -30 | Lower | -39 | Lower |
| LLOC | 17 | **9 ✅** | 36 | +8 | Higher | -19 | Lower | -27 | Lower |
| Comments (CLOC) | 12 | 10 | **18 ✅** | +2 | Higher | -6 | Lower | -8 | Lower |
| Blank Lines | 10 | **7 ✅** | 13 | +3 | Higher | -3 | Lower | -6 | Lower |
| Functions | 2 | **1 ✅** | 5 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

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
| MI (original) | 41.3662 | **59.5905 ✅** | 37.5364 | -18.2243 | Lower | +3.8298 | Higher | +22.0541 | Higher |
| MI (SEI) | 14.4501 | **42.8707 ✅** | 13.1167 | -28.4206 | Lower | +1.3335 | Higher | +29.7540 | Higher |
| Cyclomatic (sum) | 22 | **12 ✅** | 26 | +10 | Higher | -4 | Lower | -14 | Lower |
| Cognitive (sum) | 19 | **8 ✅** | 26 | +11 | Higher | -7 | Lower | -18 | Lower |
| Halstead Volume | 5225.9077 | **1858.8287 ✅** | 5586.0320 | +3367.0791 | Higher | -360.1243 | Lower | -3727.2033 | Lower |
| Halstead Effort | 453190.7199 | **92941.4332 ✅** | 482633.1657 | +360249.2867 | Higher | -29442.4459 | Lower | -389691.7326 | Lower |
| SLOC | 140 | **73 ✅** | 164 | +67 | Higher | -24 | Lower | -91 | Lower |
| PLOC | 103 | **45 ✅** | 111 | +58 | Higher | -8 | Lower | -66 | Lower |
| LLOC | 68 | **26 ✅** | 73 | +42 | Higher | -5 | Lower | -47 | Lower |
| Comments (CLOC) | 21 | 14 | **33 ✅** | +7 | Higher | -12 | Lower | -19 | Lower |
| Blank Lines | 21 | **15 ✅** | 27 | +6 | Higher | -6 | Lower | -12 | Lower |
| Functions | 5 | **2 ✅** | 7 | +3 | Higher | -2 | Lower | -5 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |

## p03951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.7204 | **74.5884 ✅** | 73.3889 | -10.8680 | Lower | -9.6686 | Lower | +1.1994 | Higher |
| MI (SEI) | 51.7016 | **76.7933 ✅** | 48.4921 | -25.0917 | Lower | +3.2095 | Higher | +28.3013 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 9 | +5 | Higher | +3 | Higher | -2 | Lower |
| Cognitive (sum) | 23 | **3 ✅** | 14 | +20 | Higher | +9 | Higher | -11 | Lower |
| Halstead Volume | 1630.8091 | **585.1497 ✅** | 1142.3054 | +1045.6593 | Higher | +488.5036 | Higher | -557.1557 | Lower |
| Halstead Effort | 67225.5731 | **16676.7670 ✅** | 44386.7258 | +50548.8061 | Higher | +22838.8472 | Higher | -27709.9589 | Lower |
| SLOC | 59 | 45 | **38 ✅** | +14 | Higher | +21 | Higher | +7 | Higher |
| PLOC | 44 | **24 ✅** | 33 | +20 | Higher | +11 | Higher | -9 | Lower |
| LLOC | 45 | **12 ✅** | 31 | +33 | Higher | +14 | Higher | -19 | Lower |
| Comments (CLOC) | 14 | **22 ✅** | 2 | -8 | Lower | +12 | Higher | +20 | Higher |
| Blank Lines | 9 | 8 | **5 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p03977

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.1542 | **78.4969 ✅** | 58.8354 | -9.3427 | Lower | +10.3187 | Higher | +19.6615 | Higher |
| MI (SEI) | 56.2690 | **79.1594 ✅** | 34.6926 | -22.8904 | Lower | +21.5764 | Higher | +44.4668 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 14 | +4 | Higher | -3 | Lower | -7 | Lower |
| Cognitive (sum) | 13 | **6 ✅** | 19 | +7 | Higher | -6 | Lower | -13 | Lower |
| Halstead Volume | 1140.1575 | **660.8156 ✅** | 1735.3797 | +479.3419 | Higher | -595.2222 | Lower | -1074.5641 | Lower |
| Halstead Effort | 37930.5966 | **10462.9141 ✅** | 90127.7844 | +27467.6825 | Higher | -52197.1878 | Lower | -79664.8703 | Lower |
| SLOC | 48 | **34 ✅** | 76 | +14 | Higher | -28 | Lower | -42 | Lower |
| PLOC | 31 | **23 ✅** | 58 | +8 | Higher | -27 | Lower | -35 | Lower |
| LLOC | 25 | **15 ✅** | 53 | +10 | Higher | -28 | Lower | -38 | Lower |
| Comments (CLOC) | 9 | **13 ✅** | 8 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 9 | **5 ✅** | 10 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03996

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.7074 | **73.8198 ✅** | 54.7745 | -17.1123 | Lower | +1.9329 | Higher | +19.0452 | Higher |
| MI (SEI) | 29.7850 | **73.7224 ✅** | 23.7245 | -43.9374 | Lower | +6.0605 | Higher | +49.9979 | Higher |
| Cyclomatic (sum) | 14 | **9 ✅** | 16 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 19 | **7 ✅** | 18 | +12 | Higher | +1 | Higher | -11 | Lower |
| Halstead Volume | 2316 | **829.8336 ✅** | 2843.9873 | +1486.1664 | Higher | -527.9873 | Lower | -2014.1537 | Lower |
| Halstead Effort | 96500 | **24065.1746 ✅** | 148858.4583 | +72434.8254 | Higher | -52358.4583 | Lower | -124793.2837 | Lower |
| SLOC | 79 | **41 ✅** | 81 | +38 | Higher | -2 | Lower | -40 | Lower |
| PLOC | 64 | **24 ✅** | 65 | +40 | Higher | -1 | Lower | -41 | Lower |
| LLOC | 48 | **16 ✅** | 55 | +32 | Higher | -7 | Lower | -39 | Lower |
| Comments (CLOC) | 7 | **17 ✅** | 5 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 10 | **8 ✅** | 11 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |

## p04005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.1563 | **77.4090 ✅** | 74.9200 | -0.2528 | Lower | +2.2363 | Higher | +2.4891 | Higher |
| MI (SEI) | 49.8496 | **73.0253 ✅** | 62.4086 | -23.1756 | Lower | -12.5590 | Lower | +10.6166 | Higher |
| Cyclomatic (sum) | 9 | **6 ✅** | 8 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 859.0506 | **712.5395 ✅** | 966.4985 | +146.5111 | Higher | -107.4479 | Lower | -253.9590 | Lower |
| Halstead Effort | 21278.0227 | **11671.3969 ✅** | 29342.8933 | +9606.6258 | Higher | -8064.8705 | Lower | -17671.4964 | Lower |
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
| MI (original) | 65.1349 | **70.8451 ✅** | 64.3384 | -5.7101 | Lower | +0.7966 | Higher | +6.5067 | Higher |
| MI (SEI) | 19.8981 | **61.8831 ✅** | 29.7245 | -41.9850 | Lower | -9.8264 | Lower | +32.1586 | Higher |
| Cyclomatic (sum) | 16 | **10 ✅** | 16 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 18 | **8 ✅** | 18 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 1979.6813 | **982.9534 ✅** | 2163.8377 | +996.7279 | Higher | -184.1564 | Lower | -1180.8843 | Lower |
| Halstead Effort | 118610.2159 | **41705.3085 ✅** | 124047.5923 | +76904.9073 | Higher | -5437.3765 | Lower | -82342.2838 | Lower |
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
| MI (original) | 72.3837 | 68.4559 | **80.1430 ✅** | +3.9277 | Higher | -7.7593 | Lower | -11.6870 | Lower |
| MI (SEI) | 59.4059 | 58.2140 | **68.1644 ✅** | +1.1919 | Higher | -8.7585 | Lower | -9.9504 | Lower |
| Cyclomatic (sum) | 8 | 10 | **6 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 8 | 9 | **5 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Halstead Volume | 1143.2464 | 1128.4003 | **921.5188 ✅** | +14.8461 | Higher | +221.7276 | Higher | +206.8815 | Higher |
| Halstead Effort | 38633.8453 | 34552.3963 | **33761.0983 ✅** | +4081.4490 | Higher | +4872.7470 | Higher | +791.2980 | Higher |
| SLOC | 41 | 51 | **28 ✅** | -10 | Lower | +13 | Higher | +23 | Higher |
| PLOC | 28 | 33 | **23 ✅** | -5 | Lower | +5 | Higher | +10 | Higher |
| LLOC | 24 | 25 | **22 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Comments (CLOC) | 7 | **12 ✅** | 4 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 7 | 9 | **4 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p04022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.0466 | **59.9601 ✅** | 4.2885 | -25.9135 | Lower | +29.7581 | Higher | +55.6716 | Higher |
| MI (SEI) | -23.0183 | **41.9150 ✅** | -50.4404 | -64.9333 | Lower | +27.4221 | Higher | +92.3555 | Higher |
| Cyclomatic (sum) | 35 | **17 ✅** | 88 | +18 | Higher | -53 | Lower | -71 | Lower |
| Cognitive (sum) | 72 | **27 ✅** | 101 | +45 | Higher | -29 | Lower | -74 | Lower |
| Halstead Volume | 5128.5830 | **1387.7925 ✅** | 12876.6946 | +3740.7905 | Higher | -7748.1115 | Lower | -11488.9020 | Lower |
| Halstead Effort | 449077.3789 | **43531.7865 ✅** | 1469400.9191 | +405545.5924 | Higher | -1020323.5402 | Lower | -1425869.1326 | Lower |
| SLOC | 184 | **73 ✅** | 405 | +111 | Higher | -221 | Lower | -332 | Lower |
| PLOC | 161 | **50 ✅** | 352 | +111 | Higher | -191 | Lower | -302 | Lower |
| LLOC | 138 | **31 ✅** | 226 | +107 | Higher | -88 | Lower | -195 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 7 | -12 | Lower | -7 | Lower | +5 | Higher |
| Blank Lines | 23 | **12 ✅** | 51 | +11 | Higher | -28 | Lower | -39 | Lower |
| Functions | 3 | 3 | 36 | 0 | Same | -33 | Lower | -33 | Lower |
| Closures | **0 ✅** | 1 | 4 | -1 | Lower | -4 | Lower | -3 | Lower |
| Exits | 4 | 4 | 34 | 0 | Same | -30 | Lower | -30 | Lower |

## p04024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.4966 | **64.1662 ✅** | 40.7814 | -22.6696 | Lower | +0.7152 | Higher | +23.3847 | Higher |
| MI (SEI) | -5.4660 | **51.1423 ✅** | -13.2002 | -56.6082 | Lower | +7.7342 | Higher | +64.3425 | Higher |
| Cyclomatic (sum) | 34 | **9 ✅** | 36 | +25 | Higher | -2 | Lower | -27 | Lower |
| Cognitive (sum) | 45 | **9 ✅** | 47 | +36 | Higher | -2 | Lower | -38 | Lower |
| Halstead Volume | 4266.7191 | **1393.3077 ✅** | 4371.4552 | +2873.4114 | Higher | -104.7361 | Lower | -2978.1475 | Lower |
| Halstead Effort | 309337.1313 | **42068.9032 ✅** | 371865.1197 | +267268.2281 | Higher | -62527.9885 | Lower | -329796.2166 | Lower |
| SLOC | 125 | **63 ✅** | 126 | +62 | Higher | -1 | Lower | -63 | Lower |
| PLOC | 105 | **38 ✅** | 108 | +67 | Higher | -3 | Lower | -70 | Lower |
| LLOC | 79 | **26 ✅** | 84 | +53 | Higher | -5 | Lower | -58 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 0 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | 19 | **12 ✅** | 18 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
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
| MI (original) | 60.7939 | **67.8495 ✅** | 59.4301 | -7.0556 | Lower | 1.3639 | Higher | 8.4194 | Higher |
| MI (SEI) | 33.2538 | **56.7372 ✅** | 28.7141 | -23.4834 | Lower | 4.5397 | Higher | 28.0231 | Higher |
| Cyclomatic (sum) | 16.6367 | **11.4300 ✅** | 17.9233 | 5.2067 | Higher | -1.2867 | Lower | -6.4933 | Lower |
| Cognitive (sum) | 21.2733 | **10.7800 ✅** | 23.0700 | 10.4933 | Higher | -1.7967 | Lower | -12.2900 | Lower |
| Halstead Volume | 2509.7307 | **1345.9156 ✅** | 2821.5584 | 1163.8151 | Higher | -311.8277 | Lower | -1475.6428 | Lower |
| Halstead Effort | 173414.5769 | **48977.6069 ✅** | 230368.0512 | 124436.9700 | Higher | -56953.4742 | Lower | -181390.4442 | Lower |
| SLOC | 75.5767 | **56.2600 ✅** | 83.2533 | 19.3167 | Higher | -7.6767 | Lower | -26.9933 | Lower |
| PLOC | 61.3933 | **38.6133 ✅** | 69.8133 | 22.7800 | Higher | -8.4200 | Lower | -31.2000 | Lower |
| LLOC | 44 | **22.3467 ✅** | 52.9233 | 21.6533 | Higher | -8.9233 | Lower | -30.5767 | Lower |
| Comments (CLOC) | 4.8700 | **12.3333 ✅** | 4.0233 | -7.4633 | Lower | 0.8467 | Higher | 8.3100 | Higher |
| Blank Lines | 11.1033 | **8.9333 ✅** | 11.2200 | 2.1700 | Higher | -0.1167 | Lower | -2.2867 | Lower |
| Functions | 2.5500 | **2.3767 ✅** | 3.2300 | 0.1733 | Higher | -0.6800 | Lower | -0.8533 | Lower |
| Closures | 0.7733 | 0.6667 | **0.4667 ✅** | 0.1067 | Higher | 0.3067 | Higher | 0.2000 | Higher |
| Exits | **2.0767 ✅** | 2.1100 | 2.4733 | -0.0333 | Lower | -0.3967 | Lower | -0.3633 | Lower |

