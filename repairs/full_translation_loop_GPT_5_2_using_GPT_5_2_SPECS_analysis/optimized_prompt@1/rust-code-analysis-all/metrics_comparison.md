# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/code_translation_specs_ASE/r_code/full_translation_loop_all_metrics.json`

## p00004

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.0001 | **65.3644 ✅** | 64.7150 | -2.3644 | Lower | -1.7149 | Lower | +0.6494 | Higher |
| MI (SEI) | **45.5077 ✅** | 44.8114 | 41.8973 | +0.6963 | Higher | +3.6103 | Higher | +2.9141 | Higher |
| Cyclomatic (sum) | 22 | 22 | **15 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Cognitive (sum) | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1585.9236 | **1452.6968 ✅** | 1554.2445 | +133.2268 | Higher | +31.6790 | Higher | -101.5477 | Lower |
| Halstead Effort | 43740.7958 | 41912.9927 | **41613.6442 ✅** | +1827.8031 | Higher | +2127.1515 | Higher | +299.3485 | Higher |
| SLOC | 54 | **48 ✅** | 54 | +6 | Higher | 0 | Same | -6 | Lower |
| PLOC | 42 | **38 ✅** | 42 | +4 | Higher | 0 | Same | -4 | Lower |
| LLOC | 17 | **13 ✅** | 20 | +4 | Higher | -3 | Lower | -7 | Lower |
| Comments (CLOC) | **8 ✅** | 5 | 5 | +3 | Higher | +3 | Higher | 0 | Same |
| Blank Lines | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 5 | 5 | **0 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00010

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.0182 ✅** | 60.7720 | 65.2993 | +6.2462 | Higher | +1.7189 | Higher | -4.5273 | Lower |
| MI (SEI) | 45.0219 | **45.6155 ✅** | 44.1169 | -0.5936 | Lower | +0.9049 | Higher | +1.4985 | Higher |
| Cyclomatic (sum) | **5 ✅** | 9 | 7 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | **5 ✅** | 12 | 6 | -7 | Lower | -1 | Lower | +6 | Higher |
| Halstead Volume | **1974.1396 ✅** | 2315.1308 | 2097.4048 | -340.9912 | Lower | -123.2652 | Lower | +217.7260 | Higher |
| Halstead Effort | **68162.6533 ✅** | 83528.8658 | 79753.8160 | -15366.2126 | Lower | -11591.1627 | Lower | +3775.0498 | Higher |
| SLOC | **50 ✅** | 66 | 53 | -16 | Lower | -3 | Lower | +13 | Higher |
| PLOC | **35 ✅** | 41 | 38 | -6 | Lower | -3 | Lower | +3 | Higher |
| LLOC | 32 | 33 | 32 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 5 | **14 ✅** | 6 | -9 | Lower | -1 | Lower | +8 | Higher |
| Blank Lines | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.3134 ✅** | 77.3280 | 77.8860 | +3.9853 | Higher | +3.4274 | Higher | -0.5580 | Lower |
| MI (SEI) | **73.5169 ✅** | 69.4536 | 72.6377 | +4.0632 | Higher | +0.8791 | Higher | -3.1841 | Lower |
| Cyclomatic (sum) | **4 ✅** | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **4 ✅** | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | **585.4121 ✅** | 664.5142 | 709.7261 | -79.1021 | Lower | -124.3140 | Lower | -45.2119 | Lower |
| Halstead Effort | **9339.9838 ✅** | 11695.4504 | 13011.6454 | -2355.4665 | Lower | -3671.6616 | Lower | -1316.1950 | Lower |
| SLOC | **31 ✅** | 37 | 35 | -6 | Lower | -4 | Lower | +2 | Higher |
| PLOC | **19 ✅** | 23 | 22 | -4 | Lower | -3 | Lower | +1 | Higher |
| LLOC | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 6 | 8 | **9 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00015

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.5119 | **53.4062 ✅** | 50.6909 | -0.8943 | Lower | +1.8209 | Higher | +2.7152 | Higher |
| MI (SEI) | 27.9453 | **29.2617 ✅** | 23.5185 | -1.3164 | Lower | +4.4268 | Higher | +5.7431 | Higher |
| Cyclomatic (sum) | 22 | **21 ✅** | 23 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 34 | **30 ✅** | 36 | +4 | Higher | -2 | Lower | -6 | Lower |
| Halstead Volume | 2345.0064 | **2136.0520 ✅** | 2527.8161 | +208.9545 | Higher | -182.8096 | Lower | -391.7641 | Lower |
| Halstead Effort | 119595.3284 | **96834.3554 ✅** | 136558.2420 | +22760.9729 | Higher | -16962.9137 | Lower | -39723.8866 | Lower |
| SLOC | 91 | **90 ✅** | 98 | +1 | Higher | -7 | Lower | -8 | Lower |
| PLOC | 67 | **64 ✅** | 76 | +3 | Higher | -9 | Lower | -12 | Lower |
| LLOC | 45 | **41 ✅** | 56 | +4 | Higher | -11 | Lower | -15 | Lower |
| Comments (CLOC) | 11 | 11 | 10 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | **13 ✅** | 15 | 14 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00023

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.4386 ✅** | 67.7553 | 62.1239 | +0.6833 | Higher | +6.3146 | Higher | +5.6313 | Higher |
| MI (SEI) | 35.1542 | **46.3907 ✅** | 31.6280 | -11.2365 | Lower | +3.5262 | Higher | +14.7627 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 11 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 11 | **10 ✅** | 12 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 1526.2790 | **1500.3502 ✅** | 1888.8205 | +25.9288 | Higher | -362.5415 | Lower | -388.4702 | Lower |
| Halstead Effort | 40700.7724 | **35440.7050 ✅** | 63045.7639 | +5260.0674 | Higher | -22344.9915 | Lower | -27605.0589 | Lower |
| SLOC | **47 ✅** | 50 | 63 | -3 | Lower | -16 | Lower | -13 | Lower |
| PLOC | 38 | **35 ✅** | 49 | +3 | Higher | -11 | Lower | -14 | Lower |
| LLOC | 25 | **23 ✅** | 33 | +2 | Higher | -8 | Lower | -10 | Lower |
| Comments (CLOC) | 1 | **5 ✅** | 3 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | **8 ✅** | 11 | 11 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p00024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **82.2554 ✅** | 75.0697 | 79.5121 | +7.1858 | Higher | +2.7433 | Higher | -4.4424 | Lower |
| MI (SEI) | **84.2680 ✅** | 69.0634 | 74.0663 | +15.2046 | Higher | +10.2017 | Higher | -5.0029 | Lower |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **575.1841 ✅** | 778.8222 | 651.7803 | -203.6382 | Lower | -76.5962 | Lower | +127.0419 | Higher |
| Halstead Effort | **7661.4517 ✅** | 12016.1145 | 9008.5350 | -4354.6628 | Lower | -1347.0833 | Lower | +3007.5795 | Higher |
| SLOC | **29 ✅** | 41 | 33 | -12 | Lower | -4 | Lower | +8 | Higher |
| PLOC | **16 ✅** | 21 | 18 | -5 | Lower | -2 | Lower | +3 | Higher |
| LLOC | **9 ✅** | 14 | 12 | -5 | Lower | -3 | Lower | +2 | Higher |
| Comments (CLOC) | 11 | 11 | 8 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | **4 ✅** | 9 | 7 | -5 | Lower | -3 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.8796 ✅** | 58.8127 | 62.1031 | +9.0669 | Higher | +5.7765 | Higher | -3.2904 | Lower |
| MI (SEI) | **44.5712 ✅** | 37.7804 | 38.1986 | +6.7908 | Higher | +6.3726 | Higher | -0.4182 | Lower |
| Cyclomatic (sum) | **13 ✅** | 18 | 16 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | **17 ✅** | 26 | 21 | -9 | Lower | -4 | Lower | +5 | Higher |
| Halstead Volume | **1104 ✅** | 1521.8657 | 1379.1222 | -417.8657 | Lower | -275.1222 | Lower | +142.7435 | Higher |
| Halstead Effort | **31558.6286 ✅** | 64932.9370 | 54698.4214 | -33374.3084 | Lower | -23139.7928 | Lower | +10234.5156 | Higher |
| SLOC | **51 ✅** | 75 | 65 | -24 | Lower | -14 | Lower | +10 | Higher |
| PLOC | **38 ✅** | 54 | 46 | -16 | Lower | -8 | Lower | +8 | Higher |
| LLOC | **24 ✅** | 39 | 31 | -15 | Lower | -7 | Lower | +8 | Higher |
| Comments (CLOC) | 4 | **10 ✅** | 6 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | **10 ✅** | 13 | 13 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00071

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.9230 ✅** | 48.0575 | 57.0424 | +12.8655 | Higher | +3.8807 | Higher | -8.9849 | Lower |
| MI (SEI) | **40.0283 ✅** | 19.8009 | 36.0591 | +20.2274 | Higher | +3.9692 | Higher | -16.2582 | Lower |
| Cyclomatic (sum) | **14 ✅** | 24 | 18 | -10 | Lower | -4 | Lower | +6 | Higher |
| Cognitive (sum) | **18 ✅** | 59 | 19 | -41 | Lower | -1 | Lower | +40 | Higher |
| Halstead Volume | **2190.3234 ✅** | 2964.9260 | 2652.0164 | -774.6026 | Lower | -461.6930 | Lower | +312.9096 | Higher |
| Halstead Effort | **65285.0467 ✅** | 156625.4368 | 99233.6320 | -91340.3900 | Lower | -33948.5852 | Lower | +57391.8048 | Higher |
| SLOC | **62 ✅** | 108 | 70 | -46 | Lower | -8 | Lower | +38 | Higher |
| PLOC | **45 ✅** | 87 | 51 | -42 | Lower | -6 | Lower | +36 | Higher |
| LLOC | **33 ✅** | 59 | 36 | -26 | Lower | -3 | Lower | +23 | Higher |
| Comments (CLOC) | 8 | **11 ✅** | 10 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.0297 | 61.6269 | **67.6342 ✅** | +2.4028 | Higher | -3.6044 | Lower | -6.0073 | Lower |
| MI (SEI) | **44.3382 ✅** | 38.4913 | 41.2753 | +5.8469 | Higher | +3.0629 | Higher | -2.7840 | Lower |
| Cyclomatic (sum) | 18 | 19 | **12 ✅** | -1 | Lower | +6 | Higher | +7 | Higher |
| Cognitive (sum) | 11 | 11 | **5 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| Halstead Volume | 1312.1144 | 1613.1606 | **1138.6907 ✅** | -301.0462 | Lower | +173.4236 | Higher | +474.4698 | Higher |
| Halstead Effort | 46376.4555 | 57520.6964 | **34069.6262 ✅** | -11144.2409 | Lower | +12306.8293 | Higher | +23451.0702 | Higher |
| SLOC | 57 | 61 | **52 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| PLOC | 47 | 47 | **41 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| LLOC | 15 | 21 | 15 | -6 | Lower | 0 | Same | +6 | Higher |
| Comments (CLOC) | **7 ✅** | 6 | 3 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | **7 ✅** | 10 | 8 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00077

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.6801 ✅** | 68.1957 | 68.2237 | +1.4844 | Higher | +1.4563 | Higher | -0.0281 | Lower |
| MI (SEI) | **60.9831 ✅** | 51.3522 | 59.8228 | +9.6309 | Higher | +1.1603 | Higher | -8.4706 | Lower |
| Cyclomatic (sum) | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 22 | **17 ✅** | 23 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | **932.0418 ✅** | 1024.3700 | 974.7752 | -92.3281 | Lower | -42.7334 | Lower | +49.5948 | Higher |
| Halstead Effort | **23162.9656 ✅** | 24320.5259 | 26736.6910 | -1157.5603 | Lower | -3573.7254 | Lower | -2416.1651 | Lower |
| SLOC | **51 ✅** | 55 | 55 | -4 | Lower | -4 | Lower | 0 | Same |
| PLOC | **32 ✅** | 35 | 33 | -3 | Lower | -1 | Lower | +2 | Higher |
| LLOC | **24 ✅** | 25 | 25 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 13 | 8 | **15 ✅** | +5 | Higher | -2 | Lower | -7 | Lower |
| Blank Lines | 8 | 12 | **7 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.8248 ✅** | 71.4442 | 70.9214 | +3.3807 | Higher | +3.9034 | Higher | +0.5228 | Higher |
| MI (SEI) | 63.2457 | **64.3717 ✅** | 60.9244 | -1.1259 | Lower | +2.3213 | Higher | +3.4473 | Higher |
| Cyclomatic (sum) | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | **772.0879 ✅** | 801.9089 | 869.1539 | -29.8210 | Lower | -97.0660 | Lower | -67.2450 | Lower |
| Halstead Effort | 16985.9327 | **16923.0422 ✅** | 18779.9315 | +62.8905 | Higher | -1793.9988 | Lower | -1856.8892 | Lower |
| SLOC | **40 ✅** | 48 | 49 | -8 | Lower | -9 | Lower | -1 | Lower |
| PLOC | **26 ✅** | 28 | 28 | -2 | Lower | -2 | Lower | 0 | Same |
| LLOC | **17 ✅** | 18 | 19 | -1 | Lower | -2 | Lower | -1 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 11 | -6 | Lower | -4 | Lower | +2 | Higher |
| Blank Lines | 9 | **7 ✅** | 10 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.3818 ✅** | 62.7641 | 45.7214 | +7.6177 | Higher | +24.6604 | Higher | +17.0426 | Higher |
| MI (SEI) | **53.2933 ✅** | 44.0083 | 16.1554 | +9.2849 | Higher | +37.1378 | Higher | +27.8529 | Higher |
| Cyclomatic (sum) | **9 ✅** | 15 | 25 | -6 | Lower | -16 | Lower | -10 | Lower |
| Cognitive (sum) | **10 ✅** | 15 | 35 | -5 | Lower | -25 | Lower | -20 | Lower |
| Halstead Volume | **1123.1402 ✅** | 1399.2361 | 3040.8474 | -276.0959 | Lower | -1917.7072 | Lower | -1641.6113 | Lower |
| Halstead Effort | **23187.4109 ✅** | 33007.6216 | 134772.6524 | -9820.2107 | Lower | -111585.2414 | Lower | -101765.0308 | Lower |
| SLOC | **46 ✅** | 63 | 122 | -17 | Lower | -76 | Lower | -59 | Lower |
| PLOC | **33 ✅** | 45 | 94 | -12 | Lower | -61 | Lower | -49 | Lower |
| LLOC | **21 ✅** | 28 | 57 | -7 | Lower | -36 | Lower | -29 | Lower |
| Comments (CLOC) | 6 | 9 | **12 ✅** | -3 | Lower | -6 | Lower | -3 | Lower |
| Blank Lines | **7 ✅** | 9 | 16 | -2 | Lower | -9 | Lower | -7 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.6038 | **57.4903 ✅** | 55.6538 | -4.8865 | Lower | -3.0500 | Lower | +1.8364 | Higher |
| MI (SEI) | 31.3023 | **37.7214 ✅** | 27.4912 | -6.4191 | Lower | +3.8111 | Higher | +10.2302 | Higher |
| Cyclomatic (sum) | 22 | 19 | **16 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| Cognitive (sum) | 25 | **18 ✅** | 25 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 2014.9502 | **1661.6838 ✅** | 1922.2345 | +353.2664 | Higher | +92.7157 | Higher | -260.5507 | Lower |
| Halstead Effort | 74970.8910 | **50834.4063 ✅** | 91461.1567 | +24136.4847 | Higher | -16490.2657 | Lower | -40626.7504 | Lower |
| SLOC | 95 | **78 ✅** | 87 | +17 | Higher | +8 | Higher | -9 | Lower |
| PLOC | 68 | **60 ✅** | 66 | +8 | Higher | +2 | Higher | -6 | Lower |
| LLOC | 42 | **39 ✅** | 56 | +3 | Higher | -14 | Lower | -17 | Lower |
| Comments (CLOC) | **15 ✅** | 12 | 7 | +3 | Higher | +8 | Higher | +5 | Higher |
| Blank Lines | 14 | **10 ✅** | 15 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p00118

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.0005 | **54.4328 ✅** | 45.6576 | -7.4322 | Lower | +1.3429 | Higher | +8.7751 | Higher |
| MI (SEI) | 21.3243 | **22.5696 ✅** | 18.8226 | -1.2452 | Lower | +2.5018 | Higher | +3.7470 | Higher |
| Cyclomatic (sum) | 29 | **23 ✅** | 30 | +6 | Higher | -1 | Lower | -7 | Lower |
| Cognitive (sum) | 32 | **18 ✅** | 38 | +14 | Higher | -6 | Lower | -20 | Lower |
| Halstead Volume | 3480 | **2717.7811 ✅** | 3607.3983 | +762.2189 | Higher | -127.3983 | Lower | -889.6172 | Lower |
| Halstead Effort | 292037.8378 | **154536.0525 ✅** | 287389.3995 | +137501.7854 | Higher | +4648.4383 | Higher | -132853.3470 | Lower |
| SLOC | 102 | **76 ✅** | 108 | +26 | Higher | -6 | Lower | -32 | Lower |
| PLOC | 76 | **62 ✅** | 83 | +14 | Higher | -7 | Lower | -21 | Lower |
| LLOC | 56 | **40 ✅** | 63 | +16 | Higher | -7 | Lower | -23 | Lower |
| Comments (CLOC) | 13 | 4 | 13 | +9 | Higher | 0 | Same | -9 | Lower |
| Blank Lines | 14 | **11 ✅** | 13 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00124

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.0279 | **66.1539 ✅** | 62.7857 | -0.1260 | Lower | +3.2422 | Higher | +3.3681 | Higher |
| MI (SEI) | **47.6803 ✅** | 38.9753 | 44.3566 | +8.7051 | Higher | +3.3238 | Higher | -5.3813 | Lower |
| Cyclomatic (sum) | **9 ✅** | 12 | 11 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **5 ✅** | 7 | 11 | -2 | Lower | -6 | Lower | -4 | Lower |
| Halstead Volume | **1260.1960 ✅** | 1426.4770 | 1438.7219 | -166.2810 | Lower | -178.5259 | Lower | -12.2449 | Lower |
| Halstead Effort | **27667.0308 ✅** | 33052.5161 | 38081.1696 | -5385.4854 | Lower | -10414.1389 | Lower | -5028.6535 | Lower |
| SLOC | 58 | **53 ✅** | 66 | +5 | Higher | -8 | Lower | -13 | Lower |
| PLOC | **39 ✅** | 43 | 46 | -4 | Lower | -7 | Lower | -3 | Lower |
| LLOC | 26 | **24 ✅** | 30 | +2 | Higher | -4 | Lower | -6 | Lower |
| Comments (CLOC) | 8 | 3 | **10 ✅** | +5 | Higher | -2 | Lower | -7 | Lower |
| Blank Lines | 12 | **8 ✅** | 12 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00136

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.6201 | **70.6656 ✅** | 60.4394 | -10.0455 | Lower | +0.1807 | Higher | +10.2262 | Higher |
| MI (SEI) | 31.4269 | **46.7372 ✅** | 31.4287 | -15.3103 | Lower | -0.0018 | Lower | +15.3085 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 12 | 12 | **9 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 1819.8617 | **1089.5845 ✅** | 2067.8673 | +730.2772 | Higher | -248.0056 | Lower | -978.2828 | Lower |
| Halstead Effort | 54595.8505 | **18924.3618 ✅** | 68001.0205 | +35671.4888 | Higher | -13405.1700 | Lower | -49076.6588 | Lower |
| SLOC | 68 | **45 ✅** | 66 | +23 | Higher | +2 | Higher | -21 | Lower |
| PLOC | 51 | **35 ✅** | 49 | +16 | Higher | +2 | Higher | -14 | Lower |
| LLOC | 46 | **18 ✅** | 45 | +28 | Higher | +1 | Higher | -27 | Lower |
| Comments (CLOC) | 4 | 3 | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | 13 | **7 ✅** | 13 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00137

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.3107 ✅** | 72.7294 | 75.6800 | +6.5813 | Higher | +3.6307 | Higher | -2.9506 | Lower |
| MI (SEI) | **65.2667 ✅** | 59.2973 | 58.1796 | +5.9694 | Higher | +7.0871 | Higher | +1.1177 | Higher |
| Cyclomatic (sum) | **5 ✅** | 8 | 6 | -3 | Lower | -1 | Lower | +2 | Higher |
| Cognitive (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **745.6809 ✅** | 922.1952 | 839.5814 | -176.5143 | Lower | -93.9006 | Lower | +82.6138 | Higher |
| Halstead Effort | **14093.3683 ✅** | 18735.9323 | 16468.7124 | -4642.5640 | Lower | -2375.3441 | Lower | +2267.2199 | Higher |
| SLOC | **32 ✅** | 43 | 38 | -11 | Lower | -6 | Lower | +5 | Higher |
| PLOC | **22 ✅** | 27 | 25 | -5 | Lower | -3 | Lower | +2 | Higher |
| LLOC | 19 | **16 ✅** | 22 | +3 | Higher | -3 | Lower | -6 | Lower |
| Comments (CLOC) | 4 | **7 ✅** | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | **6 ✅** | 9 | 9 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00146

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.5854 | 41.3919 | **48.1460 ✅** | +6.1935 | Higher | -0.5606 | Lower | -6.7541 | Lower |
| MI (SEI) | **23.5901 ✅** | 10.9669 | 22.6971 | +12.6232 | Higher | +0.8930 | Higher | -11.7303 | Lower |
| Cyclomatic (sum) | **20 ✅** | 23 | 21 | -3 | Lower | -1 | Lower | +2 | Higher |
| Cognitive (sum) | 37 | 36 | **33 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Halstead Volume | 3659.7622 | 4459.8959 | **3633.8201 ✅** | -800.1336 | Lower | +25.9421 | Higher | +826.0757 | Higher |
| Halstead Effort | 235329.6162 | 240206.2216 | **234280.4590 ✅** | -4876.6054 | Lower | +1049.1572 | Higher | +5925.7626 | Higher |
| SLOC | 110 | 145 | **105 ✅** | -35 | Lower | +5 | Higher | +40 | Higher |
| PLOC | 82 | 109 | **77 ✅** | -27 | Lower | +5 | Higher | +32 | Higher |
| LLOC | 70 | 84 | **65 ✅** | -14 | Lower | +5 | Higher | +19 | Higher |
| Comments (CLOC) | **17 ✅** | 16 | 14 | +1 | Higher | +3 | Higher | +2 | Higher |
| Blank Lines | **11 ✅** | 20 | 14 | -9 | Lower | -3 | Lower | +6 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p00156

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.3871 | **47.8489 ✅** | 32.4529 | -13.4618 | Lower | +1.9342 | Higher | +15.3960 | Higher |
| MI (SEI) | -2.6027 | **7.1107 ✅** | -7.3428 | -9.7134 | Lower | +4.7401 | Higher | +14.4535 | Higher |
| Cyclomatic (sum) | 40 | **27 ✅** | 43 | +13 | Higher | -3 | Lower | -16 | Lower |
| Cognitive (sum) | 46 | **36 ✅** | 53 | +10 | Higher | -7 | Lower | -17 | Lower |
| Halstead Volume | 5306.4388 | **3776.9993 ✅** | 5617.8136 | +1529.4395 | Higher | -311.3748 | Lower | -1840.8143 | Lower |
| Halstead Effort | 382063.5930 | **216128.2945 ✅** | 436171.7938 | +165935.2985 | Higher | -54108.2008 | Lower | -220043.4993 | Lower |
| SLOC | 166 | **97 ✅** | 176 | +69 | Higher | -10 | Lower | -79 | Lower |
| PLOC | 135 | **80 ✅** | 145 | +55 | Higher | -10 | Lower | -65 | Lower |
| LLOC | 108 | **63 ✅** | 120 | +45 | Higher | -12 | Lower | -57 | Lower |
| Comments (CLOC) | **11 ✅** | 2 | 9 | +9 | Higher | +2 | Higher | -7 | Lower |
| Blank Lines | 21 | **15 ✅** | 22 | +6 | Higher | -1 | Lower | -7 | Lower |
| Functions | 7 | **1 ✅** | 5 | +6 | Higher | +2 | Higher | -4 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 7 | **0 ✅** | 5 | +7 | Higher | +2 | Higher | -5 | Lower |

## p00167

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.5574 | 64.4179 | **68.9562 ✅** | +0.1395 | Higher | -4.3989 | Lower | -4.5383 | Lower |
| MI (SEI) | 49.4834 | 46.0703 | **51.0410 ✅** | +3.4131 | Higher | -1.5576 | Lower | -4.9707 | Lower |
| Cyclomatic (sum) | 18 | 16 | **12 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Cognitive (sum) | 18 | 14 | **12 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Halstead Volume | 1185.5068 | 1260.1960 | **1133.1465 ✅** | -74.6892 | Lower | +52.3603 | Higher | +127.0495 | Higher |
| Halstead Effort | 35258.6082 | 36388.1600 | **34541.4314 ✅** | -1129.5519 | Lower | +717.1768 | Higher | +1846.7287 | Higher |
| SLOC | 57 | 58 | **48 ✅** | -1 | Lower | +9 | Higher | +10 | Higher |
| PLOC | 44 | 44 | **37 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| LLOC | **23 ✅** | 25 | 27 | -2 | Lower | -4 | Lower | -2 | Lower |
| Comments (CLOC) | **10 ✅** | 8 | 6 | +2 | Higher | +4 | Higher | +2 | Higher |
| Blank Lines | 8 | 11 | **7 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 7 | 2 | **1 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |

## p00187

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **43.6803 ✅** | 38.8748 | 38.2540 | +4.8055 | Higher | +5.4264 | Higher | +0.6208 | Higher |
| MI (SEI) | **10.5123 ✅** | -0.3472 | -4.3415 | +10.8596 | Higher | +14.8538 | Higher | +3.9942 | Higher |
| Cyclomatic (sum) | **21 ✅** | 24 | 33 | -3 | Lower | -12 | Lower | -9 | Lower |
| Cognitive (sum) | 22 | 22 | 28 | 0 | Same | -6 | Lower | -6 | Lower |
| Halstead Volume | **4741.9310 ✅** | 4808.6099 | 5471.4589 | -66.6789 | Lower | -729.5279 | Lower | -662.8489 | Lower |
| Halstead Effort | 263551.5349 | **187484.0826 ✅** | 384628.7713 | +76067.4524 | Higher | -121077.2364 | Lower | -197144.6887 | Lower |
| SLOC | **127 ✅** | 163 | 143 | -36 | Lower | -16 | Lower | +20 | Higher |
| PLOC | **99 ✅** | 134 | 122 | -35 | Lower | -23 | Lower | +12 | Higher |
| LLOC | 77 | **56 ✅** | 81 | +21 | Higher | -4 | Lower | -25 | Lower |
| Comments (CLOC) | **10 ✅** | 8 | 4 | +2 | Higher | +6 | Higher | +4 | Higher |
| Blank Lines | 18 | 21 | **17 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 7 | 2 | -5 | Lower | 0 | Same | +5 | Higher |

## p00190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 28.1362 | **36.2507 ✅** | 28.9344 | -8.1145 | Lower | -0.7982 | Lower | +7.3163 | Higher |
| MI (SEI) | -7.0997 | **9.2350 ✅** | -9.0331 | -16.3347 | Lower | +1.9333 | Higher | +18.2681 | Higher |
| Cyclomatic (sum) | 39 | **31 ✅** | 36 | +8 | Higher | +3 | Higher | -5 | Lower |
| Cognitive (sum) | 64 | **36 ✅** | 61 | +28 | Higher | +3 | Higher | -25 | Lower |
| Halstead Volume | 6772.8455 | **5221.6470 ✅** | 7105.6187 | +1551.1985 | Higher | -332.7731 | Lower | -1883.9717 | Lower |
| Halstead Effort | 465780.3659 | **279558.9460 ✅** | 605506.6444 | +186221.4199 | Higher | -139726.2785 | Lower | -325947.6984 | Lower |
| SLOC | 229 | **169 ✅** | 224 | +60 | Higher | +5 | Higher | -55 | Lower |
| PLOC | 190 | **131 ✅** | 195 | +59 | Higher | -5 | Lower | -64 | Lower |
| LLOC | 119 | **72 ✅** | 124 | +47 | Higher | -5 | Lower | -52 | Lower |
| Comments (CLOC) | 24 | **28 ✅** | 18 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 29 | **23 ✅** | 26 | +6 | Higher | +3 | Higher | -3 | Lower |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 11 | **5 ✅** | 12 | +6 | Higher | -1 | Lower | -7 | Lower |

## p00203

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.3326 | **49.6938 ✅** | 30.1535 | -0.3613 | Lower | +19.1791 | Higher | +19.5404 | Higher |
| MI (SEI) | **25.1483 ✅** | 23.5431 | -2.9473 | +1.6053 | Higher | +28.0956 | Higher | +26.4904 | Higher |
| Cyclomatic (sum) | **26 ✅** | 27 | 49 | -1 | Lower | -23 | Lower | -22 | Lower |
| Cognitive (sum) | 66 | **60 ✅** | 83 | +6 | Higher | -17 | Lower | -23 | Lower |
| Halstead Volume | **2461.7354 ✅** | 2565.5803 | 5550.5267 | -103.8449 | Lower | -3088.7913 | Lower | -2984.9464 | Lower |
| Halstead Effort | **143916.8362 ✅** | 169530.8438 | 477837.6817 | -25614.0076 | Lower | -333920.8455 | Lower | -308306.8379 | Lower |
| SLOC | 103 | **98 ✅** | 187 | +5 | Higher | -84 | Lower | -89 | Lower |
| PLOC | 79 | **77 ✅** | 149 | +2 | Higher | -70 | Lower | -72 | Lower |
| LLOC | **56 ✅** | 58 | 95 | -2 | Lower | -39 | Lower | -37 | Lower |
| Comments (CLOC) | 14 | 11 | **20 ✅** | +3 | Higher | -6 | Lower | -9 | Lower |
| Blank Lines | 11 | **10 ✅** | 20 | +1 | Higher | -9 | Lower | -10 | Lower |
| Functions | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 6 | 0 | Same | -6 | Lower | -6 | Lower |

## p00208

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.2240 ✅** | 63.6960 | 61.5501 | +5.5280 | Higher | +7.6739 | Higher | +2.1459 | Higher |
| MI (SEI) | **58.7957 ✅** | 48.9073 | 40.1303 | +9.8884 | Higher | +18.6654 | Higher | +8.7770 | Higher |
| Cyclomatic (sum) | **10 ✅** | 14 | 15 | -4 | Lower | -5 | Lower | -1 | Lower |
| Cognitive (sum) | 17 | 17 | 30 | 0 | Same | -13 | Lower | -13 | Lower |
| Halstead Volume | **863.5100 ✅** | 1222.5407 | 1458.7781 | -359.0307 | Lower | -595.2681 | Lower | -236.2374 | Lower |
| Halstead Effort | **21122.7830 ✅** | 34231.1386 | 62226.0032 | -13108.3556 | Lower | -41103.2202 | Lower | -27994.8645 | Lower |
| SLOC | **53 ✅** | 63 | 67 | -10 | Lower | -14 | Lower | -4 | Lower |
| PLOC | **33 ✅** | 41 | 51 | -8 | Lower | -18 | Lower | -10 | Lower |
| LLOC | **25 ✅** | 27 | 43 | -2 | Lower | -18 | Lower | -16 | Lower |
| Comments (CLOC) | 12 | 12 | 8 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00244

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.3087 | **51.6320 ✅** | 28.4631 | -5.3233 | Lower | +17.8457 | Higher | +23.1689 | Higher |
| MI (SEI) | 22.3653 | **24.8409 ✅** | -20.0706 | -2.4755 | Lower | +42.4359 | Higher | +44.9114 | Higher |
| Cyclomatic (sum) | 21 | **17 ✅** | 41 | +4 | Higher | -20 | Lower | -24 | Lower |
| Cognitive (sum) | 24 | **21 ✅** | 45 | +3 | Higher | -21 | Lower | -24 | Lower |
| Halstead Volume | 3596.5373 | **3237.8667 ✅** | 6690.5956 | +358.6706 | Higher | -3094.0582 | Lower | -3452.7288 | Lower |
| Halstead Effort | 173532.9269 | **140764.8835 ✅** | 496713.2022 | +32768.0435 | Higher | -323180.2752 | Lower | -355948.3187 | Lower |
| SLOC | 118 | **93 ✅** | 219 | +25 | Higher | -101 | Lower | -126 | Lower |
| PLOC | 82 | **71 ✅** | 185 | +11 | Higher | -103 | Lower | -114 | Lower |
| LLOC | 58 | **53 ✅** | 123 | +5 | Higher | -65 | Lower | -70 | Lower |
| Comments (CLOC) | **19 ✅** | 10 | 4 | +9 | Higher | +15 | Higher | +6 | Higher |
| Blank Lines | 17 | **12 ✅** | 30 | +5 | Higher | -13 | Lower | -18 | Lower |
| Functions | 2 | **1 ✅** | 10 | +1 | Higher | -8 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 8 | +1 | Higher | -7 | Lower | -8 | Lower |

## p00252

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.8230 | 79.1900 | **80.2598 ✅** | +0.6330 | Higher | -0.4367 | Lower | -1.0698 | Lower |
| MI (SEI) | 62.4049 | **67.6041 ✅** | 57.3696 | -5.1992 | Lower | +5.0353 | Higher | +10.2345 | Higher |
| Cyclomatic (sum) | 17 | **14 ✅** | 17 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 857.5470 | **777.0015 ✅** | 895.3935 | +80.5455 | Higher | -37.8465 | Lower | -118.3920 | Lower |
| Halstead Effort | 21081.3638 | **15817.5306 ✅** | 22973.9111 | +5263.8332 | Higher | -1892.5474 | Lower | -7156.3806 | Lower |
| SLOC | 25 | 28 | **24 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| PLOC | 20 | 21 | 20 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 12 | **8 ✅** | 12 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 2 | **4 ✅** | 1 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **45.0190 ✅** | 37.0522 | 41.7448 | +7.9668 | Higher | +3.2742 | Higher | -4.6926 | Lower |
| MI (SEI) | **20.2225 ✅** | 12.0076 | 6.2373 | +8.2149 | Higher | +13.9852 | Higher | +5.7703 | Higher |
| Cyclomatic (sum) | **20 ✅** | 36 | 24 | -16 | Lower | -4 | Lower | +12 | Higher |
| Cognitive (sum) | 42 | **24 ✅** | 55 | +18 | Higher | -13 | Lower | -31 | Lower |
| Halstead Volume | **3926.9481 ✅** | 5907.6579 | 4651.4578 | -1980.7098 | Lower | -724.5096 | Lower | +1256.2001 | Higher |
| Halstead Effort | **237362.1988 ✅** | 400604.0467 | 330770.3304 | -163241.8480 | Lower | -93408.1316 | Lower | +69833.7164 | Higher |
| SLOC | **126 ✅** | 144 | 138 | -18 | Lower | -12 | Lower | +6 | Higher |
| PLOC | **88 ✅** | 111 | 111 | -23 | Lower | -23 | Lower | 0 | Same |
| LLOC | 93 | **67 ✅** | 125 | +26 | Higher | -32 | Lower | -58 | Lower |
| Comments (CLOC) | 20 | **26 ✅** | 9 | -6 | Lower | +11 | Higher | +17 | Higher |
| Blank Lines | 19 | **18 ✅** | 20 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 2 | 9 | 2 | -7 | Lower | 0 | Same | +7 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 8 | 1 | -7 | Lower | 0 | Same | +7 | Higher |

## p00261

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.6064 ✅** | 54.3496 | 63.3434 | +13.2568 | Higher | +4.2630 | Higher | -8.9938 | Lower |
| MI (SEI) | **57.5187 ✅** | 18.2439 | 42.3819 | +39.2747 | Higher | +15.1368 | Higher | -24.1379 | Lower |
| Cyclomatic (sum) | **7 ✅** | 32 | 10 | -25 | Lower | -3 | Lower | +22 | Higher |
| Cognitive (sum) | **11 ✅** | 25 | 14 | -14 | Lower | -3 | Lower | +11 | Higher |
| Halstead Volume | **1133.7128 ✅** | 1710.5305 | 1350.8653 | -576.8176 | Lower | -217.1524 | Lower | +359.6652 | Higher |
| Halstead Effort | **18491.7756 ✅** | 62619.2837 | 27692.7379 | -44127.5080 | Lower | -9200.9623 | Lower | +34926.5458 | Higher |
| SLOC | **56 ✅** | 78 | 66 | -22 | Lower | -10 | Lower | +12 | Higher |
| PLOC | **41 ✅** | 68 | 48 | -27 | Lower | -7 | Lower | +20 | Higher |
| LLOC | 19 | **17 ✅** | 23 | +2 | Higher | -4 | Lower | -6 | Lower |
| Comments (CLOC) | **14 ✅** | 2 | 8 | +12 | Higher | +6 | Higher | -6 | Lower |
| Blank Lines | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00270

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 24.3935 | **29.3473 ✅** | 18.6799 | -4.9538 | Lower | +5.7136 | Higher | +10.6674 | Higher |
| MI (SEI) | -21.9945 | **-10.6511 ✅** | -26.9273 | -11.3434 | Lower | +4.9327 | Higher | +16.2762 | Higher |
| Cyclomatic (sum) | 51 | **42 ✅** | 59 | +9 | Higher | -8 | Lower | -17 | Lower |
| Cognitive (sum) | **65 ✅** | 71 | 68 | -6 | Lower | -3 | Lower | +3 | Higher |
| Halstead Volume | 7963.0641 | **6945.9387 ✅** | 9419.2381 | +1017.1254 | Higher | -1456.1740 | Lower | -2473.2994 | Lower |
| Halstead Effort | 622340.6092 | **429396.3170 ✅** | 809673.9027 | +192944.2921 | Higher | -187333.2935 | Lower | -380277.5857 | Lower |
| SLOC | 231 | **202 ✅** | 278 | +29 | Higher | -47 | Lower | -76 | Lower |
| PLOC | 207 | **162 ✅** | 241 | +45 | Higher | -34 | Lower | -79 | Lower |
| LLOC | 138 | **121 ✅** | 173 | +17 | Higher | -35 | Lower | -52 | Lower |
| Comments (CLOC) | 7 | 12 | 12 | -5 | Lower | -5 | Lower | 0 | Same |
| Blank Lines | **18 ✅** | 28 | 25 | -10 | Lower | -7 | Lower | +3 | Higher |
| Functions | 10 | **2 ✅** | 13 | +8 | Higher | -3 | Lower | -11 | Lower |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 8 | **4 ✅** | 8 | +4 | Higher | 0 | Same | -4 | Lower |

## p00271

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 90.3083 | **91.1548 ✅** | 90.1873 | -0.8465 | Lower | +0.1209 | Higher | +0.9675 | Higher |
| MI (SEI) | 86.8274 | 84.9851 | **89.6888 ✅** | +1.8423 | Higher | -2.8614 | Lower | -4.7037 | Lower |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 425 | **423.7291 ✅** | 435 | +1.2709 | Higher | -10 | Lower | -11.2709 | Lower |
| Halstead Effort | **4131.9444 ✅** | 4413.8449 | 4398.3333 | -281.9004 | Lower | -266.3889 | Lower | +15.5115 | Higher |
| SLOC | 20 | **19 ✅** | 20 | +1 | Higher | 0 | Same | -1 | Lower |
| PLOC | 12 | 12 | 12 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 4 | 3 | **5 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00283

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **50.4871 ✅** | 43.8870 | 50.0321 | +6.6002 | Higher | +0.4550 | Higher | -6.1452 | Lower |
| MI (SEI) | **18.3552 ✅** | 16.8416 | 14.1153 | +1.5136 | Higher | +4.2400 | Higher | +2.7263 | Higher |
| Cyclomatic (sum) | 22 | 27 | 22 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | **21 ✅** | 35 | 24 | -14 | Lower | -3 | Lower | +11 | Higher |
| Halstead Volume | 3027.2059 | 3173.1094 | **2999.0052 ✅** | -145.9035 | Lower | +28.2007 | Higher | +174.1042 | Higher |
| Halstead Effort | 156493.3824 | **136496.5885 ✅** | 167227.1381 | +19996.7939 | Higher | -10733.7557 | Lower | -30730.5496 | Lower |
| SLOC | **95 ✅** | 131 | 98 | -36 | Lower | -3 | Lower | +33 | Higher |
| PLOC | **76 ✅** | 98 | 80 | -22 | Lower | -4 | Lower | +18 | Higher |
| LLOC | 54 | 57 | **51 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Comments (CLOC) | 6 | **17 ✅** | 4 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | 15 | 21 | **14 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |

## p00288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 21.3545 | **23.5538 ✅** | 11.6016 | -2.1993 | Lower | +9.7529 | Higher | +11.9522 | Higher |
| MI (SEI) | -25.3271 | **-15.1888 ✅** | -38.0531 | -10.1383 | Lower | +12.7261 | Higher | +22.8643 | Higher |
| Cyclomatic (sum) | 45 | **43 ✅** | 67 | +2 | Higher | -22 | Lower | -24 | Lower |
| Cognitive (sum) | 87 | **81 ✅** | 100 | +6 | Higher | -13 | Lower | -19 | Lower |
| Halstead Volume | 9681 | **9005.4605 ✅** | 12256.1048 | +675.5395 | Higher | -2575.1048 | Lower | -3250.6444 | Lower |
| Halstead Effort | 874412.9032 | **690884.9372 ✅** | 1256869.1710 | +183527.9660 | Higher | -382456.2678 | Lower | -565984.2338 | Lower |
| SLOC | 285 | **262 ✅** | 353 | +23 | Higher | -68 | Lower | -91 | Lower |
| PLOC | 239 | **206 ✅** | 301 | +33 | Higher | -62 | Lower | -95 | Lower |
| LLOC | 168 | **145 ✅** | 227 | +23 | Higher | -59 | Lower | -82 | Lower |
| Comments (CLOC) | 11 | **23 ✅** | 12 | -12 | Lower | -1 | Lower | +11 | Higher |
| Blank Lines | 36 | **34 ✅** | 40 | +2 | Higher | -4 | Lower | -6 | Lower |
| Functions | 6 | **5 ✅** | 13 | +1 | Higher | -7 | Lower | -8 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 4 | 4 | 9 | 0 | Same | -5 | Lower | -5 | Lower |

## p00292

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.6773 | 69.1925 | **75.3584 ✅** | +2.4848 | Higher | -3.6811 | Lower | -6.1660 | Lower |
| MI (SEI) | 56.9588 | 53.5905 | **65.8013 ✅** | +3.3683 | Higher | -8.8425 | Lower | -12.2108 | Lower |
| Cyclomatic (sum) | 13 | 13 | **5 ✅** | 0 | Same | +8 | Higher | +8 | Higher |
| Cognitive (sum) | 7 | 7 | **2 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Halstead Volume | 1049.7573 | 1106.1958 | **860.9613 ✅** | -56.4386 | Lower | +188.7960 | Higher | +245.2345 | Higher |
| Halstead Effort | 29393.2030 | 32632.7765 | **16341.7072 ✅** | -3239.5735 | Lower | +13051.4958 | Higher | +16291.0693 | Higher |
| SLOC | 41 | 47 | **39 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| PLOC | 28 | 30 | **23 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| LLOC | **14 ✅** | 17 | 18 | -3 | Lower | -4 | Lower | -1 | Lower |
| Comments (CLOC) | 6 | 7 | **8 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 9 | 10 | **8 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | 3 | **0 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Exits | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00315

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.0236 ✅** | 49.3862 | 51.4537 | +3.6374 | Higher | +1.5699 | Higher | -2.0675 | Lower |
| MI (SEI) | **29.7717 ✅** | 29.4184 | 23.7715 | +0.3532 | Higher | +6.0001 | Higher | +5.6469 | Higher |
| Cyclomatic (sum) | **16 ✅** | 19 | 18 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **19 ✅** | 27 | 20 | -8 | Lower | -1 | Lower | +7 | Higher |
| Halstead Volume | 3304.5790 | **3127.5645 ✅** | 3430.4783 | +177.0144 | Higher | -125.8993 | Lower | -302.9138 | Lower |
| Halstead Effort | 220764.2328 | **194366.6927 ✅** | 215162.7902 | +26397.5401 | Higher | +5601.4426 | Higher | -20796.0974 | Lower |
| SLOC | **86 ✅** | 105 | 91 | -19 | Lower | -5 | Lower | +14 | Higher |
| PLOC | **63 ✅** | 75 | 70 | -12 | Lower | -7 | Lower | +5 | Higher |
| LLOC | **52 ✅** | 62 | 55 | -10 | Lower | -3 | Lower | +7 | Higher |
| Comments (CLOC) | 12 | **21 ✅** | 9 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | 13 | **11 ✅** | 12 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.1430 | **42.1512 ✅** | 19.1504 | -11.0082 | Lower | +11.9926 | Higher | +23.0007 | Higher |
| MI (SEI) | -2.7947 | **2.1466 ✅** | -19.6726 | -4.9413 | Lower | +16.8779 | Higher | +21.8192 | Higher |
| Cyclomatic (sum) | 41 | **34 ✅** | 60 | +7 | Higher | -19 | Lower | -26 | Lower |
| Cognitive (sum) | 73 | **50 ✅** | 106 | +23 | Higher | -33 | Lower | -56 | Lower |
| Halstead Volume | 5737.0175 | **3580.5475 ✅** | 7961.3049 | +2156.4700 | Higher | -2224.2875 | Lower | -4380.7575 | Lower |
| Halstead Effort | 405689.0913 | **178886.9596 ✅** | 714029.5358 | +226802.1316 | Higher | -308340.4446 | Lower | -535142.5762 | Lower |
| SLOC | 195 | **127 ✅** | 281 | +68 | Higher | -86 | Lower | -154 | Lower |
| PLOC | 162 | **108 ✅** | 230 | +54 | Higher | -68 | Lower | -122 | Lower |
| LLOC | 126 | **57 ✅** | 171 | +69 | Higher | -45 | Lower | -114 | Lower |
| Comments (CLOC) | 20 | 4 | **25 ✅** | +16 | Higher | -5 | Lower | -21 | Lower |
| Blank Lines | 16 | **15 ✅** | 30 | +1 | Higher | -14 | Lower | -15 | Lower |
| Functions | 7 | **5 ✅** | 8 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
| Exits | **5 ✅** | 13 | 12 | -8 | Lower | -7 | Lower | +1 | Higher |

## p00320

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.7954 | **58.3515 ✅** | 45.6589 | -11.5561 | Lower | +1.1365 | Higher | +12.6927 | Higher |
| MI (SEI) | 14.5413 | **37.2307 ✅** | 12.1391 | -22.6895 | Lower | +2.4022 | Higher | +25.0916 | Higher |
| Cyclomatic (sum) | 24 | **20 ✅** | 23 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 22 | **18 ✅** | 25 | +4 | Higher | -3 | Lower | -7 | Lower |
| Halstead Volume | 3569.3907 | **2065.5684 ✅** | 3730.2655 | +1503.8223 | Higher | -160.8748 | Lower | -1664.6971 | Lower |
| Halstead Effort | 241573.6685 | **75674.9142 ✅** | 261584.8699 | +165898.7543 | Higher | -20011.2015 | Lower | -185909.9558 | Lower |
| SLOC | 110 | **68 ✅** | 118 | +42 | Higher | -8 | Lower | -50 | Lower |
| PLOC | 88 | **50 ✅** | 98 | +38 | Higher | -10 | Lower | -48 | Lower |
| LLOC | 51 | **21 ✅** | 64 | +30 | Higher | -13 | Lower | -43 | Lower |
| Comments (CLOC) | 8 | **9 ✅** | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 14 | **9 ✅** | 13 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 3 | **0 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Exits | 4 | **0 ✅** | 6 | +4 | Higher | -2 | Lower | -6 | Lower |

## p00329

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.2392 | **49.9255 ✅** | 37.3367 | -11.6863 | Lower | +0.9025 | Higher | +12.5888 | Higher |
| MI (SEI) | 4.3467 | **17.3804 ✅** | -1.7019 | -13.0337 | Lower | +6.0485 | Higher | +19.0822 | Higher |
| Cyclomatic (sum) | 36 | **24 ✅** | 32 | +12 | Higher | +4 | Higher | -8 | Lower |
| Cognitive (sum) | 45 | 35 | **32 ✅** | +10 | Higher | +13 | Higher | +3 | Higher |
| Halstead Volume | 4140.4665 | **2714.7670 ✅** | 4625.2054 | +1425.6995 | Higher | -484.7389 | Lower | -1910.4384 | Lower |
| Halstead Effort | 267405.1310 | **148342.6254 ✅** | 337560.2512 | +119062.5056 | Higher | -70155.1202 | Lower | -189217.6258 | Lower |
| SLOC | 150 | **99 ✅** | 162 | +51 | Higher | -12 | Lower | -63 | Lower |
| PLOC | 126 | **79 ✅** | 141 | +47 | Higher | -15 | Lower | -62 | Lower |
| LLOC | 82 | **52 ✅** | 95 | +30 | Higher | -13 | Lower | -43 | Lower |
| Comments (CLOC) | **12 ✅** | 6 | 8 | +6 | Higher | +4 | Higher | -2 | Lower |
| Blank Lines | **13 ✅** | 14 | 15 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 7 | **2 ✅** | 9 | +5 | Higher | -2 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |

## p00336

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.0832 | 68.1752 | **73.1329 ✅** | +4.9080 | Higher | -0.0498 | Lower | -4.9577 | Lower |
| MI (SEI) | **60.5294 ✅** | 47.0983 | 58.6507 | +13.4311 | Higher | +1.8787 | Higher | -11.5524 | Lower |
| Cyclomatic (sum) | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 7 | 13 | 7 | -6 | Lower | 0 | Same | +6 | Higher |
| Halstead Volume | 1179.0891 | 1324.0769 | **1167.8596 ✅** | -144.9879 | Lower | +11.2294 | Higher | +156.2173 | Higher |
| Halstead Effort | 37583.4640 | 47517.2766 | **36787.5789 ✅** | -9933.8126 | Lower | +795.8851 | Higher | +10729.6977 | Higher |
| SLOC | 40 | 50 | 40 | -10 | Lower | 0 | Same | +10 | Higher |
| PLOC | 26 | 38 | 26 | -12 | Lower | 0 | Same | +12 | Higher |
| LLOC | 21 | 31 | 21 | -10 | Lower | 0 | Same | +10 | Higher |
| Comments (CLOC) | **7 ✅** | 5 | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | **7 ✅** | 9 | 8 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00378

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.2998 ✅** | 75.6318 | 77.1536 | +2.6680 | Higher | +1.1462 | Higher | -1.5218 | Lower |
| MI (SEI) | **68.9542 ✅** | 60.9840 | 61.5549 | +7.9702 | Higher | +7.3993 | Higher | -0.5709 | Lower |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **866.5159 ✅** | 920.8124 | 894.2888 | -54.2966 | Lower | -27.7729 | Lower | +26.5236 | Higher |
| Halstead Effort | 22604.7616 | 23867.4578 | **22282.6961 ✅** | -1262.6962 | Lower | +322.0655 | Higher | +1584.7617 | Higher |
| SLOC | **32 ✅** | 37 | 34 | -5 | Lower | -2 | Lower | +3 | Higher |
| PLOC | **25 ✅** | 27 | 26 | -2 | Lower | -1 | Lower | +1 | Higher |
| LLOC | **21 ✅** | 23 | 22 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | **6 ✅** | 5 | 4 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 4 | 8 | 4 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00428

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.8960 ✅** | 62.2890 | 59.3375 | +4.6070 | Higher | +7.5585 | Higher | +2.9515 | Higher |
| MI (SEI) | **50.5782 ✅** | 43.6205 | 34.9081 | +6.9578 | Higher | +15.6701 | Higher | +8.7123 | Higher |
| Cyclomatic (sum) | **12 ✅** | 16 | 16 | -4 | Lower | -4 | Lower | 0 | Same |
| Cognitive (sum) | **9 ✅** | 12 | 27 | -3 | Lower | -18 | Lower | -15 | Lower |
| Halstead Volume | **1312.3687 ✅** | 1541.7262 | 1782.8580 | -229.3575 | Lower | -470.4893 | Lower | -241.1318 | Lower |
| Halstead Effort | **34544.9298 ✅** | 43168.3325 | 71537.1757 | -8623.4027 | Lower | -36992.2459 | Lower | -28368.8432 | Lower |
| SLOC | **52 ✅** | 62 | 71 | -10 | Lower | -19 | Lower | -9 | Lower |
| PLOC | **37 ✅** | 45 | 56 | -8 | Lower | -19 | Lower | -11 | Lower |
| LLOC | 26 | 26 | 44 | 0 | Same | -18 | Lower | -18 | Lower |
| Comments (CLOC) | 8 | **9 ✅** | 7 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.7237 | 80.0639 | **82.1683 ✅** | -2.3402 | Lower | -4.4446 | Lower | -2.1044 | Lower |
| MI (SEI) | 72.1148 | **78.0742 ✅** | 76.2135 | -5.9593 | Lower | -4.0987 | Lower | +1.8606 | Higher |
| Cyclomatic (sum) | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 9 | 5 | 5 | +4 | Higher | +4 | Higher | 0 | Same |
| Halstead Volume | 641.6858 | **560.8010 ✅** | 652.4704 | +80.8848 | Higher | -10.7846 | Lower | -91.6694 | Lower |
| Halstead Effort | 9625.2866 | **7570.8137 ✅** | 9317.2774 | +2054.4729 | Higher | +308.0092 | Higher | -1746.4638 | Lower |
| SLOC | 36 | 33 | **28 ✅** | +3 | Higher | +8 | Higher | +5 | Higher |
| PLOC | 24 | 19 | **18 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| LLOC | 11 | **8 ✅** | 14 | +3 | Higher | -3 | Lower | -6 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 6 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00445

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.1276 | 75.1660 | **79.5125 ✅** | +3.9616 | Higher | -0.3849 | Lower | -4.3465 | Lower |
| MI (SEI) | 62.0065 | 55.4835 | **70.8056 ✅** | +6.5230 | Higher | -8.7990 | Lower | -15.3221 | Lower |
| Cyclomatic (sum) | 7 | 11 | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 707.0159 | 879.2259 | **656.5780 ✅** | -172.2100 | Lower | +50.4379 | Higher | +222.6479 | Higher |
| Halstead Effort | **15230.3015 ✅** | 20009.9694 | 15476.4823 | -4779.6679 | Lower | -246.1808 | Lower | +4533.4871 | Higher |
| SLOC | 32 | 36 | 32 | -4 | Lower | 0 | Same | +4 | Higher |
| PLOC | 23 | 26 | **22 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| LLOC | 17 | 20 | **16 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Comments (CLOC) | 3 | 3 | **6 ✅** | 0 | Same | -3 | Lower | -3 | Lower |
| Blank Lines | 6 | 7 | **4 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.1349 | 55.4374 | **65.8328 ✅** | +9.6975 | Higher | -0.6979 | Lower | -10.3954 | Lower |
| MI (SEI) | 44.9233 | 41.5975 | **50.4883 ✅** | +3.3258 | Higher | -5.5650 | Lower | -8.8908 | Lower |
| Cyclomatic (sum) | 12 | 17 | 12 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 16 | **15 ✅** | 16 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1242.4056 | 1850.1572 | **1145.7884 ✅** | -607.7516 | Lower | +96.6172 | Higher | +704.3688 | Higher |
| Halstead Effort | **35427.9714 ✅** | 70400.8536 | 37132.0309 | -34972.8822 | Lower | -1704.0595 | Lower | +33268.8226 | Higher |
| SLOC | 59 | 88 | **58 ✅** | -29 | Lower | +1 | Higher | +30 | Higher |
| PLOC | 45 | 52 | **42 ✅** | -7 | Lower | +3 | Higher | +10 | Higher |
| LLOC | 36 | 34 | 34 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 7 | **23 ✅** | 10 | -16 | Lower | -3 | Lower | +13 | Higher |
| Blank Lines | 8 | 14 | **7 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00453

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 26.9852 | 38.1796 | **40.5402 ✅** | -11.1944 | Lower | -13.5550 | Lower | -2.3605 | Lower |
| MI (SEI) | -1.7744 | 7.6039 | **10.5620 ✅** | -9.3782 | Lower | -12.3364 | Lower | -2.9581 | Lower |
| Cyclomatic (sum) | 51 | 34 | **28 ✅** | +17 | Higher | +23 | Higher | +6 | Higher |
| Cognitive (sum) | 116 | 70 | **66 ✅** | +46 | Higher | +50 | Higher | +4 | Higher |
| Halstead Volume | 6911.8909 | **4390.5473 ✅** | 4594.7793 | +2521.3436 | Higher | +2317.1116 | Higher | -204.2320 | Lower |
| Halstead Effort | 806582.5185 | 284641.4117 | **264576.4310 ✅** | +521941.1068 | Higher | +542006.0876 | Higher | +20064.9808 | Higher |
| SLOC | 206 | 152 | **141 ✅** | +54 | Higher | +65 | Higher | +11 | Higher |
| PLOC | 150 | 115 | **107 ✅** | +35 | Higher | +43 | Higher | +8 | Higher |
| LLOC | 112 | **80 ✅** | 90 | +32 | Higher | +22 | Higher | -10 | Lower |
| Comments (CLOC) | **35 ✅** | 17 | 16 | +18 | Higher | +19 | Higher | +1 | Higher |
| Blank Lines | 21 | 20 | **18 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 5 | 0 | 0 | +5 | Higher | +5 | Higher | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00465

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 27.4034 | **42.0746 ✅** | 26.9729 | -14.6712 | Lower | +0.4305 | Higher | +15.1017 | Higher |
| MI (SEI) | -17.3179 | **7.5254 ✅** | -18.6102 | -24.8433 | Lower | +1.2923 | Higher | +26.1356 | Higher |
| Cyclomatic (sum) | 44 | **23 ✅** | 46 | +21 | Higher | -2 | Lower | -23 | Lower |
| Cognitive (sum) | 51 | **37 ✅** | 45 | +14 | Higher | +6 | Higher | -8 | Lower |
| Halstead Volume | 6512.8182 | **4461.5570 ✅** | 6566.0415 | +2051.2611 | Higher | -53.2233 | Lower | -2104.4845 | Lower |
| Halstead Effort | 508949.6018 | **255707.1478 ✅** | 492933.5522 | +253242.4541 | Higher | +16016.0496 | Higher | -237226.4045 | Lower |
| SLOC | 226 | **139 ✅** | 225 | +87 | Higher | +1 | Higher | -86 | Lower |
| PLOC | 200 | **107 ✅** | 191 | +93 | Higher | +9 | Higher | -84 | Lower |
| LLOC | 119 | **79 ✅** | 119 | +40 | Higher | 0 | Same | -40 | Lower |
| Comments (CLOC) | 8 | **10 ✅** | 7 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | **19 ✅** | 23 | 28 | -4 | Lower | -9 | Lower | -5 | Lower |
| Functions | 9 | **2 ✅** | 12 | +7 | Higher | -3 | Lower | -10 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **1 ✅** | 9 | +6 | Higher | -2 | Lower | -8 | Lower |

## p00468

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.4550 | 60.5219 | **62.1231 ✅** | -14.0670 | Lower | -15.6681 | Lower | -1.6011 | Lower |
| MI (SEI) | 20.0040 | 38.6805 | **41.3865 ✅** | -18.6764 | Lower | -21.3824 | Lower | -2.7060 | Lower |
| Cyclomatic (sum) | 26 | 17 | **14 ✅** | +9 | Higher | +12 | Higher | +3 | Higher |
| Cognitive (sum) | 30 | 23 | **18 ✅** | +7 | Higher | +12 | Higher | +5 | Higher |
| Halstead Volume | 3297.8614 | **1553.8196 ✅** | 1575.1781 | +1744.0417 | Higher | +1722.6832 | Higher | -21.3585 | Lower |
| Halstead Effort | 224127.7325 | **50741.9221 ✅** | 53739.8270 | +173385.8104 | Higher | +170387.9055 | Higher | -2997.9049 | Lower |
| SLOC | 112 | 68 | **64 ✅** | +44 | Higher | +48 | Higher | +4 | Higher |
| PLOC | 83 | 49 | **48 ✅** | +34 | Higher | +35 | Higher | +1 | Higher |
| LLOC | 60 | 39 | **38 ✅** | +21 | Higher | +22 | Higher | +1 | Higher |
| Comments (CLOC) | **14 ✅** | 8 | 8 | +6 | Higher | +6 | Higher | 0 | Same |
| Blank Lines | 16 | 11 | **10 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| Functions | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |

## p00479

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.1494 | **73.1570 ✅** | 71.5500 | -16.0076 | Lower | -14.4006 | Lower | +1.6070 | Higher |
| MI (SEI) | 31.5135 | **56.2030 ✅** | 48.2321 | -24.6895 | Lower | -16.7186 | Lower | +7.9709 | Higher |
| Cyclomatic (sum) | 18 | **6 ✅** | 8 | +12 | Higher | +10 | Higher | -2 | Lower |
| Cognitive (sum) | 23 | 10 | **9 ✅** | +13 | Higher | +14 | Higher | +1 | Higher |
| Halstead Volume | 2185.0081 | **1076.4032 ✅** | 1156.9905 | +1108.6049 | Higher | +1028.0176 | Higher | -80.5873 | Lower |
| Halstead Effort | 99990.1312 | **21886.8652 ✅** | 26391.3527 | +78103.2660 | Higher | +73598.7785 | Higher | -4504.4875 | Lower |
| SLOC | 74 | **41 ✅** | 43 | +33 | Higher | +31 | Higher | -2 | Lower |
| PLOC | 58 | **26 ✅** | 31 | +32 | Higher | +27 | Higher | -5 | Lower |
| LLOC | 44 | 20 | 20 | +24 | Higher | +24 | Higher | 0 | Same |
| Comments (CLOC) | **7 ✅** | 5 | 3 | +2 | Higher | +4 | Higher | +2 | Higher |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **0 ✅** | 1 | +5 | Higher | +4 | Higher | -1 | Lower |

## p00492

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.6502 | 48.0345 | **52.6905 ✅** | +4.6157 | Higher | -0.0402 | Lower | -4.6559 | Lower |
| MI (SEI) | 26.9034 | 17.9668 | **28.9479 ✅** | +8.9365 | Higher | -2.0445 | Lower | -10.9811 | Lower |
| Cyclomatic (sum) | 18 | 29 | 18 | -11 | Lower | 0 | Same | +11 | Higher |
| Cognitive (sum) | 27 | 54 | 27 | -27 | Lower | 0 | Same | +27 | Higher |
| Halstead Volume | 2920.7308 | 3231.0884 | **2799.0715 ✅** | -310.3576 | Lower | +121.6593 | Higher | +432.0169 | Higher |
| Halstead Effort | **157063.7879 ✅** | 206569.3574 | 161972.9390 | -49505.5695 | Lower | -4909.1511 | Lower | +44596.4184 | Higher |
| SLOC | **89 ✅** | 98 | 90 | -9 | Lower | -1 | Lower | +8 | Higher |
| PLOC | 67 | 82 | **64 ✅** | -15 | Lower | +3 | Higher | +18 | Higher |
| LLOC | **53 ✅** | 55 | 57 | -2 | Lower | -4 | Lower | -2 | Lower |
| Comments (CLOC) | 10 | 8 | **12 ✅** | +2 | Higher | -2 | Lower | -4 | Lower |
| Blank Lines | 12 | **9 ✅** | 14 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.5372 | **57.8710 ✅** | 57.1187 | -7.3338 | Lower | -6.5815 | Lower | +0.7524 | Higher |
| MI (SEI) | 24.8654 | **41.6237 ✅** | 32.1740 | -16.7583 | Lower | -7.3086 | Lower | +9.4497 | Higher |
| Cyclomatic (sum) | 17 | 17 | **15 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 27 | 24 | **17 ✅** | +3 | Higher | +10 | Higher | +7 | Higher |
| Halstead Volume | 2738.3689 | **1621.5434 ✅** | 2217.5709 | +1116.8255 | Higher | +520.7981 | Higher | -596.0274 | Lower |
| Halstead Effort | 122512.2449 | **55107.1398 ✅** | 98127.5102 | +67405.1051 | Higher | +24384.7347 | Higher | -43020.3704 | Lower |
| SLOC | 105 | 79 | **77 ✅** | +26 | Higher | +28 | Higher | +2 | Higher |
| PLOC | 81 | 57 | **55 ✅** | +24 | Higher | +26 | Higher | +2 | Higher |
| LLOC | 68 | **36 ✅** | 41 | +32 | Higher | +27 | Higher | -5 | Lower |
| Comments (CLOC) | 13 | **16 ✅** | 8 | -3 | Lower | +5 | Higher | +8 | Higher |
| Blank Lines | 14 | **9 ✅** | 14 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |

## p00507

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.8200 | 60.6627 | **62.8398 ✅** | -6.8427 | Lower | -9.0198 | Lower | -2.1771 | Lower |
| MI (SEI) | 32.2285 | 35.2978 | **51.2529 ✅** | -3.0692 | Lower | -19.0244 | Lower | -15.9551 | Lower |
| Cyclomatic (sum) | 22 | 14 | **13 ✅** | +8 | Higher | +9 | Higher | +1 | Higher |
| Cognitive (sum) | 26 | **15 ✅** | 26 | +11 | Higher | 0 | Same | -11 | Lower |
| Halstead Volume | 2024.1721 | 1650.1288 | **1434.4167 ✅** | +374.0433 | Higher | +589.7554 | Higher | +215.7121 | Higher |
| Halstead Effort | 65036.6481 | 67792.7907 | **52055.4435 ✅** | -2756.1427 | Lower | +12981.2045 | Higher | +15737.3472 | Higher |
| SLOC | 88 | 69 | **64 ✅** | +19 | Higher | +24 | Higher | +5 | Higher |
| PLOC | 63 | 55 | **40 ✅** | +8 | Higher | +23 | Higher | +15 | Higher |
| LLOC | **27 ✅** | 45 | 31 | -18 | Lower | -4 | Lower | +14 | Higher |
| Comments (CLOC) | 13 | 6 | **16 ✅** | +7 | Higher | -3 | Lower | -10 | Lower |
| Blank Lines | 12 | 8 | 8 | +4 | Higher | +4 | Higher | 0 | Same |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00514

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.2112 | 39.9191 | **48.9969 ✅** | +2.2921 | Higher | -6.7856 | Lower | -9.0777 | Lower |
| MI (SEI) | 7.8926 | 10.6766 | **22.3121 ✅** | -2.7841 | Lower | -14.4195 | Lower | -11.6354 | Lower |
| Cyclomatic (sum) | 26 | 27 | **18 ✅** | -1 | Lower | +8 | Higher | +9 | Higher |
| Cognitive (sum) | 18 | 23 | **17 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Halstead Volume | 3640.1475 | 4282.5644 | **3135.7739 ✅** | -642.4169 | Lower | +504.3736 | Higher | +1146.7904 | Higher |
| Halstead Effort | 257464.6003 | 241964.8873 | **202330.3447 ✅** | +15499.7130 | Higher | +55134.2556 | Higher | +39634.5425 | Higher |
| SLOC | 141 | 152 | **109 ✅** | -11 | Lower | +32 | Higher | +43 | Higher |
| PLOC | 115 | 113 | **83 ✅** | +2 | Higher | +32 | Higher | +30 | Higher |
| LLOC | 84 | 79 | **73 ✅** | +5 | Higher | +11 | Higher | +6 | Higher |
| Comments (CLOC) | 10 | **19 ✅** | 13 | -9 | Lower | -3 | Lower | +6 | Higher |
| Blank Lines | 19 | 21 | **14 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Functions | 7 | 8 | **5 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 8 | 7 | **1 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |

## p00518

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.5263 | 55.6037 | **57.0025 ✅** | -0.0775 | Lower | -1.4762 | Lower | -1.3988 | Lower |
| MI (SEI) | **39.1011 ✅** | 30.8678 | 29.3239 | +8.2333 | Higher | +9.7771 | Higher | +1.5439 | Higher |
| Cyclomatic (sum) | **17 ✅** | 19 | 18 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 27 | **24 ✅** | 29 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | **1818.8151 ✅** | 1968.0972 | 1985.8877 | -149.2821 | Lower | -167.0726 | Lower | -17.7905 | Lower |
| Halstead Effort | **62492.1130 ✅** | 63983.2405 | 81421.3956 | -1491.1275 | Lower | -18929.2826 | Lower | -17438.1551 | Lower |
| SLOC | 88 | 83 | **77 ✅** | +5 | Higher | +11 | Higher | +6 | Higher |
| PLOC | **57 ✅** | 64 | 59 | -7 | Lower | -2 | Lower | +5 | Higher |
| LLOC | **45 ✅** | 46 | 51 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | **19 ✅** | 9 | 6 | +10 | Higher | +13 | Higher | +3 | Higher |
| Blank Lines | 13 | 13 | 14 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00519

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **45.4862 ✅** | 38.5396 | 28.7373 | +6.9466 | Higher | +16.7489 | Higher | +9.8023 | Higher |
| MI (SEI) | **20.1961 ✅** | 4.1433 | -14.1070 | +16.0528 | Higher | +34.3031 | Higher | +18.2503 | Higher |
| Cyclomatic (sum) | **33 ✅** | 42 | 44 | -9 | Lower | -11 | Lower | -2 | Lower |
| Cognitive (sum) | **32 ✅** | 42 | 46 | -10 | Lower | -14 | Lower | -4 | Lower |
| Halstead Volume | **3672.3369 ✅** | 4912.3820 | 6060.6190 | -1240.0451 | Lower | -2388.2820 | Lower | -1148.2369 | Lower |
| Halstead Effort | **255500.8873 ✅** | 362327.7904 | 468364.6327 | -106826.9030 | Lower | -212863.7453 | Lower | -106036.8423 | Lower |
| SLOC | **104 ✅** | 128 | 213 | -24 | Lower | -109 | Lower | -85 | Lower |
| PLOC | **77 ✅** | 105 | 182 | -28 | Lower | -105 | Lower | -77 | Lower |
| LLOC | **50 ✅** | 58 | 102 | -8 | Lower | -52 | Lower | -44 | Lower |
| Comments (CLOC) | **14 ✅** | 9 | 9 | +5 | Higher | +5 | Higher | 0 | Same |
| Blank Lines | **13 ✅** | 14 | 22 | -1 | Lower | -9 | Lower | -8 | Lower |
| Functions | 1 | 1 | 8 | 0 | Same | -7 | Lower | -7 | Lower |
| Closures | 8 | 9 | **0 ✅** | -1 | Lower | +8 | Higher | +9 | Higher |
| Exits | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |

## p00528

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.1914 | **40.7191 ✅** | 18.4873 | -1.5277 | Lower | +20.7042 | Higher | +22.2318 | Higher |
| MI (SEI) | 8.0728 | **11.1292 ✅** | -26.2878 | -3.0564 | Lower | +34.3606 | Higher | +37.4170 | Higher |
| Cyclomatic (sum) | 28 | **27 ✅** | 55 | +1 | Higher | -27 | Lower | -28 | Lower |
| Cognitive (sum) | 32 | **30 ✅** | 49 | +2 | Higher | -17 | Lower | -19 | Lower |
| Halstead Volume | **5120.9961 ✅** | 5192.5472 | 9800.0626 | -71.5511 | Lower | -4679.0665 | Lower | -4607.5154 | Lower |
| Halstead Effort | 350032.6785 | **292080.7816 ✅** | 1107003.5413 | +57951.8969 | Higher | -756970.8628 | Lower | -814922.7597 | Lower |
| SLOC | 148 | **136 ✅** | 294 | +12 | Higher | -146 | Lower | -158 | Lower |
| PLOC | 115 | **104 ✅** | 252 | +11 | Higher | -137 | Lower | -148 | Lower |
| LLOC | 85 | **70 ✅** | 194 | +15 | Higher | -109 | Lower | -124 | Lower |
| Comments (CLOC) | 16 | 16 | 15 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 20 | **16 ✅** | 27 | +4 | Higher | -7 | Lower | -11 | Lower |
| Functions | 2 | **1 ✅** | 11 | +1 | Higher | -9 | Lower | -10 | Lower |
| Closures | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | **3 ✅** | 4 | 10 | -1 | Lower | -7 | Lower | -6 | Lower |

## p00534

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.9880 ✅** | 56.5015 | 66.9982 | +12.4865 | Higher | +1.9898 | Higher | -10.4967 | Lower |
| MI (SEI) | **55.7238 ✅** | 38.7877 | 50.7649 | +16.9361 | Higher | +4.9589 | Higher | -11.9772 | Lower |
| Cyclomatic (sum) | **8 ✅** | 15 | 10 | -7 | Lower | -2 | Lower | +5 | Higher |
| Cognitive (sum) | 10 | 22 | 10 | -12 | Lower | 0 | Same | +12 | Higher |
| Halstead Volume | **1344.2105 ✅** | 1976.5169 | 1493.5152 | -632.3064 | Lower | -149.3047 | Lower | +483.0017 | Higher |
| Halstead Effort | **44309.1610 ✅** | 77726.5262 | 53119.3575 | -33417.3652 | Lower | -8810.1965 | Lower | +24607.1687 | Higher |
| SLOC | **48 ✅** | 83 | 51 | -35 | Lower | -3 | Lower | +32 | Higher |
| PLOC | **29 ✅** | 55 | 35 | -26 | Lower | -6 | Lower | +20 | Higher |
| LLOC | **25 ✅** | 39 | 28 | -14 | Lower | -3 | Lower | +11 | Higher |
| Comments (CLOC) | 9 | **16 ✅** | 8 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 10 | 13 | **8 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 4 | 1 | -4 | Lower | -1 | Lower | +3 | Higher |

## p00543

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.1688 ✅** | 71.8117 | 66.8673 | +1.3570 | Higher | +6.3015 | Higher | +4.9444 | Higher |
| MI (SEI) | **63.7599 ✅** | 56.0626 | 51.5818 | +7.6973 | Higher | +12.1781 | Higher | +4.4808 | Higher |
| Cyclomatic (sum) | **7 ✅** | 8 | 10 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 8 | 12 | **7 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Halstead Volume | **1027.4919 ✅** | 1100.1849 | 1358.6195 | -72.6930 | Lower | -331.1276 | Lower | -258.4347 | Lower |
| Halstead Effort | 30578.1590 | **28358.2133 ✅** | 41547.4618 | +2219.9456 | Higher | -10969.3029 | Lower | -13189.2485 | Lower |
| SLOC | **41 ✅** | 43 | 53 | -2 | Lower | -12 | Lower | -10 | Lower |
| PLOC | **27 ✅** | 31 | 39 | -4 | Lower | -12 | Lower | -8 | Lower |
| LLOC | **21 ✅** | 22 | 33 | -1 | Lower | -12 | Lower | -11 | Lower |
| Comments (CLOC) | 9 | 6 | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Blank Lines | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00554

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.8548 | **67.6346 ✅** | 59.2507 | -4.7798 | Lower | +3.6042 | Higher | +8.3839 | Higher |
| MI (SEI) | 48.9284 | **52.5870 ✅** | 38.4956 | -3.6586 | Lower | +10.4328 | Higher | +14.0914 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 14 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 17 | **11 ✅** | 19 | +6 | Higher | -2 | Lower | -8 | Lower |
| Halstead Volume | 1556.2200 | **1225.2439 ✅** | 1896.1119 | +330.9762 | Higher | -339.8918 | Lower | -670.8680 | Lower |
| Halstead Effort | 56490.7876 | **35225.7614 ✅** | 79230.3893 | +21265.0262 | Higher | -22739.6018 | Lower | -44004.6279 | Lower |
| SLOC | 65 | **53 ✅** | 72 | +12 | Higher | -7 | Lower | -19 | Lower |
| PLOC | 43 | **35 ✅** | 53 | +8 | Higher | -10 | Lower | -18 | Lower |
| LLOC | 41 | **27 ✅** | 45 | +14 | Higher | -4 | Lower | -18 | Lower |
| Comments (CLOC) | **14 ✅** | 9 | 10 | +5 | Higher | +4 | Higher | -1 | Lower |
| Blank Lines | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00571

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.7776 ✅** | 56.4003 | 50.2022 | +2.3773 | Higher | +8.5754 | Higher | +6.1981 | Higher |
| MI (SEI) | 33.8486 | **44.6634 ✅** | 16.6746 | -10.8148 | Lower | +17.1740 | Higher | +27.9888 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 19 | +2 | Higher | -4 | Lower | -6 | Lower |
| Cognitive (sum) | 17 | 13 | **12 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Halstead Volume | 1986.8409 | **1771.4721 ✅** | 2595.5515 | +215.3689 | Higher | -608.7106 | Lower | -824.0794 | Lower |
| Halstead Effort | 72562.8868 | **46803.0979 ✅** | 108101.9590 | +25759.7889 | Higher | -35539.0722 | Lower | -61298.8611 | Lower |
| SLOC | **72 ✅** | 89 | 106 | -17 | Lower | -34 | Lower | -17 | Lower |
| PLOC | 55 | **51 ✅** | 85 | +4 | Higher | -30 | Lower | -34 | Lower |
| LLOC | 38 | **30 ✅** | 48 | +8 | Higher | -10 | Lower | -18 | Lower |
| Comments (CLOC) | 7 | **27 ✅** | 6 | -20 | Lower | +1 | Higher | +21 | Higher |
| Blank Lines | **10 ✅** | 12 | 15 | -2 | Lower | -5 | Lower | -3 | Lower |
| Functions | 2 | **1 ✅** | 5 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 4 | -1 | Lower | -3 | Lower | -2 | Lower |

## p00594

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.9767 | **67.4760 ✅** | 59.4293 | -3.4993 | Lower | +4.5474 | Higher | +8.0467 | Higher |
| MI (SEI) | **47.1659 ✅** | 33.3240 | 41.9735 | +13.8419 | Higher | +5.1924 | Higher | -8.6495 | Lower |
| Cyclomatic (sum) | 12 | **10 ✅** | 15 | +2 | Higher | -3 | Lower | -5 | Lower |
| Cognitive (sum) | 11 | 11 | 24 | 0 | Same | -13 | Lower | -13 | Lower |
| Halstead Volume | 1399.2361 | **1282.4162 ✅** | 1679.0834 | +116.8199 | Higher | -279.8472 | Lower | -396.6672 | Lower |
| Halstead Effort | 43716.1346 | **37889.5691 ✅** | 64764.6439 | +5826.5655 | Higher | -21048.5093 | Lower | -26875.0748 | Lower |
| SLOC | 61 | **52 ✅** | 73 | +9 | Higher | -12 | Lower | -21 | Lower |
| PLOC | 45 | **41 ✅** | 53 | +4 | Higher | -8 | Lower | -12 | Lower |
| LLOC | 37 | **33 ✅** | 43 | +4 | Higher | -6 | Lower | -10 | Lower |
| Comments (CLOC) | 10 | 1 | **13 ✅** | +9 | Higher | -3 | Lower | -12 | Lower |
| Blank Lines | 9 | 10 | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00629

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.2131 ✅** | 54.3085 | 49.1898 | +1.9046 | Higher | +7.0234 | Higher | +5.1187 | Higher |
| MI (SEI) | 24.3092 | **26.6623 ✅** | 23.3737 | -2.3531 | Lower | +0.9355 | Higher | +3.2886 | Higher |
| Cyclomatic (sum) | 18 | **17 ✅** | 25 | +1 | Higher | -7 | Lower | -8 | Lower |
| Cognitive (sum) | 20 | **17 ✅** | 28 | +3 | Higher | -8 | Lower | -11 | Lower |
| Halstead Volume | **2133.9623 ✅** | 2143.3222 | 2899.8147 | -9.3599 | Lower | -765.8525 | Lower | -756.4926 | Lower |
| Halstead Effort | 61031.3204 | **57333.8677 ✅** | 121731.8055 | +3697.4527 | Higher | -60700.4851 | Lower | -64397.9378 | Lower |
| SLOC | **79 ✅** | 90 | 100 | -11 | Lower | -21 | Lower | -10 | Lower |
| PLOC | **64 ✅** | 70 | 77 | -6 | Lower | -13 | Lower | -7 | Lower |
| LLOC | 36 | **33 ✅** | 55 | +3 | Higher | -19 | Lower | -22 | Lower |
| Comments (CLOC) | 4 | 8 | **12 ✅** | -4 | Lower | -8 | Lower | -4 | Lower |
| Blank Lines | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00641

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.3444 | **44.7929 ✅** | 40.3866 | -6.4485 | Lower | -2.0422 | Lower | +4.4063 | Higher |
| MI (SEI) | -2.1729 | **9.1529 ✅** | 3.1527 | -11.3258 | Lower | -5.3256 | Lower | +6.0002 | Higher |
| Cyclomatic (sum) | 31 | **22 ✅** | 28 | +9 | Higher | +3 | Higher | -6 | Lower |
| Cognitive (sum) | 26 | 24 | **17 ✅** | +2 | Higher | +9 | Higher | +7 | Higher |
| Halstead Volume | 4663.3704 | **3946.1494 ✅** | 4245.6719 | +717.2210 | Higher | +417.6985 | Higher | -299.5225 | Lower |
| Halstead Effort | 297128.4412 | **213092.0680 ✅** | 286359.3985 | +84036.3732 | Higher | +10769.0427 | Higher | -73267.3305 | Lower |
| SLOC | 154 | **124 ✅** | 146 | +30 | Higher | +8 | Higher | -22 | Lower |
| PLOC | 129 | **99 ✅** | 119 | +30 | Higher | +10 | Higher | -20 | Lower |
| LLOC | 84 | **78 ✅** | 84 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 6 | 7 | **8 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 19 | **18 ✅** | 20 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 8 | **4 ✅** | 8 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |

## p00649

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.9852 | **55.3364 ✅** | 36.3711 | -10.3512 | Lower | +8.6141 | Higher | +18.9653 | Higher |
| MI (SEI) | 14.1737 | **39.0153 ✅** | -4.5536 | -24.8415 | Lower | +18.7273 | Higher | +43.5688 | Higher |
| Cyclomatic (sum) | 25 | **12 ✅** | 32 | +13 | Higher | -7 | Lower | -20 | Lower |
| Cognitive (sum) | 36 | **6 ✅** | 38 | +30 | Higher | -2 | Lower | -32 | Lower |
| Halstead Volume | 3503.3718 | **2353.3828 ✅** | 4705.7379 | +1149.9890 | Higher | -1202.3661 | Lower | -2352.3550 | Lower |
| Halstead Effort | 142236.8945 | **64741.1001 ✅** | 262898.5017 | +77495.7944 | Higher | -120661.6072 | Lower | -198157.4016 | Lower |
| SLOC | 122 | **88 ✅** | 171 | +34 | Higher | -49 | Lower | -83 | Lower |
| PLOC | 95 | **59 ✅** | 145 | +36 | Higher | -50 | Lower | -86 | Lower |
| LLOC | 63 | **36 ✅** | 86 | +27 | Higher | -23 | Lower | -50 | Lower |
| Comments (CLOC) | 11 | **20 ✅** | 7 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 17 | **11 ✅** | 19 | +6 | Higher | -2 | Lower | -8 | Lower |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 1 | 1 | 5 | 0 | Same | -4 | Lower | -4 | Lower |

## p00653

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 27.7780 | **34.8086 ✅** | 33.4352 | -7.0306 | Lower | -5.6572 | Lower | +1.3734 | Higher |
| MI (SEI) | -15.6282 | **4.9132 ✅** | -7.4361 | -20.5414 | Lower | -8.1922 | Lower | +12.3492 | Higher |
| Cyclomatic (sum) | 43 | **32 ✅** | 33 | +11 | Higher | +10 | Higher | -1 | Lower |
| Cognitive (sum) | 30 | 42 | 30 | -12 | Lower | 0 | Same | +12 | Higher |
| Halstead Volume | 7507.7901 | **5913.3861 ✅** | 5992.3903 | +1594.4040 | Higher | +1515.3997 | Higher | -79.0042 | Lower |
| Halstead Effort | 637089.6151 | **519650.1759 ✅** | 531017.9743 | +117439.4393 | Higher | +106071.6409 | Higher | -11367.7984 | Lower |
| SLOC | 214 | **175 ✅** | 187 | +39 | Higher | +27 | Higher | -12 | Lower |
| PLOC | 188 | **131 ✅** | 148 | +57 | Higher | +40 | Higher | -17 | Lower |
| LLOC | **103 ✅** | 127 | 109 | -24 | Lower | -6 | Lower | +18 | Higher |
| Comments (CLOC) | 9 | **24 ✅** | 9 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | **18 ✅** | 20 | 30 | -2 | Lower | -12 | Lower | -10 | Lower |
| Functions | 12 | **2 ✅** | 6 | +10 | Higher | +6 | Higher | -4 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 14 | **3 ✅** | 8 | +11 | Higher | +6 | Higher | -5 | Lower |

## p00655

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 19.1645 | 18.1531 | **25.1048 ✅** | +1.0114 | Higher | -5.9403 | Lower | -6.9517 | Lower |
| MI (SEI) | -28.5202 | -29.6071 | **-23.3224 ✅** | +1.0869 | Higher | -5.1978 | Lower | -6.2847 | Lower |
| Cyclomatic (sum) | 61 | 72 | **55 ✅** | -11 | Lower | +6 | Higher | +17 | Higher |
| Cognitive (sum) | 55 | 85 | **48 ✅** | -30 | Lower | +7 | Higher | +37 | Higher |
| Halstead Volume | 9226.5055 | **7225.4325 ✅** | 7602.6733 | +2001.0730 | Higher | +1623.8322 | Higher | -377.2408 | Lower |
| Halstead Effort | 1188748.3163 | **691648.2969 ✅** | 959872.0649 | +497100.0194 | Higher | +228876.2514 | Higher | -268223.7680 | Lower |
| SLOC | 264 | 260 | **212 ✅** | +4 | Higher | +52 | Higher | +48 | Higher |
| PLOC | 238 | 231 | **190 ✅** | +7 | Higher | +48 | Higher | +41 | Higher |
| LLOC | 155 | **109 ✅** | 116 | +46 | Higher | +39 | Higher | -7 | Lower |
| Comments (CLOC) | **8 ✅** | 7 | 4 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | 20 | 24 | **18 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Functions | 13 | 13 | **11 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 13 | 13 | **11 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -10.0371 | **-8.3995 ✅** | -9.4033 | -1.6376 | Lower | -0.6338 | Lower | +1.0037 | Higher |
| MI (SEI) | **-59.4154 ✅** | -63.7828 | -62.1162 | +4.3673 | Higher | +2.7007 | Higher | -1.6666 | Lower |
| Cyclomatic (sum) | 140 | **134 ✅** | 138 | +6 | Higher | +2 | Higher | -4 | Lower |
| Cognitive (sum) | 178 | **111 ✅** | 181 | +67 | Higher | -3 | Lower | -70 | Lower |
| Halstead Volume | 15133.5808 | 15131.5041 | **14738.9583 ✅** | +2.0767 | Higher | +394.6225 | Higher | +392.5458 | Higher |
| Halstead Effort | 1998396.9911 | **1501966.2575 ✅** | 2037415.3350 | +496430.7336 | Higher | -39018.3439 | Lower | -535449.0775 | Lower |
| SLOC | 445 | **438 ✅** | 444 | +7 | Higher | +1 | Higher | -6 | Lower |
| PLOC | 396 | **390 ✅** | 401 | +6 | Higher | -5 | Lower | -11 | Lower |
| LLOC | 260 | **233 ✅** | 264 | +27 | Higher | -4 | Lower | -31 | Lower |
| Comments (CLOC) | **21 ✅** | 8 | 13 | +13 | Higher | +8 | Higher | -5 | Lower |
| Blank Lines | **28 ✅** | 40 | 31 | -12 | Lower | -3 | Lower | +9 | Higher |
| Functions | 12 | 23 | 12 | -11 | Lower | 0 | Same | +11 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **21 ✅** | 35 | 28 | -14 | Lower | -7 | Lower | +7 | Higher |

## p00680

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **18.2286 ✅** | 14.1084 | 12.9954 | +4.1202 | Higher | +5.2331 | Higher | +1.1129 | Higher |
| MI (SEI) | **-26.8374 ✅** | -29.7124 | -40.7097 | +2.8749 | Higher | +13.8723 | Higher | +10.9974 | Higher |
| Cyclomatic (sum) | **61 ✅** | 71 | 67 | -10 | Lower | -6 | Lower | +4 | Higher |
| Cognitive (sum) | **82 ✅** | 107 | 88 | -25 | Lower | -6 | Lower | +19 | Higher |
| Halstead Volume | **9094.2591 ✅** | 10100.4997 | 11137.7001 | -1006.2406 | Lower | -2043.4411 | Lower | -1037.2005 | Lower |
| Halstead Effort | **791481.8039 ✅** | 923084.6634 | 1038905.7532 | -131602.8595 | Lower | -247423.9493 | Lower | -115821.0898 | Lower |
| SLOC | **281 ✅** | 304 | 334 | -23 | Lower | -53 | Lower | -30 | Lower |
| PLOC | **237 ✅** | 253 | 300 | -16 | Lower | -63 | Lower | -47 | Lower |
| LLOC | **152 ✅** | 161 | 182 | -9 | Lower | -30 | Lower | -21 | Lower |
| Comments (CLOC) | 13 | **18 ✅** | 5 | -5 | Lower | +8 | Higher | +13 | Higher |
| Blank Lines | 32 | 34 | **29 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Functions | 10 | **8 ✅** | 16 | +2 | Higher | -6 | Lower | -8 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | **10 ✅** | 11 | 13 | -1 | Lower | -3 | Lower | -2 | Lower |

## p00690

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.5044 | **46.4204 ✅** | 43.6177 | -2.9161 | Lower | -0.1133 | Lower | +2.8028 | Higher |
| MI (SEI) | 12.0791 | **18.6657 ✅** | 4.8880 | -6.5866 | Lower | +7.1911 | Higher | +13.7777 | Higher |
| Cyclomatic (sum) | 23 | **18 ✅** | 26 | +5 | Higher | -3 | Lower | -8 | Lower |
| Cognitive (sum) | 42 | **24 ✅** | 30 | +18 | Higher | +12 | Higher | -6 | Lower |
| Halstead Volume | 4174.8662 | **3358.9976 ✅** | 3664.3761 | +815.8686 | Higher | +510.4901 | Higher | -305.3786 | Lower |
| Halstead Effort | 209064.4548 | **133229.4716 ✅** | 192761.4531 | +75834.9833 | Higher | +16303.0017 | Higher | -59531.9816 | Lower |
| SLOC | 130 | **125 ✅** | 129 | +5 | Higher | +1 | Higher | -4 | Lower |
| PLOC | 103 | **100 ✅** | 113 | +3 | Higher | -10 | Lower | -13 | Lower |
| LLOC | 81 | **61 ✅** | 66 | +20 | Higher | +15 | Higher | -5 | Lower |
| Comments (CLOC) | 12 | **15 ✅** | 5 | -3 | Lower | +7 | Higher | +10 | Higher |
| Blank Lines | 15 | 15 | **11 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00692

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.4131 | **46.1566 ✅** | 45.2317 | -1.7436 | Lower | -0.8186 | Lower | +0.9250 | Higher |
| MI (SEI) | 15.1557 | 16.6797 | **21.2486 ✅** | -1.5240 | Lower | -6.0929 | Lower | -4.5689 | Lower |
| Cyclomatic (sum) | 31 | **26 ✅** | 34 | +5 | Higher | -3 | Lower | -8 | Lower |
| Cognitive (sum) | **25 ✅** | 46 | 37 | -21 | Lower | -12 | Lower | +9 | Higher |
| Halstead Volume | 4260.4707 | **3305.2300 ✅** | 4169.2758 | +955.2407 | Higher | +91.1949 | Higher | -864.0458 | Lower |
| Halstead Effort | 304344.2826 | **173073.8625 ✅** | 284360.0505 | +131270.4201 | Higher | +19984.2321 | Higher | -111286.1880 | Lower |
| SLOC | 109 | 114 | **100 ✅** | -5 | Lower | +9 | Higher | +14 | Higher |
| PLOC | 85 | 85 | **75 ✅** | 0 | Same | +10 | Higher | +10 | Higher |
| LLOC | 64 | 64 | 65 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 11 | 11 | **15 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 15 | 18 | **11 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| Functions | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 5 | 3 | **2 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p00723

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.1937 | **51.7823 ✅** | 50.2844 | -1.5886 | Lower | -0.0907 | Lower | +1.4979 | Higher |
| MI (SEI) | 20.4503 | **34.8673 ✅** | 19.9933 | -14.4170 | Lower | +0.4570 | Higher | +14.8740 | Higher |
| Cyclomatic (sum) | 17 | **10 ✅** | 19 | +7 | Higher | -2 | Lower | -9 | Lower |
| Cognitive (sum) | 11 | **4 ✅** | 29 | +7 | Higher | -18 | Lower | -25 | Lower |
| Halstead Volume | 3013.9182 | **2614.4844 ✅** | 3063.3750 | +399.4337 | Higher | -49.4569 | Lower | -448.8906 | Lower |
| Halstead Effort | 140996.5718 | **102876.9227 ✅** | 171625.5867 | +38119.6492 | Higher | -30629.0148 | Lower | -68748.6640 | Lower |
| SLOC | 104 | 109 | **100 ✅** | -5 | Lower | +4 | Higher | +9 | Higher |
| PLOC | 81 | **63 ✅** | 72 | +18 | Higher | +9 | Higher | -9 | Lower |
| LLOC | 51 | **44 ✅** | 48 | +7 | Higher | +3 | Higher | -4 | Lower |
| Comments (CLOC) | 9 | **27 ✅** | 8 | -18 | Lower | +1 | Higher | +19 | Higher |
| Blank Lines | **14 ✅** | 19 | 20 | -5 | Lower | -6 | Lower | -1 | Lower |
| Functions | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p00740

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.9353 ✅** | 59.9802 | 62.6189 | +2.9550 | Higher | +0.3164 | Higher | -2.6387 | Lower |
| MI (SEI) | **39.3587 ✅** | 33.4723 | 34.8104 | +5.8864 | Higher | +4.5482 | Higher | -1.3381 | Lower |
| Cyclomatic (sum) | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 18 | 18 | 21 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | **1347.3976 ✅** | 1451.1213 | 1364.4009 | -103.7237 | Lower | -17.0033 | Lower | +86.7204 | Higher |
| Halstead Effort | **38217.0954 ✅** | 40147.6890 | 44911.5294 | -1930.5936 | Lower | -6694.4340 | Lower | -4763.8404 | Lower |
| SLOC | **64 ✅** | 75 | 65 | -11 | Lower | -1 | Lower | +10 | Higher |
| PLOC | **50 ✅** | 57 | 53 | -7 | Lower | -3 | Lower | +4 | Higher |
| LLOC | **40 ✅** | 49 | 43 | -9 | Lower | -3 | Lower | +6 | Higher |
| Comments (CLOC) | 6 | 6 | 4 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 11 | 12 | **10 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00749

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 20.2244 | 18.0139 | **21.0165 ✅** | +2.2105 | Higher | -0.7921 | Lower | -3.0026 | Lower |
| MI (SEI) | **-20.1852 ✅** | -23.2816 | -23.3841 | +3.0964 | Higher | +3.1989 | Higher | +0.1025 | Higher |
| Cyclomatic (sum) | 57 | 67 | **55 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| Cognitive (sum) | **78 ✅** | 124 | 91 | -46 | Lower | -13 | Lower | +33 | Higher |
| Halstead Volume | **8570.5535 ✅** | 8623.2034 | 9051.5430 | -52.6499 | Lower | -480.9894 | Lower | -428.3395 | Lower |
| Halstead Effort | 796849.8610 | **763920.0101 ✅** | 828424.2625 | +32929.8509 | Higher | -31574.4016 | Lower | -64504.2525 | Lower |
| SLOC | 268 | 266 | **258 ✅** | +2 | Higher | +10 | Higher | +8 | Higher |
| PLOC | 225 | 223 | **220 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| LLOC | 166 | **156 ✅** | 169 | +10 | Higher | -3 | Lower | -13 | Lower |
| Comments (CLOC) | **20 ✅** | 18 | 12 | +2 | Higher | +8 | Higher | +6 | Higher |
| Blank Lines | 28 | 29 | **27 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 6 | **1 ✅** | 6 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 13 | **0 ✅** | 8 | +13 | Higher | +5 | Higher | -8 | Lower |

## p00762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **37.5597 ✅** | 27.0521 | 35.4757 | +10.5076 | Higher | +2.0840 | Higher | -8.4236 | Lower |
| MI (SEI) | **5.5855 ✅** | -11.7330 | 4.1413 | +17.3185 | Higher | +1.4442 | Higher | -15.8743 | Lower |
| Cyclomatic (sum) | **31 ✅** | 42 | 32 | -11 | Lower | -1 | Lower | +10 | Higher |
| Cognitive (sum) | **19 ✅** | 59 | 20 | -40 | Lower | -1 | Lower | +39 | Higher |
| Halstead Volume | **5422.9919 ✅** | 6479.3252 | 5798.6491 | -1056.3333 | Lower | -375.6573 | Lower | +680.6761 | Higher |
| Halstead Effort | **422993.3651 ✅** | 530286.4873 | 437465.3012 | -107293.1223 | Lower | -14471.9361 | Lower | +92821.1862 | Higher |
| SLOC | **154 ✅** | 238 | 169 | -84 | Lower | -15 | Lower | +69 | Higher |
| PLOC | **126 ✅** | 197 | 135 | -71 | Lower | -9 | Lower | +62 | Higher |
| LLOC | **100 ✅** | 148 | 107 | -48 | Lower | -7 | Lower | +41 | Higher |
| Comments (CLOC) | 16 | 18 | **20 ✅** | -2 | Lower | -4 | Lower | -2 | Lower |
| Blank Lines | **21 ✅** | 24 | 23 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |

## p00792

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.2288 | **79.6974 ✅** | 50.6735 | -29.4686 | Lower | -0.4447 | Lower | +29.0239 | Higher |
| MI (SEI) | 26.4328 | **70.4660 ✅** | 22.7183 | -44.0332 | Lower | +3.7145 | Higher | +47.7477 | Higher |
| Cyclomatic (sum) | 20 | **5 ✅** | 21 | +15 | Higher | -1 | Lower | -16 | Lower |
| Cognitive (sum) | 22 | **3 ✅** | 24 | +19 | Higher | -2 | Lower | -21 | Lower |
| Halstead Volume | 3154.8203 | **628.9629 ✅** | 3261.9521 | +2525.8574 | Higher | -107.1318 | Lower | -2632.9892 | Lower |
| Halstead Effort | 186941.4431 | **8748.3017 ✅** | 198163.5876 | +178193.1415 | Higher | -11222.1445 | Lower | -189415.2860 | Lower |
| SLOC | 98 | **33 ✅** | 93 | +65 | Higher | +5 | Higher | -60 | Lower |
| PLOC | 73 | **21 ✅** | 73 | +52 | Higher | 0 | Same | -52 | Lower |
| LLOC | 70 | **15 ✅** | 70 | +55 | Higher | 0 | Same | -55 | Lower |
| Comments (CLOC) | **14 ✅** | 6 | 9 | +8 | Higher | +5 | Higher | -3 | Lower |
| Blank Lines | 12 | **6 ✅** | 12 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00799

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.8726 | 40.0264 | **44.8556 ✅** | +3.8462 | Higher | -0.9830 | Lower | -4.8293 | Lower |
| MI (SEI) | **18.5701 ✅** | 4.6723 | 11.3494 | +13.8978 | Higher | +7.2207 | Higher | -6.6771 | Lower |
| Cyclomatic (sum) | **23 ✅** | 31 | 24 | -8 | Lower | -1 | Lower | +7 | Higher |
| Cognitive (sum) | **44 ✅** | 46 | 47 | -2 | Lower | -3 | Lower | -1 | Lower |
| Halstead Volume | **2954.1390 ✅** | 3662.9709 | 3155.2034 | -708.8319 | Lower | -201.0644 | Lower | +507.7675 | Higher |
| Halstead Effort | **131089.9200 ✅** | 194070.8588 | 170347.4173 | -62980.9387 | Lower | -39257.4973 | Lower | +23723.4415 | Higher |
| SLOC | 142 | 150 | **129 ✅** | -8 | Lower | +13 | Higher | +21 | Higher |
| PLOC | **104 ✅** | 121 | 108 | -17 | Lower | -4 | Lower | +13 | Higher |
| LLOC | **68 ✅** | 76 | 83 | -8 | Lower | -15 | Lower | -7 | Lower |
| Comments (CLOC) | **22 ✅** | 10 | 9 | +12 | Higher | +13 | Higher | +1 | Higher |
| Blank Lines | 17 | 20 | **14 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Functions | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00811

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.7468 ✅** | 45.6274 | 51.7172 | +7.1194 | Higher | +1.0296 | Higher | -6.0898 | Lower |
| MI (SEI) | **18.3724 ✅** | 15.4391 | 16.5539 | +2.9333 | Higher | +1.8185 | Higher | -1.1148 | Lower |
| Cyclomatic (sum) | 19 | 32 | 19 | -13 | Lower | 0 | Same | +13 | Higher |
| Cognitive (sum) | **23 ✅** | 37 | 24 | -14 | Lower | -1 | Lower | +13 | Higher |
| Halstead Volume | **2649.1213 ✅** | 2731.0729 | 2820.0539 | -81.9517 | Lower | -170.9327 | Lower | -88.9810 | Lower |
| Halstead Effort | 133221.3650 | **118801.6728 ✅** | 151405.5628 | +14419.6922 | Higher | -18184.1978 | Lower | -32603.8901 | Lower |
| SLOC | **90 ✅** | 115 | 94 | -25 | Lower | -4 | Lower | +21 | Higher |
| PLOC | **70 ✅** | 90 | 74 | -20 | Lower | -4 | Lower | +16 | Higher |
| LLOC | 60 | **58 ✅** | 59 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 4 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 16 | 16 | 17 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00816

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.3241 ✅** | 46.5102 | 40.5055 | +5.8139 | Higher | +11.8186 | Higher | +6.0047 | Higher |
| MI (SEI) | **26.6390 ✅** | 15.8556 | -1.8377 | +10.7834 | Higher | +28.4767 | Higher | +17.6933 | Higher |
| Cyclomatic (sum) | **19 ✅** | 27 | 37 | -8 | Lower | -18 | Lower | -10 | Lower |
| Cognitive (sum) | **31 ✅** | 43 | 45 | -12 | Lower | -14 | Lower | -2 | Lower |
| Halstead Volume | **2275.4064 ✅** | 2584.5501 | 4001.0421 | -309.1437 | Lower | -1725.6356 | Lower | -1416.4920 | Lower |
| Halstead Effort | **96307.9002 ✅** | 133736.7204 | 266103.2045 | -37428.8202 | Lower | -169795.3043 | Lower | -132366.4841 | Lower |
| SLOC | **97 ✅** | 119 | 130 | -22 | Lower | -33 | Lower | -11 | Lower |
| PLOC | **76 ✅** | 95 | 109 | -19 | Lower | -33 | Lower | -14 | Lower |
| LLOC | **58 ✅** | 62 | 84 | -4 | Lower | -26 | Lower | -22 | Lower |
| Comments (CLOC) | **11 ✅** | 10 | 3 | +1 | Higher | +8 | Higher | +7 | Higher |
| Blank Lines | **13 ✅** | 17 | 18 | -4 | Lower | -5 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 4 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 4 | +1 | Higher | -3 | Lower | -4 | Lower |

## p00820

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.7303 ✅** | 68.1162 | 73.9283 | +8.6141 | Higher | +2.8020 | Higher | -5.8121 | Lower |
| MI (SEI) | 48.9406 | 51.8980 | **58.6153 ✅** | -2.9574 | Lower | -9.6747 | Lower | -6.7173 | Lower |
| Cyclomatic (sum) | **8 ✅** | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **7 ✅** | 12 | 9 | -5 | Lower | -2 | Lower | +3 | Higher |
| Halstead Volume | **888.0090 ✅** | 1068.5533 | 985.2023 | -180.5443 | Lower | -97.1933 | Lower | +83.3510 | Higher |
| Halstead Effort | **24509.0494 ✅** | 28868.1740 | 27268.9927 | -4359.1245 | Lower | -2759.9433 | Lower | +1599.1812 | Higher |
| SLOC | **34 ✅** | 53 | 38 | -19 | Lower | -4 | Lower | +15 | Higher |
| PLOC | **29 ✅** | 34 | 31 | -5 | Lower | -2 | Lower | +3 | Higher |
| LLOC | 21 | 21 | **20 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 1 | **8 ✅** | 5 | -7 | Lower | -4 | Lower | +3 | Higher |
| Blank Lines | 4 | 11 | 4 | -7 | Lower | 0 | Same | +7 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.1650 | **40.2610 ✅** | 36.1407 | -0.0961 | Lower | +4.0242 | Higher | +4.1203 | Higher |
| MI (SEI) | 8.4587 | **9.3596 ✅** | -0.3450 | -0.9009 | Lower | +8.8037 | Higher | +9.7046 | Higher |
| Cyclomatic (sum) | **23 ✅** | 28 | 31 | -5 | Lower | -8 | Lower | -3 | Lower |
| Cognitive (sum) | **44 ✅** | 62 | 51 | -18 | Lower | -7 | Lower | +11 | Higher |
| Halstead Volume | **4587.3758 ✅** | 4742.5834 | 5333.2274 | -155.2076 | Lower | -745.8516 | Lower | -590.6440 | Lower |
| Halstead Effort | **325945.1204 ✅** | 330224.3249 | 426260.1898 | -4279.2044 | Lower | -100315.0693 | Lower | -96035.8649 | Lower |
| SLOC | 155 | **142 ✅** | 169 | +13 | Higher | -14 | Lower | -27 | Lower |
| PLOC | 114 | **109 ✅** | 136 | +5 | Higher | -22 | Lower | -27 | Lower |
| LLOC | 111 | **80 ✅** | 123 | +31 | Higher | -12 | Lower | -43 | Lower |
| Comments (CLOC) | **16 ✅** | 15 | 12 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | 26 | **20 ✅** | 22 | +6 | Higher | +4 | Higher | -2 | Lower |
| Functions | **1 ✅** | 5 | 2 | -4 | Lower | -1 | Lower | +3 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 4 | 2 | -4 | Lower | -2 | Lower | +2 | Higher |

## p00846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **40.6167 ✅** | 30.2252 | 39.4644 | +10.3914 | Higher | +1.1523 | Higher | -9.2392 | Lower |
| MI (SEI) | **6.3645 ✅** | -10.8611 | 0.0997 | +17.2255 | Higher | +6.2648 | Higher | -10.9608 | Lower |
| Cyclomatic (sum) | **20 ✅** | 51 | 21 | -31 | Lower | -1 | Lower | +30 | Higher |
| Cognitive (sum) | 21 | 42 | 21 | -21 | Lower | 0 | Same | +21 | Higher |
| Halstead Volume | **4436.0221 ✅** | 5643.4064 | 4719.5772 | -1207.3843 | Lower | -283.5551 | Lower | +923.8292 | Higher |
| Halstead Effort | **222216.9836 ✅** | 377833.8996 | 267493.4562 | -155616.9160 | Lower | -45276.4726 | Lower | +110340.4434 | Higher |
| SLOC | **159 ✅** | 180 | 165 | -21 | Lower | -6 | Lower | +15 | Higher |
| PLOC | **128 ✅** | 149 | 136 | -21 | Lower | -8 | Lower | +13 | Higher |
| LLOC | **49 ✅** | 77 | 67 | -28 | Lower | -18 | Lower | +10 | Higher |
| Comments (CLOC) | **13 ✅** | 8 | 8 | +5 | Higher | +5 | Higher | 0 | Same |
| Blank Lines | **20 ✅** | 23 | 21 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 18 | 4 | -14 | Lower | 0 | Same | +14 | Higher |

## p00855

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.5496 ✅** | 47.1362 | 62.4097 | +22.4134 | Higher | +7.1399 | Higher | -15.2735 | Lower |
| MI (SEI) | **49.5039 ✅** | 14.1998 | 34.8508 | +35.3041 | Higher | +14.6531 | Higher | -20.6510 | Lower |
| Cyclomatic (sum) | **11 ✅** | 28 | 16 | -17 | Lower | -5 | Lower | +12 | Higher |
| Cognitive (sum) | **12 ✅** | 29 | 13 | -17 | Lower | -1 | Lower | +16 | Higher |
| Halstead Volume | **990.9302 ✅** | 2800.9038 | 1364.4917 | -1809.9736 | Lower | -373.5615 | Lower | +1436.4121 | Higher |
| Halstead Effort | **29581.1027 ✅** | 154049.7112 | 37296.1072 | -124468.6085 | Lower | -7715.0045 | Lower | +116753.6039 | Higher |
| SLOC | **49 ✅** | 110 | 64 | -61 | Lower | -15 | Lower | +46 | Higher |
| PLOC | **36 ✅** | 92 | 52 | -56 | Lower | -16 | Lower | +40 | Higher |
| LLOC | **25 ✅** | 60 | 31 | -35 | Lower | -6 | Lower | +29 | Higher |
| Comments (CLOC) | 5 | **7 ✅** | 4 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | **2 ✅** | 6 | 4 | -4 | Lower | -2 | Lower | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 11 | 7 | -10 | Lower | -6 | Lower | +4 | Higher |

## p00856

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **47.4361 ✅** | 47.0671 | 46.0372 | +0.3690 | Higher | +1.3990 | Higher | +1.0299 | Higher |
| MI (SEI) | **19.2095 ✅** | 11.8923 | 12.7987 | +7.3171 | Higher | +6.4107 | Higher | -0.9064 | Lower |
| Cyclomatic (sum) | **26 ✅** | 29 | 29 | -3 | Lower | -3 | Lower | 0 | Same |
| Cognitive (sum) | **27 ✅** | 43 | 28 | -16 | Lower | -1 | Lower | +15 | Higher |
| Halstead Volume | **3148.3202 ✅** | 3332.8833 | 3405.9203 | -184.5631 | Lower | -257.6001 | Lower | -73.0370 | Lower |
| Halstead Effort | **178265.5994 ✅** | 179095.3132 | 188713.2148 | -829.7138 | Lower | -10447.6154 | Lower | -9617.9015 | Lower |
| SLOC | 107 | **103 ✅** | 109 | +4 | Higher | -2 | Lower | -6 | Lower |
| PLOC | 81 | 81 | 86 | 0 | Same | -5 | Lower | -5 | Lower |
| LLOC | 50 | **47 ✅** | 54 | +3 | Higher | -4 | Lower | -7 | Lower |
| Comments (CLOC) | **11 ✅** | 5 | 7 | +6 | Higher | +4 | Higher | -2 | Lower |
| Blank Lines | **15 ✅** | 17 | 17 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |

## p00866

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **47.1700 ✅** | 37.6381 | 45.7864 | +9.5319 | Higher | +1.3836 | Higher | -8.1483 | Lower |
| MI (SEI) | **13.9691 ✅** | 13.5218 | 11.8622 | +0.4474 | Higher | +2.1069 | Higher | +1.6595 | Higher |
| Cyclomatic (sum) | **18 ✅** | 27 | 20 | -9 | Lower | -2 | Lower | +7 | Higher |
| Cognitive (sum) | **25 ✅** | 49 | 31 | -24 | Lower | -6 | Lower | +18 | Higher |
| Halstead Volume | **3670.3962 ✅** | 4953.1292 | 3944.3925 | -1282.7330 | Lower | -273.9964 | Lower | +1008.7366 | Higher |
| Halstead Effort | **171679.8212 ✅** | 325717.7740 | 207080.6082 | -154037.9528 | Lower | -35400.7871 | Lower | +118637.1658 | Higher |
| SLOC | **116 ✅** | 167 | 120 | -51 | Lower | -4 | Lower | +47 | Higher |
| PLOC | **93 ✅** | 113 | 96 | -20 | Lower | -3 | Lower | +17 | Higher |
| LLOC | 59 | 90 | 59 | -31 | Lower | 0 | Same | +31 | Higher |
| Comments (CLOC) | 8 | **34 ✅** | 8 | -26 | Lower | 0 | Same | +26 | Higher |
| Blank Lines | 16 | 21 | 16 | -5 | Lower | 0 | Same | +5 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00867

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 11.9614 | **23.2673 ✅** | 4.8625 | -11.3059 | Lower | +7.0989 | Higher | +18.4048 | Higher |
| MI (SEI) | -31.7969 | **-18.9016 ✅** | -48.2476 | -12.8953 | Lower | +16.4507 | Higher | +29.3460 | Higher |
| Cyclomatic (sum) | 74 | **58 ✅** | 94 | +16 | Higher | -20 | Lower | -36 | Lower |
| Cognitive (sum) | 94 | **91 ✅** | 111 | +3 | Higher | -17 | Lower | -20 | Lower |
| Halstead Volume | 12327.6897 | **7354.3475 ✅** | 13934.3279 | +4973.3422 | Higher | -1606.6382 | Lower | -6579.9804 | Lower |
| Halstead Effort | 1304680.4942 | **456719.9862 ✅** | 1625349.6713 | +847960.5080 | Higher | -320669.1771 | Lower | -1168629.6851 | Lower |
| SLOC | 312 | **230 ✅** | 350 | +82 | Higher | -38 | Lower | -120 | Lower |
| PLOC | 263 | **191 ✅** | 319 | +72 | Higher | -56 | Lower | -128 | Lower |
| LLOC | 188 | **133 ✅** | 201 | +55 | Higher | -13 | Lower | -68 | Lower |
| Comments (CLOC) | **20 ✅** | 12 | 7 | +8 | Higher | +13 | Higher | +5 | Higher |
| Blank Lines | 33 | 27 | 27 | +6 | Higher | +6 | Higher | 0 | Same |
| Functions | 8 | **5 ✅** | 16 | +3 | Higher | -8 | Lower | -11 | Lower |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | **11 ✅** | 15 | 17 | -4 | Lower | -6 | Lower | -2 | Lower |

## p00870

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.8888 ✅** | 53.7851 | 48.1596 | +3.1037 | Higher | +8.7292 | Higher | +5.6255 | Higher |
| MI (SEI) | **30.7548 ✅** | 20.6098 | 20.5653 | +10.1450 | Higher | +10.1895 | Higher | +0.0445 | Higher |
| Cyclomatic (sum) | **17 ✅** | 20 | 26 | -3 | Lower | -9 | Lower | -6 | Lower |
| Cognitive (sum) | 27 | 27 | 45 | 0 | Same | -18 | Lower | -18 | Lower |
| Halstead Volume | **2209.7674 ✅** | 2671.2959 | 2993.1856 | -461.5286 | Lower | -783.4182 | Lower | -321.8896 | Lower |
| Halstead Effort | **96356.1352 ✅** | 127739.0986 | 166620.6632 | -31382.9633 | Lower | -70264.5280 | Lower | -38881.5647 | Lower |
| SLOC | **76 ✅** | 83 | 104 | -7 | Lower | -28 | Lower | -21 | Lower |
| PLOC | **58 ✅** | 67 | 84 | -9 | Lower | -26 | Lower | -17 | Lower |
| LLOC | **45 ✅** | 51 | 58 | -6 | Lower | -13 | Lower | -7 | Lower |
| Comments (CLOC) | 7 | 4 | **11 ✅** | +3 | Higher | -4 | Lower | -7 | Lower |
| Blank Lines | 11 | 12 | **9 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00874

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.8558 ✅** | 68.5054 | 50.7702 | +1.3504 | Higher | +19.0856 | Higher | +17.7352 | Higher |
| MI (SEI) | 48.0515 | **51.7822 ✅** | 20.4007 | -3.7307 | Lower | +27.6508 | Higher | +31.3815 | Higher |
| Cyclomatic (sum) | **9 ✅** | 10 | 24 | -1 | Lower | -15 | Lower | -14 | Lower |
| Cognitive (sum) | 8 | 8 | 36 | 0 | Same | -28 | Lower | -28 | Lower |
| Halstead Volume | **1242.6959 ✅** | 1266.0753 | 2624.1296 | -23.3794 | Lower | -1381.4337 | Lower | -1358.0543 | Lower |
| Halstead Effort | **31440.2052 ✅** | 35956.5371 | 131964.5597 | -4516.3319 | Lower | -100524.3544 | Lower | -96008.0225 | Lower |
| SLOC | **46 ✅** | 49 | 95 | -3 | Lower | -49 | Lower | -46 | Lower |
| PLOC | 35 | 35 | 79 | 0 | Same | -44 | Lower | -44 | Lower |
| LLOC | 29 | **27 ✅** | 58 | +2 | Higher | -29 | Lower | -31 | Lower |
| Comments (CLOC) | 4 | 7 | 7 | -3 | Lower | -3 | Lower | 0 | Same |
| Blank Lines | 9 | **8 ✅** | 10 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p00889

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.5993 | **52.7031 ✅** | 33.7340 | -12.1038 | Lower | +6.8653 | Higher | +18.9691 | Higher |
| MI (SEI) | 10.7156 | **37.9320 ✅** | -5.8076 | -27.2164 | Lower | +16.5232 | Higher | +43.7396 | Higher |
| Cyclomatic (sum) | 31 | **20 ✅** | 39 | +11 | Higher | -8 | Lower | -19 | Lower |
| Cognitive (sum) | 43 | **21 ✅** | 60 | +22 | Higher | -17 | Lower | -39 | Lower |
| Halstead Volume | 4555.4562 | **2386.7083 ✅** | 5430.9296 | +2168.7479 | Higher | -875.4733 | Lower | -3044.2212 | Lower |
| Halstead Effort | 390166.3916 | **88407.6537 ✅** | 458913.5471 | +301758.7380 | Higher | -68747.1555 | Lower | -370505.8935 | Lower |
| SLOC | 135 | **92 ✅** | 174 | +43 | Higher | -39 | Lower | -82 | Lower |
| PLOC | 108 | **60 ✅** | 148 | +48 | Higher | -40 | Lower | -88 | Lower |
| LLOC | 100 | **44 ✅** | 118 | +56 | Higher | -18 | Lower | -74 | Lower |
| Comments (CLOC) | 15 | **24 ✅** | 9 | -9 | Lower | +6 | Higher | +15 | Higher |
| Blank Lines | 16 | **11 ✅** | 18 | +5 | Higher | -2 | Lower | -7 | Lower |
| Functions | 1 | 1 | 4 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 6 | 3 | **0 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| Exits | 5 | **1 ✅** | 7 | +4 | Higher | -2 | Lower | -6 | Lower |

## p00897

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 33.1711 | **43.1731 ✅** | 21.5965 | -10.0020 | Lower | +11.5746 | Higher | +21.5766 | Higher |
| MI (SEI) | -2.9871 | **10.9027 ✅** | -16.8517 | -13.8898 | Lower | +13.8646 | Higher | +27.7544 | Higher |
| Cyclomatic (sum) | 35 | **20 ✅** | 52 | +15 | Higher | -17 | Lower | -32 | Lower |
| Cognitive (sum) | 39 | **27 ✅** | 50 | +12 | Higher | -11 | Lower | -23 | Lower |
| Halstead Volume | 5868.0158 | **4517.2843 ✅** | 8308.3043 | +1350.7315 | Higher | -2440.2885 | Lower | -3791.0200 | Lower |
| Halstead Effort | 348727.7967 | **223810.9027 ✅** | 579763.8593 | +124916.8939 | Higher | -231036.0626 | Lower | -355952.9565 | Lower |
| SLOC | 186 | **135 ✅** | 267 | +51 | Higher | -81 | Lower | -132 | Lower |
| PLOC | 154 | **106 ✅** | 223 | +48 | Higher | -69 | Lower | -117 | Lower |
| LLOC | 95 | **77 ✅** | 143 | +18 | Higher | -48 | Lower | -66 | Lower |
| Comments (CLOC) | 15 | 12 | **24 ✅** | +3 | Higher | -9 | Lower | -12 | Lower |
| Blank Lines | 20 | **19 ✅** | 24 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 6 | **1 ✅** | 11 | +5 | Higher | -5 | Lower | -10 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 7 | **0 ✅** | 13 | +7 | Higher | -6 | Lower | -13 | Lower |

## p00907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.0497 ✅** | 51.6211 | 45.2284 | +6.4286 | Higher | +12.8213 | Higher | +6.3928 | Higher |
| MI (SEI) | **36.1207 ✅** | 22.4294 | 14.9255 | +13.6913 | Higher | +21.1953 | Higher | +7.5039 | Higher |
| Cyclomatic (sum) | **20 ✅** | 21 | 35 | -1 | Lower | -15 | Lower | -14 | Lower |
| Cognitive (sum) | **28 ✅** | 33 | 48 | -5 | Lower | -20 | Lower | -15 | Lower |
| Halstead Volume | **1831.9589 ✅** | 2384.2647 | 3328.8051 | -552.3058 | Lower | -1496.8462 | Lower | -944.5404 | Lower |
| Halstead Effort | **85186.0905 ✅** | 99484.6153 | 214508.2034 | -14298.5248 | Lower | -129322.1129 | Lower | -115023.5881 | Lower |
| SLOC | **72 ✅** | 97 | 106 | -25 | Lower | -34 | Lower | -9 | Lower |
| PLOC | **57 ✅** | 74 | 88 | -17 | Lower | -31 | Lower | -14 | Lower |
| LLOC | **42 ✅** | 53 | 65 | -11 | Lower | -23 | Lower | -12 | Lower |
| Comments (CLOC) | 9 | 8 | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | **8 ✅** | 15 | 10 | -7 | Lower | -2 | Lower | +5 | Higher |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 6 | -1 | Lower | -5 | Lower | -4 | Lower |

## p00924

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.5681 ✅** | 48.4195 | 52.5052 | +4.1486 | Higher | +0.0629 | Higher | -4.0857 | Lower |
| MI (SEI) | 23.0279 | **23.9656 ✅** | 18.7633 | -0.9377 | Lower | +4.2647 | Higher | +5.2023 | Higher |
| Cyclomatic (sum) | **17 ✅** | 25 | 20 | -8 | Lower | -3 | Lower | +5 | Higher |
| Cognitive (sum) | **21 ✅** | 24 | 22 | -3 | Lower | -1 | Lower | +2 | Higher |
| Halstead Volume | **3212.5189 ✅** | 3260.1294 | 3416.7832 | -47.6105 | Lower | -204.2642 | Lower | -156.6537 | Lower |
| Halstead Effort | **162162.3694 ✅** | 182997.8300 | 216189.1888 | -20835.4606 | Lower | -54026.8194 | Lower | -33191.3588 | Lower |
| SLOC | 88 | 101 | **83 ✅** | -13 | Lower | +5 | Higher | +18 | Higher |
| PLOC | **70 ✅** | 72 | 72 | -2 | Lower | -2 | Lower | 0 | Same |
| LLOC | 58 | **51 ✅** | 61 | +7 | Higher | -3 | Lower | -10 | Lower |
| Comments (CLOC) | 7 | **14 ✅** | 4 | -7 | Lower | +3 | Higher | +10 | Higher |
| Blank Lines | 12 | 15 | **7 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 5 | **3 ✅** | 9 | +2 | Higher | -4 | Lower | -6 | Lower |

## p00949

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.4781 | **44.3411 ✅** | 38.4731 | -2.8630 | Lower | +3.0050 | Higher | +5.8680 | Higher |
| MI (SEI) | **8.9841 ✅** | 7.0695 | 1.0138 | +1.9146 | Higher | +7.9704 | Higher | +6.0557 | Higher |
| Cyclomatic (sum) | 25 | **23 ✅** | 25 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | **26 ✅** | 29 | 32 | -3 | Lower | -6 | Lower | -3 | Lower |
| Halstead Volume | 3930.1955 | **3730.2655 ✅** | 4876.2273 | +199.9300 | Higher | -946.0317 | Lower | -1145.9617 | Lower |
| Halstead Effort | 269000.0498 | **191486.9636 ✅** | 370088.8355 | +77513.0862 | Higher | -101088.7857 | Lower | -178601.8719 | Lower |
| SLOC | 146 | **128 ✅** | 164 | +18 | Higher | -18 | Lower | -36 | Lower |
| PLOC | 112 | **104 ✅** | 129 | +8 | Higher | -17 | Lower | -25 | Lower |
| LLOC | 98 | **74 ✅** | 127 | +24 | Higher | -29 | Lower | -53 | Lower |
| Comments (CLOC) | **13 ✅** | 6 | 10 | +7 | Higher | +3 | Higher | -4 | Lower |
| Blank Lines | 22 | **18 ✅** | 25 | +4 | Higher | -3 | Lower | -7 | Lower |
| Functions | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 10 | **4 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |

## p00971

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.3871 | 22.1207 | **45.6648 ✅** | +20.2664 | Higher | -3.2777 | Lower | -23.5441 | Lower |
| MI (SEI) | **17.7000 ✅** | -0.5065 | 11.1014 | +18.2065 | Higher | +6.5986 | Higher | -11.6079 | Lower |
| Cyclomatic (sum) | 22 | 49 | **15 ✅** | -27 | Lower | +7 | Higher | +34 | Higher |
| Cognitive (sum) | 42 | 85 | **15 ✅** | -43 | Lower | +27 | Higher | +70 | Higher |
| Halstead Volume | **4391.3172 ✅** | 6849.0230 | 4908.5309 | -2457.7058 | Lower | -517.2137 | Lower | +1940.4921 | Higher |
| Halstead Effort | **344852.8265 ✅** | 464649.8594 | 381062.2745 | -119797.0329 | Lower | -36209.4480 | Lower | +83587.5849 | Higher |
| SLOC | 139 | 287 | **121 ✅** | -148 | Lower | +18 | Higher | +166 | Higher |
| PLOC | 102 | 179 | **98 ✅** | -77 | Lower | +4 | Higher | +81 | Higher |
| LLOC | 96 | 90 | **85 ✅** | +6 | Higher | +11 | Higher | +5 | Higher |
| Comments (CLOC) | 24 | **91 ✅** | 8 | -67 | Lower | +16 | Higher | +83 | Higher |
| Blank Lines | **15 ✅** | 23 | 16 | -8 | Lower | -1 | Lower | +7 | Higher |
| Functions | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 3 | -1 | Lower | -3 | Lower | -2 | Lower |

## p01050

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.4892 ✅** | 74.2800 | 70.5034 | +0.2092 | Higher | +3.9858 | Higher | +3.7766 | Higher |
| MI (SEI) | 62.3417 | **63.4640 ✅** | 55.0745 | -1.1223 | Lower | +7.2672 | Higher | +8.3894 | Higher |
| Cyclomatic (sum) | 7 | 7 | 9 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 12 | **9 ✅** | 13 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | 797.0770 | **769.7799 ✅** | 1026.0791 | +27.2972 | Higher | -229.0021 | Lower | -256.2993 | Lower |
| Halstead Effort | 20013.5642 | **15210.8500 ✅** | 29023.3817 | +4802.7142 | Higher | -9009.8174 | Lower | -13812.5317 | Lower |
| SLOC | **41 ✅** | 42 | 47 | -1 | Lower | -6 | Lower | -5 | Lower |
| PLOC | 30 | **28 ✅** | 35 | +2 | Higher | -5 | Lower | -7 | Lower |
| LLOC | 29 | **24 ✅** | 29 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | 7 | **8 ✅** | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p01065

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.4138 ✅** | 48.1015 | 50.5529 | +9.3123 | Higher | +6.8609 | Higher | -2.4514 | Lower |
| MI (SEI) | **35.8827 ✅** | 22.1892 | 25.6649 | +13.6935 | Higher | +10.2178 | Higher | -3.4757 | Lower |
| Cyclomatic (sum) | **16 ✅** | 21 | 19 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | **36 ✅** | 58 | 40 | -22 | Lower | -4 | Lower | +18 | Higher |
| Halstead Volume | **2367.0028 ✅** | 3262.1238 | 2909.2175 | -895.1209 | Lower | -542.2147 | Lower | +352.9062 | Higher |
| Halstead Effort | **111342.5683 ✅** | 151976.5889 | 162916.1824 | -40634.0206 | Lower | -51573.6141 | Lower | -10939.5935 | Lower |
| SLOC | **73 ✅** | 109 | 100 | -36 | Lower | -27 | Lower | +9 | Higher |
| PLOC | **52 ✅** | 81 | 75 | -29 | Lower | -23 | Lower | +6 | Higher |
| LLOC | **40 ✅** | 58 | 58 | -18 | Lower | -18 | Lower | 0 | Same |
| Comments (CLOC) | 10 | **14 ✅** | 13 | -4 | Lower | -3 | Lower | +1 | Higher |
| Blank Lines | **11 ✅** | 15 | 12 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p01106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.5268 ✅** | 54.3530 | 56.1111 | +3.1739 | Higher | +1.4157 | Higher | -1.7581 | Lower |
| MI (SEI) | 32.2616 | 33.1386 | **34.6076 ✅** | -0.8769 | Lower | -2.3460 | Lower | -1.4691 | Lower |
| Cyclomatic (sum) | 20 | 22 | 20 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 16 | 22 | 16 | -6 | Lower | 0 | Same | +6 | Higher |
| Halstead Volume | **1860 ✅** | 1893.2009 | 1992 | -33.2009 | Lower | -132 | Lower | -98.7991 | Lower |
| Halstead Effort | 63054 | **61639.1005 ✅** | 67047.8049 | +1414.8995 | Higher | -3993.8049 | Lower | -5408.7044 | Lower |
| SLOC | **74 ✅** | 87 | 79 | -13 | Lower | -5 | Lower | +8 | Higher |
| PLOC | 59 | 60 | 59 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 39 | **34 ✅** | 40 | +5 | Higher | -1 | Lower | -6 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 11 | -6 | Lower | -4 | Lower | +2 | Higher |
| Blank Lines | 11 | 14 | **9 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01119

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 27.4188 | **46.5877 ✅** | 26.5045 | -19.1689 | Lower | +0.9143 | Higher | +20.0831 | Higher |
| MI (SEI) | -10.2685 | **21.5871 ✅** | -16.2289 | -31.8556 | Lower | +5.9605 | Higher | +37.8160 | Higher |
| Cyclomatic (sum) | 50 | **24 ✅** | 51 | +26 | Higher | -1 | Lower | -27 | Lower |
| Cognitive (sum) | 63 | **27 ✅** | 68 | +36 | Higher | -5 | Lower | -41 | Lower |
| Halstead Volume | 5264.8929 | **2907.6576 ✅** | 5759.3983 | +2357.2352 | Higher | -494.5054 | Lower | -2851.7406 | Lower |
| Halstead Effort | 323366.3228 | **110103.3027 ✅** | 386329.6362 | +213263.0201 | Higher | -62963.3134 | Lower | -276226.3335 | Lower |
| SLOC | 222 | **119 ✅** | 225 | +103 | Higher | -3 | Lower | -106 | Lower |
| PLOC | 190 | **87 ✅** | 194 | +103 | Higher | -4 | Lower | -107 | Lower |
| LLOC | 130 | **58 ✅** | 154 | +72 | Higher | -24 | Lower | -96 | Lower |
| Comments (CLOC) | 17 | 17 | 10 | 0 | Same | +7 | Higher | +7 | Higher |
| Blank Lines | 17 | **15 ✅** | 22 | +2 | Higher | -5 | Lower | -7 | Lower |
| Functions | 10 | **1 ✅** | 8 | +9 | Higher | +2 | Higher | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 14 | **0 ✅** | 15 | +14 | Higher | -1 | Lower | -15 | Lower |

## p01125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.3753 ✅** | 52.4496 | 56.4491 | +5.9257 | Higher | +1.9262 | Higher | -3.9995 | Lower |
| MI (SEI) | **31.2904 ✅** | 27.0564 | 25.8277 | +4.2340 | Higher | +5.4628 | Higher | +1.2288 | Higher |
| Cyclomatic (sum) | 14 | 19 | 14 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | **14 ✅** | 20 | 17 | -6 | Lower | -3 | Lower | +3 | Higher |
| Halstead Volume | **2060.1653 ✅** | 2370.1075 | 2167.1368 | -309.9422 | Lower | -106.9715 | Lower | +202.9707 | Higher |
| Halstead Effort | **70162.6745 ✅** | 93867.2803 | 77139.7494 | -23704.6058 | Lower | -6977.0748 | Lower | +16727.5310 | Higher |
| SLOC | **74 ✅** | 95 | 82 | -21 | Lower | -8 | Lower | +13 | Higher |
| PLOC | **56 ✅** | 72 | 62 | -16 | Lower | -6 | Lower | +10 | Higher |
| LLOC | **44 ✅** | 52 | 49 | -8 | Lower | -5 | Lower | +3 | Higher |
| Comments (CLOC) | 6 | **11 ✅** | 5 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | **14 ✅** | 15 | 15 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p01128

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **46.6512 ✅** | 38.3218 | 41.6431 | +8.3294 | Higher | +5.0081 | Higher | -3.3213 | Lower |
| MI (SEI) | **18.7213 ✅** | 4.9320 | 8.0301 | +13.7894 | Higher | +10.6912 | Higher | -3.0981 | Lower |
| Cyclomatic (sum) | **21 ✅** | 23 | 25 | -2 | Lower | -4 | Lower | -2 | Lower |
| Cognitive (sum) | 30 | **19 ✅** | 39 | +11 | Higher | -9 | Lower | -20 | Lower |
| Halstead Volume | **3749.2248 ✅** | 5087.8535 | 4437.1629 | -1338.6287 | Lower | -687.9380 | Lower | +650.6907 | Higher |
| Halstead Effort | **173417.0137 ✅** | 227929.6324 | 216456.3791 | -54512.6186 | Lower | -43039.3654 | Lower | +11473.2532 | Higher |
| SLOC | **114 ✅** | 168 | 139 | -54 | Lower | -25 | Lower | +29 | Higher |
| PLOC | **81 ✅** | 125 | 107 | -44 | Lower | -26 | Lower | +18 | Higher |
| LLOC | 52 | **51 ✅** | 61 | +1 | Higher | -9 | Lower | -10 | Lower |
| Comments (CLOC) | 13 | **16 ✅** | 11 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 21 | 27 | 21 | -6 | Lower | 0 | Same | +6 | Higher |
| Functions | 2 | 7 | 2 | -5 | Lower | 0 | Same | +5 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **2 ✅** | 8 | 3 | -6 | Lower | -1 | Lower | +5 | Higher |

## p01138

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.1324 ✅** | 59.6165 | 47.7271 | +5.5159 | Higher | +17.4053 | Higher | +11.8894 | Higher |
| MI (SEI) | **39.2319 ✅** | 33.9385 | 19.1156 | +5.2934 | Higher | +20.1163 | Higher | +14.8229 | Higher |
| Cyclomatic (sum) | **10 ✅** | 14 | 26 | -4 | Lower | -16 | Lower | -12 | Lower |
| Cognitive (sum) | **8 ✅** | 13 | 27 | -5 | Lower | -19 | Lower | -14 | Lower |
| Halstead Volume | **1512 ✅** | 2111.7666 | 3455.6390 | -599.7666 | Lower | -1943.6390 | Lower | -1343.8724 | Lower |
| Halstead Effort | **33079.6098 ✅** | 59709.1640 | 174304.7716 | -26629.5543 | Lower | -141225.1619 | Lower | -114595.6076 | Lower |
| SLOC | **57 ✅** | 68 | 102 | -11 | Lower | -45 | Lower | -34 | Lower |
| PLOC | **43 ✅** | 51 | 80 | -8 | Lower | -37 | Lower | -29 | Lower |
| LLOC | **31 ✅** | 36 | 61 | -5 | Lower | -30 | Lower | -25 | Lower |
| Comments (CLOC) | 4 | 6 | **10 ✅** | -2 | Lower | -6 | Lower | -4 | Lower |
| Blank Lines | **11 ✅** | 12 | 13 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 2 | 2 | 5 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | 6 | 0 | Same | -5 | Lower | -5 | Lower |

## p01150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 27.9663 | **47.5528 ✅** | 32.3296 | -19.5866 | Lower | -4.3634 | Lower | +15.2232 | Higher |
| MI (SEI) | -4.8979 | **21.5792 ✅** | -6.8807 | -26.4771 | Lower | +1.9828 | Higher | +28.4599 | Higher |
| Cyclomatic (sum) | 42 | **22 ✅** | 42 | +20 | Higher | 0 | Same | -20 | Lower |
| Cognitive (sum) | 43 | **25 ✅** | 44 | +18 | Higher | -1 | Lower | -19 | Lower |
| Halstead Volume | 4965.4678 | **2856.9218 ✅** | 5704.3579 | +2108.5459 | Higher | -738.8902 | Lower | -2847.4361 | Lower |
| Halstead Effort | 493443.3579 | **138034.4345 ✅** | 575621.5733 | +355408.9234 | Higher | -82178.2153 | Lower | -437587.1388 | Lower |
| SLOC | 245 | **116 ✅** | 179 | +129 | Higher | +66 | Higher | -63 | Lower |
| PLOC | 193 | **86 ✅** | 144 | +107 | Higher | +49 | Higher | -58 | Lower |
| LLOC | 82 | **61 ✅** | 128 | +21 | Higher | -46 | Lower | -67 | Lower |
| Comments (CLOC) | **31 ✅** | 15 | 10 | +16 | Higher | +21 | Higher | +5 | Higher |
| Blank Lines | 33 | **17 ✅** | 28 | +16 | Higher | +5 | Higher | -11 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |

## p01153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.2677 ✅** | 59.0236 | 62.1282 | +4.2441 | Higher | +1.1395 | Higher | -3.1045 | Lower |
| MI (SEI) | 38.7933 | **39.3376 ✅** | 36.9151 | -0.5443 | Lower | +1.8782 | Higher | +2.4225 | Higher |
| Cyclomatic (sum) | 14 | 14 | 15 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 33 | **26 ✅** | 37 | +7 | Higher | -4 | Lower | -11 | Lower |
| Halstead Volume | **1628.5187 ✅** | 1980.7439 | 1748.4396 | -352.2251 | Lower | -119.9209 | Lower | +232.3042 | Higher |
| Halstead Effort | **70128.0872 ✅** | 73331.5393 | 76211.3974 | -3203.4521 | Lower | -6083.3102 | Lower | -2879.8581 | Lower |
| SLOC | **59 ✅** | 72 | 61 | -13 | Lower | -2 | Lower | +11 | Higher |
| PLOC | **45 ✅** | 50 | 47 | -5 | Lower | -2 | Lower | +3 | Higher |
| LLOC | **35 ✅** | 38 | 36 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 5 | **11 ✅** | 5 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | **9 ✅** | 11 | 11 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01184

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **38.1140 ✅** | 34.4082 | 20.9781 | +3.7059 | Higher | +17.1360 | Higher | +13.4301 | Higher |
| MI (SEI) | **5.1286 ✅** | 1.9079 | -18.7291 | +3.2208 | Higher | +23.8578 | Higher | +20.6370 | Higher |
| Cyclomatic (sum) | **34 ✅** | 40 | 51 | -6 | Lower | -17 | Lower | -11 | Lower |
| Cognitive (sum) | **45 ✅** | 49 | 66 | -4 | Lower | -21 | Lower | -17 | Lower |
| Halstead Volume | **4100.6691 ✅** | 4564.1498 | 8528.8896 | -463.4807 | Lower | -4428.2205 | Lower | -3964.7398 | Lower |
| Halstead Effort | **257111.9530 ✅** | 292866.2783 | 928461.3966 | -35754.3253 | Lower | -671349.4435 | Lower | -635595.1182 | Lower |
| SLOC | **156 ✅** | 174 | 279 | -18 | Lower | -123 | Lower | -105 | Lower |
| PLOC | **126 ✅** | 135 | 226 | -9 | Lower | -100 | Lower | -91 | Lower |
| LLOC | 95 | **93 ✅** | 183 | +2 | Higher | -88 | Lower | -90 | Lower |
| Comments (CLOC) | 14 | 18 | **23 ✅** | -4 | Lower | -9 | Lower | -5 | Lower |
| Blank Lines | **17 ✅** | 21 | 32 | -4 | Lower | -15 | Lower | -11 | Lower |
| Functions | **3 ✅** | 4 | 5 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **4 ✅** | 11 | 9 | -7 | Lower | -5 | Lower | +2 | Higher |

## p01255

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -0.8446 | **21.4197 ✅** | -1.8746 | -22.2644 | Lower | +1.0300 | Higher | +23.2944 | Higher |
| MI (SEI) | -54.9102 | **-15.7498 ✅** | -57.5502 | -39.1604 | Lower | +2.6400 | Higher | +41.8004 | Higher |
| Cyclomatic (sum) | 102 | **53 ✅** | 104 | +49 | Higher | -2 | Lower | -51 | Lower |
| Cognitive (sum) | 179 | **52 ✅** | 175 | +127 | Higher | +4 | Higher | -123 | Lower |
| Halstead Volume | 14784.2087 | **9262.1932 ✅** | 15053.8842 | +5522.0155 | Higher | -269.6755 | Lower | -5791.6910 | Lower |
| Halstead Effort | 1555575.9575 | **649173.1879 ✅** | 1630189.9721 | +906402.7695 | Higher | -74614.0146 | Lower | -981016.7842 | Lower |
| SLOC | 436 | **257 ✅** | 449 | +179 | Higher | -13 | Lower | -192 | Lower |
| PLOC | 384 | **198 ✅** | 403 | +186 | Higher | -19 | Lower | -205 | Lower |
| LLOC | 219 | **113 ✅** | 244 | +106 | Higher | -25 | Lower | -131 | Lower |
| Comments (CLOC) | 10 | **26 ✅** | 8 | -16 | Lower | +2 | Higher | +18 | Higher |
| Blank Lines | 43 | **34 ✅** | 41 | +9 | Higher | +2 | Higher | -7 | Lower |
| Functions | 17 | **9 ✅** | 18 | +8 | Higher | -1 | Lower | -9 | Lower |
| Closures | 3 | 6 | 3 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 20 | **9 ✅** | 22 | +11 | Higher | -2 | Lower | -13 | Lower |

## p01275

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.7444 ✅** | 56.7339 | 48.8316 | +0.0104 | Higher | +7.9128 | Higher | +7.9024 | Higher |
| MI (SEI) | 26.9211 | **44.2024 ✅** | 16.4739 | -17.2813 | Lower | +10.4473 | Higher | +27.7286 | Higher |
| Cyclomatic (sum) | 15 | **12 ✅** | 23 | +3 | Higher | -8 | Lower | -11 | Lower |
| Cognitive (sum) | 12 | **9 ✅** | 27 | +3 | Higher | -15 | Lower | -18 | Lower |
| Halstead Volume | 2383.0825 | **2079.2716 ✅** | 2830.5175 | +303.8109 | Higher | -447.4351 | Lower | -751.2459 | Lower |
| Halstead Effort | 106805.4228 | **77338.7599 ✅** | 131619.0641 | +29466.6629 | Higher | -24813.6413 | Lower | -54280.3042 | Lower |
| SLOC | **77 ✅** | 84 | 106 | -7 | Lower | -29 | Lower | -22 | Lower |
| PLOC | 62 | **49 ✅** | 85 | +13 | Higher | -23 | Lower | -36 | Lower |
| LLOC | 41 | **39 ✅** | 60 | +2 | Higher | -19 | Lower | -21 | Lower |
| Comments (CLOC) | 5 | **24 ✅** | 7 | -19 | Lower | -2 | Lower | +17 | Higher |
| Blank Lines | 12 | **11 ✅** | 15 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |

## p01288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.3081 | **59.5440 ✅** | 42.7985 | -1.2359 | Lower | +15.5097 | Higher | +16.7456 | Higher |
| MI (SEI) | **38.1886 ✅** | 37.0414 | 11.0337 | +1.1471 | Higher | +27.1549 | Higher | +26.0077 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 30 | +1 | Higher | -18 | Lower | -19 | Lower |
| Cognitive (sum) | 14 | **9 ✅** | 40 | +5 | Higher | -26 | Lower | -31 | Lower |
| Halstead Volume | 1935.1138 | **1878.9788 ✅** | 3682.2497 | +56.1350 | Higher | -1747.1359 | Lower | -1803.2709 | Lower |
| Halstead Effort | 67830.8311 | **51448.2279 ✅** | 250392.9799 | +16382.6032 | Higher | -182562.1488 | Lower | -198944.7520 | Lower |
| SLOC | 78 | **74 ✅** | 128 | +4 | Higher | -50 | Lower | -54 | Lower |
| PLOC | 55 | **53 ✅** | 104 | +2 | Higher | -49 | Lower | -51 | Lower |
| LLOC | 50 | **39 ✅** | 81 | +11 | Higher | -31 | Lower | -42 | Lower |
| Comments (CLOC) | **12 ✅** | 9 | 11 | +3 | Higher | +1 | Higher | -2 | Lower |
| Blank Lines | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | **1 ✅** | 2 | 4 | -1 | Lower | -3 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 3 | -1 | Lower | -3 | Lower | -2 | Lower |

## p01316

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.7692 | **54.7554 ✅** | 53.6684 | -0.9862 | Lower | +0.1008 | Higher | +1.0870 | Higher |
| MI (SEI) | 25.1795 | 21.5235 | **30.2271 ✅** | +3.6560 | Higher | -5.0476 | Lower | -8.7036 | Lower |
| Cyclomatic (sum) | 20 | 18 | 18 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 25 | **22 ✅** | 30 | +3 | Higher | -5 | Lower | -8 | Lower |
| Halstead Volume | 2314.0453 | **2168.0308 ✅** | 2240.7165 | +146.0144 | Higher | +73.3288 | Higher | -72.6856 | Lower |
| Halstead Effort | 88234.5454 | 76513.4220 | **76232.0355 ✅** | +11721.1234 | Higher | +12002.5099 | Higher | +281.3865 | Higher |
| SLOC | 87 | **86 ✅** | 91 | +1 | Higher | -4 | Lower | -5 | Lower |
| PLOC | 68 | 68 | **65 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| LLOC | 47 | **46 ✅** | 50 | +1 | Higher | -3 | Lower | -4 | Lower |
| Comments (CLOC) | 7 | 4 | **12 ✅** | +3 | Higher | -5 | Lower | -8 | Lower |
| Blank Lines | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p01317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.3651 | 26.3193 | **45.7754 ✅** | +17.0458 | Higher | -2.4103 | Lower | -19.4560 | Lower |
| MI (SEI) | **18.3724 ✅** | -15.1297 | 17.1712 | +33.5022 | Higher | +1.2012 | Higher | -32.3009 | Lower |
| Cyclomatic (sum) | 22 | 50 | 22 | -28 | Lower | 0 | Same | +28 | Higher |
| Cognitive (sum) | 44 | 88 | **37 ✅** | -44 | Lower | +7 | Higher | +51 | Higher |
| Halstead Volume | 3984.9450 | 6238.1094 | **3812.6044 ✅** | -2253.1644 | Lower | +172.3406 | Higher | +2425.5050 | Higher |
| Halstead Effort | **198305.3542 ✅** | 456962.8753 | 200311.2431 | -258657.5211 | Lower | -2005.8889 | Lower | +256651.6322 | Higher |
| SLOC | 135 | 225 | **118 ✅** | -90 | Lower | +17 | Higher | +107 | Higher |
| PLOC | 97 | 183 | **88 ✅** | -86 | Lower | +9 | Higher | +95 | Higher |
| LLOC | 81 | 136 | **71 ✅** | -55 | Lower | +10 | Higher | +65 | Higher |
| Comments (CLOC) | **22 ✅** | 12 | 13 | +10 | Higher | +9 | Higher | -1 | Lower |
| Blank Lines | 17 | 30 | 17 | -13 | Lower | 0 | Same | +13 | Higher |
| Functions | **1 ✅** | 5 | 2 | -4 | Lower | -1 | Lower | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 6 | 1 | -6 | Lower | -1 | Lower | +5 | Higher |

## p01339

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **44.3403 ✅** | 42.8413 | 40.1817 | +1.4989 | Higher | +4.1586 | Higher | +2.6597 | Higher |
| MI (SEI) | 15.6818 | **19.0774 ✅** | 5.7472 | -3.3956 | Lower | +9.9347 | Higher | +13.3303 | Higher |
| Cyclomatic (sum) | **25 ✅** | 26 | 31 | -1 | Lower | -6 | Lower | -5 | Lower |
| Cognitive (sum) | 40 | 42 | **37 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Halstead Volume | **3770.0283 ✅** | 4153.2853 | 4715.3851 | -383.2570 | Lower | -945.3567 | Lower | -562.0998 | Lower |
| Halstead Effort | **246345.4792 ✅** | 258258.8330 | 309752.0200 | -11913.3538 | Lower | -63406.5408 | Lower | -51493.1870 | Lower |
| SLOC | **124 ✅** | 130 | 137 | -6 | Lower | -13 | Lower | -7 | Lower |
| PLOC | 97 | **95 ✅** | 114 | +2 | Higher | -17 | Lower | -19 | Lower |
| LLOC | **76 ✅** | 77 | 82 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | 14 | **23 ✅** | 10 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 1 | 6 | 0 | Same | -5 | Lower | -5 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |

## p01353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.9023 | 49.2808 | **50.0463 ✅** | +0.6215 | Higher | -0.1440 | Lower | -0.7655 | Lower |
| MI (SEI) | 23.3499 | **29.5464 ✅** | 22.6876 | -6.1965 | Lower | +0.6623 | Higher | +6.8588 | Higher |
| Cyclomatic (sum) | 18 | **15 ✅** | 18 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 22 | **15 ✅** | 24 | +7 | Higher | -2 | Lower | -9 | Lower |
| Halstead Volume | 3142.8944 | **2948.3198 ✅** | 3151.4125 | +194.5746 | Higher | -8.5180 | Lower | -203.0927 | Lower |
| Halstead Effort | 182411.1283 | **127787.8253 ✅** | 176479.0994 | +54623.3030 | Higher | +5932.0289 | Higher | -48691.2742 | Lower |
| SLOC | 103 | 114 | **102 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| PLOC | 77 | **74 ✅** | 80 | +3 | Higher | -3 | Lower | -6 | Lower |
| LLOC | 60 | **51 ✅** | 62 | +9 | Higher | -2 | Lower | -11 | Lower |
| Comments (CLOC) | 12 | **24 ✅** | 11 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 15 | 16 | **14 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |

## p01368

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.9488 ✅** | 31.0037 | 35.9220 | +8.9450 | Higher | +4.0268 | Higher | -4.9182 | Lower |
| MI (SEI) | **5.7868 ✅** | -12.1175 | 2.3626 | +17.9043 | Higher | +3.4242 | Higher | -14.4801 | Lower |
| Cyclomatic (sum) | 34 | 47 | **33 ✅** | -13 | Lower | +1 | Higher | +14 | Higher |
| Cognitive (sum) | 55 | 83 | **49 ✅** | -28 | Lower | +6 | Higher | +34 | Higher |
| Halstead Volume | **4418.2512 ✅** | 6110.8010 | 4998.7269 | -1692.5498 | Lower | -580.4757 | Lower | +1112.0741 | Higher |
| Halstead Effort | **363255.2778 ✅** | 437261.7621 | 405159.9685 | -74006.4843 | Lower | -41904.6907 | Lower | +32101.7936 | Higher |
| SLOC | **136 ✅** | 177 | 170 | -41 | Lower | -34 | Lower | +7 | Higher |
| PLOC | **113 ✅** | 148 | 135 | -35 | Lower | -22 | Lower | +13 | Higher |
| LLOC | **84 ✅** | 109 | 103 | -25 | Lower | -19 | Lower | +6 | Higher |
| Comments (CLOC) | 10 | 6 | **16 ✅** | +4 | Higher | -6 | Lower | -10 | Lower |
| Blank Lines | **14 ✅** | 23 | 19 | -9 | Lower | -5 | Lower | +4 | Higher |
| Functions | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |

## p01452

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.4852 ✅** | 50.6321 | 54.2830 | +4.8531 | Higher | +1.2022 | Higher | -3.6509 | Lower |
| MI (SEI) | 24.6685 | 21.1163 | **29.3063 ✅** | +3.5523 | Higher | -4.6378 | Lower | -8.1901 | Lower |
| Cyclomatic (sum) | 13 | 19 | **12 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Cognitive (sum) | 8 | 11 | **6 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Halstead Volume | **3062.1476 ✅** | 3474.3965 | 3331.2632 | -412.2489 | Lower | -269.1156 | Lower | +143.1333 | Higher |
| Halstead Effort | **164815.5888 ✅** | 222361.3754 | 173715.5768 | -57545.7866 | Lower | -8899.9880 | Lower | +48645.7986 | Higher |
| SLOC | **79 ✅** | 94 | 84 | -15 | Lower | -5 | Lower | +10 | Higher |
| PLOC | 62 | 70 | **61 ✅** | -8 | Lower | +1 | Higher | +9 | Higher |
| LLOC | 50 | 51 | **38 ✅** | -1 | Lower | +12 | Higher | +13 | Higher |
| Comments (CLOC) | 5 | 8 | **10 ✅** | -3 | Lower | -5 | Lower | -2 | Lower |
| Blank Lines | **12 ✅** | 16 | 13 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |

## p01459

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **32.3298 ✅** | 28.4765 | 29.2030 | +3.8534 | Higher | +3.1269 | Higher | -0.7265 | Lower |
| MI (SEI) | **1.6962 ✅** | 0.3749 | -9.8531 | +1.3213 | Higher | +11.5493 | Higher | +10.2280 | Higher |
| Cyclomatic (sum) | **35 ✅** | 49 | 47 | -14 | Lower | -12 | Lower | +2 | Higher |
| Cognitive (sum) | 61 | **56 ✅** | 84 | +5 | Higher | -23 | Lower | -28 | Lower |
| Halstead Volume | **5860.0187 ✅** | 6514.9342 | 6815.5616 | -654.9155 | Lower | -955.5429 | Lower | -300.6274 | Lower |
| Halstead Effort | **372419.6105 ✅** | 397312.2771 | 534169.6416 | -24892.6666 | Lower | -161750.0311 | Lower | -136857.3645 | Lower |
| SLOC | 196 | 197 | **191 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| PLOC | 149 | **148 ✅** | 155 | +1 | Higher | -6 | Lower | -7 | Lower |
| LLOC | **80 ✅** | 87 | 116 | -7 | Lower | -36 | Lower | -29 | Lower |
| Comments (CLOC) | 27 | **34 ✅** | 12 | -7 | Lower | +15 | Higher | +22 | Higher |
| Blank Lines | **22 ✅** | 24 | 24 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 2 | 6 | 2 | -4 | Lower | 0 | Same | +4 | Higher |

## p01471

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.6841 ✅** | 54.3251 | 45.0747 | +1.3590 | Higher | +10.6094 | Higher | +9.2504 | Higher |
| MI (SEI) | **36.1523 ✅** | 30.3100 | 14.9626 | +5.8423 | Higher | +21.1897 | Higher | +15.3474 | Higher |
| Cyclomatic (sum) | **15 ✅** | 17 | 25 | -2 | Lower | -10 | Lower | -8 | Lower |
| Cognitive (sum) | **18 ✅** | 21 | 43 | -3 | Lower | -25 | Lower | -22 | Lower |
| Halstead Volume | **1997.4875 ✅** | 2212.1756 | 3192.5737 | -214.6882 | Lower | -1195.0862 | Lower | -980.3980 | Lower |
| Halstead Effort | **75305.2787 ✅** | 88093.7503 | 176731.7577 | -12788.4716 | Lower | -101426.4791 | Lower | -88638.0074 | Lower |
| SLOC | **87 ✅** | 89 | 125 | -2 | Lower | -38 | Lower | -36 | Lower |
| PLOC | **62 ✅** | 66 | 101 | -4 | Lower | -39 | Lower | -35 | Lower |
| LLOC | 44 | 44 | 81 | 0 | Same | -37 | Lower | -37 | Lower |
| Comments (CLOC) | **15 ✅** | 11 | 12 | +4 | Higher | +3 | Higher | -1 | Lower |
| Blank Lines | 13 | 15 | 13 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 5 | 0 | Same | -4 | Lower | -4 | Lower |

## p01521

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.8241 ✅** | 68.6236 | 72.3767 | +8.2005 | Higher | +4.4475 | Higher | -3.7531 | Lower |
| MI (SEI) | 69.9696 | 58.5577 | **70.2791 ✅** | +11.4118 | Higher | -0.3096 | Lower | -11.7214 | Lower |
| Cyclomatic (sum) | **6 ✅** | 11 | 7 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | **6 ✅** | 14 | 7 | -8 | Lower | -1 | Lower | +7 | Higher |
| Halstead Volume | **673.7652 ✅** | 1045.3239 | 1110.0211 | -371.5587 | Lower | -436.2559 | Lower | -64.6972 | Lower |
| Halstead Effort | **12077.8647 ✅** | 23678.1698 | 28322.8812 | -11600.3051 | Lower | -16245.0165 | Lower | -4644.7114 | Lower |
| SLOC | **38 ✅** | 51 | 42 | -13 | Lower | -4 | Lower | +9 | Higher |
| PLOC | **22 ✅** | 32 | 23 | -10 | Lower | -1 | Lower | +9 | Higher |
| LLOC | **11 ✅** | 25 | 21 | -14 | Lower | -10 | Lower | +4 | Higher |
| Comments (CLOC) | 9 | 12 | **16 ✅** | -3 | Lower | -7 | Lower | -4 | Lower |
| Blank Lines | 7 | 8 | **5 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p01529

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.7780 ✅** | 54.5953 | 46.1346 | +2.1827 | Higher | +10.6435 | Higher | +8.4607 | Higher |
| MI (SEI) | 30.3914 | **32.3251 ✅** | 20.5668 | -1.9337 | Lower | +9.8247 | Higher | +11.7584 | Higher |
| Cyclomatic (sum) | **15 ✅** | 16 | 27 | -1 | Lower | -12 | Lower | -11 | Lower |
| Cognitive (sum) | 26 | **24 ✅** | 41 | +2 | Higher | -15 | Lower | -17 | Lower |
| Halstead Volume | **2466.1128 ✅** | 2628.3881 | 3450.7656 | -162.2752 | Lower | -984.6528 | Lower | -822.3775 | Lower |
| Halstead Effort | 154631.9378 | **138826.6779 ✅** | 286043.8161 | +15805.2598 | Higher | -131411.8784 | Lower | -147217.1382 | Lower |
| SLOC | **76 ✅** | 84 | 111 | -8 | Lower | -35 | Lower | -27 | Lower |
| PLOC | 62 | **60 ✅** | 83 | +2 | Higher | -21 | Lower | -23 | Lower |
| LLOC | 56 | **54 ✅** | 76 | +2 | Higher | -20 | Lower | -22 | Lower |
| Comments (CLOC) | 7 | 12 | **15 ✅** | -5 | Lower | -8 | Lower | -3 | Lower |
| Blank Lines | **8 ✅** | 13 | 14 | -5 | Lower | -6 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p01545

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.0292 ✅** | 45.6033 | 47.5167 | +9.4259 | Higher | +7.5125 | Higher | -1.9134 | Lower |
| MI (SEI) | **35.4058 ✅** | 20.5779 | 30.6658 | +14.8279 | Higher | +4.7400 | Higher | -10.0880 | Lower |
| Cyclomatic (sum) | **14 ✅** | 26 | 20 | -12 | Lower | -6 | Lower | +6 | Higher |
| Cognitive (sum) | 17 | 39 | **16 ✅** | -22 | Lower | +1 | Higher | +23 | Higher |
| Halstead Volume | **2545.9953 ✅** | 3390.5678 | 3060.0189 | -844.5725 | Lower | -514.0236 | Lower | +330.5489 | Higher |
| Halstead Effort | **156676.6343 ✅** | 253515.5803 | 200813.7421 | -96838.9460 | Lower | -44137.1078 | Lower | +52701.8382 | Higher |
| SLOC | **85 ✅** | 117 | 117 | -32 | Lower | -32 | Lower | 0 | Same |
| PLOC | **60 ✅** | 91 | 75 | -31 | Lower | -15 | Lower | +16 | Higher |
| LLOC | **58 ✅** | 74 | 59 | -16 | Lower | -1 | Lower | +15 | Higher |
| Comments (CLOC) | 15 | 17 | **31 ✅** | -2 | Lower | -16 | Lower | -14 | Lower |
| Blank Lines | **10 ✅** | 11 | 13 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | **2 ✅** | 4 | 5 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 6 | 5 | -5 | Lower | -4 | Lower | +1 | Higher |

## p01570

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.8499 ✅** | 50.8874 | 42.0010 | +4.9625 | Higher | +13.8489 | Higher | +8.8864 | Higher |
| MI (SEI) | **30.5102 ✅** | 29.4854 | 10.6404 | +1.0248 | Higher | +19.8698 | Higher | +18.8450 | Higher |
| Cyclomatic (sum) | **12 ✅** | 18 | 25 | -6 | Lower | -13 | Lower | -7 | Lower |
| Cognitive (sum) | **20 ✅** | 35 | 38 | -15 | Lower | -18 | Lower | -3 | Lower |
| Halstead Volume | **2558.2948 ✅** | 2942.0558 | 4642.8028 | -383.7610 | Lower | -2084.5080 | Lower | -1700.7469 | Lower |
| Halstead Effort | **109105.4590 ✅** | 137340.5162 | 308099.4353 | -28235.0572 | Lower | -198993.9764 | Lower | -170758.9192 | Lower |
| SLOC | **83 ✅** | 99 | 134 | -16 | Lower | -51 | Lower | -35 | Lower |
| PLOC | **59 ✅** | 68 | 109 | -9 | Lower | -50 | Lower | -41 | Lower |
| LLOC | **53 ✅** | 55 | 99 | -2 | Lower | -46 | Lower | -44 | Lower |
| Comments (CLOC) | 9 | **17 ✅** | 13 | -8 | Lower | -4 | Lower | +4 | Higher |
| Blank Lines | 16 | 16 | **13 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p01600

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.5343 | **65.8320 ✅** | 57.0502 | -4.2977 | Lower | +4.4841 | Higher | +8.7818 | Higher |
| MI (SEI) | 42.0304 | **54.2871 ✅** | 35.0977 | -12.2567 | Lower | +6.9327 | Higher | +19.1894 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 13 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 20 | **8 ✅** | 18 | +12 | Higher | +2 | Higher | -10 | Lower |
| Halstead Volume | 1936.5195 | **1613.8656 ✅** | 2179.2358 | +322.6539 | Higher | -242.7164 | Lower | -565.3703 | Lower |
| Halstead Effort | 88727.8018 | **45470.6627 ✅** | 100863.2809 | +43257.1391 | Higher | -12135.4790 | Lower | -55392.6182 | Lower |
| SLOC | 63 | **55 ✅** | 80 | +8 | Higher | -17 | Lower | -25 | Lower |
| PLOC | 47 | **34 ✅** | 57 | +13 | Higher | -10 | Lower | -23 | Lower |
| LLOC | 39 | **25 ✅** | 46 | +14 | Higher | -7 | Lower | -21 | Lower |
| Comments (CLOC) | 9 | **13 ✅** | 11 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | **7 ✅** | 9 | 12 | -2 | Lower | -5 | Lower | -3 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p01601

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.5981 ✅** | 63.0725 | 72.0904 | +12.5256 | Higher | +3.5077 | Higher | -9.0178 | Lower |
| MI (SEI) | 51.5774 | 46.9984 | **52.0774 ✅** | +4.5791 | Higher | -0.5000 | Lower | -5.0791 | Lower |
| Cyclomatic (sum) | **8 ✅** | 12 | 10 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | **5 ✅** | 17 | 6 | -12 | Lower | -1 | Lower | +11 | Higher |
| Halstead Volume | **780.6987 ✅** | 1242.9923 | 954.5007 | -462.2936 | Lower | -173.8020 | Lower | +288.4917 | Higher |
| Halstead Effort | **25680.8790 ✅** | 44348.1909 | 28833.8745 | -18667.3119 | Lower | -3152.9955 | Lower | +15514.3164 | Higher |
| SLOC | **38 ✅** | 67 | 43 | -29 | Lower | -5 | Lower | +24 | Higher |
| PLOC | **30 ✅** | 49 | 34 | -19 | Lower | -4 | Lower | +15 | Higher |
| LLOC | **22 ✅** | 30 | 23 | -8 | Lower | -1 | Lower | +7 | Higher |
| Comments (CLOC) | 2 | **12 ✅** | 4 | -10 | Lower | -2 | Lower | +8 | Higher |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |

## p01656

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.1426 ✅** | 71.5347 | 70.5292 | +5.6080 | Higher | +6.6134 | Higher | +1.0054 | Higher |
| MI (SEI) | **69.3543 ✅** | 59.7560 | 62.9499 | +9.5982 | Higher | +6.4044 | Higher | -3.1939 | Lower |
| Cyclomatic (sum) | **4 ✅** | 6 | 8 | -2 | Lower | -4 | Lower | -2 | Lower |
| Cognitive (sum) | **3 ✅** | 7 | 11 | -4 | Lower | -8 | Lower | -4 | Lower |
| Halstead Volume | **819.3660 ✅** | 1027.4919 | 999.4184 | -208.1259 | Lower | -180.0524 | Lower | +28.0735 | Higher |
| Halstead Effort | **14594.9565 ✅** | 16406.7255 | 21580.0333 | -1811.7690 | Lower | -6985.0768 | Lower | -5173.3078 | Lower |
| SLOC | **36 ✅** | 46 | 48 | -10 | Lower | -12 | Lower | -2 | Lower |
| PLOC | **20 ✅** | 28 | 28 | -8 | Lower | -8 | Lower | 0 | Same |
| LLOC | **18 ✅** | 19 | 20 | -1 | Lower | -2 | Lower | -1 | Lower |
| Comments (CLOC) | 8 | 9 | **13 ✅** | -1 | Lower | -5 | Lower | -4 | Lower |
| Blank Lines | 8 | 10 | **7 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p01658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.2886 | 65.1591 | **65.7436 ✅** | -1.8705 | Lower | -2.4550 | Lower | -0.5846 | Lower |
| MI (SEI) | 44.4596 | **49.9442 ✅** | 46.2026 | -5.4846 | Lower | -1.7430 | Lower | +3.7417 | Higher |
| Cyclomatic (sum) | 12 | 14 | **10 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 27 | 23 | **18 ✅** | +4 | Higher | +9 | Higher | +5 | Higher |
| Halstead Volume | 1444.4768 | **1260.3364 ✅** | 1420.5243 | +184.1403 | Higher | +23.9525 | Higher | -160.1879 | Lower |
| Halstead Effort | 65334.7951 | 53418.8743 | **49963.2675 ✅** | +11915.9208 | Higher | +15371.5276 | Higher | +3455.6068 | Higher |
| SLOC | 63 | 57 | **56 ✅** | +6 | Higher | +7 | Higher | +1 | Higher |
| PLOC | 47 | **39 ✅** | 43 | +8 | Higher | +4 | Higher | -4 | Lower |
| LLOC | 43 | **30 ✅** | 39 | +13 | Higher | +4 | Higher | -9 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 7 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 9 | 8 | **7 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p01668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **33.7634 ✅** | 33.6571 | 30.5565 | +0.1063 | Higher | +3.2068 | Higher | +3.1005 | Higher |
| MI (SEI) | 1.0163 | **2.5661 ✅** | -3.7012 | -1.5499 | Lower | +4.7174 | Higher | +6.2673 | Higher |
| Cyclomatic (sum) | 44 | 44 | 46 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **46 ✅** | 113 | 51 | -67 | Lower | -5 | Lower | +62 | Higher |
| Halstead Volume | 5621.6944 | **5415.1494 ✅** | 6065.2346 | +206.5450 | Higher | -443.5402 | Lower | -650.0852 | Lower |
| Halstead Effort | 461670.8428 | **383772.3445 ✅** | 504930.7834 | +77898.4983 | Higher | -43259.9406 | Lower | -121158.4390 | Lower |
| SLOC | **160 ✅** | 163 | 185 | -3 | Lower | -25 | Lower | -22 | Lower |
| PLOC | **124 ✅** | 125 | 145 | -1 | Lower | -21 | Lower | -20 | Lower |
| LLOC | **66 ✅** | 91 | 81 | -25 | Lower | -15 | Lower | +10 | Higher |
| Comments (CLOC) | 16 | **19 ✅** | 18 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | 25 | **19 ✅** | 27 | +6 | Higher | -2 | Lower | -8 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Exits | 10 | **2 ✅** | 13 | +8 | Higher | -3 | Lower | -11 | Lower |

## p01725

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.6949 ✅** | 28.6207 | 36.6563 | +11.0741 | Higher | +3.0386 | Higher | -8.0356 | Lower |
| MI (SEI) | 2.8808 | **4.2288 ✅** | -0.1434 | -1.3480 | Lower | +3.0242 | Higher | +4.3722 | Higher |
| Cyclomatic (sum) | **28 ✅** | 45 | 32 | -17 | Lower | -4 | Lower | +13 | Higher |
| Cognitive (sum) | **47 ✅** | 50 | 58 | -3 | Lower | -11 | Lower | -8 | Lower |
| Halstead Volume | **4366.7302 ✅** | 5516.3629 | 4886.0422 | -1149.6327 | Lower | -519.3120 | Lower | +630.3208 | Higher |
| Halstead Effort | **319733.4658 ✅** | 343450.2677 | 343714.2746 | -23716.8019 | Lower | -23980.8088 | Lower | -264.0069 | Lower |
| SLOC | **151 ✅** | 218 | 166 | -67 | Lower | -15 | Lower | +52 | Higher |
| PLOC | **128 ✅** | 150 | 142 | -22 | Lower | -14 | Lower | +8 | Higher |
| LLOC | 100 | **84 ✅** | 111 | +16 | Higher | -11 | Lower | -27 | Lower |
| Comments (CLOC) | 9 | **51 ✅** | 11 | -42 | Lower | -2 | Lower | +40 | Higher |
| Blank Lines | 15 | 19 | 15 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **2 ✅** | 4 | 4 | -2 | Lower | -2 | Lower | 0 | Same |

## p01767

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.4119 ✅** | 58.7913 | 58.8032 | +3.6207 | Higher | +3.6088 | Higher | -0.0119 | Lower |
| MI (SEI) | **36.7549 ✅** | 35.9554 | 33.9336 | +0.7995 | Higher | +2.8213 | Higher | +2.0219 | Higher |
| Cyclomatic (sum) | 11 | 11 | 14 | 0 | Same | -3 | Lower | -3 | Lower |
| Cognitive (sum) | 12 | 12 | 13 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **1878.3912 ✅** | 2171.6599 | 2158.5612 | -293.2687 | Lower | -280.1700 | Lower | +13.0987 | Higher |
| Halstead Effort | **70439.6704 ✅** | 79874.7116 | 93976.3856 | -9435.0412 | Lower | -23536.7153 | Lower | -14101.6741 | Lower |
| SLOC | **62 ✅** | 74 | 71 | -12 | Lower | -9 | Lower | +3 | Higher |
| PLOC | **48 ✅** | 55 | 53 | -7 | Lower | -5 | Lower | +2 | Higher |
| LLOC | **35 ✅** | 40 | 36 | -5 | Lower | -1 | Lower | +4 | Higher |
| Comments (CLOC) | 5 | **9 ✅** | 7 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | **9 ✅** | 11 | 12 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p01768

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.4689 | **56.4713 ✅** | 55.0565 | -4.0024 | Lower | -2.5876 | Lower | +1.4147 | Higher |
| MI (SEI) | 33.2243 | **33.6176 ✅** | 25.8208 | -0.3934 | Lower | +7.4035 | Higher | +7.7969 | Higher |
| Cyclomatic (sum) | 14 | **11 ✅** | 16 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 23 | **12 ✅** | 13 | +11 | Higher | +10 | Higher | -1 | Lower |
| Halstead Volume | 2434.1057 | **2372.8129 ✅** | 2693.8593 | +61.2928 | Higher | -259.7536 | Lower | -321.0464 | Lower |
| Halstead Effort | 82404.6207 | **67493.3452 ✅** | 109438.0335 | +14911.2755 | Higher | -27033.4128 | Lower | -41944.6882 | Lower |
| SLOC | 101 | 83 | **81 ✅** | +18 | Higher | +20 | Higher | +2 | Higher |
| PLOC | 64 | **59 ✅** | 63 | +5 | Higher | +1 | Higher | -4 | Lower |
| LLOC | 46 | 42 | **40 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Comments (CLOC) | **20 ✅** | 11 | 6 | +9 | Higher | +14 | Higher | +5 | Higher |
| Blank Lines | 18 | 15 | **13 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| Functions | **1 ✅** | 2 | 4 | -1 | Lower | -3 | Lower | -2 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 1 | **0 ✅** | 4 | +1 | Higher | -3 | Lower | -4 | Lower |

## p01782

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **21.4147 ✅** | 13.1313 | 20.4409 | +8.2834 | Higher | +0.9738 | Higher | -7.3096 | Lower |
| MI (SEI) | -18.9250 | **-14.4877 ✅** | -26.1700 | -4.4373 | Lower | +7.2451 | Higher | +11.6823 | Higher |
| Cyclomatic (sum) | 46 | 64 | **40 ✅** | -18 | Lower | +6 | Higher | +24 | Higher |
| Cognitive (sum) | **52 ✅** | 146 | 58 | -94 | Lower | -6 | Lower | +88 | Higher |
| Halstead Volume | **8861.6522 ✅** | 9559.2428 | 10233.6305 | -697.5906 | Lower | -1371.9783 | Lower | -674.3876 | Lower |
| Halstead Effort | **649577.5668 ✅** | 1006669.6242 | 888155.0806 | -357092.0574 | Lower | -238577.5138 | Lower | +118514.5437 | Higher |
| SLOC | **288 ✅** | 363 | 318 | -75 | Lower | -30 | Lower | +45 | Higher |
| PLOC | **235 ✅** | 239 | 270 | -4 | Lower | -35 | Lower | -31 | Lower |
| LLOC | **139 ✅** | 163 | 165 | -24 | Lower | -26 | Lower | -2 | Lower |
| Comments (CLOC) | 23 | **96 ✅** | 14 | -73 | Lower | +9 | Higher | +82 | Higher |
| Blank Lines | 33 | **28 ✅** | 38 | +5 | Higher | -5 | Lower | -10 | Lower |
| Functions | 13 | **4 ✅** | 10 | +9 | Higher | +3 | Higher | -6 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 7 | 5 | **4 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p01841

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **42.4027 ✅** | 41.5740 | 35.6039 | +0.8288 | Higher | +6.7988 | Higher | +5.9700 | Higher |
| MI (SEI) | **14.0918 ✅** | 3.9715 | 1.2714 | +10.1203 | Higher | +12.8204 | Higher | +2.7001 | Higher |
| Cyclomatic (sum) | 28 | **22 ✅** | 28 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 32 | **9 ✅** | 31 | +23 | Higher | +1 | Higher | -22 | Lower |
| Halstead Volume | 4038.6463 | **3967.0954 ✅** | 6392.1063 | +71.5509 | Higher | -2353.4600 | Lower | -2425.0108 | Lower |
| Halstead Effort | 185888.3785 | **138171.0312 ✅** | 361808.6177 | +47717.3473 | Higher | -175920.2392 | Lower | -223637.5865 | Lower |
| SLOC | **131 ✅** | 151 | 172 | -20 | Lower | -41 | Lower | -21 | Lower |
| PLOC | **100 ✅** | 122 | 139 | -22 | Lower | -39 | Lower | -17 | Lower |
| LLOC | 65 | **53 ✅** | 123 | +12 | Higher | -58 | Lower | -70 | Lower |
| Comments (CLOC) | 16 | 8 | 16 | +8 | Higher | 0 | Same | -8 | Lower |
| Blank Lines | **15 ✅** | 22 | 20 | -7 | Lower | -5 | Lower | +2 | Higher |
| Functions | 6 | 10 | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 8 | **2 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |

## p01847

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.5778 | **51.5366 ✅** | 42.1907 | -4.9589 | Lower | +4.3870 | Higher | +9.3459 | Higher |
| MI (SEI) | 8.1266 | **25.6156 ✅** | -0.7396 | -17.4890 | Lower | +8.8662 | Higher | +26.3552 | Higher |
| Cyclomatic (sum) | 24 | **16 ✅** | 28 | +8 | Higher | -4 | Lower | -12 | Lower |
| Cognitive (sum) | 24 | **19 ✅** | 44 | +5 | Higher | -20 | Lower | -25 | Lower |
| Halstead Volume | 3154.4023 | **2584.9598 ✅** | 3420.1399 | +569.4426 | Higher | -265.7376 | Lower | -835.1802 | Lower |
| Halstead Effort | 165807.4661 | **81177.6785 ✅** | 226869.2816 | +84629.7876 | Higher | -61061.8155 | Lower | -145691.6031 | Lower |
| SLOC | 116 | **102 ✅** | 140 | +14 | Higher | -24 | Lower | -38 | Lower |
| PLOC | 95 | **77 ✅** | 117 | +18 | Higher | -22 | Lower | -40 | Lower |
| LLOC | 72 | **55 ✅** | 103 | +17 | Higher | -31 | Lower | -48 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 3 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 17 | **14 ✅** | 20 | +3 | Higher | -3 | Lower | -6 | Lower |
| Functions | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 5 | +2 | Higher | -2 | Lower | -4 | Lower |

## p01907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.3063 | **58.9028 ✅** | 35.9607 | -20.5965 | Lower | +2.3457 | Higher | +22.9421 | Higher |
| MI (SEI) | -0.6493 | **31.9585 ✅** | -3.1152 | -32.6077 | Lower | +2.4659 | Higher | +35.0736 | Higher |
| Cyclomatic (sum) | 32 | **9 ✅** | 28 | +23 | Higher | +4 | Higher | -19 | Lower |
| Cognitive (sum) | 32 | **10 ✅** | 28 | +22 | Higher | +4 | Higher | -18 | Lower |
| Halstead Volume | 5086.7119 | **2641.7210 ✅** | 5180.0885 | +2444.9909 | Higher | -93.3767 | Lower | -2538.3676 | Lower |
| Halstead Effort | 308315.4762 | **101993.4013 ✅** | 342336.2857 | +206322.0749 | Higher | -34020.8095 | Lower | -240342.8844 | Lower |
| SLOC | 148 | **71 ✅** | 180 | +77 | Higher | -32 | Lower | -109 | Lower |
| PLOC | 130 | **55 ✅** | 150 | +75 | Higher | -20 | Lower | -95 | Lower |
| LLOC | 106 | **54 ✅** | 127 | +52 | Higher | -21 | Lower | -73 | Lower |
| Comments (CLOC) | 7 | 6 | **10 ✅** | +1 | Higher | -3 | Lower | -4 | Lower |
| Blank Lines | 12 | **11 ✅** | 20 | +1 | Higher | -8 | Lower | -9 | Lower |
| Functions | 6 | **1 ✅** | 5 | +5 | Higher | +1 | Higher | -4 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 4 | **0 ✅** | 4 | +4 | Higher | 0 | Same | -4 | Lower |

## p01916

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.0004 ✅** | 82.1909 | 78.6013 | +5.8096 | Higher | +9.3991 | Higher | +3.5896 | Higher |
| MI (SEI) | **79.2967 ✅** | 77.8714 | 75.5455 | +1.4253 | Higher | +3.7511 | Higher | +2.3258 | Higher |
| Cyclomatic (sum) | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **1 ✅** | 4 | 8 | -3 | Lower | -7 | Lower | -4 | Lower |
| Halstead Volume | **544.3964 ✅** | 582.3703 | 676.9695 | -37.9739 | Lower | -132.5731 | Lower | -94.5992 | Lower |
| Halstead Effort | **7151.3894 ✅** | 8298.7765 | 11576.1788 | -1147.3871 | Lower | -4424.7894 | Lower | -3277.4023 | Lower |
| SLOC | **21 ✅** | 29 | 34 | -8 | Lower | -13 | Lower | -5 | Lower |
| PLOC | **14 ✅** | 19 | 19 | -5 | Lower | -5 | Lower | 0 | Same |
| LLOC | **11 ✅** | 15 | 15 | -4 | Lower | -4 | Lower | 0 | Same |
| Comments (CLOC) | 3 | 7 | **10 ✅** | -4 | Lower | -7 | Lower | -3 | Lower |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01917

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.7920 | **51.1757 ✅** | 50.7545 | -0.3837 | Lower | +0.0374 | Higher | +0.4211 | Higher |
| MI (SEI) | **21.4393 ✅** | 20.1465 | 17.5429 | +1.2928 | Higher | +3.8964 | Higher | +2.6035 | Higher |
| Cyclomatic (sum) | **22 ✅** | 25 | 24 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | 51 | 51 | 59 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | 2763.2067 | **2748.2821 ✅** | 2812.4078 | +14.9246 | Higher | -49.2011 | Lower | -64.1257 | Lower |
| Halstead Effort | 140442.9864 | **116658.8497 ✅** | 149727.2360 | +23784.1367 | Higher | -9284.2496 | Lower | -33068.3863 | Lower |
| SLOC | 96 | **90 ✅** | 93 | +6 | Higher | +3 | Higher | -3 | Lower |
| PLOC | 76 | **73 ✅** | 77 | +3 | Higher | -1 | Lower | -4 | Lower |
| LLOC | 60 | **48 ✅** | 60 | +12 | Higher | 0 | Same | -12 | Lower |
| Comments (CLOC) | **8 ✅** | 6 | 5 | +2 | Higher | +3 | Higher | +1 | Higher |
| Blank Lines | 12 | 12 | **11 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p02035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.0370 | 44.5376 | **51.5861 ✅** | +4.4994 | Higher | -2.5491 | Lower | -7.0485 | Lower |
| MI (SEI) | 13.8535 | 13.7750 | **21.1798 ✅** | +0.0786 | Higher | -7.3263 | Lower | -7.4049 | Lower |
| Cyclomatic (sum) | 18 | 24 | **16 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Cognitive (sum) | 18 | 17 | **14 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Halstead Volume | 4069.9632 | 4427.6964 | **3653.5366 ✅** | -357.7332 | Lower | +416.4267 | Higher | +774.1599 | Higher |
| Halstead Effort | 309943.3548 | 370798.6895 | **288385.8186 ✅** | -60855.3347 | Lower | +21557.5362 | Higher | +82412.8709 | Higher |
| SLOC | 100 | 118 | **91 ✅** | -18 | Lower | +9 | Higher | +27 | Higher |
| PLOC | 83 | 90 | **73 ✅** | -7 | Lower | +10 | Higher | +17 | Higher |
| LLOC | 71 | 58 | 58 | +13 | Higher | +13 | Higher | 0 | Same |
| Comments (CLOC) | 5 | **11 ✅** | 7 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 12 | 17 | **11 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Functions | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 10 | **4 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |

## p02038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.6968 | **69.6025 ✅** | 64.7117 | -0.9057 | Lower | +3.9851 | Higher | +4.8908 | Higher |
| MI (SEI) | 57.4968 | **58.7015 ✅** | 47.4679 | -1.2048 | Lower | +10.0288 | Higher | +11.2336 | Higher |
| Cyclomatic (sum) | 13 | **12 ✅** | 17 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | **9 ✅** | 10 | 15 | -1 | Lower | -6 | Lower | -5 | Lower |
| Halstead Volume | 1216.8195 | **1068.5533 ✅** | 1423.5606 | +148.2662 | Higher | -206.7412 | Lower | -355.0073 | Lower |
| Halstead Effort | 31637.3060 | **26066.2246 ✅** | 46028.4605 | +5571.0815 | Higher | -14391.1545 | Lower | -19962.2359 | Lower |
| SLOC | 47 | 47 | 54 | 0 | Same | -7 | Lower | -7 | Lower |
| PLOC | 31 | **29 ✅** | 39 | +2 | Higher | -8 | Lower | -10 | Lower |
| LLOC | 17 | 17 | 21 | 0 | Same | -4 | Lower | -4 | Lower |
| Comments (CLOC) | 10 | 10 | 8 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **46.8154 ✅** | 43.4917 | 41.8893 | +3.3237 | Higher | +4.9262 | Higher | +1.6024 | Higher |
| MI (SEI) | **18.8352 ✅** | 14.5375 | 9.1042 | +4.2977 | Higher | +9.7310 | Higher | +5.4333 | Higher |
| Cyclomatic (sum) | **33 ✅** | 37 | 38 | -4 | Lower | -5 | Lower | -1 | Lower |
| Cognitive (sum) | 22 | **21 ✅** | 26 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | **3315.8481 ✅** | 4014.4243 | 3668.3075 | -698.5762 | Lower | -352.4594 | Lower | +346.1168 | Higher |
| Halstead Effort | **201834.2320 ✅** | 261370.5102 | 222975.5543 | -59536.2782 | Lower | -21141.3223 | Lower | +38394.9559 | Higher |
| SLOC | **99 ✅** | 108 | 121 | -9 | Lower | -22 | Lower | -13 | Lower |
| PLOC | **79 ✅** | 88 | 98 | -9 | Lower | -19 | Lower | -10 | Lower |
| LLOC | 49 | **34 ✅** | 62 | +15 | Higher | -13 | Lower | -28 | Lower |
| Comments (CLOC) | 10 | **11 ✅** | 9 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | **11 ✅** | 12 | 14 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | **2 ✅** | 5 | 3 | -3 | Lower | -1 | Lower | +2 | Higher |
| Closures | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 4 | 8 | -3 | Lower | -7 | Lower | -4 | Lower |

## p02046

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.8783 | 66.0808 | **75.2439 ✅** | +2.7975 | Higher | -6.3655 | Lower | -9.1631 | Lower |
| MI (SEI) | 51.6333 | 47.3075 | **58.8767 ✅** | +4.3258 | Higher | -7.2434 | Lower | -11.5691 | Lower |
| Cyclomatic (sum) | 14 | 14 | **10 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Cognitive (sum) | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 1202.1414 | 1179.8645 | **988.3702 ✅** | +22.2769 | Higher | +213.7712 | Higher | +191.4943 | Higher |
| Halstead Effort | 40502.9169 | 31774.9708 | **27533.1703 ✅** | +8727.9461 | Higher | +12969.7466 | Higher | +4241.8005 | Higher |
| SLOC | 46 | 55 | **35 ✅** | -9 | Lower | +11 | Higher | +20 | Higher |
| PLOC | 33 | 40 | **25 ✅** | -7 | Lower | +8 | Higher | +15 | Higher |
| LLOC | 23 | **19 ✅** | 23 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 4 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 7 | 10 | **6 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.1719 | **47.7977 ✅** | 24.7305 | -4.6258 | Lower | +18.4414 | Higher | +23.0672 | Higher |
| MI (SEI) | 12.8271 | **25.2147 ✅** | -17.6018 | -12.3876 | Lower | +30.4289 | Higher | +42.8165 | Higher |
| Cyclomatic (sum) | 22 | **16 ✅** | 41 | +6 | Higher | -19 | Lower | -25 | Lower |
| Cognitive (sum) | 20 | **18 ✅** | 40 | +2 | Higher | -20 | Lower | -22 | Lower |
| Halstead Volume | 4232.6882 | **3856.1607 ✅** | 8231.1808 | +376.5275 | Higher | -3998.4926 | Lower | -4375.0201 | Lower |
| Halstead Effort | 203578.6495 | **179373.6705 ✅** | 620111.1681 | +24204.9790 | Higher | -416532.5186 | Lower | -440737.4976 | Lower |
| SLOC | 134 | **113 ✅** | 258 | +21 | Higher | -124 | Lower | -145 | Lower |
| PLOC | 101 | **78 ✅** | 221 | +23 | Higher | -120 | Lower | -143 | Lower |
| LLOC | 65 | **63 ✅** | 132 | +2 | Higher | -67 | Lower | -69 | Lower |
| Comments (CLOC) | 14 | **20 ✅** | 15 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 19 | **16 ✅** | 24 | +3 | Higher | -5 | Lower | -8 | Lower |
| Functions | 3 | **1 ✅** | 11 | +2 | Higher | -8 | Lower | -10 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | **1 ✅** | 9 | +1 | Higher | -7 | Lower | -8 | Lower |

## p02061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.9794 | **47.8950 ✅** | 47.1926 | -1.9156 | Lower | -1.2132 | Lower | +0.7024 | Higher |
| MI (SEI) | 18.3605 | **25.1980 ✅** | 16.1109 | -6.8375 | Lower | +2.2495 | Higher | +9.0870 | Higher |
| Cyclomatic (sum) | 25 | 28 | **19 ✅** | -3 | Lower | +6 | Higher | +9 | Higher |
| Cognitive (sum) | 39 | 47 | **28 ✅** | -8 | Lower | +11 | Higher | +19 | Higher |
| Halstead Volume | 3777.1424 | **2638.3841 ✅** | 3315.0205 | +1138.7583 | Higher | +462.1219 | Higher | -676.6364 | Lower |
| Halstead Effort | 228643.0183 | **162088.5525 ✅** | 210039.6992 | +66554.4658 | Higher | +18603.3191 | Higher | -47951.1466 | Lower |
| SLOC | 112 | **107 ✅** | 118 | +5 | Higher | -6 | Lower | -11 | Lower |
| PLOC | 89 | **77 ✅** | 95 | +12 | Higher | -6 | Lower | -18 | Lower |
| LLOC | 69 | **51 ✅** | 81 | +18 | Higher | -12 | Lower | -30 | Lower |
| Comments (CLOC) | 13 | **17 ✅** | 10 | -4 | Lower | +3 | Higher | +7 | Higher |
| Blank Lines | **10 ✅** | 13 | 13 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02104

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.2544 ✅** | 58.7015 | 49.1725 | +2.5529 | Higher | +12.0819 | Higher | +9.5290 | Higher |
| MI (SEI) | **44.3754 ✅** | 43.0357 | 27.5202 | +1.3397 | Higher | +16.8552 | Higher | +15.5155 | Higher |
| Cyclomatic (sum) | **12 ✅** | 14 | 22 | -2 | Lower | -10 | Lower | -8 | Lower |
| Cognitive (sum) | **18 ✅** | 29 | 33 | -11 | Lower | -15 | Lower | -4 | Lower |
| Halstead Volume | **1763.2523 ✅** | 1780.6588 | 2940.1667 | -17.4065 | Lower | -1176.9144 | Lower | -1159.5079 | Lower |
| Halstead Effort | 72811.9471 | **66552.1223 ✅** | 211118.3089 | +6259.8247 | Higher | -138306.3618 | Lower | -144566.1865 | Lower |
| SLOC | **67 ✅** | 76 | 104 | -9 | Lower | -37 | Lower | -28 | Lower |
| PLOC | **46 ✅** | 54 | 75 | -8 | Lower | -29 | Lower | -21 | Lower |
| LLOC | 40 | **39 ✅** | 64 | +1 | Higher | -24 | Lower | -25 | Lower |
| Comments (CLOC) | 12 | 16 | **18 ✅** | -4 | Lower | -6 | Lower | -2 | Lower |
| Blank Lines | 9 | **6 ✅** | 11 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.4872 | **63.4432 ✅** | 47.3923 | -15.9560 | Lower | +0.0949 | Higher | +16.0510 | Higher |
| MI (SEI) | 15.8047 | **42.6796 ✅** | 19.9916 | -26.8749 | Lower | -4.1869 | Lower | +22.6880 | Higher |
| Cyclomatic (sum) | 24 | **12 ✅** | 21 | +12 | Higher | +3 | Higher | -9 | Lower |
| Cognitive (sum) | 43 | **25 ✅** | 37 | +18 | Higher | +6 | Higher | -12 | Lower |
| Halstead Volume | 3405.9203 | **1814.1754 ✅** | 3435.5605 | +1591.7449 | Higher | -29.6402 | Lower | -1621.3851 | Lower |
| Halstead Effort | 201463.4001 | **64972.7934 ✅** | 187675.9132 | +136490.6067 | Higher | +13787.4868 | Higher | -122703.1198 | Lower |
| SLOC | 107 | **58 ✅** | 112 | +49 | Higher | -5 | Lower | -54 | Lower |
| PLOC | 86 | **43 ✅** | 82 | +43 | Higher | +4 | Higher | -39 | Lower |
| LLOC | 69 | **31 ✅** | 74 | +38 | Higher | -5 | Lower | -43 | Lower |
| Comments (CLOC) | 8 | 7 | **13 ✅** | +1 | Higher | -5 | Lower | -6 | Lower |
| Blank Lines | 13 | **10 ✅** | 17 | +3 | Higher | -4 | Lower | -7 | Lower |
| Functions | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **1 ✅** | 2 | +4 | Higher | +3 | Higher | -1 | Lower |

## p02153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **42.1236 ✅** | 41.3587 | 25.3446 | +0.7649 | Higher | +16.7790 | Higher | +16.0142 | Higher |
| MI (SEI) | **11.6164 ✅** | 5.3881 | -16.9269 | +6.2282 | Higher | +28.5433 | Higher | +22.3151 | Higher |
| Cyclomatic (sum) | **28 ✅** | 33 | 56 | -5 | Lower | -28 | Lower | -23 | Lower |
| Cognitive (sum) | **44 ✅** | 57 | 54 | -13 | Lower | -10 | Lower | +3 | Higher |
| Halstead Volume | **4580.3395 ✅** | 4815.5088 | 6945.9387 | -235.1693 | Lower | -2365.5992 | Lower | -2130.4300 | Lower |
| Halstead Effort | **260464.0827 ✅** | 298853.3925 | 546413.8469 | -38389.3098 | Lower | -285949.7642 | Lower | -247560.4543 | Lower |
| SLOC | 128 | **123 ✅** | 212 | +5 | Higher | -84 | Lower | -89 | Lower |
| PLOC | 102 | **99 ✅** | 187 | +3 | Higher | -85 | Lower | -88 | Lower |
| LLOC | 82 | **71 ✅** | 103 | +11 | Higher | -21 | Lower | -32 | Lower |
| Comments (CLOC) | **13 ✅** | 7 | 10 | +6 | Higher | +3 | Higher | -3 | Lower |
| Blank Lines | **15 ✅** | 17 | 17 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 8 | 0 | Same | -7 | Lower | -7 | Lower |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | **0 ✅** | 2 | 9 | -2 | Lower | -9 | Lower | -7 | Lower |

## p02162

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.6924 | **69.6884 ✅** | 64.9133 | -3.9960 | Lower | +0.7791 | Higher | +4.7751 | Higher |
| MI (SEI) | **39.7330 ✅** | 26.7728 | 38.1644 | +12.9602 | Higher | +1.5686 | Higher | -11.3916 | Lower |
| Cyclomatic (sum) | 21 | **19 ✅** | 20 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 16 | **11 ✅** | 15 | +5 | Higher | +1 | Higher | -4 | Lower |
| Halstead Volume | **1255.3371 ✅** | 1274.5391 | 1348.8816 | -19.2021 | Lower | -93.5445 | Lower | -74.3425 | Lower |
| Halstead Effort | 35351.1889 | **33095.5329 ✅** | 36376.2917 | +2255.6560 | Higher | -1025.1028 | Lower | -3280.7588 | Lower |
| SLOC | 50 | **40 ✅** | 52 | +10 | Higher | -2 | Lower | -12 | Lower |
| PLOC | 43 | **34 ✅** | 43 | +9 | Higher | 0 | Same | -9 | Lower |
| LLOC | 12 | 11 | 11 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 3 | 0 | 3 | +3 | Higher | 0 | Same | -3 | Lower |
| Blank Lines | **4 ✅** | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02166

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.0023 ✅** | 55.7294 | 60.1299 | +5.2730 | Higher | +0.8724 | Higher | -4.4005 | Lower |
| MI (SEI) | **44.8658 ✅** | 39.4541 | 36.7451 | +5.4118 | Higher | +8.1207 | Higher | +2.7089 | Higher |
| Cyclomatic (sum) | **12 ✅** | 17 | 15 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | **19 ✅** | 23 | 23 | -4 | Lower | -4 | Lower | 0 | Same |
| Halstead Volume | **1767.3597 ✅** | 2098.7952 | 2008.8627 | -331.4355 | Lower | -241.5030 | Lower | +89.9325 | Higher |
| Halstead Effort | **71578.0689 ✅** | 89272.0114 | 87226.9347 | -17693.9425 | Lower | -15648.8657 | Lower | +2045.0767 | Higher |
| SLOC | 68 | 83 | **66 ✅** | -15 | Lower | +2 | Higher | +17 | Higher |
| PLOC | **50 ✅** | 60 | 54 | -10 | Lower | -4 | Lower | +6 | Higher |
| LLOC | **39 ✅** | 46 | 40 | -7 | Lower | -1 | Lower | +6 | Higher |
| Comments (CLOC) | 13 | **18 ✅** | 7 | -5 | Lower | +6 | Higher | +11 | Higher |
| Blank Lines | 6 | 11 | **5 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02186

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 30.0499 | **38.1559 ✅** | 32.3360 | -8.1060 | Lower | -2.2861 | Lower | +5.8199 | Higher |
| MI (SEI) | -3.5643 | **2.9018 ✅** | -7.6773 | -6.4661 | Lower | +4.1130 | Higher | +10.5791 | Higher |
| Cyclomatic (sum) | 29 | 28 | 28 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | **14 ✅** | 15 | 15 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | 7395.2696 | **5097.7330 ✅** | 7490.4879 | +2297.5366 | Higher | -95.2183 | Lower | -2392.7550 | Lower |
| Halstead Effort | 635222.8436 | **386500.8445 ✅** | 727771.4638 | +248721.9990 | Higher | -92548.6202 | Lower | -341270.6193 | Lower |
| SLOC | 228 | **158 ✅** | 200 | +70 | Higher | +28 | Higher | -42 | Lower |
| PLOC | 178 | **124 ✅** | 160 | +54 | Higher | +18 | Higher | -36 | Lower |
| LLOC | 88 | **69 ✅** | 88 | +19 | Higher | 0 | Same | -19 | Lower |
| Comments (CLOC) | **28 ✅** | 12 | 12 | +16 | Higher | +16 | Higher | 0 | Same |
| Blank Lines | 29 | **26 ✅** | 31 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 12 | 11 | 11 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 12 | **7 ✅** | 14 | +5 | Higher | -2 | Lower | -7 | Lower |

## p02190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.8663 ✅** | 78.2106 | 71.9786 | +0.6556 | Higher | +6.8876 | Higher | +6.2320 | Higher |
| MI (SEI) | **64.2676 ✅** | 63.4235 | 56.1277 | +0.8440 | Higher | +8.1398 | Higher | +7.2958 | Higher |
| Cyclomatic (sum) | **5 ✅** | 6 | 9 | -1 | Lower | -4 | Lower | -3 | Lower |
| Cognitive (sum) | 4 | 4 | 9 | 0 | Same | -5 | Lower | -5 | Lower |
| Halstead Volume | **737.9720 ✅** | 800.9165 | 948.8860 | -62.9445 | Lower | -210.9140 | Lower | -147.9695 | Lower |
| Halstead Effort | **13796.8680 ✅** | 14170.0609 | 21824.3772 | -373.1929 | Lower | -8027.5092 | Lower | -7654.3163 | Lower |
| SLOC | 33 | 33 | 44 | 0 | Same | -11 | Lower | -11 | Lower |
| PLOC | 22 | 22 | 32 | 0 | Same | -10 | Lower | -10 | Lower |
| LLOC | 18 | 18 | 22 | 0 | Same | -4 | Lower | -4 | Lower |
| Comments (CLOC) | 4 | 4 | **6 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p02205

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.3229 | **82.3333 ✅** | 80.8523 | -2.0104 | Lower | -0.5294 | Lower | +1.4810 | Higher |
| MI (SEI) | 66.9982 | **71.1208 ✅** | 68.1514 | -4.1226 | Lower | -1.1532 | Lower | +2.9694 | Higher |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 708.2439 | **660.6753 ✅** | 708.4857 | +47.5686 | Higher | -0.2418 | Lower | -47.8104 | Lower |
| Halstead Effort | 15283.1578 | **11925.1888 ✅** | 13759.5372 | +3357.9690 | Higher | +1523.6205 | Higher | -1834.3485 | Lower |
| SLOC | 31 | **28 ✅** | 30 | +3 | Higher | +1 | Higher | -2 | Lower |
| PLOC | 21 | **18 ✅** | 21 | +3 | Higher | 0 | Same | -3 | Lower |
| LLOC | 21 | **14 ✅** | 21 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02242

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.6951 | **48.1001 ✅** | 32.7366 | -3.4050 | Lower | +11.9585 | Higher | +15.3635 | Higher |
| MI (SEI) | 10.5989 | **20.9288 ✅** | -0.0351 | -10.3299 | Lower | +10.6340 | Higher | +20.9639 | Higher |
| Cyclomatic (sum) | 26 | **24 ✅** | 41 | +2 | Higher | -15 | Lower | -17 | Lower |
| Cognitive (sum) | **24 ✅** | 38 | 49 | -14 | Lower | -25 | Lower | -11 | Lower |
| Halstead Volume | 3368.9756 | **3027.2059 ✅** | 4811.3365 | +341.7697 | Higher | -1442.3609 | Lower | -1784.1306 | Lower |
| Halstead Effort | 230391.9897 | **165081.4339 ✅** | 346299.8220 | +65310.5558 | Higher | -115907.8322 | Lower | -181218.3881 | Lower |
| SLOC | 124 | **107 ✅** | 187 | +17 | Higher | -63 | Lower | -80 | Lower |
| PLOC | 105 | **84 ✅** | 150 | +21 | Higher | -45 | Lower | -66 | Lower |
| LLOC | **64 ✅** | 65 | 96 | -1 | Lower | -32 | Lower | -31 | Lower |
| Comments (CLOC) | 8 | 12 | **20 ✅** | -4 | Lower | -12 | Lower | -8 | Lower |
| Blank Lines | 14 | **13 ✅** | 17 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 4 | **2 ✅** | 7 | +2 | Higher | -3 | Lower | -5 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | **1 ✅** | 6 | +2 | Higher | -3 | Lower | -5 | Lower |

## p02249

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.5838 | **41.5598 ✅** | 38.8702 | -0.9760 | Lower | +1.7136 | Higher | +2.6895 | Higher |
| MI (SEI) | 7.4028 | **15.0298 ✅** | -0.0439 | -7.6270 | Lower | +7.4467 | Higher | +15.0737 | Higher |
| Cyclomatic (sum) | 31 | **21 ✅** | 31 | +10 | Higher | 0 | Same | -10 | Lower |
| Cognitive (sum) | 40 | **28 ✅** | 37 | +12 | Higher | +3 | Higher | -9 | Lower |
| Halstead Volume | **4569.0742 ✅** | 5035.1523 | 4574.9796 | -466.0780 | Lower | -5.9054 | Lower | +460.1727 | Higher |
| Halstead Effort | 287851.6772 | **244352.9785 ✅** | 264332.1565 | +43498.6986 | Higher | +23519.5206 | Higher | -19979.1780 | Lower |
| SLOC | **135 ✅** | 142 | 150 | -7 | Lower | -15 | Lower | -8 | Lower |
| PLOC | 111 | **102 ✅** | 125 | +9 | Higher | -14 | Lower | -23 | Lower |
| LLOC | 79 | **74 ✅** | 82 | +5 | Higher | -3 | Lower | -8 | Lower |
| Comments (CLOC) | 11 | **22 ✅** | 7 | -11 | Lower | +4 | Higher | +15 | Higher |
| Blank Lines | **13 ✅** | 18 | 18 | -5 | Lower | -5 | Lower | 0 | Same |
| Functions | 6 | **2 ✅** | 7 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 6 | +2 | Higher | -2 | Lower | -4 | Lower |

## p02256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.7813 | **76.4447 ✅** | 64.6809 | -10.6634 | Lower | +1.1004 | Higher | +11.7637 | Higher |
| MI (SEI) | 47.1938 | **60.4055 ✅** | 32.8920 | -13.2117 | Lower | +14.3018 | Higher | +27.5134 | Higher |
| Cyclomatic (sum) | 15 | **8 ✅** | 13 | +7 | Higher | +2 | Higher | -5 | Lower |
| Cognitive (sum) | 7 | **5 ✅** | 11 | +2 | Higher | -4 | Lower | -6 | Lower |
| Halstead Volume | 1266.0753 | **857.1308 ✅** | 1230.9163 | +408.9445 | Higher | +35.1590 | Higher | -373.7855 | Lower |
| Halstead Effort | 38043.5192 | **18565.4524 ✅** | 32311.5529 | +19478.0669 | Higher | +5731.9664 | Higher | -13746.1005 | Lower |
| SLOC | 54 | **35 ✅** | 60 | +19 | Higher | -6 | Lower | -25 | Lower |
| PLOC | 39 | **25 ✅** | 47 | +14 | Higher | -8 | Lower | -22 | Lower |
| LLOC | 21 | **16 ✅** | 26 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | **7 ✅** | 4 | 2 | +3 | Higher | +5 | Higher | +2 | Higher |
| Blank Lines | 8 | **6 ✅** | 11 | +2 | Higher | -3 | Lower | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02266

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.3466 | **65.0048 ✅** | 54.6373 | -5.6583 | Lower | +4.7092 | Higher | +10.3675 | Higher |
| MI (SEI) | 44.2620 | **48.4455 ✅** | 36.2081 | -4.1835 | Lower | +8.0539 | Higher | +12.2374 | Higher |
| Cyclomatic (sum) | 14 | **10 ✅** | 14 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 32 | **17 ✅** | 26 | +15 | Higher | +6 | Higher | -9 | Lower |
| Halstead Volume | 1394.1995 | **1254.4146 ✅** | 1819.3223 | +139.7849 | Higher | -425.1228 | Lower | -564.9077 | Lower |
| Halstead Effort | 35731.3409 | **30047.1492 ✅** | 56351.1145 | +5684.1917 | Higher | -20619.7736 | Lower | -26303.9653 | Lower |
| SLOC | 79 | **61 ✅** | 97 | +18 | Higher | -18 | Lower | -36 | Lower |
| PLOC | 54 | **42 ✅** | 69 | +12 | Higher | -15 | Lower | -27 | Lower |
| LLOC | 29 | **28 ✅** | 37 | +1 | Higher | -8 | Lower | -9 | Lower |
| Comments (CLOC) | 17 | 10 | **19 ✅** | +7 | Higher | -2 | Lower | -9 | Lower |
| Blank Lines | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02276

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.4935 | **67.6126 ✅** | 62.3238 | -3.1191 | Lower | +2.1697 | Higher | +5.2888 | Higher |
| MI (SEI) | 43.5000 | **47.7721 ✅** | 36.4156 | -4.2721 | Lower | +7.0844 | Higher | +11.3565 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 12 | +1 | Higher | -3 | Lower | -4 | Lower |
| Cognitive (sum) | 10 | 10 | 14 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | 1523.0751 | **1364.7284 ✅** | 1655.7065 | +158.3467 | Higher | -132.6315 | Lower | -290.9781 | Lower |
| Halstead Effort | 48917.5872 | **39949.3225 ✅** | 62916.8485 | +8968.2647 | Higher | -13999.2613 | Lower | -22967.5260 | Lower |
| SLOC | 60 | **52 ✅** | 64 | +8 | Higher | -4 | Lower | -12 | Lower |
| PLOC | 43 | **40 ✅** | 49 | +3 | Higher | -6 | Lower | -9 | Lower |
| LLOC | 31 | **30 ✅** | 32 | +1 | Higher | -1 | Lower | -2 | Lower |
| Comments (CLOC) | **7 ✅** | 6 | 5 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 10 | **7 ✅** | 11 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |

## p02277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.9031 | **62.3099 ✅** | 56.1203 | -6.4068 | Lower | -0.2172 | Lower | +6.1895 | Higher |
| MI (SEI) | 26.8226 | **40.0282 ✅** | 24.0735 | -13.2056 | Lower | +2.7491 | Higher | +15.9546 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 17 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 17 | 17 | 18 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 2302.9142 | **1588.3136 ✅** | 2270.6236 | +714.6005 | Higher | +32.2905 | Higher | -682.3100 | Lower |
| Halstead Effort | 91976.1448 | **46171.3946 ✅** | 104108.0939 | +45804.7502 | Higher | -12131.9491 | Lower | -57936.6993 | Lower |
| SLOC | 82 | **64 ✅** | 79 | +18 | Higher | +3 | Higher | -15 | Lower |
| PLOC | 66 | **50 ✅** | 66 | +16 | Higher | 0 | Same | -16 | Lower |
| LLOC | 43 | **34 ✅** | 36 | +9 | Higher | +7 | Higher | -2 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 4 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 11 | **8 ✅** | 11 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p02279

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.5531 | **56.6128 ✅** | 50.2288 | -6.0596 | Lower | +0.3244 | Higher | +6.3840 | Higher |
| MI (SEI) | 23.7591 | **33.9377 ✅** | 11.7252 | -10.1785 | Lower | +12.0339 | Higher | +22.2125 | Higher |
| Cyclomatic (sum) | 16 | 16 | 18 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 19 | 19 | 20 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 2770.4359 | **2246.2738 ✅** | 3042.7407 | +524.1621 | Higher | -272.3048 | Lower | -796.4669 | Lower |
| Halstead Effort | 112021.9743 | **75352.2753 ✅** | 125289.3229 | +36669.6990 | Higher | -13267.3486 | Lower | -49937.0476 | Lower |
| SLOC | 106 | **78 ✅** | 102 | +28 | Higher | +4 | Higher | -24 | Lower |
| PLOC | 82 | **61 ✅** | 88 | +21 | Higher | -6 | Lower | -27 | Lower |
| LLOC | 53 | **37 ✅** | 56 | +16 | Higher | -3 | Lower | -19 | Lower |
| Comments (CLOC) | **12 ✅** | 10 | 3 | +2 | Higher | +9 | Higher | +7 | Higher |
| Blank Lines | 14 | **9 ✅** | 12 | +5 | Higher | +2 | Higher | -3 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02297

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.7569 ✅** | 77.4634 | 68.2493 | +2.2934 | Higher | +11.5076 | Higher | +9.2142 | Higher |
| MI (SEI) | **66.9779 ✅** | 64.9211 | 54.3878 | +2.0568 | Higher | +12.5901 | Higher | +10.5333 | Higher |
| Cyclomatic (sum) | **4 ✅** | 5 | 8 | -1 | Lower | -4 | Lower | -3 | Lower |
| Cognitive (sum) | **2 ✅** | 5 | 6 | -3 | Lower | -4 | Lower | -1 | Lower |
| Halstead Volume | 972.0530 | **966.4985 ✅** | 1453.0128 | +5.5546 | Higher | -480.9598 | Lower | -486.5144 | Lower |
| Halstead Effort | **21241.1592 ✅** | 25090.3000 | 38060.8639 | -3849.1408 | Lower | -16819.7047 | Lower | -12970.5639 | Lower |
| SLOC | **29 ✅** | 33 | 49 | -4 | Lower | -20 | Lower | -16 | Lower |
| PLOC | **20 ✅** | 21 | 33 | -1 | Lower | -13 | Lower | -12 | Lower |
| LLOC | **17 ✅** | 19 | 26 | -2 | Lower | -9 | Lower | -7 | Lower |
| Comments (CLOC) | 4 | 5 | **9 ✅** | -1 | Lower | -5 | Lower | -4 | Lower |
| Blank Lines | **5 ✅** | 7 | 7 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02305

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.3019 | **72.6831 ✅** | 70.9508 | -0.3813 | Lower | +1.3511 | Higher | +1.7324 | Higher |
| MI (SEI) | 52.5826 | **59.6343 ✅** | 47.9209 | -7.0518 | Lower | +4.6617 | Higher | +11.7135 | Higher |
| Cyclomatic (sum) | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 6 | **5 ✅** | 7 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 1213.8990 | **1179.0891 ✅** | 1440.7960 | +34.8100 | Higher | -226.8970 | Lower | -261.7070 | Lower |
| Halstead Effort | 25844.3018 | **21908.2356 ✅** | 40435.2436 | +3936.0662 | Higher | -14590.9418 | Lower | -18527.0080 | Lower |
| SLOC | 41 | 41 | 41 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 31 | 31 | 32 | 0 | Same | -1 | Lower | -1 | Lower |
| LLOC | 18 | 18 | 26 | 0 | Same | -8 | Lower | -8 | Lower |
| Comments (CLOC) | 4 | **7 ✅** | 3 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.4417 | **72.8886 ✅** | 60.4604 | -7.4469 | Lower | +4.9813 | Higher | +12.4282 | Higher |
| MI (SEI) | 40.0166 | **63.7269 ✅** | 38.0176 | -23.7102 | Lower | +1.9990 | Higher | +25.7092 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 13 | +4 | Higher | -3 | Lower | -7 | Lower |
| Cognitive (sum) | 11 | **10 ✅** | 13 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 1592.3790 | **909.5830 ✅** | 1793.1840 | +682.7960 | Higher | -200.8050 | Lower | -883.6010 | Lower |
| Halstead Effort | 64106.0955 | **21725.0400 ✅** | 74596.4534 | +42381.0555 | Higher | -10490.3580 | Lower | -52871.4135 | Lower |
| SLOC | 55 | **44 ✅** | 69 | +11 | Higher | -14 | Lower | -25 | Lower |
| PLOC | 44 | **27 ✅** | 52 | +17 | Higher | -8 | Lower | -25 | Lower |
| LLOC | 36 | **21 ✅** | 41 | +15 | Higher | -5 | Lower | -20 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 8 | -6 | Lower | -4 | Lower | +2 | Higher |
| Blank Lines | 7 | 7 | 9 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **0 ✅** | 5 | +3 | Higher | -2 | Lower | -5 | Lower |

## p02328

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.1639 | **66.3573 ✅** | 55.7775 | -1.1934 | Lower | +9.3864 | Higher | +10.5798 | Higher |
| MI (SEI) | 41.2958 | **47.6174 ✅** | 27.8001 | -6.3216 | Lower | +13.4957 | Higher | +19.8173 | Higher |
| Cyclomatic (sum) | **10 ✅** | 11 | 15 | -1 | Lower | -5 | Lower | -4 | Lower |
| Cognitive (sum) | 15 | **14 ✅** | 19 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | 1423.5996 | **1352.6779 ✅** | 2109.3633 | +70.9217 | Higher | -685.7636 | Lower | -756.6853 | Lower |
| Halstead Effort | 43743.3337 | **43716.0918 ✅** | 110824.8356 | +27.2419 | Higher | -67081.5019 | Lower | -67108.7438 | Lower |
| SLOC | 58 | **54 ✅** | 85 | +4 | Higher | -27 | Lower | -31 | Lower |
| PLOC | 43 | **39 ✅** | 67 | +4 | Higher | -24 | Lower | -28 | Lower |
| LLOC | 26 | 26 | 48 | 0 | Same | -22 | Lower | -22 | Lower |
| Comments (CLOC) | 5 | 7 | 7 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 10 | **8 ✅** | 12 | +2 | Higher | -2 | Lower | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p02333

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.5631 ✅** | 56.3964 | 57.1890 | +1.1666 | Higher | +0.3740 | Higher | -0.7926 | Lower |
| MI (SEI) | 26.9685 | **30.9910 ✅** | 29.9555 | -4.0225 | Lower | -2.9871 | Lower | +1.0354 | Higher |
| Cyclomatic (sum) | 15 | 15 | 15 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | 2739.7895 | **2447.5994 ✅** | 2696.7460 | +292.1902 | Higher | +43.0435 | Higher | -249.1467 | Lower |
| Halstead Effort | 171579.3180 | **142266.7124 ✅** | 168442.9064 | +29312.6055 | Higher | +3136.4116 | Higher | -26176.1940 | Lower |
| SLOC | **70 ✅** | 78 | 72 | -8 | Lower | -2 | Lower | +6 | Higher |
| PLOC | **56 ✅** | 59 | 57 | -3 | Lower | -1 | Lower | +2 | Higher |
| LLOC | 42 | 48 | 42 | -6 | Lower | 0 | Same | +6 | Higher |
| Comments (CLOC) | 4 | **8 ✅** | 6 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 4 | **3 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p02340

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.2609 ✅** | 69.9535 | 63.6202 | +3.3074 | Higher | +9.6407 | Higher | +6.3333 | Higher |
| MI (SEI) | 56.6404 | **59.1932 ✅** | 48.2108 | -2.5528 | Lower | +8.4296 | Higher | +10.9824 | Higher |
| Cyclomatic (sum) | **6 ✅** | 9 | 11 | -3 | Lower | -5 | Lower | -2 | Lower |
| Cognitive (sum) | **6 ✅** | 8 | 8 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | **1139.4677 ✅** | 1219.5714 | 1737.7060 | -80.1037 | Lower | -598.2383 | Lower | -518.1346 | Lower |
| Halstead Effort | **31280.2269 ✅** | 40572.5287 | 55794.4534 | -9292.3018 | Lower | -24514.2265 | Lower | -15221.9247 | Lower |
| SLOC | **40 ✅** | 46 | 59 | -6 | Lower | -19 | Lower | -13 | Lower |
| PLOC | 28 | 28 | 39 | 0 | Same | -11 | Lower | -11 | Lower |
| LLOC | **19 ✅** | 22 | 22 | -3 | Lower | -3 | Lower | 0 | Same |
| Comments (CLOC) | 5 | 10 | **11 ✅** | -5 | Lower | -6 | Lower | -1 | Lower |
| Blank Lines | **7 ✅** | 8 | 9 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | **0 ✅** | 2 | 4 | -2 | Lower | -4 | Lower | -2 | Lower |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02347

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.1735 | **49.1879 ✅** | 38.6784 | -9.0144 | Lower | +1.4951 | Higher | +10.5095 | Higher |
| MI (SEI) | 1.0269 | **16.4947 ✅** | 2.7743 | -15.4678 | Lower | -1.7474 | Lower | +13.7204 | Higher |
| Cyclomatic (sum) | 29 | **19 ✅** | 29 | +10 | Higher | 0 | Same | -10 | Lower |
| Cognitive (sum) | 29 | **23 ✅** | 28 | +6 | Higher | +1 | Higher | -5 | Lower |
| Halstead Volume | 4931.8128 | **3063.6374 ✅** | 5641.5825 | +1868.1754 | Higher | -709.7696 | Lower | -2577.9450 | Lower |
| Halstead Effort | 344569.3237 | **133268.2288 ✅** | 427892.3330 | +211301.0949 | Higher | -83323.0093 | Lower | -294624.1042 | Lower |
| SLOC | 139 | **107 ✅** | 146 | +32 | Higher | -7 | Lower | -39 | Lower |
| PLOC | 115 | **84 ✅** | 116 | +31 | Higher | -1 | Lower | -32 | Lower |
| LLOC | 59 | **50 ✅** | 68 | +9 | Higher | -9 | Lower | -18 | Lower |
| Comments (CLOC) | 6 | 7 | **10 ✅** | -1 | Lower | -4 | Lower | -3 | Lower |
| Blank Lines | 21 | **16 ✅** | 23 | +5 | Higher | -2 | Lower | -7 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p02353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.4978 | **37.1289 ✅** | 35.3288 | -0.6311 | Lower | +1.1690 | Higher | +1.8001 | Higher |
| MI (SEI) | -1.9871 | **0.3375 ✅** | -8.1418 | -2.3246 | Lower | +6.1547 | Higher | +8.4793 | Higher |
| Cyclomatic (sum) | 33 | 33 | 34 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 24 | **23 ✅** | 25 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | **4910.9083 ✅** | 5180.1266 | 5360.3518 | -269.2183 | Lower | -449.4435 | Lower | -180.2252 | Lower |
| Halstead Effort | **326575.3998 ✅** | 400026.0242 | 341871.3243 | -73450.6244 | Lower | -15295.9245 | Lower | +58154.6999 | Higher |
| SLOC | 165 | **156 ✅** | 170 | +9 | Higher | -5 | Lower | -14 | Lower |
| PLOC | 136 | **132 ✅** | 145 | +4 | Higher | -9 | Lower | -13 | Lower |
| LLOC | 65 | **63 ✅** | 72 | +2 | Higher | -7 | Lower | -9 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 5 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | 23 | **16 ✅** | 20 | +7 | Higher | +3 | Higher | -4 | Lower |
| Functions | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |

## p02355

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.0059 | **53.3603 ✅** | 45.1595 | -1.3544 | Lower | +6.8465 | Higher | +8.2009 | Higher |
| MI (SEI) | 25.1635 | **29.8966 ✅** | 8.3315 | -4.7330 | Lower | +16.8321 | Higher | +21.5651 | Higher |
| Cyclomatic (sum) | 25 | **24 ✅** | 25 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 33 | 31 | **24 ✅** | +2 | Higher | +9 | Higher | +7 | Higher |
| Halstead Volume | 2187.6845 | **2097.4048 ✅** | 2779.6857 | +90.2797 | Higher | -592.0012 | Lower | -682.2809 | Lower |
| Halstead Effort | 145173.3729 | **107491.9939 ✅** | 182461.4202 | +37681.3790 | Higher | -37288.0472 | Lower | -74969.4263 | Lower |
| SLOC | 92 | **87 ✅** | 130 | +5 | Higher | -38 | Lower | -43 | Lower |
| PLOC | 72 | **66 ✅** | 107 | +6 | Higher | -35 | Lower | -41 | Lower |
| LLOC | 62 | **45 ✅** | 89 | +17 | Higher | -27 | Lower | -44 | Lower |
| Comments (CLOC) | 9 | **11 ✅** | 6 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | 13 | **11 ✅** | 17 | +2 | Higher | -4 | Lower | -6 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 5 | 5 | -2 | Lower | -2 | Lower | 0 | Same |

## p02369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.7760 ✅** | 64.2394 | 55.9711 | +1.5366 | Higher | +9.8049 | Higher | +8.2683 | Higher |
| MI (SEI) | **44.7961 ✅** | 42.1962 | 31.9114 | +2.5999 | Higher | +12.8847 | Higher | +10.2848 | Higher |
| Cyclomatic (sum) | **11 ✅** | 13 | 15 | -2 | Lower | -4 | Lower | -2 | Lower |
| Cognitive (sum) | 16 | 16 | 20 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | **1428.6093 ✅** | 1489.2593 | 2032.2752 | -60.6500 | Lower | -603.6659 | Lower | -543.0159 | Lower |
| Halstead Effort | **41251.0939 ✅** | 49299.6188 | 73536.2727 | -8048.5250 | Lower | -32285.1789 | Lower | -24236.6539 | Lower |
| SLOC | **55 ✅** | 58 | 85 | -3 | Lower | -30 | Lower | -27 | Lower |
| PLOC | **42 ✅** | 45 | 63 | -3 | Lower | -21 | Lower | -18 | Lower |
| LLOC | 32 | **29 ✅** | 36 | +3 | Higher | -4 | Lower | -7 | Lower |
| Comments (CLOC) | 6 | 6 | **10 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 7 | 7 | 13 | 0 | Same | -6 | Lower | -6 | Lower |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |

## p02370

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.9770 | **68.2351 ✅** | 48.0621 | -0.2580 | Lower | +19.9149 | Higher | +20.1729 | Higher |
| MI (SEI) | 44.3984 | **49.1194 ✅** | 19.9688 | -4.7210 | Lower | +24.4296 | Higher | +29.1506 | Higher |
| Cyclomatic (sum) | 8 | 8 | 18 | 0 | Same | -10 | Lower | -10 | Lower |
| Cognitive (sum) | 10 | 10 | 17 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | 1437.7080 | **1368.1055 ✅** | 2855.3029 | +69.6025 | Higher | -1417.5948 | Lower | -1487.1973 | Lower |
| Halstead Effort | 40812.3571 | **38124.5408 ✅** | 150632.4144 | +2687.8163 | Higher | -109820.0573 | Lower | -112507.8735 | Lower |
| SLOC | 50 | 50 | 119 | 0 | Same | -69 | Lower | -69 | Lower |
| PLOC | 37 | **35 ✅** | 87 | +2 | Higher | -50 | Lower | -52 | Lower |
| LLOC | 31 | **27 ✅** | 75 | +4 | Higher | -44 | Lower | -48 | Lower |
| Comments (CLOC) | 4 | 6 | **13 ✅** | -2 | Lower | -9 | Lower | -7 | Lower |
| Blank Lines | 9 | 9 | 19 | 0 | Same | -10 | Lower | -10 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p02371

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.5706 ✅** | 56.8784 | 38.8044 | +0.6922 | Higher | +18.7662 | Higher | +18.0740 | Higher |
| MI (SEI) | **39.1547 ✅** | 33.2955 | -0.0859 | +5.8592 | Higher | +39.2406 | Higher | +33.3815 | Higher |
| Cyclomatic (sum) | **11 ✅** | 15 | 23 | -4 | Lower | -12 | Lower | -8 | Lower |
| Cognitive (sum) | **17 ✅** | 19 | 23 | -2 | Lower | -6 | Lower | -4 | Lower |
| Halstead Volume | 2426.4357 | **2418.9757 ✅** | 6080.7137 | +7.4600 | Higher | -3654.2780 | Lower | -3661.7380 | Lower |
| Halstead Effort | **92435.6448 ✅** | 96534.0064 | 505523.7400 | -4098.3616 | Lower | -413088.0952 | Lower | -408989.7336 | Lower |
| SLOC | 77 | **76 ✅** | 154 | +1 | Higher | -77 | Lower | -78 | Lower |
| PLOC | **53 ✅** | 58 | 134 | -5 | Lower | -81 | Lower | -76 | Lower |
| LLOC | 44 | **37 ✅** | 118 | +7 | Higher | -74 | Lower | -81 | Lower |
| Comments (CLOC) | **14 ✅** | 9 | 8 | +5 | Higher | +6 | Higher | +1 | Higher |
| Blank Lines | 11 | **10 ✅** | 15 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 2 | 2 | 6 | 0 | Same | -4 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |

## p02393

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 89.9348 | 89.9348 | 73.2467 | 0 | Same | +16.6881 | Higher | +16.6881 | Higher |
| MI (SEI) | 83.1232 | 83.1232 | 62.5966 | 0 | Same | +20.5265 | Higher | +20.5265 | Higher |
| Cyclomatic (sum) | 2 | 2 | 14 | 0 | Same | -12 | Lower | -12 | Lower |
| Cognitive (sum) | 0 | 0 | 8 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | 560 | 560 | 1022.5800 | 0 | Same | -462.5800 | Lower | -462.5800 | Lower |
| Halstead Effort | 6130.5263 | 6130.5263 | 28255.5009 | 0 | Same | -22124.9745 | Lower | -22124.9745 | Lower |
| SLOC | 19 | 19 | 37 | 0 | Same | -18 | Lower | -18 | Lower |
| PLOC | 12 | 12 | 25 | 0 | Same | -13 | Lower | -13 | Lower |
| LLOC | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 3 | 3 | **7 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02396

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.4883 | **91.7723 ✅** | 83.3374 | -3.2841 | Lower | +5.1509 | Higher | +8.4350 | Higher |
| MI (SEI) | 74.8829 | 57.1059 | **75.5847 ✅** | +17.7770 | Higher | -0.7018 | Lower | -18.4788 | Lower |
| Cyclomatic (sum) | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |
| Halstead Volume | 428.7735 | **360 ✅** | 498.5531 | +68.7735 | Higher | -69.7796 | Lower | -138.5531 | Lower |
| Halstead Effort | 5447.9456 | **4500 ✅** | 7478.2967 | +947.9456 | Higher | -2030.3511 | Lower | -2978.2967 | Lower |
| SLOC | 22 | **19 ✅** | 28 | +3 | Higher | -6 | Lower | -9 | Lower |
| PLOC | 16 | **14 ✅** | 18 | +2 | Higher | -2 | Lower | -4 | Lower |
| LLOC | 12 | **10 ✅** | 11 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | 2 | 0 | **5 ✅** | +2 | Higher | -3 | Lower | -5 | Lower |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02412

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.4423 ✅** | 67.2019 | 68.2612 | +4.2404 | Higher | +3.1811 | Higher | -1.0593 | Lower |
| MI (SEI) | **51.5498 ✅** | 45.8843 | 49.7680 | +5.6655 | Higher | +1.7818 | Higher | -3.8837 | Lower |
| Cyclomatic (sum) | 14 | 15 | 14 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 19 | **16 ✅** | 21 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | **905.8642 ✅** | 1083.6204 | 1043.9532 | -177.7562 | Lower | -138.0890 | Lower | +39.6672 | Higher |
| Halstead Effort | **23250.5135 ✅** | 27671.0206 | 25295.7893 | -4420.5072 | Lower | -2045.2759 | Lower | +2375.2313 | Higher |
| SLOC | **43 ✅** | 52 | 50 | -9 | Lower | -7 | Lower | +2 | Higher |
| PLOC | **33 ✅** | 41 | 36 | -8 | Lower | -3 | Lower | +5 | Higher |
| LLOC | **23 ✅** | 25 | 25 | -2 | Lower | -2 | Lower | 0 | Same |
| Comments (CLOC) | 4 | 5 | **6 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | **6 ✅** | 9 | 9 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02418

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **89.8663 ✅** | 87.4592 | 82.0964 | +2.4071 | Higher | +7.7699 | Higher | +5.3628 | Higher |
| MI (SEI) | **81.8867 ✅** | 77.3198 | 75.8408 | +4.5670 | Higher | +6.0459 | Higher | +1.4790 | Higher |
| Cyclomatic (sum) | 3 | 3 | 7 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | **397.4581 ✅** | 475.6028 | 542.8421 | -78.1447 | Lower | -145.3840 | Lower | -67.2393 | Lower |
| Halstead Effort | **3444.6372 ✅** | 5495.8551 | 6979.3988 | -2051.2179 | Lower | -3534.7616 | Lower | -1483.5436 | Lower |
| SLOC | **21 ✅** | 23 | 29 | -2 | Lower | -8 | Lower | -6 | Lower |
| PLOC | **14 ✅** | 16 | 20 | -2 | Lower | -6 | Lower | -4 | Lower |
| LLOC | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |
| Comments (CLOC) | 3 | 3 | **6 ✅** | 0 | Same | -3 | Lower | -3 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p02430

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.3280 | 68.6348 | **71.5200 ✅** | +1.6932 | Higher | -1.1920 | Lower | -2.8853 | Lower |
| MI (SEI) | 59.5446 | 35.8446 | **62.9561 ✅** | +23.7000 | Higher | -3.4115 | Lower | -27.1115 | Lower |
| Cyclomatic (sum) | 10 | 13 | **8 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Cognitive (sum) | 10 | 18 | **8 ✅** | -8 | Lower | +2 | Higher | +10 | Higher |
| Halstead Volume | 1015.3641 | 1231.4296 | **1009.9948 ✅** | -216.0655 | Lower | +5.3693 | Higher | +221.4348 | Higher |
| Halstead Effort | **23770.3997 ✅** | 37455.9842 | 23987.3777 | -13685.5846 | Lower | -216.9780 | Lower | +13468.6066 | Higher |
| SLOC | 47 | 47 | **45 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| PLOC | 31 | 37 | **27 ✅** | -6 | Lower | +4 | Higher | +10 | Higher |
| LLOC | **22 ✅** | 27 | 23 | -5 | Lower | -1 | Lower | +4 | Higher |
| Comments (CLOC) | 10 | 1 | **11 ✅** | +9 | Higher | -1 | Lower | -10 | Lower |
| Blank Lines | **6 ✅** | 9 | 7 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02433

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.8866 | **57.7152 ✅** | 29.3009 | -19.8286 | Lower | +8.5857 | Higher | +28.4143 | Higher |
| MI (SEI) | 4.8915 | **44.2669 ✅** | -11.3202 | -39.3754 | Lower | +16.2116 | Higher | +55.5871 | Higher |
| Cyclomatic (sum) | 30 | **17 ✅** | 41 | +13 | Higher | -11 | Lower | -24 | Lower |
| Cognitive (sum) | **20 ✅** | 30 | 45 | -10 | Lower | -25 | Lower | -15 | Lower |
| Halstead Volume | 3912.0983 | **1606.6742 ✅** | 6119.8681 | +2305.4240 | Higher | -2207.7698 | Lower | -4513.1939 | Lower |
| Halstead Effort | 193648.8640 | **66884.2938 ✅** | 385325.0289 | +126764.5702 | Higher | -191676.1649 | Lower | -318440.7350 | Lower |
| SLOC | 170 | **80 ✅** | 214 | +90 | Higher | -44 | Lower | -134 | Lower |
| PLOC | 131 | **57 ✅** | 167 | +74 | Higher | -36 | Lower | -110 | Lower |
| LLOC | 77 | **36 ✅** | 120 | +41 | Higher | -43 | Lower | -84 | Lower |
| Comments (CLOC) | 16 | **20 ✅** | 12 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 25 | **6 ✅** | 35 | +19 | Higher | -10 | Lower | -29 | Lower |
| Functions | 8 | **1 ✅** | 7 | +7 | Higher | +1 | Higher | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 8 | **1 ✅** | 10 | +7 | Higher | -2 | Lower | -9 | Lower |

## p02444

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.7002 | **70.5442 ✅** | 65.0144 | -2.8440 | Lower | +2.6858 | Higher | +5.5298 | Higher |
| MI (SEI) | 48.3477 | **53.4836 ✅** | 49.5575 | -5.1358 | Lower | -1.2098 | Lower | +3.9260 | Higher |
| Cyclomatic (sum) | 8 | **6 ✅** | 10 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 10 | **5 ✅** | 11 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | 1516.3170 | **1331.1784 ✅** | 1634.3767 | +185.1385 | Higher | -118.0597 | Lower | -303.1983 | Lower |
| Halstead Effort | 47258.5454 | **33254.8093 ✅** | 57101.0359 | +14003.7361 | Higher | -9842.4905 | Lower | -23846.2265 | Lower |
| SLOC | 50 | **45 ✅** | 56 | +5 | Higher | -6 | Lower | -11 | Lower |
| PLOC | 35 | **31 ✅** | 38 | +4 | Higher | -3 | Lower | -7 | Lower |
| LLOC | 28 | **24 ✅** | 33 | +4 | Higher | -5 | Lower | -9 | Lower |
| Comments (CLOC) | 6 | 6 | **10 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02449

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.2913 | **46.2965 ✅** | 45.1655 | -0.0052 | Lower | +1.1258 | Higher | +1.1311 | Higher |
| MI (SEI) | **21.0585 ✅** | 19.7716 | 13.7753 | +1.2869 | Higher | +7.2832 | Higher | +5.9963 | Higher |
| Cyclomatic (sum) | 25 | 25 | **21 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Cognitive (sum) | 22 | **20 ✅** | 30 | +2 | Higher | -8 | Lower | -10 | Lower |
| Halstead Volume | **2796 ✅** | 2942.0533 | 3744.5255 | -146.0533 | Lower | -948.5255 | Lower | -802.4722 | Lower |
| Halstead Effort | **157104.9730 ✅** | 168979.4691 | 281641.8102 | -11874.4962 | Lower | -124536.8372 | Lower | -112662.3411 | Lower |
| SLOC | 121 | **119 ✅** | 125 | +2 | Higher | -4 | Lower | -6 | Lower |
| PLOC | 88 | **87 ✅** | 101 | +1 | Higher | -13 | Lower | -14 | Lower |
| LLOC | 57 | 57 | 95 | 0 | Same | -38 | Lower | -38 | Lower |
| Comments (CLOC) | **17 ✅** | 15 | 11 | +2 | Higher | +6 | Higher | +4 | Higher |
| Blank Lines | 18 | 19 | **14 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| Functions | 4 | 4 | **1 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 8 | 8 | **2 ✅** | 0 | Same | +6 | Higher | +6 | Higher |

## p02450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.9001 ✅** | 65.1055 | 65.3744 | +2.7947 | Higher | +2.5257 | Higher | -0.2690 | Lower |
| MI (SEI) | 46.9207 | **49.1177 ✅** | 46.0848 | -2.1970 | Lower | +0.8358 | Higher | +3.0329 | Higher |
| Cyclomatic (sum) | **9 ✅** | 11 | 12 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **13 ✅** | 17 | 16 | -4 | Lower | -3 | Lower | +1 | Higher |
| Halstead Volume | 1486.6829 | **1305.9659 ✅** | 1476.5434 | +180.7170 | Higher | +10.1395 | Higher | -170.5775 | Lower |
| Halstead Effort | 51554.3250 | **34292.1359 ✅** | 54816.6739 | +17262.1891 | Higher | -3262.3489 | Lower | -20524.5380 | Lower |
| SLOC | **49 ✅** | 59 | 55 | -10 | Lower | -6 | Lower | +4 | Higher |
| PLOC | **35 ✅** | 41 | 38 | -6 | Lower | -3 | Lower | +3 | Higher |
| LLOC | 24 | 30 | 24 | -6 | Lower | 0 | Same | +6 | Higher |
| Comments (CLOC) | 5 | **10 ✅** | 7 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02457

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.5717 | **64.3527 ✅** | 36.9285 | -25.7810 | Lower | +1.6433 | Higher | +27.4242 | Higher |
| MI (SEI) | 7.5832 | **48.3530 ✅** | 8.0789 | -40.7698 | Lower | -0.4957 | Lower | +40.2741 | Higher |
| Cyclomatic (sum) | 36 | **12 ✅** | 36 | +24 | Higher | 0 | Same | -24 | Lower |
| Cognitive (sum) | 44 | **11 ✅** | 43 | +33 | Higher | +1 | Higher | -32 | Lower |
| Halstead Volume | 4410.6990 | **1523.0751 ✅** | 4621.7507 | +2887.6240 | Higher | -211.0516 | Lower | -3098.6756 | Lower |
| Halstead Effort | 321072.9435 | **32416.8002 ✅** | 390868.0551 | +288656.1433 | Higher | -69795.1117 | Lower | -358451.2549 | Lower |
| SLOC | 144 | **58 ✅** | 157 | +86 | Higher | -13 | Lower | -99 | Lower |
| PLOC | 117 | **41 ✅** | 121 | +76 | Higher | -4 | Lower | -80 | Lower |
| LLOC | 65 | **21 ✅** | 69 | +44 | Higher | -4 | Lower | -48 | Lower |
| Comments (CLOC) | 15 | 10 | **21 ✅** | +5 | Higher | -6 | Lower | -11 | Lower |
| Blank Lines | 12 | **7 ✅** | 15 | +5 | Higher | -3 | Lower | -8 | Lower |
| Functions | 6 | **1 ✅** | 6 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 11 | **1 ✅** | 10 | +10 | Higher | +1 | Higher | -9 | Lower |

## p02542

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 13.1618 | **19.1369 ✅** | 7.1073 | -5.9751 | Lower | +6.0544 | Higher | +12.0296 | Higher |
| MI (SEI) | -26.4017 | **-17.3382 ✅** | -45.2413 | -9.0636 | Lower | +18.8395 | Higher | +27.9031 | Higher |
| Cyclomatic (sum) | 57 | **55 ✅** | 63 | +2 | Higher | -6 | Lower | -8 | Lower |
| Cognitive (sum) | 146 | **90 ✅** | 139 | +56 | Higher | +7 | Higher | -49 | Lower |
| Halstead Volume | 11704.1360 | **9528.8303 ✅** | 15114.2019 | +2175.3057 | Higher | -3410.0659 | Lower | -5585.3716 | Lower |
| Halstead Effort | 1391542.2318 | **817163.9914 ✅** | 2577121.0700 | +574378.2404 | Higher | -1185578.8382 | Lower | -1759957.0786 | Lower |
| SLOC | 375 | **285 ✅** | 461 | +90 | Higher | -86 | Lower | -176 | Lower |
| PLOC | 295 | **219 ✅** | 396 | +76 | Higher | -101 | Lower | -177 | Lower |
| LLOC | 237 | **157 ✅** | 298 | +80 | Higher | -61 | Lower | -141 | Lower |
| Comments (CLOC) | **41 ✅** | 33 | 15 | +8 | Higher | +26 | Higher | +18 | Higher |
| Blank Lines | 44 | **34 ✅** | 56 | +10 | Higher | -12 | Lower | -22 | Lower |
| Functions | **2 ✅** | 4 | 5 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | **4 ✅** | 9 | 6 | -5 | Lower | -2 | Lower | +3 | Higher |

## p02562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 17.9192 | **29.4930 ✅** | 7.5690 | -11.5738 | Lower | +10.3502 | Higher | +21.9240 | Higher |
| MI (SEI) | -18.6855 | **-6.4281 ✅** | -41.8068 | -12.2574 | Lower | +23.1213 | Higher | +35.3787 | Higher |
| Cyclomatic (sum) | 55 | **38 ✅** | 64 | +17 | Higher | -9 | Lower | -26 | Lower |
| Cognitive (sum) | 107 | **73 ✅** | 127 | +34 | Higher | -20 | Lower | -54 | Lower |
| Halstead Volume | 10264.4367 | **6833.7070 ✅** | 14465.0111 | +3430.7297 | Higher | -4200.5744 | Lower | -7631.3041 | Lower |
| Halstead Effort | 1191577.0255 | **530645.2959 ✅** | 2271232.7554 | +660931.7296 | Higher | -1079655.7299 | Lower | -1740587.4595 | Lower |
| SLOC | 300 | **213 ✅** | 448 | +87 | Higher | -148 | Lower | -235 | Lower |
| PLOC | 233 | **166 ✅** | 383 | +67 | Higher | -150 | Lower | -217 | Lower |
| LLOC | 180 | **129 ✅** | 277 | +51 | Higher | -97 | Lower | -148 | Lower |
| Comments (CLOC) | **36 ✅** | 20 | 21 | +16 | Higher | +15 | Higher | -1 | Lower |
| Blank Lines | 32 | **29 ✅** | 51 | +3 | Higher | -19 | Lower | -22 | Lower |
| Functions | **2 ✅** | 4 | 6 | -2 | Lower | -4 | Lower | -2 | Lower |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |

## p02584

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.1392 | **79.2642 ✅** | 76.9508 | -2.1251 | Lower | +0.1884 | Higher | +2.3135 | Higher |
| MI (SEI) | 63.6272 | 62.2351 | **67.2841 ✅** | +1.3921 | Higher | -3.6569 | Lower | -5.0490 | Lower |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 895.1247 | 868.1631 | **850.1643 ✅** | +26.9616 | Higher | +44.9604 | Higher | +17.9988 | Higher |
| Halstead Effort | 18486.2708 | **16853.6881 ✅** | 19128.6958 | +1632.5827 | Higher | -642.4250 | Lower | -2275.0077 | Lower |
| SLOC | 35 | **31 ✅** | 36 | +4 | Higher | -1 | Lower | -5 | Lower |
| PLOC | 25 | 24 | **23 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| LLOC | 21 | **14 ✅** | 22 | +7 | Higher | -1 | Lower | -8 | Lower |
| Comments (CLOC) | 5 | 3 | **7 ✅** | +2 | Higher | -2 | Lower | -4 | Lower |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02585

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.6169 ✅** | 45.5419 | 49.7770 | +6.0750 | Higher | +1.8398 | Higher | -4.2352 | Lower |
| MI (SEI) | **28.0281 ✅** | 26.9082 | 25.9146 | +1.1200 | Higher | +2.1135 | Higher | +0.9936 | Higher |
| Cyclomatic (sum) | **16 ✅** | 19 | 19 | -3 | Lower | -3 | Lower | 0 | Same |
| Cognitive (sum) | **26 ✅** | 41 | 29 | -15 | Lower | -3 | Lower | +12 | Higher |
| Halstead Volume | **2883.2235 ✅** | 3211.1049 | 3080.2283 | -327.8813 | Lower | -197.0047 | Lower | +130.8766 | Higher |
| Halstead Effort | 216393.5144 | **153257.2774 ✅** | 230650.4260 | +63136.2370 | Higher | -14256.9116 | Lower | -77393.1486 | Lower |
| SLOC | **98 ✅** | 132 | 103 | -34 | Lower | -5 | Lower | +29 | Higher |
| PLOC | **68 ✅** | 84 | 72 | -16 | Lower | -4 | Lower | +12 | Higher |
| LLOC | **71 ✅** | 75 | 72 | -4 | Lower | -1 | Lower | +3 | Higher |
| Comments (CLOC) | 14 | **33 ✅** | 15 | -19 | Lower | -1 | Lower | +18 | Higher |
| Blank Lines | 16 | 16 | 16 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02606

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 91.5001 | **92.4261 ✅** | 85.9668 | -0.9260 | Lower | +5.5333 | Higher | +6.4593 | Higher |
| MI (SEI) | **86.0659 ✅** | 83.1812 | 83.4981 | +2.8848 | Higher | +2.5678 | Higher | -0.3169 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 490.4654 | 490.4654 | 580.1053 | 0 | Same | -89.6399 | Lower | -89.6399 | Lower |
| Halstead Effort | 5452.8216 | 5452.8216 | 7052.8595 | 0 | Same | -1600.0379 | Lower | -1600.0379 | Lower |
| SLOC | 18 | **17 ✅** | 24 | +1 | Higher | -6 | Lower | -7 | Lower |
| PLOC | 11 | 11 | 13 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 3 | 2 | **6 ✅** | +1 | Higher | -3 | Lower | -4 | Lower |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02617

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.5152 ✅** | 79.8645 | 74.0971 | +1.6508 | Higher | +7.4182 | Higher | +5.7674 | Higher |
| MI (SEI) | **70.4885 ✅** | 63.9238 | 64.8955 | +6.5647 | Higher | +5.5929 | Higher | -0.9718 | Lower |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **828.5343 ✅** | 910.9526 | 939.0055 | -82.4183 | Lower | -110.4712 | Lower | -28.0530 | Lower |
| Halstead Effort | **19884.8230 ✅** | 21243.4140 | 25822.6521 | -1358.5909 | Lower | -5937.8291 | Lower | -4579.2382 | Lower |
| SLOC | **27 ✅** | 29 | 41 | -2 | Lower | -14 | Lower | -12 | Lower |
| PLOC | **18 ✅** | 20 | 24 | -2 | Lower | -6 | Lower | -4 | Lower |
| LLOC | **15 ✅** | 17 | 23 | -2 | Lower | -8 | Lower | -6 | Lower |
| Comments (CLOC) | 4 | 3 | **9 ✅** | +1 | Higher | -5 | Lower | -6 | Lower |
| Blank Lines | **5 ✅** | 6 | 8 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02618

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.8183 | **44.6905 ✅** | 42.0397 | -3.8722 | Lower | -1.2214 | Lower | +2.6508 | Higher |
| MI (SEI) | 11.0607 | **18.1568 ✅** | 10.8183 | -7.0961 | Lower | +0.2424 | Higher | +7.3385 | Higher |
| Cyclomatic (sum) | 21 | **20 ✅** | 23 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | **16 ✅** | 21 | 17 | -5 | Lower | -1 | Lower | +4 | Higher |
| Halstead Volume | 4332.2657 | **3887.3487 ✅** | 4392.4135 | +444.9170 | Higher | -60.1478 | Lower | -505.0648 | Lower |
| Halstead Effort | 223565.8709 | **202859.7949 ✅** | 241875.5701 | +20706.0760 | Higher | -18309.6992 | Lower | -39015.7752 | Lower |
| SLOC | 156 | **129 ✅** | 140 | +27 | Higher | +16 | Higher | -11 | Lower |
| PLOC | 121 | **99 ✅** | 108 | +22 | Higher | +13 | Higher | -9 | Lower |
| LLOC | 68 | 74 | **66 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Comments (CLOC) | **19 ✅** | 18 | 14 | +1 | Higher | +5 | Higher | +4 | Higher |
| Blank Lines | 18 | **14 ✅** | 18 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |

## p02639

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.2230 | **82.8804 ✅** | 80.5399 | -13.6574 | Lower | -11.3169 | Lower | +2.3404 | Higher |
| MI (SEI) | 45.6785 | **69.7989 ✅** | 65.4883 | -24.1204 | Lower | -19.8099 | Lower | +4.3105 | Higher |
| Cyclomatic (sum) | 18 | 8 | **7 ✅** | +10 | Higher | +11 | Higher | +1 | Higher |
| Cognitive (sum) | 10 | 6 | **5 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Halstead Volume | 1082.6181 | **627.6685 ✅** | 816.8482 | +454.9496 | Higher | +265.7700 | Higher | -189.1796 | Lower |
| Halstead Effort | 27065.4536 | 12116.7318 | **11175.9679 ✅** | +14948.7218 | Higher | +15889.4857 | Higher | +940.7639 | Higher |
| SLOC | 44 | **26 ✅** | 28 | +18 | Higher | +16 | Higher | -2 | Lower |
| PLOC | 37 | **19 ✅** | 22 | +18 | Higher | +15 | Higher | -3 | Lower |
| LLOC | 14 | **11 ✅** | 14 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 5 | 2 | **0 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |

## p02644

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.1894 | **54.8302 ✅** | 52.5555 | -0.6408 | Lower | +1.6339 | Higher | +2.2747 | Higher |
| MI (SEI) | 29.3749 | **33.7508 ✅** | 21.3238 | -4.3759 | Lower | +8.0510 | Higher | +12.4269 | Higher |
| Cyclomatic (sum) | 14 | 14 | 16 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 26 | 26 | **20 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| Halstead Volume | 3104.6442 | **2958.6623 ✅** | 3249.5612 | +145.9819 | Higher | -144.9170 | Lower | -290.8989 | Lower |
| Halstead Effort | **122492.3258 ✅** | 123467.2557 | 174257.7196 | -974.9299 | Lower | -51765.3938 | Lower | -50790.4639 | Lower |
| SLOC | 84 | **82 ✅** | 89 | +2 | Higher | -5 | Lower | -7 | Lower |
| PLOC | 59 | **56 ✅** | 68 | +3 | Higher | -9 | Lower | -12 | Lower |
| LLOC | 47 | **45 ✅** | 58 | +2 | Higher | -11 | Lower | -13 | Lower |
| Comments (CLOC) | 10 | **13 ✅** | 6 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 15 | **13 ✅** | 15 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.1540 ✅** | 55.6730 | 56.7043 | +3.4810 | Higher | +2.4496 | Higher | -1.0313 | Lower |
| MI (SEI) | **38.8170 ✅** | 31.9588 | 35.8656 | +6.8582 | Higher | +2.9513 | Higher | -3.9068 | Lower |
| Cyclomatic (sum) | **12 ✅** | 17 | 15 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | **22 ✅** | 25 | 24 | -3 | Lower | -2 | Lower | +1 | Higher |
| Halstead Volume | **1858.3536 ✅** | 2121.6628 | 2131.9182 | -263.3092 | Lower | -273.5646 | Lower | -10.2554 | Lower |
| Halstead Effort | 77065.3596 | **75389.7499 ✅** | 109053.5382 | +1675.6097 | Higher | -31988.1786 | Lower | -33663.7883 | Lower |
| SLOC | **75 ✅** | 83 | 80 | -8 | Lower | -5 | Lower | +3 | Higher |
| PLOC | **54 ✅** | 64 | 57 | -10 | Lower | -3 | Lower | +7 | Higher |
| LLOC | **49 ✅** | 52 | 50 | -3 | Lower | -1 | Lower | +2 | Higher |
| Comments (CLOC) | 11 | 10 | **12 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.3521 ✅** | 53.6530 | 54.9190 | +11.6991 | Higher | +10.4331 | Higher | -1.2660 | Lower |
| MI (SEI) | **51.3737 ✅** | 33.3123 | 39.8247 | +18.0614 | Higher | +11.5490 | Higher | -6.5124 | Lower |
| Cyclomatic (sum) | **6 ✅** | 12 | 10 | -6 | Lower | -4 | Lower | +2 | Higher |
| Cognitive (sum) | **8 ✅** | 12 | 16 | -4 | Lower | -8 | Lower | -4 | Lower |
| Halstead Volume | **2047.3444 ✅** | 3033.0446 | 2597.5384 | -985.7002 | Lower | -550.1940 | Lower | +435.5062 | Higher |
| Halstead Effort | **101250.4864 ✅** | 194262.8075 | 107148.4601 | -93012.3210 | Lower | -5897.9736 | Lower | +87114.3474 | Higher |
| SLOC | **54 ✅** | 90 | 90 | -36 | Lower | -36 | Lower | 0 | Same |
| PLOC | **36 ✅** | 60 | 52 | -24 | Lower | -16 | Lower | +8 | Higher |
| LLOC | **34 ✅** | 56 | 46 | -22 | Lower | -12 | Lower | +10 | Higher |
| Comments (CLOC) | 11 | 16 | **23 ✅** | -5 | Lower | -12 | Lower | -7 | Lower |
| Blank Lines | **8 ✅** | 14 | 15 | -6 | Lower | -7 | Lower | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02678

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.0720 ✅** | 62.7583 | 55.6828 | +6.3137 | Higher | +13.3892 | Higher | +7.0755 | Higher |
| MI (SEI) | **52.8058 ✅** | 48.4268 | 33.3849 | +4.3790 | Higher | +19.4209 | Higher | +15.0419 | Higher |
| Cyclomatic (sum) | **7 ✅** | 10 | 11 | -3 | Lower | -4 | Lower | -1 | Lower |
| Cognitive (sum) | **8 ✅** | 13 | 10 | -5 | Lower | -2 | Lower | +3 | Higher |
| Halstead Volume | **1476.2112 ✅** | 1836.8222 | 2660.1955 | -360.6110 | Lower | -1183.9843 | Lower | -823.3733 | Lower |
| Halstead Effort | **39773.3478 ✅** | 66560.6354 | 115637.0686 | -26787.2877 | Lower | -75863.7209 | Lower | -49076.4332 | Lower |
| SLOC | **47 ✅** | 62 | 84 | -15 | Lower | -37 | Lower | -22 | Lower |
| PLOC | **33 ✅** | 43 | 62 | -10 | Lower | -29 | Lower | -19 | Lower |
| LLOC | **27 ✅** | 34 | 54 | -7 | Lower | -27 | Lower | -20 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 12 | -6 | Lower | -5 | Lower | +1 | Higher |
| Blank Lines | 8 | 8 | 14 | 0 | Same | -6 | Lower | -6 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p02703

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.5970 | **42.5472 ✅** | 40.3924 | -0.9502 | Lower | +1.2045 | Higher | +2.1547 | Higher |
| MI (SEI) | **19.3301 ✅** | 15.1066 | 9.2935 | +4.2235 | Higher | +10.0366 | Higher | +5.8130 | Higher |
| Cyclomatic (sum) | 24 | **22 ✅** | 27 | +2 | Higher | -3 | Lower | -5 | Lower |
| Cognitive (sum) | **25 ✅** | 43 | 31 | -18 | Lower | -6 | Lower | +12 | Higher |
| Halstead Volume | **4283.3434 ✅** | 4355.0798 | 4627.2909 | -71.7363 | Lower | -343.9475 | Lower | -272.2112 | Lower |
| Halstead Effort | 220300.1407 | **196869.4013 ✅** | 245756.7818 | +23430.7394 | Higher | -25456.6410 | Lower | -48887.3804 | Lower |
| SLOC | 143 | **138 ✅** | 144 | +5 | Higher | -1 | Lower | -6 | Lower |
| PLOC | **95 ✅** | 101 | 104 | -6 | Lower | -9 | Lower | -3 | Lower |
| LLOC | **73 ✅** | 74 | 77 | -1 | Lower | -4 | Lower | -3 | Lower |
| Comments (CLOC) | **30 ✅** | 19 | 15 | +11 | Higher | +15 | Higher | +4 | Higher |
| Blank Lines | 21 | **18 ✅** | 26 | +3 | Higher | -5 | Lower | -8 | Lower |
| Functions | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **1 ✅** | 5 | +3 | Higher | -1 | Lower | -4 | Lower |

## p02727

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.4085 | 45.9335 | **46.5332 ✅** | -2.5250 | Lower | -3.1247 | Lower | -0.5997 | Lower |
| MI (SEI) | 15.0279 | **19.1239 ✅** | 11.8704 | -4.0960 | Lower | +3.1575 | Higher | +7.2535 | Higher |
| Cyclomatic (sum) | 42 | 38 | **34 ✅** | +4 | Higher | +8 | Higher | +4 | Higher |
| Cognitive (sum) | 31 | **25 ✅** | 31 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 3366 | **3149.2215 ✅** | 3246.0563 | +216.7785 | Higher | +119.9437 | Higher | -96.8347 | Lower |
| Halstead Effort | 254223.9730 | **166989.4902 ✅** | 227181.2263 | +87234.4828 | Higher | +27042.7467 | Higher | -60191.7362 | Lower |
| SLOC | 107 | **99 ✅** | 100 | +8 | Higher | +7 | Higher | -1 | Lower |
| PLOC | 87 | **81 ✅** | 86 | +6 | Higher | +1 | Higher | -5 | Lower |
| LLOC | 50 | **44 ✅** | 50 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 11 | 11 | 5 | 0 | Same | +6 | Higher | +6 | Higher |
| Blank Lines | 9 | **8 ✅** | 9 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 11 | 11 | **3 ✅** | 0 | Same | +8 | Higher | +8 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02728

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 28.3202 | **40.1151 ✅** | 33.8142 | -11.7949 | Lower | -5.4940 | Lower | +6.3009 | Higher |
| MI (SEI) | 7.6724 | **14.7621 ✅** | -4.6427 | -7.0897 | Lower | +12.3151 | Higher | +19.4047 | Higher |
| Cyclomatic (sum) | 40 | **24 ✅** | 30 | +16 | Higher | +10 | Higher | -6 | Lower |
| Cognitive (sum) | 60 | 32 | **19 ✅** | +28 | Higher | +41 | Higher | +13 | Higher |
| Halstead Volume | 5847.5228 | **5011.1565 ✅** | 6578.3938 | +836.3663 | Higher | -730.8710 | Lower | -1567.2373 | Lower |
| Halstead Effort | 549619.9837 | **343264.2177 ✅** | 762401.2141 | +206355.7660 | Higher | -212781.2304 | Lower | -419136.9964 | Lower |
| SLOC | 234 | **149 ✅** | 185 | +85 | Higher | +49 | Higher | -36 | Lower |
| PLOC | 137 | **106 ✅** | 151 | +31 | Higher | -14 | Lower | -45 | Lower |
| LLOC | 111 | **92 ✅** | 117 | +19 | Higher | -6 | Lower | -25 | Lower |
| Comments (CLOC) | **75 ✅** | 26 | 12 | +49 | Higher | +63 | Higher | +14 | Higher |
| Blank Lines | 23 | **17 ✅** | 22 | +6 | Higher | +1 | Higher | -5 | Lower |
| Functions | 3 | 3 | 12 | 0 | Same | -9 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |

## p02731

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 95.0944 | **95.2389 ✅** | 94.1123 | -0.1445 | Lower | +0.9821 | Higher | +1.1266 | Higher |
| MI (SEI) | 92.7765 | 92.9849 | **94.4138 ✅** | -0.2084 | Lower | -1.6373 | Lower | -1.4289 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 354.6326 | **344.9167 ✅** | 354.6326 | +9.7160 | Higher | 0 | Same | -9.7160 | Lower |
| Halstead Effort | 3806.3900 | **3702.1054 ✅** | 3806.3900 | +104.2847 | Higher | 0 | Same | -104.2847 | Lower |
| SLOC | 16 | 16 | 17 | 0 | Same | -1 | Lower | -1 | Lower |
| PLOC | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 3 | 3 | **4 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02733

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.9010 ✅** | 47.5793 | 44.7006 | +1.3217 | Higher | +4.2004 | Higher | +2.8787 | Higher |
| MI (SEI) | **22.5853 ✅** | 19.3146 | 19.3349 | +3.2707 | Higher | +3.2505 | Higher | -0.0202 | Lower |
| Cyclomatic (sum) | **21 ✅** | 26 | 24 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | **52 ✅** | 65 | 67 | -13 | Lower | -15 | Lower | -2 | Lower |
| Halstead Volume | **2878.7142 ✅** | 2975.3343 | 3330.4612 | -96.6201 | Lower | -451.7469 | Lower | -355.1268 | Lower |
| Halstead Effort | **134594.5769 ✅** | 143265.1553 | 253991.4853 | -8670.5784 | Lower | -119396.9084 | Lower | -110726.3300 | Lower |
| SLOC | 108 | 108 | 128 | 0 | Same | -20 | Lower | -20 | Lower |
| PLOC | **80 ✅** | 85 | 94 | -5 | Lower | -14 | Lower | -9 | Lower |
| LLOC | 72 | **68 ✅** | 92 | +4 | Higher | -20 | Lower | -24 | Lower |
| Comments (CLOC) | 13 | 11 | **19 ✅** | +2 | Higher | -6 | Lower | -8 | Lower |
| Blank Lines | 16 | **12 ✅** | 15 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02734

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.0996 | **63.3614 ✅** | 52.4088 | -1.2617 | Lower | +9.6908 | Higher | +10.9525 | Higher |
| MI (SEI) | 42.2472 | **53.4622 ✅** | 29.5446 | -11.2150 | Lower | +12.7026 | Higher | +23.9176 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 17 | +1 | Higher | -8 | Lower | -9 | Lower |
| Cognitive (sum) | 12 | **9 ✅** | 25 | +3 | Higher | -13 | Lower | -16 | Lower |
| Halstead Volume | 1973.9264 | **1470.7134 ✅** | 3197.8685 | +503.2130 | Higher | -1223.9421 | Lower | -1727.1551 | Lower |
| Halstead Effort | 92281.0598 | **51229.8517 ✅** | 276820.6170 | +41051.2081 | Higher | -184539.5572 | Lower | -225590.7653 | Lower |
| SLOC | **64 ✅** | 66 | 89 | -2 | Lower | -25 | Lower | -23 | Lower |
| PLOC | 43 | **35 ✅** | 62 | +8 | Higher | -19 | Lower | -27 | Lower |
| LLOC | 43 | **30 ✅** | 57 | +13 | Higher | -14 | Lower | -27 | Lower |
| Comments (CLOC) | 9 | **19 ✅** | 13 | -10 | Lower | -4 | Lower | +6 | Higher |
| Blank Lines | 12 | 12 | 15 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p02743

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.3326 ✅** | 87.1954 | 86.7886 | +1.1373 | Higher | +1.5440 | Higher | +0.4068 | Higher |
| MI (SEI) | 75.6703 | 67.0857 | **76.9849 ✅** | +8.5846 | Higher | -1.3146 | Lower | -9.8992 | Lower |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 594.5414 | 635.5533 | 594.5414 | -41.0119 | Lower | 0 | Same | +41.0119 | Higher |
| Halstead Effort | 9043.2872 | 9723.9657 | 9043.2872 | -680.6784 | Lower | 0 | Same | +680.6784 | Higher |
| SLOC | **20 ✅** | 21 | 22 | -1 | Lower | -2 | Lower | -1 | Lower |
| PLOC | 15 | 16 | 15 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 2 | 1 | **3 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02761

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.3913 ✅** | 49.2728 | 55.2995 | +7.1185 | Higher | +1.0918 | Higher | -6.0267 | Lower |
| MI (SEI) | **41.1128 ✅** | 22.8107 | 32.9614 | +18.3022 | Higher | +8.1514 | Higher | -10.1508 | Lower |
| Cyclomatic (sum) | **19 ✅** | 28 | 22 | -9 | Lower | -3 | Lower | +6 | Higher |
| Cognitive (sum) | 27 | **26 ✅** | 29 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | **1897.0459 ✅** | 2499.2287 | 2404.5478 | -602.1829 | Lower | -507.5020 | Lower | +94.6809 | Higher |
| Halstead Effort | **75482.4564 ✅** | 113871.1098 | 121583.8027 | -38388.6534 | Lower | -46101.3463 | Lower | -7712.6929 | Lower |
| SLOC | 80 | 100 | **76 ✅** | -20 | Lower | +4 | Higher | +24 | Higher |
| PLOC | **53 ✅** | 78 | 58 | -25 | Lower | -5 | Lower | +20 | Higher |
| LLOC | **40 ✅** | 47 | 44 | -7 | Lower | -4 | Lower | +3 | Higher |
| Comments (CLOC) | **18 ✅** | 11 | 10 | +7 | Higher | +8 | Higher | +1 | Higher |
| Blank Lines | 12 | 11 | 11 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 7 | 1 | -7 | Lower | -1 | Lower | +6 | Higher |

## p02762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.6213 ✅** | 45.9790 | 48.3671 | +2.6424 | Higher | +0.2543 | Higher | -2.3881 | Lower |
| MI (SEI) | **20.8617 ✅** | 16.2978 | 13.8447 | +4.5639 | Higher | +7.0170 | Higher | +2.4531 | Higher |
| Cyclomatic (sum) | 19 | 21 | **18 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 17 | **15 ✅** | 19 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | **3134.3219 ✅** | 3636.5197 | 3749.7489 | -502.1978 | Lower | -615.4270 | Lower | -113.2291 | Lower |
| Halstead Effort | **165549.1840 ✅** | 193010.0009 | 221155.4004 | -27460.8169 | Lower | -55606.2163 | Lower | -28145.3995 | Lower |
| SLOC | 110 | 120 | **107 ✅** | -10 | Lower | +3 | Higher | +13 | Higher |
| PLOC | **84 ✅** | 94 | 85 | -10 | Lower | -1 | Lower | +9 | Higher |
| LLOC | **65 ✅** | 69 | 72 | -4 | Lower | -7 | Lower | -3 | Lower |
| Comments (CLOC) | 12 | 12 | 6 | 0 | Same | +6 | Higher | +6 | Higher |
| Blank Lines | 14 | 14 | 16 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 5 | **2 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p02783

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.3768 | 89.3417 | **89.8066 ✅** | -0.9648 | Lower | -1.4297 | Lower | -0.4649 | Lower |
| MI (SEI) | **85.7662 ✅** | 85.4329 | 82.4002 | +0.3334 | Higher | +3.3660 | Higher | +3.0326 | Higher |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | **457.8717 ✅** | 511.8226 | 468.0466 | -53.9509 | Lower | -10.1749 | Lower | +43.7760 | Higher |
| Halstead Effort | **6181.2674 ✅** | 7098.1710 | 6552.6521 | -916.9036 | Lower | -371.3848 | Lower | +545.5188 | Higher |
| SLOC | 22 | 20 | 20 | +2 | Higher | +2 | Higher | 0 | Same |
| PLOC | 13 | **12 ✅** | 13 | +1 | Higher | 0 | Same | -1 | Lower |
| LLOC | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Comments (CLOC) | **5 ✅** | 4 | 3 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02812

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.4679 | **78.7598 ✅** | 75.2386 | -0.2919 | Lower | +3.2293 | Higher | +3.5212 | Higher |
| MI (SEI) | 68.7941 | **73.0828 ✅** | 67.4278 | -4.2887 | Lower | +1.3663 | Higher | +5.6550 | Higher |
| Cyclomatic (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 762.2515 | **720.6450 ✅** | 806.4400 | +41.6066 | Higher | -44.1885 | Lower | -85.7951 | Lower |
| Halstead Effort | 15366.9912 | **13812.3620 ✅** | 17612.6506 | +1554.6293 | Higher | -2245.6593 | Lower | -3800.2886 | Lower |
| SLOC | 33 | 33 | 39 | 0 | Same | -6 | Lower | -6 | Lower |
| PLOC | 21 | **20 ✅** | 24 | +1 | Higher | -3 | Lower | -4 | Lower |
| LLOC | 18 | **17 ✅** | 18 | +1 | Higher | 0 | Same | -1 | Lower |
| Comments (CLOC) | 6 | 8 | **9 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | 7 | **6 ✅** | 8 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02831

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.0774 ✅** | 84.3866 | 78.4774 | +2.6908 | Higher | +8.6000 | Higher | +5.9091 | Higher |
| MI (SEI) | **85.8944 ✅** | 81.9263 | 71.5751 | +3.9681 | Higher | +14.3193 | Higher | +10.3511 | Higher |
| Cyclomatic (sum) | 4 | 4 | 9 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 3 | 3 | 6 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | **489.6886 ✅** | 498.5531 | 733.3552 | -8.8645 | Lower | -243.6666 | Lower | -234.8021 | Lower |
| Halstead Effort | **7494.9563 ✅** | 8032.2446 | 16133.8137 | -537.2883 | Lower | -8638.8574 | Lower | -8101.5691 | Lower |
| SLOC | **23 ✅** | 27 | 32 | -4 | Lower | -9 | Lower | -5 | Lower |
| PLOC | 15 | 15 | 23 | 0 | Same | -8 | Lower | -8 | Lower |
| LLOC | 10 | 10 | 13 | 0 | Same | -3 | Lower | -3 | Lower |
| Comments (CLOC) | 6 | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p02837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.4721 | **60.6863 ✅** | 58.8947 | -2.2143 | Lower | -0.4226 | Lower | +1.7916 | Higher |
| MI (SEI) | 38.5742 | 35.7369 | **39.2534 ✅** | +2.8373 | Higher | -0.6793 | Lower | -3.5166 | Lower |
| Cyclomatic (sum) | 16 | 15 | 15 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 26 | 26 | **24 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1931.1400 | **1722.3972 ✅** | 1942.6160 | +208.7428 | Higher | -11.4760 | Lower | -220.2188 | Lower |
| Halstead Effort | 74302.9103 | **55683.2882 ✅** | 86252.1493 | +18619.6221 | Higher | -11949.2389 | Lower | -30568.8610 | Lower |
| SLOC | 73 | **67 ✅** | 72 | +6 | Higher | +1 | Higher | -5 | Lower |
| PLOC | 56 | **52 ✅** | 54 | +4 | Higher | +2 | Higher | -2 | Lower |
| LLOC | 41 | **38 ✅** | 40 | +3 | Higher | +1 | Higher | -2 | Lower |
| Comments (CLOC) | 11 | 6 | 11 | +5 | Higher | 0 | Same | -5 | Lower |
| Blank Lines | 10 | 10 | **8 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p02840

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.7989 | **47.8713 ✅** | 42.5529 | -5.0724 | Lower | +0.2459 | Higher | +5.3184 | Higher |
| MI (SEI) | 14.8395 | **30.4036 ✅** | 9.2105 | -15.5641 | Lower | +5.6290 | Higher | +21.1931 | Higher |
| Cyclomatic (sum) | 23 | **16 ✅** | 23 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 35 | **21 ✅** | 32 | +14 | Higher | +3 | Higher | -11 | Lower |
| Halstead Volume | 3881.4639 | **2846.5701 ✅** | 4162.0325 | +1034.8938 | Higher | -280.5686 | Lower | -1315.4624 | Lower |
| Halstead Effort | 279166.8275 | **136514.2362 ✅** | 324789.8838 | +142652.5914 | Higher | -45623.0563 | Lower | -188275.6477 | Lower |
| SLOC | 139 | **124 ✅** | 138 | +15 | Higher | +1 | Higher | -14 | Lower |
| PLOC | 106 | **72 ✅** | 109 | +34 | Higher | -3 | Lower | -37 | Lower |
| LLOC | 84 | **58 ✅** | 84 | +26 | Higher | 0 | Same | -26 | Lower |
| Comments (CLOC) | 18 | **32 ✅** | 11 | -14 | Lower | +7 | Higher | +21 | Higher |
| Blank Lines | **15 ✅** | 20 | 18 | -5 | Lower | -3 | Lower | +2 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **2 ✅** | 5 | 3 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.0153 ✅** | 66.1734 | 63.2054 | +3.8419 | Higher | +6.8099 | Higher | +2.9680 | Higher |
| MI (SEI) | 48.3042 | **53.2396 ✅** | 39.8051 | -4.9354 | Lower | +8.4990 | Higher | +13.4344 | Higher |
| Cyclomatic (sum) | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 7 | **5 ✅** | 7 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | **1409.9474 ✅** | 1560.9671 | 2347.8442 | -151.0197 | Lower | -937.8967 | Lower | -786.8771 | Lower |
| Halstead Effort | 43412.7364 | **40493.3236 ✅** | 112696.5204 | +2919.4128 | Higher | -69283.7840 | Lower | -72203.1968 | Lower |
| SLOC | **45 ✅** | 56 | 59 | -11 | Lower | -14 | Lower | -3 | Lower |
| PLOC | 35 | **34 ✅** | 45 | +1 | Higher | -10 | Lower | -11 | Lower |
| LLOC | **26 ✅** | 28 | 48 | -2 | Lower | -22 | Lower | -20 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 6 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | **6 ✅** | 10 | 8 | -4 | Lower | -2 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02862

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.7363 ✅** | 59.0232 | 57.4981 | +0.7130 | Higher | +2.2382 | Higher | +1.5251 | Higher |
| MI (SEI) | 30.0780 | **31.1294 ✅** | 24.3278 | -1.0514 | Lower | +5.7502 | Higher | +6.8016 | Higher |
| Cyclomatic (sum) | **11 ✅** | 14 | 15 | -3 | Lower | -4 | Lower | -1 | Lower |
| Cognitive (sum) | **6 ✅** | 10 | 9 | -4 | Lower | -3 | Lower | +1 | Higher |
| Halstead Volume | 2467.8439 | **2261.7498 ✅** | 2541.1462 | +206.0941 | Higher | -73.3023 | Lower | -279.3964 | Lower |
| Halstead Effort | 118192.0944 | **118031.7782 ✅** | 132759.3928 | +160.3163 | Higher | -14567.2984 | Lower | -14727.6146 | Lower |
| SLOC | **67 ✅** | 69 | 72 | -2 | Lower | -5 | Lower | -3 | Lower |
| PLOC | **53 ✅** | 56 | 58 | -3 | Lower | -5 | Lower | -2 | Lower |
| LLOC | 46 | **44 ✅** | 45 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | 4 | **5 ✅** | 3 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | **10 ✅** | 11 | 12 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 5 | 7 | 0 | Same | -2 | Lower | -2 | Lower |

## p02884

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.1186 ✅** | 52.4119 | 46.9336 | +0.7067 | Higher | +6.1850 | Higher | +5.4783 | Higher |
| MI (SEI) | 29.0744 | **32.8616 ✅** | 19.9408 | -3.7872 | Lower | +9.1336 | Higher | +12.9208 | Higher |
| Cyclomatic (sum) | 17 | **14 ✅** | 27 | +3 | Higher | -10 | Lower | -13 | Lower |
| Cognitive (sum) | 33 | **21 ✅** | 51 | +12 | Higher | -18 | Lower | -30 | Lower |
| Halstead Volume | **2432.7529 ✅** | 2703.3073 | 2877.7142 | -270.5544 | Lower | -444.9613 | Lower | -174.4069 | Lower |
| Halstead Effort | 142072.7710 | **102560.1692 ✅** | 205230.1567 | +39512.6018 | Higher | -63157.3856 | Lower | -102669.9874 | Lower |
| SLOC | **93 ✅** | 98 | 112 | -5 | Lower | -19 | Lower | -14 | Lower |
| PLOC | 64 | **62 ✅** | 81 | +2 | Higher | -17 | Lower | -19 | Lower |
| LLOC | 61 | **47 ✅** | 72 | +14 | Higher | -11 | Lower | -25 | Lower |
| Comments (CLOC) | 12 | **19 ✅** | 13 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 17 | 17 | 18 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |

## p02885

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 89.7849 | **90.6159 ✅** | 90.2482 | -0.8310 | Lower | -0.4632 | Lower | +0.3677 | Higher |
| MI (SEI) | **89.1083 ✅** | 87.9584 | 87.4279 | +1.1499 | Higher | +1.6804 | Higher | +0.5305 | Higher |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 470 | 470 | 504.4394 | 0 | Same | -34.4394 | Lower | -34.4394 | Lower |
| Halstead Effort | 5117.7778 | 5117.7778 | 6132.9213 | 0 | Same | -1015.1435 | Lower | -1015.1435 | Lower |
| SLOC | 20 | 19 | 19 | +1 | Higher | +1 | Higher | 0 | Same |
| PLOC | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | **5 ✅** | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02923

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.6758 ✅** | 73.1192 | 51.7208 | +2.5566 | Higher | +23.9550 | Higher | +21.3984 | Higher |
| MI (SEI) | **62.9591 ✅** | 59.7578 | 20.6296 | +3.2013 | Higher | +42.3295 | Higher | +39.1282 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 24 | -1 | Lower | -18 | Lower | -17 | Lower |
| Cognitive (sum) | **7 ✅** | 9 | 21 | -2 | Lower | -14 | Lower | -12 | Lower |
| Halstead Volume | **840.2535 ✅** | 894.2888 | 2415.4774 | -54.0353 | Lower | -1575.2238 | Lower | -1521.1886 | Lower |
| Halstead Effort | **18748.1567 ✅** | 23139.7228 | 126985.0955 | -4391.5661 | Lower | -108236.9387 | Lower | -103845.3726 | Lower |
| SLOC | **38 ✅** | 43 | 92 | -5 | Lower | -54 | Lower | -49 | Lower |
| PLOC | **26 ✅** | 29 | 72 | -3 | Lower | -46 | Lower | -43 | Lower |
| LLOC | **21 ✅** | 23 | 48 | -2 | Lower | -27 | Lower | -25 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | **6 ✅** | 8 | 14 | -2 | Lower | -8 | Lower | -6 | Lower |
| Functions | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 6 | -1 | Lower | -6 | Lower | -5 | Lower |

## p02945

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.9495 | **89.4935 ✅** | 78.2494 | -1.5440 | Lower | +9.7001 | Higher | +11.2441 | Higher |
| MI (SEI) | **82.0657 ✅** | 81.8467 | 68.5805 | +0.2190 | Higher | +13.4852 | Higher | +13.2662 | Higher |
| Cyclomatic (sum) | 2 | 2 | 7 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | 519.5726 | 519.5726 | 760.5802 | 0 | Same | -241.0076 | Lower | -241.0076 | Lower |
| Halstead Effort | 6927.6346 | 6927.6346 | 14197.4969 | 0 | Same | -7269.8623 | Lower | -7269.8623 | Lower |
| SLOC | 22 | **20 ✅** | 33 | +2 | Higher | -11 | Lower | -13 | Lower |
| PLOC | 13 | 13 | 22 | 0 | Same | -9 | Lower | -9 | Lower |
| LLOC | 10 | 10 | 19 | 0 | Same | -9 | Lower | -9 | Lower |
| Comments (CLOC) | 4 | 3 | **6 ✅** | +1 | Higher | -2 | Lower | -3 | Lower |
| Blank Lines | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.2454 | 80.0546 | **82.7451 ✅** | +0.1908 | Higher | -2.4998 | Lower | -2.6905 | Lower |
| MI (SEI) | **77.0873 ✅** | 75.2740 | 74.5266 | +1.8133 | Higher | +2.5607 | Higher | +0.7474 | Higher |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 3 | 6 | 3 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 651.1817 | 646.2897 | **610.3717 ✅** | +4.8920 | Higher | +40.8100 | Higher | +35.9180 | Higher |
| Halstead Effort | **9922.7683 ✅** | 10612.7565 | 10053.1808 | -689.9882 | Lower | -130.4124 | Lower | +559.5758 | Higher |
| SLOC | 32 | 32 | **28 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| PLOC | 19 | 19 | **18 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | **10 ✅** | 11 | 16 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | **9 ✅** | 8 | 5 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02953

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.9840 ✅** | 76.9917 | 70.1854 | +1.9923 | Higher | +8.7986 | Higher | +6.8063 | Higher |
| MI (SEI) | **67.6005 ✅** | 66.0044 | 58.9535 | +1.5961 | Higher | +8.6469 | Higher | +7.0509 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 9 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | **6 ✅** | 10 | 10 | -4 | Lower | -4 | Lower | 0 | Same |
| Halstead Volume | **759.6771 ✅** | 806.4400 | 1021.5380 | -46.7630 | Lower | -261.8609 | Lower | -215.0979 | Lower |
| Halstead Effort | **17040.9378 ✅** | 19757.7811 | 32178.4461 | -2716.8433 | Lower | -15137.5083 | Lower | -12420.6649 | Lower |
| SLOC | **32 ✅** | 35 | 48 | -3 | Lower | -16 | Lower | -13 | Lower |
| PLOC | **22 ✅** | 24 | 31 | -2 | Lower | -9 | Lower | -7 | Lower |
| LLOC | **16 ✅** | 17 | 22 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | 6 | **10 ✅** | -1 | Lower | -5 | Lower | -4 | Lower |
| Blank Lines | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p02956

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 30.3392 | **35.3139 ✅** | 27.4197 | -4.9747 | Lower | +2.9195 | Higher | +7.8942 | Higher |
| MI (SEI) | -0.2891 | **0.7787 ✅** | -9.7881 | -1.0678 | Lower | +9.4990 | Higher | +10.5668 | Higher |
| Cyclomatic (sum) | 36 | 36 | 40 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 34 | 34 | 44 | 0 | Same | -10 | Lower | -10 | Lower |
| Halstead Volume | 6533.8258 | **5503.4386 ✅** | 7856.6895 | +1030.3872 | Higher | -1322.8637 | Lower | -2353.2510 | Lower |
| Halstead Effort | 726859.4652 | **493780.7402 ✅** | 1017441.2967 | +233078.7250 | Higher | -290581.8315 | Lower | -523660.5565 | Lower |
| SLOC | 211 | **164 ✅** | 225 | +47 | Higher | -14 | Lower | -61 | Lower |
| PLOC | 161 | **136 ✅** | 180 | +25 | Higher | -19 | Lower | -44 | Lower |
| LLOC | 157 | **105 ✅** | 185 | +52 | Higher | -28 | Lower | -80 | Lower |
| Comments (CLOC) | **31 ✅** | 14 | 20 | +17 | Higher | +11 | Higher | -6 | Lower |
| Blank Lines | 22 | **14 ✅** | 27 | +8 | Higher | -5 | Lower | -13 | Lower |
| Functions | 5 | 6 | **4 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 3 | 2 | **0 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Exits | **2 ✅** | 5 | 3 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02957

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.9044 ✅** | 86.9223 | 82.8303 | +1.9821 | Higher | +6.0741 | Higher | +4.0920 | Higher |
| MI (SEI) | **80.4990 ✅** | 76.5452 | 80.4443 | +3.9538 | Higher | +0.0547 | Higher | -3.8991 | Lower |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **478.2215 ✅** | 527.3324 | 562.6210 | -49.1108 | Lower | -84.3995 | Lower | -35.2886 | Lower |
| Halstead Effort | **6163.7439 ✅** | 6538.9211 | 7890.7590 | -375.1773 | Lower | -1727.0151 | Lower | -1351.8379 | Lower |
| SLOC | **21 ✅** | 23 | 29 | -2 | Lower | -8 | Lower | -6 | Lower |
| PLOC | **14 ✅** | 18 | 16 | -4 | Lower | -2 | Lower | +2 | Higher |
| LLOC | 9 | 9 | 12 | 0 | Same | -3 | Lower | -3 | Lower |
| Comments (CLOC) | 3 | 3 | **8 ✅** | 0 | Same | -5 | Lower | -5 | Lower |
| Blank Lines | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02959

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.3374 ✅** | 68.7053 | 69.3251 | +1.6321 | Higher | +1.0124 | Higher | -0.6198 | Lower |
| MI (SEI) | 54.0375 | **59.1041 ✅** | 50.6361 | -5.0666 | Lower | +3.4013 | Higher | +8.4679 | Higher |
| Cyclomatic (sum) | 9 | **7 ✅** | 11 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 15 | **7 ✅** | 16 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 1397.6145 | **1156.0872 ✅** | 1554.2445 | +241.5273 | Higher | -156.6300 | Lower | -398.1573 | Lower |
| Halstead Effort | 58816.2766 | **42775.2278 ✅** | 73222.1874 | +16041.0488 | Higher | -14405.9108 | Lower | -30446.9596 | Lower |
| SLOC | 43 | 52 | 43 | -9 | Lower | 0 | Same | +9 | Higher |
| PLOC | **27 ✅** | 31 | 30 | -4 | Lower | -3 | Lower | +1 | Higher |
| LLOC | 29 | 29 | 29 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 6 | **13 ✅** | 5 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02988

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.6798 | **76.3165 ✅** | 65.0953 | -3.6367 | Lower | +7.5844 | Higher | +11.2212 | Higher |
| MI (SEI) | 58.0996 | **60.6527 ✅** | 54.1241 | -2.5530 | Lower | +3.9755 | Higher | +6.5285 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 14 | +1 | Higher | -4 | Lower | -5 | Lower |
| Cognitive (sum) | 18 | **10 ✅** | 22 | +8 | Higher | -4 | Lower | -12 | Lower |
| Halstead Volume | 988.5336 | **919.9486 ✅** | 1208.5912 | +68.5850 | Higher | -220.0577 | Lower | -288.6427 | Lower |
| Halstead Effort | 31676.0536 | **24201.7236 ✅** | 47692.8688 | +7474.3299 | Higher | -16016.8152 | Lower | -23491.1451 | Lower |
| SLOC | 41 | **34 ✅** | 58 | +7 | Higher | -17 | Lower | -24 | Lower |
| PLOC | 27 | **23 ✅** | 38 | +4 | Higher | -11 | Lower | -15 | Lower |
| LLOC | 20 | **18 ✅** | 24 | +2 | Higher | -4 | Lower | -6 | Lower |
| Comments (CLOC) | 6 | 4 | **14 ✅** | +2 | Higher | -8 | Lower | -10 | Lower |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03003

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.8914 ✅** | 63.8352 | 66.0994 | +4.0561 | Higher | +1.7920 | Higher | -2.2642 | Lower |
| MI (SEI) | 44.8866 | **50.0047 ✅** | 44.1958 | -5.1181 | Lower | +0.6908 | Higher | +5.8089 | Higher |
| Cyclomatic (sum) | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 16 | **14 ✅** | 17 | +2 | Higher | -1 | Lower | -3 | Lower |
| Halstead Volume | **1519.2903 ✅** | 1682.4359 | 1625.0220 | -163.1456 | Lower | -105.7317 | Lower | +57.4139 | Higher |
| Halstead Effort | 80171.7805 | **75946.2086 ✅** | 85638.6610 | +4225.5719 | Higher | -5466.8805 | Lower | -9692.4524 | Lower |
| SLOC | **48 ✅** | 58 | 51 | -10 | Lower | -3 | Lower | +7 | Higher |
| PLOC | **36 ✅** | 40 | 39 | -4 | Lower | -3 | Lower | +1 | Higher |
| LLOC | 32 | **30 ✅** | 32 | +2 | Higher | 0 | Same | -2 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 5 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03007

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.8889 | **55.8678 ✅** | 44.2729 | -9.9788 | Lower | +1.6160 | Higher | +11.5948 | Higher |
| MI (SEI) | 14.4042 | **43.4607 ✅** | 7.1517 | -29.0565 | Lower | +7.2525 | Higher | +36.3090 | Higher |
| Cyclomatic (sum) | 19 | **13 ✅** | 21 | +6 | Higher | -2 | Lower | -8 | Lower |
| Cognitive (sum) | 36 | **16 ✅** | 37 | +20 | Higher | -1 | Lower | -21 | Lower |
| Halstead Volume | 4492.5476 | **2264.8176 ✅** | 4795.1917 | +2227.7300 | Higher | -302.6441 | Lower | -2530.3741 | Lower |
| Halstead Effort | 547665.8337 | **119069.4533 ✅** | 571734.3948 | +428596.3803 | Higher | -24068.5612 | Lower | -452664.9415 | Lower |
| SLOC | 116 | **85 ✅** | 122 | +31 | Higher | -6 | Lower | -37 | Lower |
| PLOC | 96 | **53 ✅** | 105 | +43 | Higher | -9 | Lower | -52 | Lower |
| LLOC | 79 | **49 ✅** | 78 | +30 | Higher | +1 | Higher | -29 | Lower |
| Comments (CLOC) | 10 | **25 ✅** | 6 | -15 | Lower | +4 | Higher | +19 | Higher |
| Blank Lines | 10 | **9 ✅** | 12 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 5 | **0 ✅** | 6 | +5 | Higher | -1 | Lower | -6 | Lower |

## p03029

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **95.3225 ✅** | 94.2770 | 93.3304 | +1.0455 | Higher | +1.9922 | Higher | +0.9466 | Higher |
| MI (SEI) | 81.4951 | 86.5530 | **89.4409 ✅** | -5.0579 | Lower | -7.9458 | Lower | -2.8879 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 415 | 415 | **412.1788 ✅** | 0 | Same | +2.8212 | Higher | +2.8212 | Higher |
| Halstead Effort | 4394.1176 | 4394.1176 | **3782.3467 ✅** | 0 | Same | +611.7709 | Higher | +611.7709 | Higher |
| SLOC | **15 ✅** | 16 | 17 | -1 | Lower | -2 | Lower | -1 | Lower |
| PLOC | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 1 | 2 | **3 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03034

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.7312 ✅** | 59.1751 | 64.2368 | +5.5561 | Higher | +0.4945 | Higher | -5.0616 | Lower |
| MI (SEI) | **48.4648 ✅** | 44.7437 | 45.1960 | +3.7211 | Higher | +3.2688 | Higher | -0.4523 | Lower |
| Cyclomatic (sum) | 12 | 16 | 12 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | **10 ✅** | 23 | 11 | -13 | Lower | -1 | Lower | +12 | Higher |
| Halstead Volume | **1274.1980 ✅** | 1488 | 1476.6407 | -213.8020 | Lower | -202.4427 | Lower | +11.3593 | Higher |
| Halstead Effort | **44198.7435 ✅** | 60805.0909 | 58834.9023 | -16606.3474 | Lower | -14636.1588 | Lower | +1970.1886 | Higher |
| SLOC | 60 | 76 | **59 ✅** | -16 | Lower | +1 | Higher | +17 | Higher |
| PLOC | **39 ✅** | 50 | 42 | -11 | Lower | -3 | Lower | +8 | Higher |
| LLOC | **31 ✅** | 36 | 35 | -5 | Lower | -4 | Lower | +1 | Higher |
| Comments (CLOC) | 10 | **17 ✅** | 8 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03043

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.2141 | **76.7989 ✅** | 63.3209 | -0.5849 | Lower | +12.8931 | Higher | +13.4780 | Higher |
| MI (SEI) | **73.6526 ✅** | 63.2381 | 48.6056 | +10.4145 | Higher | +25.0470 | Higher | +14.6325 | Higher |
| Cyclomatic (sum) | **4 ✅** | 5 | 10 | -1 | Lower | -6 | Lower | -5 | Lower |
| Cognitive (sum) | **3 ✅** | 4 | 10 | -1 | Lower | -7 | Lower | -6 | Lower |
| Halstead Volume | **763.3676 ✅** | 914.3047 | 1422.8020 | -150.9371 | Lower | -659.4344 | Lower | -508.4973 | Lower |
| Halstead Effort | **13435.2694 ✅** | 19471.3039 | 52987.1073 | -6036.0344 | Lower | -39551.8379 | Lower | -33515.8035 | Lower |
| SLOC | 39 | **35 ✅** | 65 | +4 | Higher | -26 | Lower | -30 | Lower |
| PLOC | **21 ✅** | 23 | 41 | -2 | Lower | -20 | Lower | -18 | Lower |
| LLOC | **18 ✅** | 20 | 36 | -2 | Lower | -18 | Lower | -16 | Lower |
| Comments (CLOC) | 13 | 5 | 13 | +8 | Higher | 0 | Same | -8 | Lower |
| Blank Lines | **5 ✅** | 8 | 12 | -3 | Lower | -7 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |

## p03051

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.3605 | **61.3120 ✅** | 53.7480 | -3.9515 | Lower | +3.6125 | Higher | +7.5640 | Higher |
| MI (SEI) | 31.0654 | **46.6333 ✅** | 32.9657 | -15.5679 | Lower | -1.9003 | Lower | +13.6677 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 14 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 13 | 13 | 15 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 2623.6965 | **1662.1084 ✅** | 2822.5600 | +961.5880 | Higher | -198.8635 | Lower | -1160.4516 | Lower |
| Halstead Effort | 168814.1554 | **67984.8521 ✅** | 179199.7395 | +100829.3033 | Higher | -10385.5841 | Lower | -111214.8874 | Lower |
| SLOC | 75 | **70 ✅** | 89 | +5 | Higher | -14 | Lower | -19 | Lower |
| PLOC | 57 | **47 ✅** | 60 | +10 | Higher | -3 | Lower | -13 | Lower |
| LLOC | 48 | **37 ✅** | 51 | +11 | Higher | -3 | Lower | -14 | Lower |
| Comments (CLOC) | 7 | 15 | 15 | -8 | Lower | -8 | Lower | 0 | Same |
| Blank Lines | 11 | **9 ✅** | 15 | +2 | Higher | -4 | Lower | -6 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |

## p03053

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.0589 ✅** | 55.1722 | 54.3252 | +3.8866 | Higher | +4.7337 | Higher | +0.8471 | Higher |
| MI (SEI) | 35.0344 | **35.5156 ✅** | 21.4827 | -0.4813 | Lower | +13.5517 | Higher | +14.0329 | Higher |
| Cyclomatic (sum) | **15 ✅** | 16 | 18 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | 20 | 30 | 20 | -10 | Lower | 0 | Same | +10 | Higher |
| Halstead Volume | 2355.3531 | **2108.7599 ✅** | 2950.1694 | +246.5932 | Higher | -594.8162 | Lower | -841.4094 | Lower |
| Halstead Effort | **95260.9495 ✅** | 103931.7400 | 164809.9837 | -8670.7905 | Lower | -69549.0343 | Lower | -60878.2437 | Lower |
| SLOC | **67 ✅** | 87 | 80 | -20 | Lower | -13 | Lower | +7 | Higher |
| PLOC | **49 ✅** | 64 | 64 | -15 | Lower | -15 | Lower | 0 | Same |
| LLOC | **38 ✅** | 51 | 51 | -13 | Lower | -13 | Lower | 0 | Same |
| Comments (CLOC) | 7 | **15 ✅** | 4 | -8 | Lower | +3 | Higher | +11 | Higher |
| Blank Lines | 11 | **9 ✅** | 12 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03055

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.4835 | **59.4978 ✅** | 28.4617 | -13.0143 | Lower | +18.0218 | Higher | +31.0361 | Higher |
| MI (SEI) | 23.9294 | **35.9439 ✅** | -12.4446 | -12.0145 | Lower | +36.3740 | Higher | +48.3885 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 37 | +5 | Higher | -21 | Lower | -26 | Lower |
| Cognitive (sum) | 28 | **12 ✅** | 64 | +16 | Higher | -36 | Lower | -52 | Lower |
| Halstead Volume | 3717.2937 | **2064.6813 ✅** | 7987.5595 | +1652.6125 | Higher | -4270.2658 | Lower | -5922.8782 | Lower |
| Halstead Effort | 218176.5468 | **68627.1898 ✅** | 983059.0663 | +149549.3570 | Higher | -764882.5196 | Lower | -914431.8765 | Lower |
| SLOC | 124 | **72 ✅** | 219 | +52 | Higher | -95 | Lower | -147 | Lower |
| PLOC | 85 | **52 ✅** | 186 | +33 | Higher | -101 | Lower | -134 | Lower |
| LLOC | 81 | **40 ✅** | 173 | +41 | Higher | -92 | Lower | -133 | Lower |
| Comments (CLOC) | **23 ✅** | 8 | 13 | +15 | Higher | +10 | Higher | -5 | Lower |
| Blank Lines | 18 | **12 ✅** | 22 | +6 | Higher | -4 | Lower | -10 | Lower |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | **0 ✅** | 1 | 4 | -1 | Lower | -4 | Lower | -3 | Lower |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.9974 | 52.5958 | **62.6573 ✅** | +6.4016 | Higher | -3.6599 | Lower | -10.0615 | Lower |
| MI (SEI) | 50.6214 | 15.8911 | **52.0323 ✅** | +34.7302 | Higher | -1.4110 | Lower | -36.1412 | Lower |
| Cyclomatic (sum) | 9 | 18 | **6 ✅** | -9 | Lower | +3 | Higher | +12 | Higher |
| Cognitive (sum) | 7 | 17 | **6 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| Halstead Volume | **1656.1511 ✅** | 2754.0170 | 1755.4983 | -1097.8659 | Lower | -99.3471 | Lower | +998.5188 | Higher |
| Halstead Effort | 74526.8017 | 158608.8994 | **66440.7325 ✅** | -84082.0977 | Lower | +8086.0692 | Higher | +92168.1670 | Higher |
| SLOC | 82 | 91 | **67 ✅** | -9 | Lower | +15 | Higher | +24 | Higher |
| PLOC | 39 | 72 | 39 | -33 | Lower | 0 | Same | +33 | Higher |
| LLOC | **36 ✅** | 59 | 43 | -23 | Lower | -7 | Lower | +16 | Higher |
| Comments (CLOC) | **30 ✅** | 3 | 19 | +27 | Higher | +11 | Higher | -16 | Lower |
| Blank Lines | 13 | 16 | **10 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.8790 ✅** | 52.0431 | 53.0592 | +8.8359 | Higher | +7.8198 | Higher | -1.0161 | Lower |
| MI (SEI) | **45.3766 ✅** | 31.6500 | 33.2251 | +13.7265 | Higher | +12.1515 | Higher | -1.5751 | Lower |
| Cyclomatic (sum) | **11 ✅** | 15 | 16 | -4 | Lower | -5 | Lower | -1 | Lower |
| Cognitive (sum) | **16 ✅** | 19 | 25 | -3 | Lower | -9 | Lower | -6 | Lower |
| Halstead Volume | **1807.5085 ✅** | 2307.2301 | 2572.0218 | -499.7215 | Lower | -764.5133 | Lower | -264.7918 | Lower |
| Halstead Effort | **61235.4576 ✅** | 107771.9300 | 125589.9433 | -46536.4723 | Lower | -64354.4857 | Lower | -17818.0134 | Lower |
| SLOC | **69 ✅** | 104 | 93 | -35 | Lower | -24 | Lower | +11 | Higher |
| PLOC | **46 ✅** | 69 | 63 | -23 | Lower | -17 | Lower | +6 | Higher |
| LLOC | **40 ✅** | 69 | 57 | -29 | Lower | -17 | Lower | +12 | Higher |
| Comments (CLOC) | 14 | **19 ✅** | 17 | -5 | Lower | -3 | Lower | +2 | Higher |
| Blank Lines | **9 ✅** | 16 | 13 | -7 | Lower | -4 | Lower | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.1718 ✅** | 69.3800 | 67.9923 | +3.7918 | Higher | +5.1795 | Higher | +1.3877 | Higher |
| MI (SEI) | **57.0210 ✅** | 53.2477 | 49.2782 | +3.7733 | Higher | +7.7428 | Higher | +3.9695 | Higher |
| Cyclomatic (sum) | **11 ✅** | 12 | 13 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **9 ✅** | 10 | 15 | -1 | Lower | -6 | Lower | -5 | Lower |
| Halstead Volume | **929.1717 ✅** | 979.4744 | 1149.0793 | -50.3027 | Lower | -219.9077 | Lower | -169.6049 | Lower |
| Halstead Effort | **23892.9859 ✅** | 24780.7023 | 31868.9971 | -887.7164 | Lower | -7976.0112 | Lower | -7088.2949 | Lower |
| SLOC | **40 ✅** | 49 | 50 | -9 | Lower | -10 | Lower | -1 | Lower |
| PLOC | **32 ✅** | 35 | 35 | -3 | Lower | -3 | Lower | 0 | Same |
| LLOC | 18 | 18 | 20 | 0 | Same | -2 | Lower | -2 | Lower |
| Comments (CLOC) | 5 | **7 ✅** | 6 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | **5 ✅** | 7 | 9 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.0902 ✅** | 76.4472 | 86.8951 | +13.6430 | Higher | +3.1951 | Higher | -10.4478 | Lower |
| MI (SEI) | **87.2000 ✅** | 66.5915 | 80.0699 | +20.6085 | Higher | +7.1301 | Higher | -13.4784 | Lower |
| Cyclomatic (sum) | 3 | 13 | 3 | -10 | Lower | 0 | Same | +10 | Higher |
| Cognitive (sum) | 2 | 6 | 2 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | **520 ✅** | 824.8989 | 530.0990 | -304.8989 | Lower | -10.0990 | Lower | +294.7998 | Higher |
| Halstead Effort | **6652.9412 ✅** | 17538.9209 | 7454.5173 | -10885.9797 | Lower | -801.5761 | Lower | +10084.4036 | Higher |
| SLOC | **19 ✅** | 33 | 23 | -14 | Lower | -4 | Lower | +10 | Higher |
| PLOC | **12 ✅** | 23 | 15 | -11 | Lower | -3 | Lower | +8 | Higher |
| LLOC | **9 ✅** | 10 | 10 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 4 | **6 ✅** | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

## p03106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.5845 ✅** | 76.8730 | 55.4842 | +0.7115 | Higher | +22.1003 | Higher | +21.3888 | Higher |
| MI (SEI) | **69.2656 ✅** | 67.8516 | 31.5790 | +1.4140 | Higher | +37.6866 | Higher | +36.2727 | Higher |
| Cyclomatic (sum) | 7 | 7 | 25 | 0 | Same | -18 | Lower | -18 | Lower |
| Cognitive (sum) | **8 ✅** | 9 | 45 | -1 | Lower | -37 | Lower | -36 | Lower |
| Halstead Volume | **787.5601 ✅** | 825.0630 | 1666.3767 | -37.5029 | Lower | -878.8165 | Lower | -841.3136 | Lower |
| Halstead Effort | **16876.2888 ✅** | 19251.4702 | 93896.7020 | -2375.1814 | Lower | -77020.4132 | Lower | -74645.2318 | Lower |
| SLOC | **34 ✅** | 35 | 81 | -1 | Lower | -47 | Lower | -46 | Lower |
| PLOC | **22 ✅** | 24 | 69 | -2 | Lower | -47 | Lower | -45 | Lower |
| LLOC | **18 ✅** | 21 | 48 | -3 | Lower | -30 | Lower | -27 | Lower |
| Comments (CLOC) | 7 | 7 | **9 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | 6 | 5 | **4 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p03108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **49.5379 ✅** | 49.2322 | 47.3057 | +0.3057 | Higher | +2.2322 | Higher | +1.9265 | Higher |
| MI (SEI) | **26.6174 ✅** | 21.8492 | 21.9038 | +4.7682 | Higher | +4.7136 | Higher | -0.0546 | Lower |
| Cyclomatic (sum) | **15 ✅** | 17 | 18 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **8 ✅** | 10 | 22 | -2 | Lower | -14 | Lower | -12 | Lower |
| Halstead Volume | **3136.4157 ✅** | 3318.6421 | 3879.9630 | -182.2264 | Lower | -743.5473 | Lower | -561.3208 | Lower |
| Halstead Effort | **136434.0833 ✅** | 144994.4923 | 271721.2376 | -8560.4091 | Lower | -135287.1544 | Lower | -126726.7453 | Lower |
| SLOC | 110 | **107 ✅** | 113 | +3 | Higher | -3 | Lower | -6 | Lower |
| PLOC | **78 ✅** | 81 | 83 | -3 | Lower | -5 | Lower | -2 | Lower |
| LLOC | 56 | **53 ✅** | 67 | +3 | Higher | -11 | Lower | -14 | Lower |
| Comments (CLOC) | **18 ✅** | 12 | 16 | +6 | Higher | +2 | Higher | -4 | Lower |
| Blank Lines | 15 | **14 ✅** | 16 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |

## p03115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.2032 | **63.4206 ✅** | 60.9263 | -4.2174 | Lower | -1.7231 | Lower | +2.4943 | Higher |
| MI (SEI) | 33.5459 | **45.4583 ✅** | 32.8524 | -11.9124 | Lower | +0.6935 | Higher | +12.6059 | Higher |
| Cyclomatic (sum) | 16 | **12 ✅** | 16 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 2092.8831 | **1727.5790 ✅** | 1906.2070 | +365.3041 | Higher | +186.6761 | Higher | -178.6280 | Lower |
| Halstead Effort | 85522.8131 | **61392.2586 ✅** | 87484.8695 | +24130.5545 | Higher | -1962.0563 | Lower | -26092.6108 | Lower |
| SLOC | 68 | **59 ✅** | 63 | +9 | Higher | +5 | Higher | -4 | Lower |
| PLOC | 56 | **42 ✅** | 51 | +14 | Higher | +5 | Higher | -9 | Lower |
| LLOC | 37 | **29 ✅** | 34 | +8 | Higher | +3 | Higher | -5 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 4 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |

## p03123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.1328 | **62.2425 ✅** | 39.2437 | -21.1097 | Lower | +1.8890 | Higher | +22.9987 | Higher |
| MI (SEI) | 5.7426 | **57.8152 ✅** | -0.2715 | -52.0726 | Lower | +6.0141 | Higher | +58.0867 | Higher |
| Cyclomatic (sum) | 24 | **10 ✅** | 26 | +14 | Higher | -2 | Lower | -16 | Lower |
| Cognitive (sum) | 24 | **12 ✅** | 25 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 4297.6067 | **1329.7127 ✅** | 4795.9946 | +2967.8940 | Higher | -498.3878 | Lower | -3466.2819 | Lower |
| Halstead Effort | 246196.5047 | **45431.8517 ✅** | 288866.4436 | +200764.6530 | Higher | -42669.9389 | Lower | -243434.5919 | Lower |
| SLOC | 147 | **71 ✅** | 155 | +76 | Higher | -8 | Lower | -84 | Lower |
| PLOC | 117 | **37 ✅** | 126 | +80 | Higher | -9 | Lower | -89 | Lower |
| LLOC | 87 | **29 ✅** | 96 | +58 | Higher | -9 | Lower | -67 | Lower |
| Comments (CLOC) | 10 | **31 ✅** | 7 | -21 | Lower | +3 | Higher | +24 | Higher |
| Blank Lines | 20 | **6 ✅** | 22 | +14 | Higher | -2 | Lower | -16 | Lower |
| Functions | 8 | **2 ✅** | 9 | +6 | Higher | -1 | Lower | -7 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |

## p03142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.8118 | **52.5380 ✅** | 51.0945 | -3.7261 | Lower | -2.2827 | Lower | +1.4434 | Higher |
| MI (SEI) | 19.9228 | **36.3560 ✅** | 26.6390 | -16.4333 | Lower | -6.7162 | Lower | +9.7170 | Higher |
| Cyclomatic (sum) | 32 | 24 | **23 ✅** | +8 | Higher | +9 | Higher | +1 | Higher |
| Cognitive (sum) | 43 | **23 ✅** | 42 | +20 | Higher | +1 | Higher | -19 | Lower |
| Halstead Volume | 2868.5045 | **2456.7661 ✅** | 2663.3167 | +411.7383 | Higher | +205.1878 | Higher | -206.5505 | Lower |
| Halstead Effort | 181791.4696 | **121886.3092 ✅** | 138682.7034 | +59905.1604 | Higher | +43108.7663 | Higher | -16796.3942 | Lower |
| SLOC | 93 | **87 ✅** | 94 | +6 | Higher | -1 | Lower | -7 | Lower |
| PLOC | 75 | **57 ✅** | 71 | +18 | Higher | +4 | Higher | -14 | Lower |
| LLOC | 48 | **34 ✅** | 50 | +14 | Higher | -2 | Lower | -16 | Lower |
| Comments (CLOC) | 8 | **20 ✅** | 12 | -12 | Lower | -4 | Lower | +8 | Higher |
| Blank Lines | **10 ✅** | 11 | 11 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 6 | 4 | **0 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03145

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.2232 | **87.7973 ✅** | 79.8293 | -0.5741 | Lower | +7.3939 | Higher | +7.9680 | Higher |
| MI (SEI) | **88.0565 ✅** | 87.3488 | 71.4777 | +0.7077 | Higher | +16.5788 | Higher | +15.8711 | Higher |
| Cyclomatic (sum) | 2 | 2 | 5 | 0 | Same | -3 | Lower | -3 | Lower |
| Cognitive (sum) | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | **520.1906 ✅** | 535 | 745.0699 | -14.8094 | Lower | -224.8793 | Lower | -210.0699 | Lower |
| Halstead Effort | **5259.7051 ✅** | 5825.5556 | 11014.0773 | -565.8505 | Lower | -5754.3722 | Lower | -5188.5217 | Lower |
| SLOC | 23 | **22 ✅** | 31 | +1 | Higher | -8 | Lower | -9 | Lower |
| PLOC | 12 | 12 | 21 | 0 | Same | -9 | Lower | -9 | Lower |
| LLOC | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | **7 ✅** | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03157

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.0039 | 46.4053 | **50.1562 ✅** | +3.5986 | Higher | -0.1522 | Lower | -3.7508 | Lower |
| MI (SEI) | 22.7155 | 10.7082 | **22.8333 ✅** | +12.0073 | Higher | -0.1178 | Lower | -12.1251 | Lower |
| Cyclomatic (sum) | 27 | 29 | **26 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | **39 ✅** | 60 | 57 | -21 | Lower | -18 | Lower | +3 | Higher |
| Halstead Volume | **3151.4125 ✅** | 3461.3518 | 3198.8956 | -309.9394 | Lower | -47.4831 | Lower | +262.4562 | Higher |
| Halstead Effort | **179388.0956 ✅** | 228522.8674 | 194066.3340 | -49134.7718 | Lower | -14678.2385 | Lower | +34456.5333 | Higher |
| SLOC | 90 | 106 | 90 | -16 | Lower | 0 | Same | +16 | Higher |
| PLOC | 71 | 88 | **70 ✅** | -17 | Lower | +1 | Higher | +18 | Higher |
| LLOC | 49 | 66 | 49 | -17 | Lower | 0 | Same | +17 | Higher |
| Comments (CLOC) | 9 | 5 | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Blank Lines | 11 | 13 | 11 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p03164

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.0184 | 50.7094 | **54.7676 ✅** | +2.3090 | Higher | -1.7492 | Lower | -4.0582 | Lower |
| MI (SEI) | 29.0454 | 29.1097 | **34.3025 ✅** | -0.0643 | Lower | -5.2571 | Lower | -5.1928 | Lower |
| Cyclomatic (sum) | 26 | 26 | **24 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 24 | 26 | **23 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 2374.1105 | 2511.6669 | **2248.5146 ✅** | -137.5564 | Lower | +125.5959 | Higher | +263.1522 | Higher |
| Halstead Effort | 114912.7391 | 109257.5084 | **105074.8175 ✅** | +5655.2307 | Higher | +9837.9216 | Higher | +4182.6910 | Higher |
| SLOC | 83 | 94 | **78 ✅** | -11 | Lower | +5 | Higher | +16 | Higher |
| PLOC | 63 | 70 | **55 ✅** | -7 | Lower | +8 | Higher | +15 | Higher |
| LLOC | 40 | 38 | **35 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Comments (CLOC) | 10 | **15 ✅** | 12 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 10 | **9 ✅** | 11 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03165

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.4522 | **62.5437 ✅** | 59.1368 | -1.0915 | Lower | +2.3155 | Higher | +3.4069 | Higher |
| MI (SEI) | 42.0060 | **43.1813 ✅** | 41.8112 | -1.1754 | Lower | +0.1948 | Higher | +1.3701 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 14 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 21 | 21 | **18 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 2161.3733 | **1821.1283 ✅** | 2024.4267 | +340.2450 | Higher | +136.9465 | Higher | -203.2984 | Lower |
| Halstead Effort | 113171.9055 | **94091.6293 ✅** | 97959.7609 | +19080.2761 | Higher | +15212.1446 | Higher | -3868.1315 | Lower |
| SLOC | **62 ✅** | 63 | 71 | -1 | Lower | -9 | Lower | -8 | Lower |
| PLOC | 46 | 46 | 50 | 0 | Same | -4 | Lower | -4 | Lower |
| LLOC | 40 | 39 | **38 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Comments (CLOC) | 9 | 9 | **13 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p03173

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.6076 ✅** | 58.1773 | 62.5475 | +6.4303 | Higher | +2.0602 | Higher | -4.3701 | Lower |
| MI (SEI) | **44.5637 ✅** | 37.3472 | 40.9979 | +7.2164 | Higher | +3.5658 | Higher | -3.6507 | Lower |
| Cyclomatic (sum) | **10 ✅** | 13 | 12 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **10 ✅** | 13 | 11 | -3 | Lower | -1 | Lower | +2 | Higher |
| Halstead Volume | **1767.3657 ✅** | 2058.5727 | 1939.2024 | -291.2071 | Lower | -171.8368 | Lower | +119.3703 | Higher |
| Halstead Effort | 92256.4882 | **82710.5120 ✅** | 96852.3883 | +9545.9762 | Higher | -4595.9001 | Lower | -14141.8764 | Lower |
| SLOC | **56 ✅** | 76 | 60 | -20 | Lower | -4 | Lower | +16 | Higher |
| PLOC | **38 ✅** | 54 | 42 | -16 | Lower | -4 | Lower | +12 | Higher |
| LLOC | **26 ✅** | 33 | 30 | -7 | Lower | -4 | Lower | +3 | Higher |
| Comments (CLOC) | 7 | **11 ✅** | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 11 | 12 | 11 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **2 ✅** | 6 | 3 | -4 | Lower | -1 | Lower | +3 | Higher |

## p03183

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.0786 ✅** | 63.1708 | 55.3364 | +3.9078 | Higher | +11.7423 | Higher | +7.8345 | Higher |
| MI (SEI) | **47.1036 ✅** | 45.5936 | 32.0688 | +1.5099 | Higher | +15.0348 | Higher | +13.5249 | Higher |
| Cyclomatic (sum) | **9 ✅** | 10 | 16 | -1 | Lower | -7 | Lower | -6 | Lower |
| Cognitive (sum) | **6 ✅** | 11 | 26 | -5 | Lower | -20 | Lower | -15 | Lower |
| Halstead Volume | **1446.8697 ✅** | 1696.7294 | 2118.1739 | -249.8597 | Lower | -671.3041 | Lower | -421.4445 | Lower |
| Halstead Effort | **33704.6442 ✅** | 49128.0278 | 68386.7558 | -15423.3836 | Lower | -34682.1115 | Lower | -19258.7279 | Lower |
| SLOC | **52 ✅** | 62 | 86 | -10 | Lower | -34 | Lower | -24 | Lower |
| PLOC | **38 ✅** | 44 | 66 | -6 | Lower | -28 | Lower | -22 | Lower |
| LLOC | **22 ✅** | 26 | 38 | -4 | Lower | -16 | Lower | -12 | Lower |
| Comments (CLOC) | 6 | 10 | **11 ✅** | -4 | Lower | -5 | Lower | -1 | Lower |
| Blank Lines | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03194

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.0669 ✅** | 74.5404 | 68.4293 | +0.5265 | Higher | +6.6376 | Higher | +6.1111 | Higher |
| MI (SEI) | **60.0543 ✅** | 56.4545 | 57.4615 | +3.5999 | Higher | +2.5928 | Higher | -1.0071 | Lower |
| Cyclomatic (sum) | 8 | 8 | 12 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | **10 ✅** | 11 | 12 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | **864.6653 ✅** | 882.4241 | 1104.2397 | -17.7588 | Lower | -239.5744 | Lower | -221.8156 | Lower |
| Halstead Effort | **28121.2743 ✅** | 28678.7824 | 42402.8047 | -557.5082 | Lower | -14281.5304 | Lower | -13724.0222 | Lower |
| SLOC | **38 ✅** | 39 | 50 | -1 | Lower | -12 | Lower | -11 | Lower |
| PLOC | 28 | 28 | 34 | 0 | Same | -6 | Lower | -6 | Lower |
| LLOC | 25 | 25 | 25 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 5 | 4 | **11 ✅** | +1 | Higher | -6 | Lower | -7 | Lower |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p03206

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.2473 | **93.2283 ✅** | 85.1318 | -5.9809 | Lower | +2.1155 | Higher | +8.0964 | Higher |
| MI (SEI) | **77.3196 ✅** | 59.5119 | 76.8698 | +17.8078 | Higher | +0.4498 | Higher | -17.3579 | Lower |
| Cyclomatic (sum) | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 433.8179 | **406.9970 ✅** | 480.8030 | +26.8209 | Higher | -46.9851 | Lower | -73.8060 | Lower |
| Halstead Effort | 4635.0017 | **3846.1219 ✅** | 5494.8917 | +788.8798 | Higher | -859.8900 | Lower | -1648.7698 | Lower |
| SLOC | 23 | **16 ✅** | 25 | +7 | Higher | -2 | Lower | -9 | Lower |
| PLOC | 18 | **13 ✅** | 18 | +5 | Higher | 0 | Same | -5 | Lower |
| LLOC | 11 | **4 ✅** | 11 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 3 | 0 | **4 ✅** | +3 | Higher | -1 | Lower | -4 | Lower |
| Blank Lines | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03209

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **82.5555 ✅** | 75.0100 | 79.2394 | +7.5455 | Higher | +3.3161 | Higher | -4.2294 | Lower |
| MI (SEI) | 59.2578 | 57.2131 | **62.7552 ✅** | +2.0447 | Higher | -3.4974 | Lower | -5.5421 | Lower |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **824.7922 ✅** | 955.0286 | 884.3401 | -130.2363 | Lower | -59.5478 | Lower | +70.6885 | Higher |
| Halstead Effort | 20863.4947 | 25480.1627 | **18902.7687 ✅** | -4616.6680 | Lower | +1960.7260 | Higher | +6577.3940 | Higher |
| SLOC | **25 ✅** | 38 | 30 | -13 | Lower | -5 | Lower | +8 | Higher |
| PLOC | **21 ✅** | 26 | 22 | -5 | Lower | -1 | Lower | +4 | Higher |
| LLOC | 11 | 22 | 11 | -11 | Lower | 0 | Same | +11 | Higher |
| Comments (CLOC) | 1 | **4 ✅** | 3 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | **3 ✅** | 8 | 5 | -5 | Lower | -2 | Lower | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03213

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.9173 ✅** | 58.7690 | 66.7887 | +12.1483 | Higher | +4.1286 | Higher | -8.0197 | Lower |
| MI (SEI) | **56.1105 ✅** | 47.7498 | 49.0241 | +8.3607 | Higher | +7.0864 | Higher | -1.2743 | Lower |
| Cyclomatic (sum) | **8 ✅** | 13 | 12 | -5 | Lower | -4 | Lower | +1 | Higher |
| Cognitive (sum) | **11 ✅** | 20 | 17 | -9 | Lower | -6 | Lower | +3 | Higher |
| Halstead Volume | **1134.1230 ✅** | 1506.4117 | 1423.2862 | -372.2887 | Lower | -289.1632 | Lower | +83.1255 | Higher |
| Halstead Effort | **39255.2894 ✅** | 56361.3165 | 60144.0082 | -17106.0270 | Lower | -20888.7188 | Lower | -3782.6917 | Lower |
| SLOC | **45 ✅** | 81 | 51 | -36 | Lower | -6 | Lower | +30 | Higher |
| PLOC | **32 ✅** | 51 | 39 | -19 | Lower | -7 | Lower | +12 | Higher |
| LLOC | **23 ✅** | 50 | 26 | -27 | Lower | -3 | Lower | +24 | Higher |
| Comments (CLOC) | 7 | **24 ✅** | 7 | -17 | Lower | 0 | Same | +17 | Higher |
| Blank Lines | 6 | 11 | 6 | -5 | Lower | 0 | Same | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03216

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **59.9790 ✅** | 59.7086 | 56.6419 | +0.2705 | Higher | +3.3371 | Higher | +3.0667 | Higher |
| MI (SEI) | 34.3680 | **46.2414 ✅** | 32.1953 | -11.8734 | Lower | +2.1727 | Higher | +14.0461 | Higher |
| Cyclomatic (sum) | 16 | **14 ✅** | 16 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | **18 ✅** | 25 | 31 | -7 | Lower | -13 | Lower | -6 | Lower |
| Halstead Volume | **1647.1401 ✅** | 1813.6186 | 1911.4633 | -166.4785 | Lower | -264.3233 | Lower | -97.8447 | Lower |
| Halstead Effort | **55343.9058 ✅** | 66429.0532 | 62233.6899 | -11085.1474 | Lower | -6889.7841 | Lower | +4195.3633 | Higher |
| SLOC | **70 ✅** | 71 | 82 | -1 | Lower | -12 | Lower | -11 | Lower |
| PLOC | 54 | **42 ✅** | 62 | +12 | Higher | -8 | Lower | -20 | Lower |
| LLOC | 44 | **34 ✅** | 55 | +10 | Higher | -11 | Lower | -21 | Lower |
| Comments (CLOC) | 6 | **17 ✅** | 9 | -11 | Lower | -3 | Lower | +8 | Higher |
| Blank Lines | **10 ✅** | 12 | 12 | -2 | Lower | -2 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03218

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.6991 ✅** | 65.8963 | 65.2995 | +3.8028 | Higher | +4.3996 | Higher | +0.5968 | Higher |
| MI (SEI) | **49.6395 ✅** | 44.4720 | 46.8331 | +5.1675 | Higher | +2.8064 | Higher | -2.3611 | Lower |
| Cyclomatic (sum) | **8 ✅** | 10 | 11 | -2 | Lower | -3 | Lower | -1 | Lower |
| Cognitive (sum) | **4 ✅** | 6 | 9 | -2 | Lower | -5 | Lower | -3 | Lower |
| Halstead Volume | **1172.4109 ✅** | 1305.4228 | 1326.9561 | -133.0119 | Lower | -154.5453 | Lower | -21.5333 | Lower |
| Halstead Effort | **27285.1990 ✅** | 35937.5224 | 32255.2416 | -8652.3234 | Lower | -4970.0427 | Lower | +3682.2808 | Higher |
| SLOC | **48 ✅** | 57 | 58 | -9 | Lower | -10 | Lower | -1 | Lower |
| PLOC | **34 ✅** | 40 | 41 | -6 | Lower | -7 | Lower | -1 | Lower |
| LLOC | 23 | 28 | **19 ✅** | -5 | Lower | +4 | Higher | +9 | Higher |
| Comments (CLOC) | 5 | 6 | **8 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p03268

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.4355 | **86.8511 ✅** | 80.1786 | -1.4156 | Lower | +5.2569 | Higher | +6.6724 | Higher |
| MI (SEI) | **77.4183 ✅** | 72.4191 | 69.4180 | +4.9992 | Higher | +8.0004 | Higher | +3.0012 | Higher |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 614.7155 | **614.0075 ✅** | 761.0979 | +0.7080 | Higher | -146.3824 | Lower | -147.0904 | Lower |
| Halstead Effort | **11647.2410 ✅** | 11973.1466 | 12707.0261 | -325.9056 | Lower | -1059.7851 | Lower | -733.8795 | Lower |
| SLOC | 24 | **22 ✅** | 31 | +2 | Higher | -7 | Lower | -9 | Lower |
| PLOC | 16 | **15 ✅** | 20 | +1 | Higher | -4 | Lower | -5 | Lower |
| LLOC | **9 ✅** | 12 | 12 | -3 | Lower | -3 | Lower | 0 | Same |
| Comments (CLOC) | 4 | 2 | **5 ✅** | +2 | Higher | -1 | Lower | -3 | Lower |
| Blank Lines | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.7275 | 48.8320 | **51.8608 ✅** | +0.8955 | Higher | -2.1334 | Lower | -3.0288 | Lower |
| MI (SEI) | **26.5910 ✅** | 26.1379 | 19.7339 | +0.4531 | Higher | +6.8571 | Higher | +6.4041 | Higher |
| Cyclomatic (sum) | 18 | 18 | **17 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 16 | 15 | 15 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | **2724.7510 ✅** | 2814.7040 | 2806.6729 | -89.9530 | Lower | -81.9219 | Lower | +8.0310 | Higher |
| Halstead Effort | **132604.5491 ✅** | 138948.9437 | 178463.3245 | -6344.3946 | Lower | -45858.7754 | Lower | -39514.3808 | Lower |
| SLOC | 109 | 114 | **96 ✅** | -5 | Lower | +13 | Higher | +18 | Higher |
| PLOC | 76 | 80 | **74 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| LLOC | **53 ✅** | 54 | 59 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | 17 | **19 ✅** | 6 | -2 | Lower | +11 | Higher | +13 | Higher |
| Blank Lines | 18 | 16 | 16 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p03278

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.6211 | **38.3216 ✅** | 35.5246 | -3.7005 | Lower | -0.9036 | Lower | +2.7970 | Higher |
| MI (SEI) | 1.5654 | **11.0520 ✅** | -4.3552 | -9.4866 | Lower | +5.9206 | Higher | +15.4072 | Higher |
| Cyclomatic (sum) | 33 | 26 | **22 ✅** | +7 | Higher | +11 | Higher | +4 | Higher |
| Cognitive (sum) | 61 | 45 | **28 ✅** | +16 | Higher | +33 | Higher | +17 | Higher |
| Halstead Volume | 6079.1284 | **5289.5532 ✅** | 7345.3271 | +789.5752 | Higher | -1266.1987 | Lower | -2055.7739 | Lower |
| Halstead Effort | 613801.9970 | **481084.8638 ✅** | 793408.3356 | +132717.1332 | Higher | -179606.3386 | Lower | -312323.4718 | Lower |
| SLOC | 173 | **159 ✅** | 180 | +14 | Higher | -7 | Lower | -21 | Lower |
| PLOC | 134 | **113 ✅** | 146 | +21 | Higher | -12 | Lower | -33 | Lower |
| LLOC | 120 | **104 ✅** | 112 | +16 | Higher | +8 | Higher | -8 | Lower |
| Comments (CLOC) | 18 | **25 ✅** | 10 | -7 | Lower | +8 | Higher | +15 | Higher |
| Blank Lines | 21 | 21 | 24 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p03294

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **86.4623 ✅** | 82.1984 | 84.1982 | +4.2638 | Higher | +2.2640 | Higher | -1.9998 | Lower |
| MI (SEI) | 75.8815 | 75.6826 | **77.6956 ✅** | +0.1989 | Higher | -1.8141 | Lower | -2.0130 | Lower |
| Cyclomatic (sum) | **3 ✅** | 4 | 5 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | 576.1088 | 607.8213 | **556.2803 ✅** | -31.7124 | Lower | +19.8285 | Higher | +51.5409 | Higher |
| Halstead Effort | 8394.7288 | 9377.8137 | **7430.3157 ✅** | -983.0848 | Lower | +964.4132 | Higher | +1947.4980 | Higher |
| SLOC | **23 ✅** | 29 | 26 | -6 | Lower | -3 | Lower | +3 | Higher |
| PLOC | **15 ✅** | 17 | 16 | -2 | Lower | -1 | Lower | +1 | Higher |
| LLOC | 12 | 13 | **10 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Comments (CLOC) | 3 | **6 ✅** | 5 | -3 | Lower | -2 | Lower | +1 | Higher |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03296

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.3786 ✅** | 79.1145 | 78.9333 | +1.2641 | Higher | +1.4453 | Higher | +0.1812 | Higher |
| MI (SEI) | 67.5698 | 64.9836 | **71.8256 ✅** | +2.5862 | Higher | -4.2558 | Lower | -6.8420 | Lower |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **742.4827 ✅** | 774.3513 | 801.8106 | -31.8686 | Lower | -59.3279 | Lower | -27.4593 | Lower |
| Halstead Effort | 20047.0329 | 17406.0702 | **17188.8137 ✅** | +2640.9627 | Higher | +2858.2192 | Higher | +217.2565 | Higher |
| SLOC | **30 ✅** | 32 | 32 | -2 | Lower | -2 | Lower | 0 | Same |
| PLOC | 22 | 23 | 22 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 19 | 19 | 20 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 4 | 4 | **7 ✅** | 0 | Same | -3 | Lower | -3 | Lower |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03311

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.4065 ✅** | 76.7815 | 55.6365 | +0.6251 | Higher | +21.7700 | Higher | +21.1449 | Higher |
| MI (SEI) | **72.8180 ✅** | 68.4614 | 32.5380 | +4.3566 | Higher | +40.2800 | Higher | +35.9234 | Higher |
| Cyclomatic (sum) | 4 | 4 | 15 | 0 | Same | -11 | Lower | -11 | Lower |
| Cognitive (sum) | 2 | 2 | 23 | 0 | Same | -21 | Lower | -21 | Lower |
| Halstead Volume | **778.8222 ✅** | 806.4400 | 2167.3275 | -27.6178 | Lower | -1388.5052 | Lower | -1360.8874 | Lower |
| Halstead Effort | **14392.6349 ✅** | 15580.4217 | 116015.7639 | -1187.7868 | Lower | -101623.1290 | Lower | -100435.3422 | Lower |
| SLOC | **36 ✅** | 37 | 85 | -1 | Lower | -49 | Lower | -48 | Lower |
| PLOC | 20 | 20 | 60 | 0 | Same | -40 | Lower | -40 | Lower |
| LLOC | 16 | 16 | 57 | 0 | Same | -41 | Lower | -41 | Lower |
| Comments (CLOC) | 10 | 8 | **11 ✅** | +2 | Higher | -1 | Lower | -3 | Lower |
| Blank Lines | **6 ✅** | 9 | 14 | -3 | Lower | -8 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.9360 ✅** | 75.4560 | 75.1061 | +6.4800 | Higher | +6.8299 | Higher | +0.3498 | Higher |
| MI (SEI) | **75.7765 ✅** | 70.2174 | 65.5391 | +5.5591 | Higher | +10.2374 | Higher | +4.6783 | Higher |
| Cyclomatic (sum) | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | **713.1384 ✅** | 883.1796 | 864.6653 | -170.0413 | Lower | -151.5270 | Lower | +18.5143 | Higher |
| Halstead Effort | **15764.1112 ✅** | 17476.7661 | 18739.9579 | -1712.6549 | Lower | -2975.8467 | Lower | -1263.1918 | Lower |
| SLOC | **28 ✅** | 39 | 39 | -11 | Lower | -11 | Lower | 0 | Same |
| PLOC | **17 ✅** | 23 | 25 | -6 | Lower | -8 | Lower | -2 | Lower |
| LLOC | **13 ✅** | 15 | 15 | -2 | Lower | -2 | Lower | 0 | Same |
| Comments (CLOC) | 6 | **11 ✅** | 8 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03318

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.5254 ✅** | 67.4526 | 66.5868 | +4.0728 | Higher | +4.9385 | Higher | +0.8658 | Higher |
| MI (SEI) | **56.8860 ✅** | 51.8520 | 51.3932 | +5.0339 | Higher | +5.4928 | Higher | +0.4589 | Higher |
| Cyclomatic (sum) | **7 ✅** | 9 | 8 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | **7 ✅** | 12 | 8 | -5 | Lower | -1 | Lower | +4 | Higher |
| Halstead Volume | **1054.5813 ✅** | 1130.5903 | 1248.8106 | -76.0090 | Lower | -194.2292 | Lower | -118.2203 | Lower |
| Halstead Effort | **37015.8053 ✅** | 41791.4641 | 40371.0319 | -4775.6588 | Lower | -3355.2266 | Lower | +1420.4321 | Higher |
| SLOC | **45 ✅** | 55 | 57 | -10 | Lower | -12 | Lower | -2 | Lower |
| PLOC | **32 ✅** | 36 | 36 | -4 | Lower | -4 | Lower | 0 | Same |
| LLOC | **29 ✅** | 30 | 31 | -1 | Lower | -2 | Lower | -1 | Lower |
| Comments (CLOC) | 7 | 9 | **10 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | **7 ✅** | 10 | 11 | -3 | Lower | -4 | Lower | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p03361

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.8171 ✅** | 60.7760 | 63.9212 | +8.0411 | Higher | +4.8959 | Higher | -3.1452 | Lower |
| MI (SEI) | **53.2138 ✅** | 41.2540 | 51.7539 | +11.9598 | Higher | +1.4598 | Higher | -10.5000 | Lower |
| Cyclomatic (sum) | **12 ✅** | 18 | 15 | -6 | Lower | -3 | Lower | +3 | Higher |
| Cognitive (sum) | **21 ✅** | 34 | 25 | -13 | Lower | -4 | Lower | +9 | Higher |
| Halstead Volume | **1328.8775 ✅** | 1710 | 1529.8847 | -381.1225 | Lower | -201.0071 | Lower | +180.1153 | Higher |
| Halstead Effort | **39137.5871 ✅** | 63180 | 45896.5399 | -24042.4129 | Lower | -6758.9529 | Lower | +17283.4601 | Higher |
| SLOC | **46 ✅** | 64 | 57 | -18 | Lower | -11 | Lower | +7 | Higher |
| PLOC | **31 ✅** | 46 | 37 | -15 | Lower | -6 | Lower | +9 | Higher |
| LLOC | **22 ✅** | 32 | 23 | -10 | Lower | -1 | Lower | +9 | Higher |
| Comments (CLOC) | 7 | 9 | **13 ✅** | -2 | Lower | -6 | Lower | -4 | Lower |
| Blank Lines | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p03365

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.4820 ✅** | 55.5902 | 58.4388 | +2.8918 | Higher | +0.0432 | Higher | -2.8486 | Lower |
| MI (SEI) | 30.2468 | 25.3579 | **31.9876 ✅** | +4.8890 | Higher | -1.7407 | Lower | -6.6297 | Lower |
| Cyclomatic (sum) | 13 | 16 | 13 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 9 | 12 | 9 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | **2623.3331 ✅** | 2846.5701 | 2645.2271 | -223.2370 | Lower | -21.8940 | Lower | +201.3430 | Higher |
| Halstead Effort | 161334.9876 | 186417.2446 | **146088.6803 ✅** | -25082.2570 | Lower | +15246.3072 | Higher | +40328.5643 | Higher |
| SLOC | 69 | 77 | 69 | -8 | Lower | 0 | Same | +8 | Higher |
| PLOC | 54 | 63 | **53 ✅** | -9 | Lower | +1 | Higher | +10 | Higher |
| LLOC | 46 | 51 | 46 | -5 | Lower | 0 | Same | +5 | Higher |
| Comments (CLOC) | 5 | 5 | **6 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p03369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **92.3964 ✅** | 90.2073 | 85.8834 | +2.1891 | Higher | +6.5130 | Higher | +4.3239 | Higher |
| MI (SEI) | **96.8969 ✅** | 78.3748 | 83.6832 | +18.5221 | Higher | +13.2137 | Higher | -5.3084 | Lower |
| Cyclomatic (sum) | **3 ✅** | 4 | 5 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **0 ✅** | 3 | 4 | -3 | Lower | -4 | Lower | -1 | Lower |
| Halstead Volume | **394.9548 ✅** | 414.5863 | 516.2342 | -19.6315 | Lower | -121.2794 | Lower | -101.6479 | Lower |
| Halstead Effort | **4157.4189 ✅** | 4698.6444 | 7904.8361 | -541.2256 | Lower | -3747.4172 | Lower | -3206.1916 | Lower |
| SLOC | **18 ✅** | 20 | 24 | -2 | Lower | -6 | Lower | -4 | Lower |
| PLOC | **9 ✅** | 14 | 15 | -5 | Lower | -6 | Lower | -1 | Lower |
| LLOC | **6 ✅** | 10 | 11 | -4 | Lower | -5 | Lower | -1 | Lower |
| Comments (CLOC) | 6 | 2 | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Blank Lines | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03406

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.5368 | **66.6474 ✅** | 45.5045 | -20.1106 | Lower | +1.0323 | Higher | +21.1429 | Higher |
| MI (SEI) | 11.0207 | **42.5014 ✅** | 11.1183 | -31.4807 | Lower | -0.0976 | Lower | +31.3831 | Higher |
| Cyclomatic (sum) | 20 | **12 ✅** | 24 | +8 | Higher | -4 | Lower | -12 | Lower |
| Cognitive (sum) | 32 | **9 ✅** | 34 | +23 | Higher | -2 | Lower | -25 | Lower |
| Halstead Volume | 4233.0262 | **1376.6255 ✅** | 4093.4028 | +2856.4006 | Higher | +139.6234 | Higher | -2716.7773 | Lower |
| Halstead Effort | 306609.4830 | **39355.2947 ✅** | 303167.6470 | +267254.1883 | Higher | +3441.8360 | Higher | -263812.3523 | Lower |
| SLOC | 112 | **52 ✅** | 114 | +60 | Higher | -2 | Lower | -62 | Lower |
| PLOC | 93 | **41 ✅** | 94 | +52 | Higher | -1 | Lower | -53 | Lower |
| LLOC | 79 | **24 ✅** | 72 | +55 | Higher | +7 | Higher | -48 | Lower |
| Comments (CLOC) | 6 | 4 | **7 ✅** | +2 | Higher | -1 | Lower | -3 | Lower |
| Blank Lines | 13 | **7 ✅** | 13 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p03408

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.6990 | 60.4120 | **71.4354 ✅** | +10.2870 | Higher | -0.7365 | Lower | -11.0235 | Lower |
| MI (SEI) | **46.7853 ✅** | 39.3506 | 40.0798 | +7.4347 | Higher | +6.7055 | Higher | -0.7292 | Lower |
| Cyclomatic (sum) | 10 | 15 | 10 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 15 | 19 | 15 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 1082.6181 | 1584.0926 | 1082.6181 | -501.4744 | Lower | 0 | Same | +501.4744 | Higher |
| Halstead Effort | 30464.8746 | 51235.4938 | 30464.8746 | -20770.6192 | Lower | 0 | Same | +20770.6192 | Higher |
| SLOC | 45 | 70 | **43 ✅** | -25 | Lower | +2 | Higher | +27 | Higher |
| PLOC | 34 | 54 | 34 | -20 | Lower | 0 | Same | +20 | Higher |
| LLOC | 26 | 36 | 26 | -10 | Lower | 0 | Same | +10 | Higher |
| Comments (CLOC) | 3 | **9 ✅** | 1 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03414

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 14.9604 | **26.6384 ✅** | 11.9450 | -11.6780 | Lower | +3.0155 | Higher | +14.6935 | Higher |
| MI (SEI) | -27.4914 | **-8.7068 ✅** | -37.8577 | -18.7846 | Lower | +10.3663 | Higher | +29.1509 | Higher |
| Cyclomatic (sum) | 55 | **48 ✅** | 62 | +7 | Higher | -7 | Lower | -14 | Lower |
| Cognitive (sum) | 88 | 104 | **81 ✅** | -16 | Lower | +7 | Higher | +23 | Higher |
| Halstead Volume | 11837.9902 | **7174.6786 ✅** | 12258.2047 | +4663.3116 | Higher | -420.2145 | Lower | -5083.5261 | Lower |
| Halstead Effort | 1482116.3742 | **541596.2495 ✅** | 1411882.5039 | +940520.1247 | Higher | +70233.8703 | Higher | -870286.2544 | Lower |
| SLOC | 344 | **217 ✅** | 371 | +127 | Higher | -27 | Lower | -154 | Lower |
| PLOC | 296 | **176 ✅** | 331 | +120 | Higher | -35 | Lower | -155 | Lower |
| LLOC | 214 | **139 ✅** | 212 | +75 | Higher | +2 | Higher | -73 | Lower |
| Comments (CLOC) | **27 ✅** | 22 | 13 | +5 | Higher | +14 | Higher | +9 | Higher |
| Blank Lines | 30 | **20 ✅** | 28 | +10 | Higher | +2 | Higher | -8 | Lower |
| Functions | 8 | **1 ✅** | 10 | +7 | Higher | -2 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **2 ✅** | 10 | +7 | Higher | -1 | Lower | -8 | Lower |

## p03434

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.8759 ✅** | 74.7548 | 54.8266 | +1.1211 | Higher | +21.0493 | Higher | +19.9282 | Higher |
| MI (SEI) | **65.1235 ✅** | 62.8393 | 19.0685 | +2.2842 | Higher | +46.0550 | Higher | +43.7708 | Higher |
| Cyclomatic (sum) | 5 | 5 | 13 | 0 | Same | -8 | Lower | -8 | Lower |
| Cognitive (sum) | 5 | 5 | 13 | 0 | Same | -8 | Lower | -8 | Lower |
| Halstead Volume | **845.1050 ✅** | 893.5940 | 2766.8516 | -48.4890 | Lower | -1921.7466 | Lower | -1873.2576 | Lower |
| Halstead Effort | **16682.3723 ✅** | 20793.2450 | 172565.8978 | -4110.8727 | Lower | -155883.5255 | Lower | -151772.6528 | Lower |
| SLOC | **38 ✅** | 40 | 85 | -2 | Lower | -47 | Lower | -45 | Lower |
| PLOC | **24 ✅** | 25 | 68 | -1 | Lower | -44 | Lower | -43 | Lower |
| LLOC | **19 ✅** | 20 | 59 | -1 | Lower | -40 | Lower | -39 | Lower |
| Comments (CLOC) | 7 | 7 | 3 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | **7 ✅** | 8 | 14 | -1 | Lower | -7 | Lower | -6 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.0764 | **76.5592 ✅** | 66.5723 | -1.4828 | Lower | +8.5041 | Higher | +9.9869 | Higher |
| MI (SEI) | **58.2154 ✅** | 49.1896 | 44.9799 | +9.0258 | Higher | +13.2355 | Higher | +4.2097 | Higher |
| Cyclomatic (sum) | 9 | 9 | 13 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 10 | **9 ✅** | 21 | +1 | Higher | -11 | Lower | -12 | Lower |
| Halstead Volume | **977.2383 ✅** | 1060.5239 | 1419.5554 | -83.2856 | Lower | -442.3172 | Lower | -359.0315 | Lower |
| Halstead Effort | **34691.9582 ✅** | 36455.5089 | 64987.2473 | -1763.5507 | Lower | -30295.2890 | Lower | -28531.7383 | Lower |
| SLOC | 36 | **32 ✅** | 51 | +4 | Higher | -15 | Lower | -19 | Lower |
| PLOC | 26 | **25 ✅** | 39 | +1 | Higher | -13 | Lower | -14 | Lower |
| LLOC | 20 | **18 ✅** | 29 | +2 | Higher | -9 | Lower | -11 | Lower |
| Comments (CLOC) | 4 | 1 | **5 ✅** | +3 | Higher | -1 | Lower | -4 | Lower |
| Blank Lines | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p03448

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.3076 | 76.3076 | **77.4056 ✅** | 0 | Same | -1.0980 | Lower | -1.0980 | Lower |
| MI (SEI) | 57.5096 | 57.5096 | **61.9184 ✅** | 0 | Same | -4.4088 | Lower | -4.4088 | Lower |
| Cyclomatic (sum) | 9 | 9 | **6 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 12 | 12 | **10 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 921.5188 | 921.5188 | **851.9862 ✅** | 0 | Same | +69.5327 | Higher | +69.5327 | Higher |
| Halstead Effort | 20273.4139 | 20273.4139 | **12321.0305 ✅** | 0 | Same | +7952.3834 | Higher | +7952.3834 | Higher |
| SLOC | 34 | 34 | 34 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 25 | 25 | 25 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 18 | 18 | **16 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 3 | 3 | **4 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03454

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **49.9935 ✅** | 42.9032 | 48.3326 | +7.0903 | Higher | +1.6608 | Higher | -5.4294 | Lower |
| MI (SEI) | 14.5763 | **17.8255 ✅** | 17.5928 | -3.2492 | Lower | -3.0165 | Lower | +0.2327 | Higher |
| Cyclomatic (sum) | **24 ✅** | 25 | 25 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 40 | **21 ✅** | 40 | +19 | Higher | 0 | Same | -19 | Lower |
| Halstead Volume | **3148.9865 ✅** | 3643.1194 | 3214.8615 | -494.1329 | Lower | -65.8750 | Lower | +428.2579 | Higher |
| Halstead Effort | **240897.4681 ✅** | 242631.7542 | 247142.4775 | -1734.2861 | Lower | -6245.0093 | Lower | -4510.7232 | Lower |
| SLOC | **94 ✅** | 137 | 102 | -43 | Lower | -8 | Lower | +35 | Higher |
| PLOC | **75 ✅** | 97 | 79 | -22 | Lower | -4 | Lower | +18 | Higher |
| LLOC | 60 | 64 | 60 | -4 | Lower | 0 | Same | +4 | Higher |
| Comments (CLOC) | 4 | **22 ✅** | 8 | -18 | Lower | -4 | Lower | +14 | Higher |
| Blank Lines | 15 | 18 | 15 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | **4 ✅** | 10 | 5 | -6 | Lower | -1 | Lower | +5 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 13 | 4 | -10 | Lower | -1 | Lower | +9 | Higher |

## p03463

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.3582 | 78.1878 | **81.7482 ✅** | +0.1704 | Higher | -3.3900 | Lower | -3.5604 | Lower |
| MI (SEI) | 62.0948 | 57.7931 | **78.8831 ✅** | +4.3017 | Higher | -16.7884 | Lower | -21.0901 | Lower |
| Cyclomatic (sum) | 12 | 12 | **3 ✅** | 0 | Same | +9 | Higher | +9 | Higher |
| Cognitive (sum) | 5 | 5 | **2 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Halstead Volume | 803.4553 | 830.2185 | **692.7700 ✅** | -26.7632 | Lower | +110.6853 | Higher | +137.4486 | Higher |
| Halstead Effort | 16434.3129 | 16965.3347 | **8155.7917 ✅** | -531.0218 | Lower | +8278.5212 | Higher | +8809.5430 | Higher |
| SLOC | 30 | 30 | **29 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| PLOC | 23 | 24 | **15 ✅** | -1 | Lower | +8 | Higher | +9 | Higher |
| LLOC | **9 ✅** | 10 | 12 | -1 | Lower | -3 | Lower | -2 | Lower |
| Comments (CLOC) | 3 | 2 | **8 ✅** | +1 | Higher | -5 | Lower | -6 | Lower |
| Blank Lines | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | 3 | **0 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Exits | 3 | 3 | **0 ✅** | 0 | Same | +3 | Higher | +3 | Higher |

## p03473

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **106.0590 ✅** | 102.2958 | 96.1997 | +3.7632 | Higher | +9.8593 | Higher | +6.0961 | Higher |
| MI (SEI) | 77.5136 | 72.0844 | **98.2610 ✅** | +5.4292 | Higher | -20.7474 | Lower | -26.1766 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **258.5242 ✅** | 285.2933 | 286.7268 | -26.7691 | Lower | -28.2026 | Lower | -1.4336 | Lower |
| Halstead Effort | **2197.4556 ✅** | 2516.6940 | 2334.7756 | -319.2385 | Lower | -137.3200 | Lower | +181.9185 | Higher |
| SLOC | **9 ✅** | 11 | 16 | -2 | Lower | -7 | Lower | -5 | Lower |
| PLOC | **7 ✅** | 8 | 8 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 0 | 0 | **4 ✅** | 0 | Same | -4 | Lower | -4 | Lower |
| Blank Lines | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03478

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.7814 ✅** | 74.7248 | 76.8256 | +4.0566 | Higher | +1.9557 | Higher | -2.1008 | Lower |
| MI (SEI) | **67.3081 ✅** | 54.0022 | 61.6814 | +13.3058 | Higher | +5.6267 | Higher | -7.6792 | Lower |
| Cyclomatic (sum) | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | **789.8694 ✅** | 923.4712 | 1202.5542 | -133.6019 | Lower | -412.6848 | Lower | -279.0829 | Lower |
| Halstead Effort | **16653.0790 ✅** | 20580.2161 | 34210.5928 | -3927.1371 | Lower | -17557.5138 | Lower | -13630.3768 | Lower |
| SLOC | 32 | 38 | 32 | -6 | Lower | 0 | Same | +6 | Higher |
| PLOC | 22 | 29 | 22 | -7 | Lower | 0 | Same | +7 | Higher |
| LLOC | 19 | 22 | **18 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Comments (CLOC) | **5 ✅** | 3 | 4 | +2 | Higher | +1 | Higher | -1 | Lower |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.1259 ✅** | 69.6692 | 54.5695 | +5.4567 | Higher | +20.5564 | Higher | +15.0997 | Higher |
| MI (SEI) | **65.4658 ✅** | 59.8294 | 31.6226 | +5.6364 | Higher | +33.8432 | Higher | +28.2068 | Higher |
| Cyclomatic (sum) | **5 ✅** | 6 | 15 | -1 | Lower | -10 | Lower | -9 | Lower |
| Cognitive (sum) | **4 ✅** | 5 | 15 | -1 | Lower | -11 | Lower | -10 | Lower |
| Halstead Volume | **900.3406 ✅** | 1134.3875 | 2388.4331 | -234.0469 | Lower | -1488.0925 | Lower | -1254.0456 | Lower |
| Halstead Effort | **19285.2956 ✅** | 25788.4093 | 115156.5941 | -6503.1136 | Lower | -95871.2985 | Lower | -89368.1848 | Lower |
| SLOC | **39 ✅** | 50 | 88 | -11 | Lower | -49 | Lower | -38 | Lower |
| PLOC | **23 ✅** | 29 | 61 | -6 | Lower | -38 | Lower | -32 | Lower |
| LLOC | **19 ✅** | 24 | 50 | -5 | Lower | -31 | Lower | -26 | Lower |
| Comments (CLOC) | 8 | 12 | 12 | -4 | Lower | -4 | Lower | 0 | Same |
| Blank Lines | **8 ✅** | 9 | 16 | -1 | Lower | -8 | Lower | -7 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03544

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.8504 | **88.6040 ✅** | 31.7208 | -0.7536 | Lower | +56.1296 | Higher | +56.8832 | Higher |
| MI (SEI) | **85.0068 ✅** | 83.7219 | -5.8337 | +1.2848 | Higher | +90.8405 | Higher | +89.5556 | Higher |
| Cyclomatic (sum) | 3 | 3 | 46 | 0 | Same | -43 | Lower | -43 | Lower |
| Cognitive (sum) | 1 | 1 | 40 | 0 | Same | -39 | Lower | -39 | Lower |
| Halstead Volume | 506.6527 | 506.6527 | 5190.1859 | 0 | Same | -4683.5333 | Lower | -4683.5333 | Lower |
| Halstead Effort | 8386.4347 | 8386.4347 | 566202.1015 | 0 | Same | -557815.6669 | Lower | -557815.6669 | Lower |
| SLOC | 22 | **21 ✅** | 181 | +1 | Higher | -159 | Lower | -160 | Lower |
| PLOC | 13 | 13 | 154 | 0 | Same | -141 | Lower | -141 | Lower |
| LLOC | 12 | 12 | 111 | 0 | Same | -99 | Lower | -99 | Lower |
| Comments (CLOC) | 5 | 4 | **12 ✅** | +1 | Higher | -7 | Lower | -8 | Lower |
| Blank Lines | 4 | 4 | 18 | 0 | Same | -14 | Lower | -14 | Lower |
| Functions | 1 | 1 | 12 | 0 | Same | -11 | Lower | -11 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 12 | 0 | Same | -12 | Lower | -12 | Lower |

## p03562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.5925 | 19.6919 | **35.8481 ✅** | +14.9007 | Higher | -1.2556 | Lower | -16.1562 | Lower |
| MI (SEI) | **-1.9441 ✅** | -8.8919 | -2.4102 | +6.9478 | Higher | +0.4661 | Higher | -6.4817 | Lower |
| Cyclomatic (sum) | **33 ✅** | 63 | 36 | -30 | Lower | -3 | Lower | +27 | Higher |
| Cognitive (sum) | **42 ✅** | 67 | 55 | -25 | Lower | -13 | Lower | +12 | Higher |
| Halstead Volume | 5897.5530 | 7197.5932 | **5260.1305 ✅** | -1300.0402 | Lower | +637.4224 | Higher | +1937.4626 | Higher |
| Halstead Effort | 529949.1262 | 584529.2439 | **481696.4538 ✅** | -54580.1177 | Lower | +48252.6725 | Higher | +102832.7901 | Higher |
| SLOC | 175 | 269 | **161 ✅** | -94 | Lower | +14 | Higher | +108 | Higher |
| PLOC | 139 | 191 | **129 ✅** | -52 | Lower | +10 | Higher | +62 | Higher |
| LLOC | 110 | 113 | **106 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| Comments (CLOC) | 13 | **54 ✅** | 9 | -41 | Lower | +4 | Higher | +45 | Higher |
| Blank Lines | 25 | 26 | **23 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 6 | 10 | **4 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 5 | 14 | **2 ✅** | -9 | Lower | +3 | Higher | +12 | Higher |

## p03568

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.8876 ✅** | 81.1474 | 80.2953 | +0.7402 | Higher | +1.5923 | Higher | +0.8521 | Higher |
| MI (SEI) | **70.1537 ✅** | 61.6645 | 69.7899 | +8.4892 | Higher | +0.3638 | Higher | -8.1254 | Lower |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **617.3437 ✅** | 711.7859 | 681.2068 | -94.4422 | Lower | -63.8631 | Lower | +30.5791 | Higher |
| Halstead Effort | **10612.4315 ✅** | 12347.9380 | 11704.3713 | -1735.5065 | Lower | -1091.9398 | Lower | +643.5667 | Higher |
| SLOC | 29 | 29 | 31 | 0 | Same | -2 | Lower | -2 | Lower |
| PLOC | **19 ✅** | 21 | 20 | -2 | Lower | -1 | Lower | +1 | Higher |
| LLOC | **15 ✅** | 17 | 16 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 4 | 2 | **5 ✅** | +2 | Higher | -1 | Lower | -3 | Lower |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03592

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.4986 | **83.8273 ✅** | 81.8177 | -1.3287 | Lower | +0.6809 | Higher | +2.0096 | Higher |
| MI (SEI) | 71.9073 | **74.7841 ✅** | 62.9618 | -2.8769 | Lower | +8.9454 | Higher | +11.8223 | Higher |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 685.7667 | **675.0516 ✅** | 697.9744 | +10.7151 | Higher | -12.2077 | Lower | -22.9228 | Lower |
| Halstead Effort | 12735.6665 | 12536.6717 | **11018.0238 ✅** | +198.9948 | Higher | +1717.6427 | Higher | +1518.6479 | Higher |
| SLOC | 27 | **25 ✅** | 28 | +2 | Higher | -1 | Lower | -3 | Lower |
| PLOC | 19 | 19 | 21 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 4 | 4 | 2 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03608

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.3887 | 46.7873 | **52.6608 ✅** | +5.6014 | Higher | -0.2721 | Lower | -5.8734 | Lower |
| MI (SEI) | **23.9944 ✅** | 20.4753 | 16.1357 | +3.5191 | Higher | +7.8587 | Higher | +4.3396 | Higher |
| Cyclomatic (sum) | 18 | 23 | 18 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 30 | 30 | 30 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 2966.3176 | 3083.2590 | **2914.8260 ✅** | -116.9414 | Lower | +51.4916 | Higher | +168.4330 | Higher |
| Halstead Effort | **144038.8646 ✅** | 181059.4634 | 144177.2481 | -37020.5988 | Lower | -138.3835 | Lower | +36882.2153 | Higher |
| SLOC | 90 | 117 | **89 ✅** | -27 | Lower | +1 | Higher | +28 | Higher |
| PLOC | **74 ✅** | 88 | 75 | -14 | Lower | -1 | Lower | +13 | Higher |
| LLOC | **51 ✅** | 66 | 52 | -15 | Lower | -1 | Lower | +14 | Higher |
| Comments (CLOC) | 8 | **15 ✅** | 3 | -7 | Lower | +5 | Higher | +12 | Higher |
| Blank Lines | 11 | 14 | 11 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p03626

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.3167 | 59.1445 | **66.0816 ✅** | +4.1722 | Higher | -2.7649 | Lower | -6.9372 | Lower |
| MI (SEI) | **53.9018 ✅** | 41.1938 | 43.6587 | +12.7080 | Higher | +10.2431 | Higher | -2.4649 | Lower |
| Cyclomatic (sum) | **10 ✅** | 12 | 11 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 16 | **11 ✅** | 15 | +5 | Higher | +1 | Higher | -4 | Lower |
| Halstead Volume | **1237.2308 ✅** | 1583.5069 | 1511.8393 | -346.2761 | Lower | -274.6084 | Lower | +71.6676 | Higher |
| Halstead Effort | **36993.2015 ✅** | 54951.9681 | 53006.0009 | -17958.7666 | Lower | -16012.7994 | Lower | +1945.9673 | Higher |
| SLOC | 68 | 79 | **53 ✅** | -11 | Lower | +15 | Higher | +26 | Higher |
| PLOC | 42 | 52 | **40 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| LLOC | 38 | 37 | **35 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Comments (CLOC) | **20 ✅** | 14 | 5 | +6 | Higher | +15 | Higher | +9 | Higher |
| Blank Lines | 10 | 13 | **8 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03632

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **83.9875 ✅** | 82.6909 | 68.4755 | +1.2967 | Higher | +15.5121 | Higher | +14.2154 | Higher |
| MI (SEI) | **75.4312 ✅** | 75.4191 | 51.3915 | +0.0121 | Higher | +24.0397 | Higher | +24.0276 | Higher |
| Cyclomatic (sum) | 4 | 4 | 9 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 4 | 4 | 11 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | 776.9541 | 776.9541 | 1249.7825 | 0 | Same | -472.8284 | Lower | -472.8284 | Lower |
| Halstead Effort | 12607.8465 | 12607.8465 | 33790.4155 | 0 | Same | -21182.5690 | Lower | -21182.5690 | Lower |
| SLOC | **24 ✅** | 26 | 50 | -2 | Lower | -26 | Lower | -24 | Lower |
| PLOC | 15 | 15 | 35 | 0 | Same | -20 | Lower | -20 | Lower |
| LLOC | 12 | 12 | 28 | 0 | Same | -16 | Lower | -16 | Lower |
| Comments (CLOC) | 4 | 5 | **7 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | **5 ✅** | 6 | 8 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03646

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.9920 ✅** | 65.1733 | 70.6473 | +5.8187 | Higher | +0.3447 | Higher | -5.4740 | Lower |
| MI (SEI) | **54.5368 ✅** | 49.5368 | 51.9274 | +5.0000 | Higher | +2.6095 | Higher | -2.3906 | Lower |
| Cyclomatic (sum) | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 17 | 23 | 17 | -6 | Lower | 0 | Same | +6 | Higher |
| Halstead Volume | **1023.3007 ✅** | 1300.7345 | 1093.4286 | -277.4338 | Lower | -70.1280 | Lower | +207.3058 | Higher |
| Halstead Effort | **26935.9144 ✅** | 42680.3499 | 31490.7444 | -15744.4355 | Lower | -4554.8300 | Lower | +11189.6055 | Higher |
| SLOC | 45 | 58 | 45 | -13 | Lower | 0 | Same | +13 | Higher |
| PLOC | **34 ✅** | 44 | 35 | -10 | Lower | -1 | Lower | +9 | Higher |
| LLOC | **24 ✅** | 29 | 25 | -5 | Lower | -1 | Lower | +4 | Higher |
| Comments (CLOC) | 6 | **10 ✅** | 5 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.2461 | 45.2754 | **63.8324 ✅** | +10.9707 | Higher | -7.5863 | Lower | -18.5570 | Lower |
| MI (SEI) | 44.1797 | 21.8100 | **57.6142 ✅** | +22.3696 | Higher | -13.4345 | Lower | -35.8041 | Lower |
| Cyclomatic (sum) | 15 | 23 | **10 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| Cognitive (sum) | 17 | 22 | 17 | -5 | Lower | 0 | Same | +5 | Higher |
| Halstead Volume | 1506.4117 | 3042.1347 | **1229.6215 ✅** | -1535.7230 | Lower | +276.7902 | Higher | +1812.5132 | Higher |
| Halstead Effort | 45694.4872 | 142250.2174 | **30952.5401 ✅** | -96555.7301 | Lower | +14741.9471 | Higher | +111297.6773 | Higher |
| SLOC | 92 | 129 | **66 ✅** | -37 | Lower | +26 | Higher | +63 | Higher |
| PLOC | 54 | 89 | **34 ✅** | -35 | Lower | +20 | Higher | +55 | Higher |
| LLOC | 35 | 50 | **22 ✅** | -15 | Lower | +13 | Higher | +28 | Higher |
| Comments (CLOC) | **27 ✅** | 22 | 24 | +5 | Higher | +3 | Higher | -2 | Lower |
| Blank Lines | 11 | 19 | **9 ✅** | -8 | Lower | +2 | Higher | +10 | Higher |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 6 | **0 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |

## p03701

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.5223 ✅** | 74.0144 | 71.3151 | +4.5079 | Higher | +7.2071 | Higher | +2.6992 | Higher |
| MI (SEI) | **56.9059 ✅** | 44.6331 | 47.7804 | +12.2728 | Higher | +9.1255 | Higher | -3.1473 | Lower |
| Cyclomatic (sum) | **7 ✅** | 11 | 9 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | **7 ✅** | 15 | 10 | -8 | Lower | -3 | Lower | +5 | Higher |
| Halstead Volume | **721.6932 ✅** | 927.1038 | 1078.0243 | -205.4106 | Lower | -356.3311 | Lower | -150.9205 | Lower |
| Halstead Effort | **17011.3400 ✅** | 26886.0095 | 31946.8352 | -9874.6695 | Lower | -14935.4952 | Lower | -5060.8257 | Lower |
| SLOC | **33 ✅** | 38 | 44 | -5 | Lower | -11 | Lower | -6 | Lower |
| PLOC | **24 ✅** | 30 | 33 | -6 | Lower | -9 | Lower | -3 | Lower |
| LLOC | 15 | 15 | 27 | 0 | Same | -12 | Lower | -12 | Lower |
| Comments (CLOC) | 2 | 1 | **3 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03709

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.2212 | **49.7482 ✅** | 32.2189 | -14.5270 | Lower | +3.0023 | Higher | +17.5292 | Higher |
| MI (SEI) | 0.3628 | **37.7331 ✅** | -0.5840 | -37.3703 | Lower | +0.9469 | Higher | +38.3172 | Higher |
| Cyclomatic (sum) | 32 | **19 ✅** | 33 | +13 | Higher | -1 | Lower | -14 | Lower |
| Cognitive (sum) | 51 | **14 ✅** | 54 | +37 | Higher | -3 | Lower | -40 | Lower |
| Halstead Volume | 6947.2740 | **2453.5220 ✅** | 7446.6798 | +4493.7521 | Higher | -499.4057 | Lower | -4993.1578 | Lower |
| Halstead Effort | 742839.8698 | **110558.7034 ✅** | 686813.0052 | +632281.1664 | Higher | +56026.8646 | Higher | -576254.3018 | Lower |
| SLOC | 162 | **111 ✅** | 188 | +51 | Higher | -26 | Lower | -77 | Lower |
| PLOC | 128 | **58 ✅** | 145 | +70 | Higher | -17 | Lower | -87 | Lower |
| LLOC | 109 | **37 ✅** | 125 | +72 | Higher | -16 | Lower | -88 | Lower |
| Comments (CLOC) | 14 | **39 ✅** | 22 | -25 | Lower | -8 | Lower | +17 | Higher |
| Blank Lines | 22 | **14 ✅** | 24 | +8 | Higher | -2 | Lower | -10 | Lower |
| Functions | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |

## p03719

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.9879 ✅** | 88.4428 | 31.2689 | +2.5451 | Higher | +59.7189 | Higher | +57.1738 | Higher |
| MI (SEI) | 73.8286 | **83.5911 ✅** | -2.7882 | -9.7625 | Lower | +76.6168 | Higher | +86.3793 | Higher |
| Cyclomatic (sum) | 4 | 4 | 46 | 0 | Same | -42 | Lower | -42 | Lower |
| Cognitive (sum) | 3 | 3 | 44 | 0 | Same | -41 | Lower | -41 | Lower |
| Halstead Volume | **495.4196 ✅** | 500 | 5114.4726 | -4.5804 | Lower | -4619.0529 | Lower | -4614.4726 | Lower |
| Halstead Effort | **6270.1547 ✅** | 6750 | 470166.1573 | -479.8453 | Lower | -463896.0026 | Lower | -463416.1573 | Lower |
| SLOC | **18 ✅** | 21 | 187 | -3 | Lower | -169 | Lower | -166 | Lower |
| PLOC | 14 | 14 | 151 | 0 | Same | -137 | Lower | -137 | Lower |
| LLOC | 9 | 9 | 108 | 0 | Same | -99 | Lower | -99 | Lower |
| Comments (CLOC) | 1 | 4 | **18 ✅** | -3 | Lower | -17 | Lower | -14 | Lower |
| Blank Lines | 3 | 3 | 21 | 0 | Same | -18 | Lower | -18 | Lower |
| Functions | 1 | 1 | 12 | 0 | Same | -11 | Lower | -11 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 11 | 0 | Same | -11 | Lower | -11 | Lower |

## p03722

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.1020 | 49.8906 | **58.2294 ✅** | +7.2114 | Higher | -1.1274 | Lower | -8.3388 | Lower |
| MI (SEI) | **33.6721 ✅** | 29.2344 | 32.7661 | +4.4377 | Higher | +0.9060 | Higher | -3.5317 | Lower |
| Cyclomatic (sum) | 17 | 23 | **15 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Cognitive (sum) | 26 | 50 | **19 ✅** | -24 | Lower | +7 | Higher | +31 | Higher |
| Halstead Volume | 2036.3351 | 2378.1618 | **2027.1251 ✅** | -341.8267 | Lower | +9.2100 | Higher | +351.0367 | Higher |
| Halstead Effort | 86035.1596 | 102260.9579 | **80172.7987 ✅** | -16225.7983 | Lower | +5862.3608 | Higher | +22088.1591 | Higher |
| SLOC | 77 | 105 | **74 ✅** | -28 | Lower | +3 | Higher | +31 | Higher |
| PLOC | **58 ✅** | 78 | 59 | -20 | Lower | -1 | Lower | +19 | Higher |
| LLOC | 39 | 53 | **35 ✅** | -14 | Lower | +4 | Higher | +18 | Higher |
| Comments (CLOC) | 9 | **19 ✅** | 7 | -10 | Lower | +2 | Higher | +12 | Higher |
| Blank Lines | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03736

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.7495 | **62.3797 ✅** | 51.8259 | -5.6302 | Lower | +4.9236 | Higher | +10.5538 | Higher |
| MI (SEI) | 28.0181 | **48.4529 ✅** | 20.5029 | -20.4348 | Lower | +7.5152 | Higher | +27.9500 | Higher |
| Cyclomatic (sum) | 10 | 10 | 12 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **6 ✅** | 12 | 7 | -6 | Lower | -1 | Lower | +5 | Higher |
| Halstead Volume | 2852.9540 | **1789.4963 ✅** | 3412.9734 | +1063.4577 | Higher | -560.0194 | Lower | -1623.4771 | Lower |
| Halstead Effort | 140908.0951 | **61668.7960 ✅** | 178703.2875 | +79239.2991 | Higher | -37795.1924 | Lower | -117034.4915 | Lower |
| SLOC | 78 | **64 ✅** | 97 | +14 | Higher | -19 | Lower | -33 | Lower |
| PLOC | 55 | **41 ✅** | 73 | +14 | Higher | -18 | Lower | -32 | Lower |
| LLOC | 49 | **32 ✅** | 56 | +17 | Higher | -7 | Lower | -24 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 7 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 17 | **11 ✅** | 17 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p03762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.2546 ✅** | 68.1476 | 68.7625 | +2.1070 | Higher | +1.4922 | Higher | -0.6149 | Lower |
| MI (SEI) | **58.2016 ✅** | 57.3619 | 52.7276 | +0.8396 | Higher | +5.4739 | Higher | +4.6343 | Higher |
| Cyclomatic (sum) | 6 | 6 | 8 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **1407.4106 ✅** | 1429.0581 | 1602.8300 | -21.6475 | Lower | -195.4194 | Lower | -173.7719 | Lower |
| Halstead Effort | 39189.1041 | **34801.7667 ✅** | 54095.5111 | +4387.3374 | Higher | -14906.4070 | Lower | -19293.7444 | Lower |
| SLOC | **45 ✅** | 51 | 46 | -6 | Lower | -1 | Lower | +5 | Higher |
| PLOC | 31 | 31 | 34 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 27 | **22 ✅** | 27 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 7 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03766

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.9411 | **65.2356 ✅** | 61.2601 | -4.2945 | Lower | -0.3190 | Lower | +3.9755 | Higher |
| MI (SEI) | 43.2372 | **47.4658 ✅** | 39.3763 | -4.2287 | Lower | +3.8608 | Higher | +8.0895 | Higher |
| Cyclomatic (sum) | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 1951.2472 | **1588.9536 ✅** | 2016.5341 | +362.2936 | Higher | -65.2869 | Lower | -427.5805 | Lower |
| Halstead Effort | 102022.3521 | **61855.6926 ✅** | 89987.8348 | +40166.6596 | Higher | +12034.5173 | Higher | -28132.1422 | Lower |
| SLOC | 69 | **59 ✅** | 66 | +10 | Higher | +3 | Higher | -7 | Lower |
| PLOC | 45 | **37 ✅** | 46 | +8 | Higher | -1 | Lower | -9 | Lower |
| LLOC | 39 | **34 ✅** | 35 | +5 | Higher | +4 | Higher | -1 | Lower |
| Comments (CLOC) | **12 ✅** | 9 | 8 | +3 | Higher | +4 | Higher | +1 | Higher |
| Blank Lines | 14 | 13 | **12 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |

## p03797

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 83.3388 | **85.1094 ✅** | 82.6034 | -1.7706 | Lower | +0.7354 | Higher | +2.5060 | Higher |
| MI (SEI) | 69.7282 | 69.2179 | **72.0584 ✅** | +0.5103 | Higher | -2.3301 | Lower | -2.8405 | Lower |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **583.4588 ✅** | 599.0871 | 672.0903 | -15.6284 | Lower | -88.6315 | Lower | -73.0031 | Lower |
| Halstead Effort | **10225.8828 ✅** | 10783.5685 | 14785.9855 | -557.6857 | Lower | -4560.1027 | Lower | -4002.4170 | Lower |
| SLOC | 27 | **24 ✅** | 27 | +3 | Higher | 0 | Same | -3 | Lower |
| PLOC | 19 | 19 | 19 | 0 | Same | 0 | Same | 0 | Same |
| LLOC | 8 | 8 | 16 | 0 | Same | -8 | Lower | -8 | Lower |
| Comments (CLOC) | 3 | 2 | **4 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03808

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.4688 | **58.7665 ✅** | 54.1965 | -1.2977 | Lower | +3.2723 | Higher | +4.5700 | Higher |
| MI (SEI) | 39.0817 | **40.3955 ✅** | 31.9866 | -1.3138 | Lower | +7.0952 | Higher | +8.4090 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 17 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 15 | 15 | 30 | 0 | Same | -15 | Lower | -15 | Lower |
| Halstead Volume | 1914 | **1758.5314 ✅** | 2190 | +155.4686 | Higher | -276 | Lower | -431.4686 | Lower |
| Halstead Effort | 131236.6000 | **76872.9441 ✅** | 148920 | +54363.6559 | Higher | -17683.4000 | Lower | -72047.0559 | Lower |
| SLOC | 79 | **76 ✅** | 90 | +3 | Higher | -11 | Lower | -14 | Lower |
| PLOC | 54 | **50 ✅** | 66 | +4 | Higher | -12 | Lower | -16 | Lower |
| LLOC | 47 | **38 ✅** | 53 | +9 | Higher | -6 | Lower | -15 | Lower |
| Comments (CLOC) | **14 ✅** | 13 | 13 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 12 | 13 | 12 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 6 | **2 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p03815

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.0577 | 87.0577 | 86.9297 | 0 | Same | +0.1281 | Higher | +0.1281 | Higher |
| MI (SEI) | 76.8424 | 72.3596 | **80.2216 ✅** | +4.4829 | Higher | -3.3791 | Lower | -7.8620 | Lower |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 491.5424 | 491.5424 | 503.8010 | 0 | Same | -12.2586 | Lower | -12.2586 | Lower |
| Halstead Effort | 7938.4099 | 7938.4099 | 8564.6177 | 0 | Same | -626.2078 | Lower | -626.2078 | Lower |
| SLOC | 23 | 23 | 23 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 16 | 16 | **15 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 3 | 2 | **4 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03817

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.0422 ✅** | 86.7174 | 85.5367 | +3.3248 | Higher | +4.5055 | Higher | +1.1807 | Higher |
| MI (SEI) | 54.6099 | 71.8685 | **74.2488 ✅** | -17.2587 | Lower | -19.6390 | Lower | -2.3803 | Lower |
| Cyclomatic (sum) | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **502.1132 ✅** | 524.7928 | 551.8279 | -22.6795 | Lower | -49.7146 | Lower | -27.0351 | Lower |
| Halstead Effort | 7632.1208 | **7222.1479 ✅** | 10719.2561 | +409.9729 | Higher | -3087.1353 | Lower | -3497.1082 | Lower |
| SLOC | **19 ✅** | 23 | 24 | -4 | Lower | -5 | Lower | -1 | Lower |
| PLOC | **16 ✅** | 17 | 17 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **7 ✅** | 8 | 14 | -1 | Lower | -7 | Lower | -6 | Lower |
| Comments (CLOC) | 0 | 2 | **3 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.1445 ✅** | 51.5864 | 56.1515 | +5.5581 | Higher | +0.9930 | Higher | -4.5652 | Lower |
| MI (SEI) | **32.3548 ✅** | 27.0431 | 28.0213 | +5.3117 | Higher | +4.3336 | Higher | -0.9781 | Lower |
| Cyclomatic (sum) | **15 ✅** | 20 | 16 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | 25 | 33 | **20 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| Halstead Volume | **2298.3198 ✅** | 2766.7671 | 2661.5258 | -468.4473 | Lower | -363.2060 | Lower | +105.2414 | Higher |
| Halstead Effort | **88485.3114 ✅** | 116639.7284 | 142067.9286 | -28154.4170 | Lower | -53582.6172 | Lower | -25428.2002 | Lower |
| SLOC | 76 | 94 | 76 | -18 | Lower | 0 | Same | +18 | Higher |
| PLOC | 59 | 68 | 59 | -9 | Lower | 0 | Same | +9 | Higher |
| LLOC | **49 ✅** | 55 | 50 | -6 | Lower | -1 | Lower | +5 | Higher |
| Comments (CLOC) | 8 | **12 ✅** | 6 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | **9 ✅** | 14 | 11 | -5 | Lower | -2 | Lower | +3 | Higher |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |

## p03838

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1041 | **85.0313 ✅** | 69.6693 | -10.9272 | Lower | +4.4347 | Higher | +15.3620 | Higher |
| MI (SEI) | 55.9268 | **85.6995 ✅** | 53.8712 | -29.7727 | Lower | +2.0555 | Higher | +31.8282 | Higher |
| Cyclomatic (sum) | 9 | **3 ✅** | 9 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 12 | **2 ✅** | 15 | +10 | Higher | -3 | Lower | -13 | Lower |
| Halstead Volume | 918.1417 | **585.0582 ✅** | 1204.5925 | +333.0834 | Higher | -286.4509 | Lower | -619.5343 | Lower |
| Halstead Effort | 27748.2813 | **8320.8281 ✅** | 37722.7659 | +19427.4532 | Higher | -9974.4846 | Lower | -29401.9378 | Lower |
| SLOC | 39 | **25 ✅** | 47 | +14 | Higher | -8 | Lower | -22 | Lower |
| PLOC | 31 | **12 ✅** | 35 | +19 | Higher | -4 | Lower | -23 | Lower |
| LLOC | 26 | **9 ✅** | 29 | +17 | Higher | -3 | Lower | -20 | Lower |
| Comments (CLOC) | 4 | **8 ✅** | 7 | -4 | Lower | -3 | Lower | +1 | Higher |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03845

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.1297 | **76.6696 ✅** | 70.9382 | -3.5399 | Lower | +2.1915 | Higher | +5.7314 | Higher |
| MI (SEI) | 47.4956 | **60.6529 ✅** | 49.7672 | -13.1573 | Lower | -2.2715 | Lower | +10.8858 | Higher |
| Cyclomatic (sum) | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 1321.6751 | **1072.3128 ✅** | 1323.5366 | +249.3623 | Higher | -1.8615 | Lower | -251.2238 | Lower |
| Halstead Effort | 27466.0608 | **21446.2560 ✅** | 28526.9406 | +6019.8048 | Higher | -1060.8798 | Lower | -7080.6846 | Lower |
| SLOC | 39 | **34 ✅** | 44 | +5 | Higher | -5 | Lower | -10 | Lower |
| PLOC | 29 | **25 ✅** | 31 | +4 | Higher | -2 | Lower | -6 | Lower |
| LLOC | 23 | **21 ✅** | 26 | +2 | Higher | -3 | Lower | -5 | Lower |
| Comments (CLOC) | 2 | 4 | 4 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03865

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.8267 | **84.5164 ✅** | 75.4594 | -1.6897 | Lower | +7.3673 | Higher | +9.0570 | Higher |
| MI (SEI) | **85.2701 ✅** | 82.1136 | 66.6013 | +3.1565 | Higher | +18.6689 | Higher | +15.5123 | Higher |
| Cyclomatic (sum) | 4 | 4 | 8 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 3 | 3 | 9 | 0 | Same | -6 | Lower | -6 | Lower |
| Halstead Volume | **437.5941 ✅** | 486.2570 | 801.8106 | -48.6629 | Lower | -364.2165 | Lower | -315.5535 | Lower |
| Halstead Effort | **5951.2795 ✅** | 7502.2509 | 20324.1544 | -1550.9714 | Lower | -14372.8749 | Lower | -12821.9035 | Lower |
| SLOC | 31 | **27 ✅** | 38 | +4 | Higher | -7 | Lower | -11 | Lower |
| PLOC | **14 ✅** | 15 | 25 | -1 | Lower | -11 | Lower | -10 | Lower |
| LLOC | 9 | 9 | 15 | 0 | Same | -6 | Lower | -6 | Lower |
| Comments (CLOC) | **12 ✅** | 7 | 8 | +5 | Higher | +4 | Higher | -1 | Lower |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03899

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.1995 | **54.0055 ✅** | 52.7122 | -1.8060 | Lower | -0.5127 | Lower | +1.2933 | Higher |
| MI (SEI) | 32.2194 | **38.6526 ✅** | 25.4078 | -6.4332 | Lower | +6.8116 | Higher | +13.2448 | Higher |
| Cyclomatic (sum) | 13 | 13 | 14 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 23 | 31 | **20 ✅** | -8 | Lower | +3 | Higher | +11 | Higher |
| Halstead Volume | 2374.1105 | **2291.2883 ✅** | 2472.1983 | +82.8222 | Higher | -98.0878 | Lower | -180.9101 | Lower |
| Halstead Effort | 117396.7209 | **74466.8695 ✅** | 99986.6885 | +42929.8514 | Higher | +17410.0324 | Higher | -25519.8190 | Lower |
| SLOC | 105 | **95 ✅** | 99 | +10 | Higher | +6 | Higher | -4 | Lower |
| PLOC | 70 | **58 ✅** | 73 | +12 | Higher | -3 | Lower | -15 | Lower |
| LLOC | 55 | **43 ✅** | 52 | +12 | Higher | +3 | Higher | -9 | Lower |
| Comments (CLOC) | 20 | **24 ✅** | 10 | -4 | Lower | +10 | Higher | +14 | Higher |
| Blank Lines | 16 | **13 ✅** | 17 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03901

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.9525 | **82.8451 ✅** | 45.6624 | -9.8925 | Lower | +27.2902 | Higher | +37.1827 | Higher |
| MI (SEI) | 64.6155 | **69.3407 ✅** | 24.5585 | -4.7252 | Lower | +40.0570 | Higher | +44.7821 | Higher |
| Cyclomatic (sum) | 4 | 4 | 22 | 0 | Same | -18 | Lower | -18 | Lower |
| Cognitive (sum) | **1 ✅** | 2 | 18 | -1 | Lower | -17 | Lower | -16 | Lower |
| Halstead Volume | 915.1977 | **754.2508 ✅** | 3024.1085 | +160.9469 | Higher | -2108.9107 | Lower | -2269.8577 | Lower |
| Halstead Effort | 24161.2194 | **15740.8863 ✅** | 135765.0227 | +8420.3331 | Higher | -111603.8033 | Lower | -120024.1364 | Lower |
| SLOC | 45 | **26 ✅** | 128 | +19 | Higher | -83 | Lower | -102 | Lower |
| PLOC | 26 | **18 ✅** | 89 | +8 | Higher | -63 | Lower | -71 | Lower |
| LLOC | 21 | **9 ✅** | 54 | +12 | Higher | -33 | Lower | -45 | Lower |
| Comments (CLOC) | 11 | 3 | **26 ✅** | +8 | Higher | -15 | Lower | -23 | Lower |
| Blank Lines | 8 | **5 ✅** | 13 | +3 | Higher | -5 | Lower | -8 | Lower |
| Functions | 2 | 2 | 5 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 0 | 5 | 0 | Same | -5 | Lower | -5 | Lower |
| Exits | 2 | **1 ✅** | 6 | +1 | Higher | -4 | Lower | -5 | Lower |

## p03914

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.9429 | **76.7975 ✅** | 66.7058 | -8.8546 | Lower | +1.2371 | Higher | +10.0917 | Higher |
| MI (SEI) | 58.3049 | **81.6540 ✅** | 56.7277 | -23.3491 | Lower | +1.5772 | Higher | +24.9263 | Higher |
| Cyclomatic (sum) | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 1300.2379 | **526.1548 ✅** | 1393.7652 | +774.0832 | Higher | -93.5272 | Lower | -867.6104 | Lower |
| Halstead Effort | 41867.6618 | **6213.6375 ✅** | 43713.5443 | +35654.0243 | Higher | -1845.8826 | Lower | -37499.9068 | Lower |
| SLOC | 54 | **43 ✅** | 57 | +11 | Higher | -3 | Lower | -14 | Lower |
| PLOC | 31 | **12 ✅** | 33 | +19 | Higher | -2 | Lower | -21 | Lower |
| LLOC | 31 | **8 ✅** | 33 | +23 | Higher | -2 | Lower | -25 | Lower |
| Comments (CLOC) | 14 | **25 ✅** | 15 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03929

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.6851 | **61.0651 ✅** | 60.7535 | -1.3800 | Lower | -1.0684 | Lower | +0.3115 | Higher |
| MI (SEI) | 41.0513 | 38.6246 | **42.1854 ✅** | +2.4266 | Higher | -1.1341 | Lower | -3.5608 | Lower |
| Cyclomatic (sum) | 18 | **12 ✅** | 14 | +6 | Higher | +4 | Higher | -2 | Lower |
| Cognitive (sum) | 19 | **13 ✅** | 24 | +6 | Higher | -5 | Lower | -11 | Lower |
| Halstead Volume | 1595.3683 | 1595.3815 | **1550.4786 ✅** | -0.0131 | Lower | +44.8898 | Higher | +44.9029 | Higher |
| Halstead Effort | 67005.4704 | **55397.5219 ✅** | 61274.9135 | +11607.9485 | Higher | +5730.5570 | Higher | -5877.3916 | Lower |
| SLOC | 70 | 70 | 70 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | **49 ✅** | 50 | 50 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **37 ✅** | 41 | 48 | -4 | Lower | -11 | Lower | -7 | Lower |
| Comments (CLOC) | 11 | 8 | 11 | +3 | Higher | 0 | Same | -3 | Lower |
| Blank Lines | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 4 | **0 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |

## p03932

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.5379 | **48.5155 ✅** | 44.7133 | -5.9776 | Lower | -2.1753 | Lower | +3.8023 | Higher |
| MI (SEI) | 14.6447 | **26.8849 ✅** | 17.0569 | -12.2402 | Lower | -2.4122 | Lower | +9.8281 | Higher |
| Cyclomatic (sum) | 19 | 23 | 19 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 19 | 63 | **18 ✅** | -44 | Lower | +1 | Higher | +45 | Higher |
| Halstead Volume | 4091.7968 | **2837.7202 ✅** | 4462.5276 | +1254.0767 | Higher | -370.7308 | Lower | -1624.8074 | Lower |
| Halstead Effort | 285147.0926 | **132475.2027 ✅** | 376602.3294 | +152671.8899 | Higher | -91455.2368 | Lower | -244127.1267 | Lower |
| SLOC | 147 | **108 ✅** | 125 | +39 | Higher | +22 | Higher | -17 | Lower |
| PLOC | 108 | **71 ✅** | 91 | +37 | Higher | +17 | Higher | -20 | Lower |
| LLOC | 62 | **56 ✅** | 67 | +6 | Higher | -5 | Lower | -11 | Lower |
| Comments (CLOC) | **20 ✅** | 19 | 16 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | 19 | 18 | 18 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 4 | **1 ✅** | 3 | +3 | Higher | +1 | Higher | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **0 ✅** | 6 | +5 | Higher | -1 | Lower | -6 | Lower |

## p03951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.3039 ✅** | 78.0052 | 66.3805 | +2.2986 | Higher | +13.9234 | Higher | +11.6247 | Higher |
| MI (SEI) | **74.1329 ✅** | 56.0581 | 50.5089 | +18.0748 | Higher | +23.6240 | Higher | +5.5492 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 11 | -2 | Lower | -7 | Lower | -5 | Lower |
| Cognitive (sum) | **3 ✅** | 8 | 15 | -5 | Lower | -12 | Lower | -7 | Lower |
| Halstead Volume | **710.8407 ✅** | 833.1883 | 1271.8301 | -122.3476 | Lower | -560.9895 | Lower | -438.6418 | Lower |
| Halstead Effort | **13598.6913 ✅** | 17160.4750 | 46058.4200 | -3561.7837 | Lower | -32459.7287 | Lower | -28897.9450 | Lower |
| SLOC | **31 ✅** | 33 | 55 | -2 | Lower | -24 | Lower | -22 | Lower |
| PLOC | **19 ✅** | 26 | 37 | -7 | Lower | -18 | Lower | -11 | Lower |
| LLOC | **15 ✅** | 21 | 30 | -6 | Lower | -15 | Lower | -9 | Lower |
| Comments (CLOC) | 7 | 2 | **9 ✅** | +5 | Higher | -2 | Lower | -7 | Lower |
| Blank Lines | 5 | 5 | 9 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p03977

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.7347 | 53.1977 | **56.7806 ✅** | +2.5370 | Higher | -1.0459 | Lower | -3.5829 | Lower |
| MI (SEI) | 36.7661 | 33.4364 | **37.7289 ✅** | +3.3296 | Higher | -0.9628 | Lower | -4.2924 | Lower |
| Cyclomatic (sum) | 15 | 25 | **12 ✅** | -10 | Lower | +3 | Higher | +13 | Higher |
| Cognitive (sum) | 17 | 24 | **13 ✅** | -7 | Lower | +4 | Higher | +11 | Higher |
| Halstead Volume | **1842.9499 ✅** | 1998 | 1986.1154 | -155.0501 | Lower | -143.1655 | Lower | +11.8846 | Higher |
| Halstead Effort | 103442.9925 | 107606.5714 | **85824.2587 ✅** | -4163.5790 | Lower | +17618.7338 | Higher | +21782.3127 | Higher |
| SLOC | 89 | 88 | **85 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| PLOC | 63 | **62 ✅** | 63 | +1 | Higher | 0 | Same | -1 | Lower |
| LLOC | 59 | **42 ✅** | 62 | +17 | Higher | -3 | Lower | -20 | Lower |
| Comments (CLOC) | **16 ✅** | 15 | 15 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 10 | 13 | **9 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03996

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.1769 | **57.6647 ✅** | 52.3214 | -4.4878 | Lower | +0.8555 | Higher | +5.3433 | Higher |
| MI (SEI) | 31.1832 | **45.6265 ✅** | 25.4852 | -14.4433 | Lower | +5.6980 | Higher | +20.1413 | Higher |
| Cyclomatic (sum) | **14 ✅** | 17 | 17 | -3 | Lower | -3 | Lower | 0 | Same |
| Cognitive (sum) | **17 ✅** | 23 | 21 | -6 | Lower | -4 | Lower | +2 | Higher |
| Halstead Volume | 2488.3095 | **1446.5567 ✅** | 2568.7772 | +1041.7527 | Higher | -80.4677 | Lower | -1122.2205 | Lower |
| Halstead Effort | 99797.0927 | **50956.1272 ✅** | 118277.9187 | +48840.9655 | Higher | -18480.8260 | Lower | -67321.7915 | Lower |
| SLOC | 96 | **83 ✅** | 96 | +13 | Higher | 0 | Same | -13 | Lower |
| PLOC | 71 | **49 ✅** | 74 | +22 | Higher | -3 | Lower | -25 | Lower |
| LLOC | 55 | **36 ✅** | 56 | +19 | Higher | -1 | Lower | -20 | Lower |
| Comments (CLOC) | 15 | **23 ✅** | 10 | -8 | Lower | +5 | Higher | +13 | Higher |
| Blank Lines | 13 | **11 ✅** | 12 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p04005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.7227 | **77.3497 ✅** | 69.1442 | -0.6270 | Lower | +7.5786 | Higher | +8.2056 | Higher |
| MI (SEI) | **66.0920 ✅** | 64.5917 | 61.2417 | +1.5004 | Higher | +4.8503 | Higher | +3.3500 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 9 | **5 ✅** | 12 | +4 | Higher | -3 | Lower | -7 | Lower |
| Halstead Volume | 889.2935 | **823.9246 ✅** | 1033.2181 | +65.3689 | Higher | -143.9246 | Lower | -209.2934 | Lower |
| Halstead Effort | 25678.3491 | **18909.0700 ✅** | 28528.2987 | +6769.2791 | Higher | -2849.9496 | Lower | -9619.2287 | Lower |
| SLOC | 34 | 34 | 51 | 0 | Same | -17 | Lower | -17 | Lower |
| PLOC | 24 | **23 ✅** | 30 | +1 | Higher | -6 | Lower | -7 | Lower |
| LLOC | 20 | **19 ✅** | 25 | +1 | Higher | -5 | Lower | -6 | Lower |
| Comments (CLOC) | 6 | 5 | **14 ✅** | +1 | Higher | -8 | Lower | -9 | Lower |
| Blank Lines | **4 ✅** | 6 | 7 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p04013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.6361 | **60.0714 ✅** | 50.7480 | -2.4353 | Lower | +6.8881 | Higher | +9.3234 | Higher |
| MI (SEI) | 40.9462 | **41.4647 ✅** | 29.8179 | -0.5185 | Lower | +11.1283 | Higher | +11.6468 | Higher |
| Cyclomatic (sum) | 19 | **13 ✅** | 23 | +6 | Higher | -4 | Lower | -10 | Lower |
| Cognitive (sum) | 24 | 27 | 24 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 2073.3186 | **2022.3814 ✅** | 2943.2809 | +50.9372 | Higher | -869.9623 | Lower | -920.8995 | Lower |
| Halstead Effort | 111111.0286 | **89883.6192 ✅** | 190971.0162 | +21227.4094 | Higher | -79859.9877 | Lower | -101087.3970 | Lower |
| SLOC | 72 | **68 ✅** | 93 | +4 | Higher | -21 | Lower | -25 | Lower |
| PLOC | 48 | 48 | 65 | 0 | Same | -17 | Lower | -17 | Lower |
| LLOC | **33 ✅** | 40 | 44 | -7 | Lower | -11 | Lower | -4 | Lower |
| Comments (CLOC) | 14 | 11 | **16 ✅** | +3 | Higher | -2 | Lower | -5 | Lower |
| Blank Lines | 10 | **9 ✅** | 13 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p04020

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.9655 ✅** | 76.2288 | 75.0614 | +1.7367 | Higher | +2.9041 | Higher | +1.1674 | Higher |
| MI (SEI) | 68.0692 | **69.9582 ✅** | 64.6038 | -1.8890 | Lower | +3.4654 | Higher | +5.3544 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 8 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **839.5814 ✅** | 855.4067 | 940.5726 | -15.8253 | Lower | -100.9911 | Lower | -85.1659 | Lower |
| Halstead Effort | 30015.0357 | **27703.5119 ✅** | 34985.2095 | +2311.5238 | Higher | -4970.1739 | Lower | -7281.6976 | Lower |
| SLOC | **33 ✅** | 36 | 37 | -3 | Lower | -4 | Lower | -1 | Lower |
| PLOC | **21 ✅** | 22 | 24 | -1 | Lower | -3 | Lower | -2 | Lower |
| LLOC | 20 | 21 | 20 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 6 | **9 ✅** | 7 | -3 | Lower | -1 | Lower | +2 | Higher |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p04022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 27.8449 | **33.3490 ✅** | 29.5614 | -5.5042 | Lower | -1.7165 | Lower | +3.7877 | Higher |
| MI (SEI) | -9.8755 | **1.5967 ✅** | -16.4289 | -11.4722 | Lower | +6.5534 | Higher | +18.0256 | Higher |
| Cyclomatic (sum) | 40 | 39 | 39 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 90 | 88 | **86 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Halstead Volume | 5845.0446 | **4520.0619 ✅** | 5834.3230 | +1324.9827 | Higher | +10.7216 | Higher | -1314.2610 | Lower |
| Halstead Effort | 488541.0412 | **342291.9629 ✅** | 502214.8154 | +146249.0783 | Higher | -13673.7743 | Lower | -159922.8525 | Lower |
| SLOC | 241 | **189 ✅** | 220 | +52 | Higher | +21 | Higher | -31 | Lower |
| PLOC | 193 | **147 ✅** | 186 | +46 | Higher | +7 | Higher | -39 | Lower |
| LLOC | 154 | **110 ✅** | 155 | +44 | Higher | -1 | Lower | -45 | Lower |
| Comments (CLOC) | 20 | **22 ✅** | 6 | -2 | Lower | +14 | Higher | +16 | Higher |
| Blank Lines | 30 | **20 ✅** | 29 | +10 | Higher | +1 | Higher | -9 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |

## p04024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.7986 ✅** | 38.8107 | 35.8487 | +0.9880 | Higher | +3.9499 | Higher | +2.9619 | Higher |
| MI (SEI) | **9.4602 ✅** | 2.5081 | 0.3699 | +6.9521 | Higher | +9.0903 | Higher | +2.1382 | Higher |
| Cyclomatic (sum) | **34 ✅** | 38 | 41 | -4 | Lower | -7 | Lower | -3 | Lower |
| Cognitive (sum) | **47 ✅** | 66 | 56 | -19 | Lower | -9 | Lower | +10 | Higher |
| Halstead Volume | **4155.0063 ✅** | 4607.5186 | 4941.4652 | -452.5123 | Lower | -786.4589 | Lower | -333.9466 | Lower |
| Halstead Effort | **296621.2849 ✅** | 318371.9823 | 437319.6692 | -21750.6975 | Lower | -140698.3843 | Lower | -118947.6868 | Lower |
| SLOC | 140 | **136 ✅** | 153 | +4 | Higher | -13 | Lower | -17 | Lower |
| PLOC | **106 ✅** | 108 | 123 | -2 | Lower | -17 | Lower | -15 | Lower |
| LLOC | 83 | 83 | 92 | 0 | Same | -9 | Lower | -9 | Lower |
| Comments (CLOC) | **15 ✅** | 8 | 11 | +7 | Higher | +4 | Higher | -3 | Lower |
| Blank Lines | 19 | 20 | 19 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **3 ✅** | 7 | +1 | Higher | -3 | Lower | -4 | Lower |

## p04030

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.5758 | **81.4424 ✅** | 69.0207 | -9.8666 | Lower | +2.5551 | Higher | +12.4217 | Higher |
| MI (SEI) | 55.4448 | **80.3929 ✅** | 57.6391 | -24.9481 | Lower | -2.1944 | Lower | +22.7537 | Higher |
| Cyclomatic (sum) | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 14 | **7 ✅** | 14 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 1071.6827 | **494.8981 ✅** | 1105.8853 | +576.7846 | Higher | -34.2026 | Lower | -610.9872 | Lower |
| Halstead Effort | 26327.6709 | **6309.9504 ✅** | 28384.3895 | +20017.7205 | Higher | -2056.7187 | Lower | -22074.4391 | Lower |
| SLOC | 44 | **32 ✅** | 51 | +12 | Higher | -7 | Lower | -19 | Lower |
| PLOC | 33 | **18 ✅** | 34 | +15 | Higher | -1 | Lower | -16 | Lower |
| LLOC | 27 | **11 ✅** | 29 | +16 | Higher | -2 | Lower | -18 | Lower |
| Comments (CLOC) | 6 | 10 | **11 ✅** | -4 | Lower | -5 | Lower | -1 | Lower |
| Blank Lines | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.0361 | **59.1129 ✅** | 54.9312 | -0.0767 | Lower | 4.1049 | Higher | 4.1817 | Higher |
| MI (SEI) | 36.7958 | **37.8259 ✅** | 29.7901 | -1.0301 | Lower | 7.0057 | Higher | 8.0358 | Higher |
| Cyclomatic (sum) | **17.6867 ✅** | 17.7533 | 20.5133 | -0.0667 | Lower | -2.8267 | Lower | -2.7600 | Lower |
| Cognitive (sum) | **22.6400 ✅** | 23.0533 | 26.1200 | -0.4133 | Lower | -3.4800 | Lower | -3.0667 | Lower |
| Halstead Volume | 2619.5529 | **2433.0887 ✅** | 3097.4951 | 186.4641 | Higher | -477.9422 | Lower | -664.4064 | Lower |
| Halstead Effort | 168419.2300 | **130573.8437 ✅** | 223420.1453 | 37845.3863 | Higher | -55000.9153 | Lower | -92846.3016 | Lower |
| SLOC | 89.1000 | **86.7267 ✅** | 102.5067 | 2.3733 | Higher | -13.4067 | Lower | -15.7800 | Lower |
| PLOC | 68.2233 | **63.9733 ✅** | 80.8633 | 4.2500 | Higher | -12.6400 | Lower | -16.8900 | Lower |
| LLOC | 49.3567 | **43.9967 ✅** | 59.5433 | 5.3600 | Higher | -10.1867 | Lower | -15.5467 | Lower |
| Comments (CLOC) | 10.1300 | **11.7033 ✅** | 9.2633 | -1.5733 | Lower | 0.8667 | Higher | 2.4400 | Higher |
| Blank Lines | **11.7567 ✅** | 11.8800 | 13.1333 | -0.1233 | Lower | -1.3767 | Lower | -1.2533 | Lower |
| Functions | 2.3500 | **2.1267 ✅** | 2.9100 | 0.2233 | Higher | -0.5600 | Lower | -0.7833 | Lower |
| Closures | 0.4833 | 0.4600 | **0.3167 ✅** | 0.0233 | Higher | 0.1667 | Higher | 0.1433 | Higher |
| Exits | **2.0033 ✅** | 2.1800 | 2.7733 | -0.1767 | Lower | -0.7700 | Lower | -0.5933 | Lower |

