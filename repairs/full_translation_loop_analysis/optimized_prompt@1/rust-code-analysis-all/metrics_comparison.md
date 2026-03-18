# Rust Metrics Comparison

Source: `/Users/lithe/Documents/research/code_translation_specs/codenet_statistics copy/r_code/full_translation_loop_all_metrics.json`

## p00004

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.7925 | **79.6209 ✅** | 66.4530 | -3.8284 | Lower | +9.3395 | Higher | +13.1679 | Higher |
| MI (SEI) | 59.6931 | **76.3900 ✅** | 48.9471 | -16.6969 | Lower | +10.7460 | Higher | +27.4429 | Higher |
| Cyclomatic (sum) | 7 | **6 ✅** | 16 | +1 | Higher | -9 | Lower | -10 | Lower |
| Cognitive (sum) | **3 ✅** | 5 | 13 | -2 | Lower | -10 | Lower | -8 | Lower |
| Halstead Volume | 1111.5857 | **672.1053 ✅** | 1272.0008 | +439.4805 | Higher | -160.4151 | Lower | -599.8956 | Lower |
| Halstead Effort | 29974.4846 | **14727.8716 ✅** | 37688.9140 | +15246.6130 | Higher | -7714.4294 | Lower | -22961.0424 | Lower |
| SLOC | 34 | **32 ✅** | 51 | +2 | Higher | -17 | Lower | -19 | Lower |
| PLOC | 26 | **21 ✅** | 41 | +5 | Higher | -15 | Lower | -20 | Lower |
| LLOC | 18 | **12 ✅** | 18 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 4 | **9 ✅** | 7 | -5 | Lower | -3 | Lower | +2 | Higher |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

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

## p00015

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.0239 | **71.1359 ✅** | 60.0698 | -10.1120 | Lower | +0.9541 | Higher | +11.0661 | Higher |
| MI (SEI) | 34.3416 | **59.3844 ✅** | 25.7126 | -25.0429 | Lower | +8.6289 | Higher | +33.6718 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 15 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 18 | **16 ✅** | 25 | +2 | Higher | -7 | Lower | -9 | Lower |
| Halstead Volume | 1848 | **1015.4543 ✅** | 1939.2024 | +832.5457 | Higher | -91.2024 | Lower | -923.7482 | Lower |
| Halstead Effort | 74741.3333 | **22766.4850 ✅** | 93912.8037 | +51974.8484 | Higher | -19171.4703 | Lower | -71146.3187 | Lower |
| SLOC | 66 | **46 ✅** | 67 | +20 | Higher | -1 | Lower | -21 | Lower |
| PLOC | 50 | **31 ✅** | 54 | +19 | Higher | -4 | Lower | -23 | Lower |
| LLOC | 38 | **19 ✅** | 44 | +19 | Higher | -6 | Lower | -25 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 2 | -4 | Lower | +3 | Higher | +7 | Higher |
| Blank Lines | 12 | **8 ✅** | 13 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
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

## p00024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.6188 | 78.8671 | **89.4368 ✅** | +6.7517 | Higher | -3.8180 | Lower | -10.5697 | Lower |
| MI (SEI) | 77.9882 | 81.2156 | **85.6719 ✅** | -3.2274 | Lower | -7.6837 | Lower | -4.4563 | Lower |
| Cyclomatic (sum) | 6 | 12 | **4 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Cognitive (sum) | 6 | 10 | **4 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Halstead Volume | 519.6825 | 493.3052 | **480.8030 ✅** | +26.3774 | Higher | +38.8795 | Higher | +12.5022 | Higher |
| Halstead Effort | 8413.9079 | **6637.1971 ✅** | 7692.8484 | +1776.7108 | Higher | +721.0595 | Higher | -1055.6514 | Lower |
| SLOC | 24 | 34 | **20 ✅** | -10 | Lower | +4 | Higher | +14 | Higher |
| PLOC | 19 | 18 | **15 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| LLOC | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 4 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

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

## p00071

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.8480 | 58.9943 | **66.3525 ✅** | +2.8537 | Higher | -4.5045 | Lower | -7.3582 | Lower |
| MI (SEI) | 39.8419 | **46.0005 ✅** | 32.6710 | -6.1586 | Lower | +7.1709 | Higher | +13.3295 | Higher |
| Cyclomatic (sum) | 18 | 14 | **13 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Cognitive (sum) | 18 | 15 | 15 | +3 | Higher | +3 | Higher | 0 | Same |
| Halstead Volume | 2231.0590 | 2080.6686 | **2042.2893 ✅** | +150.3904 | Higher | +188.7697 | Higher | +38.3793 | Higher |
| Halstead Effort | 93704.4788 | 87075.9806 | **73522.4160 ✅** | +6628.4982 | Higher | +20182.0627 | Higher | +13553.5645 | Higher |
| SLOC | 55 | 71 | **46 ✅** | -16 | Lower | +9 | Higher | +25 | Higher |
| PLOC | 44 | 45 | **38 ✅** | -1 | Lower | +6 | Higher | +7 | Higher |
| LLOC | 32 | 31 | **26 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| Comments (CLOC) | 6 | **18 ✅** | 1 | -12 | Lower | +5 | Higher | +17 | Higher |
| Blank Lines | 9 | 14 | **7 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.0214 | 60.7781 | **64.3927 ✅** | +0.2433 | Higher | -3.3712 | Lower | -3.6146 | Lower |
| MI (SEI) | 47.8272 | 51.2751 | **52.1968 ✅** | -3.4479 | Lower | -4.3696 | Lower | -0.9217 | Lower |
| Cyclomatic (sum) | 21 | 23 | **15 ✅** | -2 | Lower | +6 | Higher | +8 | Higher |
| Cognitive (sum) | 9 | 7 | 7 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 1500.2973 | 1765.3642 | **1323.5947 ✅** | -265.0670 | Lower | +176.7026 | Higher | +441.7695 | Higher |
| Halstead Effort | 61375.7970 | 73866.5557 | **44671.3207 ✅** | -12490.7587 | Lower | +16704.4764 | Higher | +29195.2350 | Higher |
| SLOC | 63 | 59 | **58 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| PLOC | 42 | 41 | **40 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| LLOC | 24 | 22 | 22 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 14 | **17 ✅** | 13 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 11 | 10 | 10 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

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

## p00080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.3594 | **84.9961 ✅** | 79.5379 | -7.6367 | Lower | -2.1784 | Lower | +5.4582 | Higher |
| MI (SEI) | 55.0676 | **85.4040 ✅** | 39.8626 | -30.3365 | Lower | +15.2050 | Higher | +45.5414 | Higher |
| Cyclomatic (sum) | 8 | **6 ✅** | 8 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 7 | 10 | 7 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 786.8084 | **456.5062 ✅** | 764.3204 | +330.3022 | Higher | +22.4880 | Higher | -307.8142 | Lower |
| Halstead Effort | 19527.1537 | **7008.7127 ✅** | 18725.8505 | +12518.4410 | Higher | +801.3032 | Higher | -11717.1378 | Lower |
| SLOC | 34 | **26 ✅** | 30 | +8 | Higher | +4 | Higher | -4 | Lower |
| PLOC | 28 | **17 ✅** | 26 | +11 | Higher | +2 | Higher | -9 | Lower |
| LLOC | 18 | **10 ✅** | 17 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 2 | **8 ✅** | 0 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 6 | **2 ✅** | 4 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.0743 | **66.6648 ✅** | 54.4345 | -6.5905 | Lower | +5.6398 | Higher | +12.2303 | Higher |
| MI (SEI) | 30.1460 | **57.4242 ✅** | 16.0618 | -27.2782 | Lower | +14.0843 | Higher | +41.3625 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 15 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 24 | **10 ✅** | 27 | +14 | Higher | -3 | Lower | -17 | Lower |
| Halstead Volume | 1691.5702 | **1189.9096 ✅** | 2633.2474 | +501.6607 | Higher | -941.6772 | Lower | -1443.3378 | Lower |
| Halstead Effort | 45383.5912 | **25355.5729 ✅** | 113634.7522 | +20028.0183 | Higher | -68251.1610 | Lower | -88279.1793 | Lower |
| SLOC | 72 | **56 ✅** | 86 | +16 | Higher | -14 | Lower | -30 | Lower |
| PLOC | 61 | **35 ✅** | 75 | +26 | Higher | -14 | Lower | -40 | Lower |
| LLOC | 41 | **19 ✅** | 61 | +22 | Higher | -20 | Lower | -42 | Lower |
| Comments (CLOC) | 4 | **15 ✅** | 2 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | **9 ✅** | 10 | 11 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.0923 ✅** | 62.8817 | 59.4955 | +0.2106 | Higher | +3.5968 | Higher | +3.3862 | Higher |
| MI (SEI) | 37.6262 | **44.8795 ✅** | 31.4834 | -7.2533 | Lower | +6.1428 | Higher | +13.3961 | Higher |
| Cyclomatic (sum) | **13 ✅** | 16 | 16 | -3 | Lower | -3 | Lower | 0 | Same |
| Cognitive (sum) | 29 | **16 ✅** | 38 | +13 | Higher | -9 | Lower | -22 | Lower |
| Halstead Volume | **1366.4289 ✅** | 1523.6045 | 1586.1294 | -157.1757 | Lower | -219.7005 | Lower | -62.5248 | Lower |
| Halstead Effort | **48560.7794 ✅** | 75064.7313 | 64766.9485 | -26503.9519 | Lower | -16206.1691 | Lower | +10297.7828 | Higher |
| SLOC | 64 | **60 ✅** | 73 | +4 | Higher | -9 | Lower | -13 | Lower |
| PLOC | 48 | **44 ✅** | 55 | +4 | Higher | -7 | Lower | -11 | Lower |
| LLOC | 39 | **29 ✅** | 46 | +10 | Higher | -7 | Lower | -17 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 5 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 11 | **10 ✅** | 13 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00118

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.5724 | **58.8704 ✅** | 50.5740 | -10.2980 | Lower | -2.0016 | Lower | +8.2964 | Higher |
| MI (SEI) | 13.1165 | **40.3271 ✅** | 0.0111 | -27.2106 | Lower | +13.1054 | Higher | +40.3160 | Higher |
| Cyclomatic (sum) | 29 | **17 ✅** | 27 | +12 | Higher | +2 | Higher | -10 | Lower |
| Cognitive (sum) | 33 | **16 ✅** | 33 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 3429.9130 | **2235.5027 ✅** | 3374.6603 | +1194.4103 | Higher | +55.2526 | Higher | -1139.1576 | Lower |
| Halstead Effort | 390152.5982 | **114010.6373 ✅** | 373399.9143 | +276141.9609 | Higher | +16752.6839 | Higher | -259389.2770 | Lower |
| SLOC | 93 | **67 ✅** | 85 | +26 | Higher | +8 | Higher | -18 | Lower |
| PLOC | 74 | **49 ✅** | 71 | +25 | Higher | +3 | Higher | -22 | Lower |
| LLOC | 53 | **33 ✅** | 54 | +20 | Higher | -1 | Lower | -21 | Lower |
| Comments (CLOC) | 4 | **11 ✅** | 0 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | 15 | **12 ✅** | 14 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00124

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.2740 | 62.6087 | **66.0044 ✅** | +2.6653 | Higher | -0.7304 | Lower | -3.3957 | Lower |
| MI (SEI) | **45.6268 ✅** | 44.1012 | 38.2401 | +1.5257 | Higher | +7.3867 | Higher | +5.8610 | Higher |
| Cyclomatic (sum) | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 7 | 7 | 10 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | 1487.5248 | 1488.5491 | **1429.0581 ✅** | -1.0242 | Lower | +58.4668 | Higher | +59.4910 | Higher |
| Halstead Effort | 42277.0220 | **37605.4504 ✅** | 38668.6297 | +4671.5717 | Higher | +3608.3924 | Higher | -1063.1793 | Lower |
| SLOC | 56 | 66 | **55 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| PLOC | 44 | 50 | 44 | -6 | Lower | 0 | Same | +6 | Higher |
| LLOC | 28 | **26 ✅** | 29 | +2 | Higher | -1 | Lower | -3 | Lower |
| Comments (CLOC) | 7 | **10 ✅** | 3 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00136

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **71.5188 ✅** | 67.6790 | 58.0344 | +3.8398 | Higher | +13.4844 | Higher | +9.6446 | Higher |
| MI (SEI) | 64.9997 | **65.2319 ✅** | 27.5818 | -0.2322 | Lower | +37.4179 | Higher | +37.6501 | Higher |
| Cyclomatic (sum) | **10 ✅** | 16 | 19 | -6 | Lower | -9 | Lower | -3 | Lower |
| Cognitive (sum) | 11 | **6 ✅** | 15 | +5 | Higher | -4 | Lower | -9 | Lower |
| Halstead Volume | **1065.3932 ✅** | 1296 | 1763.3147 | -230.6068 | Lower | -697.9215 | Lower | -467.3147 | Lower |
| Halstead Effort | **20837.8377 ✅** | 27216 | 54907.6609 | -6378.1623 | Lower | -34069.8232 | Lower | -27691.6609 | Lower |
| SLOC | **43 ✅** | 47 | 74 | -4 | Lower | -31 | Lower | -27 | Lower |
| PLOC | **32 ✅** | 35 | 60 | -3 | Lower | -28 | Lower | -25 | Lower |
| LLOC | 26 | **21 ✅** | 50 | +5 | Higher | -24 | Lower | -29 | Lower |
| Comments (CLOC) | 12 | **19 ✅** | 4 | -7 | Lower | +8 | Higher | +15 | Higher |
| Blank Lines | 6 | 6 | 10 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00137

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 80.1427 | 80.5391 | **80.6306 ✅** | -0.3964 | Lower | -0.4879 | Lower | -0.0915 | Lower |
| MI (SEI) | 71.9298 | **79.4330 ✅** | 70.5882 | -7.5033 | Lower | +1.3416 | Higher | +8.8448 | Higher |
| Cyclomatic (sum) | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 4 | 3 | 3 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 701.4924 | **679.3981 ✅** | 739.3422 | +22.0942 | Higher | -37.8498 | Lower | -59.9441 | Lower |
| Halstead Effort | 13552.8326 | **9827.0085 ✅** | 16899.2498 | +3725.8240 | Higher | -3346.4172 | Lower | -7072.2412 | Lower |
| SLOC | 31 | 31 | **30 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| PLOC | 21 | **16 ✅** | 22 | +5 | Higher | -1 | Lower | -6 | Lower |
| LLOC | 16 | **11 ✅** | 21 | +5 | Higher | -5 | Lower | -10 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 5 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 7 | 6 | **5 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00146

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.6909 | **56.7594 ✅** | 48.2877 | -3.0685 | Lower | +5.4033 | Higher | +8.4718 | Higher |
| MI (SEI) | 3.6934 | **41.9387 ✅** | -3.7964 | -38.2453 | Lower | +7.4898 | Higher | +45.7351 | Higher |
| Cyclomatic (sum) | 19 | **15 ✅** | 22 | +4 | Higher | -3 | Lower | -7 | Lower |
| Cognitive (sum) | 28 | **27 ✅** | 31 | +1 | Higher | -3 | Lower | -4 | Lower |
| Halstead Volume | 3188.6498 | **2474.9644 ✅** | 4621.0258 | +713.6854 | Higher | -1432.3760 | Lower | -2146.0614 | Lower |
| Halstead Effort | 264028.5918 | **146594.0446 ✅** | 211856.2585 | +117434.5472 | Higher | +52172.3334 | Higher | -65262.2139 | Lower |
| SLOC | 80 | **76 ✅** | 95 | +4 | Higher | -15 | Lower | -19 | Lower |
| PLOC | 70 | **60 ✅** | 82 | +10 | Higher | -12 | Lower | -22 | Lower |
| LLOC | 58 | **51 ✅** | 64 | +7 | Higher | -6 | Lower | -13 | Lower |
| Comments (CLOC) | 0 | **18 ✅** | 0 | -18 | Lower | 0 | Same | +18 | Higher |
| Blank Lines | 10 | **9 ✅** | 13 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p00156

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.9622 | **56.4284 ✅** | 46.5663 | -8.4662 | Lower | +1.3959 | Higher | +9.8621 | Higher |
| MI (SEI) | 11.3133 | **39.4075 ✅** | 7.7907 | -28.0942 | Lower | +3.5227 | Higher | +31.6169 | Higher |
| Cyclomatic (sum) | 26 | **21 ✅** | 30 | +5 | Higher | -4 | Lower | -9 | Lower |
| Cognitive (sum) | 62 | **33 ✅** | 62 | +29 | Higher | 0 | Same | -29 | Lower |
| Halstead Volume | 3405.7986 | **2393.9309 ✅** | 3849.7417 | +1011.8677 | Higher | -443.9431 | Lower | -1455.8108 | Lower |
| Halstead Effort | 236238.5757 | **88627.4853 ✅** | 344231.0701 | +147611.0904 | Higher | -107992.4944 | Lower | -255603.5848 | Lower |
| SLOC | 101 | **72 ✅** | 100 | +29 | Higher | +1 | Higher | -28 | Lower |
| PLOC | 83 | **53 ✅** | 84 | +30 | Higher | -1 | Lower | -31 | Lower |
| LLOC | 70 | **36 ✅** | 82 | +34 | Higher | -12 | Lower | -46 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 3 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | 17 | **12 ✅** | 15 | +5 | Higher | +2 | Higher | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00167

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.7175 | **71.4548 ✅** | 66.2306 | -18.7372 | Lower | -13.5130 | Lower | +5.2242 | Higher |
| MI (SEI) | 24.9319 | **57.4734 ✅** | 40.1455 | -32.5415 | Lower | -15.2136 | Lower | +17.3279 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 12 | +5 | Higher | +4 | Higher | -1 | Lower |
| Cognitive (sum) | 24 | **12 ✅** | 18 | +12 | Higher | +6 | Higher | -6 | Lower |
| Halstead Volume | 2570.5059 | **960.6136 ✅** | 2043.6783 | +1609.8924 | Higher | +526.8276 | Higher | -1083.0648 | Lower |
| Halstead Effort | 83521.0410 | **36434.6997 ✅** | 45165.2912 | +47086.3413 | Higher | +38355.7497 | Higher | -8730.5916 | Lower |
| SLOC | 95 | **44 ✅** | 47 | +51 | Higher | +48 | Higher | -3 | Lower |
| PLOC | 72 | **33 ✅** | 39 | +39 | Higher | +33 | Higher | -6 | Lower |
| LLOC | 38 | **20 ✅** | 21 | +18 | Higher | +17 | Higher | -1 | Lower |
| Comments (CLOC) | **9 ✅** | 7 | 3 | +2 | Higher | +6 | Higher | +4 | Higher |
| Blank Lines | 19 | 7 | **6 ✅** | +12 | Higher | +13 | Higher | +1 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00187

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.2502 | **68.4545 ✅** | 51.9331 | -19.2043 | Lower | -2.6829 | Lower | +16.5214 | Higher |
| MI (SEI) | 5.5899 | **50.3172 ✅** | 13.4128 | -44.7272 | Lower | -7.8229 | Lower | +36.9044 | Higher |
| Cyclomatic (sum) | 22 | **12 ✅** | 21 | +10 | Higher | +1 | Higher | -9 | Lower |
| Cognitive (sum) | 33 | **11 ✅** | 32 | +22 | Higher | +1 | Higher | -21 | Lower |
| Halstead Volume | 4103.3205 | **1247.9299 ✅** | 3937.2158 | +2855.3906 | Higher | +166.1047 | Higher | -2689.2859 | Lower |
| Halstead Effort | 238257.3176 | **28296.0845 ✅** | 245124.4910 | +209961.2331 | Higher | -6867.1734 | Lower | -216828.4065 | Lower |
| SLOC | 93 | **48 ✅** | 81 | +45 | Higher | +12 | Higher | -33 | Lower |
| PLOC | 81 | **35 ✅** | 70 | +46 | Higher | +11 | Higher | -35 | Lower |
| LLOC | 38 | **14 ✅** | 57 | +24 | Higher | -19 | Lower | -43 | Lower |
| Comments (CLOC) | 1 | **6 ✅** | 2 | -5 | Lower | -1 | Lower | +4 | Higher |
| Blank Lines | 12 | **7 ✅** | 11 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p00190

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 33.7487 | **54.1150 ✅** | 32.0768 | -20.3663 | Lower | +1.6719 | Higher | +22.0383 | Higher |
| MI (SEI) | -17.7408 | **32.1439 ✅** | -18.0277 | -49.8847 | Lower | +0.2869 | Higher | +50.1716 | Higher |
| Cyclomatic (sum) | 34 | **16 ✅** | 35 | +18 | Higher | -1 | Lower | -19 | Lower |
| Cognitive (sum) | 65 | **16 ✅** | 62 | +49 | Higher | +3 | Higher | -46 | Lower |
| Halstead Volume | 6405.6675 | **2030.5764 ✅** | 6352.0169 | +4375.0911 | Higher | +53.6507 | Higher | -4321.4404 | Lower |
| Halstead Effort | 565441.4588 | **52482.5911 ✅** | 658299.9284 | +512958.8677 | Higher | -92858.4696 | Lower | -605817.3373 | Lower |
| SLOC | 177 | **94 ✅** | 194 | +83 | Higher | -17 | Lower | -100 | Lower |
| PLOC | 159 | **65 ✅** | 178 | +94 | Higher | -19 | Lower | -113 | Lower |
| LLOC | 93 | **27 ✅** | 128 | +66 | Higher | -35 | Lower | -101 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 2 | -13 | Lower | -1 | Lower | +12 | Higher |
| Blank Lines | 18 | **15 ✅** | 16 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |

## p00203

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.1618 | **64.9133 ✅** | 49.8366 | -15.7515 | Lower | -0.6749 | Lower | +15.0767 | Higher |
| MI (SEI) | 6.0732 | **51.1066 ✅** | -0.9508 | -45.0333 | Lower | +7.0241 | Higher | +52.0574 | Higher |
| Cyclomatic (sum) | 28 | **12 ✅** | 28 | +16 | Higher | 0 | Same | -16 | Lower |
| Cognitive (sum) | 39 | **6 ✅** | 44 | +33 | Higher | -5 | Lower | -38 | Lower |
| Halstead Volume | 3200.8668 | **1230.3606 ✅** | 3223.9306 | +1970.5062 | Higher | -23.0638 | Lower | -1993.5699 | Lower |
| Halstead Effort | 253301.0280 | **36910.8195 ✅** | 266941.4534 | +216390.2085 | Higher | -13640.4254 | Lower | -230030.6339 | Lower |
| SLOC | 93 | **60 ✅** | 89 | +33 | Higher | +4 | Higher | -29 | Lower |
| PLOC | 77 | **41 ✅** | 80 | +36 | Higher | -3 | Lower | -39 | Lower |
| LLOC | 49 | **23 ✅** | 52 | +26 | Higher | -3 | Lower | -29 | Lower |
| Comments (CLOC) | 1 | **12 ✅** | 0 | -11 | Lower | +1 | Higher | +12 | Higher |
| Blank Lines | 16 | 11 | **9 ✅** | +5 | Higher | +7 | Higher | +2 | Higher |
| Functions | 5 | **1 ✅** | 6 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | 4 | 4 | **1 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Exits | 3 | **0 ✅** | 3 | +3 | Higher | 0 | Same | -3 | Lower |

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

## p00244

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.6536 | **49.0133 ✅** | 29.7619 | -6.3597 | Lower | +12.8918 | Higher | +19.2514 | Higher |
| MI (SEI) | -3.9448 | **18.5807 ✅** | -20.6122 | -22.5256 | Lower | +16.6674 | Higher | +39.1930 | Higher |
| Cyclomatic (sum) | 32 | **26 ✅** | 43 | +6 | Higher | -11 | Lower | -17 | Lower |
| Cognitive (sum) | 33 | **22 ✅** | 47 | +11 | Higher | -14 | Lower | -25 | Lower |
| Halstead Volume | 3923.7526 | **2617.6090 ✅** | 6634.5540 | +1306.1436 | Higher | -2710.8014 | Lower | -4016.9450 | Lower |
| Halstead Effort | 203054.1965 | **115050.1489 ✅** | 582717.9790 | +88004.0476 | Higher | -379663.7825 | Lower | -467667.8301 | Lower |
| SLOC | 123 | **103 ✅** | 197 | +20 | Higher | -74 | Lower | -94 | Lower |
| PLOC | 101 | **80 ✅** | 168 | +21 | Higher | -67 | Lower | -88 | Lower |
| LLOC | 51 | **26 ✅** | 108 | +25 | Higher | -57 | Lower | -82 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 2 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 22 | **18 ✅** | 27 | +4 | Higher | -5 | Lower | -9 | Lower |
| Functions | **5 ✅** | 7 | 10 | -2 | Lower | -5 | Lower | -3 | Lower |
| Closures | **0 ✅** | 1 | 3 | -1 | Lower | -3 | Lower | -2 | Lower |
| Exits | 6 | 6 | 10 | 0 | Same | -4 | Lower | -4 | Lower |

## p00252

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.3117 | **83.2001 ✅** | 79.1129 | -13.8883 | Lower | -9.8012 | Lower | +4.0872 | Higher |
| MI (SEI) | 56.2086 | **72.7242 ✅** | 65.2800 | -16.5156 | Lower | -9.0714 | Lower | +7.4442 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 17 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | **5 ✅** | 7 | 6 | -2 | Lower | -1 | Lower | +1 | Higher |
| Halstead Volume | 1133.1465 | **796.1685 ✅** | 869.9561 | +336.9781 | Higher | +263.1904 | Higher | -73.7877 | Lower |
| Halstead Effort | 31581.8897 | 23864.1017 | **20604.2245 ✅** | +7717.7880 | Higher | +10977.6651 | Higher | +3259.8772 | Higher |
| SLOC | 45 | **22 ✅** | 26 | +23 | Higher | +19 | Higher | -4 | Lower |
| PLOC | 30 | **17 ✅** | 20 | +13 | Higher | +10 | Higher | -3 | Lower |
| LLOC | 12 | **8 ✅** | 12 | +4 | Higher | 0 | Same | -4 | Lower |
| Comments (CLOC) | **8 ✅** | 3 | 3 | +5 | Higher | +5 | Higher | 0 | Same |
| Blank Lines | 8 | **2 ✅** | 3 | +6 | Higher | +5 | Higher | -1 | Lower |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p00256

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **50.1366 ✅** | 47.8623 | 47.2872 | +2.2743 | Higher | +2.8495 | Higher | +0.5751 | Higher |
| MI (SEI) | 14.3973 | **30.2262 ✅** | 8.1555 | -15.8289 | Lower | +6.2417 | Higher | +22.0707 | Higher |
| Cyclomatic (sum) | **21 ✅** | 41 | 24 | -20 | Lower | -3 | Lower | +17 | Higher |
| Cognitive (sum) | 44 | **21 ✅** | 50 | +23 | Higher | -6 | Lower | -29 | Lower |
| Halstead Volume | 3384.7422 | **3060.3982 ✅** | 4236.6074 | +324.3440 | Higher | -851.8652 | Lower | -1176.2092 | Lower |
| Halstead Effort | 219506.7976 | **125631.9380 ✅** | 356834.2542 | +93874.8596 | Higher | -137327.4566 | Lower | -231202.3162 | Lower |
| SLOC | 95 | **85 ✅** | 101 | +10 | Higher | -6 | Lower | -16 | Lower |
| PLOC | 83 | **61 ✅** | 92 | +22 | Higher | -9 | Lower | -31 | Lower |
| LLOC | 72 | **28 ✅** | 110 | +44 | Higher | -38 | Lower | -82 | Lower |
| Comments (CLOC) | 4 | **18 ✅** | 3 | -14 | Lower | +1 | Higher | +15 | Higher |
| Blank Lines | 10 | 13 | **9 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 2 | 5 | **1 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Closures | 1 | 8 | **0 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Exits | 1 | 17 | **0 ✅** | -16 | Lower | +1 | Higher | +17 | Higher |

## p00261

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.9858 | 62.4136 | **70.3395 ✅** | +5.5723 | Higher | -2.3537 | Lower | -7.9259 | Lower |
| MI (SEI) | 23.2983 | **50.8969 ✅** | 26.4904 | -27.5986 | Lower | -3.1920 | Lower | +24.4066 | Higher |
| Cyclomatic (sum) | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 17 | 18 | **11 ✅** | -1 | Lower | +6 | Higher | +7 | Higher |
| Halstead Volume | 1145.2226 | 2158.0828 | **1083.4332 ✅** | -1012.8602 | Lower | +61.7894 | Higher | +1074.6496 | Higher |
| Halstead Effort | **18082.4623 ✅** | 86862.8332 | 22836.7413 | -68780.3709 | Lower | -4754.2790 | Lower | +64026.0920 | Higher |
| SLOC | 53 | 61 | **48 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| PLOC | 46 | 45 | **41 ✅** | +1 | Higher | +5 | Higher | +4 | Higher |
| LLOC | 23 | 24 | **19 ✅** | -1 | Lower | +4 | Higher | +5 | Higher |
| Comments (CLOC) | 0 | **16 ✅** | 0 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00270

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.3208 | **53.5321 ✅** | 24.2869 | -14.2113 | Lower | +15.0338 | Higher | +29.2452 | Higher |
| MI (SEI) | -9.1400 | **26.5664 ✅** | -28.8868 | -35.7064 | Lower | +19.7468 | Higher | +55.4532 | Higher |
| Cyclomatic (sum) | 32 | **16 ✅** | 46 | +16 | Higher | -14 | Lower | -30 | Lower |
| Cognitive (sum) | 51 | **18 ✅** | 57 | +33 | Higher | -6 | Lower | -39 | Lower |
| Halstead Volume | 5204.3435 | **2512.9814 ✅** | 9361.1441 | +2691.3621 | Higher | -4156.8005 | Lower | -6848.1627 | Lower |
| Halstead Effort | 350838.8421 | **83070.6305 ✅** | 912934.4301 | +267768.2115 | Higher | -562095.5880 | Lower | -829863.7995 | Lower |
| SLOC | 138 | **91 ✅** | 237 | +47 | Higher | -99 | Lower | -146 | Lower |
| PLOC | 119 | **76 ✅** | 208 | +43 | Higher | -89 | Lower | -132 | Lower |
| LLOC | 84 | **36 ✅** | 169 | +48 | Higher | -85 | Lower | -133 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 2 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 19 | **15 ✅** | 27 | +4 | Higher | -8 | Lower | -12 | Lower |
| Functions | **3 ✅** | 5 | 11 | -2 | Lower | -8 | Lower | -6 | Lower |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | **2 ✅** | 4 | 5 | -2 | Lower | -3 | Lower | -1 | Lower |

## p00271

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 84.0301 | 79.6142 | **89.2115 ✅** | +4.4160 | Higher | -5.1814 | Lower | -9.5973 | Lower |
| MI (SEI) | 74.4830 | 75.9503 | **85.2451 ✅** | -1.4673 | Lower | -10.7621 | Lower | -9.2948 | Lower |
| Cyclomatic (sum) | 4 | 6 | **3 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 1 | 7 | 1 | -6 | Lower | 0 | Same | +6 | Higher |
| Halstead Volume | 600.5375 | 611.4563 | **524.7928 ✅** | -10.9189 | Lower | +75.7447 | Higher | +86.6636 | Higher |
| Halstead Effort | 8107.2560 | 9758.8432 | **5204.1948 ✅** | -1651.5873 | Lower | +2903.0612 | Higher | +4554.6484 | Higher |
| SLOC | 26 | 33 | **20 ✅** | -7 | Lower | +6 | Higher | +13 | Higher |
| PLOC | 17 | 19 | **13 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| LLOC | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Comments (CLOC) | 4 | **9 ✅** | 4 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00283

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **60.2010 ✅** | 56.1445 | 58.8417 | +4.0564 | Higher | +1.3593 | Higher | -2.6971 | Lower |
| MI (SEI) | 12.5762 | **30.4890 ✅** | 27.1544 | -17.9127 | Lower | -14.5781 | Lower | +3.3346 | Higher |
| Cyclomatic (sum) | **14 ✅** | 15 | 16 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 21 | **13 ✅** | 20 | +8 | Higher | +1 | Higher | -7 | Lower |
| Halstead Volume | **2394.2357 ✅** | 2469.1139 | 2582.1849 | -74.8782 | Lower | -187.9492 | Lower | -113.0710 | Lower |
| Halstead Effort | 131102.5439 | **91004.4836 ✅** | 149766.7258 | +40098.0602 | Higher | -18664.1819 | Lower | -58762.2421 | Lower |
| SLOC | **63 ✅** | 79 | 65 | -16 | Lower | -2 | Lower | +14 | Higher |
| PLOC | **53 ✅** | 62 | 57 | -9 | Lower | -4 | Lower | +5 | Higher |
| LLOC | 42 | **32 ✅** | 46 | +10 | Higher | -4 | Lower | -14 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 3 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 10 | 13 | **8 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |

## p00288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 19.4220 | **47.0659 ✅** | 18.1071 | -27.6439 | Lower | +1.3149 | Higher | +28.9588 | Higher |
| MI (SEI) | -42.3862 | **16.3824 ✅** | -37.4277 | -58.7686 | Lower | -4.9585 | Lower | +53.8101 | Higher |
| Cyclomatic (sum) | 52 | **20 ✅** | 56 | +32 | Higher | -4 | Lower | -36 | Lower |
| Cognitive (sum) | 68 | **21 ✅** | 88 | +47 | Higher | -20 | Lower | -67 | Lower |
| Halstead Volume | 10645.6126 | **3618.3701 ✅** | 10873.5893 | +7027.2424 | Higher | -227.9767 | Lower | -7255.2191 | Lower |
| Halstead Effort | 1130637.4709 | **193513.2190 ✅** | 1240322.2276 | +937124.2519 | Higher | -109684.7567 | Lower | -1046809.0086 | Lower |
| SLOC | 282 | **114 ✅** | 287 | +168 | Higher | -5 | Lower | -173 | Lower |
| PLOC | 245 | **92 ✅** | 254 | +153 | Higher | -9 | Lower | -162 | Lower |
| LLOC | 153 | **66 ✅** | 174 | +87 | Higher | -21 | Lower | -108 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 2 | -10 | Lower | -2 | Lower | +8 | Higher |
| Blank Lines | 37 | **21 ✅** | 31 | +16 | Higher | +6 | Higher | -10 | Lower |
| Functions | 13 | **5 ✅** | 14 | +8 | Higher | -1 | Lower | -9 | Lower |
| Closures | 4 | 3 | **0 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Exits | 8 | **5 ✅** | 10 | +3 | Higher | -2 | Lower | -5 | Lower |

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

## p00315

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.0431 | **61.3526 ✅** | 52.4064 | -9.3094 | Lower | -0.3632 | Lower | +8.9462 | Higher |
| MI (SEI) | 29.5421 | **43.8873 ✅** | 20.1898 | -14.3452 | Lower | +9.3523 | Higher | +23.6976 | Higher |
| Cyclomatic (sum) | 18 | **13 ✅** | 18 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 20 | 26 | **19 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Halstead Volume | 3652.4777 | **1819.3223 ✅** | 3665.1147 | +1833.1554 | Higher | -12.6370 | Lower | -1845.7924 | Lower |
| Halstead Effort | 245324.7525 | **58218.3137 ✅** | 255641.7485 | +187106.4388 | Higher | -10316.9960 | Lower | -197423.4348 | Lower |
| SLOC | 86 | **65 ✅** | 84 | +21 | Higher | +2 | Higher | -19 | Lower |
| PLOC | 71 | **45 ✅** | 69 | +26 | Higher | +2 | Higher | -24 | Lower |
| LLOC | 47 | **33 ✅** | 47 | +14 | Higher | 0 | Same | -14 | Lower |
| Comments (CLOC) | **13 ✅** | 11 | 5 | +2 | Higher | +8 | Higher | +6 | Higher |
| Blank Lines | 15 | **11 ✅** | 15 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p00317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.2565 | **54.5546 ✅** | 35.4886 | -11.2981 | Lower | +7.7679 | Higher | +19.0660 | Higher |
| MI (SEI) | -10.2403 | **34.8523 ✅** | -12.4227 | -45.0927 | Lower | +2.1824 | Higher | +47.2750 | Higher |
| Cyclomatic (sum) | 30 | **21 ✅** | 34 | +9 | Higher | -4 | Lower | -13 | Lower |
| Cognitive (sum) | 75 | **23 ✅** | 63 | +52 | Higher | +12 | Higher | -40 | Lower |
| Halstead Volume | 4122.6726 | **2204.1866 ✅** | 6278.6934 | +1918.4860 | Higher | -2156.0208 | Lower | -4074.5068 | Lower |
| Halstead Effort | 279764.5633 | **77881.2606 ✅** | 572692.9460 | +201883.3027 | Higher | -292928.3828 | Lower | -494811.6855 | Lower |
| SLOC | 120 | **83 ✅** | 160 | +37 | Higher | -40 | Lower | -77 | Lower |
| PLOC | 109 | **62 ✅** | 148 | +47 | Higher | -39 | Lower | -86 | Lower |
| LLOC | 86 | **34 ✅** | 124 | +52 | Higher | -38 | Lower | -90 | Lower |
| Comments (CLOC) | 0 | **14 ✅** | 2 | -14 | Lower | -2 | Lower | +12 | Higher |
| Blank Lines | 11 | **10 ✅** | 12 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | **2 ✅** | 4 | 6 | -2 | Lower | -4 | Lower | -2 | Lower |
| Closures | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | **2 ✅** | 7 | 7 | -5 | Lower | -5 | Lower | 0 | Same |

## p00320

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.2850 | **56.4117 ✅** | 53.0473 | -0.1267 | Lower | +3.2378 | Higher | +3.3644 | Higher |
| MI (SEI) | 7.0286 | **37.8621 ✅** | 2.5611 | -30.8335 | Lower | +4.4675 | Higher | +35.3010 | Higher |
| Cyclomatic (sum) | **15 ✅** | 18 | 17 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | 17 | 17 | 18 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 2825.5995 | **2054.0155 ✅** | 3035.9367 | +771.5839 | Higher | -210.3372 | Lower | -981.9211 | Lower |
| Halstead Effort | 202139.0391 | **62305.1380 ✅** | 256003.3071 | +139833.9011 | Higher | -53864.2681 | Lower | -193698.1691 | Lower |
| SLOC | **75 ✅** | 79 | 87 | -4 | Lower | -12 | Lower | -8 | Lower |
| PLOC | 67 | **58 ✅** | 79 | +9 | Higher | -12 | Lower | -21 | Lower |
| LLOC | 43 | **32 ✅** | 51 | +11 | Higher | -8 | Lower | -19 | Lower |
| Comments (CLOC) | 0 | **14 ✅** | 0 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |
| Exits | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |

## p00329

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1376 | **58.7618 ✅** | 48.4283 | -3.6242 | Lower | +6.7092 | Higher | +10.3335 | Higher |
| MI (SEI) | 5.7805 | **34.2812 ✅** | 9.0143 | -28.5008 | Lower | -3.2338 | Lower | +25.2669 | Higher |
| Cyclomatic (sum) | 19 | **18 ✅** | 21 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 24 | **18 ✅** | 21 | +6 | Higher | +3 | Higher | -3 | Lower |
| Halstead Volume | 2235.5027 | **1822.9655 ✅** | 3063.3750 | +412.5371 | Higher | -827.8723 | Lower | -1240.4095 | Lower |
| Halstead Effort | 115351.9389 | **106643.4846 ✅** | 169979.9566 | +8708.4542 | Higher | -54628.0177 | Lower | -63336.4719 | Lower |
| SLOC | 82 | **71 ✅** | 109 | +11 | Higher | -27 | Lower | -38 | Lower |
| PLOC | 73 | **65 ✅** | 93 | +8 | Higher | -20 | Lower | -28 | Lower |
| LLOC | 40 | **25 ✅** | 60 | +15 | Higher | -20 | Lower | -35 | Lower |
| Comments (CLOC) | 0 | **7 ✅** | 3 | -7 | Lower | -3 | Lower | +4 | Higher |
| Blank Lines | 9 | **6 ✅** | 13 | +3 | Higher | -4 | Lower | -7 | Lower |
| Functions | **4 ✅** | 5 | 7 | -1 | Lower | -3 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **4 ✅** | 6 | 5 | -2 | Lower | -1 | Lower | +1 | Higher |

## p00336

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.9953 ✅** | 67.0134 | 73.7323 | +7.9819 | Higher | +1.2630 | Higher | -6.7189 | Lower |
| MI (SEI) | **64.3048 ✅** | 55.2500 | 61.4659 | +9.0548 | Higher | +2.8389 | Higher | -6.2159 | Lower |
| Cyclomatic (sum) | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 7 | 10 | 7 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 1040.7173 | 1302.6127 | 1040.7173 | -261.8954 | Lower | 0 | Same | +261.8954 | Higher |
| Halstead Effort | 35952.0509 | 54397.1057 | 35952.0509 | -18445.0548 | Lower | 0 | Same | +18445.0548 | Higher |
| SLOC | **37 ✅** | 54 | 40 | -17 | Lower | -3 | Lower | +14 | Higher |
| PLOC | 24 | 32 | 24 | -8 | Lower | 0 | Same | +8 | Higher |
| LLOC | 19 | 22 | 19 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 7 | **12 ✅** | 7 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | **8 ✅** | 11 | 9 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00378

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.8115 | **93.7379 ✅** | 77.0260 | -16.9264 | Lower | -0.2145 | Lower | +16.7119 | Higher |
| MI (SEI) | 58.4442 | **103.6089 ✅** | 58.2405 | -45.1648 | Lower | +0.2036 | Higher | +45.3684 | Higher |
| Cyclomatic (sum) | 8 | **3 ✅** | 6 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 9 | **0 ✅** | 9 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 959.4556 | **257.8430 ✅** | 916.5072 | +701.6125 | Higher | +42.9484 | Higher | -658.6641 | Lower |
| Halstead Effort | 26126.7127 | **3094.1164 ✅** | 25343.4154 | +23032.5964 | Higher | +783.2973 | Higher | -22249.2990 | Lower |
| SLOC | 33 | **19 ✅** | 34 | +14 | Higher | -1 | Lower | -15 | Lower |
| PLOC | 25 | **10 ✅** | 26 | +15 | Higher | -1 | Lower | -16 | Lower |
| LLOC | 21 | **5 ✅** | 22 | +16 | Higher | -1 | Lower | -17 | Lower |
| Comments (CLOC) | 3 | **9 ✅** | 3 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00428

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.3291 | **67.7938 ✅** | 62.0268 | -4.4647 | Lower | +1.3022 | Higher | +5.7670 | Higher |
| MI (SEI) | 27.4807 | **49.3640 ✅** | 29.8040 | -21.8833 | Lower | -2.3233 | Lower | +19.5600 | Higher |
| Cyclomatic (sum) | 16 | **12 ✅** | 18 | +4 | Higher | -2 | Lower | -6 | Lower |
| Cognitive (sum) | 29 | **9 ✅** | 25 | +20 | Higher | +4 | Higher | -16 | Lower |
| Halstead Volume | 1640.2347 | **1416.9946 ✅** | 1826.9097 | +223.2401 | Higher | -186.6750 | Lower | -409.9151 | Lower |
| Halstead Effort | 51737.6881 | **38790.2272 ✅** | 87628.6672 | +12947.4609 | Higher | -35890.9791 | Lower | -48838.4400 | Lower |
| SLOC | 57 | **48 ✅** | 58 | +9 | Higher | -1 | Lower | -10 | Lower |
| PLOC | 47 | **35 ✅** | 52 | +12 | Higher | -5 | Lower | -17 | Lower |
| LLOC | 33 | **19 ✅** | 42 | +14 | Higher | -9 | Lower | -23 | Lower |
| Comments (CLOC) | 1 | **6 ✅** | 2 | -5 | Lower | -1 | Lower | +4 | Higher |
| Blank Lines | 10 | 10 | **6 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

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

## p00450

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.3964 | **74.7158 ✅** | 68.4574 | -4.3194 | Lower | +1.9390 | Higher | +6.2585 | Higher |
| MI (SEI) | 57.8620 | **68.0809 ✅** | 54.9760 | -10.2189 | Lower | +2.8860 | Higher | +13.1049 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 15 | +3 | Higher | -2 | Lower | -5 | Lower |
| Cognitive (sum) | 19 | **12 ✅** | 16 | +7 | Higher | +3 | Higher | -4 | Lower |
| Halstead Volume | 1077.7608 | **721.6932 ✅** | 1335.4941 | +356.0676 | Higher | -257.7333 | Lower | -613.8009 | Lower |
| Halstead Effort | 39405.6298 | **22552.9129 ✅** | 55398.2735 | +16852.7169 | Higher | -15992.6437 | Lower | -32845.3606 | Lower |
| SLOC | 44 | **40 ✅** | 45 | +4 | Higher | -1 | Lower | -5 | Lower |
| PLOC | 31 | **27 ✅** | 37 | +4 | Higher | -6 | Lower | -10 | Lower |
| LLOC | 19 | **18 ✅** | 26 | +1 | Higher | -7 | Lower | -8 | Lower |
| Comments (CLOC) | 8 | **10 ✅** | 8 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 8 | 7 | **5 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00453

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.7888 | **70.3441 ✅** | 51.1314 | -17.5553 | Lower | +1.6573 | Higher | +19.2126 | Higher |
| MI (SEI) | 15.0757 | **55.1112 ✅** | 0.5099 | -40.0355 | Lower | +14.5659 | Higher | +54.6013 | Higher |
| Cyclomatic (sum) | 23 | **9 ✅** | 24 | +14 | Higher | -1 | Lower | -15 | Lower |
| Cognitive (sum) | 64 | **16 ✅** | 46 | +48 | Higher | +18 | Higher | -30 | Lower |
| Halstead Volume | 3438.5278 | **1131.3217 ✅** | 4022.7094 | +2307.2060 | Higher | -584.1816 | Lower | -2891.3877 | Lower |
| Halstead Effort | 226552.0912 | **24388.6859 ✅** | 362043.8450 | +202163.4053 | Higher | -135491.7538 | Lower | -337655.1590 | Lower |
| SLOC | 78 | **46 ✅** | 81 | +32 | Higher | -3 | Lower | -35 | Lower |
| PLOC | 68 | **32 ✅** | 72 | +36 | Higher | -4 | Lower | -40 | Lower |
| LLOC | 54 | **25 ✅** | 57 | +29 | Higher | -3 | Lower | -32 | Lower |
| Comments (CLOC) | 2 | **7 ✅** | 0 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **0 ✅** | 3 | +2 | Higher | -1 | Lower | -3 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00465

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 31.4114 | **50.6938 ✅** | 36.0096 | -19.2823 | Lower | -4.5981 | Lower | +14.6842 | Higher |
| MI (SEI) | -26.4128 | **26.5339 ✅** | -11.2846 | -52.9466 | Lower | -15.1282 | Lower | +37.8184 | Higher |
| Cyclomatic (sum) | 39 | **17 ✅** | 37 | +22 | Higher | +2 | Higher | -20 | Lower |
| Cognitive (sum) | 38 | **13 ✅** | 55 | +25 | Higher | -17 | Lower | -42 | Lower |
| Halstead Volume | 6896.3227 | **3878.0932 ✅** | 5276.4440 | +3018.2296 | Higher | +1619.8787 | Higher | -1398.3509 | Lower |
| Halstead Effort | 579291.1089 | **165550.6754 ✅** | 447002.7497 | +413740.4335 | Higher | +132288.3592 | Higher | -281452.0743 | Lower |
| SLOC | 186 | **93 ✅** | 157 | +93 | Higher | +29 | Higher | -64 | Lower |
| PLOC | 158 | **67 ✅** | 139 | +91 | Higher | +19 | Higher | -72 | Lower |
| LLOC | 109 | **44 ✅** | 94 | +65 | Higher | +15 | Higher | -50 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 2 | -13 | Lower | -2 | Lower | +11 | Higher |
| Blank Lines | 28 | **14 ✅** | 18 | +14 | Higher | +10 | Higher | -4 | Lower |
| Functions | 7 | **3 ✅** | 9 | +4 | Higher | -2 | Lower | -6 | Lower |
| Closures | 3 | 6 | **0 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Exits | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |

## p00468

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.6579 | **64.8536 ✅** | 61.5926 | -6.1957 | Lower | -2.9348 | Lower | +3.2610 | Higher |
| MI (SEI) | 34.1345 | **52.4395 ✅** | 14.6858 | -18.3049 | Lower | +19.4487 | Higher | +37.7537 | Higher |
| Cyclomatic (sum) | 15 | **12 ✅** | 15 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 30 | **16 ✅** | 18 | +14 | Higher | +12 | Higher | -2 | Lower |
| Halstead Volume | 2321.4014 | **1728.1044 ✅** | 1752.7794 | +593.2970 | Higher | +568.6219 | Higher | -24.6750 | Lower |
| Halstead Effort | **50244.0298 ✅** | 56722.4854 | 73798.0581 | -6478.4556 | Lower | -23554.0283 | Lower | -17075.5727 | Lower |
| SLOC | 69 | **54 ✅** | 63 | +15 | Higher | +6 | Higher | -9 | Lower |
| PLOC | 51 | **38 ✅** | 51 | +13 | Higher | 0 | Same | -13 | Lower |
| LLOC | 38 | **33 ✅** | 41 | +5 | Higher | -3 | Lower | -8 | Lower |
| Comments (CLOC) | 7 | **12 ✅** | 0 | -5 | Lower | +7 | Higher | +12 | Higher |
| Blank Lines | 11 | **9 ✅** | 12 | +2 | Higher | -1 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00479

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1819 | 67.1737 | **77.0343 ✅** | +7.0083 | Higher | -2.8524 | Lower | -9.8607 | Lower |
| MI (SEI) | 60.5805 | 57.9079 | **66.7258 ✅** | +2.6726 | Higher | -6.1453 | Lower | -8.8179 | Lower |
| Cyclomatic (sum) | 7 | 12 | **6 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Cognitive (sum) | **8 ✅** | 11 | 10 | -3 | Lower | -2 | Lower | +1 | Higher |
| Halstead Volume | **988.1571 ✅** | 1106.1229 | 1004.2264 | -117.9658 | Lower | -16.0693 | Lower | +101.8965 | Higher |
| Halstead Effort | **27066.9125 ✅** | 29988.2220 | 27189.4304 | -2921.3095 | Lower | -122.5179 | Lower | +2798.7916 | Higher |
| SLOC | 39 | 54 | **33 ✅** | -15 | Lower | +6 | Higher | +21 | Higher |
| PLOC | 26 | 34 | **21 ✅** | -8 | Lower | +5 | Higher | +13 | Higher |
| LLOC | 15 | 19 | **13 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Comments (CLOC) | 6 | **14 ✅** | 6 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 7 | 11 | **6 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00492

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.7919 | **57.7476 ✅** | 57.3549 | -2.9557 | Lower | -2.5630 | Lower | +0.3927 | Higher |
| MI (SEI) | 23.2094 | **40.4180 ✅** | 18.7317 | -17.2086 | Lower | +4.4777 | Higher | +21.6863 | Higher |
| Cyclomatic (sum) | 22 | **20 ✅** | 21 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 32 | **23 ✅** | 32 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 2880.7630 | **2028 ✅** | 2754.9445 | +852.7630 | Higher | +125.8185 | Higher | -726.9445 | Lower |
| Halstead Effort | 211053.7958 | **84500 ✅** | 200142.9972 | +126553.7958 | Higher | +10910.7986 | Higher | -115642.9972 | Lower |
| SLOC | 74 | 71 | **65 ✅** | +3 | Higher | +9 | Higher | +6 | Higher |
| PLOC | 62 | **49 ✅** | 58 | +13 | Higher | +4 | Higher | -9 | Lower |
| LLOC | 51 | **33 ✅** | 48 | +18 | Higher | +3 | Higher | -15 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 1 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | 8 | 10 | **7 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.3260 | **72.7726 ✅** | 63.1878 | -9.4466 | Lower | +0.1382 | Higher | +9.5849 | Higher |
| MI (SEI) | 16.9829 | **65.9230 ✅** | 16.7835 | -48.9401 | Lower | +0.1994 | Higher | +49.1395 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 15 | **13 ✅** | 15 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 1775.2521 | **900.8062 ✅** | 1823.0704 | +874.4459 | Higher | -47.8182 | Lower | -922.2642 | Lower |
| Halstead Effort | 69234.8334 | **31571.1131 ✅** | 71099.7447 | +37663.7202 | Higher | -1864.9113 | Lower | -39528.6316 | Lower |
| SLOC | 58 | **42 ✅** | 58 | +16 | Higher | 0 | Same | -16 | Lower |
| PLOC | 49 | **28 ✅** | 49 | +21 | Higher | 0 | Same | -21 | Lower |
| LLOC | 35 | **22 ✅** | 35 | +13 | Higher | 0 | Same | -13 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 0 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

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

## p00518

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.4541 | **73.0330 ✅** | 62.7751 | -13.5789 | Lower | -3.3210 | Lower | +10.2579 | Higher |
| MI (SEI) | 37.3100 | **67.0823 ✅** | 30.3460 | -29.7722 | Lower | +6.9640 | Higher | +36.7362 | Higher |
| Cyclomatic (sum) | 17 | **9 ✅** | 15 | +8 | Higher | +2 | Higher | -6 | Lower |
| Cognitive (sum) | 27 | **3 ✅** | 27 | +24 | Higher | 0 | Same | -24 | Lower |
| Halstead Volume | 1998.1463 | **832.2642 ✅** | 1625.4956 | +1165.8821 | Higher | +372.6507 | Higher | -793.2314 | Lower |
| Halstead Effort | 81257.9480 | **18812.0408 ✅** | 69994.8254 | +62445.9073 | Higher | +11263.1227 | Higher | -51182.7846 | Lower |
| SLOC | 67 | **43 ✅** | 60 | +24 | Higher | +7 | Higher | -17 | Lower |
| PLOC | 51 | **25 ✅** | 48 | +26 | Higher | +3 | Higher | -23 | Lower |
| LLOC | 40 | **15 ✅** | 39 | +25 | Higher | +1 | Higher | -24 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 2 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 13 | **9 ✅** | 12 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00519

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **47.0448 ✅** | 44.3304 | 36.1709 | +2.7144 | Higher | +10.8740 | Higher | +8.1596 | Higher |
| MI (SEI) | 4.0894 | **15.3266 ✅** | -10.3766 | -11.2373 | Lower | +14.4660 | Higher | +25.7032 | Higher |
| Cyclomatic (sum) | **17 ✅** | 28 | 29 | -11 | Lower | -12 | Lower | -1 | Lower |
| Cognitive (sum) | **18 ✅** | 20 | 29 | -2 | Lower | -11 | Lower | -9 | Lower |
| Halstead Volume | 4148.6140 | **3479.6870 ✅** | 5584.3773 | +668.9270 | Higher | -1435.7633 | Lower | -2104.6903 | Lower |
| Halstead Effort | 180183.4474 | **162608.4489 ✅** | 426529.3314 | +17574.9984 | Higher | -246345.8841 | Lower | -263920.8825 | Lower |
| SLOC | **114 ✅** | 122 | 171 | -8 | Lower | -57 | Lower | -49 | Lower |
| PLOC | 97 | **96 ✅** | 147 | +1 | Higher | -50 | Lower | -51 | Lower |
| LLOC | 55 | **43 ✅** | 94 | +12 | Higher | -39 | Lower | -51 | Lower |
| Comments (CLOC) | 2 | **13 ✅** | 3 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | **17 ✅** | 21 | 23 | -4 | Lower | -6 | Lower | -2 | Lower |
| Functions | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |

## p00528

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.3122 | **48.9166 ✅** | 26.4157 | -12.6044 | Lower | +9.8965 | Higher | +22.5009 | Higher |
| MI (SEI) | -14.3084 | **21.0520 ✅** | -24.3550 | -35.3604 | Lower | +10.0466 | Higher | +45.4069 | Higher |
| Cyclomatic (sum) | 29 | **25 ✅** | 43 | +4 | Higher | -14 | Lower | -18 | Lower |
| Cognitive (sum) | 25 | **16 ✅** | 41 | +9 | Higher | -16 | Lower | -25 | Lower |
| Halstead Volume | 6309.5167 | **3153.4203 ✅** | 8118.7923 | +3156.0963 | Higher | -1809.2756 | Lower | -4965.3719 | Lower |
| Halstead Effort | 457439.9591 | **120373.6661 ✅** | 966136.2794 | +337066.2930 | Higher | -508696.3203 | Lower | -845762.6133 | Lower |
| SLOC | 163 | **99 ✅** | 227 | +64 | Higher | -64 | Lower | -128 | Lower |
| PLOC | 141 | **78 ✅** | 198 | +63 | Higher | -57 | Lower | -120 | Lower |
| LLOC | 110 | **38 ✅** | 169 | +72 | Higher | -59 | Lower | -131 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 3 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 22 | **19 ✅** | 29 | +3 | Higher | -7 | Lower | -10 | Lower |
| Functions | **4 ✅** | 5 | 11 | -1 | Lower | -7 | Lower | -6 | Lower |
| Closures | 4 | 2 | 2 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **4 ✅** | 5 | 8 | -1 | Lower | -4 | Lower | -3 | Lower |

## p00534

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.5662 | **71.6466 ✅** | 68.0206 | -16.0804 | Lower | -12.4544 | Lower | +3.6260 | Higher |
| MI (SEI) | 30.5452 | **61.2376 ✅** | 49.8868 | -30.6924 | Lower | -19.3416 | Lower | +11.3508 | Higher |
| Cyclomatic (sum) | 21 | **7 ✅** | 9 | +14 | Higher | +12 | Higher | -2 | Lower |
| Cognitive (sum) | 15 | **7 ✅** | 9 | +8 | Higher | +6 | Higher | -2 | Lower |
| Halstead Volume | 2292.3656 | **1277.3263 ✅** | 1768.6336 | +1015.0393 | Higher | +523.7320 | Higher | -491.3073 | Lower |
| Halstead Effort | 117716.0703 | **43578.1143 ✅** | 67650.2352 | +74137.9560 | Higher | +50065.8351 | Higher | -24072.1210 | Lower |
| SLOC | 77 | **42 ✅** | 46 | +35 | Higher | +31 | Higher | -4 | Lower |
| PLOC | 60 | **28 ✅** | 32 | +32 | Higher | +28 | Higher | -4 | Lower |
| LLOC | 38 | **21 ✅** | 28 | +17 | Higher | +10 | Higher | -7 | Lower |
| Comments (CLOC) | 8 | **9 ✅** | 6 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 11 | **6 ✅** | 8 | +5 | Higher | +3 | Higher | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | **0 ✅** | 1 | +3 | Higher | +2 | Higher | -1 | Lower |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00543

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.1986 | **82.6205 ✅** | 74.4883 | -15.4219 | Lower | -7.2896 | Lower | +8.1322 | Higher |
| MI (SEI) | 50.1493 | **85.9772 ✅** | 54.3022 | -35.8279 | Lower | -4.1528 | Lower | +31.6750 | Higher |
| Cyclomatic (sum) | 10 | **7 ✅** | 9 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 12 | **2 ✅** | 12 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 1735.7771 | **613.1679 ✅** | 1094.2540 | +1122.6092 | Higher | +641.5231 | Higher | -481.0861 | Lower |
| Halstead Effort | 33383.9865 | **12583.8779 ✅** | 35396.7382 | +20800.1086 | Higher | -2012.7517 | Lower | -22812.8603 | Lower |
| SLOC | 48 | **27 ✅** | 36 | +21 | Higher | +12 | Higher | -9 | Lower |
| PLOC | 33 | **16 ✅** | 29 | +17 | Higher | +4 | Higher | -13 | Lower |
| LLOC | 20 | **10 ✅** | 22 | +10 | Higher | -2 | Lower | -12 | Lower |
| Comments (CLOC) | 7 | **11 ✅** | 3 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 8 | **4 ✅** | 5 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00554

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.2245 | 65.6247 | **66.1077 ✅** | -3.4002 | Lower | -3.8832 | Lower | -0.4829 | Lower |
| MI (SEI) | 38.1944 | **53.2616 ✅** | 44.0002 | -15.0671 | Lower | -5.8058 | Lower | +9.2614 | Higher |
| Cyclomatic (sum) | 11 | **8 ✅** | 12 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 1852.6429 | **1423.3770 ✅** | 1527.2011 | +429.2659 | Higher | +325.4418 | Higher | -103.8241 | Lower |
| Halstead Effort | 74723.2640 | **29517.2797 ✅** | 63496.3217 | +45205.9842 | Higher | +11226.9422 | Higher | -33979.0420 | Lower |
| SLOC | 63 | 58 | **52 ✅** | +5 | Higher | +11 | Higher | +6 | Higher |
| PLOC | 45 | **33 ✅** | 40 | +12 | Higher | +5 | Higher | -7 | Lower |
| LLOC | 41 | **24 ✅** | 36 | +17 | Higher | +5 | Higher | -12 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 5 | -7 | Lower | +1 | Higher | +8 | Higher |
| Blank Lines | 12 | 13 | **9 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00571

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.6650 | **71.7482 ✅** | 56.6599 | -6.0832 | Lower | +9.0051 | Higher | +15.0883 | Higher |
| MI (SEI) | 47.7012 | **66.8634 ✅** | 26.3520 | -19.1622 | Lower | +21.3492 | Higher | +40.5114 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 14 | +1 | Higher | -6 | Lower | -7 | Lower |
| Cognitive (sum) | 9 | **2 ✅** | 15 | +7 | Higher | -6 | Lower | -13 | Lower |
| Halstead Volume | 1666.5372 | **1083.6204 ✅** | 2247.4573 | +582.9168 | Higher | -580.9201 | Lower | -1163.8369 | Lower |
| Halstead Effort | 47408.5975 | **25894.7907 ✅** | 103443.7756 | +21513.8068 | Higher | -56035.1782 | Lower | -77548.9850 | Lower |
| SLOC | 55 | **44 ✅** | 80 | +11 | Higher | -25 | Lower | -36 | Lower |
| PLOC | 43 | **30 ✅** | 68 | +13 | Higher | -25 | Lower | -38 | Lower |
| LLOC | 28 | **20 ✅** | 44 | +8 | Higher | -16 | Lower | -24 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 5 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 9 | **8 ✅** | 12 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | **1 ✅** | 2 | 4 | -1 | Lower | -3 | Lower | -2 | Lower |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | **0 ✅** | 1 | 4 | -1 | Lower | -4 | Lower | -3 | Lower |

## p00594

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.6177 | 60.3549 | **65.3166 ✅** | -1.7372 | Lower | -6.6989 | Lower | -4.9617 | Lower |
| MI (SEI) | 36.7619 | 40.0239 | **48.8477 ✅** | -3.2620 | Lower | -12.0858 | Lower | -8.8238 | Lower |
| Cyclomatic (sum) | 15 | 15 | **12 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 23 | 21 | **19 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Halstead Volume | 2236.8590 | **1405.3202 ✅** | 1411.5594 | +831.5388 | Higher | +825.2996 | Higher | -6.2392 | Lower |
| Halstead Effort | **58108.6258 ✅** | 60428.7682 | 59966.9366 | -2320.1424 | Lower | -1858.3108 | Lower | +461.8317 | Higher |
| SLOC | 70 | 73 | **56 ✅** | -3 | Lower | +14 | Higher | +17 | Higher |
| PLOC | 50 | 54 | **44 ✅** | -4 | Lower | +6 | Higher | +10 | Higher |
| LLOC | **33 ✅** | 41 | 34 | -8 | Lower | -1 | Lower | +7 | Higher |
| Comments (CLOC) | 9 | **10 ✅** | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 14 | 12 | **9 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00629

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.0205 | **60.1247 ✅** | 55.8048 | -3.1042 | Lower | +1.2157 | Higher | +4.3199 | Higher |
| MI (SEI) | 27.2908 | **39.5783 ✅** | 24.9029 | -12.2875 | Lower | +2.3879 | Higher | +14.6754 | Higher |
| Cyclomatic (sum) | **17 ✅** | 19 | 22 | -2 | Lower | -5 | Lower | -3 | Lower |
| Cognitive (sum) | 20 | **10 ✅** | 19 | +10 | Higher | +1 | Higher | -9 | Lower |
| Halstead Volume | **1909.7018 ✅** | 2153.4617 | 2582.1849 | -243.7599 | Lower | -672.4831 | Lower | -428.7233 | Lower |
| Halstead Effort | **70097.2898 ✅** | 71011.7114 | 135180.8704 | -914.4216 | Lower | -65083.5806 | Lower | -64169.1590 | Lower |
| SLOC | 79 | **61 ✅** | 72 | +18 | Higher | +7 | Higher | -11 | Lower |
| PLOC | 65 | **45 ✅** | 64 | +20 | Higher | +1 | Higher | -19 | Lower |
| LLOC | 35 | **24 ✅** | 40 | +11 | Higher | -5 | Lower | -16 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 4 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 13 | 10 | **7 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 9 | **1 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00641

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.2158 | **58.7194 ✅** | 46.8782 | -9.5035 | Lower | +2.3377 | Higher | +11.8412 | Higher |
| MI (SEI) | 5.1944 | **33.6627 ✅** | 7.0267 | -28.4683 | Lower | -1.8323 | Lower | +26.6360 | Higher |
| Cyclomatic (sum) | 21 | **14 ✅** | 24 | +7 | Higher | -3 | Lower | -10 | Lower |
| Cognitive (sum) | 15 | **10 ✅** | 16 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | 3553.2680 | **2100.1036 ✅** | 3513.0537 | +1453.1644 | Higher | +40.2142 | Higher | -1412.9502 | Lower |
| Halstead Effort | 192835.8527 | **84529.1686 ✅** | 289826.9337 | +108306.6840 | Higher | -96991.0810 | Lower | -205297.7650 | Lower |
| SLOC | 99 | **72 ✅** | 110 | +27 | Higher | -11 | Lower | -38 | Lower |
| PLOC | 83 | **58 ✅** | 93 | +25 | Higher | -10 | Lower | -35 | Lower |
| LLOC | 56 | **31 ✅** | 68 | +25 | Higher | -12 | Lower | -37 | Lower |
| Comments (CLOC) | 1 | **7 ✅** | 3 | -6 | Lower | -2 | Lower | +4 | Higher |
| Blank Lines | 16 | **10 ✅** | 15 | +6 | Higher | +1 | Higher | -5 | Lower |
| Functions | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | **3 ✅** | 4 | 5 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00649

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.5949 | **54.7782 ✅** | 44.2859 | -5.1833 | Lower | +5.3090 | Higher | +10.4923 | Higher |
| MI (SEI) | 15.1884 | **24.4518 ✅** | 4.4642 | -9.2634 | Lower | +10.7242 | Higher | +19.9876 | Higher |
| Cyclomatic (sum) | 24 | **22 ✅** | 26 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 31 | **19 ✅** | 40 | +12 | Higher | -9 | Lower | -21 | Lower |
| Halstead Volume | 2718.1882 | **2265.5417 ✅** | 3467.5998 | +452.6466 | Higher | -749.4115 | Lower | -1202.0581 | Lower |
| Halstead Effort | 102670.6966 | **86702.8917 ✅** | 212578.9424 | +15967.8049 | Higher | -109908.2458 | Lower | -125876.0507 | Lower |
| SLOC | 101 | **80 ✅** | 126 | +21 | Higher | -25 | Lower | -46 | Lower |
| PLOC | 88 | **61 ✅** | 109 | +27 | Higher | -21 | Lower | -48 | Lower |
| LLOC | 47 | **34 ✅** | 65 | +13 | Higher | -18 | Lower | -31 | Lower |
| Comments (CLOC) | 5 | 5 | 4 | 0 | Same | +1 | Higher | +1 | Higher |
| Blank Lines | **13 ✅** | 15 | 17 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p00653

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.3370 | **54.2922 ✅** | 44.2003 | -8.9552 | Lower | +1.1367 | Higher | +10.0919 | Higher |
| MI (SEI) | -7.5443 | **33.1820 ✅** | -9.0824 | -40.7262 | Lower | +1.5381 | Higher | +42.2643 | Higher |
| Cyclomatic (sum) | 27 | **17 ✅** | 28 | +10 | Higher | -1 | Lower | -11 | Lower |
| Cognitive (sum) | 27 | **11 ✅** | 30 | +16 | Higher | -3 | Lower | -19 | Lower |
| Halstead Volume | 4927.9055 | **2226.2085 ✅** | 4926.0635 | +2701.6970 | Higher | +1.8420 | Higher | -2699.8550 | Lower |
| Halstead Effort | 437450.1695 | **98357.9373 ✅** | 448733.5926 | +339092.2322 | Higher | -11283.4231 | Lower | -350375.6553 | Lower |
| SLOC | 104 | **89 ✅** | 110 | +15 | Higher | -6 | Lower | -21 | Lower |
| PLOC | 86 | **64 ✅** | 91 | +22 | Higher | -5 | Lower | -27 | Lower |
| LLOC | 75 | **28 ✅** | 80 | +47 | Higher | -5 | Lower | -52 | Lower |
| Comments (CLOC) | 0 | **14 ✅** | 0 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 18 | **13 ✅** | 19 | +5 | Higher | -1 | Lower | -6 | Lower |
| Functions | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |

## p00655

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 28.9788 | **43.3226 ✅** | 29.8184 | -14.3438 | Lower | -0.8396 | Lower | +13.5042 | Higher |
| MI (SEI) | -21.3367 | **15.9556 ✅** | -20.1024 | -37.2923 | Lower | -1.2344 | Lower | +36.0580 | Higher |
| Cyclomatic (sum) | 46 | **41 ✅** | 45 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 48 | **30 ✅** | 41 | +18 | Higher | +7 | Higher | -11 | Lower |
| Halstead Volume | 7317.5424 | **2935.9872 ✅** | 7184.6694 | +4381.5552 | Higher | +132.8729 | Higher | -4248.6823 | Lower |
| Halstead Effort | 925166.9239 | **105499.8056 ✅** | 933161.7719 | +819667.1183 | Higher | -7994.8479 | Lower | -827661.9663 | Lower |
| SLOC | 192 | **114 ✅** | 186 | +78 | Higher | +6 | Higher | -72 | Lower |
| PLOC | 175 | **98 ✅** | 172 | +77 | Higher | +3 | Higher | -74 | Lower |
| LLOC | 113 | **33 ✅** | 120 | +80 | Higher | -7 | Lower | -87 | Lower |
| Comments (CLOC) | 2 | **13 ✅** | 2 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 17 | **12 ✅** | 14 | +5 | Higher | +3 | Higher | -2 | Lower |
| Functions | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 9 | **4 ✅** | 9 | +5 | Higher | 0 | Same | -5 | Lower |

## p00658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | -2.7207 | **41.2008 ✅** | -8.5673 | -43.9215 | Lower | +5.8466 | Higher | +49.7681 | Higher |
| MI (SEI) | -66.4912 | **16.7528 ✅** | -65.3628 | -83.2440 | Lower | -1.1285 | Lower | +82.1155 | Higher |
| Cyclomatic (sum) | 129 | **33 ✅** | 146 | +96 | Higher | -17 | Lower | -113 | Lower |
| Cognitive (sum) | 175 | **18 ✅** | 211 | +157 | Higher | -36 | Lower | -193 | Lower |
| Halstead Volume | 13321.9936 | **4079.2260 ✅** | 13408.7570 | +9242.7675 | Higher | -86.7634 | Lower | -9329.5309 | Lower |
| Halstead Effort | 1827777.5195 | **245330.8114 ✅** | 1873007.4882 | +1582446.7081 | Higher | -45229.9687 | Lower | -1627676.6768 | Lower |
| SLOC | 345 | **131 ✅** | 388 | +214 | Higher | -43 | Lower | -257 | Lower |
| PLOC | 322 | **111 ✅** | 369 | +211 | Higher | -47 | Lower | -258 | Lower |
| LLOC | 222 | **55 ✅** | 265 | +167 | Higher | -43 | Lower | -210 | Lower |
| Comments (CLOC) | 0 | **22 ✅** | 4 | -22 | Lower | -4 | Lower | +18 | Higher |
| Blank Lines | 23 | **12 ✅** | 15 | +11 | Higher | +8 | Higher | -3 | Lower |
| Functions | 12 | **8 ✅** | 10 | +4 | Higher | +2 | Higher | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 23 | **1 ✅** | 27 | +22 | Higher | -4 | Lower | -26 | Lower |

## p00680

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 16.0814 | **45.0964 ✅** | 15.7847 | -29.0149 | Lower | +0.2967 | Higher | +29.3117 | Higher |
| MI (SEI) | -38.5227 | **20.9171 ✅** | -39.2767 | -59.4397 | Lower | +0.7541 | Higher | +60.1938 | Higher |
| Cyclomatic (sum) | 61 | **25 ✅** | 60 | +36 | Higher | +1 | Higher | -35 | Lower |
| Cognitive (sum) | 81 | **19 ✅** | 83 | +62 | Higher | -2 | Lower | -64 | Lower |
| Halstead Volume | 11687.7158 | **3705.8104 ✅** | 10760.9355 | +7981.9055 | Higher | +926.7803 | Higher | -7055.1252 | Lower |
| Halstead Effort | 1351392.1434 | **171475.4745 ✅** | 1184829.0547 | +1179916.6689 | Higher | +166563.0887 | Higher | -1013353.5802 | Lower |
| SLOC | 296 | **119 ✅** | 314 | +177 | Higher | -18 | Lower | -195 | Lower |
| PLOC | 257 | **96 ✅** | 273 | +161 | Higher | -16 | Lower | -177 | Lower |
| LLOC | 169 | **35 ✅** | 185 | +134 | Higher | -16 | Lower | -150 | Lower |
| Comments (CLOC) | 3 | **19 ✅** | 3 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 38 | **16 ✅** | 39 | +22 | Higher | -1 | Lower | -23 | Lower |
| Functions | 14 | **6 ✅** | 14 | +8 | Higher | 0 | Same | -8 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 12 | **7 ✅** | 12 | +5 | Higher | 0 | Same | -5 | Lower |

## p00690

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.8158 | **52.0124 ✅** | 45.6078 | -5.1966 | Lower | +1.2080 | Higher | +6.4046 | Higher |
| MI (SEI) | -5.3090 | **14.8014 ✅** | -6.9500 | -20.1104 | Lower | +1.6410 | Higher | +21.7514 | Higher |
| Cyclomatic (sum) | 28 | **17 ✅** | 29 | +11 | Higher | -1 | Lower | -12 | Lower |
| Cognitive (sum) | 41 | **22 ✅** | 38 | +19 | Higher | +3 | Higher | -16 | Lower |
| Halstead Volume | 3547.7069 | **3009.4780 ✅** | 3595.3536 | +538.2289 | Higher | -47.6467 | Lower | -585.8756 | Lower |
| Halstead Effort | 207540.8522 | **113836.7751 ✅** | 227705.7270 | +93704.0772 | Higher | -20164.8748 | Lower | -113868.9520 | Lower |
| SLOC | 104 | **93 ✅** | 110 | +11 | Higher | -6 | Lower | -17 | Lower |
| PLOC | 95 | **75 ✅** | 98 | +20 | Higher | -3 | Lower | -23 | Lower |
| LLOC | 60 | **47 ✅** | 67 | +13 | Higher | -7 | Lower | -20 | Lower |
| Comments (CLOC) | 0 | **3 ✅** | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | **9 ✅** | 15 | 12 | -6 | Lower | -3 | Lower | +3 | Higher |
| Functions | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | **0 ✅** | 3 | 1 | -3 | Lower | -1 | Lower | +2 | Higher |

## p00692

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.0221 | **74.2798 ✅** | 47.3877 | -15.2576 | Lower | +11.6344 | Higher | +26.8920 | Higher |
| MI (SEI) | 11.8937 | **65.3628 ✅** | 16.7583 | -53.4690 | Lower | -4.8645 | Lower | +48.6045 | Higher |
| Cyclomatic (sum) | 24 | **7 ✅** | 31 | +17 | Higher | -7 | Lower | -24 | Lower |
| Cognitive (sum) | 20 | **9 ✅** | 28 | +11 | Higher | -8 | Lower | -19 | Lower |
| Halstead Volume | 2785.4124 | **829.8336 ✅** | 4366.8416 | +1955.5788 | Higher | -1581.4292 | Lower | -3537.0080 | Lower |
| Halstead Effort | 174201.1999 | **14820.8282 ✅** | 182763.8136 | +159380.3717 | Higher | -8562.6137 | Lower | -167942.9854 | Lower |
| SLOC | 56 | **41 ✅** | 90 | +15 | Higher | -34 | Lower | -49 | Lower |
| PLOC | 51 | **28 ✅** | 72 | +23 | Higher | -21 | Lower | -44 | Lower |
| LLOC | 39 | **16 ✅** | 50 | +23 | Higher | -11 | Lower | -34 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 7 | -9 | Lower | -7 | Lower | +2 | Higher |
| Blank Lines | **5 ✅** | 7 | 11 | -2 | Lower | -6 | Lower | -4 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00723

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.6939 | **62.6765 ✅** | 44.1088 | -7.9826 | Lower | +10.5851 | Higher | +18.5677 | Higher |
| MI (SEI) | 4.8349 | **41.5216 ✅** | 0.2511 | -36.6867 | Lower | +4.5837 | Higher | +41.2705 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 23 | +5 | Higher | -7 | Lower | -12 | Lower |
| Cognitive (sum) | 28 | **10 ✅** | 31 | +18 | Higher | -3 | Lower | -21 | Lower |
| Halstead Volume | 3002.4025 | **1469.2608 ✅** | 4895.3206 | +1533.1417 | Higher | -1892.9181 | Lower | -3426.0598 | Lower |
| Halstead Effort | 144733.4614 | **35099.0079 ✅** | 175613.1857 | +109634.4535 | Higher | -30879.7243 | Lower | -140514.1779 | Lower |
| SLOC | 80 | **66 ✅** | 119 | +14 | Higher | -39 | Lower | -53 | Lower |
| PLOC | 66 | **45 ✅** | 102 | +21 | Higher | -36 | Lower | -57 | Lower |
| LLOC | 44 | **27 ✅** | 73 | +17 | Higher | -29 | Lower | -46 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 2 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 14 | **13 ✅** | 17 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |

## p00740

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.6487 | **75.3427 ✅** | 68.6647 | -6.6939 | Lower | -0.0159 | Lower | +6.6780 | Higher |
| MI (SEI) | 49.6550 | **67.4702 ✅** | 35.4502 | -17.8152 | Lower | +14.2048 | Higher | +32.0200 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 12 | +5 | Higher | +4 | Higher | -1 | Lower |
| Cognitive (sum) | 16 | **6 ✅** | 16 | +10 | Higher | 0 | Same | -10 | Lower |
| Halstead Volume | 1231.5459 | **849.8521 ✅** | 1055.3719 | +381.6938 | Higher | +176.1740 | Higher | -205.5198 | Lower |
| Halstead Effort | 49072.3684 | **21582.7021 ✅** | 40775.7318 | +27489.6663 | Higher | +8296.6366 | Higher | -19193.0297 | Lower |
| SLOC | 45 | **36 ✅** | 50 | +9 | Higher | -5 | Lower | -14 | Lower |
| PLOC | 37 | **28 ✅** | 44 | +9 | Higher | -7 | Lower | -16 | Lower |
| LLOC | 26 | **13 ✅** | 33 | +13 | Higher | -7 | Lower | -20 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 1 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00749

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 22.7892 | **69.5896 ✅** | 23.5174 | -46.8004 | Lower | -0.7281 | Lower | +46.0723 | Higher |
| MI (SEI) | -37.0192 | **58.9319 ✅** | -36.3760 | -95.9511 | Lower | -0.6432 | Lower | +95.3079 | Higher |
| Cyclomatic (sum) | 57 | **10 ✅** | 53 | +47 | Higher | +4 | Higher | -43 | Lower |
| Cognitive (sum) | 83 | **11 ✅** | 78 | +72 | Higher | +5 | Higher | -67 | Lower |
| Halstead Volume | 8093.6519 | **965.0994 ✅** | 8069.7523 | +7128.5525 | Higher | +23.8996 | Higher | -7104.6529 | Lower |
| Halstead Effort | 817458.8459 | **21786.2255 ✅** | 800863.5811 | +795672.6204 | Higher | +16595.2649 | Higher | -779077.3555 | Lower |
| SLOC | 233 | **50 ✅** | 236 | +183 | Higher | -3 | Lower | -186 | Lower |
| PLOC | 211 | **31 ✅** | 210 | +180 | Higher | +1 | Higher | -179 | Lower |
| LLOC | 138 | **18 ✅** | 137 | +120 | Higher | +1 | Higher | -119 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 0 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 22 | **9 ✅** | 26 | +13 | Higher | -4 | Lower | -17 | Lower |
| Functions | 8 | **2 ✅** | 9 | +6 | Higher | -1 | Lower | -7 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 13 | **1 ✅** | 13 | +12 | Higher | 0 | Same | -12 | Lower |

## p00762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.0361 | **67.9460 ✅** | 39.1929 | -28.9099 | Lower | -0.1568 | Lower | +28.7532 | Higher |
| MI (SEI) | -3.6052 | **54.4048 ✅** | 4.5237 | -58.0100 | Lower | -8.1289 | Lower | +49.8811 | Higher |
| Cyclomatic (sum) | 30 | **9 ✅** | 30 | +21 | Higher | 0 | Same | -21 | Lower |
| Cognitive (sum) | 15 | 15 | 16 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 5147.7444 | **1154.0223 ✅** | 4889.0490 | +3993.7222 | Higher | +258.6955 | Higher | -3735.0267 | Lower |
| Halstead Effort | 441494.7859 | **27289.2322 ✅** | 437841.4958 | +414205.5537 | Higher | +3653.2902 | Higher | -410552.2636 | Lower |
| SLOC | 145 | **53 ✅** | 146 | +92 | Higher | -1 | Lower | -93 | Lower |
| PLOC | 121 | **37 ✅** | 118 | +84 | Higher | +3 | Higher | -81 | Lower |
| LLOC | 82 | **20 ✅** | 92 | +62 | Higher | -10 | Lower | -72 | Lower |
| Comments (CLOC) | 4 | 10 | **11 ✅** | -6 | Lower | -7 | Lower | -1 | Lower |
| Blank Lines | 20 | **8 ✅** | 24 | +12 | Higher | -4 | Lower | -16 | Lower |
| Functions | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 5 | **0 ✅** | 5 | +5 | Higher | 0 | Same | -5 | Lower |

## p00792

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.7425 | **57.6186 ✅** | 53.3229 | -0.8761 | Lower | +3.4196 | Higher | +4.2957 | Higher |
| MI (SEI) | 20.8486 | **43.3042 ✅** | 18.2438 | -22.4556 | Lower | +2.6047 | Higher | +25.0604 | Higher |
| Cyclomatic (sum) | 18 | 18 | 22 | 0 | Same | -4 | Lower | -4 | Lower |
| Cognitive (sum) | 27 | **17 ✅** | 41 | +10 | Higher | -14 | Lower | -24 | Lower |
| Halstead Volume | 2688 | **1764 ✅** | 2997.2194 | +924 | Higher | -309.2194 | Lower | -1233.2194 | Lower |
| Halstead Effort | 161824.8649 | **59213.1892 ✅** | 174082.6846 | +102611.6757 | Higher | -12257.8198 | Lower | -114869.4955 | Lower |
| SLOC | **71 ✅** | 77 | 80 | -6 | Lower | -9 | Lower | -3 | Lower |
| PLOC | 59 | **54 ✅** | 69 | +5 | Higher | -10 | Lower | -15 | Lower |
| LLOC | 54 | **40 ✅** | 60 | +14 | Higher | -6 | Lower | -20 | Lower |
| Comments (CLOC) | 2 | **18 ✅** | 3 | -16 | Lower | -1 | Lower | +15 | Higher |
| Blank Lines | 12 | 12 | **10 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00799

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.5217 | **63.3453 ✅** | 47.8498 | -17.8236 | Lower | -2.3281 | Lower | +15.4955 | Higher |
| MI (SEI) | 0.6262 | **44.9407 ✅** | 8.0056 | -44.3144 | Lower | -7.3794 | Lower | +36.9351 | Higher |
| Cyclomatic (sum) | 33 | **14 ✅** | 22 | +19 | Higher | +11 | Higher | -8 | Lower |
| Cognitive (sum) | 81 | **23 ✅** | 55 | +58 | Higher | +26 | Higher | -32 | Lower |
| Halstead Volume | 2895.4406 | **1374.7009 ✅** | 2848.5800 | +1520.7397 | Higher | +46.8606 | Higher | -1473.8791 | Lower |
| Halstead Effort | 145185.6640 | **36085.8989 ✅** | 189227.0984 | +109099.7651 | Higher | -44041.4344 | Lower | -153141.1995 | Lower |
| SLOC | 112 | **62 ✅** | 114 | +50 | Higher | -2 | Lower | -52 | Lower |
| PLOC | 101 | **47 ✅** | 100 | +54 | Higher | +1 | Higher | -53 | Lower |
| LLOC | 61 | **31 ✅** | 76 | +30 | Higher | -15 | Lower | -45 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 3 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 11 | **9 ✅** | 14 | +2 | Higher | -3 | Lower | -5 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00811

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.2738 | **61.6838 ✅** | 58.2751 | -5.4100 | Lower | -2.0013 | Lower | +3.4087 | Higher |
| MI (SEI) | 22.9678 | **34.7974 ✅** | 19.4415 | -11.8296 | Lower | +3.5263 | Higher | +15.3559 | Higher |
| Cyclomatic (sum) | 21 | 20 | **17 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Cognitive (sum) | 27 | 28 | **25 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Halstead Volume | 2264.4103 | **1607.2298 ✅** | 2393.5869 | +657.1805 | Higher | -129.1766 | Lower | -786.3571 | Lower |
| Halstead Effort | 143490.3525 | **66821.7973 ✅** | 153552.2274 | +76668.5552 | Higher | -10061.8748 | Lower | -86730.4301 | Lower |
| SLOC | 74 | **60 ✅** | 68 | +14 | Higher | +6 | Higher | -8 | Lower |
| PLOC | 61 | **48 ✅** | 58 | +13 | Higher | +3 | Higher | -10 | Lower |
| LLOC | 44 | **28 ✅** | 50 | +16 | Higher | -6 | Lower | -22 | Lower |
| Comments (CLOC) | 3 | **4 ✅** | 1 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 13 | **9 ✅** | 10 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p00816

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.0718 | **67.3990 ✅** | 53.9768 | -15.3272 | Lower | -1.9051 | Lower | +13.4221 | Higher |
| MI (SEI) | 13.7442 | **46.1725 ✅** | 27.2119 | -32.4283 | Lower | -13.4678 | Lower | +18.9606 | Higher |
| Cyclomatic (sum) | 23 | **13 ✅** | 20 | +10 | Higher | +3 | Higher | -7 | Lower |
| Cognitive (sum) | 24 | **15 ✅** | 28 | +9 | Higher | -4 | Lower | -13 | Lower |
| Halstead Volume | 3377.4695 | **1210.9126 ✅** | 2480.3099 | +2166.5569 | Higher | +897.1597 | Higher | -1269.3973 | Lower |
| Halstead Effort | 132594.7940 | **36162.2527 ✅** | 122948.3825 | +96432.5413 | Higher | +9646.4116 | Higher | -86786.1298 | Lower |
| SLOC | 82 | **51 ✅** | 84 | +31 | Higher | -2 | Lower | -33 | Lower |
| PLOC | 69 | **39 ✅** | 74 | +30 | Higher | -5 | Lower | -35 | Lower |
| LLOC | 46 | **20 ✅** | 53 | +26 | Higher | -7 | Lower | -33 | Lower |
| Comments (CLOC) | 2 | 5 | **8 ✅** | -3 | Lower | -6 | Lower | -3 | Lower |
| Blank Lines | 12 | **7 ✅** | 8 | +5 | Higher | +4 | Higher | -1 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

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

## p00837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.2712 | **66.4378 ✅** | 45.4185 | -21.1667 | Lower | -0.1474 | Lower | +21.0193 | Higher |
| MI (SEI) | 5.8629 | **53.2463 ✅** | -1.2513 | -47.3833 | Lower | +7.1142 | Higher | +54.4976 | Higher |
| Cyclomatic (sum) | 22 | **14 ✅** | 19 | +8 | Higher | +3 | Higher | -5 | Lower |
| Cognitive (sum) | 42 | **22 ✅** | 39 | +20 | Higher | +3 | Higher | -17 | Lower |
| Halstead Volume | 4091.7968 | **1393.7233 ✅** | 4202.8909 | +2698.0736 | Higher | -111.0940 | Lower | -2809.1676 | Lower |
| Halstead Effort | 341409.2992 | **43717.8198 ✅** | 385590.8024 | +297691.4794 | Higher | -44181.5032 | Lower | -341872.9826 | Lower |
| SLOC | 119 | **51 ✅** | 122 | +68 | Higher | -3 | Lower | -71 | Lower |
| PLOC | 101 | **35 ✅** | 105 | +66 | Higher | -4 | Lower | -70 | Lower |
| LLOC | 101 | **22 ✅** | 108 | +79 | Higher | -7 | Lower | -86 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 1 | -6 | Lower | +3 | Higher | +9 | Higher |
| Blank Lines | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |

## p00846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **48.2257 ✅** | 46.3679 | 47.6452 | +1.8578 | Higher | +0.5805 | Higher | -1.2773 | Lower |
| MI (SEI) | 6.5188 | **23.5857 ✅** | 12.8033 | -17.0668 | Lower | -6.2844 | Lower | +10.7824 | Higher |
| Cyclomatic (sum) | 19 | 29 | **18 ✅** | -10 | Lower | +1 | Higher | +11 | Higher |
| Cognitive (sum) | 19 | 21 | **17 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Halstead Volume | 4150.9451 | **3196.0168 ✅** | 4308.1601 | +954.9283 | Higher | -157.2149 | Lower | -1112.1432 | Lower |
| Halstead Effort | 214352.0845 | **180998.9128 ✅** | 233752.4260 | +33353.1717 | Higher | -19400.3415 | Lower | -52753.5132 | Lower |
| SLOC | **103 ✅** | 109 | 107 | -6 | Lower | -4 | Lower | +2 | Higher |
| PLOC | 88 | **78 ✅** | 87 | +10 | Higher | +1 | Higher | -9 | Lower |
| LLOC | 50 | **37 ✅** | 54 | +13 | Higher | -4 | Lower | -17 | Lower |
| Comments (CLOC) | 2 | **18 ✅** | 6 | -16 | Lower | -4 | Lower | +12 | Higher |
| Blank Lines | **15 ✅** | 18 | 17 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **3 ✅** | 10 | 5 | -7 | Lower | -2 | Lower | +5 | Higher |

## p00855

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.4005 | 64.2118 | **73.8658 ✅** | +8.1887 | Higher | -1.4653 | Lower | -9.6540 | Lower |
| MI (SEI) | 61.6820 | 38.4535 | **68.1803 ✅** | +23.2284 | Higher | -6.4984 | Lower | -29.7268 | Lower |
| Cyclomatic (sum) | 13 | 17 | **10 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |
| Cognitive (sum) | 12 | 16 | **10 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Halstead Volume | 913.4548 | 1254.4146 | **849.8521 ✅** | -340.9597 | Lower | +63.6027 | Higher | +404.5625 | Higher |
| Halstead Effort | 28018.4704 | 43120.5012 | **27523.6190 ✅** | -15102.0308 | Lower | +494.8514 | Higher | +15596.8822 | Higher |
| SLOC | 41 | 58 | **40 ✅** | -17 | Lower | +1 | Higher | +18 | Higher |
| PLOC | 32 | 49 | **31 ✅** | -17 | Lower | +1 | Higher | +18 | Higher |
| LLOC | **19 ✅** | 25 | 23 | -6 | Lower | -4 | Lower | +2 | Higher |
| Comments (CLOC) | 8 | 4 | **11 ✅** | +4 | Higher | -3 | Lower | -7 | Lower |
| Blank Lines | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 3 | 7 | **2 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |

## p00856

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.9436 | 53.5576 | **54.1230 ✅** | +0.3861 | Higher | -0.1794 | Lower | -0.5654 | Lower |
| MI (SEI) | 14.1154 | **34.6662 ✅** | 24.7507 | -20.5509 | Lower | -10.6353 | Lower | +9.9155 | Higher |
| Cyclomatic (sum) | 28 | **23 ✅** | 26 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 21 | 29 | **17 ✅** | -8 | Lower | +4 | Higher | +12 | Higher |
| Halstead Volume | 2958.6623 | **2538.0485 ✅** | 2863.8334 | +420.6138 | Higher | +94.8289 | Higher | -325.7849 | Lower |
| Halstead Effort | 195600.4553 | **113402.1692 ✅** | 169339.7157 | +82198.2860 | Higher | +26260.7396 | Higher | -55937.5465 | Lower |
| SLOC | **71 ✅** | 82 | 73 | -11 | Lower | -2 | Lower | +9 | Higher |
| PLOC | 61 | 59 | **58 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| LLOC | 39 | **37 ✅** | 40 | +2 | Higher | -1 | Lower | -3 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 5 | -14 | Lower | -4 | Lower | +10 | Higher |
| Blank Lines | 10 | 13 | 10 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 7 | **3 ✅** | 7 | +4 | Higher | 0 | Same | -4 | Lower |

## p00866

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.2523 | **62.1310 ✅** | 49.4957 | -11.8786 | Lower | +0.7566 | Higher | +12.6352 | Higher |
| MI (SEI) | 6.5252 | **47.0762 ✅** | 5.2542 | -40.5510 | Lower | +1.2710 | Higher | +41.8221 | Higher |
| Cyclomatic (sum) | 19 | **14 ✅** | 18 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 34 | **12 ✅** | 32 | +22 | Higher | +2 | Higher | -20 | Lower |
| Halstead Volume | 3282.5874 | **1826.5263 ✅** | 3726.3237 | +1456.0611 | Higher | -443.7362 | Lower | -1899.7973 | Lower |
| Halstead Effort | 202806.8159 | **57917.8759 ✅** | 226149.2990 | +144888.9400 | Higher | -23342.4831 | Lower | -168231.4231 | Lower |
| SLOC | 98 | **61 ✅** | 100 | +37 | Higher | -2 | Lower | -39 | Lower |
| PLOC | 86 | **41 ✅** | 88 | +45 | Higher | -2 | Lower | -47 | Lower |
| LLOC | 53 | **28 ✅** | 54 | +25 | Higher | -1 | Lower | -26 | Lower |
| Comments (CLOC) | 1 | **12 ✅** | 1 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | **0 ✅** | 4 | 1 | -4 | Lower | -1 | Lower | +3 | Higher |

## p00867

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 14.4699 | **47.5786 ✅** | 9.4550 | -33.1088 | Lower | +5.0149 | Higher | +38.1236 | Higher |
| MI (SEI) | -34.9657 | **27.1799 ✅** | -46.9960 | -62.1455 | Lower | +12.0303 | Higher | +74.1758 | Higher |
| Cyclomatic (sum) | 79 | **28 ✅** | 86 | +51 | Higher | -7 | Lower | -58 | Lower |
| Cognitive (sum) | 100 | **24 ✅** | 101 | +76 | Higher | -1 | Lower | -77 | Lower |
| Halstead Volume | 11720.3674 | **3571.8894 ✅** | 13266.8706 | +8148.4780 | Higher | -1546.5033 | Lower | -9694.9813 | Lower |
| Halstead Effort | 1754199.7214 | **228243.7310 ✅** | 2078476.4006 | +1525955.9904 | Higher | -324276.6792 | Lower | -1850232.6696 | Lower |
| SLOC | 253 | **99 ✅** | 300 | +154 | Higher | -47 | Lower | -201 | Lower |
| PLOC | 227 | **67 ✅** | 265 | +160 | Higher | -38 | Lower | -198 | Lower |
| LLOC | 150 | **40 ✅** | 171 | +110 | Higher | -21 | Lower | -131 | Lower |
| Comments (CLOC) | 6 | **19 ✅** | 2 | -13 | Lower | +4 | Higher | +17 | Higher |
| Blank Lines | 26 | **15 ✅** | 33 | +11 | Higher | -7 | Lower | -18 | Lower |
| Functions | 14 | **5 ✅** | 16 | +9 | Higher | -2 | Lower | -11 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 14 | **9 ✅** | 16 | +5 | Higher | -2 | Lower | -7 | Lower |

## p00870

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.9819 | **74.8090 ✅** | 53.6359 | -15.8271 | Lower | +5.3461 | Higher | +21.1732 | Higher |
| MI (SEI) | 11.1230 | **73.6670 ✅** | 22.2726 | -62.5439 | Lower | -11.1496 | Lower | +51.3943 | Higher |
| Cyclomatic (sum) | 17 | **9 ✅** | 20 | +8 | Higher | -3 | Lower | -11 | Lower |
| Cognitive (sum) | 42 | **11 ✅** | 38 | +31 | Higher | +4 | Higher | -27 | Lower |
| Halstead Volume | 2188.0707 | **591.4629 ✅** | 2749.0881 | +1596.6078 | Higher | -561.0174 | Lower | -2157.6252 | Lower |
| Halstead Effort | 117608.7986 | **18963.7779 ✅** | 133155.2994 | +98645.0207 | Higher | -15546.5009 | Lower | -114191.5215 | Lower |
| SLOC | 67 | **43 ✅** | 83 | +24 | Higher | -16 | Lower | -40 | Lower |
| PLOC | 58 | **22 ✅** | 66 | +36 | Higher | -8 | Lower | -44 | Lower |
| LLOC | 47 | **16 ✅** | 51 | +31 | Higher | -4 | Lower | -35 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 5 | -16 | Lower | -5 | Lower | +11 | Higher |
| Blank Lines | 9 | **5 ✅** | 12 | +4 | Higher | -3 | Lower | -7 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00874

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.1100 | 61.1426 | **72.0491 ✅** | +4.9674 | Higher | -5.9391 | Lower | -10.9065 | Lower |
| MI (SEI) | **54.8122 ✅** | 46.1476 | 29.1604 | +8.6646 | Higher | +25.6518 | Higher | +16.9871 | Higher |
| Cyclomatic (sum) | 15 | 21 | **9 ✅** | -6 | Lower | +6 | Higher | +12 | Higher |
| Cognitive (sum) | 17 | 39 | **11 ✅** | -22 | Lower | +6 | Higher | +28 | Higher |
| Halstead Volume | 1259.7972 | 1540.6490 | **1166.4637 ✅** | -280.8518 | Lower | +93.3335 | Higher | +374.1853 | Higher |
| Halstead Effort | 33283.0287 | 70869.8541 | **31090.7429 ✅** | -37586.8254 | Lower | +2192.2858 | Higher | +39779.1112 | Higher |
| SLOC | 53 | 62 | **41 ✅** | -9 | Lower | +12 | Higher | +21 | Higher |
| PLOC | 38 | 46 | **32 ✅** | -8 | Lower | +6 | Higher | +14 | Higher |
| LLOC | **22 ✅** | 29 | 25 | -7 | Lower | -3 | Lower | +4 | Higher |
| Comments (CLOC) | 12 | 12 | 0 | 0 | Same | +12 | Higher | +12 | Higher |
| Blank Lines | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00889

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.2047 | **67.4032 ✅** | 44.4772 | -20.1985 | Lower | +2.7275 | Higher | +22.9260 | Higher |
| MI (SEI) | -5.2570 | **54.6384 ✅** | 2.9912 | -59.8954 | Lower | -8.2482 | Lower | +51.6472 | Higher |
| Cyclomatic (sum) | 23 | **9 ✅** | 25 | +14 | Higher | -2 | Lower | -16 | Lower |
| Cognitive (sum) | 44 | **7 ✅** | 44 | +37 | Higher | 0 | Same | -37 | Lower |
| Halstead Volume | 3758.6705 | **1208.5264 ✅** | 3765.8709 | +2550.1442 | Higher | -7.2004 | Lower | -2557.3445 | Lower |
| Halstead Effort | 316582.5676 | **36739.2018 ✅** | 310889.0163 | +279843.3657 | Higher | +5693.5513 | Higher | -274149.8145 | Lower |
| SLOC | 107 | **54 ✅** | 123 | +53 | Higher | -16 | Lower | -69 | Lower |
| PLOC | 97 | **36 ✅** | 107 | +61 | Higher | -10 | Lower | -71 | Lower |
| LLOC | 86 | **24 ✅** | 88 | +62 | Higher | -2 | Lower | -64 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 3 | -11 | Lower | -3 | Lower | +8 | Higher |
| Blank Lines | 10 | 10 | 14 | 0 | Same | -4 | Lower | -4 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |

## p00897

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 34.9528 | **47.1493 ✅** | 21.5254 | -12.1965 | Lower | +13.4274 | Higher | +25.6239 | Higher |
| MI (SEI) | -16.1238 | **15.6605 ✅** | -33.0749 | -31.7844 | Lower | +16.9511 | Higher | +48.7354 | Higher |
| Cyclomatic (sum) | 32 | **23 ✅** | 48 | +9 | Higher | -16 | Lower | -25 | Lower |
| Cognitive (sum) | 34 | **22 ✅** | 62 | +12 | Higher | -28 | Lower | -40 | Lower |
| Halstead Volume | 6069.9593 | **3034.6264 ✅** | 10052.7610 | +3035.3329 | Higher | -3982.8017 | Lower | -7018.1346 | Lower |
| Halstead Effort | 344114.2345 | **109452.2873 ✅** | 1011559.0760 | +234661.9472 | Higher | -667444.8415 | Lower | -902106.7887 | Lower |
| SLOC | 172 | **115 ✅** | 267 | +57 | Higher | -95 | Lower | -152 | Lower |
| PLOC | 142 | **93 ✅** | 223 | +49 | Higher | -81 | Lower | -130 | Lower |
| LLOC | 89 | **35 ✅** | 182 | +54 | Higher | -93 | Lower | -147 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 2 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 29 | **19 ✅** | 42 | +10 | Higher | -13 | Lower | -23 | Lower |
| Functions | 9 | **7 ✅** | 12 | +2 | Higher | -3 | Lower | -5 | Lower |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |

## p00907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.3037 | **62.3383 ✅** | 61.8705 | -5.0347 | Lower | -4.5668 | Lower | +0.4679 | Higher |
| MI (SEI) | 25.3096 | **51.7393 ✅** | 29.5501 | -26.4296 | Lower | -4.2405 | Lower | +22.1892 | Higher |
| Cyclomatic (sum) | 22 | **13 ✅** | 20 | +9 | Higher | +2 | Higher | -7 | Lower |
| Cognitive (sum) | 36 | **12 ✅** | 33 | +24 | Higher | +3 | Higher | -21 | Lower |
| Halstead Volume | 2422.0514 | **1435.2384 ✅** | 1550.5638 | +986.8129 | Higher | +871.4876 | Higher | -115.3253 | Lower |
| Halstead Effort | 81590.8369 | **48711.1227 ✅** | 79940.1763 | +32879.7143 | Higher | +1650.6606 | Higher | -31229.0536 | Lower |
| SLOC | 67 | 66 | **60 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |
| PLOC | 56 | **44 ✅** | 52 | +12 | Higher | +4 | Higher | -8 | Lower |
| LLOC | 37 | **28 ✅** | 38 | +9 | Higher | -1 | Lower | -10 | Lower |
| Comments (CLOC) | 3 | **18 ✅** | 2 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 10 | 12 | **7 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p00924

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.9628 | **72.3597 ✅** | 51.6569 | -23.3969 | Lower | -2.6941 | Lower | +20.7028 | Higher |
| MI (SEI) | 8.7241 | **67.0832 ✅** | 22.3492 | -58.3591 | Lower | -13.6251 | Lower | +44.7340 | Higher |
| Cyclomatic (sum) | 23 | **12 ✅** | 21 | +11 | Higher | +2 | Higher | -9 | Lower |
| Cognitive (sum) | 25 | **8 ✅** | 25 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 4595.0789 | **962.2906 ✅** | 3445.2078 | +3632.7882 | Higher | +1149.8711 | Higher | -2482.9172 | Lower |
| Halstead Effort | 179990.2164 | **27064.4241 ✅** | 229400.4214 | +152925.7923 | Higher | -49410.2050 | Lower | -202335.9972 | Lower |
| SLOC | 90 | **41 ✅** | 86 | +49 | Higher | +4 | Higher | -45 | Lower |
| PLOC | 78 | **27 ✅** | 72 | +51 | Higher | +6 | Higher | -45 | Lower |
| LLOC | 59 | **17 ✅** | 61 | +42 | Higher | -2 | Lower | -44 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 7 | -10 | Lower | -5 | Lower | +5 | Higher |
| Blank Lines | 12 | **6 ✅** | 9 | +6 | Higher | +3 | Higher | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 8 | **1 ✅** | 8 | +7 | Higher | 0 | Same | -7 | Lower |

## p00949

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.8887 | **62.7021 ✅** | 43.0834 | -15.8134 | Lower | +3.8053 | Higher | +19.6187 | Higher |
| MI (SEI) | -5.8148 | **47.5828 ✅** | 8.5779 | -53.3976 | Lower | -14.3927 | Lower | +39.0049 | Higher |
| Cyclomatic (sum) | 22 | **13 ✅** | 24 | +9 | Higher | -2 | Lower | -11 | Lower |
| Cognitive (sum) | 22 | **15 ✅** | 29 | +7 | Higher | -7 | Lower | -14 | Lower |
| Halstead Volume | 3723.7648 | **1626.0594 ✅** | 4129.8801 | +2097.7054 | Higher | -406.1153 | Lower | -2503.8207 | Lower |
| Halstead Effort | 264048.7766 | **71911.0742 ✅** | 319877.9849 | +192137.7024 | Higher | -55829.2083 | Lower | -247966.9106 | Lower |
| SLOC | 111 | **62 ✅** | 132 | +49 | Higher | -21 | Lower | -70 | Lower |
| PLOC | 94 | **40 ✅** | 109 | +54 | Higher | -15 | Lower | -69 | Lower |
| LLOC | 77 | **30 ✅** | 88 | +47 | Higher | -11 | Lower | -58 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 9 | -12 | Lower | -9 | Lower | +3 | Higher |
| Blank Lines | 17 | **12 ✅** | 18 | +5 | Higher | -1 | Lower | -6 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | **1 ✅** | 2 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |

## p00971

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.9312 | **58.0971 ✅** | 52.5472 | -12.1659 | Lower | -6.6159 | Lower | +5.5499 | Higher |
| MI (SEI) | 2.9564 | **43.2896 ✅** | 1.5341 | -40.3332 | Lower | +1.4223 | Higher | +41.7555 | Higher |
| Cyclomatic (sum) | 18 | 16 | **14 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | **13 ✅** | 16 | 14 | -3 | Lower | -1 | Lower | +2 | Higher |
| Halstead Volume | 6167.9863 | **2263.1613 ✅** | 3555.7945 | +3904.8250 | Higher | +2612.1918 | Higher | -1292.6331 | Lower |
| Halstead Effort | 236763.2542 | **140462.0126 ✅** | 318766.5155 | +96301.2416 | Higher | -82003.2613 | Lower | -178304.5029 | Lower |
| SLOC | 106 | **71 ✅** | 89 | +35 | Higher | +17 | Higher | -18 | Lower |
| PLOC | 90 | **48 ✅** | 78 | +42 | Higher | +12 | Higher | -30 | Lower |
| LLOC | 69 | **44 ✅** | 71 | +25 | Higher | -2 | Lower | -27 | Lower |
| Comments (CLOC) | 2 | **16 ✅** | 0 | -14 | Lower | +2 | Higher | +16 | Higher |
| Blank Lines | 15 | 12 | **11 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| Functions | 5 | **1 ✅** | 3 | +4 | Higher | +2 | Higher | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | **0 ✅** | 2 | +4 | Higher | +2 | Higher | -2 | Lower |

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

## p01065

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.4728 | **77.5161 ✅** | 58.4835 | -18.0433 | Lower | +0.9893 | Higher | +19.0325 | Higher |
| MI (SEI) | 28.4141 | **71.2884 ✅** | 10.6076 | -42.8743 | Lower | +17.8065 | Higher | +60.6808 | Higher |
| Cyclomatic (sum) | 17 | **6 ✅** | 19 | +11 | Higher | -2 | Lower | -13 | Lower |
| Cognitive (sum) | 31 | **6 ✅** | 34 | +25 | Higher | -3 | Lower | -28 | Lower |
| Halstead Volume | 2411.8837 | **915.3716 ✅** | 2542.4418 | +1496.5120 | Higher | -130.5582 | Lower | -1627.0702 | Lower |
| Halstead Effort | 157722.5732 | **24340.5639 ✅** | 156995.7827 | +133382.0094 | Higher | +726.7906 | Higher | -132655.2188 | Lower |
| SLOC | 63 | **33 ✅** | 64 | +30 | Higher | -1 | Lower | -31 | Lower |
| PLOC | 52 | **23 ✅** | 56 | +29 | Higher | -4 | Lower | -33 | Lower |
| LLOC | 34 | **12 ✅** | 41 | +22 | Higher | -7 | Lower | -29 | Lower |
| Comments (CLOC) | 3 | **8 ✅** | 0 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 8 | **4 ✅** | 8 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p01106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.6160 | **77.2670 ✅** | 68.8888 | -7.6510 | Lower | +0.7272 | Higher | +8.3782 | Higher |
| MI (SEI) | 54.8206 | **68.9094 ✅** | 59.8642 | -14.0888 | Lower | -5.0436 | Lower | +9.0452 | Higher |
| Cyclomatic (sum) | 11 | **8 ✅** | 9 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 17 | +8 | Higher | 0 | Same | -8 | Lower |
| Halstead Volume | 1368.1055 | **800.9165 ✅** | 1399.6763 | +567.1890 | Higher | -31.5708 | Lower | -598.7599 | Lower |
| Halstead Effort | 43401.9686 | **14478.1057 ✅** | 52972.3659 | +28923.8629 | Higher | -9570.3973 | Lower | -38494.2602 | Lower |
| SLOC | 44 | **34 ✅** | 47 | +10 | Higher | -3 | Lower | -13 | Lower |
| PLOC | 34 | **25 ✅** | 37 | +9 | Higher | -3 | Lower | -12 | Lower |
| LLOC | 23 | **13 ✅** | 31 | +10 | Higher | -8 | Lower | -18 | Lower |
| Comments (CLOC) | 7 | 7 | **12 ✅** | 0 | Same | -5 | Lower | -5 | Lower |
| Blank Lines | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01119

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.7252 | **66.6473 ✅** | 32.7177 | -13.9220 | Lower | +20.0076 | Higher | +33.9296 | Higher |
| MI (SEI) | 13.7448 | **63.6879 ✅** | -14.6264 | -49.9431 | Lower | +28.3712 | Higher | +78.3143 | Higher |
| Cyclomatic (sum) | 19 | **13 ✅** | 40 | +6 | Higher | -21 | Lower | -27 | Lower |
| Cognitive (sum) | 59 | **8 ✅** | 67 | +51 | Higher | -8 | Lower | -59 | Lower |
| Halstead Volume | 2660.1088 | **1399.2361 ✅** | 5219.2513 | +1260.8727 | Higher | -2559.1425 | Lower | -3820.0152 | Lower |
| Halstead Effort | 104140.4308 | **52430.2010 ✅** | 356424.3550 | +51710.2298 | Higher | -252283.9242 | Lower | -303994.1540 | Lower |
| SLOC | 90 | **51 ✅** | 185 | +39 | Higher | -95 | Lower | -134 | Lower |
| PLOC | 78 | **33 ✅** | 166 | +45 | Higher | -88 | Lower | -133 | Lower |
| LLOC | 70 | **21 ✅** | 123 | +49 | Higher | -53 | Lower | -102 | Lower |
| Comments (CLOC) | 2 | **21 ✅** | 3 | -19 | Lower | -1 | Lower | +18 | Higher |
| Blank Lines | 12 | **10 ✅** | 17 | +2 | Higher | -5 | Lower | -7 | Lower |
| Functions | **1 ✅** | 2 | 8 | -1 | Lower | -7 | Lower | -6 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 3 | 9 | -3 | Lower | -9 | Lower | -6 | Lower |

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

## p01128

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.2471 | **55.5227 ✅** | 46.0800 | -10.2756 | Lower | -0.8329 | Lower | +9.4427 | Higher |
| MI (SEI) | 6.3088 | **31.4003 ✅** | -6.5741 | -25.0916 | Lower | +12.8828 | Higher | +37.9744 | Higher |
| Cyclomatic (sum) | 25 | **15 ✅** | 26 | +10 | Higher | -1 | Lower | -11 | Lower |
| Cognitive (sum) | 38 | **16 ✅** | 37 | +22 | Higher | +1 | Higher | -21 | Lower |
| Halstead Volume | 3897.9808 | **2298.4811 ✅** | 3969.6291 | +1599.4997 | Higher | -71.6483 | Lower | -1671.1480 | Lower |
| Halstead Effort | 213623.2700 | **60574.5548 ✅** | 247949.1419 | +153048.7152 | Higher | -34325.8719 | Lower | -187374.5871 | Lower |
| SLOC | 116 | **84 ✅** | 108 | +32 | Higher | +8 | Higher | -24 | Lower |
| PLOC | 98 | **62 ✅** | 97 | +36 | Higher | +1 | Higher | -35 | Lower |
| LLOC | 56 | **28 ✅** | 62 | +28 | Higher | -6 | Lower | -34 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 0 | -6 | Lower | +4 | Higher | +10 | Higher |
| Blank Lines | 18 | 14 | **11 ✅** | +4 | Higher | +7 | Higher | +3 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | **1 ✅** | 2 | 5 | -1 | Lower | -4 | Lower | -3 | Lower |

## p01138

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.6426 | **66.8349 ✅** | 59.1130 | -1.1923 | Lower | +6.5297 | Higher | +7.7219 | Higher |
| MI (SEI) | 47.7707 | **51.0924 ✅** | 36.0369 | -3.3217 | Lower | +11.7339 | Higher | +15.0556 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 11 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 6 | 13 | **5 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Halstead Volume | 1601.2991 | **1409.0684 ✅** | 2656.6166 | +192.2306 | Higher | -1055.3175 | Lower | -1247.5482 | Lower |
| Halstead Effort | 50040.5963 | **36999.4098 ✅** | 86094.0566 | +13041.1865 | Higher | -36053.4603 | Lower | -49094.6468 | Lower |
| SLOC | 55 | **54 ✅** | 68 | +1 | Higher | -13 | Lower | -14 | Lower |
| PLOC | 40 | **37 ✅** | 51 | +3 | Higher | -11 | Lower | -14 | Lower |
| LLOC | **25 ✅** | 32 | 26 | -7 | Lower | -1 | Lower | +6 | Higher |
| Comments (CLOC) | 8 | **9 ✅** | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 10 | 10 | 13 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p01150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 39.8820 | **65.0009 ✅** | 24.6466 | -25.1189 | Lower | +15.2354 | Higher | +40.3543 | Higher |
| MI (SEI) | -14.1923 | **52.9725 ✅** | -18.3439 | -67.1648 | Lower | +4.1515 | Higher | +71.3163 | Higher |
| Cyclomatic (sum) | 39 | **14 ✅** | 60 | +25 | Higher | -21 | Lower | -46 | Lower |
| Cognitive (sum) | 41 | **11 ✅** | 76 | +30 | Higher | -35 | Lower | -65 | Lower |
| Halstead Volume | 4440.3773 | **1230.7398 ✅** | 6655.5412 | +3209.6375 | Higher | -2215.1639 | Lower | -5424.8014 | Lower |
| Halstead Effort | 547246.4981 | **43967.1181 ✅** | 958397.9280 | +503279.3800 | Higher | -411151.4298 | Lower | -914430.8098 | Lower |
| SLOC | 127 | **58 ✅** | 212 | +69 | Higher | -85 | Lower | -154 | Lower |
| PLOC | 111 | **42 ✅** | 187 | +69 | Higher | -76 | Lower | -145 | Lower |
| LLOC | 64 | **17 ✅** | 127 | +47 | Higher | -63 | Lower | -110 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 9 | -13 | Lower | -9 | Lower | +4 | Higher |
| Blank Lines | 16 | **4 ✅** | 19 | +12 | Higher | -3 | Lower | -15 | Lower |
| Functions | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |

## p01153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.0902 | 65.6289 | **70.1983 ✅** | +4.4613 | Higher | -0.1081 | Lower | -4.5694 | Lower |
| MI (SEI) | 57.0130 | 56.8479 | **60.1510 ✅** | +0.1651 | Higher | -3.1381 | Lower | -3.3032 | Lower |
| Cyclomatic (sum) | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 15 | **4 ✅** | 17 | +11 | Higher | -2 | Lower | -13 | Lower |
| Halstead Volume | 1364.3745 | **1290.1263 ✅** | 1336.3009 | +74.2482 | Higher | +28.0735 | Higher | -46.1746 | Lower |
| Halstead Effort | 49484.8133 | **41140.6954 ✅** | 43095.7054 | +8344.1179 | Higher | +6389.1079 | Higher | -1955.0100 | Lower |
| SLOC | 44 | 59 | 44 | -15 | Lower | 0 | Same | +15 | Higher |
| PLOC | 31 | 35 | **29 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| LLOC | **22 ✅** | 23 | 23 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | 8 | **17 ✅** | 10 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p01184

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 35.2098 | **60.3098 ✅** | 33.9059 | -25.0999 | Lower | +1.3040 | Higher | +26.4039 | Higher |
| MI (SEI) | -15.1389 | **39.7236 ✅** | -14.6729 | -54.8626 | Lower | -0.4660 | Lower | +54.3965 | Higher |
| Cyclomatic (sum) | 37 | **15 ✅** | 38 | +22 | Higher | -1 | Lower | -23 | Lower |
| Cognitive (sum) | 64 | **23 ✅** | 63 | +41 | Higher | +1 | Higher | -40 | Lower |
| Halstead Volume | 5172.7031 | **1851.7551 ✅** | 5394.8112 | +3320.9480 | Higher | -222.1081 | Lower | -3543.0562 | Lower |
| Halstead Effort | 414483.6934 | **55506.3577 ✅** | 477706.1121 | +358977.3357 | Higher | -63222.4187 | Lower | -422199.7544 | Lower |
| SLOC | 166 | **67 ✅** | 175 | +99 | Higher | -9 | Lower | -108 | Lower |
| PLOC | 142 | **50 ✅** | 149 | +92 | Higher | -7 | Lower | -99 | Lower |
| LLOC | 101 | **29 ✅** | 110 | +72 | Higher | -9 | Lower | -81 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 2 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 24 | **11 ✅** | 24 | +13 | Higher | 0 | Same | -13 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 7 | **1 ✅** | 9 | +6 | Higher | -2 | Lower | -8 | Lower |

## p01255

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 29.4336 | **61.5929 ✅** | 7.2559 | -32.1593 | Lower | +22.1777 | Higher | +54.3370 | Higher |
| MI (SEI) | -20.9403 | **44.3358 ✅** | -56.5782 | -65.2761 | Lower | +35.6379 | Higher | +100.9140 | Higher |
| Cyclomatic (sum) | 45 | **14 ✅** | 85 | +31 | Higher | -40 | Lower | -71 | Lower |
| Cognitive (sum) | 125 | **17 ✅** | 160 | +108 | Higher | -35 | Lower | -143 | Lower |
| Halstead Volume | 6171.1050 | **1662 ✅** | 13330.1043 | +4509.1050 | Higher | -7158.9993 | Lower | -11668.1043 | Lower |
| Halstead Effort | 393716.4978 | **48364.2000 ✅** | 1557680.9201 | +345352.2978 | Higher | -1163964.4224 | Lower | -1509316.7201 | Lower |
| SLOC | 200 | **65 ✅** | 348 | +135 | Higher | -148 | Lower | -283 | Lower |
| PLOC | 177 | **50 ✅** | 320 | +127 | Higher | -143 | Lower | -270 | Lower |
| LLOC | 105 | **31 ✅** | 197 | +74 | Higher | -92 | Lower | -166 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 0 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 23 | **12 ✅** | 28 | +11 | Higher | -5 | Lower | -16 | Lower |
| Functions | 5 | **3 ✅** | 17 | +2 | Higher | -12 | Lower | -14 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 6 | **2 ✅** | 27 | +4 | Higher | -21 | Lower | -25 | Lower |

## p01275

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.5058 | **60.6797 ✅** | 48.9444 | -3.1739 | Lower | +8.5614 | Higher | +11.7353 | Higher |
| MI (SEI) | 27.0692 | **44.5523 ✅** | 21.2866 | -17.4831 | Lower | +5.7826 | Higher | +23.2657 | Higher |
| Cyclomatic (sum) | 18 | 18 | 23 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 17 | **10 ✅** | 24 | +7 | Higher | -7 | Lower | -14 | Lower |
| Halstead Volume | 2321.0473 | **1659.8363 ✅** | 2852.7990 | +661.2111 | Higher | -531.7516 | Lower | -1192.9627 | Lower |
| Halstead Effort | 114118.1610 | **59656.4678 ✅** | 150744.4905 | +54461.6931 | Higher | -36626.3295 | Lower | -91088.0227 | Lower |
| SLOC | 71 | **65 ✅** | 105 | +6 | Higher | -34 | Lower | -40 | Lower |
| PLOC | 57 | **43 ✅** | 85 | +14 | Higher | -28 | Lower | -42 | Lower |
| LLOC | 36 | **23 ✅** | 54 | +13 | Higher | -18 | Lower | -31 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 11 | -8 | Lower | -7 | Lower | +1 | Higher |
| Blank Lines | **11 ✅** | 13 | 15 | -2 | Lower | -4 | Lower | -2 | Lower |
| Functions | 5 | **3 ✅** | 6 | +2 | Higher | -1 | Lower | -3 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01288

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.9794 | **70.0748 ✅** | 62.5028 | -9.0954 | Lower | -1.5234 | Lower | +7.5720 | Higher |
| MI (SEI) | 47.1564 | **63.5344 ✅** | 15.5917 | -16.3780 | Lower | +31.5648 | Higher | +47.9428 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 11 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 23 | **6 ✅** | 22 | +17 | Higher | +1 | Higher | -16 | Lower |
| Halstead Volume | 1948.1853 | **998.3469 ✅** | 1941.7650 | +949.8384 | Higher | +6.4202 | Higher | -943.4182 | Lower |
| Halstead Effort | 75778.6768 | **34263.2642 ✅** | 92401.2327 | +41515.4126 | Higher | -16622.5558 | Lower | -58137.9684 | Lower |
| SLOC | 66 | **48 ✅** | 61 | +18 | Higher | +5 | Higher | -13 | Lower |
| PLOC | 52 | **31 ✅** | 53 | +21 | Higher | -1 | Lower | -22 | Lower |
| LLOC | 47 | **19 ✅** | 50 | +28 | Higher | -3 | Lower | -31 | Lower |
| Comments (CLOC) | **15 ✅** | 14 | 0 | +1 | Higher | +15 | Higher | +14 | Higher |
| Blank Lines | 10 | **7 ✅** | 8 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p01316

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.5899 | **63.6446 ✅** | 58.6504 | -2.0547 | Lower | +2.9395 | Higher | +4.9942 | Higher |
| MI (SEI) | 34.1528 | **53.9023 ✅** | 28.8654 | -19.7495 | Lower | +5.2875 | Higher | +25.0369 | Higher |
| Cyclomatic (sum) | 15 | 15 | 17 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | 33 | **19 ✅** | 35 | +14 | Higher | -2 | Lower | -16 | Lower |
| Halstead Volume | 2041.5827 | **1241.6484 ✅** | 2127.9117 | +799.9343 | Higher | -86.3290 | Lower | -886.2633 | Lower |
| Halstead Effort | 77835.3400 | **58512.6790 ✅** | 85116.4661 | +19322.6610 | Higher | -7281.1261 | Lower | -26603.7871 | Lower |
| SLOC | **60 ✅** | 62 | 69 | -2 | Lower | -9 | Lower | -7 | Lower |
| PLOC | 47 | **41 ✅** | 57 | +6 | Higher | -10 | Lower | -16 | Lower |
| LLOC | 32 | **27 ✅** | 35 | +5 | Higher | -3 | Lower | -8 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 4 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 12 | **8 ✅** | 12 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 2 | 2 | 4 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |

## p01317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.2192 | **56.5318 ✅** | 50.2723 | -5.3127 | Lower | +0.9469 | Higher | +6.2596 | Higher |
| MI (SEI) | 14.0560 | **33.6173 ✅** | 12.4233 | -19.5613 | Lower | +1.6327 | Higher | +21.1940 | Higher |
| Cyclomatic (sum) | 18 | **14 ✅** | 19 | +4 | Higher | -1 | Lower | -5 | Lower |
| Cognitive (sum) | 46 | **21 ✅** | 41 | +25 | Higher | +5 | Higher | -20 | Lower |
| Halstead Volume | 3846 | **2492.5257 ✅** | 3723.3468 | +1353.4743 | Higher | +122.6532 | Higher | -1230.8210 | Lower |
| Halstead Effort | 232906.6047 | **132407.8301 ✅** | 198950.8295 | +100498.7745 | Higher | +33955.7752 | Higher | -66542.9993 | Lower |
| SLOC | 89 | **78 ✅** | 94 | +11 | Higher | -5 | Lower | -16 | Lower |
| PLOC | 77 | **59 ✅** | 80 | +18 | Higher | -3 | Lower | -21 | Lower |
| LLOC | 67 | **31 ✅** | 69 | +36 | Higher | -2 | Lower | -38 | Lower |
| Comments (CLOC) | 3 | **10 ✅** | 3 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 12 | **11 ✅** | 14 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p01339

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 36.6739 | **47.1982 ✅** | 34.4175 | -10.5243 | Lower | +2.2564 | Higher | +12.7807 | Higher |
| MI (SEI) | -8.7370 | **18.8200 ✅** | -9.5030 | -27.5570 | Lower | +0.7659 | Higher | +28.3229 | Higher |
| Cyclomatic (sum) | 34 | **23 ✅** | 35 | +11 | Higher | -1 | Lower | -12 | Lower |
| Cognitive (sum) | 31 | **22 ✅** | 35 | +9 | Higher | -4 | Lower | -13 | Lower |
| Halstead Volume | 5198.6770 | **3089.1586 ✅** | 5683.6515 | +2109.5184 | Higher | -484.9745 | Lower | -2594.4929 | Lower |
| Halstead Effort | 373005.0749 | **128132.9280 ✅** | 470059.0290 | +244872.1470 | Higher | -97053.9540 | Lower | -341926.1010 | Lower |
| SLOC | 158 | **114 ✅** | 174 | +44 | Higher | -16 | Lower | -60 | Lower |
| PLOC | 141 | **87 ✅** | 155 | +54 | Higher | -14 | Lower | -68 | Lower |
| LLOC | 90 | **45 ✅** | 103 | +45 | Higher | -13 | Lower | -58 | Lower |
| Comments (CLOC) | 3 | **12 ✅** | 5 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 17 | 17 | 17 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.4453 | **62.9206 ✅** | 55.8862 | -4.4753 | Lower | +2.5591 | Higher | +7.0343 | Higher |
| MI (SEI) | 22.2525 | **52.4774 ✅** | 31.8975 | -30.2249 | Lower | -9.6450 | Lower | +20.5799 | Higher |
| Cyclomatic (sum) | **11 ✅** | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 11 | **9 ✅** | 12 | +2 | Higher | -1 | Lower | -3 | Lower |
| Halstead Volume | 2226 | **1341.2426 ✅** | 2638.2415 | +884.7574 | Higher | -412.2415 | Lower | -1296.9989 | Lower |
| Halstead Effort | 128185.8000 | **34024.1693 ✅** | 154370.1066 | +94161.6307 | Higher | -26184.3066 | Lower | -120345.9373 | Lower |
| SLOC | 75 | **66 ✅** | 82 | +9 | Higher | -7 | Lower | -16 | Lower |
| PLOC | 64 | **40 ✅** | 68 | +24 | Higher | -4 | Lower | -28 | Lower |
| LLOC | 49 | **23 ✅** | 54 | +26 | Higher | -5 | Lower | -31 | Lower |
| Comments (CLOC) | 2 | **18 ✅** | 10 | -16 | Lower | -8 | Lower | +8 | Higher |
| Blank Lines | **10 ✅** | 13 | 12 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |

## p01368

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.0391 | **57.2833 ✅** | 43.3287 | -17.2443 | Lower | -3.2896 | Lower | +13.9546 | Higher |
| MI (SEI) | -0.2489 | **34.1575 ✅** | -0.0598 | -34.4063 | Lower | -0.1890 | Lower | +34.2173 | Higher |
| Cyclomatic (sum) | 31 | **13 ✅** | 31 | +18 | Higher | 0 | Same | -18 | Lower |
| Cognitive (sum) | 53 | **15 ✅** | 52 | +38 | Higher | +1 | Higher | -37 | Lower |
| Halstead Volume | 4737.9190 | **2004.5892 ✅** | 3992.6728 | +2733.3298 | Higher | +745.2462 | Higher | -1988.0836 | Lower |
| Halstead Effort | 237539.3724 | **86751.2361 ✅** | 395274.6053 | +150788.1363 | Higher | -157735.2329 | Lower | -308523.3692 | Lower |
| SLOC | 138 | **81 ✅** | 119 | +57 | Higher | +19 | Higher | -38 | Lower |
| PLOC | 117 | **64 ✅** | 104 | +53 | Higher | +13 | Higher | -40 | Lower |
| LLOC | 67 | **31 ✅** | 86 | +36 | Higher | -19 | Lower | -55 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 2 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 16 | **11 ✅** | 13 | +5 | Higher | +3 | Higher | -2 | Lower |
| Functions | 5 | 3 | 3 | +2 | Higher | +2 | Higher | 0 | Same |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 4 | **1 ✅** | 3 | +3 | Higher | +1 | Higher | -2 | Lower |

## p01452

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.9997 | **66.9437 ✅** | 53.5544 | -12.9440 | Lower | +0.4452 | Higher | +13.3892 | Higher |
| MI (SEI) | 19.0972 | **57.9212 ✅** | 22.0661 | -38.8239 | Lower | -2.9689 | Lower | +35.8551 | Higher |
| Cyclomatic (sum) | 19 | **15 ✅** | 17 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 10 | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 3251.4434 | **1286.7992 ✅** | 3440.4658 | +1964.6441 | Higher | -189.0225 | Lower | -2153.6666 | Lower |
| Halstead Effort | 189828.9483 | **54639.4742 ✅** | 202495.9877 | +135189.4741 | Higher | -12667.0394 | Lower | -147856.5135 | Lower |
| SLOC | 78 | **50 ✅** | 81 | +28 | Higher | -3 | Lower | -31 | Lower |
| PLOC | 66 | **42 ✅** | 67 | +24 | Higher | -1 | Lower | -25 | Lower |
| LLOC | 36 | **26 ✅** | 37 | +10 | Higher | -1 | Lower | -11 | Lower |
| Comments (CLOC) | 3 | **13 ✅** | 5 | -10 | Lower | -2 | Lower | +8 | Higher |
| Blank Lines | 12 | **6 ✅** | 13 | +6 | Higher | -1 | Lower | -7 | Lower |
| Functions | 6 | **5 ✅** | 7 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |

## p01459

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.9473 | **45.9022 ✅** | 39.5026 | -3.9549 | Lower | +2.4447 | Higher | +6.3996 | Higher |
| MI (SEI) | 1.6672 | **11.1255 ✅** | -4.0078 | -9.4584 | Lower | +5.6750 | Higher | +15.1334 | Higher |
| Cyclomatic (sum) | 29 | **27 ✅** | 32 | +2 | Higher | -3 | Lower | -5 | Lower |
| Cognitive (sum) | 50 | **27 ✅** | 62 | +23 | Higher | -12 | Lower | -35 | Lower |
| Halstead Volume | 5401.3452 | **3930.0344 ✅** | 5381.7462 | +1471.3107 | Higher | +19.5990 | Higher | -1451.7118 | Lower |
| Halstead Effort | 319293.1547 | **225509.1194 ✅** | 390176.6007 | +93784.0354 | Higher | -70883.4460 | Lower | -164667.4814 | Lower |
| SLOC | 121 | **108 ✅** | 135 | +13 | Higher | -14 | Lower | -27 | Lower |
| PLOC | 101 | **88 ✅** | 114 | +13 | Higher | -13 | Lower | -26 | Lower |
| LLOC | 71 | **39 ✅** | 90 | +32 | Higher | -19 | Lower | -51 | Lower |
| Comments (CLOC) | 4 | **6 ✅** | 3 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 20 | **16 ✅** | 18 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 7 | 1 | -6 | Lower | 0 | Same | +6 | Higher |

## p01471

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.9196 ✅** | 56.2389 | 56.5021 | +6.6807 | Higher | +6.4174 | Higher | -0.2633 | Lower |
| MI (SEI) | **44.2209 ✅** | 36.5183 | 16.1625 | +7.7026 | Higher | +28.0584 | Higher | +20.3559 | Higher |
| Cyclomatic (sum) | **11 ✅** | 15 | 17 | -4 | Lower | -6 | Lower | -2 | Lower |
| Cognitive (sum) | 15 | **13 ✅** | 19 | +2 | Higher | -4 | Lower | -6 | Lower |
| Halstead Volume | **1792.2131 ✅** | 2002.7753 | 2028.7927 | -210.5622 | Lower | -236.5796 | Lower | -26.0174 | Lower |
| Halstead Effort | **67155.2787 ✅** | 70097.1346 | 86099.9820 | -2941.8559 | Lower | -18944.7033 | Lower | -16002.8474 | Lower |
| SLOC | **61 ✅** | 84 | 80 | -23 | Lower | -19 | Lower | +4 | Higher |
| PLOC | **49 ✅** | 64 | 70 | -15 | Lower | -21 | Lower | -6 | Lower |
| LLOC | **33 ✅** | 35 | 43 | -2 | Lower | -10 | Lower | -8 | Lower |
| Comments (CLOC) | 9 | **14 ✅** | 1 | -5 | Lower | +8 | Higher | +13 | Higher |
| Blank Lines | **9 ✅** | 13 | 10 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | **1 ✅** | 3 | 4 | -2 | Lower | -3 | Lower | -1 | Lower |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 3 | 3 | -3 | Lower | -3 | Lower | 0 | Same |

## p01521

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **84.4364 ✅** | 84.2560 | 82.9755 | +0.1804 | Higher | +1.4610 | Higher | +1.2806 | Higher |
| MI (SEI) | 74.9065 | **82.0434 ✅** | 80.0941 | -7.1370 | Lower | -5.1876 | Lower | +1.9493 | Higher |
| Cyclomatic (sum) | 7 | 7 | **6 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | **432.4343 ✅** | 447.6967 | 598.6120 | -15.2624 | Lower | -166.1776 | Lower | -150.9153 | Lower |
| Halstead Effort | **5765.7912 ✅** | 6367.2424 | 10213.8170 | -601.4512 | Lower | -4448.0258 | Lower | -3846.5747 | Lower |
| SLOC | 27 | 27 | 27 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 19 | **16 ✅** | 18 | +3 | Higher | +1 | Higher | -2 | Lower |
| LLOC | 11 | 10 | 10 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 4 | 7 | 7 | -3 | Lower | -3 | Lower | 0 | Same |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01529

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.2562 | **73.5110 ✅** | 60.3603 | -14.2548 | Lower | -1.1041 | Lower | +13.1507 | Higher |
| MI (SEI) | 34.6382 | **66.1390 ✅** | 26.4328 | -31.5008 | Lower | +8.2054 | Higher | +39.7062 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 14 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 21 | **12 ✅** | 24 | +9 | Higher | -3 | Lower | -12 | Lower |
| Halstead Volume | 2161.3733 | **994.0254 ✅** | 2322 | +1167.3479 | Higher | -160.6267 | Lower | -1327.9746 | Lower |
| Halstead Effort | 112391.4096 | **35655.2593 ✅** | 139965 | +76736.1503 | Higher | -27573.5904 | Lower | -104309.7407 | Lower |
| SLOC | 70 | **40 ✅** | 63 | +30 | Higher | +7 | Higher | -23 | Lower |
| PLOC | 54 | **27 ✅** | 52 | +27 | Higher | +2 | Higher | -25 | Lower |
| LLOC | 44 | **19 ✅** | 45 | +25 | Higher | -1 | Lower | -26 | Lower |
| Comments (CLOC) | 7 | **10 ✅** | 2 | -3 | Lower | +5 | Higher | +8 | Higher |
| Blank Lines | 10 | **5 ✅** | 10 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |

## p01545

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **56.8716 ✅** | 46.6229 | 51.1778 | +10.2488 | Higher | +5.6938 | Higher | -4.5550 | Lower |
| MI (SEI) | 23.1201 | **27.6752 ✅** | 22.2162 | -4.5551 | Lower | +0.9040 | Higher | +5.4590 | Higher |
| Cyclomatic (sum) | **15 ✅** | 21 | 21 | -6 | Lower | -6 | Lower | 0 | Same |
| Cognitive (sum) | 11 | **10 ✅** | 16 | +1 | Higher | -5 | Lower | -6 | Lower |
| Halstead Volume | **2524.1627 ✅** | 2564.8324 | 2960.4244 | -40.6697 | Lower | -436.2617 | Lower | -395.5921 | Lower |
| Halstead Effort | 157298.4326 | **137181.8913 ✅** | 194446.0601 | +20116.5413 | Higher | -37147.6275 | Lower | -57264.1688 | Lower |
| SLOC | **75 ✅** | 129 | 93 | -54 | Lower | -18 | Lower | +36 | Higher |
| PLOC | **63 ✅** | 86 | 77 | -23 | Lower | -14 | Lower | +9 | Higher |
| LLOC | 41 | **37 ✅** | 55 | +4 | Higher | -14 | Lower | -18 | Lower |
| Comments (CLOC) | 3 | **30 ✅** | 8 | -27 | Lower | -5 | Lower | +22 | Higher |
| Blank Lines | **10 ✅** | 20 | 12 | -10 | Lower | -2 | Lower | +8 | Higher |
| Functions | **4 ✅** | 10 | 5 | -6 | Lower | -1 | Lower | +5 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **5 ✅** | 6 | 6 | -1 | Lower | -1 | Lower | 0 | Same |

## p01570

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.1693 | **72.8018 ✅** | 44.8835 | -19.6325 | Lower | +8.2859 | Higher | +27.9183 | Higher |
| MI (SEI) | 13.9393 | **65.1159 ✅** | -8.6059 | -51.1765 | Lower | +22.5452 | Higher | +73.7217 | Higher |
| Cyclomatic (sum) | 14 | **8 ✅** | 23 | +6 | Higher | -9 | Lower | -15 | Lower |
| Cognitive (sum) | 15 | **5 ✅** | 38 | +10 | Higher | -23 | Lower | -33 | Lower |
| Halstead Volume | 3154.8203 | **846.5938 ✅** | 4109.2533 | +2308.2264 | Higher | -954.4330 | Lower | -3262.6594 | Lower |
| Halstead Effort | 144987.4849 | **19556.3176 ✅** | 292079.2760 | +125431.1673 | Higher | -147091.7911 | Lower | -272522.9584 | Lower |
| SLOC | 89 | **44 ✅** | 120 | +45 | Higher | -31 | Lower | -76 | Lower |
| PLOC | 72 | **25 ✅** | 107 | +47 | Higher | -35 | Lower | -82 | Lower |
| LLOC | 58 | **14 ✅** | 90 | +44 | Higher | -32 | Lower | -76 | Lower |
| Comments (CLOC) | 2 | **11 ✅** | 0 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 17 | **8 ✅** | 13 | +9 | Higher | +4 | Higher | -5 | Lower |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
| Exits | 2 | **0 ✅** | 4 | +2 | Higher | -2 | Lower | -4 | Lower |

## p01600

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.2076 ✅** | 62.0757 | 65.2359 | +4.1318 | Higher | +0.9716 | Higher | -3.1602 | Lower |
| MI (SEI) | **42.0572 ✅** | 40.2310 | 34.8382 | +1.8262 | Higher | +7.2190 | Higher | +5.3928 | Higher |
| Cyclomatic (sum) | **12 ✅** | 13 | 13 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 19 | **17 ✅** | 21 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | **1591.5674 ✅** | 1929.5069 | 1835.5416 | -337.9395 | Lower | -243.9741 | Lower | +93.9653 | Higher |
| Halstead Effort | **67527.9330 ✅** | 102565.3522 | 83707.0259 | -35037.4192 | Lower | -16179.0929 | Lower | +18858.3263 | Higher |
| SLOC | 51 | 61 | 51 | -10 | Lower | 0 | Same | +10 | Higher |
| PLOC | **38 ✅** | 46 | 41 | -8 | Lower | -3 | Lower | +5 | Higher |
| LLOC | **26 ✅** | 31 | 30 | -5 | Lower | -4 | Lower | +1 | Higher |
| Comments (CLOC) | 4 | **7 ✅** | 2 | -3 | Lower | +2 | Higher | +5 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

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

## p01658

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.9133 | 68.4104 | **69.0443 ✅** | +0.5029 | Higher | -0.1309 | Lower | -0.6338 | Lower |
| MI (SEI) | 54.1928 | **54.4054 ✅** | 51.4654 | -0.2126 | Lower | +2.7274 | Higher | +2.9400 | Higher |
| Cyclomatic (sum) | **9 ✅** | 11 | 10 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 1304.6578 | **1089.0677 ✅** | 1389.7442 | +215.5901 | Higher | -85.0864 | Lower | -300.6765 | Lower |
| Halstead Effort | 52238.4995 | **31561.1816 ✅** | 59258.6931 | +20677.3179 | Higher | -7020.1936 | Lower | -27697.5115 | Lower |
| SLOC | 48 | 51 | **46 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| PLOC | 37 | **33 ✅** | 37 | +4 | Higher | 0 | Same | -4 | Lower |
| LLOC | 36 | **22 ✅** | 37 | +14 | Higher | -1 | Lower | -15 | Lower |
| Comments (CLOC) | 8 | **9 ✅** | 6 | -1 | Lower | +2 | Higher | +3 | Higher |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.3135 | **59.1016 ✅** | 44.3125 | -15.7881 | Lower | -0.9991 | Lower | +14.7891 | Higher |
| MI (SEI) | -9.6490 | **41.5006 ✅** | -8.3095 | -51.1497 | Lower | -1.3395 | Lower | +49.8101 | Higher |
| Cyclomatic (sum) | 35 | **15 ✅** | 34 | +20 | Higher | +1 | Higher | -19 | Lower |
| Cognitive (sum) | 28 | **18 ✅** | 34 | +10 | Higher | -6 | Lower | -16 | Lower |
| Halstead Volume | 4286.4820 | **1788.3235 ✅** | 4975.3574 | +2498.1585 | Higher | -688.8754 | Lower | -3187.0339 | Lower |
| Halstead Effort | 328326.2804 | **61313.9481 ✅** | 446289.5597 | +267012.3323 | Higher | -117963.2794 | Lower | -384975.6116 | Lower |
| SLOC | 110 | **73 ✅** | 100 | +37 | Higher | +10 | Higher | -27 | Lower |
| PLOC | 92 | **50 ✅** | 82 | +42 | Higher | +10 | Higher | -32 | Lower |
| LLOC | 50 | **21 ✅** | 65 | +29 | Higher | -15 | Lower | -44 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 18 | **11 ✅** | 18 | +7 | Higher | 0 | Same | -7 | Lower |
| Functions | 5 | **3 ✅** | 4 | +2 | Higher | +1 | Higher | -1 | Lower |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 9 | **5 ✅** | 8 | +4 | Higher | +1 | Higher | -3 | Lower |

## p01725

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.5849 | **64.1875 ✅** | 41.3922 | -21.6026 | Lower | +1.1927 | Higher | +22.7953 | Higher |
| MI (SEI) | 3.8175 | **50.4875 ✅** | 3.1252 | -46.6700 | Lower | +0.6923 | Higher | +47.3623 | Higher |
| Cyclomatic (sum) | 29 | **14 ✅** | 29 | +15 | Higher | 0 | Same | -15 | Lower |
| Cognitive (sum) | 49 | **14 ✅** | 46 | +35 | Higher | +3 | Higher | -32 | Lower |
| Halstead Volume | 4109.3430 | **1364.4917 ✅** | 4476.3861 | +2744.8513 | Higher | -367.0431 | Lower | -3111.8944 | Lower |
| Halstead Effort | 219782.4848 | **45772.4952 ✅** | 299446.6709 | +174009.9895 | Higher | -79664.1862 | Lower | -253674.1757 | Lower |
| SLOC | 127 | **59 ✅** | 133 | +68 | Higher | -6 | Lower | -74 | Lower |
| PLOC | 111 | **37 ✅** | 119 | +74 | Higher | -8 | Lower | -82 | Lower |
| LLOC | 54 | **20 ✅** | 77 | +34 | Higher | -23 | Lower | -57 | Lower |
| Comments (CLOC) | 5 | **12 ✅** | 6 | -7 | Lower | -1 | Lower | +6 | Higher |
| Blank Lines | 15 | **11 ✅** | 12 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p01767

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.2238 | **65.6695 ✅** | 60.4217 | -10.4457 | Lower | -5.1979 | Lower | +5.2478 | Higher |
| MI (SEI) | 28.9570 | **52.9199 ✅** | 26.2136 | -23.9629 | Lower | +2.7434 | Higher | +26.7063 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 13 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 13 | **6 ✅** | 13 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 3220 | **1440.9203 ✅** | 2175.9819 | +1779.0797 | Higher | +1044.0181 | Higher | -735.0616 | Lower |
| Halstead Effort | 90160 | **48386.1045 ✅** | 91140.1643 | +41773.8955 | Higher | -980.1643 | Lower | -42754.0597 | Lower |
| SLOC | 79 | **56 ✅** | 65 | +23 | Higher | +14 | Higher | -9 | Lower |
| PLOC | 55 | **36 ✅** | 52 | +19 | Higher | +3 | Higher | -16 | Lower |
| LLOC | 36 | **22 ✅** | 36 | +14 | Higher | 0 | Same | -14 | Lower |
| Comments (CLOC) | 8 | **12 ✅** | 2 | -4 | Lower | +6 | Higher | +10 | Higher |
| Blank Lines | 16 | **10 ✅** | 12 | +6 | Higher | +4 | Higher | -2 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |

## p01768

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.4481 | **54.9988 ✅** | 46.9927 | -2.5507 | Lower | +5.4554 | Higher | +8.0060 | Higher |
| MI (SEI) | 9.7810 | **24.2469 ✅** | 1.0050 | -14.4658 | Lower | +8.7761 | Higher | +23.2419 | Higher |
| Cyclomatic (sum) | 17 | **15 ✅** | 18 | +2 | Higher | -1 | Lower | -3 | Lower |
| Cognitive (sum) | **9 ✅** | 11 | 11 | -2 | Lower | -2 | Lower | 0 | Same |
| Halstead Volume | 2961.5176 | **1790.6972 ✅** | 3507.2544 | +1170.8204 | Higher | -545.7368 | Lower | -1716.5572 | Lower |
| Halstead Effort | 89284.2726 | **51706.3826 ✅** | 141848.9573 | +37577.8900 | Higher | -52564.6847 | Lower | -90142.5746 | Lower |
| SLOC | **91 ✅** | 94 | 119 | -3 | Lower | -28 | Lower | -25 | Lower |
| PLOC | 73 | **71 ✅** | 99 | +2 | Higher | -26 | Lower | -28 | Lower |
| LLOC | 34 | **28 ✅** | 53 | +6 | Higher | -19 | Lower | -25 | Lower |
| Comments (CLOC) | 1 | **6 ✅** | 1 | -5 | Lower | 0 | Same | +5 | Higher |
| Blank Lines | 18 | **17 ✅** | 20 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 6 | 6 | 7 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |

## p01847

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.6031 | 52.5759 | **55.0722 ✅** | +0.0272 | Higher | -2.4692 | Lower | -2.4964 | Lower |
| MI (SEI) | 11.5075 | **37.4111 ✅** | 5.8898 | -25.9036 | Lower | +5.6177 | Higher | +31.5212 | Higher |
| Cyclomatic (sum) | 26 | 21 | 21 | +5 | Higher | +5 | Higher | 0 | Same |
| Cognitive (sum) | 60 | **26 ✅** | 50 | +34 | Higher | +10 | Higher | -24 | Lower |
| Halstead Volume | 2999.2855 | **2421.0668 ✅** | 2853.0282 | +578.2187 | Higher | +146.2574 | Higher | -431.9614 | Lower |
| Halstead Effort | 177177.3049 | **105719.9166 ✅** | 173219.5667 | +71457.3883 | Higher | +3957.7382 | Higher | -67499.6501 | Lower |
| SLOC | 79 | 91 | **74 ✅** | -12 | Lower | +5 | Higher | +17 | Higher |
| PLOC | 65 | 67 | **63 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| LLOC | 67 | **37 ✅** | 79 | +30 | Higher | -12 | Lower | -42 | Lower |
| Comments (CLOC) | 1 | **23 ✅** | 0 | -22 | Lower | +1 | Higher | +23 | Higher |
| Blank Lines | 14 | 13 | **11 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p01907

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.4512 | **62.4856 ✅** | 38.7965 | -11.0344 | Lower | +12.6547 | Higher | +23.6891 | Higher |
| MI (SEI) | 7.8717 | **40.0781 ✅** | -4.8437 | -32.2064 | Lower | +12.7154 | Higher | +44.9218 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 26 | +5 | Higher | -10 | Lower | -15 | Lower |
| Cognitive (sum) | 13 | **3 ✅** | 28 | +10 | Higher | -15 | Lower | -25 | Lower |
| Halstead Volume | 2794.9868 | **1677.5569 ✅** | 5022.0740 | +1117.4299 | Higher | -2227.0872 | Lower | -3344.5170 | Lower |
| Halstead Effort | 110429.3803 | **42455.0946 ✅** | 362844.8441 | +67974.2857 | Higher | -252415.4638 | Lower | -320389.7495 | Lower |
| SLOC | 100 | **64 ✅** | 157 | +36 | Higher | -57 | Lower | -93 | Lower |
| PLOC | 82 | **53 ✅** | 129 | +29 | Higher | -47 | Lower | -76 | Lower |
| LLOC | 45 | **20 ✅** | 115 | +25 | Higher | -70 | Lower | -95 | Lower |
| Comments (CLOC) | 1 | **7 ✅** | 4 | -6 | Lower | -3 | Lower | +3 | Higher |
| Blank Lines | 18 | **10 ✅** | 26 | +8 | Higher | -8 | Lower | -16 | Lower |
| Functions | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |

## p01916

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.3217 | 84.5281 | **84.6841 ✅** | -2.2064 | Lower | -2.3625 | Lower | -0.1560 | Lower |
| MI (SEI) | 81.5643 | **85.0360 ✅** | 80.8511 | -3.4717 | Lower | +0.7131 | Higher | +4.1848 | Higher |
| Cyclomatic (sum) | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 6 | **1 ✅** | 4 | +5 | Higher | +2 | Higher | -3 | Lower |
| Halstead Volume | 567.9005 | **452.7842 ✅** | 506.6527 | +115.1163 | Higher | +61.2479 | Higher | -53.8685 | Lower |
| Halstead Effort | 7441.8630 | **4624.8671 ✅** | 7706.4535 | +2816.9959 | Higher | -264.5905 | Lower | -3081.5864 | Lower |
| SLOC | 29 | 28 | **26 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| PLOC | 18 | **14 ✅** | 17 | +4 | Higher | +1 | Higher | -3 | Lower |
| LLOC | 14 | **9 ✅** | 13 | +5 | Higher | +1 | Higher | -4 | Lower |
| Comments (CLOC) | 9 | 9 | 6 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p01917

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.0507 | **67.7550 ✅** | 33.3747 | -11.7044 | Lower | +22.6759 | Higher | +34.3803 | Higher |
| MI (SEI) | 7.0977 | **58.4095 ✅** | -23.2748 | -51.3117 | Lower | +30.3726 | Higher | +81.6843 | Higher |
| Cyclomatic (sum) | 19 | **11 ✅** | 42 | +8 | Higher | -23 | Lower | -31 | Lower |
| Cognitive (sum) | 29 | **14 ✅** | 73 | +15 | Higher | -44 | Lower | -59 | Lower |
| Halstead Volume | 2812.4078 | **1235.3550 ✅** | 6882.7433 | +1577.0528 | Higher | -4070.3354 | Lower | -5647.3882 | Lower |
| Halstead Effort | 144504.1929 | **36788.1464 ✅** | 456842.0847 | +107716.0465 | Higher | -312337.8918 | Lower | -420053.9383 | Lower |
| SLOC | 72 | **51 ✅** | 158 | +21 | Higher | -86 | Lower | -107 | Lower |
| PLOC | 63 | **37 ✅** | 138 | +26 | Higher | -75 | Lower | -101 | Lower |
| LLOC | 47 | **19 ✅** | 105 | +28 | Higher | -58 | Lower | -86 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 0 | -13 | Lower | 0 | Same | +13 | Higher |
| Blank Lines | 9 | **7 ✅** | 20 | +2 | Higher | -11 | Lower | -13 | Lower |
| Functions | 3 | **2 ✅** | 6 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 3 | **0 ✅** | 4 | +3 | Higher | -1 | Lower | -4 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p02035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.3631 | 56.9358 | **58.4271 ✅** | -0.5727 | Lower | -2.0639 | Lower | -1.4912 | Lower |
| MI (SEI) | 7.3449 | **40.0709 ✅** | 26.9317 | -32.7260 | Lower | -19.5868 | Lower | +13.1392 | Higher |
| Cyclomatic (sum) | 17 | **15 ✅** | 16 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 14 | **8 ✅** | 14 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 3022.1971 | **2120.5737 ✅** | 3240.0339 | +901.6233 | Higher | -217.8368 | Lower | -1119.4602 | Lower |
| Halstead Effort | 258311.5001 | **149500.4475 ✅** | 305754.3740 | +108811.0526 | Higher | -47442.8739 | Lower | -156253.9264 | Lower |
| SLOC | 71 | 79 | **62 ✅** | -8 | Lower | +9 | Higher | +17 | Higher |
| PLOC | 61 | 60 | **54 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |
| LLOC | 41 | 42 | 41 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 0 | **16 ✅** | 3 | -16 | Lower | -3 | Lower | +13 | Higher |
| Blank Lines | 10 | 15 | **8 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | 8 | **4 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p02038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.5621 | 66.4590 | **75.6014 ✅** | +1.1032 | Higher | -8.0392 | Lower | -9.1424 | Lower |
| MI (SEI) | 49.8877 | 49.0335 | **58.7692 ✅** | +0.8542 | Higher | -8.8815 | Lower | -9.7357 | Lower |
| Cyclomatic (sum) | 18 | 12 | **7 ✅** | +6 | Higher | +11 | Higher | +5 | Higher |
| Cognitive (sum) | 10 | 12 | **5 ✅** | -2 | Lower | +5 | Higher | +7 | Higher |
| Halstead Volume | 1213.2411 | 1133.1465 | **965.0994 ✅** | +80.0946 | Higher | +248.1417 | Higher | +168.0471 | Higher |
| Halstead Effort | 38667.1687 | 30217.2401 | **16286.0525 ✅** | +8449.9286 | Higher | +22381.1162 | Higher | +13931.1876 | Higher |
| SLOC | 47 | 56 | **36 ✅** | -9 | Lower | +11 | Higher | +20 | Higher |
| PLOC | 32 | 41 | **26 ✅** | -9 | Lower | +6 | Higher | +15 | Higher |
| LLOC | 19 | 24 | **16 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| Comments (CLOC) | 6 | **8 ✅** | 4 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 9 | 7 | 7 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Exits | 8 | **0 ✅** | 1 | +8 | Higher | +7 | Higher | -1 | Lower |

## p02045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.9555 | 54.2546 | **59.5951 ✅** | -0.2990 | Lower | -5.6395 | Lower | -5.3405 | Lower |
| MI (SEI) | 4.0751 | **31.1936 ✅** | 21.7816 | -27.1185 | Lower | -17.7065 | Lower | +9.4120 | Higher |
| Cyclomatic (sum) | 19 | 20 | **17 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 25 | 15 | **13 ✅** | +10 | Higher | +12 | Higher | +2 | Higher |
| Halstead Volume | 2915.4074 | **2351.3102 ✅** | 2476.2303 | +564.0971 | Higher | +439.1770 | Higher | -124.9201 | Lower |
| Halstead Effort | 151442.7370 | **91009.5367 ✅** | 118033.6462 | +60433.2003 | Higher | +33409.0909 | Higher | -27024.1094 | Lower |
| SLOC | 81 | 84 | **62 ✅** | -3 | Lower | +19 | Higher | +22 | Higher |
| PLOC | 71 | 63 | **54 ✅** | +8 | Higher | +17 | Higher | +9 | Higher |
| LLOC | 59 | **35 ✅** | 49 | +24 | Higher | +10 | Higher | -14 | Lower |
| Comments (CLOC) | 0 | **11 ✅** | 1 | -11 | Lower | -1 | Lower | +10 | Higher |
| Blank Lines | 10 | 12 | **8 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |

## p02046

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.8854 ✅** | 74.2833 | 75.7507 | +4.6021 | Higher | +3.1348 | Higher | -1.4674 | Lower |
| MI (SEI) | 53.7589 | **70.4718 ✅** | 48.1273 | -16.7129 | Lower | +5.6316 | Higher | +22.3445 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 12 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 11 | **6 ✅** | 12 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | 900.0687 | **829.2698 ✅** | 985.7896 | +70.7989 | Higher | -85.7208 | Lower | -156.5198 | Lower |
| Halstead Effort | 32307.7304 | **22107.5795 ✅** | 38238.2585 | +10200.1510 | Higher | -5930.5281 | Lower | -16130.6791 | Lower |
| SLOC | **28 ✅** | 41 | 33 | -13 | Lower | -5 | Lower | +8 | Higher |
| PLOC | 25 | **23 ✅** | 28 | +2 | Higher | -3 | Lower | -5 | Lower |
| LLOC | **10 ✅** | 14 | 22 | -4 | Lower | -12 | Lower | -8 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 1 | -12 | Lower | 0 | Same | +12 | Higher |
| Blank Lines | **3 ✅** | 8 | 5 | -5 | Lower | -2 | Lower | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02104

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.0076 | **73.8578 ✅** | 61.7807 | -9.8502 | Lower | +2.2269 | Higher | +12.0770 | Higher |
| MI (SEI) | 43.3920 | **69.0880 ✅** | 28.4869 | -25.6960 | Lower | +14.9051 | Higher | +40.6011 | Higher |
| Cyclomatic (sum) | 11 | **6 ✅** | 13 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 14 | **5 ✅** | 15 | +9 | Higher | -1 | Lower | -10 | Lower |
| Halstead Volume | 1701.1924 | **810.9700 ✅** | 1941.2722 | +890.2224 | Higher | -240.0798 | Lower | -1130.3022 | Lower |
| Halstead Effort | 88097.4623 | **12080.6561 ✅** | 113971.4650 | +76016.8061 | Higher | -25874.0027 | Lower | -101890.8089 | Lower |
| SLOC | 58 | **43 ✅** | 62 | +15 | Higher | -4 | Lower | -19 | Lower |
| PLOC | 44 | **22 ✅** | 52 | +22 | Higher | -8 | Lower | -30 | Lower |
| LLOC | 42 | **15 ✅** | 49 | +27 | Higher | -7 | Lower | -34 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 2 | -6 | Lower | +5 | Higher | +11 | Higher |
| Blank Lines | 8 | 8 | 10 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.6976 ✅** | 59.4778 | 64.3488 | +6.2198 | Higher | +1.3488 | Higher | -4.8711 | Lower |
| MI (SEI) | **49.6553 ✅** | 47.7080 | 42.6353 | +1.9473 | Higher | +7.0200 | Higher | +5.0727 | Higher |
| Cyclomatic (sum) | 10 | 16 | 10 | -6 | Lower | 0 | Same | +6 | Higher |
| Cognitive (sum) | 21 | **16 ✅** | 21 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | **1605.0868 ✅** | 2079.0331 | 1964.8028 | -473.9463 | Lower | -359.7160 | Lower | +114.2303 | Higher |
| Halstead Effort | **47572.9891 ✅** | 136176.6656 | 91738.9533 | -88603.6765 | Lower | -44165.9642 | Lower | +44437.7123 | Higher |
| SLOC | **54 ✅** | 67 | 55 | -13 | Lower | -1 | Lower | +12 | Higher |
| PLOC | **37 ✅** | 48 | 47 | -11 | Lower | -10 | Lower | +1 | Higher |
| LLOC | 29 | **23 ✅** | 45 | +6 | Higher | -16 | Lower | -22 | Lower |
| Comments (CLOC) | 9 | **18 ✅** | 6 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | 11 | 9 | **8 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

## p02153

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **39.2339 ✅** | 35.9020 | 27.2139 | +3.3319 | Higher | +12.0200 | Higher | +8.6881 | Higher |
| MI (SEI) | -9.0914 | **-5.8214 ✅** | -26.0377 | -3.2700 | Lower | +16.9463 | Higher | +20.2163 | Higher |
| Cyclomatic (sum) | **33 ✅** | 41 | 49 | -8 | Lower | -16 | Lower | -8 | Lower |
| Cognitive (sum) | 40 | **26 ✅** | 52 | +14 | Higher | -12 | Lower | -26 | Lower |
| Halstead Volume | 5421.9439 | **5095.8119 ✅** | 7449.3210 | +326.1320 | Higher | -2027.3772 | Lower | -2353.5091 | Lower |
| Halstead Effort | 328027.6036 | **268945.6280 ✅** | 731481.9390 | +59081.9756 | Higher | -403454.3354 | Lower | -462536.3110 | Lower |
| SLOC | **135 ✅** | 151 | 204 | -16 | Lower | -69 | Lower | -53 | Lower |
| PLOC | **117 ✅** | 127 | 178 | -10 | Lower | -61 | Lower | -51 | Lower |
| LLOC | 75 | **48 ✅** | 115 | +27 | Higher | -40 | Lower | -67 | Lower |
| Comments (CLOC) | 1 | **5 ✅** | 1 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | **18 ✅** | 24 | 26 | -6 | Lower | -8 | Lower | -2 | Lower |
| Functions | **4 ✅** | 8 | 11 | -4 | Lower | -7 | Lower | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **4 ✅** | 8 | 7 | -4 | Lower | -3 | Lower | +1 | Higher |

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

## p02166

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.0830 | 61.8706 | **67.1817 ✅** | +3.2123 | Higher | -2.0987 | Lower | -5.3111 | Lower |
| MI (SEI) | 40.8340 | **48.3294 ✅** | 33.8673 | -7.4953 | Lower | +6.9667 | Higher | +14.4621 | Higher |
| Cyclomatic (sum) | 14 | 16 | **13 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 11 | 15 | 11 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 1923.6383 | **1513.5354 ✅** | 1741.2623 | +410.1028 | Higher | +182.3759 | Higher | -227.7269 | Lower |
| Halstead Effort | 83526.3980 | **47163.7164 ✅** | 80996.7838 | +36362.6816 | Higher | +2529.6142 | Higher | -33833.0674 | Lower |
| SLOC | 50 | 64 | **46 ✅** | -14 | Lower | +4 | Higher | +18 | Higher |
| PLOC | 41 | 46 | 41 | -5 | Lower | 0 | Same | +5 | Higher |
| LLOC | 34 | **29 ✅** | 33 | +5 | Higher | +1 | Higher | -4 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 1 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 6 | 1 | -5 | Lower | 0 | Same | +5 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p02205

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.6821 | **83.6183 ✅** | 80.9301 | -1.9362 | Lower | +0.7520 | Higher | +2.6882 | Higher |
| MI (SEI) | 75.4102 | **84.7604 ✅** | 75.4847 | -9.3502 | Lower | -0.0744 | Lower | +9.2758 | Higher |
| Cyclomatic (sum) | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 748.8245 | **679.4320 ✅** | 697.9744 | +69.3925 | Higher | +50.8502 | Higher | -18.5424 | Lower |
| Halstead Effort | 13931.2565 | **10701.0539 ✅** | 13959.4872 | +3230.2025 | Higher | -28.2307 | Lower | -3258.4333 | Lower |
| SLOC | 28 | **26 ✅** | 30 | +2 | Higher | -2 | Lower | -4 | Lower |
| PLOC | 21 | **16 ✅** | 21 | +5 | Higher | 0 | Same | -5 | Lower |
| LLOC | 20 | **14 ✅** | 20 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 7 | -3 | Lower | -1 | Lower | +2 | Higher |
| Blank Lines | 5 | 5 | 6 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02242

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.2029 | 47.1092 | **48.9396 ✅** | +1.0937 | Higher | -0.7367 | Lower | -1.8304 | Lower |
| MI (SEI) | 10.7614 | **19.4014 ✅** | 8.3165 | -8.6400 | Lower | +2.4449 | Higher | +11.0849 | Higher |
| Cyclomatic (sum) | 32 | **22 ✅** | 25 | +10 | Higher | +7 | Higher | -3 | Lower |
| Cognitive (sum) | **20 ✅** | 28 | 23 | -8 | Lower | -3 | Lower | +5 | Higher |
| Halstead Volume | 3119.2062 | 4120.2849 | **3042.7431 ✅** | -1001.0787 | Lower | +76.4631 | Higher | +1077.5418 | Higher |
| Halstead Effort | 139246.0731 | **110680.6818 ✅** | 164490.6916 | +28565.3913 | Higher | -25244.6186 | Lower | -53810.0099 | Lower |
| SLOC | **94 ✅** | 106 | 100 | -12 | Lower | -6 | Lower | +6 | Higher |
| PLOC | **82 ✅** | 87 | 90 | -5 | Lower | -8 | Lower | -3 | Lower |
| LLOC | 50 | 50 | 67 | 0 | Same | -17 | Lower | -17 | Lower |
| Comments (CLOC) | 3 | **12 ✅** | 2 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 12 | 19 | **10 ✅** | -7 | Lower | +2 | Higher | +9 | Higher |
| Functions | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 13 | 3 | 3 | +10 | Higher | +10 | Higher | 0 | Same |

## p02249

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.5661 | **60.9645 ✅** | 44.4631 | -13.3984 | Lower | +3.1030 | Higher | +16.5014 | Higher |
| MI (SEI) | -4.8374 | **30.2608 ✅** | 4.8894 | -35.0982 | Lower | -9.7268 | Lower | +25.3714 | Higher |
| Cyclomatic (sum) | 22 | **14 ✅** | 25 | +8 | Higher | -3 | Lower | -11 | Lower |
| Cognitive (sum) | 38 | **13 ✅** | 39 | +25 | Higher | -1 | Lower | -26 | Lower |
| Halstead Volume | 4386.7154 | **2067.2705 ✅** | 3973.9863 | +2319.4449 | Higher | +412.7291 | Higher | -1906.7158 | Lower |
| Halstead Effort | 158506.6494 | **54044.3570 ✅** | 303037.5955 | +104462.2924 | Higher | -144530.9461 | Lower | -248993.2385 | Lower |
| SLOC | 101 | **63 ✅** | 121 | +38 | Higher | -20 | Lower | -58 | Lower |
| PLOC | 86 | **53 ✅** | 105 | +33 | Higher | -19 | Lower | -52 | Lower |
| LLOC | 64 | **17 ✅** | 84 | +47 | Higher | -20 | Lower | -67 | Lower |
| Comments (CLOC) | 0 | 3 | **4 ✅** | -3 | Lower | -4 | Lower | -1 | Lower |
| Blank Lines | 15 | **8 ✅** | 14 | +7 | Higher | +1 | Higher | -6 | Lower |
| Functions | **2 ✅** | 5 | 4 | -3 | Lower | -2 | Lower | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **0 ✅** | 5 | 2 | -5 | Lower | -2 | Lower | +3 | Higher |

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

## p02266

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.8918 | **71.9592 ✅** | 54.1918 | -18.0674 | Lower | -0.3000 | Lower | +17.7674 | Higher |
| MI (SEI) | 11.6754 | **62.0868 ✅** | 4.0087 | -50.4114 | Lower | +7.6667 | Higher | +58.0781 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 15 | +7 | Higher | +1 | Higher | -6 | Lower |
| Cognitive (sum) | 41 | **7 ✅** | 29 | +34 | Higher | +12 | Higher | -22 | Lower |
| Halstead Volume | 2191.4564 | **829.2698 ✅** | 2091.2992 | +1362.1866 | Higher | +100.1572 | Higher | -1262.0294 | Lower |
| Halstead Effort | 72952.4305 | **16665.1338 ✅** | 70929.8984 | +56287.2967 | Higher | +2022.5321 | Higher | -54264.7646 | Lower |
| SLOC | 93 | **46 ✅** | 94 | +47 | Higher | -1 | Lower | -48 | Lower |
| PLOC | 85 | **35 ✅** | 85 | +50 | Higher | 0 | Same | -50 | Lower |
| LLOC | 51 | **25 ✅** | 54 | +26 | Higher | -3 | Lower | -29 | Lower |
| Comments (CLOC) | 1 | **10 ✅** | 0 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | 8 | **5 ✅** | 9 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02276

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.8398 | 66.0188 | **68.9295 ✅** | -0.1790 | Lower | -3.0897 | Lower | -2.9107 | Lower |
| MI (SEI) | 48.7157 | **53.4239 ✅** | 52.5446 | -4.7081 | Lower | -3.8289 | Lower | +0.8792 | Higher |
| Cyclomatic (sum) | 11 | 10 | **9 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Cognitive (sum) | 11 | 11 | **10 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 1583.8162 | 1347.3063 | **1300.6046 ✅** | +236.5099 | Higher | +283.2116 | Higher | +46.7018 | Higher |
| Halstead Effort | 57424.6509 | **41934.9099 ✅** | 47704.3178 | +15489.7410 | Higher | +9720.3331 | Higher | -5769.4079 | Lower |
| SLOC | 53 | 56 | **48 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| PLOC | 45 | **40 ✅** | 41 | +5 | Higher | +4 | Higher | -1 | Lower |
| LLOC | 27 | 27 | **26 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 8 | **12 ✅** | 7 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 8 | 7 | **6 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |

## p02277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.0146 | 57.7500 | **58.3747 ✅** | -0.7355 | Lower | -1.3601 | Lower | -0.6247 | Lower |
| MI (SEI) | 28.9095 | **38.6247 ✅** | 29.8934 | -9.7151 | Lower | -0.9839 | Lower | +8.7313 | Higher |
| Cyclomatic (sum) | 15 | 16 | 15 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 17 | 17 | 17 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 2173.2817 | 2218.8005 | **2146.9269 ✅** | -45.5188 | Lower | +26.3547 | Higher | +71.8735 | Higher |
| Halstead Effort | 90308.6633 | **64827.5610 ✅** | 86341.2771 | +25481.1023 | Higher | +3967.3862 | Higher | -21513.7160 | Lower |
| SLOC | 78 | 73 | **72 ✅** | +5 | Higher | +6 | Higher | +1 | Higher |
| PLOC | 61 | **53 ✅** | 61 | +8 | Higher | 0 | Same | -8 | Lower |
| LLOC | 38 | **32 ✅** | 38 | +6 | Higher | 0 | Same | -6 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 5 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02279

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **53.2633 ✅** | 50.0304 | 52.2358 | +3.2330 | Higher | +1.0275 | Higher | -2.2054 | Lower |
| MI (SEI) | 22.1401 | **22.4439 ✅** | 1.3904 | -0.3038 | Lower | +20.7497 | Higher | +21.0535 | Higher |
| Cyclomatic (sum) | **15 ✅** | 20 | 17 | -5 | Lower | -2 | Lower | +3 | Higher |
| Cognitive (sum) | 14 | **7 ✅** | 20 | +7 | Higher | -6 | Lower | -13 | Lower |
| Halstead Volume | 2862.8452 | **2566.6888 ✅** | 2698.0208 | +296.1564 | Higher | +164.8244 | Higher | -131.3320 | Lower |
| Halstead Effort | 105925.2724 | 108805.2861 | **103528.7044 ✅** | -2880.0137 | Lower | +2396.5681 | Higher | +5276.5818 | Higher |
| SLOC | **90 ✅** | 106 | 95 | -16 | Lower | -5 | Lower | +11 | Higher |
| PLOC | **71 ✅** | 93 | 83 | -22 | Lower | -12 | Lower | +10 | Higher |
| LLOC | 42 | **33 ✅** | 58 | +9 | Higher | -16 | Lower | -25 | Lower |
| Comments (CLOC) | 6 | **11 ✅** | 0 | -5 | Lower | +6 | Higher | +11 | Higher |
| Blank Lines | 14 | 12 | 12 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 3 | 6 | 3 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

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

## p02305

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.1235 | 69.8330 | **72.7997 ✅** | +1.2905 | Higher | -1.6762 | Lower | -2.9667 | Lower |
| MI (SEI) | 57.4832 | **62.3412 ✅** | 51.4286 | -4.8580 | Lower | +6.0546 | Higher | +10.9126 | Higher |
| Cyclomatic (sum) | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 8 | 9 | **7 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Halstead Volume | 1236.9954 | **1167.2783 ✅** | 1224.0067 | +69.7171 | Higher | +12.9887 | Higher | -56.7284 | Lower |
| Halstead Effort | 42185.8093 | **30211.9100 ✅** | 41888.2309 | +11973.8993 | Higher | +297.5784 | Higher | -11676.3209 | Lower |
| SLOC | 42 | 47 | **38 ✅** | -5 | Lower | +4 | Higher | +9 | Higher |
| PLOC | 34 | 33 | **31 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| LLOC | 18 | **14 ✅** | 22 | +4 | Higher | -4 | Lower | -8 | Lower |
| Comments (CLOC) | 7 | **13 ✅** | 3 | -6 | Lower | +4 | Higher | +10 | Higher |
| Blank Lines | 5 | 7 | **4 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02328

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.3748 | **65.0028 ✅** | 59.2753 | -12.6281 | Lower | -6.9005 | Lower | +5.7276 | Higher |
| MI (SEI) | 15.5452 | **55.7824 ✅** | 29.2350 | -40.2372 | Lower | -13.6897 | Lower | +26.5474 | Higher |
| Cyclomatic (sum) | 17 | **11 ✅** | 13 | +6 | Higher | +4 | Higher | -2 | Lower |
| Cognitive (sum) | 16 | 14 | 14 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 3108.7730 | **1483.0752 ✅** | 2153.4617 | +1625.6977 | Higher | +955.3113 | Higher | -670.3864 | Lower |
| Halstead Effort | 76252.9218 | **74365.6291 ✅** | 113493.2498 | +1887.2928 | Higher | -37240.3280 | Lower | -39127.6207 | Lower |
| SLOC | 90 | **57 ✅** | 70 | +33 | Higher | +20 | Higher | -13 | Lower |
| PLOC | 73 | **43 ✅** | 59 | +30 | Higher | +14 | Higher | -16 | Lower |
| LLOC | 41 | **28 ✅** | 46 | +13 | Higher | -5 | Lower | -18 | Lower |
| Comments (CLOC) | 3 | **16 ✅** | 4 | -13 | Lower | -1 | Lower | +12 | Higher |
| Blank Lines | 15 | **7 ✅** | 10 | +8 | Higher | +5 | Higher | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02333

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.9858 | 50.4429 | **59.4839 ✅** | +8.5429 | Higher | -0.4981 | Lower | -9.0410 | Lower |
| MI (SEI) | 20.9276 | 16.0855 | **25.2703 ✅** | +4.8421 | Higher | -4.3427 | Lower | -9.1849 | Lower |
| Cyclomatic (sum) | 18 | 20 | **15 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Cognitive (sum) | 13 | 13 | **11 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | **2534.0561 ✅** | 2842.8451 | 2629.3114 | -308.7891 | Lower | -95.2554 | Lower | +213.5337 | Higher |
| Halstead Effort | 189645.4862 | 195258.5746 | **175287.4295 ✅** | -5613.0884 | Lower | +14358.0567 | Higher | +19971.1451 | Higher |
| SLOC | 63 | 100 | 63 | -37 | Lower | 0 | Same | +37 | Higher |
| PLOC | 54 | 81 | **52 ✅** | -27 | Lower | +2 | Higher | +29 | Higher |
| LLOC | **39 ✅** | 53 | 40 | -14 | Lower | -1 | Lower | +13 | Higher |
| Comments (CLOC) | 1 | **5 ✅** | 2 | -4 | Lower | -1 | Lower | +3 | Higher |
| Blank Lines | **8 ✅** | 17 | 11 | -9 | Lower | -3 | Lower | +6 | Higher |
| Functions | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 6 | 9 | **4 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |

## p02340

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.3727 | 70.8307 | **70.8319 ✅** | -7.4581 | Lower | -7.4592 | Lower | -0.0012 | Lower |
| MI (SEI) | 28.7283 | **59.4172 ✅** | 39.1413 | -30.6890 | Lower | -10.4130 | Lower | +20.2759 | Higher |
| Cyclomatic (sum) | 22 | 13 | **8 ✅** | +9 | Higher | +14 | Higher | +5 | Higher |
| Cognitive (sum) | 9 | 10 | **6 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Halstead Volume | 1764 | **1235.3550 ✅** | 1429.3474 | +528.6450 | Higher | +334.6526 | Higher | -193.9923 | Lower |
| Halstead Effort | 70887.6000 | **41172.0672 ✅** | 46020.6537 | +29715.5328 | Higher | +24866.9463 | Higher | -4848.5865 | Lower |
| SLOC | 51 | **41 ✅** | 42 | +10 | Higher | +9 | Higher | -1 | Lower |
| PLOC | 43 | **25 ✅** | 36 | +18 | Higher | +7 | Higher | -11 | Lower |
| LLOC | 24 | **17 ✅** | 21 | +7 | Higher | +3 | Higher | -4 | Lower |
| Comments (CLOC) | 1 | **8 ✅** | 1 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 7 | 9 | **5 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 5 | 2 | **0 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| Exits | 9 | 5 | **2 ✅** | +4 | Higher | +7 | Higher | +3 | Higher |

## p02347

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **42.1935 ✅** | 40.8587 | 41.0160 | +1.3348 | Higher | +1.1775 | Higher | -0.1574 | Lower |
| MI (SEI) | **2.1704 ✅** | 1.9066 | -13.9818 | +0.2638 | Higher | +16.1522 | Higher | +15.8884 | Higher |
| Cyclomatic (sum) | 31 | 29 | **25 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Cognitive (sum) | 28 | **16 ✅** | 33 | +12 | Higher | -5 | Lower | -17 | Lower |
| Halstead Volume | 4596.0762 | **4134.8234 ✅** | 4895.3150 | +461.2528 | Higher | -299.2388 | Lower | -760.4916 | Lower |
| Halstead Effort | **299288.1274 ✅** | 315624.8542 | 349553.2034 | -16336.7268 | Lower | -50265.0760 | Lower | -33928.3493 | Lower |
| SLOC | **122 ✅** | 141 | 140 | -19 | Lower | -18 | Lower | +1 | Higher |
| PLOC | **99 ✅** | 120 | 123 | -21 | Lower | -24 | Lower | -3 | Lower |
| LLOC | **48 ✅** | 55 | 76 | -7 | Lower | -28 | Lower | -21 | Lower |
| Comments (CLOC) | 4 | **6 ✅** | 0 | -2 | Lower | +4 | Higher | +6 | Higher |
| Blank Lines | 21 | 20 | **17 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | **4 ✅** | 6 | 5 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 4 | 4 | **0 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| Exits | **3 ✅** | 4 | 4 | -1 | Lower | -1 | Lower | 0 | Same |

## p02353

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **43.5655 ✅** | 43.1575 | 38.8162 | +0.4080 | Higher | +4.7494 | Higher | +4.3413 | Higher |
| MI (SEI) | 3.3822 | **13.9867 ✅** | -10.9096 | -10.6045 | Lower | +14.2918 | Higher | +24.8963 | Higher |
| Cyclomatic (sum) | **24 ✅** | 26 | 25 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | **13 ✅** | 19 | 14 | -6 | Lower | -1 | Lower | +5 | Higher |
| Halstead Volume | 4690.4410 | **4416.2325 ✅** | 5667.2093 | +274.2086 | Higher | -976.7683 | Lower | -1250.9769 | Lower |
| Halstead Effort | **281272.6778 ✅** | 427244.8139 | 397371.3836 | -145972.1361 | Lower | -116098.7058 | Lower | +29873.4303 | Higher |
| SLOC | **123 ✅** | 125 | 153 | -2 | Lower | -30 | Lower | -28 | Lower |
| PLOC | 107 | **97 ✅** | 138 | +10 | Higher | -31 | Lower | -41 | Lower |
| LLOC | **62 ✅** | 66 | 77 | -4 | Lower | -15 | Lower | -11 | Lower |
| Comments (CLOC) | 4 | **14 ✅** | 1 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 16 | 17 | **15 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 7 | **6 ✅** | 9 | +1 | Higher | -2 | Lower | -3 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 9 | **7 ✅** | 11 | +2 | Higher | -2 | Lower | -4 | Lower |

## p02355

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.9856 | **65.3985 ✅** | 53.9871 | -15.4128 | Lower | -4.0015 | Lower | +11.4114 | Higher |
| MI (SEI) | 19.5759 | **54.9443 ✅** | 13.0912 | -35.3684 | Lower | +6.4847 | Higher | +41.8531 | Higher |
| Cyclomatic (sum) | 28 | **13 ✅** | 24 | +15 | Higher | +4 | Higher | -11 | Lower |
| Cognitive (sum) | 31 | **20 ✅** | 29 | +11 | Higher | +2 | Higher | -9 | Lower |
| Halstead Volume | 2474.5939 | **1329.3790 ✅** | 2152.8577 | +1145.2149 | Higher | +321.7362 | Higher | -823.4786 | Lower |
| Halstead Effort | 177720.8338 | **72477.7438 ✅** | 163509.5388 | +105243.0900 | Higher | +14211.2950 | Higher | -91031.7950 | Lower |
| SLOC | 96 | **56 ✅** | 83 | +40 | Higher | +13 | Higher | -27 | Lower |
| PLOC | 79 | **42 ✅** | 71 | +37 | Higher | +8 | Higher | -29 | Lower |
| LLOC | 62 | **32 ✅** | 61 | +30 | Higher | +1 | Higher | -29 | Lower |
| Comments (CLOC) | 7 | **14 ✅** | 1 | -7 | Lower | +6 | Higher | +13 | Higher |
| Blank Lines | 12 | **6 ✅** | 12 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | **1 ✅** | 2 | 3 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |

## p02370

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **64.1798 ✅** | 63.0180 | 56.1068 | +1.1618 | Higher | +8.0730 | Higher | +6.9112 | Higher |
| MI (SEI) | 37.8576 | **47.5218 ✅** | 15.4423 | -9.6642 | Lower | +22.4153 | Higher | +32.0795 | Higher |
| Cyclomatic (sum) | 10 | 10 | 15 | 0 | Same | -5 | Lower | -5 | Lower |
| Cognitive (sum) | 15 | 15 | 19 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | 1815.9741 | **1662.3737 ✅** | 2487.0966 | +153.6005 | Higher | -671.1225 | Lower | -824.7229 | Lower |
| Halstead Effort | 54933.2168 | **45976.5056 ✅** | 137491.8000 | +8956.7112 | Higher | -82558.5832 | Lower | -91515.2944 | Lower |
| SLOC | **57 ✅** | 63 | 79 | -6 | Lower | -22 | Lower | -16 | Lower |
| PLOC | 45 | **42 ✅** | 69 | +3 | Higher | -24 | Lower | -27 | Lower |
| LLOC | 35 | **32 ✅** | 61 | +3 | Higher | -26 | Lower | -29 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 1 | -8 | Lower | +3 | Higher | +11 | Higher |
| Blank Lines | **8 ✅** | 11 | 10 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p02371

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.5019 | **65.1063 ✅** | 40.3634 | -23.6044 | Lower | +1.1385 | Higher | +24.7428 | Higher |
| MI (SEI) | -4.0776 | **53.6961 ✅** | -8.9375 | -57.7738 | Lower | +4.8599 | Higher | +62.6336 | Higher |
| Cyclomatic (sum) | 23 | **10 ✅** | 21 | +13 | Higher | +2 | Higher | -11 | Lower |
| Cognitive (sum) | 21 | **4 ✅** | 21 | +17 | Higher | 0 | Same | -17 | Lower |
| Halstead Volume | 5583.2218 | **1798.4002 ✅** | 5812.2453 | +3784.8216 | Higher | -229.0235 | Lower | -4013.8451 | Lower |
| Halstead Effort | 532530.1182 | **62044.8057 ✅** | 600340.3549 | +470485.3125 | Higher | -67810.2367 | Lower | -538295.5492 | Lower |
| SLOC | 134 | **54 ✅** | 146 | +80 | Higher | -12 | Lower | -92 | Lower |
| PLOC | 115 | **43 ✅** | 124 | +72 | Higher | -9 | Lower | -81 | Lower |
| LLOC | 101 | **22 ✅** | 110 | +79 | Higher | -9 | Lower | -88 | Lower |
| Comments (CLOC) | 2 | **13 ✅** | 1 | -11 | Lower | +1 | Higher | +12 | Higher |
| Blank Lines | 19 | **8 ✅** | 22 | +11 | Higher | -3 | Lower | -14 | Lower |
| Functions | 6 | **4 ✅** | 6 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 3 | 1 | 1 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |

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

## p02430

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.0532 ✅** | 64.2519 | 67.7969 | +7.8013 | Higher | +4.2563 | Higher | -3.5450 | Lower |
| MI (SEI) | 47.3994 | **53.0745 ✅** | 23.3312 | -5.6751 | Lower | +24.0682 | Higher | +29.7433 | Higher |
| Cyclomatic (sum) | 15 | 16 | **12 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Cognitive (sum) | 10 | **6 ✅** | 8 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | **1230.7398 ✅** | 1625.4956 | 1512.1640 | -394.7558 | Lower | -281.4242 | Lower | +113.3316 | Higher |
| Halstead Effort | **38192.6349 ✅** | 51071.3142 | 43852.7558 | -12878.6794 | Lower | -5660.1209 | Lower | +7218.5584 | Higher |
| SLOC | **37 ✅** | 54 | 47 | -17 | Lower | -10 | Lower | +7 | Higher |
| PLOC | **32 ✅** | 37 | 41 | -5 | Lower | -9 | Lower | -4 | Lower |
| LLOC | **19 ✅** | 21 | 22 | -2 | Lower | -3 | Lower | -1 | Lower |
| Comments (CLOC) | 2 | **13 ✅** | 0 | -11 | Lower | +2 | Higher | +13 | Higher |
| Blank Lines | **4 ✅** | 7 | 6 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Closures | **1 ✅** | 4 | 2 | -3 | Lower | -1 | Lower | +2 | Higher |
| Exits | 7 | 4 | 4 | +3 | Higher | +3 | Higher | 0 | Same |

## p02444

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.9683 | **66.9257 ✅** | 66.7618 | -1.9574 | Lower | -1.7935 | Lower | +0.1639 | Higher |
| MI (SEI) | 48.2435 | 51.5643 | **55.1416 ✅** | -3.3208 | Lower | -6.8981 | Lower | -3.5774 | Lower |
| Cyclomatic (sum) | 11 | 9 | 9 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 11 | **7 ✅** | 9 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | 1577.5761 | **1404.1926 ✅** | 1449.1639 | +173.3836 | Higher | +128.4123 | Higher | -44.9713 | Lower |
| Halstead Effort | 47108.1762 | 46288.2044 | **40418.0862 ✅** | +819.9717 | Higher | +6690.0900 | Higher | +5870.1182 | Higher |
| SLOC | 56 | 53 | 53 | +3 | Higher | +3 | Higher | 0 | Same |
| PLOC | 38 | 36 | 36 | +2 | Higher | +2 | Higher | 0 | Same |
| LLOC | 26 | **23 ✅** | 27 | +3 | Higher | -1 | Lower | -4 | Lower |
| Comments (CLOC) | 9 | 9 | **12 ✅** | 0 | Same | -3 | Lower | -3 | Lower |
| Blank Lines | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02449

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.7025 ✅** | 52.5489 | 50.8251 | +3.1536 | Higher | +4.8774 | Higher | +1.7238 | Higher |
| MI (SEI) | 25.4026 | **33.0613 ✅** | 24.4500 | -7.6587 | Lower | +0.9526 | Higher | +8.6113 | Higher |
| Cyclomatic (sum) | **16 ✅** | 17 | 21 | -1 | Lower | -5 | Lower | -4 | Lower |
| Cognitive (sum) | 14 | 14 | 51 | 0 | Same | -37 | Lower | -37 | Lower |
| Halstead Volume | 2676 | **2540.3440 ✅** | 2607.5024 | +135.6560 | Higher | +68.4976 | Higher | -67.1584 | Lower |
| Halstead Effort | 165185.6571 | **149372.2253 ✅** | 187151.3791 | +15813.4319 | Higher | -21965.7219 | Lower | -37779.1538 | Lower |
| SLOC | **78 ✅** | 95 | 99 | -17 | Lower | -21 | Lower | -4 | Lower |
| PLOC | **57 ✅** | 58 | 86 | -1 | Lower | -29 | Lower | -28 | Lower |
| LLOC | 42 | **41 ✅** | 77 | +1 | Higher | -35 | Lower | -36 | Lower |
| Comments (CLOC) | 5 | **18 ✅** | 11 | -13 | Lower | -6 | Lower | +7 | Higher |
| Blank Lines | 18 | 19 | **8 ✅** | -1 | Lower | +10 | Higher | +11 | Higher |
| Functions | 3 | 3 | **1 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 4 | 4 | **0 ✅** | 0 | Same | +4 | Higher | +4 | Higher |

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

## p02457

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.8605 | **55.4311 ✅** | 38.7414 | -11.5706 | Lower | +5.1191 | Higher | +16.6897 | Higher |
| MI (SEI) | -9.3689 | **37.2349 ✅** | -6.5946 | -46.6038 | Lower | -2.7742 | Lower | +43.8296 | Higher |
| Cyclomatic (sum) | 30 | **18 ✅** | 36 | +12 | Higher | -6 | Lower | -18 | Lower |
| Cognitive (sum) | 26 | **10 ✅** | 37 | +16 | Higher | -11 | Lower | -27 | Lower |
| Halstead Volume | 3767.5003 | **1974.5007 ✅** | 5871.0945 | +1792.9996 | Higher | -2103.5942 | Lower | -3896.5938 | Lower |
| Halstead Effort | 217450.2885 | **71560.6911 ✅** | 291184.7417 | +145889.5974 | Higher | -73734.4532 | Lower | -219624.0506 | Lower |
| SLOC | 119 | **85 ✅** | 130 | +34 | Higher | -11 | Lower | -45 | Lower |
| PLOC | 106 | **70 ✅** | 112 | +36 | Higher | -6 | Lower | -42 | Lower |
| LLOC | 57 | **27 ✅** | 62 | +30 | Higher | -5 | Lower | -35 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 2 | -16 | Lower | -2 | Lower | +14 | Higher |
| Blank Lines | 13 | **10 ✅** | 18 | +3 | Higher | -5 | Lower | -8 | Lower |
| Functions | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 7 | **3 ✅** | 14 | +4 | Higher | -7 | Lower | -11 | Lower |

## p02584

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.8659 | **83.6620 ✅** | 81.3149 | -10.7961 | Lower | -8.4490 | Lower | +2.3472 | Higher |
| MI (SEI) | 56.5735 | **88.5572 ✅** | 66.3009 | -31.9837 | Lower | -9.7274 | Lower | +22.2563 | Higher |
| Cyclomatic (sum) | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 1507.6488 | **438.8623 ✅** | 803.6111 | +1068.7865 | Higher | +704.0376 | Higher | -364.7489 | Lower |
| Halstead Effort | 27103.4133 | **7926.9501 ✅** | 18483.0563 | +19176.4632 | Higher | +8620.3570 | Higher | -10556.1062 | Lower |
| SLOC | 38 | 29 | **28 ✅** | +9 | Higher | +10 | Higher | +1 | Higher |
| PLOC | 28 | **18 ✅** | 21 | +10 | Higher | +7 | Higher | -3 | Lower |
| LLOC | 19 | **12 ✅** | 18 | +7 | Higher | +1 | Higher | -6 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 3 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 8 | **4 ✅** | 5 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02585

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.7273 | **72.9047 ✅** | 55.1846 | -18.1774 | Lower | -0.4573 | Lower | +17.7201 | Higher |
| MI (SEI) | 19.5703 | **67.8084 ✅** | 5.5429 | -48.2381 | Lower | +14.0275 | Higher | +62.2655 | Higher |
| Cyclomatic (sum) | 16 | **8 ✅** | 16 | +8 | Higher | 0 | Same | -8 | Lower |
| Cognitive (sum) | 26 | **8 ✅** | 24 | +18 | Higher | +2 | Higher | -16 | Lower |
| Halstead Volume | 2869.9226 | **1116.9445 ✅** | 3077.4900 | +1752.9781 | Higher | -207.5674 | Lower | -1960.5455 | Lower |
| Halstead Effort | 239595.0552 | **38394.9668 ✅** | 301010.3581 | +201200.0884 | Higher | -61415.3029 | Lower | -262615.3913 | Lower |
| SLOC | 81 | **40 ✅** | 77 | +41 | Higher | +4 | Higher | -37 | Lower |
| PLOC | 64 | **28 ✅** | 62 | +36 | Higher | +2 | Higher | -34 | Lower |
| LLOC | 67 | **21 ✅** | 64 | +46 | Higher | +3 | Higher | -43 | Lower |
| Comments (CLOC) | 3 | **12 ✅** | 0 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | 15 | **6 ✅** | 15 | +9 | Higher | 0 | Same | -9 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

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

## p02617

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1843 | 76.3324 | **76.8035 ✅** | -2.1482 | Lower | -2.6193 | Lower | -0.4711 | Lower |
| MI (SEI) | 62.4460 | **70.9876 ✅** | 60.9480 | -8.5416 | Lower | +1.4980 | Higher | +10.0396 | Higher |
| Cyclomatic (sum) | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 1032.3814 | **841.1357 ✅** | 999.8260 | +191.2457 | Higher | +32.5554 | Higher | -158.6903 | Lower |
| Halstead Effort | 28291.2214 | **15798.7221 ✅** | 26828.6642 | +12492.4993 | Higher | +1462.5572 | Higher | -11029.9421 | Lower |
| SLOC | 39 | 37 | **34 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| PLOC | 23 | **20 ✅** | 23 | +3 | Higher | 0 | Same | -3 | Lower |
| LLOC | 18 | **16 ✅** | 19 | +2 | Higher | -1 | Lower | -3 | Lower |
| Comments (CLOC) | 7 | **10 ✅** | 4 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02618

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.1632 | **63.2084 ✅** | 49.1943 | -12.0452 | Lower | +1.9689 | Higher | +14.0141 | Higher |
| MI (SEI) | 17.4090 | **49.5569 ✅** | 10.5273 | -32.1479 | Lower | +6.8817 | Higher | +39.0296 | Higher |
| Cyclomatic (sum) | 16 | **7 ✅** | 19 | +9 | Higher | -3 | Lower | -12 | Lower |
| Cognitive (sum) | 13 | **5 ✅** | 16 | +8 | Higher | -3 | Lower | -11 | Lower |
| Halstead Volume | 3830.4429 | **1830 ✅** | 3897.9808 | +2000.4429 | Higher | -67.5379 | Lower | -2067.9808 | Lower |
| Halstead Effort | 189889.4997 | **48707.7907 ✅** | 193557.1129 | +141181.7090 | Higher | -3667.6132 | Lower | -144849.3222 | Lower |
| SLOC | 92 | **63 ✅** | 99 | +29 | Higher | -7 | Lower | -36 | Lower |
| PLOC | 74 | **40 ✅** | 80 | +34 | Higher | -6 | Lower | -40 | Lower |
| LLOC | 52 | **24 ✅** | 59 | +28 | Higher | -7 | Lower | -35 | Lower |
| Comments (CLOC) | 5 | **14 ✅** | 3 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 18 | **11 ✅** | 19 | +7 | Higher | -1 | Lower | -8 | Lower |
| Functions | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p02639

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **90.7129 ✅** | 82.6432 | 77.2988 | +8.0697 | Higher | +13.4141 | Higher | +5.3444 | Higher |
| MI (SEI) | **91.3626 ✅** | 79.1104 | 64.9890 | +12.2522 | Higher | +26.3736 | Higher | +14.1214 | Higher |
| Cyclomatic (sum) | **5 ✅** | 6 | 8 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | **3 ✅** | 6 | 5 | -3 | Lower | -2 | Lower | +1 | Higher |
| Halstead Volume | **422.2594 ✅** | 569.7578 | 873.6286 | -147.4984 | Lower | -451.3692 | Lower | -303.8708 | Lower |
| Halstead Effort | **6150.9122 ✅** | 8917.9482 | 15422.9054 | -2767.0360 | Lower | -9271.9932 | Lower | -6504.9572 | Lower |
| SLOC | **19 ✅** | 28 | 33 | -9 | Lower | -14 | Lower | -5 | Lower |
| PLOC | **15 ✅** | 19 | 24 | -4 | Lower | -9 | Lower | -5 | Lower |
| LLOC | **7 ✅** | 12 | 15 | -5 | Lower | -8 | Lower | -3 | Lower |
| Comments (CLOC) | 5 | **7 ✅** | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02644

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.0956 | 56.8825 | **58.3835 ✅** | +0.2131 | Higher | -1.2879 | Lower | -1.5010 | Lower |
| MI (SEI) | 30.2081 | **37.3024 ✅** | 10.2596 | -7.0943 | Lower | +19.9485 | Higher | +27.0428 | Higher |
| Cyclomatic (sum) | **16 ✅** | 17 | 17 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 26 | 26 | **21 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Halstead Volume | 2867.7987 | **2404.1045 ✅** | 3125.9343 | +463.6943 | Higher | -258.1356 | Lower | -721.8299 | Lower |
| Halstead Effort | **124629.7531 ✅** | 161293.5546 | 167819.0563 | -36663.8015 | Lower | -43189.3032 | Lower | -6525.5017 | Lower |
| SLOC | 70 | 74 | **62 ✅** | -4 | Lower | +8 | Higher | +12 | Higher |
| PLOC | 54 | 54 | **51 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| LLOC | 39 | **29 ✅** | 42 | +10 | Higher | -3 | Lower | -13 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 0 | -6 | Lower | +6 | Higher | +12 | Higher |
| Blank Lines | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.9725 | **75.7403 ✅** | 65.9692 | -11.7678 | Lower | -1.9967 | Lower | +9.7711 | Higher |
| MI (SEI) | 46.6857 | **72.8908 ✅** | 20.5926 | -26.2051 | Lower | +26.0931 | Higher | +52.2982 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 11 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 20 | **11 ✅** | 20 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 1729.8320 | **862.7761 ✅** | 1639.3309 | +867.0559 | Higher | +90.5011 | Higher | -776.5548 | Lower |
| Halstead Effort | 80168.7647 | **26353.8880 ✅** | 74062.6292 | +53814.8767 | Higher | +6106.1355 | Higher | -47708.7412 | Lower |
| SLOC | 57 | **37 ✅** | 52 | +20 | Higher | +5 | Higher | -15 | Lower |
| PLOC | 49 | **21 ✅** | 47 | +28 | Higher | +2 | Higher | -26 | Lower |
| LLOC | 42 | **16 ✅** | 43 | +26 | Higher | -1 | Lower | -27 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 0 | -3 | Lower | +9 | Higher | +12 | Higher |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02668

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.5846 | **71.3372 ✅** | 46.4609 | -21.7526 | Lower | +3.1236 | Higher | +24.8763 | Higher |
| MI (SEI) | 11.8804 | **67.3295 ✅** | -6.6355 | -55.4491 | Lower | +18.5159 | Higher | +73.9650 | Higher |
| Cyclomatic (sum) | 22 | **11 ✅** | 20 | +11 | Higher | +2 | Higher | -9 | Lower |
| Cognitive (sum) | 19 | **12 ✅** | 17 | +7 | Higher | +2 | Higher | -5 | Lower |
| Halstead Volume | 3979.6016 | **916.1414 ✅** | 4177.9612 | +3063.4602 | Higher | -198.3595 | Lower | -3261.8198 | Lower |
| Halstead Effort | 380634.3360 | **42727.8166 ✅** | 305974.2138 | +337906.5194 | Higher | +74660.1222 | Higher | -263246.3971 | Lower |
| SLOC | 92 | **45 ✅** | 113 | +47 | Higher | -21 | Lower | -68 | Lower |
| PLOC | 78 | **29 ✅** | 93 | +49 | Higher | -15 | Lower | -64 | Lower |
| LLOC | 60 | **20 ✅** | 55 | +40 | Higher | +5 | Higher | -35 | Lower |
| Comments (CLOC) | 3 | **15 ✅** | 0 | -12 | Lower | +3 | Higher | +15 | Higher |
| Blank Lines | 14 | **7 ✅** | 20 | +7 | Higher | -6 | Lower | -13 | Lower |
| Functions | 6 | **2 ✅** | 7 | +4 | Higher | -1 | Lower | -5 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |

## p02678

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.5423 | **63.2230 ✅** | 60.8685 | -5.6807 | Lower | -3.3262 | Lower | +2.3546 | Higher |
| MI (SEI) | 27.4327 | **49.2146 ✅** | 22.3352 | -21.7819 | Lower | +5.0975 | Higher | +26.8794 | Higher |
| Cyclomatic (sum) | 11 | 9 | **8 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | 8 | 11 | 8 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 2252.3613 | **1846.9795 ✅** | 2266.6478 | +405.3818 | Higher | -14.2865 | Lower | -419.6683 | Lower |
| Halstead Effort | 71870.7999 | **50048.6395 ✅** | 87662.6035 | +21822.1604 | Higher | -15791.8035 | Lower | -37613.9639 | Lower |
| SLOC | 79 | **61 ✅** | 67 | +18 | Higher | +12 | Higher | -6 | Lower |
| PLOC | 64 | **43 ✅** | 56 | +21 | Higher | +8 | Higher | -13 | Lower |
| LLOC | 37 | **33 ✅** | 47 | +4 | Higher | -10 | Lower | -14 | Lower |
| Comments (CLOC) | 5 | **13 ✅** | 1 | -8 | Lower | +4 | Higher | +12 | Higher |
| Blank Lines | 15 | **8 ✅** | 11 | +7 | Higher | +4 | Higher | -3 | Lower |
| Functions | 4 | **1 ✅** | 2 | +3 | Higher | +2 | Higher | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |

## p02727

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **54.9424 ✅** | 46.1692 | 54.6338 | +8.7732 | Higher | +0.3086 | Higher | -8.4646 | Lower |
| MI (SEI) | **28.7986 ✅** | 18.7386 | 19.8296 | +10.0600 | Higher | +8.9690 | Higher | -1.0910 | Lower |
| Cyclomatic (sum) | 24 | 24 | **19 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| Cognitive (sum) | **16 ✅** | 37 | 23 | -21 | Lower | -7 | Lower | +14 | Higher |
| Halstead Volume | **2456.7661 ✅** | 4026.2233 | 2659.8896 | -1569.4572 | Lower | -203.1234 | Lower | +1366.3338 | Higher |
| Halstead Effort | 106711.8413 | **101623.4251 ✅** | 132706.9227 | +5088.4162 | Higher | -25995.0814 | Lower | -31083.4976 | Lower |
| SLOC | **75 ✅** | 110 | 80 | -35 | Lower | -5 | Lower | +30 | Higher |
| PLOC | **57 ✅** | 86 | 69 | -29 | Lower | -12 | Lower | +17 | Higher |
| LLOC | **32 ✅** | 56 | 50 | -24 | Lower | -18 | Lower | +6 | Higher |
| Comments (CLOC) | 7 | **13 ✅** | 3 | -6 | Lower | +4 | Higher | +10 | Higher |
| Blank Lines | 13 | 14 | **11 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 4 | 3 | **1 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Exits | 11 | **0 ✅** | 1 | +11 | Higher | +10 | Higher | -1 | Lower |

## p02728

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.3395 | **66.2037 ✅** | 42.3415 | -24.8642 | Lower | -1.0021 | Lower | +23.8621 | Higher |
| MI (SEI) | -6.7651 | **57.5589 ✅** | -12.2731 | -64.3240 | Lower | +5.5079 | Higher | +69.8320 | Higher |
| Cyclomatic (sum) | 24 | **9 ✅** | 23 | +15 | Higher | +1 | Higher | -14 | Lower |
| Cognitive (sum) | 16 | **12 ✅** | 15 | +4 | Higher | +1 | Higher | -3 | Lower |
| Halstead Volume | 6513.7715 | **1359.0516 ✅** | 6203.6920 | +5154.7199 | Higher | +310.0795 | Higher | -4844.6404 | Lower |
| Halstead Effort | 765618.6755 | **33553.8818 ✅** | 756982.4162 | +732064.7936 | Higher | +8636.2593 | Higher | -723428.5344 | Lower |
| SLOC | 127 | **56 ✅** | 123 | +71 | Higher | +4 | Higher | -67 | Lower |
| PLOC | 111 | **39 ✅** | 102 | +72 | Higher | +9 | Higher | -63 | Lower |
| LLOC | 91 | **24 ✅** | 85 | +67 | Higher | +6 | Higher | -61 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 0 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 16 | **9 ✅** | 21 | +7 | Higher | -5 | Lower | -12 | Lower |
| Functions | 11 | **2 ✅** | 11 | +9 | Higher | 0 | Same | -9 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |

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

## p02733

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 50.4408 | **64.8355 ✅** | 49.3595 | -14.3947 | Lower | +1.0813 | Higher | +15.4760 | Higher |
| MI (SEI) | 34.8784 | **51.2086 ✅** | 11.4426 | -16.3301 | Lower | +23.4358 | Higher | +39.7659 | Higher |
| Cyclomatic (sum) | 25 | **10 ✅** | 25 | +15 | Higher | 0 | Same | -15 | Lower |
| Cognitive (sum) | 68 | **20 ✅** | 66 | +48 | Higher | +2 | Higher | -46 | Lower |
| Halstead Volume | 2764.4248 | **1172.0182 ✅** | 2989.0065 | +1592.4066 | Higher | -224.5817 | Lower | -1816.9883 | Lower |
| Halstead Effort | 175738.4336 | **43317.7944 ✅** | 195901.1048 | +132420.6392 | Higher | -20162.6712 | Lower | -152583.3104 | Lower |
| SLOC | 94 | **63 ✅** | 98 | +31 | Higher | -4 | Lower | -35 | Lower |
| PLOC | 78 | **44 ✅** | 90 | +34 | Higher | -12 | Lower | -46 | Lower |
| LLOC | 72 | **33 ✅** | 88 | +39 | Higher | -16 | Lower | -55 | Lower |
| Comments (CLOC) | **24 ✅** | 13 | 3 | +11 | Higher | +21 | Higher | +10 | Higher |
| Blank Lines | 12 | **7 ✅** | 8 | +5 | Higher | +4 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02734

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.3283 | **81.9968 ✅** | 65.8883 | -14.6685 | Lower | +1.4400 | Higher | +16.1085 | Higher |
| MI (SEI) | 22.3497 | **83.6670 ✅** | 39.0744 | -61.3173 | Lower | -16.7247 | Lower | +44.5926 | Higher |
| Cyclomatic (sum) | 9 | **6 ✅** | 10 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 12 | **6 ✅** | 12 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 2020.5005 | **722.5705 ✅** | 2094.2868 | +1297.9300 | Higher | -73.7863 | Lower | -1371.7162 | Lower |
| Halstead Effort | 145476.0371 | **19148.1192 ✅** | 160343.8310 | +126327.9180 | Higher | -14867.7938 | Lower | -141195.7118 | Lower |
| SLOC | 46 | **27 ✅** | 49 | +19 | Higher | -3 | Lower | -22 | Lower |
| PLOC | 40 | **20 ✅** | 39 | +20 | Higher | +1 | Higher | -19 | Lower |
| LLOC | 41 | **14 ✅** | 41 | +27 | Higher | 0 | Same | -27 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 3 | -10 | Lower | -3 | Lower | +7 | Higher |
| Blank Lines | 6 | **3 ✅** | 7 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02743

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.5331 | **90.2487 ✅** | 84.1286 | -4.7157 | Lower | +1.4045 | Higher | +6.1202 | Higher |
| MI (SEI) | 78.2067 | **95.7577 ✅** | 80.5880 | -17.5509 | Lower | -2.3813 | Lower | +15.1697 | Higher |
| Cyclomatic (sum) | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 659.0224 | **353.2961 ✅** | 637.0550 | +305.7262 | Higher | +21.9674 | Higher | -283.7588 | Lower |
| Halstead Effort | 9872.6621 | **4327.8775 ✅** | 10447.7013 | +5544.7846 | Higher | -575.0393 | Lower | -6119.8238 | Lower |
| SLOC | 23 | **21 ✅** | 25 | +2 | Higher | -2 | Lower | -4 | Lower |
| PLOC | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| LLOC | 10 | **6 ✅** | 8 | +4 | Higher | +2 | Higher | -2 | Lower |
| Comments (CLOC) | 4 | **8 ✅** | 6 | -4 | Lower | -2 | Lower | +2 | Higher |
| Blank Lines | 4 | **3 ✅** | 5 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02761

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.9538 | **65.0250 ✅** | 61.5771 | -3.0711 | Lower | +0.3767 | Higher | +3.4478 | Higher |
| MI (SEI) | 25.8914 | **55.3170 ✅** | 15.0707 | -29.4256 | Lower | +10.8207 | Higher | +40.2463 | Higher |
| Cyclomatic (sum) | 19 | **17 ✅** | 19 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 24 | **16 ✅** | 25 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 1977.3497 | **1340.3334 ✅** | 2011.8639 | +637.0163 | Higher | -34.5142 | Lower | -671.5305 | Lower |
| Halstead Effort | 115504.4971 | **62634.8102 ✅** | 106094.3866 | +52869.6870 | Higher | +9410.1106 | Higher | -43459.5764 | Lower |
| SLOC | 56 | **54 ✅** | 57 | +2 | Higher | -1 | Lower | -3 | Lower |
| PLOC | 49 | **36 ✅** | 50 | +13 | Higher | -1 | Lower | -14 | Lower |
| LLOC | 39 | **25 ✅** | 38 | +14 | Higher | +1 | Higher | -13 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 0 | -13 | Lower | +1 | Higher | +14 | Higher |
| Blank Lines | 7 | 8 | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 6 | 0 | -6 | Lower | 0 | Same | +6 | Higher |

## p02762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 51.2498 | **54.1127 ✅** | 51.4539 | -2.8629 | Lower | -0.2040 | Lower | +2.6589 | Higher |
| MI (SEI) | -0.1339 | **32.9708 ✅** | 8.2012 | -33.1046 | Lower | -8.3350 | Lower | +24.7696 | Higher |
| Cyclomatic (sum) | 16 | 18 | 16 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 12 | **8 ✅** | 15 | +4 | Higher | -3 | Lower | -7 | Lower |
| Halstead Volume | 3523.0272 | **2740.1774 ✅** | 3622.2011 | +782.8498 | Higher | -99.1739 | Lower | -882.0237 | Lower |
| Halstead Effort | 199755.6409 | **136095.4753 ✅** | 233505.6121 | +63660.1656 | Higher | -33749.9712 | Lower | -97410.1368 | Lower |
| SLOC | 94 | **83 ✅** | 92 | +11 | Higher | +2 | Higher | -9 | Lower |
| PLOC | 77 | **67 ✅** | 82 | +10 | Higher | -5 | Lower | -15 | Lower |
| LLOC | 68 | **40 ✅** | 71 | +28 | Higher | -3 | Lower | -31 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 1 | -13 | Lower | -1 | Lower | +12 | Higher |
| Blank Lines | 17 | 13 | **10 ✅** | +4 | Higher | +7 | Higher | +3 | Higher |
| Functions | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p02831

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.7024 | 68.1160 | **89.9339 ✅** | +0.5864 | Higher | -21.2315 | Lower | -21.8179 | Lower |
| MI (SEI) | 43.3380 | 55.4102 | **83.3255 ✅** | -12.0722 | Lower | -39.9875 | Lower | -27.9153 | Lower |
| Cyclomatic (sum) | 12 | 14 | **4 ✅** | -2 | Lower | +8 | Higher | +10 | Higher |
| Cognitive (sum) | 7 | 9 | **3 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| Halstead Volume | 1115.8024 | 950.0527 | **512.6840 ✅** | +165.7498 | Higher | +603.1184 | Higher | +437.3686 | Higher |
| Halstead Effort | 31530.4169 | 25255.5665 | **7792.7970 ✅** | +6274.8505 | Higher | +23737.6199 | Higher | +17462.7694 | Higher |
| SLOC | 49 | 52 | **19 ✅** | -3 | Lower | +30 | Higher | +33 | Higher |
| PLOC | 41 | 40 | **14 ✅** | +1 | Higher | +27 | Higher | +26 | Higher |
| LLOC | 24 | 20 | **9 ✅** | +4 | Higher | +15 | Higher | +11 | Higher |
| Comments (CLOC) | 3 | **10 ✅** | 3 | -7 | Lower | 0 | Same | +7 | Higher |
| Blank Lines | 5 | 5 | **3 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 2 | 5 | **0 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |

## p02837

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.5300 | 59.9027 | **63.0172 ✅** | +0.6272 | Higher | -2.4872 | Lower | -3.1144 | Lower |
| MI (SEI) | 33.6871 | **46.5216 ✅** | 26.7402 | -12.8345 | Lower | +6.9469 | Higher | +19.7814 | Higher |
| Cyclomatic (sum) | 15 | 14 | 14 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 26 | 24 | **23 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Halstead Volume | 1774.9682 | **1747.1450 ✅** | 1802.3630 | +27.8233 | Higher | -27.3948 | Lower | -55.2180 | Lower |
| Halstead Effort | 88630.0809 | **58452.2771 ✅** | 94841.5848 | +30177.8038 | Higher | -6211.5040 | Lower | -36389.3077 | Lower |
| SLOC | 67 | 71 | **58 ✅** | -4 | Lower | +9 | Higher | +13 | Higher |
| PLOC | 53 | **49 ✅** | 50 | +4 | Higher | +3 | Higher | -1 | Lower |
| LLOC | 36 | 37 | **35 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Comments (CLOC) | 5 | **17 ✅** | 1 | -12 | Lower | +4 | Higher | +16 | Higher |
| Blank Lines | 9 | 13 | **7 ✅** | -4 | Lower | +2 | Higher | +6 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p02840

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.0125 | **75.0573 ✅** | 48.5608 | -28.0447 | Lower | -1.5482 | Lower | +26.4965 | Higher |
| MI (SEI) | -5.6361 | **75.0194 ✅** | -3.5043 | -80.6555 | Lower | -2.1318 | Lower | +78.5237 | Higher |
| Cyclomatic (sum) | 22 | **9 ✅** | 21 | +13 | Higher | +1 | Higher | -12 | Lower |
| Cognitive (sum) | 25 | **7 ✅** | 31 | +18 | Higher | -6 | Lower | -24 | Lower |
| Halstead Volume | 3740.0861 | **706.3936 ✅** | 3786.8349 | +3033.6925 | Higher | -46.7488 | Lower | -3080.4413 | Lower |
| Halstead Effort | 288136.2322 | **24488.3109 ✅** | 309258.1859 | +263647.9213 | Higher | -21121.9537 | Lower | -284769.8750 | Lower |
| SLOC | 110 | **40 ✅** | 101 | +70 | Higher | +9 | Higher | -61 | Lower |
| PLOC | 94 | **27 ✅** | 89 | +67 | Higher | +5 | Higher | -62 | Lower |
| LLOC | 63 | **18 ✅** | 76 | +45 | Higher | -13 | Lower | -58 | Lower |
| Comments (CLOC) | 0 | **16 ✅** | 0 | -16 | Lower | 0 | Same | +16 | Higher |
| Blank Lines | 16 | **6 ✅** | 12 | +10 | Higher | +4 | Higher | -6 | Lower |
| Functions | 3 | 2 | 2 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p02846

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.5082 | **84.1744 ✅** | 62.1077 | -18.6662 | Lower | +3.4005 | Higher | +22.0667 | Higher |
| MI (SEI) | 44.6219 | **92.7412 ✅** | 33.6835 | -48.1193 | Lower | +10.9384 | Higher | +59.0577 | Higher |
| Cyclomatic (sum) | 9 | 7 | **6 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | 7 | **4 ✅** | 7 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1844.2537 | **656.3911 ✅** | 2484.5223 | +1187.8626 | Higher | -640.2686 | Lower | -1828.1312 | Lower |
| Halstead Effort | 69591.7625 | **14302.4171 ✅** | 172260.2128 | +55289.3454 | Higher | -102668.4502 | Lower | -157957.7957 | Lower |
| SLOC | 53 | **24 ✅** | 62 | +29 | Higher | -9 | Lower | -38 | Lower |
| PLOC | 39 | **17 ✅** | 50 | +22 | Higher | -11 | Lower | -33 | Lower |
| LLOC | 28 | **11 ✅** | 53 | +17 | Higher | -25 | Lower | -42 | Lower |
| Comments (CLOC) | 6 | **14 ✅** | 4 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 8 | **3 ✅** | 8 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p02862

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **58.3470 ✅** | 56.5710 | 55.9709 | +1.7760 | Higher | +2.3761 | Higher | +0.6001 | Higher |
| MI (SEI) | 18.9796 | **36.5738 ✅** | 19.1547 | -17.5942 | Lower | -0.1751 | Lower | +17.4191 | Higher |
| Cyclomatic (sum) | 14 | 14 | 14 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 7 | 9 | **6 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Halstead Volume | **2256 ✅** | 2286.2226 | 3412.9211 | -30.2226 | Lower | -1156.9211 | Lower | -1126.6985 | Lower |
| Halstead Effort | 121072 | 126299.1415 | **98607.9020 ✅** | -5227.1415 | Lower | +22464.0980 | Higher | +27691.2394 | Higher |
| SLOC | **72 ✅** | 80 | 73 | -8 | Lower | -1 | Lower | +7 | Higher |
| PLOC | **57 ✅** | 59 | 59 | -2 | Lower | -2 | Lower | 0 | Same |
| LLOC | 40 | 40 | **38 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | 1 | **13 ✅** | 2 | -12 | Lower | -1 | Lower | +11 | Higher |
| Blank Lines | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p02884

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.1714 | **56.8675 ✅** | 51.1427 | -4.6961 | Lower | +1.0287 | Higher | +5.7248 | Higher |
| MI (SEI) | 23.6584 | **29.2871 ✅** | 19.5953 | -5.6287 | Lower | +4.0631 | Higher | +9.6917 | Higher |
| Cyclomatic (sum) | 20 | **17 ✅** | 23 | +3 | Higher | -3 | Lower | -6 | Lower |
| Cognitive (sum) | 35 | **17 ✅** | 39 | +18 | Higher | -4 | Lower | -22 | Lower |
| Halstead Volume | 2643.6632 | **2312.3049 ✅** | 2728.1392 | +331.3582 | Higher | -84.4760 | Lower | -415.8343 | Lower |
| Halstead Effort | 153699.6388 | **76158.4683 ✅** | 196756.7042 | +77541.1705 | Higher | -43057.0653 | Lower | -120598.2358 | Lower |
| SLOC | 92 | **75 ✅** | 93 | +17 | Higher | -1 | Lower | -18 | Lower |
| PLOC | 68 | **58 ✅** | 73 | +10 | Higher | -5 | Lower | -15 | Lower |
| LLOC | 63 | **31 ✅** | 67 | +32 | Higher | -4 | Lower | -36 | Lower |
| Comments (CLOC) | **8 ✅** | 6 | 6 | +2 | Higher | +2 | Higher | 0 | Same |
| Blank Lines | 17 | **12 ✅** | 18 | +5 | Higher | -1 | Lower | -6 | Lower |
| Functions | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p02885

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 86.3656 | 81.2820 | **88.9578 ✅** | +5.0836 | Higher | -2.5923 | Lower | -7.6758 | Lower |
| MI (SEI) | 87.0228 | 80.2679 | **87.9150 ✅** | +6.7548 | Higher | -0.8922 | Lower | -7.6470 | Lower |
| Cyclomatic (sum) | 4 | 7 | **3 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 561.5282 | 634.8730 | **551.0324 ✅** | -73.3447 | Lower | +10.4959 | Higher | +83.8406 | Higher |
| Halstead Effort | 9096.7576 | 11514.2872 | **7013.1395 ✅** | -2417.5296 | Lower | +2083.6181 | Higher | +4501.1477 | Higher |
| SLOC | 23 | 29 | **20 ✅** | -6 | Lower | +3 | Higher | +9 | Higher |
| PLOC | 12 | 19 | **11 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| LLOC | 8 | 9 | 8 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 7 | **9 ✅** | 5 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p02923

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.5440 ✅** | 68.5414 | 54.1012 | +7.0026 | Higher | +21.4428 | Higher | +14.4402 | Higher |
| MI (SEI) | **66.4179 ✅** | 58.9694 | 16.0900 | +7.4485 | Higher | +50.3279 | Higher | +42.8794 | Higher |
| Cyclomatic (sum) | **5 ✅** | 8 | 20 | -3 | Lower | -15 | Lower | -12 | Lower |
| Cognitive (sum) | **5 ✅** | 7 | 15 | -2 | Lower | -10 | Lower | -8 | Lower |
| Halstead Volume | **900.8062 ✅** | 906.1558 | 2250.5191 | -5.3495 | Lower | -1349.7129 | Lower | -1344.3633 | Lower |
| Halstead Effort | **23270.8273 ✅** | 29450.0622 | 109063.6174 | -6179.2350 | Lower | -85792.7901 | Lower | -79613.5552 | Lower |
| SLOC | **38 ✅** | 56 | 86 | -18 | Lower | -48 | Lower | -30 | Lower |
| PLOC | **26 ✅** | 34 | 74 | -8 | Lower | -48 | Lower | -40 | Lower |
| LLOC | 24 | **20 ✅** | 47 | +4 | Higher | -23 | Lower | -27 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 2 | -6 | Lower | +6 | Higher | +12 | Higher |
| Blank Lines | **7 ✅** | 9 | 12 | -2 | Lower | -5 | Lower | -3 | Lower |
| Functions | **1 ✅** | 2 | 9 | -1 | Lower | -8 | Lower | -7 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | **0 ✅** | 2 | 5 | -2 | Lower | -5 | Lower | -3 | Lower |

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

## p02951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.5202 | 79.3096 | **84.3477 ✅** | -1.7894 | Lower | -6.8275 | Lower | -5.0381 | Lower |
| MI (SEI) | 66.3045 | 73.7742 | **78.4318 ✅** | -7.4697 | Lower | -12.1273 | Lower | -4.6576 | Lower |
| Cyclomatic (sum) | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 4 | **2 ✅** | 3 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 697.4653 | 677.6598 | **610.7609 ✅** | +19.8055 | Higher | +86.7044 | Higher | +66.8989 | Higher |
| Halstead Effort | 13121.0668 | **11294.3307 ✅** | 11342.7030 | +1826.7361 | Higher | +1778.3638 | Higher | -48.3723 | Lower |
| SLOC | 36 | 33 | **25 ✅** | +3 | Higher | +11 | Higher | +8 | Higher |
| PLOC | 22 | 19 | **18 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| LLOC | 16 | **12 ✅** | 14 | +4 | Higher | +2 | Higher | -2 | Lower |
| Comments (CLOC) | 6 | **8 ✅** | 5 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 8 | 7 | **6 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p02953

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.1687 | **87.4193 ✅** | 79.4909 | -14.2506 | Lower | -6.3222 | Lower | +7.9284 | Higher |
| MI (SEI) | 61.0601 | **94.8204 ✅** | 73.1636 | -33.7604 | Lower | -12.1036 | Lower | +21.6568 | Higher |
| Cyclomatic (sum) | 10 | **5 ✅** | 6 | +5 | Higher | +4 | Higher | -1 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 6 | +11 | Higher | +8 | Higher | -3 | Lower |
| Halstead Volume | 971.7788 | **438.6884 ✅** | 760.7724 | +533.0904 | Higher | +211.0064 | Higher | -322.0840 | Lower |
| Halstead Effort | 27061.9278 | **6668.0634 ✅** | 15768.7367 | +20393.8644 | Higher | +11293.1911 | Higher | -9100.6733 | Lower |
| SLOC | 40 | **23 ✅** | 31 | +17 | Higher | +9 | Higher | -8 | Lower |
| PLOC | 32 | **16 ✅** | 22 | +16 | Higher | +10 | Higher | -6 | Lower |
| LLOC | 19 | **10 ✅** | 16 | +9 | Higher | +3 | Higher | -6 | Lower |
| Comments (CLOC) | 7 | **11 ✅** | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 5 | **3 ✅** | 4 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 2 | **1 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

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

## p02959

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.1842 | 64.1201 | **72.8295 ✅** | +7.0641 | Higher | -1.6453 | Lower | -8.7094 | Lower |
| MI (SEI) | **63.9669 ✅** | 48.9082 | 63.2691 | +15.0587 | Higher | +0.6979 | Higher | -14.3609 | Lower |
| Cyclomatic (sum) | 10 | **8 ✅** | 9 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 18 | **3 ✅** | 15 | +15 | Higher | +3 | Higher | -12 | Lower |
| Halstead Volume | **1317.9578 ✅** | 1469.2675 | 1382.2484 | -151.3097 | Lower | -64.2906 | Lower | +87.0191 | Higher |
| Halstead Effort | **60877.0981 ✅** | 63913.1354 | 64504.9261 | -3036.0373 | Lower | -3627.8281 | Lower | -591.7907 | Lower |
| SLOC | 41 | 63 | **37 ✅** | -22 | Lower | +4 | Higher | +26 | Higher |
| PLOC | 25 | 41 | **24 ✅** | -16 | Lower | +1 | Higher | +17 | Higher |
| LLOC | **25 ✅** | 27 | 26 | -2 | Lower | -1 | Lower | +1 | Higher |
| Comments (CLOC) | 11 | **12 ✅** | 8 | -1 | Lower | +3 | Higher | +4 | Higher |
| Blank Lines | 9 | 11 | **8 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p02988

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.4565 | 74.0591 | **75.2258 ✅** | +0.3974 | Higher | -0.7692 | Lower | -1.1667 | Lower |
| MI (SEI) | 53.9816 | **69.5506 ✅** | 62.8498 | -15.5690 | Lower | -8.8682 | Lower | +6.7008 | Higher |
| Cyclomatic (sum) | 9 | 10 | **8 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 7 | **6 ✅** | 11 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | 1010.8782 | 1043.9532 | **911.3016 ✅** | -33.0750 | Lower | +99.5766 | Higher | +132.6516 | Higher |
| Halstead Effort | 33015.2824 | 42255.2490 | **27794.7002 ✅** | -9239.9665 | Lower | +5220.5823 | Higher | +14460.5488 | Higher |
| SLOC | 37 | 37 | 37 | 0 | Same | 0 | Same | 0 | Same |
| PLOC | 26 | 26 | **24 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| LLOC | 20 | 18 | 18 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 3 | **11 ✅** | 6 | -8 | Lower | -3 | Lower | +5 | Higher |
| Blank Lines | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p03003

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3564 | **75.1626 ✅** | 68.1363 | -9.8062 | Lower | -2.7799 | Lower | +7.0263 | Higher |
| MI (SEI) | 44.8225 | **68.4199 ✅** | 48.3048 | -23.5974 | Lower | -3.4822 | Lower | +20.1152 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 10 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 16 | **9 ✅** | 12 | +7 | Higher | +4 | Higher | -3 | Lower |
| Halstead Volume | 1844.3904 | **1050.0802 ✅** | 1772.1708 | +794.3102 | Higher | +72.2195 | Higher | -722.0907 | Lower |
| Halstead Effort | 100348.4992 | **44103.3681 ✅** | 95992.5874 | +56245.1311 | Higher | +4355.9118 | Higher | -51889.2193 | Lower |
| SLOC | 52 | **36 ✅** | 45 | +16 | Higher | +7 | Higher | -9 | Lower |
| PLOC | 38 | **24 ✅** | 32 | +14 | Higher | +6 | Higher | -8 | Lower |
| LLOC | 33 | **18 ✅** | 27 | +15 | Higher | +6 | Higher | -9 | Lower |
| Comments (CLOC) | 6 | **9 ✅** | 5 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p03034

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.2276 | **78.9333 ✅** | 71.8932 | -10.7057 | Lower | -3.6656 | Lower | +7.0401 | Higher |
| MI (SEI) | 42.2706 | **76.7730 ✅** | 56.1801 | -34.5024 | Lower | -13.9095 | Lower | +20.5929 | Higher |
| Cyclomatic (sum) | 10 | **5 ✅** | 8 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 5 | +4 | Higher | +1 | Higher | -3 | Lower |
| Halstead Volume | 1254.0714 | **801.8106 ✅** | 1083.0835 | +452.2608 | Higher | +170.9878 | Higher | -281.2730 | Lower |
| Halstead Effort | 44582.2366 | **20696.7349 ✅** | 36283.2988 | +23885.5017 | Higher | +8298.9378 | Higher | -15586.5639 | Lower |
| SLOC | 50 | **32 ✅** | 43 | +18 | Higher | +7 | Higher | -11 | Lower |
| PLOC | 38 | **19 ✅** | 29 | +19 | Higher | +9 | Higher | -10 | Lower |
| LLOC | 31 | **16 ✅** | 27 | +15 | Higher | +4 | Higher | -11 | Lower |
| Comments (CLOC) | 3 | **10 ✅** | 6 | -7 | Lower | -3 | Lower | +4 | Higher |
| Blank Lines | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03043

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.8140 | 77.5713 | **79.1284 ✅** | +0.2427 | Higher | -1.3144 | Lower | -1.5571 | Lower |
| MI (SEI) | 55.9545 | **69.0429 ✅** | 52.9207 | -13.0884 | Lower | +3.0338 | Higher | +16.1222 | Higher |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 994.3939 | **862.5902 ✅** | 944.2801 | +131.8037 | Higher | +50.1138 | Higher | -81.6899 | Lower |
| Halstead Effort | 22871.0590 | **18689.4542 ✅** | 26243.1179 | +4181.6048 | Higher | -3372.0589 | Lower | -7553.6637 | Lower |
| SLOC | 32 | 34 | **30 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| PLOC | 23 | 23 | 24 | 0 | Same | -1 | Lower | -1 | Lower |
| LLOC | 20 | **14 ✅** | 23 | +6 | Higher | -3 | Lower | -9 | Lower |
| Comments (CLOC) | 2 | **7 ✅** | 1 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 7 | 7 | **5 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03051

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.2209 | **77.7095 ✅** | 60.2209 | -17.4885 | Lower | 0 | Same | +17.4885 | Higher |
| MI (SEI) | 12.4014 | **70.2738 ✅** | 12.4014 | -57.8724 | Lower | 0 | Same | +57.8724 | Higher |
| Cyclomatic (sum) | 12 | **5 ✅** | 12 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 13 | **4 ✅** | 13 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | 2480.8941 | **702.9572 ✅** | 2480.8941 | +1777.9369 | Higher | 0 | Same | -1777.9369 | Lower |
| Halstead Effort | 174564.7268 | **15069.6449 ✅** | 174564.7268 | +159495.0819 | Higher | 0 | Same | -159495.0819 | Lower |
| SLOC | 64 | **36 ✅** | 64 | +28 | Higher | 0 | Same | -28 | Lower |
| PLOC | 54 | **22 ✅** | 54 | +32 | Higher | 0 | Same | -32 | Lower |
| LLOC | 43 | **15 ✅** | 43 | +28 | Higher | 0 | Same | -28 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 10 | **6 ✅** | 10 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03053

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.2302 | 59.1905 | **63.9178 ✅** | +3.0397 | Higher | -1.6875 | Lower | -4.7272 | Lower |
| MI (SEI) | 39.1110 | **42.8551 ✅** | 32.6576 | -3.7441 | Lower | +6.4534 | Higher | +10.1974 | Higher |
| Cyclomatic (sum) | 13 | 16 | 13 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 16 | 24 | 16 | -8 | Lower | 0 | Same | +8 | Higher |
| Halstead Volume | **2078.0145 ✅** | 2197.0984 | 2098.0253 | -119.0839 | Lower | -20.0107 | Lower | +99.0732 | Higher |
| Halstead Effort | 77818.9794 | 112823.9739 | **74300.7968 ✅** | -35004.9945 | Lower | +3518.1826 | Higher | +38523.1771 | Higher |
| SLOC | 59 | 67 | **53 ✅** | -8 | Lower | +6 | Higher | +14 | Higher |
| PLOC | **43 ✅** | 50 | 45 | -7 | Lower | -2 | Lower | +5 | Higher |
| LLOC | 34 | **31 ✅** | 35 | +3 | Higher | -1 | Lower | -4 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 2 | -7 | Lower | +4 | Higher | +11 | Higher |
| Blank Lines | 10 | 10 | **8 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.6399 | **79.8234 ✅** | 65.9961 | -11.1835 | Lower | +2.6438 | Higher | +13.8273 | Higher |
| MI (SEI) | 51.4672 | **71.7009 ✅** | 49.6489 | -20.2338 | Lower | +1.8183 | Higher | +22.0520 | Higher |
| Cyclomatic (sum) | 5 | **3 ✅** | 8 | +2 | Higher | -3 | Lower | -5 | Lower |
| Cognitive (sum) | 4 | **1 ✅** | 8 | +3 | Higher | -4 | Lower | -7 | Lower |
| Halstead Volume | 1539.1187 | **558.3519 ✅** | 1563.7223 | +980.7668 | Higher | -24.6036 | Lower | -1005.3704 | Lower |
| Halstead Effort | 61034.0184 | **7178.8101 ✅** | 69110.9402 | +53855.2082 | Higher | -8076.9219 | Lower | -61932.1301 | Lower |
| SLOC | 49 | **35 ✅** | 55 | +14 | Higher | -6 | Lower | -20 | Lower |
| PLOC | 32 | **23 ✅** | 40 | +9 | Higher | -8 | Lower | -17 | Lower |
| LLOC | 34 | **11 ✅** | 39 | +23 | Higher | -5 | Lower | -28 | Lower |
| Comments (CLOC) | 7 | 7 | **9 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | 11 | **6 ✅** | 10 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.8482 | **73.1211 ✅** | 58.9980 | -13.2728 | Lower | +0.8502 | Higher | +14.1230 | Higher |
| MI (SEI) | 30.2115 | **72.0663 ✅** | 10.6371 | -41.8547 | Lower | +19.5744 | Higher | +61.4291 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 12 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | **9 ✅** | 10 | 11 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 2206.5419 | **818.2861 ✅** | 2374.1105 | +1388.2558 | Higher | -167.5686 | Lower | -1555.8244 | Lower |
| Halstead Effort | 88672.1949 | **29151.4427 ✅** | 105532.1073 | +59520.7521 | Higher | -16859.9125 | Lower | -76380.6646 | Lower |
| SLOC | 68 | **43 ✅** | 70 | +25 | Higher | -2 | Lower | -27 | Lower |
| PLOC | 58 | **31 ✅** | 61 | +27 | Higher | -3 | Lower | -30 | Lower |
| LLOC | 49 | **24 ✅** | 48 | +25 | Higher | +1 | Higher | -24 | Lower |
| Comments (CLOC) | 4 | **17 ✅** | 0 | -13 | Lower | +4 | Higher | +17 | Higher |
| Blank Lines | 10 | **6 ✅** | 9 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

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

## p03105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.3790 | 78.3896 | **85.8950 ✅** | +6.9894 | Higher | -0.5159 | Lower | -7.5054 | Lower |
| MI (SEI) | **82.8537 ✅** | 73.3643 | 81.6828 | +9.4894 | Higher | +1.1709 | Higher | -8.3185 | Lower |
| Cyclomatic (sum) | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 4 | 4 | 4 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **594.5414 ✅** | 644.2129 | 614.7155 | -49.6716 | Lower | -20.1741 | Lower | +29.4974 | Higher |
| Halstead Effort | **8561.3958 ✅** | 11763.8883 | 8664.5613 | -3202.4925 | Lower | -103.1655 | Lower | +3099.3270 | Higher |
| SLOC | 24 | 35 | **23 ✅** | -11 | Lower | +1 | Higher | +12 | Higher |
| PLOC | **14 ✅** | 22 | 15 | -8 | Lower | -1 | Lower | +7 | Higher |
| LLOC | 10 | 13 | 10 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 6 | **9 ✅** | 5 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p03106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.6204 ✅** | 65.9637 | 66.0300 | +7.6567 | Higher | +7.5904 | Higher | -0.0663 | Lower |
| MI (SEI) | **55.5345 ✅** | 51.8165 | 31.9505 | +3.7180 | Higher | +23.5839 | Higher | +19.8659 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 17 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | 12 | **11 ✅** | 30 | +1 | Higher | -18 | Lower | -19 | Lower |
| Halstead Volume | **882.4241 ✅** | 1167.8347 | 1242.6146 | -285.4106 | Lower | -360.1906 | Lower | -74.7800 | Lower |
| Halstead Effort | **28295.1198 ✅** | 35521.6376 | 54024.1503 | -7226.5178 | Lower | -25729.0305 | Lower | -18502.5127 | Lower |
| SLOC | **39 ✅** | 58 | 52 | -19 | Lower | -13 | Lower | +6 | Higher |
| PLOC | **28 ✅** | 42 | 45 | -14 | Lower | -17 | Lower | -3 | Lower |
| LLOC | **19 ✅** | 27 | 35 | -8 | Lower | -16 | Lower | -8 | Lower |
| Comments (CLOC) | 4 | **11 ✅** | 1 | -7 | Lower | +3 | Higher | +10 | Higher |
| Blank Lines | 7 | 8 | 7 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.3596 | **60.2622 ✅** | 56.5334 | -6.9026 | Lower | -3.1738 | Lower | +3.7288 | Higher |
| MI (SEI) | 28.9792 | **39.1011 ✅** | 19.7627 | -10.1219 | Lower | +9.2165 | Higher | +19.3384 | Higher |
| Cyclomatic (sum) | 15 | 13 | **12 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 3123.3550 | **1862.8380 ✅** | 3346.2583 | +1260.5171 | Higher | -222.9033 | Lower | -1483.4204 | Lower |
| Halstead Effort | 151615.6283 | **74294.3609 ✅** | 245299.3257 | +77321.2675 | Higher | -93683.6973 | Lower | -171004.9648 | Lower |
| SLOC | 87 | **69 ✅** | 73 | +18 | Higher | +14 | Higher | -4 | Lower |
| PLOC | 73 | **56 ✅** | 61 | +17 | Higher | +12 | Higher | -5 | Lower |
| LLOC | 49 | **33 ✅** | 52 | +16 | Higher | -3 | Lower | -19 | Lower |
| Comments (CLOC) | **11 ✅** | 9 | 2 | +2 | Higher | +9 | Higher | +7 | Higher |
| Blank Lines | 14 | **10 ✅** | 12 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 6 | **4 ✅** | 5 | +2 | Higher | +1 | Higher | -1 | Lower |

## p03115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.3126 | **67.5533 ✅** | 31.8797 | -3.2408 | Lower | +32.4329 | Higher | +35.6737 | Higher |
| MI (SEI) | 18.6099 | **58.2988 ✅** | -13.4896 | -39.6889 | Lower | +32.0995 | Higher | +71.7884 | Higher |
| Cyclomatic (sum) | 15 | **6 ✅** | 45 | +9 | Higher | -30 | Lower | -39 | Lower |
| Cognitive (sum) | 10 | **2 ✅** | 26 | +8 | Higher | -16 | Lower | -24 | Lower |
| Halstead Volume | 1888.8205 | **1197.1354 ✅** | 5949.6282 | +691.6851 | Higher | -4060.8077 | Lower | -4752.4928 | Lower |
| Halstead Effort | 104298.3045 | **34675.6453 ✅** | 510160.2429 | +69622.6592 | Higher | -405861.9383 | Lower | -475484.5975 | Lower |
| SLOC | **52 ✅** | 56 | 174 | -4 | Lower | -122 | Lower | -118 | Lower |
| PLOC | 44 | **30 ✅** | 151 | +14 | Higher | -107 | Lower | -121 | Lower |
| LLOC | 30 | **20 ✅** | 83 | +10 | Higher | -53 | Lower | -63 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 4 | -15 | Lower | -4 | Lower | +11 | Higher |
| Blank Lines | **8 ✅** | 14 | 23 | -6 | Lower | -15 | Lower | -9 | Lower |
| Functions | 3 | **2 ✅** | 19 | +1 | Higher | -16 | Lower | -17 | Lower |
| Closures | 1 | 1 | 3 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 3 | **1 ✅** | 17 | +2 | Higher | -14 | Lower | -16 | Lower |

## p03123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.9773 | **77.4070 ✅** | 41.6257 | -36.4297 | Lower | -0.6483 | Lower | +35.7813 | Higher |
| MI (SEI) | -2.2270 | **78.4905 ✅** | 3.9324 | -80.7176 | Lower | -6.1595 | Lower | +74.5581 | Higher |
| Cyclomatic (sum) | 29 | **8 ✅** | 25 | +21 | Higher | +4 | Higher | -17 | Lower |
| Cognitive (sum) | 23 | **7 ✅** | 24 | +16 | Higher | -1 | Lower | -17 | Lower |
| Halstead Volume | 4522.6663 | **599.0871 ✅** | 4452.0740 | +3923.5792 | Higher | +70.5924 | Higher | -3852.9868 | Lower |
| Halstead Effort | 264999.9790 | **20209.2061 ✅** | 273802.5483 | +244790.7729 | Higher | -8802.5693 | Lower | -253593.3422 | Lower |
| SLOC | 136 | **37 ✅** | 139 | +99 | Higher | -3 | Lower | -102 | Lower |
| PLOC | 116 | **22 ✅** | 120 | +94 | Higher | -4 | Lower | -98 | Lower |
| LLOC | 81 | **15 ✅** | 91 | +66 | Higher | -10 | Lower | -76 | Lower |
| Comments (CLOC) | 3 | **15 ✅** | 7 | -12 | Lower | -4 | Lower | +8 | Higher |
| Blank Lines | 20 | **5 ✅** | 19 | +15 | Higher | +1 | Higher | -14 | Lower |
| Functions | 9 | **2 ✅** | 9 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 5 | 0 | 0 | +5 | Higher | +5 | Higher | 0 | Same |
| Exits | 8 | **2 ✅** | 8 | +6 | Higher | 0 | Same | -6 | Lower |

## p03142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.7898 | **73.0385 ✅** | 55.9894 | -19.2486 | Lower | -2.1995 | Lower | +17.0491 | Higher |
| MI (SEI) | 25.2551 | **63.3683 ✅** | 6.5002 | -38.1132 | Lower | +18.7549 | Higher | +56.8681 | Higher |
| Cyclomatic (sum) | 17 | **5 ✅** | 14 | +12 | Higher | +3 | Higher | -9 | Lower |
| Cognitive (sum) | 22 | **5 ✅** | 20 | +17 | Higher | +2 | Higher | -15 | Lower |
| Halstead Volume | 2935.9872 | **1151.0155 ✅** | 2556.7787 | +1784.9717 | Higher | +379.2085 | Higher | -1405.7632 | Lower |
| Halstead Effort | 175388.5335 | **23691.7361 ✅** | 123812.0092 | +151696.7974 | Higher | +51576.5243 | Higher | -100120.2731 | Lower |
| SLOC | 84 | **41 ✅** | 80 | +43 | Higher | +4 | Higher | -39 | Lower |
| PLOC | 68 | **27 ✅** | 69 | +41 | Higher | -1 | Lower | -42 | Lower |
| LLOC | 59 | **22 ✅** | 52 | +37 | Higher | +7 | Higher | -30 | Lower |
| Comments (CLOC) | 7 | **9 ✅** | 0 | -2 | Lower | +7 | Higher | +9 | Higher |
| Blank Lines | 9 | **6 ✅** | 11 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 4 | 0 | 0 | +4 | Higher | +4 | Higher | 0 | Same |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

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

## p03164

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3986 | 61.9682 | **75.4185 ✅** | +3.4305 | Higher | -10.0198 | Lower | -13.4503 | Lower |
| MI (SEI) | 44.1499 | 42.4648 | **47.5439 ✅** | +1.6851 | Higher | -3.3940 | Lower | -5.0791 | Lower |
| Cyclomatic (sum) | 10 | 13 | **9 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 12 | 9 | 9 | +3 | Higher | +3 | Higher | 0 | Same |
| Halstead Volume | 1605.6136 | 1696.1909 | **1320.6623 ✅** | -90.5773 | Lower | +284.9513 | Higher | +375.5285 | Higher |
| Halstead Effort | 48533.3206 | 61741.3480 | **46324.7716 ✅** | -13208.0274 | Lower | +2208.5490 | Higher | +15416.5764 | Higher |
| SLOC | 55 | 64 | **32 ✅** | -9 | Lower | +23 | Higher | +32 | Higher |
| PLOC | 39 | 47 | **27 ✅** | -8 | Lower | +12 | Higher | +20 | Higher |
| LLOC | 32 | 26 | **20 ✅** | +6 | Higher | +12 | Higher | +6 | Higher |
| Comments (CLOC) | 6 | **9 ✅** | 1 | -3 | Lower | +5 | Higher | +8 | Higher |
| Blank Lines | 10 | 8 | **5 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | **0 ✅** | 2 | 2 | -2 | Lower | -2 | Lower | 0 | Same |
| Exits | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03165

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.6162 | **61.7648 ✅** | 60.5141 | -2.1486 | Lower | -0.8978 | Lower | +1.2507 | Higher |
| MI (SEI) | 40.0049 | **40.9523 ✅** | 38.6055 | -0.9474 | Lower | +1.3994 | Higher | +2.3469 | Higher |
| Cyclomatic (sum) | 15 | 13 | 13 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | **15 ✅** | 16 | 17 | -1 | Lower | -2 | Lower | -1 | Lower |
| Halstead Volume | 2115.9571 | **1852.5462 ✅** | 2038.3816 | +263.4109 | Higher | +77.5755 | Higher | -185.8354 | Lower |
| Halstead Effort | **109500.7796 ✅** | 111152.7732 | 123661.8157 | -1651.9936 | Lower | -14161.0362 | Lower | -12509.0425 | Lower |
| SLOC | 67 | **63 ✅** | 66 | +4 | Higher | +1 | Higher | -3 | Lower |
| PLOC | 48 | **44 ✅** | 48 | +4 | Higher | 0 | Same | -4 | Lower |
| LLOC | 41 | **34 ✅** | 43 | +7 | Higher | -2 | Lower | -9 | Lower |
| Comments (CLOC) | **10 ✅** | 8 | 8 | +2 | Higher | +2 | Higher | 0 | Same |
| Blank Lines | **10 ✅** | 11 | 11 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p03173

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.9690 ✅** | 69.5161 | 69.3553 | +0.4529 | Higher | +0.6137 | Higher | +0.1608 | Higher |
| MI (SEI) | 26.1595 | **57.1361 ✅** | 41.6704 | -30.9766 | Lower | -15.5109 | Lower | +15.4657 | Higher |
| Cyclomatic (sum) | 9 | **6 ✅** | 9 | +3 | Higher | 0 | Same | -3 | Lower |
| Cognitive (sum) | 8 | **2 ✅** | 8 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 1740.1893 | **1168.2782 ✅** | 1688.1570 | +571.9110 | Higher | +52.0322 | Higher | -519.8788 | Lower |
| Halstead Effort | 96487.2803 | **29206.9558 ✅** | 85513.8857 | +67280.3245 | Higher | +10973.3945 | Higher | -56306.9300 | Lower |
| SLOC | **41 ✅** | 50 | 43 | -9 | Lower | -2 | Lower | +7 | Higher |
| PLOC | 34 | **29 ✅** | 35 | +5 | Higher | -1 | Lower | -6 | Lower |
| LLOC | 26 | **21 ✅** | 27 | +5 | Higher | -1 | Lower | -6 | Lower |
| Comments (CLOC) | 0 | **10 ✅** | 2 | -10 | Lower | -2 | Lower | +8 | Higher |
| Blank Lines | **7 ✅** | 11 | 8 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

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

## p03209

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.1539 | 78.1666 | **81.7936 ✅** | -0.0127 | Lower | -3.6397 | Lower | -3.6270 | Lower |
| MI (SEI) | 57.2351 | **78.0905 ✅** | 63.7418 | -20.8554 | Lower | -6.5067 | Lower | +14.3486 | Higher |
| Cyclomatic (sum) | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1042.4815 | **739.3422 ✅** | 845.1050 | +303.1393 | Higher | +197.3765 | Higher | -105.7628 | Lower |
| Halstead Effort | 27573.6363 | **16547.1820 ✅** | 20231.8132 | +11026.4542 | Higher | +7341.8231 | Higher | -3684.6312 | Lower |
| SLOC | 30 | 33 | **26 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| PLOC | 24 | 22 | 22 | +2 | Higher | +2 | Higher | 0 | Same |
| LLOC | 14 | **9 ✅** | 11 | +5 | Higher | +3 | Higher | -2 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 5 | 4 | 4 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 4 | 2 | -2 | Lower | 0 | Same | +2 | Higher |

## p03213

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.5550 | **72.9931 ✅** | 72.7383 | -2.4381 | Lower | -2.1832 | Lower | +0.2549 | Higher |
| MI (SEI) | 38.7078 | **70.7285 ✅** | 30.0528 | -32.0207 | Lower | +8.6550 | Higher | +40.6757 | Higher |
| Cyclomatic (sum) | 9 | 9 | **8 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 1340.3466 | **780.6987 ✅** | 1353.1137 | +559.6479 | Higher | -12.7671 | Lower | -572.4150 | Lower |
| Halstead Effort | 57072.8246 | **30447.2501 ✅** | 62919.7869 | +26625.5745 | Higher | -5846.9623 | Lower | -32472.5368 | Lower |
| SLOC | 43 | 44 | **38 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| PLOC | 35 | **31 ✅** | 32 | +4 | Higher | +3 | Higher | -1 | Lower |
| LLOC | 24 | 22 | 22 | +2 | Higher | +2 | Higher | 0 | Same |
| Comments (CLOC) | 1 | **16 ✅** | 0 | -15 | Lower | +1 | Higher | +16 | Higher |
| Blank Lines | 7 | 9 | **6 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03216

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.9718 | 57.5485 | **60.5469 ✅** | -0.5766 | Lower | -3.5751 | Lower | -2.9985 | Lower |
| MI (SEI) | 35.6458 | **41.7590 ✅** | 13.0753 | -6.1132 | Lower | +22.5704 | Higher | +28.6836 | Higher |
| Cyclomatic (sum) | 18 | 21 | **14 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| Cognitive (sum) | **15 ✅** | 16 | 30 | -1 | Lower | -15 | Lower | -14 | Lower |
| Halstead Volume | **1844.2537 ✅** | 1848.8669 | 1849.2107 | -4.6132 | Lower | -4.9569 | Lower | -0.3437 | Lower |
| Halstead Effort | **68852.1399 ✅** | 82549.4094 | 82768.1186 | -13697.2695 | Lower | -13915.9786 | Lower | -218.7091 | Lower |
| SLOC | 79 | 73 | **67 ✅** | +6 | Higher | +12 | Higher | +6 | Higher |
| PLOC | 59 | **53 ✅** | 58 | +6 | Higher | +1 | Higher | -5 | Lower |
| LLOC | 45 | **35 ✅** | 55 | +10 | Higher | -10 | Lower | -20 | Lower |
| Comments (CLOC) | 11 | **15 ✅** | 0 | -4 | Lower | +11 | Higher | +15 | Higher |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03218

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.8563 | **73.0412 ✅** | 66.9721 | -8.1849 | Lower | -2.1158 | Lower | +6.0690 | Higher |
| MI (SEI) | 41.0556 | **64.5844 ✅** | 43.0504 | -23.5288 | Lower | -1.9948 | Lower | +21.5340 | Higher |
| Cyclomatic (sum) | 12 | **6 ✅** | 9 | +6 | Higher | +3 | Higher | -3 | Lower |
| Cognitive (sum) | 7 | 4 | 4 | +3 | Higher | +3 | Higher | 0 | Same |
| Halstead Volume | 1382.4835 | **1021.0366 ✅** | 1668.7433 | +361.4470 | Higher | -286.2598 | Lower | -647.7068 | Lower |
| Halstead Effort | 42938.3115 | **23237.3838 ✅** | 27466.6131 | +19700.9277 | Higher | +15471.6984 | Higher | -4229.2293 | Lower |
| SLOC | 58 | **42 ✅** | 50 | +16 | Higher | +8 | Higher | -8 | Lower |
| PLOC | 44 | **28 ✅** | 35 | +16 | Higher | +9 | Higher | -7 | Lower |
| LLOC | 23 | 17 | **15 ✅** | +6 | Higher | +8 | Higher | +2 | Higher |
| Comments (CLOC) | 5 | **10 ✅** | 4 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 9 | **8 ✅** | 11 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p03268

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.2664 | 76.1921 | **83.8099 ✅** | +2.0742 | Higher | -5.5436 | Lower | -7.6178 | Lower |
| MI (SEI) | 65.9777 | **72.1439 ✅** | 71.0800 | -6.1662 | Lower | -5.1024 | Lower | +1.0638 | Higher |
| Cyclomatic (sum) | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 865.6692 | **708.4490 ✅** | 739.9578 | +157.2201 | Higher | +125.7114 | Higher | -31.5087 | Lower |
| Halstead Effort | 15880.5520 | **12468.7033 ✅** | 13477.8024 | +3411.8487 | Higher | +2402.7496 | Higher | -1009.0991 | Lower |
| SLOC | 33 | 40 | **25 ✅** | -7 | Lower | +8 | Higher | +15 | Higher |
| PLOC | 23 | 20 | **18 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| LLOC | 15 | **12 ✅** | 13 | +3 | Higher | +2 | Higher | -1 | Lower |
| Comments (CLOC) | 5 | **12 ✅** | 3 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | 5 | 8 | **4 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p03277

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 52.6242 | **76.6371 ✅** | 55.1967 | -24.0130 | Lower | -2.5725 | Lower | +21.4404 | Higher |
| MI (SEI) | 18.2452 | **76.3745 ✅** | 26.0230 | -58.1293 | Lower | -7.7778 | Lower | +50.3515 | Higher |
| Cyclomatic (sum) | 16 | **6 ✅** | 16 | +10 | Higher | 0 | Same | -10 | Lower |
| Cognitive (sum) | 12 | **4 ✅** | 13 | +8 | Higher | -1 | Lower | -9 | Lower |
| Halstead Volume | 3568.4525 | **987.6748 ✅** | 2622.2181 | +2580.7776 | Higher | +946.2343 | Higher | -1634.5433 | Lower |
| Halstead Effort | 126604.1377 | **27806.8453 ✅** | 136729.9451 | +98797.2924 | Higher | -10125.8075 | Lower | -108923.0998 | Lower |
| SLOC | 86 | **34 ✅** | 81 | +52 | Higher | +5 | Higher | -47 | Lower |
| PLOC | 70 | **24 ✅** | 71 | +46 | Higher | -1 | Lower | -47 | Lower |
| LLOC | 51 | **19 ✅** | 55 | +32 | Higher | -4 | Lower | -36 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 6 | -9 | Lower | -2 | Lower | +7 | Higher |
| Blank Lines | 16 | **6 ✅** | 10 | +10 | Higher | +6 | Higher | -4 | Lower |
| Functions | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p03278

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 45.8013 | **62.1089 ✅** | 43.1558 | -16.3076 | Lower | +2.6455 | Higher | +18.9531 | Higher |
| MI (SEI) | -0.1226 | **49.6270 ✅** | -4.4977 | -49.7496 | Lower | +4.3751 | Higher | +54.1248 | Higher |
| Cyclomatic (sum) | 19 | **14 ✅** | 20 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 19 | **12 ✅** | 19 | +7 | Higher | 0 | Same | -7 | Lower |
| Halstead Volume | 6420.3360 | **1658.9053 ✅** | 5760.3386 | +4761.4307 | Higher | +659.9974 | Higher | -4101.4332 | Lower |
| Halstead Effort | 980224.7977 | **66485.8156 ✅** | 762137.1021 | +913738.9821 | Higher | +218087.6955 | Higher | -695651.2866 | Lower |
| SLOC | 104 | **63 ✅** | 125 | +41 | Higher | -21 | Lower | -62 | Lower |
| PLOC | 92 | **49 ✅** | 110 | +43 | Higher | -18 | Lower | -61 | Lower |
| LLOC | 86 | **20 ✅** | 82 | +66 | Higher | +4 | Higher | -62 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 1 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 12 | **11 ✅** | 15 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 5 | **4 ✅** | 6 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |

## p03294

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.7069 | 72.0088 | **88.0775 ✅** | +4.6981 | Higher | -11.3706 | Lower | -16.0687 | Lower |
| MI (SEI) | 64.7830 | 63.0629 | **85.3344 ✅** | +1.7201 | Higher | -20.5515 | Lower | -22.2715 | Lower |
| Cyclomatic (sum) | 6 | 8 | **3 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Cognitive (sum) | 4 | 8 | **1 ✅** | -4 | Lower | +3 | Higher | +7 | Higher |
| Halstead Volume | 748.8245 | 802.9035 | **485 ✅** | -54.0790 | Lower | +263.8245 | Higher | +317.9035 | Higher |
| Halstead Effort | 13931.2565 | 14753.3520 | **5469.7222 ✅** | -822.0955 | Lower | +8461.5343 | Higher | +9283.6297 | Higher |
| SLOC | 37 | 47 | **22 ✅** | -10 | Lower | +15 | Higher | +25 | Higher |
| PLOC | 23 | 30 | **13 ✅** | -7 | Lower | +10 | Higher | +17 | Higher |
| LLOC | 17 | 17 | **10 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Comments (CLOC) | 6 | **11 ✅** | 5 | -5 | Lower | +1 | Higher | +6 | Higher |
| Blank Lines | 8 | 6 | **5 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03296

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.9368 | **78.2758 ✅** | 77.3876 | -0.3390 | Lower | +0.5492 | Higher | +0.8882 | Higher |
| MI (SEI) | 56.2334 | **68.6186 ✅** | 67.2354 | -12.3852 | Lower | -11.0020 | Lower | +1.3832 | Higher |
| Cyclomatic (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 4 | 6 | 4 | -2 | Lower | 0 | Same | +2 | Higher |
| Halstead Volume | 929.1717 | **756.7280 ✅** | 938.2737 | +172.4437 | Higher | -9.1020 | Lower | -181.5457 | Lower |
| Halstead Effort | 22476.3424 | **15891.2877 ✅** | 25802.5268 | +6585.0546 | Higher | -3326.1844 | Lower | -9911.2390 | Lower |
| SLOC | **32 ✅** | 33 | 33 | -1 | Lower | -1 | Lower | 0 | Same |
| PLOC | 25 | **23 ✅** | 28 | +2 | Higher | -3 | Lower | -5 | Lower |
| LLOC | 22 | **17 ✅** | 22 | +5 | Higher | 0 | Same | -5 | Lower |
| Comments (CLOC) | 2 | 6 | 6 | -4 | Lower | -4 | Lower | 0 | Same |
| Blank Lines | 7 | 5 | 5 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03311

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.5040 | **74.2042 ✅** | 60.1263 | -15.7002 | Lower | -1.6223 | Lower | +14.0779 | Higher |
| MI (SEI) | 28.3486 | **69.8867 ✅** | 28.1369 | -41.5381 | Lower | +0.2117 | Higher | +41.7498 | Higher |
| Cyclomatic (sum) | 14 | **10 ✅** | 12 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 17 | **11 ✅** | 17 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 2499.2674 | **796.3187 ✅** | 1998.6585 | +1702.9487 | Higher | +500.6090 | Higher | -1202.3398 | Lower |
| Halstead Effort | 85421.3909 | **18280.7075 ✅** | 113205.2656 | +67140.6834 | Higher | -27783.8747 | Lower | -94924.5581 | Lower |
| SLOC | 69 | **40 ✅** | 69 | +29 | Higher | 0 | Same | -29 | Lower |
| PLOC | 55 | **25 ✅** | 54 | +30 | Higher | +1 | Higher | -29 | Lower |
| LLOC | 45 | **12 ✅** | 50 | +33 | Higher | -5 | Lower | -38 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 3 | -8 | Lower | +1 | Higher | +9 | Higher |
| Blank Lines | 10 | **6 ✅** | 12 | +4 | Higher | -2 | Lower | -6 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p03317

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.8645 | **83.3279 ✅** | 78.4639 | -11.4635 | Lower | -6.5994 | Lower | +4.8640 | Higher |
| MI (SEI) | 54.8212 | **83.8140 ✅** | 65.5199 | -28.9928 | Lower | -10.6987 | Lower | +18.2941 | Higher |
| Cyclomatic (sum) | 5 | **3 ✅** | 8 | +2 | Higher | -3 | Lower | -5 | Lower |
| Cognitive (sum) | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 1685.7504 | **638.7535 ✅** | 1044.3360 | +1046.9969 | Higher | +641.4144 | Higher | -405.5825 | Lower |
| Halstead Effort | 28556.6118 | **9652.2751 ✅** | 25102.7439 | +18904.3367 | Higher | +3453.8679 | Higher | -15450.4688 | Lower |
| SLOC | 39 | **27 ✅** | 29 | +12 | Higher | +10 | Higher | -2 | Lower |
| PLOC | 26 | **14 ✅** | 20 | +12 | Higher | +6 | Higher | -6 | Lower |
| LLOC | 17 | **9 ✅** | 16 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 4 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 8 | 5 | 5 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 4 | 0 | Same | -4 | Lower | -4 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p03361

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.8798 ✅** | 62.1923 | 69.9547 | +8.6875 | Higher | +0.9250 | Higher | -7.7624 | Lower |
| MI (SEI) | **49.0660 ✅** | 40.5205 | 47.2305 | +8.5455 | Higher | +1.8355 | Higher | -6.7100 | Lower |
| Cyclomatic (sum) | 14 | 16 | 14 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 32 | 31 | **29 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Halstead Volume | **1483.5314 ✅** | 1570.6657 | 1510.6165 | -87.1343 | Lower | -27.0851 | Lower | +60.0492 | Higher |
| Halstead Effort | 57090.3807 | **48466.2557 ✅** | 52116.2701 | +8624.1250 | Higher | +4974.1106 | Higher | -3650.0144 | Lower |
| SLOC | **38 ✅** | 62 | 40 | -24 | Lower | -2 | Lower | +22 | Higher |
| PLOC | **29 ✅** | 48 | 31 | -19 | Lower | -2 | Lower | +17 | Higher |
| LLOC | **21 ✅** | 34 | 23 | -13 | Lower | -2 | Lower | +11 | Higher |
| Comments (CLOC) | 3 | **7 ✅** | 3 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p03365

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.1986 | **63.2004 ✅** | 62.5651 | -0.0017 | Lower | +0.6335 | Higher | +0.6352 | Higher |
| MI (SEI) | 16.7992 | **51.5445 ✅** | 15.8852 | -34.7453 | Lower | +0.9140 | Higher | +35.6593 | Higher |
| Cyclomatic (sum) | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 2272.8966 | **1554.2445 ✅** | 2292.3656 | +718.6521 | Higher | -19.4690 | Lower | -738.1210 | Lower |
| Halstead Effort | 160163.4489 | **95294.6186 ✅** | 150080.4798 | +64868.8303 | Higher | +10082.9691 | Higher | -54785.8612 | Lower |
| SLOC | **54 ✅** | 61 | 56 | -7 | Lower | -2 | Lower | +5 | Higher |
| PLOC | **45 ✅** | 49 | 46 | -4 | Lower | -1 | Lower | +3 | Higher |
| LLOC | 36 | **30 ✅** | 38 | +6 | Higher | -2 | Lower | -8 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 0 | -15 | Lower | 0 | Same | +15 | Higher |
| Blank Lines | 9 | **8 ✅** | 10 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 3 | 6 | 3 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 7 | 3 | -4 | Lower | 0 | Same | +4 | Higher |

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

## p03406

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.0893 | **56.5430 ✅** | 45.4199 | -7.4537 | Lower | +3.6694 | Higher | +11.1231 | Higher |
| MI (SEI) | 5.0118 | **35.1350 ✅** | -7.9337 | -30.1232 | Lower | +12.9455 | Higher | +43.0688 | Higher |
| Cyclomatic (sum) | 21 | **16 ✅** | 22 | +5 | Higher | -1 | Lower | -6 | Lower |
| Cognitive (sum) | 32 | **12 ✅** | 32 | +20 | Higher | 0 | Same | -20 | Lower |
| Halstead Volume | 3640.7941 | **2370 ✅** | 5226.9621 | +1270.7941 | Higher | -1586.1680 | Lower | -2856.9621 | Lower |
| Halstead Effort | 332653.6094 | **169203.6364 ✅** | 235622.3621 | +163449.9730 | Higher | +97031.2473 | Higher | -66418.7257 | Lower |
| SLOC | 99 | **77 ✅** | 109 | +22 | Higher | -10 | Lower | -32 | Lower |
| PLOC | 87 | **64 ✅** | 96 | +23 | Higher | -9 | Lower | -32 | Lower |
| LLOC | 64 | **50 ✅** | 67 | +14 | Higher | -3 | Lower | -17 | Lower |
| Comments (CLOC) | 1 | **11 ✅** | 0 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | **12 ✅** | 13 | 13 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 7 | 2 | -5 | Lower | 0 | Same | +5 | Higher |

## p03408

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.9405 | **66.5875 ✅** | 65.5016 | -3.6471 | Lower | -2.5611 | Lower | +1.0859 | Higher |
| MI (SEI) | 44.6680 | **57.1568 ✅** | 45.2487 | -12.4888 | Lower | -0.5807 | Lower | +11.9081 | Higher |
| Cyclomatic (sum) | 10 | **6 ✅** | 10 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 15 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 1530.8001 | 1524.6948 | **1264.8957 ✅** | +6.1053 | Higher | +265.9044 | Higher | +259.7991 | Higher |
| Halstead Effort | 47733.1296 | 45451.6766 | **41437.9827 ✅** | +2281.4530 | Higher | +6295.1469 | Higher | +4013.6939 | Higher |
| SLOC | 65 | **55 ✅** | 59 | +10 | Higher | +6 | Higher | -4 | Lower |
| PLOC | 45 | **40 ✅** | 41 | +5 | Higher | +4 | Higher | -1 | Lower |
| LLOC | 37 | **30 ✅** | 33 | +7 | Higher | +4 | Higher | -3 | Lower |
| Comments (CLOC) | 10 | **15 ✅** | 7 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 11 | **8 ✅** | 11 | +3 | Higher | 0 | Same | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03414

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 23.4677 | **53.6615 ✅** | 22.2444 | -30.1937 | Lower | +1.2234 | Higher | +31.4171 | Higher |
| MI (SEI) | -17.8312 | **20.9361 ✅** | -21.0318 | -38.7672 | Lower | +3.2006 | Higher | +41.9679 | Higher |
| Cyclomatic (sum) | 50 | **24 ✅** | 54 | +26 | Higher | -4 | Lower | -30 | Lower |
| Cognitive (sum) | 65 | **24 ✅** | 61 | +41 | Higher | +4 | Higher | -37 | Lower |
| Halstead Volume | 8944.1803 | **2380.1980 ✅** | 9733.0273 | +6563.9824 | Higher | -788.8470 | Lower | -7352.8294 | Lower |
| Halstead Effort | 1193633.9906 | **83333.3756 ✅** | 1329145.5707 | +1110300.6150 | Higher | -135511.5801 | Lower | -1245812.1951 | Lower |
| SLOC | 239 | **82 ✅** | 237 | +157 | Higher | +2 | Higher | -155 | Lower |
| PLOC | 220 | **70 ✅** | 217 | +150 | Higher | +3 | Higher | -147 | Lower |
| LLOC | 173 | **32 ✅** | 163 | +141 | Higher | +10 | Higher | -131 | Lower |
| Comments (CLOC) | **15 ✅** | 4 | 12 | +11 | Higher | +3 | Higher | -8 | Lower |
| Blank Lines | 19 | **10 ✅** | 20 | +9 | Higher | -1 | Lower | -10 | Lower |
| Functions | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | **0 ✅** | 4 | +1 | Higher | -3 | Lower | -4 | Lower |
| Exits | 8 | 8 | 9 | 0 | Same | -1 | Lower | -1 | Lower |

## p03434

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.8773 | **69.7397 ✅** | 56.9089 | -11.8624 | Lower | +0.9684 | Higher | +12.8308 | Higher |
| MI (SEI) | 9.1221 | **62.2758 ✅** | 7.7250 | -53.1537 | Lower | +1.3971 | Higher | +54.5508 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 13 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 13 | **10 ✅** | 14 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | 2369.7701 | **1024.3700 ✅** | 2522.4121 | +1345.4002 | Higher | -152.6420 | Lower | -1498.0422 | Lower |
| Halstead Effort | 130929.7998 | **24175.1314 ✅** | 137736.9780 | +106754.6683 | Higher | -6807.1783 | Lower | -113561.8466 | Lower |
| SLOC | 74 | **50 ✅** | 77 | +24 | Higher | -3 | Lower | -27 | Lower |
| PLOC | 60 | **32 ✅** | 63 | +28 | Higher | -3 | Lower | -31 | Lower |
| LLOC | 52 | **22 ✅** | 56 | +30 | Higher | -4 | Lower | -34 | Lower |
| Comments (CLOC) | 0 | **14 ✅** | 0 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 14 | **8 ✅** | 14 | +6 | Higher | 0 | Same | -6 | Lower |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03435

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.4858 | 67.8290 | **68.4780 ✅** | -0.3432 | Lower | -0.9922 | Lower | -0.6490 | Lower |
| MI (SEI) | **53.3769 ✅** | 51.7253 | 46.1497 | +1.6515 | Higher | +7.2271 | Higher | +5.5756 | Higher |
| Cyclomatic (sum) | 14 | 14 | **12 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Cognitive (sum) | 15 | **10 ✅** | 19 | +5 | Higher | -4 | Lower | -9 | Lower |
| Halstead Volume | **1139.3324 ✅** | 1375.6039 | 1326.5101 | -236.2716 | Lower | -187.1777 | Lower | +49.0938 | Higher |
| Halstead Effort | **45003.6278 ✅** | 55976.4976 | 72958.0547 | -10972.8698 | Lower | -27954.4269 | Lower | -16981.5571 | Lower |
| SLOC | 51 | 47 | 47 | +4 | Higher | +4 | Higher | 0 | Same |
| PLOC | 35 | **33 ✅** | 38 | +2 | Higher | -3 | Lower | -5 | Lower |
| LLOC | 25 | **22 ✅** | 28 | +3 | Higher | -3 | Lower | -6 | Lower |
| Comments (CLOC) | **9 ✅** | 7 | 4 | +2 | Higher | +5 | Higher | +3 | Higher |
| Blank Lines | 8 | 7 | **6 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |
| Exits | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |

## p03448

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.2874 | **82.7252 ✅** | 75.2874 | -7.4378 | Lower | 0 | Same | +7.4378 | Higher |
| MI (SEI) | 57.6132 | **86.8117 ✅** | 57.6132 | -29.1985 | Lower | 0 | Same | +29.1985 | Higher |
| Cyclomatic (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 10 | 10 | 10 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 905.4263 | **536.5737 ✅** | 905.4263 | +368.8526 | Higher | 0 | Same | -368.8526 | Lower |
| Halstead Effort | 20330.9358 | **10929.1590 ✅** | 20330.9358 | +9401.7768 | Higher | 0 | Same | -9401.7768 | Lower |
| SLOC | 38 | **28 ✅** | 38 | +10 | Higher | 0 | Same | -10 | Lower |
| PLOC | 26 | **21 ✅** | 26 | +5 | Higher | 0 | Same | -5 | Lower |
| LLOC | 20 | **13 ✅** | 20 | +7 | Higher | 0 | Same | -7 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 4 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 8 | **4 ✅** | 8 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p03478

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.8515 | **81.7022 ✅** | 69.1612 | -32.8507 | Lower | -20.3097 | Lower | +12.5410 | Higher |
| MI (SEI) | 21.0919 | **86.9976 ✅** | 51.7576 | -65.9057 | Lower | -30.6657 | Lower | +35.2401 | Higher |
| Cyclomatic (sum) | 18 | 8 | **6 ✅** | +10 | Higher | +12 | Higher | +2 | Higher |
| Cognitive (sum) | 12 | 4 | 4 | +8 | Higher | +8 | Higher | 0 | Same |
| Halstead Volume | 3134.2450 | **560.2526 ✅** | 1862.7222 | +2573.9924 | Higher | +1271.5228 | Higher | -1302.4696 | Lower |
| Halstead Effort | 102748.7289 | **11205.0527 ✅** | 34483.0776 | +91543.6762 | Higher | +68265.6513 | Higher | -23278.0249 | Lower |
| SLOC | 110 | **29 ✅** | 44 | +81 | Higher | +66 | Higher | -15 | Lower |
| PLOC | 74 | **20 ✅** | 29 | +54 | Higher | +45 | Higher | -9 | Lower |
| LLOC | 46 | **11 ✅** | 20 | +35 | Higher | +26 | Higher | -9 | Lower |
| Comments (CLOC) | 12 | **14 ✅** | 6 | -2 | Lower | +6 | Higher | +8 | Higher |
| Blank Lines | 24 | **4 ✅** | 11 | +20 | Higher | +13 | Higher | -7 | Lower |
| Functions | 4 | 2 | **1 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 3 | 1 | **0 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |

## p03495

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.1900 | **76.0351 ✅** | 56.1100 | -17.8452 | Lower | +2.0800 | Higher | +19.9252 | Higher |
| MI (SEI) | 9.5732 | **69.4752 ✅** | 21.5406 | -59.9020 | Lower | -11.9674 | Lower | +47.9346 | Higher |
| Cyclomatic (sum) | 13 | **5 ✅** | 14 | +8 | Higher | -1 | Lower | -9 | Lower |
| Cognitive (sum) | 11 | **1 ✅** | 14 | +10 | Higher | -3 | Lower | -13 | Lower |
| Halstead Volume | 2328.0908 | **969.9847 ✅** | 2598 | +1358.1060 | Higher | -269.9092 | Lower | -1628.0153 | Lower |
| Halstead Effort | 115618.0207 | **19690.6901 ✅** | 136561.5385 | +95927.3306 | Higher | -20943.5177 | Lower | -116870.8484 | Lower |
| SLOC | 73 | **36 ✅** | 79 | +37 | Higher | -6 | Lower | -43 | Lower |
| PLOC | 57 | **24 ✅** | 63 | +33 | Higher | -6 | Lower | -39 | Lower |
| LLOC | 50 | **15 ✅** | 55 | +35 | Higher | -5 | Lower | -40 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 3 | -9 | Lower | -3 | Lower | +6 | Higher |
| Blank Lines | 16 | **6 ✅** | 16 | +10 | Higher | 0 | Same | -10 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p03562

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.8616 | **69.4277 ✅** | 41.8039 | -26.5661 | Lower | +1.0577 | Higher | +27.6238 | Higher |
| MI (SEI) | -10.7082 | **60.0649 ✅** | -5.3005 | -70.7731 | Lower | -5.4077 | Lower | +65.3654 | Higher |
| Cyclomatic (sum) | 31 | **9 ✅** | 31 | +22 | Higher | 0 | Same | -22 | Lower |
| Cognitive (sum) | 51 | **4 ✅** | 50 | +47 | Higher | +1 | Higher | -46 | Lower |
| Halstead Volume | 4729.5281 | **1108.2491 ✅** | 4708.9242 | +3621.2790 | Higher | +20.6039 | Higher | -3600.6751 | Lower |
| Halstead Effort | 484122.5468 | **25864.5783 ✅** | 483074.2041 | +458257.9685 | Higher | +1048.3427 | Higher | -457209.6258 | Lower |
| SLOC | 116 | **49 ✅** | 124 | +67 | Higher | -8 | Lower | -75 | Lower |
| PLOC | 102 | **29 ✅** | 109 | +73 | Higher | -7 | Lower | -80 | Lower |
| LLOC | 81 | **22 ✅** | 87 | +59 | Higher | -6 | Lower | -65 | Lower |
| Comments (CLOC) | 0 | **12 ✅** | 1 | -12 | Lower | -1 | Lower | +11 | Higher |
| Blank Lines | 14 | **10 ✅** | 15 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p03608

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 49.1196 | 50.3074 | **51.1974 ✅** | -1.1878 | Lower | -2.0778 | Lower | -0.8900 | Lower |
| MI (SEI) | 12.2288 | **28.3382 ✅** | 0.0960 | -16.1093 | Lower | +12.1329 | Higher | +28.2422 | Higher |
| Cyclomatic (sum) | 20 | 20 | **19 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 27 | **23 ✅** | 30 | +4 | Higher | -3 | Lower | -7 | Lower |
| Halstead Volume | 3344.0994 | **3208.3610 ✅** | 3966.1560 | +135.7384 | Higher | -622.0567 | Lower | -757.7950 | Lower |
| Halstead Effort | 138131.7773 | 180951.5604 | **132142.9863 ✅** | -42819.7832 | Lower | +5988.7910 | Higher | +48808.5742 | Higher |
| SLOC | 103 | 97 | **87 ✅** | +6 | Higher | +16 | Higher | +10 | Higher |
| PLOC | 88 | 74 | **72 ✅** | +14 | Higher | +16 | Higher | +2 | Higher |
| LLOC | 53 | **45 ✅** | 52 | +8 | Higher | +1 | Higher | -7 | Lower |
| Comments (CLOC) | 4 | **16 ✅** | 0 | -12 | Lower | +4 | Higher | +16 | Higher |
| Blank Lines | **14 ✅** | 15 | 15 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 4 | 5 | **2 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 4 | **1 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p03626

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.1740 | **76.5160 ✅** | 66.0917 | -10.3419 | Lower | +0.0824 | Higher | +10.4243 | Higher |
| MI (SEI) | 41.1472 | **69.7903 ✅** | 44.8475 | -28.6431 | Lower | -3.7003 | Lower | +24.9428 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 9 | +4 | Higher | 0 | Same | -4 | Lower |
| Cognitive (sum) | 15 | **3 ✅** | 15 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 1530.8001 | **811.9636 ✅** | 1388.6472 | +718.8365 | Higher | +142.1529 | Higher | -576.6836 | Lower |
| Halstead Effort | 57558.0828 | **14284.5449 ✅** | 52546.4105 | +43273.5379 | Higher | +5011.6723 | Higher | -38261.8656 | Lower |
| SLOC | 54 | **37 ✅** | 56 | +17 | Higher | -2 | Lower | -19 | Lower |
| PLOC | 42 | **22 ✅** | 41 | +20 | Higher | +1 | Higher | -19 | Lower |
| LLOC | 40 | **15 ✅** | 39 | +25 | Higher | +1 | Higher | -24 | Lower |
| Comments (CLOC) | 4 | **9 ✅** | 6 | -5 | Lower | -2 | Lower | +3 | Higher |
| Blank Lines | 9 | **8 ✅** | 10 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p03646

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.2748 | **73.4775 ✅** | 69.6340 | -1.2028 | Lower | +2.6407 | Higher | +3.8435 | Higher |
| MI (SEI) | 29.4860 | **69.3093 ✅** | 41.9210 | -39.8233 | Lower | -12.4350 | Lower | +27.3883 | Higher |
| Cyclomatic (sum) | 9 | **7 ✅** | 11 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 15 | **1 ✅** | 19 | +14 | Higher | -4 | Lower | -18 | Lower |
| Halstead Volume | 1415.2564 | **968.2597 ✅** | 1271.1819 | +446.9967 | Higher | +144.0746 | Higher | -302.9221 | Lower |
| Halstead Effort | 47262.1162 | **20304.9764 ✅** | 37686.8039 | +26957.1399 | Higher | +9575.3124 | Higher | -17381.8275 | Lower |
| SLOC | **38 ✅** | 41 | 45 | -3 | Lower | -7 | Lower | -4 | Lower |
| PLOC | 34 | **26 ✅** | 41 | +8 | Higher | -7 | Lower | -15 | Lower |
| LLOC | 26 | **11 ✅** | 30 | +15 | Higher | -4 | Lower | -19 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 2 | -13 | Lower | -2 | Lower | +11 | Higher |
| Blank Lines | 4 | 5 | 4 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03647

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.5395 | **72.4000 ✅** | 68.5184 | -7.8606 | Lower | -3.9789 | Lower | +3.8816 | Higher |
| MI (SEI) | 47.6248 | **63.9228 ✅** | 49.2826 | -16.2980 | Lower | -1.6578 | Lower | +14.6402 | Higher |
| Cyclomatic (sum) | 11 | **8 ✅** | 10 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 12 | **8 ✅** | 16 | +4 | Higher | -4 | Lower | -8 | Lower |
| Halstead Volume | 1713.2098 | **796.3187 ✅** | 1049.4768 | +916.8912 | Higher | +663.7331 | Higher | -253.1581 | Lower |
| Halstead Effort | 28735.2016 | **14652.2641 ✅** | 27328.3751 | +14082.9375 | Higher | +1406.8264 | Higher | -12676.1111 | Lower |
| SLOC | 56 | **46 ✅** | 52 | +10 | Higher | +4 | Higher | -6 | Lower |
| PLOC | 40 | **29 ✅** | 39 | +11 | Higher | +1 | Higher | -10 | Lower |
| LLOC | 26 | **20 ✅** | 29 | +6 | Higher | -3 | Lower | -9 | Lower |
| Comments (CLOC) | 9 | **11 ✅** | 6 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | 10 | **7 ✅** | 9 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03701

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.6463 | 69.0775 | **74.3276 ✅** | -4.4312 | Lower | -9.6813 | Lower | -5.2501 | Lower |
| MI (SEI) | 50.9250 | **61.4509 ✅** | 56.0456 | -10.5259 | Lower | -5.1206 | Lower | +5.4053 | Higher |
| Cyclomatic (sum) | 14 | 12 | **7 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| Cognitive (sum) | 17 | 11 | **9 ✅** | +6 | Higher | +8 | Higher | +2 | Higher |
| Halstead Volume | 1185.5369 | **916.5072 ✅** | 960.8600 | +269.0297 | Higher | +224.6769 | Higher | -44.3528 | Lower |
| Halstead Effort | 36518.7705 | **23792.5259 ✅** | 28951.1285 | +12726.2446 | Higher | +7567.6420 | Higher | -5158.6026 | Lower |
| SLOC | 60 | 51 | **39 ✅** | +9 | Higher | +21 | Higher | +12 | Higher |
| PLOC | 44 | 33 | **31 ✅** | +11 | Higher | +13 | Higher | +2 | Higher |
| LLOC | 28 | **20 ✅** | 28 | +8 | Higher | 0 | Same | -8 | Lower |
| Comments (CLOC) | 12 | **14 ✅** | 4 | -2 | Lower | +8 | Higher | +10 | Higher |
| Blank Lines | 10 | 9 | **6 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 4 | 4 | **0 ✅** | 0 | Same | +4 | Higher | +4 | Higher |

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

## p03722

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.6682 ✅** | 57.5327 | 61.3245 | +4.1354 | Higher | +0.3437 | Higher | -3.7918 | Lower |
| MI (SEI) | 37.3270 | **41.6689 ✅** | 13.9935 | -4.3419 | Lower | +23.3335 | Higher | +27.6754 | Higher |
| Cyclomatic (sum) | 12 | 22 | 12 | -10 | Lower | 0 | Same | +10 | Higher |
| Cognitive (sum) | 17 | 38 | 17 | -21 | Lower | 0 | Same | +21 | Higher |
| Halstead Volume | **1878.1906 ✅** | 2114.7311 | 2006.5142 | -236.5406 | Lower | -128.3236 | Lower | +108.2170 | Higher |
| Halstead Effort | **68980.8178 ✅** | 119354.1438 | 82449.4911 | -50373.3260 | Lower | -13468.6733 | Lower | +36904.6527 | Higher |
| SLOC | 64 | 69 | 64 | -5 | Lower | 0 | Same | +5 | Higher |
| PLOC | **48 ✅** | 53 | 54 | -5 | Lower | -6 | Lower | -1 | Lower |
| LLOC | **32 ✅** | 38 | 38 | -6 | Lower | -6 | Lower | 0 | Same |
| Comments (CLOC) | 6 | **14 ✅** | 0 | -8 | Lower | +6 | Higher | +14 | Higher |
| Blank Lines | 10 | **8 ✅** | 10 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p03736

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.4116 | **69.3655 ✅** | 60.3631 | -7.9539 | Lower | +1.0485 | Higher | +9.0024 | Higher |
| MI (SEI) | 13.8138 | **66.6932 ✅** | 33.7990 | -52.8794 | Lower | -19.9853 | Lower | +32.8941 | Higher |
| Cyclomatic (sum) | 9 | **4 ✅** | 11 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 5 | **1 ✅** | 7 | +4 | Higher | -2 | Lower | -6 | Lower |
| Halstead Volume | 2754.9445 | **1235.2362 ✅** | 2785.4872 | +1519.7084 | Higher | -30.5426 | Lower | -1550.2510 | Lower |
| Halstead Effort | 154793.4458 | **35353.3109 ✅** | 156509.5594 | +119440.1348 | Higher | -1716.1136 | Lower | -121156.2484 | Lower |
| SLOC | 60 | **51 ✅** | 62 | +9 | Higher | -2 | Lower | -11 | Lower |
| PLOC | 50 | **29 ✅** | 50 | +21 | Higher | 0 | Same | -21 | Lower |
| LLOC | 45 | **29 ✅** | 44 | +16 | Higher | +1 | Higher | -15 | Lower |
| Comments (CLOC) | 0 | **21 ✅** | 5 | -21 | Lower | -5 | Lower | +16 | Higher |
| Blank Lines | **10 ✅** | 11 | 12 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 3 | **2 ✅** | 4 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |

## p03762

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.4193 | **79.7674 ✅** | 66.8028 | -14.3481 | Lower | -1.3835 | Lower | +12.9646 | Higher |
| MI (SEI) | 49.1519 | **80.2095 ✅** | 53.1618 | -31.0576 | Lower | -4.0099 | Lower | +27.0477 | Higher |
| Cyclomatic (sum) | 9 | **5 ✅** | 8 | +4 | Higher | +1 | Higher | -3 | Lower |
| Cognitive (sum) | 6 | **0 ✅** | 6 | +6 | Higher | 0 | Same | -6 | Lower |
| Halstead Volume | 1769.9274 | **835.0936 ✅** | 1694.1496 | +934.8339 | Higher | +75.7778 | Higher | -859.0560 | Lower |
| Halstead Effort | 79097.4461 | **15104.3007 ✅** | 73797.1566 | +63993.1454 | Higher | +5300.2895 | Higher | -58692.8558 | Lower |
| SLOC | 54 | **30 ✅** | 51 | +24 | Higher | +3 | Higher | -21 | Lower |
| PLOC | 37 | **15 ✅** | 32 | +22 | Higher | +5 | Higher | -17 | Lower |
| LLOC | 30 | **10 ✅** | 31 | +20 | Higher | -1 | Lower | -21 | Lower |
| Comments (CLOC) | 9 | **11 ✅** | 10 | -2 | Lower | -1 | Lower | +1 | Higher |
| Blank Lines | 10 | **6 ✅** | 11 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03766

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.2524 ✅** | 63.7105 | 66.5289 | +5.5419 | Higher | +2.7235 | Higher | -2.8184 | Lower |
| MI (SEI) | 41.2396 | **49.6975 ✅** | 36.3399 | -8.4579 | Lower | +4.8997 | Higher | +13.3576 | Higher |
| Cyclomatic (sum) | 8 | 13 | 8 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Halstead Volume | 1675.3826 | **1563.2145 ✅** | 1899.3886 | +112.1680 | Higher | -224.0061 | Lower | -336.1741 | Lower |
| Halstead Effort | 85444.5108 | **57399.2833 ✅** | 92638.3633 | +28045.2275 | Higher | -7193.8525 | Lower | -35239.0800 | Lower |
| SLOC | **44 ✅** | 59 | 50 | -15 | Lower | -6 | Lower | +9 | Higher |
| PLOC | **36 ✅** | 49 | 41 | -13 | Lower | -5 | Lower | +8 | Higher |
| LLOC | 28 | 28 | 33 | 0 | Same | -5 | Lower | -5 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 2 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p03797

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.4294 | 82.6886 | **83.2057 ✅** | -7.2592 | Lower | -7.7762 | Lower | -0.5171 | Lower |
| MI (SEI) | 51.3341 | **91.2620 ✅** | 60.0939 | -39.9279 | Lower | -8.7598 | Lower | +31.1681 | Higher |
| Cyclomatic (sum) | 8 | **4 ✅** | 5 | +4 | Higher | +3 | Higher | -1 | Lower |
| Cognitive (sum) | 6 | **2 ✅** | 4 | +4 | Higher | +2 | Higher | -2 | Lower |
| Halstead Volume | 806.4400 | **282.0264 ✅** | 760.7724 | +524.4137 | Higher | +45.6677 | Higher | -478.7460 | Lower |
| Halstead Effort | 21370.6612 | **3948.3694 ✅** | 14884.6771 | +17422.2918 | Higher | +6485.9841 | Higher | -10936.3077 | Lower |
| SLOC | 38 | 36 | **25 ✅** | +2 | Higher | +13 | Higher | +11 | Higher |
| PLOC | 29 | **14 ✅** | 20 | +15 | Higher | +9 | Higher | -6 | Lower |
| LLOC | 13 | **6 ✅** | 17 | +7 | Higher | -4 | Lower | -11 | Lower |
| Comments (CLOC) | 2 | **23 ✅** | 1 | -21 | Lower | +1 | Higher | +22 | Higher |
| Blank Lines | 7 | **3 ✅** | 4 | +4 | Higher | +3 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p03808

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.1584 | **68.2550 ✅** | 62.5324 | -8.0966 | Lower | -2.3740 | Lower | +5.7226 | Higher |
| MI (SEI) | 33.9025 | **61.9644 ✅** | 37.4608 | -28.0619 | Lower | -3.5583 | Lower | +24.5037 | Higher |
| Cyclomatic (sum) | 18 | 13 | 13 | +5 | Higher | +5 | Higher | 0 | Same |
| Cognitive (sum) | 26 | **12 ✅** | 22 | +14 | Higher | +4 | Higher | -10 | Lower |
| Halstead Volume | 1925.6463 | **966.8211 ✅** | 1860.6705 | +958.8252 | Higher | +64.9758 | Higher | -893.8494 | Lower |
| Halstead Effort | 133799.2194 | **24587.2607 ✅** | 127588.8369 | +109211.9587 | Higher | +6210.3826 | Higher | -103001.5761 | Lower |
| SLOC | 64 | **52 ✅** | 60 | +12 | Higher | +4 | Higher | -8 | Lower |
| PLOC | 54 | **34 ✅** | 54 | +20 | Higher | 0 | Same | -20 | Lower |
| LLOC | 42 | **16 ✅** | 45 | +26 | Higher | -3 | Lower | -29 | Lower |
| Comments (CLOC) | 5 | **16 ✅** | 5 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 9 | 8 | **6 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 3 | 5 | **0 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |

## p03815

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.0326 | **90.4373 ✅** | 87.1140 | -2.4047 | Lower | +0.9186 | Higher | +3.3233 | Higher |
| MI (SEI) | 82.3892 | **94.0706 ✅** | 80.4875 | -11.6814 | Lower | +1.9017 | Higher | +13.5831 | Higher |
| Cyclomatic (sum) | 4 | **3 ✅** | 4 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Halstead Volume | 468.0466 | **356.1224 ✅** | 486.2570 | +111.9242 | Higher | -18.2104 | Lower | -130.1346 | Lower |
| Halstead Effort | 7956.7919 | **4115.1922 ✅** | 8314.9948 | +3841.5997 | Higher | -358.2029 | Lower | -4199.8026 | Lower |
| SLOC | 22 | **21 ✅** | 23 | +1 | Higher | -1 | Lower | -2 | Lower |
| PLOC | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| LLOC | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 4 | **7 ✅** | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 4 | 4 | 5 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03817

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 83.6018 | **89.8841 ✅** | 68.0360 | -6.2823 | Lower | +15.5659 | Higher | +21.8482 | Higher |
| MI (SEI) | 76.3387 | **96.0033 ✅** | 39.0232 | -19.6646 | Lower | +37.3155 | Higher | +56.9800 | Higher |
| Cyclomatic (sum) | 5 | **2 ✅** | 13 | +3 | Higher | -8 | Lower | -11 | Lower |
| Cognitive (sum) | 4 | **0 ✅** | 9 | +4 | Higher | -5 | Lower | -9 | Lower |
| Halstead Volume | 554.6772 | **358.1520 ✅** | 1139.4677 | +196.5252 | Higher | -584.7906 | Lower | -781.3157 | Lower |
| Halstead Effort | 9464.1791 | **4382.0948 ✅** | 33393.7558 | +5082.0842 | Higher | -23929.5767 | Lower | -29011.6609 | Lower |
| SLOC | 27 | **22 ✅** | 50 | +5 | Higher | -23 | Lower | -28 | Lower |
| PLOC | 17 | **10 ✅** | 39 | +7 | Higher | -22 | Lower | -29 | Lower |
| LLOC | 8 | **7 ✅** | 10 | +1 | Higher | -2 | Lower | -3 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 2 | -4 | Lower | +3 | Higher | +7 | Higher |
| Blank Lines | 5 | 5 | 10 | 0 | Same | -5 | Lower | -5 | Lower |
| Functions | 1 | 1 | 7 | 0 | Same | -6 | Lower | -6 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 6 | 0 | Same | -6 | Lower | -6 | Lower |

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

## p03838

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.2566 | **78.7472 ✅** | 72.6453 | -14.4906 | Lower | -8.3887 | Lower | +6.1019 | Higher |
| MI (SEI) | 38.8160 | **69.4005 ✅** | 58.7709 | -30.5845 | Lower | -19.9549 | Lower | +10.6296 | Higher |
| Cyclomatic (sum) | 15 | **8 ✅** | 11 | +7 | Higher | +4 | Higher | -3 | Lower |
| Cognitive (sum) | 14 | **2 ✅** | 12 | +12 | Higher | +2 | Higher | -10 | Lower |
| Halstead Volume | 1603.1646 | **661.2389 ✅** | 1112.5606 | +941.9257 | Higher | +490.6039 | Higher | -451.3218 | Lower |
| Halstead Effort | 35932.4672 | **10170.4835 ✅** | 36046.9645 | +25761.9837 | Higher | -114.4973 | Lower | -25876.4810 | Lower |
| SLOC | 55 | **33 ✅** | 39 | +22 | Higher | +16 | Higher | -6 | Lower |
| PLOC | 45 | **22 ✅** | 31 | +23 | Higher | +14 | Higher | -9 | Lower |
| LLOC | 13 | **11 ✅** | 26 | +2 | Higher | -13 | Lower | -15 | Lower |
| Comments (CLOC) | 4 | 6 | 6 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p03845

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.3089 | **72.1984 ✅** | 71.2737 | -2.8895 | Lower | -1.9647 | Lower | +0.9248 | Higher |
| MI (SEI) | 56.2604 | **63.0456 ✅** | 59.9719 | -6.7853 | Lower | -3.7115 | Lower | +3.0737 | Higher |
| Cyclomatic (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | **3 ✅** | 4 | 6 | -1 | Lower | -3 | Lower | -2 | Lower |
| Halstead Volume | 1474.2467 | **1115.8024 ✅** | 1240.8509 | +358.4443 | Higher | +233.3959 | Higher | -125.0485 | Lower |
| Halstead Effort | 34960.7081 | 27151.1923 | **25217.2920 ✅** | +7809.5158 | Higher | +9743.4161 | Higher | +1933.9003 | Higher |
| SLOC | 47 | **43 ✅** | 44 | +4 | Higher | +3 | Higher | -1 | Lower |
| PLOC | 29 | **26 ✅** | 29 | +3 | Higher | 0 | Same | -3 | Lower |
| LLOC | 23 | **18 ✅** | 21 | +5 | Higher | +2 | Higher | -3 | Lower |
| Comments (CLOC) | 9 | **10 ✅** | 9 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | 9 | 8 | **7 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

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

## p03899

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 58.5999 | **61.7118 ✅** | 58.7546 | -3.1119 | Lower | -0.1547 | Lower | +2.9572 | Higher |
| MI (SEI) | 32.5254 | **47.4614 ✅** | 20.1401 | -14.9360 | Lower | +12.3854 | Higher | +27.3214 | Higher |
| Cyclomatic (sum) | 16 | **11 ✅** | 15 | +5 | Higher | +1 | Higher | -4 | Lower |
| Cognitive (sum) | 14 | **7 ✅** | 15 | +7 | Higher | -1 | Lower | -8 | Lower |
| Halstead Volume | **2245.8527 ✅** | 2380.2573 | 2744.5536 | -134.4046 | Lower | -498.7009 | Lower | -364.2964 | Lower |
| Halstead Effort | 88609.0975 | **58456.3180 ✅** | 88899.6723 | +30152.7795 | Higher | -290.5748 | Lower | -30443.3543 | Lower |
| SLOC | 69 | **60 ✅** | 65 | +9 | Higher | +4 | Higher | -5 | Lower |
| PLOC | 51 | **37 ✅** | 52 | +14 | Higher | -1 | Lower | -15 | Lower |
| LLOC | 38 | **23 ✅** | 36 | +15 | Higher | +2 | Higher | -13 | Lower |
| Comments (CLOC) | 6 | **13 ✅** | 1 | -7 | Lower | +5 | Higher | +12 | Higher |
| Blank Lines | 12 | 13 | 12 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 5 | 3 | 3 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03901

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.0978 ✅** | 72.9837 | 45.3269 | +1.1142 | Higher | +28.7709 | Higher | +27.6568 | Higher |
| MI (SEI) | **65.9053 ✅** | 61.7974 | 18.6633 | +4.1079 | Higher | +47.2420 | Higher | +43.1341 | Higher |
| Cyclomatic (sum) | **4 ✅** | 9 | 14 | -5 | Lower | -10 | Lower | -5 | Lower |
| Cognitive (sum) | **1 ✅** | 4 | 11 | -3 | Lower | -10 | Lower | -7 | Lower |
| Halstead Volume | 910.3489 | **904.0980 ✅** | 3399.4941 | +6.2509 | Higher | -2489.1452 | Lower | -2495.3961 | Lower |
| Halstead Effort | 18763.3021 | **18364.4897 ✅** | 148522.7230 | +398.8124 | Higher | -129759.4210 | Lower | -130158.2334 | Lower |
| SLOC | 42 | 42 | 141 | 0 | Same | -99 | Lower | -99 | Lower |
| PLOC | **25 ✅** | 27 | 103 | -2 | Lower | -78 | Lower | -76 | Lower |
| LLOC | 15 | 15 | 69 | 0 | Same | -54 | Lower | -54 | Lower |
| Comments (CLOC) | 10 | 8 | **20 ✅** | +2 | Higher | -10 | Lower | -12 | Lower |
| Blank Lines | 9 | **7 ✅** | 20 | +2 | Higher | -11 | Lower | -13 | Lower |
| Functions | 2 | **1 ✅** | 5 | +1 | Higher | -3 | Lower | -4 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p03914

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.2309 | 73.0069 | **78.1423 ✅** | +2.2240 | Higher | -2.9113 | Lower | -5.1354 | Lower |
| MI (SEI) | 52.0557 | **72.2227 ✅** | 37.5437 | -20.1671 | Lower | +14.5120 | Higher | +34.6790 | Higher |
| Cyclomatic (sum) | 6 | 9 | **5 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 6 | 11 | 6 | -5 | Lower | 0 | Same | +5 | Higher |
| Halstead Volume | 1420.5243 | **900.0687 ✅** | 1268.6181 | +520.4555 | Higher | +151.9062 | Higher | -368.5493 | Lower |
| Halstead Effort | 44746.5146 | **31288.1037 ✅** | 39863.8831 | +13458.4109 | Higher | +4882.6315 | Higher | -8575.7793 | Lower |
| SLOC | 33 | 42 | **29 ✅** | -9 | Lower | +4 | Higher | +13 | Higher |
| PLOC | 25 | 26 | **24 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| LLOC | 23 | **17 ✅** | 22 | +6 | Higher | +1 | Higher | -5 | Lower |
| Comments (CLOC) | 2 | **17 ✅** | 0 | -15 | Lower | +2 | Higher | +17 | Higher |
| Blank Lines | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p03929

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.9682 | **80.9824 ✅** | 67.1079 | -14.0142 | Lower | -0.1397 | Lower | +13.8745 | Higher |
| MI (SEI) | 51.1289 | **77.0528 ✅** | 43.3694 | -25.9239 | Lower | +7.7595 | Higher | +33.6834 | Higher |
| Cyclomatic (sum) | 14 | **5 ✅** | 14 | +9 | Higher | 0 | Same | -9 | Lower |
| Cognitive (sum) | 16 | **4 ✅** | 24 | +12 | Higher | -8 | Lower | -20 | Lower |
| Halstead Volume | 1258.5799 | **596.8891 ✅** | 1153.2892 | +661.6908 | Higher | +105.2908 | Higher | -556.4000 | Lower |
| Halstead Effort | 49955.9418 | **9861.6464 ✅** | 47625.5997 | +40094.2955 | Higher | +2330.3422 | Higher | -37763.9533 | Lower |
| SLOC | 51 | **31 ✅** | 52 | +20 | Higher | -1 | Lower | -21 | Lower |
| PLOC | 34 | **18 ✅** | 41 | +16 | Higher | -7 | Lower | -23 | Lower |
| LLOC | 28 | **13 ✅** | 31 | +15 | Higher | -3 | Lower | -18 | Lower |
| Comments (CLOC) | 8 | 8 | 4 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 9 | **6 ✅** | 7 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03932

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 43.1436 | **60.5139 ✅** | 36.4814 | -17.3703 | Lower | +6.6623 | Higher | +24.0325 | Higher |
| MI (SEI) | 18.7098 | **45.4478 ✅** | 12.0027 | -26.7380 | Lower | +6.7070 | Higher | +33.4451 | Higher |
| Cyclomatic (sum) | 21 | **13 ✅** | 27 | +8 | Higher | -6 | Lower | -14 | Lower |
| Cognitive (sum) | 18 | **8 ✅** | 28 | +10 | Higher | -10 | Lower | -20 | Lower |
| Halstead Volume | 4553.2665 | **1945.1520 ✅** | 5853.2478 | +2608.1145 | Higher | -1299.9814 | Lower | -3908.0959 | Lower |
| Halstead Effort | 425253.9096 | **108157.1573 ✅** | 466749.3120 | +317096.7524 | Higher | -41495.4024 | Lower | -358592.1547 | Lower |
| SLOC | 133 | **67 ✅** | 170 | +66 | Higher | -37 | Lower | -103 | Lower |
| PLOC | 97 | **41 ✅** | 116 | +56 | Higher | -19 | Lower | -75 | Lower |
| LLOC | 63 | **29 ✅** | 75 | +34 | Higher | -12 | Lower | -46 | Lower |
| Comments (CLOC) | 23 | 14 | **35 ✅** | +9 | Higher | -12 | Lower | -21 | Lower |
| Blank Lines | 20 | **14 ✅** | 26 | +6 | Higher | -6 | Lower | -12 | Lower |
| Functions | 5 | **2 ✅** | 7 | +3 | Higher | -2 | Lower | -5 | Lower |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 5 | **2 ✅** | 6 | +3 | Higher | -1 | Lower | -4 | Lower |

## p03951

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.3167 | **71.2285 ✅** | 69.5860 | -10.9118 | Lower | -9.2694 | Lower | +1.6425 | Higher |
| MI (SEI) | 44.9947 | **60.1104 ✅** | 50.4981 | -15.1157 | Lower | -5.5034 | Lower | +9.6123 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 11 | +5 | Higher | +2 | Higher | -3 | Lower |
| Cognitive (sum) | 23 | **7 ✅** | 15 | +16 | Higher | +8 | Higher | -8 | Lower |
| Halstead Volume | 2447.4426 | **1145.7028 ✅** | 1282.9664 | +1301.7398 | Higher | +1164.4762 | Higher | -137.2636 | Lower |
| Halstead Effort | 60928.4405 | **37015.0137 ✅** | 58313.8789 | +23913.4268 | Higher | +2614.5616 | Higher | -21298.8652 | Lower |
| SLOC | 63 | **44 ✅** | 45 | +19 | Higher | +18 | Higher | -1 | Lower |
| PLOC | 47 | **28 ✅** | 37 | +19 | Higher | +10 | Higher | -9 | Lower |
| LLOC | 42 | **23 ✅** | 34 | +19 | Higher | +8 | Higher | -11 | Lower |
| Comments (CLOC) | **13 ✅** | 9 | 5 | +4 | Higher | +8 | Higher | +4 | Higher |
| Blank Lines | 10 | 9 | **7 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03977

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.5097 | **69.0360 ✅** | 59.3961 | -3.5263 | Lower | +6.1136 | Higher | +9.6399 | Higher |
| MI (SEI) | 51.5947 | **62.1531 ✅** | 35.7966 | -10.5584 | Lower | +15.7981 | Higher | +26.3565 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 14 | +3 | Higher | -1 | Lower | -4 | Lower |
| Cognitive (sum) | 18 | **13 ✅** | 19 | +5 | Higher | -1 | Lower | -6 | Lower |
| Halstead Volume | 1231.4296 | **1009.2852 ✅** | 1692.9565 | +222.1444 | Higher | -461.5269 | Lower | -683.6713 | Lower |
| Halstead Effort | 41948.0541 | **23571.0158 ✅** | 81098.0779 | +18377.0383 | Higher | -39150.0237 | Lower | -57527.0621 | Lower |
| SLOC | 57 | **51 ✅** | 74 | +6 | Higher | -17 | Lower | -23 | Lower |
| PLOC | 36 | **34 ✅** | 56 | +2 | Higher | -20 | Lower | -22 | Lower |
| LLOC | 29 | **22 ✅** | 49 | +7 | Higher | -20 | Lower | -27 | Lower |
| Comments (CLOC) | 11 | **15 ✅** | 8 | -4 | Lower | +3 | Higher | +7 | Higher |
| Blank Lines | 11 | **6 ✅** | 10 | +5 | Higher | +1 | Higher | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p03996

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.3642 | **78.7016 ✅** | 55.0409 | -22.3374 | Lower | +1.3233 | Higher | +23.6607 | Higher |
| MI (SEI) | 25.9254 | **81.7599 ✅** | 24.2199 | -55.8345 | Lower | +1.7055 | Higher | +57.5401 | Higher |
| Cyclomatic (sum) | 14 | **6 ✅** | 16 | +8 | Higher | -2 | Lower | -10 | Lower |
| Cognitive (sum) | 19 | **5 ✅** | 21 | +14 | Higher | -2 | Lower | -16 | Lower |
| Halstead Volume | 2378.9574 | **606.6998 ✅** | 2808.5993 | +1772.2576 | Higher | -429.6419 | Lower | -2201.8995 | Lower |
| Halstead Effort | 99384.0760 | **14081.8217 ✅** | 148966.6284 | +85302.2543 | Higher | -49582.5524 | Lower | -134884.8067 | Lower |
| SLOC | 80 | **35 ✅** | 80 | +45 | Higher | 0 | Same | -45 | Lower |
| PLOC | 65 | **20 ✅** | 64 | +45 | Higher | +1 | Higher | -44 | Lower |
| LLOC | 50 | **13 ✅** | 51 | +37 | Higher | -1 | Lower | -38 | Lower |
| Comments (CLOC) | 5 | **16 ✅** | 5 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 10 | **6 ✅** | 11 | +4 | Higher | -1 | Lower | -5 | Lower |
| Functions | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | **1 ✅** | 3 | 2 | -2 | Lower | -1 | Lower | +1 | Higher |

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

## p04013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.7366 | **70.7282 ✅** | 62.6594 | -7.9916 | Lower | +0.0772 | Higher | +8.0688 | Higher |
| MI (SEI) | 37.0026 | **62.9185 ✅** | 27.1896 | -25.9159 | Lower | +9.8130 | Higher | +35.7289 | Higher |
| Cyclomatic (sum) | 17 | **10 ✅** | 18 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 20 | **8 ✅** | 20 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | 2081.2895 | **1005.2933 ✅** | 2273.1806 | +1075.9962 | Higher | -191.8911 | Lower | -1267.8873 | Lower |
| Halstead Effort | 119674.1461 | **43945.6764 ✅** | 155712.8686 | +75728.4697 | Higher | -36038.7225 | Lower | -111767.1922 | Lower |
| SLOC | 54 | **46 ✅** | 52 | +8 | Higher | +2 | Higher | -6 | Lower |
| PLOC | 41 | **29 ✅** | 42 | +12 | Higher | -1 | Lower | -13 | Lower |
| LLOC | 35 | **22 ✅** | 35 | +13 | Higher | 0 | Same | -13 | Lower |
| Comments (CLOC) | 4 | **12 ✅** | 1 | -8 | Lower | +3 | Higher | +11 | Higher |
| Blank Lines | 9 | **7 ✅** | 9 | +2 | Higher | 0 | Same | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p04020

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.8015 ✅** | 49.3633 | 75.4942 | +26.4382 | Higher | +0.3073 | Higher | -26.1309 | Lower |
| MI (SEI) | **62.0029 ✅** | 36.0660 | 59.5045 | +25.9369 | Higher | +2.4983 | Higher | -23.4386 | Lower |
| Cyclomatic (sum) | 7 | 26 | **6 ✅** | -19 | Lower | +1 | Higher | +20 | Higher |
| Cognitive (sum) | 5 | 26 | 5 | -21 | Lower | 0 | Same | +21 | Higher |
| Halstead Volume | **1013.8419 ✅** | 2173.3109 | 1350.3881 | -1159.4690 | Lower | -336.5461 | Lower | +822.9229 | Higher |
| Halstead Effort | 34217.1648 | 124930.3262 | **31649.7199 ✅** | -90713.1614 | Lower | +2567.4449 | Higher | +93280.6062 | Higher |
| SLOC | 35 | 107 | **33 ✅** | -72 | Lower | +2 | Higher | +74 | Higher |
| PLOC | **23 ✅** | 70 | 26 | -47 | Lower | -3 | Lower | +44 | Higher |
| LLOC | **21 ✅** | 40 | 22 | -19 | Lower | -1 | Lower | +18 | Higher |
| Comments (CLOC) | 5 | **33 ✅** | 4 | -28 | Lower | +1 | Higher | +29 | Higher |
| Blank Lines | 7 | 20 | **5 ✅** | -13 | Lower | +2 | Higher | +15 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 8 | 0 | -8 | Lower | 0 | Same | +8 | Higher |

## p04022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 32.7245 | **60.6803 ✅** | 15.9261 | -27.9558 | Lower | +16.7985 | Higher | +44.7542 | Higher |
| MI (SEI) | -24.7220 | **43.5002 ✅** | -38.2636 | -68.2222 | Lower | +13.5416 | Higher | +81.7638 | Higher |
| Cyclomatic (sum) | 37 | **14 ✅** | 66 | +23 | Higher | -29 | Lower | -52 | Lower |
| Cognitive (sum) | 85 | **24 ✅** | 96 | +61 | Higher | -11 | Lower | -72 | Lower |
| Halstead Volume | 5301.6684 | **1322.5020 ✅** | 9452.5693 | +3979.1664 | Higher | -4150.9009 | Lower | -8130.0673 | Lower |
| Halstead Effort | 488538.9272 | **54883.8336 ✅** | 978818.8588 | +433655.0936 | Higher | -490279.9315 | Lower | -923935.0252 | Lower |
| SLOC | 192 | **74 ✅** | 298 | +118 | Higher | -106 | Lower | -224 | Lower |
| PLOC | 170 | **50 ✅** | 271 | +120 | Higher | -101 | Lower | -221 | Lower |
| LLOC | 150 | **40 ✅** | 196 | +110 | Higher | -46 | Lower | -156 | Lower |
| Comments (CLOC) | 0 | **13 ✅** | 3 | -13 | Lower | -3 | Lower | +10 | Higher |
| Blank Lines | 22 | **12 ✅** | 27 | +10 | Higher | -5 | Lower | -15 | Lower |
| Functions | 3 | **2 ✅** | 20 | +1 | Higher | -17 | Lower | -18 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 3 | **2 ✅** | 18 | +1 | Higher | -15 | Lower | -16 | Lower |

## p04024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.7127 | **62.1734 ✅** | 40.7904 | -21.4608 | Lower | -0.0777 | Lower | +21.3830 | Higher |
| MI (SEI) | -6.4742 | **48.9794 ✅** | -6.0499 | -55.4537 | Lower | -0.4243 | Lower | +55.0294 | Higher |
| Cyclomatic (sum) | 36 | **15 ✅** | 38 | +21 | Higher | -2 | Lower | -23 | Lower |
| Cognitive (sum) | 47 | **15 ✅** | 51 | +32 | Higher | -4 | Lower | -36 | Lower |
| Halstead Volume | 4217.5473 | **1293.9998 ✅** | 4198.6073 | +2923.5475 | Higher | +18.9401 | Higher | -2904.6075 | Lower |
| Halstead Effort | 329979.1467 | **56288.9920 ✅** | 370223.8605 | +273690.1547 | Higher | -40244.7138 | Lower | -313934.8685 | Lower |
| SLOC | 128 | **67 ✅** | 124 | +61 | Higher | +4 | Higher | -57 | Lower |
| PLOC | 106 | **45 ✅** | 106 | +61 | Higher | 0 | Same | -61 | Lower |
| LLOC | 79 | **30 ✅** | 74 | +49 | Higher | +5 | Higher | -44 | Lower |
| Comments (CLOC) | 1 | **15 ✅** | 1 | -14 | Lower | 0 | Same | +14 | Higher |
| Blank Lines | 21 | **11 ✅** | 18 | +10 | Higher | +3 | Higher | -7 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
| Exits | 9 | **3 ✅** | 6 | +6 | Higher | +3 | Higher | -3 | Lower |

## p04030

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.5622 | 73.3310 | **81.6274 ✅** | +5.2311 | Higher | -3.0652 | Lower | -8.2963 | Lower |
| MI (SEI) | 76.0241 | 69.4034 | **82.8929 ✅** | +6.6207 | Higher | -6.8688 | Lower | -13.4895 | Lower |
| Cyclomatic (sum) | 6 | 10 | **5 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Cognitive (sum) | 9 | 11 | **7 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Halstead Volume | **570.8234 ✅** | 872.1673 | 583.9732 | -301.3438 | Lower | -13.1497 | Lower | +288.1941 | Higher |
| Halstead Effort | **9296.2674 ✅** | 23015.5254 | 10591.1497 | -13719.2579 | Lower | -1294.8823 | Lower | +12424.3756 | Higher |
| SLOC | 36 | 41 | **30 ✅** | -5 | Lower | +6 | Higher | +11 | Higher |
| PLOC | 24 | 27 | **20 ✅** | -3 | Lower | +4 | Higher | +7 | Higher |
| LLOC | 17 | 18 | **16 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Comments (CLOC) | 11 | **13 ✅** | 11 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.4986 | **67.6159 ✅** | 60.6158 | -6.1173 | Lower | 0.8828 | Higher | 7.0001 | Higher |
| MI (SEI) | 33.6438 | **55.7100 ✅** | 30.0590 | -22.0662 | Lower | 3.5848 | Higher | 25.6510 | Higher |
| Cyclomatic (sum) | 16.2049 | **11.9011 ✅** | 17.2049 | 4.3039 | Higher | -1 | Lower | -5.3039 | Lower |
| Cognitive (sum) | 20.2898 | **11.1625 ✅** | 21.7986 | 9.1272 | Higher | -1.5088 | Lower | -10.6360 | Lower |
| Halstead Volume | 2404.0190 | **1430.9229 ✅** | 2647.0305 | 973.0960 | Higher | -243.0115 | Lower | -1216.1076 | Lower |
| Halstead Effort | 149237.4321 | **54322.4753 ✅** | 188076.5041 | 94914.9568 | Higher | -38839.0719 | Lower | -133754.0287 | Lower |
| SLOC | 72.0601 | **56.1378 ✅** | 78.0919 | 15.9223 | Higher | -6.0318 | Lower | -21.9541 | Lower |
| PLOC | 58.4417 | **39.8057 ✅** | 65.3675 | 18.6360 | Higher | -6.9258 | Lower | -25.5618 | Lower |
| LLOC | 40.9823 | **23.4806 ✅** | 48.7102 | 17.5018 | Higher | -7.7279 | Lower | -25.2297 | Lower |
| Comments (CLOC) | 4.5972 | **11.3357 ✅** | 3.6996 | -6.7385 | Lower | 0.8975 | Higher | 7.6360 | Higher |
| Blank Lines | 10.7138 | **8.8233 ✅** | 10.6714 | 1.8905 | Higher | 0.0424 | Higher | -1.8481 | Lower |
| Functions | 2.4346 | **2.3534 ✅** | 3.0495 | 0.0813 | Higher | -0.6148 | Lower | -0.6961 | Lower |
| Closures | 0.8127 | 0.7633 | **0.4841 ✅** | 0.0495 | Higher | 0.3286 | Higher | 0.2792 | Higher |
| Exits | **2.0071 ✅** | 2.0389 | 2.2191 | -0.0318 | Lower | -0.2120 | Lower | -0.1802 | Lower |

