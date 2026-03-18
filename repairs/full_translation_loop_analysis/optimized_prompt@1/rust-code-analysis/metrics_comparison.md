# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/codenet_statistics copy/r_code/full_translation_loop_pass_intersection_metrics.json`

## p00010

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.0119 | 66.4477 | **68.2920 ✅** | +0.5642 | Higher | -1.2802 | Lower | -1.8443 | Lower |
| MI (SEI) | 45.1146 | **49.7107 ✅** | 39.3067 | -4.5961 | Lower | +5.8079 | Higher | +10.4040 | Higher |
| Cyclomatic (sum) | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 5 | **1 ✅** | 5 | +4 | Higher | 0 | Same | -4 | Lower |
| Halstead Volume | **1891.0235 ✅** | 2203.0526 | 1916.8842 | -312.0291 | Lower | -25.8607 | Lower | +286.1685 | Higher |
| Halstead Effort | **70913.3810 ✅** | 86399.2053 | 74758.4832 | -15485.8243 | Lower | -3845.1022 | Lower | +11640.7221 | Higher |
| SLOC | 50 | 50 | **46 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| PLOC | 34 | **32 ✅** | 35 | +2 | Higher | -1 | Lower | -3 | Lower |
| LLOC | 28 | **17 ✅** | 29 | +11 | Higher | -1 | Lower | -12 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 2 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | 11 | 10 | 10 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.6294 ✅** | 81.3474 | 83.3951 | +6.2820 | Higher | +4.2343 | Higher | -2.0477 | Lower |
| MI (SEI) | **87.5138 ✅** | 79.5290 | 84.2163 | +7.9848 | Higher | +3.2975 | Higher | -4.6873 | Lower |
| Cyclomatic (sum) | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 10 | 21 | 10 | -11 | Lower | 0 | Same | +11 | Higher |
| Halstead Volume | **462.9591 ✅** | 487.2819 | 552.2021 | -24.3228 | Lower | -89.2429 | Lower | -64.9202 | Lower |
| Halstead Effort | **5967.0286 ✅** | 6360.3109 | 11657.5990 | -393.2823 | Lower | -5690.5703 | Lower | -5297.2880 | Lower |
| SLOC | **22 ✅** | 31 | 27 | -9 | Lower | -5 | Lower | +4 | Higher |
| PLOC | **18 ✅** | 20 | 22 | -2 | Lower | -4 | Lower | -2 | Lower |
| LLOC | 10 | 10 | 17 | 0 | Same | -7 | Lower | -7 | Lower |
| Comments (CLOC) | 6 | 9 | 9 | -3 | Lower | -3 | Lower | 0 | Same |
| Blank Lines | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00023

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.0083 | **69.3654 ✅** | 66.0236 | -4.3571 | Lower | -1.0153 | Lower | +3.3417 | Higher |
| MI (SEI) | 42.5295 | **57.5668 ✅** | 20.5693 | -15.0373 | Lower | +21.9602 | Higher | +36.9975 | Higher |
| Cyclomatic (sum) | 9 | **7 ✅** | 10 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | 10 | **9 ✅** | 11 | +1 | Higher | -1 | Lower | -2 | Lower |
| Halstead Volume | 1379.5064 | **1306.6549 ✅** | 1423.7788 | +72.8516 | Higher | -44.2723 | Lower | -117.1239 | Lower |
| Halstead Effort | 38948.0644 | **32143.7095 ✅** | 42103.1719 | +6804.3549 | Higher | -3155.1076 | Lower | -9959.4624 | Lower |
| SLOC | 60 | **48 ✅** | 55 | +12 | Higher | +5 | Higher | -7 | Lower |
| PLOC | 51 | **33 ✅** | 46 | +18 | Higher | +5 | Higher | -13 | Lower |
| LLOC | **17 ✅** | 18 | 19 | -1 | Lower | -2 | Lower | -1 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 0 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p00056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.7531 | 58.5187 | **70.3616 ✅** | +9.2344 | Higher | -2.6084 | Lower | -11.8429 | Lower |
| MI (SEI) | 34.4406 | **45.2454 ✅** | 27.2349 | -10.8048 | Lower | +7.2056 | Higher | +18.0104 | Higher |
| Cyclomatic (sum) | 15 | 17 | **14 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 21 | 22 | 21 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 1101.3004 | 1550.7086 | **1038.0538 ✅** | -449.4082 | Lower | +63.2466 | Higher | +512.6548 | Higher |
| Halstead Effort | **35461.8738 ✅** | 58549.1902 | 41457.2751 | -23087.3165 | Lower | -5995.4014 | Lower | +17091.9151 | Higher |
| SLOC | 50 | 77 | **44 ✅** | -27 | Lower | +6 | Higher | +33 | Higher |
| PLOC | 43 | 54 | **38 ✅** | -11 | Lower | +5 | Higher | +16 | Higher |
| LLOC | **21 ✅** | 34 | 24 | -13 | Lower | -3 | Lower | +10 | Higher |
| Comments (CLOC) | 1 | **19 ✅** | 0 | -18 | Lower | +1 | Higher | +19 | Higher |
| Blank Lines | 6 | 10 | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00077

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.0834 | 70.8443 | **79.2842 ✅** | +5.2391 | Higher | -3.2008 | Lower | -8.4399 | Lower |
| MI (SEI) | 53.0767 | 60.1077 | **65.7033 ✅** | -7.0310 | Lower | -12.6266 | Lower | -5.5956 | Lower |
| Cyclomatic (sum) | 9 | 11 | **6 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Cognitive (sum) | 29 | 19 | **11 ✅** | +10 | Higher | +18 | Higher | +8 | Higher |
| Halstead Volume | 879.0549 | 823.7780 | **791.6176 ✅** | +55.2769 | Higher | +87.4373 | Higher | +32.1604 | Higher |
| Halstead Effort | 16380.8498 | **15350.7855 ✅** | 16821.8737 | +1030.0643 | Higher | -441.0239 | Lower | -1471.0881 | Lower |
| SLOC | 35 | 48 | **31 ✅** | -13 | Lower | +4 | Higher | +17 | Higher |
| PLOC | 32 | 33 | **27 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| LLOC | 24 | **15 ✅** | 21 | +9 | Higher | +3 | Higher | -6 | Lower |
| Comments (CLOC) | 2 | **10 ✅** | 4 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | **3 ✅** | 5 | 4 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00208

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.9196 ✅** | 70.2128 | 71.0180 | +1.7068 | Higher | +0.9016 | Higher | -0.8052 | Lower |
| MI (SEI) | **64.8415 ✅** | 62.0266 | 57.7750 | +2.8148 | Higher | +7.0664 | Higher | +4.2516 | Higher |
| Cyclomatic (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 15 | 15 | **8 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Halstead Volume | 894.8170 | 894.8170 | 993.8036 | 0 | Same | -98.9866 | Lower | -98.9866 | Lower |
| Halstead Effort | 23676.8588 | 23676.8588 | 33644.3941 | 0 | Same | -9967.5353 | Lower | -9967.5353 | Lower |
| SLOC | **45 ✅** | 50 | 46 | -5 | Lower | -1 | Lower | +4 | Higher |
| PLOC | 33 | 33 | 36 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | 29 | 29 | 33 | 0 | Same | -4 | Lower | -4 | Lower |
| Comments (CLOC) | 12 | **13 ✅** | 8 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00292

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.4946 | 77.0369 | **87.7219 ✅** | +3.4576 | Higher | -7.2273 | Lower | -10.6850 | Lower |
| MI (SEI) | 72.8729 | 73.2358 | **78.7931 ✅** | -0.3629 | Lower | -5.9202 | Lower | -5.5573 | Lower |
| Cyclomatic (sum) | 5 | 4 | **3 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 726.1044 | 767.7751 | **600.3206 ✅** | -41.6707 | Lower | +125.7838 | Higher | +167.4545 | Higher |
| Halstead Effort | 14159.0359 | 12426.5824 | **8271.0840 ✅** | +1732.4535 | Higher | +5887.9519 | Higher | +4155.4983 | Higher |
| SLOC | 30 | 37 | **21 ✅** | -7 | Lower | +9 | Higher | +16 | Higher |
| PLOC | 18 | 19 | **14 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| LLOC | 13 | 13 | **10 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Comments (CLOC) | 6 | **11 ✅** | 3 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 7 | 8 | **4 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.0088 | 70.0417 | **82.7071 ✅** | +6.9671 | Higher | -5.6983 | Lower | -12.6654 | Lower |
| MI (SEI) | 72.5595 | 58.8498 | **76.5183 ✅** | +13.7097 | Higher | -3.9588 | Lower | -17.6685 | Lower |
| Cyclomatic (sum) | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 5 | 13 | 5 | -8 | Lower | 0 | Same | +8 | Higher |
| Halstead Volume | 626.8336 | 855.4067 | **527.3324 ✅** | -228.5731 | Lower | +99.5012 | Higher | +328.0743 | Higher |
| Halstead Effort | 9075.4602 | 16474.4991 | **6403.3214 ✅** | -7399.0389 | Lower | +2672.1388 | Higher | +10071.1777 | Higher |
| SLOC | 39 | 52 | **29 ✅** | -13 | Lower | +10 | Higher | +23 | Higher |
| PLOC | 20 | 33 | **18 ✅** | -13 | Lower | +2 | Higher | +15 | Higher |
| LLOC | 11 | 16 | **9 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Comments (CLOC) | 11 | 11 | 6 | 0 | Same | +5 | Higher | +5 | Higher |
| Blank Lines | 8 | 8 | **5 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00445

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.2214 | 78.5106 | **83.9194 ✅** | +0.7108 | Higher | -4.6980 | Lower | -5.4088 | Lower |
| MI (SEI) | 67.6609 | **74.9067 ✅** | 74.5269 | -7.2458 | Lower | -6.8659 | Lower | +0.3799 | Higher |
| Cyclomatic (sum) | 7 | **5 ✅** | 6 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 11 | **2 ✅** | 9 | +9 | Higher | +2 | Higher | -7 | Lower |
| Halstead Volume | 630.9011 | 657.8627 | **561.5282 ✅** | -26.9616 | Lower | +69.3729 | Higher | +96.3345 | Higher |
| Halstead Effort | 12904.7960 | 12499.3918 | **10612.8838 ✅** | +405.4042 | Higher | +2291.9122 | Higher | +1886.5080 | Higher |
| SLOC | 33 | 35 | **26 ✅** | -2 | Lower | +7 | Higher | +9 | Higher |
| PLOC | 23 | 21 | **20 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| LLOC | 17 | 15 | 15 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 5 | **10 ✅** | 4 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 6 | 7 | **4 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00507

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.4506 | 70.8031 | **72.7783 ✅** | -0.3525 | Lower | -2.3278 | Lower | -1.9753 | Lower |
| MI (SEI) | 58.1111 | **59.1078 ✅** | 47.1886 | -0.9967 | Lower | +10.9224 | Higher | +11.9192 | Higher |
| Cyclomatic (sum) | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 9 | 9 | 20 | 0 | Same | -11 | Lower | -11 | Lower |
| Halstead Volume | 1083.4332 | **990.9302 ✅** | 1094.9081 | +92.5030 | Higher | -11.4749 | Lower | -103.9779 | Lower |
| Halstead Effort | 25479.3610 | **23206.9469 ✅** | 35879.2962 | +2272.4142 | Higher | -10399.9352 | Lower | -12672.3493 | Lower |
| SLOC | 47 | 46 | **40 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |
| PLOC | 31 | 31 | 32 | 0 | Same | -1 | Lower | -1 | Lower |
| LLOC | 21 | **17 ✅** | 26 | +4 | Higher | -5 | Lower | -9 | Lower |
| Comments (CLOC) | 9 | 9 | 2 | 0 | Same | +7 | Higher | +7 | Higher |
| Blank Lines | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p00514

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.0164 | **69.2987 ✅** | 51.8868 | -19.2822 | Lower | -1.8703 | Lower | +17.4119 | Higher |
| MI (SEI) | 13.8697 | **55.2216 ✅** | 1.2942 | -41.3520 | Lower | +12.5755 | Higher | +53.9275 | Higher |
| Cyclomatic (sum) | 22 | **11 ✅** | 21 | +11 | Higher | +1 | Higher | -10 | Lower |
| Cognitive (sum) | 21 | **7 ✅** | 18 | +14 | Higher | +3 | Higher | -11 | Lower |
| Halstead Volume | 2738.3689 | **1184.0767 ✅** | 2764.1264 | +1554.2923 | Higher | -25.7575 | Lower | -1580.0497 | Lower |
| Halstead Effort | 152166.1828 | **42300.1184 ✅** | 178009.7402 | +109866.0644 | Higher | -25843.5574 | Lower | -135709.6218 | Lower |
| SLOC | 101 | **47 ✅** | 91 | +54 | Higher | +10 | Higher | -44 | Lower |
| PLOC | 85 | **37 ✅** | 81 | +48 | Higher | +4 | Higher | -44 | Lower |
| LLOC | 56 | **21 ✅** | 53 | +35 | Higher | +3 | Higher | -32 | Lower |
| Comments (CLOC) | 4 | **8 ✅** | 0 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 14 | **8 ✅** | 10 | +6 | Higher | +4 | Higher | -2 | Lower |
| Functions | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p00820

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **81.0160 ✅** | 75.0022 | 77.1759 | +6.0138 | Higher | +3.8401 | Higher | -2.1737 | Lower |
| MI (SEI) | 41.9951 | 51.3484 | **64.0874 ✅** | -9.3534 | Lower | -22.0924 | Lower | -12.7390 | Lower |
| Cyclomatic (sum) | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Cognitive (sum) | 7 | 25 | 7 | -18 | Lower | 0 | Same | +18 | Higher |
| Halstead Volume | **713.1384 ✅** | 797.0770 | 744.6997 | -83.9387 | Lower | -31.5614 | Lower | +52.3773 | Higher |
| Halstead Effort | 22226.1457 | **20377.4472 ✅** | 22850.5233 | +1848.6985 | Higher | -624.3776 | Lower | -2473.0761 | Lower |
| SLOC | **28 ✅** | 37 | 35 | -9 | Lower | -7 | Lower | +2 | Higher |
| PLOC | **25 ✅** | 32 | 26 | -7 | Lower | -1 | Lower | +6 | Higher |
| LLOC | **17 ✅** | 18 | 18 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 0 | 2 | **5 ✅** | -2 | Lower | -5 | Lower | -3 | Lower |
| Blank Lines | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01050

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.6935 | **79.9646 ✅** | 74.0878 | -6.2711 | Lower | -0.3943 | Lower | +5.8768 | Higher |
| MI (SEI) | 63.9752 | **75.9378 ✅** | 63.6072 | -11.9626 | Lower | +0.3681 | Higher | +12.3307 | Higher |
| Cyclomatic (sum) | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 13 | **4 ✅** | 14 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 766.1299 | **544.3964 ✅** | 823.7780 | +221.7335 | Higher | -57.6480 | Lower | -279.3815 | Lower |
| Halstead Effort | 21317.5655 | **8792.0023 ✅** | 23244.8652 | +12525.5632 | Higher | -1927.2997 | Lower | -14452.8628 | Lower |
| SLOC | 43 | **34 ✅** | 41 | +9 | Higher | +2 | Higher | -7 | Lower |
| PLOC | 31 | **18 ✅** | 32 | +13 | Higher | -1 | Lower | -14 | Lower |
| LLOC | 28 | **12 ✅** | 29 | +16 | Higher | -1 | Lower | -17 | Lower |
| Comments (CLOC) | 9 | 9 | 8 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p01125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.8198 | **60.3143 ✅** | 59.6573 | -1.4945 | Lower | -0.8375 | Lower | +0.6570 | Higher |
| MI (SEI) | 41.1450 | **46.4013 ✅** | 11.8938 | -5.2563 | Lower | +29.2512 | Higher | +34.5075 | Higher |
| Cyclomatic (sum) | 20 | 18 | **15 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | 25 | 27 | **11 ✅** | -2 | Lower | +14 | Higher | +16 | Higher |
| Halstead Volume | 2280.0933 | **1547.1351 ✅** | 2307.1484 | +732.9582 | Higher | -27.0551 | Lower | -760.0133 | Lower |
| Halstead Effort | 88048.7185 | **60338.2698 ✅** | 81823.2869 | +27710.4487 | Higher | +6225.4316 | Higher | -21485.0171 | Lower |
| SLOC | **64 ✅** | 68 | 65 | -4 | Lower | -1 | Lower | +3 | Higher |
| PLOC | **50 ✅** | 52 | 55 | -2 | Lower | -5 | Lower | -3 | Lower |
| LLOC | 35 | **29 ✅** | 45 | +6 | Higher | -10 | Lower | -16 | Lower |
| Comments (CLOC) | 11 | **15 ✅** | 0 | -4 | Lower | +11 | Higher | +15 | Higher |
| Blank Lines | 11 | **8 ✅** | 10 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p01601

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.3207 | 74.4941 | **76.7869 ✅** | -3.1734 | Lower | -5.4662 | Lower | -2.2928 | Lower |
| MI (SEI) | 39.6560 | **69.6534 ✅** | 48.8378 | -29.9974 | Lower | -9.1818 | Lower | +20.8157 | Higher |
| Cyclomatic (sum) | 10 | **7 ✅** | 8 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 5 | +4 | Higher | +1 | Higher | -3 | Lower |
| Halstead Volume | 960.6136 | **796.3187 ✅** | 802.5364 | +164.2949 | Higher | +158.0771 | Higher | -6.2177 | Lower |
| Halstead Effort | 35193.3873 | **15289.3190 ✅** | 28511.1633 | +19904.0683 | Higher | +6682.2241 | Higher | -13221.8443 | Lower |
| SLOC | 45 | 41 | **35 ✅** | +4 | Higher | +10 | Higher | +6 | Higher |
| PLOC | 36 | **27 ✅** | 29 | +9 | Higher | +7 | Higher | -2 | Lower |
| LLOC | 25 | **18 ✅** | 21 | +7 | Higher | +4 | Higher | -3 | Lower |
| Comments (CLOC) | 1 | **12 ✅** | 1 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 8 | 7 | **6 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 4 | **1 ✅** | 0 | Same | +3 | Higher | +3 | Higher |

## p02162

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.0991 | **72.7154 ✅** | 69.4948 | -7.6163 | Lower | -4.3957 | Lower | +3.2205 | Higher |
| MI (SEI) | 51.7459 | **65.8404 ✅** | 53.0535 | -14.0945 | Lower | -1.3076 | Lower | +12.7869 | Higher |
| Cyclomatic (sum) | 18 | **10 ✅** | 14 | +8 | Higher | +4 | Higher | -4 | Lower |
| Cognitive (sum) | 17 | 16 | **13 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Halstead Volume | 1128.7712 | **910.7838 ✅** | 1226.3358 | +217.9874 | Higher | -97.5646 | Lower | -315.5520 | Lower |
| Halstead Effort | 28199.8194 | **18063.8795 ✅** | 34465.1467 | +10135.9399 | Higher | -6265.3273 | Lower | -16401.2672 | Lower |
| SLOC | 56 | **42 ✅** | 44 | +14 | Higher | +12 | Higher | -2 | Lower |
| PLOC | 45 | **29 ✅** | 35 | +16 | Higher | +10 | Higher | -6 | Lower |
| LLOC | **14 ✅** | 17 | 19 | -3 | Lower | -5 | Lower | -2 | Lower |
| Comments (CLOC) | 11 | 11 | 6 | 0 | Same | +5 | Higher | +5 | Higher |
| Blank Lines | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |
| Exits | 6 | **0 ✅** | 2 | +6 | Higher | +4 | Higher | -2 | Lower |

## p02190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.4906 | **81.9987 ✅** | 76.1354 | -1.5081 | Lower | +4.3552 | Higher | +5.8632 | Higher |
| MI (SEI) | 69.9699 | **75.4962 ✅** | 59.1317 | -5.5264 | Lower | +10.8381 | Higher | +16.3645 | Higher |
| Cyclomatic (sum) | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | **2 ✅** | 6 | 4 | -4 | Lower | -2 | Lower | +2 | Higher |
| Halstead Volume | 685.7667 | **604.2966 ✅** | 835.8092 | +81.4701 | Higher | -150.0425 | Lower | -231.5126 | Lower |
| Halstead Effort | 10733.7390 | **7172.7378 ✅** | 17606.5025 | +3561.0012 | Higher | -6872.7635 | Lower | -10433.7647 | Lower |
| SLOC | 31 | **29 ✅** | 37 | +2 | Higher | -6 | Lower | -8 | Lower |
| PLOC | 19 | **18 ✅** | 25 | +1 | Higher | -6 | Lower | -7 | Lower |
| LLOC | 13 | **11 ✅** | 18 | +2 | Higher | -5 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 4 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 7 | **5 ✅** | 8 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p02256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.4924 | **68.9668 ✅** | 63.4922 | -0.4744 | Lower | +5.0002 | Higher | +5.4746 | Higher |
| MI (SEI) | **55.0441 ✅** | 51.1581 | 47.9991 | +3.8860 | Higher | +7.0450 | Higher | +3.1590 | Higher |
| Cyclomatic (sum) | **11 ✅** | 13 | 12 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 7 | **6 ✅** | 10 | +1 | Higher | -3 | Lower | -4 | Lower |
| Halstead Volume | 1214.3191 | **1081.9189 ✅** | 1260.1425 | +132.4003 | Higher | -45.8234 | Lower | -178.2236 | Lower |
| Halstead Effort | 35332.7697 | **33153.0848 ✅** | 37195.2061 | +2179.6849 | Higher | -1862.4364 | Lower | -4042.1214 | Lower |
| SLOC | 49 | **48 ✅** | 65 | +1 | Higher | -16 | Lower | -17 | Lower |
| PLOC | 37 | **36 ✅** | 48 | +1 | Higher | -11 | Lower | -12 | Lower |
| LLOC | **20 ✅** | 22 | 31 | -2 | Lower | -11 | Lower | -9 | Lower |
| Comments (CLOC) | 9 | 6 | **12 ✅** | +3 | Higher | -3 | Lower | -6 | Lower |
| Blank Lines | 10 | **7 ✅** | 14 | +3 | Higher | -4 | Lower | -7 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |

## p02297

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.4828 | 66.2674 | **78.4165 ✅** | +8.2154 | Higher | -3.9337 | Lower | -12.1491 | Lower |
| MI (SEI) | 61.2380 | 49.8122 | **69.4394 ✅** | +11.4258 | Higher | -8.2014 | Lower | -19.6272 | Lower |
| Cyclomatic (sum) | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 3 | 5 | **2 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | 1056.9386 | 1403.2117 | **977.6705 ✅** | -346.2731 | Lower | +79.2681 | Higher | +425.5412 | Higher |
| Halstead Effort | **24573.8224 ✅** | 43006.5424 | 26885.9378 | -18432.7200 | Lower | -2312.1155 | Lower | +16120.6046 | Higher |
| SLOC | 38 | 56 | **31 ✅** | -18 | Lower | +7 | Higher | +25 | Higher |
| PLOC | 25 | 37 | **21 ✅** | -12 | Lower | +4 | Higher | +16 | Higher |
| LLOC | 18 | 25 | **16 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| Comments (CLOC) | 6 | **9 ✅** | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 8 | 11 | **6 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02393

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **86.9728 ✅** | 84.4364 | 75.3405 | +2.5364 | Higher | +11.6323 | Higher | +9.0959 | Higher |
| MI (SEI) | **82.6635 ✅** | 82.5236 | 63.4562 | +0.1400 | Higher | +19.2074 | Higher | +19.0674 | Higher |
| Cyclomatic (sum) | 4 | 4 | 13 | 0 | Same | -9 | Lower | -9 | Lower |
| Cognitive (sum) | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |
| Halstead Volume | **437.5941 ✅** | 555.4087 | 1240.0092 | -117.8146 | Lower | -802.4152 | Lower | -684.6006 | Lower |
| Halstead Effort | **5803.8794 ✅** | 7570.4620 | 56656.6129 | -1766.5826 | Lower | -50852.7335 | Lower | -49086.1509 | Lower |
| SLOC | **24 ✅** | 26 | 31 | -2 | Lower | -7 | Lower | -5 | Lower |
| PLOC | **14 ✅** | 16 | 23 | -2 | Lower | -9 | Lower | -7 | Lower |
| LLOC | 7 | 8 | **5 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Comments (CLOC) | 5 | **7 ✅** | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02396

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 89.1626 | **90.4473 ✅** | 83.6279 | -1.2848 | Lower | +5.5347 | Higher | +6.8194 | Higher |
| MI (SEI) | 81.1769 | **86.4830 ✅** | 76.0038 | -5.3061 | Lower | +5.1731 | Higher | +10.4792 | Higher |
| Cyclomatic (sum) | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 10 | 3 | **2 ✅** | +7 | Higher | +8 | Higher | +1 | Higher |
| Halstead Volume | 398.5071 | **340.0587 ✅** | 471.4646 | +58.4485 | Higher | -72.9574 | Lower | -131.4059 | Lower |
| Halstead Effort | 5927.7936 | **4372.1829 ✅** | 7425.5671 | +1555.6107 | Higher | -1497.7734 | Lower | -3053.3842 | Lower |
| SLOC | 21 | 21 | 28 | 0 | Same | -7 | Lower | -7 | Lower |
| PLOC | 17 | **14 ✅** | 18 | +3 | Higher | -1 | Lower | -4 | Lower |
| LLOC | 11 | **10 ✅** | 12 | +1 | Higher | -1 | Lower | -2 | Lower |
| Comments (CLOC) | 3 | 4 | **5 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | **2 ✅** | 4 | 6 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02412

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.8396 | 73.6566 | **75.5319 ✅** | +1.1830 | Higher | -0.6923 | Lower | -1.8753 | Lower |
| MI (SEI) | **57.9750 ✅** | 57.3311 | 47.5362 | +0.6439 | Higher | +10.4388 | Higher | +9.7949 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 25 | **14 ✅** | 25 | +11 | Higher | 0 | Same | -11 | Lower |
| Halstead Volume | **786.8084 ✅** | 819.2373 | 818.9147 | -32.4290 | Lower | -32.1064 | Lower | +0.3226 | Higher |
| Halstead Effort | 21843.2996 | **21013.4379 ✅** | 22520.1554 | +829.8617 | Higher | -676.8559 | Lower | -1506.7176 | Lower |
| SLOC | 37 | 41 | **35 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| PLOC | **27 ✅** | 29 | 28 | -2 | Lower | -1 | Lower | +1 | Higher |
| LLOC | 18 | 18 | 19 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 4 | **5 ✅** | 1 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02418

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.5811 | **92.4176 ✅** | 82.9406 | -11.8365 | Lower | -2.3596 | Lower | +9.4769 | Higher |
| MI (SEI) | 74.8382 | **99.5560 ✅** | 78.2835 | -24.7177 | Lower | -3.4453 | Lower | +21.2725 | Higher |
| Cyclomatic (sum) | 7 | 4 | **3 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Cognitive (sum) | 4 | 2 | 2 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 590.1941 | **271.0286 ✅** | 495.5960 | +319.1655 | Higher | +94.5982 | Higher | -224.5674 | Lower |
| Halstead Effort | 9836.5685 | **2516.6940 ✅** | 6112.3501 | +7319.8745 | Higher | +3724.2185 | Higher | -3595.6560 | Lower |
| SLOC | 31 | **20 ✅** | 30 | +11 | Higher | +1 | Higher | -10 | Lower |
| PLOC | 18 | **12 ✅** | 17 | +6 | Higher | +1 | Higher | -5 | Lower |
| LLOC | 12 | **4 ✅** | 12 | +8 | Higher | 0 | Same | -8 | Lower |
| Comments (CLOC) | 7 | **8 ✅** | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.5835 | 70.2372 | **74.0074 ✅** | +1.3463 | Higher | -2.4239 | Lower | -3.7702 | Lower |
| MI (SEI) | 44.9698 | **63.7584 ✅** | 49.7382 | -18.7886 | Lower | -4.7684 | Lower | +14.0202 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1236.9954 | **1179.8645 ✅** | 1254.5655 | +57.1309 | Higher | -17.5701 | Lower | -74.7010 | Lower |
| Halstead Effort | 40087.8145 | **31982.7548 ✅** | 38934.7906 | +8105.0597 | Higher | +1153.0239 | Higher | -6952.0358 | Lower |
| SLOC | 42 | 47 | **36 ✅** | -5 | Lower | +6 | Higher | +11 | Higher |
| PLOC | 35 | **29 ✅** | 30 | +6 | Higher | +5 | Higher | -1 | Lower |
| LLOC | **18 ✅** | 20 | 19 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 2 | **14 ✅** | 2 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | 5 | 8 | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02606

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 86.2800 | 83.4569 | **90.5053 ✅** | +2.8231 | Higher | -4.2252 | Lower | -7.0483 | Lower |
| MI (SEI) | 76.0476 | **84.9750 ✅** | 79.7618 | -8.9274 | Lower | -3.7143 | Lower | +5.2131 | Higher |
| Cyclomatic (sum) | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 716.2669 | **387.6444 ✅** | 593.8777 | +328.6226 | Higher | +122.3893 | Higher | -206.2333 | Lower |
| Halstead Effort | 9125.7715 | **6008.4875 ✅** | 6326.0884 | +3117.2839 | Higher | +2799.6831 | Higher | -317.6008 | Lower |
| SLOC | 22 | 31 | **18 ✅** | -9 | Lower | +4 | Higher | +13 | Higher |
| PLOC | 14 | 15 | **12 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| LLOC | 11 | 9 | 9 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 3 | **11 ✅** | 2 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 5 | 6 | **4 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p02731

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 91.1202 | 91.1891 | **92.5774 ✅** | -0.0689 | Lower | -1.4572 | Lower | -1.3883 | Lower |
| MI (SEI) | 93.4769 | 93.5763 | **98.4513 ✅** | -0.0994 | Lower | -4.9743 | Lower | -4.8750 | Lower |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 380 | 375 | **336.8853 ✅** | +5 | Higher | +43.1147 | Higher | +38.1147 | Higher |
| Halstead Effort | 3990 | **3335.5263 ✅** | 3467.9374 | +654.4737 | Higher | +522.0626 | Higher | -132.4111 | Lower |
| SLOC | 20 | 20 | **19 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| PLOC | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 6 | 6 | **7 ✅** | 0 | Same | -1 | Lower | -1 | Lower |
| Blank Lines | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02783

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.2582 | 84.8367 | **87.2524 ✅** | -8.5785 | Lower | -10.9942 | Lower | -2.4157 | Lower |
| MI (SEI) | 69.8256 | **84.8686 ✅** | 83.5393 | -15.0429 | Lower | -13.7137 | Lower | +1.3292 | Higher |
| Cyclomatic (sum) | 9 | 3 | 3 | +6 | Higher | +6 | Higher | 0 | Same |
| Cognitive (sum) | 4 | 1 | 1 | +3 | Higher | +3 | Higher | 0 | Same |
| Halstead Volume | 714.8752 | 537.5147 | **494.8981 ✅** | +177.3605 | Higher | +219.9771 | Higher | +42.6167 | Higher |
| Halstead Effort | 15369.8168 | 6753.9895 | **6520.2821 ✅** | +8615.8273 | Higher | +8849.5347 | Higher | +233.7075 | Higher |
| SLOC | 37 | 26 | **23 ✅** | +11 | Higher | +14 | Higher | +3 | Higher |
| PLOC | 24 | 13 | 13 | +11 | Higher | +11 | Higher | 0 | Same |
| LLOC | 13 | **9 ✅** | 10 | +4 | Higher | +3 | Higher | -1 | Lower |
| Comments (CLOC) | **9 ✅** | 8 | 5 | +1 | Higher | +4 | Higher | +3 | Higher |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |

## p02812

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.5758 | **80.2332 ✅** | 75.3702 | -7.6574 | Lower | -2.7944 | Lower | +4.8630 | Higher |
| MI (SEI) | 65.4826 | **81.9725 ✅** | 64.9475 | -16.4899 | Lower | +0.5351 | Higher | +17.0251 | Higher |
| Cyclomatic (sum) | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 900.6639 | **542.8367 ✅** | 926.4271 | +357.8272 | Higher | -25.7632 | Lower | -383.5905 | Lower |
| Halstead Effort | 26835.6905 | **15877.9725 ✅** | 22645.9964 | +10957.7181 | Higher | +4189.6942 | Higher | -6768.0239 | Lower |
| SLOC | 45 | **33 ✅** | 37 | +12 | Higher | +8 | Higher | -4 | Lower |
| PLOC | 30 | **23 ✅** | 25 | +7 | Higher | +5 | Higher | -2 | Lower |
| LLOC | 25 | **17 ✅** | 24 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 12 | **13 ✅** | 7 | -1 | Lower | +5 | Higher | +6 | Higher |
| Blank Lines | 8 | **4 ✅** | 7 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p02945

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.1935 ✅** | 71.8747 | 76.9340 | +7.3188 | Higher | +2.2595 | Higher | -5.0593 | Lower |
| MI (SEI) | **69.6372 ✅** | 66.4138 | 69.2569 | +3.2234 | Higher | +0.3803 | Higher | -2.8431 | Lower |
| Cyclomatic (sum) | **4 ✅** | 11 | 6 | -7 | Lower | -2 | Lower | +5 | Higher |
| Cognitive (sum) | **1 ✅** | 6 | 3 | -5 | Lower | -2 | Lower | +3 | Higher |
| Halstead Volume | **724.2976 ✅** | 886.0854 | 780.6987 | -161.7879 | Lower | -56.4012 | Lower | +105.3867 | Higher |
| Halstead Effort | **13278.7888 ✅** | 24810.3918 | 15426.6067 | -11531.6030 | Lower | -2147.8180 | Lower | +9383.7851 | Higher |
| SLOC | **33 ✅** | 44 | 36 | -11 | Lower | -3 | Lower | +8 | Higher |
| PLOC | **21 ✅** | 29 | 24 | -8 | Lower | -3 | Lower | +5 | Higher |
| LLOC | 14 | **11 ✅** | 20 | +3 | Higher | -6 | Lower | -9 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 8 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p02957

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1929 | 72.6919 | **85.7932 ✅** | +1.5010 | Higher | -11.6003 | Lower | -13.1013 | Lower |
| MI (SEI) | 57.9012 | 58.9528 | **84.9271 ✅** | -1.0517 | Lower | -27.0259 | Lower | -25.9742 | Lower |
| Cyclomatic (sum) | 8 | 8 | **3 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Cognitive (sum) | 5 | 5 | **1 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Halstead Volume | 807.3024 | 864.6653 | **505.3170 ✅** | -57.3629 | Lower | +301.9854 | Higher | +359.3483 | Higher |
| Halstead Effort | 18918.9564 | 21582.0463 | **6657.5512 ✅** | -2663.0899 | Lower | +12261.4052 | Higher | +14924.4951 | Higher |
| SLOC | 41 | 44 | **25 ✅** | -3 | Lower | +16 | Higher | +19 | Higher |
| PLOC | 29 | 31 | **13 ✅** | -2 | Lower | +16 | Higher | +18 | Higher |
| LLOC | 15 | 17 | **9 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| Comments (CLOC) | 5 | 7 | 7 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 3 | **1 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p03029

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 91.7577 | 90.3630 | **91.9320 ✅** | +1.3947 | Higher | -0.1743 | Lower | -1.5690 | Lower |
| MI (SEI) | 90.2357 | **93.1039 ✅** | 90.4872 | -2.8682 | Lower | -0.2515 | Lower | +2.6167 | Higher |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 466.7648 | 515.7359 | **451.3769 ✅** | -48.9711 | Lower | +15.3878 | Higher | +64.3590 | Higher |
| Halstead Effort | 5251.1035 | 5626.2096 | **4739.4575 ✅** | -375.1061 | Lower | +511.6460 | Higher | +886.7521 | Higher |
| SLOC | 18 | 19 | 18 | -1 | Lower | 0 | Same | +1 | Higher |
| PLOC | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 7 | 8 | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 4 | **6 ✅** | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.4717 | 70.4262 | **74.4402 ✅** | -0.9545 | Lower | -4.9685 | Lower | -4.0140 | Lower |
| MI (SEI) | 54.4352 | **61.9941 ✅** | 56.2387 | -7.5589 | Lower | -1.8034 | Lower | +5.7555 | Higher |
| Cyclomatic (sum) | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 10 | **9 ✅** | 12 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 888.0090 | 892.7389 | **760.9787 ✅** | -4.7298 | Lower | +127.0304 | Higher | +131.7602 | Higher |
| Halstead Effort | 25326.0178 | 22913.6309 | **15067.3777 ✅** | +2412.3869 | Higher | +10258.6400 | Higher | +7846.2532 | Higher |
| SLOC | 51 | 48 | **40 ✅** | +3 | Higher | +11 | Higher | +8 | Higher |
| PLOC | 34 | 33 | **31 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| LLOC | 21 | **18 ✅** | 19 | +3 | Higher | +2 | Higher | -1 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 4 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 9 | 7 | **5 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |

## p03145

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.7627 | **88.2334 ✅** | 88.1397 | -0.4707 | Lower | -0.3770 | Lower | +0.0937 | Higher |
| MI (SEI) | 87.2988 | **90.7892 ✅** | 85.9597 | -3.4903 | Lower | +1.3391 | Higher | +4.8295 | Higher |
| Cyclomatic (sum) | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **538.5747 ✅** | 568.6918 | 579.0316 | -30.1170 | Lower | -40.4569 | Lower | -10.3399 | Lower |
| Halstead Effort | **4932.8548 ✅** | 6702.4385 | 5400.0991 | -1769.5837 | Lower | -467.2443 | Lower | +1302.3394 | Higher |
| SLOC | 22 | 21 | 21 | +1 | Higher | +1 | Higher | 0 | Same |
| PLOC | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| LLOC | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 6 | **7 ✅** | 5 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03157

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.7239 | 47.6983 | **57.4136 ✅** | +4.0256 | Higher | -5.6896 | Lower | -9.7152 | Lower |
| MI (SEI) | 10.5446 | **17.9675 ✅** | 8.7585 | -7.4229 | Lower | +1.7861 | Higher | +9.2090 | Higher |
| Cyclomatic (sum) | 29 | 27 | **16 ✅** | +2 | Higher | +13 | Higher | +11 | Higher |
| Cognitive (sum) | 27 | 38 | **20 ✅** | -11 | Lower | +7 | Higher | +18 | Higher |
| Halstead Volume | 3110.3886 | 3224.9152 | **2581.0842 ✅** | -114.5266 | Lower | +529.3044 | Higher | +643.8310 | Higher |
| Halstead Effort | 162362.2861 | 180419.3495 | **128330.2496 ✅** | -18057.0634 | Lower | +34032.0365 | Higher | +52089.0999 | Higher |
| SLOC | 79 | 103 | **71 ✅** | -24 | Lower | +8 | Higher | +32 | Higher |
| PLOC | 67 | 80 | **59 ✅** | -13 | Lower | +8 | Higher | +21 | Higher |
| LLOC | 50 | 43 | **41 ✅** | +7 | Higher | +9 | Higher | +2 | Higher |
| Comments (CLOC) | 1 | **9 ✅** | 0 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 12 | 14 | 12 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 12 | 4 | **1 ✅** | +8 | Higher | +11 | Higher | +3 | Higher |

## p03183

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.2941 ✅** | 68.3688 | 65.6917 | +0.9254 | Higher | +3.6025 | Higher | +2.6771 | Higher |
| MI (SEI) | 48.7281 | **54.0398 ✅** | 30.5483 | -5.3117 | Lower | +18.1798 | Higher | +23.4915 | Higher |
| Cyclomatic (sum) | **7 ✅** | 8 | 9 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 4 | 4 | 7 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | **1242.2611 ✅** | 1253.6060 | 1780.3047 | -11.3449 | Lower | -538.0435 | Lower | -526.6987 | Lower |
| Halstead Effort | **27650.3288 ✅** | 35187.4236 | 51160.3350 | -7537.0948 | Lower | -23510.0062 | Lower | -15972.9114 | Lower |
| SLOC | **49 ✅** | 51 | 53 | -2 | Lower | -4 | Lower | -2 | Lower |
| PLOC | **36 ✅** | 37 | 42 | -1 | Lower | -6 | Lower | -5 | Lower |
| LLOC | 18 | 18 | 23 | 0 | Same | -5 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 1 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 8 | **7 ✅** | 10 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p03194

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.7803 | 61.9902 | **81.3337 ✅** | +11.7901 | Higher | -7.5534 | Lower | -19.3435 | Lower |
| MI (SEI) | **59.8901 ✅** | 49.6267 | 42.3516 | +10.2634 | Higher | +17.5386 | Higher | +7.2751 | Higher |
| Cyclomatic (sum) | 9 | 13 | **7 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Cognitive (sum) | 9 | 11 | **7 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Halstead Volume | 903.0170 | 1277.5850 | **785.3350 ✅** | -374.5680 | Lower | +117.6820 | Higher | +492.2500 | Higher |
| Halstead Effort | 30211.8076 | 65476.2326 | **22578.3810 ✅** | -35264.4250 | Lower | +7633.4265 | Higher | +42897.8515 | Higher |
| SLOC | 40 | 70 | **27 ✅** | -30 | Lower | +13 | Higher | +43 | Higher |
| PLOC | 28 | 47 | **23 ✅** | -19 | Lower | +5 | Higher | +24 | Higher |
| LLOC | 25 | 38 | **20 ✅** | -13 | Lower | +5 | Higher | +18 | Higher |
| Comments (CLOC) | 6 | **17 ✅** | 0 | -11 | Lower | +6 | Higher | +17 | Higher |
| Blank Lines | 7 | 12 | **4 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03206

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **88.2015 ✅** | 87.8630 | 86.5935 | +0.3384 | Higher | +1.6080 | Higher | +1.2696 | Higher |
| MI (SEI) | 86.7616 | **88.7049 ✅** | 75.8752 | -1.9434 | Lower | +10.8864 | Higher | +12.8297 | Higher |
| Cyclomatic (sum) | 9 | 8 | **6 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 6 | **3 ✅** | 4 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | **419.8342 ✅** | 468.3297 | 430.8598 | -48.4955 | Lower | -11.0256 | Lower | +37.4699 | Higher |
| Halstead Effort | 4758.1209 | 5365.4291 | **4452.2177 ✅** | -607.3081 | Lower | +305.9033 | Higher | +913.2114 | Higher |
| SLOC | 21 | 21 | 24 | 0 | Same | -3 | Lower | -3 | Lower |
| PLOC | 15 | 15 | 18 | 0 | Same | -3 | Lower | -3 | Lower |
| LLOC | **4 ✅** | 5 | 11 | -1 | Lower | -7 | Lower | -6 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 3 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03318

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.9121 ✅** | 68.6367 | 74.6965 | +6.2754 | Higher | +0.2157 | Higher | -6.0597 | Lower |
| MI (SEI) | 33.0872 | 47.8564 | **54.1241 ✅** | -14.7692 | Lower | -21.0369 | Lower | -6.2677 | Lower |
| Cyclomatic (sum) | 7 | 12 | 7 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 7 | 10 | 7 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 1011.7379 | **997.5770 ✅** | 1054.5813 | +14.1610 | Higher | -42.8434 | Lower | -57.0044 | Lower |
| Halstead Effort | 30840.5637 | **28644.7096 ✅** | 39293.7010 | +2195.8541 | Higher | -8453.1373 | Lower | -10648.9914 | Lower |
| SLOC | 37 | 51 | 37 | -14 | Lower | 0 | Same | +14 | Higher |
| PLOC | **31 ✅** | 44 | 33 | -13 | Lower | -2 | Lower | +11 | Higher |
| LLOC | **26 ✅** | 31 | 29 | -5 | Lower | -3 | Lower | +2 | Higher |
| Comments (CLOC) | 0 | **5 ✅** | 3 | -5 | Lower | -3 | Lower | +2 | Higher |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p03369

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 86.8730 | **88.0872 ✅** | 85.6761 | -1.2142 | Lower | +1.1969 | Higher | +2.4111 | Higher |
| MI (SEI) | 85.0091 | **89.5067 ✅** | 79.7259 | -4.4976 | Lower | +5.2831 | Higher | +9.7807 | Higher |
| Cyclomatic (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 446.0738 | **403.2542 ✅** | 437.5941 | +42.8197 | Higher | +8.4798 | Higher | -34.3399 | Lower |
| Halstead Effort | 5416.6109 | **4870.8857 ✅** | 6928.5730 | +545.7252 | Higher | -1511.9621 | Lower | -2057.6873 | Lower |
| SLOC | 24 | **23 ✅** | 26 | +1 | Higher | -2 | Lower | -3 | Lower |
| PLOC | 12 | **11 ✅** | 14 | +1 | Higher | -2 | Lower | -3 | Lower |
| LLOC | 8 | 8 | 10 | 0 | Same | -2 | Lower | -2 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 5 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 6 | **5 ✅** | 7 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03463

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.3959 | 78.5014 | **88.4574 ✅** | +1.8945 | Higher | -8.0614 | Lower | -9.9560 | Lower |
| MI (SEI) | 69.9351 | **71.8004 ✅** | 69.1978 | -1.8653 | Lower | +0.7373 | Higher | +2.6027 | Higher |
| Cyclomatic (sum) | 5 | 5 | **3 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 668.1505 | 659.0256 | **606.6998 ✅** | +9.1249 | Higher | +61.4507 | Higher | +52.3258 | Higher |
| Halstead Effort | 11160.5875 | 11152.7405 | **6875.9311 ✅** | +7.8470 | Higher | +4284.6564 | Higher | +4276.8094 | Higher |
| SLOC | 31 | 35 | **20 ✅** | -4 | Lower | +11 | Higher | +15 | Higher |
| PLOC | 20 | 22 | **15 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| LLOC | 11 | 13 | **10 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Comments (CLOC) | 5 | **8 ✅** | 1 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03473

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 91.7133 | 90.5501 | **98.9770 ✅** | +1.1632 | Higher | -7.2636 | Lower | -8.4269 | Lower |
| MI (SEI) | 91.9922 | 92.1743 | **100.1579 ✅** | -0.1821 | Lower | -8.1657 | Lower | -7.9836 | Lower |
| Cyclomatic (sum) | 4 | 4 | **2 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 310.3352 | 333.4034 | **254.7898 ✅** | -23.0682 | Lower | +55.5454 | Higher | +78.6136 | Higher |
| Halstead Effort | 3258.5199 | 3621.1313 | **2097.7694 ✅** | -362.6114 | Lower | +1160.7505 | Higher | +1523.3619 | Higher |
| SLOC | 20 | 21 | **14 ✅** | -1 | Lower | +6 | Higher | +7 | Higher |
| PLOC | 12 | 13 | **7 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| LLOC | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 5 | **6 ✅** | 3 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03544

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.4240 ✅** | 79.6094 | 36.0385 | +7.8146 | Higher | +51.3855 | Higher | +43.5709 | Higher |
| MI (SEI) | **84.4934 ✅** | 82.0784 | -2.7100 | +2.4150 | Higher | +87.2034 | Higher | +84.7884 | Higher |
| Cyclomatic (sum) | **4 ✅** | 6 | 39 | -2 | Lower | -35 | Lower | -33 | Lower |
| Cognitive (sum) | **2 ✅** | 3 | 34 | -1 | Lower | -32 | Lower | -31 | Lower |
| Halstead Volume | **526.1548 ✅** | 612.0217 | 4617.1979 | -85.8669 | Lower | -4091.0431 | Lower | -4005.1762 | Lower |
| Halstead Effort | **11385.4050 ✅** | 11905.2799 | 426524.5490 | -519.8748 | Lower | -415139.1439 | Lower | -414619.2691 | Lower |
| SLOC | **22 ✅** | 33 | 159 | -11 | Lower | -137 | Lower | -126 | Lower |
| PLOC | **15 ✅** | 23 | 137 | -8 | Lower | -122 | Lower | -114 | Lower |
| LLOC | **13 ✅** | 18 | 96 | -5 | Lower | -83 | Lower | -78 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 8 | -9 | Lower | -3 | Lower | +6 | Higher |
| Blank Lines | **4 ✅** | 5 | 20 | -1 | Lower | -16 | Lower | -15 | Lower |
| Functions | **1 ✅** | 2 | 11 | -1 | Lower | -10 | Lower | -9 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 3 | 12 | -3 | Lower | -12 | Lower | -9 | Lower |

## p03568

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **83.4385 ✅** | 80.8609 | 80.5194 | +2.5776 | Higher | +2.9191 | Higher | +0.3415 | Higher |
| MI (SEI) | 76.0012 | 72.8641 | **76.3848 ✅** | +3.1371 | Higher | -0.3836 | Lower | -3.5207 | Lower |
| Cyclomatic (sum) | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 3 | 3 | 4 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **598.2637 ✅** | 638.6314 | 652.4704 | -40.3676 | Lower | -54.2067 | Lower | -13.8390 | Lower |
| Halstead Effort | **9153.4352 ✅** | 9927.8150 | 13049.4082 | -774.3798 | Lower | -3895.9730 | Lower | -3121.5932 | Lower |
| SLOC | **27 ✅** | 31 | 31 | -4 | Lower | -4 | Lower | 0 | Same |
| PLOC | **16 ✅** | 18 | 20 | -2 | Lower | -4 | Lower | -2 | Lower |
| LLOC | **12 ✅** | 14 | 16 | -2 | Lower | -4 | Lower | -2 | Lower |
| Comments (CLOC) | 5 | 6 | **8 ✅** | -1 | Lower | -3 | Lower | -2 | Lower |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03592

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.3132 ✅** | 76.5960 | 52.4839 | +2.7172 | Higher | +26.8294 | Higher | +24.1122 | Higher |
| MI (SEI) | 53.4928 | **67.0778 ✅** | 2.2574 | -13.5850 | Lower | +51.2354 | Higher | +64.8204 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 22 | +1 | Higher | -14 | Lower | -15 | Lower |
| Cognitive (sum) | **8 ✅** | 10 | 18 | -2 | Lower | -10 | Lower | -8 | Lower |
| Halstead Volume | 798.0630 | **797.0770 ✅** | 2278.7366 | +0.9860 | Higher | -1480.6736 | Lower | -1481.6596 | Lower |
| Halstead Effort | 19552.5430 | **12967.8299 ✅** | 100176.7659 | +6584.7131 | Higher | -80624.2230 | Lower | -87208.9360 | Lower |
| SLOC | **30 ✅** | 36 | 92 | -6 | Lower | -62 | Lower | -56 | Lower |
| PLOC | **23 ✅** | 27 | 77 | -4 | Lower | -54 | Lower | -50 | Lower |
| LLOC | **16 ✅** | 19 | 47 | -3 | Lower | -31 | Lower | -28 | Lower |
| Comments (CLOC) | 1 | **7 ✅** | 0 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 6 | **4 ✅** | 15 | +2 | Higher | -9 | Lower | -11 | Lower |
| Functions | 1 | 1 | 9 | 0 | Same | -8 | Lower | -8 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **0 ✅** | 7 | +2 | Higher | -5 | Lower | -7 | Lower |

## p03632

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.8329 | **81.0524 ✅** | 71.3845 | -13.2196 | Lower | -3.5517 | Lower | +9.6679 | Higher |
| MI (SEI) | 50.5323 | **83.4726 ✅** | 53.0406 | -32.9403 | Lower | -2.5084 | Lower | +30.4319 | Higher |
| Cyclomatic (sum) | 18 | **5 ✅** | 8 | +13 | Higher | +10 | Higher | -3 | Lower |
| Cognitive (sum) | 17 | **3 ✅** | 7 | +14 | Higher | +10 | Higher | -4 | Lower |
| Halstead Volume | 1231.5029 | **533.4454 ✅** | 1111.8397 | +698.0575 | Higher | +119.6632 | Higher | -578.3942 | Lower |
| Halstead Effort | 50993.3422 | **10328.0963 ✅** | 32675.7325 | +40665.2459 | Higher | +18317.6097 | Higher | -22347.6362 | Lower |
| SLOC | 46 | **32 ✅** | 44 | +14 | Higher | +2 | Higher | -12 | Lower |
| PLOC | 32 | **21 ✅** | 32 | +11 | Higher | 0 | Same | -11 | Lower |
| LLOC | 22 | **11 ✅** | 26 | +11 | Higher | -4 | Lower | -15 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 5 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03719

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.9600 | **83.6820 ✅** | 36.2659 | -2.7219 | Lower | +44.6941 | Higher | +47.4161 | Higher |
| MI (SEI) | 68.5104 | **70.7517 ✅** | -5.3410 | -2.2412 | Lower | +73.8514 | Higher | +76.0926 | Higher |
| Cyclomatic (sum) | 6 | 6 | 41 | 0 | Same | -35 | Lower | -35 | Lower |
| Cognitive (sum) | 4 | 4 | 36 | 0 | Same | -32 | Lower | -32 | Lower |
| Halstead Volume | 635.2096 | **587.7626 ✅** | 4654.6587 | +47.4470 | Higher | -4019.4490 | Lower | -4066.8961 | Lower |
| Halstead Effort | 10137.9456 | **8739.7743 ✅** | 406506.8572 | +1398.1713 | Higher | -396368.9116 | Lower | -397767.0829 | Lower |
| SLOC | 30 | **26 ✅** | 152 | +4 | Higher | -122 | Lower | -126 | Lower |
| PLOC | 20 | **19 ✅** | 131 | +1 | Higher | -111 | Lower | -112 | Lower |
| LLOC | 12 | **11 ✅** | 93 | +1 | Higher | -81 | Lower | -82 | Lower |
| Comments (CLOC) | 4 | 3 | **5 ✅** | +1 | Higher | -1 | Lower | -2 | Lower |
| Blank Lines | 6 | **4 ✅** | 19 | +2 | Higher | -13 | Lower | -15 | Lower |
| Functions | 1 | 1 | 12 | 0 | Same | -11 | Lower | -11 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | **0 ✅** | 13 | +1 | Higher | -12 | Lower | -13 | Lower |

## p03837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.6572 | **59.7029 ✅** | 58.9693 | -2.0457 | Lower | -1.3121 | Lower | +0.7336 | Higher |
| MI (SEI) | 34.6248 | **44.6850 ✅** | 24.4237 | -10.0602 | Lower | +10.2011 | Higher | +20.2613 | Higher |
| Cyclomatic (sum) | **14 ✅** | 17 | 16 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | 19 | 29 | 19 | -10 | Lower | 0 | Same | +10 | Higher |
| Halstead Volume | 2365.2588 | **1904.8124 ✅** | 2519.5944 | +460.4463 | Higher | -154.3356 | Lower | -614.7819 | Lower |
| Halstead Effort | 121145.5973 | **107529.7331 ✅** | 151870.7218 | +13615.8642 | Higher | -30725.1245 | Lower | -44340.9888 | Lower |
| SLOC | 74 | 67 | **65 ✅** | +7 | Higher | +9 | Higher | +2 | Higher |
| PLOC | 56 | **53 ✅** | 54 | +3 | Higher | +2 | Higher | -1 | Lower |
| LLOC | 46 | **35 ✅** | 44 | +11 | Higher | +2 | Higher | -9 | Lower |
| Comments (CLOC) | 9 | **14 ✅** | 2 | -5 | Lower | +7 | Higher | +12 | Higher |
| Blank Lines | 11 | **7 ✅** | 11 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03865

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.4446 | 85.2261 | **92.6308 ✅** | +3.2185 | Higher | -4.1862 | Lower | -7.4047 | Lower |
| MI (SEI) | 85.3612 | 83.7647 | **94.7820 ✅** | +1.5965 | Higher | -9.4208 | Lower | -11.0172 | Lower |
| Cyclomatic (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 4 | 5 | **3 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 376.4723 | 456.5062 | **361.2099 ✅** | -80.0339 | Lower | +15.2624 | Higher | +95.2963 | Higher |
| Halstead Effort | 5270.6115 | **4964.5048 ✅** | 5056.9381 | +306.1067 | Higher | +213.6734 | Higher | -92.4333 | Lower |
| SLOC | 23 | 26 | **18 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| PLOC | 14 | 17 | **12 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| LLOC | 8 | 10 | **5 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Comments (CLOC) | 5 | **7 ✅** | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p04005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.9768 | **77.4090 ✅** | 74.9200 | -1.4323 | Lower | +1.0568 | Higher | +2.4891 | Higher |
| MI (SEI) | 63.9333 | **73.0253 ✅** | 62.4086 | -9.0920 | Lower | +1.5246 | Higher | +10.6166 | Higher |
| Cyclomatic (sum) | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 8 | **3 ✅** | 9 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | 788.7516 | **712.5395 ✅** | 966.4985 | +76.2121 | Higher | -177.7468 | Lower | -253.9590 | Lower |
| Halstead Effort | 16311.3834 | **11671.3969 ✅** | 29342.8933 | +4639.9866 | Higher | -13031.5098 | Lower | -17671.4964 | Lower |
| SLOC | 37 | **36 ✅** | 37 | +1 | Higher | 0 | Same | -1 | Lower |
| PLOC | 26 | **22 ✅** | 26 | +4 | Higher | 0 | Same | -4 | Lower |
| LLOC | 17 | **12 ✅** | 22 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.2389 ✅** | 75.9691 | 75.2873 | 0.2698 | Higher | 0.9516 | Higher | 0.6818 | Higher |
| MI (SEI) | 62.1678 | **68.5071 ✅** | 56.8520 | -6.3394 | Lower | 5.3158 | Higher | 11.6551 | Higher |
| Cyclomatic (sum) | 8.3673 | **7.9184 ✅** | 9.1429 | 0.4490 | Higher | -0.7755 | Lower | -1.2245 | Lower |
| Cognitive (sum) | 8.4490 | **8 ✅** | 8.7755 | 0.4490 | Higher | -0.3265 | Lower | -0.7755 | Lower |
| Halstead Volume | 943.6409 | **877.6426 ✅** | 1143.7994 | 65.9983 | Higher | -200.1586 | Lower | -266.1569 | Lower |
| Halstead Effort | 28508.0878 | **25090.1926 ✅** | 48510.6867 | 3417.8952 | Higher | -20002.5989 | Lower | -23420.4941 | Lower |
| SLOC | **38.7755 ✅** | 40.3878 | 42.3673 | -1.6122 | Lower | -3.5918 | Lower | -1.9796 | Lower |
| PLOC | 28.0204 | **27.3673 ✅** | 32.7551 | 0.6531 | Higher | -4.7347 | Lower | -5.3878 | Lower |
| LLOC | 18.2857 | **16.5918 ✅** | 23.0612 | 1.6939 | Higher | -4.7755 | Lower | -6.4694 | Lower |
| Comments (CLOC) | 5.5714 | **9.1837 ✅** | 4.1633 | -3.6122 | Lower | 1.4082 | Higher | 5.0204 | Higher |
| Blank Lines | 6.5714 | **6.2857 ✅** | 6.8980 | 0.2857 | Higher | -0.3265 | Lower | -0.6122 | Lower |
| Functions | **1.3265 ✅** | 1.6327 | 1.9388 | -0.3061 | Lower | -0.6122 | Lower | -0.3061 | Lower |
| Closures | 0.4490 | 0.3673 | **0.3265 ✅** | 0.0816 | Higher | 0.1224 | Higher | 0.0408 | Higher |
| Exits | 1.0816 | 1.2245 | **1.0204 ✅** | -0.1429 | Lower | 0.0612 | Higher | 0.2041 | Higher |

