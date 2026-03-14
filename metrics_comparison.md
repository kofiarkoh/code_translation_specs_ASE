# Rust Metrics Comparison

Source: `r_code/metrics.json`

## p00001

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.5440 | 66.8800 | **79.1514 ✅** | +3.6641 | Higher | -8.6074 | Lower | -12.2715 | Lower |
| MI (SEI) | 48.0468 | 56.1627 | **65.6136 ✅** | -8.1159 | Lower | -17.5668 | Lower | -9.4509 | Lower |
| Cyclomatic (sum) | 16 | 10 | **7 ✅** | +6 | Higher | +9 | Higher | +3 | Higher |
| Cognitive (sum) | 20 | **4 ✅** | 6 | +16 | Higher | +14 | Higher | -2 | Lower |
| Halstead Volume | 1143.1713 | 2107.1385 | **776.9541 ✅** | -963.9672 | Lower | +366.2172 | Higher | +1330.1844 | Higher |
| Halstead Effort | 57215.7232 | 36701.0343 | **18011.2093 ✅** | +20514.6889 | Higher | +39204.5140 | Higher | +18689.8250 | Higher |
| SLOC | 41 | 46 | **31 ✅** | -5 | Lower | +10 | Higher | +15 | Higher |
| PLOC | 34 | 33 | **24 ✅** | +1 | Higher | +10 | Higher | +9 | Higher |
| LLOC | 32 | **16 ✅** | 22 | +16 | Higher | +10 | Higher | -6 | Lower |
| Comments (CLOC) | 3 | **11 ✅** | 4 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00002

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.1901 | 75.1249 | **88.0576 ✅** | +10.0651 | Higher | -2.8675 | Lower | -12.9327 | Lower |
| MI (SEI) | 80.1934 | 64.2720 | **83.7840 ✅** | +15.9213 | Higher | -3.5906 | Lower | -19.5120 | Lower |
| Cyclomatic (sum) | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 6 | 6 | 6 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | **589.8675 ✅** | 1667.8020 | 599.7113 | -1077.9344 | Lower | -9.8438 | Lower | +1068.0907 | Higher |
| Halstead Effort | 8941.1502 | 25594.3454 | **8875.7272 ✅** | -16653.1952 | Lower | +65.4229 | Higher | +16718.6182 | Higher |
| SLOC | 24 | 32 | **20 ✅** | -8 | Lower | +4 | Higher | +12 | Higher |
| PLOC | 16 | 22 | 16 | -6 | Lower | 0 | Same | +6 | Higher |
| LLOC | 10 | **9 ✅** | 10 | +1 | Higher | 0 | Same | -1 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 4 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00003

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.5855 ✅** | 73.8273 | 61.5021 | +3.7582 | Higher | +16.0834 | Higher | +12.3252 | Higher |
| MI (SEI) | **67.1331 ✅** | 64.3485 | 32.5224 | +2.7846 | Higher | +34.6108 | Higher | +31.8262 | Higher |
| Cyclomatic (sum) | **6 ✅** | 9 | 22 | -3 | Lower | -16 | Lower | -13 | Lower |
| Cognitive (sum) | **6 ✅** | 7 | 38 | -1 | Lower | -32 | Lower | -31 | Lower |
| Halstead Volume | **823.0107 ✅** | 1049.9507 | 1693.1877 | -226.9400 | Lower | -870.1769 | Lower | -643.2369 | Lower |
| Halstead Effort | **18929.2468 ✅** | 34998.3580 | 105231.6137 | -16069.1112 | Lower | -86302.3669 | Lower | -70233.2557 | Lower |
| SLOC | **34 ✅** | 38 | 58 | -4 | Lower | -24 | Lower | -20 | Lower |
| PLOC | **23 ✅** | 25 | 51 | -2 | Lower | -28 | Lower | -26 | Lower |
| LLOC | 14 | **13 ✅** | 29 | +1 | Higher | -15 | Lower | -16 | Lower |
| Comments (CLOC) | 6 | **8 ✅** | 3 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00004

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.2753 | 68.2142 | **73.4008 ✅** | +1.0611 | Higher | -4.1255 | Lower | -5.1866 | Lower |
| MI (SEI) | 53.7729 | 53.3213 | **59.0303 ✅** | +0.4516 | Higher | -5.2573 | Lower | -5.7089 | Lower |
| Cyclomatic (sum) | 11 | **8 ✅** | 20 | +3 | Higher | -9 | Lower | -12 | Lower |
| Cognitive (sum) | 17 | **9 ✅** | 21 | +8 | Higher | -4 | Lower | -12 | Lower |
| Halstead Volume | 1271.8301 | **1145.5841 ✅** | 1463.2602 | +126.2460 | Higher | -191.4300 | Lower | -317.6761 | Lower |
| Halstead Effort | 38688.2523 | **30080.8210 ✅** | 60578.9713 | +8607.4313 | Higher | -21890.7190 | Lower | -30498.1503 | Lower |
| SLOC | 46 | 53 | **30 ✅** | -7 | Lower | +16 | Higher | +23 | Higher |
| PLOC | 31 | 36 | **20 ✅** | -5 | Lower | +11 | Higher | +16 | Higher |
| LLOC | 17 | 22 | **16 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Comments (CLOC) | 7 | **9 ✅** | 4 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | 9 | 9 | **6 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00005

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.4608 | 69.1308 | **78.9736 ✅** | -2.6700 | Lower | -12.5127 | Lower | -9.8428 | Lower |
| MI (SEI) | 40.2658 | 42.2356 | **53.3366 ✅** | -1.9697 | Lower | -13.0708 | Lower | -11.1011 | Lower |
| Cyclomatic (sum) | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | **7 ✅** | 12 | 15 | -5 | Lower | -8 | Lower | -3 | Lower |
| Halstead Volume | 2495.2617 | 2285.9115 | **905.8642 ✅** | +209.3502 | Higher | +1589.3976 | Higher | +1380.0473 | Higher |
| Halstead Effort | 55368.9977 | 47312.0767 | **29152.3557 ✅** | +8056.9209 | Higher | +26216.6420 | Higher | +18159.7210 | Higher |
| SLOC | 46 | 39 | **29 ✅** | +7 | Higher | +17 | Higher | +10 | Higher |
| PLOC | 36 | 33 | **23 ✅** | +3 | Higher | +13 | Higher | +10 | Higher |
| LLOC | 21 | **11 ✅** | 19 | +10 | Higher | +2 | Higher | -8 | Lower |
| Comments (CLOC) | **3 ✅** | 2 | 1 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 7 | **4 ✅** | 5 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00006

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 88.2926 | **89.6434 ✅** | 86.6025 | -1.3508 | Lower | +1.6901 | Higher | +3.0409 | Higher |
| MI (SEI) | 90.4046 | **91.6499 ✅** | 84.5170 | -1.2454 | Lower | +5.8875 | Higher | +7.1329 | Higher |
| Cyclomatic (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 312.4781 | 312.4781 | 491.1429 | 0 | Same | -178.6648 | Lower | -178.6648 | Lower |
| Halstead Effort | 3281.0197 | 3281.0197 | 5469.5457 | 0 | Same | -2188.5259 | Lower | -2188.5259 | Lower |
| SLOC | 25 | **23 ✅** | 24 | +2 | Higher | +1 | Higher | -1 | Lower |
| PLOC | 11 | 11 | 13 | 0 | Same | -2 | Lower | -2 | Lower |
| LLOC | 7 | 7 | 9 | 0 | Same | -2 | Lower | -2 | Lower |
| Comments (CLOC) | **8 ✅** | 7 | 6 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00007

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.4037 | 71.6547 | **89.7658 ✅** | +4.7491 | Higher | -13.3621 | Lower | -18.1111 | Lower |
| MI (SEI) | 64.1756 | 62.4502 | **89.1825 ✅** | +1.7254 | Higher | -25.0069 | Lower | -26.7323 | Lower |
| Cyclomatic (sum) | 5 | 7 | **4 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 1439.7750 | 898.3536 | **451.3218 ✅** | +541.4214 | Higher | +988.4532 | Higher | +447.0318 | Higher |
| Halstead Effort | 22813.5773 | 20129.7746 | **8023.4981 ✅** | +2683.8028 | Higher | +14790.0793 | Higher | +12106.2765 | Higher |
| SLOC | 31 | 47 | **20 ✅** | -16 | Lower | +11 | Higher | +27 | Higher |
| PLOC | 21 | 29 | **14 ✅** | -8 | Lower | +7 | Higher | +15 | Higher |
| LLOC | 11 | 14 | 11 | -3 | Lower | 0 | Same | +3 | Higher |
| Comments (CLOC) | 5 | **11 ✅** | 5 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 7 | 9 | **4 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Functions | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00008

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.3775 | **77.6535 ✅** | 77.3918 | -2.2760 | Lower | -2.0143 | Lower | +0.2617 | Higher |
| MI (SEI) | **62.7127 ✅** | 50.5780 | 62.5095 | +12.1347 | Higher | +0.2032 | Higher | -11.9315 | Lower |
| Cyclomatic (sum) | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 21 | 21 | 26 | 0 | Same | -5 | Lower | -5 | Lower |
| Halstead Volume | 657.8627 | **651.1518 ✅** | 655.1318 | +6.7110 | Higher | +2.7309 | Higher | -3.9800 | Lower |
| Halstead Effort | 14353.3686 | 13589.2543 | **13102.6359 ✅** | +764.1142 | Higher | +1250.7327 | Higher | +486.6185 | Higher |
| SLOC | 39 | 34 | 34 | +5 | Higher | +5 | Higher | 0 | Same |
| PLOC | 28 | 28 | **27 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | 15 | 15 | **13 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Comments (CLOC) | **6 ✅** | 1 | 4 | +5 | Higher | +2 | Higher | -3 | Lower |
| Blank Lines | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00009

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.0654 ✅** | 73.1786 | 70.0042 | +0.8868 | Higher | +4.0612 | Higher | +3.1744 | Higher |
| MI (SEI) | 49.8951 | **65.9650 ✅** | 48.4692 | -16.0699 | Lower | +1.4259 | Higher | +17.4958 | Higher |
| Cyclomatic (sum) | 11 | 11 | 11 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 15 | 20 | **8 ✅** | -5 | Lower | +7 | Higher | +12 | Higher |
| Halstead Volume | 997.5770 | **927.9584 ✅** | 1105.4886 | +69.6185 | Higher | -107.9117 | Lower | -177.5302 | Lower |
| Halstead Effort | 26079.5117 | **25723.0071 ✅** | 34052.9985 | +356.5046 | Higher | -7973.4868 | Lower | -8329.9914 | Lower |
| SLOC | **37 ✅** | 40 | 46 | -3 | Lower | -9 | Lower | -6 | Lower |
| PLOC | 32 | **29 ✅** | 38 | +3 | Higher | -6 | Lower | -9 | Lower |
| LLOC | 21 | 21 | **20 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 2 | **10 ✅** | 4 | -8 | Lower | -2 | Lower | +6 | Higher |
| Blank Lines | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 2 | **1 ✅** | 3 | +1 | Higher | -1 | Lower | -2 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | **0 ✅** | 2 | +1 | Higher | -1 | Lower | -2 | Lower |

## p00010

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.5457 | **65.2251 ✅** | 61.9546 | -3.6795 | Lower | -0.4089 | Lower | +3.2706 | Higher |
| MI (SEI) | 14.3126 | **49.4027 ✅** | 38.9889 | -35.0901 | Lower | -24.6763 | Lower | +10.4138 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 12 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 10 | **4 ✅** | 11 | +6 | Higher | -1 | Lower | -7 | Lower |
| Halstead Volume | 2477.5974 | **2398.4014 ✅** | 2550 | +79.1960 | Higher | -72.4026 | Lower | -151.5986 | Lower |
| Halstead Effort | 114516.8555 | **102286.3678 ✅** | 115869.5122 | +12230.4877 | Higher | -1352.6567 | Lower | -13583.1444 | Lower |
| SLOC | 59 | **51 ✅** | 57 | +8 | Higher | +2 | Higher | -6 | Lower |
| PLOC | 51 | **34 ✅** | 48 | +17 | Higher | +3 | Higher | -14 | Lower |
| LLOC | 29 | **21 ✅** | 47 | +8 | Higher | -18 | Lower | -26 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 6 | -9 | Lower | -6 | Lower | +3 | Higher |
| Blank Lines | **8 ✅** | 11 | 9 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00011

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.1926 | **79.9974 ✅** | 74.4673 | -12.8048 | Lower | -7.2747 | Lower | +5.5301 | Higher |
| MI (SEI) | 49.7444 | **81.7992 ✅** | 59.3040 | -32.0548 | Lower | -9.5596 | Lower | +22.4953 | Higher |
| Cyclomatic (sum) | 11 | 10 | **6 ✅** | +1 | Higher | +5 | Higher | +4 | Higher |
| Cognitive (sum) | 8 | 7 | **3 ✅** | +1 | Higher | +5 | Higher | +4 | Higher |
| Halstead Volume | 1464.0554 | **578.2493 ✅** | 1151.9401 | +885.8061 | Higher | +312.1153 | Higher | -573.6907 | Lower |
| Halstead Effort | 50958.5732 | **14799.5686 ✅** | 28798.5018 | +36159.0046 | Higher | +22160.0714 | Higher | -13998.9332 | Lower |
| SLOC | 50 | **31 ✅** | 37 | +19 | Higher | +13 | Higher | -6 | Lower |
| PLOC | 36 | **18 ✅** | 26 | +18 | Higher | +10 | Higher | -8 | Lower |
| LLOC | 25 | **12 ✅** | 21 | +13 | Higher | +4 | Higher | -9 | Lower |
| Comments (CLOC) | 7 | **12 ✅** | 5 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 8 | **4 ✅** | 6 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00012

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.6320 | 69.1043 | **69.2044 ✅** | -1.4723 | Lower | -1.5724 | Lower | -0.1000 | Lower |
| MI (SEI) | 38.2924 | **41.2065 ✅** | 24.7509 | -2.9141 | Lower | +13.5415 | Higher | +16.4556 | Higher |
| Cyclomatic (sum) | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 6 | 6 | **4 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1947.2146 | 1851.7551 | **1847.3189 ✅** | +95.4596 | Higher | +99.8957 | Higher | +4.4362 | Higher |
| Halstead Effort | **61580.6627 ✅** | 66615.7010 | 69216.7296 | -5035.0383 | Lower | -7636.0669 | Lower | -2601.0286 | Lower |
| SLOC | 47 | **43 ✅** | 44 | +4 | Higher | +3 | Higher | -1 | Lower |
| PLOC | 38 | **34 ✅** | 37 | +4 | Higher | +1 | Higher | -3 | Lower |
| LLOC | 22 | **16 ✅** | 29 | +6 | Higher | -7 | Lower | -13 | Lower |
| Comments (CLOC) | 2 | 2 | 0 | 0 | Same | +2 | Higher | +2 | Higher |
| Blank Lines | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00013

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.5272 | 76.3748 | **83.2994 ✅** | -8.8475 | Lower | -15.7722 | Lower | -6.9247 | Lower |
| MI (SEI) | 48.2101 | 69.1687 | **87.2633 ✅** | -20.9586 | Lower | -39.0532 | Lower | -18.0945 | Lower |
| Cyclomatic (sum) | 10 | 8 | **5 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Cognitive (sum) | 22 | 15 | **5 ✅** | +7 | Higher | +17 | Higher | +10 | Higher |
| Halstead Volume | 2470.1152 | **620.1165 ✅** | 661.2389 | +1849.9987 | Higher | +1808.8763 | Higher | -41.1224 | Lower |
| Halstead Effort | 44335.4004 | **9733.1329 ✅** | 14282.7595 | +34602.2674 | Higher | +30052.6409 | Higher | -4549.6266 | Lower |
| SLOC | 42 | 39 | **26 ✅** | +3 | Higher | +16 | Higher | +13 | Higher |
| PLOC | 34 | 26 | **23 ✅** | +8 | Higher | +11 | Higher | +3 | Higher |
| LLOC | 15 | **13 ✅** | 21 | +2 | Higher | -6 | Lower | -8 | Lower |
| Comments (CLOC) | 5 | 9 | **11 ✅** | -4 | Lower | -6 | Lower | -2 | Lower |
| Blank Lines | 4 | 4 | **3 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00014

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 81.1815 | 72.1914 | **84.9648 ✅** | +8.9901 | Higher | -3.7833 | Lower | -12.7734 | Lower |
| MI (SEI) | 78.1017 | 62.3485 | **79.9702 ✅** | +15.7532 | Higher | -1.8685 | Lower | -17.6217 | Lower |
| Cyclomatic (sum) | 8 | 9 | **6 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Cognitive (sum) | 15 | 6 | **5 ✅** | +9 | Higher | +10 | Higher | +1 | Higher |
| Halstead Volume | **557.1854 ✅** | 1562.7156 | 589.3307 | -1005.5302 | Lower | -32.1453 | Lower | +973.3849 | Higher |
| Halstead Effort | 11612.9169 | 25935.7523 | **10944.7134 ✅** | -14322.8354 | Lower | +668.2036 | Higher | +14991.0390 | Higher |
| SLOC | 30 | 37 | **24 ✅** | -7 | Lower | +6 | Higher | +13 | Higher |
| PLOC | 24 | 28 | **19 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| LLOC | 14 | **8 ✅** | 13 | +6 | Higher | +1 | Higher | -5 | Lower |
| Comments (CLOC) | 8 | 8 | 5 | 0 | Same | +3 | Higher | +3 | Higher |
| Blank Lines | **2 ✅** | 4 | 5 | -2 | Lower | -3 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00016

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.8558 | 66.4971 | **73.9264 ✅** | +4.3587 | Higher | -3.0706 | Lower | -7.4293 | Lower |
| MI (SEI) | 56.5075 | 55.4167 | **62.3795 ✅** | +1.0908 | Higher | -5.8720 | Lower | -6.9628 | Lower |
| Cyclomatic (sum) | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 10 | 10 | **8 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1126.6269 | 2029.3683 | **950.0527 ✅** | -902.7415 | Lower | +176.5742 | Higher | +1079.3157 | Higher |
| Halstead Effort | 33197.9380 | 41328.4282 | **23542.3048 ✅** | -8130.4902 | Lower | +9655.6332 | Higher | +17786.1233 | Higher |
| SLOC | 44 | 47 | **39 ✅** | -3 | Lower | +5 | Higher | +8 | Higher |
| PLOC | 31 | 35 | **27 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| LLOC | 21 | 18 | 18 | +3 | Higher | +3 | Higher | 0 | Same |
| Comments (CLOC) | 7 | **11 ✅** | 7 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00017

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.5114 ✅** | 66.8819 | 73.6566 | +11.6295 | Higher | +4.8549 | Higher | -6.7747 | Lower |
| MI (SEI) | 38.6872 | **49.6923 ✅** | 48.8667 | -11.0051 | Lower | -10.1795 | Lower | +0.8256 | Higher |
| Cyclomatic (sum) | 11 | **10 ✅** | 11 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 9 | **8 ✅** | 13 | +1 | Higher | -4 | Lower | -5 | Lower |
| Halstead Volume | **815.4045 ✅** | 1844.7969 | 915.9339 | -1029.3924 | Lower | -100.5293 | Lower | +928.8630 | Higher |
| Halstead Effort | **16489.2915 ✅** | 37305.8926 | 21015.5933 | -20816.6011 | Lower | -4526.3018 | Lower | +16290.2993 | Higher |
| SLOC | **30 ✅** | 48 | 39 | -18 | Lower | -9 | Lower | +9 | Higher |
| PLOC | **25 ✅** | 34 | 31 | -9 | Lower | -6 | Lower | +3 | Higher |
| LLOC | **13 ✅** | 15 | 17 | -2 | Lower | -4 | Lower | -2 | Lower |
| Comments (CLOC) | 0 | **7 ✅** | 2 | -7 | Lower | -2 | Lower | +5 | Higher |
| Blank Lines | **5 ✅** | 7 | 6 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00018

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.2484 | 76.8629 | **78.9327 ✅** | -0.6145 | Lower | -2.6843 | Lower | -2.0698 | Lower |
| MI (SEI) | 66.0965 | 66.2441 | **67.2444 ✅** | -0.1476 | Lower | -1.1479 | Lower | -1.0003 | Lower |
| Cyclomatic (sum) | 7 | **6 ✅** | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 4 | **1 ✅** | 8 | +3 | Higher | -4 | Lower | -7 | Lower |
| Halstead Volume | 1229.9732 | 1731.5949 | **666.9203 ✅** | -501.6217 | Lower | +563.0529 | Higher | +1064.6746 | Higher |
| Halstead Effort | 16983.7393 | 24308.9281 | **14138.7109 ✅** | -7325.1888 | Lower | +2845.0285 | Higher | +10170.2172 | Higher |
| SLOC | 32 | **28 ✅** | 33 | +4 | Higher | -1 | Lower | -5 | Lower |
| PLOC | 23 | **18 ✅** | 24 | +5 | Higher | -1 | Lower | -6 | Lower |
| LLOC | 12 | **8 ✅** | 19 | +4 | Higher | -7 | Lower | -11 | Lower |
| Comments (CLOC) | **6 ✅** | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 5 | 5 | **4 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00019

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 85.9508 | 75.6649 | **90.3350 ✅** | +10.2859 | Higher | -4.3842 | Lower | -14.6701 | Lower |
| MI (SEI) | 86.0314 | 63.2115 | **86.2192 ✅** | +22.8199 | Higher | -0.1878 | Lower | -23.0077 | Lower |
| Cyclomatic (sum) | 6 | 6 | **3 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Halstead Volume | 487.5372 | 1587.7892 | **363.1964 ✅** | -1100.2520 | Lower | +124.3409 | Higher | +1224.5929 | Higher |
| Halstead Effort | 7429.1388 | 22247.0922 | **4443.8145 ✅** | -14817.9535 | Lower | +2985.3243 | Higher | +17803.2778 | Higher |
| SLOC | 24 | 31 | **21 ✅** | -7 | Lower | +3 | Higher | +10 | Higher |
| PLOC | 15 | 22 | **12 ✅** | -7 | Lower | +3 | Higher | +10 | Higher |
| LLOC | **9 ✅** | 10 | 10 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | **7 ✅** | 5 | 4 | +2 | Higher | +3 | Higher | +1 | Higher |
| Blank Lines | 5 | 5 | 5 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00020

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 77.8124 | 82.8264 | **87.8746 ✅** | -5.0140 | Lower | -10.0622 | Lower | -5.0482 | Lower |
| MI (SEI) | 73.8207 | 79.7131 | **84.6407 ✅** | -5.8923 | Lower | -10.8200 | Lower | -4.9276 | Lower |
| Cyclomatic (sum) | 6 | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Cognitive (sum) | 6 | 4 | 4 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 513.8337 | 418.6847 | **401.9096 ✅** | +95.1490 | Higher | +111.9242 | Higher | +16.7751 | Higher |
| Halstead Effort | 6896.1898 | **4710.2030 ✅** | 4822.9148 | +2185.9868 | Higher | +2073.2750 | Higher | -112.7118 | Lower |
| SLOC | 39 | 31 | **23 ✅** | +8 | Higher | +16 | Higher | +8 | Higher |
| PLOC | 21 | 19 | **14 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| LLOC | 13 | **8 ✅** | 9 | +5 | Higher | +4 | Higher | -1 | Lower |
| Comments (CLOC) | **11 ✅** | 8 | 5 | +3 | Higher | +6 | Higher | +3 | Higher |
| Blank Lines | 7 | **4 ✅** | 5 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 1 | **0 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |

## p00021

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.9798 | 72.1705 | **76.5095 ✅** | -7.1907 | Lower | -11.5297 | Lower | -4.3390 | Lower |
| MI (SEI) | 47.8983 | 60.4206 | **60.5238 ✅** | -12.5223 | Lower | -12.6255 | Lower | -0.1032 | Lower |
| Cyclomatic (sum) | 8 | 7 | **5 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 7 | 9 | **4 ✅** | -2 | Lower | +3 | Higher | +5 | Higher |
| Halstead Volume | 2558.5741 | 1154.9122 | **1057.9888 ✅** | +1403.6619 | Higher | +1500.5853 | Higher | +96.9234 | Higher |
| Halstead Effort | 47910.9979 | 25045.0965 | **20584.1062 ✅** | +22865.9014 | Higher | +27326.8918 | Higher | +4460.9903 | Higher |
| SLOC | 50 | 42 | **34 ✅** | +8 | Higher | +16 | Higher | +8 | Higher |
| PLOC | 34 | 28 | **24 ✅** | +6 | Higher | +10 | Higher | +4 | Higher |
| LLOC | 15 | 14 | **13 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Comments (CLOC) | 8 | 8 | 4 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 8 | 6 | 6 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00022

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.4607 ✅** | 65.3587 | 68.0761 | +4.1020 | Higher | +1.3846 | Higher | -2.7174 | Lower |
| MI (SEI) | 25.4261 | **51.4077 ✅** | 23.5304 | -25.9816 | Lower | +1.8957 | Higher | +27.8773 | Higher |
| Cyclomatic (sum) | **9 ✅** | 11 | 10 | -2 | Lower | -1 | Lower | +1 | Higher |
| Cognitive (sum) | 11 | **8 ✅** | 19 | +3 | Higher | -8 | Lower | -11 | Lower |
| Halstead Volume | 1435.8305 | 1463.4857 | **1375.0409 ✅** | -27.6552 | Lower | +60.7896 | Higher | +88.4448 | Higher |
| Halstead Effort | 62271.3452 | 76101.2584 | **56795.1694 ✅** | -13829.9132 | Lower | +5476.1758 | Higher | +19306.0890 | Higher |
| SLOC | **45 ✅** | 56 | 49 | -11 | Lower | -4 | Lower | +7 | Higher |
| PLOC | **35 ✅** | 40 | 40 | -5 | Lower | -5 | Lower | 0 | Same |
| LLOC | 29 | 31 | **28 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Comments (CLOC) | 0 | **11 ✅** | 0 | -11 | Lower | 0 | Same | +11 | Higher |
| Blank Lines | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00023

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.1635 | **67.7443 ✅** | 64.7568 | -0.5808 | Lower | +2.4068 | Higher | +2.9876 | Higher |
| MI (SEI) | 45.1162 | **57.0856 ✅** | 18.4361 | -11.9694 | Lower | +26.6801 | Higher | +38.6494 | Higher |
| Cyclomatic (sum) | 8 | 9 | **7 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Cognitive (sum) | 12 | **7 ✅** | 12 | +5 | Higher | 0 | Same | -5 | Lower |
| Halstead Volume | 1487.8148 | **1352.3986 ✅** | 1502.4089 | +135.4162 | Higher | -14.5941 | Lower | -150.0103 | Lower |
| Halstead Effort | 42479.6768 | **39132.3068 ✅** | 39293.7700 | +3347.3700 | Higher | +3185.9068 | Higher | -161.4632 | Lower |
| SLOC | 52 | **51 ✅** | 61 | +1 | Higher | -9 | Lower | -10 | Lower |
| PLOC | 42 | **34 ✅** | 52 | +8 | Higher | -10 | Lower | -18 | Lower |
| LLOC | 17 | 16 | 16 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | 5 | **12 ✅** | 0 | -7 | Lower | +5 | Higher | +12 | Higher |
| Blank Lines | 9 | 9 | 9 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00024

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 82.0578 | 73.7825 | **90.8541 ✅** | +8.2752 | Higher | -8.7964 | Lower | -17.0716 | Lower |
| MI (SEI) | 79.1622 | 62.7427 | **91.3628 ✅** | +16.4195 | Higher | -12.2006 | Lower | -28.6201 | Lower |
| Cyclomatic (sum) | 6 | 9 | **3 ✅** | -3 | Lower | +3 | Higher | +6 | Higher |
| Cognitive (sum) | 6 | 10 | **1 ✅** | -4 | Lower | +5 | Higher | +9 | Higher |
| Halstead Volume | 514.3250 | 1808.9224 | **448.9511 ✅** | -1294.5975 | Lower | +65.3739 | Higher | +1359.9714 | Higher |
| Halstead Effort | 7551.2260 | 22844.1063 | **5292.8969 ✅** | -15292.8803 | Lower | +2258.3291 | Higher | +17551.2094 | Higher |
| SLOC | 30 | 32 | **19 ✅** | -2 | Lower | +11 | Higher | +13 | Higher |
| PLOC | 20 | 23 | **14 ✅** | -3 | Lower | +6 | Higher | +9 | Higher |
| LLOC | 10 | 9 | 9 | +1 | Higher | +1 | Higher | 0 | Same |
| Comments (CLOC) | **8 ✅** | 6 | 5 | +2 | Higher | +3 | Higher | +1 | Higher |
| Blank Lines | **2 ✅** | 3 | 4 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00025

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.1353 | **66.6881 ✅** | 64.8885 | -7.5528 | Lower | -5.7532 | Lower | +1.7996 | Higher |
| MI (SEI) | 11.1407 | **51.6288 ✅** | 46.5456 | -40.4881 | Lower | -35.4049 | Lower | +5.0832 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 14 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | **15 ✅** | 17 | 16 | -2 | Lower | -1 | Lower | +1 | Higher |
| Halstead Volume | 3108.9006 | **1231.5029 ✅** | 1257.6344 | +1877.3977 | Higher | +1851.2663 | Higher | -26.1315 | Lower |
| Halstead Effort | 75057.7439 | 42882.6902 | **42704.8885 ✅** | +32175.0537 | Higher | +32352.8555 | Higher | +177.8017 | Higher |
| SLOC | 61 | **53 ✅** | 58 | +8 | Higher | +3 | Higher | -5 | Lower |
| PLOC | 48 | **41 ✅** | 48 | +7 | Higher | 0 | Same | -7 | Lower |
| LLOC | **19 ✅** | 28 | 32 | -9 | Lower | -13 | Lower | -4 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 8 | -9 | Lower | -8 | Lower | +1 | Higher |
| Blank Lines | 13 | 9 | **7 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00026

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 42.5976 | **68.7193 ✅** | 38.6917 | -26.1218 | Lower | +3.9058 | Higher | +30.0276 | Higher |
| MI (SEI) | -10.1728 | **62.1774 ✅** | -4.1176 | -72.3502 | Lower | -6.0552 | Lower | +66.2950 | Higher |
| Cyclomatic (sum) | 40 | **12 ✅** | 42 | +28 | Higher | -2 | Lower | -30 | Lower |
| Cognitive (sum) | 47 | **15 ✅** | 35 | +32 | Higher | +12 | Higher | -20 | Lower |
| Halstead Volume | 3626.1056 | **1044.3360 ✅** | 4136.2777 | +2581.7695 | Higher | -510.1722 | Lower | -3091.9417 | Lower |
| Halstead Effort | 335673.7712 | **33585.8468 ✅** | 508891.4166 | +302087.9244 | Higher | -173217.6455 | Lower | -475305.5699 | Lower |
| SLOC | 113 | **50 ✅** | 134 | +63 | Higher | -21 | Lower | -84 | Lower |
| PLOC | 103 | **32 ✅** | 123 | +71 | Higher | -20 | Lower | -91 | Lower |
| LLOC | 67 | **25 ✅** | 76 | +42 | Higher | -9 | Lower | -51 | Lower |
| Comments (CLOC) | 0 | **15 ✅** | 3 | -15 | Lower | -3 | Lower | +12 | Higher |
| Blank Lines | 10 | **9 ✅** | 11 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 7 | **1 ✅** | 8 | +6 | Higher | -1 | Lower | -7 | Lower |
| Closures | 0 | 0 | 3 | 0 | Same | -3 | Lower | -3 | Lower |
| Exits | 2 | **0 ✅** | 7 | +2 | Higher | -5 | Lower | -7 | Lower |

## p00027

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.2128 ✅** | 67.2294 | 67.8580 | +1.9833 | Higher | +1.3547 | Higher | -0.6286 | Lower |
| MI (SEI) | 41.5897 | **51.5598 ✅** | 49.9073 | -9.9701 | Lower | -8.3175 | Lower | +1.6525 | Higher |
| Cyclomatic (sum) | 12 | **7 ✅** | 14 | +5 | Higher | -2 | Lower | -7 | Lower |
| Cognitive (sum) | 7 | **4 ✅** | 6 | +3 | Higher | +1 | Higher | -2 | Lower |
| Halstead Volume | 1414.4418 | 1970.3806 | **1367.9512 ✅** | -555.9388 | Lower | +46.4906 | Higher | +602.4294 | Higher |
| Halstead Effort | 32670.5316 | **32147.1879 ✅** | 36064.1687 | +523.3437 | Higher | -3393.6372 | Lower | -3916.9809 | Lower |
| SLOC | **44 ✅** | 48 | 47 | -4 | Lower | -3 | Lower | +1 | Higher |
| PLOC | 35 | 33 | 33 | +2 | Higher | +2 | Higher | 0 | Same |
| LLOC | **15 ✅** | 18 | 18 | -3 | Lower | -3 | Lower | 0 | Same |
| Comments (CLOC) | 2 | **8 ✅** | 6 | -6 | Lower | -4 | Lower | +2 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |

## p00028

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 76.5988 | 78.2923 | **78.5634 ✅** | -1.6935 | Lower | -1.9646 | Lower | -0.2711 | Lower |
| MI (SEI) | 63.6622 | **68.9479 ✅** | 66.9153 | -5.2857 | Lower | -3.2531 | Lower | +2.0326 | Higher |
| Cyclomatic (sum) | 12 | 10 | **9 ✅** | +2 | Higher | +3 | Higher | +1 | Higher |
| Cognitive (sum) | 18 | 17 | **12 ✅** | +1 | Higher | +6 | Higher | +5 | Higher |
| Halstead Volume | 697.1726 | 660.5912 | **655.3899 ✅** | +36.5814 | Higher | +41.7827 | Higher | +5.2014 | Higher |
| Halstead Effort | 22154.5953 | **14831.8459 ✅** | 16212.2759 | +7322.7495 | Higher | +5942.3195 | Higher | -1380.4300 | Lower |
| SLOC | 35 | 33 | 33 | +2 | Higher | +2 | Higher | 0 | Same |
| PLOC | 26 | **23 ✅** | 25 | +3 | Higher | +1 | Higher | -2 | Lower |
| LLOC | 17 | **14 ✅** | 17 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 5 | **6 ✅** | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Blank Lines | **4 ✅** | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00029

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **77.5554 ✅** | 73.4990 | 63.3576 | +4.0564 | Higher | +14.1978 | Higher | +10.1414 | Higher |
| MI (SEI) | **70.0515 ✅** | 64.6074 | 40.3048 | +5.4441 | Higher | +29.7467 | Higher | +24.3026 | Higher |
| Cyclomatic (sum) | **5 ✅** | 6 | 14 | -1 | Lower | -9 | Lower | -8 | Lower |
| Cognitive (sum) | **5 ✅** | 6 | 31 | -1 | Lower | -26 | Lower | -25 | Lower |
| Halstead Volume | **724.1001 ✅** | 808.8476 | 1371.4571 | -84.7475 | Lower | -647.3570 | Lower | -562.6094 | Lower |
| Halstead Effort | **13654.4591 ✅** | 16036.2831 | 40229.4069 | -2381.8240 | Lower | -26574.9478 | Lower | -24193.1238 | Lower |
| SLOC | **36 ✅** | 44 | 62 | -8 | Lower | -26 | Lower | -18 | Lower |
| PLOC | **21 ✅** | 26 | 53 | -5 | Lower | -32 | Lower | -27 | Lower |
| LLOC | **17 ✅** | 22 | 46 | -5 | Lower | -29 | Lower | -24 | Lower |
| Comments (CLOC) | 8 | **10 ✅** | 6 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 7 | 8 | **5 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00030

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.1489 ✅** | 64.9057 | 65.9513 | +5.2432 | Higher | +4.1976 | Higher | -1.0456 | Lower |
| MI (SEI) | **52.2978 ✅** | 43.0942 | 31.2335 | +9.2036 | Higher | +21.0644 | Higher | +11.8607 | Higher |
| Cyclomatic (sum) | 13 | 16 | 13 | -3 | Lower | 0 | Same | +3 | Higher |
| Cognitive (sum) | 12 | 10 | 10 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | **1306.5674 ✅** | 1821.8321 | 1338.6924 | -515.2647 | Lower | -32.1250 | Lower | +483.1397 | Higher |
| Halstead Effort | 54805.8346 | 94670.2032 | **52131.7706 ✅** | -39864.3686 | Lower | +2674.0640 | Higher | +42538.4326 | Higher |
| SLOC | **42 ✅** | 50 | 54 | -8 | Lower | -12 | Lower | -4 | Lower |
| PLOC | **34 ✅** | 46 | 44 | -12 | Lower | -10 | Lower | +2 | Higher |
| LLOC | 25 | 29 | 25 | -4 | Lower | 0 | Same | +4 | Higher |
| Comments (CLOC) | 5 | 5 | 1 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 7 | **4 ✅** | 10 | +3 | Higher | -3 | Lower | -6 | Lower |
| Functions | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |

## p00031

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.8263 | 66.9959 | **77.4310 ✅** | +2.8304 | Higher | -7.6047 | Lower | -10.4351 | Lower |
| MI (SEI) | 57.4141 | 46.1751 | **73.9589 ✅** | +11.2390 | Higher | -16.5449 | Lower | -27.7838 | Lower |
| Cyclomatic (sum) | 10 | 10 | **7 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 26 | 20 | **16 ✅** | +6 | Higher | +10 | Higher | +4 | Higher |
| Halstead Volume | 1118.2129 | 1927.1401 | **811.1568 ✅** | -808.9271 | Lower | +307.0561 | Higher | +1115.9832 | Higher |
| Halstead Effort | 24712.5054 | 31860.6524 | **11724.9032 ✅** | -7148.1471 | Lower | +12987.6022 | Higher | +20135.7492 | Higher |
| SLOC | 47 | 47 | **34 ✅** | 0 | Same | +13 | Higher | +13 | Higher |
| PLOC | 34 | 38 | **26 ✅** | -4 | Lower | +8 | Higher | +12 | Higher |
| LLOC | 20 | **17 ✅** | 18 | +3 | Higher | +2 | Higher | -1 | Lower |
| Comments (CLOC) | 9 | 5 | **10 ✅** | +4 | Higher | -1 | Lower | -5 | Lower |
| Blank Lines | 6 | **5 ✅** | 6 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00032

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.6796 ✅** | 77.2228 | 71.4628 | +1.4568 | Higher | +7.2168 | Higher | +5.7600 | Higher |
| MI (SEI) | **65.0347 ✅** | 60.4567 | 58.1763 | +4.5780 | Higher | +6.8583 | Higher | +2.2804 | Higher |
| Cyclomatic (sum) | **8 ✅** | 12 | 12 | -4 | Lower | -4 | Lower | 0 | Same |
| Cognitive (sum) | 20 | 22 | **13 ✅** | -2 | Lower | +7 | Higher | +9 | Higher |
| Halstead Volume | **813.9397 ✅** | 999.5173 | 1060.7435 | -185.5776 | Lower | -246.8038 | Lower | -61.2263 | Lower |
| Halstead Effort | **20200.5038 ✅** | 32506.0397 | 32638.2627 | -12305.5359 | Lower | -12437.7589 | Lower | -132.2230 | Lower |
| SLOC | 31 | **30 ✅** | 42 | +1 | Higher | -11 | Lower | -12 | Lower |
| PLOC | 23 | 23 | 33 | 0 | Same | -10 | Lower | -10 | Lower |
| LLOC | 16 | 16 | 19 | 0 | Same | -3 | Lower | -3 | Lower |
| Comments (CLOC) | 4 | 3 | **7 ✅** | +1 | Higher | -3 | Lower | -4 | Lower |
| Blank Lines | 4 | 4 | 6 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00033

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **76.4684 ✅** | 76.0009 | 72.5155 | +0.4675 | Higher | +3.9528 | Higher | +3.4854 | Higher |
| MI (SEI) | 62.2927 | **66.9318 ✅** | 46.3145 | -4.6391 | Lower | +15.9783 | Higher | +20.6173 | Higher |
| Cyclomatic (sum) | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Cognitive (sum) | 10 | **9 ✅** | 12 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | 750.0573 | **696.4818 ✅** | 1034.0075 | +53.5755 | Higher | -283.9503 | Lower | -337.5258 | Lower |
| Halstead Effort | 11688.3926 | **11428.6325 ✅** | 25490.5336 | +259.7601 | Higher | -13802.1410 | Lower | -14061.9011 | Lower |
| SLOC | **37 ✅** | 39 | 42 | -2 | Lower | -5 | Lower | -3 | Lower |
| PLOC | 25 | 25 | 34 | 0 | Same | -9 | Lower | -9 | Lower |
| LLOC | 17 | **14 ✅** | 29 | +3 | Higher | -12 | Lower | -15 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 2 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | 7 | 6 | 6 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00034

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **74.6992 ✅** | 64.3969 | 61.8840 | +10.3024 | Higher | +12.8152 | Higher | +2.5129 | Higher |
| MI (SEI) | **65.4028 ✅** | 45.3698 | 38.8048 | +20.0330 | Higher | +26.5981 | Higher | +6.5651 | Higher |
| Cyclomatic (sum) | **7 ✅** | 10 | 12 | -3 | Lower | -5 | Lower | -2 | Lower |
| Cognitive (sum) | **8 ✅** | 10 | 16 | -2 | Lower | -8 | Lower | -6 | Lower |
| Halstead Volume | **969.9847 ✅** | 2463.0171 | 1562.1785 | -1493.0324 | Lower | -592.1937 | Lower | +900.8387 | Higher |
| Halstead Effort | **23710.7379 ✅** | 55527.8416 | 56863.2965 | -31817.1037 | Lower | -33152.5585 | Lower | -1335.4548 | Lower |
| SLOC | **38 ✅** | 51 | 67 | -13 | Lower | -29 | Lower | -16 | Lower |
| PLOC | **29 ✅** | 38 | 50 | -9 | Lower | -21 | Lower | -12 | Lower |
| LLOC | 22 | 22 | 42 | 0 | Same | -20 | Lower | -20 | Lower |
| Comments (CLOC) | **8 ✅** | 7 | 7 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 8 | 8 | 10 | 0 | Same | -2 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00035

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.0874 | 72.1217 | **75.6871 ✅** | +0.9657 | Higher | -2.5997 | Lower | -3.5654 | Lower |
| MI (SEI) | **48.4109 ✅** | 41.9303 | 34.3071 | +6.4806 | Higher | +14.1038 | Higher | +7.6232 | Higher |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 7 | 7 | **5 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Halstead Volume | 1497.3722 | 1802.9442 | **1310.8830 ✅** | -305.5720 | Lower | +186.4892 | Higher | +492.0612 | Higher |
| Halstead Effort | 46278.1585 | 61531.2478 | **42351.6036 ✅** | -15253.0893 | Lower | +3926.5549 | Higher | +19179.6442 | Higher |
| SLOC | 36 | 36 | **32 ✅** | 0 | Same | +4 | Higher | +4 | Higher |
| PLOC | 30 | 29 | **27 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| LLOC | 18 | **14 ✅** | 16 | +4 | Higher | +2 | Higher | -2 | Lower |
| Comments (CLOC) | **2 ✅** | 1 | 0 | +1 | Higher | +2 | Higher | +1 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |
| Exits | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |

## p00036

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **61.0343 ✅** | 59.1511 | 60.4015 | +1.8832 | Higher | +0.6328 | Higher | -1.2504 | Lower |
| MI (SEI) | 39.7251 | **41.4437 ✅** | 35.2830 | -1.7187 | Lower | +4.4421 | Higher | +6.1607 | Higher |
| Cyclomatic (sum) | 20 | 20 | **19 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 22 | **9 ✅** | 25 | +13 | Higher | -3 | Lower | -16 | Lower |
| Halstead Volume | **1233.0564 ✅** | 2756.4247 | 1329.8989 | -1523.3683 | Lower | -96.8425 | Lower | +1426.5258 | Higher |
| Halstead Effort | **35888.4310 ✅** | 55539.9016 | 44676.2920 | -19651.4706 | Lower | -8787.8609 | Lower | +10863.6096 | Higher |
| SLOC | 68 | **59 ✅** | 70 | +9 | Higher | -2 | Lower | -11 | Lower |
| PLOC | 54 | **43 ✅** | 59 | +11 | Higher | -5 | Lower | -16 | Lower |
| LLOC | 29 | **18 ✅** | 40 | +11 | Higher | -11 | Lower | -22 | Lower |
| Comments (CLOC) | 8 | **10 ✅** | 6 | -2 | Lower | +2 | Higher | +4 | Higher |
| Blank Lines | 10 | **8 ✅** | 9 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p00037

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.8311 | 63.3873 | **66.5144 ✅** | +1.4438 | Higher | -1.6832 | Lower | -3.1271 | Lower |
| MI (SEI) | 40.8756 | **51.4199 ✅** | 21.6846 | -10.5443 | Lower | +19.1910 | Higher | +29.7353 | Higher |
| Cyclomatic (sum) | 16 | **12 ✅** | 14 | +4 | Higher | +2 | Higher | -2 | Lower |
| Cognitive (sum) | 17 | **14 ✅** | 17 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | 1968.2061 | 2291.0298 | **1771.2980 ✅** | -322.8237 | Lower | +196.9080 | Higher | +519.7317 | Higher |
| Halstead Effort | 95986.0499 | **55268.1412 ✅** | 88466.4967 | +40717.9087 | Higher | +7519.5532 | Higher | -33198.3555 | Lower |
| SLOC | 49 | 54 | **47 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| PLOC | 39 | 40 | 39 | -1 | Lower | 0 | Same | +1 | Higher |
| LLOC | 37 | **24 ✅** | 36 | +13 | Higher | +1 | Higher | -12 | Lower |
| Comments (CLOC) | 4 | **13 ✅** | 0 | -9 | Lower | +4 | Higher | +13 | Higher |
| Blank Lines | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00038

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.1655 | **55.5207 ✅** | 54.7271 | -0.3551 | Lower | +0.4385 | Higher | +0.7936 | Higher |
| MI (SEI) | 35.7241 | **36.8328 ✅** | 35.3555 | -1.1087 | Lower | +0.3686 | Higher | +1.4773 | Higher |
| Cyclomatic (sum) | **24 ✅** | 25 | 25 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 31 | **17 ✅** | 47 | +14 | Higher | -16 | Lower | -30 | Lower |
| Halstead Volume | **2001.8094 ✅** | 2018.0723 | 2168.0524 | -16.2629 | Lower | -166.2430 | Lower | -149.9801 | Lower |
| Halstead Effort | 92699.1753 | **71298.0665 ✅** | 102982.4896 | +21401.1087 | Higher | -10283.3143 | Lower | -31684.4231 | Lower |
| SLOC | 79 | **76 ✅** | 78 | +3 | Higher | +1 | Higher | -2 | Lower |
| PLOC | 67 | **56 ✅** | 66 | +11 | Higher | +1 | Higher | -10 | Lower |
| LLOC | 43 | **30 ✅** | 43 | +13 | Higher | 0 | Same | -13 | Lower |
| Comments (CLOC) | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Blank Lines | 10 | 10 | **9 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |

## p00039

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.8678 | **70.3310 ✅** | 68.6120 | -1.4632 | Lower | +0.2558 | Higher | +1.7190 | Higher |
| MI (SEI) | 49.9065 | **50.5512 ✅** | 49.1525 | -0.6447 | Lower | +0.7539 | Higher | +1.3986 | Higher |
| Cyclomatic (sum) | 9 | **8 ✅** | 14 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | 10 | 10 | 13 | 0 | Same | -3 | Lower | -3 | Lower |
| Halstead Volume | 1089.6179 | **1038.2544 ✅** | 1265.3239 | +51.3635 | Higher | -175.7060 | Lower | -227.0695 | Lower |
| Halstead Effort | 22365.8416 | **20557.4376 ✅** | 35183.0339 | +1808.4040 | Higher | -12817.1922 | Lower | -14625.5963 | Lower |
| SLOC | 51 | 48 | **46 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| PLOC | 39 | **36 ✅** | 37 | +3 | Higher | +2 | Higher | -1 | Lower |
| LLOC | 19 | 19 | 33 | 0 | Same | -14 | Lower | -14 | Lower |
| Comments (CLOC) | **6 ✅** | 5 | 5 | +1 | Higher | +1 | Higher | 0 | Same |
| Blank Lines | 8 | 8 | **7 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00040

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.7623 | **64.0713 ✅** | 55.0458 | -8.3090 | Lower | +0.7165 | Higher | +9.0255 | Higher |
| MI (SEI) | 16.9732 | **54.3304 ✅** | 22.0212 | -37.3571 | Lower | -5.0479 | Lower | +32.3092 | Higher |
| Cyclomatic (sum) | 27 | **13 ✅** | 25 | +14 | Higher | +2 | Higher | -12 | Lower |
| Cognitive (sum) | 24 | **10 ✅** | 24 | +14 | Higher | 0 | Same | -14 | Lower |
| Halstead Volume | 2736.6190 | **2296.3143 ✅** | 2856.6753 | +440.3047 | Higher | -120.0563 | Lower | -560.3610 | Lower |
| Halstead Effort | 134583.0108 | **47052.7793 ✅** | 147248.6256 | +87530.2315 | Higher | -12665.6147 | Lower | -100195.8463 | Lower |
| SLOC | 66 | **51 ✅** | 70 | +15 | Higher | -4 | Lower | -19 | Lower |
| PLOC | 58 | **37 ✅** | 58 | +21 | Higher | 0 | Same | -21 | Lower |
| LLOC | 23 | **13 ✅** | 25 | +10 | Higher | -2 | Lower | -12 | Lower |
| Comments (CLOC) | 1 | **14 ✅** | 3 | -13 | Lower | -2 | Lower | +11 | Higher |
| Blank Lines | 8 | **7 ✅** | 9 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 6 | **2 ✅** | 6 | +4 | Higher | 0 | Same | -4 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 6 | **3 ✅** | 6 | +3 | Higher | 0 | Same | -3 | Lower |

## p00042

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.3592 | **66.3790 ✅** | 54.6281 | -11.0198 | Lower | +0.7311 | Higher | +11.7509 | Higher |
| MI (SEI) | 14.7799 | **39.4298 ✅** | 5.1472 | -24.6500 | Lower | +9.6326 | Higher | +34.2826 | Higher |
| Cyclomatic (sum) | 18 | **10 ✅** | 20 | +8 | Higher | -2 | Lower | -10 | Lower |
| Cognitive (sum) | 41 | **11 ✅** | 31 | +30 | Higher | +10 | Higher | -20 | Lower |
| Halstead Volume | 2723.9596 | **1682.3757 ✅** | 2989.0065 | +1041.5840 | Higher | -265.0469 | Lower | -1306.6309 | Lower |
| Halstead Effort | 193656.5054 | **64491.0676 ✅** | 209828.2597 | +129165.4378 | Higher | -16171.7542 | Lower | -145337.1921 | Lower |
| SLOC | 77 | **51 ✅** | 76 | +26 | Higher | +1 | Higher | -25 | Lower |
| PLOC | 66 | **40 ✅** | 64 | +26 | Higher | +2 | Higher | -24 | Lower |
| LLOC | 57 | **31 ✅** | 56 | +26 | Higher | +1 | Higher | -25 | Lower |
| Comments (CLOC) | 1 | **3 ✅** | 0 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 11 | **10 ✅** | 12 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | **0 ✅** | 1 | 1 | -1 | Lower | -1 | Lower | 0 | Same |

## p00043

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 40.5386 | **64.4359 ✅** | 39.4227 | -23.8973 | Lower | +1.1158 | Higher | +25.0131 | Higher |
| MI (SEI) | -13.3469 | **47.1267 ✅** | -14.8549 | -60.4736 | Lower | +1.5080 | Higher | +61.9816 | Higher |
| Cyclomatic (sum) | 38 | **14 ✅** | 39 | +24 | Higher | -1 | Lower | -25 | Lower |
| Cognitive (sum) | 36 | **11 ✅** | 39 | +25 | Higher | -3 | Lower | -28 | Lower |
| Halstead Volume | 3542.7248 | **1300.8475 ✅** | 3744.4458 | +2241.8773 | Higher | -201.7210 | Lower | -2443.5983 | Lower |
| Halstead Effort | 221936.9461 | **42841.2444 ✅** | 262589.2179 | +179095.7017 | Higher | -40652.2717 | Lower | -219747.9734 | Lower |
| SLOC | 133 | **59 ✅** | 138 | +74 | Higher | -5 | Lower | -79 | Lower |
| PLOC | 120 | **41 ✅** | 125 | +79 | Higher | -5 | Lower | -84 | Lower |
| LLOC | 68 | **22 ✅** | 73 | +46 | Higher | -5 | Lower | -51 | Lower |
| Comments (CLOC) | 0 | **9 ✅** | 0 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 13 | **9 ✅** | 13 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 10 | **3 ✅** | 10 | +7 | Higher | 0 | Same | -7 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 11 | **4 ✅** | 11 | +7 | Higher | 0 | Same | -7 | Lower |

## p00044

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.3454 | 54.7532 | **69.3404 ✅** | +11.5923 | Higher | -2.9949 | Lower | -14.5872 | Lower |
| MI (SEI) | **42.7652 ✅** | 33.8199 | 40.9035 | +8.9453 | Higher | +1.8617 | Higher | -7.0836 | Lower |
| Cyclomatic (sum) | 17 | 22 | **10 ✅** | -5 | Lower | +7 | Higher | +12 | Higher |
| Cognitive (sum) | 34 | 25 | **17 ✅** | +9 | Higher | +17 | Higher | +8 | Higher |
| Halstead Volume | 1242.4056 | 3027.9704 | **1149.7999 ✅** | -1785.5648 | Lower | +92.6057 | Higher | +1878.1705 | Higher |
| Halstead Effort | 52114.4766 | 78424.4337 | **46830.3914 ✅** | -26309.9571 | Lower | +5284.0852 | Higher | +31594.0423 | Higher |
| SLOC | 51 | 73 | **48 ✅** | -22 | Lower | +3 | Higher | +25 | Higher |
| PLOC | 46 | 55 | **41 ✅** | -9 | Lower | +5 | Higher | +14 | Higher |
| LLOC | 27 | 27 | 38 | 0 | Same | -11 | Lower | -11 | Lower |
| Comments (CLOC) | 4 | **11 ✅** | 2 | -7 | Lower | +2 | Higher | +9 | Higher |
| Blank Lines | **3 ✅** | 13 | 5 | -10 | Lower | -2 | Lower | +8 | Higher |
| Functions | 2 | 3 | **1 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00045

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **78.1966 ✅** | 75.2458 | 72.4424 | +2.9507 | Higher | +5.7542 | Higher | +2.8034 | Higher |
| MI (SEI) | 72.2702 | **73.9561 ✅** | 61.0165 | -1.6859 | Lower | +11.2537 | Higher | +12.9396 | Higher |
| Cyclomatic (sum) | **6 ✅** | 7 | 9 | -1 | Lower | -3 | Lower | -2 | Lower |
| Cognitive (sum) | **6 ✅** | 12 | 9 | -6 | Lower | -3 | Lower | +3 | Higher |
| Halstead Volume | **803.0872 ✅** | 873.2046 | 1003.2774 | -70.1175 | Lower | -200.1902 | Lower | -130.0727 | Lower |
| Halstead Effort | **18397.9973 ✅** | 20247.4326 | 25368.5854 | -1849.4353 | Lower | -6970.5881 | Lower | -5121.1528 | Lower |
| SLOC | **33 ✅** | 38 | 42 | -5 | Lower | -9 | Lower | -4 | Lower |
| PLOC | **23 ✅** | 27 | 32 | -4 | Lower | -9 | Lower | -5 | Lower |
| LLOC | 19 | 19 | 20 | 0 | Same | -1 | Lower | -1 | Lower |
| Comments (CLOC) | 8 | **14 ✅** | 8 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 6 | **5 ✅** | 9 | +1 | Higher | -3 | Lower | -4 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00046

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.4200 | 78.4683 | **81.5747 ✅** | -5.0482 | Lower | -8.1546 | Lower | -3.1064 | Lower |
| MI (SEI) | 61.3084 | **72.8658 ✅** | 56.8856 | -11.5575 | Lower | +4.4228 | Higher | +15.9802 | Higher |
| Cyclomatic (sum) | 12 | 8 | **7 ✅** | +4 | Higher | +5 | Higher | +1 | Higher |
| Cognitive (sum) | 36 | **7 ✅** | 10 | +29 | Higher | +26 | Higher | -3 | Lower |
| Halstead Volume | 784.7803 | 697.6731 | **600.1261 ✅** | +87.1072 | Higher | +184.6542 | Higher | +97.5470 | Higher |
| Halstead Effort | 20011.8977 | 13255.7888 | **10865.4412 ✅** | +6756.1089 | Higher | +9146.4565 | Higher | +2390.3476 | Higher |
| SLOC | 41 | 33 | **29 ✅** | +8 | Higher | +12 | Higher | +4 | Higher |
| PLOC | 31 | **19 ✅** | 24 | +12 | Higher | +7 | Higher | -5 | Lower |
| LLOC | 21 | **12 ✅** | 21 | +9 | Higher | 0 | Same | -9 | Lower |
| Comments (CLOC) | 7 | **8 ✅** | 1 | -1 | Lower | +6 | Higher | +7 | Higher |
| Blank Lines | **3 ✅** | 6 | 4 | -3 | Lower | -1 | Lower | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00047

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 68.3724 | **79.5578 ✅** | 71.1623 | -11.1854 | Lower | -2.7900 | Lower | +8.3955 | Higher |
| MI (SEI) | 54.9129 | **72.2803 ✅** | 57.3570 | -17.3674 | Lower | -2.4441 | Lower | +14.9233 | Higher |
| Cyclomatic (sum) | 14 | **8 ✅** | 14 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 10 | 10 | 12 | 0 | Same | -2 | Lower | -2 | Lower |
| Halstead Volume | 1021.8772 | **846.2119 ✅** | 889.9074 | +175.6653 | Higher | +131.9698 | Higher | -43.6955 | Lower |
| Halstead Effort | 26228.1813 | **12814.0659 ✅** | 17455.8763 | +13414.1153 | Higher | +8772.3049 | Higher | -4641.8104 | Lower |
| SLOC | 50 | **29 ✅** | 44 | +21 | Higher | +6 | Higher | -15 | Lower |
| PLOC | 35 | **20 ✅** | 33 | +15 | Higher | +2 | Higher | -13 | Lower |
| LLOC | 17 | **13 ✅** | 15 | +4 | Higher | +2 | Higher | -2 | Lower |
| Comments (CLOC) | **9 ✅** | 6 | 7 | +3 | Higher | +2 | Higher | -1 | Lower |
| Blank Lines | 9 | **4 ✅** | 7 | +5 | Higher | +2 | Higher | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00048

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.7966 | **79.6128 ✅** | 75.0944 | -6.8162 | Lower | -2.2978 | Lower | +4.5184 | Higher |
| MI (SEI) | 50.6820 | **75.2805 ✅** | 34.0630 | -24.5985 | Lower | +16.6190 | Higher | +41.2175 | Higher |
| Cyclomatic (sum) | 27 | **7 ✅** | 14 | +20 | Higher | +13 | Higher | -7 | Lower |
| Cognitive (sum) | 10 | **4 ✅** | 16 | +6 | Higher | -6 | Lower | -12 | Lower |
| Halstead Volume | 896.1108 | **710.9874 ✅** | 716.7697 | +185.1234 | Higher | +179.3410 | Higher | -5.7824 | Lower |
| Halstead Effort | 13429.8704 | 11096.4815 | **8140.4563 ✅** | +2333.3889 | Higher | +5289.4141 | Higher | +2956.0252 | Higher |
| SLOC | 33 | **31 ✅** | 37 | +2 | Higher | -4 | Lower | -6 | Lower |
| PLOC | 29 | **20 ✅** | 34 | +9 | Higher | -5 | Lower | -14 | Lower |
| LLOC | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Comments (CLOC) | 2 | **8 ✅** | 0 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | **2 ✅** | 4 | 3 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00049

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **63.3378 ✅** | 61.4993 | 55.8739 | +1.8386 | Higher | +7.4639 | Higher | +5.6253 | Higher |
| MI (SEI) | **47.9787 ✅** | 46.8382 | 35.2268 | +1.1405 | Higher | +12.7519 | Higher | +11.6114 | Higher |
| Cyclomatic (sum) | 17 | **14 ✅** | 22 | +3 | Higher | -5 | Lower | -8 | Lower |
| Cognitive (sum) | 13 | **11 ✅** | 29 | +2 | Higher | -16 | Lower | -18 | Lower |
| Halstead Volume | **1268.2438 ✅** | 2413.3309 | 1765.4359 | -1145.0871 | Lower | -497.1921 | Lower | +647.8950 | Higher |
| Halstead Effort | **31238.8476 ✅** | 47404.7143 | 85682.4888 | -16165.8666 | Lower | -54443.6412 | Lower | -38277.7746 | Lower |
| SLOC | 61 | **58 ✅** | 81 | +3 | Higher | -20 | Lower | -23 | Lower |
| PLOC | 41 | **38 ✅** | 57 | +3 | Higher | -16 | Lower | -19 | Lower |
| LLOC | 20 | **19 ✅** | 45 | +1 | Higher | -25 | Lower | -26 | Lower |
| Comments (CLOC) | 11 | 12 | 12 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | 11 | **10 ✅** | 13 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00050

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 64.3450 | **73.3414 ✅** | 67.2298 | -8.9964 | Lower | -2.8849 | Lower | +6.1115 | Higher |
| MI (SEI) | 47.4460 | **67.0024 ✅** | 51.0426 | -19.5564 | Lower | -3.5967 | Lower | +15.9597 | Higher |
| Cyclomatic (sum) | 12 | 7 | 7 | +5 | Higher | +5 | Higher | 0 | Same |
| Cognitive (sum) | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Halstead Volume | 1701.5550 | **797.6609 ✅** | 1734.9453 | +903.8941 | Higher | -33.3903 | Lower | -937.2844 | Lower |
| Halstead Effort | 74017.6422 | **18380.8820 ✅** | 81362.9540 | +55636.7601 | Higher | -7345.3118 | Lower | -62982.0719 | Lower |
| SLOC | 56 | **44 ✅** | 50 | +12 | Higher | +6 | Higher | -6 | Lower |
| PLOC | 47 | **28 ✅** | 39 | +19 | Higher | +8 | Higher | -11 | Lower |
| LLOC | 40 | **20 ✅** | 49 | +20 | Higher | -9 | Lower | -29 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 8 | -3 | Lower | +1 | Higher | +4 | Higher |
| Blank Lines | 6 | 7 | **4 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00051

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.9988 | **75.9100 ✅** | 68.6571 | -1.9112 | Lower | +5.3418 | Higher | +7.2529 | Higher |
| MI (SEI) | 61.8504 | **71.3380 ✅** | 47.9972 | -9.4876 | Lower | +13.8533 | Higher | +23.3408 | Higher |
| Cyclomatic (sum) | 6 | **5 ✅** | 11 | +1 | Higher | -5 | Lower | -6 | Lower |
| Cognitive (sum) | **3 ✅** | 4 | 22 | -1 | Lower | -19 | Lower | -18 | Lower |
| Halstead Volume | 988.7168 | **839.5814 ✅** | 1104.7124 | +149.1354 | Higher | -115.9956 | Lower | -265.1310 | Lower |
| Halstead Effort | 23926.9469 | **15162.8404 ✅** | 41426.7147 | +8764.1065 | Higher | -17499.7677 | Lower | -26263.8743 | Lower |
| SLOC | 40 | **38 ✅** | 50 | +2 | Higher | -10 | Lower | -12 | Lower |
| PLOC | 24 | **20 ✅** | 38 | +4 | Higher | -14 | Lower | -18 | Lower |
| LLOC | 16 | **15 ✅** | 32 | +1 | Higher | -16 | Lower | -17 | Lower |
| Comments (CLOC) | 7 | **11 ✅** | 5 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 9 | 9 | **7 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00052

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.2175 | 69.3772 | **83.3269 ✅** | -4.1597 | Lower | -18.1094 | Lower | -13.9496 | Lower |
| MI (SEI) | 30.8299 | 45.4212 | **59.9144 ✅** | -14.5914 | Lower | -29.0845 | Lower | -14.4931 | Lower |
| Cyclomatic (sum) | 11 | 9 | **7 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Cognitive (sum) | 9 | **6 ✅** | 12 | +3 | Higher | -3 | Lower | -6 | Lower |
| Halstead Volume | 2775.4367 | 1808.9224 | **535.2886 ✅** | +966.5142 | Higher | +2240.1481 | Higher | +1273.6338 | Higher |
| Halstead Effort | 62864.6843 | 34500.0673 | **12232.9190 ✅** | +28364.6170 | Higher | +50631.7653 | Higher | +22267.1482 | Higher |
| SLOC | 46 | 42 | **27 ✅** | +4 | Higher | +19 | Higher | +15 | Higher |
| PLOC | 36 | 33 | **24 ✅** | +3 | Higher | +12 | Higher | +9 | Higher |
| LLOC | 18 | **16 ✅** | 18 | +2 | Higher | 0 | Same | -2 | Lower |
| Comments (CLOC) | 1 | **3 ✅** | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 10 | 8 | **3 ✅** | +2 | Higher | +7 | Higher | +5 | Higher |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00053

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.5284 | **69.7122 ✅** | 64.2209 | -3.1837 | Lower | +2.3075 | Higher | +5.4913 | Higher |
| MI (SEI) | 49.8401 | 48.1339 | **51.8464 ✅** | +1.7062 | Higher | -2.0063 | Lower | -3.7125 | Lower |
| Cyclomatic (sum) | **11 ✅** | 14 | 13 | -3 | Lower | -2 | Lower | +1 | Higher |
| Cognitive (sum) | 30 | **10 ✅** | 21 | +20 | Higher | +9 | Higher | -11 | Lower |
| Halstead Volume | 993.3187 | **957.6739 ✅** | 1214.1937 | +35.6448 | Higher | -220.8750 | Lower | -256.5198 | Lower |
| Halstead Effort | 26654.0515 | **26602.0520 ✅** | 39548.0221 | +51.9995 | Higher | -12893.9706 | Lower | -12945.9700 | Lower |
| SLOC | 59 | **47 ✅** | 62 | +12 | Higher | -3 | Lower | -15 | Lower |
| PLOC | 42 | **40 ✅** | 53 | +2 | Higher | -11 | Lower | -13 | Lower |
| LLOC | 35 | **23 ✅** | 46 | +12 | Higher | -11 | Lower | -23 | Lower |
| Comments (CLOC) | 9 | 4 | **14 ✅** | +5 | Higher | -5 | Lower | -10 | Lower |
| Blank Lines | 11 | **7 ✅** | 9 | +4 | Higher | +2 | Higher | -2 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

## p00054

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.9015 | 66.2859 | **75.2909 ✅** | +1.6156 | Higher | -7.3894 | Lower | -9.0050 | Lower |
| MI (SEI) | 49.5846 | **57.1830 ✅** | 54.7172 | -7.5984 | Lower | -5.1326 | Lower | +2.4658 | Higher |
| Cyclomatic (sum) | 15 | 13 | **7 ✅** | +2 | Higher | +8 | Higher | +6 | Higher |
| Cognitive (sum) | 27 | **7 ✅** | 10 | +20 | Higher | +17 | Higher | -3 | Lower |
| Halstead Volume | 1139.8562 | 1120.8195 | **865.6692 ✅** | +19.0367 | Higher | +274.1870 | Higher | +255.1503 | Higher |
| Halstead Effort | 38966.1943 | 32870.9305 | **27111.1850 ✅** | +6095.2638 | Higher | +11855.0093 | Higher | +5759.7455 | Higher |
| SLOC | 49 | 56 | **38 ✅** | -7 | Lower | +11 | Higher | +18 | Higher |
| PLOC | 43 | 38 | **27 ✅** | +5 | Higher | +16 | Higher | +11 | Higher |
| LLOC | 26 | **23 ✅** | 24 | +3 | Higher | +2 | Higher | -1 | Lower |
| Comments (CLOC) | 6 | **15 ✅** | 3 | -9 | Lower | +3 | Higher | +12 | Higher |
| Blank Lines | **5 ✅** | 8 | 8 | -3 | Lower | -3 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 6 | 0 | -6 | Lower | 0 | Same | +6 | Higher |

## p00056

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.6539 | **65.4206 ✅** | 65.1799 | -2.7667 | Lower | -2.5261 | Lower | +0.2406 | Higher |
| MI (SEI) | 41.8642 | 38.4264 | **47.9178 ✅** | +3.4377 | Higher | -6.0536 | Lower | -9.4913 | Lower |
| Cyclomatic (sum) | 19 | 17 | 17 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 29 | **19 ✅** | 40 | +10 | Higher | -11 | Lower | -21 | Lower |
| Halstead Volume | 1394.0262 | 1316.6237 | **1228.6918 ✅** | +77.4025 | Higher | +165.3344 | Higher | +87.9319 | Higher |
| Halstead Effort | 56075.8273 | **43568.2748 ✅** | 44185.6456 | +12507.5525 | Higher | +11890.1817 | Higher | -617.3708 | Lower |
| SLOC | 60 | **53 ✅** | 55 | +7 | Higher | +5 | Higher | -2 | Lower |
| PLOC | 51 | **47 ✅** | 48 | +4 | Higher | +3 | Higher | -1 | Lower |
| LLOC | 32 | **26 ✅** | 33 | +6 | Higher | -1 | Lower | -7 | Lower |
| Comments (CLOC) | 7 | 3 | **8 ✅** | +4 | Higher | -1 | Lower | -5 | Lower |
| Blank Lines | 8 | 6 | 6 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 3 | 4 | **2 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 5 | **0 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |

## p00057

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 86.7188 | 72.5672 | **90.0962 ✅** | +14.1516 | Higher | -3.3774 | Lower | -17.5290 | Lower |
| MI (SEI) | 79.5752 | 60.4518 | **86.7251 ✅** | +19.1233 | Higher | -7.1499 | Lower | -26.2732 | Lower |
| Cyclomatic (sum) | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Halstead Volume | 420.6012 | 1527.7003 | **405.2134 ✅** | -1107.0990 | Lower | +15.3878 | Higher | +1122.4869 | Higher |
| Halstead Effort | **6012.1231 ✅** | 23097.3729 | 6221.2169 | -17085.2498 | Lower | -209.0937 | Lower | +16876.1561 | Higher |
| SLOC | 24 | 38 | **20 ✅** | -14 | Lower | +4 | Higher | +18 | Higher |
| PLOC | 18 | 28 | **15 ✅** | -10 | Lower | +3 | Higher | +13 | Higher |
| LLOC | **8 ✅** | 11 | 10 | -3 | Lower | -2 | Lower | +1 | Higher |
| Comments (CLOC) | 4 | **7 ✅** | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00058

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.8400 | 72.9670 | **80.9154 ✅** | +5.8731 | Higher | -2.0754 | Lower | -7.9485 | Lower |
| MI (SEI) | 65.0625 | 54.0826 | **74.5079 ✅** | +10.9798 | Higher | -9.4454 | Lower | -20.4252 | Lower |
| Cyclomatic (sum) | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 7 | 7 | 8 | 0 | Same | -1 | Lower | -1 | Lower |
| Halstead Volume | 862.2087 | 1248.2385 | **794.3062 ✅** | -386.0298 | Lower | +67.9024 | Higher | +453.9323 | Higher |
| Halstead Effort | **15853.5139 ✅** | 30217.7728 | 17125.2418 | -14364.2589 | Lower | -1271.7279 | Lower | +13092.5310 | Higher |
| SLOC | 31 | 39 | **28 ✅** | -8 | Lower | +3 | Higher | +11 | Higher |
| PLOC | 25 | 29 | **21 ✅** | -4 | Lower | +4 | Higher | +8 | Higher |
| LLOC | **10 ✅** | 14 | 14 | -4 | Lower | -4 | Lower | 0 | Same |
| Comments (CLOC) | 4 | 4 | **6 ✅** | 0 | Same | -2 | Lower | -2 | Lower |
| Blank Lines | **4 ✅** | 6 | 5 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00059

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.1862 | 69.0897 | **73.3783 ✅** | -3.9035 | Lower | -8.1921 | Lower | -4.2886 | Lower |
| MI (SEI) | 36.4702 | **58.1507 ✅** | 31.3834 | -21.6805 | Lower | +5.0868 | Higher | +26.7673 | Higher |
| Cyclomatic (sum) | 17 | **11 ✅** | 12 | +6 | Higher | +5 | Higher | -1 | Lower |
| Cognitive (sum) | 14 | **12 ✅** | 14 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | 2642.0177 | 1318.0457 | **1186.3098 ✅** | +1323.9720 | Higher | +1455.7079 | Higher | +131.7359 | Higher |
| Halstead Effort | 60691.8347 | **32167.4402 ✅** | 45079.7737 | +28524.3945 | Higher | +15612.0611 | Higher | -12912.3334 | Lower |
| SLOC | 43 | 46 | **36 ✅** | -3 | Lower | +7 | Higher | +10 | Higher |
| PLOC | 31 | **30 ✅** | 31 | +1 | Higher | 0 | Same | -1 | Lower |
| LLOC | 13 | 14 | **12 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Comments (CLOC) | 2 | **10 ✅** | 0 | -8 | Lower | +2 | Higher | +10 | Higher |
| Blank Lines | 11 | 8 | **5 ✅** | +3 | Higher | +6 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00060

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.5348 | **72.5916 ✅** | 69.1167 | -2.0568 | Lower | +1.4180 | Higher | +3.4749 | Higher |
| MI (SEI) | 56.5534 | **61.9576 ✅** | 56.1309 | -5.4042 | Lower | +0.4225 | Higher | +5.8267 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 17 | +2 | Higher | -2 | Lower | -4 | Lower |
| Cognitive (sum) | 20 | **10 ✅** | 21 | +10 | Higher | -1 | Lower | -11 | Lower |
| Halstead Volume | 960.6136 | **880.4978 ✅** | 1076.8490 | +80.1158 | Higher | -116.2355 | Lower | -196.3513 | Lower |
| Halstead Effort | 25187.2873 | **22283.3671 ✅** | 35256.8354 | +2903.9202 | Higher | -10069.5481 | Lower | -12973.4683 | Lower |
| SLOC | 44 | **41 ✅** | 45 | +3 | Higher | -1 | Lower | -4 | Lower |
| PLOC | 34 | **29 ✅** | 38 | +5 | Higher | -4 | Lower | -9 | Lower |
| LLOC | 17 | **16 ✅** | 21 | +1 | Higher | -4 | Lower | -5 | Lower |
| Comments (CLOC) | 7 | 8 | 8 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00061

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.8888 | **67.0048 ✅** | 62.1391 | -9.1160 | Lower | -4.2503 | Lower | +4.8657 | Higher |
| MI (SEI) | 39.1304 | **52.6344 ✅** | 40.0599 | -13.5040 | Lower | -0.9295 | Lower | +12.5745 | Higher |
| Cyclomatic (sum) | 19 | **11 ✅** | 14 | +8 | Higher | +5 | Higher | -3 | Lower |
| Cognitive (sum) | 23 | **8 ✅** | 22 | +15 | Higher | +1 | Higher | -14 | Lower |
| Halstead Volume | 1891.8954 | **1517.8989 ✅** | 1649.2888 | +373.9964 | Higher | +242.6066 | Higher | -131.3899 | Lower |
| Halstead Effort | 74657.1015 | **42550.1343 ✅** | 67833.6522 | +32106.9672 | Higher | +6823.4493 | Higher | -25283.5179 | Lower |
| SLOC | 73 | **50 ✅** | 63 | +23 | Higher | +10 | Higher | -13 | Lower |
| PLOC | 56 | **36 ✅** | 54 | +20 | Higher | +2 | Higher | -18 | Lower |
| LLOC | 39 | **28 ✅** | 42 | +11 | Higher | -3 | Lower | -14 | Lower |
| Comments (CLOC) | **12 ✅** | 9 | 7 | +3 | Higher | +5 | Higher | +2 | Higher |
| Blank Lines | 13 | 9 | **5 ✅** | +4 | Higher | +8 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00062

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **79.5353 ✅** | 72.3084 | 74.7516 | +7.2268 | Higher | +4.7836 | Higher | -2.4432 | Lower |
| MI (SEI) | 63.0334 | **65.7158 ✅** | 50.4580 | -2.6824 | Lower | +12.5754 | Higher | +15.2578 | Higher |
| Cyclomatic (sum) | **8 ✅** | 9 | 9 | -1 | Lower | -1 | Lower | 0 | Same |
| Cognitive (sum) | 9 | **7 ✅** | 13 | +2 | Higher | -4 | Lower | -6 | Lower |
| Halstead Volume | **690.4452 ✅** | 890.5708 | 878.9685 | -200.1255 | Lower | -188.5232 | Lower | +11.6023 | Higher |
| Halstead Effort | **14873.3413 ✅** | 25007.2275 | 23439.1597 | -10133.8862 | Lower | -8565.8184 | Lower | +1568.0677 | Higher |
| SLOC | **31 ✅** | 44 | 38 | -13 | Lower | -7 | Lower | +6 | Higher |
| PLOC | **23 ✅** | 27 | 31 | -4 | Lower | -8 | Lower | -4 | Lower |
| LLOC | **14 ✅** | 16 | 24 | -2 | Lower | -10 | Lower | -8 | Lower |
| Comments (CLOC) | 3 | **12 ✅** | 2 | -9 | Lower | +1 | Higher | +10 | Higher |
| Blank Lines | **6 ✅** | 8 | 7 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00063

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 74.1710 | 74.2941 | **77.5854 ✅** | -0.1231 | Lower | -3.4144 | Lower | -3.2913 | Lower |
| MI (SEI) | 49.2996 | **62.6837 ✅** | 58.8522 | -13.3840 | Lower | -9.5526 | Lower | +3.8315 | Higher |
| Cyclomatic (sum) | 10 | 10 | **7 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Cognitive (sum) | 13 | 12 | **6 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |
| Halstead Volume | 801.4150 | 782.6647 | **719.4328 ✅** | +18.7503 | Higher | +81.9822 | Higher | +63.2319 | Higher |
| Halstead Effort | 18653.6245 | 18783.9528 | **16171.5972 ✅** | -130.3283 | Lower | +2482.0274 | Higher | +2612.3556 | Higher |
| SLOC | 40 | 40 | **35 ✅** | 0 | Same | +5 | Higher | +5 | Higher |
| PLOC | 33 | 33 | **25 ✅** | 0 | Same | +8 | Higher | +8 | Higher |
| LLOC | 17 | 17 | **16 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Comments (CLOC) | 2 | **7 ✅** | 3 | -5 | Lower | -1 | Lower | +4 | Higher |
| Blank Lines | 5 | 5 | 8 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00064

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 75.4603 | 75.1047 | **79.2404 ✅** | +0.3556 | Higher | -3.7800 | Lower | -4.1356 | Lower |
| MI (SEI) | 64.4907 | 64.3447 | **72.4722 ✅** | +0.1459 | Higher | -7.9815 | Lower | -8.1274 | Lower |
| Cyclomatic (sum) | 8 | **6 ✅** | 7 | +2 | Higher | +1 | Higher | -1 | Lower |
| Cognitive (sum) | 19 | **6 ✅** | 12 | +13 | Higher | +7 | Higher | -6 | Lower |
| Halstead Volume | 739.3422 | 1601.8447 | **691.8507 ✅** | -862.5025 | Lower | +47.4915 | Higher | +909.9940 | Higher |
| Halstead Effort | **16601.5925 ✅** | 21699.2155 | 17587.5718 | -5097.6229 | Lower | -985.9793 | Lower | +4111.6436 | Higher |
| SLOC | 39 | 32 | 32 | +7 | Higher | +7 | Higher | 0 | Same |
| PLOC | 31 | **22 ✅** | 24 | +9 | Higher | +7 | Higher | -2 | Lower |
| LLOC | 28 | **10 ✅** | 20 | +18 | Higher | +8 | Higher | -10 | Lower |
| Comments (CLOC) | 7 | 6 | 7 | +1 | Higher | 0 | Same | -1 | Lower |
| Blank Lines | 6 | **5 ✅** | 7 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00065

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.2551 | **74.2600 ✅** | 56.3883 | -7.0049 | Lower | +10.8668 | Higher | +17.8717 | Higher |
| MI (SEI) | 47.1849 | **63.5494 ✅** | 24.8750 | -16.3645 | Lower | +22.3100 | Higher | +38.6745 | Higher |
| Cyclomatic (sum) | 9 | 9 | 19 | 0 | Same | -10 | Lower | -10 | Lower |
| Cognitive (sum) | **7 ✅** | 11 | 32 | -4 | Lower | -25 | Lower | -21 | Lower |
| Halstead Volume | 2353.5124 | **1049.8173 ✅** | 2138.1631 | +1303.6951 | Higher | +215.3494 | Higher | -1088.3458 | Lower |
| Halstead Effort | 39490.5539 | **25013.9159 ✅** | 112053.1080 | +14476.6381 | Higher | -72562.5541 | Lower | -87039.1921 | Lower |
| SLOC | 44 | **37 ✅** | 77 | +7 | Higher | -33 | Lower | -40 | Lower |
| PLOC | 32 | **25 ✅** | 64 | +7 | Higher | -32 | Lower | -39 | Lower |
| LLOC | 16 | 16 | 43 | 0 | Same | -27 | Lower | -27 | Lower |
| Comments (CLOC) | 5 | **7 ✅** | 4 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 7 | **5 ✅** | 10 | +2 | Higher | -3 | Lower | -5 | Lower |
| Functions | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 3 | 2 | **0 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Exits | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |

## p00066

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.7221 | **71.9277 ✅** | 51.4916 | -5.2055 | Lower | +15.2306 | Higher | +20.4361 | Higher |
| MI (SEI) | 44.8543 | **64.5668 ✅** | 17.2973 | -19.7125 | Lower | +27.5569 | Higher | +47.2694 | Higher |
| Cyclomatic (sum) | 22 | **8 ✅** | 35 | +14 | Higher | -13 | Lower | -27 | Lower |
| Cognitive (sum) | 21 | **6 ✅** | 43 | +15 | Higher | -22 | Lower | -37 | Lower |
| Halstead Volume | **1277.5070 ✅** | 1581.3648 | 2814.1400 | -303.8578 | Lower | -1536.6330 | Lower | -1232.7752 | Lower |
| Halstead Effort | 42010.3263 | **24977.9429 ✅** | 138788.2676 | +17032.3834 | Higher | -96777.9413 | Lower | -113810.3247 | Lower |
| SLOC | 46 | **38 ✅** | 76 | +8 | Higher | -30 | Lower | -38 | Lower |
| PLOC | 40 | **25 ✅** | 71 | +15 | Higher | -31 | Lower | -46 | Lower |
| LLOC | 13 | **9 ✅** | 37 | +4 | Higher | -24 | Lower | -28 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 3 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 5 | 6 | 5 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00067

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **62.3212 ✅** | 52.9769 | 57.5265 | +9.3443 | Higher | +4.7948 | Higher | -4.5496 | Lower |
| MI (SEI) | **47.4526 ✅** | 26.7499 | 26.4152 | +20.7027 | Higher | +21.0374 | Higher | +0.3347 | Higher |
| Cyclomatic (sum) | **15 ✅** | 26 | 18 | -11 | Lower | -3 | Lower | +8 | Higher |
| Cognitive (sum) | **19 ✅** | 25 | 25 | -6 | Lower | -6 | Lower | 0 | Same |
| Halstead Volume | **1684.7103 ✅** | 3892.7266 | 1795.5249 | -2208.0162 | Lower | -110.8146 | Lower | +2097.2017 | Higher |
| Halstead Effort | 104249.8750 | 124984.3277 | **81157.7255 ✅** | -20734.4527 | Lower | +23092.1495 | Higher | +43826.6022 | Higher |
| SLOC | **61 ✅** | 71 | 77 | -10 | Lower | -16 | Lower | -6 | Lower |
| PLOC | **41 ✅** | 59 | 61 | -18 | Lower | -20 | Lower | -2 | Lower |
| LLOC | **30 ✅** | 35 | 36 | -5 | Lower | -6 | Lower | -1 | Lower |
| Comments (CLOC) | **12 ✅** | 7 | 4 | +5 | Higher | +8 | Higher | +3 | Higher |
| Blank Lines | 11 | 11 | 12 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | **2 ✅** | 3 | 3 | -1 | Lower | -1 | Lower | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 2 | 2 | -1 | Lower | -1 | Lower | 0 | Same |

## p00068

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.4452 | **53.8587 ✅** | 42.1265 | -5.4134 | Lower | +6.3187 | Higher | +11.7322 | Higher |
| MI (SEI) | 14.8257 | **28.9337 ✅** | -2.3504 | -14.1080 | Lower | +17.1761 | Higher | +31.2841 | Higher |
| Cyclomatic (sum) | 19 | **17 ✅** | 29 | +2 | Higher | -10 | Lower | -12 | Lower |
| Cognitive (sum) | 33 | **20 ✅** | 78 | +13 | Higher | -45 | Lower | -58 | Lower |
| Halstead Volume | 4646.6734 | **2692.5737 ✅** | 4379.7164 | +1954.0997 | Higher | +266.9570 | Higher | -1687.1427 | Lower |
| Halstead Effort | 142714.1091 | **123044.3555 ✅** | 348561.3317 | +19669.7536 | Higher | -205847.2225 | Lower | -225516.9762 | Lower |
| SLOC | 98 | **86 ✅** | 128 | +12 | Higher | -30 | Lower | -42 | Lower |
| PLOC | 83 | **67 ✅** | 113 | +16 | Higher | -30 | Lower | -46 | Lower |
| LLOC | 41 | **38 ✅** | 43 | +3 | Higher | -2 | Lower | -5 | Lower |
| Comments (CLOC) | 6 | **10 ✅** | 2 | -4 | Lower | +4 | Higher | +8 | Higher |
| Blank Lines | 12 | 12 | 13 | 0 | Same | -1 | Lower | -1 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p00069

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 47.0128 | **57.3895 ✅** | 47.6630 | -10.3767 | Lower | -0.6502 | Lower | +9.7265 | Higher |
| MI (SEI) | 9.7399 | **48.8979 ✅** | -4.4940 | -39.1580 | Lower | +14.2339 | Higher | +53.3919 | Higher |
| Cyclomatic (sum) | 24 | **22 ✅** | 24 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 52 | **26 ✅** | 40 | +26 | Higher | +12 | Higher | -14 | Lower |
| Halstead Volume | 4466.1459 | **1903.8438 ✅** | 3597.6905 | +2562.3020 | Higher | +868.4553 | Higher | -1693.8467 | Lower |
| Halstead Effort | 145149.7408 | **115182.5513 ✅** | 265689.4463 | +29967.1895 | Higher | -120539.7055 | Lower | -150506.8950 | Lower |
| SLOC | 101 | **72 ✅** | 104 | +29 | Higher | -3 | Lower | -32 | Lower |
| PLOC | 86 | **54 ✅** | 89 | +32 | Higher | -3 | Lower | -35 | Lower |
| LLOC | 59 | **27 ✅** | 68 | +32 | Higher | -9 | Lower | -41 | Lower |
| Comments (CLOC) | 4 | **25 ✅** | 0 | -21 | Lower | +4 | Higher | +25 | Higher |
| Blank Lines | 15 | **5 ✅** | 15 | +10 | Higher | 0 | Same | -10 | Lower |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 7 | 2 | -5 | Lower | 0 | Same | +5 | Higher |

## p00070

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.0525 | **75.0741 ✅** | 69.9535 | -6.0216 | Lower | -0.9010 | Lower | +5.1206 | Higher |
| MI (SEI) | 48.9386 | **70.8113 ✅** | 48.5502 | -21.8727 | Lower | +0.3884 | Higher | +22.2611 | Higher |
| Cyclomatic (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Cognitive (sum) | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 1417.6314 | **938.2737 ✅** | 1464.0554 | +479.3577 | Higher | -46.4240 | Lower | -525.7817 | Lower |
| Halstead Effort | 40989.7994 | **29375.1843 ✅** | 45062.7638 | +11614.6151 | Higher | -4072.9644 | Lower | -15687.5795 | Lower |
| SLOC | 47 | **37 ✅** | 44 | +10 | Higher | +3 | Higher | -7 | Lower |
| PLOC | 32 | **23 ✅** | 32 | +9 | Higher | 0 | Same | -9 | Lower |
| LLOC | 21 | **16 ✅** | 20 | +5 | Higher | +1 | Higher | -4 | Lower |
| Comments (CLOC) | 5 | **11 ✅** | 4 | -6 | Lower | +1 | Higher | +7 | Higher |
| Blank Lines | 10 | **6 ✅** | 9 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00071

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.4008 | **67.0884 ✅** | 62.0321 | -6.6875 | Lower | -1.6312 | Lower | +5.0563 | Higher |
| MI (SEI) | 43.1205 | **57.2804 ✅** | 25.8007 | -14.1600 | Lower | +17.3198 | Higher | +31.4798 | Higher |
| Cyclomatic (sum) | 17 | **12 ✅** | 17 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 19 | **14 ✅** | 24 | +5 | Higher | -5 | Lower | -10 | Lower |
| Halstead Volume | 1921.0482 | **1622.8783 ✅** | 2127.9750 | +298.1699 | Higher | -206.9268 | Lower | -505.0968 | Lower |
| Halstead Effort | 94462.5763 | **52997.1187 ✅** | 138359.2996 | +41465.4576 | Higher | -43896.7233 | Lower | -85362.1809 | Lower |
| SLOC | 64 | **48 ✅** | 56 | +16 | Higher | +8 | Higher | -8 | Lower |
| PLOC | 44 | **32 ✅** | 49 | +12 | Higher | -5 | Lower | -17 | Lower |
| LLOC | 29 | **26 ✅** | 40 | +3 | Higher | -11 | Lower | -14 | Lower |
| Comments (CLOC) | 11 | **12 ✅** | 1 | -1 | Lower | +10 | Higher | +11 | Higher |
| Blank Lines | 9 | 9 | **6 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00072

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.8714 | **58.8135 ✅** | 55.9268 | -3.9421 | Lower | -1.0554 | Lower | +2.8867 | Higher |
| MI (SEI) | 13.8069 | **42.9731 ✅** | 6.6135 | -29.1662 | Lower | +7.1934 | Higher | +36.3595 | Higher |
| Cyclomatic (sum) | 18 | **15 ✅** | 16 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 15 | 20 | **10 ✅** | -5 | Lower | +5 | Higher | +10 | Higher |
| Halstead Volume | 2459.3350 | **1890.2050 ✅** | 2278.7366 | +569.1300 | Higher | +180.5984 | Higher | -388.5316 | Lower |
| Halstead Effort | 102003.8464 | **65112.5877 ✅** | 91092.4949 | +36891.2587 | Higher | +10911.3515 | Higher | -25979.9072 | Lower |
| SLOC | 82 | **73 ✅** | 81 | +9 | Higher | +1 | Higher | -8 | Lower |
| PLOC | 69 | **47 ✅** | 68 | +22 | Higher | +1 | Higher | -21 | Lower |
| LLOC | 42 | 42 | **35 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Comments (CLOC) | 1 | **15 ✅** | 0 | -14 | Lower | +1 | Higher | +15 | Higher |
| Blank Lines | 13 | 13 | 13 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 4 | **1 ✅** | 4 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 2 | 0 | Same | -2 | Lower | -2 | Lower |
| Exits | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |

## p00073

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.6238 | 76.5311 | **80.2185 ✅** | +2.0927 | Higher | -1.5947 | Lower | -3.6874 | Lower |
| MI (SEI) | **71.6845 ✅** | 70.7453 | 64.7400 | +0.9392 | Higher | +6.9445 | Higher | +6.0052 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 7 | 7 | 7 | 0 | Same | 0 | Same | 0 | Same |
| Halstead Volume | 745.2417 | **628.9629 ✅** | 745.2417 | +116.2788 | Higher | 0 | Same | -116.2788 | Lower |
| Halstead Effort | 15929.5416 | **12579.2573 ✅** | 15011.2974 | +3350.2843 | Higher | +918.2443 | Higher | -2432.0401 | Lower |
| SLOC | 32 | 39 | **29 ✅** | -7 | Lower | +3 | Higher | +10 | Higher |
| PLOC | **22 ✅** | 25 | 23 | -3 | Lower | -1 | Lower | +2 | Higher |
| LLOC | 11 | 15 | 11 | -4 | Lower | 0 | Same | +4 | Higher |
| Comments (CLOC) | 7 | **10 ✅** | 3 | -3 | Lower | +4 | Higher | +7 | Higher |
| Blank Lines | 6 | 8 | **4 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00074

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 72.0473 | 63.8257 | **76.2864 ✅** | +8.2216 | Higher | -4.2390 | Lower | -12.4606 | Lower |
| MI (SEI) | 58.4151 | 49.2158 | **63.0575 ✅** | +9.1993 | Higher | -4.6424 | Lower | -13.8417 | Lower |
| Cyclomatic (sum) | 9 | 11 | **7 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Cognitive (sum) | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | **1005.9566 ✅** | 1505.5920 | 1010.8782 | -499.6354 | Lower | -4.9216 | Lower | +494.7137 | Higher |
| Halstead Effort | 31040.9462 | 52410.8766 | **24766.5162 ✅** | -21369.9304 | Lower | +6274.4300 | Higher | +27644.3604 | Higher |
| SLOC | 43 | 61 | **34 ✅** | -18 | Lower | +9 | Higher | +27 | Higher |
| PLOC | 28 | 40 | **24 ✅** | -12 | Lower | +4 | Higher | +16 | Higher |
| LLOC | 17 | 24 | 17 | -7 | Lower | 0 | Same | +7 | Higher |
| Comments (CLOC) | 7 | **12 ✅** | 5 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | 9 | 11 | **5 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00075

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.0019 ✅** | 66.4776 | 70.8838 | +8.5242 | Higher | +4.1180 | Higher | -4.4062 | Lower |
| MI (SEI) | **64.3716 ✅** | 52.4848 | 61.5473 | +11.8868 | Higher | +2.8243 | Higher | -9.0625 | Lower |
| Cyclomatic (sum) | **10 ✅** | 17 | 13 | -7 | Lower | -3 | Lower | +4 | Higher |
| Cognitive (sum) | 22 | 19 | **13 ✅** | +3 | Higher | +9 | Higher | +6 | Higher |
| Halstead Volume | **801.4276 ✅** | 1288.2988 | 981.3296 | -486.8712 | Lower | -179.9020 | Lower | +306.9692 | Higher |
| Halstead Effort | **17384.8136 ✅** | 42964.7659 | 22973.6265 | -25579.9523 | Lower | -5588.8129 | Lower | +19991.1394 | Higher |
| SLOC | **38 ✅** | 50 | 44 | -12 | Lower | -6 | Lower | +6 | Higher |
| PLOC | **29 ✅** | 38 | 31 | -9 | Lower | -2 | Lower | +7 | Higher |
| LLOC | **12 ✅** | 18 | 13 | -6 | Lower | -1 | Lower | +5 | Higher |
| Comments (CLOC) | 7 | 9 | **10 ✅** | -2 | Lower | -3 | Lower | -1 | Lower |
| Blank Lines | **2 ✅** | 7 | 8 | -5 | Lower | -6 | Lower | -1 | Lower |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 8 | 0 | -8 | Lower | 0 | Same | +8 | Higher |

## p00077

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.5491 | **74.1632 ✅** | 69.3321 | -0.6141 | Lower | +4.2170 | Higher | +4.8311 | Higher |
| MI (SEI) | **68.2900 ✅** | 63.9422 | 62.0723 | +4.3478 | Higher | +6.2177 | Higher | +1.8699 | Higher |
| Cyclomatic (sum) | 7 | 7 | 9 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **12 ✅** | 15 | 13 | -3 | Lower | -1 | Lower | +2 | Higher |
| Halstead Volume | 955.0286 | **916.5072 ✅** | 1128.8126 | +38.5214 | Higher | -173.7841 | Lower | -212.3055 | Lower |
| Halstead Effort | **19100.5718 ✅** | 21097.2898 | 32969.1141 | -1996.7181 | Lower | -13868.5423 | Lower | -11871.8243 | Lower |
| SLOC | 41 | **40 ✅** | 49 | +1 | Higher | -8 | Lower | -9 | Lower |
| PLOC | 30 | **28 ✅** | 40 | +2 | Higher | -10 | Lower | -12 | Lower |
| LLOC | **20 ✅** | 22 | 34 | -2 | Lower | -14 | Lower | -12 | Lower |
| Comments (CLOC) | 12 | 8 | **14 ✅** | +4 | Higher | -2 | Lower | -6 | Lower |
| Blank Lines | **6 ✅** | 7 | 7 | -1 | Lower | -1 | Lower | 0 | Same |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00078

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.0622 ✅** | 64.0761 | 70.3248 | +8.9861 | Higher | +2.7374 | Higher | -6.2487 | Lower |
| MI (SEI) | **64.0439 ✅** | 53.3712 | 38.3462 | +10.6727 | Higher | +25.6977 | Higher | +15.0250 | Higher |
| Cyclomatic (sum) | **8 ✅** | 14 | 10 | -6 | Lower | -2 | Lower | +4 | Higher |
| Cognitive (sum) | 13 | **12 ✅** | 15 | +1 | Higher | -2 | Lower | -3 | Lower |
| Halstead Volume | **1083.6204 ✅** | 1394.0262 | 1247.7500 | -310.4058 | Lower | -164.1296 | Lower | +146.2762 | Higher |
| Halstead Effort | **44022.0783 ✅** | 58549.0996 | 54143.4354 | -14527.0213 | Lower | -10121.3571 | Lower | +4405.6642 | Higher |
| SLOC | **40 ✅** | 59 | 44 | -19 | Lower | -4 | Lower | +15 | Higher |
| PLOC | **33 ✅** | 43 | 37 | -10 | Lower | -4 | Lower | +6 | Higher |
| LLOC | 30 | **29 ✅** | 31 | +1 | Higher | -1 | Lower | -2 | Lower |
| Comments (CLOC) | 9 | **15 ✅** | 1 | -6 | Lower | +8 | Higher | +14 | Higher |
| Blank Lines | **6 ✅** | 10 | 7 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00079

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.4168 | **70.1551 ✅** | 67.1469 | -6.7383 | Lower | -3.7300 | Lower | +3.0082 | Higher |
| MI (SEI) | 36.6384 | **45.8989 ✅** | 33.1653 | -9.2605 | Lower | +3.4730 | Higher | +12.7335 | Higher |
| Cyclomatic (sum) | 12 | **9 ✅** | 10 | +3 | Higher | +2 | Higher | -1 | Lower |
| Cognitive (sum) | 13 | 11 | **8 ✅** | +2 | Higher | +5 | Higher | +3 | Higher |
| Halstead Volume | 1728.8394 | **1256.3354 ✅** | 1644.0752 | +472.5040 | Higher | +84.7642 | Higher | -387.7398 | Lower |
| Halstead Effort | 56355.9493 | **34660.0772 ✅** | 54064.7806 | +21695.8721 | Higher | +2291.1687 | Higher | -19404.7034 | Lower |
| SLOC | 59 | **45 ✅** | 49 | +14 | Higher | +10 | Higher | -4 | Lower |
| PLOC | 46 | **38 ✅** | 41 | +8 | Higher | +5 | Higher | -3 | Lower |
| LLOC | 23 | **17 ✅** | 19 | +6 | Higher | +4 | Higher | -2 | Lower |
| Comments (CLOC) | **4 ✅** | 3 | 1 | +1 | Higher | +3 | Higher | +2 | Higher |
| Blank Lines | 9 | **6 ✅** | 7 | +3 | Higher | +2 | Higher | -1 | Lower |
| Functions | 4 | **2 ✅** | 4 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 5 | **2 ✅** | 4 | +3 | Higher | +1 | Higher | -2 | Lower |

## p00080

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.9052 | 66.4311 | **87.3580 ✅** | +13.4741 | Higher | -7.4528 | Lower | -20.9269 | Lower |
| MI (SEI) | **73.8633 ✅** | 56.3977 | 67.4222 | +17.4656 | Higher | +6.4411 | Higher | -11.0245 | Lower |
| Cyclomatic (sum) | 7 | 10 | **5 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Cognitive (sum) | 3 | 3 | **2 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 672.1053 | 1665.5996 | **589.3307 ✅** | -993.4944 | Lower | +82.7745 | Higher | +1076.2689 | Higher |
| Halstead Effort | 14170.2190 | 25001.5269 | **11225.3471 ✅** | -10831.3079 | Lower | +2944.8720 | Higher | +13776.1798 | Higher |
| SLOC | 31 | 51 | **21 ✅** | -20 | Lower | +10 | Higher | +30 | Higher |
| PLOC | 19 | 34 | **16 ✅** | -15 | Lower | +3 | Higher | +18 | Higher |
| LLOC | 11 | 15 | 11 | -4 | Lower | 0 | Same | +4 | Higher |
| Comments (CLOC) | 7 | **13 ✅** | 1 | -6 | Lower | +6 | Higher | +12 | Higher |
| Blank Lines | 6 | 8 | **4 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00081

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.8887 | **71.8531 ✅** | 67.5294 | -9.9644 | Lower | -5.6407 | Lower | +4.3237 | Higher |
| MI (SEI) | 24.5584 | **63.7603 ✅** | 33.7272 | -39.2019 | Lower | -9.1688 | Lower | +30.0331 | Higher |
| Cyclomatic (sum) | 11 | **5 ✅** | 9 | +6 | Higher | +2 | Higher | -4 | Lower |
| Cognitive (sum) | 8 | **3 ✅** | 7 | +5 | Higher | +1 | Higher | -4 | Lower |
| Halstead Volume | 2185.1900 | **1246.3539 ✅** | 1702.4745 | +938.8361 | Higher | +482.7156 | Higher | -456.1205 | Lower |
| Halstead Effort | 97058.8579 | **29584.5061 ✅** | 67779.7644 | +67474.3518 | Higher | +29279.0934 | Higher | -38195.2583 | Lower |
| SLOC | 61 | **43 ✅** | 48 | +18 | Higher | +13 | Higher | -5 | Lower |
| PLOC | 51 | **25 ✅** | 41 | +26 | Higher | +10 | Higher | -16 | Lower |
| LLOC | 33 | 17 | **15 ✅** | +16 | Higher | +18 | Higher | +2 | Higher |
| Comments (CLOC) | 1 | **11 ✅** | 1 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 9 | 9 | **6 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00082

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.1658 | **67.1454 ✅** | 53.1101 | -5.9796 | Lower | +8.0557 | Higher | +14.0353 | Higher |
| MI (SEI) | 34.6949 | **52.8663 ✅** | 28.1778 | -18.1714 | Lower | +6.5171 | Higher | +24.6886 | Higher |
| Cyclomatic (sum) | 13 | **10 ✅** | 24 | +3 | Higher | -11 | Lower | -14 | Lower |
| Cognitive (sum) | 14 | **3 ✅** | 15 | +11 | Higher | -1 | Lower | -12 | Lower |
| Halstead Volume | **1885.8510 ✅** | 2299.6825 | 2050.3381 | -413.8316 | Lower | -164.4872 | Lower | +249.3444 | Higher |
| Halstead Effort | 78335.3478 | **39126.5432 ✅** | 100834.5786 | +39208.8046 | Higher | -22499.2308 | Lower | -61708.0354 | Lower |
| SLOC | 65 | **44 ✅** | 89 | +21 | Higher | -24 | Lower | -45 | Lower |
| PLOC | 50 | **31 ✅** | 71 | +19 | Higher | -21 | Lower | -40 | Lower |
| LLOC | 46 | **9 ✅** | 60 | +37 | Higher | -14 | Lower | -51 | Lower |
| Comments (CLOC) | 5 | 8 | **10 ✅** | -3 | Lower | -5 | Lower | -2 | Lower |
| Blank Lines | 11 | **8 ✅** | 13 | +3 | Higher | -2 | Lower | -5 | Lower |
| Functions | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Closures | 2 | 2 | **1 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00083

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.1076 ✅** | 60.5075 | 42.3324 | +5.6001 | Higher | +23.7752 | Higher | +18.1752 | Higher |
| MI (SEI) | 34.0565 | **40.2144 ✅** | -4.2875 | -6.1579 | Lower | +38.3440 | Higher | +44.5019 | Higher |
| Cyclomatic (sum) | 25 | **19 ✅** | 34 | +6 | Higher | -9 | Lower | -15 | Lower |
| Cognitive (sum) | 20 | **15 ✅** | 72 | +5 | Higher | -52 | Lower | -57 | Lower |
| Halstead Volume | **1671.6500 ✅** | 3100.0649 | 3544.1344 | -1428.4149 | Lower | -1872.4843 | Lower | -444.0694 | Lower |
| Halstead Effort | **59622.1847 ✅** | 72979.6281 | 267677.9314 | -13357.4434 | Lower | -208055.7467 | Lower | -194698.3033 | Lower |
| SLOC | **42 ✅** | 53 | 126 | -11 | Lower | -84 | Lower | -73 | Lower |
| PLOC | **35 ✅** | 44 | 116 | -9 | Lower | -81 | Lower | -72 | Lower |
| LLOC | 29 | **14 ✅** | 109 | +15 | Higher | -80 | Lower | -95 | Lower |
| Comments (CLOC) | 1 | **7 ✅** | 1 | -6 | Lower | 0 | Same | +6 | Higher |
| Blank Lines | 7 | 7 | 10 | 0 | Same | -3 | Lower | -3 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00084

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **87.5073 ✅** | 83.8247 | 79.9840 | +3.6826 | Higher | +7.5233 | Higher | +3.8407 | Higher |
| MI (SEI) | 82.2415 | **84.4678 ✅** | 66.8147 | -2.2263 | Lower | +15.4268 | Higher | +17.6531 | Higher |
| Cyclomatic (sum) | **4 ✅** | 8 | 7 | -4 | Lower | -3 | Lower | +1 | Higher |
| Cognitive (sum) | **1 ✅** | 5 | 8 | -4 | Lower | -7 | Lower | -3 | Lower |
| Halstead Volume | 598.5472 | **591.4629 ✅** | 662.0043 | +7.0844 | Higher | -63.4571 | Lower | -70.5414 | Lower |
| Halstead Effort | **8158.4590 ✅** | 9130.7079 | 10009.5050 | -972.2488 | Lower | -1851.0460 | Lower | -878.7971 | Lower |
| SLOC | **21 ✅** | 25 | 31 | -4 | Lower | -10 | Lower | -6 | Lower |
| PLOC | **13 ✅** | 14 | 21 | -1 | Lower | -8 | Lower | -7 | Lower |
| LLOC | **7 ✅** | 9 | 15 | -2 | Lower | -8 | Lower | -6 | Lower |
| Comments (CLOC) | 4 | **8 ✅** | 4 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | **4 ✅** | 5 | 6 | -1 | Lower | -2 | Lower | -1 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00085

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 73.4871 | 63.4797 | **73.5169 ✅** | +10.0074 | Higher | -0.0297 | Lower | -10.0372 | Lower |
| MI (SEI) | **63.1705 ✅** | 46.0568 | 31.2780 | +17.1137 | Higher | +31.8926 | Higher | +14.7789 | Higher |
| Cyclomatic (sum) | 9 | 16 | 9 | -7 | Lower | 0 | Same | +7 | Higher |
| Cognitive (sum) | 11 | 14 | **9 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Halstead Volume | 955.3893 | 2121 | **949.9411 ✅** | -1165.6107 | Lower | +5.4482 | Higher | +1171.0589 | Higher |
| Halstead Effort | 33655.7588 | 44062.7353 | **31212.3505 ✅** | -10406.9765 | Lower | +2443.4083 | Higher | +12850.3848 | Higher |
| SLOC | 40 | 52 | 40 | -12 | Lower | 0 | Same | +12 | Higher |
| PLOC | **32 ✅** | 40 | 35 | -8 | Lower | -3 | Lower | +5 | Higher |
| LLOC | 27 | **20 ✅** | 32 | +7 | Higher | -5 | Lower | -12 | Lower |
| Comments (CLOC) | 8 | 8 | 0 | 0 | Same | +8 | Higher | +8 | Higher |
| Blank Lines | 7 | 10 | **5 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00086

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **65.6722 ✅** | 64.6828 | 63.3229 | +0.9894 | Higher | +2.3493 | Higher | +1.3599 | Higher |
| MI (SEI) | 50.2295 | **51.7366 ✅** | 41.1965 | -1.5070 | Lower | +9.0330 | Higher | +10.5401 | Higher |
| Cyclomatic (sum) | **16 ✅** | 17 | 18 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 24 | **21 ✅** | 26 | +3 | Higher | -2 | Lower | -5 | Lower |
| Halstead Volume | 1236.9954 | **1145.7884 ✅** | 1350.0372 | +91.2070 | Higher | -113.0418 | Lower | -204.2488 | Lower |
| Halstead Effort | 57004.8722 | **52008.8292 ✅** | 70671.5142 | +4996.0430 | Higher | -13666.6420 | Lower | -18662.6850 | Lower |
| SLOC | **54 ✅** | 58 | 59 | -4 | Lower | -5 | Lower | -1 | Lower |
| PLOC | 41 | **39 ✅** | 46 | +2 | Higher | -5 | Lower | -7 | Lower |
| LLOC | 29 | **27 ✅** | 35 | +2 | Higher | -6 | Lower | -8 | Lower |
| Comments (CLOC) | 9 | **12 ✅** | 6 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | **8 ✅** | 10 | 9 | -2 | Lower | -1 | Lower | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00088

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.6754 | **70.3747 ✅** | 64.1621 | -3.6993 | Lower | +2.5134 | Higher | +6.2126 | Higher |
| MI (SEI) | 48.2201 | **62.1365 ✅** | 40.0287 | -13.9164 | Lower | +8.1915 | Higher | +22.1079 | Higher |
| Cyclomatic (sum) | **8 ✅** | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Cognitive (sum) | **7 ✅** | 9 | 11 | -2 | Lower | -4 | Lower | -2 | Lower |
| Halstead Volume | 1634.2362 | **1327.6049 ✅** | 1822.1825 | +306.6313 | Higher | -187.9463 | Lower | -494.5776 | Lower |
| Halstead Effort | 32895.5932 | **22474.4544 ✅** | 37205.1591 | +10421.1388 | Higher | -4309.5659 | Lower | -14730.7047 | Lower |
| SLOC | 52 | **43 ✅** | 57 | +9 | Higher | -5 | Lower | -14 | Lower |
| PLOC | 36 | **27 ✅** | 50 | +9 | Higher | -14 | Lower | -23 | Lower |
| LLOC | 25 | **13 ✅** | 43 | +12 | Higher | -18 | Lower | -30 | Lower |
| Comments (CLOC) | 7 | **11 ✅** | 5 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 9 | 6 | **4 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00089

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.1064 | **65.6505 ✅** | 53.8525 | -2.5441 | Lower | +9.2539 | Higher | +11.7980 | Higher |
| MI (SEI) | 27.0350 | **50.3751 ✅** | 15.2950 | -23.3401 | Lower | +11.7400 | Higher | +35.0801 | Higher |
| Cyclomatic (sum) | 13 | **8 ✅** | 13 | +5 | Higher | 0 | Same | -5 | Lower |
| Cognitive (sum) | 18 | **3 ✅** | 14 | +15 | Higher | +4 | Higher | -11 | Lower |
| Halstead Volume | **2185.0269 ✅** | 2248.9587 | 3732.1680 | -63.9318 | Lower | -1547.1411 | Lower | -1483.2093 | Lower |
| Halstead Effort | 115044.9775 | **37003.8747 ✅** | 112930.2558 | +78041.1028 | Higher | +2114.7218 | Higher | -75926.3811 | Lower |
| SLOC | 55 | **50 ✅** | 82 | +5 | Higher | -27 | Lower | -32 | Lower |
| PLOC | 46 | **34 ✅** | 63 | +12 | Higher | -17 | Lower | -29 | Lower |
| LLOC | 26 | **10 ✅** | 35 | +16 | Higher | -9 | Lower | -25 | Lower |
| Comments (CLOC) | 1 | **9 ✅** | 2 | -8 | Lower | -1 | Lower | +7 | Higher |
| Blank Lines | **9 ✅** | 11 | 19 | -2 | Lower | -10 | Lower | -8 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | 5 | **1 ✅** | 2 | +4 | Higher | +3 | Higher | -1 | Lower |

## p00090

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 44.6757 | **55.0443 ✅** | 43.0075 | -10.3687 | Lower | +1.6682 | Higher | +12.0369 | Higher |
| MI (SEI) | 8.5320 | **26.1689 ✅** | 8.1843 | -17.6369 | Lower | +0.3477 | Higher | +17.9846 | Higher |
| Cyclomatic (sum) | 35 | **16 ✅** | 35 | +19 | Higher | 0 | Same | -19 | Lower |
| Cognitive (sum) | 94 | **14 ✅** | 67 | +80 | Higher | +27 | Higher | -53 | Lower |
| Halstead Volume | 3298.6248 | **3037.0746 ✅** | 3466.8224 | +261.5502 | Higher | -168.1976 | Lower | -429.7478 | Lower |
| Halstead Effort | 286273.5056 | **76306.4985 ✅** | 309413.8952 | +209967.0072 | Higher | -23140.3896 | Lower | -233107.3968 | Lower |
| SLOC | 110 | **78 ✅** | 120 | +32 | Higher | -10 | Lower | -42 | Lower |
| PLOC | 92 | **58 ✅** | 98 | +34 | Higher | -6 | Lower | -40 | Lower |
| LLOC | 73 | **27 ✅** | 82 | +46 | Higher | -9 | Lower | -55 | Lower |
| Comments (CLOC) | 5 | 6 | **7 ✅** | -1 | Lower | -2 | Lower | -1 | Lower |
| Blank Lines | **13 ✅** | 16 | 15 | -3 | Lower | -2 | Lower | +1 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00092

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.1780 | **70.7329 ✅** | 59.9380 | -10.5549 | Lower | +0.2400 | Higher | +10.7949 | Higher |
| MI (SEI) | 12.7468 | **43.1335 ✅** | 12.4005 | -30.3867 | Lower | +0.3463 | Higher | +30.7330 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 16 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 24 | **12 ✅** | 34 | +12 | Higher | -10 | Lower | -22 | Lower |
| Halstead Volume | 1817.0931 | **1049.8173 ✅** | 1817.0931 | +767.2758 | Higher | 0 | Same | -767.2758 | Lower |
| Halstead Effort | 86792.9173 | **29100.9354 ✅** | 78362.1398 | +57691.9820 | Higher | +8430.7775 | Higher | -49261.2044 | Lower |
| SLOC | 67 | **46 ✅** | 68 | +21 | Higher | -1 | Lower | -22 | Lower |
| PLOC | 60 | **36 ✅** | 61 | +24 | Higher | -1 | Lower | -25 | Lower |
| LLOC | 47 | **26 ✅** | 46 | +21 | Higher | +1 | Higher | -20 | Lower |
| Comments (CLOC) | 0 | **2 ✅** | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Blank Lines | 7 | 9 | 7 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p00093

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **70.3581 ✅** | 69.3659 | 69.5562 | +0.9923 | Higher | +0.8019 | Higher | -0.1903 | Lower |
| MI (SEI) | **52.2726 ✅** | 49.7697 | 37.6194 | +2.5029 | Higher | +14.6532 | Higher | +12.1502 | Higher |
| Cyclomatic (sum) | 15 | **14 ✅** | 15 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 17 | 17 | **10 ✅** | 0 | Same | +7 | Higher | +7 | Higher |
| Halstead Volume | 993.8036 | **958.6399 ✅** | 1081.1143 | +35.1638 | Higher | -87.3106 | Lower | -122.4744 | Lower |
| Halstead Effort | 27253.1537 | **27192.1890 ✅** | 30181.1066 | +60.9647 | Higher | -2927.9529 | Lower | -2988.9176 | Lower |
| SLOC | **44 ✅** | 48 | 45 | -4 | Lower | -1 | Lower | +3 | Higher |
| PLOC | 36 | **35 ✅** | 39 | +1 | Higher | -3 | Lower | -4 | Lower |
| LLOC | 22 | **21 ✅** | 26 | +1 | Higher | -4 | Lower | -5 | Lower |
| Comments (CLOC) | 5 | 5 | 1 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 7 | 11 | **6 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00096

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 78.7348 | 76.5197 | **85.4158 ✅** | +2.2151 | Higher | -6.6810 | Lower | -8.8961 | Lower |
| MI (SEI) | 69.5863 | 66.1208 | **69.8636 ✅** | +3.4655 | Higher | -0.2772 | Lower | -3.7427 | Lower |
| Cyclomatic (sum) | 10 | 9 | **7 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Cognitive (sum) | 34 | **9 ✅** | 13 | +25 | Higher | +21 | Higher | -4 | Lower |
| Halstead Volume | 606.6998 | 576.9780 | **516.9925 ✅** | +29.7218 | Higher | +89.7073 | Higher | +59.9855 | Higher |
| Halstead Effort | 12740.6959 | 11539.5593 | **9305.8650 ✅** | +1201.1366 | Higher | +3434.8309 | Higher | +2233.6943 | Higher |
| SLOC | 33 | 39 | **24 ✅** | -6 | Lower | +9 | Higher | +15 | Higher |
| PLOC | 26 | 27 | **21 ✅** | -1 | Lower | +5 | Higher | +6 | Higher |
| LLOC | 16 | **13 ✅** | 14 | +3 | Higher | +2 | Higher | -1 | Lower |
| Comments (CLOC) | 6 | **7 ✅** | 2 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | 3 | 5 | 3 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00097

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.3814 | 64.7366 | **66.3500 ✅** | +0.6448 | Higher | -0.9686 | Lower | -1.6134 | Lower |
| MI (SEI) | 53.7972 | **55.5133 ✅** | 39.4227 | -1.7161 | Lower | +14.3745 | Higher | +16.0906 | Higher |
| Cyclomatic (sum) | 12 | **11 ✅** | 12 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 17 | **5 ✅** | 20 | +12 | Higher | -3 | Lower | -15 | Lower |
| Halstead Volume | **1394.0607 ✅** | 1958.1282 | 1457.6461 | -564.0675 | Lower | -63.5854 | Lower | +500.4822 | Higher |
| Halstead Effort | 46130.7363 | **40564.1725 ✅** | 48764.8872 | +5566.5639 | Higher | -2634.1509 | Lower | -8200.7147 | Lower |
| SLOC | 56 | 53 | **52 ✅** | +3 | Higher | +4 | Higher | +1 | Higher |
| PLOC | 39 | **32 ✅** | 41 | +7 | Higher | -2 | Lower | -9 | Lower |
| LLOC | 25 | **15 ✅** | 33 | +10 | Higher | -8 | Lower | -18 | Lower |
| Comments (CLOC) | 13 | **15 ✅** | 3 | -2 | Lower | +10 | Higher | +12 | Higher |
| Blank Lines | 9 | 8 | 8 | +1 | Higher | +1 | Higher | 0 | Same |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 2 | **0 ✅** | 1 | +2 | Higher | +1 | Higher | -1 | Lower |
| Exits | 0 | 4 | 0 | -4 | Lower | 0 | Same | +4 | Higher |

## p00098

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.4347 | **63.1536 ✅** | 58.6290 | -2.7189 | Lower | +1.8056 | Higher | +4.5246 | Higher |
| MI (SEI) | 33.8437 | **46.2906 ✅** | 30.2604 | -12.4468 | Lower | +3.5834 | Higher | +16.0302 | Higher |
| Cyclomatic (sum) | 15 | **13 ✅** | 15 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 29 | **17 ✅** | 29 | +12 | Higher | 0 | Same | -12 | Lower |
| Halstead Volume | **1986.7970 ✅** | 2292.6644 | 2044.4354 | -305.8674 | Lower | -57.6384 | Lower | +248.2290 | Higher |
| Halstead Effort | 94458.0063 | **44067.8495 ✅** | 125937.2186 | +50390.1568 | Higher | -31479.2123 | Lower | -81869.3691 | Lower |
| SLOC | 65 | **54 ✅** | 72 | +11 | Higher | -7 | Lower | -18 | Lower |
| PLOC | 54 | **40 ✅** | 59 | +14 | Higher | -5 | Lower | -19 | Lower |
| LLOC | 40 | **22 ✅** | 47 | +18 | Higher | -7 | Lower | -25 | Lower |
| Comments (CLOC) | 5 | **9 ✅** | 5 | -4 | Lower | 0 | Same | +4 | Higher |
| Blank Lines | 8 | **7 ✅** | 10 | +1 | Higher | -2 | Lower | -3 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00100

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.5520 | 61.3471 | **65.8538 ✅** | +4.2049 | Higher | -0.3018 | Lower | -4.5067 | Lower |
| MI (SEI) | 37.7432 | **46.4914 ✅** | 38.7739 | -8.7482 | Lower | -1.0306 | Lower | +7.7176 | Higher |
| Cyclomatic (sum) | **10 ✅** | 15 | 11 | -5 | Lower | -1 | Lower | +4 | Higher |
| Cognitive (sum) | 14 | **11 ✅** | 14 | +3 | Higher | 0 | Same | -3 | Lower |
| Halstead Volume | **1650.6758 ✅** | 2254.1801 | 1780.6588 | -603.5043 | Lower | -129.9830 | Lower | +473.5213 | Higher |
| Halstead Effort | 67620.7869 | **39974.8288 ✅** | 94438.5109 | +27645.9581 | Higher | -26817.7240 | Lower | -54463.6821 | Lower |
| SLOC | 54 | 59 | **51 ✅** | -5 | Lower | +3 | Higher | +8 | Higher |
| PLOC | 42 | 39 | 39 | +3 | Higher | +3 | Higher | 0 | Same |
| LLOC | 41 | **20 ✅** | 39 | +21 | Higher | +2 | Higher | -19 | Lower |
| Comments (CLOC) | 3 | **12 ✅** | 3 | -9 | Lower | 0 | Same | +9 | Higher |
| Blank Lines | 10 | **8 ✅** | 9 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00101

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 71.1792 | 66.1948 | **81.6150 ✅** | +4.9844 | Higher | -10.4358 | Lower | -15.4202 | Lower |
| MI (SEI) | 60.6561 | 55.3778 | **75.9083 ✅** | +5.2783 | Higher | -15.2522 | Lower | -20.5305 | Lower |
| Cyclomatic (sum) | 6 | 12 | **5 ✅** | -6 | Lower | +1 | Higher | +7 | Higher |
| Cognitive (sum) | 4 | 8 | **3 ✅** | -4 | Lower | +1 | Higher | +5 | Higher |
| Halstead Volume | 1100.1849 | 2200.3899 | **812.7943 ✅** | -1100.2051 | Lower | +287.3906 | Higher | +1387.5957 | Higher |
| Halstead Effort | 24967.0985 | 38400.6649 | **13661.1958 ✅** | -13433.5664 | Lower | +11305.9027 | Higher | +24739.4691 | Higher |
| SLOC | 46 | 46 | **27 ✅** | 0 | Same | +19 | Higher | +19 | Higher |
| PLOC | 30 | 32 | **19 ✅** | -2 | Lower | +11 | Higher | +13 | Higher |
| LLOC | 24 | **14 ✅** | 17 | +10 | Higher | +7 | Higher | -3 | Lower |
| Comments (CLOC) | 10 | **11 ✅** | 6 | -1 | Lower | +4 | Higher | +5 | Higher |
| Blank Lines | 9 | 7 | **3 ✅** | +2 | Higher | +6 | Higher | +4 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |

## p00103

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 61.0523 | **71.9179 ✅** | 70.8197 | -10.8656 | Lower | -9.7674 | Lower | +1.0982 | Higher |
| MI (SEI) | 46.5765 | **63.6576 ✅** | 27.6922 | -17.0811 | Lower | +18.8842 | Higher | +35.9653 | Higher |
| Cyclomatic (sum) | 16 | **9 ✅** | 12 | +7 | Higher | +4 | Higher | -3 | Lower |
| Cognitive (sum) | 31 | **9 ✅** | 33 | +22 | Higher | -2 | Lower | -24 | Lower |
| Halstead Volume | 1955.6602 | **686.5287 ✅** | 1038.4296 | +1269.1315 | Higher | +917.2306 | Higher | -351.9009 | Lower |
| Halstead Effort | 51203.9411 | **12357.5170 ✅** | 34268.1784 | +38846.4241 | Higher | +16935.7627 | Higher | -21910.6613 | Lower |
| SLOC | 62 | 49 | **44 ✅** | +13 | Higher | +18 | Higher | +5 | Higher |
| PLOC | 53 | **31 ✅** | 39 | +22 | Higher | +14 | Higher | -8 | Lower |
| LLOC | 34 | **20 ✅** | 33 | +14 | Higher | +1 | Higher | -13 | Lower |
| Comments (CLOC) | **13 ✅** | 12 | 0 | +1 | Higher | +13 | Higher | +12 | Higher |
| Blank Lines | 6 | 7 | **5 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00104

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.2629 | 58.3751 | **64.0597 ✅** | -1.1122 | Lower | -6.7968 | Lower | -5.6846 | Lower |
| MI (SEI) | 21.5278 | **44.1678 ✅** | 38.8882 | -22.6400 | Lower | -17.3604 | Lower | +5.2796 | Higher |
| Cyclomatic (sum) | 19 | **14 ✅** | 15 | +5 | Higher | +4 | Higher | -1 | Lower |
| Cognitive (sum) | 20 | **9 ✅** | 12 | +11 | Higher | +8 | Higher | -3 | Lower |
| Halstead Volume | 2133.8889 | 1896 | **1868.6959 ✅** | +237.8889 | Higher | +265.1930 | Higher | +27.3041 | Higher |
| Halstead Effort | 104027.0855 | 69870.1622 | **68154.0104 ✅** | +34156.9233 | Higher | +35873.0751 | Higher | +1716.1518 | Higher |
| SLOC | 73 | 76 | **53 ✅** | -3 | Lower | +20 | Higher | +23 | Higher |
| PLOC | 61 | 50 | **45 ✅** | +11 | Higher | +16 | Higher | +5 | Higher |
| LLOC | 36 | 33 | **29 ✅** | +3 | Higher | +7 | Higher | +4 | Higher |
| Comments (CLOC) | 2 | **18 ✅** | 4 | -16 | Lower | -2 | Lower | +14 | Higher |
| Blank Lines | 10 | 13 | **8 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 7 | **1 ✅** | 3 | +6 | Higher | +4 | Higher | -2 | Lower |

## p00105

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.6565 | **63.8827 ✅** | 49.8246 | -3.2262 | Lower | +10.8320 | Higher | +14.0581 | Higher |
| MI (SEI) | 32.5008 | **40.5866 ✅** | 15.8504 | -8.0858 | Lower | +16.6504 | Higher | +24.7362 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 24 | +2 | Higher | -12 | Lower | -14 | Lower |
| Cognitive (sum) | 12 | **10 ✅** | 48 | +2 | Higher | -36 | Lower | -38 | Lower |
| Halstead Volume | 2789.6519 | **2559.4126 ✅** | 2949.6643 | +230.2393 | Higher | -160.0124 | Lower | -390.2517 | Lower |
| Halstead Effort | 50860.1175 | **48803.8420 ✅** | 182141.7724 | +2056.2755 | Higher | -131281.6549 | Lower | -133337.9304 | Lower |
| SLOC | 60 | **52 ✅** | 97 | +8 | Higher | -37 | Lower | -45 | Lower |
| PLOC | 46 | **36 ✅** | 88 | +10 | Higher | -42 | Lower | -52 | Lower |
| LLOC | 22 | **19 ✅** | 70 | +3 | Higher | -48 | Lower | -51 | Lower |
| Comments (CLOC) | 4 | 5 | 5 | -1 | Lower | -1 | Lower | 0 | Same |
| Blank Lines | 11 | 11 | **9 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00106

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.6983 | 63.2741 | **71.3304 ✅** | -5.5758 | Lower | -13.6321 | Lower | -8.0563 | Lower |
| MI (SEI) | 26.8933 | **51.3584 ✅** | 45.7259 | -24.4651 | Lower | -18.8326 | Lower | +5.6325 | Higher |
| Cyclomatic (sum) | 17 | 13 | **11 ✅** | +4 | Higher | +6 | Higher | +2 | Higher |
| Cognitive (sum) | 13 | 15 | **9 ✅** | -2 | Lower | +4 | Higher | +6 | Higher |
| Halstead Volume | 2055.0218 | 2240.1353 | **1553.4093 ✅** | -185.1136 | Lower | +501.6125 | Higher | +686.7260 | Higher |
| Halstead Effort | 84590.4303 | **55037.0504 ✅** | 55513.9425 | +29553.3799 | Higher | +29076.4878 | Higher | -476.8921 | Lower |
| SLOC | 74 | 54 | **38 ✅** | +20 | Higher | +36 | Higher | +16 | Higher |
| PLOC | 61 | 34 | **30 ✅** | +27 | Higher | +31 | Higher | +4 | Higher |
| LLOC | 23 | 20 | **18 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| Comments (CLOC) | 4 | **13 ✅** | 2 | -9 | Lower | +2 | Higher | +11 | Higher |
| Blank Lines | 10 | 9 | **6 ✅** | +1 | Higher | +4 | Higher | +3 | Higher |
| Functions | 5 | **1 ✅** | 2 | +4 | Higher | +3 | Higher | -1 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 4 | 1 | 1 | +3 | Higher | +3 | Higher | 0 | Same |

## p00107

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **66.6257 ✅** | 63.8293 | 63.3042 | +2.7964 | Higher | +3.3215 | Higher | +0.5251 | Higher |
| MI (SEI) | 42.4700 | **42.7212 ✅** | 34.1669 | -0.2512 | Lower | +8.3032 | Higher | +8.5544 | Higher |
| Cyclomatic (sum) | 12 | 12 | 14 | 0 | Same | -2 | Lower | -2 | Lower |
| Cognitive (sum) | **9 ✅** | 14 | 11 | -5 | Lower | -2 | Lower | +3 | Higher |
| Halstead Volume | **1382.3949 ✅** | 2366.9164 | 1617.1234 | -984.5215 | Lower | -234.7285 | Lower | +749.7930 | Higher |
| Halstead Effort | **47277.9067 ✅** | 55690.1622 | 101070.2118 | -8412.2554 | Lower | -53792.3050 | Lower | -45380.0496 | Lower |
| SLOC | 52 | 52 | 59 | 0 | Same | -7 | Lower | -7 | Lower |
| PLOC | 39 | **38 ✅** | 47 | +1 | Higher | -8 | Lower | -9 | Lower |
| LLOC | 32 | **19 ✅** | 41 | +13 | Higher | -9 | Lower | -22 | Lower |
| Comments (CLOC) | 4 | **6 ✅** | 3 | -2 | Lower | +1 | Higher | +3 | Higher |
| Blank Lines | 10 | **9 ✅** | 11 | +1 | Higher | -1 | Lower | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00108

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.5829 | **67.4513 ✅** | 53.4681 | -10.8684 | Lower | +3.1148 | Higher | +13.9832 | Higher |
| MI (SEI) | 24.5443 | **48.1683 ✅** | 3.4737 | -23.6240 | Lower | +21.0706 | Higher | +44.6946 | Higher |
| Cyclomatic (sum) | 19 | **12 ✅** | 20 | +7 | Higher | -1 | Lower | -8 | Lower |
| Cognitive (sum) | 40 | **14 ✅** | 26 | +26 | Higher | +14 | Higher | -12 | Lower |
| Halstead Volume | 1630.3018 | **1253.0030 ✅** | 2206.2343 | +377.2988 | Higher | -575.9325 | Lower | -953.2313 | Lower |
| Halstead Effort | 55313.8112 | **37531.3546 ✅** | 123549.1202 | +17782.4566 | Higher | -68235.3090 | Lower | -86017.7656 | Lower |
| SLOC | 83 | **51 ✅** | 90 | +32 | Higher | -7 | Lower | -39 | Lower |
| PLOC | 67 | **37 ✅** | 79 | +30 | Higher | -12 | Lower | -42 | Lower |
| LLOC | 47 | **26 ✅** | 63 | +21 | Higher | -16 | Lower | -37 | Lower |
| Comments (CLOC) | 4 | **6 ✅** | 0 | -2 | Lower | +4 | Higher | +6 | Higher |
| Blank Lines | 12 | **9 ✅** | 11 | +3 | Higher | +1 | Higher | -2 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | **0 ✅** | 2 | 1 | -2 | Lower | -1 | Lower | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00109

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.2320 | **62.0041 ✅** | 57.9401 | -2.7720 | Lower | +1.2919 | Higher | +4.0640 | Higher |
| MI (SEI) | 35.0647 | **51.9596 ✅** | 33.0957 | -16.8949 | Lower | +1.9690 | Higher | +18.8640 | Higher |
| Cyclomatic (sum) | 16 | **10 ✅** | 18 | +6 | Higher | -2 | Lower | -8 | Lower |
| Cognitive (sum) | 16 | **8 ✅** | 19 | +8 | Higher | -3 | Lower | -11 | Lower |
| Halstead Volume | **1988.7877 ✅** | 2613.9283 | 2135.0654 | -625.1406 | Lower | -146.2777 | Lower | +478.8629 | Higher |
| Halstead Effort | 97176.2838 | **50971.6024 ✅** | 117772.9647 | +46204.6813 | Higher | -20596.6809 | Lower | -66801.3622 | Lower |
| SLOC | 69 | **58 ✅** | 71 | +11 | Higher | -2 | Lower | -13 | Lower |
| PLOC | 63 | **29 ✅** | 65 | +34 | Higher | -2 | Lower | -36 | Lower |
| LLOC | 47 | **9 ✅** | 48 | +38 | Higher | -1 | Lower | -39 | Lower |
| Comments (CLOC) | 7 | **17 ✅** | 7 | -10 | Lower | 0 | Same | +10 | Higher |
| Blank Lines | 6 | 14 | 6 | -8 | Lower | 0 | Same | +8 | Higher |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |

## p00110

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 69.2814 | 64.8740 | **70.1931 ✅** | +4.4075 | Higher | -0.9117 | Lower | -5.3192 | Lower |
| MI (SEI) | 36.5911 | **41.6292 ✅** | 26.5846 | -5.0382 | Lower | +10.0064 | Higher | +15.0446 | Higher |
| Cyclomatic (sum) | 10 | 12 | 10 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 14 | **11 ✅** | 12 | +3 | Higher | +2 | Higher | -1 | Lower |
| Halstead Volume | **1327.7858 ✅** | 1625.7119 | 1374.7744 | -297.9261 | Lower | -46.9886 | Lower | +250.9375 | Higher |
| Halstead Effort | 37878.7776 | 41739.2079 | **37534.9585 ✅** | -3860.4303 | Lower | +343.8191 | Higher | +4204.2495 | Higher |
| SLOC | 46 | 55 | **43 ✅** | -9 | Lower | +3 | Higher | +12 | Higher |
| PLOC | 39 | 42 | **37 ✅** | -3 | Lower | +2 | Higher | +5 | Higher |
| LLOC | 25 | 24 | **23 ✅** | +1 | Higher | +2 | Higher | +1 | Higher |
| Comments (CLOC) | 1 | **5 ✅** | 0 | -4 | Lower | +1 | Higher | +5 | Higher |
| Blank Lines | 6 | 10 | 6 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p00111

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 57.2343 | **60.7055 ✅** | 58.2205 | -3.4711 | Lower | -0.9862 | Lower | +2.4849 | Higher |
| MI (SEI) | 7.6854 | **39.2395 ✅** | 9.1082 | -31.5541 | Lower | -1.4228 | Lower | +30.1313 | Higher |
| Cyclomatic (sum) | 8 | 13 | 8 | -5 | Lower | 0 | Same | +5 | Higher |
| Cognitive (sum) | 15 | **11 ✅** | 12 | +4 | Higher | +3 | Higher | -1 | Lower |
| Halstead Volume | 3977.7619 | **2162.3900 ✅** | 3441.4434 | +1815.3718 | Higher | +536.3185 | Higher | -1279.0533 | Lower |
| Halstead Effort | 77088.4080 | **35069.2564 ✅** | 62232.7678 | +42019.1516 | Higher | +14855.6402 | Higher | -27163.5114 | Lower |
| SLOC | 70 | **64 ✅** | 69 | +6 | Higher | +1 | Higher | -5 | Lower |
| PLOC | 64 | **45 ✅** | 63 | +19 | Higher | +1 | Higher | -18 | Lower |
| LLOC | 83 | **15 ✅** | 83 | +68 | Higher | 0 | Same | -68 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 6 | 13 | 6 | -7 | Lower | 0 | Same | +7 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00112

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.0058 | **72.5041 ✅** | 69.4486 | -5.4982 | Lower | -2.4428 | Lower | +3.0554 | Higher |
| MI (SEI) | 50.4998 | **57.2650 ✅** | 36.8323 | -6.7652 | Lower | +13.6675 | Higher | +20.4327 | Higher |
| Cyclomatic (sum) | 12 | 10 | 10 | +2 | Higher | +2 | Higher | 0 | Same |
| Cognitive (sum) | 21 | **8 ✅** | 11 | +13 | Higher | +10 | Higher | -3 | Lower |
| Halstead Volume | 1210.9004 | **881.5094 ✅** | 1285.7686 | +329.3909 | Higher | -74.8682 | Lower | -404.2591 | Lower |
| Halstead Effort | 47383.0579 | **26905.2013 ✅** | 54928.0326 | +20477.8566 | Higher | -7544.9747 | Lower | -28022.8313 | Lower |
| SLOC | 53 | **43 ✅** | 46 | +10 | Higher | +7 | Higher | -3 | Lower |
| PLOC | 40 | **31 ✅** | 39 | +9 | Higher | +1 | Higher | -8 | Lower |
| LLOC | 29 | **20 ✅** | 35 | +9 | Higher | -6 | Lower | -15 | Lower |
| Comments (CLOC) | **8 ✅** | 6 | 1 | +2 | Higher | +7 | Higher | +5 | Higher |
| Blank Lines | 9 | 9 | **6 ✅** | 0 | Same | +3 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00113

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 55.9555 | 62.6436 | **82.1154 ✅** | -6.6881 | Lower | -26.1599 | Lower | -19.4718 | Lower |
| MI (SEI) | 26.9989 | 35.2157 | **90.4543 ✅** | -8.2169 | Lower | -63.4554 | Lower | -55.2385 | Lower |
| Cyclomatic (sum) | **10 ✅** | 12 | 16 | -2 | Lower | -6 | Lower | -4 | Lower |
| Cognitive (sum) | **9 ✅** | 12 | 13 | -3 | Lower | -4 | Lower | -1 | Lower |
| Halstead Volume | 3459.9575 | **1808.1385 ✅** | 3511.6111 | +1651.8190 | Higher | -51.6536 | Lower | -1703.4726 | Lower |
| Halstead Effort | 104205.7778 | **76173.6292 ✅** | 112176.4651 | +28032.1486 | Higher | -7970.6873 | Lower | -36002.8359 | Lower |
| SLOC | 77 | 61 | **14 ✅** | +16 | Higher | +63 | Higher | +47 | Higher |
| PLOC | 63 | **47 ✅** | 62 | +16 | Higher | +1 | Higher | -15 | Lower |
| LLOC | 49 | **35 ✅** | 53 | +14 | Higher | -4 | Lower | -18 | Lower |
| Comments (CLOC) | 6 | 4 | **8 ✅** | +2 | Higher | -2 | Lower | -4 | Lower |
| Blank Lines | 13 | 13 | **-48 ✅** | 0 | Same | +61 | Higher | +61 | Higher |
| Functions | 2 | 2 | 3 | 0 | Same | -1 | Lower | -1 | Lower |
| Closures | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |

## p00114

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.5494 | 55.1049 | **61.7295 ✅** | +5.4445 | Higher | -1.1800 | Lower | -6.6246 | Lower |
| MI (SEI) | 26.9070 | **27.4579 ✅** | 14.9851 | -0.5509 | Lower | +11.9219 | Higher | +12.4728 | Higher |
| Cyclomatic (sum) | 17 | 20 | **16 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 13 | 11 | 11 | +2 | Higher | +2 | Higher | 0 | Same |
| Halstead Volume | 1866.9340 | 3227.4332 | **1806.0874 ✅** | -1360.4992 | Lower | +60.8466 | Higher | +1421.3458 | Higher |
| Halstead Effort | **80253.5980 ✅** | 90291.2855 | 84478.2807 | -10037.6874 | Lower | -4224.6827 | Lower | +5813.0048 | Higher |
| SLOC | 64 | 72 | **61 ✅** | -8 | Lower | +3 | Higher | +11 | Higher |
| PLOC | 52 | 52 | **51 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| LLOC | 41 | **26 ✅** | 43 | +15 | Higher | -2 | Lower | -17 | Lower |
| Comments (CLOC) | 2 | **6 ✅** | 0 | -4 | Lower | +2 | Higher | +6 | Higher |
| Blank Lines | 11 | 16 | **10 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| Functions | 3 | 4 | 3 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |

## p00115

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.5507 | **61.0699 ✅** | 45.7881 | -12.5192 | Lower | +2.7626 | Higher | +15.2818 | Higher |
| MI (SEI) | 4.1494 | **50.8656 ✅** | 8.5051 | -46.7162 | Lower | -4.3557 | Lower | +42.3605 | Higher |
| Cyclomatic (sum) | 22 | **11 ✅** | 23 | +11 | Higher | -1 | Lower | -12 | Lower |
| Cognitive (sum) | 23 | **4 ✅** | 25 | +19 | Higher | -2 | Lower | -21 | Lower |
| Halstead Volume | **3313.6551 ✅** | 3739.2997 | 3837.7737 | -425.6446 | Lower | -524.1186 | Lower | -98.4740 | Lower |
| Halstead Effort | 168167.9970 | **85706.0726 ✅** | 246292.6831 | +82461.9245 | Higher | -78124.6861 | Lower | -160586.6105 | Lower |
| SLOC | 104 | **54 ✅** | 116 | +50 | Higher | -12 | Lower | -62 | Lower |
| PLOC | 85 | **37 ✅** | 98 | +48 | Higher | -13 | Lower | -61 | Lower |
| LLOC | 41 | **19 ✅** | 66 | +22 | Higher | -25 | Lower | -47 | Lower |
| Comments (CLOC) | 1 | **16 ✅** | 5 | -15 | Lower | -4 | Lower | +11 | Higher |
| Blank Lines | 18 | **9 ✅** | 13 | +9 | Higher | +5 | Higher | -4 | Lower |
| Functions | 5 | **2 ✅** | 5 | +3 | Higher | 0 | Same | -3 | Lower |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 5 | **4 ✅** | 5 | +1 | Higher | 0 | Same | -1 | Lower |

## p00116

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 62.9326 | 55.7476 | **64.6048 ✅** | +7.1850 | Higher | -1.6723 | Lower | -8.8573 | Lower |
| MI (SEI) | **33.8343 ✅** | 23.3321 | 33.2582 | +10.5023 | Higher | +0.5762 | Higher | -9.9261 | Lower |
| Cyclomatic (sum) | 16 | 17 | **13 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Cognitive (sum) | 40 | **28 ✅** | 37 | +12 | Higher | +3 | Higher | -9 | Lower |
| Halstead Volume | 1589.8739 | 2256.5852 | **1548.5721 ✅** | -666.7113 | Lower | +41.3018 | Higher | +708.0131 | Higher |
| Halstead Effort | 78187.7282 | 102067.0835 | **65384.1561 ✅** | -23879.3553 | Lower | +12803.5721 | Higher | +36682.9274 | Higher |
| SLOC | 59 | 81 | **56 ✅** | -22 | Lower | +3 | Higher | +25 | Higher |
| PLOC | 51 | 69 | **48 ✅** | -18 | Lower | +3 | Higher | +21 | Higher |
| LLOC | 38 | 45 | 38 | -7 | Lower | 0 | Same | +7 | Higher |
| Comments (CLOC) | 3 | **4 ✅** | 2 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |

## p00117

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.3580 | 45.9799 | **61.5013 ✅** | +13.3780 | Higher | -2.1434 | Lower | -15.5214 | Lower |
| MI (SEI) | 37.5929 | 24.4648 | **43.4928 ✅** | +13.1281 | Higher | -5.9000 | Lower | -19.0281 | Lower |
| Cyclomatic (sum) | 11 | 27 | 11 | -16 | Lower | 0 | Same | +16 | Higher |
| Cognitive (sum) | 16 | 21 | 16 | -5 | Lower | 0 | Same | +5 | Higher |
| Halstead Volume | 2421.6670 | 3456.9712 | **2354.1753 ✅** | -1035.3042 | Lower | +67.4917 | Higher | +1102.7959 | Higher |
| Halstead Effort | **94445.0131 ✅** | 188750.6286 | 98875.3629 | -94305.6155 | Lower | -4430.3498 | Lower | +89875.2657 | Higher |
| SLOC | 69 | 112 | **61 ✅** | -43 | Lower | +8 | Higher | +51 | Higher |
| PLOC | 49 | 74 | **47 ✅** | -25 | Lower | +2 | Higher | +27 | Higher |
| LLOC | 40 | 41 | 40 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | 9 | **21 ✅** | 10 | -12 | Lower | -1 | Lower | +11 | Higher |
| Blank Lines | 12 | 20 | **8 ✅** | -8 | Lower | +4 | Higher | +12 | Higher |
| Functions | 2 | 7 | 2 | -5 | Lower | 0 | Same | +5 | Higher |
| Closures | 0 | 3 | 0 | -3 | Lower | 0 | Same | +3 | Higher |
| Exits | 1 | 5 | 1 | -4 | Lower | 0 | Same | +4 | Higher |

## p00118

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.9921 | 55.3487 | **62.2821 ✅** | +5.6434 | Higher | -1.2900 | Lower | -6.9335 | Lower |
| MI (SEI) | **39.0749 ✅** | 33.8193 | 15.7824 | +5.2556 | Higher | +23.2925 | Higher | +18.0369 | Higher |
| Cyclomatic (sum) | 17 | 20 | **16 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | **20 ✅** | 26 | 27 | -6 | Lower | -7 | Lower | -1 | Lower |
| Halstead Volume | 1991.1911 | 2498.3865 | **1900.2625 ✅** | -507.1954 | Lower | +90.9286 | Higher | +598.1240 | Higher |
| Halstead Effort | 122956.0489 | 120288.1697 | **107259.2605 ✅** | +2667.8793 | Higher | +15696.7884 | Higher | +13028.9092 | Higher |
| SLOC | 61 | 77 | **58 ✅** | -16 | Lower | +3 | Higher | +19 | Higher |
| PLOC | 53 | 58 | **51 ✅** | -5 | Lower | +2 | Higher | +7 | Higher |
| LLOC | 36 | 38 | **35 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Comments (CLOC) | 7 | **11 ✅** | 0 | -4 | Lower | +7 | Higher | +11 | Higher |
| Blank Lines | 8 | 15 | **7 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00120

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.8132 | **58.9946 ✅** | 55.3756 | -4.1815 | Lower | -0.5624 | Lower | +3.6190 | Higher |
| MI (SEI) | 13.6220 | **42.1652 ✅** | 6.0220 | -28.5432 | Lower | +7.6000 | Higher | +36.1432 | Higher |
| Cyclomatic (sum) | 18 | **12 ✅** | 18 | +6 | Higher | 0 | Same | -6 | Lower |
| Cognitive (sum) | 19 | **17 ✅** | 21 | +2 | Higher | -2 | Lower | -4 | Lower |
| Halstead Volume | **2307.1484 ✅** | 2723 | 2410.5528 | -415.8516 | Lower | -103.4044 | Lower | +312.4472 | Higher |
| Halstead Effort | 114085.5313 | **66286.3627 ✅** | 128263.8345 | +47799.1686 | Higher | -14178.3031 | Lower | -61977.4717 | Lower |
| SLOC | 84 | **67 ✅** | 80 | +17 | Higher | +4 | Higher | -13 | Lower |
| PLOC | 69 | **48 ✅** | 67 | +21 | Higher | +2 | Higher | -19 | Lower |
| LLOC | 37 | **30 ✅** | 43 | +7 | Higher | -6 | Lower | -13 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 0 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 14 | **10 ✅** | 13 | +4 | Higher | +1 | Higher | -3 | Lower |
| Functions | 5 | **1 ✅** | 3 | +4 | Higher | +2 | Higher | -2 | Lower |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 5 | **0 ✅** | 4 | +5 | Higher | +1 | Higher | -4 | Lower |

## p00121

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **55.7635 ✅** | 53.2787 | 48.4863 | +2.4847 | Higher | +7.2772 | Higher | +4.7924 | Higher |
| MI (SEI) | **20.8616 ✅** | 19.3932 | 6.0672 | +1.4684 | Higher | +14.7944 | Higher | +13.3260 | Higher |
| Cyclomatic (sum) | **14 ✅** | 18 | 16 | -4 | Lower | -2 | Lower | +2 | Higher |
| Cognitive (sum) | 29 | **19 ✅** | 23 | +10 | Higher | +6 | Higher | -4 | Lower |
| Halstead Volume | **2568.9093 ✅** | 2879.9957 | 3286.4099 | -311.0864 | Lower | -717.5006 | Lower | -406.4141 | Lower |
| Halstead Effort | 144221.2039 | **99430.6727 ✅** | 212813.2970 | +44790.5313 | Higher | -68592.0931 | Lower | -113382.6244 | Lower |
| SLOC | **81 ✅** | 86 | 114 | -5 | Lower | -33 | Lower | -28 | Lower |
| PLOC | **69 ✅** | 74 | 98 | -5 | Lower | -29 | Lower | -24 | Lower |
| LLOC | 51 | **40 ✅** | 69 | +11 | Higher | -18 | Lower | -29 | Lower |
| Comments (CLOC) | 3 | **4 ✅** | 2 | -1 | Lower | +1 | Higher | +2 | Higher |
| Blank Lines | **11 ✅** | 12 | 15 | -1 | Lower | -4 | Lower | -3 | Lower |
| Functions | **2 ✅** | 5 | 4 | -3 | Lower | -2 | Lower | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | **1 ✅** | 5 | 2 | -4 | Lower | -1 | Lower | +3 | Higher |

## p00122

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.4034 | **77.5686 ✅** | 46.6354 | -29.1652 | Lower | +1.7680 | Higher | +30.9332 | Higher |
| MI (SEI) | 15.5603 | **82.3945 ✅** | 8.9463 | -66.8341 | Lower | +6.6141 | Higher | +73.4482 | Higher |
| Cyclomatic (sum) | 25 | **17 ✅** | 25 | +8 | Higher | 0 | Same | -8 | Lower |
| Cognitive (sum) | 36 | **14 ✅** | 36 | +22 | Higher | 0 | Same | -22 | Lower |
| Halstead Volume | 3830.7235 | **1502.3702 ✅** | 3952.4705 | +2328.3533 | Higher | -121.7470 | Lower | -2450.1004 | Lower |
| Halstead Effort | 248752.5129 | **29105.6188 ✅** | 252167.6198 | +219646.8940 | Higher | -3415.1069 | Lower | -223062.0009 | Lower |
| SLOC | 96 | **24 ✅** | 106 | +72 | Higher | -10 | Lower | -82 | Lower |
| PLOC | 79 | **29 ✅** | 89 | +50 | Higher | -10 | Lower | -60 | Lower |
| LLOC | 63 | **13 ✅** | 75 | +50 | Higher | -12 | Lower | -62 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 4 | -6 | Lower | +2 | Higher | +8 | Higher |
| Blank Lines | 15 | **-12 ✅** | 15 | +27 | Higher | 0 | Same | -27 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |
| Exits | 2 | 8 | 2 | -6 | Lower | 0 | Same | +6 | Higher |

## p00123

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **72.5535 ✅** | 65.4696 | 70.6310 | +7.0839 | Higher | +1.9225 | Higher | -5.1614 | Lower |
| MI (SEI) | **57.8666 ✅** | 32.4911 | 56.6794 | +25.3756 | Higher | +1.1872 | Higher | -24.1884 | Lower |
| Cyclomatic (sum) | 19 | 20 | 19 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 14 | **13 ✅** | 19 | +1 | Higher | -5 | Lower | -6 | Lower |
| Halstead Volume | **936.5803 ✅** | 2191 | 1150.4906 | -1254.4197 | Lower | -213.9103 | Lower | +1040.5094 | Higher |
| Halstead Effort | **13407.8860 ✅** | 43988.5385 | 18528.9536 | -30580.6524 | Lower | -5121.0676 | Lower | +25459.5849 | Higher |
| SLOC | **37 ✅** | 43 | 39 | -6 | Lower | -2 | Lower | +4 | Higher |
| PLOC | **30 ✅** | 37 | 32 | -7 | Lower | -2 | Lower | +5 | Higher |
| LLOC | **8 ✅** | 10 | 17 | -2 | Lower | -9 | Lower | -7 | Lower |
| Comments (CLOC) | 5 | 1 | **6 ✅** | +4 | Higher | -1 | Lower | -5 | Lower |
| Blank Lines | 4 | 5 | **3 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00124

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **69.5169 ✅** | 62.4954 | 62.7023 | +7.0215 | Higher | +6.8146 | Higher | -0.2069 | Lower |
| MI (SEI) | **50.2966 ✅** | 44.4233 | 39.0246 | +5.8733 | Higher | +11.2721 | Higher | +5.3988 | Higher |
| Cyclomatic (sum) | 10 | 12 | **9 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Cognitive (sum) | 9 | 9 | 16 | 0 | Same | -7 | Lower | -7 | Lower |
| Halstead Volume | **1358.9211 ✅** | 1601.9498 | 2041.5115 | -243.0288 | Lower | -682.5905 | Lower | -439.5617 | Lower |
| Halstead Effort | **41644.3548 ✅** | 55765.1727 | 95023.0810 | -14120.8179 | Lower | -53378.7261 | Lower | -39257.9082 | Lower |
| SLOC | **45 ✅** | 64 | 61 | -19 | Lower | -16 | Lower | +3 | Higher |
| PLOC | **32 ✅** | 44 | 48 | -12 | Lower | -16 | Lower | -4 | Lower |
| LLOC | 23 | **22 ✅** | 48 | +1 | Higher | -25 | Lower | -26 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 6 | -5 | Lower | -1 | Lower | +4 | Higher |
| Blank Lines | 10 | 11 | **9 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |

## p00125

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.1925 | **68.1428 ✅** | 53.6661 | -13.9503 | Lower | +0.5264 | Higher | +14.4766 | Higher |
| MI (SEI) | 34.0208 | **48.6250 ✅** | 27.2888 | -14.6042 | Lower | +6.7320 | Higher | +21.3362 | Higher |
| Cyclomatic (sum) | 25 | **8 ✅** | 23 | +17 | Higher | +2 | Higher | -15 | Lower |
| Cognitive (sum) | 12 | **3 ✅** | 9 | +9 | Higher | +3 | Higher | -6 | Lower |
| Halstead Volume | 3520.4347 | **2227.8740 ✅** | 3727.2874 | +1292.5607 | Higher | -206.8527 | Lower | -1499.4134 | Lower |
| Halstead Effort | 91545.8495 | **39042.9397 ✅** | 87532.0904 | +52502.9098 | Higher | +4013.7591 | Higher | -48489.1506 | Lower |
| SLOC | 69 | **43 ✅** | 72 | +26 | Higher | -3 | Lower | -29 | Lower |
| PLOC | 48 | **30 ✅** | 53 | +18 | Higher | -5 | Lower | -23 | Lower |
| LLOC | 24 | **12 ✅** | 18 | +12 | Higher | +6 | Higher | -6 | Lower |
| Comments (CLOC) | **11 ✅** | 5 | 7 | +6 | Higher | +4 | Higher | -2 | Lower |
| Blank Lines | 13 | **8 ✅** | 13 | +5 | Higher | 0 | Same | -5 | Lower |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 0 | 2 | 0 | -2 | Lower | 0 | Same | +2 | Higher |
| Exits | 3 | 0 | 0 | +3 | Higher | +3 | Higher | 0 | Same |

## p00126

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 54.5556 | **61.6582 ✅** | 55.4170 | -7.1026 | Lower | -0.8614 | Lower | +6.2412 | Higher |
| MI (SEI) | 14.2315 | **47.5678 ✅** | 6.5908 | -33.3364 | Lower | +7.6407 | Higher | +40.9771 | Higher |
| Cyclomatic (sum) | 24 | **15 ✅** | 23 | +9 | Higher | +1 | Higher | -8 | Lower |
| Cognitive (sum) | 47 | **31 ✅** | 44 | +16 | Higher | +3 | Higher | -13 | Lower |
| Halstead Volume | 2438.1480 | **1913.8326 ✅** | 2443.9809 | +524.3155 | Higher | -5.8329 | Lower | -530.1484 | Lower |
| Halstead Effort | 136070.1729 | **93951.7799 ✅** | 146638.8550 | +42118.3930 | Higher | -10568.6820 | Lower | -52687.0751 | Lower |
| SLOC | 77 | **61 ✅** | 74 | +16 | Higher | +3 | Higher | -13 | Lower |
| PLOC | 67 | **48 ✅** | 64 | +19 | Higher | +3 | Higher | -16 | Lower |
| LLOC | 48 | **37 ✅** | 47 | +11 | Higher | +1 | Higher | -10 | Lower |
| Comments (CLOC) | 1 | **13 ✅** | 0 | -12 | Lower | +1 | Higher | +13 | Higher |
| Blank Lines | 10 | **6 ✅** | 10 | +4 | Higher | 0 | Same | -4 | Lower |
| Functions | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00127

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **75.2754 ✅** | 64.5768 | 71.4672 | +10.6986 | Higher | +3.8082 | Higher | -6.8904 | Lower |
| MI (SEI) | 52.0128 | 52.1930 | **61.4878 ✅** | -0.1802 | Lower | -9.4750 | Lower | -9.2948 | Lower |
| Cyclomatic (sum) | **10 ✅** | 11 | 12 | -1 | Lower | -2 | Lower | -1 | Lower |
| Cognitive (sum) | 8 | 8 | 12 | 0 | Same | -4 | Lower | -4 | Lower |
| Halstead Volume | **982.4014 ✅** | 2019.2334 | 1059.8469 | -1036.8321 | Lower | -77.4456 | Lower | +959.3865 | Higher |
| Halstead Effort | **21556.6933 ✅** | 41958.4833 | 30735.5614 | -20401.7900 | Lower | -9178.8680 | Lower | +11222.9219 | Higher |
| SLOC | **35 ✅** | 53 | 42 | -18 | Lower | -7 | Lower | +11 | Higher |
| PLOC | **28 ✅** | 37 | 31 | -9 | Lower | -3 | Lower | +6 | Higher |
| LLOC | 15 | 15 | 18 | 0 | Same | -3 | Lower | -3 | Lower |
| Comments (CLOC) | 2 | **12 ✅** | 9 | -10 | Lower | -7 | Lower | +3 | Higher |
| Blank Lines | 6 | 7 | 6 | -1 | Lower | 0 | Same | +1 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00128

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.1880 | **75.9542 ✅** | 63.4545 | -9.7662 | Lower | +2.7335 | Higher | +12.4997 | Higher |
| MI (SEI) | 45.1901 | **71.7598 ✅** | 42.2200 | -26.5697 | Lower | +2.9701 | Higher | +29.5398 | Higher |
| Cyclomatic (sum) | 11 | **7 ✅** | 13 | +4 | Higher | -2 | Lower | -6 | Lower |
| Cognitive (sum) | 20 | **7 ✅** | 12 | +13 | Higher | +8 | Higher | -5 | Lower |
| Halstead Volume | 1247.7500 | **601.3779 ✅** | 1480.1258 | +646.3721 | Higher | -232.3758 | Lower | -878.7479 | Lower |
| Halstead Effort | 35667.8236 | **8218.8310 ✅** | 47400.1250 | +27448.9927 | Higher | -11732.3014 | Lower | -39181.2941 | Lower |
| SLOC | 56 | **41 ✅** | 61 | +15 | Higher | -5 | Lower | -20 | Lower |
| PLOC | 41 | **25 ✅** | 47 | +16 | Higher | -6 | Lower | -22 | Lower |
| LLOC | 34 | **14 ✅** | 36 | +20 | Higher | -2 | Lower | -22 | Lower |
| Comments (CLOC) | 6 | **12 ✅** | 7 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 10 | **7 ✅** | 11 | +3 | Higher | -1 | Lower | -4 | Lower |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00129

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 41.9261 | **56.4214 ✅** | 41.2920 | -14.4953 | Lower | +0.6341 | Higher | +15.1294 | Higher |
| MI (SEI) | 2.6157 | **31.8490 ✅** | -5.9251 | -29.2333 | Lower | +8.5408 | Higher | +37.7741 | Higher |
| Cyclomatic (sum) | 34 | **13 ✅** | 31 | +21 | Higher | +3 | Higher | -18 | Lower |
| Cognitive (sum) | 46 | **10 ✅** | 45 | +36 | Higher | +1 | Higher | -35 | Lower |
| Halstead Volume | 5379.9144 | **2459.3350 ✅** | 5754.9290 | +2920.5794 | Higher | -375.0146 | Lower | -3295.5940 | Lower |
| Halstead Effort | 332271.7879 | **101242.6237 ✅** | 357435.0441 | +231029.1642 | Higher | -25163.2562 | Lower | -256192.4204 | Lower |
| SLOC | 113 | **80 ✅** | 120 | +33 | Higher | -7 | Lower | -40 | Lower |
| PLOC | 97 | **61 ✅** | 106 | +36 | Higher | -9 | Lower | -45 | Lower |
| LLOC | 60 | **30 ✅** | 70 | +30 | Higher | -10 | Lower | -40 | Lower |
| Comments (CLOC) | 4 | **9 ✅** | 1 | -5 | Lower | +3 | Higher | +8 | Higher |
| Blank Lines | 16 | 14 | 14 | +2 | Higher | +2 | Higher | 0 | Same |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Exits | 8 | **5 ✅** | 8 | +3 | Higher | 0 | Same | -3 | Lower |

## p00130

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.8200 ✅** | 70.7709 | 73.3991 | +3.0491 | Higher | +0.4209 | Higher | -2.6282 | Lower |
| MI (SEI) | 64.4398 | 60.8746 | **66.5322 ✅** | +3.5653 | Higher | -2.0923 | Lower | -5.6576 | Lower |
| Cyclomatic (sum) | 10 | 11 | 10 | -1 | Lower | 0 | Same | +1 | Higher |
| Cognitive (sum) | 14 | **10 ✅** | 17 | +4 | Higher | -3 | Lower | -7 | Lower |
| Halstead Volume | **1005.9320 ✅** | 1021.5380 | 1005.9566 | -15.6060 | Lower | -0.0246 | Lower | +15.5814 | Higher |
| Halstead Effort | 21819.8477 | 26976.1694 | **21806.5429 ✅** | -5156.3217 | Lower | +13.3048 | Higher | +5169.6265 | Higher |
| SLOC | **38 ✅** | 45 | 39 | -7 | Lower | -1 | Lower | +6 | Higher |
| PLOC | 30 | 31 | **29 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| LLOC | 21 | **19 ✅** | 21 | +2 | Higher | 0 | Same | -2 | Lower |
| Comments (CLOC) | 8 | 10 | 10 | -2 | Lower | -2 | Lower | 0 | Same |
| Blank Lines | 5 | 7 | 5 | -2 | Lower | 0 | Same | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 2 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |

## p00131

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 60.1924 | 63.4847 | **66.5075 ✅** | -3.2923 | Lower | -6.3151 | Lower | -3.0228 | Lower |
| MI (SEI) | 42.8244 | **48.3214 ✅** | 44.8864 | -5.4970 | Lower | -2.0620 | Lower | +3.4350 | Higher |
| Cyclomatic (sum) | 26 | **10 ✅** | 13 | +16 | Higher | +13 | Higher | -3 | Lower |
| Cognitive (sum) | 60 | **15 ✅** | 30 | +45 | Higher | +30 | Higher | -15 | Lower |
| Halstead Volume | 2897.7703 | 2763.0064 | **1437.3657 ✅** | +134.7639 | Higher | +1460.4046 | Higher | +1325.6407 | Higher |
| Halstead Effort | 74494.5661 | **56960.4391 ✅** | 63575.7915 | +17534.1270 | Higher | +10918.7746 | Higher | -6615.3524 | Lower |
| SLOC | **50 ✅** | 52 | 51 | -2 | Lower | -1 | Lower | +1 | Higher |
| PLOC | 48 | **39 ✅** | 42 | +9 | Higher | +6 | Higher | -3 | Lower |
| LLOC | 34 | **22 ✅** | 34 | +12 | Higher | 0 | Same | -12 | Lower |
| Comments (CLOC) | 8 | **10 ✅** | 5 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | **-2 ✅** | 7 | 6 | -9 | Lower | -8 | Lower | +1 | Higher |
| Functions | 2 | 1 | 1 | +1 | Higher | +1 | Higher | 0 | Same |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00132

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 18.1334 | **38.9233 ✅** | 19.4598 | -20.7899 | Lower | -1.3264 | Lower | +19.4634 | Higher |
| MI (SEI) | -42.8197 | **3.9597 ✅** | -41.3134 | -46.7795 | Lower | -1.5063 | Lower | +45.2731 | Higher |
| Cyclomatic (sum) | 66 | **44 ✅** | 62 | +22 | Higher | +4 | Higher | -18 | Lower |
| Cognitive (sum) | 104 | **44 ✅** | 116 | +60 | Higher | -12 | Lower | -72 | Lower |
| Halstead Volume | 9925.1235 | **4386.7109 ✅** | 10392.1326 | +5538.4126 | Higher | -467.0091 | Lower | -6005.4217 | Lower |
| Halstead Effort | 1139989.5029 | **260440.6498 ✅** | 1092487.8689 | +879548.8530 | Higher | +47501.6339 | Higher | -832047.2191 | Lower |
| SLOC | 256 | **126 ✅** | 246 | +130 | Higher | +10 | Higher | -120 | Lower |
| PLOC | 218 | **110 ✅** | 213 | +108 | Higher | +5 | Higher | -103 | Lower |
| LLOC | 153 | **38 ✅** | 161 | +115 | Higher | -8 | Lower | -123 | Lower |
| Comments (CLOC) | 0 | **8 ✅** | 0 | -8 | Lower | 0 | Same | +8 | Higher |
| Blank Lines | 38 | **14 ✅** | 33 | +24 | Higher | +5 | Higher | -19 | Lower |
| Functions | 6 | 9 | 6 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 5 | 7 | **3 ✅** | -2 | Lower | +2 | Higher | +4 | Higher |
| Exits | 8 | 10 | **7 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |

## p00133

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 59.2426 | **74.9095 ✅** | 68.3258 | -15.6669 | Lower | -9.0832 | Lower | +6.5837 | Higher |
| MI (SEI) | 30.6645 | **58.1906 ✅** | 55.9941 | -27.5261 | Lower | -25.3296 | Lower | +2.1965 | Higher |
| Cyclomatic (sum) | 14 | **8 ✅** | 9 | +6 | Higher | +5 | Higher | -1 | Lower |
| Cognitive (sum) | 8 | 9 | **6 ✅** | -1 | Lower | +2 | Higher | +3 | Higher |
| Halstead Volume | 3351.3814 | **1151.5023 ✅** | 1369.8434 | +2199.8790 | Higher | +1981.5380 | Higher | -218.3410 | Lower |
| Halstead Effort | 75275.1680 | **24955.2149 ✅** | 40948.5320 | +50319.9532 | Higher | +34326.6360 | Higher | -15993.3172 | Lower |
| SLOC | 60 | **35 ✅** | 49 | +25 | Higher | +11 | Higher | -14 | Lower |
| PLOC | 43 | **27 ✅** | 33 | +16 | Higher | +10 | Higher | -6 | Lower |
| LLOC | 22 | **20 ✅** | 21 | +2 | Higher | +1 | Higher | -1 | Lower |
| Comments (CLOC) | 4 | 4 | **10 ✅** | 0 | Same | -6 | Lower | -6 | Lower |
| Blank Lines | 13 | **4 ✅** | 7 | +9 | Higher | +6 | Higher | -3 | Lower |
| Functions | 3 | **2 ✅** | 3 | +1 | Higher | 0 | Same | -1 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00134

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 79.3340 | 76.0854 | **88.0316 ✅** | +3.2486 | Higher | -8.6976 | Lower | -11.9461 | Lower |
| MI (SEI) | 72.7091 | 68.1000 | **82.2860 ✅** | +4.6091 | Higher | -9.5769 | Lower | -14.1860 | Lower |
| Cyclomatic (sum) | 8 | 5 | **3 ✅** | +3 | Higher | +5 | Higher | +2 | Higher |
| Cognitive (sum) | 5 | 3 | **1 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Halstead Volume | 650.1045 | 691.8507 | **489.3062 ✅** | -41.7462 | Lower | +160.7982 | Higher | +202.5444 | Higher |
| Halstead Effort | 12660.7846 | 10994.4093 | **6931.8383 ✅** | +1666.3753 | Higher | +5728.9464 | Higher | +4062.5710 | Higher |
| SLOC | 32 | 40 | **22 ✅** | -8 | Lower | +10 | Higher | +18 | Higher |
| PLOC | 22 | 23 | **14 ✅** | -1 | Lower | +8 | Higher | +9 | Higher |
| LLOC | 12 | 19 | **11 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| Comments (CLOC) | 7 | **9 ✅** | 4 | -2 | Lower | +3 | Higher | +5 | Higher |
| Blank Lines | 6 | 8 | **5 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 3 | 2 | **0 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |

## p00135

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.1947 | 67.8126 | **74.6420 ✅** | -0.6179 | Lower | -7.4473 | Lower | -6.8294 | Lower |
| MI (SEI) | 44.6719 | 55.2894 | **59.7597 ✅** | -10.6175 | Lower | -15.0878 | Lower | -4.4703 | Lower |
| Cyclomatic (sum) | 9 | 12 | **8 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | **9 ✅** | 19 | 10 | -10 | Lower | -1 | Lower | +9 | Higher |
| Halstead Volume | 2381.0052 | 1243.2742 | **1019.5698 ✅** | +1137.7310 | Higher | +1361.4354 | Higher | +223.7044 | Higher |
| Halstead Effort | 49349.7024 | 33102.1762 | **23059.2712 ✅** | +16247.5262 | Higher | +26290.4311 | Higher | +10042.9050 | Higher |
| SLOC | 44 | 50 | **37 ✅** | -6 | Lower | +7 | Higher | +13 | Higher |
| PLOC | 33 | 32 | **26 ✅** | +1 | Higher | +7 | Higher | +6 | Higher |
| LLOC | 18 | 19 | **15 ✅** | -1 | Lower | +3 | Higher | +4 | Higher |
| Comments (CLOC) | 4 | **10 ✅** | 5 | -6 | Lower | -1 | Lower | +5 | Higher |
| Blank Lines | 7 | 8 | **6 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00136

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.4230 | 65.8452 | **74.0603 ✅** | -0.4223 | Lower | -8.6373 | Lower | -8.2150 | Lower |
| MI (SEI) | 44.6941 | **54.9172 ✅** | 44.7608 | -10.2231 | Lower | -0.0667 | Lower | +10.1564 | Higher |
| Cyclomatic (sum) | 15 | 14 | **10 ✅** | +1 | Higher | +5 | Higher | +4 | Higher |
| Cognitive (sum) | 14 | 13 | **11 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Halstead Volume | 1281.0393 | 1234.5401 | **1043.7263 ✅** | +46.4991 | Higher | +237.3130 | Higher | +190.8139 | Higher |
| Halstead Effort | 32177.6836 | 29929.2568 | **22309.6489 ✅** | +2248.4269 | Higher | +9868.0348 | Higher | +7619.6079 | Higher |
| SLOC | 55 | 55 | **37 ✅** | 0 | Same | +18 | Higher | +18 | Higher |
| PLOC | 44 | 42 | **32 ✅** | +2 | Higher | +12 | Higher | +10 | Higher |
| LLOC | 30 | 28 | **26 ✅** | +2 | Higher | +4 | Higher | +2 | Higher |
| Comments (CLOC) | 6 | **13 ✅** | 1 | -7 | Lower | +5 | Higher | +12 | Higher |
| Blank Lines | 6 | 6 | **5 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | 2 | **0 ✅** | 0 | Same | +2 | Higher | +2 | Higher |

## p00137

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | **73.6861 ✅** | 70.3493 | 71.1260 | +3.3368 | Higher | +2.5602 | Higher | -0.7767 | Lower |
| MI (SEI) | 48.3965 | **57.0151 ✅** | 27.7268 | -8.6186 | Lower | +20.6697 | Higher | +29.2884 | Higher |
| Cyclomatic (sum) | 8 | **7 ✅** | 8 | +1 | Higher | 0 | Same | -1 | Lower |
| Cognitive (sum) | 5 | **3 ✅** | 5 | +2 | Higher | 0 | Same | -2 | Lower |
| Halstead Volume | **961.0998 ✅** | 2065 | 1168.5196 | -1103.9002 | Lower | -207.4198 | Lower | +896.4804 | Higher |
| Halstead Effort | **25027.0383 ✅** | 28147.2523 | 38431.3120 | -3120.2140 | Lower | -13404.2737 | Lower | -10284.0597 | Lower |
| SLOC | 40 | **39 ✅** | 44 | +1 | Higher | -4 | Lower | -5 | Lower |
| PLOC | 32 | **22 ✅** | 37 | +10 | Higher | -5 | Lower | -15 | Lower |
| LLOC | 23 | **10 ✅** | 30 | +13 | Higher | -7 | Lower | -20 | Lower |
| Comments (CLOC) | 2 | **7 ✅** | 0 | -5 | Lower | +2 | Higher | +7 | Higher |
| Blank Lines | **6 ✅** | 10 | 7 | -4 | Lower | -1 | Lower | +3 | Higher |
| Functions | 3 | **1 ✅** | 3 | +2 | Higher | 0 | Same | -2 | Lower |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 2 | **0 ✅** | 2 | +2 | Higher | 0 | Same | -2 | Lower |

## p00138

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.4983 | **66.0406 ✅** | 63.4126 | -0.5423 | Lower | +2.0856 | Higher | +2.6279 | Higher |
| MI (SEI) | 42.6201 | **49.3833 ✅** | 38.9006 | -6.7632 | Lower | +3.7194 | Higher | +10.4827 | Higher |
| Cyclomatic (sum) | 11 | **10 ✅** | 13 | +1 | Higher | -2 | Lower | -3 | Lower |
| Cognitive (sum) | 13 | **5 ✅** | 22 | +8 | Higher | -9 | Lower | -17 | Lower |
| Halstead Volume | **1595.6553 ✅** | 1795.5249 | 1655.3690 | -199.8696 | Lower | -59.7137 | Lower | +140.1559 | Higher |
| Halstead Effort | **43791.8727 ✅** | 45324.1786 | 56683.8480 | -1532.3059 | Lower | -12891.9753 | Lower | -11359.6694 | Lower |
| SLOC | 54 | **51 ✅** | 59 | +3 | Higher | -5 | Lower | -8 | Lower |
| PLOC | 41 | **35 ✅** | 46 | +6 | Higher | -5 | Lower | -11 | Lower |
| LLOC | 27 | **21 ✅** | 31 | +6 | Higher | -4 | Lower | -10 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 5 | -3 | Lower | 0 | Same | +3 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 3 | **0 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00139

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.9830 | **70.6576 ✅** | 60.3693 | -13.6746 | Lower | -3.3863 | Lower | +10.2883 | Higher |
| MI (SEI) | 36.4117 | **62.3228 ✅** | 13.7355 | -25.9111 | Lower | +22.6762 | Higher | +48.5873 | Higher |
| Cyclomatic (sum) | 24 | **14 ✅** | 23 | +10 | Higher | +1 | Higher | -9 | Lower |
| Cognitive (sum) | 65 | **13 ✅** | 63 | +52 | Higher | +2 | Higher | -50 | Lower |
| Halstead Volume | 1968.2816 | **914.3047 ✅** | 2014.2106 | +1053.9769 | Higher | -45.9290 | Lower | -1099.9059 | Lower |
| Halstead Effort | 69163.2289 | **21418.4342 ✅** | 78828.8795 | +47744.7947 | Higher | -9665.6505 | Lower | -57410.4452 | Lower |
| SLOC | 71 | **45 ✅** | 58 | +26 | Higher | +13 | Higher | -13 | Lower |
| PLOC | 56 | **33 ✅** | 54 | +23 | Higher | +2 | Higher | -21 | Lower |
| LLOC | 39 | **17 ✅** | 41 | +22 | Higher | -2 | Lower | -24 | Lower |
| Comments (CLOC) | 10 | **11 ✅** | 0 | -1 | Lower | +10 | Higher | +11 | Higher |
| Blank Lines | 7 | 6 | **4 ✅** | +1 | Higher | +3 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 4 | **0 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |

## p00140

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.4519 | 62.2434 | **67.8592 ✅** | +5.2086 | Higher | -0.4073 | Lower | -5.6158 | Lower |
| MI (SEI) | **48.8342 ✅** | 43.6015 | 34.9676 | +5.2328 | Higher | +13.8666 | Higher | +8.6339 | Higher |
| Cyclomatic (sum) | 14 | 14 | **13 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Cognitive (sum) | 28 | **19 ✅** | 28 | +9 | Higher | 0 | Same | -9 | Lower |
| Halstead Volume | **1298.9854 ✅** | 2613.0758 | 1636.8835 | -1314.0904 | Lower | -337.8981 | Lower | +976.1923 | Higher |
| Halstead Effort | **43214.4608 ✅** | 65292.5123 | 53618.9948 | -22078.0515 | Lower | -10404.5340 | Lower | +11673.5175 | Higher |
| SLOC | 49 | 54 | **45 ✅** | -5 | Lower | +4 | Higher | +9 | Higher |
| PLOC | **39 ✅** | 41 | 41 | -2 | Lower | -2 | Lower | 0 | Same |
| LLOC | 25 | **17 ✅** | 27 | +8 | Higher | -2 | Lower | -10 | Lower |
| Comments (CLOC) | 6 | **8 ✅** | 1 | -2 | Lower | +5 | Higher | +7 | Higher |
| Blank Lines | 4 | 7 | 4 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 1 | **0 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00141

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 48.7635 | **57.3667 ✅** | 47.3417 | -8.6032 | Lower | +1.4219 | Higher | +10.0250 | Higher |
| MI (SEI) | 14.6425 | **42.0615 ✅** | 6.6222 | -27.4191 | Lower | +8.0203 | Higher | +35.4393 | Higher |
| Cyclomatic (sum) | 28 | **21 ✅** | 31 | +7 | Higher | -3 | Lower | -10 | Lower |
| Cognitive (sum) | 39 | **14 ✅** | 44 | +25 | Higher | -5 | Lower | -30 | Lower |
| Halstead Volume | 2935.4389 | **2182.0248 ✅** | 3172.9445 | +753.4141 | Higher | -237.5056 | Lower | -990.9197 | Lower |
| Halstead Effort | 241684.4679 | **97463.7723 ✅** | 280945.5689 | +144220.6956 | Higher | -39261.1010 | Lower | -183481.7966 | Lower |
| SLOC | 98 | **70 ✅** | 100 | +28 | Higher | -2 | Lower | -30 | Lower |
| PLOC | 85 | **53 ✅** | 90 | +32 | Higher | -5 | Lower | -37 | Lower |
| LLOC | 74 | **33 ✅** | 78 | +41 | Higher | -4 | Lower | -45 | Lower |
| Comments (CLOC) | 5 | **15 ✅** | 2 | -10 | Lower | +3 | Higher | +13 | Higher |
| Blank Lines | 8 | 12 | 8 | -4 | Lower | 0 | Same | +4 | Higher |
| Functions | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |
| Closures | 0 | 0 | 1 | 0 | Same | -1 | Lower | -1 | Lower |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00142

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 63.4109 | **81.0043 ✅** | 60.0636 | -17.5934 | Lower | +3.3473 | Higher | +20.9407 | Higher |
| MI (SEI) | 42.2590 | **74.1634 ✅** | 38.7828 | -31.9044 | Lower | +3.4762 | Higher | +35.3806 | Higher |
| Cyclomatic (sum) | 14 | **6 ✅** | 16 | +8 | Higher | -2 | Lower | -10 | Lower |
| Cognitive (sum) | 32 | **8 ✅** | 39 | +24 | Higher | -7 | Lower | -31 | Lower |
| Halstead Volume | 1427.9958 | **699.9882 ✅** | 1550.5196 | +728.0077 | Higher | -122.5238 | Lower | -850.5314 | Lower |
| Halstead Effort | 61453.0628 | **13299.7749 ✅** | 82177.5390 | +48153.2878 | Higher | -20724.4762 | Lower | -68877.7641 | Lower |
| SLOC | 61 | **29 ✅** | 71 | +32 | Higher | -10 | Lower | -42 | Lower |
| PLOC | 46 | **22 ✅** | 54 | +24 | Higher | -8 | Lower | -32 | Lower |
| LLOC | 36 | **14 ✅** | 45 | +22 | Higher | -9 | Lower | -31 | Lower |
| Comments (CLOC) | 7 | 6 | **9 ✅** | +1 | Higher | -2 | Lower | -3 | Lower |
| Blank Lines | 9 | **5 ✅** | 11 | +4 | Higher | -2 | Lower | -6 | Lower |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00143

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.0265 | **67.5048 ✅** | 66.6258 | -1.4783 | Lower | -0.5993 | Lower | +0.8790 | Higher |
| MI (SEI) | 46.6107 | **49.2001 ✅** | 43.4935 | -2.5894 | Lower | +3.1172 | Higher | +5.7067 | Higher |
| Cyclomatic (sum) | 10 | 18 | 10 | -8 | Lower | 0 | Same | +8 | Higher |
| Cognitive (sum) | 5 | 13 | 5 | -8 | Lower | 0 | Same | +8 | Higher |
| Halstead Volume | **2174.6347 ✅** | 3691.5276 | 2212.6712 | -1516.8929 | Lower | -38.0365 | Lower | +1478.8564 | Higher |
| Halstead Effort | **88344.5350 ✅** | 95507.5445 | 89121.4776 | -7163.0095 | Lower | -776.9427 | Lower | +6386.0669 | Higher |
| SLOC | 48 | **33 ✅** | 46 | +15 | Higher | +2 | Higher | -13 | Lower |
| PLOC | 34 | 37 | 34 | -3 | Lower | 0 | Same | +3 | Higher |
| LLOC | **14 ✅** | 15 | 15 | -1 | Lower | -1 | Lower | 0 | Same |
| Comments (CLOC) | **6 ✅** | 4 | 4 | +2 | Higher | +2 | Higher | 0 | Same |
| Blank Lines | 9 | **-8 ✅** | 8 | +17 | Higher | +1 | Higher | -16 | Lower |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |

## p00144

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 56.7119 | 51.8342 | **57.8991 ✅** | +4.8777 | Higher | -1.1872 | Lower | -6.0649 | Lower |
| MI (SEI) | **30.5369 ✅** | 12.8277 | 9.2553 | +17.7092 | Higher | +21.2816 | Higher | +3.5724 | Higher |
| Cyclomatic (sum) | 16 | 22 | **14 ✅** | -6 | Lower | +2 | Higher | +8 | Higher |
| Cognitive (sum) | 15 | 19 | 15 | -4 | Lower | 0 | Same | +4 | Higher |
| Halstead Volume | 2490.2904 | 2862.9072 | **2458.9448 ✅** | -372.6168 | Lower | +31.3456 | Higher | +403.9624 | Higher |
| Halstead Effort | **104015.1773 ✅** | 132373.0658 | 104471.0018 | -28357.8885 | Lower | -455.8244 | Lower | +27902.0641 | Higher |
| SLOC | 75 | 89 | **72 ✅** | -14 | Lower | +3 | Higher | +17 | Higher |
| PLOC | 59 | 76 | 59 | -17 | Lower | 0 | Same | +17 | Higher |
| LLOC | 44 | **39 ✅** | 46 | +5 | Higher | -2 | Lower | -7 | Lower |
| Comments (CLOC) | **7 ✅** | 2 | 0 | +5 | Higher | +7 | Higher | +2 | Higher |
| Blank Lines | 16 | 13 | 13 | +3 | Higher | +3 | Higher | 0 | Same |
| Functions | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Closures | 2 | 0 | 0 | +2 | Higher | +2 | Higher | 0 | Same |
| Exits | 1 | 4 | 1 | -3 | Lower | 0 | Same | +3 | Higher |

## p00145

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 65.7494 | 67.1066 | **68.5318 ✅** | -1.3573 | Lower | -2.7825 | Lower | -1.4252 | Lower |
| MI (SEI) | 43.3814 | **52.3145 ✅** | 40.2291 | -8.9331 | Lower | +3.1524 | Higher | +12.0855 | Higher |
| Cyclomatic (sum) | 11 | **8 ✅** | 10 | +3 | Higher | +1 | Higher | -2 | Lower |
| Cognitive (sum) | 12 | **7 ✅** | 11 | +5 | Higher | +1 | Higher | -4 | Lower |
| Halstead Volume | 1710.1319 | 2360.0301 | **1642.3622 ✅** | -649.8982 | Lower | +67.7698 | Higher | +717.6680 | Higher |
| Halstead Effort | 100897.7830 | **45461.6328 ✅** | 93614.6426 | +55436.1502 | Higher | +7283.1404 | Higher | -48153.0098 | Lower |
| SLOC | 52 | 45 | 45 | +7 | Higher | +7 | Higher | 0 | Same |
| PLOC | 40 | **31 ✅** | 37 | +9 | Higher | +3 | Higher | -6 | Lower |
| LLOC | 38 | **16 ✅** | 36 | +22 | Higher | +2 | Higher | -20 | Lower |
| Comments (CLOC) | 5 | **8 ✅** | 2 | -3 | Lower | +3 | Higher | +6 | Higher |
| Blank Lines | 8 | 8 | **6 ✅** | 0 | Same | +2 | Higher | +2 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Exits | 1 | 0 | 0 | +1 | Higher | +1 | Higher | 0 | Same |

## p00146

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 53.4897 | **59.1621 ✅** | 53.3497 | -5.6724 | Lower | +0.1400 | Higher | +5.8124 | Higher |
| MI (SEI) | 3.4030 | **47.1299 ✅** | 3.2010 | -43.7269 | Lower | +0.2020 | Higher | +43.9289 | Higher |
| Cyclomatic (sum) | 19 | **12 ✅** | 19 | +7 | Higher | 0 | Same | -7 | Lower |
| Cognitive (sum) | 28 | **17 ✅** | 28 | +11 | Higher | 0 | Same | -11 | Lower |
| Halstead Volume | 3188.6498 | 3357.3692 | **3152.8317 ✅** | -168.7194 | Lower | +35.8181 | Higher | +204.5375 | Higher |
| Halstead Effort | 264028.5918 | **96728.5281 ✅** | 236000.9857 | +167300.0637 | Higher | +28027.6062 | Higher | -139272.4575 | Lower |
| SLOC | 81 | **62 ✅** | 82 | +19 | Higher | -1 | Lower | -20 | Lower |
| PLOC | 70 | **48 ✅** | 72 | +22 | Higher | -2 | Lower | -24 | Lower |
| LLOC | 58 | **34 ✅** | 58 | +24 | Higher | 0 | Same | -24 | Lower |
| Comments (CLOC) | 0 | **17 ✅** | 0 | -17 | Lower | 0 | Same | +17 | Higher |
| Blank Lines | 11 | **9 ✅** | 10 | +2 | Higher | +1 | Higher | -1 | Lower |
| Functions | 2 | 2 | 2 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | **0 ✅** | 1 | +1 | Higher | 0 | Same | -1 | Lower |
| Exits | 2 | **1 ✅** | 2 | +1 | Higher | 0 | Same | -1 | Lower |

## p00148

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 87.1787 | 75.1858 | **87.7172 ✅** | +11.9928 | Higher | -0.5386 | Lower | -12.5314 | Lower |
| MI (SEI) | **87.8028 ✅** | 67.1564 | 77.9975 | +20.6464 | Higher | +9.8052 | Higher | -10.8411 | Lower |
| Cyclomatic (sum) | 6 | 8 | 6 | -2 | Lower | 0 | Same | +2 | Higher |
| Cognitive (sum) | 10 | **7 ✅** | 11 | +3 | Higher | -1 | Lower | -4 | Lower |
| Halstead Volume | **385 ✅** | 1593.6195 | 396.3357 | -1208.6195 | Lower | -11.3357 | Lower | +1197.2838 | Higher |
| Halstead Effort | **6108.6667 ✅** | 22792.2618 | 6737.7070 | -16683.5951 | Lower | -629.0403 | Lower | +16054.5548 | Higher |
| SLOC | 24 | 31 | **23 ✅** | -7 | Lower | +1 | Higher | +8 | Higher |
| PLOC | **16 ✅** | 20 | 18 | -4 | Lower | -2 | Lower | +2 | Higher |
| LLOC | 11 | **8 ✅** | 11 | +3 | Higher | 0 | Same | -3 | Lower |
| Comments (CLOC) | 7 | 7 | 3 | 0 | Same | +4 | Higher | +4 | Higher |
| Blank Lines | 2 | 5 | 2 | -3 | Lower | 0 | Same | +3 | Higher |
| Functions | 1 | 1 | 1 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00149

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 67.3036 | **68.0875 ✅** | 67.5700 | -0.7839 | Lower | -0.2664 | Lower | +0.5175 | Higher |
| MI (SEI) | 51.1663 | **63.0669 ✅** | 50.1513 | -11.9006 | Lower | +1.0149 | Higher | +12.9156 | Higher |
| Cyclomatic (sum) | 12 | **10 ✅** | 12 | +2 | Higher | 0 | Same | -2 | Lower |
| Cognitive (sum) | 21 | **9 ✅** | 22 | +12 | Higher | -1 | Lower | -13 | Lower |
| Halstead Volume | 1213.4291 | **1140.1575 ✅** | 1224.7168 | +73.2716 | Higher | -11.2877 | Lower | -84.5593 | Lower |
| Halstead Effort | **30492.2985 ✅** | 35471.5665 | 30775.9478 | -4979.2680 | Lower | -283.6493 | Lower | +4695.6187 | Higher |
| SLOC | 52 | 52 | **51 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| PLOC | 37 | **32 ✅** | 39 | +5 | Higher | -2 | Lower | -7 | Lower |
| LLOC | 29 | **24 ✅** | 31 | +5 | Higher | -2 | Lower | -7 | Lower |
| Comments (CLOC) | 8 | **18 ✅** | 7 | -10 | Lower | +1 | Higher | +11 | Higher |
| Blank Lines | 7 | 9 | **6 ✅** | -2 | Lower | +1 | Higher | +3 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |

## p00150

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 70.9919 | 68.8131 | **71.5903 ✅** | +2.1788 | Higher | -0.5984 | Lower | -2.7772 | Lower |
| MI (SEI) | **52.8815 ✅** | 46.5185 | 45.0985 | +6.3630 | Higher | +7.7830 | Higher | +1.4200 | Higher |
| Cyclomatic (sum) | 12 | 15 | **11 ✅** | -3 | Lower | +1 | Higher | +4 | Higher |
| Cognitive (sum) | 16 | 16 | **15 ✅** | 0 | Same | +1 | Higher | +1 | Higher |
| Halstead Volume | 1004.6064 | **956.5627 ✅** | 1005.3806 | +48.0437 | Higher | -0.7742 | Lower | -48.8179 | Lower |
| Halstead Effort | 26956.9384 | **26049.2525 ✅** | 28753.8846 | +907.6859 | Higher | -1796.9462 | Lower | -2704.6321 | Lower |
| SLOC | 44 | 49 | **43 ✅** | -5 | Lower | +1 | Higher | +6 | Higher |
| PLOC | 33 | 39 | 33 | -6 | Lower | 0 | Same | +6 | Higher |
| LLOC | 20 | 21 | 20 | -1 | Lower | 0 | Same | +1 | Higher |
| Comments (CLOC) | **5 ✅** | 4 | 2 | +1 | Higher | +3 | Higher | +2 | Higher |
| Blank Lines | 8 | 8 | 8 | 0 | Same | 0 | Same | 0 | Same |
| Functions | 2 | 3 | 2 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## p00151

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.2654 | **55.2120 ✅** | 40.5569 | -8.9466 | Lower | +5.7085 | Higher | +14.6551 | Higher |
| MI (SEI) | 9.0933 | **31.9837 ✅** | 4.5250 | -22.8904 | Lower | +4.5683 | Higher | +27.4587 | Higher |
| Cyclomatic (sum) | 33 | **21 ✅** | 43 | +12 | Higher | -10 | Lower | -22 | Lower |
| Cognitive (sum) | 71 | **29 ✅** | 117 | +42 | Higher | -46 | Lower | -88 | Lower |
| Halstead Volume | 2810.6404 | **2095.7840 ✅** | 2764.5564 | +714.8564 | Higher | +46.0839 | Higher | -668.7724 | Lower |
| Halstead Effort | 254254.8531 | **113114.1197 ✅** | 286131.5913 | +141140.7333 | Higher | -31876.7383 | Lower | -173017.4716 | Lower |
| SLOC | 108 | **81 ✅** | 134 | +27 | Higher | -26 | Lower | -53 | Lower |
| PLOC | 92 | **64 ✅** | 117 | +28 | Higher | -25 | Lower | -53 | Lower |
| LLOC | 84 | **52 ✅** | 100 | +32 | Higher | -16 | Lower | -48 | Lower |
| Comments (CLOC) | 4 | **10 ✅** | 7 | -6 | Lower | -3 | Lower | +3 | Higher |
| Blank Lines | 12 | 13 | **11 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Functions | 1 | 2 | 1 | -1 | Lower | 0 | Same | +1 | Higher |
| Closures | 0 | 0 | 0 | 0 | Same | 0 | Same | 0 | Same |
| Exits | 0 | 1 | 0 | -1 | Lower | 0 | Same | +1 | Higher |

## p00152

| Metric | specs_and_code | specs_only | code_only | Delta (s+c - s) | Change | Delta (s+c - c) | Change | Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 46.2467 | **57.4878 ✅** | 45.5430 | -11.2412 | Lower | +0.7036 | Higher | +11.9448 | Higher |
| MI (SEI) | 8.6654 | **35.6094 ✅** | -0.1701 | -26.9440 | Lower | +8.8356 | Higher | +35.7795 | Higher |
| Cyclomatic (sum) | 23 | **17 ✅** | 27 | +6 | Higher | -4 | Lower | -10 | Lower |
| Cognitive (sum) | 31 | **11 ✅** | 37 | +20 | Higher | -6 | Lower | -26 | Lower |
| Halstead Volume | 2854.6023 | **1969.3143 ✅** | 3112.8630 | +885.2881 | Higher | -258.2606 | Lower | -1143.5487 | Lower |
| Halstead Effort | 169360.5522 | **76146.8186 ✅** | 215606.7195 | +93213.7336 | Higher | -46246.1673 | Lower | -139459.9009 | Lower |
| SLOC | 124 | **76 ✅** | 119 | +48 | Higher | +5 | Higher | -43 | Lower |
| PLOC | 105 | **57 ✅** | 105 | +48 | Higher | 0 | Same | -48 | Lower |
| LLOC | 96 | **32 ✅** | 98 | +64 | Higher | -2 | Lower | -66 | Lower |
| Comments (CLOC) | 5 | **10 ✅** | 1 | -5 | Lower | +4 | Higher | +9 | Higher |
| Blank Lines | 19 | **12 ✅** | 14 | +7 | Higher | +5 | Higher | -2 | Lower |
| Functions | 3 | 3 | 3 | 0 | Same | 0 | Same | 0 | Same |
| Closures | 1 | 2 | **0 ✅** | -1 | Lower | +1 | Higher | +2 | Higher |
| Exits | 1 | 3 | 1 | -2 | Lower | 0 | Same | +2 | Higher |

## Overall Averages

| Metric | Avg specs_and_code | Avg specs_only | Avg code_only | Avg Delta (s+c - s) | Change | Avg Delta (s+c - c) | Change | Avg Delta (s - c) | Change |
|---|---:|---:|---:|---:|---|---:|---|---:|---|
| MI (original) | 66.0670 | **67.5234 ✅** | 66.7178 | -1.4564 | Lower | -0.6508 | Lower | 0.8055 | Higher |
| MI (SEI) | 43.6029 | **52.6893 ✅** | 40.6393 | -9.0864 | Lower | 2.9636 | Higher | 12.0500 | Higher |
| Cyclomatic (sum) | 14.0286 | **11.8071 ✅** | 14.1214 | 2.2214 | Higher | -0.0929 | Lower | -2.3143 | Lower |
| Cognitive (sum) | 18.9000 | **11.4214 ✅** | 19.8643 | 7.4786 | Higher | -0.9643 | Lower | -8.4429 | Lower |
| Halstead Volume | 1755.3964 | **1687.5377 ✅** | 1709.0312 | 67.8587 | Higher | 46.3652 | Higher | -21.4935 | Lower |
| Halstead Effort | 73971.0965 | **46854.6873 ✅** | 84730.2670 | 27116.4093 | Higher | -10759.1705 | Lower | -37875.5797 | Lower |
| SLOC | 55.8643 | **50.3571 ✅** | 56.5000 | 5.5071 | Higher | -0.6357 | Lower | -6.1429 | Lower |
| PLOC | 44.2643 | **36.3786 ✅** | 46.9857 | 7.8857 | Higher | -2.7214 | Lower | -10.6071 | Lower |
| LLOC | 29.4571 | **20.1786 ✅** | 34.2214 | 9.2786 | Higher | -4.7643 | Lower | -14.0429 | Lower |
| Comments (CLOC) | 5.3071 | **8.9143 ✅** | 3.9857 | -3.6071 | Lower | 1.3214 | Higher | 4.9286 | Higher |
| Blank Lines | 8.2429 | 7.8714 | **7.3214 ✅** | 0.3714 | Higher | 0.9214 | Higher | 0.5500 | Higher |
| Functions | 1.7357 | 1.9071 | **1.6571 ✅** | -0.1714 | Lower | 0.0786 | Higher | 0.2500 | Higher |
| Closures | 0.4857 | 0.7143 | **0.3786 ✅** | -0.2286 | Lower | 0.1071 | Higher | 0.3357 | Higher |
| Exits | 0.9857 | 1.5071 | **0.7714 ✅** | -0.5214 | Lower | 0.2143 | Higher | 0.7357 | Higher |

