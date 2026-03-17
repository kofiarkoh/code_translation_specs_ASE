# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/codenet_statistics copy/r_code/metrics.json`

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

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **80.0379 ✅** | 77.4460 | 78.1736 | 2.5919 | Higher | 1.8643 | Higher | -0.7276 | Lower |
| MI (SEI) | 68.8393 | **71.0967 ✅** | 64.7513 | -2.2574 | Lower | 4.0880 | Higher | 6.3454 | Higher |
| Cyclomatic (sum) | **6.9583 ✅** | 7.3750 | 8.5833 | -0.4167 | Lower | -1.6250 | Lower | -1.2083 | Lower |
| Cognitive (sum) | 7.9167 | **6.8333 ✅** | 8.4583 | 1.0833 | Higher | -0.5417 | Lower | -1.6250 | Lower |
| Halstead Volume | **695.7246 ✅** | 753.5214 | 946.8748 | -57.7968 | Lower | -251.1502 | Lower | -193.3533 | Lower |
| Halstead Effort | **15039.9753 ✅** | 16985.4937 | 38227.2175 | -1945.5184 | Lower | -23187.2423 | Lower | -21241.7238 | Lower |
| SLOC | **32.6250 ✅** | 37.4583 | 38.2083 | -4.8333 | Lower | -5.5833 | Lower | -0.7500 | Lower |
| PLOC | **22.8333 ✅** | 24.7500 | 28.8750 | -1.9167 | Lower | -6.0417 | Lower | -4.1250 | Lower |
| LLOC | **14.0417 ✅** | 14.2917 | 19.5417 | -0.2500 | Lower | -5.5000 | Lower | -5.2500 | Lower |
| Comments (CLOC) | 5.0417 | **8.8750 ✅** | 5.3333 | -3.8333 | Lower | -0.2917 | Lower | 3.5417 | Higher |
| Blank Lines | **5.5833 ✅** | 5.8750 | 6.1250 | -0.2917 | Lower | -0.5417 | Lower | -0.2500 | Lower |
| Functions | **1.1250 ✅** | 1.3750 | 1.8750 | -0.2500 | Lower | -0.7500 | Lower | -0.5000 | Lower |
| Closures | 0.3750 | 0.5833 | 0.3750 | -0.2083 | Lower | 0 | Same | 0.2083 | Higher |
| Exits | **0.6250 ✅** | 0.7917 | 1 | -0.1667 | Lower | -0.3750 | Lower | -0.2083 | Lower |

