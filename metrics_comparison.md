# Rust Metrics Comparison

Source: `r_code/metrics.json`

## p00001

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.3510 | 67.0060 | **73.4711 ✅** | +2.3450 | Higher | -4.1200 | Lower | -6.4651 | Lower |
| MI (SEI) | 65.2704 | 64.0318 | **68.6319 ✅** | +1.2386 | Higher | -3.3615 | Lower | -4.6001 | Lower |
| Cyclomatic (sum) | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 857.3785 | 921.4989 | **835.7781 ✅** | -64.1205 | Lower | +21.6004 | Higher | +85.7208 | Higher |
| Halstead Effort | 18948.0642 | 22845.4947 | **18648.2991 ✅** | -3897.4305 | Lower | +299.7651 | Higher | +4197.1955 | Higher |
| SLOC | 55 | 63 | **43 ✅** | -8 | Lower | +12 | Higher | +20 | Higher |
| PLOC | 28 | 29 | **24 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| LLOC | 22 | 23 | 22 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 20 | **27 ✅** | 13 | -7 | Lower | +7 | Higher | +14 | Higher |
| Blank Lines | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00002

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8536 | 66.0163 | **70.1449 ✅** | -6.1627 | Lower | -10.2913 | Lower | -4.1286 | Lower |
| MI (SEI) | 57.7334 | 63.0034 | **65.0374 ✅** | -5.2700 | Lower | -7.3040 | Lower | -2.0340 | Lower |
| Cyclomatic (sum) | 13 | 12 | **11 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 944.5291 | **813.9397 ✅** | 829.8336 | +130.5894 | Higher | +114.6955 | Higher | -15.8939 | Lower |
| Halstead Effort | 18610.7214 | **13038.6881 ✅** | 13062.1957 | +5572.0333 | Higher | +5548.5258 | Higher | -23.5076 | Lower |
| SLOC | 88 | 64 | **50 ✅** | +24 | Higher | +38 | Higher | +14 | Higher |
| PLOC | 30 | 30 | **28 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| LLOC | 13 | 13 | **12 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | **49 ✅** | 27 | 16 | +22 | Higher | +33 | Higher | +11 | Higher |
| Blank Lines | 9 | 9 | **8 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p00003

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.1106 ✅** | 62.4276 | 50.9688 | +0.6830 | Higher | +12.1418 | Higher | +11.4588 | Higher |
| MI (SEI) | 50.3183 | **56.6047 ✅** | 21.3071 | -6.2864 | Lower | +29.0112 | Higher | +35.2976 | Higher |
| Cyclomatic (sum) | 17 | **9 ✅** | 33 | +8 | Higher | -16 | Lower | -24 | Lower |
| Cognitive (sum) | 12 | **10 ✅** | 45 | +2 | Higher | -33 | Lower | -35 | Lower |
| Halstead Volume | 1469.8883 | **1341.2426 ✅** | 2488.9406 | +128.6457 | Higher | -1019.0523 | Lower | -1147.6980 | Lower |
| Halstead Effort | 49312.3811 | **30703.5809 ✅** | 173460.0127 | +18608.8002 | Higher | -124147.6316 | Lower | -142756.4318 | Lower |
| SLOC | **59 ✅** | 71 | 84 | -12 | Lower | -25 | Lower | -13 | Lower |
| PLOC | 39 | **33 ✅** | 71 | +6 | Higher | -32 | Lower | -38 | Lower |
| LLOC | 20 | 20 | 44 | 0 | Same | -24 | Lower | -24 | Lower |
| Comments (CLOC) | 13 | **28 ✅** | 6 | -15 | Lower | +7 | Higher | +22 | Higher |
| Blank Lines | 8 | 10 | **7 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 4 | **0 ✅** | 4 | +4 | Higher | 0 | Same | -4 | Lower |
| Exits | 5 | 1 | 1 | +4 | Higher | +4 | Higher | 0 | Same |

## p00004

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.0685 | 53.8538 | **66.8605 ✅** | +5.2147 | Higher | -7.7920 | Lower | -13.0067 | Lower |
| MI (SEI) | 46.2589 | 40.0327 | **56.1252 ✅** | +6.2263 | Higher | -9.8662 | Lower | -16.0925 | Lower |
| Cyclomatic (sum) | 24 | 32 | **9 ✅** | -8 | Lower | +15 | Higher | +23 | Higher |
| Cognitive (sum) | 15 | 19 | **9 ✅** | -4 | Lower | +6 | Higher | +10 | Higher |
| Halstead Volume | 1912.7296 | 2312.9876 | **1341.4776 ✅** | -400.2580 | Lower | +571.2519 | Higher | +971.5099 | Higher |
| Halstead Effort | 73614.9212 | 90997.8001 | **32791.6758 ✅** | -17382.8790 | Lower | +40823.2453 | Higher | +58206.1243 | Higher |
| SLOC | 63 | 73 | **54 ✅** | -10 | Lower | +9 | Higher | +19 | Higher |
| PLOC | 39 | 48 | **33 ✅** | -9 | Lower | +6 | Higher | +15 | Higher |
| LLOC | 28 | 25 | 25 | +3 | Higher | +3 | Higher | 0 | Same |
| Comments (CLOC) | 15 | **18 ✅** | 13 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.8500 | 59.4353 | **66.7798 ✅** | -2.5853 | Lower | -9.9298 | Lower | -7.3445 | Lower |
| MI (SEI) | 49.0621 | 53.3659 | **58.6459 ✅** | -4.3038 | Lower | -9.5838 | Lower | -5.2800 | Lower |
| Cyclomatic (sum) | 15 | 16 | **8 ✅** | -1 | Lower | +7 | Higher | +8 | Higher |
| Cognitive (sum) | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 1341.2426 | 1254.5655 | **1139.8562 ✅** | +86.6771 | Higher | +201.3864 | Higher | +114.7093 | Higher |
| Halstead Effort | 36328.5143 | **36301.4591 ✅** | 37147.0984 | +27.0552 | Higher | -818.5841 | Lower | -845.6393 | Lower |
| SLOC | 92 | 79 | **58 ✅** | +13 | Higher | +34 | Higher | +21 | Higher |
| PLOC | 47 | 43 | **32 ✅** | +4 | Higher | +15 | Higher | +11 | Higher |
| LLOC | 26 | **24 ✅** | 27 | +2 | Higher | -1 | Lower | -3 | Lower |
| Comments (CLOC) | **36 ✅** | 32 | 17 | +4 | Higher | +19 | Higher | +15 | Higher |
| Blank Lines | 11 | **8 ✅** | 9 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | 3 | **0 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p00006

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.1150 | 49.9935 | **79.5538 ✅** | +11.1215 | Higher | -18.4388 | Lower | -29.5604 | Lower |
| MI (SEI) | 56.1438 | 37.8027 | **77.3384 ✅** | +18.3412 | Higher | -21.1945 | Lower | -39.5357 | Lower |
| Cyclomatic (sum) | 12 | 19 | **6 ✅** | -7 | Lower | +6 | Higher | +13 | Higher |
| Cognitive (sum) | 10 | 19 | **3 ✅** | -9 | Lower | +7 | Higher | +16 | Higher |
| Halstead Volume | 1174.1920 | 1743.6368 | **618.5942 ✅** | -569.4447 | Lower | +555.5979 | Higher | +1125.0426 | Higher |
| Halstead Effort | 45725.7479 | 89431.6928 | **8907.7562 ✅** | -43705.9448 | Lower | +36817.9917 | Higher | +80523.9365 | Higher |
| SLOC | 77 | 122 | **33 ✅** | -45 | Lower | +44 | Higher | +89 | Higher |
| PLOC | 34 | 70 | **18 ✅** | -36 | Lower | +16 | Higher | +52 | Higher |
| LLOC | 26 | 44 | **11 ✅** | -18 | Lower | +15 | Higher | +33 | Higher |
| Comments (CLOC) | 33 | **42 ✅** | 10 | -9 | Lower | +23 | Higher | +32 | Higher |
| Blank Lines | 10 | 10 | **5 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p00007

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.5465 ✅** | 73.7748 | 73.4659 | +4.7716 | Higher | +5.0805 | Higher | +0.3089 | Higher |
| MI (SEI) | **77.0175 ✅** | 71.2384 | 70.8323 | +5.7792 | Higher | +6.1852 | Higher | +0.4061 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 9 | -2 | Lower | -5 | Lower | -3 | Lower |
| Cognitive (sum) | **3 ✅** | 7 | 8 | -4 | Lower | -5 | Lower | -1 | Lower |
| Halstead Volume | **625.5088 ✅** | 715.1855 | 765.7774 | -89.6767 | Lower | -140.2686 | Lower | -50.5919 | Lower |
| Halstead Effort | **11657.2098 ✅** | 14303.7108 | 17919.1917 | -2646.5010 | Lower | -6261.9818 | Lower | -3615.4808 | Lower |
| SLOC | **36 ✅** | 45 | 43 | -9 | Lower | -7 | Lower | +2 | Higher |
| PLOC | **18 ✅** | 22 | 23 | -4 | Lower | -5 | Lower | -1 | Lower |
| LLOC | 16 | **13 ✅** | 15 | +3 | Higher | +1 | Higher | -2 | Lower |
| Comments (CLOC) | 12 | **16 ✅** | 15 | -4 | Lower | -3 | Lower | +1 | Higher |
| Blank Lines | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |

## p00008

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.1884 | 45.9444 | **69.3289 ✅** | +20.2440 | Higher | -3.1405 | Lower | -23.3845 | Lower |
| MI (SEI) | 63.2517 | 35.6714 | **66.5731 ✅** | +27.5804 | Higher | -3.3214 | Lower | -30.9017 | Lower |
| Cyclomatic (sum) | 12 | 24 | **11 ✅** | -12 | Lower | +1 | Higher | +13 | Higher |
| Cognitive (sum) | 29 | 28 | **18 ✅** | +1 | Higher | +11 | Higher | +10 | Higher |
| Halstead Volume | 787.4402 | 2074.1966 | **763.8589 ✅** | -1286.7565 | Lower | +23.5812 | Higher | +1310.3377 | Higher |
| Halstead Effort | 16035.1449 | 93387.0852 | **15145.4786 ✅** | -77351.9403 | Lower | +889.6663 | Higher | +78241.6066 | Higher |
| SLOC | 64 | 138 | **54 ✅** | -74 | Lower | +10 | Higher | +84 | Higher |
| PLOC | 33 | 63 | **30 ✅** | -30 | Lower | +3 | Higher | +33 | Higher |
| LLOC | 17 | 50 | **16 ✅** | -33 | Lower | +1 | Higher | +34 | Higher |
| Comments (CLOC) | 27 | **60 ✅** | 21 | -33 | Lower | +6 | Higher | +39 | Higher |
| Blank Lines | 4 | 17 | **3 ✅** | -13 | Lower | +1 | Higher | +14 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00009

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.8457 | 49.2914 | **57.1172 ✅** | +5.5543 | Higher | -2.2715 | Lower | -7.8258 | Lower |
| MI (SEI) | **45.5956 ✅** | 40.3552 | 43.4682 | +5.2405 | Higher | +2.1274 | Higher | -3.1131 | Lower |
| Cyclomatic (sum) | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | **20 ✅** | 21 | 25 | -1 | Lower | -5 | Lower | -4 | Lower |
| Halstead Volume | **1368 ✅** | 1788.8593 | 1679.8442 | -420.8593 | Lower | -311.8442 | Lower | +109.0151 | Higher |
| Halstead Effort | **50696.4706 ✅** | 68642.2754 | 73316.3592 | -17945.8048 | Lower | -22619.8886 | Lower | -4674.0838 | Lower |
| SLOC | 102 | 130 | **83 ✅** | -28 | Lower | +19 | Higher | +47 | Higher |
| PLOC | 51 | 55 | **50 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| LLOC | 35 | 38 | 35 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 38 | **59 ✅** | 21 | -21 | Lower | +17 | Higher | +38 | Higher |
| Blank Lines | 13 | 16 | **12 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |

## p00010

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.2872 ✅** | 53.0693 | 53.3347 | +4.2179 | Higher | +3.9525 | Higher | -0.2654 | Lower |
| MI (SEI) | 36.7156 | **40.9916 ✅** | 31.9020 | -4.2760 | Lower | +4.8136 | Higher | +9.0896 | Higher |
| Cyclomatic (sum) | **12 ✅** | 23 | 20 | -11 | Lower | -8 | Lower | +3 | Higher |
| Cognitive (sum) | **12 ✅** | 17 | 14 | -5 | Lower | -2 | Lower | +3 | Higher |
| Halstead Volume | **2354.9084 ✅** | 2492.6497 | 2806.3204 | -137.7413 | Lower | -451.4120 | Lower | -313.6707 | Lower |
| Halstead Effort | **107552.0290 ✅** | 115794.9088 | 143322.7903 | -8242.8799 | Lower | -35770.7614 | Lower | -27527.8815 | Lower |
| SLOC | **78 ✅** | 85 | 84 | -7 | Lower | -6 | Lower | +1 | Higher |
| PLOC | 53 | **46 ✅** | 58 | +7 | Higher | -5 | Lower | -12 | Lower |
| LLOC | 54 | **28 ✅** | 53 | +26 | Higher | +1 | Higher | -25 | Lower |
| Comments (CLOC) | 12 | **26 ✅** | 13 | -14 | Lower | -1 | Lower | +13 | Higher |
| Blank Lines | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 6 | -1 | Lower | -6 | Lower | -5 | Lower |
| Exits | 1 | 11 | 1 | -10 | Lower | 0 | Same | +10 | Higher |

## p00011

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.2752 | **72.6082 ✅** | 41.3644 | -31.3330 | Lower | -0.0892 | Lower | +31.2438 | Higher |
| MI (SEI) | 21.0914 | **60.8484 ✅** | 21.8995 | -39.7570 | Lower | -0.8082 | Lower | +38.9489 | Higher |
| Cyclomatic (sum) | 31 | **5 ✅** | 29 | +26 | Higher | +2 | Higher | -24 | Lower |
| Cognitive (sum) | 20 | **3 ✅** | 29 | +17 | Higher | -9 | Lower | -26 | Lower |
| Halstead Volume | 3201.8350 | **1159.8802 ✅** | 3225.9849 | +2041.9547 | Higher | -24.1499 | Lower | -2066.1047 | Lower |
| Halstead Effort | 125052.3123 | **16591.9453 ✅** | 167720.7813 | +108460.3670 | Higher | -42668.4690 | Lower | -151128.8360 | Lower |
| SLOC | 145 | **42 ✅** | 148 | +103 | Higher | -3 | Lower | -106 | Lower |
| PLOC | 92 | **26 ✅** | 95 | +66 | Higher | -3 | Lower | -69 | Lower |
| LLOC | 48 | **21 ✅** | 62 | +27 | Higher | -14 | Lower | -41 | Lower |
| Comments (CLOC) | 34 | 8 | **37 ✅** | +26 | Higher | -3 | Lower | -29 | Lower |
| Blank Lines | 19 | **8 ✅** | 18 | +11 | Higher | +1 | Higher | -10 | Lower |
| Functions | 8 | **1 ✅** | 6 | +7 | Higher | +2 | Higher | -5 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **0 ✅** | 5 | +7 | Higher | +2 | Higher | -5 | Lower |

## p00012

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.0598 | **64.5765 ✅** | 59.1205 | -9.5168 | Lower | -4.0607 | Lower | +5.4561 | Higher |
| MI (SEI) | 41.9418 | **44.7301 ✅** | 39.2491 | -2.7882 | Lower | +2.6927 | Higher | +5.4809 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 22 | +2 | Higher | -10 | Lower | -12 | Lower |
| Cognitive (sum) | 10 | **8 ✅** | 21 | +2 | Higher | -11 | Lower | -13 | Lower |
| Halstead Volume | 2314.1393 | **1880.6262 ✅** | 2825.8257 | +433.5131 | Higher | -511.6863 | Lower | -945.1995 | Lower |
| Halstead Effort | 80700.3492 | **57208.6484 ✅** | 130970.8758 | +23491.7007 | Higher | -50270.5267 | Lower | -73762.2274 | Lower |
| SLOC | 90 | **55 ✅** | 57 | +35 | Higher | +33 | Higher | -2 | Lower |
| PLOC | 54 | 40 | **39 ✅** | +14 | Higher | +15 | Higher | +1 | Higher |
| LLOC | 43 | **24 ✅** | 32 | +19 | Higher | +11 | Higher | -8 | Lower |
| Comments (CLOC) | **26 ✅** | 7 | 8 | +19 | Higher | +18 | Higher | -1 | Lower |
| Blank Lines | 11 | **8 ✅** | 10 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p00013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.7287 | 68.5595 | **71.8677 ✅** | -0.8307 | Lower | -4.1389 | Lower | -3.3082 | Lower |
| MI (SEI) | 65.8716 | 63.4142 | **71.1329 ✅** | +2.4574 | Higher | -5.2613 | Lower | -7.7187 | Lower |
| Cyclomatic (sum) | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 806.4400 | 998.3469 | **698.8072 ✅** | -191.9068 | Lower | +107.6328 | Higher | +299.5396 | Higher |
| Halstead Effort | 21370.6612 | 27062.3310 | **15312.9936 ✅** | -5691.6698 | Lower | +6057.6676 | Higher | +11749.3374 | Higher |
| SLOC | 62 | 55 | **51 ✅** | +7 | Higher | +11 | Higher | +4 | Higher |
| PLOC | 29 | 32 | **24 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| LLOC | 22 | 25 | **21 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Comments (CLOC) | **28 ✅** | 19 | 22 | +9 | Higher | +6 | Higher | -3 | Lower |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00014

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.8220 | **72.7755 ✅** | 71.7767 | -1.9535 | Lower | -0.9547 | Lower | +0.9988 | Higher |
| MI (SEI) | 66.1790 | **71.9786 ✅** | 66.5662 | -5.7996 | Lower | -0.3872 | Lower | +5.4124 | Higher |
| Cyclomatic (sum) | 7 | **4 ✅** | 9 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 6 | **3 ✅** | 8 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | 817.4872 | **603.5117 ✅** | 705.4328 | +213.9755 | Higher | +112.0544 | Higher | -101.9211 | Lower |
| Halstead Effort | 16664.1615 | **8997.8103 ✅** | 16194.2830 | +7666.3513 | Higher | +469.8785 | Higher | -7196.4728 | Lower |
| SLOC | 51 | 52 | **49 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| PLOC | 25 | **22 ✅** | 26 | +3 | Higher | -1 | Lower | -4 | Lower |
| LLOC | 19 | **15 ✅** | 19 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | 17 | **22 ✅** | 15 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p00016

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.0059 | 42.9871 | **47.9061 ✅** | -8.9812 | Lower | -13.9002 | Lower | -4.9190 | Lower |
| MI (SEI) | 15.2323 | 26.9500 | **32.8785 ✅** | -11.7176 | Lower | -17.6461 | Lower | -5.9285 | Lower |
| Cyclomatic (sum) | 46 | 25 | **21 ✅** | +21 | Higher | +25 | Higher | +4 | Higher |
| Cognitive (sum) | 53 | 27 | **18 ✅** | +26 | Higher | +35 | Higher | +9 | Higher |
| Halstead Volume | 3781.8283 | 3069.3157 | **2210.5654 ✅** | +712.5127 | Higher | +1571.2629 | Higher | +858.7503 | Higher |
| Halstead Effort | 209603.0274 | 130445.9154 | **97213.4690 ✅** | +79157.1120 | Higher | +112389.5584 | Higher | +33232.4464 | Higher |
| SLOC | 174 | 144 | **125 ✅** | +30 | Higher | +49 | Higher | +19 | Higher |
| PLOC | 104 | 85 | **74 ✅** | +19 | Higher | +30 | Higher | +11 | Higher |
| LLOC | 82 | 63 | **58 ✅** | +19 | Higher | +24 | Higher | +5 | Higher |
| Comments (CLOC) | **51 ✅** | 45 | 37 | +6 | Higher | +14 | Higher | +8 | Higher |
| Blank Lines | 22 | 16 | 16 | +6 | Higher | +6 | Higher | 0 | Same |
| Functions | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |

## p00017

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.5120 | 53.0119 | **57.1614 ✅** | -0.4998 | Lower | -4.6494 | Lower | -4.1496 | Lower |
| MI (SEI) | 40.1592 | 43.3899 | **45.4155 ✅** | -3.2307 | Lower | -5.2563 | Lower | -2.0257 | Lower |
| Cyclomatic (sum) | 23 | **16 ✅** | 17 | +7 | Higher | +6 | Higher | -1 | Lower |
| Cognitive (sum) | 27 | 21 | **19 ✅** | +6 | Higher | +8 | Higher | +2 | Higher |
| Halstead Volume | 1962.0779 | 1832.1881 | **1787.2067 ✅** | +129.8898 | Higher | +174.8712 | Higher | +44.9814 | Higher |
| Halstead Effort | 71942.8559 | **52704.6440 ✅** | 56640.7046 | +19238.2118 | Higher | +15302.1513 | Higher | -3936.0605 | Lower |
| SLOC | 95 | 104 | **80 ✅** | -9 | Lower | +15 | Higher | +24 | Higher |
| PLOC | 60 | 51 | 51 | +9 | Higher | +9 | Higher | 0 | Same |
| LLOC | 32 | **24 ✅** | 30 | +8 | Higher | +2 | Higher | -6 | Lower |
| Comments (CLOC) | 29 | **40 ✅** | 23 | -11 | Lower | +6 | Higher | +17 | Higher |
| Blank Lines | 8 | 14 | **6 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |

## p00018

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.3362 ✅** | 66.0874 | 70.0674 | +9.2488 | Higher | +5.2688 | Higher | -3.9800 | Lower |
| MI (SEI) | **71.9909 ✅** | 64.4235 | 65.8216 | +7.5673 | Higher | +6.1693 | Higher | -1.3981 | Lower |
| Cyclomatic (sum) | 7 | 7 | 9 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **8 ✅** | 9 | 11 | -1 | Lower | -3 | Lower | -2 | Lower |
| Halstead Volume | **677.2602 ✅** | 910.0497 | 814.3479 | -232.7895 | Lower | -137.0877 | Lower | +95.7018 | Higher |
| Halstead Effort | **16931.5044 ✅** | 20476.1183 | 21099.0139 | -3544.6140 | Lower | -4167.5095 | Lower | -622.8955 | Lower |
| SLOC | **41 ✅** | 66 | 52 | -25 | Lower | -11 | Lower | +14 | Higher |
| PLOC | **22 ✅** | 27 | 28 | -5 | Lower | -6 | Lower | -1 | Lower |
| LLOC | 17 | **14 ✅** | 20 | +3 | Higher | -3 | Lower | -6 | Lower |
| Comments (CLOC) | 13 | **32 ✅** | 18 | -19 | Lower | -5 | Lower | +14 | Higher |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00019

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.7633 | **57.6797 ✅** | 57.4508 | -3.9164 | Lower | -3.6876 | Lower | +0.2289 | Higher |
| MI (SEI) | 44.5515 | **52.1532 ✅** | 46.2241 | -7.6017 | Lower | -1.6726 | Lower | +5.9291 | Higher |
| Cyclomatic (sum) | 16 | 13 | 13 | +3 | Higher | +3 | Higher | 0 | Same |
| Cognitive (sum) | 16 | **12 ✅** | 13 | +4 | Higher | +3 | Higher | -1 | Lower |
| Halstead Volume | 1494.3200 | **1168.2422 ✅** | 1305.4007 | +326.0778 | Higher | +188.9193 | Higher | -137.1585 | Lower |
| Halstead Effort | 41781.9731 | 33803.6523 | **30595.3288 ✅** | +7978.3209 | Higher | +11186.6443 | Higher | +3208.3235 | Higher |
| SLOC | 106 | 94 | **92 ✅** | +12 | Higher | +14 | Higher | +2 | Higher |
| PLOC | 55 | **40 ✅** | 52 | +15 | Higher | +3 | Higher | -12 | Lower |
| LLOC | 40 | **29 ✅** | 30 | +11 | Higher | +10 | Higher | -1 | Lower |
| Comments (CLOC) | 41 | **43 ✅** | 28 | -2 | Lower | +13 | Higher | +15 | Higher |
| Blank Lines | **10 ✅** | 11 | 12 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **0 ✅** | 1 | +4 | Higher | +3 | Higher | -1 | Lower |

## p00020

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.8855 | **77.4124 ✅** | 58.1958 | -3.5270 | Lower | +15.6897 | Higher | +19.2167 | Higher |
| MI (SEI) | 77.8888 | **80.1063 ✅** | 52.9640 | -2.2175 | Lower | +24.9248 | Higher | +27.1424 | Higher |
| Cyclomatic (sum) | 6 | **5 ✅** | 13 | +1 | Higher | -7 | Lower | -8 | Lower |
| Cognitive (sum) | 5 | **4 ✅** | 9 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | 474.0603 | **324.3303 ✅** | 1346.2774 | +149.7299 | Higher | -872.2171 | Lower | -1021.9470 | Lower |
| Halstead Effort | 6648.6951 | **3567.6337 ✅** | 45971.4127 | +3081.0613 | Higher | -39322.7176 | Lower | -42403.7790 | Lower |
| SLOC | 51 | **47 ✅** | 87 | +4 | Higher | -36 | Lower | -40 | Lower |
| PLOC | **16 ✅** | 19 | 39 | -3 | Lower | -23 | Lower | -20 | Lower |
| LLOC | 11 | **7 ✅** | 25 | +4 | Higher | -14 | Lower | -18 | Lower |
| Comments (CLOC) | 30 | 22 | **40 ✅** | +8 | Higher | -10 | Lower | -18 | Lower |
| Blank Lines | 6 | 6 | 10 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00021

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.2200 ✅** | 55.5351 | 69.4221 | +16.6849 | Higher | +2.7979 | Higher | -13.8870 | Lower |
| MI (SEI) | 63.2979 | 41.1546 | **63.8516 ✅** | +22.1433 | Higher | -0.5536 | Lower | -22.6970 | Lower |
| Cyclomatic (sum) | **5 ✅** | 13 | 7 | -8 | Lower | -2 | Lower | +6 | Higher |
| Cognitive (sum) | **6 ✅** | 16 | 7 | -10 | Lower | -1 | Lower | +9 | Higher |
| Halstead Volume | **1249.7825 ✅** | 1824.3948 | 1475.7212 | -574.6123 | Lower | -225.9387 | Lower | +348.6737 | Higher |
| Halstead Effort | **23321.8340 ✅** | 56936.3221 | 35712.4524 | -33614.4881 | Lower | -12390.6184 | Lower | +21223.8697 | Higher |
| SLOC | **42 ✅** | 93 | 46 | -51 | Lower | -4 | Lower | +47 | Higher |
| PLOC | **25 ✅** | 56 | 28 | -31 | Lower | -3 | Lower | +28 | Higher |
| LLOC | 19 | 42 | 19 | -23 | Lower | 0 | Same | +23 | Higher |
| Comments (CLOC) | 10 | **24 ✅** | 15 | -14 | Lower | -5 | Lower | +9 | Higher |
| Blank Lines | 7 | 13 | **6 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.0564 | **63.6627 ✅** | 57.8043 | -3.6062 | Lower | +2.2521 | Higher | +5.8584 | Higher |
| MI (SEI) | 46.2082 | **51.9327 ✅** | 43.6498 | -5.7245 | Lower | +2.5584 | Higher | +8.2829 | Higher |
| Cyclomatic (sum) | 15 | **10 ✅** | 17 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 23 | **15 ✅** | 43 | +8 | Higher | -20 | Lower | -28 | Lower |
| Halstead Volume | 1696.1909 | **1106.1229 ✅** | 1853.0326 | +590.0679 | Higher | -156.8417 | Lower | -746.9096 | Lower |
| Halstead Effort | 64196.1131 | **33429.4933 ✅** | 82432.6985 | +30766.6198 | Higher | -18236.5854 | Lower | -49003.2051 | Lower |
| SLOC | 70 | **69 ✅** | 76 | +1 | Higher | -6 | Lower | -7 | Lower |
| PLOC | 47 | **41 ✅** | 51 | +6 | Higher | -4 | Lower | -10 | Lower |
| LLOC | 36 | **32 ✅** | 42 | +4 | Higher | -6 | Lower | -10 | Lower |
| Comments (CLOC) | 16 | 17 | **18 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00023

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.8787 | **67.6145 ✅** | 58.3542 | -6.7358 | Lower | +2.5246 | Higher | +9.2604 | Higher |
| MI (SEI) | 46.2977 | **62.4129 ✅** | 37.7909 | -16.1152 | Lower | +8.5068 | Higher | +24.6220 | Higher |
| Cyclomatic (sum) | 8 | **6 ✅** | 9 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 13 | **6 ✅** | 13 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 1807.7880 | **1183.1274 ✅** | 2105.0047 | +624.6606 | Higher | -297.2167 | Lower | -921.8773 | Lower |
| Halstead Effort | 41579.1246 | **24977.1345 ✅** | 55066.9227 | +16601.9900 | Higher | -13487.7981 | Lower | -30089.7882 | Lower |
| SLOC | 72 | **56 ✅** | 79 | +16 | Higher | -7 | Lower | -23 | Lower |
| PLOC | 46 | **27 ✅** | 57 | +19 | Higher | -11 | Lower | -30 | Lower |
| LLOC | 22 | **19 ✅** | 45 | +3 | Higher | -23 | Lower | -26 | Lower |
| Comments (CLOC) | 16 | **20 ✅** | 12 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p00024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.6374 ✅** | 79.1607 | 76.1456 | +0.4767 | Higher | +3.4918 | Higher | +3.0151 | Higher |
| MI (SEI) | 78.3769 | **79.1223 ✅** | 78.6327 | -0.7454 | Lower | -0.2558 | Lower | +0.4896 | Higher |
| Cyclomatic (sum) | 6 | 5 | **3 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 4 | 3 | **1 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Halstead Volume | 554.6772 | 531.7739 | **517.6625 ✅** | +22.9032 | Higher | +37.0147 | Higher | +14.1115 | Higher |
| Halstead Effort | 7987.3511 | **6891.7903 ✅** | 7764.9371 | +1095.5608 | Higher | +222.4141 | Higher | -873.1467 | Lower |
| SLOC | **34 ✅** | 36 | 45 | -2 | Lower | -11 | Lower | -9 | Lower |
| PLOC | 18 | **16 ✅** | 17 | +2 | Higher | +1 | Higher | -1 | Lower |
| LLOC | 10 | 9 | **8 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Comments (CLOC) | 11 | 13 | **22 ✅** | -2 | Lower | -11 | Lower | -9 | Lower |
| Blank Lines | **5 ✅** | 7 | 6 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00025

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.2204 | 62.3397 | **71.7859 ✅** | +3.8807 | Higher | -5.5655 | Lower | -9.4462 | Lower |
| MI (SEI) | 59.2287 | 57.6335 | **61.5460 ✅** | +1.5952 | Higher | -2.3173 | Lower | -3.9125 | Lower |
| Cyclomatic (sum) | 11 | 13 | **9 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 20 | 20 | **18 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1000.1752 | 1094.1716 | **801.8106 ✅** | -93.9964 | Lower | +198.3647 | Higher | +292.3611 | Higher |
| Halstead Effort | 27088.0792 | 33235.4626 | **18241.1901 ✅** | -6147.3834 | Lower | +8846.8891 | Higher | +14994.2725 | Higher |
| SLOC | 60 | 72 | **47 ✅** | -12 | Lower | +13 | Higher | +25 | Higher |
| PLOC | 34 | 35 | **30 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| LLOC | 23 | 24 | **19 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| Comments (CLOC) | 19 | **30 ✅** | 10 | -11 | Lower | +9 | Higher | +20 | Higher |
| Blank Lines | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00026

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.2903 | **51.8942 ✅** | 9.1653 | -9.6038 | Lower | +33.1251 | Higher | +42.7289 | Higher |
| MI (SEI) | 11.6263 | **34.3165 ✅** | -21.7086 | -22.6902 | Lower | +33.3349 | Higher | +56.0251 | Higher |
| Cyclomatic (sum) | 29 | **22 ✅** | 86 | +7 | Higher | -57 | Lower | -64 | Lower |
| Cognitive (sum) | **28 ✅** | 35 | 107 | -7 | Lower | -79 | Lower | -72 | Lower |
| Halstead Volume | 3855.8914 | **2467.8439 ✅** | 9241.3231 | +1388.0476 | Higher | -5385.4317 | Lower | -6773.4792 | Lower |
| Halstead Effort | 279955.6532 | **136872.7912 ✅** | 1341796.7995 | +143082.8620 | Higher | -1061841.1463 | Lower | -1204924.0083 | Lower |
| SLOC | 132 | **93 ✅** | 343 | +39 | Higher | -211 | Lower | -250 | Lower |
| PLOC | 106 | **66 ✅** | 256 | +40 | Higher | -150 | Lower | -190 | Lower |
| LLOC | 52 | **46 ✅** | 190 | +6 | Higher | -138 | Lower | -144 | Lower |
| Comments (CLOC) | 13 | 20 | **69 ✅** | -7 | Lower | -56 | Lower | -49 | Lower |
| Blank Lines | 13 | **8 ✅** | 20 | +5 | Higher | -7 | Lower | -12 | Lower |
| Functions | 8 | **1 ✅** | 7 | +7 | Higher | +1 | Higher | -6 | Lower |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | 3 | **0 ✅** | 4 | +3 | Higher | -1 | Lower | -4 | Lower |

## p00027

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.6497 | 54.0646 | **62.0730 ✅** | +7.5851 | Higher | -0.4233 | Lower | -8.0084 | Lower |
| MI (SEI) | **48.0106 ✅** | 44.8963 | 45.4311 | +3.1143 | Higher | +2.5795 | Higher | -0.5347 | Lower |
| Cyclomatic (sum) | 16 | 17 | 16 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 1579.2302 | 1778.7688 | **1455.7671 ✅** | -199.5386 | Lower | +123.4631 | Higher | +323.0017 | Higher |
| Halstead Effort | 40080.8627 | 48916.1428 | **34893.8464 ✅** | -8835.2801 | Lower | +5187.0163 | Higher | +14022.2964 | Higher |
| SLOC | 64 | 97 | 64 | -33 | Lower | 0 | Same | +33 | Higher |
| PLOC | **42 ✅** | 50 | 45 | -8 | Lower | -3 | Lower | +5 | Higher |
| LLOC | 20 | 23 | **17 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Comments (CLOC) | 14 | **37 ✅** | 11 | -23 | Lower | +3 | Higher | +26 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00028

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.7181 | 35.7642 | **69.6612 ✅** | +8.9539 | Higher | -24.9431 | Lower | -33.8970 | Lower |
| MI (SEI) | 33.1343 | 21.5479 | **67.8380 ✅** | +11.5864 | Higher | -34.7037 | Lower | -46.2901 | Lower |
| Cyclomatic (sum) | 28 | 33 | **9 ✅** | -5 | Lower | +19 | Higher | +24 | Higher |
| Cognitive (sum) | 35 | 26 | **14 ✅** | +9 | Higher | +21 | Higher | +12 | Higher |
| Halstead Volume | 2200.0310 | 3307.3549 | **739.3422 ✅** | -1107.3239 | Lower | +1460.6888 | Higher | +2568.0127 | Higher |
| Halstead Effort | 83516.5606 | 168675.1010 | **15963.0697 ✅** | -85158.5405 | Lower | +67553.4908 | Higher | +152712.0313 | Higher |
| SLOC | 138 | 196 | **55 ✅** | -58 | Lower | +83 | Higher | +141 | Higher |
| PLOC | 69 | 94 | **27 ✅** | -25 | Lower | +42 | Higher | +67 | Higher |
| LLOC | 42 | 60 | **17 ✅** | -18 | Lower | +25 | Higher | +43 | Higher |
| Comments (CLOC) | 55 | **83 ✅** | 23 | -28 | Lower | +32 | Higher | +60 | Higher |
| Blank Lines | 14 | 19 | **5 ✅** | -5 | Lower | +9 | Higher | +14 | Higher |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 10 | 0 | -10 | Lower | 0 | Same | +10 | Higher |

## p00029

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.7399 ✅** | 50.6453 | 48.8808 | +1.0946 | Higher | +2.8591 | Higher | +1.7645 | Higher |
| MI (SEI) | 29.9424 | **41.2822 ✅** | 29.9390 | -11.3398 | Lower | +0.0034 | Higher | +11.3433 | Higher |
| Cyclomatic (sum) | 18 | **17 ✅** | 23 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | **20 ✅** | 43 | 33 | -23 | Lower | -13 | Lower | +10 | Higher |
| Halstead Volume | 2497.1616 | **1816.0299 ✅** | 2645.2271 | +681.1317 | Higher | -148.0655 | Lower | -829.1973 | Lower |
| Halstead Effort | 88972.9428 | **55809.6986 ✅** | 111044.4307 | +33163.2443 | Higher | -22071.4879 | Lower | -55234.7321 | Lower |
| SLOC | **99 ✅** | 119 | 108 | -20 | Lower | -9 | Lower | +11 | Higher |
| PLOC | 68 | **62 ✅** | 75 | +6 | Higher | -7 | Lower | -13 | Lower |
| LLOC | 52 | **50 ✅** | 60 | +2 | Higher | -8 | Lower | -10 | Lower |
| Comments (CLOC) | 16 | **50 ✅** | 23 | -34 | Lower | -7 | Lower | +27 | Higher |
| Blank Lines | 15 | **12 ✅** | 13 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 4 | **1 ✅** | 3 | +3 | Higher | +1 | Higher | -2 | Lower |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
| Exits | 4 | **0 ✅** | 4 | +4 | Higher | 0 | Same | -4 | Lower |

## p00030

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.6926 | 56.7956 | **61.6016 ✅** | +2.8971 | Higher | -1.9090 | Lower | -4.8061 | Lower |
| MI (SEI) | 44.7768 | **46.5913 ✅** | 46.0281 | -1.8145 | Lower | -1.2513 | Lower | +0.5632 | Higher |
| Cyclomatic (sum) | 20 | 20 | **13 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Cognitive (sum) | 15 | 16 | **14 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 1596 | **1390.2106 ✅** | 1578.0019 | +205.7894 | Higher | +17.9981 | Higher | -187.7913 | Lower |
| Halstead Effort | 56485.4595 | **48637.5125 ✅** | 58583.3205 | +7847.9470 | Higher | -2097.8611 | Lower | -9945.8080 | Lower |
| SLOC | 68 | 85 | **67 ✅** | -17 | Lower | +1 | Higher | +18 | Higher |
| PLOC | 47 | 51 | **42 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| LLOC | **26 ✅** | 31 | 32 | -5 | Lower | -6 | Lower | -1 | Lower |
| Comments (CLOC) | 14 | **27 ✅** | 13 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | **8 ✅** | 9 | 12 | -1 | Lower | -4 | Lower | -3 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

## p00031

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.4996 | 52.6484 | **53.9107 ✅** | -1.1488 | Lower | -2.4110 | Lower | -1.2623 | Lower |
| MI (SEI) | 37.2310 | 32.9663 | **37.4810 ✅** | +4.2647 | Higher | -0.2500 | Lower | -4.5147 | Lower |
| Cyclomatic (sum) | 19 | 22 | **18 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | **26 ✅** | 27 | 28 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 2022.4122 | **1997.7293 ✅** | 1998.6678 | +24.6829 | Higher | +23.7443 | Higher | -0.9385 | Lower |
| Halstead Effort | 70784.4253 | 87856.6593 | **54109.7672 ✅** | -17072.2341 | Lower | +16674.6581 | Higher | +33746.8922 | Higher |
| SLOC | 106 | 95 | **93 ✅** | +11 | Higher | +13 | Higher | +2 | Higher |
| PLOC | **64 ✅** | 69 | 66 | -5 | Lower | -2 | Lower | +3 | Higher |
| LLOC | **43 ✅** | 48 | 44 | -5 | Lower | -1 | Lower | +4 | Higher |
| Comments (CLOC) | **30 ✅** | 17 | 21 | +13 | Higher | +9 | Higher | -4 | Lower |
| Blank Lines | 13 | 9 | **7 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00032

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.3006 ✅** | 39.1647 | 37.9403 | +0.1359 | Higher | +1.3603 | Higher | +1.2244 | Higher |
| MI (SEI) | **24.3813 ✅** | 22.1563 | 16.8116 | +2.2250 | Higher | +7.5697 | Higher | +5.3447 | Higher |
| Cyclomatic (sum) | **30 ✅** | 34 | 37 | -4 | Lower | -7 | Lower | -3 | Lower |
| Cognitive (sum) | **24 ✅** | 33 | 29 | -9 | Lower | -5 | Lower | +4 | Higher |
| Halstead Volume | **3035.9329 ✅** | 3157.4040 | 3238.5200 | -121.4711 | Lower | -202.5872 | Lower | -81.1160 | Lower |
| Halstead Effort | 168494.2738 | **157449.2121 ✅** | 185342.9926 | +11045.0616 | Higher | -16848.7188 | Lower | -27893.7804 | Lower |
| SLOC | 169 | **159 ✅** | 163 | +10 | Higher | +6 | Higher | -4 | Lower |
| PLOC | 94 | **92 ✅** | 106 | +2 | Higher | -12 | Lower | -14 | Lower |
| LLOC | **72 ✅** | 73 | 85 | -1 | Lower | -13 | Lower | -12 | Lower |
| Comments (CLOC) | **62 ✅** | 49 | 38 | +13 | Higher | +24 | Higher | +11 | Higher |
| Blank Lines | **19 ✅** | 22 | 23 | -3 | Lower | -4 | Lower | -1 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |

## p00033

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.6379 | 61.2571 | **65.4523 ✅** | -0.6192 | Lower | -4.8145 | Lower | -4.1952 | Lower |
| MI (SEI) | 39.8801 | 51.5903 | **55.3828 ✅** | -11.7101 | Lower | -15.5026 | Lower | -3.7925 | Lower |
| Cyclomatic (sum) | 17 | 15 | **10 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| Cognitive (sum) | 20 | 19 | **13 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |
| Halstead Volume | 1388.3415 | **1132.4342 ✅** | 1150.9775 | +255.9073 | Higher | +237.3640 | Higher | -18.5433 | Lower |
| Halstead Effort | 36493.5479 | **26835.2592 ✅** | 27753.4091 | +9658.2887 | Higher | +8740.1388 | Higher | -918.1499 | Lower |
| SLOC | 70 | 74 | **61 ✅** | -4 | Lower | +9 | Higher | +13 | Higher |
| PLOC | 51 | 43 | **37 ✅** | +8 | Higher | +14 | Higher | +6 | Higher |
| LLOC | 36 | **28 ✅** | 30 | +8 | Higher | +6 | Higher | -2 | Lower |
| Comments (CLOC) | 9 | **22 ✅** | 16 | -13 | Lower | -7 | Lower | +6 | Higher |
| Blank Lines | 10 | 9 | **8 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 3 | 4 | **1 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p00034

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.8252 ✅** | 51.0623 | 30.9671 | +5.7628 | Higher | +25.8581 | Higher | +20.0952 | Higher |
| MI (SEI) | **50.8661 ✅** | 34.9438 | 10.7922 | +15.9224 | Higher | +40.0740 | Higher | +24.1516 | Higher |
| Cyclomatic (sum) | **13 ✅** | 21 | 40 | -8 | Lower | -27 | Lower | -19 | Lower |
| Cognitive (sum) | **13 ✅** | 18 | 35 | -5 | Lower | -22 | Lower | -17 | Lower |
| Halstead Volume | **1576.6529 ✅** | 2073.9574 | 4083.6837 | -497.3045 | Lower | -2507.0308 | Lower | -2009.7263 | Lower |
| Halstead Effort | **46353.5946 ✅** | 79187.4653 | 234520.1198 | -32833.8707 | Lower | -188166.5252 | Lower | -155332.6545 | Lower |
| SLOC | **90 ✅** | 105 | 223 | -15 | Lower | -133 | Lower | -118 | Lower |
| PLOC | **39 ✅** | 69 | 141 | -30 | Lower | -102 | Lower | -72 | Lower |
| LLOC | **29 ✅** | 48 | 84 | -19 | Lower | -55 | Lower | -36 | Lower |
| Comments (CLOC) | 41 | 26 | **68 ✅** | +15 | Higher | -27 | Lower | -42 | Lower |
| Blank Lines | **10 ✅** | 12 | 16 | -2 | Lower | -6 | Lower | -4 | Lower |
| Functions | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |
| Closures | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 3 | 13 | -2 | Lower | -12 | Lower | -10 | Lower |

## p00035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.8436 ✅** | 36.6111 | 27.3231 | +3.2326 | Higher | +12.5206 | Higher | +9.2880 | Higher |
| MI (SEI) | **22.0851 ✅** | 19.9907 | 3.6930 | +2.0943 | Higher | +18.3921 | Higher | +16.2978 | Higher |
| Cyclomatic (sum) | **26 ✅** | 31 | 53 | -5 | Lower | -27 | Lower | -22 | Lower |
| Cognitive (sum) | **38 ✅** | 52 | 65 | -14 | Lower | -27 | Lower | -13 | Lower |
| Halstead Volume | 3517.0993 | **3495.8880 ✅** | 5265.1782 | +21.2113 | Higher | -1748.0789 | Lower | -1769.2901 | Lower |
| Halstead Effort | **153761.6360 ✅** | 215491.8148 | 316170.6983 | -61730.1788 | Lower | -162409.0623 | Lower | -100678.8836 | Lower |
| SLOC | **165 ✅** | 188 | 214 | -23 | Lower | -49 | Lower | -26 | Lower |
| PLOC | **101 ✅** | 104 | 144 | -3 | Lower | -43 | Lower | -40 | Lower |
| LLOC | 79 | **71 ✅** | 112 | +8 | Higher | -33 | Lower | -41 | Lower |
| Comments (CLOC) | 50 | **66 ✅** | 52 | -16 | Lower | -2 | Lower | +14 | Higher |
| Blank Lines | **14 ✅** | 20 | 22 | -6 | Lower | -8 | Lower | -2 | Lower |
| Functions | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | **2 ✅** | 4 | 9 | -2 | Lower | -7 | Lower | -5 | Lower |

## p00036

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.5568 | 32.8804 | **55.6785 ✅** | +7.6764 | Higher | -15.1217 | Lower | -22.7982 | Lower |
| MI (SEI) | 25.1050 | 18.1301 | **35.2377 ✅** | +6.9749 | Higher | -10.1328 | Lower | -17.1077 | Lower |
| Cyclomatic (sum) | 30 | 37 | **21 ✅** | -7 | Lower | +9 | Higher | +16 | Higher |
| Cognitive (sum) | 37 | 36 | **35 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Halstead Volume | 2428.8900 | 3012.2198 | **1648.7771 ✅** | -583.3298 | Lower | +780.1129 | Higher | +1363.4427 | Higher |
| Halstead Effort | 96926.4604 | 118982.6835 | **53860.0530 ✅** | -22056.2232 | Lower | +43066.4074 | Higher | +65122.6306 | Higher |
| SLOC | 168 | 228 | **85 ✅** | -60 | Lower | +83 | Higher | +143 | Higher |
| PLOC | 98 | 115 | **64 ✅** | -17 | Lower | +34 | Higher | +51 | Higher |
| LLOC | 56 | 49 | **35 ✅** | +7 | Higher | +21 | Higher | +14 | Higher |
| Comments (CLOC) | 57 | **99 ✅** | 13 | -42 | Lower | +44 | Higher | +86 | Higher |
| Blank Lines | 14 | 14 | **10 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p00037

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.2292 | 45.1696 | **55.6744 ✅** | +7.0596 | Higher | -3.4452 | Lower | -10.5048 | Lower |
| MI (SEI) | 39.0785 | 28.0439 | **40.7939 ✅** | +11.0345 | Higher | -1.7154 | Lower | -12.7500 | Lower |
| Cyclomatic (sum) | 17 | 28 | **15 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| Cognitive (sum) | 19 | 23 | **17 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Halstead Volume | 2164.7110 | 2745.2584 | **2074.6025 ✅** | -580.5475 | Lower | +90.1085 | Higher | +670.6559 | Higher |
| Halstead Effort | 112402.6180 | 131772.4050 | **102119.5785 ✅** | -19369.7871 | Lower | +10283.0395 | Higher | +29652.8265 | Higher |
| SLOC | 102 | 125 | **86 ✅** | -23 | Lower | +16 | Higher | +39 | Higher |
| PLOC | 60 | 80 | **53 ✅** | -20 | Lower | +7 | Higher | +27 | Higher |
| LLOC | 51 | 56 | **46 ✅** | -5 | Lower | +5 | Higher | +10 | Higher |
| Comments (CLOC) | 31 | **33 ✅** | 21 | -2 | Lower | +10 | Higher | +12 | Higher |
| Blank Lines | 12 | 16 | 12 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **32.7466 ✅** | 26.8433 | 30.2202 | +5.9033 | Higher | +2.5264 | Higher | -3.3769 | Lower |
| MI (SEI) | **14.7171 ✅** | 5.8637 | 4.4810 | +8.8534 | Higher | +10.2362 | Higher | +1.3828 | Higher |
| Cyclomatic (sum) | **44 ✅** | 55 | 52 | -11 | Lower | -8 | Lower | +3 | Higher |
| Cognitive (sum) | 59 | **46 ✅** | 78 | +13 | Higher | -19 | Lower | -32 | Lower |
| Halstead Volume | **4001.9652 ✅** | 4780.9455 | 4642.3986 | -778.9803 | Lower | -640.4334 | Lower | +138.5469 | Higher |
| Halstead Effort | **235115.4551 ✅** | 306228.8698 | 356931.9551 | -71113.4147 | Lower | -121816.5000 | Lower | -50703.0853 | Lower |
| SLOC | 190 | 221 | **189 ✅** | -31 | Lower | +1 | Higher | +32 | Higher |
| PLOC | **118 ✅** | 132 | 145 | -14 | Lower | -27 | Lower | -13 | Lower |
| LLOC | **87 ✅** | 90 | 98 | -3 | Lower | -11 | Lower | -8 | Lower |
| Comments (CLOC) | 62 | **65 ✅** | 36 | -3 | Lower | +26 | Higher | +29 | Higher |
| Blank Lines | 17 | 27 | **11 ✅** | -10 | Lower | +6 | Higher | +16 | Higher |
| Functions | 4 | 6 | **3 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 10 | 18 | **9 ✅** | -8 | Lower | +1 | Higher | +9 | Higher |

## p00039

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.3146 | 62.0762 | **67.2221 ✅** | -1.7617 | Lower | -6.9075 | Lower | -5.1458 | Lower |
| MI (SEI) | 55.9427 | 52.7231 | **59.4046 ✅** | +3.2196 | Higher | -3.4619 | Lower | -6.6815 | Lower |
| Cyclomatic (sum) | 15 | 16 | **14 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 16 | **14 ✅** | 15 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 1064.7584 | 1204.4736 | **1063.2403 ✅** | -139.7152 | Lower | +1.5181 | Higher | +141.2333 | Higher |
| Halstead Effort | 25919.2614 | 30111.8391 | **25872.1808 ✅** | -4192.5776 | Lower | +47.0806 | Higher | +4239.6583 | Higher |
| SLOC | 80 | 68 | **53 ✅** | +12 | Higher | +27 | Higher | +15 | Higher |
| PLOC | 38 | 40 | **33 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| LLOC | 26 | 26 | **22 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Comments (CLOC) | **36 ✅** | 20 | 15 | +16 | Higher | +21 | Higher | +5 | Higher |
| Blank Lines | 7 | 9 | **6 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00040

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.8080 | 37.6178 | **47.7032 ✅** | -0.8098 | Lower | -10.8952 | Lower | -10.0854 | Lower |
| MI (SEI) | 14.8962 | 20.2990 | **29.6371 ✅** | -5.4028 | Lower | -14.7409 | Lower | -9.3381 | Lower |
| Cyclomatic (sum) | 42 | 30 | **26 ✅** | +12 | Higher | +16 | Higher | +4 | Higher |
| Cognitive (sum) | 45 | 49 | **26 ✅** | -4 | Lower | +19 | Higher | +23 | Higher |
| Halstead Volume | 4181.4512 | 3569.7476 | **3267.7817 ✅** | +611.7036 | Higher | +913.6695 | Higher | +301.9658 | Higher |
| Halstead Effort | 254023.1597 | **167268.1713 ✅** | 168593.3315 | +86754.9884 | Higher | +85429.8282 | Higher | -1325.1602 | Lower |
| SLOC | 150 | 178 | **104 ✅** | -28 | Lower | +46 | Higher | +74 | Higher |
| PLOC | 104 | 101 | **70 ✅** | +3 | Higher | +34 | Higher | +31 | Higher |
| LLOC | 60 | 72 | **37 ✅** | -12 | Lower | +23 | Higher | +35 | Higher |
| Comments (CLOC) | 33 | **58 ✅** | 24 | -25 | Lower | +9 | Higher | +34 | Higher |
| Blank Lines | 14 | 20 | **10 ✅** | -6 | Lower | +4 | Higher | +10 | Higher |
| Functions | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 10 | 7 | **6 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |

## p00042

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 33.9682 | **40.7077 ✅** | 34.5360 | -6.7395 | Lower | -0.5678 | Lower | +6.1717 | Higher |
| MI (SEI) | 13.3845 | **22.2245 ✅** | 9.0908 | -8.8400 | Lower | +4.2937 | Higher | +13.1337 | Higher |
| Cyclomatic (sum) | 41 | **25 ✅** | 37 | +16 | Higher | +4 | Higher | -12 | Lower |
| Cognitive (sum) | 72 | **28 ✅** | 56 | +44 | Higher | +16 | Higher | -28 | Lower |
| Halstead Volume | 3796.7131 | **3707.8068 ✅** | 4201.1510 | +88.9062 | Higher | -404.4379 | Lower | -493.3441 | Lower |
| Halstead Effort | 208111.3565 | **150960.7072 ✅** | 279765.5360 | +57150.6493 | Higher | -71654.1795 | Lower | -128804.8288 | Lower |
| SLOC | 187 | **156 ✅** | 185 | +31 | Higher | +2 | Higher | -29 | Lower |
| PLOC | 126 | **95 ✅** | 131 | +31 | Higher | -5 | Lower | -36 | Lower |
| LLOC | 85 | **61 ✅** | 105 | +24 | Higher | -20 | Lower | -44 | Lower |
| Comments (CLOC) | **50 ✅** | 44 | 35 | +6 | Higher | +15 | Higher | +9 | Higher |
| Blank Lines | **12 ✅** | 19 | 19 | -7 | Lower | -7 | Lower | 0 | Same |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | 9 | **1 ✅** | 4 | +8 | Higher | +5 | Higher | -3 | Lower |

## p00043

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.0223 | **44.6735 ✅** | 39.3212 | -6.6512 | Lower | -1.2989 | Lower | +5.3523 | Higher |
| MI (SEI) | 15.7541 | **26.9935 ✅** | 9.5470 | -11.2393 | Lower | +6.2071 | Higher | +17.4464 | Higher |
| Cyclomatic (sum) | 37 | **30 ✅** | 32 | +7 | Higher | +5 | Higher | -2 | Lower |
| Cognitive (sum) | 92 | 69 | **64 ✅** | +23 | Higher | +28 | Higher | +5 | Higher |
| Halstead Volume | 3444.4407 | **2696.6417 ✅** | 3697.5229 | +747.7990 | Higher | -253.0822 | Lower | -1000.8812 | Lower |
| Halstead Effort | 175436.8470 | **133668.8268 ✅** | 228144.2717 | +41768.0202 | Higher | -52707.4247 | Lower | -94475.4449 | Lower |
| SLOC | 159 | **126 ✅** | 154 | +33 | Higher | +5 | Higher | -28 | Lower |
| PLOC | 116 | **78 ✅** | 126 | +38 | Higher | -10 | Lower | -48 | Lower |
| LLOC | 73 | **56 ✅** | 78 | +17 | Higher | -5 | Lower | -22 | Lower |
| Comments (CLOC) | **34 ✅** | 32 | 18 | +2 | Higher | +16 | Higher | +14 | Higher |
| Blank Lines | **9 ✅** | 17 | 10 | -8 | Lower | -1 | Lower | +7 | Higher |
| Functions | 4 | 2 | 2 | +2 | Higher | +2 | Higher | 0 | Same |
| Closures | **0 ✅** | 1 | 2 | -1 | Lower | -2 | Lower | -1 | Lower |
| Exits | 8 | **3 ✅** | 8 | +5 | Higher | 0 | Same | -5 | Lower |

## p00044

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.4467 | 42.9999 | **61.1766 ✅** | +9.4467 | Higher | -8.7300 | Lower | -18.1767 | Lower |
| MI (SEI) | 44.9867 | 28.8268 | **55.8932 ✅** | +16.1599 | Higher | -10.9064 | Lower | -27.0663 | Lower |
| Cyclomatic (sum) | 16 | 27 | **11 ✅** | -11 | Lower | +5 | Higher | +16 | Higher |
| Cognitive (sum) | 21 | 51 | **17 ✅** | -30 | Lower | +4 | Higher | +34 | Higher |
| Halstead Volume | 1577.1832 | 2742.7872 | **1263.2471 ✅** | -1165.6040 | Lower | +313.9361 | Higher | +1479.5402 | Higher |
| Halstead Effort | 49503.8374 | 181743.9379 | **43136.1714 ✅** | -132240.1005 | Lower | +6367.6659 | Higher | +138607.7664 | Higher |
| SLOC | 113 | 145 | **76 ✅** | -32 | Lower | +37 | Higher | +69 | Higher |
| PLOC | 54 | 86 | **38 ✅** | -32 | Lower | +16 | Higher | +48 | Higher |
| LLOC | 35 | 54 | **29 ✅** | -19 | Lower | +6 | Higher | +25 | Higher |
| Comments (CLOC) | **52 ✅** | 51 | 32 | +1 | Higher | +20 | Higher | +19 | Higher |
| Blank Lines | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.9483 | 39.3539 | **45.0188 ✅** | +1.5944 | Higher | -4.0705 | Lower | -5.6649 | Lower |
| MI (SEI) | 27.2247 | 25.7913 | **31.3475 ✅** | +1.4334 | Higher | -4.1228 | Lower | -5.5562 | Lower |
| Cyclomatic (sum) | 28 | 27 | **23 ✅** | +1 | Higher | +5 | Higher | +4 | Higher |
| Cognitive (sum) | **19 ✅** | 31 | 33 | -12 | Lower | -14 | Lower | -2 | Lower |
| Halstead Volume | 2810.0380 | 2868.7499 | **2369.7701 ✅** | -58.7119 | Lower | +440.2679 | Higher | +498.9798 | Higher |
| Halstead Effort | 143873.9449 | **136086.3239 ✅** | 146368.1552 | +7787.6210 | Higher | -2494.2103 | Lower | -10281.8313 | Lower |
| SLOC | 161 | 179 | **142 ✅** | -18 | Lower | +19 | Higher | +37 | Higher |
| PLOC | 84 | 93 | **76 ✅** | -9 | Lower | +8 | Higher | +17 | Higher |
| LLOC | **55 ✅** | 68 | 79 | -13 | Lower | -24 | Lower | -11 | Lower |
| Comments (CLOC) | 62 | **74 ✅** | 50 | -12 | Lower | +12 | Higher | +24 | Higher |
| Blank Lines | 20 | **15 ✅** | 16 | +5 | Higher | +4 | Higher | -1 | Lower |
| Functions | 3 | 3 | **1 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | 6 | **0 ✅** | +3 | Higher | +9 | Higher | +6 | Higher |

## p00046

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.9671 | 52.0452 | **71.1346 ✅** | +14.9218 | Higher | -4.1675 | Lower | -19.0894 | Lower |
| MI (SEI) | 60.8414 | 39.2275 | **66.5617 ✅** | +21.6139 | Higher | -5.7203 | Lower | -27.3342 | Lower |
| Cyclomatic (sum) | **7 ✅** | 16 | 9 | -9 | Lower | -2 | Lower | +7 | Higher |
| Cognitive (sum) | **7 ✅** | 37 | 13 | -30 | Lower | -6 | Lower | +24 | Higher |
| Halstead Volume | 933.6510 | 1801.2294 | **663.2550 ✅** | -867.5784 | Lower | +270.3960 | Higher | +1137.9744 | Higher |
| Halstead Effort | 21865.5049 | 64844.2601 | **12327.8926 ✅** | -42978.7552 | Lower | +9537.6122 | Higher | +52516.3675 | Higher |
| SLOC | 62 | 111 | **52 ✅** | -49 | Lower | +10 | Higher | +59 | Higher |
| PLOC | 34 | 68 | **31 ✅** | -34 | Lower | +3 | Higher | +37 | Higher |
| LLOC | 24 | 53 | **22 ✅** | -29 | Lower | +2 | Higher | +31 | Higher |
| Comments (CLOC) | 21 | **35 ✅** | 17 | -14 | Lower | +4 | Higher | +18 | Higher |
| Blank Lines | 7 | 9 | **4 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00047

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.7735 | **65.8703 ✅** | 56.2880 | -7.0968 | Lower | +2.4855 | Higher | +9.5823 | Higher |
| MI (SEI) | 52.8376 | **58.6084 ✅** | 46.2291 | -5.7708 | Lower | +6.6085 | Higher | +12.3793 | Higher |
| Cyclomatic (sum) | 18 | **15 ✅** | 17 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 12 | **8 ✅** | 12 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | 1038.2544 | **944.5291 ✅** | 1322.5020 | +93.7253 | Higher | -284.2476 | Lower | -377.9729 | Lower |
| Halstead Effort | 24582.2003 | **24198.8354 ✅** | 41210.8692 | +383.3649 | Higher | -16628.6689 | Lower | -17012.0338 | Lower |
| SLOC | 85 | **59 ✅** | 93 | +26 | Higher | -8 | Lower | -34 | Lower |
| PLOC | 41 | **34 ✅** | 53 | +7 | Higher | -12 | Lower | -19 | Lower |
| LLOC | 22 | **19 ✅** | 41 | +3 | Higher | -19 | Lower | -22 | Lower |
| Comments (CLOC) | **35 ✅** | 18 | 31 | +17 | Higher | +4 | Higher | -13 | Lower |
| Blank Lines | 9 | **7 ✅** | 10 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00048

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.7766 | 55.5425 | **67.8180 ✅** | +7.2341 | Higher | -5.0414 | Lower | -12.2755 | Lower |
| MI (SEI) | 59.2993 | 49.8610 | **60.4988 ✅** | +9.4383 | Higher | -1.1995 | Lower | -10.6378 | Lower |
| Cyclomatic (sum) | 16 | 17 | **15 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 16 | **14 ✅** | 15 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 775.7744 | 1077.4809 | **764.1086 ✅** | -301.7066 | Lower | +11.6658 | Higher | +313.3723 | Higher |
| Halstead Effort | 9846.3670 | 18448.8678 | **9698.3014 ✅** | -8602.5008 | Lower | +148.0657 | Higher | +8750.5664 | Higher |
| SLOC | 75 | 104 | **56 ✅** | -29 | Lower | +19 | Higher | +48 | Higher |
| PLOC | 37 | 48 | **35 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| LLOC | 7 | 17 | **6 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| Comments (CLOC) | 33 | **49 ✅** | 16 | -16 | Lower | +17 | Higher | +33 | Higher |
| Blank Lines | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00049

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.4380 | 43.4510 | **51.6427 ✅** | +6.9870 | Higher | -1.2047 | Lower | -8.1917 | Lower |
| MI (SEI) | 38.4912 | 32.5211 | **38.9299 ✅** | +5.9701 | Higher | -0.4388 | Lower | -6.4088 | Lower |
| Cyclomatic (sum) | 28 | 34 | **19 ✅** | -6 | Lower | +9 | Higher | +15 | Higher |
| Cognitive (sum) | 32 | 44 | **26 ✅** | -12 | Lower | +6 | Higher | +18 | Higher |
| Halstead Volume | 1878 | 2472.5415 | **1856.2233 ✅** | -594.5415 | Lower | +21.7767 | Higher | +616.3182 | Higher |
| Halstead Effort | 86337.2432 | 121445.4208 | **81454.0089 ✅** | -35108.1775 | Lower | +4883.2343 | Higher | +39991.4118 | Higher |
| SLOC | **102 ✅** | 132 | 108 | -30 | Lower | -6 | Lower | +24 | Higher |
| PLOC | **58 ✅** | 77 | 60 | -19 | Lower | -2 | Lower | +17 | Higher |
| LLOC | **42 ✅** | 49 | 45 | -7 | Lower | -3 | Lower | +4 | Higher |
| Comments (CLOC) | 33 | **55 ✅** | 34 | -22 | Lower | -1 | Lower | +21 | Higher |
| Blank Lines | **11 ✅** | 12 | 14 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00050

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.9943 ✅** | 51.3751 | 46.4969 | +12.6192 | Higher | +17.4974 | Higher | +4.8782 | Higher |
| MI (SEI) | **61.5298 ✅** | 40.9564 | 27.0881 | +20.5733 | Higher | +34.4417 | Higher | +13.8683 | Higher |
| Cyclomatic (sum) | **12 ✅** | 16 | 30 | -4 | Lower | -18 | Lower | -14 | Lower |
| Cognitive (sum) | **12 ✅** | 15 | 22 | -3 | Lower | -10 | Lower | -7 | Lower |
| Halstead Volume | **994.1233 ✅** | 1835.0249 | 2982.8568 | -840.9016 | Lower | -1988.7335 | Lower | -1147.8319 | Lower |
| Halstead Effort | **33800.1931 ✅** | 76723.8805 | 199425.2827 | -42923.6874 | Lower | -165625.0896 | Lower | -122701.4022 | Lower |
| SLOC | **68 ✅** | 115 | 109 | -47 | Lower | -41 | Lower | +6 | Higher |
| PLOC | **28 ✅** | 62 | 81 | -34 | Lower | -53 | Lower | -19 | Lower |
| LLOC | **17 ✅** | 43 | 63 | -26 | Lower | -46 | Lower | -20 | Lower |
| Comments (CLOC) | 32 | **44 ✅** | 23 | -12 | Lower | +9 | Higher | +21 | Higher |
| Blank Lines | 8 | 10 | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00051

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.6971 | **66.9837 ✅** | 58.8567 | -8.2867 | Lower | -0.1596 | Lower | +8.1271 | Higher |
| MI (SEI) | 50.8878 | **57.9898 ✅** | 45.4530 | -7.1021 | Lower | +5.4348 | Higher | +12.5369 | Higher |
| Cyclomatic (sum) | 10 | **7 ✅** | 16 | +3 | Higher | -6 | Lower | -9 | Lower |
| Cognitive (sum) | 16 | **8 ✅** | 26 | +8 | Higher | -10 | Lower | -18 | Lower |
| Halstead Volume | 1300.6046 | **1145.5841 ✅** | 1459.0020 | +155.0205 | Higher | -158.3974 | Lower | -313.4179 | Lower |
| Halstead Effort | 44287.8282 | **33753.8170 ✅** | 67843.5919 | +10534.0113 | Higher | -23555.7637 | Lower | -34089.7750 | Lower |
| SLOC | 89 | **58 ✅** | 78 | +31 | Higher | +11 | Higher | -20 | Lower |
| PLOC | 43 | **33 ✅** | 50 | +10 | Higher | -7 | Lower | -17 | Lower |
| LLOC | 39 | **29 ✅** | 40 | +10 | Higher | -1 | Lower | -11 | Lower |
| Comments (CLOC) | **34 ✅** | 16 | 19 | +18 | Higher | +15 | Higher | -3 | Lower |
| Blank Lines | 13 | 10 | 10 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |

## p00052

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.1670 ✅** | 70.1887 | 72.6495 | +4.9783 | Higher | +2.5175 | Higher | -2.4608 | Lower |
| MI (SEI) | **72.8636 ✅** | 68.3552 | 63.3811 | +4.5084 | Higher | +9.4825 | Higher | +4.9741 | Higher |
| Cyclomatic (sum) | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | **8 ✅** | 10 | 14 | -2 | Lower | -6 | Lower | -4 | Lower |
| Halstead Volume | **537.2002 ✅** | 631.5631 | 777.6424 | -94.3629 | Lower | -240.4422 | Lower | -146.0793 | Lower |
| Halstead Effort | **9848.6705 ✅** | 13205.4104 | 17108.1337 | -3356.7399 | Lower | -7259.4631 | Lower | -3902.7233 | Lower |
| SLOC | **44 ✅** | 56 | 45 | -12 | Lower | -1 | Lower | +11 | Higher |
| PLOC | **23 ✅** | 27 | 29 | -4 | Lower | -6 | Lower | -2 | Lower |
| LLOC | **15 ✅** | 18 | 21 | -3 | Lower | -6 | Lower | -3 | Lower |
| Comments (CLOC) | 15 | **23 ✅** | 10 | -8 | Lower | +5 | Higher | +13 | Higher |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00053

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.0010 | 54.0650 | **56.0204 ✅** | +0.9359 | Higher | -1.0194 | Lower | -1.9554 | Lower |
| MI (SEI) | **45.0782 ✅** | 40.5540 | 44.8817 | +4.5242 | Higher | +0.1965 | Higher | -4.3277 | Lower |
| Cyclomatic (sum) | 15 | 16 | 15 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | **25 ✅** | 32 | 32 | -7 | Lower | -7 | Lower | 0 | Same |
| Halstead Volume | **1306.3298 ✅** | 1542 | 1423.5606 | -235.6702 | Lower | -117.2309 | Lower | +118.4394 | Higher |
| Halstead Effort | **37111.6410 ✅** | 62587.0588 | 43544.2076 | -25475.4179 | Lower | -6432.5667 | Lower | +19042.8512 | Higher |
| SLOC | 104 | 103 | **95 ✅** | +1 | Higher | +9 | Higher | +8 | Higher |
| PLOC | 54 | 63 | **52 ✅** | -9 | Lower | +2 | Higher | +11 | Higher |
| LLOC | **35 ✅** | 47 | 42 | -12 | Lower | -7 | Lower | +5 | Higher |
| Comments (CLOC) | **37 ✅** | 29 | 30 | +8 | Higher | +7 | Higher | -1 | Lower |
| Blank Lines | 14 | **12 ✅** | 13 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |

## p00054

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.3031 | 53.3037 | **56.9295 ✅** | -5.0006 | Lower | -8.6264 | Lower | -3.6258 | Lower |
| MI (SEI) | 38.2293 | 45.0554 | **45.9369 ✅** | -6.8260 | Lower | -7.7076 | Lower | -0.8816 | Lower |
| Cyclomatic (sum) | 24 | 18 | **13 ✅** | +6 | Higher | +11 | Higher | +5 | Higher |
| Cognitive (sum) | 16 | 9 | **8 ✅** | +7 | Higher | +8 | Higher | +1 | Higher |
| Halstead Volume | 2036.8237 | **1494.1913 ✅** | 1600.0789 | +542.6324 | Higher | +436.7448 | Higher | -105.8876 | Lower |
| Halstead Effort | 82830.8309 | **42480.6897 ✅** | 44240.0196 | +40350.1412 | Higher | +38590.8114 | Higher | -1759.3298 | Lower |
| SLOC | 120 | 106 | **89 ✅** | +14 | Higher | +31 | Higher | +17 | Higher |
| PLOC | 63 | 50 | 50 | +13 | Higher | +13 | Higher | 0 | Same |
| LLOC | 30 | **28 ✅** | 39 | +2 | Higher | -9 | Lower | -11 | Lower |
| Comments (CLOC) | **49 ✅** | 44 | 28 | +5 | Higher | +21 | Higher | +16 | Higher |
| Blank Lines | **9 ✅** | 12 | 13 | -3 | Lower | -4 | Lower | -1 | Lower |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.3229 | **54.7243 ✅** | 51.8155 | -5.4014 | Lower | -2.4926 | Lower | +2.9088 | Higher |
| MI (SEI) | **39.2750 ✅** | 37.5057 | 37.4252 | +1.7693 | Higher | +1.8497 | Higher | +0.0805 | Higher |
| Cyclomatic (sum) | 19 | 20 | 19 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 44 | **40 ✅** | 45 | +4 | Higher | -1 | Lower | -5 | Lower |
| Halstead Volume | 1667.1650 | **1564.4780 ✅** | 1795.5624 | +102.6870 | Higher | -128.3974 | Lower | -231.0845 | Lower |
| Halstead Effort | 65644.6211 | **62917.3835 ✅** | 75366.3692 | +2727.2376 | Higher | -9721.7481 | Lower | -12448.9857 | Lower |
| SLOC | 129 | **93 ✅** | 108 | +36 | Higher | +21 | Higher | -15 | Lower |
| PLOC | **62 ✅** | 65 | 67 | -3 | Lower | -5 | Lower | -2 | Lower |
| LLOC | **45 ✅** | 46 | 49 | -1 | Lower | -4 | Lower | -3 | Lower |
| Comments (CLOC) | **53 ✅** | 19 | 30 | +34 | Higher | +23 | Higher | -11 | Lower |
| Blank Lines | 14 | **9 ✅** | 13 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00057

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.3386 | 34.7498 | **81.0367 ✅** | +17.5888 | Higher | -28.6981 | Lower | -46.2869 | Lower |
| MI (SEI) | 42.2508 | 19.3175 | **82.0275 ✅** | +22.9333 | Higher | -39.7766 | Lower | -62.7099 | Lower |
| Cyclomatic (sum) | 20 | 34 | **6 ✅** | -14 | Lower | +14 | Higher | +28 | Higher |
| Cognitive (sum) | 21 | 33 | **5 ✅** | -12 | Lower | +16 | Higher | +28 | Higher |
| Halstead Volume | 1800.6880 | 2924.9543 | **465.1154 ✅** | -1124.2663 | Lower | +1335.5726 | Higher | +2459.8389 | Higher |
| Halstead Effort | 84117.8526 | 150178.1207 | **5338.7158 ✅** | -66060.2681 | Lower | +78779.1368 | Higher | +144839.4048 | Higher |
| SLOC | 103 | 214 | **33 ✅** | -111 | Lower | +70 | Higher | +181 | Higher |
| PLOC | 53 | 115 | **16 ✅** | -62 | Lower | +37 | Higher | +99 | Higher |
| LLOC | 44 | 76 | **9 ✅** | -32 | Lower | +35 | Higher | +67 | Higher |
| Comments (CLOC) | 38 | **85 ✅** | 12 | -47 | Lower | +26 | Higher | +73 | Higher |
| Blank Lines | 12 | 14 | **5 ✅** | -2 | Lower | +7 | Higher | +9 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 7 | 2 | **0 ✅** | +5 | Higher | +7 | Higher | +2 | Higher |

## p00058

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.2530 | 59.7762 | **71.2404 ✅** | +8.4768 | Higher | -2.9873 | Lower | -11.4641 | Lower |
| MI (SEI) | 59.4196 | 50.7226 | **61.2464 ✅** | +8.6970 | Higher | -1.8268 | Lower | -10.5238 | Lower |
| Cyclomatic (sum) | 9 | 10 | **8 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 10 | **7 ✅** | 9 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 1087.8649 | 1473.1782 | **1065.8011 ✅** | -385.3133 | Lower | +22.0638 | Higher | +407.3771 | Higher |
| Halstead Effort | 22426.7543 | 39343.4668 | **21056.4046 ✅** | -16916.7126 | Lower | +1370.3497 | Higher | +18287.0623 | Higher |
| SLOC | 53 | 80 | **45 ✅** | -27 | Lower | +8 | Higher | +35 | Higher |
| PLOC | 33 | 43 | **31 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| LLOC | 21 | 31 | **20 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| Comments (CLOC) | 14 | **27 ✅** | 10 | -13 | Lower | +4 | Higher | +17 | Higher |
| Blank Lines | 6 | 10 | **4 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00059

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.9754 ✅** | 60.5477 | 57.8781 | +2.4277 | Higher | +5.0973 | Higher | +2.6696 | Higher |
| MI (SEI) | **48.4475 ✅** | 44.0685 | 38.9054 | +4.3790 | Higher | +9.5421 | Higher | +5.1631 | Higher |
| Cyclomatic (sum) | **18 ✅** | 19 | 27 | -1 | Lower | -9 | Lower | -8 | Lower |
| Cognitive (sum) | **12 ✅** | 13 | 32 | -1 | Lower | -20 | Lower | -19 | Lower |
| Halstead Volume | 1607.0352 | **1482.0891 ✅** | 1821.8321 | +124.9462 | Higher | -214.7969 | Lower | -339.7430 | Lower |
| Halstead Effort | 41356.5595 | **36246.7432 ✅** | 85853.8372 | +5109.8163 | Higher | -44497.2777 | Lower | -49607.0940 | Lower |
| SLOC | **57 ✅** | 67 | 66 | -10 | Lower | -9 | Lower | +1 | Higher |
| PLOC | **39 ✅** | 48 | 50 | -9 | Lower | -11 | Lower | -2 | Lower |
| LLOC | **23 ✅** | 32 | 37 | -9 | Lower | -14 | Lower | -5 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 10 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 9 | 7 | **6 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | 3 | **1 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |

## p00060

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8107 | **60.6011 ✅** | 59.6391 | -0.7903 | Lower | +0.1716 | Higher | +0.9620 | Higher |
| MI (SEI) | **45.6893 ✅** | 44.6467 | 39.2919 | +1.0426 | Higher | +6.3974 | Higher | +5.3548 | Higher |
| Cyclomatic (sum) | 22 | 22 | 22 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 16 | **13 ✅** | 16 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | **1304.7622 ✅** | 1346.2774 | 1476 | -41.5152 | Lower | -171.2378 | Lower | -129.7226 | Lower |
| Halstead Effort | **38855.0512 ✅** | 41160.4509 | 42103.8462 | -2305.3997 | Lower | -3248.7949 | Lower | -943.3952 | Lower |
| SLOC | 70 | **66 ✅** | 68 | +4 | Higher | +2 | Higher | -2 | Lower |
| PLOC | 47 | **46 ✅** | 51 | +1 | Higher | -4 | Lower | -5 | Lower |
| LLOC | 20 | 20 | 25 | 0 | Same | -5 | Lower | -5 | Lower |
| Comments (CLOC) | **15 ✅** | 12 | 9 | +3 | Higher | +6 | Higher | +3 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.5490 | 27.8937 | **43.7337 ✅** | +15.6553 | Higher | -0.1848 | Lower | -15.8401 | Lower |
| MI (SEI) | **25.2548 ✅** | 6.0740 | 22.6850 | +19.1808 | Higher | +2.5698 | Higher | -16.6110 | Lower |
| Cyclomatic (sum) | 31 | 48 | **28 ✅** | -17 | Lower | +3 | Higher | +20 | Higher |
| Cognitive (sum) | **34 ✅** | 53 | 37 | -19 | Lower | -3 | Lower | +16 | Higher |
| Halstead Volume | **3049.4679 ✅** | 4897.9261 | 3202.1429 | -1848.4582 | Lower | -152.6751 | Lower | +1695.7832 | Higher |
| Halstead Effort | 184459.6586 | 321124.5909 | **178854.2371 ✅** | -136664.9323 | Lower | +5605.4215 | Higher | +142270.3538 | Higher |
| SLOC | **128 ✅** | 227 | 130 | -99 | Lower | -2 | Lower | +97 | Higher |
| PLOC | **90 ✅** | 154 | 92 | -64 | Lower | -2 | Lower | +62 | Higher |
| LLOC | **65 ✅** | 111 | 81 | -46 | Lower | -16 | Lower | +30 | Higher |
| Comments (CLOC) | 32 | **64 ✅** | 27 | -32 | Lower | +5 | Higher | +37 | Higher |
| Blank Lines | **9 ✅** | 17 | 13 | -8 | Lower | -4 | Lower | +4 | Higher |
| Functions | **1 ✅** | 4 | 3 | -3 | Lower | -2 | Lower | +1 | Higher |
| Closures | 4 | 1 | **0 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Exits | 7 | 11 | **4 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |

## p00062

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.0388 | 61.6481 | **67.6761 ✅** | +2.3907 | Higher | -3.6373 | Lower | -6.0280 | Lower |
| MI (SEI) | 54.8458 | 57.2180 | **64.0181 ✅** | -2.3722 | Lower | -9.1723 | Lower | -6.8001 | Lower |
| Cyclomatic (sum) | **7 ✅** | 9 | 9 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **9 ✅** | 12 | 14 | -3 | Lower | -5 | Lower | -2 | Lower |
| Halstead Volume | 1287.7021 | 1162.4873 | **870.2800 ✅** | +125.2149 | Higher | +417.4221 | Higher | +292.2072 | Higher |
| Halstead Effort | 35620.6257 | 37199.5928 | **20564.3947 ✅** | -1578.9671 | Lower | +15056.2310 | Higher | +16635.1981 | Higher |
| SLOC | 67 | 78 | **59 ✅** | -11 | Lower | +8 | Higher | +19 | Higher |
| PLOC | 39 | 34 | **29 ✅** | +5 | Higher | +10 | Higher | +5 | Higher |
| LLOC | 18 | 28 | 18 | -10 | Lower | 0 | Same | +10 | Higher |
| Comments (CLOC) | 20 | **35 ✅** | 23 | -15 | Lower | -3 | Lower | +12 | Higher |
| Blank Lines | 8 | 9 | **7 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00063

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.3216 | 43.0179 | **69.7118 ✅** | +19.3037 | Higher | -7.3902 | Lower | -26.6939 | Lower |
| MI (SEI) | 57.5056 | 29.5084 | **62.9244 ✅** | +27.9972 | Higher | -5.4188 | Lower | -33.4160 | Lower |
| Cyclomatic (sum) | 12 | 28 | **8 ✅** | -16 | Lower | +4 | Higher | +20 | Higher |
| Cognitive (sum) | 15 | 27 | **7 ✅** | -12 | Lower | +8 | Higher | +20 | Higher |
| Halstead Volume | 1147.6377 | 2212.1756 | **968.2597 ✅** | -1064.5379 | Lower | +179.3780 | Higher | +1243.9159 | Higher |
| Halstead Effort | 36218.0971 | 80633.8024 | **20421.4782 ✅** | -44415.7053 | Lower | +15796.6189 | Higher | +60212.3242 | Higher |
| SLOC | 72 | 153 | **51 ✅** | -81 | Lower | +21 | Higher | +102 | Higher |
| PLOC | 38 | 91 | **27 ✅** | -53 | Lower | +11 | Higher | +64 | Higher |
| LLOC | 26 | 52 | **15 ✅** | -26 | Lower | +11 | Higher | +37 | Higher |
| Comments (CLOC) | 30 | **56 ✅** | 15 | -26 | Lower | +15 | Higher | +41 | Higher |
| Blank Lines | **7 ✅** | 15 | 10 | -8 | Lower | -3 | Lower | +5 | Higher |
| Functions | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | **1 ✅** | 3 | +3 | Higher | +1 | Higher | -2 | Lower |

## p00064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.4009 | 62.5477 | **70.0939 ✅** | +3.8531 | Higher | -3.6931 | Lower | -7.5462 | Lower |
| MI (SEI) | **65.0003 ✅** | 54.4559 | 62.9905 | +10.5445 | Higher | +2.0098 | Higher | -8.5346 | Lower |
| Cyclomatic (sum) | **8 ✅** | 11 | 10 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | **17 ✅** | 19 | 19 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | **682.4384 ✅** | 1100.1849 | 875.8947 | -417.7465 | Lower | -193.4563 | Lower | +224.2901 | Higher |
| Halstead Effort | **10934.5242 ✅** | 27027.8749 | 25546.9298 | -16093.3506 | Lower | -14612.4056 | Lower | +1480.9451 | Higher |
| SLOC | 70 | 73 | **50 ✅** | -3 | Lower | +20 | Higher | +23 | Higher |
| PLOC | **29 ✅** | 40 | 31 | -11 | Lower | -2 | Lower | +9 | Higher |
| LLOC | 25 | 31 | 25 | -6 | Lower | 0 | Same | +6 | Higher |
| Comments (CLOC) | **34 ✅** | 24 | 14 | +10 | Higher | +20 | Higher | +10 | Higher |
| Blank Lines | 7 | 9 | **5 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00065

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.6740 | 38.0201 | **40.0996 ✅** | -2.3461 | Lower | -4.4256 | Lower | -2.0795 | Lower |
| MI (SEI) | 14.2499 | 20.5314 | **22.3318 ✅** | -6.2815 | Lower | -8.0819 | Lower | -1.8003 | Lower |
| Cyclomatic (sum) | 36 | 40 | **32 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| Cognitive (sum) | **38 ✅** | 39 | 39 | -1 | Lower | -1 | Lower | 0 | Same |
| Halstead Volume | 3712.4397 | **3017.6918 ✅** | 3248.7339 | +694.7479 | Higher | +463.7059 | Higher | -231.0421 | Lower |
| Halstead Effort | 177705.7536 | **174422.5848 ✅** | 183777.5134 | +3283.1688 | Higher | -6071.7597 | Lower | -9354.9286 | Lower |
| SLOC | 182 | 159 | **153 ✅** | +23 | Higher | +29 | Higher | +6 | Higher |
| PLOC | 114 | 99 | **93 ✅** | +15 | Higher | +21 | Higher | +6 | Higher |
| LLOC | 78 | **72 ✅** | 74 | +6 | Higher | +4 | Higher | -2 | Lower |
| Comments (CLOC) | 45 | **47 ✅** | 44 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 24 | **15 ✅** | 19 | +9 | Higher | +5 | Higher | -4 | Lower |
| Functions | 5 | 3 | 3 | +2 | Higher | +2 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **3 ✅** | 4 | +6 | Higher | +5 | Higher | -1 | Lower |

## p00066

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.9562 | **60.3177 ✅** | 49.6640 | -8.3616 | Lower | +2.2922 | Higher | +10.6537 | Higher |
| MI (SEI) | 42.4616 | **55.0519 ✅** | 35.7890 | -12.5903 | Lower | +6.6726 | Higher | +19.2629 | Higher |
| Cyclomatic (sum) | 31 | **11 ✅** | 34 | +20 | Higher | -3 | Lower | -23 | Lower |
| Cognitive (sum) | 42 | **17 ✅** | 51 | +25 | Higher | -9 | Lower | -34 | Lower |
| Halstead Volume | 1878.1906 | **1132.4342 ✅** | 2155.7370 | +745.7564 | Higher | -277.5464 | Lower | -1023.3028 | Lower |
| Halstead Effort | 90843.6593 | **24120.8487 ✅** | 111205.2333 | +66722.8106 | Higher | -20361.5740 | Lower | -87084.3846 | Lower |
| SLOC | 89 | **83 ✅** | 94 | +6 | Higher | -5 | Lower | -11 | Lower |
| PLOC | 50 | **39 ✅** | 61 | +11 | Higher | -11 | Lower | -22 | Lower |
| LLOC | **17 ✅** | 19 | 30 | -2 | Lower | -13 | Lower | -11 | Lower |
| Comments (CLOC) | 32 | **36 ✅** | 26 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 7 | 8 | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00067

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.9104 ✅** | 53.2344 | 42.0438 | +2.6760 | Higher | +13.8665 | Higher | +11.1905 | Higher |
| MI (SEI) | **43.6036 ✅** | 43.5090 | 22.7737 | +0.0946 | Higher | +20.8300 | Higher | +20.7354 | Higher |
| Cyclomatic (sum) | 17 | 17 | 33 | 0 | Same | -16 | Lower | -16 | Lower |
| Cognitive (sum) | 55 | **26 ✅** | 35 | +29 | Higher | +20 | Higher | -9 | Lower |
| Halstead Volume | **1750.5337 ✅** | 1839.8467 | 3016.9931 | -89.3129 | Lower | -1266.4593 | Lower | -1177.1464 | Lower |
| Halstead Effort | **80009.6880 ✅** | 89394.9024 | 194136.9451 | -9385.2144 | Lower | -114127.2570 | Lower | -104742.0426 | Lower |
| SLOC | **87 ✅** | 101 | 137 | -14 | Lower | -50 | Lower | -36 | Lower |
| PLOC | 55 | **52 ✅** | 91 | +3 | Higher | -36 | Lower | -39 | Lower |
| LLOC | 42 | **41 ✅** | 64 | +1 | Higher | -22 | Lower | -23 | Lower |
| Comments (CLOC) | 25 | **38 ✅** | 33 | -13 | Lower | -8 | Lower | +5 | Higher |
| Blank Lines | **7 ✅** | 11 | 14 | -4 | Lower | -7 | Lower | -3 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |

## p00068

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.7128 | **38.8687 ✅** | 11.8243 | -6.1558 | Lower | +20.8885 | Higher | +27.0443 | Higher |
| MI (SEI) | 8.7456 | **18.9304 ✅** | -22.8379 | -10.1847 | Lower | +31.5835 | Higher | +41.7683 | Higher |
| Cyclomatic (sum) | 47 | **26 ✅** | 83 | +21 | Higher | -36 | Lower | -57 | Lower |
| Cognitive (sum) | 42 | **28 ✅** | 113 | +14 | Higher | -71 | Lower | -85 | Lower |
| Halstead Volume | 4723.7262 | **3727.3205 ✅** | 8849.2261 | +996.4057 | Higher | -4125.4999 | Lower | -5121.9056 | Lower |
| Halstead Effort | 265414.3649 | **172272.0954 ✅** | 790513.7135 | +93142.2695 | Higher | -525099.3487 | Lower | -618241.6181 | Lower |
| SLOC | 173 | **172 ✅** | 308 | +1 | Higher | -135 | Lower | -136 | Lower |
| PLOC | 130 | **111 ✅** | 249 | +19 | Higher | -119 | Lower | -138 | Lower |
| LLOC | 62 | **59 ✅** | 164 | +3 | Higher | -102 | Lower | -105 | Lower |
| Comments (CLOC) | 36 | **46 ✅** | 43 | -10 | Lower | -7 | Lower | +3 | Higher |
| Blank Lines | **12 ✅** | 16 | 17 | -4 | Lower | -5 | Lower | -1 | Lower |
| Functions | 4 | **2 ✅** | 7 | +2 | Higher | -3 | Lower | -5 | Lower |
| Closures | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 17 | **1 ✅** | 16 | +16 | Higher | +1 | Higher | -15 | Lower |

## p00069

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.5899 | **46.6834 ✅** | 41.3149 | -7.0935 | Lower | -1.7250 | Lower | +5.3685 | Higher |
| MI (SEI) | 16.1622 | **26.8183 ✅** | 16.1212 | -10.6562 | Lower | +0.0410 | Higher | +10.6971 | Higher |
| Cyclomatic (sum) | 36 | 33 | **27 ✅** | +3 | Higher | +9 | Higher | +6 | Higher |
| Cognitive (sum) | **50 ✅** | 51 | 52 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 3626.3942 | **2749.0881 ✅** | 3792.4920 | +877.3061 | Higher | -166.0978 | Lower | -1043.4039 | Lower |
| Halstead Effort | 196173.9786 | **154948.6023 ✅** | 215944.4945 | +41225.3763 | Higher | -19770.5159 | Lower | -60995.8922 | Lower |
| SLOC | 144 | **106 ✅** | 145 | +38 | Higher | -1 | Lower | -39 | Lower |
| PLOC | 106 | **77 ✅** | 107 | +29 | Higher | -1 | Lower | -30 | Lower |
| LLOC | 50 | **44 ✅** | 67 | +6 | Higher | -17 | Lower | -23 | Lower |
| Comments (CLOC) | **27 ✅** | 21 | 24 | +6 | Higher | +3 | Higher | -3 | Lower |
| Blank Lines | 14 | **11 ✅** | 14 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | 2 | **0 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p00070

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.5734 | 47.6034 | **59.5499 ✅** | +9.9700 | Higher | -1.9765 | Lower | -11.9464 | Lower |
| MI (SEI) | 46.1057 | 32.9510 | **50.3240 ✅** | +13.1547 | Higher | -4.2183 | Lower | -17.3730 | Lower |
| Cyclomatic (sum) | 17 | 23 | **11 ✅** | -6 | Lower | +6 | Higher | +12 | Higher |
| Cognitive (sum) | 13 | 20 | 13 | -7 | Lower | 0 | Same | +7 | Higher |
| Halstead Volume | 1937.1703 | 2499.8621 | **1800 ✅** | -562.6917 | Lower | +137.1703 | Higher | +699.8621 | Higher |
| Halstead Effort | 63237.8496 | 115465.3276 | **60480 ✅** | -52227.4780 | Lower | +2757.8496 | Higher | +54985.3276 | Higher |
| SLOC | 76 | 119 | **75 ✅** | -43 | Lower | +1 | Higher | +44 | Higher |
| PLOC | 45 | 69 | **39 ✅** | -24 | Lower | +6 | Higher | +30 | Higher |
| LLOC | **23 ✅** | 42 | 26 | -19 | Lower | -3 | Lower | +16 | Higher |
| Comments (CLOC) | 22 | **36 ✅** | 25 | -14 | Lower | -3 | Lower | +11 | Higher |
| Blank Lines | 13 | 16 | **11 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 5 | **0 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |

## p00071

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.0415 | 48.2992 | **48.8282 ✅** | -0.2577 | Lower | -0.7867 | Lower | -0.5290 | Lower |
| MI (SEI) | 23.2708 | **27.8502 ✅** | 27.3547 | -4.5795 | Lower | -4.0839 | Lower | +0.4955 | Higher |
| Cyclomatic (sum) | 21 | **19 ✅** | 22 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | **31 ✅** | 40 | 32 | -9 | Lower | -1 | Lower | +8 | Higher |
| Halstead Volume | 2949.4903 | 2903.3865 | **2875.0716 ✅** | +46.1038 | Higher | +74.4187 | Higher | +28.3149 | Higher |
| Halstead Effort | 130957.3715 | **106563.3581 ✅** | 170234.5052 | +24394.0134 | Higher | -39277.1337 | Lower | -63671.1471 | Lower |
| SLOC | 113 | 115 | **107 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| PLOC | 83 | 80 | 80 | +3 | Higher | +3 | Higher | 0 | Same |
| LLOC | 65 | **46 ✅** | 61 | +19 | Higher | +4 | Higher | -15 | Lower |
| Comments (CLOC) | 16 | **23 ✅** | 19 | -7 | Lower | -3 | Lower | +4 | Higher |
| Blank Lines | 14 | 12 | **8 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00072

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **40.0650 ✅** | 33.3414 | 33.9958 | +6.7236 | Higher | +6.0692 | Higher | -0.6544 | Lower |
| MI (SEI) | **15.5913 ✅** | 9.5913 | 8.2057 | +6.0000 | Higher | +7.3857 | Higher | +1.3857 | Higher |
| Cyclomatic (sum) | **32 ✅** | 37 | 39 | -5 | Lower | -7 | Lower | -2 | Lower |
| Cognitive (sum) | **28 ✅** | 44 | 47 | -16 | Lower | -19 | Lower | -3 | Lower |
| Halstead Volume | **3627.1004 ✅** | 4346.2253 | 4339.1577 | -719.1249 | Lower | -712.0572 | Lower | +7.0677 | Higher |
| Halstead Effort | **186674.7697 ✅** | 273228.3753 | 255781.9254 | -86553.6056 | Lower | -69107.1557 | Lower | +17446.4498 | Higher |
| SLOC | **148 ✅** | 197 | 184 | -49 | Lower | -36 | Lower | +13 | Higher |
| PLOC | **104 ✅** | 134 | 130 | -30 | Lower | -26 | Lower | +4 | Higher |
| LLOC | **64 ✅** | 87 | 91 | -23 | Lower | -27 | Lower | -4 | Lower |
| Comments (CLOC) | 26 | **44 ✅** | 34 | -18 | Lower | -8 | Lower | +10 | Higher |
| Blank Lines | **19 ✅** | 20 | 20 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | 9 | **3 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |

## p00073

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.0878 ✅** | 68.3974 | 72.0136 | +5.6904 | Higher | +2.0742 | Higher | -3.6162 | Lower |
| MI (SEI) | **69.1691 ✅** | 61.3352 | 64.0806 | +7.8339 | Higher | +5.0885 | Higher | -2.7454 | Lower |
| Cyclomatic (sum) | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 7 | **5 ✅** | 8 | +2 | Higher | -1 | Lower | -3 | Lower |
| Halstead Volume | **823.7780 ✅** | 955.0286 | 942.5240 | -131.2506 | Lower | -118.7460 | Lower | +12.5046 | Higher |
| Halstead Effort | **15952.7771 ✅** | 22222.7806 | 24360.6199 | -6270.0035 | Lower | -8407.8428 | Lower | -2137.8393 | Lower |
| SLOC | **41 ✅** | 54 | 44 | -13 | Lower | -3 | Lower | +10 | Higher |
| PLOC | **23 ✅** | 29 | 27 | -6 | Lower | -4 | Lower | +2 | Higher |
| LLOC | **12 ✅** | 17 | 16 | -5 | Lower | -4 | Lower | +1 | Higher |
| Comments (CLOC) | 12 | **16 ✅** | 11 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | **6 ✅** | 9 | 7 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.0307 ✅** | 53.4118 | 59.9018 | +7.6188 | Higher | +1.1289 | Higher | -6.4899 | Lower |
| MI (SEI) | **55.8606 ✅** | 38.5676 | 46.3923 | +17.2930 | Higher | +9.4683 | Higher | -7.8247 | Lower |
| Cyclomatic (sum) | **11 ✅** | 22 | 19 | -11 | Lower | -8 | Lower | +3 | Higher |
| Cognitive (sum) | **8 ✅** | 12 | 11 | -4 | Lower | -3 | Lower | +1 | Higher |
| Halstead Volume | **1353.9443 ✅** | 1906.3022 | 1464.0554 | -552.3579 | Lower | -110.1111 | Lower | +442.2468 | Higher |
| Halstead Effort | **41402.1910 ✅** | 70186.5804 | 44072.3734 | -28784.3894 | Lower | -2670.1824 | Lower | +26114.2070 | Higher |
| SLOC | 75 | 92 | **70 ✅** | -17 | Lower | +5 | Higher | +22 | Higher |
| PLOC | **36 ✅** | 57 | 46 | -21 | Lower | -10 | Lower | +11 | Higher |
| LLOC | 25 | 26 | **21 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| Comments (CLOC) | **32 ✅** | 23 | 16 | +9 | Higher | +16 | Higher | +7 | Higher |
| Blank Lines | **7 ✅** | 12 | 8 | -5 | Lower | -1 | Lower | +4 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 2 | **0 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |

## p00075

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 26.7443 | 14.4707 | **65.0851 ✅** | +12.2736 | Higher | -38.3408 | Lower | -50.6144 | Lower |
| MI (SEI) | 3.8260 | -9.9201 | **61.8816 ✅** | +13.7462 | Higher | -58.0555 | Lower | -71.8017 | Lower |
| Cyclomatic (sum) | 62 | 70 | **10 ✅** | -8 | Lower | +52 | Higher | +60 | Higher |
| Cognitive (sum) | 52 | 78 | **15 ✅** | -26 | Lower | +37 | Higher | +63 | Higher |
| Halstead Volume | 4518.5587 | 7142.6687 | **1174.1920 ✅** | -2624.1100 | Lower | +3344.3667 | Higher | +5968.4767 | Higher |
| Halstead Effort | 350226.5951 | 668236.3421 | **27271.5572 ✅** | -318009.7470 | Lower | +322955.0379 | Higher | +640964.7849 | Higher |
| SLOC | 205 | 337 | **62 ✅** | -132 | Lower | +143 | Higher | +275 | Higher |
| PLOC | 136 | 205 | **29 ✅** | -69 | Lower | +107 | Higher | +176 | Higher |
| LLOC | 100 | 160 | **16 ✅** | -60 | Lower | +84 | Higher | +144 | Higher |
| Comments (CLOC) | 50 | **103 ✅** | 27 | -53 | Lower | +23 | Higher | +76 | Higher |
| Blank Lines | 21 | 31 | **6 ✅** | -10 | Lower | +15 | Higher | +25 | Higher |
| Functions | 5 | 5 | **1 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 15 | 21 | **0 ✅** | -6 | Lower | +15 | Higher | +21 | Higher |

## p00077

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.9705 | **63.5833 ✅** | 62.0707 | -5.6128 | Lower | -4.1002 | Lower | +1.5126 | Higher |
| MI (SEI) | 52.0617 | 53.2851 | **56.2046 ✅** | -1.2233 | Lower | -4.1429 | Lower | -2.9195 | Lower |
| Cyclomatic (sum) | 17 | 12 | **10 ✅** | +5 | Higher | +7 | Higher | +2 | Higher |
| Cognitive (sum) | 23 | 18 | **16 ✅** | +5 | Higher | +7 | Higher | +2 | Higher |
| Halstead Volume | 1416.4331 | 1364.8963 | **1111.7954 ✅** | +51.5369 | Higher | +304.6378 | Higher | +253.1009 | Higher |
| Halstead Effort | 49346.7026 | 40625.7358 | **33790.6378 ✅** | +8720.9667 | Higher | +15556.0648 | Higher | +6835.0981 | Higher |
| SLOC | 82 | **63 ✅** | 76 | +19 | Higher | +6 | Higher | -13 | Lower |
| PLOC | 41 | **38 ✅** | 41 | +3 | Higher | 0 | Same | -3 | Lower |
| LLOC | 32 | 29 | 29 | +3 | Higher | +3 | Higher | 0 | Same |
| Comments (CLOC) | **35 ✅** | 17 | 30 | +18 | Higher | +5 | Higher | -13 | Lower |
| Blank Lines | 7 | 9 | **5 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | 4 | **0 ✅** | +3 | Higher | +7 | Higher | +4 | Higher |

## p00078

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.1227 | 60.4778 | **60.6978 ✅** | -8.3552 | Lower | -8.5751 | Lower | -0.2199 | Lower |
| MI (SEI) | 44.0307 | 50.4100 | **51.9570 ✅** | -6.3793 | Lower | -7.9263 | Lower | -1.5470 | Lower |
| Cyclomatic (sum) | 13 | 12 | **9 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Cognitive (sum) | 21 | 23 | **13 ✅** | -2 | Lower | +8 | Higher | +10 | Higher |
| Halstead Volume | 1631.4400 | **1382.4211 ✅** | 1452.8239 | +249.0189 | Higher | +178.6161 | Higher | -70.4028 | Lower |
| Halstead Effort | 71077.8742 | **52488.8018 ✅** | 57452.5815 | +18589.0725 | Higher | +13625.2927 | Higher | -4963.7797 | Lower |
| SLOC | 119 | **76 ✅** | 77 | +43 | Higher | +42 | Higher | -1 | Lower |
| PLOC | 51 | 41 | 41 | +10 | Higher | +10 | Higher | 0 | Same |
| LLOC | 39 | **31 ✅** | 36 | +8 | Higher | +3 | Higher | -5 | Lower |
| Comments (CLOC) | **54 ✅** | 23 | 26 | +31 | Higher | +28 | Higher | -3 | Lower |
| Blank Lines | 14 | 12 | **10 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00079

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 27.4470 | **29.8778 ✅** | 21.0867 | -2.4308 | Lower | +6.3603 | Higher | +8.7911 | Higher |
| MI (SEI) | 4.9758 | **12.7392 ✅** | -4.7703 | -7.7633 | Lower | +9.7462 | Higher | +17.5095 | Higher |
| Cyclomatic (sum) | 56 | **33 ✅** | 53 | +23 | Higher | +3 | Higher | -20 | Lower |
| Cognitive (sum) | 54 | **28 ✅** | 79 | +26 | Higher | -25 | Lower | -51 | Lower |
| Halstead Volume | 5389.1095 | **4867.2588 ✅** | 6563.8665 | +521.8507 | Higher | -1174.7570 | Lower | -1696.6077 | Lower |
| Halstead Effort | 357258.5278 | **299720.6716 ✅** | 406959.7237 | +57537.8563 | Higher | -49701.1958 | Lower | -107239.0521 | Lower |
| SLOC | **202 ✅** | 249 | 293 | -47 | Lower | -91 | Lower | -44 | Lower |
| PLOC | **130 ✅** | 133 | 203 | -3 | Lower | -73 | Lower | -70 | Lower |
| LLOC | 84 | **79 ✅** | 135 | +5 | Higher | -51 | Lower | -56 | Lower |
| Comments (CLOC) | 52 | **103 ✅** | 74 | -51 | Lower | -22 | Lower | +29 | Higher |
| Blank Lines | 20 | **16 ✅** | 18 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 6 | 6 | 10 | 0 | Same | -4 | Lower | -4 | Lower |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 16 | **13 ✅** | 16 | +3 | Higher | 0 | Same | -3 | Lower |

## p00080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.5181 ✅** | 66.9000 | 63.3944 | +5.6180 | Higher | +9.1236 | Higher | +3.5056 | Higher |
| MI (SEI) | **75.9615 ✅** | 67.0273 | 59.2074 | +8.9342 | Higher | +16.7541 | Higher | +7.8199 | Higher |
| Cyclomatic (sum) | **5 ✅** | 6 | 11 | -1 | Lower | -6 | Lower | -5 | Lower |
| Cognitive (sum) | 5 | 5 | 16 | 0 | Same | -11 | Lower | -11 | Lower |
| Halstead Volume | **606.6998 ✅** | 648.0387 | 934.8721 | -41.3389 | Lower | -328.1723 | Lower | -286.8334 | Lower |
| Halstead Effort | 13411.2588 | **12707.1930 ✅** | 20389.8832 | +704.0658 | Higher | -6978.6244 | Lower | -7682.6902 | Lower |
| SLOC | **52 ✅** | 71 | 73 | -19 | Lower | -21 | Lower | -2 | Lower |
| PLOC | **17 ✅** | 22 | 33 | -5 | Lower | -16 | Lower | -11 | Lower |
| LLOC | **12 ✅** | 16 | 15 | -4 | Lower | -3 | Lower | +1 | Higher |
| Comments (CLOC) | 31 | **39 ✅** | 31 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | **5 ✅** | 10 | 9 | -5 | Lower | -4 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00081

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.4761 | **56.5770 ✅** | 30.2465 | -6.1009 | Lower | +20.2297 | Higher | +26.3305 | Higher |
| MI (SEI) | 33.8221 | **45.9989 ✅** | 8.1127 | -12.1768 | Lower | +25.7094 | Higher | +37.8862 | Higher |
| Cyclomatic (sum) | 21 | **14 ✅** | 32 | +7 | Higher | -11 | Lower | -18 | Lower |
| Cognitive (sum) | 33 | **15 ✅** | 42 | +18 | Higher | -9 | Lower | -27 | Lower |
| Halstead Volume | 2464.7783 | **2036.1212 ✅** | 5315.0850 | +428.6571 | Higher | -2850.3067 | Lower | -3278.9638 | Lower |
| Halstead Effort | 105788.2835 | **84817.1730 ✅** | 397813.6660 | +20971.1105 | Higher | -292025.3826 | Lower | -312996.4930 | Lower |
| SLOC | 103 | **83 ✅** | 240 | +20 | Higher | -137 | Lower | -157 | Lower |
| PLOC | 69 | **50 ✅** | 151 | +19 | Higher | -82 | Lower | -101 | Lower |
| LLOC | 47 | **34 ✅** | 112 | +13 | Higher | -65 | Lower | -78 | Lower |
| Comments (CLOC) | 25 | 27 | **69 ✅** | -2 | Lower | -44 | Lower | -42 | Lower |
| Blank Lines | 9 | **6 ✅** | 24 | +3 | Higher | -15 | Lower | -18 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | **1 ✅** | 7 | +1 | Higher | -5 | Lower | -6 | Lower |

## p00082

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.0379 | 51.1119 | **57.2173 ✅** | +4.9260 | Higher | -1.1794 | Lower | -6.1054 | Lower |
| MI (SEI) | 35.0561 | 38.5121 | **42.2725 ✅** | -3.4560 | Lower | -7.2164 | Lower | -3.7604 | Lower |
| Cyclomatic (sum) | 18 | 20 | **17 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 22 | 21 | **19 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Halstead Volume | 2041.7160 | 2279.7476 | **1838.7744 ✅** | -238.0316 | Lower | +202.9417 | Higher | +440.9733 | Higher |
| Halstead Effort | 68242.1395 | 77511.4198 | **62150.5743 ✅** | -9269.2803 | Lower | +6091.5652 | Higher | +15360.8454 | Higher |
| SLOC | 81 | 103 | **79 ✅** | -22 | Lower | +2 | Higher | +24 | Higher |
| PLOC | 62 | 62 | **56 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| LLOC | 46 | **39 ✅** | 44 | +7 | Higher | +2 | Higher | -5 | Lower |
| Comments (CLOC) | 12 | **33 ✅** | 18 | -21 | Lower | -6 | Lower | +15 | Higher |
| Blank Lines | **7 ✅** | 11 | 8 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00083

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.9817 | 39.8035 | **41.5487 ✅** | -1.8218 | Lower | -3.5671 | Lower | -1.7453 | Lower |
| MI (SEI) | 12.2061 | **22.4594 ✅** | 13.9286 | -10.2533 | Lower | -1.7225 | Lower | +8.5308 | Higher |
| Cyclomatic (sum) | 39 | **30 ✅** | 35 | +9 | Higher | +4 | Higher | -5 | Lower |
| Cognitive (sum) | 83 | **32 ✅** | 76 | +51 | Higher | +7 | Higher | -44 | Lower |
| Halstead Volume | 3440.0372 | **2807.5204 ✅** | 3179.8102 | +632.5168 | Higher | +260.2270 | Higher | -372.2898 | Lower |
| Halstead Effort | 261595.7150 | **104884.7233 ✅** | 237425.8287 | +156710.9918 | Higher | +24169.8864 | Higher | -132541.1054 | Lower |
| SLOC | 155 | 168 | **135 ✅** | -13 | Lower | +20 | Higher | +33 | Higher |
| PLOC | 119 | **107 ✅** | 111 | +12 | Higher | +8 | Higher | -4 | Lower |
| LLOC | 120 | **67 ✅** | 114 | +53 | Higher | +6 | Higher | -47 | Lower |
| Comments (CLOC) | 25 | **51 ✅** | 17 | -26 | Lower | +8 | Higher | +34 | Higher |
| Blank Lines | 11 | 13 | **9 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00084

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.9227 ✅** | 55.5566 | 58.9571 | +6.3661 | Higher | +2.9655 | Higher | -3.4006 | Lower |
| MI (SEI) | **48.8228 ✅** | 42.8906 | 43.2019 | +5.9322 | Higher | +5.6209 | Higher | -0.3113 | Lower |
| Cyclomatic (sum) | **15 ✅** | 20 | 17 | -5 | Lower | -2 | Lower | +3 | Higher |
| Cognitive (sum) | **16 ✅** | 22 | 21 | -6 | Lower | -5 | Lower | +1 | Higher |
| Halstead Volume | **1423.0529 ✅** | 1900.0927 | 1613.1777 | -477.0398 | Lower | -190.1247 | Lower | +286.9151 | Higher |
| Halstead Effort | **44460.2390 ✅** | 65924.9565 | 51944.3204 | -21464.7174 | Lower | -7484.0814 | Lower | +13980.6360 | Higher |
| SLOC | **66 ✅** | 83 | 74 | -17 | Lower | -8 | Lower | +9 | Higher |
| PLOC | **45 ✅** | 60 | 50 | -15 | Lower | -5 | Lower | +10 | Higher |
| LLOC | 35 | 46 | **33 ✅** | -11 | Lower | +2 | Higher | +13 | Higher |
| Comments (CLOC) | 15 | **23 ✅** | 15 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 6 | **5 ✅** | 10 | +1 | Higher | -4 | Lower | -5 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00085

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.3719 | 57.8117 | **65.5990 ✅** | +6.5603 | Higher | -1.2271 | Lower | -7.7874 | Lower |
| MI (SEI) | 51.1620 | 42.8310 | **51.3549 ✅** | +8.3310 | Higher | -0.1929 | Lower | -8.5239 | Lower |
| Cyclomatic (sum) | 14 | 20 | **9 ✅** | -6 | Lower | +5 | Higher | +11 | Higher |
| Cognitive (sum) | **7 ✅** | 15 | 9 | -8 | Lower | -2 | Lower | +6 | Higher |
| Halstead Volume | 1128.4003 | 1494.5033 | **1111.7954 ✅** | -366.1029 | Lower | +16.6050 | Higher | +382.7079 | Higher |
| Halstead Effort | 36944.6626 | 48571.3564 | **34589.1892 ✅** | -11626.6938 | Lower | +2355.4734 | Higher | +13982.1672 | Higher |
| SLOC | 62 | 78 | 62 | -16 | Lower | 0 | Same | +16 | Higher |
| PLOC | 41 | 55 | **40 ✅** | -14 | Lower | +1 | Higher | +15 | Higher |
| LLOC | **32 ✅** | 38 | 35 | -6 | Lower | -3 | Lower | +3 | Higher |
| Comments (CLOC) | 13 | **17 ✅** | 12 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 9 | 9 | 10 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00086

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.2166 | **55.8227 ✅** | 53.9612 | -0.6062 | Lower | +1.2553 | Higher | +1.8615 | Higher |
| MI (SEI) | 43.6683 | **48.5108 ✅** | 38.3495 | -4.8425 | Lower | +5.3189 | Higher | +10.1614 | Higher |
| Cyclomatic (sum) | 20 | **19 ✅** | 21 | +1 | Higher | -1 | Lower | -2 | Lower |
| Cognitive (sum) | 22 | **20 ✅** | 31 | +2 | Higher | -9 | Lower | -11 | Lower |
| Halstead Volume | 1632.0617 | **1416.6153 ✅** | 1676.5533 | +215.4463 | Higher | -44.4916 | Lower | -259.9379 | Lower |
| Halstead Effort | 64349.8604 | **42804.7555 ✅** | 82654.0762 | +21545.1050 | Higher | -18304.2157 | Lower | -39849.3207 | Lower |
| SLOC | **89 ✅** | 91 | 94 | -2 | Lower | -5 | Lower | -3 | Lower |
| PLOC | 52 | **50 ✅** | 62 | +2 | Higher | -10 | Lower | -12 | Lower |
| LLOC | 37 | **30 ✅** | 45 | +7 | Higher | -8 | Lower | -15 | Lower |
| Comments (CLOC) | 27 | **37 ✅** | 22 | -10 | Lower | +5 | Higher | +15 | Higher |
| Blank Lines | 11 | **9 ✅** | 10 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 4 | 1 | **0 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |

## p00088

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.1937 | 29.5559 | **59.7697 ✅** | +9.6379 | Higher | -20.5760 | Lower | -30.2139 | Lower |
| MI (SEI) | 17.7684 | 11.5021 | **48.1800 ✅** | +6.2663 | Higher | -30.4116 | Lower | -36.6778 | Lower |
| Cyclomatic (sum) | 49 | 61 | **10 ✅** | -12 | Lower | +39 | Higher | +51 | Higher |
| Cognitive (sum) | 17 | 22 | **10 ✅** | -5 | Lower | +7 | Higher | +12 | Higher |
| Halstead Volume | 3422.0558 | 4196.5500 | **2139.3217 ✅** | -774.4942 | Lower | +1282.7342 | Higher | +2057.2283 | Higher |
| Halstead Effort | 99873.3336 | 143141.1477 | **50333.4854 ✅** | -43267.8140 | Lower | +49539.8483 | Higher | +92807.6623 | Higher |
| SLOC | 125 | 179 | **71 ✅** | -54 | Lower | +54 | Higher | +108 | Higher |
| PLOC | 92 | 117 | **43 ✅** | -25 | Lower | +49 | Higher | +74 | Higher |
| LLOC | 35 | 44 | **29 ✅** | -9 | Lower | +6 | Higher | +15 | Higher |
| Comments (CLOC) | 25 | **57 ✅** | 20 | -32 | Lower | +5 | Higher | +37 | Higher |
| Blank Lines | 9 | 10 | **8 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00089

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.8218 | 37.8131 | **52.3692 ✅** | -5.9913 | Lower | -20.5474 | Lower | -14.5561 | Lower |
| MI (SEI) | 11.6721 | 15.3932 | **38.6533 ✅** | -3.7212 | Lower | -26.9813 | Lower | -23.2601 | Lower |
| Cyclomatic (sum) | 39 | 26 | **14 ✅** | +13 | Higher | +25 | Higher | +12 | Higher |
| Cognitive (sum) | 63 | 52 | **17 ✅** | +11 | Higher | +46 | Higher | +35 | Higher |
| Halstead Volume | 4566.8182 | 3578.3351 | **2481.2155 ✅** | +988.4830 | Higher | +2085.6026 | Higher | +1097.1196 | Higher |
| Halstead Effort | 356412.1151 | 212655.3440 | **122406.6333 ✅** | +143756.7711 | Higher | +234005.4818 | Higher | +90248.7107 | Higher |
| SLOC | 207 | 186 | **101 ✅** | +21 | Higher | +106 | Higher | +85 | Higher |
| PLOC | 123 | 120 | **59 ✅** | +3 | Higher | +64 | Higher | +61 | Higher |
| LLOC | 95 | 72 | **37 ✅** | +23 | Higher | +58 | Higher | +35 | Higher |
| Comments (CLOC) | **62 ✅** | 43 | 30 | +19 | Higher | +32 | Higher | +13 | Higher |
| Blank Lines | 25 | 23 | **13 ✅** | +2 | Higher | +12 | Higher | +10 | Higher |
| Functions | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **4 ✅** | 8 | 6 | -4 | Lower | -2 | Lower | +2 | Higher |

## p00090

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **36.2301 ✅** | 15.9092 | 35.0664 | +20.3209 | Higher | +1.1637 | Higher | -19.1572 | Lower |
| MI (SEI) | **10.5988 ✅** | -10.6675 | 7.6575 | +21.2663 | Higher | +2.9413 | Higher | -18.3250 | Lower |
| Cyclomatic (sum) | 40 | 73 | 40 | -33 | Lower | 0 | Same | +33 | Higher |
| Cognitive (sum) | 106 | 111 | **77 ✅** | -5 | Lower | +29 | Higher | +34 | Higher |
| Halstead Volume | **4428.8891 ✅** | 6606.6336 | 4570.2646 | -2177.7444 | Lower | -141.3754 | Lower | +2036.3690 | Higher |
| Halstead Effort | **333374.5641 ✅** | 540441.2356 | 340988.2149 | -207066.6715 | Lower | -7613.6508 | Lower | +199453.0207 | Higher |
| SLOC | **157 ✅** | 303 | 167 | -146 | Lower | -10 | Lower | +136 | Higher |
| PLOC | **118 ✅** | 212 | 125 | -94 | Lower | -7 | Lower | +87 | Higher |
| LLOC | 96 | 143 | **91 ✅** | -47 | Lower | +5 | Higher | +52 | Higher |
| Comments (CLOC) | 27 | **74 ✅** | 26 | -47 | Lower | +1 | Higher | +48 | Higher |
| Blank Lines | **14 ✅** | 18 | 16 | -4 | Lower | -2 | Lower | +2 | Higher |
| Functions | 1 | 8 | 1 | -7 | Lower | 0 | Same | +7 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 13 | 0 | -13 | Lower | 0 | Same | +13 | Higher |

## p00092

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.1662 | 38.0114 | **51.9570 ✅** | +9.1548 | Higher | -4.7907 | Lower | -13.9455 | Lower |
| MI (SEI) | 29.5991 | 18.0757 | **34.7864 ✅** | +11.5234 | Higher | -5.1873 | Lower | -16.7107 | Lower |
| Cyclomatic (sum) | 26 | 34 | **20 ✅** | -8 | Lower | +6 | Higher | +14 | Higher |
| Cognitive (sum) | 38 | 53 | **25 ✅** | -15 | Lower | +13 | Higher | +28 | Higher |
| Halstead Volume | 2648.8879 | 3718.4303 | **2262.7448 ✅** | -1069.5424 | Lower | +386.1431 | Higher | +1455.6855 | Higher |
| Halstead Effort | 128200.7671 | 192748.7976 | **98380.2086 ✅** | -64548.0305 | Lower | +29820.5585 | Higher | +94368.5890 | Higher |
| SLOC | 115 | 162 | **98 ✅** | -47 | Lower | +17 | Higher | +64 | Higher |
| PLOC | 84 | 107 | **68 ✅** | -23 | Lower | +16 | Higher | +39 | Higher |
| LLOC | 58 | 65 | **49 ✅** | -7 | Lower | +9 | Higher | +16 | Higher |
| Comments (CLOC) | 28 | **42 ✅** | 22 | -14 | Lower | +6 | Higher | +20 | Higher |
| Blank Lines | **7 ✅** | 14 | 9 | -7 | Lower | -2 | Lower | +5 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00093

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.0039 | **65.8660 ✅** | 65.8040 | -2.8620 | Lower | -2.8001 | Lower | +0.0619 | Higher |
| MI (SEI) | 56.6776 | **60.5129 ✅** | 55.2484 | -3.8353 | Lower | +1.4292 | Higher | +5.2645 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 15 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 13 | **11 ✅** | 18 | +2 | Higher | -5 | Lower | -7 | Lower |
| Halstead Volume | 962.2906 | **884.8984 ✅** | 1008.9778 | +77.3923 | Higher | -46.6872 | Lower | -124.0795 | Lower |
| Halstead Effort | **23493.1645 ✅** | 23927.6515 | 26809.9823 | -434.4870 | Lower | -3316.8178 | Lower | -2882.3309 | Lower |
| SLOC | 70 | 62 | **58 ✅** | +8 | Higher | +12 | Higher | +4 | Higher |
| PLOC | 36 | **29 ✅** | 37 | +7 | Higher | -1 | Lower | -8 | Lower |
| LLOC | 20 | **18 ✅** | 25 | +2 | Higher | -5 | Lower | -7 | Lower |
| Comments (CLOC) | **25 ✅** | 22 | 14 | +3 | Higher | +11 | Higher | +8 | Higher |
| Blank Lines | 9 | 11 | **7 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00096

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **68.7086 ✅** | 35.4409 | 66.8697 | +33.2677 | Higher | +1.8389 | Higher | -31.4288 | Lower |
| MI (SEI) | **65.4727 ✅** | 19.0764 | 63.6398 | +46.3963 | Higher | +1.8329 | Higher | -44.5634 | Lower |
| Cyclomatic (sum) | 10 | 37 | 10 | -27 | Lower | 0 | Same | +27 | Higher |
| Cognitive (sum) | 16 | 35 | 16 | -19 | Lower | 0 | Same | +19 | Higher |
| Halstead Volume | **760.1050 ✅** | 3651.7396 | 876.3718 | -2891.6347 | Lower | -116.2668 | Lower | +2775.3678 | Higher |
| Halstead Effort | **16215.5732 ✅** | 203960.3997 | 18476.8394 | -187744.8264 | Lower | -2261.2662 | Lower | +185483.5603 | Higher |
| SLOC | **57 ✅** | 183 | 61 | -126 | Lower | -4 | Lower | +122 | Higher |
| PLOC | **29 ✅** | 102 | 31 | -73 | Lower | -2 | Lower | +71 | Higher |
| LLOC | **18 ✅** | 55 | 20 | -37 | Lower | -2 | Lower | +35 | Higher |
| Comments (CLOC) | 22 | **65 ✅** | 25 | -43 | Lower | -3 | Lower | +40 | Higher |
| Blank Lines | 6 | 18 | **5 ✅** | -12 | Lower | +1 | Higher | +13 | Higher |
| Functions | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 1 | 8 | 1 | -7 | Lower | 0 | Same | +7 | Higher |

## p00097

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.7417 | 56.0068 | **58.8309 ✅** | -8.2651 | Lower | -11.0893 | Lower | -2.8242 | Lower |
| MI (SEI) | 33.5192 | 43.4350 | **50.0180 ✅** | -9.9158 | Lower | -16.4988 | Lower | -6.5830 | Lower |
| Cyclomatic (sum) | 24 | 18 | **13 ✅** | +6 | Higher | +11 | Higher | +5 | Higher |
| Cognitive (sum) | 30 | **15 ✅** | 24 | +15 | Higher | +6 | Higher | -9 | Lower |
| Halstead Volume | 2269.0302 | 1704.3301 | **1674.2972 ✅** | +564.7001 | Higher | +594.7330 | Higher | +30.0329 | Higher |
| Halstead Effort | 95724.7116 | 59337.5976 | **55129.2971 ✅** | +36387.1140 | Higher | +40595.4145 | Higher | +4208.3005 | Higher |
| SLOC | 120 | 86 | **78 ✅** | +34 | Higher | +42 | Higher | +8 | Higher |
| PLOC | 74 | 53 | **46 ✅** | +21 | Higher | +28 | Higher | +7 | Higher |
| LLOC | 53 | **34 ✅** | 37 | +19 | Higher | +16 | Higher | -3 | Lower |
| Comments (CLOC) | **37 ✅** | 24 | 27 | +13 | Higher | +10 | Higher | -3 | Lower |
| Blank Lines | 13 | 10 | **6 ✅** | +3 | Higher | +7 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00098

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.0471 | **60.9049 ✅** | 45.9412 | -1.8578 | Lower | +13.1059 | Higher | +14.9637 | Higher |
| MI (SEI) | 51.6159 | **55.0483 ✅** | 29.4375 | -3.4323 | Lower | +22.1784 | Higher | +25.6107 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 21 | +3 | Higher | -8 | Lower | -11 | Lower |
| Cognitive (sum) | 22 | **20 ✅** | 44 | +2 | Higher | -22 | Lower | -24 | Lower |
| Halstead Volume | 1543.6230 | **1283.0542 ✅** | 3069.9412 | +260.5689 | Higher | -1526.3182 | Lower | -1786.8870 | Lower |
| Halstead Effort | 56028.6927 | **28043.8985 ✅** | 134787.7968 | +27984.7941 | Higher | -78759.1041 | Lower | -106743.8982 | Lower |
| SLOC | 79 | **78 ✅** | 127 | +1 | Higher | -48 | Lower | -49 | Lower |
| PLOC | 41 | **36 ✅** | 77 | +5 | Higher | -36 | Lower | -41 | Lower |
| LLOC | 30 | 30 | 62 | 0 | Same | -32 | Lower | -32 | Lower |
| Comments (CLOC) | 30 | 32 | **36 ✅** | -2 | Lower | -6 | Lower | -4 | Lower |
| Blank Lines | **9 ✅** | 10 | 14 | -1 | Lower | -5 | Lower | -4 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 3 | +1 | Higher | -2 | Lower | -3 | Lower |

## p00100

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.1726 | **61.3935 ✅** | 52.7360 | -1.2209 | Lower | +7.4366 | Higher | +8.6575 | Higher |
| MI (SEI) | 44.7647 | **49.0644 ✅** | 39.8812 | -4.2997 | Lower | +4.8835 | Higher | +9.1832 | Higher |
| Cyclomatic (sum) | 15 | **12 ✅** | 19 | +3 | Higher | -4 | Lower | -7 | Lower |
| Cognitive (sum) | 14 | **10 ✅** | 20 | +4 | Higher | -6 | Lower | -10 | Lower |
| Halstead Volume | 1734.7710 | **1639.2957 ✅** | 2396.8490 | +95.4753 | Higher | -662.0780 | Lower | -757.5533 | Lower |
| Halstead Effort | 51501.0144 | **37636.8908 ✅** | 109492.4202 | +13864.1236 | Higher | -57991.4058 | Lower | -71855.5294 | Lower |
| SLOC | 69 | **68 ✅** | 93 | +1 | Higher | -24 | Lower | -25 | Lower |
| PLOC | 46 | **44 ✅** | 54 | +2 | Higher | -8 | Lower | -10 | Lower |
| LLOC | 35 | **32 ✅** | 41 | +3 | Higher | -6 | Lower | -9 | Lower |
| Comments (CLOC) | 14 | 17 | **28 ✅** | -3 | Lower | -14 | Lower | -11 | Lower |
| Blank Lines | 10 | **9 ✅** | 11 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 6 | +1 | Higher | -5 | Lower | -6 | Lower |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p00101

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 27.3454 | 40.9805 | **44.0800 ✅** | -13.6351 | Lower | -16.7346 | Lower | -3.0996 | Lower |
| MI (SEI) | 2.4778 | 21.8490 | **26.7453 ✅** | -19.3712 | Lower | -24.2676 | Lower | -4.8963 | Lower |
| Cyclomatic (sum) | 54 | 28 | 28 | +26 | Higher | +26 | Higher | 0 | Same |
| Cognitive (sum) | 41 | **33 ✅** | 38 | +8 | Higher | +3 | Higher | -5 | Lower |
| Halstead Volume | 4734.5880 | 3020.4092 | **2995.8542 ✅** | +1714.1789 | Higher | +1738.7339 | Higher | +24.5550 | Higher |
| Halstead Effort | 290157.4069 | **145286.8012 ✅** | 158120.1669 | +144870.6057 | Higher | +132037.2400 | Higher | -12833.3657 | Lower |
| SLOC | 218 | 157 | **130 ✅** | +61 | Higher | +88 | Higher | +27 | Higher |
| PLOC | 153 | 96 | **82 ✅** | +57 | Higher | +71 | Higher | +14 | Higher |
| LLOC | 69 | 74 | **68 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Comments (CLOC) | **48 ✅** | 41 | 35 | +7 | Higher | +13 | Higher | +6 | Higher |
| Blank Lines | 18 | 23 | **13 ✅** | -5 | Lower | +5 | Higher | +10 | Higher |
| Functions | 5 | 3 | 3 | +2 | Higher | +2 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 17 | 7 | 7 | +10 | Higher | +10 | Higher | 0 | Same |

## p00103

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.5326 | 44.5060 | **62.1828 ✅** | +10.0266 | Higher | -7.6502 | Lower | -17.6768 | Lower |
| MI (SEI) | 45.1528 | 32.2967 | **49.2999 ✅** | +12.8561 | Higher | -4.1471 | Lower | -17.0032 | Lower |
| Cyclomatic (sum) | 18 | 30 | **16 ✅** | -12 | Lower | +2 | Higher | +14 | Higher |
| Cognitive (sum) | 39 | 39 | **37 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1506.4117 | 2246.2738 | **1295.0401 ✅** | -739.8621 | Lower | +211.3715 | Higher | +951.2336 | Higher |
| Halstead Effort | 62219.3667 | 93862.1547 | **51846.2625 ✅** | -31642.7880 | Lower | +10373.1042 | Higher | +42015.8923 | Higher |
| SLOC | 98 | 135 | **66 ✅** | -37 | Lower | +32 | Higher | +69 | Higher |
| PLOC | 55 | 79 | **46 ✅** | -24 | Lower | +9 | Higher | +33 | Higher |
| LLOC | 43 | 53 | **35 ✅** | -10 | Lower | +8 | Higher | +18 | Higher |
| Comments (CLOC) | 36 | **51 ✅** | 15 | -15 | Lower | +21 | Higher | +36 | Higher |
| Blank Lines | 9 | 13 | **7 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 6 | **2 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p00104

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 38.2389 | **43.2739 ✅** | 36.5831 | -5.0350 | Lower | +1.6558 | Higher | +6.6908 | Higher |
| MI (SEI) | 11.7943 | **24.3494 ✅** | 8.6256 | -12.5551 | Lower | +3.1687 | Higher | +15.7238 | Higher |
| Cyclomatic (sum) | 36 | **27 ✅** | 40 | +9 | Higher | -4 | Lower | -13 | Lower |
| Cognitive (sum) | 25 | **14 ✅** | 28 | +11 | Higher | -3 | Lower | -14 | Lower |
| Halstead Volume | 3321.3955 | **3105.2155 ✅** | 4221.4145 | +216.1801 | Higher | -900.0190 | Lower | -1116.1990 | Lower |
| Halstead Effort | 184435.1408 | **125455.7964 ✅** | 305292.6989 | +58979.3444 | Higher | -120857.5581 | Lower | -179836.9025 | Lower |
| SLOC | 161 | **137 ✅** | 156 | +24 | Higher | +5 | Higher | -19 | Lower |
| PLOC | 121 | **88 ✅** | 118 | +33 | Higher | +3 | Higher | -30 | Lower |
| LLOC | 72 | **59 ✅** | 80 | +13 | Higher | -8 | Lower | -21 | Lower |
| Comments (CLOC) | 25 | **34 ✅** | 22 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | 16 | 16 | 17 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **3 ✅** | 11 | +4 | Higher | -4 | Lower | -8 | Lower |

## p00105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.1167 ✅** | 35.0336 | 41.8313 | +22.0831 | Higher | +15.2854 | Higher | -6.7976 | Lower |
| MI (SEI) | **50.4805 ✅** | 17.4321 | 24.2929 | +33.0485 | Higher | +26.1876 | Higher | -6.8609 | Lower |
| Cyclomatic (sum) | **15 ✅** | 30 | 24 | -15 | Lower | -9 | Lower | +6 | Higher |
| Cognitive (sum) | **15 ✅** | 55 | 48 | -40 | Lower | -33 | Lower | +7 | Higher |
| Halstead Volume | **1364.4917 ✅** | 3956.6731 | 3528.2008 | -2592.1814 | Lower | -2163.7091 | Lower | +428.4723 | Higher |
| Halstead Effort | **33657.4626 ✅** | 219293.5784 | 180768.4068 | -185636.1157 | Lower | -147110.9442 | Lower | +38525.1715 | Higher |
| SLOC | **90 ✅** | 202 | 150 | -112 | Lower | -60 | Lower | +52 | Higher |
| PLOC | **46 ✅** | 109 | 96 | -63 | Lower | -50 | Lower | +13 | Higher |
| LLOC | **26 ✅** | 78 | 78 | -52 | Lower | -52 | Lower | 0 | Same |
| Comments (CLOC) | 38 | **70 ✅** | 44 | -32 | Lower | -6 | Lower | +26 | Higher |
| Blank Lines | **11 ✅** | 26 | 12 | -15 | Lower | -1 | Lower | +14 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 3 | -2 | Lower | -3 | Lower | -1 | Lower |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.5528 | 51.2533 | **59.4994 ✅** | +4.2994 | Higher | -3.9466 | Lower | -8.2461 | Lower |
| MI (SEI) | 41.0585 | 44.0967 | **45.0991 ✅** | -3.0381 | Lower | -4.0406 | Lower | -1.0024 | Lower |
| Cyclomatic (sum) | 13 | 16 | **12 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 14 | 42 | **11 ✅** | -28 | Lower | +3 | Higher | +31 | Higher |
| Halstead Volume | 2320.1476 | 2157.5076 | **2155.8930 ✅** | +162.6400 | Higher | +164.2546 | Higher | +1.6146 | Higher |
| Halstead Effort | 93965.9782 | 86055.1336 | **83815.8407 ✅** | +7910.8447 | Higher | +10150.1375 | Higher | +2239.2928 | Higher |
| SLOC | 86 | 110 | **70 ✅** | -24 | Lower | +16 | Higher | +40 | Higher |
| PLOC | 55 | 55 | **47 ✅** | 0 | Same | +8 | Higher | +8 | Higher |
| LLOC | 33 | 31 | **27 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Comments (CLOC) | 22 | **54 ✅** | 16 | -32 | Lower | +6 | Higher | +38 | Higher |
| Blank Lines | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |

## p00107

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.4796 | 49.0303 | **57.0828 ✅** | +5.4493 | Higher | -2.6033 | Lower | -8.0525 | Lower |
| MI (SEI) | 39.3643 | 37.1488 | **40.6817 ✅** | +2.2155 | Higher | -1.3174 | Lower | -3.5329 | Lower |
| Cyclomatic (sum) | **21 ✅** | 26 | 24 | -5 | Lower | -3 | Lower | +2 | Higher |
| Cognitive (sum) | 19 | **18 ✅** | 19 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 1863.6634 | 1801.6285 | **1499.6431 ✅** | +62.0350 | Higher | +364.0203 | Higher | +301.9854 | Higher |
| Halstead Effort | 68644.9369 | **65967.3201 ✅** | 69894.0818 | +2677.6168 | Higher | -1249.1449 | Lower | -3926.7617 | Lower |
| SLOC | 88 | 116 | **77 ✅** | -28 | Lower | +11 | Higher | +39 | Higher |
| PLOC | 58 | 66 | **54 ✅** | -8 | Lower | +4 | Higher | +12 | Higher |
| LLOC | **27 ✅** | 28 | 31 | -1 | Lower | -4 | Lower | -3 | Lower |
| Comments (CLOC) | 21 | **40 ✅** | 15 | -19 | Lower | +6 | Higher | +25 | Higher |
| Blank Lines | 9 | 11 | 9 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p00108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.7493 ✅** | 45.3228 | 45.5038 | +11.4265 | Higher | +11.2455 | Higher | -0.1810 | Lower |
| MI (SEI) | **49.1897 ✅** | 30.3976 | 30.9738 | +18.7920 | Higher | +18.2159 | Higher | -0.5761 | Lower |
| Cyclomatic (sum) | **14 ✅** | 23 | 23 | -9 | Lower | -9 | Lower | 0 | Same |
| Cognitive (sum) | **7 ✅** | 31 | 44 | -24 | Lower | -37 | Lower | -13 | Lower |
| Halstead Volume | **1429.3077 ✅** | 2443.2903 | 2359.7087 | -1013.9826 | Lower | -930.4010 | Lower | +83.5816 | Higher |
| Halstead Effort | **38214.1294 ✅** | 112147.0248 | 113885.4409 | -73932.8954 | Lower | -75671.3115 | Lower | -1738.4161 | Lower |
| SLOC | **92 ✅** | 138 | 138 | -46 | Lower | -46 | Lower | 0 | Same |
| PLOC | **44 ✅** | 84 | 82 | -40 | Lower | -38 | Lower | +2 | Higher |
| LLOC | **29 ✅** | 68 | 65 | -39 | Lower | -36 | Lower | +3 | Higher |
| Comments (CLOC) | 37 | 44 | **45 ✅** | -7 | Lower | -8 | Lower | -1 | Lower |
| Blank Lines | **11 ✅** | 12 | 14 | -1 | Lower | -3 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | **0 ✅** | 1 | +3 | Higher | +2 | Higher | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00109

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.1291 | 34.8193 | **50.0125 ✅** | +12.3098 | Higher | -2.8834 | Lower | -15.1932 | Lower |
| MI (SEI) | **25.6410 ✅** | 14.3892 | 23.1558 | +11.2518 | Higher | +2.4852 | Higher | -8.7666 | Lower |
| Cyclomatic (sum) | 23 | 34 | **22 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| Cognitive (sum) | 10 | 32 | 10 | -22 | Lower | 0 | Same | +22 | Higher |
| Halstead Volume | 2811.5674 | 3855.7589 | **2740.4354 ✅** | -1044.1915 | Lower | +71.1320 | Higher | +1115.3235 | Higher |
| Halstead Effort | 171538.3029 | 218952.0206 | **132739.8381 ✅** | -47413.7177 | Lower | +38798.4648 | Higher | +86212.1825 | Higher |
| SLOC | 118 | 195 | **101 ✅** | -77 | Lower | +17 | Higher | +94 | Higher |
| PLOC | 86 | 129 | **80 ✅** | -43 | Lower | +6 | Higher | +49 | Higher |
| LLOC | **52 ✅** | 78 | 53 | -26 | Lower | -1 | Lower | +25 | Higher |
| Comments (CLOC) | 22 | **54 ✅** | 11 | -32 | Lower | +11 | Higher | +43 | Higher |
| Blank Lines | 12 | 13 | **10 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 8 | 12 | **6 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |

## p00110

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.0904 | 52.1277 | **57.3559 ✅** | +3.9627 | Higher | -1.2656 | Lower | -5.2283 | Lower |
| MI (SEI) | 43.1076 | 39.0338 | **47.2925 ✅** | +4.0738 | Higher | -4.1849 | Lower | -8.2587 | Lower |
| Cyclomatic (sum) | 19 | 18 | **14 ✅** | +1 | Higher | +5 | Higher | +4 | Higher |
| Cognitive (sum) | **16 ✅** | 18 | 18 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | 1861.2356 | 2111.8951 | **1820.3251 ✅** | -250.6595 | Lower | +40.9105 | Higher | +291.5700 | Higher |
| Halstead Effort | 59027.7577 | 62611.4796 | **49729.7338 ✅** | -3583.7219 | Lower | +9298.0239 | Higher | +12881.7458 | Higher |
| SLOC | 82 | 102 | 82 | -20 | Lower | 0 | Same | +20 | Higher |
| PLOC | 53 | 61 | **51 ✅** | -8 | Lower | +2 | Higher | +10 | Higher |
| LLOC | **31 ✅** | 40 | 32 | -9 | Lower | -1 | Lower | +8 | Higher |
| Comments (CLOC) | 22 | **31 ✅** | 27 | -9 | Lower | -5 | Lower | +4 | Higher |
| Blank Lines | 7 | 10 | **6 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 7 | **1 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |

## p00111

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.2516 | 46.9237 | **51.0152 ✅** | -0.6721 | Lower | -4.7636 | Lower | -4.0915 | Lower |
| MI (SEI) | 25.0309 | **36.2469 ✅** | 30.8522 | -11.2160 | Lower | -5.8213 | Lower | +5.3947 | Higher |
| Cyclomatic (sum) | 17 | 18 | **10 ✅** | -1 | Lower | +7 | Higher | +8 | Higher |
| Cognitive (sum) | 19 | 45 | 19 | -26 | Lower | 0 | Same | +26 | Higher |
| Halstead Volume | 4456.6249 | **2634.7903 ✅** | 3963.2588 | +1821.8346 | Higher | +493.3661 | Higher | -1328.4685 | Lower |
| Halstead Effort | 102951.2423 | 103719.5338 | **82147.5462 ✅** | -768.2915 | Lower | +20803.6961 | Higher | +21571.9876 | Higher |
| SLOC | 117 | 131 | **100 ✅** | -14 | Lower | +17 | Higher | +31 | Higher |
| PLOC | 86 | **67 ✅** | 74 | +19 | Higher | +12 | Higher | -7 | Lower |
| LLOC | 61 | **36 ✅** | 55 | +25 | Higher | +6 | Higher | -19 | Lower |
| Comments (CLOC) | 24 | **56 ✅** | 20 | -32 | Lower | +4 | Higher | +36 | Higher |
| Blank Lines | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 6 | 0 | 0 | +6 | Higher | +6 | Higher | 0 | Same |

## p00112

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.0918 ✅** | 47.7229 | 51.2473 | +10.3689 | Higher | +6.8445 | Higher | -3.5244 | Lower |
| MI (SEI) | **52.0864 ✅** | 32.9482 | 36.6633 | +19.1382 | Higher | +15.4230 | Higher | -3.7152 | Lower |
| Cyclomatic (sum) | **16 ✅** | 20 | 17 | -4 | Lower | -1 | Lower | +3 | Higher |
| Cognitive (sum) | **12 ✅** | 35 | 21 | -23 | Lower | -9 | Lower | +14 | Higher |
| Halstead Volume | **1202.7928 ✅** | 2068.1389 | 2319.3233 | -865.3461 | Lower | -1116.5306 | Lower | -251.1845 | Lower |
| Halstead Effort | **31312.7054 ✅** | 105061.4538 | 119861.4409 | -73748.7484 | Lower | -88548.7356 | Lower | -14799.9872 | Lower |
| SLOC | **87 ✅** | 131 | 106 | -44 | Lower | -19 | Lower | +25 | Higher |
| PLOC | **42 ✅** | 74 | 69 | -32 | Lower | -27 | Lower | +5 | Higher |
| LLOC | **24 ✅** | 58 | 59 | -34 | Lower | -35 | Lower | -1 | Lower |
| Comments (CLOC) | 37 | **40 ✅** | 30 | -3 | Lower | +7 | Higher | +10 | Higher |
| Blank Lines | 11 | 17 | **9 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 2 | **1 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00113

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **52.0544 ✅** | 47.7201 | 45.4756 | +4.3344 | Higher | +6.5788 | Higher | +2.2444 | Higher |
| MI (SEI) | **41.4542 ✅** | 37.7034 | 25.9315 | +3.7509 | Higher | +15.5228 | Higher | +11.7719 | Higher |
| Cyclomatic (sum) | **12 ✅** | 19 | 23 | -7 | Lower | -11 | Lower | -4 | Lower |
| Cognitive (sum) | **13 ✅** | 21 | 42 | -8 | Lower | -29 | Lower | -21 | Lower |
| Halstead Volume | **2029.9053 ✅** | 2269.0302 | 3395.4312 | -239.1249 | Lower | -1365.5259 | Lower | -1126.4010 | Lower |
| Halstead Effort | **82646.1454 ✅** | 92179.3519 | 199305.4063 | -9533.2065 | Lower | -116659.2609 | Lower | -107126.0544 | Lower |
| SLOC | **113 ✅** | 129 | 123 | -16 | Lower | -10 | Lower | +6 | Higher |
| PLOC | **58 ✅** | 60 | 82 | -2 | Lower | -24 | Lower | -22 | Lower |
| LLOC | 51 | **43 ✅** | 64 | +8 | Higher | -13 | Lower | -21 | Lower |
| Comments (CLOC) | 43 | **56 ✅** | 28 | -13 | Lower | +15 | Higher | +28 | Higher |
| Blank Lines | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p00114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.2973 | 46.5235 | **49.8982 ✅** | -1.2262 | Lower | -4.6009 | Lower | -3.3747 | Lower |
| MI (SEI) | 28.4508 | **32.6287 ✅** | 29.2510 | -4.1778 | Lower | -0.8002 | Lower | +3.3777 | Higher |
| Cyclomatic (sum) | 33 | **24 ✅** | 25 | +9 | Higher | +8 | Higher | -1 | Lower |
| Cognitive (sum) | 12 | 15 | 12 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 2710.0655 | 2525.5220 | **2453.2475 ✅** | +184.5435 | Higher | +256.8180 | Higher | +72.2745 | Higher |
| Halstead Effort | 151763.6689 | 120295.8568 | **118491.8555 ✅** | +31467.8121 | Higher | +33271.8134 | Higher | +1804.0014 | Higher |
| SLOC | 116 | 125 | **101 ✅** | -9 | Lower | +15 | Higher | +24 | Higher |
| PLOC | 76 | **68 ✅** | 74 | +8 | Higher | +2 | Higher | -6 | Lower |
| LLOC | **45 ✅** | 54 | 47 | -9 | Lower | -2 | Lower | +7 | Higher |
| Comments (CLOC) | 30 | **41 ✅** | 18 | -11 | Lower | +12 | Higher | +23 | Higher |
| Blank Lines | 10 | 16 | **9 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 6 | **0 ✅** | 1 | +6 | Higher | +5 | Higher | -1 | Lower |
| Exits | **2 ✅** | 8 | 3 | -6 | Lower | -1 | Lower | +5 | Higher |

## p00115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.5103 | **47.7529 ✅** | 41.3031 | -5.2426 | Lower | +1.2072 | Higher | +6.4498 | Higher |
| MI (SEI) | 17.5390 | **28.0899 ✅** | 13.8613 | -10.5509 | Lower | +3.6778 | Higher | +14.2286 | Higher |
| Cyclomatic (sum) | 25 | **20 ✅** | 23 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 32 | **27 ✅** | 29 | +5 | Higher | +3 | Higher | -2 | Lower |
| Halstead Volume | 4209.6415 | **3350.3306 ✅** | 4347.1975 | +859.3108 | Higher | -137.5560 | Lower | -996.8669 | Lower |
| Halstead Effort | 237968.5551 | **153511.0504 ✅** | 275641.7927 | +84457.5047 | Higher | -37673.2376 | Lower | -122130.7423 | Lower |
| SLOC | 134 | **112 ✅** | 147 | +22 | Higher | -13 | Lower | -35 | Lower |
| PLOC | 101 | **76 ✅** | 114 | +25 | Higher | -13 | Lower | -38 | Lower |
| LLOC | 72 | **64 ✅** | 73 | +8 | Higher | -1 | Lower | -9 | Lower |
| Comments (CLOC) | 22 | **24 ✅** | 21 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 13 | **12 ✅** | 15 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 3 | **1 ✅** | 5 | +2 | Higher | -2 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **1 ✅** | 5 | +5 | Higher | +1 | Higher | -4 | Lower |

## p00116

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 37.0624 | 31.2937 | **51.4072 ✅** | +5.7687 | Higher | -14.3449 | Lower | -20.1135 | Lower |
| MI (SEI) | 13.4603 | 11.7865 | **32.1943 ✅** | +1.6738 | Higher | -18.7340 | Lower | -20.4078 | Lower |
| Cyclomatic (sum) | 42 | 46 | **23 ✅** | -4 | Lower | +19 | Higher | +23 | Higher |
| Cognitive (sum) | 60 | 70 | **35 ✅** | -10 | Lower | +25 | Higher | +35 | Higher |
| Halstead Volume | 3523.8134 | 4259.8731 | **2133.9623 ✅** | -736.0597 | Lower | +1389.8512 | Higher | +2125.9109 | Higher |
| Halstead Effort | 197227.8381 | 285174.8407 | **96028.3013 ✅** | -87947.0026 | Lower | +101199.5369 | Higher | +189146.5395 | Higher |
| SLOC | 156 | 198 | **99 ✅** | -42 | Lower | +57 | Higher | +99 | Higher |
| PLOC | 118 | 126 | **69 ✅** | -8 | Lower | +49 | Higher | +57 | Higher |
| LLOC | 72 | 94 | **44 ✅** | -22 | Lower | +28 | Higher | +50 | Higher |
| Comments (CLOC) | 30 | **60 ✅** | 19 | -30 | Lower | +11 | Higher | +41 | Higher |
| Blank Lines | 12 | 13 | **11 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | **0 ✅** | 5 | 1 | -5 | Lower | -1 | Lower | +4 | Higher |

## p00117

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 23.0289 | **41.2394 ✅** | 37.4111 | -18.2105 | Lower | -14.3821 | Lower | +3.8284 | Higher |
| MI (SEI) | -6.3467 | **21.0194 ✅** | 11.3325 | -27.3661 | Lower | -17.6791 | Lower | +9.6869 | Higher |
| Cyclomatic (sum) | 64 | **24 ✅** | 30 | +40 | Higher | +34 | Higher | -6 | Lower |
| Cognitive (sum) | 41 | **30 ✅** | 34 | +11 | Higher | +7 | Higher | -4 | Lower |
| Halstead Volume | 6686.5315 | **4300.8483 ✅** | 4447.6853 | +2385.6832 | Higher | +2238.8462 | Higher | -146.8370 | Lower |
| Halstead Effort | 461249.8305 | **192616.5638 ✅** | 260091.4776 | +268633.2667 | Higher | +201158.3529 | Higher | -67474.9138 | Lower |
| SLOC | 221 | **146 ✅** | 168 | +75 | Higher | +53 | Higher | -22 | Lower |
| PLOC | 162 | **96 ✅** | 120 | +66 | Higher | +42 | Higher | -24 | Lower |
| LLOC | 90 | **75 ✅** | 92 | +15 | Higher | -2 | Lower | -17 | Lower |
| Comments (CLOC) | **37 ✅** | 36 | 29 | +1 | Higher | +8 | Higher | +7 | Higher |
| Blank Lines | 22 | **14 ✅** | 19 | +8 | Higher | +3 | Higher | -5 | Lower |
| Functions | 8 | **3 ✅** | 8 | +5 | Higher | 0 | Same | -5 | Lower |
| Closures | 4 | 1 | **0 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Exits | 21 | **4 ✅** | 8 | +17 | Higher | +13 | Higher | -4 | Lower |

## p00118

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.7935 | 44.2123 | **50.8213 ✅** | -0.4188 | Lower | -7.0278 | Lower | -6.6090 | Lower |
| MI (SEI) | 27.9423 | **31.4547 ✅** | 29.5079 | -3.5124 | Lower | -1.5657 | Lower | +1.9467 | Higher |
| Cyclomatic (sum) | 30 | 29 | **27 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 42 | **33 ✅** | 43 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 2829.2310 | 2794.2540 | **2431.5132 ✅** | +34.9770 | Higher | +397.7179 | Higher | +362.7408 | Higher |
| Halstead Effort | 125294.5179 | 174250.9800 | **118800.5623 ✅** | -48956.4621 | Lower | +6493.9556 | Higher | +55450.4177 | Higher |
| SLOC | 131 | 130 | **93 ✅** | +1 | Higher | +38 | Higher | +37 | Higher |
| PLOC | 78 | 73 | 73 | +5 | Higher | +5 | Higher | 0 | Same |
| LLOC | 47 | **43 ✅** | 44 | +4 | Higher | +3 | Higher | -1 | Lower |
| Comments (CLOC) | 39 | **48 ✅** | 15 | -9 | Lower | +24 | Higher | +33 | Higher |
| Blank Lines | 16 | 12 | **6 ✅** | +4 | Higher | +10 | Higher | +6 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 4 | **0 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p00120

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.0543 | **43.5915 ✅** | 37.4636 | -1.5372 | Lower | +4.5907 | Higher | +6.1279 | Higher |
| MI (SEI) | **24.1073 ✅** | 20.4652 | 14.8587 | +3.6421 | Higher | +9.2486 | Higher | +5.6065 | Higher |
| Cyclomatic (sum) | **24 ✅** | 29 | 31 | -5 | Lower | -7 | Lower | -2 | Lower |
| Cognitive (sum) | **36 ✅** | 74 | 49 | -38 | Lower | -13 | Lower | +25 | Higher |
| Halstead Volume | 3243.4228 | **3148.5488 ✅** | 3986.4606 | +94.8740 | Higher | -743.0378 | Lower | -837.9118 | Lower |
| Halstead Effort | 145143.1690 | **140897.5566 ✅** | 217234.7964 | +4245.6123 | Higher | -72091.6274 | Lower | -76337.2398 | Lower |
| SLOC | 152 | **130 ✅** | 171 | +22 | Higher | -19 | Lower | -41 | Lower |
| PLOC | **94 ✅** | 97 | 119 | -3 | Lower | -25 | Lower | -22 | Lower |
| LLOC | **51 ✅** | 63 | 71 | -12 | Lower | -20 | Lower | -8 | Lower |
| Comments (CLOC) | **43 ✅** | 23 | 38 | +20 | Higher | +5 | Higher | -15 | Lower |
| Blank Lines | 16 | **13 ✅** | 15 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 3 | **1 ✅** | 4 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | **0 ✅** | 8 | +4 | Higher | -4 | Lower | -8 | Lower |

## p00121

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **45.2635 ✅** | 35.8093 | 42.0093 | +9.4542 | Higher | +3.2542 | Higher | -6.2000 | Lower |
| MI (SEI) | **27.7840 ✅** | 13.6801 | 19.6096 | +14.1039 | Higher | +8.1744 | Higher | -5.9295 | Lower |
| Cyclomatic (sum) | **20 ✅** | 35 | 22 | -15 | Lower | -2 | Lower | +13 | Higher |
| Cognitive (sum) | 28 | 44 | 28 | -16 | Lower | 0 | Same | +16 | Higher |
| Halstead Volume | **2953.2506 ✅** | 4348.9409 | 3803.2366 | -1395.6902 | Lower | -849.9860 | Lower | +545.7042 | Higher |
| Halstead Effort | **145099.3338 ✅** | 224606.8850 | 188199.8439 | -79507.5513 | Lower | -43100.5101 | Lower | +36407.0412 | Higher |
| SLOC | **136 ✅** | 174 | 149 | -38 | Lower | -13 | Lower | +25 | Higher |
| PLOC | **82 ✅** | 120 | 100 | -38 | Lower | -18 | Lower | +20 | Higher |
| LLOC | **56 ✅** | 76 | 83 | -20 | Lower | -27 | Lower | -7 | Lower |
| Comments (CLOC) | 37 | **41 ✅** | 31 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 19 | **14 ✅** | 20 | +5 | Higher | -1 | Lower | -6 | Lower |
| Functions | **2 ✅** | 5 | 3 | -3 | Lower | -1 | Lower | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 9 | 2 | -8 | Lower | -1 | Lower | +7 | Higher |

## p00122

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **45.5903 ✅** | 38.1670 | 42.2556 | +7.4233 | Higher | +3.3347 | Higher | -4.0886 | Lower |
| MI (SEI) | **24.5647 ✅** | 18.6919 | 14.9638 | +5.8727 | Higher | +9.6009 | Higher | +3.7281 | Higher |
| Cyclomatic (sum) | **23 ✅** | 41 | 36 | -18 | Lower | -13 | Lower | +5 | Higher |
| Cognitive (sum) | **37 ✅** | 47 | 41 | -10 | Lower | -4 | Lower | +6 | Higher |
| Halstead Volume | **3321.3955 ✅** | 3365.3639 | 3832.8158 | -43.9683 | Lower | -511.4202 | Lower | -467.4519 | Lower |
| Halstead Effort | **148929.0038 ✅** | 178700.8217 | 250836.4994 | -29771.8179 | Lower | -101907.4956 | Lower | -72135.6777 | Lower |
| SLOC | 123 | 150 | **120 ✅** | -27 | Lower | +3 | Higher | +30 | Higher |
| PLOC | **83 ✅** | 98 | 89 | -15 | Lower | -6 | Lower | +9 | Higher |
| LLOC | **49 ✅** | 64 | 63 | -15 | Lower | -14 | Lower | +1 | Higher |
| Comments (CLOC) | 25 | **38 ✅** | 15 | -13 | Lower | +10 | Higher | +23 | Higher |
| Blank Lines | 17 | **15 ✅** | 17 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 3 | **1 ✅** | 2 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 3 | 4 | **2 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.9334 | **62.4211 ✅** | 59.8329 | -8.4877 | Lower | -5.8995 | Lower | +2.5882 | Higher |
| MI (SEI) | 39.5636 | 47.8045 | **49.3794 ✅** | -8.2409 | Lower | -9.8157 | Lower | -1.5749 | Lower |
| Cyclomatic (sum) | 26 | 24 | **22 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | **14 ✅** | 21 | 19 | -7 | Lower | -5 | Lower | +2 | Higher |
| Halstead Volume | 1848.7020 | **1231.4161 ✅** | 1422 | +617.2859 | Higher | +426.7020 | Higher | -190.5839 | Lower |
| Halstead Effort | 33611.3153 | **23329.9812 ✅** | 30209.2326 | +10281.3341 | Higher | +3402.0828 | Higher | -6879.2513 | Lower |
| SLOC | 85 | **59 ✅** | 68 | +26 | Higher | +17 | Higher | -9 | Lower |
| PLOC | 57 | 43 | 43 | +14 | Higher | +14 | Higher | 0 | Same |
| LLOC | 18 | **14 ✅** | 26 | +4 | Higher | -8 | Lower | -12 | Lower |
| Comments (CLOC) | **21 ✅** | 11 | 19 | +10 | Higher | +2 | Higher | -8 | Lower |
| Blank Lines | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00124

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.1042 ✅** | 43.4294 | 56.3883 | +19.6748 | Higher | +6.7159 | Higher | -12.9589 | Lower |
| MI (SEI) | **51.1814 ✅** | 21.0623 | 39.3742 | +30.1191 | Higher | +11.8072 | Higher | -18.3119 | Lower |
| Cyclomatic (sum) | **13 ✅** | 27 | 21 | -14 | Lower | -8 | Lower | +6 | Higher |
| Cognitive (sum) | **8 ✅** | 43 | 31 | -35 | Lower | -23 | Lower | +12 | Higher |
| Halstead Volume | **1505.0541 ✅** | 2755.3360 | 1957.1433 | -1250.2819 | Lower | -452.0892 | Lower | +798.1927 | Higher |
| Halstead Effort | **30853.6098 ✅** | 116527.7531 | 81595.6076 | -85674.1433 | Lower | -50741.9978 | Lower | +34932.1455 | Higher |
| SLOC | **62 ✅** | 141 | 77 | -79 | Lower | -15 | Lower | +64 | Higher |
| PLOC | **38 ✅** | 98 | 51 | -60 | Lower | -13 | Lower | +47 | Higher |
| LLOC | **25 ✅** | 54 | 30 | -29 | Lower | -5 | Lower | +24 | Higher |
| Comments (CLOC) | 15 | **27 ✅** | 15 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | **10 ✅** | 17 | 12 | -7 | Lower | -2 | Lower | +5 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **1 ✅** | 2 | 5 | -1 | Lower | -4 | Lower | -3 | Lower |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.4926 | 50.2651 | **58.9498 ✅** | +3.2274 | Higher | -5.4572 | Lower | -8.6847 | Lower |
| MI (SEI) | 42.3494 | 35.8216 | **51.7921 ✅** | +6.5278 | Higher | -9.4427 | Lower | -15.9705 | Lower |
| Cyclomatic (sum) | 12 | 12 | 14 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **5 ✅** | 9 | 10 | -4 | Lower | -5 | Lower | -1 | Lower |
| Halstead Volume | **2054.6266 ✅** | 2314.0189 | 2193.3554 | -259.3923 | Lower | -138.7288 | Lower | +120.6635 | Higher |
| Halstead Effort | **70252.4260 ✅** | 77824.1096 | 74492.8496 | -7571.6836 | Lower | -4240.4237 | Lower | +3331.2599 | Higher |
| SLOC | 103 | 121 | **70 ✅** | -18 | Lower | +33 | Higher | +51 | Higher |
| PLOC | 51 | 70 | **38 ✅** | -19 | Lower | +13 | Higher | +32 | Higher |
| LLOC | 35 | 68 | **28 ✅** | -33 | Lower | +7 | Higher | +40 | Higher |
| Comments (CLOC) | 36 | **37 ✅** | 27 | -1 | Lower | +9 | Higher | +10 | Higher |
| Blank Lines | 16 | 15 | **10 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |

## p00126

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **41.2890 ✅** | 38.6405 | 41.2709 | +2.6485 | Higher | +0.0181 | Higher | -2.6304 | Lower |
| MI (SEI) | **24.6987 ✅** | 21.4174 | 21.3725 | +3.2813 | Higher | +3.3262 | Higher | +0.0449 | Higher |
| Cyclomatic (sum) | 31 | **30 ✅** | 31 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 62 | **58 ✅** | 81 | +4 | Higher | -19 | Lower | -23 | Lower |
| Halstead Volume | **3193.3565 ✅** | 3384.0647 | 3420.1399 | -190.7083 | Lower | -226.7835 | Lower | -36.0752 | Lower |
| Halstead Effort | 212890.4301 | **205944.5108 ✅** | 216246.1197 | +6945.9193 | Higher | -3355.6896 | Lower | -10301.6089 | Lower |
| SLOC | 145 | 170 | **142 ✅** | -25 | Lower | +3 | Higher | +28 | Higher |
| PLOC | **88 ✅** | 103 | 96 | -15 | Lower | -8 | Lower | +7 | Higher |
| LLOC | **57 ✅** | 72 | 62 | -15 | Lower | -5 | Lower | +10 | Higher |
| Comments (CLOC) | 44 | **54 ✅** | 34 | -10 | Lower | +10 | Higher | +20 | Higher |
| Blank Lines | 13 | 14 | **12 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 5 | 5 | **1 ✅** | 0 | Same | +4 | Higher | +4 | Higher |

## p00127

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.8266 ✅** | 53.8041 | 47.7647 | +2.0225 | Higher | +8.0619 | Higher | +6.0395 | Higher |
| MI (SEI) | **37.7362 ✅** | 37.7262 | 33.7093 | +0.0100 | Higher | +4.0269 | Higher | +4.0169 | Higher |
| Cyclomatic (sum) | 21 | **19 ✅** | 27 | +2 | Higher | -6 | Lower | -8 | Lower |
| Cognitive (sum) | 13 | **12 ✅** | 20 | +1 | Higher | -7 | Lower | -8 | Lower |
| Halstead Volume | **1792.3010 ✅** | 2088.5360 | 2084.6379 | -296.2351 | Lower | -292.3369 | Lower | +3.8982 | Higher |
| Halstead Effort | **58345.1161 ✅** | 72402.5829 | 78480.4854 | -14057.4669 | Lower | -20135.3694 | Lower | -6077.9025 | Lower |
| SLOC | **82 ✅** | 91 | 118 | -9 | Lower | -36 | Lower | -27 | Lower |
| PLOC | **56 ✅** | 59 | 68 | -3 | Lower | -12 | Lower | -9 | Lower |
| LLOC | **30 ✅** | 35 | 40 | -5 | Lower | -10 | Lower | -5 | Lower |
| Comments (CLOC) | 15 | 21 | **36 ✅** | -6 | Lower | -21 | Lower | -15 | Lower |
| Blank Lines | 13 | 13 | 16 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 5 | **0 ✅** | 5 | +5 | Higher | 0 | Same | -5 | Lower |

## p00128

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.1852 | 58.0186 | **59.3951 ✅** | +0.1666 | Higher | -1.2098 | Lower | -1.3765 | Lower |
| MI (SEI) | **48.5589 ✅** | 44.5010 | 46.1024 | +4.0579 | Higher | +2.4565 | Higher | -1.6014 | Lower |
| Cyclomatic (sum) | 13 | 15 | 13 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | **21 ✅** | 27 | 22 | -6 | Lower | -1 | Lower | +5 | Higher |
| Halstead Volume | **1349.0104 ✅** | 1655.7899 | 1563.7823 | -306.7795 | Lower | -214.7719 | Lower | +92.0076 | Higher |
| Halstead Effort | **37772.2909 ✅** | 51502.7670 | 46331.5954 | -13730.4761 | Lower | -8559.3044 | Lower | +5171.1717 | Higher |
| SLOC | 87 | 80 | **77 ✅** | +7 | Higher | +10 | Higher | +3 | Higher |
| PLOC | **45 ✅** | 51 | 48 | -6 | Lower | -3 | Lower | +3 | Higher |
| LLOC | **36 ✅** | 44 | 40 | -8 | Lower | -4 | Lower | +4 | Higher |
| Comments (CLOC) | **29 ✅** | 20 | 19 | +9 | Higher | +10 | Higher | +1 | Higher |
| Blank Lines | 14 | 12 | **11 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00129

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 18.0831 | **31.9276 ✅** | 30.4315 | -13.8445 | Lower | -12.3484 | Lower | +1.4961 | Higher |
| MI (SEI) | -13.4692 | **8.3343 ✅** | -0.9253 | -21.8035 | Lower | -12.5439 | Lower | +9.2596 | Higher |
| Cyclomatic (sum) | 77 | 45 | **36 ✅** | +32 | Higher | +41 | Higher | +9 | Higher |
| Cognitive (sum) | 106 | **48 ✅** | 55 | +58 | Higher | +51 | Higher | -7 | Lower |
| Halstead Volume | 8372 | **4953.1292 ✅** | 7240.2773 | +3418.8708 | Higher | +1131.7227 | Higher | -2287.1482 | Lower |
| Halstead Effort | 587450.0211 | **294164.4114 ✅** | 467192.5193 | +293285.6096 | Higher | +120257.5018 | Higher | -173028.1078 | Lower |
| SLOC | 232 | **184 ✅** | 203 | +48 | Higher | +29 | Higher | -19 | Lower |
| PLOC | 188 | **113 ✅** | 166 | +75 | Higher | +22 | Higher | -53 | Lower |
| LLOC | 107 | **96 ✅** | 118 | +11 | Higher | -11 | Lower | -22 | Lower |
| Comments (CLOC) | 35 | **41 ✅** | 28 | -6 | Lower | +7 | Higher | +13 | Higher |
| Blank Lines | 13 | 31 | **12 ✅** | -18 | Lower | +1 | Higher | +19 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 26 | **6 ✅** | 7 | +20 | Higher | +19 | Higher | -1 | Lower |

## p00130

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **43.2248 ✅** | 35.2860 | 41.8970 | +7.9388 | Higher | +1.3278 | Higher | -6.6110 | Lower |
| MI (SEI) | **22.0369 ✅** | 18.2579 | 19.0638 | +3.7790 | Higher | +2.9731 | Higher | -0.8059 | Lower |
| Cyclomatic (sum) | 30 | 35 | 30 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | **41 ✅** | 51 | 44 | -10 | Lower | -3 | Lower | +7 | Higher |
| Halstead Volume | **2808.5536 ✅** | 3426.7205 | 3169.2960 | -618.1670 | Lower | -360.7424 | Lower | +257.4245 | Higher |
| Halstead Effort | **170760.0559 ✅** | 211634.2600 | 177282.4944 | -40874.2040 | Lower | -6522.4385 | Lower | +34351.7655 | Higher |
| SLOC | **136 ✅** | 194 | 142 | -58 | Lower | -6 | Lower | +52 | Higher |
| PLOC | **95 ✅** | 116 | 102 | -21 | Lower | -7 | Lower | +14 | Higher |
| LLOC | **74 ✅** | 91 | 81 | -17 | Lower | -7 | Lower | +10 | Higher |
| Comments (CLOC) | 28 | **67 ✅** | 27 | -39 | Lower | +1 | Higher | +40 | Higher |
| Blank Lines | 13 | **12 ✅** | 15 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00131

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.2523 | 55.1602 | **56.2264 ✅** | +0.0921 | Higher | -0.9742 | Lower | -1.0663 | Lower |
| MI (SEI) | **47.1002 ✅** | 46.8231 | 45.7133 | +0.2771 | Higher | +1.3869 | Higher | +1.1098 | Higher |
| Cyclomatic (sum) | 14 | 14 | 18 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 35 | **26 ✅** | 43 | +9 | Higher | -8 | Lower | -17 | Lower |
| Halstead Volume | **1782.6209 ✅** | 2077.7169 | 1969.0048 | -295.0960 | Lower | -186.3839 | Lower | +108.7121 | Higher |
| Halstead Effort | **53690.8430 ✅** | 81160.8162 | 108671.6912 | -27469.9733 | Lower | -54980.8482 | Lower | -27510.8749 | Lower |
| SLOC | 94 | 90 | **81 ✅** | +4 | Higher | +13 | Higher | +9 | Higher |
| PLOC | **46 ✅** | 48 | 48 | -2 | Lower | -2 | Lower | 0 | Same |
| LLOC | **31 ✅** | 32 | 37 | -1 | Lower | -6 | Lower | -5 | Lower |
| Comments (CLOC) | **38 ✅** | 36 | 26 | +2 | Higher | +12 | Higher | +10 | Higher |
| Blank Lines | 10 | 11 | **7 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00132

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **11.4110 ✅** | 2.6272 | 10.2744 | +8.7838 | Higher | +1.1366 | Higher | -7.6472 | Lower |
| MI (SEI) | **-25.6966 ✅** | -33.3770 | -31.8302 | +7.6804 | Higher | +6.1336 | Higher | -1.5468 | Lower |
| Cyclomatic (sum) | **73 ✅** | 94 | 79 | -21 | Lower | -6 | Lower | +15 | Higher |
| Cognitive (sum) | 126 | 255 | **123 ✅** | -129 | Lower | +3 | Higher | +132 | Higher |
| Halstead Volume | 12493.1233 | 12597.8100 | **11263.3462 ✅** | -104.6867 | Lower | +1229.7771 | Higher | +1334.4639 | Higher |
| Halstead Effort | **1103955.8323 ✅** | 1115062.3591 | 1228291.3651 | -11106.5269 | Lower | -124335.5328 | Lower | -113229.0059 | Lower |
| SLOC | **326 ✅** | 415 | 332 | -89 | Lower | -6 | Lower | +83 | Higher |
| PLOC | **251 ✅** | 331 | 270 | -80 | Lower | -19 | Lower | +61 | Higher |
| LLOC | **165 ✅** | 219 | 167 | -54 | Lower | -2 | Lower | +52 | Higher |
| Comments (CLOC) | 41 | **66 ✅** | 26 | -25 | Lower | +15 | Higher | +40 | Higher |
| Blank Lines | 38 | **29 ✅** | 38 | +9 | Higher | 0 | Same | -9 | Lower |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 7 | 12 | **6 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |

## p00133

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.6445 | **67.5018 ✅** | 56.8392 | -5.8574 | Lower | +4.8053 | Higher | +10.6626 | Higher |
| MI (SEI) | 51.1682 | **60.3052 ✅** | 43.9572 | -9.1370 | Lower | +7.2111 | Higher | +16.3481 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 18 | +3 | Higher | -5 | Lower | -8 | Lower |
| Cognitive (sum) | **15 ✅** | 16 | 17 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 1805.1385 | **1202.5542 ✅** | 1891.9052 | +602.5844 | Higher | -86.7667 | Lower | -689.3510 | Lower |
| Halstead Effort | 57606.4836 | **28560.6616 ✅** | 71176.5420 | +29045.8220 | Higher | -13570.0584 | Lower | -42615.8804 | Lower |
| SLOC | 64 | **53 ✅** | 79 | +11 | Higher | -15 | Lower | -26 | Lower |
| PLOC | 41 | **30 ✅** | 48 | +11 | Higher | -7 | Lower | -18 | Lower |
| LLOC | 30 | **23 ✅** | 33 | +7 | Higher | -3 | Lower | -10 | Lower |
| Comments (CLOC) | 18 | 16 | **21 ✅** | +2 | Higher | -3 | Lower | -5 | Lower |
| Blank Lines | **6 ✅** | 8 | 10 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | **0 ✅** | 1 | 7 | -1 | Lower | -7 | Lower | -6 | Lower |

## p00134

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.2123 ✅** | 68.6422 | 76.4738 | +8.5701 | Higher | +0.7385 | Higher | -7.8316 | Lower |
| MI (SEI) | **78.0058 ✅** | 68.0035 | 71.7646 | +10.0023 | Higher | +6.2412 | Higher | -3.7611 | Lower |
| Cyclomatic (sum) | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | **563.5218 ✅** | 606.6998 | 602.5359 | -43.1780 | Lower | -39.0140 | Lower | +4.1639 | Higher |
| Halstead Effort | 8571.4636 | 8520.1755 | **7963.9521 ✅** | +51.2881 | Higher | +607.5114 | Higher | +556.2233 | Higher |
| SLOC | **41 ✅** | 67 | 42 | -26 | Lower | -1 | Lower | +25 | Higher |
| PLOC | **16 ✅** | 27 | 24 | -11 | Lower | -8 | Lower | +3 | Higher |
| LLOC | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| Comments (CLOC) | 17 | **33 ✅** | 12 | -16 | Lower | +5 | Higher | +21 | Higher |
| Blank Lines | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00135

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **57.7090 ✅** | 40.3653 | 41.7090 | +17.3437 | Higher | +16.0000 | Higher | -1.3437 | Lower |
| MI (SEI) | **45.8506 ✅** | 25.9202 | 26.5240 | +19.9305 | Higher | +19.3267 | Higher | -0.6038 | Lower |
| Cyclomatic (sum) | **22 ✅** | 35 | 33 | -13 | Lower | -11 | Lower | +2 | Higher |
| Cognitive (sum) | **21 ✅** | 36 | 28 | -15 | Lower | -7 | Lower | +8 | Higher |
| Halstead Volume | **1643.9102 ✅** | 3062.1010 | 2941.6904 | -1418.1908 | Lower | -1297.7802 | Lower | +120.4105 | Higher |
| Halstead Effort | **51598.2305 ✅** | 146272.4492 | 150669.7052 | -94674.2187 | Lower | -99071.4748 | Lower | -4397.2561 | Lower |
| SLOC | **74 ✅** | 147 | 141 | -73 | Lower | -67 | Lower | +6 | Higher |
| PLOC | **51 ✅** | 87 | 87 | -36 | Lower | -36 | Lower | 0 | Same |
| LLOC | **25 ✅** | 58 | 53 | -33 | Lower | -28 | Lower | +5 | Higher |
| Comments (CLOC) | 20 | **52 ✅** | 46 | -32 | Lower | -26 | Lower | +6 | Higher |
| Blank Lines | 10 | **9 ✅** | 13 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | **1 ✅** | 3 | 5 | -2 | Lower | -4 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **4 ✅** | 10 | 8 | -6 | Lower | -4 | Lower | +2 | Higher |

## p00136

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.0122 | 54.1702 | **61.0823 ✅** | +4.8420 | Higher | -2.0701 | Lower | -6.9120 | Lower |
| MI (SEI) | 45.0762 | 42.5445 | **47.7114 ✅** | +2.5317 | Higher | -2.6351 | Lower | -5.1669 | Lower |
| Cyclomatic (sum) | **18 ✅** | 20 | 19 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 21 | 21 | **19 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | **1527.1311 ✅** | 1740.3784 | 1620 | -213.2473 | Lower | -92.8689 | Lower | +120.3784 | Higher |
| Halstead Effort | **40781.3423 ✅** | 49184.6079 | 45439.0244 | -8403.2657 | Lower | -4657.6821 | Lower | +3745.5836 | Higher |
| SLOC | 74 | 93 | **63 ✅** | -19 | Lower | +11 | Higher | +30 | Higher |
| PLOC | 51 | 54 | **43 ✅** | -3 | Lower | +8 | Higher | +11 | Higher |
| LLOC | 33 | 37 | **29 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| Comments (CLOC) | 17 | **29 ✅** | 14 | -12 | Lower | +3 | Higher | +15 | Higher |
| Blank Lines | 7 | 12 | **6 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00137

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **67.4357 ✅** | 60.3651 | 59.7198 | +7.0705 | Higher | +7.7159 | Higher | +0.6454 | Higher |
| MI (SEI) | **61.4438 ✅** | 57.0117 | 48.1440 | +4.4322 | Higher | +13.2998 | Higher | +8.8676 | Higher |
| Cyclomatic (sum) | **11 ✅** | 13 | 13 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | 8 | 12 | **6 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Halstead Volume | **1038.2544 ✅** | 1108.2491 | 1530.1837 | -69.9947 | Lower | -491.9292 | Lower | -421.9346 | Lower |
| Halstead Effort | **25550.7924 ✅** | 28814.4767 | 53365.1552 | -3263.6842 | Lower | -27814.3627 | Lower | -24550.6785 | Lower |
| SLOC | **55 ✅** | 81 | 76 | -26 | Lower | -21 | Lower | +5 | Higher |
| PLOC | **34 ✅** | 35 | 49 | -1 | Lower | -15 | Lower | -14 | Lower |
| LLOC | **20 ✅** | 21 | 36 | -1 | Lower | -16 | Lower | -15 | Lower |
| Comments (CLOC) | 18 | **40 ✅** | 21 | -22 | Lower | -3 | Lower | +19 | Higher |
| Blank Lines | **6 ✅** | 9 | 9 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00138

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.9063 | 49.8313 | **59.1363 ✅** | +7.0750 | Higher | -2.2299 | Lower | -9.3049 | Lower |
| MI (SEI) | 42.4533 | 37.9548 | **44.5627 ✅** | +4.4985 | Higher | -2.1094 | Lower | -6.6079 | Lower |
| Cyclomatic (sum) | 14 | 18 | **13 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Cognitive (sum) | 23 | 26 | **22 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Halstead Volume | 1984.7152 | 2260.2410 | **1784.0151 ✅** | -275.5258 | Lower | +200.7001 | Higher | +476.2259 | Higher |
| Halstead Effort | 72671.1119 | 85912.2226 | **64539.3707 ✅** | -13241.1108 | Lower | +8131.7411 | Higher | +21372.8519 | Higher |
| SLOC | 82 | 115 | **75 ✅** | -33 | Lower | +7 | Higher | +40 | Higher |
| PLOC | 51 | 61 | **49 ✅** | -10 | Lower | +2 | Higher | +12 | Higher |
| LLOC | 36 | 37 | **35 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Comments (CLOC) | 20 | **41 ✅** | 17 | -21 | Lower | +3 | Higher | +24 | Higher |
| Blank Lines | 11 | 13 | **9 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00139

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.9814 | 46.1862 | **52.4117 ✅** | -5.2048 | Lower | -11.4303 | Lower | -6.2256 | Lower |
| MI (SEI) | **27.6251 ✅** | 24.8480 | 27.2482 | +2.7770 | Higher | +0.3769 | Higher | -2.4001 | Lower |
| Cyclomatic (sum) | 36 | 36 | **26 ✅** | 0 | Same | +10 | Higher | +10 | Higher |
| Cognitive (sum) | 34 | **32 ✅** | 67 | +2 | Higher | -33 | Lower | -35 | Lower |
| Halstead Volume | 2715.7668 | 2499.1710 | **2073.0948 ✅** | +216.5958 | Higher | +642.6720 | Higher | +426.0762 | Higher |
| Halstead Effort | 131208.8116 | 127402.1841 | **91577.7575 ✅** | +3806.6274 | Higher | +39631.0540 | Higher | +35824.4266 | Higher |
| SLOC | 145 | 108 | **90 ✅** | +37 | Higher | +55 | Higher | +18 | Higher |
| PLOC | 90 | 83 | **73 ✅** | +7 | Higher | +17 | Higher | +10 | Higher |
| LLOC | 53 | 53 | **47 ✅** | 0 | Same | +6 | Higher | +6 | Higher |
| Comments (CLOC) | **54 ✅** | 19 | 10 | +35 | Higher | +44 | Higher | +9 | Higher |
| Blank Lines | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 3 | 3 | **1 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 18 | 15 | **1 ✅** | +3 | Higher | +17 | Higher | +14 | Higher |

## p00140

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.1223 | 39.1383 | **55.9215 ✅** | +14.9839 | Higher | -1.7993 | Lower | -16.7832 | Lower |
| MI (SEI) | **41.5308 ✅** | 20.7945 | 35.9890 | +20.7363 | Higher | +5.5418 | Higher | -15.1945 | Lower |
| Cyclomatic (sum) | 26 | 31 | **19 ✅** | -5 | Lower | +7 | Higher | +12 | Higher |
| Cognitive (sum) | 49 | 57 | **43 ✅** | -8 | Lower | +6 | Higher | +14 | Higher |
| Halstead Volume | **1718.9771 ✅** | 3290.5736 | 1997.5715 | -1571.5965 | Lower | -278.5945 | Lower | +1293.0020 | Higher |
| Halstead Effort | **72086.1346 ✅** | 176539.2710 | 97381.6116 | -104453.1363 | Lower | -25295.4769 | Lower | +79157.6594 | Higher |
| SLOC | 86 | 164 | **81 ✅** | -78 | Lower | +5 | Higher | +83 | Higher |
| PLOC | **55 ✅** | 104 | 62 | -49 | Lower | -7 | Lower | +42 | Higher |
| LLOC | **28 ✅** | 72 | 43 | -44 | Lower | -15 | Lower | +29 | Higher |
| Comments (CLOC) | 24 | **47 ✅** | 13 | -23 | Lower | +11 | Higher | +34 | Higher |
| Blank Lines | 7 | 13 | **6 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | 2 | **0 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Exits | 4 | 2 | **1 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p00141

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.0244 | 35.6089 | **41.2771 ✅** | -4.5845 | Lower | -10.2527 | Lower | -5.6682 | Lower |
| MI (SEI) | 8.4941 | 15.3152 | **17.2465 ✅** | -6.8210 | Lower | -8.7523 | Lower | -1.9313 | Lower |
| Cyclomatic (sum) | 46 | 33 | **29 ✅** | +13 | Higher | +17 | Higher | +4 | Higher |
| Cognitive (sum) | 74 | **51 ✅** | 55 | +23 | Higher | +19 | Higher | -4 | Lower |
| Halstead Volume | 5018.6293 | 4292.6007 | **3900.6296 ✅** | +726.0287 | Higher | +1117.9997 | Higher | +391.9711 | Higher |
| Halstead Effort | 391238.0051 | **225361.5367 ✅** | 262833.6027 | +165876.4684 | Higher | +128404.4024 | Higher | -37472.0660 | Lower |
| SLOC | 191 | 182 | **140 ✅** | +9 | Higher | +51 | Higher | +42 | Higher |
| PLOC | 128 | 113 | **107 ✅** | +15 | Higher | +21 | Higher | +6 | Higher |
| LLOC | 116 | 98 | **89 ✅** | +18 | Higher | +27 | Higher | +9 | Higher |
| Comments (CLOC) | 47 | **50 ✅** | 25 | -3 | Lower | +22 | Higher | +25 | Higher |
| Blank Lines | 16 | 22 | **13 ✅** | -6 | Lower | +3 | Higher | +9 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 3 | 2 | **0 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |

## p00142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.9674 | 55.8046 | **58.6754 ✅** | -1.8372 | Lower | -4.7080 | Lower | -2.8708 | Lower |
| MI (SEI) | 41.0676 | 46.8197 | **49.2913 ✅** | -5.7521 | Lower | -8.2238 | Lower | -2.4716 | Lower |
| Cyclomatic (sum) | 19 | 17 | **15 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 44 | 34 | **31 ✅** | +10 | Higher | +13 | Higher | +3 | Higher |
| Halstead Volume | 1658.8730 | **1451.3403 ✅** | 1517.6367 | +207.5327 | Higher | +141.2363 | Higher | -66.2964 | Lower |
| Halstead Effort | 73339.6476 | **52992.5286 ✅** | 56236.8706 | +20347.1190 | Higher | +17102.7770 | Higher | -3244.3420 | Lower |
| SLOC | 97 | 93 | **79 ✅** | +4 | Higher | +18 | Higher | +14 | Higher |
| PLOC | 57 | 48 | **45 ✅** | +9 | Higher | +12 | Higher | +3 | Higher |
| LLOC | 36 | **32 ✅** | 34 | +4 | Higher | +2 | Higher | -2 | Lower |
| Comments (CLOC) | 28 | **34 ✅** | 26 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 13 | 12 | **9 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00143

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1468 | 46.6648 | **57.1125 ✅** | +8.4821 | Higher | -1.9656 | Lower | -10.4477 | Lower |
| MI (SEI) | **40.4656 ✅** | 29.1944 | 36.6869 | +11.2712 | Higher | +3.7787 | Higher | -7.4925 | Lower |
| Cyclomatic (sum) | **10 ✅** | 19 | 11 | -9 | Lower | -1 | Lower | +8 | Higher |
| Cognitive (sum) | **5 ✅** | 14 | 6 | -9 | Lower | -1 | Lower | +8 | Higher |
| Halstead Volume | **2666.4993 ✅** | 3307.2290 | 2760.0706 | -640.7297 | Lower | -93.5713 | Lower | +547.1584 | Higher |
| Halstead Effort | 106166.1741 | 143533.7381 | **98503.8523 ✅** | -37367.5640 | Lower | +7662.3218 | Higher | +45029.8858 | Higher |
| SLOC | 88 | 122 | **76 ✅** | -34 | Lower | +12 | Higher | +46 | Higher |
| PLOC | **53 ✅** | 76 | 54 | -23 | Lower | -1 | Lower | +22 | Higher |
| LLOC | 36 | 40 | **29 ✅** | -4 | Lower | +7 | Higher | +11 | Higher |
| Comments (CLOC) | 23 | **32 ✅** | 12 | -9 | Lower | +11 | Higher | +20 | Higher |
| Blank Lines | 12 | 15 | **10 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **1 ✅** | 8 | 2 | -7 | Lower | -1 | Lower | +6 | Higher |

## p00144

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **51.6237 ✅** | 46.8286 | 43.3502 | +4.7951 | Higher | +8.2736 | Higher | +3.4784 | Higher |
| MI (SEI) | **36.7523 ✅** | 24.7210 | 18.4978 | +12.0313 | Higher | +18.2545 | Higher | +6.2232 | Higher |
| Cyclomatic (sum) | **20 ✅** | 22 | 33 | -2 | Lower | -13 | Lower | -11 | Lower |
| Cognitive (sum) | **18 ✅** | 37 | 31 | -19 | Lower | -13 | Lower | +6 | Higher |
| Halstead Volume | **2490.8397 ✅** | 3283.8490 | 3045.8923 | -793.0093 | Lower | -555.0527 | Lower | +237.9567 | Higher |
| Halstead Effort | **96567.9380 ✅** | 118025.3969 | 152484.9853 | -21457.4590 | Lower | -55917.0473 | Lower | -34459.5883 | Lower |
| SLOC | **97 ✅** | 116 | 126 | -19 | Lower | -29 | Lower | -10 | Lower |
| PLOC | **63 ✅** | 78 | 91 | -15 | Lower | -28 | Lower | -13 | Lower |
| LLOC | **44 ✅** | 58 | 54 | -14 | Lower | -10 | Lower | +4 | Higher |
| Comments (CLOC) | **26 ✅** | 21 | 19 | +5 | Higher | +7 | Higher | +2 | Higher |
| Blank Lines | **15 ✅** | 20 | 17 | -5 | Lower | -2 | Lower | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 2 | **1 ✅** | 9 | +1 | Higher | -7 | Lower | -8 | Lower |

## p00145

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.9994 | 48.8487 | **55.3997 ✅** | +2.1506 | Higher | -4.4004 | Lower | -6.5510 | Lower |
| MI (SEI) | 39.7165 | 31.6059 | **41.6202 ✅** | +8.1107 | Higher | -1.9037 | Lower | -10.0144 | Lower |
| Cyclomatic (sum) | 18 | 22 | **10 ✅** | -4 | Lower | +8 | Higher | +12 | Higher |
| Cognitive (sum) | 17 | 22 | **14 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Halstead Volume | 2327.3478 | 2554.3490 | **2068.1389 ✅** | -227.0011 | Lower | +259.2090 | Higher | +486.2101 | Higher |
| Halstead Effort | 114455.6409 | 130063.8844 | **93882.6190 ✅** | -15608.2434 | Lower | +20573.0219 | Higher | +36181.2653 | Higher |
| SLOC | 106 | 111 | **94 ✅** | -5 | Lower | +12 | Higher | +17 | Higher |
| PLOC | 56 | 71 | 56 | -15 | Lower | 0 | Same | +15 | Higher |
| LLOC | **39 ✅** | 50 | 44 | -11 | Lower | -5 | Lower | +6 | Higher |
| Comments (CLOC) | **38 ✅** | 27 | 26 | +11 | Higher | +12 | Higher | +1 | Higher |
| Blank Lines | **12 ✅** | 14 | 13 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 8 | 0 | -8 | Lower | 0 | Same | +8 | Higher |

## p00146

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.9693 | 41.5276 | **49.6179 ✅** | -0.5583 | Lower | -8.6486 | Lower | -8.0903 | Lower |
| MI (SEI) | 21.4574 | 16.7029 | **26.9576 ✅** | +4.7546 | Higher | -5.5002 | Lower | -10.2548 | Lower |
| Cyclomatic (sum) | 24 | 27 | **20 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| Cognitive (sum) | 41 | **35 ✅** | 37 | +6 | Higher | +4 | Higher | -2 | Lower |
| Halstead Volume | 4252.0457 | 4444.8305 | **3331.6561 ✅** | -192.7848 | Lower | +920.3896 | Higher | +1113.1744 | Higher |
| Halstead Effort | 248385.3472 | 266689.8320 | **201898.3600 ✅** | -18304.4847 | Lower | +46486.9872 | Higher | +64791.4719 | Higher |
| SLOC | 149 | 136 | **100 ✅** | +13 | Higher | +49 | Higher | +36 | Higher |
| PLOC | 98 | 101 | **75 ✅** | -3 | Lower | +23 | Higher | +26 | Higher |
| LLOC | 76 | 86 | **65 ✅** | -10 | Lower | +11 | Higher | +21 | Higher |
| Comments (CLOC) | **39 ✅** | 23 | 16 | +16 | Higher | +23 | Higher | +7 | Higher |
| Blank Lines | 14 | 12 | **10 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 3 | 4 | **1 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |

## p00148

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.3103 | 44.8333 | **74.4315 ✅** | +8.4771 | Higher | -21.1212 | Lower | -29.5983 | Lower |
| MI (SEI) | 44.4188 | 34.6512 | **75.8418 ✅** | +9.7675 | Higher | -31.4230 | Lower | -41.1905 | Lower |
| Cyclomatic (sum) | 20 | 28 | **5 ✅** | -8 | Lower | +15 | Higher | +23 | Higher |
| Cognitive (sum) | 22 | 50 | **2 ✅** | -28 | Lower | +20 | Higher | +48 | Higher |
| Halstead Volume | 1637.8441 | 1926 | **505.3170 ✅** | -288.1559 | Lower | +1132.5271 | Higher | +1420.6830 | Higher |
| Halstead Effort | 69631.7712 | 101964.7059 | **7898.9022 ✅** | -32332.9346 | Lower | +61732.8690 | Higher | +94065.8037 | Higher |
| SLOC | 100 | 143 | **49 ✅** | -43 | Lower | +51 | Higher | +94 | Higher |
| PLOC | 50 | 73 | **23 ✅** | -23 | Lower | +27 | Higher | +50 | Higher |
| LLOC | 37 | 47 | **14 ✅** | -10 | Lower | +23 | Higher | +33 | Higher |
| Comments (CLOC) | 39 | **63 ✅** | 23 | -24 | Lower | +16 | Higher | +40 | Higher |
| Blank Lines | 12 | 9 | **3 ✅** | +3 | Higher | +9 | Higher | +6 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00149

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.5783 | 59.1158 | **65.7036 ✅** | -4.5375 | Lower | -11.1253 | Lower | -6.5878 | Lower |
| MI (SEI) | 47.3243 | **52.0083 ✅** | 49.9396 | -4.6840 | Lower | -2.6153 | Lower | +2.0687 | Higher |
| Cyclomatic (sum) | 18 | 17 | **15 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 13 | **10 ✅** | 11 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | 1493.2198 | 1382.4835 | **1213.7372 ✅** | +110.7363 | Higher | +279.4827 | Higher | +168.7463 | Higher |
| Halstead Effort | 48990.7863 | **35483.7435 ✅** | 36867.2668 | +13507.0428 | Higher | +12123.5195 | Higher | -1383.5233 | Lower |
| SLOC | 98 | 77 | **55 ✅** | +21 | Higher | +43 | Higher | +22 | Higher |
| PLOC | 47 | 44 | **39 ✅** | +3 | Higher | +8 | Higher | +5 | Higher |
| LLOC | 30 | **24 ✅** | 28 | +6 | Higher | +2 | Higher | -4 | Lower |
| Comments (CLOC) | **42 ✅** | 29 | 9 | +13 | Higher | +33 | Higher | +20 | Higher |
| Blank Lines | 9 | 8 | **7 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.7416 | 50.0913 | **59.1738 ✅** | +0.6503 | Higher | -8.4323 | Lower | -9.0826 | Lower |
| MI (SEI) | 39.5735 | 37.1469 | **47.5674 ✅** | +2.4266 | Higher | -7.9939 | Lower | -10.4205 | Lower |
| Cyclomatic (sum) | 21 | 19 | **14 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| Cognitive (sum) | 31 | **26 ✅** | 27 | +5 | Higher | +4 | Higher | -1 | Lower |
| Halstead Volume | 1661.5187 | 1668.1745 | **1441.2813 ✅** | -6.6559 | Lower | +220.2373 | Higher | +226.8932 | Higher |
| Halstead Effort | 77675.9970 | 85003.3050 | **51801.3457 ✅** | -7327.3080 | Lower | +25874.6513 | Higher | +33201.9592 | Higher |
| SLOC | 115 | 123 | **79 ✅** | -8 | Lower | +36 | Higher | +44 | Higher |
| PLOC | 61 | 69 | **47 ✅** | -8 | Lower | +14 | Higher | +22 | Higher |
| LLOC | 44 | 50 | **38 ✅** | -6 | Lower | +6 | Higher | +12 | Higher |
| Comments (CLOC) | **41 ✅** | 40 | 22 | +1 | Higher | +19 | Higher | +18 | Higher |
| Blank Lines | 16 | 15 | **10 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00151

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.2848 | **38.2474 ✅** | 35.3677 | -5.9626 | Lower | -3.0829 | Lower | +2.8797 | Higher |
| MI (SEI) | 4.9036 | **19.7646 ✅** | 5.3583 | -14.8609 | Lower | -0.4547 | Lower | +14.4063 | Higher |
| Cyclomatic (sum) | 48 | **36 ✅** | 38 | +12 | Higher | +10 | Higher | -2 | Lower |
| Cognitive (sum) | 85 | **70 ✅** | 120 | +15 | Higher | -35 | Lower | -50 | Lower |
| Halstead Volume | 3915.5642 | **2958.7361 ✅** | 3604.2998 | +956.8280 | Higher | +311.2644 | Higher | -645.5637 | Lower |
| Halstead Effort | 330244.4128 | **185781.1064 ✅** | 311528.3984 | +144463.3064 | Higher | +18716.0144 | Higher | -125747.2920 | Lower |
| SLOC | 186 | **167 ✅** | 182 | +19 | Higher | +4 | Higher | -15 | Lower |
| PLOC | 147 | **112 ✅** | 151 | +35 | Higher | -4 | Lower | -39 | Lower |
| LLOC | 131 | **90 ✅** | 149 | +41 | Higher | -18 | Lower | -59 | Lower |
| Comments (CLOC) | 30 | **47 ✅** | 23 | -17 | Lower | +7 | Higher | +24 | Higher |
| Blank Lines | 12 | 12 | 12 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 6 | 1 | **0 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00152

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.6450 | 30.2489 | **41.0021 ✅** | +6.3961 | Higher | -4.3571 | Lower | -10.7532 | Lower |
| MI (SEI) | 9.3343 | 8.1379 | **11.4178 ✅** | +1.1963 | Higher | -2.0836 | Lower | -3.2799 | Lower |
| Cyclomatic (sum) | 37 | 45 | **30 ✅** | -8 | Lower | +7 | Higher | +15 | Higher |
| Cognitive (sum) | 40 | 76 | **35 ✅** | -36 | Lower | +5 | Higher | +41 | Higher |
| Halstead Volume | 3578.5251 | 4531.6371 | **3173.6079 ✅** | -953.1119 | Lower | +404.9172 | Higher | +1358.0292 | Higher |
| Halstead Effort | 219005.7371 | 285493.1345 | **164218.6521 ✅** | -66487.3974 | Lower | +54787.0850 | Higher | +121274.4824 | Higher |
| SLOC | 171 | 210 | **150 ✅** | -39 | Lower | +21 | Higher | +60 | Higher |
| PLOC | 127 | 143 | **119 ✅** | -16 | Lower | +8 | Higher | +24 | Higher |
| LLOC | 102 | **84 ✅** | 92 | +18 | Higher | +10 | Higher | -8 | Lower |
| Comments (CLOC) | 26 | **55 ✅** | 17 | -29 | Lower | +9 | Higher | +38 | Higher |
| Blank Lines | 19 | 20 | **17 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 5 | 18 | **3 ✅** | -13 | Lower | +2 | Higher | +15 | Higher |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.3991 | 49.9240 | **53.8097 ✅** | 2.4751 | Higher | -1.4106 | Lower | -3.8857 | Lower |
| MI (SEI) | **38.8858 ✅** | 36.5573 | 38.4292 | 2.3285 | Higher | 0.4566 | Higher | -1.8719 | Lower |
| Cyclomatic (sum) | 22.3000 | 23.1500 | **21.0714 ✅** | -0.8500 | Lower | 1.2286 | Higher | 2.0786 | Higher |
| Cognitive (sum) | **26.7786 ✅** | 29.3214 | 28.6643 | -2.5429 | Lower | -1.8857 | Lower | 0.6571 | Higher |
| Halstead Volume | **2296.6928 ✅** | 2412.1383 | 2370.8020 | -115.4455 | Lower | -74.1092 | Lower | 41.3363 | Higher |
| Halstead Effort | **111702.9130 ✅** | 116376.1539 | 129379.5307 | -4673.2409 | Lower | -17676.6176 | Lower | -13003.3768 | Lower |
| SLOC | 106.7786 | 121.3929 | **102.0714 ✅** | -14.6143 | Lower | 4.7071 | Higher | 19.3214 | Higher |
| PLOC | **66.3071 ✅** | 72.3357 | 68.4214 | -6.0286 | Lower | -2.1143 | Lower | 3.9143 | Higher |
| LLOC | **43.9143 ✅** | 48.2643 | 48.7571 | -4.3500 | Lower | -4.8429 | Lower | -0.4929 | Lower |
| Comments (CLOC) | 30.3000 | **38.1143 ✅** | 23.9500 | -7.8143 | Lower | 6.3500 | Higher | 14.1643 | Higher |
| Blank Lines | 11.3786 | 12.5857 | **10.6429 ✅** | -1.2071 | Lower | 0.7357 | Higher | 1.9429 | Higher |
| Functions | 1.9357 | **1.9286 ✅** | 1.9929 | 0.0071 | Higher | -0.0571 | Lower | -0.0643 | Lower |
| Closures | 0.5214 | **0.5143 ✅** | 0.6143 | 0.0071 | Higher | -0.0929 | Lower | -0.1000 | Lower |
| Exits | 3.2357 | 3.1214 | **2.2071 ✅** | 0.1143 | Higher | 1.0286 | Higher | 0.9143 | Higher |

