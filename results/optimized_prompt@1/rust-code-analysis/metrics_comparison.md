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
| MI (original) | 76.3162 | 80.0268 | **80.5028 ✅** | -3.7106 | Lower | -4.1866 | Lower | -0.4760 | Lower |
| MI (SEI) | 72.6283 | **78.2244 ✅** | 77.1225 | -5.5961 | Lower | -4.4942 | Lower | +1.1019 | Higher |
| Cyclomatic (sum) | **7 ✅** | 8 | 8 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | **10 ✅** | 21 | 12 | -11 | Lower | -2 | Lower | +9 | Higher |
| Halstead Volume | 1484.4031 | **516.9925 ✅** | 634.8730 | +967.4106 | Higher | +849.5301 | Higher | -117.8805 | Lower |
| Halstead Effort | 20393.2824 | **6824.3010 ✅** | 14632.3105 | +13568.9814 | Higher | +5760.9719 | Higher | -7808.0095 | Lower |
| SLOC | 30 | 33 | 30 | -3 | Lower | 0 | Same | +3 | Higher |
| PLOC | 25 | **21 ✅** | 25 | +4 | Higher | 0 | Same | -4 | Lower |
| LLOC | **10 ✅** | 11 | 17 | -1 | Lower | -7 | Lower | -6 | Lower |
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
| MI (original) | 72.3999 | 85.4813 | **93.3639 ✅** | -13.0814 | Lower | -20.9640 | Lower | -7.8826 | Lower |
| MI (SEI) | 57.5524 | **92.1105 ✅** | 90.4835 | -34.5581 | Lower | -32.9311 | Lower | +1.6270 | Higher |
| Cyclomatic (sum) | 8 | 5 | **4 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Cognitive (sum) | 6 | 6 | **3 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 2042.0998 | **386.4273 ✅** | 452.7842 | +1655.6725 | Higher | +1589.3156 | Higher | -66.3569 | Lower |
| Halstead Effort | 35098.5899 | **4890.7207 ✅** | 7018.1550 | +30207.8692 | Higher | +28080.4349 | Higher | -2127.4343 | Lower |
| SLOC | 34 | 27 | **16 ✅** | +7 | Higher | +18 | Higher | +11 | Higher |
| PLOC | 27 | 14 | **13 ✅** | +13 | Higher | +14 | Higher | +1 | Higher |
| LLOC | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 5 | **13 ✅** | 3 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |
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
| MI (original) | 61.2400 | 55.2866 | **66.7390 ✅** | +5.9534 | Higher | -5.4990 | Lower | -11.4524 | Lower |
| MI (SEI) | **41.3817 ✅** | 40.5596 | 21.9068 | +0.8221 | Higher | +19.4749 | Higher | +18.6528 | Higher |
| Cyclomatic (sum) | 17 | 15 | **13 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 18 | **14 ✅** | 15 | +4 | Higher | +3 | Higher | -1 | Lower |
| Halstead Volume | 2106.2621 | 2800.1149 | **2030.3809 ✅** | -693.8528 | Lower | +75.8812 | Higher | +769.7340 | Higher |
| Halstead Effort | 79755.4141 | 222959.1502 | **72452.5410 ✅** | -143203.7361 | Lower | +7302.8731 | Higher | +150506.6092 | Higher |
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
| MI (original) | **55.8256 ✅** | 53.3569 | 49.7296 | +2.4687 | Higher | +6.0959 | Higher | +3.6272 | Higher |
| MI (SEI) | 26.4697 | **39.3233 ✅** | -1.1052 | -12.8536 | Lower | +27.5749 | Higher | +40.4285 | Higher |
| Cyclomatic (sum) | **20 ✅** | 21 | 28 | -1 | Lower | -8 | Lower | -7 | Lower |
| Cognitive (sum) | 26 | 26 | 34 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | 2579.8707 | **2484.4978 ✅** | 3408.8630 | +95.3729 | Higher | -828.9922 | Lower | -924.3651 | Lower |
| Halstead Effort | 213463.4985 | **121799.5487 ✅** | 381918.9047 | +91663.9498 | Higher | -168455.4063 | Lower | -260119.3561 | Lower |
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
| MI (original) | 68.5739 | 57.9157 | **68.6869 ✅** | +10.6582 | Higher | -0.1130 | Lower | -10.7712 | Lower |
| MI (SEI) | 39.2902 | 39.2042 | **45.6237 ✅** | +0.0861 | Higher | -6.3334 | Lower | -6.4195 | Lower |
| Cyclomatic (sum) | 8 | 16 | 8 | -8 | Lower | 0 | Same | +8 | Higher |
| Cognitive (sum) | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **1281.7953 ✅** | 1820.1279 | 1335.7318 | -538.3326 | Lower | -53.9366 | Lower | +484.3961 | Higher |
| Halstead Effort | **27086.3577 ✅** | 58921.3489 | 30225.7027 | -31834.9912 | Lower | -3139.3450 | Lower | +28695.6462 | Higher |
| SLOC | 50 | 77 | **49 ✅** | -27 | Lower | +1 | Higher | +28 | Higher |
| PLOC | **37 ✅** | 58 | 39 | -21 | Lower | -2 | Lower | +19 | Higher |
| LLOC | **25 ✅** | 29 | 27 | -4 | Lower | -2 | Lower | +2 | Higher |
| Comments (CLOC) | 2 | **13 ✅** | 4 | -11 | Lower | -2 | Lower | +9 | Higher |
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
| MI (original) | 51.2958 | **64.7763 ✅** | 53.0859 | -13.4804 | Lower | -1.7901 | Lower | +11.6903 | Higher |
| MI (SEI) | 0.4416 | **57.9974 ✅** | 2.8205 | -57.5558 | Lower | -2.3789 | Lower | +55.1769 | Higher |
| Cyclomatic (sum) | 21 | **10 ✅** | 19 | +11 | Higher | +2 | Higher | -9 | Lower |
| Cognitive (sum) | 31 | **15 ✅** | 28 | +16 | Higher | +3 | Higher | -13 | Lower |
| Halstead Volume | 3437.7086 | **1454.2390 ✅** | 3193.9495 | +1983.4697 | Higher | +243.7592 | Higher | -1739.7105 | Lower |
| Halstead Effort | 294396.7736 | **77041.6137 ✅** | 235743.8896 | +217355.1600 | Higher | +58652.8840 | Higher | -158702.2759 | Lower |
| SLOC | 88 | **59 ✅** | 83 | +29 | Higher | +5 | Higher | -24 | Lower |
| PLOC | 77 | **36 ✅** | 73 | +41 | Higher | +4 | Higher | -37 | Lower |
| LLOC | 63 | **32 ✅** | 58 | +31 | Higher | +5 | Higher | -26 | Lower |
| Comments (CLOC) | 0 | **20 ✅** | 0 | -20 | Lower | 0 | Same | +20 | Higher |
| Blank Lines | 11 | **8 ✅** | 10 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p00156

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.2833 | **53.6054 ✅** | 49.8088 | -3.3221 | Lower | +0.4745 | Higher | +3.7966 | Higher |
| MI (SEI) | 20.9185 | **29.7269 ✅** | -1.2965 | -8.8084 | Lower | +22.2150 | Higher | +31.0234 | Higher |
| Cyclomatic (sum) | **22 ✅** | 23 | 25 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 57 | **37 ✅** | 56 | +20 | Higher | +1 | Higher | -19 | Lower |
| Halstead Volume | 3253.7536 | **2514.8286 ✅** | 3701.1774 | +738.9250 | Higher | -447.4238 | Lower | -1186.3488 | Lower |
| Halstead Effort | 204986.4745 | **103295.2476 ✅** | 327506.7496 | +101691.2269 | Higher | -122520.2751 | Lower | -224211.5020 | Lower |
| SLOC | 94 | **82 ✅** | 89 | +12 | Higher | +5 | Higher | -7 | Lower |
| PLOC | 77 | **60 ✅** | 78 | +17 | Higher | -1 | Lower | -18 | Lower |
| LLOC | 72 | **47 ✅** | 76 | +25 | Higher | -4 | Lower | -29 | Lower |
| Comments (CLOC) | 8 | **10 ✅** | 0 | -2 | Lower | +8 | Higher | +10 | Higher |
| Blank Lines | 15 | 14 | **11 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00167

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.9008 | 69.2796 | **76.2836 ✅** | +1.6211 | Higher | -5.3828 | Lower | -7.0040 | Lower |
| MI (SEI) | 43.3757 | **54.6627 ✅** | 48.6925 | -11.2869 | Lower | -5.3168 | Lower | +5.9702 | Higher |
| Cyclomatic (sum) | **9 ✅** | 11 | 10 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | **11 ✅** | 12 | 14 | -1 | Lower | -3 | Lower | -2 | Lower |
| Halstead Volume | 1016.4632 | 1043.7263 | **972.0580 ✅** | -27.2631 | Lower | +44.4052 | Higher | +71.6683 | Higher |
| Halstead Effort | **27522.6952 ✅** | 37574.1455 | 27893.8382 | -10051.4503 | Lower | -371.1430 | Lower | +9680.3073 | Higher |
| SLOC | 46 | 49 | **33 ✅** | -3 | Lower | +13 | Higher | +16 | Higher |
| PLOC | 39 | 34 | **29 ✅** | +5 | Higher | +10 | Higher | +5 | Higher |
| LLOC | 21 | 21 | **20 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 2 | **8 ✅** | 1 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 7 | 10 | **4 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

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
| MI (original) | 31.5275 | **56.2272 ✅** | 32.4050 | -24.6997 | Lower | -0.8774 | Lower | +23.8223 | Higher |
| MI (SEI) | -18.5761 | **32.0780 ✅** | -17.8561 | -50.6541 | Lower | -0.7200 | Lower | +49.9341 | Higher |
| Cyclomatic (sum) | 37 | **18 ✅** | 33 | +19 | Higher | +4 | Higher | -15 | Lower |
| Cognitive (sum) | 72 | **15 ✅** | 60 | +57 | Higher | +12 | Higher | -45 | Lower |
| Halstead Volume | 6673.9600 | **2046.3877 ✅** | 6018.5277 | +4627.5724 | Higher | +655.4324 | Higher | -3972.1400 | Lower |
| Halstead Effort | 593982.4440 | **54265.8156 ✅** | 584121.2574 | +539716.6284 | Higher | +9861.1865 | Higher | -529855.4419 | Lower |
| SLOC | 192 | **80 ✅** | 199 | +112 | Higher | -7 | Lower | -119 | Lower |
| PLOC | 169 | **59 ✅** | 177 | +110 | Higher | -8 | Lower | -118 | Lower |
| LLOC | 99 | **25 ✅** | 135 | +74 | Higher | -36 | Lower | -110 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 2 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 21 | **15 ✅** | 20 | +6 | Higher | +1 | Higher | -5 | Lower |
| Functions | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |

## p00203

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.7207 | **66.1956 ✅** | 49.9241 | -16.4749 | Lower | -0.2034 | Lower | +16.2715 | Higher |
| MI (SEI) | -1.3218 | **51.1342 ✅** | -0.6210 | -52.4559 | Lower | -0.7007 | Lower | +51.7552 | Higher |
| Cyclomatic (sum) | 26 | **11 ✅** | 30 | +15 | Higher | -4 | Lower | -19 | Lower |
| Cognitive (sum) | 39 | **6 ✅** | 39 | +33 | Higher | 0 | Same | -33 | Lower |
| Halstead Volume | 3037.6440 | **1179.0891 ✅** | 3228.9158 | +1858.5549 | Higher | -191.2718 | Lower | -2049.8267 | Lower |
| Halstead Effort | 229054.7741 | **35655.6534 ✅** | 252091.6906 | +193399.1207 | Higher | -23036.9165 | Lower | -216436.0372 | Lower |
| SLOC | 94 | **57 ✅** | 86 | +37 | Higher | +8 | Higher | -29 | Lower |
| PLOC | 79 | **38 ✅** | 77 | +41 | Higher | +2 | Higher | -39 | Lower |
| LLOC | 49 | **23 ✅** | 50 | +26 | Higher | -1 | Lower | -27 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 0 | -10 | Lower | 0 | Same | +10 | Higher |
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
| MI (original) | 42.3402 | **50.3972 ✅** | 33.9688 | -8.0570 | Lower | +8.3714 | Higher | +16.4284 | Higher |
| MI (SEI) | -4.3231 | **20.8498 ✅** | -14.9108 | -25.1729 | Lower | +10.5877 | Higher | +35.7606 | Higher |
| Cyclomatic (sum) | 33 | **19 ✅** | 35 | +14 | Higher | -2 | Lower | -16 | Lower |
| Cognitive (sum) | 37 | **17 ✅** | 42 | +20 | Higher | -5 | Lower | -25 | Lower |
| Halstead Volume | 3887.8902 | **2574.9670 ✅** | 5979.1460 | +1312.9232 | Higher | -2091.2558 | Lower | -3404.1789 | Lower |
| Halstead Effort | 197075.8124 | **98572.9561 ✅** | 467868.1706 | +98502.8563 | Higher | -270792.3582 | Lower | -369295.2145 | Lower |
| SLOC | 124 | **105 ✅** | 176 | +19 | Higher | -52 | Lower | -71 | Lower |
| PLOC | 102 | **84 ✅** | 148 | +18 | Higher | -46 | Lower | -64 | Lower |
| LLOC | 48 | **33 ✅** | 94 | +15 | Higher | -46 | Lower | -61 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 2 | -8 | Lower | -1 | Lower | +7 | Higher |
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
| MI (original) | **50.7220 ✅** | 44.2117 | 47.6488 | +6.5102 | Higher | +3.0731 | Higher | -3.4371 | Lower |
| MI (SEI) | 10.8336 | **21.2510 ✅** | 6.3526 | -10.4174 | Lower | +4.4810 | Higher | +14.8985 | Higher |
| Cyclomatic (sum) | **20 ✅** | 43 | 24 | -23 | Lower | -4 | Lower | +19 | Higher |
| Cognitive (sum) | 39 | **23 ✅** | 50 | +16 | Higher | -11 | Lower | -27 | Lower |
| Halstead Volume | 3493.5216 | **3203.0689 ✅** | 4076.3726 | +290.4527 | Higher | -582.8510 | Lower | -873.3037 | Lower |
| Halstead Effort | 252476.8066 | **120410.7524 ✅** | 333493.4292 | +132066.0542 | Higher | -81016.6226 | Lower | -213082.6768 | Lower |
| SLOC | **92 ✅** | 102 | 100 | -10 | Lower | -8 | Lower | +2 | Higher |
| PLOC | **79 ✅** | 84 | 90 | -5 | Lower | -11 | Lower | -6 | Lower |
| LLOC | 88 | **28 ✅** | 106 | +60 | Higher | -18 | Lower | -78 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 2 | -14 | Lower | 0 | Same | +14 | Higher |
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
| MI (original) | 39.7276 | **52.4531 ✅** | 23.5906 | -12.7255 | Lower | +16.1370 | Higher | +28.8625 | Higher |
| MI (SEI) | -5.7210 | **26.4178 ✅** | -30.0214 | -32.1388 | Lower | +24.3005 | Higher | +56.4392 | Higher |
| Cyclomatic (sum) | 32 | **15 ✅** | 46 | +17 | Higher | -14 | Lower | -31 | Lower |
| Cognitive (sum) | 53 | **18 ✅** | 59 | +35 | Higher | -6 | Lower | -41 | Lower |
| Halstead Volume | 5274.5550 | **2649.2449 ✅** | 9529.1959 | +2625.3101 | Higher | -4254.6409 | Lower | -6879.9510 | Lower |
| Halstead Effort | 365262.9312 | **92567.7340 ✅** | 930946.3820 | +272695.1972 | Higher | -565683.4508 | Lower | -838378.6481 | Lower |
| SLOC | 134 | **97 ✅** | 246 | +37 | Higher | -112 | Lower | -149 | Lower |
| PLOC | 114 | **78 ✅** | 217 | +36 | Higher | -103 | Lower | -139 | Lower |
| LLOC | 82 | **40 ✅** | 179 | +42 | Higher | -97 | Lower | -139 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 2 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 20 | **16 ✅** | 27 | +4 | Higher | -7 | Lower | -11 | Lower |
| Functions | **3 ✅** | 5 | 11 | -2 | Lower | -8 | Lower | -6 | Lower |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | **1 ✅** | 3 | 5 | -2 | Lower | -4 | Lower | -2 | Lower |

## p00271

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.8192 | 83.2400 | **89.3166 ✅** | -1.4208 | Lower | -7.4974 | Lower | -6.0766 | Lower |
| MI (SEI) | 72.8391 | 81.6280 | **81.6933 ✅** | -8.7889 | Lower | -8.8542 | Lower | -0.0653 | Lower |
| Cyclomatic (sum) | 5 | 4 | **3 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 625.5158 | 554.9672 | **514.2969 ✅** | +70.5486 | Higher | +111.2189 | Higher | +40.6703 | Higher |
| Halstead Effort | 11509.4907 | 7528.2512 | **5534.2818 ✅** | +3981.2396 | Higher | +5975.2089 | Higher | +1993.9693 | Higher |
| SLOC | 29 | 28 | **20 ✅** | +1 | Higher | +9 | Higher | +8 | Higher |
| PLOC | 19 | 16 | **13 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| LLOC | 11 | 11 | **9 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 5 | **8 ✅** | 3 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00283

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.4942 ✅** | 59.0324 | 58.6306 | +0.4618 | Higher | +0.8636 | Higher | +0.4018 | Higher |
| MI (SEI) | 11.5566 | **38.3666 ✅** | 10.5143 | -26.8101 | Lower | +1.0422 | Higher | +27.8523 | Higher |
| Cyclomatic (sum) | **14 ✅** | 18 | 16 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | 21 | 22 | **20 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 2488.3580 | **2172.6284 ✅** | 2564.2531 | +315.7296 | Higher | -75.8951 | Lower | -391.6247 | Lower |
| Halstead Effort | 144098.5502 | **75718.4117 ✅** | 148726.6791 | +68380.1385 | Higher | -4628.1288 | Lower | -73008.2673 | Lower |
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
| MI (original) | 24.2581 | **46.5082 ✅** | 17.0055 | -22.2501 | Lower | +7.2527 | Higher | +29.5028 | Higher |
| MI (SEI) | -36.2237 | **28.1829 ✅** | -39.4331 | -64.4066 | Lower | +3.2094 | Higher | +67.6160 | Higher |
| Cyclomatic (sum) | 44 | **20 ✅** | 54 | +24 | Higher | -10 | Lower | -34 | Lower |
| Cognitive (sum) | 78 | **38 ✅** | 89 | +40 | Higher | -11 | Lower | -51 | Lower |
| Halstead Volume | 10149.7662 | **2949.0218 ✅** | 11902.9134 | +7200.7444 | Higher | -1753.1473 | Lower | -8953.8917 | Lower |
| Halstead Effort | 1217971.9427 | **146247.4070 ✅** | 1353893.0908 | +1071724.5357 | Higher | -135921.1481 | Lower | -1207645.6838 | Lower |
| SLOC | 238 | **126 ✅** | 307 | +112 | Higher | -69 | Lower | -181 | Lower |
| PLOC | 217 | **92 ✅** | 276 | +125 | Higher | -59 | Lower | -184 | Lower |
| LLOC | 166 | **58 ✅** | 201 | +108 | Higher | -35 | Lower | -143 | Lower |
| Comments (CLOC) | 0 | **31 ✅** | 2 | -31 | Lower | -2 | Lower | +29 | Higher |
| Blank Lines | 21 | **18 ✅** | 29 | +3 | Higher | -8 | Lower | -11 | Lower |
| Functions | 8 | **4 ✅** | 13 | +4 | Higher | -5 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **3 ✅** | 8 | +1 | Higher | -4 | Lower | -5 | Lower |

## p00292

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.4064 ✅** | 71.0474 | 78.4540 | +10.3591 | Higher | +2.9525 | Higher | -7.4066 | Lower |
| MI (SEI) | **74.5400 ✅** | 61.8177 | 72.1633 | +12.7223 | Higher | +2.3767 | Higher | -10.3456 | Lower |
| Cyclomatic (sum) | **4 ✅** | 6 | 10 | -2 | Lower | -6 | Lower | -4 | Lower |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **707.8164 ✅** | 870.2800 | 2264.7457 | -162.4636 | Lower | -1556.9292 | Lower | -1394.4656 | Lower |
| Halstead Effort | **12167.7016 ✅** | 15304.9246 | 32650.0833 | -3137.2230 | Lower | -20482.3817 | Lower | -17345.1587 | Lower |
| SLOC | 29 | 50 | **22 ✅** | -21 | Lower | +7 | Higher | +28 | Higher |
| PLOC | **18 ✅** | 29 | 22 | -11 | Lower | -4 | Lower | +7 | Higher |
| LLOC | 13 | 13 | **11 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 6 | **12 ✅** | 5 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 6 | 9 | **-5 ✅** | -3 | Lower | +11 | Higher | +14 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00315

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.9336 | **58.3860 ✅** | 53.5197 | -4.4524 | Lower | +0.4139 | Higher | +4.8663 | Higher |
| MI (SEI) | 33.2340 | **43.8567 ✅** | 22.0160 | -10.6227 | Lower | +11.2180 | Higher | +21.8408 | Higher |
| Cyclomatic (sum) | 17 | **16 ✅** | 17 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 16 | 19 | 16 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 3457.6199 | **1535.0827 ✅** | 3463.5506 | +1922.5372 | Higher | -5.9307 | Lower | -1928.4679 | Lower |
| Halstead Effort | 268410.0909 | **52960.3525 ✅** | 268870.4855 | +215449.7384 | Higher | -460.3947 | Lower | -215910.1330 | Lower |
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
| MI (original) | 44.4231 | **56.9833 ✅** | 32.7727 | -12.5603 | Lower | +11.6503 | Higher | +24.2106 | Higher |
| MI (SEI) | 1.1534 | **39.4828 ✅** | -12.7754 | -38.3295 | Lower | +13.9287 | Higher | +52.2582 | Higher |
| Cyclomatic (sum) | 27 | **15 ✅** | 40 | +12 | Higher | -13 | Lower | -25 | Lower |
| Cognitive (sum) | 56 | **17 ✅** | 71 | +39 | Higher | -15 | Lower | -54 | Lower |
| Halstead Volume | 3963.8489 | **1801.6016 ✅** | 6032.3203 | +2162.2473 | Higher | -2068.4714 | Lower | -4230.7187 | Lower |
| Halstead Effort | 252442.7701 | **56621.7656 ✅** | 575502.8193 | +195821.0046 | Higher | -323060.0492 | Lower | -518881.0538 | Lower |
| SLOC | 118 | **83 ✅** | 176 | +35 | Higher | -58 | Lower | -93 | Lower |
| PLOC | 106 | **57 ✅** | 155 | +49 | Higher | -49 | Lower | -98 | Lower |
| LLOC | 69 | **34 ✅** | 117 | +35 | Higher | -48 | Lower | -83 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 4 | -14 | Lower | -2 | Lower | +12 | Higher |
| Blank Lines | **12 ✅** | 13 | 17 | -1 | Lower | -5 | Lower | -4 | Lower |
| Functions | **2 ✅** | 3 | 8 | -1 | Lower | -6 | Lower | -5 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **3 ✅** | 5 | 11 | -2 | Lower | -8 | Lower | -6 | Lower |

## p00320

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.0348 ✅** | 52.0722 | 52.7458 | +2.9627 | Higher | +2.2890 | Higher | -0.6736 | Lower |
| MI (SEI) | 5.3267 | **29.4310 ✅** | 2.1262 | -24.1042 | Lower | +3.2005 | Higher | +27.3048 | Higher |
| Cyclomatic (sum) | **16 ✅** | 22 | 17 | -6 | Lower | -1 | Lower | +5 | Higher |
| Cognitive (sum) | **17 ✅** | 24 | 20 | -7 | Lower | -3 | Lower | +4 | Higher |
| Halstead Volume | 3042.6321 | **2641.2841 ✅** | 3104.6060 | +401.3480 | Higher | -61.9739 | Lower | -463.3219 | Lower |
| Halstead Effort | 241813.1901 | **99429.1094 ✅** | 269065.8576 | +142384.0806 | Higher | -27252.6675 | Lower | -169636.7481 | Lower |
| SLOC | **78 ✅** | 90 | 88 | -12 | Lower | -10 | Lower | +2 | Higher |
| PLOC | 70 | 70 | 80 | 0 | Same | -10 | Lower | -10 | Lower |
| LLOC | 44 | **38 ✅** | 52 | +6 | Higher | -8 | Lower | -14 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00329

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.5962 | **60.2015 ✅** | 49.6774 | -13.6054 | Lower | -3.0812 | Lower | +10.5242 | Higher |
| MI (SEI) | 4.1992 | **51.4609 ✅** | 8.9206 | -47.2617 | Lower | -4.7214 | Lower | +42.5403 | Higher |
| Cyclomatic (sum) | 24 | **15 ✅** | 21 | +9 | Higher | +3 | Higher | -6 | Lower |
| Cognitive (sum) | 27 | **15 ✅** | 25 | +12 | Higher | +2 | Higher | -10 | Lower |
| Halstead Volume | 3410.5933 | **1890.6918 ✅** | 3055.0511 | +1519.9015 | Higher | +355.5423 | Higher | -1164.3592 | Lower |
| Halstead Effort | 212425.7051 | **90193.0025 ✅** | 191635.0208 | +122232.7026 | Higher | +20790.6844 | Higher | -101442.0182 | Lower |
| SLOC | 113 | **67 ✅** | 101 | +46 | Higher | +12 | Higher | -34 | Lower |
| PLOC | 97 | **49 ✅** | 90 | +48 | Higher | +7 | Higher | -41 | Lower |
| LLOC | 59 | **27 ✅** | 65 | +32 | Higher | -6 | Lower | -38 | Lower |
| Comments (CLOC) | 2 | **22 ✅** | 2 | -20 | Lower | 0 | Same | +20 | Higher |
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
| MI (original) | 77.9408 | **90.6799 ✅** | 78.9208 | -12.7391 | Lower | -0.9800 | Lower | +11.7591 | Higher |
| MI (SEI) | 56.0671 | **98.6609 ✅** | 57.9397 | -42.5939 | Lower | -1.8726 | Lower | +40.7212 | Higher |
| Cyclomatic (sum) | 7 | **3 ✅** | 6 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 9 | **0 ✅** | 9 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 807.0714 | **294.0315 ✅** | 848.9143 | +513.0399 | Higher | -41.8429 | Lower | -554.8828 | Lower |
| Halstead Effort | 23606.8395 | **3849.1400 ✅** | 21647.3147 | +19757.6995 | Higher | +1959.5248 | Higher | -17798.1747 | Lower |
| SLOC | 33 | **22 ✅** | 31 | +11 | Higher | +2 | Higher | -9 | Lower |
| PLOC | 25 | **11 ✅** | 24 | +14 | Higher | +1 | Higher | -13 | Lower |
| LLOC | 14 | **6 ✅** | 20 | +8 | Higher | -6 | Lower | -14 | Lower |
| Comments (CLOC) | 2 | **10 ✅** | 2 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00428

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.8190 | **65.9641 ✅** | 57.2261 | -4.1451 | Lower | +4.5929 | Higher | +8.7380 | Higher |
| MI (SEI) | 38.4630 | **45.9029 ✅** | 9.2008 | -7.4400 | Lower | +29.2621 | Higher | +36.7021 | Higher |
| Cyclomatic (sum) | 16 | **13 ✅** | 23 | +3 | Higher | -7 | Lower | -10 | Lower |
| Cognitive (sum) | 29 | **11 ✅** | 29 | +18 | Higher | 0 | Same | -18 | Lower |
| Halstead Volume | 1775.2521 | **1502.0256 ✅** | 2052.0351 | +273.2265 | Higher | -276.7830 | Lower | -550.0095 | Lower |
| Halstead Effort | 57900.5312 | **46247.8539 ✅** | 104467.2424 | +11652.6773 | Higher | -46566.7112 | Lower | -58219.3885 | Lower |
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
| MI (original) | 80.2076 | 69.0296 | **82.7071 ✅** | +11.1780 | Higher | -2.4995 | Lower | -13.6775 | Lower |
| MI (SEI) | **77.7611 ✅** | 62.1355 | 76.5183 | +15.6256 | Higher | +1.2429 | Higher | -14.3827 | Lower |
| Cyclomatic (sum) | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 5 | 13 | 5 | -8 | Lower | 0 | Same | +8 | Higher |
| Halstead Volume | **519.5448 ✅** | 780.6987 | 527.3324 | -261.1539 | Lower | -7.7875 | Lower | +253.3664 | Higher |
| Halstead Effort | 8312.7172 | 16264.5567 | **6403.3214 ✅** | -7951.8395 | Lower | +1909.3958 | Higher | +9861.2353 | Higher |
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
| MI (original) | **73.9686 ✅** | 73.2423 | 73.8080 | +0.7263 | Higher | +0.1607 | Higher | -0.5656 | Lower |
| MI (SEI) | 60.6800 | 62.2724 | **64.8843 ✅** | -1.5923 | Lower | -4.2043 | Lower | -2.6120 | Lower |
| Cyclomatic (sum) | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 19 | **13 ✅** | 20 | +6 | Higher | -1 | Lower | -7 | Lower |
| Halstead Volume | 862.5902 | **823.0107 ✅** | 1048.2109 | +39.5795 | Higher | -185.6207 | Lower | -225.2001 | Lower |
| Halstead Effort | 27430.3682 | **25474.1417 ✅** | 40251.2974 | +1956.2265 | Higher | -12820.9292 | Lower | -14777.1558 | Lower |
| SLOC | 39 | 42 | **37 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| PLOC | 29 | 31 | **28 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| LLOC | **18 ✅** | 19 | 23 | -1 | Lower | -5 | Lower | -4 | Lower |
| Comments (CLOC) | 6 | 8 | 8 | -2 | Lower | -2 | Lower | 0 | Same |
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
| MI (original) | 49.2434 | **53.2624 ✅** | 35.0693 | -4.0191 | Lower | +14.1740 | Higher | +18.1931 | Higher |
| MI (SEI) | 13.5158 | **33.2678 ✅** | -9.1856 | -19.7520 | Lower | +22.7014 | Higher | +42.4534 | Higher |
| Cyclomatic (sum) | 25 | **15 ✅** | 38 | +10 | Higher | -13 | Lower | -23 | Lower |
| Cognitive (sum) | 38 | **13 ✅** | 51 | +25 | Higher | -13 | Lower | -38 | Lower |
| Halstead Volume | **3367.3814 ✅** | 3684.8705 | 5485.8294 | -317.4891 | Lower | -2118.4480 | Lower | -1800.9589 | Lower |
| Halstead Effort | **139311.8266 ✅** | 151469.4358 | 488010.2433 | -12157.6092 | Lower | -348698.4167 | Lower | -336540.8075 | Lower |
| SLOC | 95 | **83 ✅** | 162 | +12 | Higher | -67 | Lower | -79 | Lower |
| PLOC | 77 | **57 ✅** | 137 | +20 | Higher | -60 | Lower | -80 | Lower |
| LLOC | 50 | **43 ✅** | 93 | +7 | Higher | -43 | Lower | -50 | Lower |
| Comments (CLOC) | 4 | **15 ✅** | 4 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 15 | **14 ✅** | 23 | +1 | Higher | -8 | Lower | -9 | Lower |
| Functions | 3 | 3 | 9 | 0 | Same | -6 | Lower | -6 | Lower |
| Closures | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | 2 | 2 | 6 | 0 | Same | -4 | Lower | -4 | Lower |

## p00468

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.4405 | **63.3221 ✅** | 62.8180 | -0.8816 | Lower | -0.3775 | Lower | +0.5041 | Higher |
| MI (SEI) | 40.7767 | **50.0900 ✅** | 16.4536 | -9.3133 | Lower | +24.3231 | Higher | +33.6364 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 30 | 27 | **18 ✅** | +3 | Higher | +12 | Higher | +9 | Higher |
| Halstead Volume | 1565.1781 | **1529.3548 ✅** | 1698.8007 | +35.8234 | Higher | -133.6226 | Lower | -169.4460 | Lower |
| Halstead Effort | 56617.3090 | **43658.2992 ✅** | 68740.7574 | +12959.0098 | Higher | -12123.4484 | Lower | -25082.4582 | Lower |
| SLOC | 62 | 60 | **59 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| PLOC | 46 | **42 ✅** | 50 | +4 | Higher | -4 | Lower | -8 | Lower |
| LLOC | 38 | **33 ✅** | 40 | +5 | Higher | -2 | Lower | -7 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 0 | -6 | Lower | +7 | Higher | +13 | Higher |
| Blank Lines | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
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
| MI (original) | 63.0820 | **76.1210 ✅** | 64.1997 | -13.0389 | Lower | -1.1177 | Lower | +11.9213 | Higher |
| MI (SEI) | 16.6310 | **78.3069 ✅** | 18.2434 | -61.6759 | Lower | -1.6125 | Lower | +60.0634 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 15 | **10 ✅** | 15 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1674.0394 | **473.1340 ✅** | 1674.0394 | +1200.9054 | Higher | 0 | Same | -1200.9054 | Lower |
| Halstead Effort | 73825.1390 | **13375.1355 ✅** | 73825.1390 | +60450.0036 | Higher | 0 | Same | -60450.0036 | Lower |
| SLOC | 60 | **42 ✅** | 56 | +18 | Higher | +4 | Higher | -14 | Lower |
| PLOC | 47 | **19 ✅** | 47 | +28 | Higher | 0 | Same | -28 | Lower |
| LLOC | 35 | **12 ✅** | 35 | +23 | Higher | 0 | Same | -23 | Lower |
| Comments (CLOC) | 0 | **19 ✅** | 0 | -19 | Lower | 0 | Same | +19 | Higher |
| Blank Lines | 13 | **7 ✅** | 9 | +6 | Higher | +4 | Higher | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p00507

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.8694 | 71.3246 | **72.4423 ✅** | -5.4552 | Lower | -6.5729 | Lower | -1.1177 | Lower |
| MI (SEI) | 50.5618 | **67.3847 ✅** | 41.9548 | -16.8229 | Lower | +8.6070 | Higher | +25.4299 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 10 | **9 ✅** | 20 | +1 | Higher | -10 | Lower | -11 | Lower |
| Halstead Volume | 1312.1877 | **722.5705 ✅** | 1117.4678 | +589.6172 | Higher | +194.7200 | Higher | -394.8973 | Lower |
| Halstead Effort | 38914.5678 | **15436.7341 ✅** | 35758.9694 | +23477.8336 | Higher | +3155.5984 | Higher | -20322.2352 | Lower |
| SLOC | 57 | 50 | **40 ✅** | +7 | Higher | +17 | Higher | +10 | Higher |
| PLOC | 39 | **29 ✅** | 32 | +10 | Higher | +7 | Higher | -3 | Lower |
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
| MI (original) | **49.9500 ✅** | 46.2418 | 36.4288 | +3.7082 | Higher | +13.5212 | Higher | +9.8130 | Higher |
| MI (SEI) | 8.7525 | **13.9025 ✅** | -14.2481 | -5.1500 | Lower | +23.0006 | Higher | +28.1505 | Higher |
| Cyclomatic (sum) | **16 ✅** | 25 | 29 | -9 | Lower | -13 | Lower | -4 | Lower |
| Cognitive (sum) | 21 | **17 ✅** | 29 | +4 | Higher | -8 | Lower | -12 | Lower |
| Halstead Volume | 3507.2544 | **2682.1846 ✅** | 5512.5109 | +825.0698 | Higher | -2005.2564 | Lower | -2830.3262 | Lower |
| Halstead Effort | 147238.5118 | **111525.2373 ✅** | 432043.0400 | +35713.2745 | Higher | -284804.5283 | Lower | -320517.8027 | Lower |
| SLOC | **102 ✅** | 123 | 169 | -21 | Lower | -67 | Lower | -46 | Lower |
| PLOC | **89 ✅** | 95 | 146 | -6 | Lower | -57 | Lower | -51 | Lower |
| LLOC | 48 | **28 ✅** | 93 | +20 | Higher | -45 | Lower | -65 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 1 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | **13 ✅** | 19 | 22 | -6 | Lower | -9 | Lower | -3 | Lower |
| Functions | **2 ✅** | 5 | 8 | -3 | Lower | -6 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 5 | 3 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00528

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.4059 | **51.5342 ✅** | 24.1334 | -11.1284 | Lower | +16.2725 | Higher | +27.4008 | Higher |
| MI (SEI) | -7.2486 | **24.7568 ✅** | -29.3402 | -32.0054 | Lower | +22.0916 | Higher | +54.0970 | Higher |
| Cyclomatic (sum) | 33 | **21 ✅** | 45 | +12 | Higher | -12 | Lower | -24 | Lower |
| Cognitive (sum) | 23 | **15 ✅** | 42 | +8 | Higher | -19 | Lower | -27 | Lower |
| Halstead Volume | 4986.3147 | **2503.9851 ✅** | 8972.8926 | +2482.3296 | Higher | -3986.5778 | Lower | -6468.9074 | Lower |
| Halstead Effort | 299832.8274 | **88891.4715 ✅** | 1074042.9477 | +210941.3559 | Higher | -774210.1202 | Lower | -985151.4761 | Lower |
| SLOC | 129 | **96 ✅** | 246 | +33 | Higher | -117 | Lower | -150 | Lower |
| PLOC | 107 | **76 ✅** | 210 | +31 | Higher | -103 | Lower | -134 | Lower |
| LLOC | 66 | **29 ✅** | 194 | +37 | Higher | -128 | Lower | -165 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 2 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | 22 | **17 ✅** | 34 | +5 | Higher | -12 | Lower | -17 | Lower |
| Functions | **4 ✅** | 5 | 11 | -1 | Lower | -7 | Lower | -6 | Lower |
| Closures | 8 | **0 ✅** | 2 | +8 | Higher | +6 | Higher | -2 | Lower |
| Exits | **4 ✅** | 5 | 8 | -1 | Lower | -4 | Lower | -3 | Lower |

## p00534

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.5818 | **75.0482 ✅** | 68.2357 | -7.4665 | Lower | -0.6539 | Lower | +6.8126 | Higher |
| MI (SEI) | 53.5267 | **62.9418 ✅** | 50.1970 | -9.4151 | Lower | +3.3297 | Higher | +12.7448 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1580.5444 | **1026.9765 ✅** | 1696.9826 | +553.5679 | Higher | -116.4382 | Lower | -670.0061 | Lower |
| Halstead Effort | 68206.5693 | **37328.3643 ✅** | 72758.1293 | +30878.2051 | Higher | -4551.5599 | Lower | -35429.7650 | Lower |
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
| MI (original) | 69.6940 | **83.0883 ✅** | 69.7545 | -13.3943 | Lower | -0.0606 | Lower | +13.3337 | Higher |
| MI (SEI) | 56.1703 | **86.0682 ✅** | 54.0960 | -29.8978 | Lower | +2.0744 | Higher | +31.9722 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 10 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 15 | **4 ✅** | 9 | +11 | Higher | +6 | Higher | -5 | Lower |
| Halstead Volume | 1122.6724 | **576.9780 ✅** | 1133.7474 | +545.6944 | Higher | -11.0750 | Lower | -556.7694 | Lower |
| Halstead Effort | 37307.2677 | **16969.9401 ✅** | 36279.9164 | +20337.3276 | Higher | +1027.3513 | Higher | -19309.9763 | Lower |
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
| MI (original) | 57.0216 | 66.0724 | **66.1460 ✅** | -9.0508 | Lower | -9.1244 | Lower | -0.0736 | Lower |
| MI (SEI) | 29.6518 | **54.9228 ✅** | 41.7780 | -25.2711 | Lower | -12.1263 | Lower | +13.1448 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 12 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 3178.0677 | **1305.9659 ✅** | 1515.9717 | +1872.1019 | Higher | +1662.0961 | Higher | -210.0058 | Lower |
| Halstead Effort | 82756.8833 | **34042.1768 ✅** | 63029.4370 | +48714.7064 | Higher | +19727.4463 | Higher | -28987.2602 | Lower |
| SLOC | 71 | 58 | **52 ✅** | +13 | Higher | +19 | Higher | +6 | Higher |
| PLOC | 52 | **33 ✅** | 40 | +19 | Higher | +12 | Higher | -7 | Lower |
| LLOC | 40 | **25 ✅** | 36 | +15 | Higher | +4 | Higher | -11 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 4 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 13 | 13 | **9 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00571

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3731 | **71.1134 ✅** | 57.6239 | -5.7403 | Lower | +7.7492 | Higher | +13.4895 | Higher |
| MI (SEI) | 39.5427 | **65.3241 ✅** | 21.0092 | -25.7814 | Lower | +18.5335 | Higher | +44.3149 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 12 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1666.5372 | **1065.9822 ✅** | 2297.7804 | +600.5550 | Higher | -631.2432 | Lower | -1231.7982 | Lower |
| Halstead Effort | 47408.5975 | **20653.4057 ✅** | 102579.4840 | +26755.1918 | Higher | -55170.8865 | Lower | -81926.0784 | Lower |
| SLOC | 56 | **46 ✅** | 77 | +10 | Higher | -21 | Lower | -31 | Lower |
| PLOC | 43 | **27 ✅** | 62 | +16 | Higher | -19 | Lower | -35 | Lower |
| LLOC | 28 | **21 ✅** | 42 | +7 | Higher | -14 | Lower | -21 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 2 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 9 | 9 | 13 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |

## p00594

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.0125 | 59.8085 | **64.8012 ✅** | +3.2040 | Higher | -1.7887 | Lower | -4.9927 | Lower |
| MI (SEI) | **43.4390 ✅** | 39.5717 | 43.1975 | +3.8673 | Higher | +0.2414 | Higher | -3.6259 | Lower |
| Cyclomatic (sum) | 16 | 15 | **13 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 22 | 20 | **17 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Halstead Volume | 1411.1872 | 1702.1216 | 1411.1872 | -290.9344 | Lower | 0 | Same | +290.9344 | Higher |
| Halstead Effort | 70830.7445 | 77266.0055 | **67449.5237 ✅** | -6435.2610 | Lower | +3381.2207 | Higher | +9816.4817 | Higher |
| SLOC | 61 | 71 | **57 ✅** | -10 | Lower | +4 | Higher | +14 | Higher |
| PLOC | **47 ✅** | 54 | 49 | -7 | Lower | -2 | Lower | +5 | Higher |
| LLOC | **33 ✅** | 41 | 37 | -8 | Lower | -4 | Lower | +4 | Higher |
| Comments (CLOC) | 8 | **10 ✅** | 6 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 9 | 12 | **7 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00629

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.9887 | **58.3960 ✅** | 56.9970 | -1.4073 | Lower | -0.0084 | Lower | +1.3990 | Higher |
| MI (SEI) | 30.4962 | **36.5610 ✅** | 18.0735 | -6.0647 | Lower | +12.4227 | Higher | +18.4874 | Higher |
| Cyclomatic (sum) | 17 | **15 ✅** | 21 | +2 | Higher | -4 | Lower | -6 | Lower |
| Cognitive (sum) | 20 | **12 ✅** | 19 | +8 | Higher | +1 | Higher | -7 | Lower |
| Halstead Volume | 1921.4178 | **1734.5871 ✅** | 2685.3425 | +186.8307 | Higher | -763.9248 | Lower | -950.7554 | Lower |
| Halstead Effort | 71205.4819 | **48467.5905 ✅** | 122112.4185 | +22737.8914 | Higher | -50906.9365 | Lower | -73644.8279 | Lower |
| SLOC | 79 | 77 | **67 ✅** | +2 | Higher | +12 | Higher | +10 | Higher |
| PLOC | 65 | **54 ✅** | 58 | +11 | Higher | +7 | Higher | -4 | Lower |
| LLOC | 36 | **23 ✅** | 41 | +13 | Higher | -5 | Lower | -18 | Lower |
| Comments (CLOC) | 7 | **10 ✅** | 1 | -3 | Lower | +6 | Higher | +9 | Higher |
| Blank Lines | 13 | 14 | **8 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 5 | **0 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00641

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.6547 | **53.0116 ✅** | 47.2432 | -5.3569 | Lower | +0.4114 | Higher | +5.7683 | Higher |
| MI (SEI) | 8.1745 | **36.4456 ✅** | 9.3557 | -28.2711 | Lower | -1.1812 | Lower | +27.0899 | Higher |
| Cyclomatic (sum) | 22 | **14 ✅** | 23 | +8 | Higher | -1 | Lower | -9 | Lower |
| Cognitive (sum) | 14 | **11 ✅** | 15 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | 3709.9376 | **2487.0966 ✅** | 3422.9730 | +1222.8411 | Higher | +286.9647 | Higher | -935.8764 | Lower |
| Halstead Effort | 218356.3292 | **106887.3130 ✅** | 268494.6611 | +111469.0162 | Higher | -50138.3319 | Lower | -161607.3482 | Lower |
| SLOC | 106 | **97 ✅** | 110 | +9 | Higher | -4 | Lower | -13 | Lower |
| PLOC | 87 | **71 ✅** | 92 | +16 | Higher | -5 | Lower | -21 | Lower |
| LLOC | 57 | **45 ✅** | 67 | +12 | Higher | -10 | Lower | -22 | Lower |
| Comments (CLOC) | 3 | **23 ✅** | 4 | -20 | Lower | -1 | Lower | +19 | Higher |
| Blank Lines | 17 | **14 ✅** | 16 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |

## p00649

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.1828 | **57.4761 ✅** | 46.3053 | -10.2933 | Lower | +0.8775 | Higher | +11.1709 | Higher |
| MI (SEI) | 7.5043 | **32.7318 ✅** | 7.9558 | -25.2275 | Lower | -0.4515 | Lower | +24.7760 | Higher |
| Cyclomatic (sum) | 26 | **21 ✅** | 25 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 31 | **16 ✅** | 29 | +15 | Higher | +2 | Higher | -13 | Lower |
| Halstead Volume | 2788.8061 | **2044.2297 ✅** | 3357.3333 | +744.5764 | Higher | -568.5272 | Lower | -1313.1035 | Lower |
| Halstead Effort | 111916.0013 | **68560.3203 ✅** | 231109.4533 | +43355.6810 | Higher | -119193.4519 | Lower | -162549.1330 | Lower |
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
| MI (original) | 44.4899 | **64.4566 ✅** | 43.7566 | -19.9667 | Lower | +0.7333 | Higher | +20.7000 | Higher |
| MI (SEI) | 1.6025 | **58.8077 ✅** | 0.5098 | -57.2051 | Lower | +1.0928 | Higher | +58.2979 | Higher |
| Cyclomatic (sum) | 27 | **14 ✅** | 28 | +13 | Higher | -1 | Lower | -14 | Lower |
| Cognitive (sum) | 27 | **10 ✅** | 30 | +17 | Higher | -3 | Lower | -20 | Lower |
| Halstead Volume | 4869.8848 | **1167.8596 ✅** | 4933.4604 | +3702.0252 | Higher | -63.5755 | Lower | -3765.6007 | Lower |
| Halstead Effort | 431282.9642 | **48846.9992 ✅** | 440235.8246 | +382435.9650 | Higher | -8952.8604 | Lower | -391388.8254 | Lower |
| SLOC | 110 | **61 ✅** | 113 | +49 | Higher | -3 | Lower | -52 | Lower |
| PLOC | 89 | **35 ✅** | 91 | +54 | Higher | -2 | Lower | -56 | Lower |
| LLOC | 75 | **18 ✅** | 80 | +57 | Higher | -5 | Lower | -62 | Lower |
| Comments (CLOC) | 2 | **22 ✅** | 2 | -20 | Lower | 0 | Same | +20 | Higher |
| Blank Lines | 19 | **10 ✅** | 20 | +9 | Higher | -1 | Lower | -10 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **3 ✅** | 7 | +4 | Higher | 0 | Same | -4 | Lower |

## p00655

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.5208 | **56.7020 ✅** | 29.9915 | -19.1812 | Lower | +7.5293 | Higher | +26.7105 | Higher |
| MI (SEI) | -8.6128 | **40.8222 ✅** | -16.6182 | -49.4350 | Lower | +8.0054 | Higher | +57.4404 | Higher |
| Cyclomatic (sum) | 39 | **21 ✅** | 45 | +18 | Higher | -6 | Lower | -24 | Lower |
| Cognitive (sum) | 32 | **15 ✅** | 41 | +17 | Higher | -9 | Lower | -26 | Lower |
| Halstead Volume | 4433.5497 | **1573.6168 ✅** | 6834.3583 | +2859.9329 | Higher | -2400.8086 | Lower | -5260.7415 | Lower |
| Halstead Effort | 271248.3441 | **48152.6734 ✅** | 942073.5773 | +223095.6706 | Higher | -670825.2333 | Lower | -893920.9039 | Lower |
| SLOC | 147 | **81 ✅** | 187 | +66 | Higher | -40 | Lower | -106 | Lower |
| PLOC | 133 | **63 ✅** | 170 | +70 | Higher | -37 | Lower | -107 | Lower |
| LLOC | 66 | **25 ✅** | 117 | +41 | Higher | -51 | Lower | -92 | Lower |
| Comments (CLOC) | 2 | **17 ✅** | 4 | -15 | Lower | -2 | Lower | +13 | Higher |
| Blank Lines | 14 | **11 ✅** | 15 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 8 | **6 ✅** | 10 | +2 | Higher | -2 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -3.1612 | **42.0249 ✅** | -7.8441 | -45.1861 | Lower | +4.6829 | Higher | +49.8690 | Higher |
| MI (SEI) | -67.0249 | **17.5490 ✅** | -64.4623 | -84.5740 | Lower | -2.5627 | Lower | +82.0113 | Higher |
| Cyclomatic (sum) | 130 | **31 ✅** | 144 | +99 | Higher | -14 | Lower | -113 | Lower |
| Cognitive (sum) | 176 | **18 ✅** | 185 | +158 | Higher | -9 | Lower | -167 | Lower |
| Halstead Volume | 13624.7827 | **3162.0196 ✅** | 13381.0815 | +10462.7631 | Higher | +243.7012 | Higher | -10219.0619 | Lower |
| Halstead Effort | 2020895.8922 | **145594.0613 ✅** | 2010699.7524 | +1875301.8310 | Higher | +10196.1398 | Higher | -1865105.6912 | Lower |
| SLOC | 347 | **139 ✅** | 382 | +208 | Higher | -35 | Lower | -243 | Lower |
| PLOC | 327 | **105 ✅** | 363 | +222 | Higher | -36 | Lower | -258 | Lower |
| LLOC | 226 | **28 ✅** | 261 | +198 | Higher | -35 | Lower | -233 | Lower |
| Comments (CLOC) | 0 | **23 ✅** | 4 | -23 | Lower | -4 | Lower | +19 | Higher |
| Blank Lines | 20 | 16 | **15 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Functions | 12 | **8 ✅** | 10 | +4 | Higher | +2 | Higher | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 27 | **4 ✅** | 27 | +23 | Higher | 0 | Same | -23 | Lower |

## p00680

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 17.1121 | **47.0697 ✅** | 14.9821 | -29.9577 | Lower | +2.1300 | Higher | +32.0876 | Higher |
| MI (SEI) | -38.4335 | **19.5269 ✅** | -40.3016 | -57.9604 | Lower | +1.8680 | Higher | +59.8285 | Higher |
| Cyclomatic (sum) | 60 | **22 ✅** | 62 | +38 | Higher | -2 | Lower | -40 | Lower |
| Cognitive (sum) | 81 | **23 ✅** | 85 | +58 | Higher | -4 | Lower | -62 | Lower |
| Halstead Volume | 11274.6436 | **3401.7310 ✅** | 10835.7157 | +7872.9125 | Higher | +438.9278 | Higher | -7433.9847 | Lower |
| Halstead Effort | 1301638.1596 | **139227.9911 ✅** | 1187761.1464 | +1162410.1684 | Higher | +113877.0132 | Higher | -1048533.1552 | Lower |
| SLOC | 285 | **113 ✅** | 320 | +172 | Higher | -35 | Lower | -207 | Lower |
| PLOC | 253 | **94 ✅** | 280 | +159 | Higher | -27 | Lower | -186 | Lower |
| LLOC | 165 | **35 ✅** | 188 | +130 | Higher | -23 | Lower | -153 | Lower |
| Comments (CLOC) | 2 | **13 ✅** | 3 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 30 | **14 ✅** | 38 | +16 | Higher | -8 | Lower | -24 | Lower |
| Functions | 14 | **6 ✅** | 15 | +8 | Higher | -1 | Lower | -9 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 12 | **7 ✅** | 13 | +5 | Higher | -1 | Lower | -6 | Lower |

## p00690

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.9855 | **50.2236 ✅** | 47.7650 | -1.2380 | Lower | +1.2205 | Higher | +2.4586 | Higher |
| MI (SEI) | -2.4842 | **27.0341 ✅** | 6.7202 | -29.5183 | Lower | -9.2044 | Lower | +20.3139 | Higher |
| Cyclomatic (sum) | 25 | **17 ✅** | 27 | +8 | Higher | -2 | Lower | -10 | Lower |
| Cognitive (sum) | 38 | **22 ✅** | 38 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 3424.9814 | **2996.6481 ✅** | 3282.1060 | +428.3333 | Higher | +142.8754 | Higher | -285.4579 | Lower |
| Halstead Effort | 221947.8076 | **120423.4383 ✅** | 206936.7837 | +101524.3693 | Higher | +15011.0239 | Higher | -86513.3454 | Lower |
| SLOC | **96 ✅** | 104 | 102 | -8 | Lower | -6 | Lower | +2 | Higher |
| PLOC | 87 | **73 ✅** | 88 | +14 | Higher | -1 | Lower | -15 | Lower |
| LLOC | 63 | **49 ✅** | 68 | +14 | Higher | -5 | Lower | -19 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 2 | -16 | Lower | -2 | Lower | +14 | Higher |
| Blank Lines | **9 ✅** | 18 | 12 | -9 | Lower | -3 | Lower | +6 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00692

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8826 | **72.2200 ✅** | 47.0069 | -12.3374 | Lower | +12.8756 | Higher | +25.2130 | Higher |
| MI (SEI) | 13.1351 | **63.6630 ✅** | 16.4303 | -50.5279 | Lower | -3.2952 | Lower | +47.2327 | Higher |
| Cyclomatic (sum) | 24 | **8 ✅** | 31 | +16 | Higher | -7 | Lower | -23 | Lower |
| Cognitive (sum) | 18 | **11 ✅** | 28 | +7 | Higher | -10 | Lower | -17 | Lower |
| Halstead Volume | 2643.8195 | **824.3742 ✅** | 5039.4079 | +1819.4453 | Higher | -2395.5884 | Lower | -4215.0337 | Lower |
| Halstead Effort | 156855.0262 | **16935.5129 ✅** | 202248.2372 | +139919.5133 | Higher | -45393.2110 | Lower | -185312.7242 | Lower |
| SLOC | 54 | **46 ✅** | 88 | +8 | Higher | -34 | Lower | -42 | Lower |
| PLOC | 47 | **30 ✅** | 71 | +17 | Higher | -24 | Lower | -41 | Lower |
| LLOC | 35 | **17 ✅** | 44 | +18 | Higher | -9 | Lower | -27 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 7 | -11 | Lower | -7 | Lower | +4 | Higher |
| Blank Lines | **7 ✅** | 8 | 10 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00723

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0859 | **66.7540 ✅** | 50.4045 | -13.6681 | Lower | +2.6813 | Higher | +16.3494 | Higher |
| MI (SEI) | 2.5150 | **47.3521 ✅** | -0.8443 | -44.8371 | Lower | +3.3593 | Higher | +48.1963 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 21 | +7 | Higher | -5 | Lower | -12 | Lower |
| Cognitive (sum) | 28 | **6 ✅** | 29 | +22 | Higher | -1 | Lower | -23 | Lower |
| Halstead Volume | 3149.7606 | **1156.9905 ✅** | 2918.0040 | +1992.7701 | Higher | +231.7567 | Higher | -1761.0134 | Lower |
| Halstead Effort | 151458.4886 | **25839.4549 ✅** | 146925.4423 | +125619.0337 | Higher | +4533.0463 | Higher | -121085.9874 | Lower |
| SLOC | 87 | **57 ✅** | 98 | +30 | Higher | -11 | Lower | -41 | Lower |
| PLOC | 70 | **37 ✅** | 87 | +33 | Higher | -17 | Lower | -50 | Lower |
| LLOC | 50 | **21 ✅** | 48 | +29 | Higher | +2 | Higher | -27 | Lower |
| Comments (CLOC) | 0 | **7 ✅** | 0 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 17 | 13 | **11 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Functions | 4 | **3 ✅** | 6 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 4 | 3 | -3 | Lower | -2 | Lower | +1 | Higher |

## p00740

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.0787 | **74.5943 ✅** | 60.7489 | -18.5156 | Lower | -4.6702 | Lower | +13.8453 | Higher |
| MI (SEI) | 23.4406 | **68.7193 ✅** | 22.8068 | -45.2787 | Lower | +0.6337 | Higher | +45.9125 | Higher |
| Cyclomatic (sum) | 21 | **11 ✅** | 15 | +10 | Higher | +6 | Higher | -4 | Lower |
| Cognitive (sum) | 25 | **6 ✅** | 16 | +19 | Higher | +9 | Higher | -10 | Lower |
| Halstead Volume | 3204.0455 | **829.2698 ✅** | 1625.0220 | +2374.7757 | Higher | +1579.0234 | Higher | -795.7522 | Lower |
| Halstead Effort | 93528.7778 | **20623.5798 ✅** | 54167.4010 | +72905.1980 | Higher | +39361.3768 | Higher | -33543.8212 | Lower |
| SLOC | 67 | **38 ✅** | 68 | +29 | Higher | -1 | Lower | -30 | Lower |
| PLOC | 53 | **28 ✅** | 58 | +25 | Higher | -5 | Lower | -30 | Lower |
| LLOC | 33 | **13 ✅** | 37 | +20 | Higher | -4 | Lower | -24 | Lower |
| Comments (CLOC) | 3 | **10 ✅** | 1 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 14 | **6 ✅** | 10 | +8 | Higher | +4 | Higher | -4 | Lower |
| Functions | **1 ✅** | 2 | 5 | -1 | Lower | -4 | Lower | -3 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00749

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.8515 | **67.9835 ✅** | 23.0849 | -43.1321 | Lower | +1.7666 | Higher | +44.8986 | Higher |
| MI (SEI) | -34.4513 | **57.1951 ✅** | -29.6897 | -91.6464 | Lower | -4.7616 | Lower | +86.8848 | Higher |
| Cyclomatic (sum) | 53 | **13 ✅** | 55 | +40 | Higher | -2 | Lower | -42 | Lower |
| Cognitive (sum) | 82 | **11 ✅** | 78 | +71 | Higher | +4 | Higher | -67 | Lower |
| Halstead Volume | 7554.0645 | **1225.7910 ✅** | 8027.1268 | +6328.2734 | Higher | -473.0623 | Lower | -6801.3357 | Lower |
| Halstead Effort | 749712.7136 | **30889.9342 ✅** | 789525.2557 | +718822.7794 | Higher | -39812.5421 | Lower | -758635.3215 | Lower |
| SLOC | 222 | **49 ✅** | 236 | +173 | Higher | -14 | Lower | -187 | Lower |
| PLOC | 199 | **33 ✅** | 205 | +166 | Higher | -6 | Lower | -172 | Lower |
| LLOC | 136 | **17 ✅** | 137 | +119 | Higher | -1 | Lower | -120 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 2 | -11 | Lower | -2 | Lower | +9 | Higher |
| Blank Lines | 23 | **7 ✅** | 29 | +16 | Higher | -6 | Lower | -22 | Lower |
| Functions | 7 | **2 ✅** | 9 | +5 | Higher | -2 | Lower | -7 | Lower |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 13 | **1 ✅** | 13 | +12 | Higher | 0 | Same | -12 | Lower |

## p00762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.5082 | **69.8981 ✅** | 39.0522 | -29.3898 | Lower | +1.4560 | Higher | +30.8459 | Higher |
| MI (SEI) | 7.2663 | **59.7294 ✅** | 4.2547 | -52.4631 | Lower | +3.0116 | Higher | +55.4747 | Higher |
| Cyclomatic (sum) | 30 | **8 ✅** | 30 | +22 | Higher | 0 | Same | -22 | Lower |
| Cognitive (sum) | 19 | **7 ✅** | 16 | +12 | Higher | +3 | Higher | -9 | Lower |
| Halstead Volume | 4959.1824 | **1128.4003 ✅** | 4917.4302 | +3830.7820 | Higher | +41.7521 | Higher | -3789.0299 | Lower |
| Halstead Effort | 420071.9168 | **28482.3807 ✅** | 433589.0652 | +391589.5360 | Higher | -13517.1484 | Lower | -405106.6844 | Lower |
| SLOC | 134 | **48 ✅** | 147 | +86 | Higher | -13 | Lower | -99 | Lower |
| PLOC | 107 | **30 ✅** | 119 | +77 | Higher | -12 | Lower | -89 | Lower |
| LLOC | 86 | **19 ✅** | 93 | +67 | Higher | -7 | Lower | -74 | Lower |
| Comments (CLOC) | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 23 | **9 ✅** | 24 | +14 | Higher | -1 | Lower | -15 | Lower |
| Functions | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 5 | **0 ✅** | 5 | +5 | Higher | 0 | Same | -5 | Lower |

## p00792

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.3025 | **62.0374 ✅** | 51.5583 | -6.7348 | Lower | +3.7443 | Higher | +10.4791 | Higher |
| MI (SEI) | 14.8570 | **49.6897 ✅** | 17.5220 | -34.8328 | Lower | -2.6650 | Lower | +32.1677 | Higher |
| Cyclomatic (sum) | 19 | **11 ✅** | 24 | +8 | Higher | -5 | Lower | -13 | Lower |
| Cognitive (sum) | 31 | **12 ✅** | 53 | +19 | Higher | -22 | Lower | -41 | Lower |
| Halstead Volume | 2744.1549 | **1446.5567 ✅** | 3074.8736 | +1297.5982 | Higher | -330.7186 | Lower | -1628.3168 | Lower |
| Halstead Effort | 171870.7562 | **42379.5915 ✅** | 210366.3496 | +129491.1646 | Higher | -38495.5935 | Lower | -167986.7581 | Lower |
| SLOC | 76 | **69 ✅** | 86 | +7 | Higher | -10 | Lower | -17 | Lower |
| PLOC | 63 | **48 ✅** | 71 | +15 | Higher | -8 | Lower | -23 | Lower |
| LLOC | 58 | **35 ✅** | 61 | +23 | Higher | -3 | Lower | -26 | Lower |
| Comments (CLOC) | 1 | **17 ✅** | 4 | -16 | Lower | -3 | Lower | +13 | Higher |
| Blank Lines | 13 | **11 ✅** | 12 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00799

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.4426 | **60.3456 ✅** | 49.9428 | -11.9030 | Lower | -1.5002 | Lower | +10.4028 | Higher |
| MI (SEI) | 13.8720 | **39.5349 ✅** | 13.5893 | -25.6629 | Lower | +0.2826 | Higher | +25.9455 | Higher |
| Cyclomatic (sum) | 21 | 21 | 21 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 61 | **23 ✅** | 55 | +38 | Higher | +6 | Higher | -32 | Lower |
| Halstead Volume | 2807.2116 | **1626.7294 ✅** | 2815.2988 | +1180.4822 | Higher | -8.0872 | Lower | -1188.5694 | Lower |
| Halstead Effort | 134746.1578 | **61002.3528 ✅** | 171170.1697 | +73743.8050 | Higher | -36424.0119 | Lower | -110167.8169 | Lower |
| SLOC | 112 | **64 ✅** | 102 | +48 | Higher | +10 | Higher | -38 | Lower |
| PLOC | 95 | **48 ✅** | 88 | +47 | Higher | +7 | Higher | -40 | Lower |
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
| MI (original) | 55.0998 | **69.4207 ✅** | 58.2234 | -14.3209 | Lower | -3.1236 | Lower | +11.1972 | Higher |
| MI (SEI) | 25.8299 | **60.2354 ✅** | 29.7770 | -34.4055 | Lower | -3.9471 | Lower | +30.4584 | Higher |
| Cyclomatic (sum) | 24 | **14 ✅** | 16 | +10 | Higher | +8 | Higher | -2 | Lower |
| Cognitive (sum) | 22 | **13 ✅** | 36 | +9 | Higher | -14 | Lower | -23 | Lower |
| Halstead Volume | 2485.2987 | **1083.0835 ✅** | 2114.6668 | +1402.2151 | Higher | +370.6318 | Higher | -1031.5833 | Lower |
| Halstead Effort | 140596.8955 | **41066.9178 ✅** | 109001.4631 | +99529.9777 | Higher | +31595.4324 | Higher | -67934.5453 | Lower |
| SLOC | 74 | **46 ✅** | 72 | +28 | Higher | +2 | Higher | -26 | Lower |
| PLOC | 64 | **35 ✅** | 63 | +29 | Higher | +1 | Higher | -28 | Lower |
| LLOC | 48 | **21 ✅** | 49 | +27 | Higher | -1 | Lower | -28 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 5 | -6 | Lower | 0 | Same | +6 | Higher |
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
| MI (original) | 44.7009 | **65.9415 ✅** | 46.1378 | -21.2406 | Lower | -1.4369 | Lower | +19.8037 | Higher |
| MI (SEI) | 6.7429 | **55.9941 ✅** | 2.8126 | -49.2512 | Lower | +3.9303 | Higher | +53.1815 | Higher |
| Cyclomatic (sum) | 23 | **11 ✅** | 19 | +12 | Higher | +4 | Higher | -8 | Lower |
| Cognitive (sum) | 42 | **21 ✅** | 39 | +21 | Higher | +3 | Higher | -18 | Lower |
| Halstead Volume | 4368.5201 | **1172.8421 ✅** | 4060.5249 | +3195.6779 | Higher | +307.9951 | Higher | -2887.6828 | Lower |
| Halstead Effort | 362408.8579 | **30268.3487 ✅** | 361197.8554 | +332140.5092 | Higher | +1211.0025 | Higher | -330929.5067 | Lower |
| SLOC | 119 | **58 ✅** | 118 | +61 | Higher | +1 | Higher | -60 | Lower |
| PLOC | 100 | **35 ✅** | 100 | +65 | Higher | 0 | Same | -65 | Lower |
| LLOC | 103 | **26 ✅** | 103 | +77 | Higher | 0 | Same | -77 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 2 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 16 | **10 ✅** | 16 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 3 | **0 ✅** | 1 | +3 | Higher | +2 | Higher | -1 | Lower |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.3449 | **72.8600 ✅** | 47.8965 | -24.5151 | Lower | +0.4484 | Higher | +24.9635 | Higher |
| MI (SEI) | 12.6159 | **64.3828 ✅** | 8.3384 | -51.7669 | Lower | +4.2775 | Higher | +56.0444 | Higher |
| Cyclomatic (sum) | 18 | **6 ✅** | 19 | +12 | Higher | -1 | Lower | -13 | Lower |
| Cognitive (sum) | 18 | **3 ✅** | 17 | +15 | Higher | +1 | Higher | -14 | Lower |
| Halstead Volume | 4240.3153 | **796.3187 ✅** | 4291.1149 | +3443.9966 | Higher | -50.7996 | Lower | -3494.7962 | Lower |
| Halstead Effort | 227395.3198 | **13696.6816 ✅** | 245666.3278 | +213698.6381 | Higher | -18271.0081 | Lower | -231969.6462 | Lower |
| SLOC | 103 | **46 ✅** | 104 | +57 | Higher | -1 | Lower | -58 | Lower |
| PLOC | 85 | **28 ✅** | 85 | +57 | Higher | 0 | Same | -57 | Lower |
| LLOC | 47 | **12 ✅** | 49 | +35 | Higher | -2 | Lower | -37 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 3 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 16 | **7 ✅** | 17 | +9 | Higher | -1 | Lower | -10 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 3 | **1 ✅** | 5 | +2 | Higher | -2 | Lower | -4 | Lower |

## p00855

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.5031 | 66.2455 | **76.6486 ✅** | +0.2577 | Higher | -10.1454 | Lower | -10.4031 | Lower |
| MI (SEI) | 43.9500 | 36.3213 | **73.4670 ✅** | +7.6287 | Higher | -29.5170 | Lower | -37.1457 | Lower |
| Cyclomatic (sum) | 14 | 16 | **8 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| Cognitive (sum) | 11 | 14 | **9 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Halstead Volume | 2032.6592 | 1174.2906 | **754.9244 ✅** | +858.3687 | Higher | +1277.7348 | Higher | +419.3661 | Higher |
| Halstead Effort | 44757.9726 | 39686.6724 | **23760.2534 ✅** | +5071.3001 | Higher | +20997.7192 | Higher | +15926.4190 | Higher |
| SLOC | 45 | 53 | **36 ✅** | -8 | Lower | +9 | Higher | +17 | Higher |
| PLOC | 37 | 46 | **31 ✅** | -9 | Lower | +6 | Higher | +15 | Higher |
| LLOC | **19 ✅** | 24 | 25 | -5 | Lower | -6 | Lower | -1 | Lower |
| Comments (CLOC) | 4 | 2 | **11 ✅** | +2 | Higher | -7 | Lower | -9 | Lower |
| Blank Lines | 7 | 6 | **4 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | 7 | **1 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |

## p00856

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.5186 ✅** | 50.3368 | 54.0888 | +6.1818 | Higher | +2.4298 | Higher | -3.7519 | Lower |
| MI (SEI) | 8.2819 | **32.3839 ✅** | 30.0855 | -24.1020 | Lower | -21.8036 | Lower | +2.2985 | Higher |
| Cyclomatic (sum) | 24 | 29 | **23 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Cognitive (sum) | **17 ✅** | 32 | 21 | -15 | Lower | -4 | Lower | +11 | Higher |
| Halstead Volume | 2701.9596 | **2614.1799 ✅** | 2903.6513 | +87.7797 | Higher | -201.6917 | Lower | -289.4714 | Lower |
| Halstead Effort | 150240.2120 | **121729.8538 ✅** | 172242.1240 | +28510.3582 | Higher | -22001.9120 | Lower | -50512.2702 | Lower |
| SLOC | **66 ✅** | 91 | 76 | -25 | Lower | -10 | Lower | +15 | Higher |
| PLOC | **56 ✅** | 62 | 60 | -6 | Lower | -4 | Lower | +2 | Higher |
| LLOC | **37 ✅** | 38 | 43 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | 0 | **19 ✅** | 9 | -19 | Lower | -9 | Lower | +10 | Higher |
| Blank Lines | **10 ✅** | 15 | 11 | -5 | Lower | -1 | Lower | +4 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **3 ✅** | 7 | +4 | Higher | 0 | Same | -4 | Lower |

## p00866

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.4490 | **60.8052 ✅** | 49.2689 | -8.3562 | Lower | +3.1801 | Higher | +11.5363 | Higher |
| MI (SEI) | 10.0662 | **46.7859 ✅** | 10.3404 | -36.7196 | Lower | -0.2741 | Lower | +36.4455 | Higher |
| Cyclomatic (sum) | 18 | 18 | 18 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 32 | **18 ✅** | 32 | +14 | Higher | 0 | Same | -14 | Lower |
| Halstead Volume | 3258.7290 | **1620.2520 ✅** | 3659.5919 | +1638.4770 | Higher | -400.8629 | Lower | -2039.3399 | Lower |
| Halstead Effort | 191959.5052 | **63372.7606 ✅** | 231020.0563 | +128586.7446 | Higher | -39060.5511 | Lower | -167647.2957 | Lower |
| SLOC | 87 | **65 ✅** | 102 | +22 | Higher | -15 | Lower | -37 | Lower |
| PLOC | 76 | **44 ✅** | 87 | +32 | Higher | -11 | Lower | -43 | Lower |
| LLOC | 51 | **31 ✅** | 55 | +20 | Higher | -4 | Lower | -24 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 3 | -13 | Lower | -2 | Lower | +11 | Higher |
| Blank Lines | **10 ✅** | 12 | 13 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | **0 ✅** | 5 | 1 | -5 | Lower | -1 | Lower | +4 | Higher |

## p00867

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 11.2981 | **73.3670 ✅** | 9.6127 | -62.0689 | Lower | +1.6854 | Higher | +63.7543 | Higher |
| MI (SEI) | -44.3434 | **70.5482 ✅** | -47.0440 | -114.8917 | Lower | +2.7006 | Higher | +117.5922 | Higher |
| Cyclomatic (sum) | 84 | **5 ✅** | 84 | +79 | Higher | 0 | Same | -79 | Lower |
| Cognitive (sum) | 112 | **3 ✅** | 112 | +109 | Higher | 0 | Same | -109 | Lower |
| Halstead Volume | 12330.1431 | **808.5134 ✅** | 13085.9588 | +11521.6297 | Higher | -755.8157 | Lower | -12277.4453 | Lower |
| Halstead Effort | 1831026.2530 | **17602.0115 ✅** | 2032685.5955 | +1813424.2415 | Higher | -201659.3425 | Lower | -2015083.5840 | Lower |
| SLOC | 282 | **45 ✅** | 307 | +237 | Higher | -25 | Lower | -262 | Lower |
| PLOC | 250 | **27 ✅** | 273 | +223 | Higher | -23 | Lower | -246 | Lower |
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
| MI (original) | **71.7500 ✅** | 68.1754 | 68.3542 | +3.5747 | Higher | +3.3958 | Higher | -0.1788 | Lower |
| MI (SEI) | **60.4608 ✅** | 56.9934 | 24.2371 | +3.4674 | Higher | +36.2237 | Higher | +32.7563 | Higher |
| Cyclomatic (sum) | **7 ✅** | 11 | 13 | -4 | Lower | -6 | Lower | -2 | Lower |
| Cognitive (sum) | **9 ✅** | 17 | 15 | -8 | Lower | -6 | Lower | +2 | Higher |
| Halstead Volume | **1009.9948 ✅** | 1211.9369 | 1299.7003 | -201.9420 | Lower | -289.7054 | Lower | -87.7634 | Lower |
| Halstead Effort | **23411.6806 ✅** | 42369.3127 | 38139.4799 | -18957.6321 | Lower | -14727.7993 | Lower | +4229.8328 | Higher |
| SLOC | **45 ✅** | 50 | 47 | -5 | Lower | -2 | Lower | +3 | Higher |
| PLOC | **29 ✅** | 34 | 38 | -5 | Lower | -9 | Lower | -4 | Lower |
| LLOC | **24 ✅** | 27 | 25 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 9 | **11 ✅** | 0 | -2 | Lower | +9 | Higher | +11 | Higher |
| Blank Lines | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00889

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.8166 | **69.5737 ✅** | 44.2563 | -20.7572 | Lower | +4.5603 | Higher | +25.3174 | Higher |
| MI (SEI) | -3.1352 | **62.4531 ✅** | 2.6250 | -65.5883 | Lower | -5.7603 | Lower | +59.8280 | Higher |
| Cyclomatic (sum) | 21 | **8 ✅** | 25 | +13 | Higher | -4 | Lower | -17 | Lower |
| Cognitive (sum) | 40 | **9 ✅** | 44 | +31 | Higher | -4 | Lower | -35 | Lower |
| Halstead Volume | 3605.0542 | **935.9502 ✅** | 3831.3987 | +2669.1040 | Higher | -226.3445 | Lower | -2895.4485 | Lower |
| Halstead Effort | 330829.6667 | **23448.8947 ✅** | 304516.3757 | +307380.7720 | Higher | +26313.2910 | Higher | -281067.4810 | Lower |
| SLOC | 101 | **52 ✅** | 124 | +49 | Higher | -23 | Lower | -72 | Lower |
| PLOC | 91 | **31 ✅** | 108 | +60 | Higher | -17 | Lower | -77 | Lower |
| LLOC | 82 | **20 ✅** | 89 | +62 | Higher | -7 | Lower | -69 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 3 | -15 | Lower | -3 | Lower | +12 | Higher |
| Blank Lines | 10 | 10 | 14 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |

## p00897

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.4630 | **45.4306 ✅** | 21.8753 | -20.9677 | Lower | +2.5877 | Higher | +23.5554 | Higher |
| MI (SEI) | -36.0300 | **11.5300 ✅** | -32.5326 | -47.5600 | Lower | -3.4974 | Lower | +44.0626 | Higher |
| Cyclomatic (sum) | 43 | **24 ✅** | 48 | +19 | Higher | -5 | Lower | -24 | Lower |
| Cognitive (sum) | 58 | **30 ✅** | 53 | +28 | Higher | +5 | Higher | -23 | Lower |
| Halstead Volume | 9085.2901 | **3276.7703 ✅** | 9735.4827 | +5808.5198 | Higher | -650.1926 | Lower | -6458.7124 | Lower |
| Halstead Effort | 792516.8435 | **121466.4841 ✅** | 1072545.2246 | +671050.3594 | Higher | -280028.3811 | Lower | -951078.7405 | Lower |
| SLOC | 247 | **123 ✅** | 264 | +124 | Higher | -17 | Lower | -141 | Lower |
| PLOC | 210 | **100 ✅** | 226 | +110 | Higher | -16 | Lower | -126 | Lower |
| LLOC | 151 | **41 ✅** | 183 | +110 | Higher | -32 | Lower | -142 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 2 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 37 | **19 ✅** | 36 | +18 | Higher | +1 | Higher | -17 | Lower |
| Functions | 10 | **7 ✅** | 12 | +3 | Higher | -2 | Lower | -5 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |

## p00907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.0508 ✅** | 64.4758 | 60.3975 | +14.5750 | Higher | +18.6534 | Higher | +4.0783 | Higher |
| MI (SEI) | **67.9960 ✅** | 55.0232 | 32.2165 | +12.9728 | Higher | +35.7795 | Higher | +22.8067 | Higher |
| Cyclomatic (sum) | 28 | **12 ✅** | 20 | +16 | Higher | +8 | Higher | -8 | Lower |
| Cognitive (sum) | 47 | **10 ✅** | 33 | +37 | Higher | +14 | Higher | -23 | Lower |
| Halstead Volume | 3003.2716 | **1271.1819 ✅** | 1604.0448 | +1732.0897 | Higher | +1399.2269 | Higher | -332.8629 | Lower |
| Halstead Effort | 87765.5105 | **41525.2746 ✅** | 81405.2713 | +46240.2359 | Higher | +6360.2392 | Higher | -39879.9966 | Lower |
| SLOC | **15 ✅** | 61 | 65 | -46 | Lower | -50 | Lower | -4 | Lower |
| PLOC | 57 | **36 ✅** | 53 | +21 | Higher | +4 | Higher | -17 | Lower |
| LLOC | 38 | **21 ✅** | 39 | +17 | Higher | -1 | Lower | -18 | Lower |
| Comments (CLOC) | 2 | **17 ✅** | 4 | -15 | Lower | -2 | Lower | +13 | Higher |
| Blank Lines | **-42 ✅** | 13 | 10 | -55 | Lower | -52 | Lower | +3 | Higher |
| Functions | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 5 | **1 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p00924

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.2825 | **74.1742 ✅** | 50.6973 | -24.8918 | Lower | -1.4148 | Lower | +23.4770 | Higher |
| MI (SEI) | 17.4151 | **69.8435 ✅** | 19.0490 | -52.4284 | Lower | -1.6338 | Lower | +50.7946 | Higher |
| Cyclomatic (sum) | 25 | **10 ✅** | 21 | +15 | Higher | +4 | Higher | -11 | Lower |
| Cognitive (sum) | 25 | **8 ✅** | 25 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 3955.2979 | **800.9165 ✅** | 3596.2881 | +3154.3814 | Higher | +359.0098 | Higher | -2795.3716 | Lower |
| Halstead Effort | 249799.0358 | **19455.5962 ✅** | 224887.8806 | +230343.4396 | Higher | +24911.1553 | Higher | -205432.2843 | Lower |
| SLOC | 90 | **40 ✅** | 90 | +50 | Higher | 0 | Same | -50 | Lower |
| PLOC | 74 | **24 ✅** | 74 | +50 | Higher | 0 | Same | -50 | Lower |
| LLOC | 61 | **17 ✅** | 61 | +44 | Higher | 0 | Same | -44 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 6 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 11 | **6 ✅** | 11 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 6 | **0 ✅** | 2 | +6 | Higher | +4 | Higher | -2 | Lower |
| Exits | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |

## p00949

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.9455 | **55.3071 ✅** | 42.3790 | -8.3616 | Lower | +4.5665 | Higher | +12.9282 | Higher |
| MI (SEI) | 10.5545 | **36.2656 ✅** | -0.5289 | -25.7111 | Lower | +11.0835 | Higher | +36.7946 | Higher |
| Cyclomatic (sum) | 22 | **17 ✅** | 25 | +5 | Higher | -3 | Lower | -8 | Lower |
| Cognitive (sum) | 22 | **20 ✅** | 30 | +2 | Higher | -8 | Lower | -10 | Lower |
| Halstead Volume | 3897.9412 | **2192.9603 ✅** | 4317.3179 | +1704.9809 | Higher | -419.3767 | Lower | -2124.3576 | Lower |
| Halstead Effort | 270257.2585 | **96983.6684 ✅** | 354522.0825 | +173273.5901 | Higher | -84264.8240 | Lower | -257538.4141 | Lower |
| SLOC | 109 | **84 ✅** | 134 | +25 | Higher | -25 | Lower | -50 | Lower |
| PLOC | 94 | **54 ✅** | 114 | +40 | Higher | -20 | Lower | -60 | Lower |
| LLOC | 75 | **40 ✅** | 91 | +35 | Higher | -16 | Lower | -51 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 3 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | **15 ✅** | 17 | 18 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | **0 ✅** | 2 | +3 | Higher | +1 | Higher | -2 | Lower |
| Exits | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |

## p00971

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.6496 | **64.1132 ✅** | 57.2318 | -12.4635 | Lower | -5.5822 | Lower | +6.8813 | Higher |
| MI (SEI) | 8.3236 | **52.2146 ✅** | 8.3945 | -43.8910 | Lower | -0.0709 | Lower | +43.8201 | Higher |
| Cyclomatic (sum) | 14 | **12 ✅** | 15 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | 3943.0336 | **1881.8482 ✅** | 3053.8624 | +2061.1854 | Higher | +889.1712 | Higher | -1172.0142 | Lower |
| Halstead Effort | 322202.1782 | **94092.4107 ✅** | 218860.1414 | +228109.7676 | Higher | +103342.0368 | Higher | -124767.7307 | Lower |
| SLOC | 91 | **55 ✅** | 69 | +36 | Higher | +22 | Higher | -14 | Lower |
| PLOC | 79 | **38 ✅** | 60 | +41 | Higher | +19 | Higher | -22 | Lower |
| LLOC | 65 | **34 ✅** | 51 | +31 | Higher | +14 | Higher | -17 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 0 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 12 | 9 | 9 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 5 | **1 ✅** | 3 | +4 | Higher | +2 | Higher | -2 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 3 | **0 ✅** | 2 | +3 | Higher | +1 | Higher | -2 | Lower |

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
| MI (original) | 55.2815 | **72.1254 ✅** | 59.6021 | -16.8439 | Lower | -4.3206 | Lower | +12.5233 | Higher |
| MI (SEI) | 23.3179 | **66.9905 ✅** | 25.4385 | -43.6726 | Lower | -2.1205 | Lower | +41.5520 | Higher |
| Cyclomatic (sum) | 15 | **6 ✅** | 16 | +9 | Higher | -1 | Lower | -10 | Lower |
| Cognitive (sum) | 30 | **6 ✅** | 31 | +24 | Higher | -1 | Lower | -25 | Lower |
| Halstead Volume | 3728.1303 | **982.1539 ✅** | 2341.2919 | +2745.9764 | Higher | +1386.8384 | Higher | -1359.1380 | Lower |
| Halstead Effort | 135261.2271 | **29157.6945 ✅** | 139444.5931 | +106103.5325 | Higher | -4183.3660 | Lower | -110286.8986 | Lower |
| SLOC | 73 | **45 ✅** | 64 | +28 | Higher | +9 | Higher | -19 | Lower |
| PLOC | 59 | **28 ✅** | 53 | +31 | Higher | +6 | Higher | -25 | Lower |
| LLOC | 34 | **12 ✅** | 40 | +22 | Higher | -6 | Lower | -28 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 2 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 10 | **6 ✅** | 9 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.2066 | **76.2919 ✅** | 68.9098 | -7.0854 | Lower | +0.2967 | Higher | +7.3821 | Higher |
| MI (SEI) | 52.9443 | **71.6173 ✅** | 59.8945 | -18.6729 | Lower | -6.9502 | Lower | +11.7227 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 1233.1187 | **808.5182 ✅** | 1394.0325 | +424.6005 | Higher | -160.9138 | Lower | -585.5143 | Lower |
| Halstead Effort | 39607.7734 | **16140.4192 ✅** | 52758.7677 | +23467.3542 | Higher | -13150.9942 | Lower | -36618.3484 | Lower |
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
| MI (original) | 47.7259 | **60.8497 ✅** | 31.4260 | -13.1239 | Lower | +16.2999 | Higher | +29.4237 | Higher |
| MI (SEI) | 10.4929 | **50.1332 ✅** | -18.1117 | -39.6403 | Lower | +28.6045 | Higher | +68.2449 | Higher |
| Cyclomatic (sum) | 22 | **17 ✅** | 43 | +5 | Higher | -21 | Lower | -26 | Lower |
| Cognitive (sum) | 63 | **14 ✅** | 67 | +49 | Higher | -4 | Lower | -53 | Lower |
| Halstead Volume | 4125.4041 | **1394.0262 ✅** | 5219.2513 | +2731.3779 | Higher | -1093.8472 | Lower | -3825.2251 | Lower |
| Halstead Effort | 110337.0788 | **41356.1100 ✅** | 376042.7782 | +68980.9688 | Higher | -265705.6994 | Lower | -334686.6682 | Lower |
| SLOC | 102 | **69 ✅** | 192 | +33 | Higher | -90 | Lower | -123 | Lower |
| PLOC | 88 | **47 ✅** | 171 | +41 | Higher | -83 | Lower | -124 | Lower |
| LLOC | 69 | **23 ✅** | 120 | +46 | Higher | -51 | Lower | -97 | Lower |
| Comments (CLOC) | 4 | **19 ✅** | 2 | -15 | Lower | +2 | Higher | +17 | Higher |
| Blank Lines | 12 | **11 ✅** | 19 | +1 | Higher | -7 | Lower | -8 | Lower |
| Functions | **1 ✅** | 3 | 10 | -2 | Lower | -9 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 4 | 11 | -4 | Lower | -11 | Lower | -7 | Lower |

## p01125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.7140 ✅** | 58.2392 | 59.1213 | +1.4748 | Higher | +0.5927 | Higher | -0.8821 | Lower |
| MI (SEI) | 41.8307 | **47.5959 ✅** | 20.4587 | -5.7652 | Lower | +21.3720 | Higher | +27.1372 | Higher |
| Cyclomatic (sum) | 20 | 24 | **15 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| Cognitive (sum) | 25 | 35 | **22 ✅** | -10 | Lower | +3 | Higher | +13 | Higher |
| Halstead Volume | 2229.6114 | **1615.7105 ✅** | 2222.2537 | +613.9009 | Higher | +7.3577 | Higher | -606.5432 | Lower |
| Halstead Effort | 86715.9587 | **71820.9392 ✅** | 79350.7183 | +14895.0195 | Higher | +7365.2404 | Higher | -7529.7791 | Lower |
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
| MI (original) | 68.1443 | 65.9590 | **68.4630 ✅** | +2.1853 | Higher | -0.3186 | Lower | -2.5040 | Lower |
| MI (SEI) | 44.8409 | **51.7338 ✅** | 49.7185 | -6.8928 | Lower | -4.8776 | Lower | +2.0153 | Higher |
| Cyclomatic (sum) | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 4 | 13 | 4 | -9 | Lower | 0 | Same | +9 | Higher |
| Halstead Volume | 1482.6295 | **1409.0684 ✅** | 1624.5621 | +73.5611 | Higher | -141.9325 | Lower | -215.4936 | Lower |
| Halstead Effort | 44478.8862 | **36999.4098 ✅** | 49041.4678 | +7479.4764 | Higher | -4562.5816 | Lower | -12042.0580 | Lower |
| SLOC | 49 | 57 | **48 ✅** | -8 | Lower | +1 | Higher | +9 | Higher |
| PLOC | 36 | 37 | **35 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| LLOC | **24 ✅** | 32 | 29 | -8 | Lower | -5 | Lower | +3 | Higher |
| Comments (CLOC) | 4 | **11 ✅** | 6 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p01150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.4198 | **63.5539 ✅** | 25.8738 | -39.1341 | Lower | -1.4539 | Lower | +37.6801 | Higher |
| MI (SEI) | -17.9281 | **52.0687 ✅** | -16.7989 | -69.9969 | Lower | -1.1292 | Lower | +68.8676 | Higher |
| Cyclomatic (sum) | 60 | **13 ✅** | 56 | +47 | Higher | +4 | Higher | -43 | Lower |
| Cognitive (sum) | 76 | **9 ✅** | 63 | +67 | Higher | +13 | Higher | -54 | Lower |
| Halstead Volume | 6751.8192 | **1191.4075 ✅** | 6758.0778 | +5560.4118 | Higher | -6.2586 | Lower | -5566.6703 | Lower |
| Halstead Effort | 1057785.0088 | **41302.1251 ✅** | 999114.2191 | +1016482.8837 | Higher | +58670.7897 | Higher | -957812.0940 | Lower |
| SLOC | 214 | **65 ✅** | 207 | +149 | Higher | +7 | Higher | -142 | Lower |
| PLOC | 189 | **40 ✅** | 182 | +149 | Higher | +7 | Higher | -142 | Lower |
| LLOC | 124 | **16 ✅** | 150 | +108 | Higher | -26 | Lower | -134 | Lower |
| Comments (CLOC) | 10 | **16 ✅** | 9 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 18 | **10 ✅** | 20 | +8 | Higher | -2 | Lower | -10 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |

## p01153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.2552 | **72.2123 ✅** | 67.1756 | -3.9571 | Lower | +1.0797 | Higher | +5.0367 | Higher |
| MI (SEI) | 53.3452 | **63.7512 ✅** | 37.6801 | -10.4060 | Lower | +15.6651 | Higher | +26.0711 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 12 | +4 | Higher | -2 | Lower | -6 | Lower |
| Cognitive (sum) | 20 | **2 ✅** | 32 | +18 | Higher | -12 | Lower | -30 | Lower |
| Halstead Volume | 1416.6079 | **965.8751 ✅** | 1405.3110 | +450.7328 | Higher | +11.2969 | Higher | -439.4359 | Lower |
| Halstead Effort | 56228.4368 | **24893.2362 ✅** | 58741.9989 | +31335.2006 | Higher | -2513.5622 | Lower | -33848.7627 | Lower |
| SLOC | 48 | **45 ✅** | 50 | +3 | Higher | -2 | Lower | -5 | Lower |
| PLOC | 34 | **27 ✅** | 41 | +7 | Higher | -7 | Lower | -14 | Lower |
| LLOC | 25 | **18 ✅** | 33 | +7 | Higher | -8 | Lower | -15 | Lower |
| Comments (CLOC) | 8 | **11 ✅** | 2 | -3 | Lower | +6 | Higher | +9 | Higher |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p01184

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.5581 | **62.0055 ✅** | 33.9956 | -26.4473 | Lower | +1.5625 | Higher | +28.0098 | Higher |
| MI (SEI) | -10.7220 | **49.3153 ✅** | -14.5199 | -60.0373 | Lower | +3.7980 | Higher | +63.8352 | Higher |
| Cyclomatic (sum) | 34 | **14 ✅** | 38 | +20 | Higher | -4 | Lower | -24 | Lower |
| Cognitive (sum) | 55 | **25 ✅** | 63 | +30 | Higher | -8 | Lower | -38 | Lower |
| Halstead Volume | 5129.0570 | **1333.9029 ✅** | 5397.9921 | +3795.1541 | Higher | -268.9351 | Lower | -4064.0892 | Lower |
| Halstead Effort | 411179.4011 | **37171.4273 ✅** | 459471.9467 | +374007.9738 | Higher | -48292.5456 | Lower | -422300.5194 | Lower |
| SLOC | 170 | **68 ✅** | 174 | +102 | Higher | -4 | Lower | -106 | Lower |
| PLOC | 142 | **42 ✅** | 148 | +100 | Higher | -6 | Lower | -106 | Lower |
| LLOC | 102 | **29 ✅** | 106 | +73 | Higher | -4 | Lower | -77 | Lower |
| Comments (CLOC) | 3 | **16 ✅** | 2 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | 26 | **10 ✅** | 24 | +16 | Higher | +2 | Higher | -14 | Lower |
| Functions | 5 | **1 ✅** | 5 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **1 ✅** | 9 | +6 | Higher | -2 | Lower | -8 | Lower |

## p01255

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.2986 | **63.3713 ✅** | 10.2095 | -26.0727 | Lower | +27.0891 | Higher | +53.1618 | Higher |
| MI (SEI) | -6.7184 | **40.2952 ✅** | -46.9303 | -47.0136 | Lower | +40.2119 | Higher | +87.2255 | Higher |
| Cyclomatic (sum) | 31 | **12 ✅** | 79 | +19 | Higher | -48 | Lower | -67 | Lower |
| Cognitive (sum) | 50 | **12 ✅** | 152 | +38 | Higher | -102 | Lower | -140 | Lower |
| Halstead Volume | 4777.4135 | **1572 ✅** | 11404.8054 | +3205.4135 | Higher | -6627.3919 | Lower | -9832.8054 | Lower |
| Halstead Effort | 232755.5866 | **37877.7143 ✅** | 1311686.7992 | +194877.8724 | Higher | -1078931.2125 | Lower | -1273809.0849 | Lower |
| SLOC | 163 | **61 ✅** | 332 | +102 | Higher | -169 | Lower | -271 | Lower |
| PLOC | 138 | **46 ✅** | 302 | +92 | Higher | -164 | Lower | -256 | Lower |
| LLOC | 69 | **25 ✅** | 187 | +44 | Higher | -118 | Lower | -162 | Lower |
| Comments (CLOC) | 4 | **6 ✅** | 2 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 22 | **12 ✅** | 28 | +10 | Higher | -6 | Lower | -16 | Lower |
| Functions | 6 | **3 ✅** | 17 | +3 | Higher | -11 | Lower | -14 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **2 ✅** | 26 | +5 | Higher | -19 | Lower | -24 | Lower |

## p01275

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.3887 | **64.8433 ✅** | 48.2961 | -6.4546 | Lower | +10.0925 | Higher | +16.5471 | Higher |
| MI (SEI) | 30.2809 | **51.6539 ✅** | 16.7937 | -21.3730 | Lower | +13.4872 | Higher | +34.8602 | Higher |
| Cyclomatic (sum) | 16 | **10 ✅** | 24 | +6 | Higher | -8 | Lower | -14 | Lower |
| Cognitive (sum) | 14 | **10 ✅** | 20 | +4 | Higher | -6 | Lower | -10 | Lower |
| Halstead Volume | 2236.4258 | **1293.9998 ✅** | 2751.7866 | +942.4260 | Higher | -515.3607 | Lower | -1457.7867 | Lower |
| Halstead Effort | 94753.8310 | **43133.3272 ✅** | 144468.7939 | +51620.5038 | Higher | -49714.9629 | Lower | -101335.4667 | Lower |
| SLOC | 70 | **61 ✅** | 109 | +9 | Higher | -39 | Lower | -48 | Lower |
| PLOC | 54 | **37 ✅** | 87 | +17 | Higher | -33 | Lower | -50 | Lower |
| LLOC | 34 | **27 ✅** | 56 | +7 | Higher | -22 | Lower | -29 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 8 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | **11 ✅** | 13 | 17 | -2 | Lower | -6 | Lower | -4 | Lower |
| Functions | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.4138 | **64.1211 ✅** | 62.2394 | -0.7074 | Lower | +1.1744 | Higher | +1.8818 | Higher |
| MI (SEI) | 47.8111 | **57.4321 ✅** | 15.2116 | -9.6210 | Lower | +32.5995 | Higher | +42.2205 | Higher |
| Cyclomatic (sum) | **10 ✅** | 12 | 11 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 20 | **10 ✅** | 22 | +10 | Higher | -2 | Lower | -12 | Lower |
| Halstead Volume | 1889.8564 | **1015.9928 ✅** | 1941.7650 | +873.8635 | Higher | -51.9087 | Lower | -925.7722 | Lower |
| Halstead Effort | 78251.8651 | **34520.6652 ✅** | 92401.2327 | +43731.1999 | Higher | -14149.3676 | Lower | -57880.5674 | Lower |
| SLOC | **59 ✅** | 67 | 62 | -8 | Lower | -3 | Lower | +5 | Higher |
| PLOC | 48 | **41 ✅** | 53 | +7 | Higher | -5 | Lower | -12 | Lower |
| LLOC | 44 | **23 ✅** | 50 | +21 | Higher | -6 | Lower | -27 | Lower |
| Comments (CLOC) | 11 | **23 ✅** | 0 | -12 | Lower | +11 | Higher | +23 | Higher |
| Blank Lines | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p01316

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.4027 | **70.5487 ✅** | 58.3685 | -12.1460 | Lower | +0.0343 | Higher | +12.1803 | Higher |
| MI (SEI) | 23.2314 | **70.3721 ✅** | 10.2380 | -47.1407 | Lower | +12.9934 | Higher | +60.1341 | Higher |
| Cyclomatic (sum) | 17 | **12 ✅** | 17 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 35 | **12 ✅** | 35 | +23 | Higher | 0 | Same | -23 | Lower |
| Halstead Volume | 2133.8889 | **782.3234 ✅** | 2148 | +1351.5655 | Higher | -14.1111 | Lower | -1365.6766 | Lower |
| Halstead Effort | 91251.8294 | **32857.5841 ✅** | 83127.6000 | +58394.2453 | Higher | +8124.2294 | Higher | -50270.0159 | Lower |
| SLOC | 70 | **49 ✅** | 70 | +21 | Higher | 0 | Same | -21 | Lower |
| PLOC | 57 | **25 ✅** | 58 | +32 | Higher | -1 | Lower | -33 | Lower |
| LLOC | 35 | **17 ✅** | 36 | +18 | Higher | -1 | Lower | -19 | Lower |
| Comments (CLOC) | 2 | **22 ✅** | 0 | -20 | Lower | +2 | Higher | +22 | Higher |
| Blank Lines | 13 | **6 ✅** | 12 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p01317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.1399 | **55.8504 ✅** | 49.9941 | -2.7105 | Lower | +3.1457 | Higher | +5.8563 | Higher |
| MI (SEI) | 2.6947 | **30.4507 ✅** | 6.2738 | -27.7560 | Lower | -3.5791 | Lower | +24.1770 | Higher |
| Cyclomatic (sum) | 17 | **14 ✅** | 19 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 38 | **18 ✅** | 41 | +20 | Higher | -3 | Lower | -23 | Lower |
| Halstead Volume | 3726 | **2174.0748 ✅** | 3800.5437 | +1551.9252 | Higher | -74.5437 | Lower | -1626.4689 | Lower |
| Halstead Effort | 230302.2857 | **96435.7456 ✅** | 210336.3415 | +133866.5401 | Higher | +19965.9442 | Higher | -113900.5959 | Lower |
| SLOC | **81 ✅** | 85 | 95 | -4 | Lower | -14 | Lower | -10 | Lower |
| PLOC | 69 | **63 ✅** | 79 | +6 | Higher | -10 | Lower | -16 | Lower |
| LLOC | 61 | **26 ✅** | 68 | +35 | Higher | -7 | Lower | -42 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 1 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | **12 ✅** | 14 | 16 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01339

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.1620 | **69.1642 ✅** | 36.2604 | -32.0022 | Lower | +0.9016 | Higher | +32.9038 | Higher |
| MI (SEI) | -8.1361 | **61.8823 ✅** | -11.4899 | -70.0184 | Lower | +3.3538 | Higher | +73.3722 | Higher |
| Cyclomatic (sum) | 32 | **6 ✅** | 33 | +26 | Higher | -1 | Lower | -27 | Lower |
| Cognitive (sum) | 31 | **0 ✅** | 31 | +31 | Higher | 0 | Same | -31 | Lower |
| Halstead Volume | 5488.9090 | **787.2783 ✅** | 5339.3021 | +4701.6306 | Higher | +149.6068 | Higher | -4552.0238 | Lower |
| Halstead Effort | 363038.3639 | **12596.4535 ✅** | 415104.5678 | +350441.9104 | Higher | -52066.2039 | Lower | -402508.1143 | Lower |
| SLOC | 155 | **58 ✅** | 163 | +97 | Higher | -8 | Lower | -105 | Lower |
| PLOC | 134 | **33 ✅** | 145 | +101 | Higher | -11 | Lower | -112 | Lower |
| LLOC | 90 | **10 ✅** | 94 | +80 | Higher | -4 | Lower | -84 | Lower |
| Comments (CLOC) | 3 | **17 ✅** | 2 | -14 | Lower | +1 | Higher | +15 | Higher |
| Blank Lines | 19 | **11 ✅** | 16 | +8 | Higher | +3 | Higher | -5 | Lower |
| Functions | 8 | **4 ✅** | 9 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 5 | +2 | Higher | -1 | Lower | -3 | Lower |

## p01353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.0444 | **70.9965 ✅** | 56.3726 | -14.9521 | Lower | -0.3282 | Lower | +14.6239 | Higher |
| MI (SEI) | 24.8274 | **63.8161 ✅** | 18.8289 | -38.9887 | Lower | +5.9985 | Higher | +44.9872 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 12 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 12 | +11 | Higher | +2 | Higher | -9 | Lower |
| Halstead Volume | 2374.1105 | **1019.5698 ✅** | 2402.6789 | +1354.5407 | Higher | -28.5684 | Lower | -1383.1091 | Lower |
| Halstead Effort | 121317.0470 | **20259.8394 ✅** | 138462.0750 | +101057.2075 | Higher | -17145.0280 | Lower | -118202.2355 | Lower |
| SLOC | 84 | **47 ✅** | 82 | +37 | Higher | +2 | Higher | -35 | Lower |
| PLOC | 67 | **27 ✅** | 68 | +40 | Higher | -1 | Lower | -41 | Lower |
| LLOC | 50 | **15 ✅** | 54 | +35 | Higher | -4 | Lower | -39 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 2 | -8 | Lower | +3 | Higher | +11 | Higher |
| Blank Lines | 12 | **9 ✅** | 12 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |

## p01368

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.8033 | **55.9930 ✅** | 46.4786 | -9.1896 | Lower | +0.3247 | Higher | +9.5144 | Higher |
| MI (SEI) | 12.4838 | **39.5219 ✅** | 4.4639 | -27.0381 | Lower | +8.0199 | Higher | +35.0580 | Higher |
| Cyclomatic (sum) | 26 | **13 ✅** | 26 | +13 | Higher | 0 | Same | -13 | Lower |
| Cognitive (sum) | 61 | **16 ✅** | 45 | +45 | Higher | +16 | Higher | -29 | Lower |
| Halstead Volume | 3661.2855 | **2056.4042 ✅** | 3676.7558 | +1604.8813 | Higher | -15.4703 | Lower | -1620.3516 | Lower |
| Halstead Effort | 266094.0956 | **90579.7104 ✅** | 339732.2348 | +175514.3852 | Higher | -73638.1392 | Lower | -249152.5244 | Lower |
| SLOC | 106 | **87 ✅** | 108 | +19 | Higher | -2 | Lower | -21 | Lower |
| PLOC | 86 | **60 ✅** | 96 | +26 | Higher | -10 | Lower | -36 | Lower |
| LLOC | 67 | **32 ✅** | 78 | +35 | Higher | -11 | Lower | -46 | Lower |
| Comments (CLOC) | 6 | **19 ✅** | 2 | -13 | Lower | +4 | Higher | +17 | Higher |
| Blank Lines | 14 | 13 | **10 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01452

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0789 | **65.4634 ✅** | 55.0398 | -12.3846 | Lower | -1.9609 | Lower | +10.4236 | Higher |
| MI (SEI) | 14.8621 | **55.8518 ✅** | 14.0137 | -40.9896 | Lower | +0.8485 | Higher | +41.8381 | Higher |
| Cyclomatic (sum) | 19 | **14 ✅** | 15 | +5 | Higher | +4 | Higher | -1 | Lower |
| Cognitive (sum) | 10 | 8 | **6 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Halstead Volume | 3450.7656 | **1188.6820 ✅** | 3307.5027 | +2262.0835 | Higher | +143.2628 | Higher | -2118.8207 | Lower |
| Halstead Effort | 197987.6737 | **52715.4632 ✅** | 183256.3228 | +145272.2105 | Higher | +14731.3509 | Higher | -130540.8596 | Lower |
| SLOC | 81 | **57 ✅** | 77 | +24 | Higher | +4 | Higher | -20 | Lower |
| PLOC | 68 | **40 ✅** | 62 | +28 | Higher | +6 | Higher | -22 | Lower |
| LLOC | 35 | **27 ✅** | 34 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 2 | **15 ✅** | 1 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | 13 | **8 ✅** | 15 | +5 | Higher | -2 | Lower | -7 | Lower |
| Functions | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **5 ✅** | 6 | +2 | Higher | +1 | Higher | -1 | Lower |

## p01459

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.5072 | **51.0347 ✅** | 38.6972 | -9.5275 | Lower | +2.8100 | Higher | +12.3375 | Higher |
| MI (SEI) | -3.3083 | **28.3277 ✅** | -7.4090 | -31.6360 | Lower | +4.1007 | Higher | +35.7367 | Higher |
| Cyclomatic (sum) | **28 ✅** | 30 | 32 | -2 | Lower | -4 | Lower | -2 | Lower |
| Cognitive (sum) | 47 | **43 ✅** | 62 | +4 | Higher | -15 | Lower | -19 | Lower |
| Halstead Volume | 5284.3292 | **2607.9704 ✅** | 5610.2633 | +2676.3588 | Higher | -325.9340 | Lower | -3002.2929 | Lower |
| Halstead Effort | 331054.9706 | **157968.4936 ✅** | 415782.8455 | +173086.4770 | Higher | -84727.8749 | Lower | -257814.3518 | Lower |
| SLOC | 127 | **86 ✅** | 140 | +41 | Higher | -13 | Lower | -54 | Lower |
| PLOC | 106 | **66 ✅** | 118 | +40 | Higher | -12 | Lower | -52 | Lower |
| LLOC | 63 | **41 ✅** | 98 | +22 | Higher | -35 | Lower | -57 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 19 | **13 ✅** | 20 | +6 | Higher | -1 | Lower | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 7 | 1 | -6 | Lower | 0 | Same | +6 | Higher |

## p01471

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.3132 | **61.0049 ✅** | 56.0668 | -0.6916 | Lower | +4.2465 | Higher | +4.9381 | Higher |
| MI (SEI) | 35.0968 | **44.2221 ✅** | 18.8258 | -9.1253 | Lower | +16.2710 | Higher | +25.3963 | Higher |
| Cyclomatic (sum) | 14 | **13 ✅** | 17 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 22 | **9 ✅** | 19 | +13 | Higher | +3 | Higher | -10 | Lower |
| Halstead Volume | 1934.2061 | **1414.3882 ✅** | 1966.9449 | +519.8179 | Higher | -32.7388 | Lower | -552.5567 | Lower |
| Halstead Effort | 66283.7558 | **43771.5935 ✅** | 84136.0670 | +22512.1624 | Higher | -17852.3112 | Lower | -40364.4736 | Lower |
| SLOC | **67 ✅** | 72 | 83 | -5 | Lower | -16 | Lower | -11 | Lower |
| PLOC | 57 | **54 ✅** | 70 | +3 | Higher | -13 | Lower | -16 | Lower |
| LLOC | 34 | **25 ✅** | 43 | +9 | Higher | -9 | Lower | -18 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 2 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | **9 ✅** | 11 | 11 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | **1 ✅** | 3 | 4 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 4 | 3 | -4 | Lower | -3 | Lower | +1 | Higher |

## p01521

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.4134 ✅** | 72.2804 | 80.6528 | +12.1329 | Higher | +3.7606 | Higher | -8.3723 | Lower |
| MI (SEI) | 74.7715 | 67.2929 | **75.2883 ✅** | +7.4785 | Higher | -0.5168 | Lower | -7.9953 | Lower |
| Cyclomatic (sum) | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **453.9955 ✅** | 640.2992 | 673.8746 | -186.3038 | Lower | -219.8791 | Lower | -33.5753 | Lower |
| Halstead Effort | **5864.1082 ✅** | 11414.0299 | 12736.2292 | -5549.9218 | Lower | -6872.1210 | Lower | -1322.1992 | Lower |
| SLOC | **27 ✅** | 49 | 30 | -22 | Lower | -3 | Lower | +19 | Higher |
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
| MI (original) | 60.3373 | **73.5688 ✅** | 61.9876 | -13.2315 | Lower | -1.6503 | Lower | +11.5812 | Higher |
| MI (SEI) | 26.0918 | **64.7747 ✅** | 31.6903 | -38.6829 | Lower | -5.5985 | Lower | +33.0843 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 11 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 21 | **12 ✅** | 17 | +9 | Higher | +4 | Higher | -5 | Lower |
| Halstead Volume | 2211.5936 | **983.0417 ✅** | 2144.0210 | +1228.5519 | Higher | +67.5725 | Higher | -1160.9793 | Lower |
| Halstead Effort | 119426.0529 | **37511.9778 ✅** | 122812.2054 | +81914.0751 | Higher | -3386.1526 | Lower | -85300.2277 | Lower |
| SLOC | 65 | **40 ✅** | 61 | +25 | Higher | +4 | Higher | -21 | Lower |
| PLOC | 52 | **27 ✅** | 48 | +25 | Higher | +4 | Higher | -21 | Lower |
| LLOC | 42 | **19 ✅** | 43 | +23 | Higher | -1 | Lower | -24 | Lower |
| Comments (CLOC) | 2 | **9 ✅** | 3 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 11 | **5 ✅** | 11 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |

## p01545

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.6578 | **66.7466 ✅** | 55.0872 | -8.0888 | Lower | +3.5706 | Higher | +11.6593 | Higher |
| MI (SEI) | 26.2220 | **58.3585 ✅** | 22.2802 | -32.1365 | Lower | +3.9417 | Higher | +36.0782 | Higher |
| Cyclomatic (sum) | 14 | **8 ✅** | 17 | +6 | Higher | -3 | Lower | -9 | Lower |
| Cognitive (sum) | 9 | **2 ✅** | 12 | +7 | Higher | -3 | Lower | -10 | Lower |
| Halstead Volume | 2426.4112 | **1087.6737 ✅** | 2466.0486 | +1338.7375 | Higher | -39.6374 | Lower | -1378.3749 | Lower |
| Halstead Effort | 149130.9670 | **23554.9344 ✅** | 152051.3647 | +125576.0326 | Higher | -2920.3977 | Lower | -128496.4303 | Lower |
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
| MI (original) | 50.3028 | **62.3580 ✅** | 46.5300 | -12.0552 | Lower | +3.7727 | Higher | +15.8279 | Higher |
| MI (SEI) | 6.6377 | **49.6201 ✅** | 3.6521 | -42.9824 | Lower | +2.9856 | Higher | +45.9680 | Higher |
| Cyclomatic (sum) | 19 | **10 ✅** | 20 | +9 | Higher | -1 | Lower | -10 | Lower |
| Cognitive (sum) | 23 | **7 ✅** | 24 | +16 | Higher | -1 | Lower | -17 | Lower |
| Halstead Volume | 3356.4392 | **1557.9944 ✅** | 4011.1695 | +1798.4448 | Higher | -654.7303 | Lower | -2453.1751 | Lower |
| Halstead Effort | 198248.8111 | **54947.1226 ✅** | 279611.9380 | +143301.6884 | Higher | -81363.1270 | Lower | -224664.8154 | Lower |
| SLOC | 97 | **67 ✅** | 114 | +30 | Higher | -17 | Lower | -47 | Lower |
| PLOC | 84 | **40 ✅** | 98 | +44 | Higher | -14 | Lower | -58 | Lower |
| LLOC | 68 | **24 ✅** | 90 | +44 | Higher | -22 | Lower | -66 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 2 | -15 | Lower | -1 | Lower | +14 | Higher |
| Blank Lines | 13 | 13 | 14 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p01600

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.0737 | **66.7528 ✅** | 64.2444 | -6.6791 | Lower | -4.1707 | Lower | +2.5084 | Higher |
| MI (SEI) | 37.3426 | **61.4279 ✅** | 32.9951 | -24.0853 | Lower | +4.3475 | Higher | +28.4327 | Higher |
| Cyclomatic (sum) | 13 | **9 ✅** | 13 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 14 | **12 ✅** | 21 | +2 | Higher | -7 | Lower | -9 | Lower |
| Halstead Volume | 2835.6489 | **1039.3651 ✅** | 1858.8287 | +1796.2838 | Higher | +976.8203 | Higher | -819.4636 | Lower |
| Halstead Effort | 74209.5360 | **74166.1231 ✅** | 95345.0909 | +43.4129 | Higher | -21135.5549 | Lower | -21178.9678 | Lower |
| SLOC | 61 | 59 | **54 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| PLOC | 44 | **28 ✅** | 43 | +16 | Higher | +1 | Higher | -15 | Lower |
| LLOC | 25 | **21 ✅** | 30 | +4 | Higher | -5 | Lower | -9 | Lower |
| Comments (CLOC) | 7 | **21 ✅** | 2 | -14 | Lower | +5 | Higher | +19 | Higher |
| Blank Lines | 11 | 11 | **9 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p01601

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.6433 | 73.1924 | **76.1566 ✅** | -13.5491 | Lower | -16.5133 | Lower | -2.9643 | Lower |
| MI (SEI) | 21.3671 | **68.0093 ✅** | 47.7515 | -46.6422 | Lower | -26.3844 | Lower | +20.2578 | Higher |
| Cyclomatic (sum) | 13 | 8 | 8 | +5 | Higher | +5 | Higher | 0 | Same |
| Cognitive (sum) | 6 | **3 ✅** | 5 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 2785.8047 | **785.3350 ✅** | 829.8336 | +2000.4697 | Higher | +1955.9711 | Higher | -44.4986 | Lower |
| Halstead Effort | 77306.0812 | **16148.4508 ✅** | 30572.8171 | +61157.6304 | Higher | +46733.2641 | Higher | -14424.3663 | Lower |
| SLOC | 63 | 44 | **36 ✅** | +19 | Higher | +27 | Higher | +8 | Higher |
| PLOC | 47 | **29 ✅** | 30 | +18 | Higher | +17 | Higher | -1 | Lower |
| LLOC | 27 | **19 ✅** | 22 | +8 | Higher | +5 | Higher | -3 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 1 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 16 | 7 | **6 ✅** | +9 | Higher | +10 | Higher | +1 | Higher |
| Functions | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 4 | **1 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |

## p01656

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.5942 ✅** | 73.3656 | 75.9083 | +3.2286 | Higher | +0.6859 | Higher | -2.5427 | Lower |
| MI (SEI) | 62.8410 | **68.8099 ✅** | 66.1545 | -5.9689 | Lower | -3.3135 | Lower | +2.6554 | Higher |
| Cyclomatic (sum) | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 3 | 7 | 3 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 994.0254 | **917.7708 ✅** | 1134.1714 | +76.2546 | Higher | -140.1460 | Lower | -216.4006 | Lower |
| Halstead Effort | **18523.2812 ✅** | 19517.9256 | 19756.5339 | -994.6444 | Lower | -1233.2527 | Lower | -238.6083 | Lower |
| SLOC | 35 | 42 | 35 | -7 | Lower | 0 | Same | +7 | Higher |
| PLOC | 24 | 29 | 24 | -5 | Lower | 0 | Same | +5 | Higher |
| LLOC | 20 | **16 ✅** | 20 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 7 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.9801 | **74.0557 ✅** | 68.6326 | -6.0756 | Lower | -0.6525 | Lower | +5.4231 | Higher |
| MI (SEI) | 53.8378 | **62.1361 ✅** | 57.0986 | -8.2982 | Lower | -3.2608 | Lower | +5.0375 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 10 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 13 | **7 ✅** | 13 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 1374.7009 | **895.1721 ✅** | 1406.7615 | +479.5289 | Higher | -32.0606 | Lower | -511.5894 | Lower |
| Halstead Effort | 57816.7478 | **22262.5398 ✅** | 59984.3097 | +35554.2081 | Higher | -2167.5619 | Lower | -37721.7699 | Lower |
| SLOC | 50 | **40 ✅** | 47 | +10 | Higher | +3 | Higher | -7 | Lower |
| PLOC | 38 | **26 ✅** | 37 | +12 | Higher | +1 | Higher | -11 | Lower |
| LLOC | 37 | **15 ✅** | 37 | +22 | Higher | 0 | Same | -22 | Lower |
| Comments (CLOC) | 9 | 7 | **10 ✅** | +2 | Higher | -1 | Lower | -3 | Lower |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.0500 | **59.4884 ✅** | 43.8788 | -18.4384 | Lower | -2.8288 | Lower | +15.6096 | Higher |
| MI (SEI) | -12.9145 | **43.6413 ✅** | -8.9352 | -56.5558 | Lower | -3.9793 | Lower | +52.5765 | Higher |
| Cyclomatic (sum) | 35 | **12 ✅** | 34 | +23 | Higher | +1 | Higher | -22 | Lower |
| Cognitive (sum) | 31 | **13 ✅** | 34 | +18 | Higher | -3 | Lower | -21 | Lower |
| Halstead Volume | 4677.4053 | **1895.6893 ✅** | 5084.5597 | +2781.7161 | Higher | -407.1544 | Lower | -3188.8704 | Lower |
| Halstead Effort | 356745.7034 | **82031.6441 ✅** | 451628.5383 | +274714.0593 | Higher | -94882.8349 | Lower | -369596.8942 | Lower |
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
| MI (original) | 58.5848 | **67.5743 ✅** | 60.9030 | -8.9895 | Lower | -2.3182 | Lower | +6.6713 | Higher |
| MI (SEI) | 34.2735 | **65.6015 ✅** | 22.9341 | -31.3280 | Lower | +11.3394 | Higher | +42.6673 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 12 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 12 | **11 ✅** | 12 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 2073.3186 | **1010.9352 ✅** | 2073.3186 | +1062.3834 | Higher | 0 | Same | -1062.3834 | Lower |
| Halstead Effort | 90372.2989 | **35612.4890 ✅** | 90372.2989 | +54759.8100 | Higher | 0 | Same | -54759.8100 | Lower |
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
| MI (original) | 49.2255 | **51.9429 ✅** | 48.2183 | -2.7174 | Lower | +1.0072 | Higher | +3.7246 | Higher |
| MI (SEI) | -3.0544 | **21.7030 ✅** | 9.4469 | -24.7574 | Lower | -12.5012 | Lower | +12.2561 | Higher |
| Cyclomatic (sum) | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | **7 ✅** | 11 | 11 | -4 | Lower | -4 | Lower | 0 | Same |
| Halstead Volume | 4869.6815 | **2089.7616 ✅** | 2949.2242 | +2779.9199 | Higher | +1920.4573 | Higher | -859.4625 | Lower |
| Halstead Effort | 117217.8399 | **56352.3222 ✅** | 128636.3725 | +60865.5177 | Higher | -11418.5326 | Lower | -72284.0504 | Lower |
| SLOC | **96 ✅** | 105 | 120 | -9 | Lower | -24 | Lower | -15 | Lower |
| PLOC | **78 ✅** | 80 | 102 | -2 | Lower | -24 | Lower | -22 | Lower |
| LLOC | 35 | **30 ✅** | 57 | +5 | Higher | -22 | Lower | -27 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 4 | -8 | Lower | -4 | Lower | +4 | Higher |
| Blank Lines | 18 | **17 ✅** | 18 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p01782

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 26.6626 | **61.1428 ✅** | 24.8735 | -34.4803 | Lower | +1.7891 | Higher | +36.2693 | Higher |
| MI (SEI) | -25.3961 | **46.9842 ✅** | -28.5179 | -72.3803 | Lower | +3.1218 | Higher | +75.5021 | Higher |
| Cyclomatic (sum) | 44 | **11 ✅** | 43 | +33 | Higher | +1 | Higher | -32 | Lower |
| Cognitive (sum) | 51 | **6 ✅** | 51 | +45 | Higher | 0 | Same | -45 | Lower |
| Halstead Volume | 8166.6590 | **1504.7422 ✅** | 8187.2711 | +6661.9168 | Higher | -20.6121 | Lower | -6682.5289 | Lower |
| Halstead Effort | 756395.9557 | **34784.6247 ✅** | 746181.6716 | +721611.3309 | Higher | +10214.2841 | Higher | -711397.0469 | Lower |
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
| MI (original) | 47.6268 | **51.6229 ✅** | 46.4380 | -3.9961 | Lower | +1.1888 | Higher | +5.1849 | Higher |
| MI (SEI) | 2.5770 | **25.3202 ✅** | 9.2705 | -22.7431 | Lower | -6.6934 | Lower | +16.0497 | Higher |
| Cyclomatic (sum) | 20 | 23 | 20 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | **15 ✅** | 22 | 16 | -7 | Lower | -1 | Lower | +6 | Higher |
| Halstead Volume | 4196.9807 | **2406 ✅** | 4082.7820 | +1790.9807 | Higher | +114.1987 | Higher | -1676.7820 | Lower |
| Halstead Effort | 273852.9908 | **80658.2857 ✅** | 228635.7935 | +193194.7051 | Higher | +45217.1973 | Higher | -147977.5078 | Lower |
| SLOC | 105 | **94 ✅** | 114 | +11 | Higher | -9 | Lower | -20 | Lower |
| PLOC | 89 | **77 ✅** | 93 | +12 | Higher | -4 | Lower | -16 | Lower |
| LLOC | 79 | **37 ✅** | 76 | +42 | Higher | +3 | Higher | -39 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 5 | -9 | Lower | -4 | Lower | +5 | Higher |
| Blank Lines | 16 | **12 ✅** | 16 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p01847

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.7712 | **59.2618 ✅** | 54.4284 | -8.4906 | Lower | -3.6572 | Lower | +4.8334 | Higher |
| MI (SEI) | 8.4136 | **43.5860 ✅** | 17.3155 | -35.1724 | Lower | -8.9019 | Lower | +26.2705 | Higher |
| Cyclomatic (sum) | 26 | **11 ✅** | 21 | +15 | Higher | +5 | Higher | -10 | Lower |
| Cognitive (sum) | 62 | **10 ✅** | 50 | +52 | Higher | +12 | Higher | -40 | Lower |
| Halstead Volume | 3048.1537 | **2256.7535 ✅** | 2853.0282 | +791.4002 | Higher | +195.1255 | Higher | -596.2746 | Lower |
| Halstead Effort | 192262.2949 | **75917.1880 ✅** | 173219.5667 | +116345.1069 | Higher | +19042.7282 | Higher | -97302.3787 | Lower |
| SLOC | 88 | **71 ✅** | 77 | +17 | Higher | +11 | Higher | -6 | Lower |
| PLOC | 71 | **49 ✅** | 63 | +22 | Higher | +8 | Higher | -14 | Lower |
| LLOC | 72 | **33 ✅** | 79 | +39 | Higher | -7 | Lower | -46 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 2 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | 17 | 12 | 12 | +5 | Higher | +5 | Higher | 0 | Same |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p01907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.6914 | **60.5942 ✅** | 39.5251 | -9.9028 | Lower | +11.1663 | Higher | +21.0691 | Higher |
| MI (SEI) | 6.6632 | **35.7705 ✅** | -7.6383 | -29.1074 | Lower | +14.3014 | Higher | +43.4088 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 23 | +5 | Higher | -7 | Lower | -12 | Lower |
| Cognitive (sum) | 13 | **5 ✅** | 25 | +8 | Higher | -12 | Lower | -20 | Lower |
| Halstead Volume | 2950.1694 | **1535.3698 ✅** | 4984.9707 | +1414.7996 | Higher | -2034.8013 | Lower | -3449.6009 | Lower |
| Halstead Effort | 118674.7385 | **37757.9766 ✅** | 333430.2173 | +80916.7619 | Higher | -214755.4788 | Lower | -295672.2406 | Lower |
| SLOC | 103 | **74 ✅** | 157 | +29 | Higher | -54 | Lower | -83 | Lower |
| PLOC | 84 | **57 ✅** | 131 | +27 | Higher | -47 | Lower | -74 | Lower |
| LLOC | 47 | **20 ✅** | 119 | +27 | Higher | -72 | Lower | -99 | Lower |
| Comments (CLOC) | 1 | **7 ✅** | 2 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 19 | **14 ✅** | 24 | +5 | Higher | -5 | Lower | -10 | Lower |
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
| MI (original) | 54.1884 | **70.2290 ✅** | 54.1678 | -16.0406 | Lower | +0.0205 | Higher | +16.0612 | Higher |
| MI (SEI) | 4.7165 | **57.1246 ✅** | 4.6869 | -52.4081 | Lower | +0.0296 | Higher | +52.4378 | Higher |
| Cyclomatic (sum) | 22 | **11 ✅** | 22 | +11 | Higher | 0 | Same | -11 | Lower |
| Cognitive (sum) | 47 | **7 ✅** | 47 | +40 | Higher | 0 | Same | -40 | Lower |
| Halstead Volume | 2745.9314 | **1133.7474 ✅** | 2756.7929 | +1612.1840 | Higher | -10.8615 | Lower | -1623.0455 | Lower |
| Halstead Effort | 181545.2923 | **50915.5645 ✅** | 177200.5209 | +130629.7278 | Higher | +4344.7714 | Higher | -126284.9564 | Lower |
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
| MI (original) | 55.0903 | **60.5086 ✅** | 57.8228 | -5.4183 | Lower | -2.7325 | Lower | +2.6858 | Higher |
| MI (SEI) | 5.5086 | **45.9835 ✅** | 30.1774 | -40.4750 | Lower | -24.6688 | Lower | +15.8062 | Higher |
| Cyclomatic (sum) | 17 | **12 ✅** | 16 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 3122.8684 | **1557.9944 ✅** | 3141.1308 | +1564.8741 | Higher | -18.2624 | Lower | -1583.1365 | Lower |
| Halstead Effort | 256412.8193 | **83266.1432 ✅** | 280607.6871 | +173146.6761 | Higher | -24194.8679 | Lower | -197341.5439 | Lower |
| SLOC | 76 | 73 | **65 ✅** | +3 | Higher | +11 | Higher | +8 | Higher |
| PLOC | 62 | **51 ✅** | 56 | +11 | Higher | +6 | Higher | -5 | Lower |
| LLOC | 42 | 42 | 44 | 0 | Same | -2 | Lower | -2 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 5 | -16 | Lower | -5 | Lower | +11 | Higher |
| Blank Lines | 14 | 17 | **9 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| Functions | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.5621 | 71.8405 | **72.0142 ✅** | -4.2784 | Lower | -4.4521 | Lower | -0.1737 | Lower |
| MI (SEI) | 49.8877 | **58.6195 ✅** | 42.5898 | -8.7319 | Lower | +7.2979 | Higher | +16.0297 | Higher |
| Cyclomatic (sum) | 18 | **11 ✅** | 16 | +7 | Higher | +2 | Higher | -5 | Lower |
| Cognitive (sum) | 10 | 8 | **7 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Halstead Volume | 1213.2411 | **1031.0257 ✅** | 1292.0606 | +182.2154 | Higher | -78.8195 | Lower | -261.0349 | Lower |
| Halstead Effort | 38667.1687 | **23343.0658 ✅** | 31622.3549 | +15324.1029 | Higher | +7044.8138 | Higher | -8279.2891 | Lower |
| SLOC | 47 | 42 | **36 ✅** | +5 | Higher | +11 | Higher | +6 | Higher |
| PLOC | 32 | **28 ✅** | 29 | +4 | Higher | +3 | Higher | -1 | Lower |
| LLOC | 19 | 18 | **17 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Comments (CLOC) | 6 | **7 ✅** | 1 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 9 | 7 | 7 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 8 | **0 ✅** | 7 | +8 | Higher | +1 | Higher | -7 | Lower |

## p02045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.5929 | 57.3860 | **59.2515 ✅** | -5.7931 | Lower | -7.6586 | Lower | -1.8655 | Lower |
| MI (SEI) | 18.0580 | **39.2293 ✅** | 11.5120 | -21.1712 | Lower | +6.5461 | Higher | +27.7173 | Higher |
| Cyclomatic (sum) | 26 | 17 | 17 | +9 | Higher | +9 | Higher | 0 | Same |
| Cognitive (sum) | 23 | **7 ✅** | 13 | +16 | Higher | +10 | Higher | -6 | Lower |
| Halstead Volume | 3122.8684 | **2276.7023 ✅** | 2516.7121 | +846.1661 | Higher | +606.1564 | Higher | -240.0098 | Lower |
| Halstead Effort | 182002.2959 | **79926.7838 ✅** | 124607.9394 | +102075.5121 | Higher | +57394.3565 | Higher | -44681.1556 | Lower |
| SLOC | 83 | 73 | **63 ✅** | +10 | Higher | +20 | Higher | +10 | Higher |
| PLOC | 68 | **48 ✅** | 54 | +20 | Higher | +14 | Higher | -6 | Lower |
| LLOC | 56 | **31 ✅** | 49 | +25 | Higher | +7 | Higher | -18 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 0 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 12 | 13 | **9 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 6 | 2 | 2 | +4 | Higher | +4 | Higher | 0 | Same |
| Exits | 9 | 1 | 1 | +8 | Higher | +8 | Higher | 0 | Same |

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
| MI (original) | 42.2375 | **49.7621 ✅** | 21.6287 | -7.5246 | Lower | +20.6088 | Higher | +28.1334 | Higher |
| MI (SEI) | -2.5690 | **21.4380 ✅** | -40.2208 | -24.0070 | Lower | +37.6517 | Higher | +61.6587 | Higher |
| Cyclomatic (sum) | 26 | **21 ✅** | 42 | +5 | Higher | -16 | Lower | -21 | Lower |
| Cognitive (sum) | 14 | **11 ✅** | 40 | +3 | Higher | -26 | Lower | -29 | Lower |
| Halstead Volume | 4664.6911 | **2827.5731 ✅** | 11721.1411 | +1837.1180 | Higher | -7056.4500 | Lower | -8893.5680 | Lower |
| Halstead Effort | 270552.0833 | **108154.6711 ✅** | 1547776.6840 | +162397.4122 | Higher | -1277224.6007 | Lower | -1439622.0129 | Lower |
| SLOC | 130 | **103 ✅** | 275 | +27 | Higher | -145 | Lower | -172 | Lower |
| PLOC | 111 | **82 ✅** | 254 | +29 | Higher | -143 | Lower | -172 | Lower |
| LLOC | 62 | **34 ✅** | 206 | +28 | Higher | -144 | Lower | -172 | Lower |
| Comments (CLOC) | 2 | **10 ✅** | 0 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 19 | **18 ✅** | 21 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 6 | 6 | 11 | 0 | Same | -5 | Lower | -5 | Lower |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | **4 ✅** | 7 | 6 | -3 | Lower | -2 | Lower | +1 | Higher |

## p02061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.4696 ✅** | 54.5253 | 56.9462 | +3.9442 | Higher | +1.5234 | Higher | -2.4209 | Lower |
| MI (SEI) | 10.1802 | **37.9088 ✅** | 16.7640 | -27.7286 | Lower | -6.5838 | Lower | +21.1448 | Higher |
| Cyclomatic (sum) | 15 | 20 | 15 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 25 | 25 | 25 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 2201.9904 | **1625.4956 ✅** | 2292.3656 | +576.4948 | Higher | -90.3752 | Lower | -666.8700 | Lower |
| Halstead Effort | 131783.0074 | **62697.6871 ✅** | 155809.2230 | +69085.3202 | Higher | -24026.2156 | Lower | -93111.5358 | Lower |
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
| MI (original) | 57.2983 | **84.6799 ✅** | 60.8633 | -27.3816 | Lower | -3.5650 | Lower | +23.8166 | Higher |
| MI (SEI) | 34.3239 | **82.8749 ✅** | 29.6458 | -48.5509 | Lower | +4.6781 | Higher | +53.2291 | Higher |
| Cyclomatic (sum) | 13 | **4 ✅** | 13 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 14 | **2 ✅** | 15 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 2884.9357 | **530 ✅** | 1905.9763 | +2354.9357 | Higher | +978.9594 | Higher | -1375.9763 | Lower |
| Halstead Effort | 84417.1526 | **7626.1111 ✅** | 111038.4930 | +76791.0415 | Higher | -26621.3404 | Lower | -103412.3819 | Lower |
| SLOC | 72 | **26 ✅** | 66 | +46 | Higher | +6 | Higher | -40 | Lower |
| PLOC | 51 | **16 ✅** | 52 | +35 | Higher | -1 | Lower | -36 | Lower |
| LLOC | 41 | **12 ✅** | 48 | +29 | Higher | -7 | Lower | -36 | Lower |
| Comments (CLOC) | **9 ✅** | 7 | 3 | +2 | Higher | +6 | Higher | +4 | Higher |
| Blank Lines | 12 | **4 ✅** | 12 | +8 | Higher | 0 | Same | -8 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.3249 | **66.0676 ✅** | 65.5839 | -2.7427 | Lower | -2.2590 | Lower | +0.4837 | Higher |
| MI (SEI) | 51.5206 | **61.3988 ✅** | 38.1137 | -9.8782 | Lower | +13.4069 | Higher | +23.2851 | Higher |
| Cyclomatic (sum) | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 22 | **14 ✅** | 21 | +8 | Higher | +1 | Higher | -7 | Lower |
| Halstead Volume | 1657.8086 | **1309.0842 ✅** | 1845.2640 | +348.7244 | Higher | -187.4554 | Lower | -536.1798 | Lower |
| Halstead Effort | **54470.8547 ✅** | 65454.2092 | 74575.0271 | -10983.3545 | Lower | -20104.1723 | Lower | -9120.8179 | Lower |
| SLOC | 61 | 54 | **52 ✅** | +7 | Higher | +9 | Higher | +2 | Higher |
| PLOC | 40 | **28 ✅** | 44 | +12 | Higher | -4 | Lower | -16 | Lower |
| LLOC | 31 | **18 ✅** | 42 | +13 | Higher | -11 | Lower | -24 | Lower |
| Comments (CLOC) | 15 | **20 ✅** | 3 | -5 | Lower | +12 | Higher | +17 | Higher |
| Blank Lines | 12 | **7 ✅** | 8 | +5 | Higher | +4 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.4929 | **37.5063 ✅** | 28.4203 | -0.0134 | Lower | +9.0726 | Higher | +9.0860 | Higher |
| MI (SEI) | -11.3379 | **-11.0131 ✅** | -20.5853 | -0.3248 | Lower | +9.2473 | Higher | +9.5722 | Higher |
| Cyclomatic (sum) | **37 ✅** | 40 | 46 | -3 | Lower | -9 | Lower | -6 | Lower |
| Cognitive (sum) | 42 | **32 ✅** | 50 | +10 | Higher | -8 | Lower | -18 | Lower |
| Halstead Volume | 5544.8055 | **4843.2543 ✅** | 7174.2754 | +701.5512 | Higher | -1629.4699 | Lower | -2331.0210 | Lower |
| Halstead Effort | 342444.3800 | **260790.6183 ✅** | 666771.3372 | +81653.7617 | Higher | -324326.9573 | Lower | -405980.7190 | Lower |
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
| MI (original) | 64.9175 | **68.2950 ✅** | 66.6732 | -3.3775 | Lower | -1.7558 | Lower | +1.6217 | Higher |
| MI (SEI) | 42.7961 | **58.4577 ✅** | 21.8120 | -15.6616 | Lower | +20.9841 | Higher | +36.6457 | Higher |
| Cyclomatic (sum) | 15 | **12 ✅** | 13 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 13 | 12 | **11 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Halstead Volume | 1786.2589 | **1133.1361 ✅** | 1795.6947 | +653.1228 | Higher | -9.4358 | Lower | -662.5586 | Lower |
| Halstead Effort | 86242.8122 | **31491.7415 ✅** | 81704.1107 | +54751.0707 | Higher | +4538.7015 | Higher | -50212.3692 | Lower |
| SLOC | 51 | 50 | **47 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| PLOC | 42 | **34 ✅** | 41 | +8 | Higher | +1 | Higher | -7 | Lower |
| LLOC | 34 | **21 ✅** | 33 | +13 | Higher | +1 | Higher | -12 | Lower |
| Comments (CLOC) | 5 | **12 ✅** | 0 | -7 | Lower | +5 | Higher | +12 | Higher |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02186

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.4680 | **57.9583 ✅** | 37.9102 | -19.4903 | Lower | +0.5578 | Higher | +20.0481 | Higher |
| MI (SEI) | -17.8614 | **39.4714 ✅** | -18.4625 | -57.3328 | Lower | +0.6011 | Higher | +57.9339 | Higher |
| Cyclomatic (sum) | 23 | **13 ✅** | 25 | +10 | Higher | -2 | Lower | -12 | Lower |
| Cognitive (sum) | 12 | **9 ✅** | 14 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | 6620.1347 | **2333.1361 ✅** | 7175.1126 | +4286.9986 | Higher | -554.9779 | Lower | -4841.9765 | Lower |
| Halstead Effort | 558573.8687 | **141932.4479 ✅** | 709606.4790 | +416641.4208 | Higher | -151032.6103 | Lower | -567674.0311 | Lower |
| SLOC | 153 | **74 ✅** | 150 | +79 | Higher | +3 | Higher | -76 | Lower |
| PLOC | 125 | **59 ✅** | 129 | +66 | Higher | -4 | Lower | -70 | Lower |
| LLOC | 71 | **46 ✅** | 77 | +25 | Higher | -6 | Lower | -31 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 28 | **10 ✅** | 21 | +18 | Higher | +7 | Higher | -11 | Lower |
| Functions | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 11 | **2 ✅** | 13 | +9 | Higher | -2 | Lower | -11 | Lower |

## p02190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.6113 | **82.1176 ✅** | 74.4064 | -12.5063 | Lower | -4.7951 | Lower | +7.7112 | Higher |
| MI (SEI) | 51.9778 | **75.5660 ✅** | 62.1204 | -23.5881 | Lower | -10.1426 | Lower | +13.4455 | Higher |
| Cyclomatic (sum) | 9 | **4 ✅** | 6 | +5 | Higher | +3 | Higher | -2 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 2178.3960 | **617.3437 ✅** | 846.4973 | +1561.0523 | Higher | +1331.8987 | Higher | -229.1536 | Lower |
| Halstead Effort | 30697.2299 | **6814.5242 ✅** | 17758.8080 | +23882.7056 | Higher | +12938.4219 | Higher | -10944.2837 | Lower |
| SLOC | 39 | **29 ✅** | 41 | +10 | Higher | -2 | Lower | -12 | Lower |
| PLOC | 26 | **17 ✅** | 25 | +9 | Higher | +1 | Higher | -8 | Lower |
| LLOC | 13 | **11 ✅** | 18 | +2 | Higher | -5 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | 6 | **7 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 8 | **6 ✅** | 9 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02205

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.3592 | 78.9180 | **80.8524 ✅** | -0.5588 | Lower | -2.4932 | Lower | -1.9345 | Lower |
| MI (SEI) | 76.3292 | **84.5093 ✅** | 68.1516 | -8.1801 | Lower | +8.1777 | Higher | +16.3578 | Higher |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 774.8300 | **371.5647 ✅** | 708.4702 | +403.2652 | Higher | +66.3597 | Higher | -336.9055 | Lower |
| Halstead Effort | 13656.3780 | **5152.3642 ✅** | 14523.6394 | +8504.0139 | Higher | -867.2614 | Lower | -9371.2752 | Lower |
| SLOC | 34 | 41 | **30 ✅** | -7 | Lower | +4 | Higher | +11 | Higher |
| PLOC | **19 ✅** | 20 | 21 | -1 | Lower | -2 | Lower | -1 | Lower |
| LLOC | 16 | **11 ✅** | 20 | +5 | Higher | -4 | Lower | -9 | Lower |
| Comments (CLOC) | 11 | **23 ✅** | 4 | -12 | Lower | +7 | Higher | +19 | Higher |
| Blank Lines | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02242

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.5230 ✅** | 49.2523 | 48.8920 | +3.2707 | Higher | +3.6310 | Higher | +0.3603 | Higher |
| MI (SEI) | 2.1103 | **20.1074 ✅** | 5.0959 | -17.9972 | Lower | -2.9857 | Lower | +15.0115 | Higher |
| Cyclomatic (sum) | **20 ✅** | 23 | 25 | -3 | Lower | -5 | Lower | -2 | Lower |
| Cognitive (sum) | **17 ✅** | 28 | 23 | -11 | Lower | -6 | Lower | +5 | Higher |
| Halstead Volume | **2556.3975 ✅** | 3130.1303 | 3070.6976 | -573.7328 | Lower | -514.3001 | Lower | +59.4327 | Higher |
| Halstead Effort | **101820.7702 ✅** | 105371.5684 | 172918.6596 | -3550.7981 | Lower | -71097.8894 | Lower | -67547.0913 | Lower |
| SLOC | **91 ✅** | 100 | 100 | -9 | Lower | -9 | Lower | 0 | Same |
| PLOC | **81 ✅** | 82 | 90 | -1 | Lower | -9 | Lower | -8 | Lower |
| LLOC | 51 | 51 | 67 | 0 | Same | -16 | Lower | -16 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 1 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | 10 | 16 | 10 | -6 | Lower | 0 | Same | +6 | Higher |
| Functions | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |

## p02249

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.8379 | **62.7715 ✅** | 48.9333 | -16.9337 | Lower | -3.0954 | Lower | +13.8382 | Higher |
| MI (SEI) | 3.3839 | **46.4623 ✅** | 8.0559 | -43.0784 | Lower | -4.6720 | Lower | +38.4064 | Higher |
| Cyclomatic (sum) | 24 | **11 ✅** | 22 | +13 | Higher | +2 | Higher | -11 | Lower |
| Cognitive (sum) | 33 | **13 ✅** | 36 | +20 | Higher | -3 | Lower | -23 | Lower |
| Halstead Volume | 4677.1515 | **1376.6255 ✅** | 3589.3312 | +3300.5260 | Higher | +1087.8203 | Higher | -2212.7057 | Lower |
| Halstead Effort | 222622.2460 | **38447.1846 ✅** | 244523.1891 | +184175.0614 | Higher | -21900.9431 | Lower | -206076.0046 | Lower |
| SLOC | 107 | **67 ✅** | 99 | +40 | Higher | +8 | Higher | -32 | Lower |
| PLOC | 89 | **49 ✅** | 82 | +40 | Higher | +7 | Higher | -33 | Lower |
| LLOC | 59 | **14 ✅** | 65 | +45 | Higher | -6 | Lower | -51 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 16 | **8 ✅** | 15 | +8 | Higher | +1 | Higher | -7 | Lower |
| Functions | 4 | 3 | **2 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 3 | **0 ✅** | 0 | Same | +3 | Higher | +3 | Higher |

## p02256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.6418 | **65.4989 ✅** | 63.7236 | -1.8571 | Lower | -0.0818 | Lower | +1.7753 | Higher |
| MI (SEI) | **52.6136 ✅** | 52.4001 | 46.1952 | +0.2135 | Higher | +6.4183 | Higher | +6.2049 | Higher |
| Cyclomatic (sum) | 15 | 14 | **12 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 15 | **8 ✅** | 12 | +7 | Higher | +3 | Higher | -4 | Lower |
| Halstead Volume | 1375.9311 | 1397.1893 | **1264.9509 ✅** | -21.2583 | Lower | +110.9802 | Higher | +132.2385 | Higher |
| Halstead Effort | 45484.3497 | 42206.7611 | **41043.1374 ✅** | +3277.5886 | Higher | +4441.2122 | Higher | +1163.6236 | Higher |
| SLOC | 60 | **54 ✅** | 64 | +6 | Higher | -4 | Lower | -10 | Lower |
| PLOC | 47 | **40 ✅** | 49 | +7 | Higher | -2 | Lower | -9 | Lower |
| LLOC | 26 | **22 ✅** | 30 | +4 | Higher | -4 | Lower | -8 | Lower |
| Comments (CLOC) | **15 ✅** | 11 | 10 | +4 | Higher | +5 | Higher | +1 | Higher |
| Blank Lines | 10 | **8 ✅** | 14 | +2 | Higher | -4 | Lower | -6 | Lower |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 7 | 3 | -6 | Lower | -2 | Lower | +4 | Higher |

## p02266

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.6299 | **66.1945 ✅** | 53.1697 | -12.5646 | Lower | +0.4602 | Higher | +13.0248 | Higher |
| MI (SEI) | 11.1727 | **47.7149 ✅** | 2.6359 | -36.5422 | Lower | +8.5367 | Higher | +45.0790 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 16 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 41 | **8 ✅** | 28 | +33 | Higher | +13 | Higher | -20 | Lower |
| Halstead Volume | 2087.6195 | **1157.1748 ✅** | 2435.4142 | +930.4448 | Higher | -347.7947 | Lower | -1278.2395 | Lower |
| Halstead Effort | 69480.2607 | **25383.1885 ✅** | 94135.5247 | +44097.0722 | Higher | -24655.2641 | Lower | -68752.3362 | Lower |
| SLOC | 96 | **59 ✅** | 94 | +37 | Higher | +2 | Higher | -35 | Lower |
| PLOC | 87 | **45 ✅** | 87 | +42 | Higher | 0 | Same | -42 | Lower |
| LLOC | 53 | **28 ✅** | 58 | +25 | Higher | -5 | Lower | -30 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 0 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 9 | 8 | **7 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02276

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.2122 | 60.0870 | **83.3161 ✅** | +0.1252 | Higher | -23.1039 | Lower | -23.2291 | Lower |
| MI (SEI) | 33.9473 | 45.3751 | **88.5805 ✅** | -11.4278 | Lower | -54.6332 | Lower | -43.2054 | Lower |
| Cyclomatic (sum) | **13 ✅** | 15 | 16 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **10 ✅** | 17 | 13 | -7 | Lower | -3 | Lower | +4 | Higher |
| Halstead Volume | 2761.1345 | **1686.2630 ✅** | 2787.5497 | +1074.8715 | Higher | -26.4152 | Lower | -1101.2867 | Lower |
| Halstead Effort | 65062.3688 | 65080.6364 | **64529.3294 ✅** | -18.2675 | Lower | +533.0394 | Higher | +551.3070 | Higher |
| SLOC | 61 | 70 | **14 ✅** | -9 | Lower | +47 | Higher | +56 | Higher |
| PLOC | **45 ✅** | 49 | 46 | -4 | Lower | -1 | Lower | +3 | Higher |
| LLOC | **25 ✅** | 34 | 26 | -9 | Lower | -1 | Lower | +8 | Higher |
| Comments (CLOC) | 5 | **15 ✅** | 6 | -10 | Lower | -1 | Lower | +9 | Higher |
| Blank Lines | 11 | 11 | **-32 ✅** | 0 | Same | +43 | Higher | +43 | Higher |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p02277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.0431 | 57.4191 | **58.3747 ✅** | -0.3760 | Lower | -1.3316 | Lower | -0.9556 | Lower |
| MI (SEI) | 28.9508 | **38.5535 ✅** | 10.0433 | -9.6027 | Lower | +18.9075 | Higher | +28.5101 | Higher |
| Cyclomatic (sum) | 15 | 15 | 15 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 17 | **10 ✅** | 17 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 2161.3733 | 2180.0865 | **2146.9269 ✅** | -18.7133 | Lower | +14.4463 | Higher | +33.1596 | Higher |
| Halstead Effort | 89813.8213 | **65936.4950 ✅** | 86341.2771 | +23877.3263 | Higher | +3472.5442 | Higher | -20404.7821 | Lower |
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
| MI (original) | **55.0152 ✅** | 49.9251 | 53.9846 | +5.0901 | Higher | +1.0306 | Higher | -4.0595 | Lower |
| MI (SEI) | **23.9699 ✅** | 21.6718 | 12.1738 | +2.2981 | Higher | +11.7961 | Higher | +9.4980 | Higher |
| Cyclomatic (sum) | **15 ✅** | 22 | 16 | -7 | Lower | -1 | Lower | +6 | Higher |
| Cognitive (sum) | **14 ✅** | 34 | 19 | -20 | Lower | -5 | Lower | +15 | Higher |
| Halstead Volume | 2838.1655 | **2544.0179 ✅** | 2848.9326 | +294.1476 | Higher | -10.7671 | Lower | -304.9147 | Lower |
| Halstead Effort | 104302.5821 | **96725.6803 ✅** | 120432.1512 | +7576.9018 | Higher | -16129.5691 | Lower | -23706.4709 | Lower |
| SLOC | **81 ✅** | 104 | 85 | -23 | Lower | -4 | Lower | +19 | Higher |
| PLOC | **63 ✅** | 89 | 73 | -26 | Lower | -10 | Lower | +16 | Higher |
| LLOC | 42 | **30 ✅** | 54 | +12 | Higher | -12 | Lower | -24 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 1 | -5 | Lower | +4 | Higher | +9 | Higher |
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
| MI (SEI) | 52.5789 | **60.1013 ✅** | 51.4286 | -7.5224 | Lower | +1.1503 | Higher | +8.6727 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 8 | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 1236.9954 | 1241.8920 | **1224.0067 ✅** | -4.8966 | Lower | +12.9887 | Higher | +17.8852 | Higher |
| Halstead Effort | 42185.8093 | **32254.6942 ✅** | 41888.2309 | +9931.1150 | Higher | +297.5784 | Higher | -9633.5366 | Lower |
| SLOC | 43 | 52 | **38 ✅** | -9 | Lower | +5 | Higher | +14 | Higher |
| PLOC | 34 | 32 | **31 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| LLOC | 18 | **15 ✅** | 22 | +3 | Higher | -4 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 3 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 5 | 6 | **4 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8847 | **79.6785 ✅** | 67.2269 | -19.7938 | Lower | -7.3422 | Lower | +12.4516 | Higher |
| MI (SEI) | 28.7583 | **78.8544 ✅** | 33.1723 | -50.0961 | Lower | -4.4139 | Lower | +45.6821 | Higher |
| Cyclomatic (sum) | 12 | **6 ✅** | 10 | +6 | Higher | +2 | Higher | -4 | Lower |
| Cognitive (sum) | 11 | **4 ✅** | 11 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 3073.6059 | **603.9396 ✅** | 1520.2100 | +2469.6663 | Higher | +1553.3959 | Higher | -916.2704 | Lower |
| Halstead Effort | 101337.6979 | **12078.7910 ✅** | 82091.3395 | +89258.9069 | Higher | +19246.3583 | Higher | -70012.5485 | Lower |
| SLOC | 61 | **33 ✅** | 50 | +28 | Higher | +11 | Higher | -17 | Lower |
| PLOC | 50 | **18 ✅** | 42 | +32 | Higher | +8 | Higher | -24 | Lower |
| LLOC | 34 | **11 ✅** | 35 | +23 | Higher | -1 | Lower | -24 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 1 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 9 | **5 ✅** | 8 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p02328

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.7401 | **68.5010 ✅** | 62.9715 | -7.7609 | Lower | -2.2314 | Lower | +5.5295 | Higher |
| MI (SEI) | 31.4889 | **62.5246 ✅** | 16.2678 | -31.0356 | Lower | +15.2212 | Higher | +46.2568 | Higher |
| Cyclomatic (sum) | **8 ✅** | 9 | 11 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | **10 ✅** | 14 | 14 | -4 | Lower | -4 | Lower | 0 | Same |
| Halstead Volume | 2553.3678 | **873.7173 ✅** | 1868.1823 | +1679.6505 | Higher | +685.1855 | Higher | -994.4649 | Lower |
| Halstead Effort | 53301.5526 | **45433.3009 ✅** | 93963.7297 | +7868.2517 | Higher | -40662.1771 | Lower | -48530.4288 | Lower |
| SLOC | 65 | **56 ✅** | 60 | +9 | Higher | +5 | Higher | -4 | Lower |
| PLOC | 50 | **32 ✅** | 52 | +18 | Higher | -2 | Lower | -20 | Lower |
| LLOC | 29 | **19 ✅** | 40 | +10 | Higher | -11 | Lower | -21 | Lower |
| Comments (CLOC) | 4 | **18 ✅** | 0 | -14 | Lower | +4 | Higher | +18 | Higher |
| Blank Lines | 12 | **6 ✅** | 8 | +6 | Higher | +4 | Higher | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02333

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.9479 | 54.9400 | **60.6555 ✅** | +1.0079 | Higher | -4.7077 | Lower | -5.7156 | Lower |
| MI (SEI) | 6.9494 | **34.4937 ✅** | 13.3339 | -27.5443 | Lower | -6.3845 | Lower | +21.1599 | Higher |
| Cyclomatic (sum) | 19 | 15 | 15 | +4 | Higher | +4 | Higher | 0 | Same |
| Cognitive (sum) | 13 | **10 ✅** | 11 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | 2526 | 2478 | **2443.4638 ✅** | +48 | Higher | +82.5362 | Higher | +34.5362 | Higher |
| Halstead Effort | 179420.2941 | **153990 ✅** | 169400.7626 | +25430.2941 | Higher | +10019.5315 | Higher | -15410.7626 | Lower |
| SLOC | 75 | 85 | **60 ✅** | -10 | Lower | +15 | Higher | +25 | Higher |
| PLOC | 61 | 56 | **51 ✅** | +5 | Higher | +10 | Higher | +5 | Higher |
| LLOC | 39 | 39 | **38 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 0 | **14 ✅** | 0 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 14 | 16 | **9 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |

## p02340

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.3715 | **67.3699 ✅** | 60.7736 | -5.9983 | Lower | +0.5979 | Higher | +6.5963 | Higher |
| MI (SEI) | 24.4233 | **56.0189 ✅** | 23.0961 | -31.5956 | Lower | +1.3272 | Higher | +32.9228 | Higher |
| Cyclomatic (sum) | 17 | **9 ✅** | 10 | +8 | Higher | +7 | Higher | -1 | Lower |
| Cognitive (sum) | 11 | 6 | 6 | +5 | Higher | +5 | Higher | 0 | Same |
| Halstead Volume | 1851.0739 | **1289.2432 ✅** | 3311.7582 | +561.8307 | Higher | -1460.6843 | Lower | -2022.5150 | Lower |
| Halstead Effort | 74494.4368 | **36098.8101 ✅** | 76734.1422 | +38395.6267 | Higher | -2239.7055 | Lower | -40635.3322 | Lower |
| SLOC | 61 | **53 ✅** | 58 | +8 | Higher | +3 | Higher | -5 | Lower |
| PLOC | 50 | **31 ✅** | 44 | +19 | Higher | +6 | Higher | -13 | Lower |
| LLOC | 28 | **18 ✅** | 21 | +10 | Higher | +7 | Higher | -3 | Lower |
| Comments (CLOC) | 1 | **12 ✅** | 1 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | **10 ✅** | 12 | 13 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 6 | 2 | 2 | +4 | Higher | +4 | Higher | 0 | Same |

## p02347

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.3081 | **56.0522 ✅** | 38.7696 | -15.7441 | Lower | +1.5385 | Higher | +17.2827 | Higher |
| MI (SEI) | -8.0446 | **32.8497 ✅** | -17.2227 | -40.8944 | Lower | +9.1781 | Higher | +50.0724 | Higher |
| Cyclomatic (sum) | 29 | **19 ✅** | 25 | +10 | Higher | +4 | Higher | -6 | Lower |
| Cognitive (sum) | 33 | **11 ✅** | 33 | +22 | Higher | 0 | Same | -22 | Lower |
| Halstead Volume | 4805.8171 | **1874.9386 ✅** | 6341.8257 | +2930.8786 | Higher | -1536.0086 | Lower | -4466.8871 | Lower |
| Halstead Effort | 331938.6319 | **56139.1486 ✅** | 290478.2662 | +275799.4833 | Higher | +41460.3657 | Higher | -234339.1176 | Lower |
| SLOC | 139 | **82 ✅** | 148 | +57 | Higher | -9 | Lower | -66 | Lower |
| PLOC | 117 | **65 ✅** | 130 | +52 | Higher | -13 | Lower | -65 | Lower |
| LLOC | 55 | **23 ✅** | 81 | +32 | Higher | -26 | Lower | -58 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 0 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 22 | **13 ✅** | 18 | +9 | Higher | +4 | Higher | -5 | Lower |
| Functions | **4 ✅** | 7 | 5 | -3 | Lower | -1 | Lower | +2 | Higher |
| Closures | 4 | 1 | **0 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Exits | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.3901 | **45.0138 ✅** | 40.1693 | -1.6236 | Lower | +3.2209 | Higher | +4.8445 | Higher |
| MI (SEI) | 1.0785 | **12.7675 ✅** | -8.7663 | -11.6889 | Lower | +9.8448 | Higher | +21.5338 | Higher |
| Cyclomatic (sum) | **23 ✅** | 24 | 25 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **13 ✅** | 20 | 14 | -7 | Lower | -1 | Lower | +6 | Higher |
| Halstead Volume | 4704.0166 | **4498.5078 ✅** | 5276.9679 | +205.5088 | Higher | -572.9513 | Lower | -778.4601 | Lower |
| Halstead Effort | **285526.3419 ✅** | 398217.9080 | 357888.6866 | -112691.5661 | Lower | -72362.3447 | Lower | +40329.2214 | Higher |
| SLOC | 126 | **114 ✅** | 144 | +12 | Higher | -18 | Lower | -30 | Lower |
| PLOC | 109 | **88 ✅** | 129 | +21 | Higher | -20 | Lower | -41 | Lower |
| LLOC | **64 ✅** | 65 | 71 | -1 | Lower | -7 | Lower | -6 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 1 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 17 | 17 | **15 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **7 ✅** | 11 | +2 | Higher | -2 | Lower | -4 | Lower |

## p02355

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.3297 | **71.1391 ✅** | 54.0155 | -14.8094 | Lower | +2.3142 | Higher | +17.1236 | Higher |
| MI (SEI) | 28.7874 | **59.6944 ✅** | 18.7333 | -30.9070 | Lower | +10.0540 | Higher | +40.9611 | Higher |
| Cyclomatic (sum) | 21 | **11 ✅** | 22 | +10 | Higher | -1 | Lower | -11 | Lower |
| Cognitive (sum) | 27 | **19 ✅** | 28 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 2061.6110 | **888.7342 ✅** | 2094.2209 | +1172.8768 | Higher | -32.6099 | Lower | -1205.4867 | Lower |
| Halstead Effort | 136066.3258 | **42565.6914 ✅** | 152733.6985 | +93500.6344 | Higher | -16667.3727 | Lower | -110168.0071 | Lower |
| SLOC | 76 | **46 ✅** | 86 | +30 | Higher | -10 | Lower | -40 | Lower |
| PLOC | 60 | **34 ✅** | 68 | +26 | Higher | -8 | Lower | -34 | Lower |
| LLOC | 55 | **26 ✅** | 61 | +29 | Higher | -6 | Lower | -35 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 3 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | 12 | **6 ✅** | 16 | +6 | Higher | -4 | Lower | -10 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p02369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.0509 | **60.9342 ✅** | 42.5542 | -13.8834 | Lower | +4.4967 | Higher | +18.3801 | Higher |
| MI (SEI) | 8.9949 | **48.2051 ✅** | 0.2484 | -39.2102 | Lower | +8.7465 | Higher | +47.9566 | Higher |
| Cyclomatic (sum) | 16 | **14 ✅** | 18 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 17 | **13 ✅** | 22 | +4 | Higher | -5 | Lower | -9 | Lower |
| Halstead Volume | 3170.9940 | **1497.5203 ✅** | 4449.4184 | +1673.4737 | Higher | -1278.4245 | Lower | -2951.8982 | Lower |
| Halstead Effort | 127710.2285 | **44442.5376 ✅** | 244082.3832 | +83267.6909 | Higher | -116372.1547 | Lower | -199639.8456 | Lower |
| SLOC | 126 | **70 ✅** | 145 | +56 | Higher | -19 | Lower | -75 | Lower |
| PLOC | 103 | **51 ✅** | 126 | +52 | Higher | -23 | Lower | -75 | Lower |
| LLOC | 57 | **27 ✅** | 74 | +30 | Higher | -17 | Lower | -47 | Lower |
| Comments (CLOC) | 5 | **17 ✅** | 4 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 21 | **11 ✅** | 18 | +10 | Higher | +3 | Higher | -7 | Lower |
| Functions | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 6 | 3 | -4 | Lower | -1 | Lower | +3 | Higher |

## p02370

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.1392 | **62.8834 ✅** | 56.5435 | -11.7442 | Lower | -5.4043 | Lower | +6.3399 | Higher |
| MI (SEI) | 19.4920 | **46.1922 ✅** | 22.0757 | -26.7002 | Lower | -2.5837 | Lower | +24.1165 | Higher |
| Cyclomatic (sum) | 16 | **10 ✅** | 14 | +6 | Higher | +2 | Higher | -4 | Lower |
| Cognitive (sum) | **14 ✅** | 15 | 18 | -1 | Lower | -4 | Lower | -3 | Lower |
| Halstead Volume | 4579.8995 | **1705.9665 ✅** | 2298.3198 | +2873.9330 | Higher | +2281.5797 | Higher | -592.3533 | Lower |
| Halstead Effort | 111852.7565 | **46660.4887 ✅** | 113363.0700 | +65192.2678 | Higher | -1510.3135 | Lower | -66702.5813 | Lower |
| SLOC | 87 | **63 ✅** | 80 | +24 | Higher | +7 | Higher | -17 | Lower |
| PLOC | 68 | **42 ✅** | 65 | +26 | Higher | +3 | Higher | -23 | Lower |
| LLOC | 51 | **32 ✅** | 56 | +19 | Higher | -5 | Lower | -24 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 3 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 13 | **11 ✅** | 13 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02371

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.3750 | **63.4930 ✅** | 42.6038 | -21.1180 | Lower | -0.2287 | Lower | +20.8893 | Higher |
| MI (SEI) | -2.9158 | **55.4968 ✅** | -12.2002 | -58.4126 | Lower | +9.2844 | Higher | +67.6970 | Higher |
| Cyclomatic (sum) | 21 | **8 ✅** | 20 | +13 | Higher | +1 | Higher | -12 | Lower |
| Cognitive (sum) | 24 | **10 ✅** | 22 | +14 | Higher | +2 | Higher | -12 | Lower |
| Halstead Volume | 5533.7032 | **1368.3158 ✅** | 5405.4459 | +4165.3873 | Higher | +128.2572 | Higher | -4037.1301 | Lower |
| Halstead Effort | 540581.3127 | **40973.4568 ✅** | 531057.4817 | +499607.8559 | Higher | +9523.8310 | Higher | -490084.0248 | Lower |
| SLOC | 131 | **67 ✅** | 132 | +64 | Higher | -1 | Lower | -65 | Lower |
| PLOC | 117 | **35 ✅** | 112 | +82 | Higher | +5 | Higher | -77 | Lower |
| LLOC | 103 | **18 ✅** | 114 | +85 | Higher | -11 | Lower | -96 | Lower |
| Comments (CLOC) | 2 | **22 ✅** | 0 | -20 | Lower | +2 | Higher | +22 | Higher |
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
| MI (original) | 80.5811 | **89.9215 ✅** | 72.8988 | -9.3404 | Lower | +7.6823 | Higher | +17.0227 | Higher |
| MI (SEI) | 74.8382 | **88.0965 ✅** | 61.1784 | -13.2582 | Lower | +13.6599 | Higher | +26.9181 | Higher |
| Cyclomatic (sum) | 7 | **4 ✅** | 5 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 4 | 2 | 2 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 590.1941 | **325.4847 ✅** | 1627.9279 | +264.7094 | Higher | -1037.7338 | Lower | -1302.4432 | Lower |
| Halstead Effort | 9836.5685 | **3173.4761 ✅** | 18379.8310 | +6663.0924 | Higher | -8543.2625 | Lower | -15206.3549 | Lower |
| SLOC | 31 | **22 ✅** | 37 | +9 | Higher | -6 | Lower | -15 | Lower |
| PLOC | 18 | **14 ✅** | 23 | +4 | Higher | -5 | Lower | -9 | Lower |
| LLOC | 12 | **6 ✅** | 12 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 7 | 5 | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Blank Lines | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02430

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.3782 | 63.9053 | **65.1337 ✅** | -2.5271 | Lower | -3.7555 | Lower | -1.2284 | Lower |
| MI (SEI) | 32.8261 | **48.5038 ✅** | 43.9712 | -15.6777 | Lower | -11.1451 | Lower | +4.5326 | Higher |
| Cyclomatic (sum) | 16 | 12 | 12 | +4 | Higher | +4 | Higher | 0 | Same |
| Cognitive (sum) | **6 ✅** | 10 | 9 | -4 | Lower | -3 | Lower | +1 | Higher |
| Halstead Volume | 3375.3986 | **1493.5472 ✅** | 1546.5070 | +1881.8514 | Higher | +1828.8917 | Higher | -52.9598 | Lower |
| Halstead Effort | 74802.4583 | 58888.4332 | **51518.0139 ✅** | +15914.0252 | Higher | +23284.4444 | Higher | +7370.4193 | Higher |
| SLOC | **51 ✅** | 60 | 55 | -9 | Lower | -4 | Lower | +5 | Higher |
| PLOC | **40 ✅** | 41 | 46 | -1 | Lower | -6 | Lower | -5 | Lower |
| LLOC | **19 ✅** | 34 | 25 | -15 | Lower | -6 | Lower | +9 | Higher |
| Comments (CLOC) | 3 | **11 ✅** | 6 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 10 | 10 | **8 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **1 ✅** | 5 | +6 | Higher | +2 | Higher | -4 | Lower |

## p02433

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.8469 | **44.1490 ✅** | 31.6643 | -9.3021 | Lower | +3.1826 | Higher | +12.4847 | Higher |
| MI (SEI) | -12.7914 | **6.1907 ✅** | -17.2728 | -18.9822 | Lower | +4.4814 | Higher | +23.4635 | Higher |
| Cyclomatic (sum) | 41 | **29 ✅** | 46 | +12 | Higher | -5 | Lower | -17 | Lower |
| Cognitive (sum) | 46 | **31 ✅** | 44 | +15 | Higher | +2 | Higher | -13 | Lower |
| Halstead Volume | 4647.2148 | **3196.0611 ✅** | 5070.3229 | +1451.1536 | Higher | -423.1081 | Lower | -1874.2618 | Lower |
| Halstead Effort | 255352.2219 | **132029.2855 ✅** | 305121.7191 | +123322.9364 | Higher | -49769.4972 | Lower | -173092.4336 | Lower |
| SLOC | 166 | **125 ✅** | 183 | +41 | Higher | -17 | Lower | -58 | Lower |
| PLOC | 140 | **110 ✅** | 154 | +30 | Higher | -14 | Lower | -44 | Lower |
| LLOC | 85 | **58 ✅** | 97 | +27 | Higher | -12 | Lower | -39 | Lower |
| Comments (CLOC) | 2 | **5 ✅** | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 24 | **11 ✅** | 27 | +13 | Higher | -3 | Lower | -16 | Lower |
| Functions | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 12 | **2 ✅** | 13 | +10 | Higher | -1 | Lower | -11 | Lower |

## p02444

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.6852 | 67.8715 | **67.9620 ✅** | -1.1863 | Lower | -1.2768 | Lower | -0.0905 | Lower |
| MI (SEI) | 54.6746 | **55.8207 ✅** | 55.1995 | -1.1461 | Lower | -0.5249 | Lower | +0.6212 | Higher |
| Cyclomatic (sum) | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 7 | **5 ✅** | 9 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | 1450.2317 | **1242.2611 ✅** | 1379.4857 | +207.9705 | Higher | +70.7459 | Higher | -137.2246 | Lower |
| Halstead Effort | 39273.2087 | **37626.1791 ✅** | 42917.3341 | +1647.0297 | Higher | -3644.1253 | Lower | -5291.1550 | Lower |
| SLOC | 54 | 52 | **50 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| PLOC | 36 | **31 ✅** | 34 | +5 | Higher | +2 | Higher | -3 | Lower |
| LLOC | 23 | **19 ✅** | 25 | +4 | Higher | -2 | Lower | -6 | Lower |
| Comments (CLOC) | **12 ✅** | 11 | 10 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02449

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.6506 ✅** | 52.8685 | 50.8483 | +2.7821 | Higher | +4.8023 | Higher | +2.0202 | Higher |
| MI (SEI) | 25.3278 | **33.9617 ✅** | 22.3109 | -8.6339 | Lower | +3.0169 | Higher | +11.6508 | Higher |
| Cyclomatic (sum) | **16 ✅** | 17 | 21 | -1 | Lower | -5 | Lower | -4 | Lower |
| Cognitive (sum) | 14 | **13 ✅** | 51 | +1 | Higher | -37 | Lower | -38 | Lower |
| Halstead Volume | 2702.8335 | **2238.7778 ✅** | 2595.8877 | +464.0557 | Higher | +106.9459 | Higher | -357.1098 | Lower |
| Halstead Effort | 155759.4435 | **117074.9103 ✅** | 197979.6981 | +38684.5331 | Higher | -42220.2547 | Lower | -80904.7878 | Lower |
| SLOC | **78 ✅** | 97 | 99 | -19 | Lower | -21 | Lower | -2 | Lower |
| PLOC | **57 ✅** | 59 | 86 | -2 | Lower | -29 | Lower | -27 | Lower |
| LLOC | **42 ✅** | 44 | 77 | -2 | Lower | -35 | Lower | -33 | Lower |
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
| MI (original) | 42.2085 | **63.0359 ✅** | 42.1634 | -20.8274 | Lower | +0.0451 | Higher | +20.8724 | Higher |
| MI (SEI) | -11.6504 | **46.2225 ✅** | -4.3331 | -57.8729 | Lower | -7.3173 | Lower | +50.5557 | Higher |
| Cyclomatic (sum) | 31 | **14 ✅** | 34 | +17 | Higher | -3 | Lower | -20 | Lower |
| Cognitive (sum) | 32 | **8 ✅** | 37 | +24 | Higher | -5 | Lower | -29 | Lower |
| Halstead Volume | 3946.1667 | **1200.7669 ✅** | 4374.7742 | +2745.3998 | Higher | -428.6075 | Lower | -3174.0073 | Lower |
| Halstead Effort | 222273.8822 | **29590.3281 ✅** | 353239.7499 | +192683.5540 | Higher | -130965.8678 | Lower | -323649.4218 | Lower |
| SLOC | 128 | **66 ✅** | 119 | +62 | Higher | +9 | Higher | -53 | Lower |
| PLOC | 115 | **47 ✅** | 102 | +68 | Higher | +13 | Higher | -55 | Lower |
| LLOC | 58 | **17 ✅** | 59 | +41 | Higher | -1 | Lower | -42 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 1 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 13 | **8 ✅** | 17 | +5 | Higher | -4 | Lower | -9 | Lower |
| Functions | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 9 | **4 ✅** | 14 | +5 | Higher | -5 | Lower | -10 | Lower |

## p02542

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 17.1659 | **57.2415 ✅** | 10.5734 | -40.0757 | Lower | +6.5925 | Higher | +46.6682 | Higher |
| MI (SEI) | -40.7071 | **45.1611 ✅** | -54.2357 | -85.8682 | Lower | +13.5286 | Higher | +99.3968 | Higher |
| Cyclomatic (sum) | 56 | **17 ✅** | 61 | +39 | Higher | -5 | Lower | -44 | Lower |
| Cognitive (sum) | 112 | **19 ✅** | 135 | +93 | Higher | -23 | Lower | -116 | Lower |
| Halstead Volume | 12348.5418 | **2151.8208 ✅** | 18161.8596 | +10196.7211 | Higher | -5813.3178 | Lower | -16010.0389 | Lower |
| Halstead Effort | 2064806.1784 | **121362.6916 ✅** | 4549327.0225 | +1943443.4868 | Higher | -2484520.8441 | Lower | -4427964.3309 | Lower |
| SLOC | 292 | **75 ✅** | 361 | +217 | Higher | -69 | Lower | -286 | Lower |
| PLOC | 264 | **46 ✅** | 337 | +218 | Higher | -73 | Lower | -291 | Lower |
| LLOC | 201 | **28 ✅** | 353 | +173 | Higher | -152 | Lower | -325 | Lower |
| Comments (CLOC) | 1 | **21 ✅** | 0 | -20 | Lower | +1 | Higher | +21 | Higher |
| Blank Lines | 28 | **13 ✅** | 24 | +15 | Higher | +4 | Higher | -11 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **3 ✅** | 5 | +3 | Higher | +1 | Higher | -2 | Lower |

## p02562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 13.2744 | **44.5393 ✅** | 10.8980 | -31.2649 | Lower | +2.3764 | Higher | +33.6413 | Higher |
| MI (SEI) | -45.7834 | **17.7092 ✅** | -53.6655 | -63.4926 | Lower | +7.8821 | Higher | +71.3747 | Higher |
| Cyclomatic (sum) | 63 | **26 ✅** | 62 | +37 | Higher | +1 | Higher | -36 | Lower |
| Cognitive (sum) | 113 | **24 ✅** | 119 | +89 | Higher | -6 | Lower | -95 | Lower |
| Halstead Volume | 14972.3847 | **3560.1507 ✅** | 16754.4072 | +11412.2340 | Higher | -1782.0225 | Lower | -13194.2565 | Lower |
| Halstead Effort | 2552455.1258 | **227211.5029 ✅** | 3683665.8453 | +2325243.6229 | Higher | -1131210.7195 | Lower | -3456454.3424 | Lower |
| SLOC | 316 | **123 ✅** | 358 | +193 | Higher | -42 | Lower | -235 | Lower |
| PLOC | 278 | **94 ✅** | 331 | +184 | Higher | -53 | Lower | -237 | Lower |
| LLOC | 281 | **52 ✅** | 339 | +229 | Higher | -58 | Lower | -287 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 0 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 38 | **17 ✅** | 27 | +21 | Higher | +11 | Higher | -10 | Lower |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02584

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.2333 | **81.6455 ✅** | 78.5172 | -12.4122 | Lower | -9.2839 | Lower | +3.1283 | Higher |
| MI (SEI) | 48.6249 | **82.8040 ✅** | 66.3396 | -34.1791 | Lower | -17.7147 | Lower | +16.4644 | Higher |
| Cyclomatic (sum) | 9 | 5 | **4 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 2164.9728 | **432.4343 ✅** | 824.8989 | +1732.5384 | Higher | +1340.0739 | Higher | -392.4645 | Lower |
| Halstead Effort | 43570.0772 | **7135.1666 ✅** | 19797.5725 | +36434.9105 | Higher | +23772.5046 | Higher | -12662.4059 | Lower |
| SLOC | 40 | 33 | 33 | +7 | Higher | +7 | Higher | 0 | Same |
| PLOC | 31 | **18 ✅** | 22 | +13 | Higher | +9 | Higher | -4 | Lower |
| LLOC | 21 | **12 ✅** | 19 | +9 | Higher | +2 | Higher | -7 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 5 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02585

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.3854 | **72.9409 ✅** | 56.1613 | -16.5555 | Lower | +0.2242 | Higher | +16.7796 | Higher |
| MI (SEI) | 19.6072 | **66.6423 ✅** | 6.8500 | -47.0351 | Lower | +12.7572 | Higher | +59.7923 | Higher |
| Cyclomatic (sum) | 15 | **8 ✅** | 15 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 24 | **8 ✅** | 24 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 2659.5234 | **1109.2048 ✅** | 2776.6830 | +1550.3186 | Higher | -117.1596 | Lower | -1667.4782 | Lower |
| Halstead Effort | 219676.6305 | **33544.4997 ✅** | 251289.8107 | +186132.1308 | Higher | -31613.1802 | Lower | -217745.3110 | Lower |
| SLOC | 76 | **40 ✅** | 76 | +36 | Higher | 0 | Same | -36 | Lower |
| PLOC | 60 | **28 ✅** | 62 | +32 | Higher | -2 | Lower | -34 | Lower |
| LLOC | 62 | **21 ✅** | 64 | +41 | Higher | -2 | Lower | -43 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 0 | -9 | Lower | +2 | Higher | +11 | Higher |
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
| MI (original) | 76.1576 | **77.0684 ✅** | 74.5482 | -0.9108 | Lower | +1.6094 | Higher | +2.5201 | Higher |
| MI (SEI) | 60.1180 | **79.2664 ✅** | 56.5468 | -19.1484 | Lower | +3.5712 | Higher | +22.7196 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 1083.0835 | **470.6486 ✅** | 1043.7263 | +612.4349 | Higher | +39.3573 | Higher | -573.0776 | Lower |
| Halstead Effort | 32091.3643 | **8858.2799 ✅** | 32564.2594 | +23233.0844 | Higher | -472.8951 | Lower | -23705.9795 | Lower |
| SLOC | **34 ✅** | 42 | 38 | -8 | Lower | -4 | Lower | +4 | Higher |
| PLOC | 22 | **19 ✅** | 26 | +3 | Higher | -4 | Lower | -7 | Lower |
| LLOC | 18 | **9 ✅** | 18 | +9 | Higher | 0 | Same | -9 | Lower |
| Comments (CLOC) | 4 | **19 ✅** | 4 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02618

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.5959 | **62.9957 ✅** | 50.6598 | -12.3998 | Lower | -0.0639 | Lower | +12.3359 | Higher |
| MI (SEI) | 21.8549 | **56.2034 ✅** | 7.3181 | -34.3486 | Lower | +14.5368 | Higher | +48.8853 | Higher |
| Cyclomatic (sum) | 18 | **8 ✅** | 19 | +10 | Higher | -1 | Lower | -11 | Lower |
| Cognitive (sum) | 15 | **6 ✅** | 16 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 3315.9421 | **1577.8604 ✅** | 3573.0031 | +1738.0817 | Higher | -257.0610 | Lower | -1995.1427 | Lower |
| Halstead Effort | 198840.8543 | **32959.7499 ✅** | 220890.9902 | +165881.1044 | Higher | -22050.1359 | Lower | -187931.2403 | Lower |
| SLOC | 97 | **66 ✅** | 93 | +31 | Higher | +4 | Higher | -27 | Lower |
| PLOC | 70 | **38 ✅** | 74 | +32 | Higher | -4 | Lower | -36 | Lower |
| LLOC | 52 | **20 ✅** | 58 | +32 | Higher | -6 | Lower | -38 | Lower |
| Comments (CLOC) | 9 | **24 ✅** | 1 | -15 | Lower | +8 | Higher | +23 | Higher |
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
| MI (original) | **56.9889 ✅** | 53.6480 | 55.9548 | +3.3409 | Higher | +1.0341 | Higher | -2.3068 | Lower |
| MI (SEI) | 21.0898 | **37.6554 ✅** | 6.7558 | -16.5657 | Lower | +14.3340 | Higher | +30.8996 | Higher |
| Cyclomatic (sum) | 16 | 16 | 17 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 28 | 28 | **21 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Halstead Volume | 2927.2796 | **2728.1326 ✅** | 3416.7832 | +199.1470 | Higher | -489.5035 | Lower | -688.6505 | Lower |
| Halstead Effort | **146772.4391 ✅** | 171549.2867 | 215063.2035 | -24776.8476 | Lower | -68290.7644 | Lower | -43513.9168 | Lower |
| SLOC | 70 | 88 | 70 | -18 | Lower | 0 | Same | +18 | Higher |
| PLOC | 59 | 62 | **57 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| LLOC | 43 | **31 ✅** | 53 | +12 | Higher | -10 | Lower | -22 | Lower |
| Comments (CLOC) | 2 | **21 ✅** | 0 | -19 | Lower | +2 | Higher | +21 | Higher |
| Blank Lines | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 0 | 5 | 0 | -5 | Lower | 0 | Same | +5 | Higher |

## p02647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.4759 | **78.1252 ✅** | 66.0862 | -14.6493 | Lower | -2.6103 | Lower | +12.0390 | Higher |
| MI (SEI) | 52.8634 | **82.0775 ✅** | 20.7614 | -29.2142 | Lower | +32.1019 | Higher | +61.3161 | Higher |
| Cyclomatic (sum) | 11 | **6 ✅** | 11 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 18 | **11 ✅** | 20 | +7 | Higher | -2 | Lower | -9 | Lower |
| Halstead Volume | 1695.4645 | **524.6170 ✅** | 1602.8552 | +1170.8475 | Higher | +92.6093 | Higher | -1078.2382 | Lower |
| Halstead Effort | 75769.7218 | **16545.6127 ✅** | 74414.0348 | +59224.1091 | Higher | +1355.6870 | Higher | -57868.4221 | Lower |
| SLOC | 60 | **38 ✅** | 52 | +22 | Higher | +8 | Higher | -14 | Lower |
| PLOC | 48 | **13 ✅** | 47 | +35 | Higher | +1 | Higher | -34 | Lower |
| LLOC | 42 | **10 ✅** | 43 | +32 | Higher | -1 | Lower | -33 | Lower |
| Comments (CLOC) | 16 | **19 ✅** | 0 | -3 | Lower | +16 | Higher | +19 | Higher |
| Blank Lines | 9 | 6 | **5 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.4662 | **72.8342 ✅** | 42.0192 | -23.3680 | Lower | +7.4469 | Higher | +30.8150 | Higher |
| MI (SEI) | 5.9015 | **67.8149 ✅** | -1.6760 | -61.9135 | Lower | +7.5775 | Higher | +69.4909 | Higher |
| Cyclomatic (sum) | 22 | **8 ✅** | 22 | +14 | Higher | 0 | Same | -14 | Lower |
| Cognitive (sum) | 19 | **9 ✅** | 22 | +10 | Higher | -3 | Lower | -13 | Lower |
| Halstead Volume | 3936.4122 | **903.8038 ✅** | 4409.9279 | +3032.6084 | Higher | -473.5157 | Lower | -3506.1241 | Lower |
| Halstead Effort | 373527.1091 | **38597.7379 ✅** | 273131.0172 | +334929.3712 | Higher | +100396.0919 | Higher | -234533.2794 | Lower |
| SLOC | 93 | **43 ✅** | 142 | +50 | Higher | -49 | Lower | -99 | Lower |
| PLOC | 77 | **25 ✅** | 115 | +52 | Higher | -38 | Lower | -90 | Lower |
| LLOC | 59 | **18 ✅** | 54 | +41 | Higher | +5 | Higher | -36 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 3 | -12 | Lower | -2 | Lower | +10 | Higher |
| Blank Lines | 15 | **7 ✅** | 26 | +8 | Higher | -11 | Lower | -19 | Lower |
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
| MI (original) | 44.2738 | **45.5052 ✅** | 35.8801 | -1.2314 | Lower | +8.3937 | Higher | +9.6251 | Higher |
| MI (SEI) | -9.5873 | **15.5913 ✅** | -21.2896 | -25.1785 | Lower | +11.7023 | Higher | +36.8808 | Higher |
| Cyclomatic (sum) | **22 ✅** | 29 | 26 | -7 | Lower | -4 | Lower | +3 | Higher |
| Cognitive (sum) | **21 ✅** | 24 | 26 | -3 | Lower | -5 | Lower | -2 | Lower |
| Halstead Volume | 5089 | **2870.1554 ✅** | 6995.3258 | +2218.8446 | Higher | -1906.3258 | Lower | -4125.1705 | Lower |
| Halstead Effort | 187830.3636 | **121609.5454 ✅** | 333597.8495 | +66220.8182 | Higher | -145767.4858 | Lower | -211988.3040 | Lower |
| SLOC | **118 ✅** | 119 | 169 | -1 | Lower | -51 | Lower | -50 | Lower |
| PLOC | 99 | **98 ✅** | 148 | +1 | Higher | -49 | Lower | -50 | Lower |
| LLOC | 68 | **31 ✅** | 125 | +37 | Higher | -57 | Lower | -94 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 0 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 19 | **18 ✅** | 21 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 6 | 4 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02727

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.6746 | 54.8403 | **54.9878 ✅** | -1.1657 | Lower | -1.3132 | Lower | -0.1475 | Lower |
| MI (SEI) | 28.6725 | **36.4730 ✅** | 20.4308 | -7.8004 | Lower | +8.2418 | Higher | +16.0422 | Higher |
| Cyclomatic (sum) | 28 | 26 | **19 ✅** | +2 | Higher | +9 | Higher | +7 | Higher |
| Cognitive (sum) | **16 ✅** | 20 | 23 | -4 | Lower | -7 | Lower | -3 | Lower |
| Halstead Volume | 2520.5123 | **2494.8083 ✅** | 2584.1212 | +25.7041 | Higher | -63.6089 | Lower | -89.3129 | Lower |
| Halstead Effort | 108647.3480 | **100054.9416 ✅** | 129708.5279 | +8592.4065 | Higher | -21061.1799 | Lower | -29653.5864 | Lower |
| SLOC | 76 | **73 ✅** | 79 | +3 | Higher | -3 | Lower | -6 | Lower |
| PLOC | 57 | **49 ✅** | 69 | +8 | Higher | -12 | Lower | -20 | Lower |
| LLOC | **32 ✅** | 36 | 50 | -4 | Lower | -18 | Lower | -14 | Lower |
| Comments (CLOC) | 8 | **13 ✅** | 3 | -5 | Lower | +5 | Higher | +10 | Higher |
| Blank Lines | 13 | 13 | **10 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 4 | 10 | **1 ✅** | -6 | Lower | +3 | Higher | +9 | Higher |
| Exits | 15 | 7 | **1 ✅** | +8 | Higher | +14 | Higher | +6 | Higher |

## p02728

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.5179 | **70.7189 ✅** | 39.0129 | -32.2010 | Lower | -0.4950 | Lower | +31.7060 | Higher |
| MI (SEI) | -17.4839 | **66.7270 ✅** | -16.6679 | -84.2109 | Lower | -0.8160 | Lower | +83.3949 | Higher |
| Cyclomatic (sum) | 26 | **8 ✅** | 27 | +18 | Higher | -1 | Lower | -19 | Lower |
| Cognitive (sum) | 16 | **7 ✅** | 17 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 5860.5560 | **1100.2376 ✅** | 5779.3329 | +4760.3184 | Higher | +81.2231 | Higher | -4679.0953 | Lower |
| Halstead Effort | 755602.8490 | **30806.6532 ✅** | 697723.1052 | +724796.1958 | Higher | +57879.7438 | Higher | -666916.4521 | Lower |
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
| MI (original) | 54.1890 | **66.8120 ✅** | 50.3211 | -12.6230 | Lower | +3.8678 | Higher | +16.4908 | Higher |
| MI (SEI) | 22.8633 | **56.4333 ✅** | 7.1119 | -33.5700 | Lower | +15.7515 | Higher | +49.3215 | Higher |
| Cyclomatic (sum) | 19 | **10 ✅** | 23 | +9 | Higher | -4 | Lower | -13 | Lower |
| Cognitive (sum) | 44 | **16 ✅** | 63 | +28 | Higher | -19 | Lower | -47 | Lower |
| Halstead Volume | 2488.8541 | **1094.6527 ✅** | 2894.1998 | +1394.2014 | Higher | -405.3458 | Lower | -1799.5472 | Lower |
| Halstead Effort | 131169.3355 | **38269.0566 ✅** | 216043.5043 | +92900.2788 | Higher | -84874.1688 | Lower | -177774.4476 | Lower |
| SLOC | 84 | **57 ✅** | 96 | +27 | Higher | -12 | Lower | -39 | Lower |
| PLOC | 72 | **39 ✅** | 88 | +33 | Higher | -16 | Lower | -49 | Lower |
| LLOC | 66 | **28 ✅** | 86 | +38 | Higher | -20 | Lower | -58 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 1 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 10 | **7 ✅** | 8 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02734

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.0452 | **85.7131 ✅** | 65.5889 | -17.6679 | Lower | +2.4562 | Higher | +20.1242 | Higher |
| MI (SEI) | 48.3248 | **94.4229 ✅** | 45.5055 | -46.0981 | Lower | +2.8193 | Higher | +48.9174 | Higher |
| Cyclomatic (sum) | 9 | **4 ✅** | 10 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 2021.7564 | **344.9167 ✅** | 2083.0574 | +1676.8398 | Higher | -61.3009 | Lower | -1738.1407 | Lower |
| Halstead Effort | 138571.1854 | **7901.7269 ✅** | 159484.0785 | +130669.4585 | Higher | -20912.8931 | Lower | -151582.3516 | Lower |
| SLOC | 44 | **28 ✅** | 50 | +16 | Higher | -6 | Lower | -22 | Lower |
| PLOC | 38 | **10 ✅** | 39 | +28 | Higher | -1 | Lower | -29 | Lower |
| LLOC | 36 | **7 ✅** | 41 | +29 | Higher | -5 | Lower | -34 | Lower |
| Comments (CLOC) | 5 | **16 ✅** | 6 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02743

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.6053 | **87.5308 ✅** | 84.3892 | -12.9255 | Lower | -9.7839 | Lower | +3.1416 | Higher |
| MI (SEI) | 58.9530 | **91.0532 ✅** | 83.0033 | -32.1001 | Lower | -24.0503 | Lower | +8.0499 | Higher |
| Cyclomatic (sum) | 6 | 4 | 4 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1946.6611 | **346.1296 ✅** | 633.3094 | +1600.5315 | Higher | +1313.3517 | Higher | -287.1799 | Lower |
| Halstead Effort | 30951.9115 | **4499.6842 ✅** | 8893.8674 | +26452.2273 | Higher | +22058.0441 | Higher | -4394.1832 | Lower |
| SLOC | 31 | 25 | 25 | +6 | Higher | +6 | Higher | 0 | Same |
| PLOC | 22 | 15 | 15 | +7 | Higher | +7 | Higher | 0 | Same |
| LLOC | 10 | **7 ✅** | 10 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 4 | **9 ✅** | 7 | -5 | Lower | -3 | Lower | +2 | Higher |
| Blank Lines | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02761

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.8184 | **71.6810 ✅** | 61.5771 | -9.8625 | Lower | +0.2413 | Higher | +10.1039 | Higher |
| MI (SEI) | 25.6961 | **69.3652 ✅** | 25.2587 | -43.6691 | Lower | +0.4374 | Higher | +44.1066 | Higher |
| Cyclomatic (sum) | 19 | **12 ✅** | 19 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 25 | **13 ✅** | 25 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 2029.5119 | **766.1299 ✅** | 2011.8639 | +1263.3819 | Higher | +17.6479 | Higher | -1245.7340 | Lower |
| Halstead Effort | 113652.6637 | **25958.2849 ✅** | 106094.3866 | +87694.3788 | Higher | +7558.2772 | Higher | -80136.1016 | Lower |
| SLOC | 56 | **46 ✅** | 57 | +10 | Higher | -1 | Lower | -11 | Lower |
| PLOC | 49 | **27 ✅** | 50 | +22 | Higher | -1 | Lower | -23 | Lower |
| LLOC | 39 | **18 ✅** | 38 | +21 | Higher | +1 | Higher | -20 | Lower |
| Comments (CLOC) | 1 | **17 ✅** | 1 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
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
| MI (original) | 73.6775 | **79.4895 ✅** | 75.3702 | -5.8121 | Lower | -1.6927 | Lower | +4.1193 | Higher |
| MI (SEI) | 63.7484 | **84.6789 ✅** | 64.9475 | -20.9305 | Lower | -1.1990 | Lower | +19.7314 | Higher |
| Cyclomatic (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 839.5814 | **403.5515 ✅** | 926.4271 | +436.0299 | Higher | -86.8457 | Lower | -522.8756 | Lower |
| Halstead Effort | 22248.9076 | **13922.5278 ✅** | 22645.9964 | +8326.3798 | Higher | -397.0888 | Lower | -8723.4686 | Lower |
| SLOC | 43 | 38 | **37 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| PLOC | 29 | **20 ✅** | 25 | +9 | Higher | +4 | Higher | -5 | Lower |
| LLOC | 24 | **15 ✅** | 24 | +9 | Higher | 0 | Same | -9 | Lower |
| Comments (CLOC) | 9 | **20 ✅** | 7 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | 8 | **5 ✅** | 7 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | **0 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p02831

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.0182 | 79.0938 | **81.4519 ✅** | +1.9244 | Higher | -0.4337 | Lower | -2.3581 | Lower |
| MI (SEI) | 61.5800 | 79.5184 | **80.3430 ✅** | -17.9384 | Lower | -18.7631 | Lower | -0.8246 | Lower |
| Cyclomatic (sum) | 6 | 6 | 10 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 698.1203 | **562.6210 ✅** | 1712.3506 | +135.4993 | Higher | -1014.2303 | Lower | -1149.7297 | Lower |
| Halstead Effort | 12700.4195 | **12184.2602 ✅** | 27128.2516 | +516.1593 | Higher | -14427.8321 | Lower | -14943.9914 | Lower |
| SLOC | 29 | 35 | **20 ✅** | -6 | Lower | +9 | Higher | +15 | Higher |
| PLOC | 23 | **19 ✅** | 21 | +4 | Higher | +2 | Higher | -2 | Lower |
| LLOC | 16 | 14 | **9 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| Comments (CLOC) | 2 | **13 ✅** | 6 | -11 | Lower | -4 | Lower | +7 | Higher |
| Blank Lines | 4 | 6 | **-6 ✅** | -2 | Lower | +10 | Higher | +12 | Higher |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.9241 | **70.1832 ✅** | 61.8758 | -15.2591 | Lower | -6.9517 | Lower | +8.3074 | Higher |
| MI (SEI) | 24.6181 | **60.6727 ✅** | 24.9471 | -36.0546 | Lower | -0.3290 | Lower | +35.7256 | Higher |
| Cyclomatic (sum) | 17 | **7 ✅** | 15 | +10 | Higher | +2 | Higher | -8 | Lower |
| Cognitive (sum) | 26 | **7 ✅** | 26 | +19 | Higher | 0 | Same | -19 | Lower |
| Halstead Volume | 3224.2641 | **983.1622 ✅** | 1835.3846 | +2241.1018 | Higher | +1388.8794 | Higher | -852.2224 | Lower |
| Halstead Effort | 98448.5631 | **25910.4212 ✅** | 90348.2886 | +72538.1419 | Higher | +8100.2745 | Higher | -64437.8674 | Lower |
| SLOC | 76 | **50 ✅** | 61 | +26 | Higher | +15 | Higher | -11 | Lower |
| PLOC | 61 | **28 ✅** | 53 | +33 | Higher | +8 | Higher | -25 | Lower |
| LLOC | 37 | **21 ✅** | 37 | +16 | Higher | 0 | Same | -16 | Lower |
| Comments (CLOC) | 5 | **12 ✅** | 1 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | 10 | 11 | **7 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02840

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.1570 | **75.0645 ✅** | 49.7514 | -25.9075 | Lower | -0.5944 | Lower | +25.3131 | Higher |
| MI (SEI) | -2.7460 | **75.0298 ✅** | -1.9902 | -77.7758 | Lower | -0.7558 | Lower | +77.0200 | Higher |
| Cyclomatic (sum) | 20 | **9 ✅** | 19 | +11 | Higher | +1 | Higher | -10 | Lower |
| Cognitive (sum) | 24 | **6 ✅** | 23 | +18 | Higher | +1 | Higher | -17 | Lower |
| Halstead Volume | 3756.2361 | **705.4144 ✅** | 3731.8830 | +3050.8216 | Higher | +24.3530 | Higher | -3026.4686 | Lower |
| Halstead Effort | 265464.7599 | **20686.2778 ✅** | 286243.3683 | +244778.4821 | Higher | -20778.6085 | Lower | -265557.0905 | Lower |
| SLOC | 99 | **40 ✅** | 97 | +59 | Higher | +2 | Higher | -57 | Lower |
| PLOC | 85 | **25 ✅** | 84 | +60 | Higher | +1 | Higher | -59 | Lower |
| LLOC | 72 | **14 ✅** | 73 | +58 | Higher | -1 | Lower | -59 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 0 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 14 | **6 ✅** | 13 | +8 | Higher | +1 | Higher | -7 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p02846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.1947 | **78.9648 ✅** | 61.7742 | -7.7701 | Lower | +9.4205 | Higher | +17.1906 | Higher |
| MI (SEI) | 39.5629 | **84.5539 ✅** | 33.0574 | -44.9910 | Lower | +6.5055 | Higher | +51.4965 | Higher |
| Cyclomatic (sum) | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 1393.3077 | **634.1371 ✅** | 2520.2850 | +759.1706 | Higher | -1126.9773 | Lower | -1886.1479 | Lower |
| Halstead Effort | 42561.1959 | **16555.5086 ✅** | 175519.8482 | +26005.6873 | Higher | -132958.6523 | Lower | -158964.3396 | Lower |
| SLOC | 42 | **33 ✅** | 63 | +9 | Higher | -21 | Lower | -30 | Lower |
| PLOC | 33 | **20 ✅** | 51 | +13 | Higher | -18 | Lower | -31 | Lower |
| LLOC | 22 | **10 ✅** | 54 | +12 | Higher | -32 | Lower | -44 | Lower |
| Comments (CLOC) | 1 | **18 ✅** | 4 | -17 | Lower | -3 | Lower | +14 | Higher |
| Blank Lines | 8 | **3 ✅** | 8 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02862

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.5529 | 58.3438 | **63.4415 ✅** | +4.2091 | Higher | -0.8886 | Lower | -5.0977 | Lower |
| MI (SEI) | 15.8676 | **35.7734 ✅** | 16.9460 | -19.9058 | Lower | -1.0784 | Lower | +18.8274 | Higher |
| Cyclomatic (sum) | 13 | 14 | **11 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | **1952.9764 ✅** | 2162.4177 | 2002.4359 | -209.4413 | Lower | -49.4595 | Lower | +159.9817 | Higher |
| Halstead Effort | **94891.6764 ✅** | 115547.0804 | 103216.4693 | -20655.4040 | Lower | -8324.7929 | Lower | +12330.6111 | Higher |
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
| MI (original) | 51.3750 | **61.7659 ✅** | 55.1726 | -10.3910 | Lower | -3.7976 | Lower | +6.5934 | Higher |
| MI (SEI) | 20.9678 | **42.8620 ✅** | 6.0345 | -21.8942 | Lower | +14.9333 | Higher | +36.8275 | Higher |
| Cyclomatic (sum) | 21 | **14 ✅** | 21 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 37 | **25 ✅** | 32 | +12 | Higher | +5 | Higher | -7 | Lower |
| Halstead Volume | 2667.4799 | **1959.3468 ✅** | 2575.4043 | +708.1331 | Higher | +92.0756 | Higher | -616.0575 | Lower |
| Halstead Effort | 168737.1601 | **69580.7062 ✅** | 191535.3101 | +99156.4539 | Higher | -22798.1500 | Lower | -121954.6039 | Lower |
| SLOC | 95 | **61 ✅** | 76 | +34 | Higher | +19 | Higher | -15 | Lower |
| PLOC | 71 | **45 ✅** | 65 | +26 | Higher | +6 | Higher | -20 | Lower |
| LLOC | 64 | **29 ✅** | 61 | +35 | Higher | +3 | Higher | -32 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 0 | -2 | Lower | +7 | Higher | +9 | Higher |
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
| MI (original) | 75.7357 | **80.0337 ✅** | 54.0699 | -4.2980 | Lower | +21.6658 | Higher | +25.9638 | Higher |
| MI (SEI) | 64.9212 | **83.0532 ✅** | 4.3420 | -18.1321 | Lower | +60.5792 | Higher | +78.7113 | Higher |
| Cyclomatic (sum) | **5 ✅** | 6 | 20 | -1 | Lower | -15 | Lower | -14 | Lower |
| Cognitive (sum) | **5 ✅** | 7 | 15 | -2 | Lower | -10 | Lower | -8 | Lower |
| Halstead Volume | 868.2024 | **335.2007 ✅** | 2184 | +533.0017 | Higher | -1315.7976 | Lower | -1848.7993 | Lower |
| Halstead Effort | 22648.7572 | **11878.6744 ✅** | 108714.6667 | +10770.0828 | Higher | -86065.9094 | Lower | -96835.9923 | Lower |
| SLOC | **38 ✅** | 39 | 87 | -1 | Lower | -49 | Lower | -48 | Lower |
| PLOC | 25 | **18 ✅** | 73 | +7 | Higher | -48 | Lower | -55 | Lower |
| LLOC | 22 | **13 ✅** | 46 | +9 | Higher | -24 | Lower | -33 | Lower |
| Comments (CLOC) | 7 | **17 ✅** | 0 | -10 | Lower | +7 | Higher | +17 | Higher |
| Blank Lines | 7 | **5 ✅** | 14 | +2 | Higher | -7 | Lower | -9 | Lower |
| Functions | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |
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
| MI (original) | 80.2053 | **80.7139 ✅** | 80.3542 | -0.5086 | Lower | -0.1489 | Lower | +0.3597 | Higher |
| MI (SEI) | 70.1783 | **85.4234 ✅** | 72.7722 | -15.2451 | Lower | -2.5939 | Lower | +12.6512 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 734.4261 | **494.8981 ✅** | 713.6951 | +239.5280 | Higher | +20.7310 | Higher | -218.7970 | Lower |
| Halstead Effort | 16524.5867 | **11062.4274 ✅** | 16950.2589 | +5462.1593 | Higher | -425.6722 | Lower | -5887.8314 | Lower |
| SLOC | 30 | 33 | 30 | -3 | Lower | 0 | Same | +3 | Higher |
| PLOC | 22 | **16 ✅** | 21 | +6 | Higher | +1 | Higher | -5 | Lower |
| LLOC | 16 | **8 ✅** | 16 | +8 | Higher | 0 | Same | -8 | Lower |
| Comments (CLOC) | 5 | **16 ✅** | 6 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02956

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.3046 | **70.0694 ✅** | 32.3550 | -37.7647 | Lower | -0.0504 | Lower | +37.7144 | Higher |
| MI (SEI) | -17.2435 | **66.3068 ✅** | -25.2552 | -83.5502 | Lower | +8.0117 | Higher | +91.5619 | Higher |
| Cyclomatic (sum) | 37 | **7 ✅** | 37 | +30 | Higher | 0 | Same | -30 | Lower |
| Cognitive (sum) | 48 | **2 ✅** | 48 | +46 | Higher | 0 | Same | -46 | Lower |
| Halstead Volume | 6789.7508 | **746.7564 ✅** | 6840.7260 | +6042.9944 | Higher | -50.9752 | Lower | -6093.9695 | Lower |
| Halstead Effort | 1026285.5898 | **13994.7683 ✅** | 1012712.4719 | +1012290.8215 | Higher | +13573.1179 | Higher | -998717.7036 | Lower |
| SLOC | 182 | **55 ✅** | 181 | +127 | Higher | +1 | Higher | -126 | Lower |
| PLOC | 157 | **27 ✅** | 156 | +130 | Higher | +1 | Higher | -129 | Lower |
| LLOC | 150 | **11 ✅** | 149 | +139 | Higher | +1 | Higher | -138 | Lower |
| Comments (CLOC) | 2 | **20 ✅** | 0 | -18 | Lower | +2 | Higher | +20 | Higher |
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
| MI (original) | 64.1915 | **78.2983 ✅** | 72.1450 | -14.1068 | Lower | -7.9535 | Lower | +6.1533 | Higher |
| MI (SEI) | 49.1093 | **79.1862 ✅** | 63.1682 | -30.0768 | Lower | -14.0589 | Lower | +16.0179 | Higher |
| Cyclomatic (sum) | 11 | **4 ✅** | 9 | +7 | Higher | +2 | Higher | -5 | Lower |
| Cognitive (sum) | 15 | **1 ✅** | 15 | +14 | Higher | 0 | Same | -14 | Lower |
| Halstead Volume | 2961 | **716.2669 ✅** | 1338.2215 | +2244.7331 | Higher | +1622.7785 | Higher | -621.9546 | Lower |
| Halstead Effort | 66830.0467 | **15882.4410 ✅** | 64903.7435 | +50947.6057 | Higher | +1926.3032 | Higher | -49021.3025 | Lower |
| SLOC | 48 | **35 ✅** | 39 | +13 | Higher | +9 | Higher | -4 | Lower |
| PLOC | 30 | **17 ✅** | 25 | +13 | Higher | +5 | Higher | -8 | Lower |
| LLOC | 24 | **15 ✅** | 27 | +9 | Higher | -3 | Lower | -12 | Lower |
| Comments (CLOC) | 9 | **14 ✅** | 9 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02988

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.5718 | 72.2256 | **75.8941 ✅** | +2.3462 | Higher | -1.3223 | Lower | -3.6685 | Lower |
| MI (SEI) | 54.1479 | **63.3903 ✅** | 61.8961 | -9.2424 | Lower | -7.7482 | Lower | +1.4942 | Higher |
| Cyclomatic (sum) | 9 | 9 | **8 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 7 | 7 | 11 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | 988.7168 | 972.0530 | **872.7962 ✅** | +16.6638 | Higher | +115.9206 | Higher | +99.2568 | Higher |
| Halstead Effort | 32689.4497 | 31206.9531 | **28457.7500 ✅** | +1482.4966 | Higher | +4231.6997 | Higher | +2749.2031 | Higher |
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
| MI (original) | 47.4467 | **70.7628 ✅** | 47.7475 | -23.3161 | Lower | -0.3008 | Lower | +23.0152 | Higher |
| MI (SEI) | 2.2470 | **59.7439 ✅** | 2.7512 | -57.4969 | Lower | -0.5042 | Lower | +56.9927 | Higher |
| Cyclomatic (sum) | 20 | **11 ✅** | 20 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 36 | **10 ✅** | 36 | +26 | Higher | 0 | Same | -26 | Lower |
| Halstead Volume | 4096.8625 | **1097.3458 ✅** | 4100.6763 | +2999.5167 | Higher | -3.8138 | Lower | -3003.3305 | Lower |
| Halstead Effort | 452811.1176 | **28805.3261 ✅** | 501421.5845 | +424005.7915 | Higher | -48610.4669 | Lower | -472616.2584 | Lower |
| SLOC | 107 | **44 ✅** | 105 | +63 | Higher | +2 | Higher | -61 | Lower |
| PLOC | 99 | **29 ✅** | 98 | +70 | Higher | +1 | Higher | -69 | Lower |
| LLOC | 78 | **19 ✅** | 78 | +59 | Higher | 0 | Same | -59 | Lower |
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
| MI (original) | 61.9041 | **77.9742 ✅** | 72.1107 | -16.0701 | Lower | -10.2066 | Lower | +5.8635 | Higher |
| MI (SEI) | 41.8307 | **77.0922 ✅** | 54.3494 | -35.2615 | Lower | -12.5188 | Lower | +22.7427 | Higher |
| Cyclomatic (sum) | 12 | **5 ✅** | 8 | +7 | Higher | +4 | Higher | -3 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 5 | +4 | Higher | +1 | Higher | -3 | Lower |
| Halstead Volume | 2312.5681 | **798.2752 ✅** | 1038.7213 | +1514.2928 | Higher | +1273.8467 | Higher | -240.4461 | Lower |
| Halstead Effort | 56950.1652 | **24726.5758 ✅** | 29863.2380 | +32223.5894 | Higher | +27086.9272 | Higher | -5136.6622 | Lower |
| SLOC | 59 | **34 ✅** | 43 | +25 | Higher | +16 | Higher | -9 | Lower |
| PLOC | 40 | **19 ✅** | 29 | +21 | Higher | +11 | Higher | -10 | Lower |
| LLOC | 29 | **16 ✅** | 27 | +13 | Higher | +2 | Higher | -11 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 5 | -4 | Lower | +3 | Higher | +7 | Higher |
| Blank Lines | 11 | **5 ✅** | 9 | +6 | Higher | +2 | Higher | -4 | Lower |
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
| MI (original) | 60.2642 | **76.3518 ✅** | 60.2124 | -16.0876 | Lower | +0.0518 | Higher | +16.1394 | Higher |
| MI (SEI) | 12.4638 | **76.4798 ✅** | 12.3891 | -64.0160 | Lower | +0.0747 | Higher | +64.0907 | Higher |
| Cyclomatic (sum) | 12 | **5 ✅** | 12 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 13 | **3 ✅** | 13 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 2460.3520 | **657.3039 ✅** | 2484.9789 | +1803.0481 | Higher | -24.6269 | Lower | -1827.6751 | Lower |
| Halstead Effort | 183216.8591 | **15446.6410 ✅** | 180273.9271 | +167770.2181 | Higher | +2942.9321 | Higher | -164827.2861 | Lower |
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
| MI (original) | 61.9579 | **62.8098 ✅** | 60.0943 | -0.8518 | Lower | +1.8636 | Higher | +2.7154 | Higher |
| MI (SEI) | 40.2492 | **57.7573 ✅** | 12.3206 | -17.5081 | Lower | +27.9286 | Higher | +45.4367 | Higher |
| Cyclomatic (sum) | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 16 | 23 | 16 | -7 | Lower | 0 | Same | +7 | Higher |
| Halstead Volume | 2078.0145 | **1374.7009 ✅** | 2554.3490 | +703.3136 | Higher | -476.3344 | Lower | -1179.6480 | Lower |
| Halstead Effort | 77818.9794 | **54039.9668 ✅** | 117666.6397 | +23779.0126 | Higher | -39847.6603 | Lower | -63626.6729 | Lower |
| SLOC | **60 ✅** | 65 | 63 | -5 | Lower | -3 | Lower | +2 | Higher |
| PLOC | 43 | **35 ✅** | 54 | +8 | Higher | -11 | Lower | -19 | Lower |
| LLOC | 34 | **24 ✅** | 49 | +10 | Higher | -15 | Lower | -25 | Lower |
| Comments (CLOC) | 7 | **26 ✅** | 0 | -19 | Lower | +7 | Higher | +26 | Higher |
| Blank Lines | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03055

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.6222 | **67.7705 ✅** | 31.5794 | -31.1483 | Lower | +5.0428 | Higher | +36.1911 | Higher |
| MI (SEI) | -2.1130 | **53.7990 ✅** | -26.2722 | -55.9120 | Lower | +24.1592 | Higher | +80.0712 | Higher |
| Cyclomatic (sum) | 30 | **9 ✅** | 38 | +21 | Higher | -8 | Lower | -29 | Lower |
| Cognitive (sum) | 53 | **9 ✅** | 56 | +44 | Higher | -3 | Lower | -47 | Lower |
| Halstead Volume | 5475.1316 | **1524.1995 ✅** | 6120.9711 | +3950.9321 | Higher | -645.8395 | Lower | -4596.7716 | Lower |
| Halstead Effort | 629640.1373 | **49253.4180 ✅** | 829923.8433 | +580386.7193 | Higher | -200283.7060 | Lower | -780670.4253 | Lower |
| SLOC | 165 | **49 ✅** | 194 | +116 | Higher | -29 | Lower | -145 | Lower |
| PLOC | 144 | **34 ✅** | 170 | +110 | Higher | -26 | Lower | -136 | Lower |
| LLOC | 134 | **22 ✅** | 144 | +112 | Higher | -10 | Lower | -122 | Lower |
| Comments (CLOC) | 9 | 9 | 0 | 0 | Same | +9 | Higher | +9 | Higher |
| Blank Lines | 13 | **8 ✅** | 24 | +5 | Higher | -11 | Lower | -16 | Lower |
| Functions | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |

## p03064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.9386 | **76.5294 ✅** | 69.1592 | -6.5907 | Lower | +0.7795 | Higher | +7.3702 | Higher |
| MI (SEI) | 47.8656 | **70.6965 ✅** | 46.8428 | -22.8309 | Lower | +1.0227 | Higher | +23.8537 | Higher |
| Cyclomatic (sum) | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 5 | **1 ✅** | 7 | +4 | Higher | -2 | Lower | -6 | Lower |
| Halstead Volume | 1396.6077 | **814.2399 ✅** | 1552.2664 | +582.3678 | Higher | -155.6587 | Lower | -738.0265 | Lower |
| Halstead Effort | 49844.4483 | **13528.9096 ✅** | 73991.3671 | +36315.5387 | Higher | -24146.9188 | Lower | -60462.4575 | Lower |
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
| MI (original) | 59.8627 | **72.7486 ✅** | 59.2764 | -12.8859 | Lower | +0.5863 | Higher | +13.4723 | Higher |
| MI (SEI) | 30.2324 | **71.2136 ✅** | 26.6991 | -40.9812 | Lower | +3.5333 | Higher | +44.5145 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 11 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 2200.4126 | **818.2861 ✅** | 2352.1333 | +1382.1265 | Higher | -151.7207 | Lower | -1533.8472 | Lower |
| Halstead Effort | 88425.8832 | **29151.4427 ✅** | 95951.7000 | +59274.4405 | Higher | -7525.8167 | Lower | -66800.2572 | Lower |
| SLOC | 68 | **44 ✅** | 70 | +24 | Higher | -2 | Lower | -26 | Lower |
| PLOC | 58 | **31 ✅** | 58 | +27 | Higher | 0 | Same | -27 | Lower |
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
| MI (original) | **83.9531 ✅** | 79.5865 | 80.0229 | +4.3666 | Higher | +3.9302 | Higher | -0.4364 | Lower |
| MI (SEI) | **79.6946 ✅** | 74.2754 | 71.9528 | +5.4192 | Higher | +7.7418 | Higher | +2.3226 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 9 | -2 | Lower | -5 | Lower | -3 | Lower |
| Cognitive (sum) | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **609.5060 ✅** | 614.7242 | 2023.6711 | -5.2181 | Lower | -1414.1651 | Lower | -1408.9469 | Lower |
| Halstead Effort | **9325.4425 ✅** | 11679.7595 | 29478.1427 | -2354.3171 | Lower | -20152.7003 | Lower | -17798.3832 | Lower |
| SLOC | 26 | 33 | **21 ✅** | -7 | Lower | +5 | Higher | +12 | Higher |
| PLOC | **15 ✅** | 22 | 22 | -7 | Lower | -7 | Lower | 0 | Same |
| LLOC | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Comments (CLOC) | 6 | **8 ✅** | 4 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 5 | 6 | **-5 ✅** | -1 | Lower | +10 | Higher | +11 | Higher |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
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
| MI (original) | 54.6178 | **57.0676 ✅** | 52.5288 | -2.4497 | Lower | +2.0891 | Higher | +4.5388 | Higher |
| MI (SEI) | 31.2492 | **36.9296 ✅** | 15.6398 | -5.6804 | Lower | +15.6094 | Higher | +21.2898 | Higher |
| Cyclomatic (sum) | 14 | **12 ✅** | 15 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 8 | **7 ✅** | 10 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 2967.7656 | **2184 ✅** | 3414 | +783.7656 | Higher | -446.2344 | Lower | -1230 | Lower |
| Halstead Effort | 130006.3019 | **77700 ✅** | 235925.3684 | +52306.3019 | Higher | -105919.0665 | Lower | -158225.3684 | Lower |
| SLOC | 83 | **81 ✅** | 89 | +2 | Higher | -6 | Lower | -8 | Lower |
| PLOC | 70 | **58 ✅** | 75 | +12 | Higher | -5 | Lower | -17 | Lower |
| LLOC | 45 | **37 ✅** | 58 | +8 | Higher | -13 | Lower | -21 | Lower |
| Comments (CLOC) | 11 | **13 ✅** | 3 | -2 | Lower | +8 | Higher | +10 | Higher |
| Blank Lines | 13 | **12 ✅** | 14 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |

## p03115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.3126 ✅** | 63.7200 | 16.7306 | +0.5926 | Higher | +47.5820 | Higher | +46.9894 | Higher |
| MI (SEI) | 18.6099 | **55.6884 ✅** | -34.6063 | -37.0785 | Lower | +53.2161 | Higher | +90.2947 | Higher |
| Cyclomatic (sum) | 15 | **11 ✅** | 67 | +4 | Higher | -52 | Lower | -56 | Lower |
| Cognitive (sum) | 10 | **7 ✅** | 37 | +3 | Higher | -27 | Lower | -30 | Lower |
| Halstead Volume | 1888.8205 | **1202.1225 ✅** | 8616.4453 | +686.6980 | Higher | -6727.6249 | Lower | -7414.3228 | Lower |
| Halstead Effort | 104298.3045 | **36985.3013 ✅** | 864106.3722 | +67313.0033 | Higher | -759808.0676 | Lower | -827121.0709 | Lower |
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
| MI (original) | 42.5018 | **82.3710 ✅** | 41.6295 | -39.8692 | Lower | +0.8723 | Higher | +40.7415 | Higher |
| MI (SEI) | -5.1167 | **87.4820 ✅** | -1.8548 | -92.5986 | Lower | -3.2619 | Lower | +89.3367 | Higher |
| Cyclomatic (sum) | 25 | **7 ✅** | 25 | +18 | Higher | 0 | Same | -18 | Lower |
| Cognitive (sum) | 24 | **7 ✅** | 24 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 4418.7926 | **378.9225 ✅** | 4350.5812 | +4039.8700 | Higher | +68.2113 | Higher | -3971.6587 | Lower |
| Halstead Effort | 273820.2603 | **13051.7756 ✅** | 271148.0676 | +260768.4847 | Higher | +2672.1928 | Higher | -258096.2920 | Lower |
| SLOC | 132 | **32 ✅** | 140 | +100 | Higher | -8 | Lower | -108 | Lower |
| PLOC | 116 | **16 ✅** | 120 | +100 | Higher | -4 | Lower | -104 | Lower |
| LLOC | 81 | **11 ✅** | 91 | +70 | Higher | -10 | Lower | -80 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 3 | -14 | Lower | -2 | Lower | +12 | Higher |
| Blank Lines | 16 | **5 ✅** | 18 | +11 | Higher | -2 | Lower | -13 | Lower |
| Functions | 9 | **1 ✅** | 9 | +8 | Higher | 0 | Same | -8 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 8 | **2 ✅** | 8 | +6 | Higher | 0 | Same | -6 | Lower |

## p03142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.1431 | **74.1723 ✅** | 55.9894 | -17.0291 | Lower | +1.1538 | Higher | +18.1829 | Higher |
| MI (SEI) | 17.0033 | **64.6971 ✅** | 6.5002 | -47.6938 | Lower | +10.5031 | Higher | +58.1970 | Higher |
| Cyclomatic (sum) | 14 | **4 ✅** | 14 | +10 | Higher | 0 | Same | -10 | Lower |
| Cognitive (sum) | 20 | **2 ✅** | 20 | +18 | Higher | 0 | Same | -18 | Lower |
| Halstead Volume | 2402.8665 | **1331.9501 ✅** | 2556.7787 | +1070.9165 | Higher | -153.9122 | Lower | -1224.8287 | Lower |
| Halstead Effort | 114818.0547 | **31021.5466 ✅** | 123812.0092 | +83796.5081 | Higher | -8993.9545 | Lower | -92790.4626 | Lower |
| SLOC | 76 | **37 ✅** | 80 | +39 | Higher | -4 | Lower | -43 | Lower |
| PLOC | 65 | **26 ✅** | 69 | +39 | Higher | -4 | Lower | -43 | Lower |
| LLOC | 45 | **22 ✅** | 52 | +23 | Higher | -7 | Lower | -30 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 0 | -7 | Lower | +1 | Higher | +8 | Higher |
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
| MI (original) | 49.2626 | 52.7008 | **53.7242 ✅** | -3.4382 | Lower | -4.4616 | Lower | -1.0234 | Lower |
| MI (SEI) | 7.0924 | **28.1437 ✅** | 3.1305 | -21.0514 | Lower | +3.9619 | Higher | +25.0133 | Higher |
| Cyclomatic (sum) | 33 | 23 | **13 ✅** | +10 | Higher | +20 | Higher | +10 | Higher |
| Cognitive (sum) | 27 | 20 | **13 ✅** | +7 | Higher | +14 | Higher | +7 | Higher |
| Halstead Volume | 3330.3974 | **2776.1864 ✅** | 4296.3736 | +554.2109 | Higher | -965.9762 | Lower | -1520.1872 | Lower |
| Halstead Effort | 175317.1450 | 150608.1142 | **134253.0823 ✅** | +24709.0307 | Higher | +41064.0626 | Higher | +16355.0319 | Higher |
| SLOC | 85 | 84 | **79 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| PLOC | 67 | 60 | 60 | +7 | Higher | +7 | Higher | 0 | Same |
| LLOC | 49 | **31 ✅** | 35 | +18 | Higher | +14 | Higher | -4 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 0 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 18 | **14 ✅** | 19 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 12 | 3 | **1 ✅** | +9 | Higher | +11 | Higher | +2 | Higher |

## p03164

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.1103 | 57.2871 | **62.4069 ✅** | +2.8232 | Higher | -2.2966 | Lower | -5.1199 | Lower |
| MI (SEI) | 12.8527 | **37.7473 ✅** | 15.7588 | -24.8946 | Lower | -2.9061 | Lower | +21.9885 | Higher |
| Cyclomatic (sum) | 18 | 24 | **14 ✅** | -6 | Lower | +4 | Higher | +10 | Higher |
| Cognitive (sum) | 12 | 10 | 10 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | **2041.2771 ✅** | 2330.7542 | 3218.2535 | -289.4770 | Lower | -1176.9764 | Lower | -887.4994 | Lower |
| Halstead Effort | 86783.4395 | 89544.3225 | **78833.9131 ✅** | -2760.8831 | Lower | +7949.5264 | Higher | +10710.4094 | Higher |
| SLOC | 63 | 66 | **50 ✅** | -3 | Lower | +13 | Higher | +16 | Higher |
| PLOC | 50 | 47 | **41 ✅** | +3 | Higher | +9 | Higher | +6 | Higher |
| LLOC | 39 | 31 | **27 ✅** | +8 | Higher | +12 | Higher | +4 | Higher |
| Comments (CLOC) | 0 | **10 ✅** | 0 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 13 | 9 | 9 | +4 | Higher | +4 | Higher | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 8 | 6 | **3 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Exits | 2 | 10 | **1 ✅** | -8 | Lower | +1 | Higher | +9 | Higher |

## p03165

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.8538 ✅** | 60.8705 | 61.5791 | +1.9833 | Higher | +1.2746 | Higher | -0.7087 | Lower |
| MI (SEI) | **44.7423 ✅** | 42.5157 | 43.1014 | +2.2266 | Higher | +1.6409 | Higher | -0.5857 | Lower |
| Cyclomatic (sum) | 13 | 12 | 12 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 16 | **15 ✅** | 16 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1843.1932 | **1812.7398 ✅** | 1910.6198 | +30.4534 | Higher | -67.4265 | Lower | -97.8799 | Lower |
| Halstead Effort | 95846.0487 | 101546.9998 | **93859.1962 ✅** | -5700.9510 | Lower | +1986.8525 | Higher | +7687.8036 | Higher |
| SLOC | **59 ✅** | 68 | 64 | -9 | Lower | -5 | Lower | +4 | Higher |
| PLOC | **43 ✅** | 45 | 46 | -2 | Lower | -3 | Lower | -1 | Lower |
| LLOC | 37 | **35 ✅** | 39 | +2 | Higher | -2 | Lower | -4 | Lower |
| Comments (CLOC) | 9 | **11 ✅** | 10 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | **9 ✅** | 13 | 11 | -4 | Lower | -2 | Lower | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03173

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.1752 ✅** | 66.4502 | 69.9293 | +3.7250 | Higher | +0.2459 | Higher | -3.4791 | Lower |
| MI (SEI) | 26.4569 | **56.2021 ✅** | 37.9409 | -29.7452 | Lower | -11.4840 | Lower | +18.2612 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1672.5528 | **1432.9670 ✅** | 1626.7294 | +239.5858 | Higher | +45.8234 | Higher | -193.7624 | Lower |
| Halstead Effort | 98133.8175 | **40579.0197 ✅** | 84589.9293 | +57554.7978 | Higher | +13543.8882 | Higher | -44010.9096 | Lower |
| SLOC | **41 ✅** | 55 | 42 | -14 | Lower | -1 | Lower | +13 | Higher |
| PLOC | 34 | **32 ✅** | 34 | +2 | Higher | 0 | Same | -2 | Lower |
| LLOC | 26 | **21 ✅** | 25 | +5 | Higher | +1 | Higher | -4 | Lower |
| Comments (CLOC) | 0 | **14 ✅** | 1 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | **7 ✅** | 10 | 8 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
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
| MI (original) | 77.7118 | 80.2068 | **81.7629 ✅** | -2.4950 | Lower | -4.0511 | Lower | -1.5561 | Lower |
| MI (SEI) | 56.1955 | **78.0768 ✅** | 73.7875 | -21.8813 | Lower | -17.5920 | Lower | +4.2893 | Higher |
| Cyclomatic (sum) | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 6 | **1 ✅** | 6 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1071.1211 | **596.0559 ✅** | 755.8125 | +475.0652 | Higher | +315.3086 | Higher | -159.7566 | Lower |
| Halstead Effort | 26985.3419 | **12517.1749 ✅** | 21603.6411 | +14468.1670 | Higher | +5381.7008 | Higher | -9086.4662 | Lower |
| SLOC | 31 | 33 | **27 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| PLOC | 25 | **16 ✅** | 20 | +9 | Higher | +5 | Higher | -4 | Lower |
| LLOC | 15 | **9 ✅** | 12 | +6 | Higher | +3 | Higher | -3 | Lower |
| Comments (CLOC) | 2 | **10 ✅** | 5 | -8 | Lower | -3 | Lower | +5 | Higher |
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
| MI (original) | 56.9718 | **65.1842 ✅** | 60.4963 | -8.2123 | Lower | -3.5245 | Lower | +4.6879 | Higher |
| MI (SEI) | 35.6458 | **53.1200 ✅** | 13.0023 | -17.4743 | Lower | +22.6434 | Higher | +40.1177 | Higher |
| Cyclomatic (sum) | 18 | 14 | 14 | +4 | Higher | +4 | Higher | 0 | Same |
| Cognitive (sum) | 15 | **5 ✅** | 30 | +10 | Higher | -15 | Lower | -25 | Lower |
| Halstead Volume | 1844.2537 | **1484.3714 ✅** | 1867.3021 | +359.8823 | Higher | -23.0483 | Lower | -382.9306 | Lower |
| Halstead Effort | 68852.1399 | **42632.5287 ✅** | 78185.7447 | +26219.6113 | Higher | -9333.6047 | Lower | -35553.2160 | Lower |
| SLOC | 79 | **54 ✅** | 67 | +25 | Higher | +12 | Higher | -13 | Lower |
| PLOC | 59 | **37 ✅** | 58 | +22 | Higher | +1 | Higher | -21 | Lower |
| LLOC | 45 | **24 ✅** | 55 | +21 | Higher | -10 | Lower | -31 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 0 | -1 | Lower | +11 | Higher | +12 | Higher |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03218

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.4886 ✅** | 71.4839 | 71.5482 | +3.0047 | Higher | +2.9404 | Higher | -0.0643 | Lower |
| MI (SEI) | 58.4116 | **64.8201 ✅** | 52.9216 | -6.4085 | Lower | +5.4900 | Higher | +11.8985 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | **899.8434 ✅** | 992.6744 | 1049.9730 | -92.8310 | Lower | -150.1296 | Lower | -57.2985 | Lower |
| Halstead Effort | **20350.3044 ✅** | 21462.3059 | 25724.3373 | -1112.0014 | Lower | -5374.0328 | Lower | -4262.0314 | Lower |
| SLOC | **40 ✅** | 46 | 45 | -6 | Lower | -5 | Lower | +1 | Higher |
| PLOC | **27 ✅** | 29 | 31 | -2 | Lower | -4 | Lower | -2 | Lower |
| LLOC | 15 | 15 | 16 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 5 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | **8 ✅** | 9 | 10 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p03268

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.2664 | 79.2218 | **82.7523 ✅** | -0.9555 | Lower | -4.4859 | Lower | -3.5305 | Lower |
| MI (SEI) | 65.9777 | 74.3613 | **74.9094 ✅** | -8.3836 | Lower | -8.9318 | Lower | -0.5481 | Lower |
| Cyclomatic (sum) | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 865.6692 | **599.7113 ✅** | 713.5293 | +265.9579 | Higher | +152.1399 | Higher | -113.8180 | Lower |
| Halstead Effort | 15880.5520 | **10794.8034 ✅** | 14066.7205 | +5085.7486 | Higher | +1813.8315 | Higher | -3271.9171 | Lower |
| SLOC | 33 | 35 | **27 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| PLOC | 23 | 19 | **18 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| LLOC | 15 | 11 | 11 | +4 | Higher | +4 | Higher | 0 | Same |
| Comments (CLOC) | 5 | **9 ✅** | 5 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 5 | 7 | **4 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.1509 | **74.0535 ✅** | 55.2188 | -17.9027 | Lower | +0.9320 | Higher | +18.8347 | Higher |
| MI (SEI) | 23.9139 | **70.8411 ✅** | 22.4675 | -46.9271 | Lower | +1.4465 | Higher | +48.3736 | Higher |
| Cyclomatic (sum) | 15 | **7 ✅** | 16 | +8 | Higher | -1 | Lower | -9 | Lower |
| Cognitive (sum) | 12 | **4 ✅** | 13 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 2466.1128 | **1012.8602 ✅** | 2611.0746 | +1453.2526 | Higher | -144.9618 | Lower | -1598.2144 | Lower |
| Halstead Effort | 133370.0463 | **24423.0000 ✅** | 140998.0265 | +108947.0463 | Higher | -7627.9802 | Lower | -116575.0265 | Lower |
| SLOC | 79 | **39 ✅** | 81 | +40 | Higher | -2 | Lower | -42 | Lower |
| PLOC | 65 | **26 ✅** | 71 | +39 | Higher | -6 | Lower | -45 | Lower |
| LLOC | 51 | **17 ✅** | 55 | +34 | Higher | -4 | Lower | -38 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 4 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 14 | **6 ✅** | 10 | +8 | Higher | +4 | Higher | -4 | Lower |
| Functions | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03278

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.2694 | **65.2354 ✅** | 44.9171 | -18.9659 | Lower | +1.3523 | Higher | +20.3183 | Higher |
| MI (SEI) | 0.5528 | **57.9774 ✅** | -8.8628 | -57.4247 | Lower | +9.4155 | Higher | +66.8402 | Higher |
| Cyclomatic (sum) | 19 | **8 ✅** | 20 | +11 | Higher | -1 | Lower | -12 | Lower |
| Cognitive (sum) | 19 | **6 ✅** | 21 | +13 | Higher | -2 | Lower | -15 | Lower |
| Halstead Volume | 5867.5987 | **1534.0501 ✅** | 6290.1763 | +4333.5486 | Higher | -422.5776 | Lower | -4756.1262 | Lower |
| Halstead Effort | 809657.0613 | **43873.8321 ✅** | 857073.0861 | +765783.2291 | Higher | -47416.0249 | Lower | -813199.2540 | Lower |
| SLOC | 104 | **58 ✅** | 109 | +46 | Higher | -5 | Lower | -51 | Lower |
| PLOC | 92 | **34 ✅** | 96 | +58 | Higher | -4 | Lower | -62 | Lower |
| LLOC | 86 | **21 ✅** | 90 | +65 | Higher | -4 | Lower | -69 | Lower |
| Comments (CLOC) | 1 | **19 ✅** | 0 | -18 | Lower | +1 | Higher | +19 | Higher |
| Blank Lines | **12 ✅** | 13 | 13 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03294

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.5335 | 85.2811 | **86.0192 ✅** | -12.7476 | Lower | -13.4857 | Lower | -0.7381 | Lower |
| MI (SEI) | 56.0939 | **83.6404 ✅** | 81.1859 | -27.5466 | Lower | -25.0921 | Lower | +2.4545 | Higher |
| Cyclomatic (sum) | 5 | 3 | 3 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 1607.1823 | **493.4839 ✅** | 549.4460 | +1113.6984 | Higher | +1057.7363 | Higher | -55.9621 | Lower |
| Halstead Effort | 21005.6992 | 7457.0900 | **7157.2569 ✅** | +13548.6092 | Higher | +13848.4423 | Higher | +299.8330 | Higher |
| SLOC | 38 | 26 | **24 ✅** | +12 | Higher | +14 | Higher | +2 | Higher |
| PLOC | 25 | **13 ✅** | 15 | +12 | Higher | +10 | Higher | -2 | Lower |
| LLOC | 14 | **11 ✅** | 12 | +3 | Higher | +2 | Higher | -1 | Lower |
| Comments (CLOC) | 5 | **7 ✅** | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 8 | 6 | **5 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03296

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.4958 | 77.0584 | **77.3876 ✅** | -2.5626 | Lower | -2.8918 | Lower | -0.3292 | Lower |
| MI (SEI) | 61.0333 | **71.0532 ✅** | 67.2354 | -10.0199 | Lower | -6.2021 | Lower | +3.8178 | Higher |
| Cyclomatic (sum) | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 6 | 4 | 4 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 930.2778 | **762.2515 ✅** | 938.2737 | +168.0262 | Higher | -7.9959 | Lower | -176.0222 | Lower |
| Halstead Effort | 21524.7026 | **15366.9912 ✅** | 25802.5268 | +6157.7113 | Higher | -4277.8242 | Lower | -10435.5355 | Lower |
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
| MI (original) | 61.7767 | **83.2399 ✅** | 60.2151 | -21.4632 | Lower | +1.5616 | Higher | +23.0248 | Higher |
| MI (SEI) | 33.5303 | **84.1609 ✅** | 32.6453 | -50.6306 | Lower | +0.8850 | Higher | +51.5156 | Higher |
| Cyclomatic (sum) | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 17 | **1 ✅** | 17 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 1839.4060 | **520 ✅** | 1964.8028 | +1319.4060 | Higher | -125.3967 | Lower | -1444.8028 | Lower |
| Halstead Effort | 110364.3619 | **6066.6667 ✅** | 118706.8345 | +104297.6953 | Higher | -8342.4726 | Lower | -112640.1679 | Lower |
| SLOC | 64 | **29 ✅** | 69 | +35 | Higher | -5 | Lower | -40 | Lower |
| PLOC | 51 | **14 ✅** | 55 | +37 | Higher | -4 | Lower | -41 | Lower |
| LLOC | 45 | **10 ✅** | 53 | +35 | Higher | -8 | Lower | -43 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 5 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.2335 | **83.2157 ✅** | 80.6512 | -7.9822 | Lower | -5.4177 | Lower | +2.5645 | Higher |
| MI (SEI) | 60.4093 | **86.7340 ✅** | 68.7958 | -26.3247 | Lower | -8.3865 | Lower | +17.9382 | Higher |
| Cyclomatic (sum) | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 4 | 2 | 2 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 994.1233 | **571.5856 ✅** | 873.5091 | +422.5377 | Higher | +120.6143 | Higher | -301.9234 | Lower |
| Halstead Effort | 22036.4004 | **10912.0896 ✅** | 20964.2174 | +11124.3108 | Higher | +1072.1830 | Higher | -10052.1278 | Lower |
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
| MI (original) | 73.6911 | 68.1218 | **75.2868 ✅** | +5.5693 | Higher | -1.5957 | Lower | -7.1649 | Lower |
| MI (SEI) | 43.6023 | **55.2150 ✅** | 33.6277 | -11.6127 | Lower | +9.9746 | Higher | +21.5873 | Higher |
| Cyclomatic (sum) | 7 | 12 | 7 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 1085.9754 | 1036.7536 | **1025.2978 ✅** | +49.2218 | Higher | +60.6776 | Higher | +11.4558 | Higher |
| Halstead Effort | 32941.2529 | **30888.1073 ✅** | 36733.2640 | +2053.1456 | Higher | -3792.0111 | Lower | -5845.1567 | Lower |
| SLOC | 39 | 52 | **36 ✅** | -13 | Lower | +3 | Higher | +16 | Higher |
| PLOC | 32 | 43 | 32 | -11 | Lower | 0 | Same | +11 | Higher |
| LLOC | **27 ✅** | 30 | 28 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 1 | **10 ✅** | 0 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 7 | 7 | **4 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p03361

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.8798 ✅** | 61.3340 | 67.8957 | +9.5457 | Higher | +2.9840 | Higher | -6.5617 | Lower |
| MI (SEI) | 49.0660 | 43.3528 | **54.1657 ✅** | +5.7131 | Higher | -5.0997 | Lower | -10.8129 | Lower |
| Cyclomatic (sum) | **14 ✅** | 16 | 15 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 32 | **22 ✅** | 31 | +10 | Higher | +1 | Higher | -9 | Lower |
| Halstead Volume | **1483.5314 ✅** | 1762.4421 | 1487.8148 | -278.9107 | Lower | -4.2834 | Lower | +274.6273 | Higher |
| Halstead Effort | 57090.3807 | 57160.2846 | **55459.5780 ✅** | -69.9039 | Lower | +1630.8027 | Higher | +1700.7066 | Higher |
| SLOC | **38 ✅** | 63 | 45 | -25 | Lower | -7 | Lower | +18 | Higher |
| PLOC | **29 ✅** | 51 | 34 | -22 | Lower | -5 | Lower | +17 | Higher |
| LLOC | **21 ✅** | 31 | 25 | -10 | Lower | -4 | Lower | +6 | Higher |
| Comments (CLOC) | 3 | **10 ✅** | 8 | -7 | Lower | -5 | Lower | +2 | Higher |
| Blank Lines | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03365

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.0779 | **69.1580 ✅** | 62.2583 | -5.0801 | Lower | +1.8196 | Higher | +6.8997 | Higher |
| MI (SEI) | 18.0676 | **62.7882 ✅** | 15.4425 | -44.7205 | Lower | +2.6251 | Higher | +47.3456 | Higher |
| Cyclomatic (sum) | 13 | **9 ✅** | 13 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 2158.7884 | **1167.2572 ✅** | 2301.2528 | +991.5312 | Higher | -142.4644 | Lower | -1133.9956 | Lower |
| Halstead Effort | 133714.0446 | **49024.8008 ✅** | 146231.0768 | +84689.2437 | Higher | -12517.0322 | Lower | -97206.2760 | Lower |
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
| MI (original) | **85.7070 ✅** | 79.4072 | 85.0573 | +6.2998 | Higher | +0.6497 | Higher | -5.6501 | Lower |
| MI (SEI) | **84.9044 ✅** | 76.6064 | 78.8332 | +8.2980 | Higher | +6.0712 | Higher | -2.2268 | Lower |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | **491.5424 ✅** | 605.9969 | 492.8948 | -114.4545 | Lower | -1.3524 | Lower | +113.1021 | Higher |
| Halstead Effort | **6077.2516 ✅** | 7971.1901 | 7449.4325 | -1893.9386 | Lower | -1372.1810 | Lower | +521.7576 | Higher |
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
| MI (original) | 46.3452 | **54.2138 ✅** | 48.8653 | -7.8686 | Lower | -2.5201 | Lower | +5.3485 | Higher |
| MI (SEI) | 1.0039 | **38.3212 ✅** | -3.1667 | -37.3173 | Lower | +4.1706 | Higher | +41.4879 | Higher |
| Cyclomatic (sum) | 22 | **18 ✅** | 20 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 32 | **16 ✅** | 32 | +16 | Higher | 0 | Same | -16 | Lower |
| Halstead Volume | 5218.8898 | **2406 ✅** | 3620.1113 | +2812.8898 | Higher | +1598.7785 | Higher | -1214.1113 | Lower |
| Halstead Effort | 253226.7258 | **176294.1818 ✅** | 293746.1750 | +76932.5440 | Higher | -40519.4492 | Lower | -117451.9932 | Lower |
| SLOC | 103 | **86 ✅** | 102 | +17 | Higher | +1 | Higher | -16 | Lower |
| PLOC | 88 | **57 ✅** | 89 | +31 | Higher | -1 | Lower | -32 | Lower |
| LLOC | 63 | **44 ✅** | 66 | +19 | Higher | -3 | Lower | -22 | Lower |
| Comments (CLOC) | 1 | **20 ✅** | 0 | -19 | Lower | +1 | Higher | +20 | Higher |
| Blank Lines | 14 | 15 | **13 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p03408

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.9601 | **67.2292 ✅** | 66.2425 | -4.2691 | Lower | -3.2824 | Lower | +0.9867 | Higher |
| MI (SEI) | 43.4026 | **55.5105 ✅** | 43.2127 | -12.1079 | Lower | +0.1898 | Higher | +12.2977 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 10 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 15 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 1525.0452 | 1512.5535 | **1290.5855 ✅** | +12.4917 | Higher | +234.4597 | Higher | +221.9680 | Higher |
| Halstead Effort | 47553.6818 | **37010.2924 ✅** | 42331.2037 | +10543.3893 | Higher | +5222.4781 | Higher | -5320.9112 | Lower |
| SLOC | 65 | **53 ✅** | 56 | +12 | Higher | +9 | Higher | -3 | Lower |
| PLOC | 45 | **38 ✅** | 43 | +7 | Higher | +2 | Higher | -5 | Lower |
| LLOC | 37 | **28 ✅** | 35 | +9 | Higher | +2 | Higher | -7 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 5 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 11 | **7 ✅** | 8 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03414

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.7155 | **66.9358 ✅** | 21.8488 | -44.2203 | Lower | +0.8667 | Higher | +45.0870 | Higher |
| MI (SEI) | -37.2274 | **56.8988 ✅** | -29.9626 | -94.1262 | Lower | -7.2648 | Lower | +86.8614 | Higher |
| Cyclomatic (sum) | 56 | **13 ✅** | 55 | +43 | Higher | +1 | Higher | -42 | Lower |
| Cognitive (sum) | 64 | **11 ✅** | 65 | +53 | Higher | -1 | Lower | -54 | Lower |
| Halstead Volume | 9056.1351 | **1174.2237 ✅** | 9661.7943 | +7881.9114 | Higher | -605.6592 | Lower | -8487.5706 | Lower |
| Halstead Effort | 1182439.6018 | **37218.6974 ✅** | 1324582.0209 | +1145220.9044 | Higher | -142142.4190 | Lower | -1287363.3234 | Lower |
| SLOC | 229 | **53 ✅** | 240 | +176 | Higher | -11 | Lower | -187 | Lower |
| PLOC | 209 | **31 ✅** | 220 | +178 | Higher | -11 | Lower | -189 | Lower |
| LLOC | 159 | **18 ✅** | 163 | +141 | Higher | -4 | Lower | -145 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 3 | -13 | Lower | -3 | Lower | +10 | Higher |
| Blank Lines | 20 | **9 ✅** | 20 | +11 | Higher | 0 | Same | -11 | Lower |
| Functions | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 5 | **1 ✅** | 4 | +4 | Higher | +1 | Higher | -3 | Lower |
| Exits | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |

## p03434

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.8253 | **74.8884 ✅** | 57.2608 | -17.0631 | Lower | +0.5644 | Higher | +17.6276 | Higher |
| MI (SEI) | 21.6441 | **75.5960 ✅** | 8.2327 | -53.9519 | Lower | +13.4114 | Higher | +67.3633 | Higher |
| Cyclomatic (sum) | 13 | **6 ✅** | 13 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 14 | **5 ✅** | 14 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 2393.5869 | **539.1110 ✅** | 2455.3253 | +1854.4759 | Higher | -61.7383 | Lower | -1916.2142 | Lower |
| Halstead Effort | 126957.1439 | **12558.1157 ✅** | 137259.5023 | +114399.0282 | Higher | -10302.3584 | Lower | -124701.3866 | Lower |
| SLOC | 74 | **46 ✅** | 76 | +28 | Higher | -2 | Lower | -30 | Lower |
| PLOC | 60 | **25 ✅** | 62 | +35 | Higher | -2 | Lower | -37 | Lower |
| LLOC | 55 | **16 ✅** | 56 | +39 | Higher | -1 | Lower | -40 | Lower |
| Comments (CLOC) | 2 | **20 ✅** | 0 | -18 | Lower | +2 | Higher | +20 | Higher |
| Blank Lines | 14 | **8 ✅** | 14 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.8763 | 66.4408 | **67.1500 ✅** | -2.5645 | Lower | -3.2737 | Lower | -0.7092 | Lower |
| MI (SEI) | 41.6245 | **50.8293 ✅** | 43.7142 | -9.2048 | Lower | -2.0897 | Lower | +7.1151 | Higher |
| Cyclomatic (sum) | 14 | **11 ✅** | 13 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 11 | **8 ✅** | 21 | +3 | Higher | -10 | Lower | -13 | Lower |
| Halstead Volume | 2583.6837 | 1921.2271 | **1351.1399 ✅** | +662.4566 | Higher | +1232.5438 | Higher | +570.0872 | Higher |
| Halstead Effort | **55697.8181 ✅** | 108105.9711 | 67065.6726 | -52408.1530 | Lower | -11367.8545 | Lower | +41040.2985 | Higher |
| SLOC | 49 | **48 ✅** | 50 | +1 | Higher | -1 | Lower | -2 | Lower |
| PLOC | 36 | **33 ✅** | 40 | +3 | Higher | -4 | Lower | -7 | Lower |
| LLOC | 21 | **11 ✅** | 29 | +10 | Higher | -8 | Lower | -18 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 4 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 8 | 7 | **6 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
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
| MI (original) | 49.8869 | **79.9413 ✅** | 51.3774 | -30.0544 | Lower | -1.4905 | Lower | +28.5639 | Higher |
| MI (SEI) | 6.9157 | **82.1544 ✅** | 0.9666 | -75.2387 | Lower | +5.9491 | Higher | +81.1878 | Higher |
| Cyclomatic (sum) | 26 | **6 ✅** | 25 | +20 | Higher | +1 | Higher | -19 | Lower |
| Cognitive (sum) | 41 | **4 ✅** | 40 | +37 | Higher | +1 | Higher | -36 | Lower |
| Halstead Volume | 3041.7730 | **523.1869 ✅** | 2938.2003 | +2518.5862 | Higher | +103.5728 | Higher | -2415.0134 | Lower |
| Halstead Effort | 256093.8905 | **13951.6498 ✅** | 258561.6232 | +242142.2407 | Higher | -2467.7327 | Lower | -244609.9734 | Lower |
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
| MI (original) | 62.5885 | **75.9086 ✅** | 75.8779 | -13.3201 | Lower | -13.2894 | Lower | +0.0307 | Higher |
| MI (SEI) | 37.7585 | **75.2591 ✅** | 55.8996 | -37.5006 | Lower | -18.1411 | Lower | +19.3595 | Higher |
| Cyclomatic (sum) | 10 | 8 | **5 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 2466.1098 | **626.8336 ✅** | 999.7647 | +1839.2762 | Higher | +1466.3451 | Higher | -372.9311 | Lower |
| Halstead Effort | 56469.7351 | **16693.5681 ✅** | 25194.0708 | +39776.1670 | Higher | +31275.6643 | Higher | -8500.5027 | Lower |
| SLOC | 57 | 40 | **36 ✅** | +17 | Higher | +21 | Higher | +4 | Higher |
| PLOC | 38 | 25 | **24 ✅** | +13 | Higher | +14 | Higher | +1 | Higher |
| LLOC | 21 | **17 ✅** | 20 | +4 | Higher | +1 | Higher | -3 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 3 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 14 | **8 ✅** | 9 | +6 | Higher | +5 | Higher | -1 | Lower |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.2540 | **69.2638 ✅** | 57.2540 | -12.0098 | Lower | 0 | Same | +12.0098 | Higher |
| MI (SEI) | 8.1211 | **68.3749 ✅** | 8.1211 | -60.2539 | Lower | 0 | Same | +60.2539 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 12 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 11 | **6 ✅** | 11 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 2467.1867 | **798.0630 ✅** | 2467.1867 | +1669.1238 | Higher | 0 | Same | -1669.1238 | Lower |
| Halstead Effort | 125626.4812 | **20770.6391 ✅** | 125626.4812 | +104855.8421 | Higher | 0 | Same | -104855.8421 | Lower |
| SLOC | 77 | **55 ✅** | 77 | +22 | Higher | 0 | Same | -22 | Lower |
| PLOC | 61 | **27 ✅** | 61 | +34 | Higher | 0 | Same | -34 | Lower |
| LLOC | 56 | **12 ✅** | 56 | +44 | Higher | 0 | Same | -44 | Lower |
| Comments (CLOC) | 0 | **25 ✅** | 0 | -25 | Lower | 0 | Same | +25 | Higher |
| Blank Lines | 16 | **10 ✅** | 16 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03544

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.8221 | **82.2117 ✅** | 36.3684 | -5.3896 | Lower | +40.4537 | Higher | +45.8433 | Higher |
| MI (SEI) | 65.0937 | **86.6643 ✅** | -1.3488 | -21.5706 | Lower | +66.4425 | Higher | +88.0131 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 38 | -2 | Lower | -34 | Lower | -32 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 34 | -1 | Lower | -32 | Lower | -31 | Lower |
| Halstead Volume | 1537.8951 | **450.8319 ✅** | 4529.3893 | +1087.0633 | Higher | -2991.4942 | Lower | -4078.5574 | Lower |
| Halstead Effort | 27985.8939 | **9580.1771 ✅** | 414613.3262 | +18405.7168 | Higher | -386627.4322 | Lower | -405033.1491 | Lower |
| SLOC | **30 ✅** | 31 | 159 | -1 | Lower | -129 | Lower | -128 | Lower |
| PLOC | 22 | **21 ✅** | 139 | +1 | Higher | -117 | Lower | -118 | Lower |
| LLOC | **13 ✅** | 16 | 98 | -3 | Lower | -85 | Lower | -82 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 9 | -9 | Lower | -4 | Lower | +5 | Higher |
| Blank Lines | 5 | 5 | 18 | 0 | Same | -13 | Lower | -13 | Lower |
| Functions | **1 ✅** | 2 | 11 | -1 | Lower | -10 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 12 | -3 | Lower | -12 | Lower | -9 | Lower |

## p03562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.4781 | **73.5260 ✅** | 42.0534 | -31.0479 | Lower | +0.4246 | Higher | +31.4726 | Higher |
| MI (SEI) | -11.1597 | **64.3771 ✅** | -4.8843 | -75.5369 | Lower | -6.2755 | Lower | +69.2614 | Higher |
| Cyclomatic (sum) | 32 | **8 ✅** | 31 | +24 | Higher | +1 | Higher | -23 | Lower |
| Cognitive (sum) | 52 | **4 ✅** | 44 | +48 | Higher | +8 | Higher | -40 | Lower |
| Halstead Volume | 4742.7338 | **917.7708 ✅** | 4721.5318 | +3824.9630 | Higher | +21.2020 | Higher | -3803.7610 | Lower |
| Halstead Effort | 477039.9743 | **17835.3458 ✅** | 515486.3475 | +459204.6285 | Higher | -38446.3732 | Lower | -497651.0017 | Lower |
| SLOC | 117 | **41 ✅** | 122 | +76 | Higher | -5 | Lower | -81 | Lower |
| PLOC | 104 | **24 ✅** | 105 | +80 | Higher | -1 | Lower | -81 | Lower |
| LLOC | 83 | **16 ✅** | 86 | +67 | Higher | -3 | Lower | -70 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 1 | -9 | Lower | -1 | Lower | +8 | Higher |
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
| MI (original) | 50.0492 | 51.1274 | **53.8638 ✅** | -1.0782 | Lower | -3.8146 | Lower | -2.7363 | Lower |
| MI (SEI) | 9.3603 | **25.5958 ✅** | 3.8409 | -16.2355 | Lower | +5.5194 | Higher | +21.7549 | Higher |
| Cyclomatic (sum) | 19 | 21 | **18 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 25 | **20 ✅** | 30 | +5 | Higher | -5 | Lower | -10 | Lower |
| Halstead Volume | 3307.0798 | **2989.2572 ✅** | 3101.5194 | +317.8226 | Higher | +205.5604 | Higher | -112.2622 | Lower |
| Halstead Effort | **135050.3404 ✅** | 149313.3961 | 158858.3119 | -14263.0557 | Lower | -23807.9715 | Lower | -9544.9158 | Lower |
| SLOC | 99 | 93 | **81 ✅** | +6 | Higher | +18 | Higher | +12 | Higher |
| PLOC | 85 | 74 | **69 ✅** | +11 | Higher | +16 | Higher | +5 | Higher |
| LLOC | 52 | **44 ✅** | 54 | +8 | Higher | -2 | Lower | -10 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 0 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 14 | 14 | **12 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03626

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.2353 | **86.0752 ✅** | 71.1478 | -20.8399 | Lower | -5.9125 | Lower | +14.9274 | Higher |
| MI (SEI) | 43.4166 | **92.3300 ✅** | 27.8601 | -48.9135 | Lower | +15.5564 | Higher | +64.4699 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 15 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 1549.4044 | **497.5405 ✅** | 1195.9332 | +1051.8640 | Higher | +353.4712 | Higher | -698.3927 | Lower |
| Halstead Effort | 57577.8669 | **10339.5125 ✅** | 42479.5472 | +47238.3543 | Higher | +15098.3197 | Higher | -32140.0346 | Lower |
| SLOC | 57 | **24 ✅** | 43 | +33 | Higher | +14 | Higher | -19 | Lower |
| PLOC | 42 | **17 ✅** | 36 | +25 | Higher | +6 | Higher | -19 | Lower |
| LLOC | 40 | **10 ✅** | 34 | +30 | Higher | +6 | Higher | -24 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 0 | -5 | Lower | +6 | Higher | +11 | Higher |
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
| MI (original) | 68.5492 | **73.0357 ✅** | 60.0105 | -4.4865 | Lower | +8.5388 | Higher | +13.0253 | Higher |
| MI (SEI) | 55.3769 | **65.0410 ✅** | 43.0774 | -9.6641 | Lower | +12.2995 | Higher | +21.9636 | Higher |
| Cyclomatic (sum) | 10 | **7 ✅** | 11 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 15 | **7 ✅** | 16 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 983.1622 | **788.7516 ✅** | 2700.2073 | +194.4106 | Higher | -1717.0451 | Lower | -1911.4557 | Lower |
| Halstead Effort | 21483.9153 | **13145.8603 ✅** | 68801.2828 | +8338.0550 | Higher | -47317.3675 | Lower | -55655.4225 | Lower |
| SLOC | 53 | **45 ✅** | 64 | +8 | Higher | -11 | Lower | -19 | Lower |
| PLOC | 39 | **27 ✅** | 48 | +12 | Higher | -9 | Lower | -21 | Lower |
| LLOC | 29 | **17 ✅** | 30 | +12 | Higher | -1 | Lower | -13 | Lower |
| Comments (CLOC) | 10 | 11 | **12 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 9 | **7 ✅** | 10 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03701

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.1338 | 71.8293 | **75.0546 ✅** | -1.6955 | Lower | -4.9208 | Lower | -3.2253 | Lower |
| MI (SEI) | 55.3341 | **64.5070 ✅** | 50.6915 | -9.1729 | Lower | +4.6425 | Higher | +13.8155 | Higher |
| Cyclomatic (sum) | 8 | 10 | **7 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 11 | 13 | **9 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Halstead Volume | 949.5963 | **813.4553 ✅** | 905.9093 | +136.1410 | Higher | +43.6870 | Higher | -92.4540 | Lower |
| Halstead Effort | 25833.3366 | **21556.5657 ✅** | 28083.1891 | +4276.7708 | Higher | -2249.8526 | Lower | -6526.6234 | Lower |
| SLOC | 50 | 46 | **38 ✅** | +4 | Higher | +12 | Higher | +8 | Higher |
| PLOC | 34 | **29 ✅** | 30 | +5 | Higher | +4 | Higher | -1 | Lower |
| LLOC | 27 | **20 ✅** | 27 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 2 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 10 | 7 | 7 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03709

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.6931 | **75.9722 ✅** | 19.8883 | -38.2792 | Lower | +17.8048 | Higher | +56.0840 | Higher |
| MI (SEI) | -18.0630 | **72.2835 ✅** | -35.5000 | -90.3465 | Lower | +17.4370 | Higher | +107.7835 | Higher |
| Cyclomatic (sum) | 32 | **5 ✅** | 64 | +27 | Higher | -32 | Lower | -59 | Lower |
| Cognitive (sum) | 50 | **3 ✅** | 82 | +47 | Higher | -32 | Lower | -79 | Lower |
| Halstead Volume | 6233.4140 | **765.1033 ✅** | 9460.4673 | +5468.3107 | Higher | -3227.0533 | Lower | -8695.3639 | Lower |
| Halstead Effort | 644638.8976 | **14969.4130 ✅** | 1166152.2795 | +629669.4846 | Higher | -521513.3819 | Lower | -1151182.8666 | Lower |
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
| MI (original) | 57.8183 | **70.0430 ✅** | 58.3134 | -12.2247 | Lower | -0.4951 | Lower | +11.7297 | Higher |
| MI (SEI) | 8.8333 | **67.9335 ✅** | 9.5476 | -59.1002 | Lower | -0.7143 | Lower | +58.3859 | Higher |
| Cyclomatic (sum) | 11 | **8 ✅** | 11 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 2978.8315 | **760.3168 ✅** | 2960.4437 | +2218.5148 | Higher | +18.3878 | Higher | -2200.1269 | Lower |
| Halstead Effort | 170665.2513 | **24390.1612 ✅** | 163811.2179 | +146275.0901 | Higher | +6854.0334 | Higher | -139421.0567 | Lower |
| SLOC | 71 | **54 ✅** | 69 | +17 | Higher | +2 | Higher | -15 | Lower |
| PLOC | 58 | **26 ✅** | 57 | +32 | Higher | +1 | Higher | -31 | Lower |
| LLOC | 51 | **23 ✅** | 51 | +28 | Higher | 0 | Same | -28 | Lower |
| Comments (CLOC) | 0 | **22 ✅** | 0 | -22 | Lower | 0 | Same | +22 | Higher |
| Blank Lines | 13 | **8 ✅** | 12 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.2797 | **72.6557 ✅** | 69.6659 | -7.3760 | Lower | -4.3862 | Lower | +2.9898 | Higher |
| MI (SEI) | 45.6427 | **70.6722 ✅** | 57.4540 | -25.0296 | Lower | -11.8114 | Lower | +13.2182 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 7 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 4 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | 1717.0638 | **994.2714 ✅** | 1507.9324 | +722.7924 | Higher | +209.1314 | Higher | -513.6610 | Lower |
| Halstead Effort | 74443.0256 | **21589.8934 ✅** | 56366.5136 | +52853.1323 | Higher | +18076.5120 | Higher | -34776.6203 | Lower |
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
| MI (original) | 70.0977 | **70.1368 ✅** | 67.1185 | -0.0391 | Lower | +2.9793 | Higher | +3.0183 | Higher |
| MI (SEI) | 42.8263 | **61.2711 ✅** | 37.3402 | -18.4448 | Lower | +5.4861 | Higher | +23.9310 | Higher |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 1646.0927 | **1077.7608 ✅** | 1805.9714 | +568.3318 | Higher | -159.8788 | Lower | -728.2106 | Lower |
| Halstead Effort | 83127.6793 | **38109.6224 ✅** | 86453.5994 | +45018.0569 | Higher | -3325.9201 | Lower | -48343.9770 | Lower |
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
| MI (original) | 63.5972 | **73.0811 ✅** | 62.3244 | -9.4840 | Lower | +1.2727 | Higher | +10.7567 | Higher |
| MI (SEI) | 48.0593 | **67.4321 ✅** | 15.5379 | -19.3728 | Lower | +32.5214 | Higher | +51.8941 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 13 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 22 | **5 ✅** | 22 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 1584.7222 | **781.8948 ✅** | 1839.4060 | +802.8275 | Higher | -254.6838 | Lower | -1057.5113 | Lower |
| Halstead Effort | 99837.5012 | **14886.0731 ✅** | 136965.0030 | +84951.4281 | Higher | -37127.5018 | Lower | -122078.9299 | Lower |
| SLOC | 60 | **45 ✅** | 61 | +15 | Higher | -1 | Lower | -16 | Lower |
| PLOC | 46 | **25 ✅** | 53 | +21 | Higher | -7 | Lower | -28 | Lower |
| LLOC | 40 | **14 ✅** | 44 | +26 | Higher | -4 | Lower | -30 | Lower |
| Comments (CLOC) | 11 | **13 ✅** | 0 | -2 | Lower | +11 | Higher | +13 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

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
| MI (original) | 74.5657 | **80.7037 ✅** | 73.4533 | -6.1380 | Lower | +1.1124 | Higher | +7.2504 | Higher |
| MI (SEI) | 54.1391 | **79.7723 ✅** | 57.8282 | -25.6332 | Lower | -3.6891 | Lower | +21.9441 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 12 | **3 ✅** | 12 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 989.8786 | **629.7513 ✅** | 1128.2488 | +360.1273 | Higher | -138.3701 | Lower | -498.4975 | Lower |
| Halstead Effort | 23046.9567 | **12595.0260 ✅** | 30021.2278 | +10451.9307 | Higher | -6974.2711 | Lower | -17426.2017 | Lower |
| SLOC | 37 | **31 ✅** | 38 | +6 | Higher | -1 | Lower | -7 | Lower |
| PLOC | 31 | **19 ✅** | 32 | +12 | Higher | -1 | Lower | -13 | Lower |
| LLOC | 10 | **8 ✅** | 27 | +2 | Higher | -17 | Lower | -19 | Lower |
| Comments (CLOC) | 3 | **10 ✅** | 5 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03845

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.3278 | **76.3443 ✅** | 76.2264 | -8.0165 | Lower | -7.8986 | Lower | +0.1180 | Higher |
| MI (SEI) | 55.2602 | **79.1622 ✅** | 66.7151 | -23.9020 | Lower | -11.4549 | Lower | +12.4471 | Higher |
| Cyclomatic (sum) | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | 5 | **3 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1320.6623 | **802.9035 ✅** | 1020.7273 | +517.7588 | Higher | +299.9350 | Higher | -217.8238 | Lower |
| Halstead Effort | 36837.0462 | **18318.0948 ✅** | 23637.8958 | +18518.9514 | Higher | +13199.1504 | Higher | -5319.8010 | Lower |
| SLOC | 51 | 37 | **35 ✅** | +14 | Higher | +16 | Higher | +2 | Higher |
| PLOC | 34 | 21 | 21 | +13 | Higher | +13 | Higher | 0 | Same |
| LLOC | 21 | **16 ✅** | 18 | +5 | Higher | +3 | Higher | -2 | Lower |
| Comments (CLOC) | 10 | **18 ✅** | 7 | -8 | Lower | +3 | Higher | +11 | Higher |
| Blank Lines | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03865

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.4020 ✅** | 81.9830 | 76.7076 | +6.4191 | Higher | +11.6944 | Higher | +5.2753 | Higher |
| MI (SEI) | **85.2998 ✅** | 80.1404 | 66.1219 | +5.1593 | Higher | +19.1779 | Higher | +14.0186 | Higher |
| Cyclomatic (sum) | **4 ✅** | 5 | 7 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **379.5669 ✅** | 492.4112 | 1706.8955 | -112.8442 | Lower | -1327.3285 | Lower | -1214.4843 | Lower |
| Halstead Effort | **5018.7185 ✅** | 5088.2488 | 29377.9587 | -69.5303 | Lower | -24359.2403 | Lower | -24289.7100 | Lower |
| SLOC | **23 ✅** | 31 | 28 | -8 | Lower | -5 | Lower | +3 | Higher |
| PLOC | **14 ✅** | 19 | 20 | -5 | Lower | -6 | Lower | -1 | Lower |
| LLOC | 8 | 12 | **5 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |
| Comments (CLOC) | 5 | **9 ✅** | 5 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03899

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.1402 | **71.7221 ✅** | 62.3962 | -8.5819 | Lower | +0.7440 | Higher | +9.3260 | Higher |
| MI (SEI) | 36.2966 | **69.5290 ✅** | 15.4378 | -33.2324 | Lower | +20.8588 | Higher | +54.0912 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 11 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 2010 | **1089.0677 ✅** | 2086.7347 | +920.9323 | Higher | -76.7347 | Lower | -997.6670 | Lower |
| Halstead Effort | 85769.5714 | **40295.5044 ✅** | 88384.1966 | +45474.0670 | Higher | -2614.6251 | Lower | -48088.6921 | Lower |
| SLOC | 58 | **44 ✅** | 60 | +14 | Higher | -2 | Lower | -16 | Lower |
| PLOC | 45 | **30 ✅** | 49 | +15 | Higher | -4 | Lower | -19 | Lower |
| LLOC | 34 | **24 ✅** | 35 | +10 | Higher | -1 | Lower | -11 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 0 | -13 | Lower | +4 | Higher | +17 | Higher |
| Blank Lines | 9 | **6 ✅** | 11 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03901

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.9594 | **77.5472 ✅** | 43.3237 | -11.5878 | Lower | +22.6356 | Higher | +34.2235 | Higher |
| MI (SEI) | 51.9030 | **74.3302 ✅** | 15.8601 | -22.4272 | Lower | +36.0430 | Higher | +58.4702 | Higher |
| Cyclomatic (sum) | **5 ✅** | 9 | 14 | -4 | Lower | -9 | Lower | -5 | Lower |
| Cognitive (sum) | **1 ✅** | 4 | 11 | -3 | Lower | -10 | Lower | -7 | Lower |
| Halstead Volume | 2419.9619 | **726.0704 ✅** | 4388.6575 | +1693.8914 | Higher | -1968.6957 | Lower | -3662.5871 | Lower |
| Halstead Effort | 46044.6800 | **14661.0378 ✅** | 161993.0944 | +31383.6422 | Higher | -115948.4143 | Lower | -147332.0565 | Lower |
| SLOC | 50 | **34 ✅** | 147 | +16 | Higher | -97 | Lower | -113 | Lower |
| PLOC | 32 | **19 ✅** | 110 | +13 | Higher | -78 | Lower | -91 | Lower |
| LLOC | 15 | **9 ✅** | 70 | +6 | Higher | -55 | Lower | -61 | Lower |
| Comments (CLOC) | 10 | 10 | **21 ✅** | 0 | Same | -11 | Lower | -11 | Lower |
| Blank Lines | 10 | **7 ✅** | 19 | +3 | Higher | -9 | Lower | -12 | Lower |
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
| MI (original) | 42.4066 | **59.5905 ✅** | 37.4468 | -17.1839 | Lower | +4.9598 | Higher | +22.1437 | Higher |
| MI (SEI) | 16.3115 | **42.8707 ✅** | 12.4976 | -26.5592 | Lower | +3.8139 | Higher | +30.3731 | Higher |
| Cyclomatic (sum) | 22 | **12 ✅** | 26 | +10 | Higher | -4 | Lower | -14 | Lower |
| Cognitive (sum) | 24 | **8 ✅** | 26 | +16 | Higher | -2 | Lower | -18 | Lower |
| Halstead Volume | 4682.6247 | **1858.8287 ✅** | 5576.5018 | +2823.7960 | Higher | -893.8772 | Lower | -3717.6731 | Lower |
| Halstead Effort | 436108.4427 | **92941.4332 ✅** | 455350.2167 | +343167.0095 | Higher | -19241.7741 | Lower | -362408.7836 | Lower |
| SLOC | 136 | **73 ✅** | 165 | +63 | Higher | -29 | Lower | -92 | Lower |
| PLOC | 100 | **45 ✅** | 112 | +55 | Higher | -12 | Lower | -67 | Lower |
| LLOC | 65 | **26 ✅** | 72 | +39 | Higher | -7 | Lower | -46 | Lower |
| Comments (CLOC) | 21 | 14 | **32 ✅** | +7 | Higher | -11 | Lower | -18 | Lower |
| Blank Lines | 20 | **15 ✅** | 27 | +5 | Higher | -7 | Lower | -12 | Lower |
| Functions | 5 | **2 ✅** | 7 | +3 | Higher | -2 | Lower | -5 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |

## p03951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.0529 | **74.5884 ✅** | 73.3889 | -14.5355 | Lower | -13.3361 | Lower | +1.1994 | Higher |
| MI (SEI) | 41.8039 | **76.7933 ✅** | 48.4921 | -34.9894 | Lower | -6.6882 | Lower | +28.3013 | Higher |
| Cyclomatic (sum) | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 14 | +11 | Higher | 0 | Same | -11 | Lower |
| Halstead Volume | 2926 | **585.1497 ✅** | 1142.3054 | +2340.8503 | Higher | +1783.6946 | Higher | -557.1557 | Lower |
| Halstead Effort | 64068.3585 | **16676.7670 ✅** | 44386.7258 | +47391.5915 | Higher | +19681.6327 | Higher | -27709.9589 | Lower |
| SLOC | 64 | 45 | **38 ✅** | +19 | Higher | +26 | Higher | +7 | Higher |
| PLOC | 48 | **24 ✅** | 33 | +24 | Higher | +15 | Higher | -9 | Lower |
| LLOC | 39 | **12 ✅** | 31 | +27 | Higher | +8 | Higher | -19 | Lower |
| Comments (CLOC) | 11 | **22 ✅** | 2 | -11 | Lower | +9 | Higher | +20 | Higher |
| Blank Lines | 10 | 8 | **5 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
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
| MI (original) | 51.6812 | **73.8198 ✅** | 55.3834 | -22.1386 | Lower | -3.7023 | Lower | +18.4363 | Higher |
| MI (SEI) | 18.6340 | **73.7224 ✅** | 24.3993 | -55.0884 | Lower | -5.7653 | Lower | +49.3230 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 14 | +7 | Higher | +2 | Higher | -5 | Lower |
| Cognitive (sum) | 19 | **7 ✅** | 16 | +12 | Higher | +3 | Higher | -9 | Lower |
| Halstead Volume | 4126.6361 | **829.8336 ✅** | 2763.7081 | +3296.8025 | Higher | +1362.9280 | Higher | -1933.8745 | Lower |
| Halstead Effort | 108105.3615 | **24065.1746 ✅** | 138323.5919 | +84040.1869 | Higher | -30218.2304 | Lower | -114258.4173 | Lower |
| SLOC | 87 | **41 ✅** | 81 | +46 | Higher | +6 | Higher | -40 | Lower |
| PLOC | 71 | **24 ✅** | 65 | +47 | Higher | +6 | Higher | -41 | Lower |
| LLOC | 48 | **16 ✅** | 55 | +32 | Higher | -7 | Lower | -39 | Lower |
| Comments (CLOC) | 5 | **17 ✅** | 5 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 11 | **8 ✅** | 11 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |

## p04005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.0298 | **77.4090 ✅** | 74.9200 | -9.3792 | Lower | -6.8902 | Lower | +2.4891 | Higher |
| MI (SEI) | 35.4042 | **73.0253 ✅** | 62.4086 | -37.6210 | Lower | -27.0044 | Lower | +10.6166 | Higher |
| Cyclomatic (sum) | 11 | **6 ✅** | 8 | +5 | Higher | +3 | Higher | -2 | Lower |
| Cognitive (sum) | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 2145.5262 | **712.5395 ✅** | 966.4985 | +1432.9867 | Higher | +1179.0277 | Higher | -253.9590 | Lower |
| Halstead Effort | 44435.5303 | **11671.3969 ✅** | 29342.8933 | +32764.1335 | Higher | +15092.6371 | Higher | -17671.4964 | Lower |
| SLOC | 42 | **36 ✅** | 37 | +6 | Higher | +5 | Higher | -1 | Lower |
| PLOC | 34 | **22 ✅** | 26 | +12 | Higher | +8 | Higher | -4 | Lower |
| LLOC | 17 | **12 ✅** | 22 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 6 | -9 | Lower | -5 | Lower | +4 | Higher |
| Blank Lines | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p04013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.1349 | **70.8451 ✅** | 64.4397 | -5.7101 | Lower | +0.6953 | Higher | +6.4054 | Higher |
| MI (SEI) | 19.8981 | **61.8831 ✅** | 29.8707 | -41.9850 | Lower | -9.9725 | Lower | +32.0125 | Higher |
| Cyclomatic (sum) | 16 | **10 ✅** | 16 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 18 | **8 ✅** | 18 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 1979.6813 | **982.9534 ✅** | 2122.0899 | +996.7279 | Higher | -142.4086 | Lower | -1139.1365 | Lower |
| Halstead Effort | 118610.2159 | **41705.3085 ✅** | 128304.8218 | +76904.9073 | Higher | -9694.6059 | Lower | -86599.5132 | Lower |
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
| MI (original) | 75.3452 | 73.5422 | **80.3904 ✅** | +1.8030 | Higher | -5.0452 | Lower | -6.8482 | Lower |
| MI (SEI) | 63.2684 | **67.9426 ✅** | 65.1708 | -4.6742 | Lower | -1.9024 | Lower | +2.7718 | Higher |
| Cyclomatic (sum) | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 1013.8419 | 887.1242 | **878.6965 ✅** | +126.7178 | Higher | +135.1454 | Higher | +8.4277 | Higher |
| Halstead Effort | 34217.1648 | **25412.4107 ✅** | 32951.1186 | +8804.7541 | Higher | +1266.0462 | Higher | -7538.7079 | Lower |
| SLOC | 36 | 42 | **28 ✅** | -6 | Lower | +8 | Higher | +14 | Higher |
| PLOC | 23 | 24 | 23 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 21 | **19 ✅** | 22 | +2 | Higher | -1 | Lower | -3 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 3 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 7 | 9 | **4 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p04022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.9886 | **59.9601 ✅** | 4.3117 | -27.9715 | Lower | +27.6770 | Higher | +55.6484 | Higher |
| MI (SEI) | -25.6818 | **41.9150 ✅** | -52.8526 | -67.5969 | Lower | +27.1708 | Higher | +94.7677 | Higher |
| Cyclomatic (sum) | 38 | **17 ✅** | 88 | +21 | Higher | -50 | Lower | -71 | Lower |
| Cognitive (sum) | 86 | **27 ✅** | 101 | +59 | Higher | -15 | Lower | -74 | Lower |
| Halstead Volume | 5393.6254 | **1387.7925 ✅** | 12819.4966 | +4005.8329 | Higher | -7425.8712 | Lower | -11431.7040 | Lower |
| Halstead Effort | 486967.3220 | **43531.7865 ✅** | 1467227.6658 | +443435.5355 | Higher | -980260.3438 | Lower | -1423695.8793 | Lower |
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
| MI (original) | 41.8280 | **61.7156 ✅** | 40.7789 | -19.8877 | Lower | +1.0491 | Higher | +20.9367 | Higher |
| MI (SEI) | -4.9603 | **48.2186 ✅** | -13.1020 | -53.1789 | Lower | +8.1417 | Higher | +61.3206 | Higher |
| Cyclomatic (sum) | 34 | **12 ✅** | 37 | +22 | Higher | -3 | Lower | -25 | Lower |
| Cognitive (sum) | 45 | **10 ✅** | 51 | +35 | Higher | -6 | Lower | -41 | Lower |
| Halstead Volume | 4104.7452 | **1690.9918 ✅** | 4184.3214 | +2413.7534 | Higher | -79.5762 | Lower | -2493.3296 | Lower |
| Halstead Effort | 313713.7023 | **56789.1397 ✅** | 350246.7210 | +256924.5626 | Higher | -36533.0187 | Lower | -293457.5813 | Lower |
| SLOC | 124 | **66 ✅** | 126 | +58 | Higher | -2 | Lower | -60 | Lower |
| PLOC | 104 | **40 ✅** | 108 | +64 | Higher | -4 | Lower | -68 | Lower |
| LLOC | 79 | **29 ✅** | 81 | +50 | Higher | -2 | Lower | -52 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 0 | -14 | Lower | +1 | Higher | +15 | Higher |
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
| MI (original) | 60.3680 | **68.2438 ✅** | 59.3331 | -7.8758 | Lower | 1.0349 | Higher | 8.9107 | Higher |
| MI (SEI) | 31.6106 | **58.1434 ✅** | 27.0434 | -26.5328 | Lower | 4.5672 | Higher | 31.1000 | Higher |
| Cyclomatic (sum) | 16.5967 | **11.1200 ✅** | 18.0100 | 5.4767 | Higher | -1.4133 | Lower | -6.8900 | Lower |
| Cognitive (sum) | 20.9800 | **10.4533 ✅** | 22.9267 | 10.5267 | Higher | -1.9467 | Lower | -12.4733 | Lower |
| Halstead Volume | 2637.9885 | **1297.7024 ✅** | 2879.6454 | 1340.2861 | Higher | -241.6570 | Lower | -1581.9430 | Lower |
| Halstead Effort | 174527.7001 | **46763.8879 ✅** | 232636.3027 | 127763.8123 | Higher | -58108.6025 | Lower | -185872.4148 | Lower |
| SLOC | 75.7700 | **55.7933 ✅** | 83.8033 | 19.9767 | Higher | -8.0333 | Lower | -28.0100 | Lower |
| PLOC | 61.6000 | **37.4600 ✅** | 70.4467 | 24.1400 | Higher | -8.8467 | Lower | -32.9867 | Lower |
| LLOC | 43.7400 | **21.7333 ✅** | 53.2100 | 22.0067 | Higher | -9.4700 | Lower | -31.4767 | Lower |
| Comments (CLOC) | 4.4067 | **12.8733 ✅** | 3.3933 | -8.4667 | Lower | 1.0133 | Higher | 9.4800 | Higher |
| Blank Lines | 11.0833 | **8.9533 ✅** | 11.1533 | 2.1300 | Higher | -0.0700 | Lower | -2.2000 | Lower |
| Functions | 2.5533 | **2.3167 ✅** | 3.3500 | 0.2367 | Higher | -0.7967 | Lower | -1.0333 | Lower |
| Closures | 0.7200 | 0.5967 | **0.4567 ✅** | 0.1233 | Higher | 0.2633 | Higher | 0.1400 | Higher |
| Exits | 2.0667 | 2.0667 | 2.5267 | 0 | Same | -0.4600 | Lower | -0.4600 | Lower |

